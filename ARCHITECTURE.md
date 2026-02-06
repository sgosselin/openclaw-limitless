# Architecture

## High-Level Overview

```
                      ┌─────────────────────────────────────────────┐
                      │              openclaw CLI                    │
                      │                                             │
                      │   ┌───────────┐        ┌───────────┐       │
                      │   │   fetch   │        │  daemon    │       │
                      │   │ (one-shot)│        │ (periodic, │       │
                      │   │           │        │  20 min)   │       │
                      │   └─────┬─────┘        └─────┬─────┘       │
                      │         │                    │              │
                      │         └────────┬───────────┘              │
                      │                  ▼                          │
                      │         ┌────────────────┐                  │
                      │         │   omi_client    │                  │
                      │         │                │                  │
                      │         │ GET /v1/dev/   │                  │
                      │         │ user/          │                  │
                      │         │ conversations  │                  │
                      │         └───────┬────────┘                  │
                      │                 │                           │
                      │      JSON response:                        │
                      │      [{id, title,                          │
                      │        transcript_segments,                │
                      │        created_at, ...}]                   │
                      │                 │                           │
                      │                 ▼                           │
                      │         ┌────────────────┐                  │
                      │         │    storage      │                  │
                      │         │                │                  │
                      │         │ Writes to:     │                  │
                      │         │ brain/raw/     │                  │
                      │         │  YYYY-MM-DD/   │                  │
                      │         │   HH.MM.SS_    │                  │
                      │         │   <id>.log     │                  │
                      │         └────────────────┘                  │
                      └─────────────────────────────────────────────┘
                                        │
                                        │ HTTPS
                                        ▼
                              ┌───────────────────┐
                              │     Omi API        │
                              │  api.omi.me        │
                              │                   │
                              │ Auth: Bearer token │
                              │ (OMI_API_KEY env)  │
                              └───────────────────┘
```

## Components

### CLI (`main.rs`)
Entry point. Parses subcommands via `clap`:
- `fetch` — one-shot: fetch conversations now and write to brain/
- `daemon` — loop: fetch every 20 minutes

### Omi Client (`omi_client.rs`)
HTTP client that calls the Omi Developer API.
- Reads API key from `OMI_API_KEY` environment variable
- Calls `GET https://api.omi.me/v1/dev/user/conversations`
- Deserializes JSON response into Rust structs

### Storage (`storage.rs`)
Handles writing transcript data to disk.
- Creates date-based directories: `brain/raw/YYYY-MM-DD/`
- One file per conversation: `HH.MM.SS_<conversation_id>.log`
- Timestamp derived from the conversation's `created_at` field
- Idempotent: skips writing if the file already exists

## Data Flow

1. CLI triggers a fetch (manually or on timer)
2. `omi_client` sends authenticated GET request to Omi API
3. API returns JSON array of conversation objects (with transcript segments)
4. `storage` writes each conversation to `brain/raw/YYYY-MM-DD/HH.MM.SS_<id>.log` (skips if already saved)

## Configuration

| Variable      | Source          | Description                |
|---------------|-----------------|----------------------------|
| `OMI_API_KEY` | env var / .env  | Omi Developer API key      |
