import { create } from "zustand";
import type { ExecutionResult, OrganizePlan, ScannedFile, ScanStatus, UndoResult } from "../lib/tauri";
import { api, listenScanStatus } from "../lib/tauri";

interface ScanStore {
  scanId: string | null;
  status: ScanStatus | null;
  files: ScannedFile[];
  plan: OrganizePlan | null;
  isLoading: boolean;
  error: string | null;
  // Redundant copies found in the finished scan (files minus one per group).
  duplicateCount: number;
  lastResult: ExecutionResult | null;
  undoResult: UndoResult | null;

  startScan: (path: string) => Promise<void>;
  loadPlan: () => Promise<void>;
  executePlan: (selectedIds?: string[]) => Promise<void>;
  executeCleanflow: () => Promise<void>;
  undoLast: () => Promise<void>;
  reset: () => void;
}

export const useScanStore = create<ScanStore>((set, get) => ({
  scanId: null,
  status: null,
  files: [],
  plan: null,
  isLoading: false,
  error: null,
  duplicateCount: 0,
  lastResult: null,
  undoResult: null,

  startScan: async (path) => {
    set({ isLoading: true, error: null, files: [], plan: null, duplicateCount: 0, lastResult: null, undoResult: null });
    try {
      const scanId = await api.scanDirectory(path);
      set({ scanId });

      let settled = false;
      // The Done event can arrive while listenScanStatus is still registering,
      // so handleStatus must not touch the unlisten function before it exists.
      let unlisten: (() => void) | null = null;
      const stopListening = () => unlisten?.();
      const handleStatus = async (status: ScanStatus) => {
        if (settled) return;
        set({ status });
        if (status.phase === "Done") {
          settled = true;
          stopListening();
          try {
            const files = await api.getScannedFiles(scanId);
            // Duplicates are only hashed on request; without this the scan
            // summary always showed 0 even when the plan then found some.
            const groups = await api.findDuplicates(scanId);
            const duplicateCount = groups.reduce((n, g) => n + g.files.length - 1, 0);
            set({ files, duplicateCount, isLoading: false });
          } catch (e) {
            set({ isLoading: false, error: String(e) });
          }
        } else if (status.phase === "Cancelled") {
          settled = true;
          stopListening();
          set({ isLoading: false });
        } else if (typeof status.phase === "object" && "Error" in status.phase) {
          settled = true;
          stopListening();
          set({ isLoading: false, error: status.phase.Error });
        }
      };

      unlisten = await listenScanStatus(scanId, handleStatus);
      if (settled) stopListening();

      // The scan may already have finished (very small directories) before the
      // listener above was attached; poll once to catch a missed Done event.
      await handleStatus(await api.getScanStatus(scanId));
    } catch (e) {
      set({ isLoading: false, error: String(e) });
    }
  },

  loadPlan: async () => {
    const { scanId } = get();
    if (!scanId) return;
    set({ isLoading: true });
    try {
      const plan = await api.previewPlan(scanId);
      set({ plan, isLoading: false });
    } catch (e) {
      set({ isLoading: false, error: String(e) });
    }
  },

  executePlan: async (selectedIds) => {
    const { plan } = get();
    if (!plan) return;
    set({ isLoading: true });
    try {
      const lastResult = await api.executePlan(plan.id, selectedIds);
      set({ isLoading: false, plan: null, files: [], lastResult, undoResult: null });
    } catch (e) {
      set({ isLoading: false, error: String(e) });
    }
  },

  executeCleanflow: async () => {
    const { scanId } = get();
    if (!scanId) return;
    set({ isLoading: true });
    try {
      const lastResult = await api.executeCleanflow(scanId);
      set({ isLoading: false, plan: null, files: [], lastResult, undoResult: null });
    } catch (e) {
      set({ isLoading: false, error: String(e) });
    }
  },

  undoLast: async () => {
    set({ isLoading: true, error: null });
    try {
      const undoResult = await api.undoLast();
      set({ isLoading: false, lastResult: null, undoResult });
    } catch (e) {
      set({ isLoading: false, error: String(e) });
    }
  },

  reset: () => set({ scanId: null, status: null, files: [], plan: null, error: null, duplicateCount: 0, lastResult: null, undoResult: null }),
}));
