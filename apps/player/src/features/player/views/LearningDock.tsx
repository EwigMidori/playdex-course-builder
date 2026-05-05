import { Archive, Brain, Clock3, Home, Play } from "lucide-react";
import type { ChapterRecord, CourseRecord } from "../player-model";
import "./LearningDock.css";

export function LearningDock({
  course,
  chapter,
  dueCards,
  onHome,
  onContinue,
  onReview,
  onVault,
  canContinue
}: {
  course: CourseRecord;
  chapter: ChapterRecord | null;
  dueCards: number;
  onHome: () => void;
  onContinue: () => void;
  onReview: () => void;
  onVault: () => void;
  canContinue: boolean;
}) {
  return (
    <div className="learning-dock">
      <div className="dock-copy">
        <span className="eyebrow">Learning Dock</span>
        <strong>{course.title}</strong>
        <span>{chapter ? `${chapter.lessonId} · ${chapter.title}` : "Choose a route"}</span>
      </div>
      <div className="dock-actions">
        <button className="pill-button ghost" onClick={onHome}>
          <span className="button-label">
            <Home size={15} />
            <span>Home</span>
          </span>
        </button>
        <button className="pill-button primary" onClick={onContinue} disabled={!chapter || !canContinue}>
          <span className="button-label">
            <Play size={15} />
            <span>Continue</span>
          </span>
        </button>
        <button className="pill-button ghost" onClick={onReview}>
          <span className="button-label">
            <Brain size={15} />
            <span>Recall</span>
          </span>
        </button>
        <button className="pill-button ghost" onClick={onVault}>
          <span className="button-label">
            <Archive size={15} />
            <span>Vault</span>
          </span>
        </button>
        <span className="pill accent">
          <Clock3 size={13} />
          <span>{dueCards} due</span>
        </span>
      </div>
    </div>
  );
}
