use clap::{Parser, Subcommand};
use vibe_core::adapter::{SplitDirection, TerminalAdapter, WezTermAdapter, TmuxAdapter};
use vibe_core::env::{detect_current_terminal, TerminalType};
use vibe_core::state::StateStore;

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
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    // Environment detection
    let terminal_type = detect_current_terminal()?;
    
    // Initialize adapter
    let adapter: Box<dyn TerminalAdapter> = match terminal_type {
        TerminalType::WezTerm => Box::new(WezTermAdapter),
        TerminalType::Tmux => Box::new(TmuxAdapter),
    };
    
    // Initialize state store
    let store = StateStore::new()?;

    match cli.command {
        Commands::Split { horizontal, vertical } => {
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
            let panes = store.list_active_panes()?;
            if panes.is_empty() {
                println!("No active vibe panes.");
            } else {
                println!("Active Vibe Panes:");
                for (v_id, p_id, t_type) in panes {
                    println!("- {}: (Physical ID: {}, Terminal: {})", v_id, p_id, t_type);
                }
            }
        }
        Commands::Kill => {
            let panes = store.list_active_panes()?;
            for (v_id, _p_id, _t_type) in panes {
                println!("Killing pane: {}", v_id);
                if let Err(e) = adapter.close(&v_id) {
                    eprintln!("Failed to close pane {}: {}", v_id, e);
                }
                store.remove_pane(&v_id)?;
            }
            println!("All active vibe panes killed.");
        }
    }

    Ok(())
}
