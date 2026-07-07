export function formatBytes(bytes: number): string {
  if (bytes >= 1e9) return `${(bytes / 1e9).toFixed(1)} GB`;
  if (bytes >= 1e6) return `${(bytes / 1e6).toFixed(1)} MB`;
  if (bytes >= 1e3) return `${(bytes / 1e3).toFixed(1)} KB`;
  return `${bytes} B`;
}

import { getLang, t } from "./i18n";

export function formatDate(ts: number): string {
  return new Date(ts * 1000).toLocaleDateString(getLang() === "de" ? "de-CH" : "en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
}

export function formatRelative(ts: number): string {
  const diff = Date.now() / 1000 - ts;
  if (diff < 60)        return t("justNow");
  if (diff < 3600)      return t("minutesAgo", { n: Math.floor(diff / 60) });
  if (diff < 86400)     return t("hoursAgo", { n: Math.floor(diff / 3600) });
  if (diff < 86400 * 7) return t("daysAgo", { n: Math.floor(diff / 86400) });
  return formatDate(ts);
}

export function basename(path: string): string {
  return path.split(/[/\\]/).pop() ?? path;
}

export function reasonLabel(reason: { type: string; [k: string]: unknown }): string {
  switch (reason.type) {
    case "RuleMatch":      return t("reasonRule", { name: String(reason.rule_name) });
    case "AiSuggestion":   return t("reasonAi", { explanation: String(reason.explanation) });
    case "DuplicateGroup": return t("reasonDuplicate");
    case "JunkDetected":   return t("reasonJunk", { type: String(reason.junk_type) });
    case "OldVersion":     return t("reasonOldVersion");
    default:               return reason.type;
  }
}
