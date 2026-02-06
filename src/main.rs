mod omi_client;
mod storage;

use std::thread;
use std::time::Duration;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use omi_client::OmiClient;
use storage::brain_dir;

const FETCH_INTERVAL: Duration = Duration::from_secs(20 * 60);

#[derive(Parser)]
#[command(name = "openclaw", about = "Personal life indexer powered by Omi")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Fetch conversations once and save to brain/
    Fetch,
    /// Fetch conversations every 20 minutes
    Daemon,
}

fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();
    let api_key = std::env::var("OMI_API_KEY")
        .context("OMI_API_KEY not set. Create a .env file or export the variable.")?;
    let client = OmiClient::new(api_key);

    match cli.command {
        Command::Fetch => fetch_once(&client)?,
        Command::Daemon => run_daemon(&client)?,
    }

    Ok(())
}

fn fetch_once(client: &OmiClient) -> Result<()> {
    let conversations = client.fetch_conversations()?;
    if conversations.is_empty() {
        println!("No conversations found.");
        return Ok(());
    }
    storage::save_transcripts(&brain_dir(), &conversations)?;
    Ok(())
}

fn run_daemon(client: &OmiClient) -> Result<()> {
    println!("Starting daemon (fetching every 20 minutes). Press Ctrl+C to stop.");
    loop {
        if let Err(e) = fetch_once(client) {
            eprintln!("Fetch error: {e:#}");
        }
        thread::sleep(FETCH_INTERVAL);
    }
}
