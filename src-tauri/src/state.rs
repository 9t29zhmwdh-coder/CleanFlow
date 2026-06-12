use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use cf_core::{
    models::{OrganizePlan, ScannedFile, ScanStatus},
    Journal, Store,
};

pub struct ScanSession {
    pub status: ScanStatus,
    pub files: Vec<ScannedFile>,
    pub plan: Option<OrganizePlan>,
}

pub struct AppState {
    pub store: Arc<Store>,
    pub journal: Arc<Journal>,
    pub scans: Mutex<HashMap<String, ScanSession>>,
    pub data_dir: PathBuf,
}

impl AppState {
    pub fn new(data_dir: PathBuf) -> anyhow::Result<Self> {
        std::fs::create_dir_all(&data_dir)?;
        let store = Store::open(&data_dir.join("store"))?;
        let journal = Journal::open(&data_dir.join("journal"))?;
        Ok(Self {
            store: Arc::new(store),
            journal: Arc::new(journal),
            scans: Mutex::new(HashMap::new()),
            data_dir,
        })
    }
}
