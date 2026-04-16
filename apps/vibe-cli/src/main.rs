use clap::{Parser, Subcommand};
use vibe_core::adapter::{SplitDirection, TerminalAdapter, WezTermAdapter, TmuxAdapter};
use vibe_core::env::{detect_current_terminal, TerminalType, resolve_socket_path};
use vibe_core::state::StateStore;
use vibe_core::ipc::server::MasterServer;
use vibe_core::ipc::client::WorkerClient;
use vibe_core::state::db::{DbActor, DbHandle};
use vibe_core::os::spawn_daemon;
use tokio::sync::mpsc;
use std::process::Stdio;
use tokio::process::Command;
use futures::{StreamExt, SinkExt};
use tokio_util::codec::LinesCodec;
use vibe_core::ipc::protocol::{Message, ExecuteIntentInfo};
use std::collections::HashMap;
use std::time::Duration;

mod tui;
mod mcp;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Split the current pane
    Split {
        /// Split horizontally (default)
        #[arg(short, long, default_value_t = true, conflicts_with = "vertical")]
        horizontal: bool,
        
        /// Split vertically
        #[arg(short, long, conflicts_with = "horizontal")]
        vertical: bool,
    },
    /// List active vibe panes
    List {
        /// Output in JSON format
        #[arg(short, long)]
        json: bool,
    },
    /// Interactive dashboard
    Status,
    /// Run as an MCP server for AI clients
    Mcp,
    /// Check environment compatibility for AI
    Check {
        /// Output in JSON format
        #[arg(short, long, default_value_t = true)]
        json: bool,
    },
    /// Kill all active vibe panes
    Kill,
    /// Start the master server
    Master {
        /// Run in background
        #[arg(short, long)]
        daemon: bool,
    },
    /// Run a command and track it, or start a worker listener
    Run {
        /// Trust this worker (skip confirmation gates)
        #[arg(short, long)]
        yes: bool,

        /// Command to run. If empty, starts a listener.
        #[arg(trailing_var_arg = true)]
        command: Vec<String>,
    },
    /// Inject a command into a running worker
    Inject {
        /// Target vibe ID
        vibe_id: String,
        
        /// Command to execute
        command: String,

        /// Target working directory
        #[arg(long)]
        cwd: Option<String>,

        /// Skip confirmation gate
        #[arg(short, long)]
        yes: bool,
    },
    /// Focus a specific vibe pane
    Focus {
        /// Target vibe ID
        vibe_id: String,
    },
    /// Report task summary back to master
    Report {
        /// Status (success, failed, running)
        #[arg(short, long)]
        status: String,

        /// Summary message
        #[arg(short, long)]
        message: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Split { horizontal: _, vertical } => {
            let terminal_type = detect_current_terminal();
            let adapter: Box<dyn TerminalAdapter> = match terminal_type {
                Some(TerminalType::WezTerm) => Box::new(WezTermAdapter),
                Some(TerminalType::Tmux) => Box::new(TmuxAdapter),
                None => {
                    println!("No supported terminal detected locally. Attempting external orchestration via WezTerm...");
                    Box::new(WezTermAdapter)
                }
            };
            let store = StateStore::new()?;

            let split_dir = if vertical {
                SplitDirection::Vertical
            } else {
                SplitDirection::Horizontal
            };
            
            let vibe_id = adapter.split(split_dir, None)?;
            // If we are external, vibe_id might be a new pane in a different window
            let cwd = std::env::current_dir().ok().map(|p| p.to_string_lossy().to_string());
            store.save_pane(&vibe_id, &vibe_id, &format!("{:?}", terminal_type.unwrap_or(TerminalType::WezTerm)), cwd)?;
            println!("Split new pane: {}", vibe_id);
        }
        Commands::List { json } => {
            let store = StateStore::new()?;
            let panes = store.list_active_panes()?;
            if json {
                let json_panes: Vec<_> = panes.into_iter().map(|(v_id, p_id, t_type, role, status, summary, cwd)| {
                    serde_json::json!({
                        "vibe_id": v_id,
                        "physical_id": p_id,
                        "terminal": t_type,
                        "role": role,
                        "status": status,
                        "summary": summary,
                        "cwd": cwd
                    })
                }).collect();
                println!("{}", serde_json::to_string_pretty(&json_panes)?);
            } else {
                if panes.is_empty() {
                    println!("No active vibe panes.");
                } else {
                    println!("Active Vibe Panes:");
                    for (v_id, p_id, t_type, role, status, summary, cwd) in panes {
                        let role_str = role.map(|r| format!(", Role: {}", r)).unwrap_or_default();
                        let status_str = status.map(|s| format!(", Status: {}", s)).unwrap_or_default();
                        let summary_str = summary.map(|s| format!("\n    Summary: {}", s)).unwrap_or_default();
                        let cwd_str = cwd.map(|c| format!(", CWD: {}", c)).unwrap_or_default();
                        println!("- {}: (Physical ID: {}, Terminal: {}{}{}{}){}", v_id, p_id, t_type, role_str, status_str, cwd_str, summary_str);
                    }
                }
            }
        }
        Commands::Status => {
            tui::run_status_tui().await?;
        }
        Commands::Mcp => {
            mcp::run_mcp_server().await?;
        }
        Commands::Check { json } => {
            let term_opt = detect_current_terminal();
            let term_name = vibe_core::env::get_terminal_name();
            let socket_path = resolve_socket_path()?;
            let master_alive = tokio::net::UnixStream::connect(&socket_path).await.is_ok();
            
            let store = StateStore::new()?;
            let active_workers = store.list_active_panes()?.len();

            if json {
                let res = serde_json::json!({
                    "supported": term_opt.is_some(),
                    "terminal": term_name,
                    "can_orchestrate": term_opt.is_some(),
                    "master_status": if master_alive { "running" } else { "stopped" },
                    "active_workers": active_workers,
                    "recommendation": if term_opt.is_some() {
                        "Environment supported. You can use split and focus commands."
                    } else {
                        "Current terminal does not support local orchestration. Use 'vibe run' to spawn tasks in an external WezTerm window."
                    }
                });
                println!("{}", serde_json::to_string_pretty(&res)?);
            } else {
                println!("Terminal: {}", term_name);
                println!("Supported: {}", if term_opt.is_some() { "Yes" } else { "No" });
                println!("Master Status: {}", if master_alive { "Running" } else { "Stopped" });
                println!("Active Workers: {}", active_workers);
            }
        }
        Commands::Kill => {
            let terminal_type = detect_current_terminal()
                .ok_or_else(|| anyhow::anyhow!("No supported terminal detected"))?;
            let adapter: Box<dyn TerminalAdapter> = match terminal_type {
                TerminalType::WezTerm => Box::new(WezTermAdapter),
                TerminalType::Tmux => Box::new(TmuxAdapter),
            };
            let store = StateStore::new()?;
            let panes = store.list_active_panes()?;
            for (v_id, _p_id, _t_type, _role, _status, _summary, _cwd) in panes {
                println!("Killing pane: {}", v_id);
                if let Err(e) = adapter.close(&v_id) {
                    eprintln!("Failed to close pane {}: {}", v_id, e);
                }
                store.remove_pane(&v_id)?;
            }
            println!("All active vibe panes killed.");
        }
        Commands::Master { daemon } => {
            if daemon {
                spawn_daemon()?;
            }
            
            let socket_path = resolve_socket_path()?;
            let store = StateStore::new()?;
            let (db_tx, db_rx) = mpsc::channel(100);
            let actor = DbActor::new(db_rx, store);
            tokio::spawn(async move {
                actor.run().await;
            });
            let db_handle = DbHandle::new(db_tx);
            let server = MasterServer::new(socket_path, db_handle);
            server.run().await?;
        }
        Commands::Run { yes, command } => {
            // 1. Ensure master is running
            let socket_path = resolve_socket_path()?;
            
            // Check if master is alive by attempting a connection
            let is_alive = if socket_path.exists() {
                match tokio::net::UnixStream::connect(&socket_path).await {
                    Ok(_) => true,
                    Err(_) => {
                        // Stale socket, remove it
                        let _ = std::fs::remove_file(&socket_path);
                        false
                    }
                }
            } else {
                false
            };

            if !is_alive {
                let exe = std::env::current_exe()?;
                std::process::Command::new(exe)
                    .arg("master")
                    .arg("--daemon")
                    .spawn()?;
                
                // Wait for master to start
                let mut started = false;
                for _ in 0..20 {
                    if socket_path.exists() {
                        started = true;
                        break;
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }
                if !started {
                    anyhow::bail!("Master server failed to start.");
                }
            }

            // 2. Identify current environment
            let (default_vibe_id, physical_id, term_type_str) = match detect_current_terminal() {
                Some(TerminalType::WezTerm) => {
                    let meta = WezTermAdapter.get_metadata()?;
                    (meta.pane_id.clone(), meta.pane_id, "wezterm".to_string())
                }
                Some(TerminalType::Tmux) => {
                    let meta = TmuxAdapter.get_metadata()?;
                    (meta.pane_id.clone(), meta.pane_id, "tmux".to_string())
                }
                None => {
                    // Fallback for non-supported terminals (like VSCode)
                    // We generate a synthetic ID and use the PID as physical_id
                    let pid = std::process::id();
                    (format!("ext-{}", pid), pid.to_string(), vibe_core::env::get_terminal_name())
                }
            };

            // Lookup vibe_id if possible
            let vibe_id = {
                let store = StateStore::new()?;
                store.get_vibe_id_by_physical_id(&physical_id)?.unwrap_or(default_vibe_id)
            };

            // 3. Setup worker client
            let mut worker = WorkerClient::new(
                socket_path,
                vibe_id.clone(),
                physical_id,
                term_type_str,
                Some("worker".to_string()),
            );
            worker.set_trusted(yes);

            // 4. Run the worker
            if command.is_empty() {
                println!("Vibe Worker listening for intents (Ctrl+C to stop).");
                worker.run_worker().await?;
            } else {
                // Spawn worker task in background to handle heartbeats and intents
                let w_clone = worker.clone();
                tokio::spawn(async move {
                    if let Err(e) = w_clone.run_worker().await {
                        eprintln!("Worker background task error: {}", e);
                    }
                });

                // Execute the initial command
                let logs_dir = vibe_core::env::resolve_logs_dir()?;
                let log_path = logs_dir.join(format!("{}.log", vibe_id));
                tokio::fs::create_dir_all(&logs_dir).await?;
                
                let log_file = tokio::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(log_path)
                    .await?;
                let log_file = std::sync::Arc::new(tokio::sync::Mutex::new(log_file));

                let mut child = Command::new(&command[0])
                    .args(&command[1..])
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()?;

                let stdout = child.stdout.take().unwrap();
                let stderr = child.stderr.take().unwrap();

                let log_f1 = log_file.clone();
                let t1 = tokio::spawn(async move {
                    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
                    let mut reader = BufReader::new(stdout).lines();
                    while let Ok(Some(line)) = reader.next_line().await {
                        println!("{}", line);
                        let stripped = vibe_core::os::shell::strip_ansi(&line);
                        let mut f = log_f1.lock().await;
                        let _ = f.write_all(stripped.as_bytes()).await;
                        let _ = f.write_all(b"\n").await;
                    }
                });

                let log_f2 = log_file.clone();
                let t2 = tokio::spawn(async move {
                    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
                    let mut reader = BufReader::new(stderr).lines();
                    while let Ok(Some(line)) = reader.next_line().await {
                        eprintln!("{}", line);
                        let stripped = vibe_core::os::shell::strip_ansi(&line);
                        let mut f = log_f2.lock().await;
                        let _ = f.write_all(stripped.as_bytes()).await;
                        let _ = f.write_all(b"\n").await;
                    }
                });

                let status = child.wait().await?;
                let _ = t1.await;
                let _ = t2.await;
                let code = status.code().unwrap_or(1);

                if let Err(e) = worker.send_exit_status(code).await {
                    eprintln!("Failed to send exit status: {}", e);
                }
                
                println!("Initial command finished. Worker staying alive for further intents (Ctrl+C to stop).");
                loop { tokio::time::sleep(Duration::from_secs(3600)).await; }
            }
        }
        Commands::Inject { vibe_id, command, cwd, yes } => {
            let socket_path = resolve_socket_path()?;
            let stream = tokio::net::UnixStream::connect(&socket_path).await?;
            let mut framed = tokio_util::codec::Framed::new(stream, LinesCodec::new());

            let intent = Message::ExecuteIntent(ExecuteIntentInfo {
                target_vibe_id: vibe_id.clone(),
                cmd: command,
                cwd,
                env: HashMap::new(),
                trusted: yes,
            });

            framed.send(serde_json::to_string(&intent)?).await?;
            
            // Wait for Ack
            if let Some(Ok(line)) = framed.next().await {
                let msg = Message::from_str(&line)?;
                if msg == Message::Ack {
                    println!("Command successfully injected into worker {}.", vibe_id);
                } else {
                    anyhow::bail!("Unexpected response from master: {:?}", msg);
                }
            } else {
                anyhow::bail!("Master disconnected unexpectedly.");
            }
        }
        Commands::Focus { vibe_id } => {
            let store = StateStore::new()?;
            let physical_id = store.get_pane(&vibe_id)?
                .ok_or_else(|| anyhow::anyhow!("Vibe ID {} not found in database", vibe_id))?;
            
            let terminal_type = detect_current_terminal()
                .ok_or_else(|| anyhow::anyhow!("No supported terminal detected for focus operation"))?;
            let adapter: Box<dyn TerminalAdapter> = match terminal_type {
                TerminalType::WezTerm => Box::new(WezTermAdapter),
                TerminalType::Tmux => Box::new(TmuxAdapter),
            };
            
            adapter.focus(&physical_id)?;
            println!("Focused physical pane: {}", physical_id);
        }
        Commands::Report { status, message } => {
            let socket_path = resolve_socket_path()?;
            
            // Identify current environment to get our vibe_id
            let (physical_id, term_type_str) = match detect_current_terminal() {
                Some(TerminalType::WezTerm) => (WezTermAdapter.get_metadata()?.pane_id, "wezterm".to_string()),
                Some(TerminalType::Tmux) => (TmuxAdapter.get_metadata()?.pane_id, "tmux".to_string()),
                None => (std::process::id().to_string(), vibe_core::env::get_terminal_name()),
            };

            let vibe_id = {
                let store = StateStore::new()?;
                store.get_vibe_id_by_physical_id(&physical_id)?
                    .ok_or_else(|| anyhow::anyhow!("Could not identify Vibe ID for current pane"))?
            };

            let worker = WorkerClient::new(
                socket_path,
                vibe_id.clone(),
                physical_id,
                term_type_str,
                None,
            );

            worker.send_report(status, message).await?;
            println!("Report submitted for worker {}.", vibe_id);
        }
    }

    Ok(())
}
