<div align="center">
  <img src="RayStudio.png" alt="RayStudio Logo" width="120"/>

  <h1>CleanFlow</h1>
</div>

[🇬🇧 English Version](README.md)

**KI-gestützter Datei-Organizer für macOS, Windows und Linux — entwickelt mit Rust + Tauri.**

CleanFlow durchsucht Downloads, Desktop, Dokumente oder beliebige Verzeichnisse, klassifiziert Dateien per KI, erkennt Duplikate, findet Junk-Dateien und erstellt einen übersichtlichen Aktionsplan. Ein Klick zum Ausführen — mit vollständigem Undo-System.

![Rust](https://img.shields.io/badge/Rust-1.77+-orange?logo=rust)
![Tauri](https://img.shields.io/badge/Tauri-v2-blue?logo=tauri)
![Plattform](https://img.shields.io/badge/Plattform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)
![Lizenz](https://img.shields.io/badge/Lizenz-MIT-green)

---

## Funktionen

| Funktion | Beschreibung |
|---|---|
| **Datei-Analyse** | MIME-Erkennung, SHA-256-Duplikate, Serien-Gruppen |
| **KI-Klassifizierung** | Claude Haiku erkennt Rechnungen, Verträge, Screenshots, Code |
| **Clean-Up-Engine** | DMGs, .DS_Store, Temp-Dateien, Zombie-Dateien, alte Versionen |
| **Regelwerk** | Eingebaute + benutzerdefinierte Regeln |
| **Aktionsvorschau** | Jede Aktion vor der Ausführung überprüfen |
| **Ein-Klick CleanFlow** | Alle gewählten Aktionen in einem Klick ausführen |
| **Undo-System** | Journal-basiertes Rückgängigmachen jeder Aktion |
| **CLI-Modus** | `cleanflow scan`, `cleanflow organize`, `cleanflow undo` |

---

## Voraussetzungen

- [Rust](https://rustup.rs/) 1.77+
- [Node.js](https://nodejs.org/) 20+
- [Tauri CLI v2](https://tauri.app/): `cargo install tauri-cli`
- macOS / Windows / Linux

---

## Schnellstart

```bash
git clone https://github.com/9t29zhmwdh-coder/CleanFlow
cd CleanFlow

cd frontend && npm install && cd ..

# Entwicklungsmodus
cargo tauri dev

# Release-Build
cargo tauri build
```

### Nur CLI

```bash
cargo install --path crates/cf-cli

cleanflow scan ~/Downloads
cleanflow organize ~/Downloads --execute
cleanflow undo
cleanflow rules list
```

---

## KI-Anbieter

| Anbieter | Einrichtung |
|---|---|
| **Claude (Anthropic)** | API-Key in Einstellungen eingeben → sicher im Keychain gespeichert |
| **Ollama (lokal)** | [Ollama](https://ollama.ai) installieren, `ollama pull llama3.2` ausführen |
| **Nur Regelbasiert** | Kein KI-Anbieter nötig |

Kosten: ~$0.002 pro 1.000 Dateien mit `claude-haiku-4-5`.

---

## Architektur

```
CleanFlow/
├── crates/cf-core/      — Rust: Scanner, KI, Regeln, Planner, Executor, Undo
├── crates/cf-cli/       — CLI-Binary (clap)
├── src-tauri/           — Tauri v2 Backend + IPC-Commands
└── frontend/            — React + TypeScript + Tailwind
```

---

<div align="right">
  <sub>by</sub><br/>
  <img src="RayStudio.png" alt="RayStudio" width="70"/>
</div>

**Author:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** Framework Preview · **Last Updated:** Juni 2026
