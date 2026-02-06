use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Local;

use crate::omi_client::Conversation;

pub fn save_transcripts(brain_dir: &Path, conversations: &[Conversation]) -> Result<()> {
    let now = Local::now();
    let date_dir = brain_dir
        .join("raw")
        .join(now.format("%Y-%m-%d").to_string());

    fs::create_dir_all(&date_dir)
        .with_context(|| format!("failed to create directory {}", date_dir.display()))?;

    let log_file = date_dir.join(format!("{}.log", now.format("%H.%M")));
    let content = format_conversations(conversations);

    fs::write(&log_file, &content)
        .with_context(|| format!("failed to write {}", log_file.display()))?;

    println!("Saved {} conversations to {}", conversations.len(), log_file.display());
    Ok(())
}

fn format_conversations(conversations: &[Conversation]) -> String {
    let mut output = String::new();

    for conv in conversations {
        if let Some(ref id) = Some(&conv.id) {
            output.push_str(&format!("=== Conversation {} ===\n", id));
        }

        for seg in &conv.transcript_segments {
            let speaker = match &seg.speaker {
                Some(s) => s.as_str(),
                None if seg.is_user => "You",
                None => "Unknown",
            };
            output.push_str(&format!("[{}]: {}\n", speaker, seg.text));
        }

        output.push('\n');
    }

    output
}

pub fn brain_dir() -> PathBuf {
    PathBuf::from("brain")
}
