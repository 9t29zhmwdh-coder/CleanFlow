import { create } from "zustand";
import type { OrganizePlan, ScannedFile, ScanStatus } from "../lib/tauri";
import { api, listenScanStatus } from "../lib/tauri";

interface ScanStore {
  scanId: string | null;
  status: ScanStatus | null;
  files: ScannedFile[];
  plan: OrganizePlan | null;
  isLoading: boolean;
  error: string | null;

  startScan: (path: string) => Promise<void>;
  loadPlan: () => Promise<void>;
  executePlan: (selectedIds?: string[]) => Promise<void>;
  executeCleanflow: () => Promise<void>;
  reset: () => void;
}

export const useScanStore = create<ScanStore>((set, get) => ({
  scanId: null,
  status: null,
  files: [],
  plan: null,
  isLoading: false,
  error: null,

  startScan: async (path) => {
    set({ isLoading: true, error: null, files: [], plan: null });
    try {
      const scanId = await api.scanDirectory(path);
      set({ scanId });

      const unlisten = await listenScanStatus(scanId, async (status) => {
        set({ status });
        if (status.phase === "Done") {
          unlisten();
          const files = await api.getScannedFiles(scanId);
          set({ files, isLoading: false });
        } else if (status.phase === "Cancelled") {
          unlisten();
          set({ isLoading: false });
        } else if (typeof status.phase === "object" && "Error" in status.phase) {
          unlisten();
          set({ isLoading: false, error: status.phase.Error });
        }
      });
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
      await api.executePlan(plan.id, selectedIds);
      set({ isLoading: false, plan: null });
    } catch (e) {
      set({ isLoading: false, error: String(e) });
    }
  },

  executeCleanflow: async () => {
    const { scanId } = get();
    if (!scanId) return;
    set({ isLoading: true });
    try {
      await api.executeCleanflow(scanId);
      set({ isLoading: false, plan: null });
    } catch (e) {
      set({ isLoading: false, error: String(e) });
    }
  },

  reset: () => set({ scanId: null, status: null, files: [], plan: null, error: null }),
}));
