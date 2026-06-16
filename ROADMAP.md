# Roadmap — CleanFlow

## v0.1.0 — Initial Release (2026-06-12) ✅

- Recursive file scanner (Downloads, Desktop, Documents)
- AI-powered file classification via Ollama
- Duplicate detection via content hashing
- Junk file identification (temp files, empty files, broken shortcuts)
- Structured cleanup plan generation
- One-click plan execution with full undo support
- SQLite undo journal
- Tauri v2 desktop shell (macOS, Windows, Linux)

## v0.2.0 — Rules & Scheduling

- User-defined classification rules (TOML-based)
- Scheduled scans (daily/weekly via system scheduler)
- Exclude-list for paths and file patterns
- Batch rename support (pattern-based)
- Export cleanup report as PDF/CSV
- Improved duplicate grouping UI

## v0.3.0 — Smart Suggestions & Multi-Folder

- Scan arbitrary folders (not just standard locations)
- AI-generated folder structure suggestions
- "Similar files" clustering (near-duplicates via fuzzy hash)
- Per-category statistics and disk usage visualization
- Plugin system for custom post-process actions

## v1.0.0 — Stable Release

- Stable public API for `cf-core` (semver)
- Full test coverage (unit + integration)
- Packaged installers (`.dmg`, `.msi`, `.AppImage`)
- Localization (EN + DE)
- Comprehensive documentation site

## Out of Scope

- Cloud storage scanning (Google Drive, OneDrive, iCloud) — privacy boundary
- Automatic uploads or backups to any remote service
- Windows Explorer / Finder integration via shell extensions
- Mobile platforms (iOS, Android)
