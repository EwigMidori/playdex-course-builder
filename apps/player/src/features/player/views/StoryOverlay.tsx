import { AlertTriangle, ArrowRight, BookOpen, ChevronRight } from "lucide-react";
import React from "react";
import { toast } from "sonner";
import { ExerciseRenderer } from "../exercise/ExerciseRenderer";
import { resolveExercise, type ExerciseSubmissionResult } from "../exercise/exercise-registry";
import type { AsyncState } from "../player-hooks";
import { EmptyState, LoadingState } from "../player-primitives";
import type { ChapterRecord, CourseRecord, QuestionBank, StoryManifestStep, StoryStep, TermEntry } from "../player-model";
import { normalizeExerciseText } from "../player-utils";
import "./StoryOverlay.css";

export function StoryOverlay({
  course,
  chapter,
  stepMeta,
  bundle,
  onClose,
  onComplete,
  onOpenTerm
}: {
  course: CourseRecord;
  chapter: ChapterRecord;
  stepMeta: StoryManifestStep | null;
  bundle: AsyncState<{ step: StoryStep; bank: QuestionBank }>;
  onClose: () => void;
  onComplete: () => void;
  onOpenTerm: (id: string, term: TermEntry) => void;
}) {
  const [screenIndex, setScreenIndex] = React.useState(0);
  const [showLog, setShowLog] = React.useState(false);
  const [selectedAnswer, setSelectedAnswer] = React.useState<number | null>(null);
  const [textAnswer, setTextAnswer] = React.useState("");
  const [textSubmitted, setTextSubmitted] = React.useState<string | null>(null);
  const [textAnswerCorrect, setTextAnswerCorrect] = React.useState<boolean | null>(null);
  const [answered, setAnswered] = React.useState(false);

  React.useEffect(() => {
    setScreenIndex(0);
    setShowLog(false);
    setSelectedAnswer(null);
    setTextAnswer("");
    setTextSubmitted(null);
    setTextAnswerCorrect(null);
    setAnswered(false);
  }, [stepMeta?.sequence_id]);

  const screens = bundle.data?.step.screens ?? [];
  const current = screens[screenIndex];
  const currentExercise = current?.exercise;
  const progress = screens.length === 0 ? 0 : ((screenIndex + 1) / screens.length) * 100;
  const history = screens.slice(0, screenIndex);
  const isExerciseScreen = current?.type === "exercise";
  const acceptedAnswers = currentExercise?.answers ?? [];
  const normalizedAcceptedAnswers = acceptedAnswers.map(normalizeExerciseText).filter(Boolean);
  const stepId = stepMeta?.sequence_id ?? bundle.data?.step.sequence_id ?? "unknown-step";
  const screenId = current?.id ?? `screen-${screenIndex + 1}`;
  const exerciseContext = `course=${course.courseId} · chapter=${chapter.chapterId} · step=${stepId} · screen=${screenId} · kind=${currentExercise?.kind ?? "missing"}`;
  const { definition, unsupportedReason } = resolveExercise(currentExercise);
  const nextStepBlockedReason = (() => {
    if (!current) {
      return "No scene content loaded";
    }
    if (current.type !== "exercise") {
      return null;
    }
    if (unsupportedReason) {
      return unsupportedReason;
    }
    return definition?.getBlockedReason(currentExercise ?? {}, answered) ?? null;
  })();
  const nextStepDisabled = current == null || nextStepBlockedReason !== null;
  const nextStepHint =
    nextStepBlockedReason ??
    (screenIndex === screens.length - 1 ? "Checkpoint complete. You can finish this step." : "Checkpoint complete. Continue to the next scene.");

  React.useEffect(() => {
    if (!isExerciseScreen || !unsupportedReason) {
      return;
    }
    toast.error("Unsupported checkpoint in route player", {
      description: `${unsupportedReason}. ${exerciseContext}`
    });
  }, [exerciseContext, isExerciseScreen, unsupportedReason]);

  const applySubmissionResult = React.useCallback(
    (result: ExerciseSubmissionResult) => {
      if (typeof result.answered === "boolean") {
        setAnswered(result.answered);
      }
      if (typeof result.selectedAnswer !== "undefined") {
        setSelectedAnswer(result.selectedAnswer);
      }
      if (typeof result.textSubmitted !== "undefined") {
        setTextSubmitted(result.textSubmitted);
      }
      if (typeof result.textAnswerCorrect !== "undefined") {
        setTextAnswerCorrect(result.textAnswerCorrect);
      }
      if (result.feedback) {
        if (result.feedback.level === "warning") {
          toast.warning(result.feedback.title, { description: result.feedback.description });
        } else {
          toast.message(result.feedback.title, { description: result.feedback.description });
        }
      }
    },
    []
  );

  const notifyBlockedAdvance = React.useCallback(
    (reason: string) => {
      toast.warning("Checkpoint cannot continue yet", {
        description: `${reason}. ${exerciseContext}`
      });
    },
    [exerciseContext]
  );

  const handleTextExerciseSubmit = React.useCallback(() => {
    if (!definition?.submitText || !currentExercise) {
      return;
    }
    applySubmissionResult(
      definition.submitText({
        exercise: currentExercise,
        textAnswer,
        acceptedAnswers,
        normalizedAcceptedAnswers,
        exerciseContext
      })
    );
  }, [acceptedAnswers, applySubmissionResult, currentExercise, definition, exerciseContext, normalizedAcceptedAnswers, textAnswer]);

  const resetAnswerState = React.useCallback(() => {
    setSelectedAnswer(null);
    setTextAnswer("");
    setTextSubmitted(null);
    setTextAnswerCorrect(null);
    setAnswered(false);
  }, []);

  const handleNext = React.useCallback(() => {
    if (!current) {
      return;
    }
    if (nextStepBlockedReason) {
      notifyBlockedAdvance(nextStepBlockedReason);
      return;
    }
    if (screenIndex < screens.length - 1) {
      setScreenIndex((value) => value + 1);
      resetAnswerState();
      return;
    }
    onComplete();
  }, [current, nextStepBlockedReason, notifyBlockedAdvance, onComplete, resetAnswerState, screenIndex, screens.length]);

  const handlePrev = React.useCallback(() => {
    if (screenIndex === 0) {
      return;
    }
    setScreenIndex((value) => value - 1);
    resetAnswerState();
  }, [resetAnswerState, screenIndex]);

  React.useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.code !== "Space") {
        return;
      }
      if (event.metaKey || event.ctrlKey || event.altKey) {
        return;
      }

      const target = event.target;
      if (
        target instanceof HTMLInputElement ||
        target instanceof HTMLTextAreaElement ||
        target instanceof HTMLSelectElement ||
        target instanceof HTMLButtonElement ||
        (target instanceof HTMLElement && target.isContentEditable)
      ) {
        return;
      }

      event.preventDefault();
      handleNext();
    };

    window.addEventListener("keydown", onKeyDown);
    return () => {
      window.removeEventListener("keydown", onKeyDown);
    };
  }, [handleNext]);

  if (bundle.loading || !bundle.data) {
    return (
      <div className="story-overlay">
        <div className="overlay-layer">
          <div className="dialog-shell">
            <LoadingState message={bundle.error ?? "Loading route scene..."} />
          </div>
        </div>
      </div>
    );
  }

  const parseLines = (line: string, index: number) => {
    const parts = line.split(/(<term id="[^"]+">.*?<\/term>)/g);
    return (
      <p key={`${current?.id ?? "screen"}-${index}`}>
        {parts.map((part, partIndex) => {
          const match = part.match(/<term id="([^"]+)">([\s\S]*?)<\/term>/);
          if (!match) {
            return <React.Fragment key={partIndex}>{part}</React.Fragment>;
          }
          const [, id, label] = match;
          const term = bundle.data?.step.term_catalog?.[id];
          return (
            <span
              key={partIndex}
              className="term-token"
              onClick={(event) => {
                event.stopPropagation();
                if (term) {
                  onOpenTerm(id, term);
                }
              }}
            >
              {label}
            </span>
          );
        })}
      </p>
    );
  };

  const answerIndex = typeof current?.exercise?.answer === "number" ? current.exercise.answer : Number.NaN;
  const answeredNote = definition?.getAnsweredNote(currentExercise ?? {}) ?? currentExercise?.explanation ?? "Checkpoint complete. Continue the route.";

  return (
    <div className="story-overlay">
      <div className="overlay-layer">
        <div className="overlay-controls">
          <div className="overlay-title">
            <span>{course.title}</span>
            <strong>{chapter.lessonId} · {stepMeta?.concept ?? stepMeta?.sequence_id ?? "Route Scene"}</strong>
          </div>
          <div className="topbar-actions">
            <button className="pill-button ghost" onClick={() => setShowLog(true)}>
              <span className="button-label">
                <BookOpen size={15} />
                <span>Scene Log</span>
              </span>
            </button>
            <button className="pill-button danger" onClick={onClose}>
              <span className="button-label">
                <AlertTriangle size={15} />
                <span>Exit Route</span>
              </span>
            </button>
          </div>
        </div>

        <div className="scene-stage">
          {current?.type === "exercise" ? (
            <div className="scene-exercise">
              <span className="pill accent">Checkpoint</span>
              <h3>{currentExercise?.prompt ?? current.lines?.[0] ?? "Quick check"}</h3>
              <p className="story-note">Answer first, then continue. This keeps the story from becoming passive scrolling.</p>
              <ExerciseRenderer
                acceptedAnswers={acceptedAnswers}
                answerIndex={answerIndex}
                answered={answered}
                exercise={currentExercise ?? {}}
                exerciseContext={exerciseContext}
                onSelectAnswer={(index) => {
                  if (answered || !definition?.selectChoice || !currentExercise) {
                    return;
                  }
                  applySubmissionResult(definition.selectChoice(currentExercise, index, answerIndex));
                }}
                onSubmitTextAnswer={handleTextExerciseSubmit}
                onTextAnswerChange={setTextAnswer}
                selectedAnswer={selectedAnswer}
                textAnswer={textAnswer}
                textAnswerCorrect={textAnswerCorrect}
                textSubmitted={textSubmitted}
                unsupportedReason={unsupportedReason}
              />
              {answered ? <p className="story-note">{answeredNote}</p> : null}
            </div>
          ) : (
            <div className="scene-avatar">✦</div>
          )}
        </div>

        <div className="dialog-shell" onClick={current?.type === "exercise" ? undefined : handleNext}>
          <div className="dialog-topline">
            <span className="dialog-speaker">{current?.type === "exercise" ? "System Test" : "Instructor"}</span>
            <div className="dialog-progress">
              <div className="progress-fill" style={{ width: `${progress}%` }} />
            </div>
          </div>

          <div className="dialog-lines">
            {(current?.lines ?? ["No scene content found."]).map((line, index) => parseLines(line, index))}
          </div>

          <div className="story-footer">
            <div className="topbar-actions">
              <button className="pill-button ghost" onClick={handlePrev}>
                <span className="button-label">
                  <ChevronRight className="button-icon-flip" size={15} />
                  <span>Back</span>
                </span>
              </button>
            </div>
            <div className="story-footer-status" title={nextStepHint}>
              {current?.type === "exercise" ? (answered ? "Continue" : nextStepBlockedReason ?? "Answer to Continue") : "Tap Anywhere to Continue"}
            </div>
            <div className="topbar-actions">
              <button className="pill-button primary" disabled={nextStepDisabled} onClick={handleNext} title={nextStepHint}>
                <span className="button-label">
                  <span>{screenIndex === screens.length - 1 ? "Finish Step" : "Next"}</span>
                  <ArrowRight size={15} />
                </span>
              </button>
            </div>
          </div>
        </div>
      </div>

      {showLog ? (
        <div className="story-log-scrim" onClick={() => setShowLog(false)}>
          <aside className="story-log-panel" onClick={(event) => event.stopPropagation()}>
            <div className="story-log-head">
              <h3>Narrative Log</h3>
              <button className="pill-button ghost" onClick={() => setShowLog(false)}>
                <span className="button-label">
                  <AlertTriangle size={15} />
                  <span>Close</span>
                </span>
              </button>
            </div>
            <div className="log-list">
              {history.length === 0 ? <EmptyState message="No prior lines yet." /> : null}
              {history.map((screen, index) => (
                <article key={`${screen.id ?? "log"}-${index}`} className="log-item">
                  <span className="pill">
                    <BookOpen size={13} />
                    <span>Page {index + 1}</span>
                  </span>
                  <p>{(screen.lines ?? []).join(" ")}</p>
                </article>
              ))}
            </div>
          </aside>
        </div>
      ) : null}
    </div>
  );
}
