import { Archive, Clock3, Sparkles } from "lucide-react";
import React from "react";
import { EmptyState, IconMetric, LoadingState } from "../player-primitives";
import type { CourseRecord, ReviewHistoryEntry } from "../player-model";
import type { PracticeState } from "../player-hooks";
import "./ReviewView.css";

export function ReviewView({
  course,
  practice,
  reviewHistory,
  onRate
}: {
  course: CourseRecord;
  practice: PracticeState;
  reviewHistory: Record<string, ReviewHistoryEntry>;
  onRate: (questionId: string, rating: number) => void;
}) {
  const [index, setIndex] = React.useState(0);
  const [flipped, setFlipped] = React.useState(false);

  React.useEffect(() => {
    setIndex(0);
    setFlipped(false);
  }, [course.courseId]);

  if (practice.loading) {
    return <LoadingState message="Building recall queue..." />;
  }
  if (practice.error || !practice.data) {
    return <EmptyState message={practice.error ?? "No practice assets found."} tone="danger" />;
  }

  const queue = [...practice.data.flashcards].sort((left, right) => {
    const leftHistory = left.variant.question_id ? reviewHistory[left.variant.question_id] : undefined;
    const rightHistory = right.variant.question_id ? reviewHistory[right.variant.question_id] : undefined;
    const leftScore = (leftHistory?.lastRating ?? 0) * 100 + (leftHistory?.seenCount ?? 0);
    const rightScore = (rightHistory?.lastRating ?? 0) * 100 + (rightHistory?.seenCount ?? 0);
    return leftScore - rightScore;
  });

  if (!queue.length) {
    return <EmptyState message="This course has no flashcards yet." />;
  }

  const current = queue[index % queue.length];
  const prompt =
    current.variant.front ??
    current.variant.stem ??
    current.variant.prompt_blocks?.join(" / ") ??
    current.family.learning_goal ??
    "Recall prompt";
  const answer = current.variant.back ?? current.variant.reference_answer?.join(" ") ?? current.variant.explanation ?? "No answer available.";

  return (
    <section className="review-shell">
      <div className="section-heading">
        <div>
          <h2>Recall Session</h2>
          <p>{course.title} · FSRS-flavored review loop using generated flashcard families.</p>
        </div>
      </div>

      <div className="review-summary-grid">
        <IconMetric icon={Archive} label="Total flashcards in the current course." value={queue.length} />
        <IconMetric icon={Sparkles} label="Cards you have not seen yet." value={queue.filter((item) => !item.variant.question_id || !reviewHistory[item.variant.question_id]).length} />
        <IconMetric
          icon={Clock3}
          label="Cards still worth refreshing."
          value={queue.filter((item) => {
            const id = item.variant.question_id;
            return id ? (reviewHistory[id]?.lastRating ?? 0) < 4 : true;
          }).length}
        />
      </div>

      <div className="review-card-flip" onClick={() => setFlipped((value) => !value)}>
        <article className="review-card-shell">
          <div className={`review-card-body ${flipped ? "flipped" : ""}`}>
            <div className="review-face front">
              <div className="review-card-meta">
                <span className="pill accent">{current.chapterTitle}</span>
                <span className="pill">{current.stepId}</span>
              </div>
              <h3>{prompt}</h3>
              <p className="story-note">Tap the card to reveal the target answer.</p>
            </div>

            <div className="review-face back">
              <div className="review-card-meta">
                <span className="pill accent">Definition</span>
                <span className="pill">{current.family.difficulty ?? "review"}</span>
              </div>
              <h3>{prompt}</h3>
              <div className="review-card-answer">{answer}</div>
            </div>
          </div>
        </article>
      </div>

      {flipped ? (
        <div className="card-rating-row">
          {[
            ["Again", 1],
            ["Hard", 2],
            ["Good", 3],
            ["Easy", 4]
          ].map(([label, rating]) => (
            <button
              key={label}
              className="rating-button"
              onClick={() => {
                if (current.variant.question_id) {
                  onRate(current.variant.question_id, Number(rating));
                }
                setFlipped(false);
                setIndex((value) => value + 1);
              }}
            >
              <strong>{label}</strong>
              <div className="muted">Rate the recall quality and move on.</div>
            </button>
          ))}
        </div>
      ) : null}
    </section>
  );
}
