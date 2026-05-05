import { BookOpen, CheckCircle2, ChevronRight, Compass, FolderOpen, Play, Trophy } from "lucide-react";
import { IconMetric } from "../player-primitives";
import type { ChapterRecord, CourseRecord, PlayerProgress, StoryManifest } from "../player-model";
import { chapterKey } from "../player-utils";
import "./ChapterHomeView.css";

export function ChapterHomeView({
  course,
  chapter,
  manifest,
  progress,
  activeStepId,
  onStartStory,
  onOpenTextbook,
  onSelectStep
}: {
  course: CourseRecord;
  chapter: ChapterRecord;
  manifest?: StoryManifest;
  progress: PlayerProgress;
  activeStepId: string | null;
  onStartStory: (stepId?: string) => void;
  onOpenTextbook: () => void;
  onSelectStep: (stepId: string) => void;
}) {
  const steps = manifest?.steps ?? [];
  const completed = progress.completedSteps[chapterKey(course.courseId, chapter.chapterId)] ?? [];
  const completionRatio = steps.length === 0 ? 0 : Math.round((completed.length / steps.length) * 100);

  return (
    <section className="chapter-detail">
      <div className="panel">
        <div className="chapter-copy">
          <span className="pill accent">
            <Compass size={13} />
            <span>{chapter.lessonId} Route</span>
          </span>
          <h2 className="chapter-title" style={{ fontSize: "clamp(2.3rem, 5vw, 4.2rem)", lineHeight: 0.98 }}>
            {chapter.title}
          </h2>
          <p>Start the route from the opening scene, or jump back to the exact concept node where you stopped.</p>
        </div>

        <div className="hero-actions" style={{ marginTop: 22 }}>
          <button className="pill-button primary" onClick={() => onStartStory(activeStepId ?? steps[0]?.sequence_id)}>
            <span className="button-label">
              <Play size={15} />
              <span>Start Route</span>
            </span>
          </button>
          <button className="pill-button" onClick={onOpenTextbook}>
            <span className="button-label">
              <BookOpen size={15} />
              <span>Read Textbook</span>
            </span>
          </button>
        </div>

        <div className="metric-grid" style={{ marginTop: 24 }}>
          <IconMetric icon={FolderOpen} label="Guided scenes in this chapter route." value={steps.length} />
          <IconMetric icon={Trophy} label="Completion rate for the current route." value={`${completionRatio}%`} />
          <IconMetric icon={CheckCircle2} label="Scenes already cleared and available for replay." value={completed.length} />
        </div>
      </div>

      <div className="panel">
        <div className="section-heading" style={{ marginBottom: 16 }}>
          <div>
            <h3>Scene Select</h3>
            <p>Pick a concept and jump straight into that route node.</p>
          </div>
        </div>

        <div className="step-list">
          {steps.map((step, index) => {
            const stepId = step.sequence_id ?? `step-${index + 1}`;
            const isDone = completed.includes(stepId);
            return (
              <button key={stepId} className={`step-row ${activeStepId === stepId ? "active" : ""}`} onClick={() => onSelectStep(stepId)}>
                <div className="step-index">{String(index + 1).padStart(2, "0")}</div>
                <div className="step-copy">
                  <strong>{step.concept ?? stepId}</strong>
                  <span>{isDone ? "Cleared once already" : "New route scene"}</span>
                </div>
                <div className="step-row-tail">
                  <span className="micro-pill">
                    {isDone ? <CheckCircle2 size={13} /> : <ChevronRight size={13} />}
                    <span>{isDone ? "Done" : "Ready"}</span>
                  </span>
                  <ChevronRight className="step-row-chevron" size={16} />
                </div>
              </button>
            );
          })}
        </div>
      </div>
    </section>
  );
}
