# Changelog, CleanFlow

All notable changes to this project will be documented in this file.

Format: [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), [Semantic Versioning](https://semver.org/spec/v2.0.0.html)

---

## [0.1.1] - 2026-07-07

### Fixed

- Build failure: `tauri` dependency enabled the `protocol-asset` feature without the matching asset protocol scope in `tauri.conf.json`, breaking `generate_context!`; removed the unused feature
- Missing `thiserror` and `reqwest` dependencies in `src-tauri/Cargo.toml`, despite being used in `error.rs` and `commands/mod.rs`
- Scanned files were never written back into app state after a scan completed, so the plan preview always showed zero actions regardless of what was scanned
- CI excluded the `cleanflow-app` crate from check/clippy/test, so the above issues went undetected; CI now covers the full workspace plus a new frontend typecheck/build job
- Missing Tauri icon set (`tauri.conf.json` referenced icons that were never generated)
- Several unused imports/variables and a clippy lint
- README AI Providers table had a duplicated Ollama row and no longer listed Claude as an option
- LICENSE copyright line formatting

### Added

- Full English/German UI with a language toggle (English default, German switchable)
- Onboarding sections in README: how the app runs, in practice summary, uninstall/cleanup steps
- Real EN/DE screenshots of the running app

## [0.1.0] - 2026-06-12

### Added

- Recursive file scanner for Downloads, Desktop, and Documents directories
- AI-powered file classification via Ollama (localhost:11434)
- Duplicate detection using content hashing (SHA-256)
- Junk file identification (temp files, empty files, broken shortcuts)
- Structured cleanup plan generation with per-file action annotations
- One-click plan execution with full undo support via an embedded journal database
- `cf-core` Rust crate: `scanner/`, `classifier/`, `planner/`, `executor/`, `db/`, `rules/`
- `cf-cli` binary for headless/scripted operation
- Tauri v2 desktop shell for macOS, Windows, and Linux
- React/TypeScript frontend with Zustand state management

[0.1.1]: https://github.com/9t29zhmwdh-coder/CleanFlow/releases/tag/v0.1.1
[0.1.0]: https://github.com/9t29zhmwdh-coder/CleanFlow/releases/tag/v0.1.0
