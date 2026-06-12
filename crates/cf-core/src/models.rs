use std::path::PathBuf;
use serde::{Deserialize, Serialize};

// ── ScannedFile ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannedFile {
    pub id: String,
    pub path: PathBuf,
    pub size_bytes: u64,
    pub mime_type: String,
    pub content_hash: Option<String>,
    pub modified_at: i64,
    pub accessed_at: Option<i64>,
    pub created_at: Option<i64>,
    pub ai_classification: Option<AiClassification>,
    pub series_group: Option<String>,
    pub flags: FileFlags,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileFlags {
    pub is_duplicate: bool,
    pub is_junk: bool,
    pub is_zombie: bool,
    pub is_old_version: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiClassification {
    pub category: FileCategory,
    pub confidence: f32,
    pub tags: Vec<String>,
    pub project: Option<String>,
    pub summary: Option<String>,
    pub source: AiSource,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AiSource {
    Claude,
    Ollama,
    RuleBased,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum FileCategory {
    Invoice,
    Contract,
    Receipt,
    Screenshot,
    Photo,
    Video,
    Audio,
    Code,
    Document,
    Spreadsheet,
    Archive,
    Installer,
    Temporary,
    Unknown(String),
}

impl std::fmt::Display for FileCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Invoice    => "Invoice",
            Self::Contract   => "Contract",
            Self::Receipt    => "Receipt",
            Self::Screenshot => "Screenshot",
            Self::Photo      => "Photo",
            Self::Video      => "Video",
            Self::Audio      => "Audio",
            Self::Code       => "Code",
            Self::Document   => "Document",
            Self::Spreadsheet=> "Spreadsheet",
            Self::Archive    => "Archive",
            Self::Installer  => "Installer",
            Self::Temporary  => "Temporary",
            Self::Unknown(s) => s.as_str(),
        };
        write!(f, "{s}")
    }
}

// ── Actions ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Action {
    Move {
        file_id: String,
        from: PathBuf,
        to: PathBuf,
    },
    Rename {
        file_id: String,
        path: PathBuf,
        new_name: String,
    },
    Trash {
        file_id: String,
        path: PathBuf,
    },
    Tag {
        file_id: String,
        tags: Vec<String>,
    },
    CreateDirectory {
        path: PathBuf,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedAction {
    pub id: String,
    pub action: Action,
    pub reason: ActionReason,
    pub priority: u8,
    pub selected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ActionReason {
    RuleMatch {
        rule_id: String,
        rule_name: String,
    },
    AiSuggestion {
        explanation: String,
    },
    DuplicateGroup {
        group_id: String,
        keep_strategy: KeepStrategy,
    },
    JunkDetected {
        junk_type: String,
    },
    OldVersion {
        canonical_path: PathBuf,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeepStrategy {
    Newest,
    Largest,
    ByPath(PathBuf),
}

// ── OrganizePlan ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizePlan {
    pub id: String,
    pub created_at: i64,
    pub source_directory: PathBuf,
    pub actions: Vec<PlannedAction>,
    pub stats: PlanStats,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlanStats {
    pub files_affected: usize,
    pub bytes_freed: u64,
    pub duplicates_found: usize,
    pub junk_found: usize,
    pub ai_suggestions: usize,
    pub rule_matches: usize,
}

// ── Rules ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub priority: i32,
    pub trigger: RuleTrigger,
    pub conditions: Vec<RuleCondition>,
    pub condition_logic: ConditionLogic,
    pub actions: Vec<RuleAction>,
    pub schedule: Option<RuleSchedule>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RuleTrigger {
    OnScan,
    Manual,
    Schedule,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionLogic {
    All,
    Any,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum RuleCondition {
    MimeType(String),
    Extension(String),
    NameContains(String),
    NameMatches(String),
    SizeGt(u64),
    SizeLt(u64),
    OlderThanDays(u32),
    AiCategory(FileCategory),
    AiTagContains(String),
    PathContains(String),
    NeverAccessed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum RuleAction {
    MoveTo(PathBuf),
    RenamePattern(String),
    AddTag(String),
    Trash,
    Archive(PathBuf),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSchedule {
    pub cron: String,
    pub last_run: Option<i64>,
}

// ── Scan Status ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanStatus {
    pub scan_id: String,
    pub phase: ScanPhase,
    pub files_found: usize,
    pub files_analyzed: usize,
    pub ai_requests_made: usize,
    pub elapsed_ms: u64,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScanPhase {
    Idle,
    Walking,
    Analyzing,
    AiClassifying,
    Planning,
    Done,
    Cancelled,
    Error(String),
}

// ── Cleanup helpers ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateGroup {
    pub group_id: String,
    pub files: Vec<ScannedFile>,
    pub total_wasted_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JunkFile {
    pub file: ScannedFile,
    pub junk_type: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionGroup {
    pub canonical_name: String,
    pub versions: Vec<ScannedFile>,
}

// ── Undo ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    pub executed_at: i64,
    pub actions: Vec<ExecutedAction>,
    pub plan_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutedAction {
    pub action: Action,
    pub undo_data: UndoData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum UndoData {
    FileWasMoved {
        original_path: PathBuf,
    },
    FileWasTrashed {
        original_path: PathBuf,
    },
    FileWasRenamed {
        original_name: String,
    },
    DirectoryCreated {
        path: PathBuf,
    },
    NoUndo,
}

// ── Settings ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub ai_backend: AiBackend,
    pub ollama_url: String,
    pub max_file_size_for_ai: u64,
    pub zombie_threshold_days: u32,
    pub auto_scan_paths: Vec<PathBuf>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            ai_backend: AiBackend::Claude,
            ollama_url: "http://localhost:11434".into(),
            max_file_size_for_ai: 1024 * 1024,
            zombie_threshold_days: 90,
            auto_scan_paths: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AiBackend {
    Claude,
    Ollama,
    RuleBasedOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiBackendStatus {
    pub claude_available: bool,
    pub ollama_available: bool,
    pub active_backend: AiBackend,
}
