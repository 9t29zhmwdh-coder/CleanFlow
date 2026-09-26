//! The app opens the journal once at startup and hands clones to every
//! command. These tests pin that down, together with honest undo reporting.

use cf_core::{
    Action, ActionReason, ExecutedAction, Executor, HistoryEntry, Journal, OrganizePlan,
    PlanStats, PlannedAction, UndoData,
};

fn move_plan(from: std::path::PathBuf, to: std::path::PathBuf) -> OrganizePlan {
    OrganizePlan {
        id: "plan-1".into(),
        created_at: 0,
        source_directory: from.parent().unwrap().to_path_buf(),
        actions: vec![PlannedAction {
            id: "a1".into(),
            action: Action::Move { file_id: "f1".into(), from, to },
            reason: ActionReason::JunkDetected { junk_type: "test".into() },
            priority: 1,
            selected: true,
        }],
        stats: PlanStats::default(),
    }
}

#[test]
fn a_second_open_of_the_same_journal_fails() {
    // The bug: every command opened the journal again and got this error.
    let dir = tempfile::tempdir().unwrap();
    let _held = Journal::open(&dir.path().join("journal")).unwrap();
    assert!(Journal::open(&dir.path().join("journal")).is_err());
}

#[test]
fn executor_on_a_clone_executes_and_undoes_while_the_original_stays_open() {
    let dir = tempfile::tempdir().unwrap();
    let journal = Journal::open(&dir.path().join("journal")).unwrap();
    let from = dir.path().join("a.txt");
    let to = dir.path().join("sorted").join("a.txt");
    std::fs::write(&from, "x").unwrap();

    let result = Executor::new(journal.clone()).execute_plan(&move_plan(from.clone(), to.clone()), None).unwrap();
    assert_eq!(result.executed_count, 1);
    assert!(to.exists());
    // The original handle sees what the clone wrote.
    assert_eq!(journal.list(10).unwrap().len(), 1);

    let undo = Executor::new(journal.clone()).undo_last().unwrap();
    assert_eq!(undo.undone_count, 1);
    assert!(from.exists() && !to.exists());
}

#[test]
fn undoing_a_trashed_file_is_reported_not_counted() {
    let dir = tempfile::tempdir().unwrap();
    let journal = Journal::open(&dir.path().join("journal")).unwrap();
    let path = dir.path().join("gone.tmp");
    journal
        .push(&HistoryEntry {
            id: "h1".into(),
            executed_at: 0,
            plan_id: "plan-1".into(),
            actions: vec![ExecutedAction {
                action: Action::Trash { file_id: "f1".into(), path: path.clone() },
                undo_data: UndoData::FileWasTrashed { original_path: path },
            }],
        })
        .unwrap();

    let undo = Executor::new(journal).undo_by_id("h1").unwrap();
    assert_eq!(undo.undone_count, 0);
    assert_eq!(undo.errors.len(), 1);
    assert!(undo.errors[0].contains("Put Back"));
}
