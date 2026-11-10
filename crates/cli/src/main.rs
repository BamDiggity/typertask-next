use clap::{Parser, Subcommand};
use std::fs;
use typertask_engine::{parse_script, Engine, TextExpander};
use typertask_daemon::run_server;

#[derive(Parser)]
#[command(name = "typertask-next")]
#[command(about = "High-performance Windows 11 automation & text expansion engine", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a macro script file
    Run {
        #[arg(short, long)]
        file: String,
    },
    /// Start the HTTP/REST daemon for agent control
    Serve {
        #[arg(short, long, default_value = "127.0.0.1:8080")]
        bind: String,
    },
    /// Start text expansion and global hotkey background watcher
    Watch,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file } => {
            let content = fs::read_to_string(&file)?;
            let commands = parse_script(&content)?;
            println!("Executing {} commands from {}", commands.len(), file);
            let executed = Engine::execute(&commands).await?;
            println!("Successfully executed {} commands.", executed);
        }
        Commands::Serve { bind } => {
            run_server(&bind).await?;
        }
        Commands::Watch => {
            let expander = TextExpander::new();
            expander.start_hook();
            println!("TyperTask text expander watcher running. Press Ctrl+C to exit.");
            tokio::signal::ctrl_c().await?;
        }
    }

    Ok(())
}
