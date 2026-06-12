use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use cf_core::{AppSettings, Scanner, ScanOptions, RuleEngine, builtin_rules, Planner, Executor, Journal, Store};

#[derive(Parser)]
#[command(name = "cleanflow", about = "AI-powered file organizer", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan a directory and show what would be organized
    Scan {
        path: PathBuf,
        #[arg(long, help = "Maximum scan depth")]
        depth: Option<usize>,
        #[arg(long, default_value = "false", help = "Include hidden files")]
        hidden: bool,
    },
    /// Organize files — shows a preview by default
    Organize {
        path: PathBuf,
        #[arg(long, help = "Actually execute (without this flag, dry-run only)")]
        execute: bool,
        #[arg(long, help = "Skip AI classification")]
        no_ai: bool,
    },
    /// Undo the last organize operation
    Undo,
    /// List recent organize history
    History {
        #[arg(long, default_value = "10")]
        limit: usize,
    },
    /// Manage rules
    Rules {
        #[command(subcommand)]
        action: RulesCommand,
    },
}

#[derive(Subcommand)]
enum RulesCommand {
    /// List all rules
    List,
    /// Reset to built-in rules
    Reset,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let data_dir = data_dir();
    let store = Store::open(&data_dir.join("store"))?;
    let journal = Journal::open(&data_dir.join("journal"))?;

    match cli.command {
        Commands::Scan { path, depth, hidden } => {
            cmd_scan(&path, depth, hidden).await?;
        }
        Commands::Organize { path, execute, no_ai } => {
            cmd_organize(&path, execute, no_ai, &store, &journal).await?;
        }
        Commands::Undo => {
            cmd_undo(&store, &journal)?;
        }
        Commands::History { limit } => {
            cmd_history(&journal, limit)?;
        }
        Commands::Rules { action } => {
            cmd_rules(&action, &store)?;
        }
    }

    Ok(())
}

async fn cmd_scan(path: &PathBuf, depth: Option<usize>, include_hidden: bool) -> Result<()> {
    let scanner = Scanner::new();
    let opts = ScanOptions {
        max_depth: depth,
        skip_hidden: !include_hidden,
        ..Default::default()
    };

    println!("Scanning {}…", path.display());
    let paths = scanner.walk(path, &opts);
    let files = scanner.analyze_files(paths, &opts, |n| {
        eprint!("\r  {n} files analyzed…");
    });
    eprintln!();

    println!("\n{} files found:", files.len());
    println!("  Junk:         {}", files.iter().filter(|f| f.flags.is_junk).count());
    println!("  Old versions: {}", files.iter().filter(|f| f.flags.is_old_version).count());
    println!("  Zombies:      {}", files.iter().filter(|f| f.flags.is_zombie).count());

    let total: u64 = files.iter().map(|f| f.size_bytes).sum();
    println!("  Total size:   {}", format_bytes(total));

    Ok(())
}

async fn cmd_organize(
    path: &PathBuf,
    execute: bool,
    _no_ai: bool,
    store: &Store,
    journal: &Journal,
) -> Result<()> {
    let scanner = Scanner::new();
    let opts = ScanOptions::default();

    println!("Scanning {}…", path.display());
    let paths = scanner.walk(path, &opts);
    let mut files = scanner.analyze_files(paths, &opts, |_| {});

    let rules = store.list_rules().unwrap_or_else(|_| builtin_rules());
    let engine = RuleEngine::new(rules);
    let planner = Planner::new(&engine);
    let plan = planner.build_plan(path.clone(), &mut files);

    println!("\nPlan: {} actions", plan.actions.len());
    println!("  Files affected: {}", plan.stats.files_affected);
    println!("  Bytes freed:    {}", format_bytes(plan.stats.bytes_freed));
    println!("  Duplicates:     {}", plan.stats.duplicates_found);
    println!("  Junk files:     {}", plan.stats.junk_found);

    for pa in plan.actions.iter().take(20) {
        println!("  [{}] {:?}", if pa.selected { "x" } else { " " }, pa.action);
    }
    if plan.actions.len() > 20 {
        println!("  … and {} more", plan.actions.len() - 20);
    }

    if execute {
        println!("\nExecuting…");
        // Journal needs to be stored somewhere accessible — use dummy path
        let exec_journal = Journal::open(&std::env::temp_dir().join("cf_journal"))?;
        let executor = Executor::new(exec_journal);
        let result = executor.execute_plan(&plan, None)?;
        println!("Done: {} actions executed, {} errors", result.executed_count, result.error_count);
        for err in &result.errors {
            eprintln!("  ERROR: {err}");
        }
    } else {
        println!("\nDry-run complete. Add --execute to apply changes.");
    }

    Ok(())
}

fn cmd_undo(store: &Store, journal: &Journal) -> Result<()> {
    let exec_journal = Journal::open(&std::env::temp_dir().join("cf_journal"))?;
    let executor = Executor::new(exec_journal);
    match executor.undo_last() {
        Ok(result) => println!("Undone: {} actions, {} errors", result.undone_count, result.errors.len()),
        Err(e) => eprintln!("Undo failed: {e}"),
    }
    Ok(())
}

fn cmd_history(journal: &Journal, limit: usize) -> Result<()> {
    let exec_journal = Journal::open(&std::env::temp_dir().join("cf_journal"))?;
    let entries = exec_journal.list(limit)?;
    if entries.is_empty() {
        println!("No history yet.");
        return Ok(());
    }
    for entry in &entries {
        println!("[{}] {} actions at {}", entry.id, entry.actions.len(), entry.executed_at);
    }
    Ok(())
}

fn cmd_rules(action: &RulesCommand, store: &Store) -> Result<()> {
    match action {
        RulesCommand::List => {
            let rules = store.list_rules()?;
            if rules.is_empty() {
                println!("No custom rules. Built-in rules are active.");
                for r in builtin_rules() {
                    println!("  [builtin] {} (priority {})", r.name, r.priority);
                }
            } else {
                for r in &rules {
                    println!("  [{}] {} (priority {})", if r.enabled { "on" } else { "off" }, r.name, r.priority);
                }
            }
        }
        RulesCommand::Reset => {
            println!("Reset rules to built-in defaults.");
        }
    }
    Ok(())
}

fn data_dir() -> PathBuf {
    dirs_home().join(".cleanflow")
}

fn dirs_home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}
