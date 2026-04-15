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
    List,
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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Split { horizontal: _, vertical } => {
            let terminal_type = detect_current_terminal()?;
            let adapter: Box<dyn TerminalAdapter> = match terminal_type {
                TerminalType::WezTerm => Box::new(WezTermAdapter),
                TerminalType::Tmux => Box::new(TmuxAdapter),
            };
            let store = StateStore::new()?;

            let split_dir = if vertical {
                SplitDirection::Vertical
            } else {
                SplitDirection::Horizontal
            };
            
            let vibe_id = adapter.split(split_dir, None)?;
            store.save_pane(&vibe_id, &vibe_id, &format!("{:?}", terminal_type))?;
            println!("Split new pane: {}", vibe_id);
        }
        Commands::List => {
            let store = StateStore::new()?;
            let panes = store.list_active_panes()?;
            if panes.is_empty() {
                println!("No active vibe panes.");
            } else {
                println!("Active Vibe Panes:");
                for (v_id, p_id, t_type, role) in panes {
                    let role_str = role.map(|r| format!(", Role: {}", r)).unwrap_or_default();
                    println!("- {}: (Physical ID: {}, Terminal: {}{})", v_id, p_id, t_type, role_str);
                }
            }
        }
        Commands::Kill => {
            let terminal_type = detect_current_terminal()?;
            let adapter: Box<dyn TerminalAdapter> = match terminal_type {
                TerminalType::WezTerm => Box::new(WezTermAdapter),
                TerminalType::Tmux => Box::new(TmuxAdapter),
            };
            let store = StateStore::new()?;
            let panes = store.list_active_panes()?;
            for (v_id, _p_id, _t_type, _role) in panes {
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
            if !socket_path.exists() {
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
            let terminal_type = detect_current_terminal()?;
            let (default_vibe_id, physical_id, term_type_str) = match terminal_type {
                TerminalType::WezTerm => {
                    let meta = WezTermAdapter.get_metadata()?;
                    (meta.pane_id.clone(), meta.pane_id, "wezterm".to_string())
                }
                TerminalType::Tmux => {
                    let meta = TmuxAdapter.get_metadata()?;
                    (meta.pane_id.clone(), meta.pane_id, "tmux".to_string())
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
                vibe_id,
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
                let mut child = Command::new(&command[0])
                    .args(&command[1..])
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()?;

                let status = child.wait().await?;
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
    }

    Ok(())
}
