# Copilot Instructions for CleanFlow

CleanFlow is a cross-platform (macOS, Windows, Linux) desktop file organizer built with Rust and Tauri. It classifies files with AI, detects duplicates and junk, and executes user-approved plans with full undo support. See README.md and ARCHITECTURE.md for the full picture before making changes.

## Code style
- Functions stay small and single-purpose, prefer under 20 lines
- Naming: verb+noun for functions (`scanDirectory`, `detectDuplicates`), clear intent for variables, no `x`/`temp`/`data`
- Constants in UPPER_SNAKE_CASE
- Comments explain WHY, never WHAT. No comment restates what the code already says
- No speculative abstractions: solve the task at hand, do not build for hypothetical future requirements

## Text and documentation
- Never use em-dash (—), en-dash (–), or a spaced hyphen as a sentence-break substitute, in code comments, commit messages, PR descriptions, or docs. Rephrase instead
- README.md and README.de.md must stay in sync: same structure, same sections, both languages updated together
- Any functional change needs a CHANGELOG.md entry and follows semantic versioning: patch for fixes/docs, minor for new features, major for breaking changes
- No separate License badge in README (intentional project convention, not an omission)
- README badge row is two lines: line 1 = CI, CodeQL, OpenSSF Scorecard, OpenSSF Best Practices (in that order); line 2 = platform/tech/AI badges. Never add a badge as an isolated standalone line

## Git workflow
- This repo enforces branch protection on `main`: no direct pushes, no force pushes, a pull request is required for every change. Never attempt to bypass this
- Semantic commit messages: `type(scope): description` (feat, fix, security, refactor, test, docs)
- One commit = one logical change

## Security
- Never commit secrets, API keys, or tokens. Anything sensitive belongs in a secret manager, not in code
- Validate input at actual boundaries (user input, external APIs), do not add defensive checks for cases that cannot occur internally
- Flag anything that looks like a security regression instead of silently working around it

## Before opening a PR
- Run the existing test suite and build; do not open a PR with failing checks
- Keep the diff scoped to the task described in the issue, no unrelated refactors or cleanup bundled in
