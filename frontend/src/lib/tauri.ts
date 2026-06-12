import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

// ── Types (mirrors Rust models) ───────────────────────────────────────────────

export type ScanPhase =
  | "Idle" | "Walking" | "Analyzing" | "AiClassifying" | "Planning" | "Done"
  | "Cancelled" | { Error: string };

export interface ScanStatus {
  scan_id: string;
  phase: ScanPhase;
  files_found: number;
  files_analyzed: number;
  ai_requests_made: number;
  elapsed_ms: number;
  errors: string[];
}

export interface FileFlags {
  is_duplicate: boolean;
  is_junk: boolean;
  is_zombie: boolean;
  is_old_version: boolean;
}

export interface AiClassification {
  category: string;
  confidence: number;
  tags: string[];
  summary: string | null;
  source: "Claude" | "Ollama" | "RuleBased";
}

export interface ScannedFile {
  id: string;
  path: string;
  size_bytes: number;
  mime_type: string;
  modified_at: number;
  accessed_at: number | null;
  ai_classification: AiClassification | null;
  series_group: string | null;
  flags: FileFlags;
}

export interface PlannedAction {
  id: string;
  action: Action;
  reason: ActionReason;
  priority: number;
  selected: boolean;
}

export type Action =
  | { kind: "Move";            file_id: string; from: string; to: string }
  | { kind: "Rename";          file_id: string; path: string; new_name: string }
  | { kind: "Trash";           file_id: string; path: string }
  | { kind: "Tag";             file_id: string; tags: string[] }
  | { kind: "CreateDirectory"; path: string };

export type ActionReason =
  | { type: "RuleMatch";      rule_id: string; rule_name: string }
  | { type: "AiSuggestion";   explanation: string }
  | { type: "DuplicateGroup"; group_id: string }
  | { type: "JunkDetected";   junk_type: string }
  | { type: "OldVersion";     canonical_path: string };

export interface PlanStats {
  files_affected: number;
  bytes_freed: number;
  duplicates_found: number;
  junk_found: number;
  ai_suggestions: number;
  rule_matches: number;
}

export interface OrganizePlan {
  id: string;
  created_at: number;
  source_directory: string;
  actions: PlannedAction[];
  stats: PlanStats;
}

export interface ExecutionResult {
  history_id: string;
  executed_count: number;
  error_count: number;
  errors: string[];
}

export interface DuplicateGroup {
  group_id: string;
  files: ScannedFile[];
  total_wasted_bytes: number;
}

export interface JunkFile {
  file: ScannedFile;
  junk_type: string;
  reason: string;
}

export interface AppSettings {
  ai_backend: "Claude" | "Ollama" | "RuleBasedOnly";
  ollama_url: string;
  max_file_size_for_ai: number;
  zombie_threshold_days: number;
  auto_scan_paths: string[];
}

export interface AiBackendStatus {
  claude_available: boolean;
  ollama_available: boolean;
  active_backend: string;
}

// ── Tauri Command Wrappers ────────────────────────────────────────────────────

export const api = {
  // Scan
  scanDirectory: (path: string, options?: object) =>
    invoke<string>("scan_directory", { path, options }),
  getScanStatus: (scanId: string) =>
    invoke<ScanStatus>("get_scan_status", { scanId }),
  getScannedFiles: (scanId: string) =>
    invoke<ScannedFile[]>("get_scanned_files", { scanId }),
  cancelScan: (scanId: string) =>
    invoke<void>("cancel_scan", { scanId }),

  // Organize
  previewPlan: (scanId: string) =>
    invoke<OrganizePlan>("preview_plan", { scanId }),
  executePlan: (planId: string, selectedIds?: string[]) =>
    invoke<ExecutionResult>("execute_plan", { planId, selectedIds }),
  executeCleanflow: (scanId: string) =>
    invoke<ExecutionResult>("execute_cleanflow", { scanId }),

  // Cleanup
  findDuplicates: (scanId: string) =>
    invoke<DuplicateGroup[]>("find_duplicates", { scanId }),
  findJunk: (scanId: string) =>
    invoke<JunkFile[]>("find_junk", { scanId }),

  // Undo
  listHistory: (limit = 20) =>
    invoke<object[]>("list_history", { limit }),
  undoLast: () =>
    invoke<object>("undo_last"),

  // System
  getSettings: () => invoke<AppSettings>("get_settings"),
  saveSettings: (settings: AppSettings) => invoke<void>("save_settings", { settings }),
  checkAiBackend: () => invoke<AiBackendStatus>("check_ai_backend"),
  openInFinder: (path: string) => invoke<void>("open_in_finder", { path }),
  saveApiKey: (service: string, key: string) => invoke<void>("save_api_key", { service, key }),
  hasApiKey: (service: string) => invoke<boolean>("has_api_key", { service }),
};

// ── Event helpers ─────────────────────────────────────────────────────────────

export const listenScanStatus = (
  scanId: string,
  cb: (status: ScanStatus) => void
) => listen<ScanStatus>(`scan://status/${scanId}`, (e) => cb(e.payload));
