import { ArrowRight, Trash2, Tag, FolderPlus } from "lucide-react";
import type { PlannedAction } from "../../lib/tauri";
import { basename, reasonLabel } from "../../lib/format";
import clsx from "clsx";

interface Props {
  action: PlannedAction;
  selected: boolean;
  onToggle: () => void;
}

export function ActionItem({ action, selected, onToggle }: Props) {
  const { kind } = action.action;

  return (
    <div
      onClick={onToggle}
      className={clsx(
        "flex cursor-pointer items-center gap-3 rounded-lg border px-3 py-2 text-sm transition-colors",
        selected
          ? "border-brand-200 bg-brand-50"
          : "border-transparent bg-white hover:bg-gray-50"
      )}
    >
      {/* Checkbox */}
      <div className={clsx(
        "h-4 w-4 shrink-0 rounded border-2",
        selected ? "border-brand-500 bg-brand-500" : "border-gray-300"
      )} />

      {/* Icon */}
      <ActionIcon kind={kind} />

      {/* Description */}
      <div className="min-w-0 flex-1">
        <ActionDescription action={action.action} />
      </div>

      {/* Reason badge */}
      <ReasonBadge reason={action.reason} />
    </div>
  );
}

function ActionIcon({ kind }: { kind: string }) {
  const cls = "shrink-0 text-gray-400";
  switch (kind) {
    case "Move":            return <ArrowRight size={16} className={cls} />;
    case "Trash":           return <Trash2 size={16} className="shrink-0 text-red-400" />;
    case "Tag":             return <Tag size={16} className={cls} />;
    case "CreateDirectory": return <FolderPlus size={16} className={cls} />;
    default:                return <ArrowRight size={16} className={cls} />;
  }
}

function ActionDescription({ action }: { action: PlannedAction["action"] }) {
  switch (action.kind) {
    case "Move":
      return (
        <span className="truncate">
          <span className="font-medium">{basename(action.from)}</span>
          <span className="mx-1 text-gray-400">→</span>
          <span className="text-gray-500">{action.to}</span>
        </span>
      );
    case "Rename":
      return (
        <span className="truncate">
          <span className="font-medium">{basename(action.path)}</span>
          <span className="mx-1 text-gray-400">→</span>
          <span className="text-gray-500">{action.new_name}</span>
        </span>
      );
    case "Trash":
      return <span className="truncate font-medium text-red-600">{basename(action.path)}</span>;
    case "Tag":
      return (
        <span>
          Tag: {action.tags.map(t => (
            <span key={t} className="ml-1 rounded bg-gray-100 px-1.5 py-0.5 text-xs">{t}</span>
          ))}
        </span>
      );
    case "CreateDirectory":
      return <span className="truncate text-gray-500">Create: {action.path}</span>;
  }
}

function ReasonBadge({ reason }: { reason: PlannedAction["reason"] }) {
  const colorMap: Record<string, string> = {
    RuleMatch:      "bg-blue-100 text-blue-700",
    AiSuggestion:   "bg-purple-100 text-purple-700",
    DuplicateGroup: "bg-orange-100 text-orange-700",
    JunkDetected:   "bg-red-100 text-red-700",
    OldVersion:     "bg-yellow-100 text-yellow-700",
  };
  return (
    <span className={clsx(
      "shrink-0 rounded px-2 py-0.5 text-xs font-medium",
      colorMap[reason.type] ?? "bg-gray-100 text-gray-600"
    )}>
      {reasonLabel(reason as any)}
    </span>
  );
}
