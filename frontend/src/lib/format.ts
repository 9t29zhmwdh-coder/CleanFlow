export function formatBytes(bytes: number): string {
  if (bytes >= 1e9) return `${(bytes / 1e9).toFixed(1)} GB`;
  if (bytes >= 1e6) return `${(bytes / 1e6).toFixed(1)} MB`;
  if (bytes >= 1e3) return `${(bytes / 1e3).toFixed(1)} KB`;
  return `${bytes} B`;
}

export function formatDate(ts: number): string {
  return new Date(ts * 1000).toLocaleDateString(undefined, {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
}

export function formatRelative(ts: number): string {
  const diff = Date.now() / 1000 - ts;
  if (diff < 60)        return "just now";
  if (diff < 3600)      return `${Math.floor(diff / 60)}m ago`;
  if (diff < 86400)     return `${Math.floor(diff / 3600)}h ago`;
  if (diff < 86400 * 7) return `${Math.floor(diff / 86400)}d ago`;
  return formatDate(ts);
}

export function basename(path: string): string {
  return path.split(/[/\\]/).pop() ?? path;
}

export function reasonLabel(reason: { type: string; [k: string]: unknown }): string {
  switch (reason.type) {
    case "RuleMatch":      return `Rule: ${reason.rule_name}`;
    case "AiSuggestion":   return `AI: ${reason.explanation}`;
    case "DuplicateGroup": return `Duplicate`;
    case "JunkDetected":   return `Junk: ${reason.junk_type}`;
    case "OldVersion":     return `Old version`;
    default:               return reason.type;
  }
}
