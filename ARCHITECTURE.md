# Architecture: CleanFlow

## Overview

CleanFlow is structured as a Rust workspace with a Tauri v2 desktop shell and a React/TypeScript frontend. The core library (`cf-core`) is fully independent of the GUI layer and can be driven via `cf-cli` for scripted or headless use. All AI classification runs locally via Ollama; no external network calls are made.

```
CleanFlow/
├── cf-core/              # Core library crate
│   └── src/
│       ├── scanner/      # Recursive directory walker, file metadata
│       ├── classifier/   # AI-powered file classification (Ollama)
│       ├── planner/      # Generates cleanup plan (move/delete/archive)
│       ├── executor/     # Executes plan, writes undo journal
│       ├── db/           # SQLite via rusqlite (scan history, undo log)
│       └── rules/        # User-defined classification rules (TOML)
├── cf-cli/               # CLI binary crate (headless operation)
├── src-tauri/            # Tauri v2 backend
│   └── src/
│       ├── main.rs
│       ├── error.rs
│       ├── state.rs
│       └── commands/     # IPC command handlers (scan, plan, execute, undo)
└── frontend/             # React + TypeScript + Vite
    └── src/
        ├── stores/       # Zustand state management
        └── components/   # UI components (FileTree, PlanView, UndoPanel)
```

## Component Diagram

```
┌─────────────────────────────────────────────────────────┐
│                   CleanFlow Desktop                     │
│                                                         │
│  ┌──────────────┐    Tauri IPC    ┌──────────────────┐  │
│  │   Frontend   │◄──────────────►│   src-tauri      │  │
│  │ React / TS   │                │  commands/*.rs   │  │
│  └──────────────┘                └────────┬─────────┘  │
│                                           │            │
│                                  ┌────────▼─────────┐  │
│                                  │     cf-core      │  │
│                                  │                  │  │
│                                  │  scanner         │  │
│                                  │  classifier ─────┼──┼──► Ollama
│                                  │  planner         │  │    localhost:11434
│                                  │  executor        │  │
│                                  │  db (SQLite)     │  │
│                                  │  rules (TOML)    │  │
│                                  └──────────────────┘  │
│                                                         │
│  cf-cli ──────────────────────────────► cf-core         │
└─────────────────────────────────────────────────────────┘
```

## Data Flow

1. **Scan**: `scanner` walks target directories (Downloads, Desktop, Documents) and collects file metadata (path, size, MIME type, last access, hash for duplicate detection).
2. **Classify**: `classifier` sends file metadata as prompts to Ollama; receives category labels (document, media, archive, junk, duplicate, etc.). User-defined rules in `rules/` override AI suggestions.
3. **Plan**: `planner` assembles a structured cleanup plan: a list of actions (move to category folder, delete junk, archive duplicates) with reversibility annotations.
4. **Execute**: `executor` applies the plan and writes every action to the SQLite undo journal before touching the filesystem.
5. **Undo**: any executed plan can be fully reversed by replaying the undo journal in reverse order.

## External Dependencies

| Dependency | Purpose | Network |
|------------|---------|---------|
| Ollama (localhost:11434) | AI file classification | localhost only |
| SQLite (rusqlite) | Scan history + undo journal | none |
| serde / serde_json | Serialization | none |
| Tauri v2 | Desktop shell + IPC | none |
| React + Vite | Frontend | none (build-time only) |
