<div align="center">
  <img src="RayStudio.png" alt="RayStudio Logo" width="120"/>

  <h1>CleanFlow</h1>
</div>

[🇩🇪 Deutsche Version](README.de.md)

**AI-powered file organizer for macOS, Windows and Linux, built with Rust and Tauri.**

CleanFlow scans your Downloads, Desktop, Documents or any directory, classifies files with AI, detects duplicates, identifies junk, and generates an actionable plan. One click to execute; with full undo support.

[![CI](https://github.com/9t29zhmwdh-coder/CleanFlow/actions/workflows/ci.yml/badge.svg)](https://github.com/9t29zhmwdh-coder/CleanFlow/actions) ![Platform](https://img.shields.io/badge/Platform-macOS_%7C_Windows-lightgrey) ![Rust](https://img.shields.io/badge/Rust-CE422B?logo=rust&logoColor=white) ![Tauri](https://img.shields.io/badge/Tauri-24C8D8?logo=tauri&logoColor=white) ![AI | Claude Code](https://img.shields.io/badge/AI-Claude_Code-black?logo=anthropic&logoColor=white) ![AI | Copilot](https://img.shields.io/badge/AI-Copilot-black?logo=github&logoColor=white)

---

## Features

| Feature | Description |
|---|---|
| **File Analysis** | MIME detection, SHA-256 deduplication, series grouping |
| **AI Classification** | Local AI classifies invoices, contracts, screenshots, code, etc. |
| **Clean-Up Engine** | Detects DMGs, .DS_Store, temp files, zombie files, old versions |
| **Rule Engine** | Built-in + custom rules (e.g. "PDF + invoice → Documents/Finance/2026") |
| **Action Preview** | Review every proposed action before executing |
| **One-Click CleanFlow** | Execute all selected actions in one click |
| **Full Undo** | Journal-based undo for any executed action |
| **CLI Mode** | `cleanflow scan`, `cleanflow organize`, `cleanflow undo` |

---

## Requirements

- [Rust](https://rustup.rs/) 1.77+
- [Node.js](https://nodejs.org/) 20+
- [Tauri CLI v2](https://tauri.app/): `cargo install tauri-cli`
- macOS / Windows / Linux (Tauri v2)

---

## Quick Start

```bash
git clone https://github.com/9t29zhmwdh-coder/CleanFlow
cd CleanFlow

# Install frontend dependencies
cd frontend && npm install && cd ..

# Run in development mode
cargo tauri dev

# Build release
cargo tauri build
```

### CLI Only

```bash
cargo install --path crates/cf-cli

cleanflow scan ~/Downloads
cleanflow organize ~/Downloads --execute
cleanflow undo
cleanflow rules list
```

---

## AI Providers

| Provider | Setup |
|---|---|
| **Ollama (local)** | Set URL in Settings (default: `http://localhost:11434`) |
| **Ollama (local)** | Install [Ollama](https://ollama.ai), run `ollama pull llama3.2` |
| **Rule-based only** | No AI required: uses built-in rules only |


---

## Built-in Rules

| Rule | Condition | Action |
|---|---|---|
| Screenshots | `Name matches Screenshot*` | → Pictures/Screenshots |
| DMG Installers | `Extension .dmg` | Trash |
| .DS_Store | `Name contains .DS_Store` | Trash |
| PDF Invoices | `PDF + AI: invoice` | → Documents/Finance/{year} |
| Temp Files | `Extension .tmp/.log/.cache` | Trash |
| Zombie Files | `Never accessed + older 90 days` | Archive |

---

## Architecture

```
CleanFlow/
├── crates/cf-core/      — Rust: scanner, AI, rules, planner, executor, undo
├── crates/cf-cli/       — CLI binary (clap)
├── src-tauri/           — Tauri v2 backend + IPC commands
└── frontend/            — React + TypeScript + Tailwind
```

---

**Author:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** Active · ![version](https://img.shields.io/github/v/release/9t29zhmwdh-coder/CleanFlow?color=6b7280&style=flat-square) · **License:** MIT
