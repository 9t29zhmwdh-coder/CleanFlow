import { useState } from "react";
import { Dashboard } from "./components/Dashboard/Dashboard";
import { ScanView } from "./components/Scanner/ScanView";
import { ActionPreview } from "./components/Preview/ActionPreview";
import { useScanStore } from "./stores/scanStore";

type View = "dashboard" | "scan" | "settings";

export default function App() {
  const [view, setView] = useState<View>("dashboard");
  const { plan } = useScanStore();

  // When a plan is ready, automatically show preview
  const activeView = plan ? "preview" : view;

  return (
    <div className="flex h-screen flex-col bg-gray-50 font-sans text-gray-900 antialiased">
      {/* Sidebar */}
      <div className="flex flex-1 overflow-hidden">
        <nav className="flex w-48 flex-col gap-1 border-r border-gray-200 bg-white p-3">
          <NavItem label="Home"       active={activeView === "dashboard"} onClick={() => setView("dashboard")} />
          <NavItem label="Scan"       active={activeView === "scan"}      onClick={() => setView("scan")} />
          <NavItem label="Plan"       active={activeView === "preview"}   onClick={() => {}} disabled={!plan} />
          <NavItem label="Settings"   active={activeView === "settings"}  onClick={() => setView("settings")} />
        </nav>

        {/* Main content */}
        <main className="flex-1 overflow-y-auto">
          {activeView === "dashboard" && (
            <Dashboard onNavigate={(v) => setView(v as View)} />
          )}
          {activeView === "scan" && <ScanView />}
          {activeView === "preview" && <ActionPreview />}
          {activeView === "settings" && <SettingsPlaceholder />}
        </main>
      </div>
    </div>
  );
}

function NavItem({
  label, active, onClick, disabled,
}: {
  label: string; active: boolean; onClick: () => void; disabled?: boolean;
}) {
  return (
    <button
      onClick={onClick}
      disabled={disabled}
      className={`rounded-lg px-3 py-2 text-left text-sm font-medium transition-colors disabled:opacity-40 ${
        active
          ? "bg-brand-50 text-brand-700"
          : "text-gray-600 hover:bg-gray-100 hover:text-gray-900"
      }`}
    >
      {label}
    </button>
  );
}

function SettingsPlaceholder() {
  return (
    <div className="p-6">
      <h2 className="text-2xl font-bold text-gray-900">Settings</h2>
      <p className="mt-2 text-gray-500">AI provider, API keys, custom rules.</p>
    </div>
  );
}
