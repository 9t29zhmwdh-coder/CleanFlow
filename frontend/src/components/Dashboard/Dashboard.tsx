import { useCallback, useEffect, useState } from "react";
import { FolderSearch, Settings } from "lucide-react";
import { useT } from "../../lib/i18n";
import { api, type HistoryEntry, type UndoResult } from "../../lib/tauri";

interface Props {
  onNavigate: (view: "scan" | "settings") => void;
}

export function Dashboard({ onNavigate }: Props) {
  const t = useT();
  return (
    <div className="flex flex-col items-center justify-center gap-8 p-12 text-center">
      <div>
        <div className="mb-2 text-5xl">✨</div>
        <h1 className="text-3xl font-bold text-gray-900">CleanFlow</h1>
        <p className="mt-2 text-gray-500">{t("tagline")}</p>
      </div>

      <div className="grid grid-cols-2 gap-4 w-full max-w-md">
        <ActionCard
          icon={<FolderSearch size={28} />}
          title={t("cardScanTitle")}
          description={t("cardScanDesc")}
          onClick={() => onNavigate("scan")}
          primary
        />
        <ActionCard
          icon={<Settings size={28} />}
          title={t("cardSettingsTitle")}
          description={t("cardSettingsDesc")}
          onClick={() => onNavigate("settings")}
        />
      </div>

      <History />
    </div>
  );
}

/** Journal of executed runs; any of them can be undone, not just the last. */
function History() {
  const t = useT();
  const [entries, setEntries] = useState<HistoryEntry[]>([]);
  const [result, setResult] = useState<UndoResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(() => {
    api.listHistory(10).then(setEntries).catch((e) => setError(String(e)));
  }, []);
  useEffect(load, [load]);

  const undo = async (id: string) => {
    setError(null);
    try {
      setResult(await api.undoById(id));
      load();
    } catch (e) {
      setError(String(e));
    }
  };

  return (
    <div className="w-full max-w-md text-left">
      <h2 className="mb-2 text-sm font-semibold text-gray-700">{t("historyTitle")}</h2>
      {entries.length === 0 && <p className="text-xs text-gray-500">{t("historyEmpty")}</p>}
      <ul className="flex flex-col gap-2">
        {entries.map((e) => (
          <li key={e.id} className="flex items-center justify-between rounded-lg border border-gray-200 bg-white px-3 py-2 text-sm">
            <span>
              {new Date(e.executed_at * 1000).toLocaleString()} · {t("historyEntry", { n: e.actions.length })}
            </span>
            <button onClick={() => undo(e.id)} className="rounded-md border border-gray-300 px-3 py-1 text-xs hover:bg-gray-50">
              {t("undo")}
            </button>
          </li>
        ))}
      </ul>
      {result && (
        <div className="mt-3 text-xs text-gray-600">
          <p>{t("undoneSummary", { n: result.undone_count, errors: result.errors.length })}</p>
          <ul className="mt-1 list-disc pl-5">{result.errors.map((m) => <li key={m}>{m}</li>)}</ul>
        </div>
      )}
      {error && <p className="mt-3 text-xs text-red-700">{error}</p>}
    </div>
  );
}

interface CardProps {
  icon: React.ReactNode;
  title: string;
  description: string;
  onClick: () => void;
  primary?: boolean;
}

function ActionCard({ icon, title, description, onClick, primary }: CardProps) {
  return (
    <button
      onClick={onClick}
      className={`rounded-xl border p-6 text-left transition-all hover:shadow-md ${
        primary
          ? "border-brand-200 bg-brand-50 hover:border-brand-400"
          : "border-gray-200 bg-white hover:border-gray-300"
      }`}
    >
      <div className={primary ? "text-brand-600" : "text-gray-500"}>{icon}</div>
      <div className="mt-3 font-semibold text-gray-900">{title}</div>
      <div className="mt-1 text-xs text-gray-500">{description}</div>
    </button>
  );
}
