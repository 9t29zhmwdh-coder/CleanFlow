use std::path::Path;

use sled::Db;

use crate::models::{AppSettings, Rule, ScannedFile, ScanStatus};

pub struct Store {
    db: Db,
}

const TREE_SCANS:    &[u8] = b"scans";
const TREE_FILES:    &[u8] = b"files";
const TREE_RULES:    &[u8] = b"rules";
const KEY_SETTINGS:  &[u8] = b"settings";

impl Store {
    pub fn open(path: &Path) -> anyhow::Result<Self> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    // ── Scans ────────────────────────────────────────────────

    pub fn save_scan_status(&self, status: &ScanStatus) -> anyhow::Result<()> {
        let tree = self.db.open_tree(TREE_SCANS)?;
        tree.insert(status.scan_id.as_bytes(), serde_json::to_vec(status)?)?;
        Ok(())
    }

    pub fn get_scan_status(&self, scan_id: &str) -> anyhow::Result<Option<ScanStatus>> {
        let tree = self.db.open_tree(TREE_SCANS)?;
        Ok(tree
            .get(scan_id.as_bytes())?
            .and_then(|v| serde_json::from_slice(&v).ok()))
    }

    // ── Files ────────────────────────────────────────────────

    pub fn save_files(&self, scan_id: &str, files: &[ScannedFile]) -> anyhow::Result<()> {
        let tree = self.db.open_tree(TREE_FILES)?;
        tree.insert(scan_id.as_bytes(), serde_json::to_vec(files)?)?;
        Ok(())
    }

    pub fn get_files(&self, scan_id: &str) -> anyhow::Result<Vec<ScannedFile>> {
        let tree = self.db.open_tree(TREE_FILES)?;
        Ok(tree
            .get(scan_id.as_bytes())?
            .and_then(|v| serde_json::from_slice(&v).ok())
            .unwrap_or_default())
    }

    // ── Rules ────────────────────────────────────────────────

    pub fn save_rule(&self, rule: &Rule) -> anyhow::Result<()> {
        let tree = self.db.open_tree(TREE_RULES)?;
        tree.insert(rule.id.as_bytes(), serde_json::to_vec(rule)?)?;
        Ok(())
    }

    pub fn delete_rule(&self, rule_id: &str) -> anyhow::Result<()> {
        let tree = self.db.open_tree(TREE_RULES)?;
        tree.remove(rule_id.as_bytes())?;
        Ok(())
    }

    pub fn list_rules(&self) -> anyhow::Result<Vec<Rule>> {
        let tree = self.db.open_tree(TREE_RULES)?;
        let rules: Vec<Rule> = tree
            .iter()
            .filter_map(|r| r.ok())
            .filter_map(|(_, v)| serde_json::from_slice(&v).ok())
            .collect();
        Ok(rules)
    }

    // ── Settings ─────────────────────────────────────────────

    pub fn save_settings(&self, settings: &AppSettings) -> anyhow::Result<()> {
        self.db
            .insert(KEY_SETTINGS, serde_json::to_vec(settings)?)?;
        Ok(())
    }

    pub fn get_settings(&self) -> anyhow::Result<AppSettings> {
        Ok(self
            .db
            .get(KEY_SETTINGS)?
            .and_then(|v| serde_json::from_slice(&v).ok())
            .unwrap_or_default())
    }
}
