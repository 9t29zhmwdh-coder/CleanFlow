import { FolderSearch, Zap, Settings } from "lucide-react";

interface Props {
  onNavigate: (view: "scan" | "settings") => void;
}

export function Dashboard({ onNavigate }: Props) {
  return (
    <div className="flex flex-col items-center justify-center gap-8 p-12 text-center">
      <div>
        <div className="mb-2 text-5xl">✨</div>
        <h1 className="text-3xl font-bold text-gray-900">CleanFlow</h1>
        <p className="mt-2 text-gray-500">AI-powered file organizer</p>
      </div>

      <div className="grid grid-cols-2 gap-4 w-full max-w-md">
        <ActionCard
          icon={<FolderSearch size={28} />}
          title="Scan Directory"
          description="Analyse a folder and get an AI-powered organisation plan"
          onClick={() => onNavigate("scan")}
          primary
        />
        <ActionCard
          icon={<Settings size={28} />}
          title="Settings"
          description="Configure AI provider, rules and output preferences"
          onClick={() => onNavigate("settings")}
        />
      </div>
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
