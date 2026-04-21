use clap::{Parser, Subcommand};
use vibe_core::adapter::{SplitDirection, WindowTarget, TerminalAdapter, WezTermAdapter, TmuxAdapter};
use vibe_core::env::{detect_current_terminal, TerminalType};
use vibe_core::state::{StateStore, ensure_project_vibe};
use vibe_core::ipc::bus::FileBus;
use tokio::process::Command;

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
        #[arg(long = "role")]
        role_flag: Option<String>,

        /// Positional role name
        role: Option<String>,
        
        /// Override agent command
        #[arg(long)]
        cmd: Option<String>,

        /// Spawn in a new pane instead of a new tab
        #[arg(long)]
        pane: bool,

        /// Spawn a predefined stack of agents
        #[arg(long)]
        stack: Option<String>,
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
    /// Initialize project with interactive wizard
    Init {
        /// Force re-initialization of configuration and roles
        #[arg(short, long)]
        force: bool,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Split { horizontal: _, vertical } => {
            ensure_project_vibe()?;
            let terminal_type = detect_current_terminal();
            let adapter = get_adapter(terminal_type);
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
            
            let cwd_opt = std::env::current_dir().ok().map(|p| p.to_string_lossy().to_string());
            let cwd_ref = cwd_opt.as_deref();
            
            let vibe_id = adapter.spawn(WindowTarget::Pane(split_dir), None, cwd_ref, env_vars)?;
            let cwd = std::env::current_dir().ok().map(|p| p.to_string_lossy().to_string());
            store.save_pane(&vibe_id, &vibe_id, &format!("{:?}", terminal_type.unwrap_or(TerminalType::WezTerm)), None, cwd)?;
            println!("Split new pane: {}", vibe_id);
        }
        Commands::Spawn { role_flag, role, cmd, pane, stack } => {
            ensure_project_vibe()?;
            let terminal_type = detect_current_terminal();
            let adapter = get_adapter(terminal_type);
            let store = StateStore::new()?;

            // 0. Perform cleanup
            perform_silent_cleanup(adapter.as_ref(), &store).await?;

            if let Some(stack_name) = stack {
                let config_manager = vibe_core::state::ConfigManager::new()?;
                let config = config_manager.load()?;
                let roles_to_spawn = config.stacks.get(&stack_name)
                    .ok_or_else(|| anyhow::anyhow!("Stack '{}' not found in config.json", stack_name))?;
                
                println!("🚀 Spawning stack: {}", stack_name);
                for r in roles_to_spawn {
                    spawn_role(r, None, pane, adapter.as_ref(), &store).await?;
                    // Small delay between spawns to ensure stable TTY initialization
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
            } else if let Some(r) = role_flag.or(role) {
                spawn_role(&r, cmd, pane, adapter.as_ref(), &store).await?;
            } else {
                anyhow::bail!("Either --role (or positional role) or --stack must be provided.");
            }
        }
        Commands::List { json } => {
            let terminal_type = detect_current_terminal();
            let adapter = get_adapter(terminal_type);
            let store = StateStore::new()?;
            
            // 0. Perform cleanup
            perform_silent_cleanup(adapter.as_ref(), &store).await?;
            
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
                match adapter.close(&state.physical_id) {
                    Ok(()) => store.remove_pane(&state.vibe_id)?,
                    Err(e) => eprintln!("Failed to close pane {}: {}", state.physical_id, e),
                }
            }
            println!("All active vibe panes killed.");
        }
        Commands::Signal { name, payload } => {
            let payload_value = if let Some(p) = payload {
                if p.starts_with('@') {
                    let content = std::fs::read_to_string(&p[1..])?;
                    serde_json::from_str(&content).unwrap_or(serde_json::Value::String(content))
                } else {
                    serde_json::from_str(&p).unwrap_or(serde_json::Value::String(p))
                }
            } else {
                serde_json::Value::Null
            };

            // 1. Try FileBus first
            match FileBus::send(&name, payload_value.clone()) {
                Ok(_) => {
                    println!("Signal '{}' sent via FileBus.", name);
                }
                Err(e) => {
                    eprintln!("FileBus failed: {}. Falling back to TTY injection...", e);
                    
                    // 2. Fallback to TTY injection
                    let terminal_type = detect_current_terminal()
                        .ok_or_else(|| anyhow::anyhow!("No supported terminal detected for fallback signaling"))?;
                    let adapter: Box<dyn TerminalAdapter> = match terminal_type {
                        TerminalType::WezTerm => Box::new(WezTermAdapter),
                        TerminalType::Tmux => Box::new(TmuxAdapter),
                    };

                    let target_pane = std::env::var("VIBE_MASTER_ID").or_else(|_| {
                        let store = StateStore::new()?;
                        store.get_master_pane().map(|p| p.unwrap_or_default())
                    })?;

                    if target_pane.is_empty() {
                        anyhow::bail!("Could not identify Master Pane for fallback signaling.");
                    }

                    let payload_json = serde_json::to_string(&payload_value)?;
                    let signal_str = format!("\n[vibe-signal:{}] {}\r", name, payload_json);
                    adapter.inject_text(&target_pane, &signal_str)?;
                    println!("Signal '{}' injected into pane {} (fallback).", name, target_pane);
                }
            }
        }
        Commands::Wait { name, timeout } => {
            println!("Waiting for signal '{}' (timeout: {}s)...", name, timeout);

            match FileBus::recv(&name, timeout) {
                Ok(payload) => {
                    if payload.is_null() {
                        println!("Signal '{}' received.", name);
                    } else if let Some(s) = payload.as_str() {
                        println!("{}", s);
                    } else {
                        println!("{}", serde_json::to_string_pretty(&payload)?);
                    }
                }
                Err(e) => {
                    anyhow::bail!("Wait failed: {}", e);
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
            let store = StateStore::new()?;
            let physical_id = store.get_pane(&vibe_id)?
                .ok_or_else(|| anyhow::anyhow!("Vibe ID {} not found in database", vibe_id))?;

            let terminal_type = detect_current_terminal()
                .ok_or_else(|| anyhow::anyhow!("No supported terminal detected for injection"))?;
            let adapter = get_adapter(Some(terminal_type));

            let cmd_str = if let Some(dir) = cwd {
                format!("cd -- {} && {}", shell_quote(&dir), command)
            } else {
                command
            };

            adapter.send_keys(&physical_id, &cmd_str)?;
            println!("Command injected into physical pane {}.", physical_id);
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
            
            // 1. Try to get vibe_id from environment variable first (most reliable)
            let vibe_id = if let Ok(vid) = std::env::var("VIBE_ID") {
                vid
            } else {
                // 2. Fallback to physical ID detection
                let physical_id = match detect_current_terminal() {
                    Some(TerminalType::WezTerm) => WezTermAdapter.get_metadata()?.pane_id,
                    Some(TerminalType::Tmux) => TmuxAdapter.get_metadata()?.pane_id,
                    None => std::process::id().to_string(),
                };

                store.get_vibe_id_by_physical_id(&physical_id)?
                    .ok_or_else(|| anyhow::anyhow!("Could not identify Vibe ID for current pane. Environment VIBE_ID not set and physical ID {} not found.", physical_id))?
            };

            store.update_report(vibe_id.clone(), status, message)?;
            println!("Report submitted for worker {}.", vibe_id);
        }
        Commands::Init { force } => {
            let terminal_type = detect_current_terminal();
            let adapter = get_adapter(terminal_type);
            let store = StateStore::new()?;
            
            // 1. Perform cleanup
            perform_silent_cleanup(adapter.as_ref(), &store).await?;
            
            // 2. Interactive Wizard
            println!("🚀 Welcome to Vibe-CLI Project Initializer!");
            
            let mut config_manager = vibe_core::state::ConfigManager::new()?;
            let mut config = if force {
                vibe_core::state::ProjectConfig::default()
            } else {
                config_manager.load()?
            };

            // Detect available CLIs
            let fallbacks = ["claude", "gemini", "codex"];
            let mut available_clis = Vec::new();
            for cli in fallbacks {
                if which::which(cli).is_ok() {
                    available_clis.push(cli);
                }
            }

            if available_clis.is_empty() {
                println!("⚠️ No common AI CLIs (claude, gemini, codex) found in your PATH.");
                println!("Please ensure they are installed and configured.");
            } else {
                use dialoguer::{Select, theme::ColorfulTheme};
                
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select your default AI CLI")
                    .items(&available_clis)
                    .default(0)
                    .interact()?;
                
                let selected_cli_name = available_clis[selection];
                let auto_flag = if selected_cli_name.starts_with("claude") {
                    "--dangerously-skip-permissions"
                } else {
                    "-y"
                };

                let selected_cli = format!("{} {}", selected_cli_name, auto_flag);
                println!("✅ Selected: {}", selected_cli);
                
                config.default_command = selected_cli.clone();
                for cmd in config.roles.values_mut() {
                    *cmd = selected_cli.clone();
                }
            }

            config_manager.save(&config)?;
            ensure_project_vibe()?;
            
            println!("\n✨ Vibe-CLI project initialized successfully in .vibe/");
        }
    }

    Ok(())
}

fn get_adapter(terminal_type: Option<TerminalType>) -> Box<dyn TerminalAdapter> {
    match terminal_type {
        Some(TerminalType::WezTerm) => Box::new(WezTermAdapter),
        Some(TerminalType::Tmux) => Box::new(TmuxAdapter),
        None => {
            println!("No supported terminal detected locally. Attempting external orchestration via WezTerm...");
            Box::new(WezTermAdapter)
        }
    }
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\"'\"'"))
}

async fn perform_silent_cleanup(adapter: &dyn TerminalAdapter, store: &StateStore) -> anyhow::Result<()> {
    if let Ok(physical_ids) = adapter.list_all_physical_ids() {
        store.cleanup_stale_panes(&physical_ids)?;
    }
    Ok(())
}

async fn spawn_role(role: &str, cmd_override: Option<String>, pane: bool, adapter: &dyn TerminalAdapter, store: &StateStore) -> anyhow::Result<()> {
    // 1. Load Persona
    let role_manager = vibe_core::state::RoleManager::new()?;
    let persona = role_manager.get_persona(role)?;
    
    // 2. Determine agent command
    let config_manager = vibe_core::state::ConfigManager::new()?;
    let config = config_manager.load()?;
    
    let mut agent_command = cmd_override.clone().unwrap_or_else(|| {
        config.roles.get(role)
            .cloned()
            .unwrap_or(config.default_command.clone())
    });

    // Ensure auto-approve mode for common CLIs to prevent hanging, but ONLY if no override was provided
    let is_claude = agent_command.starts_with("claude");
    let is_gemini = agent_command.starts_with("gemini");
    let is_codex = agent_command.starts_with("codex");
    let is_known_cli = is_claude || is_gemini || is_codex;

    if cmd_override.is_none() {
        if is_claude && !agent_command.contains(" --dangerously-skip-permissions") {
            if let Some(first_space) = agent_command.find(' ') {
                agent_command.insert_str(first_space, " --dangerously-skip-permissions");
            } else {
                agent_command.push_str(" --dangerously-skip-permissions");
            }
        } else if (is_gemini || is_codex) && !agent_command.contains(" -y") && !agent_command.contains(" --yolo") {
            if let Some(first_space) = agent_command.find(' ') {
                agent_command.insert_str(first_space, " -y");
            } else {
                agent_command.push_str(" -y");
            }
        }
        
        // Pass the persona as the initial positional prompt argument to the CLI
        if is_claude {
            agent_command = format!("{} --system-prompt \"$VIBE_PERSONA\"", agent_command);
        } else if is_known_cli {
            agent_command = format!("{} \"$VIBE_PERSONA\"", agent_command);
        }
    }
    
    // 3. Get master pane ID
    let terminal_type = detect_current_terminal();
    let master_pane_id = match terminal_type {
        Some(TerminalType::WezTerm) => WezTermAdapter.get_metadata()?.pane_id,
        Some(TerminalType::Tmux) => TmuxAdapter.get_metadata()?.pane_id,
        None => "0".to_string(),
    };
    
    // 4. Spawn and inject
    let vibe_id = format!("v-{}", uuid::Uuid::new_v4().to_string()[..8].to_string());
    
    let mut env_vars = std::collections::HashMap::new();
    env_vars.insert("VIBE_MASTER_ID".to_string(), master_pane_id);
    env_vars.insert("VIBE_ID".to_string(), vibe_id.clone());
    env_vars.insert("VIBE_PERSONA".to_string(), persona.clone());
    
    let target = if pane {
        WindowTarget::Pane(SplitDirection::Horizontal)
    } else {
        WindowTarget::Tab
    };

    let cwd_opt = std::env::current_dir().ok().map(|p| p.to_string_lossy().to_string());
    let cwd_ref = cwd_opt.as_deref();

    let physical_id = adapter.spawn(target, Some(&agent_command), cwd_ref, env_vars)?;

    // 5. Register in state
    let cwd = std::env::current_dir().ok().map(|p| p.to_string_lossy().to_string());
    store.save_pane(&vibe_id, &physical_id, &format!("{:?}", terminal_type.unwrap_or(TerminalType::WezTerm)), Some(role.to_string()), cwd)?;

    // We no longer need to sleep and inject keystrokes for known CLIs because they receive the persona natively!
    if !is_known_cli || cmd_override.is_some() {
        println!("Waiting 3s for custom agent TTY initialization before injecting persona...");
        std::thread::sleep(std::time::Duration::from_secs(3));
        adapter.inject_text(&physical_id, &persona)?;
        adapter.inject_text(&physical_id, "\r\r")?;
        adapter.send_keys(&physical_id, "")?;
    }
    
    let target_type = if pane { "pane" } else { "tab" };
    println!("Spawned {} in {}: {}", role, target_type, vibe_id);

    Ok(())
}
