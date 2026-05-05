import { Archive, ArrowRight, Brain, CheckCircle2, Clock3, Map as MapIcon, Play, Sparkles, Trophy } from "lucide-react";
import { IconMetric } from "../player-primitives";
import type { CourseRecord, CourseSummary, PlayerProgress, StoryManifest } from "../player-model";
import { chapterKey, safeColor } from "../player-utils";
import "./CourseHomeView.css";

export function CourseHomeView({
  course,
  manifests,
  courseStats,
  dueCards,
  progress,
  onContinueRoute,
  onOpenChapter,
  onOpenReview,
  onOpenVault
}: {
  course: CourseRecord;
  manifests: Record<string, StoryManifest>;
  courseStats?: CourseSummary;
  dueCards: number;
  progress: PlayerProgress;
  onContinueRoute: () => void;
  onOpenChapter: (chapterId: string) => void;
  onOpenReview: () => void;
  onOpenVault: () => void;
}) {
  const color = safeColor(course.brandColor);

  return (
    <section>
      <div className="hero-panel" style={{ ["--card-accent" as string]: `${color}26` }}>
        <div className="hero-copy">
          <span className="pill accent">
            <Sparkles size={13} />
            <span>Verified Course Game</span>
          </span>
          <h1>{course.title}</h1>
          <p>Enter the learner home base for this course: route selection, immersive scene play, recall loops, and a vault of generated challenges.</p>
          <div className="hero-actions" style={{ marginTop: 22 }}>
            <button className="pill-button primary" onClick={onContinueRoute}>
              <span className="button-label">
                <Play size={15} />
                <span>Continue Route</span>
              </span>
            </button>
            <button className="pill-button" onClick={onOpenReview}>
              <span className="button-label">
                <Brain size={15} />
                <span>Recall Center</span>
              </span>
            </button>
            <button className="pill-button ghost" onClick={onOpenVault}>
              <span className="button-label">
                <Archive size={15} />
                <span>Open Vault</span>
              </span>
            </button>
          </div>
        </div>
        <div className="hero-art" style={{ background: `linear-gradient(135deg, ${color}, #60a5fa)` }}>
          {course.courseId.toUpperCase()}
        </div>
      </div>

      <div className="metric-grid" style={{ marginTop: 20 }}>
        <IconMetric icon={Trophy} label="Route mastery across completed scenes." value={`${courseStats?.mastery ?? 0}%`} />
        <IconMetric icon={MapIcon} label="Guided scenes cleared so far." value={`${courseStats?.completedSteps ?? 0}/${courseStats?.totalSteps ?? 0}`} />
        <IconMetric icon={Clock3} label="Cards currently worth revisiting in recall mode." value={dueCards} />
      </div>

      <div className="section-heading" style={{ marginTop: 28 }}>
        <div>
          <h3>Choose your route</h3>
          <p>Every chapter ships with a story route, a reading room, and downstream practice families.</p>
        </div>
      </div>
      <div className="chapter-grid">
        {course.chapters.map((chapter) => {
          const manifest = manifests[chapterKey(course.courseId, chapter.chapterId)];
          const completed = progress.completedSteps[chapterKey(course.courseId, chapter.chapterId)] ?? [];
          const totalSteps = manifest?.steps?.length ?? 0;
          const progressWidth = totalSteps === 0 ? 0 : Math.round((completed.length / totalSteps) * 100);
          return (
            <article
              key={chapter.chapterId}
              className="chapter-card"
              style={{ ["--card-accent" as string]: `${color}1f` }}
              onClick={() => onOpenChapter(chapter.chapterId)}
            >
              <div className="chapter-card-head">
                <span className="pill">
                  <MapIcon size={13} />
                  <span>{chapter.lessonId}</span>
                </span>
                <h4>{chapter.title}</h4>
                <p>{totalSteps} scenes · {completed.length} cleared · story + recall + textbook</p>
              </div>
              <div className="chapter-card-meta-row">
                <span className="micro-pill">{progressWidth}% cleared</span>
                <span className="chapter-card-link">
                  <span>Open route</span>
                  <ArrowRight size={14} />
                </span>
              </div>
              <div className="progress-track" style={{ marginTop: 16 }}>
                <div className="progress-fill" style={{ width: `${progressWidth}%` }} />
              </div>
            </article>
          );
        })}
      </div>
    </section>
  );
}
