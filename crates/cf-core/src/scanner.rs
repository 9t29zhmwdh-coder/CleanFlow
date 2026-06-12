use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use rayon::prelude::*;
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

use crate::models::{ScannedFile, FileFlags};

pub struct ScanOptions {
    pub follow_links: bool,
    pub max_depth: Option<usize>,
    pub skip_hidden: bool,
    pub min_size_bytes: u64,
    pub max_size_bytes: Option<u64>,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            follow_links: false,
            max_depth: None,
            skip_hidden: true,
            min_size_bytes: 0,
            max_size_bytes: None,
        }
    }
}

pub struct Scanner {
    cancelled: Arc<AtomicBool>,
    pub files_found: Arc<AtomicUsize>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
            files_found: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    pub fn walk(&self, root: &Path, opts: &ScanOptions) -> Vec<PathBuf> {
        let mut walker = WalkDir::new(root)
            .follow_links(opts.follow_links);

        if let Some(depth) = opts.max_depth {
            walker = walker.max_depth(depth);
        }

        walker
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                if opts.skip_hidden {
                    !e.file_name().to_string_lossy().starts_with('.')
                } else {
                    true
                }
            })
            .map(|e| e.into_path())
            .collect()
    }

    pub fn analyze_files(
        &self,
        paths: Vec<PathBuf>,
        opts: &ScanOptions,
        progress_cb: impl Fn(usize) + Sync,
    ) -> Vec<ScannedFile> {
        let counter = Arc::new(AtomicUsize::new(0));
        let cancelled = self.cancelled.clone();

        paths
            .par_iter()
            .filter_map(|path| {
                if cancelled.load(Ordering::Relaxed) {
                    return None;
                }
                let n = counter.fetch_add(1, Ordering::Relaxed);
                if n % 50 == 0 {
                    progress_cb(n);
                }
                analyze_single(path, opts).ok()
            })
            .collect()
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}

fn analyze_single(path: &Path, opts: &ScanOptions) -> anyhow::Result<ScannedFile> {
    let meta = std::fs::metadata(path)?;
    let size = meta.len();

    if size < opts.min_size_bytes {
        anyhow::bail!("too small");
    }
    if let Some(max) = opts.max_size_bytes {
        if size > max {
            anyhow::bail!("too large");
        }
    }

    let mime_type = detect_mime(path);

    let modified_at = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    let accessed_at = meta
        .accessed()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64);

    let created_at = meta
        .created()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64);

    let id = file_id(path, size);
    let flags = detect_flags(path, accessed_at);

    Ok(ScannedFile {
        id,
        path: path.to_owned(),
        size_bytes: size,
        mime_type,
        content_hash: None,
        modified_at,
        accessed_at,
        created_at,
        ai_classification: None,
        series_group: detect_series_group(path),
        flags,
    })
}

fn detect_mime(path: &Path) -> String {
    // Try magic bytes first
    if let Ok(bytes) = read_first_bytes(path, 16) {
        if let Some(kind) = infer::get(&bytes) {
            return kind.mime_type().to_string();
        }
    }
    // Fallback: extension-based
    match path.extension().and_then(|e| e.to_str()) {
        Some("pdf")  => "application/pdf",
        Some("zip")  => "application/zip",
        Some("dmg")  => "application/x-apple-diskimage",
        Some("py")   => "text/x-python",
        Some("rs")   => "text/x-rust",
        Some("ts" | "tsx") => "text/typescript",
        Some("js" | "jsx") => "text/javascript",
        Some("md")   => "text/markdown",
        Some("txt")  => "text/plain",
        Some("csv")  => "text/csv",
        Some("json") => "application/json",
        Some("yaml" | "yml") => "text/yaml",
        Some("toml") => "text/toml",
        Some("log")  => "text/x-log",
        Some("tmp")  => "application/x-temporary",
        _            => "application/octet-stream",
    }
    .to_string()
}

fn read_first_bytes(path: &Path, n: usize) -> anyhow::Result<Vec<u8>> {
    use std::io::Read;
    let mut buf = vec![0u8; n];
    let mut f = std::fs::File::open(path)?;
    let read = f.read(&mut buf)?;
    buf.truncate(read);
    Ok(buf)
}

pub fn content_hash(path: &Path) -> anyhow::Result<String> {
    use std::io::Read;
    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 65536];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 { break; }
        hasher.update(&buf[..n]);
    }
    Ok(hex::encode(hasher.finalize()))
}

fn file_id(path: &Path, size: u64) -> String {
    let mut h = Sha256::new();
    h.update(path.to_string_lossy().as_bytes());
    h.update(size.to_le_bytes());
    hex::encode(h.finalize())[..16].to_string()
}

fn detect_flags(path: &Path, accessed_at: Option<i64>) -> FileFlags {
    let name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase();
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let is_junk = matches!(
        ext.as_str(),
        "dmg" | "pkg" | "tmp" | "cache" | "log"
    ) || name == ".ds_store"
        || name.starts_with("._")
        || name == "thumbs.db"
        || name == "desktop.ini";

    let is_old_version = {
        let n = name.as_str();
        n.contains(" copy") || n.contains(" v1") || n.contains(" v2")
            || n.contains("_backup") || n.contains("_old") || n.contains(" (1)")
            || n.contains(" (2)")
    };

    // Zombie: never accessed or not in 90 days
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let threshold = 90 * 24 * 3600;
    let is_zombie = accessed_at
        .map(|a| now - a > threshold)
        .unwrap_or(false);

    FileFlags {
        is_duplicate: false,
        is_junk,
        is_zombie,
        is_old_version,
    }
}

fn detect_series_group(path: &Path) -> Option<String> {
    let name = path.file_stem()?.to_string_lossy();
    // Screenshot_YYYYMMDD → group "Screenshot"
    let re = regex::Regex::new(r"^([A-Za-z_\-]+?)[\s_\-]?\d{4,}").ok()?;
    let caps = re.captures(&name)?;
    Some(caps.get(1)?.as_str().to_lowercase())
}

// ── Deduplication ────────────────────────────────────────────────────────────

pub fn find_duplicates(files: &mut Vec<ScannedFile>) -> Vec<crate::models::DuplicateGroup> {
    use std::collections::HashMap;

    // Group by size first (cheap)
    let mut by_size: HashMap<u64, Vec<usize>> = HashMap::new();
    for (i, f) in files.iter().enumerate() {
        by_size.entry(f.size_bytes).or_default().push(i);
    }

    let mut groups: HashMap<String, Vec<usize>> = HashMap::new();

    for (_, idxs) in by_size.iter().filter(|(_, v)| v.len() > 1) {
        // Compute hashes for candidates
        for &i in idxs {
            let path = files[i].path.clone();
            if let Ok(hash) = content_hash(&path) {
                files[i].content_hash = Some(hash.clone());
                groups.entry(hash).or_default().push(i);
            }
        }
    }

    let mut result = vec![];
    for (hash, idxs) in groups.iter().filter(|(_, v)| v.len() > 1) {
        let group_files: Vec<ScannedFile> = idxs.iter().map(|&i| {
            files[i].flags.is_duplicate = true;
            files[i].clone()
        }).collect();

        let wasted = group_files[0].size_bytes * (group_files.len() as u64 - 1);
        result.push(crate::models::DuplicateGroup {
            group_id: hash[..12].to_string(),
            files: group_files,
            total_wasted_bytes: wasted,
        });
    }

    result
}
