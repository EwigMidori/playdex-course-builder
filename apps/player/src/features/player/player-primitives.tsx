import type { LucideIcon } from "lucide-react";
import { AlertTriangle, LoaderCircle } from "lucide-react";
import "./player-primitives.css";

export function LoadingState({ message }: { message: string }) {
  return (
    <div className="loading-state">
      <LoaderCircle className="state-icon spin" />
      <span>{message}</span>
    </div>
  );
}

export function EmptyState({
  message,
  tone = "neutral"
}: {
  message: string;
  tone?: "neutral" | "danger";
}) {
  return (
    <div className={`empty-state ${tone === "danger" ? "danger" : ""}`}>
      <AlertTriangle className="state-icon" />
      <span>{message}</span>
    </div>
  );
}

export function IconMetric({
  icon: Icon,
  value,
  label
}: {
  icon: LucideIcon;
  value: React.ReactNode;
  label: string;
}) {
  return (
    <div className="metric-card">
      <div className="metric-card-icon">
        <Icon size={18} />
      </div>
      <strong>{value}</strong>
      <p>{label}</p>
    </div>
  );
}
