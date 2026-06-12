import { useState } from "react";
import { Play, RotateCcw, CheckSquare, Square } from "lucide-react";
import { useScanStore } from "../../stores/scanStore";
import type { PlannedAction } from "../../lib/tauri";
import { formatBytes, reasonLabel } from "../../lib/format";
import { ActionItem } from "./ActionItem";

export function ActionPreview() {
  const { plan, isLoading, executePlan, executeCleanflow, reset } = useScanStore();
  const [selected, setSelected] = useState<Set<string>>(
    () => new Set(plan?.actions.filter(a => a.selected).map(a => a.id) ?? [])
  );

  if (!plan) return null;

  const stats = plan.stats;
  const toggle = (id: string) => {
    setSelected(prev => {
      const next = new Set(prev);
      next.has(id) ? next.delete(id) : next.add(id);
      return next;
    });
  };

  const toggleAll = () => {
    if (selected.size === plan.actions.length) {
      setSelected(new Set());
    } else {
      setSelected(new Set(plan.actions.map(a => a.id)));
    }
  };

  const handleExecute = async () => {
    await executePlan(Array.from(selected));
    reset();
  };

  const handleCleanflow = async () => {
    await executeCleanflow();
    reset();
  };

  return (
    <div className="flex flex-col gap-4 p-6">
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-bold text-gray-900">CleanFlow Plan</h2>
        <button onClick={reset} className="text-sm text-gray-400 hover:text-gray-600">
          ← New Scan
        </button>
      </div>

      {/* Stats bar */}
      <div className="grid grid-cols-5 gap-3">
        {[
          { label: "Actions",    value: stats.files_affected },
          { label: "Freed",      value: formatBytes(stats.bytes_freed) },
          { label: "Duplicates", value: stats.duplicates_found },
          { label: "Junk",       value: stats.junk_found },
          { label: "AI",         value: stats.ai_suggestions },
        ].map(s => (
          <div key={s.label} className="rounded-lg bg-gray-50 p-3 text-center">
            <div className="text-lg font-bold text-gray-900">{s.value}</div>
            <div className="text-xs text-gray-500">{s.label}</div>
          </div>
        ))}
      </div>

      {/* One-click CleanFlow */}
      <button
        onClick={handleCleanflow}
        disabled={isLoading}
        className="flex items-center justify-center gap-2 rounded-xl bg-brand-600 py-3 text-base font-semibold text-white shadow-sm hover:bg-brand-700 disabled:opacity-50"
      >
        <Play size={18} />
        CleanFlow — Execute All ({plan.actions.filter(a => a.selected).length} actions)
      </button>

      {/* Action list header */}
      <div className="flex items-center justify-between border-b border-gray-200 pb-2">
        <button onClick={toggleAll} className="flex items-center gap-2 text-sm text-gray-500 hover:text-gray-700">
          {selected.size === plan.actions.length ? <CheckSquare size={16} /> : <Square size={16} />}
          {selected.size} / {plan.actions.length} selected
        </button>
        <button
          onClick={handleExecute}
          disabled={selected.size === 0 || isLoading}
          className="flex items-center gap-2 rounded-lg border border-brand-500 px-4 py-1.5 text-sm font-medium text-brand-600 hover:bg-brand-50 disabled:opacity-50"
        >
          <Play size={14} />
          Execute selected
        </button>
      </div>

      {/* Action items */}
      <div className="flex flex-col gap-1 overflow-y-auto max-h-[480px]">
        {plan.actions.map(action => (
          <ActionItem
            key={action.id}
            action={action}
            selected={selected.has(action.id)}
            onToggle={() => toggle(action.id)}
          />
        ))}
      </div>
    </div>
  );
}
