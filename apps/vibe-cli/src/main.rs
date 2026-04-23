use clap::Parser;
use tokio::process::Command;
use std::path::{Path, PathBuf};
use vibe_core::env::{detect_current_terminal, TerminalType};
use vibe_core::adapter::{WezTermAdapter, TmuxAdapter, TerminalAdapter};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    #[command(external_subcommand)]
    External(Vec<String>),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    let (cmd_name, args) = match cli.command {
        Some(Commands::External(mut v)) => {
            if v.is_empty() {
                print_help();
                return Ok(());
            }
            let name = v.remove(0);
            (name, v)
        }
        None => {
            print_help();
            return Ok(());
        }
    };

    // 1. Detect environment and set env vars
    let terminal_type = detect_current_terminal();
    let master_pane_id = match terminal_type {
        Some(TerminalType::WezTerm) => WezTermAdapter.get_metadata()?.pane_id,
        Some(TerminalType::Tmux) => TmuxAdapter.get_metadata()?.pane_id,
        None => "0".to_string(),
    };
    
    // 2. Find script
    let script_path = find_script(&cmd_name)?;

    // 3. Execute Node.js
    let mut child = Command::new("node")
        .arg(script_path)
        .args(args)
        .env("VIBE_MASTER_ID", &master_pane_id)
        .spawn()?;

    let status = child.wait().await?;
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    Ok(())
}

fn print_help() {
    println!("Vibe CLI - Lightweight Dispatcher");
    println!("Usage: vibe <command> [args]...");
    println!("\nThis dispatcher forwards commands to scripts in .vibe/scripts/ or plugin/vibe/scripts/.");
}

fn find_script(cmd: &str) -> anyhow::Result<PathBuf> {
    // 1. Check .vibe/scripts/<cmd>.js (Local project override)
    let local_path = Path::new(".vibe/scripts").join(format!("{}.js", cmd));
    if local_path.exists() {
        return Ok(local_path);
    }

    // 2. Check plugin/vibe/scripts/<cmd>.js (Relative to current working directory)
    let plugin_path = Path::new("plugin/vibe/scripts").join(format!("{}.js", cmd));
    if plugin_path.exists() {
        return Ok(plugin_path);
    }

    // 3. Check relative to executable path (For installed versions)
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            // Check adjacent plugin/vibe/scripts
            let adj_plugin = exe_dir.join("plugin/vibe/scripts").join(format!("{}.js", cmd));
            if adj_plugin.exists() {
                return Ok(adj_plugin);
            }

            // Check ../../plugin/vibe/scripts (common for target/debug/vibe)
            if let Some(p1) = exe_dir.parent() {
                if let Some(p2) = p1.parent() {
                    let p_plugin = p2.join("plugin/vibe/scripts").join(format!("{}.js", cmd));
                    if p_plugin.exists() {
                        return Ok(p_plugin);
                    }
                }
            }
        }
    }

    anyhow::bail!("Command '{}' not found. Ensure .vibe/scripts/{}.js or plugin/vibe/scripts/{}.js exists.", cmd, cmd, cmd)
}
