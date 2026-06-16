# Changelog — CleanFlow

All notable changes to this project will be documented in this file.

Format: [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) — [Semantic Versioning](https://semver.org/spec/v2.0.0.html)

---

## [0.1.0] — 2026-06-12

### Added

- Recursive file scanner for Downloads, Desktop, and Documents directories
- AI-powered file classification via Ollama (localhost:11434)
- Duplicate detection using content hashing (SHA-256)
- Junk file identification (temp files, empty files, broken shortcuts)
- Structured cleanup plan generation with per-file action annotations
- One-click plan execution with full undo support via SQLite journal
- `cf-core` Rust crate: `scanner/`, `classifier/`, `planner/`, `executor/`, `db/`, `rules/`
- `cf-cli` binary for headless/scripted operation
- Tauri v2 desktop shell for macOS, Windows, and Linux
- React/TypeScript frontend with Zustand state management

[0.1.0]: https://github.com/9t29zhmwdh-coder/CleanFlow/releases/tag/v0.1.0
