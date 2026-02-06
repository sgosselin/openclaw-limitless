# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

openclaw — a personal life indexer that fetches conversation transcripts from the Omi wearable API and stores them locally in `brain/raw/YYYY-MM-DD/HH.MM.log`.

## Build & Run

```bash
cargo build                     # build
cargo run -- fetch              # one-shot fetch
cargo run -- daemon             # fetch every 20 minutes
```

Requires `OMI_API_KEY` environment variable (or `.env` file). See `.env.example`.

## Architecture

Single Rust binary with three modules:
- `src/main.rs` — CLI entry point (clap subcommands: `fetch`, `daemon`)
- `src/omi_client.rs` — HTTP client for `GET https://api.omi.me/v1/dev/user/conversations`
- `src/storage.rs` — writes transcript data to `brain/raw/` date-organized directories

See `ARCHITECTURE.md` for the full diagram and data flow.

## Key conventions

- Blocking HTTP via `reqwest::blocking` (no async runtime)
- `anyhow` for error handling throughout
- API key loaded from env via `dotenvy`
- The `brain/` directory is gitignored — it contains local personal data
