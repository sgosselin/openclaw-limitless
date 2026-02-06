# openclaw-limitless

Personal life indexer powered by [Omi](https://omi.me). Fetches conversation transcripts from the Omi wearable API and stores them locally.

## Setup

1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Copy `.env.example` to `.env` and add your Omi developer API key
3. Build: `cargo build`

## Usage

```bash
# Fetch conversations once
cargo run -- fetch

# Run as daemon (fetches every 20 minutes)
cargo run -- daemon
```

Transcripts are saved to `brain/raw/YYYY-MM-DD/HH.MM.log`.
