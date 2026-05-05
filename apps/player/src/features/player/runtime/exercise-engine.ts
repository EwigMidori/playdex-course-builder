import { resolveExercise, type ExerciseSubmissionResult } from "../exercise/exercise-registry";
import { normalizeExerciseText } from "../player-utils";
import type { ExerciseNode, RuntimeExerciseState, SceneModel } from "./story-runtime-types";

export function createRuntimeExerciseState(): RuntimeExerciseState {
  return {
    answered: false,
    selectedAnswer: null,
    textAnswer: "",
    textSubmitted: null,
    textAnswerCorrect: null
  };
}

export function getRuntimeExerciseState(
  exerciseStateByNode: Record<string, RuntimeExerciseState>,
  nodeId: string
): RuntimeExerciseState {
  return exerciseStateByNode[nodeId] ?? createRuntimeExerciseState();
}

function applySubmissionResult(
  baseState: RuntimeExerciseState,
  result: ExerciseSubmissionResult
) {
  return {
    answered: typeof result?.answered === "boolean" ? result.answered : baseState.answered,
    selectedAnswer: typeof result?.selectedAnswer !== "undefined" ? result.selectedAnswer : baseState.selectedAnswer,
    textAnswer: baseState.textAnswer,
    textSubmitted: typeof result?.textSubmitted !== "undefined" ? result.textSubmitted : baseState.textSubmitted,
    textAnswerCorrect:
      typeof result?.textAnswerCorrect !== "undefined" ? result.textAnswerCorrect : baseState.textAnswerCorrect
  };
}

function buildExerciseContext(scene: SceneModel, node: ExerciseNode) {
  return `course=${scene.courseId ?? "unknown-course"} · chapter=${scene.chapterId} · step=${scene.stepId} · node=${node.id} · kind=${node.exercise.kind ?? "missing"}`;
}

export function getExerciseBlockedReason(node: ExerciseNode, exerciseState: RuntimeExerciseState) {
  const { definition, unsupportedReason } = resolveExercise(node.exercise);
  if (unsupportedReason) {
    return unsupportedReason;
  }
  return definition?.getBlockedReason(node.exercise, exerciseState.answered) ?? null;
}

export function getExerciseAnsweredNote(node: ExerciseNode) {
  const { definition } = resolveExercise(node.exercise);
  return definition?.getAnsweredNote(node.exercise) ?? node.exercise.explanation ?? "Checkpoint complete. Continue the route.";
}

export function submitExerciseChoice(
  scene: SceneModel,
  node: ExerciseNode,
  exerciseState: RuntimeExerciseState,
  choiceIndex: number
) {
  const { definition, unsupportedReason } = resolveExercise(node.exercise);
  if (!definition?.selectChoice || unsupportedReason) {
    return {
      nextState: exerciseState,
      feedback: null
    };
  }

  const result = definition.selectChoice(node.exercise, choiceIndex, node.exercise.answerIndex ?? Number.NaN);
  return {
    nextState: applySubmissionResult(exerciseState, result),
    feedback: result.feedback ?? null,
    exerciseContext: buildExerciseContext(scene, node)
  };
}

export function submitExerciseText(scene: SceneModel, node: ExerciseNode, exerciseState: RuntimeExerciseState) {
  const { definition, unsupportedReason } = resolveExercise(node.exercise);
  if (!definition?.submitText || unsupportedReason) {
    return {
      nextState: exerciseState,
      feedback: null
    };
  }

  const normalizedAcceptedAnswers = node.exercise.acceptedAnswers.map(normalizeExerciseText).filter(Boolean);
  const result = definition.submitText({
    exercise: node.exercise,
    textAnswer: exerciseState.textAnswer,
    acceptedAnswers: node.exercise.acceptedAnswers,
    normalizedAcceptedAnswers,
    exerciseContext: buildExerciseContext(scene, node)
  });

  return {
    nextState: applySubmissionResult(exerciseState, result),
    feedback: result.feedback ?? null,
    exerciseContext: buildExerciseContext(scene, node)
  };
}
