# Changelog, CleanFlow

All notable changes to this project will be documented in this file.

Format: [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), [Semantic Versioning](https://semver.org/spec/v2.0.0.html)

---

## [0.1.5] - 2026-07-11

### Fixed

- Added Linux to the Platform badge in README.md/README.de.md; the tagline already mentioned Linux support but the badge only listed macOS and Windows
- Removed a stray duplicate "Plattform"/"Lizenz" badge from README.de.md left over from an earlier edit

## [0.1.4] - 2026-07-11

### Fixed

- Updated actions/setup-node to its latest major version in CI, since GitHub is deprecating the Node.js 20 runtime and the previous version was being forced onto Node 24 and crashing during post-run cleanup.

## [0.1.3] - 2026-07-10

### Changed

- Moved the "New here? -> beginners guide" callout in README.md above Features (previously only appeared near Requirements)

### Added

- Added the "New here?" beginner guide callout to README.de.md (was missing)

## [0.1.2] - 2026-07-07

### Fixed

- Tailwind CSS was never actually compiled: `frontend/` had no `postcss.config.js`, so Vite passed the raw `@tailwind` directives straight through to the output CSS untouched. The app rendered with none of its intended styling (colors, grid layouts, spacing) in every build, including the previous 0.1.1 release
- The entire scan progress flow was broken: no `src-tauri/capabilities/` file existed, so Tauri's permission system rejected every `event.listen` call from the frontend with a runtime error, meaning scan results never appeared no matter how long you waited after clicking Scan
- `state.scans[id].status` was only ever set once at scan start and never updated afterwards, so `get_scan_status` always reported the initial "Walking" phase
- A follow-up race condition where a very fast scan's completion event could be emitted before the frontend had finished registering its listener is now handled by polling current status once after the listener attaches
- Scan errors were silently swallowed with no UI feedback; errors are now shown in the Scan view

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
