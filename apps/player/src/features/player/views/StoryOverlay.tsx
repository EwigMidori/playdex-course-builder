import { AlertTriangle, ArrowRight, BookOpen, ChevronRight } from "lucide-react";
import React from "react";
import { toast } from "sonner";
import { ExerciseRenderer } from "../exercise/ExerciseRenderer";
import { resolveExercise } from "../exercise/exercise-registry";
import type { AsyncState } from "../player-hooks";
import { EmptyState, LoadingState } from "../player-primitives";
import type { ChapterRecord, CourseRecord, StoryManifestStep, TermEntry } from "../player-model";
import { getExerciseAnsweredNote, getExerciseBlockedReason, getRuntimeExerciseState } from "../runtime/exercise-engine";
import {
  createStoryRuntimeState,
  evaluateStoryRuntimeAction,
  getCurrentStoryNode,
  getRuntimeBacklog,
  getRuntimeProgress
} from "../runtime/story-runtime";
import type { SceneModel, StoryRuntimeAction, StoryRuntimeEvaluation, StoryRuntimeState } from "../runtime/story-runtime-types";
import { GlossaryTooltip } from "./GlossaryTooltip";
import "./StoryOverlay.css";

function debugStoryRuntime(event: string, payload: Record<string, unknown>) {
  console.debug(`[story-runtime] ${event}`, payload);
}

type RuntimeControllerState = {
  runtimeState: StoryRuntimeState;
  lastEvaluation: StoryRuntimeEvaluation | null;
  revision: number;
};

type DialogPointerSnapshot = {
  x: number;
  y: number;
  at: number;
};

function createRuntimeControllerState(scene: SceneModel): RuntimeControllerState {
  return {
    runtimeState: createStoryRuntimeState(scene),
    lastEvaluation: null,
    revision: 0
  };
}

export function StoryOverlay({
  course,
  chapter,
  stepMeta,
  sceneState,
  onClose,
  onComplete,
  onOpenTerm
}: {
  course: CourseRecord;
  chapter: ChapterRecord;
  stepMeta: StoryManifestStep | null;
  sceneState: AsyncState<SceneModel>;
  onClose: () => void;
  onComplete: () => void;
  onOpenTerm: (id: string, term: TermEntry) => void;
}) {
  const [showLog, setShowLog] = React.useState(false);
  const dialogPointerRef = React.useRef<DialogPointerSnapshot | null>(null);
  const runtimeScene = sceneState.data ?? {
    sceneId: "pending-scene",
    courseId: course.courseId,
    chapterId: chapter.chapterId,
    lessonId: chapter.lessonId,
    stepId: stepMeta?.sequence_id ?? "unknown-step",
    concept: stepMeta?.concept ?? "Route Scene",
    termCatalog: {},
    nodes: [
      {
        id: "pending-scene:node:1",
        kind: "narration" as const,
        lines: ["Loading route scene..."],
        termRefs: [],
        rawScreenIndex: 0,
        rawScreenType: null
      }
    ]
  };
  const [runtimeController, setRuntimeController] = React.useState<RuntimeControllerState>(() =>
    createRuntimeControllerState(runtimeScene)
  );
  const runtimeState = runtimeController.runtimeState;

  React.useEffect(() => {
    if (!sceneState.data) {
      return;
    }
    debugStoryRuntime("enter-scene", {
      sceneId: sceneState.data.sceneId,
      stepId: sceneState.data.stepId,
      concept: sceneState.data.concept,
      nodeIds: sceneState.data.nodes.map((node) => node.id)
    });
    setRuntimeController(createRuntimeControllerState(sceneState.data));
    setShowLog(false);
  }, [sceneState.data?.sceneId]);

  const currentNode = getCurrentStoryNode(runtimeScene, runtimeState);
  const currentExerciseState =
    currentNode?.kind === "exercise"
      ? getRuntimeExerciseState(runtimeState.exerciseStateByNode, currentNode.id)
      : null;
  const progress = getRuntimeProgress(runtimeScene, runtimeState);
  const history = getRuntimeBacklog(runtimeState);
  const currentExercise = currentNode?.kind === "exercise" ? currentNode.exercise : null;
  const exerciseContext = currentNode
    ? `course=${course.courseId} · chapter=${chapter.chapterId} · step=${runtimeScene.stepId} · node=${currentNode.id} · kind=${currentNode.kind}`
    : `course=${course.courseId} · chapter=${chapter.chapterId} · step=${runtimeScene.stepId} · node=missing`;
  const { unsupportedReason } = resolveExercise(currentExercise);
  const nextStepBlockedReason = currentNode?.kind === "exercise" && currentExerciseState ? getExerciseBlockedReason(currentNode, currentExerciseState) : currentNode ? null : "No scene content loaded";
  const nextStepHint =
    nextStepBlockedReason ??
    (runtimeState.cursor.nodeIndex === runtimeScene.nodes.length - 1
      ? "Checkpoint complete. You can finish this step."
      : "Checkpoint complete. Continue to the next scene.");

  React.useEffect(() => {
    if (currentNode?.kind !== "exercise" || !unsupportedReason) {
      return;
    }
    toast.error("Unsupported checkpoint in route player", {
      description: `${unsupportedReason}. ${exerciseContext}`
    });
  }, [currentNode?.kind, exerciseContext, unsupportedReason]);

  const notifyBlockedAdvance = React.useCallback(
    (reason: string) => {
      toast.warning("Checkpoint cannot continue yet", {
        description: `${reason}. ${exerciseContext}`
      });
    },
    [exerciseContext]
  );

  const applyRuntimeAction = React.useCallback(
    (action: StoryRuntimeAction) => {
      setRuntimeController((current) => {
        const evaluation = evaluateStoryRuntimeAction(runtimeScene, current.runtimeState, action);
        debugStoryRuntime("apply-action", {
          action,
          sceneId: runtimeScene.sceneId,
          beforeCursor: current.runtimeState.cursor,
          beforeHistorySize: current.runtimeState.history.length,
          afterCursor: evaluation.state.cursor,
          afterHistorySize: evaluation.state.history.length,
          blockedReason: evaluation.blockedReason,
          didAdvance: evaluation.didAdvance,
          didCompleteScene: evaluation.didCompleteScene,
          feedback: evaluation.feedback
        });
        return {
          runtimeState: evaluation.state,
          lastEvaluation: evaluation,
          revision: current.revision + 1
        };
      });
    },
    [runtimeScene]
  );

  React.useEffect(() => {
    const evaluation = runtimeController.lastEvaluation;
    if (!evaluation) {
      return;
    }

    if (evaluation.feedback) {
      if (evaluation.feedback.level === "warning") {
        toast.warning(evaluation.feedback.title, { description: evaluation.feedback.description });
      } else {
        toast.message(evaluation.feedback.title, { description: evaluation.feedback.description });
      }
    }

    if (evaluation.blockedReason) {
      notifyBlockedAdvance(evaluation.blockedReason);
    }

    if (evaluation.didCompleteScene) {
      onComplete();
    }
  }, [notifyBlockedAdvance, onComplete, runtimeController.lastEvaluation, runtimeController.revision]);

  const handleTextExerciseSubmit = React.useCallback(() => {
    if (currentNode?.kind !== "exercise") {
      return;
    }
    applyRuntimeAction({ type: "submit-exercise", nodeId: currentNode.id });
  }, [applyRuntimeAction, currentNode]);

  const handleNext = React.useCallback(() => {
    if (!currentNode) {
      return;
    }
    applyRuntimeAction({ type: "advance" });
  }, [applyRuntimeAction, currentNode]);

  const handlePrev = React.useCallback(() => {
    debugStoryRuntime("back-click", {
      sceneId: runtimeScene.sceneId,
      cursor: runtimeState.cursor,
      historySize: runtimeState.history.length
    });
    if (runtimeState.history.length <= 1) {
      return;
    }
    applyRuntimeAction({ type: "back" });
  }, [applyRuntimeAction, runtimeScene.sceneId, runtimeState.cursor, runtimeState.history.length]);

  const selectionBlocksAdvance = React.useCallback((container: HTMLDivElement | null) => {
    if (!container) {
      return false;
    }
    const selection = window.getSelection();
    if (!selection || selection.isCollapsed || selection.toString().trim().length === 0) {
      return false;
    }
    const anchorNode = selection.anchorNode;
    const focusNode = selection.focusNode;
    return Boolean(anchorNode && focusNode && container.contains(anchorNode) && container.contains(focusNode));
  }, []);

  const handleDialogPointerDown = React.useCallback((event: React.PointerEvent<HTMLDivElement>) => {
    if (currentNode?.kind === "exercise") {
      return;
    }
    dialogPointerRef.current = {
      x: event.clientX,
      y: event.clientY,
      at: Date.now()
    };
  }, [currentNode?.kind]);

  const handleDialogClick = React.useCallback(
    (event: React.MouseEvent<HTMLDivElement>) => {
      if (currentNode?.kind === "exercise") {
        return;
      }

      const target = event.target instanceof Element ? event.target : null;
      if (target?.closest("button, a, input, textarea, select, label, [role='button'], [data-no-advance='true']")) {
        return;
      }

      if (selectionBlocksAdvance(event.currentTarget)) {
        return;
      }

      const pointer = dialogPointerRef.current;
      if (pointer) {
        const moved = Math.hypot(event.clientX - pointer.x, event.clientY - pointer.y);
        const held = Date.now() - pointer.at;
        if (moved > 6 || held > 600) {
          return;
        }
      }

      handleNext();
    },
    [currentNode?.kind, handleNext, selectionBlocksAdvance]
  );

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
      debugStoryRuntime("space-advance", {
        sceneId: runtimeScene.sceneId,
        cursor: runtimeState.cursor,
        historySize: runtimeState.history.length,
        currentNodeId: currentNode?.id ?? null
      });
      handleNext();
    };

    window.addEventListener("keydown", onKeyDown);
    return () => {
      window.removeEventListener("keydown", onKeyDown);
    };
  }, [currentNode?.id, handleNext, runtimeScene.sceneId, runtimeState.cursor, runtimeState.history.length]);

  if (sceneState.loading || !sceneState.data) {
    return (
      <div className="story-overlay">
        <div className="overlay-layer">
          <div className="dialog-shell">
            <LoadingState message={sceneState.error ?? "Loading route scene..."} />
          </div>
        </div>
      </div>
    );
  }

  const parseLines = (line: string, index: number) => {
    const parts = line.split(/(<term id="[^"]+">.*?<\/term>)/g);
    return (
      <p key={`${currentNode?.id ?? "node"}-${index}`}>
        {parts.map((part, partIndex) => {
          const match = part.match(/<term id="([^"]+)">([\s\S]*?)<\/term>/);
          if (!match) {
            return <React.Fragment key={partIndex}>{part}</React.Fragment>;
          }
          const [, id, label] = match;
          const term = runtimeScene.termCatalog[id];
          const labelNode = (
            <button
              key={`token-${partIndex}`}
              className="term-token"
              onClick={(event) => {
                event.stopPropagation();
                if (term) {
                  onOpenTerm(id, term);
                }
              }}
              type="button"
            >
              {label}
            </button>
          );

          if (!term) {
            return labelNode;
          }

          return (
            <GlossaryTooltip key={partIndex} termId={id} term={term}>
              {labelNode}
            </GlossaryTooltip>
          );
        })}
      </p>
    );
  };

  const answeredNote = currentNode?.kind === "exercise" ? getExerciseAnsweredNote(currentNode) : null;

  return (
    <div className="story-overlay">
      <div className="overlay-layer">
        <div className="overlay-controls">
          <div className="overlay-title">
            <span>{course.title}</span>
            <strong>{chapter.lessonId} · {runtimeScene.concept}</strong>
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
          {currentNode?.kind === "exercise" && currentExerciseState ? (
            (() => {
              const exercise = currentNode.exercise;
              return (
                <div className="scene-exercise">
                  <span className="pill accent">Checkpoint</span>
                  <h3>{exercise.prompt ?? currentNode.lines[0] ?? "Quick check"}</h3>
                  <p className="story-note">Answer first, then continue. This keeps the story from becoming passive scrolling.</p>
                  <ExerciseRenderer
                    acceptedAnswers={exercise.acceptedAnswers}
                    answerIndex={exercise.answerIndex ?? Number.NaN}
                    answered={currentExerciseState.answered}
                    exercise={exercise}
                    exerciseContext={exerciseContext}
                    onSelectAnswer={(index) => {
                      if (currentExerciseState.answered || currentNode.kind !== "exercise") {
                        return;
                      }
                      applyRuntimeAction({ type: "select-choice", nodeId: currentNode.id, choiceIndex: index });
                    }}
                    onSubmitTextAnswer={handleTextExerciseSubmit}
                    onTextAnswerChange={(value) => applyRuntimeAction({ type: "set-text-answer", nodeId: currentNode.id, value })}
                    selectedAnswer={currentExerciseState.selectedAnswer}
                    textAnswer={currentExerciseState.textAnswer}
                    textAnswerCorrect={currentExerciseState.textAnswerCorrect}
                    textSubmitted={currentExerciseState.textSubmitted}
                    unsupportedReason={unsupportedReason}
                  />
                  {currentExerciseState.answered && answeredNote ? <p className="story-note">{answeredNote}</p> : null}
                </div>
              );
            })()
          ) : (
            <div className="scene-avatar">✦</div>
          )}
        </div>

        <div
          className="dialog-shell"
          onClick={currentNode?.kind === "exercise" ? undefined : handleDialogClick}
          onPointerDown={currentNode?.kind === "exercise" ? undefined : handleDialogPointerDown}
        >
          <div className="dialog-topline">
            <span className="dialog-speaker">{currentNode?.kind === "exercise" ? "System Test" : "Instructor"}</span>
            <div className="dialog-progress">
              <div className="progress-fill" style={{ width: `${progress}%` }} />
            </div>
          </div>

          <div className="dialog-lines">
            {(currentNode?.lines ?? ["No scene content found."]).map((line, index) => parseLines(line, index))}
          </div>

          <div className="story-footer">
            <div className="topbar-actions">
              <button
                className="pill-button ghost"
                onClick={(event) => {
                  event.stopPropagation();
                  handlePrev();
                }}
              >
                <span className="button-label">
                  <ChevronRight className="button-icon-flip" size={15} />
                  <span>Back</span>
                </span>
              </button>
            </div>
            <div className="story-footer-status" title={nextStepHint}>
              {currentNode?.kind === "exercise"
                ? currentExerciseState?.answered
                  ? "Continue"
                  : nextStepBlockedReason ?? "Answer to Continue"
                : "Tap Anywhere to Continue"}
            </div>
            <div className="topbar-actions">
              <button className="pill-button primary" disabled={!currentNode} onClick={handleNext} title={nextStepHint}>
                <span className="button-label">
                  <span>{runtimeState.cursor.nodeIndex === runtimeScene.nodes.length - 1 ? "Finish Step" : "Next"}</span>
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
              {history.map((visit, index) => (
                <article key={`${visit.cursor.nodeId}-${index}`} className="log-item">
                  <span className="pill">
                    <BookOpen size={13} />
                    <span>Page {index + 1}</span>
                  </span>
                  <p>{visit.lines.join(" ")}</p>
                </article>
              ))}
            </div>
          </aside>
        </div>
      ) : null}
    </div>
  );
}
