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

      let settled = false;
      const handleStatus = async (status: ScanStatus) => {
        if (settled) return;
        set({ status });
        if (status.phase === "Done") {
          settled = true;
          unlisten();
          try {
            const files = await api.getScannedFiles(scanId);
            set({ files, isLoading: false });
          } catch (e) {
            set({ isLoading: false, error: String(e) });
          }
        } else if (status.phase === "Cancelled") {
          settled = true;
          unlisten();
          set({ isLoading: false });
        } else if (typeof status.phase === "object" && "Error" in status.phase) {
          settled = true;
          unlisten();
          set({ isLoading: false, error: status.phase.Error });
        }
      };

      const unlisten = await listenScanStatus(scanId, handleStatus);

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
