mod omi_client;
mod storage;

use std::thread;
use std::time::Duration;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use omi_client::OmiClient;
use storage::brain_dir;

const DEFAULT_PERIOD_SEC: u64 = 20 * 60;

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
    /// Fetch conversations periodically
    Daemon {
        /// Fetch interval in seconds (default: 1200)
        #[arg(long, default_value_t = DEFAULT_PERIOD_SEC)]
        period_sec: u64,
    },
}

fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();
    let api_key = std::env::var("OMI_API_KEY")
        .context("OMI_API_KEY not set. Create a .env file or export the variable.")?;
    let client = OmiClient::new(api_key);

    match cli.command {
        Command::Fetch => fetch_once(&client)?,
        Command::Daemon { period_sec } => run_daemon(&client, period_sec)?,
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

fn run_daemon(client: &OmiClient, period_sec: u64) -> Result<()> {
    println!("Starting daemon (fetching every {period_sec}s). Press Ctrl+C to stop.");
    let interval = Duration::from_secs(period_sec);
    loop {
        if let Err(e) = fetch_once(client) {
            eprintln!("Fetch error: {e:#}");
        }
        thread::sleep(interval);
    }
}
