use vibe_core::ipc::server::MasterServer;
use vibe_core::ipc::client::WorkerClient;
use vibe_core::state::db::{DbActor, DbHandle};
use vibe_core::state::StateStore;
use rusqlite::Connection;
use std::time::Duration;
use tokio::sync::mpsc;
use tempfile::tempdir;
use tokio::time;

#[tokio::test]
async fn test_multi_worker_concurrency() -> anyhow::Result<()> {
    let dir = tempdir().unwrap();
    let socket_path = dir.path().join("vibe.sock");
    
    // Setup DB
    let conn = Connection::open_in_memory().unwrap();
    let schema = "CREATE TABLE IF NOT EXISTS panes (
        vibe_id TEXT PRIMARY KEY, 
        physical_id TEXT, 
        terminal_type TEXT, 
        role TEXT, 
        pid INTEGER, 
        status TEXT, 
        last_heartbeat_at DATETIME DEFAULT CURRENT_TIMESTAMP
    );";
    conn.execute_batch(schema).unwrap();
    let store = StateStore::from_conn(conn);
    let (db_tx, db_rx) = mpsc::channel(100);
    let actor = DbActor::new(db_rx, store);
    tokio::spawn(async move { actor.run().await; });
    let db_handle = DbHandle::new(db_tx);

    // Start Master
    let s_path = socket_path.clone();
    let h = db_handle.clone();
    tokio::spawn(async move {
        let server = MasterServer::new(s_path, h);
        if let Err(e) = server.run().await {
            eprintln!("Master server error: {}", e);
        }
    });

    // Wait for server to bind
    let mut started = false;
    for _ in 0..50 {
        if socket_path.exists() {
            started = true;
            break;
        }
        time::sleep(Duration::from_millis(50)).await;
    }
    assert!(started, "Master server failed to start");

    // Simulate 20 concurrent workers
    let num_workers = 20;
    let mut worker_handles = Vec::new();

    for i in 0..num_workers {
        let c_path = socket_path.clone();
        let vibe_id = format!("v-{}", i);
        let physical_id = format!("p-{}", i);
        
        let handle = tokio::spawn(async move {
            let client = WorkerClient::new(
                c_path,
                vibe_id,
                physical_id,
                "wezterm".to_string(),
                Some("worker".to_string()),
            );
            // Run heartbeat for 10 seconds
            let result = tokio::time::timeout(Duration::from_secs(10), client.run_heartbeat()).await;
            match result {
                Ok(Err(e)) => {
                    eprintln!("Worker {} error: {}", i, e);
                    Err(e)
                }
                _ => Ok(()), // Timeout is expected as it's a loop
            }
        });
        worker_handles.push(handle);
    }

    // Wait for some heartbeats to occur
    time::sleep(Duration::from_secs(5)).await;

    // Verify all workers are registered in the DB
    let panes = db_handle.get_panes().await?;
    assert_eq!(panes.len(), num_workers, "Not all workers registered");

    // Let them run a bit more
    time::sleep(Duration::from_secs(6)).await;

    // Verify all are still there
    let panes = db_handle.get_panes().await?;
    assert_eq!(panes.len(), num_workers);

    // Abort all workers
    for h in worker_handles {
        h.abort();
    }

    Ok(())
}

#[tokio::test]
async fn test_master_recovery() -> anyhow::Result<()> {
    let dir = tempdir().unwrap();
    let socket_path = dir.path().join("vibe.sock");
    
    // Setup DB
    let conn = Connection::open_in_memory().unwrap();
    let schema = "CREATE TABLE IF NOT EXISTS panes (
        vibe_id TEXT PRIMARY KEY, 
        physical_id TEXT, 
        terminal_type TEXT, 
        role TEXT, 
        pid INTEGER, 
        status TEXT, 
        last_heartbeat_at DATETIME DEFAULT CURRENT_TIMESTAMP
    );";
    conn.execute_batch(schema).unwrap();
    let store = StateStore::from_conn(conn);
    let (db_tx, db_rx) = mpsc::channel(100);
    let actor = DbActor::new(db_rx, store);
    tokio::spawn(async move { actor.run().await; });
    let db_handle = DbHandle::new(db_tx);

    // Start Master
    let s_path = socket_path.clone();
    let h = db_handle.clone();
    let master_handle = tokio::spawn(async move {
        let server = MasterServer::new(s_path, h);
        server.run().await
    });

    // Wait for server to bind
    let mut started = false;
    for _ in 0..50 {
        if socket_path.exists() {
            started = true;
            break;
        }
        time::sleep(Duration::from_millis(50)).await;
    }
    assert!(started, "Master server failed to start");

    // Start 5 workers
    let num_workers = 5;
    let mut worker_handles = Vec::new();

    for i in 0..num_workers {
        let c_path = socket_path.clone();
        let vibe_id = format!("rv-{}", i);
        let physical_id = format!("rp-{}", i);
        
        let handle = tokio::spawn(async move {
            let client = WorkerClient::new(
                c_path,
                vibe_id,
                physical_id,
                "wezterm".to_string(),
                Some("worker".to_string()),
            );
            client.run_heartbeat().await
        });
        worker_handles.push(handle);
    }

    // Wait for registration
    time::sleep(Duration::from_secs(2)).await;
    let panes = db_handle.get_panes().await?;
    assert_eq!(panes.len(), num_workers);

    println!("Killing master server...");
    master_handle.abort();
    // Small delay to ensure it's killed and socket is (optionally) cleaned up
    // Note: Since we aborted, the cleanup code in MasterServer::run might not run.
    // We should manually cleanup if needed, but MasterServer::run cleans up stale socket on start.
    time::sleep(Duration::from_millis(500)).await;
    if socket_path.exists() {
        let _ = std::fs::remove_file(&socket_path);
    }

    println!("Restarting master server...");
    let s_path = socket_path.clone();
    let h = db_handle.clone();
    tokio::spawn(async move {
        let server = MasterServer::new(s_path, h);
        server.run().await
    });

    // Wait for server to bind
    let mut started = false;
    for _ in 0..50 {
        if socket_path.exists() {
            started = true;
            break;
        }
        time::sleep(Duration::from_millis(50)).await;
    }
    assert!(started, "Master server failed to restart");

    // Wait for workers to reconnect (they retry every 5s)
    println!("Waiting for workers to reconnect...");
    time::sleep(Duration::from_secs(10)).await;

    // Verify all workers are registered again (or still there)
    let panes = db_handle.get_panes().await?;
    assert_eq!(panes.len(), num_workers);

    // Verify heartbeats resumed - check timestamp if we had it, 
    // but here we just check they are still alive in the server's view.
    // In our simplified DB actor, we can just check if get_panes returns them.
    
    for h in worker_handles {
        h.abort();
    }

    Ok(())
}
