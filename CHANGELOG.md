# Changelog, CleanFlow

All notable changes to this project will be documented in this file.

Format: [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), [Semantic Versioning](https://semver.org/spec/v2.0.0.html)

---

## [1.1.2] - 2026-08-01

### Changed

- Dependabot no longer retries the `glib` update it cannot perform. GHSA-wrw7-89jp-8q8g is fixed in 0.20, and this project cannot reach it: `tauri` 2.x pins `gtk ^0.18`, `gtk` 0.18 requires `glib ^0.18`, and no patched 0.18.x exists, so cargo rejects the upgrade rather than resolving it. Three attempts had already failed identically, each one a red run on `main` that carried no information. Only the unreachable versions are ignored, so a backported 0.18.x fix would still arrive, and the advisory itself stays visible in the Security tab. The block goes away when Tauri moves to gtk-rs 0.20, the condition already recorded in `SECURITY.md`.

---

## [1.1.1] - 2026-07-31

### Fixed

- CI checked only macOS while the release builds for macOS, Linux and Windows. The AppImage and the Windows installer went out without ever having been compile-checked, so a fault appearing only on those platforms would have surfaced in a user's download rather than in a pull request. `check` now runs as a matrix over all three, matching what LogLens and BugRadar already do. The Linux runner installs the same GTK and WebKit packages the release workflow installs, since Tauri does not build without them.
- The very first matrix run earned its keep: `cargo clippy -D warnings` failed on Linux at `open_in_finder`, a needless borrow inside the `#[cfg(target_os = "linux")]` branch. That code had never been linted, because the branch is compiled out on macOS and macOS was the only runner. Fixed in the same change.
- The `solo-main-protection` ruleset now requires `Check (ubuntu-latest)`, `(macos-latest)` and `(windows-latest)` instead of the old single `Check`. Renaming a job without moving the required context leaves every future pull request permanently unmergeable while looking green, which is exactly what stalled a Dependabot pull request in BugRadar for three days.

---

## [1.1.0] - 2026-07-31

### Fixed

- **AI classification has never run.** `ROADMAP.md` lists it under v0.1.0 as shipped and the README advertises it in the feature table, but nothing in the application ever instantiated a classifier. Everything around it was built: `ScannedFile.ai_classification` exists, `planner.rs` and the rule engine read it in four places, `ScanPhase::AiClassifying` exists, the frontend types include it, and both a Claude and an Ollama classifier are fully implemented. The single call that fills the field was missing. It is there now, batching 50 files per request and reporting progress through the phase and counter the UI was already prepared for.
- The active backend now follows the setting instead of the presence of an API key. A Claude key stored once overrode every other choice, so anyone wanting to work locally had to delete it.

### Changed

- **A fresh installation classifies nothing.** The default backend is `RuleBasedOnly`. Since the feature never ran, nobody has been relying on it, and switching it on silently would have sent file names to Anthropic for every user who had once stored a key. Turning it on is a deliberate choice in settings.
- `AppSettings` gains `ollama_model`, defaulting to `llama3.2`. It carries a serde default so settings saved by earlier versions still load; without it, deserialisation would fail and the app would quietly fall back to factory settings, losing the user's scan paths and thresholds.

### Security

- `SECURITY.md` now states what the Claude backend transmits: the name, size and MIME type of every scanned file. File names are often the sensitive part, more so than contents, and a folder listing says a great deal about a person. No file contents are read or transmitted under any setting.

---

## [1.0.11] - 2026-07-31

### Fixed

- `SECURITY.md` described a network profile the application does not have. It named Ollama as the exception to "no external network calls", which suggests file names or contents are sent to a model. They are not: sorting is rule-based, the AI classifiers under `crates/cf-core/src/ai/` are never instantiated from the application, and the only outbound request is a two-second reachability check so the settings screen can report whether a backend is available. The file now says that.
- The supported-versions table still listed `0.1.x`, a line that no longer exists.

### Added

- `SECURITY.md` records GHSA-wrw7-89jp-8q8g against `glib` 0.18.5, which cannot be fixed from this repository because Tauri 2.11.5 pins `gtk ^0.18` and no patched 0.18.x exists.

---

## [1.0.10] - 2026-07-30

### Changed

- The README opens with what the tool is for instead of what it is built with. Both language versions previously began "AI-powered file organizer", the same phrase LifeSort used, so a reader looking at both could not tell which one to pick. The repository description is rewritten for the same reason.
- The opening now names the case this tool is **not** for and links to LifeSort by name. The exclusion is the sentence that saves the wrong reader ten minutes, and between two tools that sound alike it is worth more than any feature list.

---

## [1.0.9] - 2026-07-30

### Added

- `Cargo.lock` is committed. It was listed in `.gitignore`, so every build resolved dependencies afresh and no two builds were guaranteed to use the same versions. For an application rather than a library the lock file belongs in the repository: it is what makes a release reproducible and what lets a security advisory be checked against what actually shipped.
- A test that stores a secret and reads it back from a second process. `keyring` 4 refuses to build without a backend, so the silent variant of this defect cannot occur here, but the test still confirms the store actually works rather than merely compiling. Where the connection to the store is made differs by version: keyring 3 opens it on the first write, keyring 4 already on `Entry::new`. The test treats either failure as a missing service rather than a missing backend, which is what a Linux CI runner without a D-Bus secret service looks like.

---

## [1.0.8] - 2026-07-29

### Added

- `frontend/src/vite-env.d.ts`, referencing `vite/client`. Vite has always declared modules for `*.css` and the other asset types it handles, but nothing in this project pulled that declaration in. TypeScript 5 accepts the untyped side-effect import of `index.css` regardless, so the gap stayed invisible; TypeScript 7 rejects it with `TS2882`. The file belongs to Vite's own project scaffold and was simply missing, so this closes an existing hole rather than preparing for a specific upgrade.
### Security

- The release workflow no longer grants `contents: write` for its whole run. The permission moves to the one job that publishes the release, and everything else runs with `contents: read`. OpenSSF Scorecard scores the Token-Permissions check 0 out of 10 whenever any workflow holds a top-level write permission, regardless of how little of the run needs it, so this single line was what held the check at zero.

---

## [1.0.7] - 2026-07-29

### Changed

- `reqwest` updated from 0.12 to 0.13. The `rustls-tls` feature no longer exists in 0.13 and is replaced by `rustls`, so the automated dependency update could not build: it can raise a version number but not rename a feature, and dependency resolution failed before anything compiled.

### Security

- TLS now trusts the operating system's certificate store rather than a bundled root set. The `rustls` feature in 0.13 pulls in `rustls-platform-verifier`, where 0.12 resolved roots independently of the host. A machine that trusts an internal certificate authority, which is the normal case behind a corporate proxy, now works without extra configuration. The other side of that is real and worth naming: the trust decision moves to the machine the tool runs on, so a tampered local certificate store is enough to intercept the connection.
- The rustls crypto provider changes from `ring` to `aws-lc-rs`, which is what the `rustls` feature selects in 0.13.

---

## [1.0.6] - 2026-07-29

### Changed

Dependency and workflow updates merged since 1.0.5:

- chore(ci): bump the actions group across 1 directory with 3 updates
- chore(deps): bump the npm group across 1 directory with 3 updates

---

## [1.0.5] - 2026-07-28

### Fixed

- The CodeQL job requested `packages: read`, `actions: read` and `contents: read` at job level, repeating grants the workflow level already provides. OpenSSF Scorecard counts that as excessive token permissions and scores `Token-Permissions` at 0 out of 10 for it. The job now requests only `security-events: write`, which is the one grant that genuinely exceeds the workflow default.

## [1.0.4] - 2026-07-28

### Changed

- CodeQL moved from GitHub's default setup to an advanced setup with a committed `.github/workflows/codeql.yml`. The default setup skips pull requests that touch no code of a given language, so a dependency pull request changing only a lock file reported `skipping` on the required `Analyze (...)` checks forever and could never be merged. The workflow runs on every pull request regardless of what changed. It also uses the `security-extended` query suite, which the default setup does not allow choosing. Required checks are unchanged: verified on `BugRadar` that all eight, the generic `CodeQL` check included, turn green under this setup.
- Dependabot now groups only minor and patch updates per ecosystem; majors arrive as individual pull requests. The previous grouping put React 18 to 19, Tailwind 3 to 4 and similar breaking changes into one pull request together with urgently needed security patches, which made the whole batch unreviewable and unmergeable. Actions stay grouped wholesale. Follows `engineering-standards` v0.11.0.

## [1.0.3] - 2026-07-28

### Security

- `postcss` updated to 8.5.24, closing a high-severity path traversal in the source map auto-loading via `sourceMappingURL` that affects all versions up to and including 8.5.17.

Applied as a normal pull request rather than by merging Dependabot's, because Dependabot pull requests cannot currently pass this repository's required checks: CodeQL runs through GitHub's default setup, which does not trigger on a pull request that only touches a lock file, so its checks report `skipping` and never turn green. Bypassing a required check is not an option per `standards/ci-cd.md` section 7, so the fix takes the route that runs the full pipeline.

## [1.0.2] - 2026-07-28

### Added

- `.github/dependabot.yml`, covering GitHub Actions, the Cargo workspace and the frontend npm packages, with grouped weekly updates. The file was missing, and without it there are no version updates at all: security alerts only fire for disclosed vulnerabilities. Follows `engineering-standards` v0.10.0.

### Fixed

- The repository carried five different version numbers: 0.1.3 in both crates, 1.0.0 in `src-tauri`, 0.2.8 in `frontend/package.json`, and 1.0.1 in `tauri.conf.json`, which was the tagged one. A `[workspace.package]` section now holds a single version that the crates inherit, matching the four sibling Tauri repositories, and the frontend and Tauri config agree with it.
- `actions/checkout` was pinned to two different SHAs across the three workflows. All now use v7.0.1 with the full version in the comment, per `engineering-standards` `standards/ci-cd.md` section 2.

## [1.0.1] - 2026-07-20

### Changed

- OpenSSF Scorecard workflow and badge.
- `copilot-instructions.md` for consistent AI-assisted contributions.
- Split the README's security/CI badges onto their own line, separate from the platform/tech/AI badges (they were rendering as a single merged line).

## [1.0.0] - 2026-07-17

First stable release: a real, packaged, installable distribution exists
for end users. Real macOS/Windows/Linux installers (DMG, NSIS, AppImage/deb/rpm).

## [0.2.9] - 2026-07-17

### Changed
- CI: added an explicit `permissions: contents: read` block to the workflow(s) that were missing one (CodeQL `actions/missing-workflow-permissions`), narrowing the default GITHUB_TOKEN scope.

## [0.2.8] - 2026-07-12

### Added

- Release workflow (`.github/workflows/release.yml`): builds and attaches macOS (DMG), Windows (NSIS installer), and Linux (AppImage) bundles to a GitHub Release on every tag push. Previously, no release ever had an installer attached; users had to build from source.
- README/README.de.md: Download section linking to the latest release's installers.

### Fixed

- All GitHub Actions in `ci.yml` pinned to a commit SHA, matching the portfolio's Action Pinning standard.

### Fixed

- Removed an eszett and em-dashes across the repo (TEMPLATE_NOTES.md, ROADMAP.md, SKELETON.md, CONTRIBUTING.md, ARCHITECTURE.md, and five Rust source files). Swiss German orthography.

## [0.2.6] - 2026-07-11

### Fixed

- SemVer correction: v0.1.1 added a genuine new feature (full English/German UI with a language toggle) but was versioned as a patch. Renumbered v0.1.1 through v0.1.6 to v0.2.0 through v0.2.5 (same commits, tags and releases recreated at identical SHAs), per the portfolio's SemVer discipline (patch = fix, minor = feature, major = finished product).

## [0.2.5] - 2026-07-11

### Added

- Documented Dual-Licensing assessment (Community-only) in ROADMAP.md.

### Fixed

- Removed em-dashes from ROADMAP.md and SECURITY.md headings and body text.

## [0.2.4] - 2026-07-11

### Fixed

- Added Linux to the Platform badge in README.md/README.de.md; the tagline already mentioned Linux support but the badge only listed macOS and Windows
- Removed a stray duplicate "Plattform"/"Lizenz" badge from README.de.md left over from an earlier edit

## [0.2.3] - 2026-07-11

### Fixed

- Updated actions/setup-node to its latest major version in CI, since GitHub is deprecating the Node.js 20 runtime and the previous version was being forced onto Node 24 and crashing during post-run cleanup.

## [0.2.2] - 2026-07-10

### Changed

- Moved the "New here? -> beginners guide" callout in README.md above Features (previously only appeared near Requirements)

### Added

- Added the "New here?" beginner guide callout to README.de.md (was missing)

## [0.2.1] - 2026-07-07

### Fixed

- Tailwind CSS was never actually compiled: `frontend/` had no `postcss.config.js`, so Vite passed the raw `@tailwind` directives straight through to the output CSS untouched. The app rendered with none of its intended styling (colors, grid layouts, spacing) in every build, including the previous 0.2.0 release
- The entire scan progress flow was broken: no `src-tauri/capabilities/` file existed, so Tauri's permission system rejected every `event.listen` call from the frontend with a runtime error, meaning scan results never appeared no matter how long you waited after clicking Scan
- `state.scans[id].status` was only ever set once at scan start and never updated afterwards, so `get_scan_status` always reported the initial "Walking" phase
- A follow-up race condition where a very fast scan's completion event could be emitted before the frontend had finished registering its listener is now handled by polling current status once after the listener attaches
- Scan errors were silently swallowed with no UI feedback; errors are now shown in the Scan view

## [0.2.0] - 2026-07-07

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
