mod adapter;
mod error;
mod env;
mod os;

use clap::Parser;
use crate::env::{detect_current_terminal, resolve_config_dir, resolve_state_dir};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    // Environment detection (Fail Fast)
    let terminal_type = match detect_current_terminal() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let config_dir = resolve_config_dir()?;
    let state_dir = resolve_state_dir()?;

    println!("Vibe CLI initialized");
    println!("Detected Terminal: {:?}", terminal_type);
    println!("Config Directory: {:?}", config_dir);
    println!("State Directory: {:?}", state_dir);

    if let Some(name) = args.name {
        println!("Hello, {}!", name);
    }

    Ok(())
}
