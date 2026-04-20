use clap::{Parser, Subcommand};
use vibe_core::adapter::{SplitDirection, WindowTarget, TerminalAdapter, WezTermAdapter, TmuxAdapter};
use vibe_core::env::{detect_current_terminal, TerminalType};
use vibe_core::state::{StateStore, ensure_project_vibe};
use tokio::process::Command;
use std::time::Duration;

mod tui;

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
    /// Check environment compatibility
    Check {
        /// Output in JSON format
        #[arg(short, long, default_value_t = true)]
        json: bool,
    },
    /// Kill all active vibe panes
    Kill,
    /// Send a signal to the bus
    Signal {
        /// Signal name
        name: String,
        /// Payload (JSON string or @path)
        payload: Option<String>,
    },
    /// Wait for a signal on the bus
    Wait {
        /// Signal name to wait for
        name: String,
        /// Timeout in seconds (default: 300)
        #[arg(short, long, default_value_t = 300)]
        timeout: u64,
    },
    /// Run a command and track it
    Run {
        /// Command to run.
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
    },
    /// Focus a specific vibe pane
    Focus {
        /// Target vibe ID
        vibe_id: String,
    },
    /// Spawn a new agent role in a new tab or pane
    Spawn {
        /// Role name (e.g., Worker, Conductor)
        role: String,
        
        /// Override agent command
        #[arg(long)]
        cmd: Option<String>,

        /// Spawn in a new pane instead of a new tab
        #[arg(long)]
        pane: bool,
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
            ensure_project_vibe()?;
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

            let master_pane_id = match terminal_type {
                Some(TerminalType::WezTerm) => WezTermAdapter.get_metadata()?.pane_id,
                Some(TerminalType::Tmux) => TmuxAdapter.get_metadata()?.pane_id,
                None => "0".to_string(),
            };
            
            let mut env_vars = std::collections::HashMap::new();
            env_vars.insert("VIBE_MASTER_ID".to_string(), master_pane_id);
            
            let vibe_id = adapter.spawn(WindowTarget::Pane(split_dir), None, env_vars)?;
            let cwd = std::env::current_dir().ok().map(|p| p.to_string_lossy().to_string());
            store.save_pane(&vibe_id, &vibe_id, &format!("{:?}", terminal_type.unwrap_or(TerminalType::WezTerm)), None, cwd)?;
            println!("Split new pane: {}", vibe_id);
        }
        Commands::Spawn { role, cmd, pane } => {
            ensure_project_vibe()?;
            let terminal_type = detect_current_terminal();
            let adapter: Box<dyn TerminalAdapter> = match terminal_type {
                Some(TerminalType::WezTerm) => Box::new(WezTermAdapter),
                Some(TerminalType::Tmux) => Box::new(TmuxAdapter),
                None => {
                    println!("No supported terminal detected locally. Attempting external orchestration via WezTerm...");
                    Box::new(WezTermAdapter)
                }
            };
            
            // 1. Load Persona
            let role_manager = vibe_core::state::RoleManager::new()?;
            let persona = role_manager.get_persona(&role)?;
            
            // 2. Determine agent command
            let config_manager = vibe_core::state::ConfigManager::new()?;
            let config = config_manager.load()?;
            
            let mut agent_command = cmd.unwrap_or_else(|| {
                config.roles.get(&role)
                    .cloned()
                    .unwrap_or(config.default_command.clone())
            });

            // Ensure auto-approve mode for common CLIs to prevent hanging
            if (agent_command.starts_with("claude") || agent_command.starts_with("gemini") || agent_command.starts_with("codex")) 
                && !agent_command.contains(" -y") && !agent_command.contains(" --yolo") {
                agent_command.push_str(" -y");
            }
            
            // 3. Get master pane ID
            let master_pane_id = match terminal_type {
                Some(TerminalType::WezTerm) => WezTermAdapter.get_metadata()?.pane_id,
                Some(TerminalType::Tmux) => TmuxAdapter.get_metadata()?.pane_id,
                None => "0".to_string(),
            };
            
            // 4. Spawn and inject
            let mut env_vars = std::collections::HashMap::new();
            env_vars.insert("VIBE_MASTER_ID".to_string(), master_pane_id);
            
            let target = if pane {
                WindowTarget::Pane(SplitDirection::Horizontal)
            } else {
                WindowTarget::Tab
            };

            let vibe_id = adapter.spawn(target, Some(&agent_command), env_vars)?;

            // 5. Register in state BEFORE injection to prevent race conditions
            let store = StateStore::new()?;
            let cwd = std::env::current_dir().ok().map(|p| p.to_string_lossy().to_string());
            store.save_pane(&vibe_id, &vibe_id, &format!("{:?}", terminal_type.unwrap_or(TerminalType::WezTerm)), Some(role.clone()), cwd)?;

            // Give the new context a moment to initialize its TTY and start the agent
            std::thread::sleep(std::time::Duration::from_secs(2));

            // Inject persona directly into the agent
            adapter.inject_text(&vibe_id, &persona)?;
            adapter.inject_text(&vibe_id, "\n\n")?;
            
            // Explicitly send Enter to trigger agent processing
            adapter.send_keys(&vibe_id, "")?;
            
            let target_type = if pane { "pane" } else { "tab" };
            println!("Spawned {} in {}: {}", role, target_type, vibe_id);
        }
        Commands::List { json } => {
            let store = StateStore::new()?;
            let panes = store.list_active_panes()?;
            if json {
                println!("{}", serde_json::to_string_pretty(&panes)?);
            } else {
                if panes.is_empty() {
                    println!("No active vibe panes.");
                } else {
                    println!("Active Vibe Panes:");
                    for state in panes {
                        let role_str = state.role.map(|r| format!(", Role: {}", r)).unwrap_or_default();
                        let cwd_str = state.cwd.map(|c| format!(", CWD: {}", c)).unwrap_or_default();
                        let summary_str = if !state.summary.is_empty() {
                            format!("\n    Summary: {}", state.summary)
                        } else {
                            "".to_string()
                        };
                        println!("- {}: (Physical ID: {}, Status: {}{}{}){}", 
                            state.vibe_id, state.physical_id, state.status, role_str, cwd_str, summary_str);
                    }
                }
            }
        }
        Commands::Status => {
            tui::run_status_tui().await?;
        }
        Commands::Check { json } => {
            let term_opt = detect_current_terminal();
            let term_name = vibe_core::env::get_terminal_name();
            
            let store = StateStore::new()?;
            let active_workers = store.list_active_panes()?.len();

            if json {
                let res = serde_json::json!({
                    "supported": term_opt.is_some(),
                    "terminal": term_name,
                    "can_orchestrate": term_opt.is_some(),
                    "active_workers": active_workers,
                    "recommendation": if term_opt.is_some() {
                        "Environment supported. You can use split and focus commands."
                    } else {
                        "Current terminal does not support local orchestration."
                    }
                });
                println!("{}", serde_json::to_string_pretty(&res)?);
            } else {
                println!("Terminal: {}", term_name);
                println!("Supported: {}", if term_opt.is_some() { "Yes" } else { "No" });
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
            for state in panes {
                println!("Killing pane: {}", state.vibe_id);
                if let Err(e) = adapter.close(&state.vibe_id) {
                    eprintln!("Failed to close pane {}: {}", state.vibe_id, e);
                }
                store.remove_pane(&state.vibe_id)?;
            }
            println!("All active vibe panes killed.");
        }
        Commands::Signal { name, payload } => {
            let terminal_type = detect_current_terminal()
                .ok_or_else(|| anyhow::anyhow!("No supported terminal detected for signaling"))?;
            let adapter: Box<dyn TerminalAdapter> = match terminal_type {
                TerminalType::WezTerm => Box::new(WezTermAdapter),
                TerminalType::Tmux => Box::new(TmuxAdapter),
            };

            let target_pane = std::env::var("VIBE_MASTER_ID").or_else(|_| {
                let store = StateStore::new()?;
                store.get_master_pane().map(|p| p.unwrap_or_default())
            })?;

            if target_pane.is_empty() {
                anyhow::bail!("Could not identify Master Pane for signaling.");
            }

            let payload_json = if let Some(p) = payload {
                if p.starts_with('@') {
                    std::fs::read_to_string(&p[1..])?
                } else {
                    p
                }
            } else {
                "null".to_string()
            };

            let signal_str = format!("\n[vibe-signal:{}] {}\n", name, payload_json);
            adapter.inject_text(&target_pane, &signal_str)?;
            println!("Signal '{}' injected into pane {}.", name, target_pane);
        }
        Commands::Wait { name, timeout } => {
            use std::io::{BufRead, BufReader};
            let marker = format!("[vibe-signal:{}]", name);
            println!("Waiting for signal '{}' (timeout: {}s)...", name, timeout);

            let (tx, mut rx) = tokio::sync::mpsc::channel(1);
            
            // Move stdin reading to a dedicated thread because it's blocking
            std::thread::spawn(move || {
                let stdin = std::io::stdin();
                let reader = BufReader::new(stdin);
                for line in reader.lines() {
                    if let Ok(l) = line {
                        if l.contains(&marker) {
                            let payload = l.split(&marker).nth(1).unwrap_or("").trim().to_string();
                            let _ = tx.blocking_send(payload);
                            break;
                        }
                    }
                }
            });

            tokio::select! {
                res = rx.recv() => {
                    if let Some(payload) = res {
                        println!("{}", payload);
                    }
                }
                _ = tokio::time::sleep(Duration::from_secs(timeout)) => {
                    anyhow::bail!("Wait timed out after {} seconds.", timeout);
                }
            }
        }
        Commands::Run { command } => {
            if command.is_empty() {
                anyhow::bail!("No command provided for 'vibe run'.");
            }

            // 1. Identify current environment for Master ID
            let master_pane_id = match detect_current_terminal() {
                Some(TerminalType::WezTerm) => WezTermAdapter.get_metadata()?.pane_id,
                Some(TerminalType::Tmux) => TmuxAdapter.get_metadata()?.pane_id,
                None => "0".to_string(), // Fallback
            };

            // 2. Execute the command with VIBE_MASTER_ID injected
            let mut child = Command::new(&command[0])
                .args(&command[1..])
                .env("VIBE_MASTER_ID", &master_pane_id)
                .spawn()?;

            let status = child.wait().await?;
            if !status.success() {
                std::process::exit(status.code().unwrap_or(1));
            }
        }
        Commands::Inject { vibe_id, command, cwd } => {
            let terminal_type = detect_current_terminal()
                .ok_or_else(|| anyhow::anyhow!("No supported terminal detected for injection"))?;
            let adapter: Box<dyn TerminalAdapter> = match terminal_type {
                TerminalType::WezTerm => Box::new(WezTermAdapter),
                TerminalType::Tmux => Box::new(TmuxAdapter),
            };

            let cmd_str = if let Some(dir) = cwd {
                format!("cd {} && {}", dir, command)
            } else {
                command
            };

            adapter.send_keys(&vibe_id, &cmd_str)?;
            println!("Command injected into pane {}.", vibe_id);
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
            let store = StateStore::new()?;
            
            // Identify current environment to get our physical_id
            let physical_id = match detect_current_terminal() {
                Some(TerminalType::WezTerm) => WezTermAdapter.get_metadata()?.pane_id,
                Some(TerminalType::Tmux) => TmuxAdapter.get_metadata()?.pane_id,
                None => std::process::id().to_string(),
            };

            let vibe_id = store.get_vibe_id_by_physical_id(&physical_id)?
                .ok_or_else(|| anyhow::anyhow!("Could not identify Vibe ID for current pane"))?;

            store.update_report(vibe_id.clone(), status, message)?;
            println!("Report submitted for worker {}.", vibe_id);
        }
    }

    Ok(())
}
