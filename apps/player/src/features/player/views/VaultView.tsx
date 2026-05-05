import { Archive, BookOpen, Brain, FolderOpen, Sparkles } from "lucide-react";
import React from "react";
import { EmptyState, LoadingState } from "../player-primitives";
import type { CourseRecord } from "../player-model";
import type { PracticeState } from "../player-hooks";
import "./VaultView.css";

export function VaultView({ course, practice }: { course: CourseRecord; practice: PracticeState }) {
  const [filter, setFilter] = React.useState<"all" | "flashcards" | "quizzes" | "longform">("all");

  if (practice.loading) {
    return <LoadingState message="Opening the vault..." />;
  }
  if (practice.error || !practice.data) {
    return <EmptyState message={practice.error ?? "No vault assets found."} tone="danger" />;
  }

  const entries = [
    ...practice.data.flashcards.map((item) => ({
      key: item.variant.question_id ?? `${item.stepId}-${item.family.family_id ?? "flash"}`,
      type: "flashcards" as const,
      title: item.variant.front ?? item.family.learning_goal ?? "Flashcard",
      body: item.variant.back ?? item.variant.explanation ?? item.family.retrieval_focus ?? "Recall card",
      meta: item.chapterTitle
    })),
    ...practice.data.quizFamilies.map((family) => ({
      key: family.family_id ?? family.learning_goal ?? "quiz",
      type: "quizzes" as const,
      title: family.learning_goal ?? family.family_id ?? "Quiz family",
      body: family.variants?.[0]?.stem ?? family.variants?.[0]?.front ?? family.retrieval_focus ?? "Quiz family",
      meta: family.difficulty ?? "quiz"
    })),
    ...practice.data.longformFamilies.map((family) => ({
      key: family.family_id ?? family.learning_goal ?? "longform",
      type: "longform" as const,
      title: family.learning_goal ?? family.family_id ?? "Longform family",
      body: family.variants?.[0]?.stem ?? family.retrieval_focus ?? "Longform prompt",
      meta: family.difficulty ?? "longform"
    }))
  ].filter((entry) => filter === "all" || entry.type === filter);

  return (
    <section>
      <div className="section-heading">
        <div>
          <h2>The Vault</h2>
          <p>{course.title} · generated practice assets organized into a playable challenge bank.</p>
        </div>
      </div>

      <div className="bank-filters">
        {[
          ["all", "Everything"],
          ["flashcards", "Flashcards"],
          ["quizzes", "Quizzes"],
          ["longform", "Longform"]
        ].map(([value, label]) => (
          <button key={value} className={`pill-button ${filter === value ? "primary" : ""}`} onClick={() => setFilter(value as typeof filter)}>
            <span className="button-label">
              {value === "all" ? <Sparkles size={15} /> : null}
              {value === "flashcards" ? <Archive size={15} /> : null}
              {value === "quizzes" ? <Brain size={15} /> : null}
              {value === "longform" ? <BookOpen size={15} /> : null}
              <span>{label}</span>
            </span>
          </button>
        ))}
      </div>

      <div className="vault-grid">
        {entries.map((entry) => (
          <article key={entry.key} className="vault-card">
            <div className="vault-meta">
              <span className="pill accent">
                {entry.type === "flashcards" ? <Archive size={13} /> : null}
                {entry.type === "quizzes" ? <Brain size={13} /> : null}
                {entry.type === "longform" ? <BookOpen size={13} /> : null}
                <span>{entry.type}</span>
              </span>
              <span className="pill">
                <FolderOpen size={13} />
                <span>{entry.meta}</span>
              </span>
            </div>
            <h4>{entry.title}</h4>
            <p>{entry.body}</p>
          </article>
        ))}
      </div>
    </section>
  );
}
