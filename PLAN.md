# Implementation Plan

## Tasks

### Task 1: Project Scaffolding
- `cargo init` the project
- Set up `Cargo.toml` with dependencies: reqwest (blocking), serde, serde_json, clap (derive), chrono, anyhow, dotenvy
- Add `.gitignore` (Rust defaults + `brain/` + `.env`)
- Create module structure: `main.rs`, `omi_client.rs`, `storage.rs`

### Task 2: Omi API Client (`src/omi_client.rs`)
- Define Rust structs matching Omi API JSON response for conversations
- Implement `fetch_conversations()` that:
  - Reads `OMI_API_KEY` from environment
  - Sends `GET https://api.omi.me/v1/dev/user/conversations` with Bearer auth
  - Returns deserialized conversation data

### Task 3: Storage Layer (`src/storage.rs`)
- Implement `save_transcripts()` that:
  - Creates `brain/raw/YYYY-MM-DD/` directory if it doesn't exist
  - Writes transcript content to `HH.MM.log`
  - Formats transcript segments into readable text

### Task 4: CLI with fetch + daemon commands (`src/main.rs`)
- Define clap subcommands: `fetch`, `daemon`
- `fetch`: call omi_client, pass results to storage, exit
- `daemon`: loop with 20-minute sleep, calling fetch each iteration

### Task 5: Documentation + Final Polish
- Create `CLAUDE.md` with build/test/run instructions
- Add a `.env.example` with placeholder for `OMI_API_KEY`
- Update `README.md` with usage instructions

## Execution Order

Tasks 1 must go first (scaffolding). Tasks 2 and 3 can run in parallel.
Task 4 depends on 2 and 3. Task 5 is last.

```
Task 1 (scaffolding)
    ├──► Task 2 (omi_client)  ──┐
    └──► Task 3 (storage)     ──┤
                                └──► Task 4 (CLI wiring)
                                         └──► Task 5 (docs)
```
