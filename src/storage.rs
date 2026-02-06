use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};

use crate::omi_client::Conversation;

pub fn save_transcripts(brain_dir: &Path, conversations: &[Conversation]) -> Result<()> {
    let mut saved = 0;

    for conv in conversations {
        let ts = match &conv.created_at {
            Some(s) => s
                .parse::<DateTime<Utc>>()
                .with_context(|| format!("failed to parse created_at: {s}"))?,
            None => {
                eprintln!("Skipping conversation {} (no created_at)", conv.id);
                continue;
            }
        };

        let date_dir = brain_dir
            .join("raw")
            .join(ts.format("%Y-%m-%d").to_string());
        fs::create_dir_all(&date_dir)
            .with_context(|| format!("failed to create directory {}", date_dir.display()))?;

        let filename = format!("{}_{}.log", ts.format("%H.%M.%S"), conv.id);
        let log_file = date_dir.join(&filename);

        if log_file.exists() {
            continue;
        }

        let content = format_conversation(conv);
        fs::write(&log_file, &content)
            .with_context(|| format!("failed to write {}", log_file.display()))?;

        saved += 1;
    }

    println!("Saved {saved} new conversations ({} total from API)", conversations.len());
    Ok(())
}

fn format_conversation(conv: &Conversation) -> String {
    let mut output = String::new();

    for seg in &conv.transcript_segments {
        let speaker = match &seg.speaker {
            Some(s) => s.as_str(),
            None if seg.is_user => "You",
            None => "Unknown",
        };
        output.push_str(&format!("[{}]: {}\n", speaker, seg.text));
    }

    output
}

pub fn brain_dir() -> PathBuf {
    PathBuf::from("brain")
}
