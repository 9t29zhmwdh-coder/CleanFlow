import { useState } from "react";
import { FolderOpen, Zap } from "lucide-react";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { useScanStore } from "../../stores/scanStore";
import { formatBytes } from "../../lib/format";

export function ScanView() {
  const [path, setPath] = useState("");
  const { status, files, isLoading, startScan, loadPlan } = useScanStore();

  const pickFolder = async () => {
    const selected = await openDialog({ directory: true, multiple: false });
    if (selected) setPath(selected as string);
  };

  const handleScan = async () => {
    if (!path) return;
    await startScan(path);
  };

  const phase = status?.phase;
  const isDone = phase === "Done";

  return (
    <div className="flex flex-col gap-6 p-6">
      <h2 className="text-2xl font-bold text-gray-900">Scan Directory</h2>

      {/* Path picker */}
      <div className="flex gap-2">
        <input
          type="text"
          value={path}
          onChange={(e) => setPath(e.target.value)}
          placeholder="~/Downloads"
          className="flex-1 rounded-lg border border-gray-300 px-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-500"
        />
        <button
          onClick={pickFolder}
          className="flex items-center gap-2 rounded-lg border border-gray-300 px-4 py-2 text-sm hover:bg-gray-50"
        >
          <FolderOpen size={16} />
          Browse
        </button>
        <button
          onClick={handleScan}
          disabled={!path || isLoading}
          className="flex items-center gap-2 rounded-lg bg-brand-600 px-6 py-2 text-sm font-medium text-white hover:bg-brand-700 disabled:opacity-50"
        >
          <Zap size={16} />
          Scan
        </button>
      </div>

      {/* Progress */}
      {isLoading && status && (
        <div className="rounded-lg border border-gray-200 bg-white p-4">
          <div className="mb-2 flex justify-between text-sm text-gray-600">
            <span>{typeof phase === "string" ? phase : "Error"}</span>
            <span>{status.files_analyzed} / {status.files_found} files</span>
          </div>
          <div className="h-2 overflow-hidden rounded-full bg-gray-100">
            <div
              className="h-full rounded-full bg-brand-500 transition-all duration-300"
              style={{
                width: status.files_found > 0
                  ? `${(status.files_analyzed / status.files_found) * 100}%`
                  : "30%",
              }}
            />
          </div>
        </div>
      )}

      {/* Results */}
      {isDone && files.length > 0 && (
        <div className="rounded-lg border border-gray-200 bg-white p-4">
          <div className="mb-4 grid grid-cols-4 gap-4 text-center">
            <Stat label="Files"     value={files.length.toString()} />
            <Stat label="Junk"      value={files.filter(f => f.flags.is_junk).length.toString()} color="red" />
            <Stat label="Duplicates" value={files.filter(f => f.flags.is_duplicate).length.toString()} color="yellow" />
            <Stat
              label="Total Size"
              value={formatBytes(files.reduce((s, f) => s + f.size_bytes, 0))}
            />
          </div>
          <button
            onClick={loadPlan}
            className="w-full rounded-lg bg-brand-600 py-2 text-sm font-medium text-white hover:bg-brand-700"
          >
            Generate CleanFlow Plan →
          </button>
        </div>
      )}
    </div>
  );
}

function Stat({ label, value, color = "brand" }: { label: string; value: string; color?: string }) {
  const colors: Record<string, string> = {
    brand: "text-brand-600",
    red: "text-red-500",
    yellow: "text-yellow-500",
  };
  return (
    <div>
      <div className={`text-2xl font-bold ${colors[color] ?? "text-gray-900"}`}>{value}</div>
      <div className="text-xs text-gray-500">{label}</div>
    </div>
  );
}
