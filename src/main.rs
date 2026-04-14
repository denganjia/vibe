mod adapter;
mod error;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    println!("Vibe CLI initialized");
    if let Some(name) = args.name {
        println!("Hello, {}!", name);
    }

    Ok(())
}
