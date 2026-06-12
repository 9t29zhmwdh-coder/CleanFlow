use sled::Db;

use crate::models::HistoryEntry;

pub struct Journal {
    db: Db,
}

impl Journal {
    pub fn open(path: &std::path::Path) -> anyhow::Result<Self> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    pub fn push(&self, entry: &HistoryEntry) -> anyhow::Result<()> {
        let key = format!("{:020}:{}", entry.executed_at, entry.id);
        let val = serde_json::to_vec(entry)?;
        self.db.insert(key.as_bytes(), val)?;
        Ok(())
    }

    pub fn pop(&self) -> anyhow::Result<Option<HistoryEntry>> {
        if let Some((key, val)) = self.db.last()? {
            let entry: HistoryEntry = serde_json::from_slice(&val)?;
            self.db.remove(key)?;
            return Ok(Some(entry));
        }
        Ok(None)
    }

    pub fn get(&self, id: &str) -> anyhow::Result<Option<HistoryEntry>> {
        for item in self.db.iter() {
            let (_, val) = item?;
            let entry: HistoryEntry = serde_json::from_slice(&val)?;
            if entry.id == id {
                return Ok(Some(entry));
            }
        }
        Ok(None)
    }

    pub fn remove(&self, id: &str) -> anyhow::Result<()> {
        let key_to_del: Option<sled::IVec> = self
            .db
            .iter()
            .filter_map(|r| r.ok())
            .find(|(_, v)| {
                serde_json::from_slice::<HistoryEntry>(v)
                    .map(|e| e.id == id)
                    .unwrap_or(false)
            })
            .map(|(k, _)| k);

        if let Some(key) = key_to_del {
            self.db.remove(key)?;
        }
        Ok(())
    }

    pub fn list(&self, limit: usize) -> anyhow::Result<Vec<HistoryEntry>> {
        let mut entries: Vec<HistoryEntry> = self
            .db
            .iter()
            .rev()
            .take(limit)
            .filter_map(|r| r.ok())
            .filter_map(|(_, v)| serde_json::from_slice(&v).ok())
            .collect();
        entries.sort_by(|a, b| b.executed_at.cmp(&a.executed_at));
        Ok(entries)
    }
}
