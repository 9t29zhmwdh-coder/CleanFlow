# Roadmap: CleanFlow

## v0.1.0, Initial Release (2026-06-12) ✅

- Recursive file scanner (Downloads, Desktop, Documents)
- AI-powered file classification via Ollama
- Duplicate detection via content hashing
- Junk file identification (temp files, empty files, broken shortcuts)
- Structured cleanup plan generation
- One-click plan execution with full undo support
- SQLite undo journal
- Tauri v2 desktop shell (macOS, Windows, Linux)

## v0.2.0, Rules & Scheduling

- User-defined classification rules (TOML-based)
- Scheduled scans (daily/weekly via system scheduler)
- Exclude-list for paths and file patterns
- Batch rename support (pattern-based)
- Export cleanup report as PDF/CSV
- Improved duplicate grouping UI

## v0.3.0, Smart Suggestions & Multi-Folder

- Scan arbitrary folders (not just standard locations)
- AI-generated folder structure suggestions
- "Similar files" clustering (near-duplicates via fuzzy hash)
- Per-category statistics and disk usage visualization
- Plugin system for custom post-process actions

## v1.0.0: Stable Release

- Stable public API for `cf-core` (semver)
- Full test coverage (unit + integration)
- Packaged installers (`.dmg`, `.msi`, `.AppImage`)
- Comprehensive documentation site

## Under Consideration

- Optional handoff to [LifeSort](https://github.com/9t29zhmwdh-coder/LifeSort) for long-term filing: CleanFlow's job is a one-off cleanup pass (junk, duplicates, trash), LifeSort's is ongoing archival sorting (Photos/Documents/Media into a folder structure). A CLI pipeline (`cleanflow scan --pass-through | lifesort import`) could let files CleanFlow keeps but doesn't just delete get routed straight into LifeSort's classifier instead of being left where they landed. Not scoped yet.

## Out of Scope

- Cloud storage scanning (Google Drive, OneDrive, iCloud): privacy boundary
- Automatic uploads or backups to any remote service
- Windows Explorer / Finder integration via shell extensions
- Mobile platforms (iOS, Android)

## Dual-Licensing Readiness

Assessed 2026-07-11: Community-only, not a Dual-Licensing candidate. CleanFlow is a single-user desktop productivity tool (personal file organization) with no team, fleet or multi-tenant dimension anywhere on the roadmap, unlike the governance/observability tools in this portfolio where multi-subscription or multi-tenant scope is a natural Enterprise-tier split point. The plugin system planned for v0.3.0 is the only feature with any paid-extension potential, but it is not implemented yet and would need real third-party plugin demand to justify a Commercial tier. Revisit only if a genuine team/business use case (e.g. shared cleanup policies across a company's fleet) emerges.
