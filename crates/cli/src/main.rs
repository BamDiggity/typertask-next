use clap::{Parser, Subcommand};
use std::fs;
use typertask_engine::{parse_script, Engine, TextExpander};
use typertask_daemon::run_server;
use tracing::info;

#[derive(Parser)]
#[command(name = "typertask-next")]
#[command(about = "High-performance Windows 11 automation & text expansion engine", version)]
struct Cli {
    /// With no subcommand, starts the local API and text expander.
    #[command(subcommand)]
    command: Option<Commands>,
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
        Some(Commands::Run { file }) => {
            let content = fs::read_to_string(&file)?;
            let commands = parse_script(&content)?;
            println!("Executing {} commands from {}", commands.len(), file);
            let executed = Engine::execute(&commands).await?;
            println!("Successfully executed {} commands.", executed);
        }
        Some(Commands::Serve { bind }) => serve(&bind).await?,
        None => serve("127.0.0.1:8080").await?,
        Some(Commands::Watch) => {
            let expander = TextExpander::new();
            expander.start_hook();
            info!("TyperTask-Next engine and real-time text expander active.");
            println!("TyperTask text expander watcher running. Press Ctrl+C to exit.");
            tokio::signal::ctrl_c().await?;
        }
    }

    Ok(())
}

async fn serve(bind: &str) -> Result<(), Box<dyn std::error::Error>> {
    let expander = TextExpander::new();
    expander.start_hook();
    info!("TyperTask-Next engine and real-time text expander active.");
    println!("TyperTask-Next is running.");
    println!("API: http://{bind}/api/v1/schema");
    println!("Leave this window open. Close it to stop.");
    run_server(bind).await?;
    Ok(())
}
