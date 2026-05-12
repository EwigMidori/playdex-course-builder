import { resolveExercise, type ExerciseSubmissionResult } from "../exercise/exercise-registry";
import { normalizeExerciseText } from "../player-utils";
import type { ExerciseNode, RuntimeExerciseState, SceneModel } from "./story-runtime-types";

export function createRuntimeExerciseState(): RuntimeExerciseState {
  return {
    answered: false,
    selectedAnswer: null,
    selectedAnswers: [],
    orderedChoiceIndices: [],
    textAnswer: "",
    textSubmitted: null,
    answerCorrect: null
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
    selectedAnswers: typeof result?.selectedAnswers !== "undefined" ? result.selectedAnswers : baseState.selectedAnswers,
    orderedChoiceIndices:
      typeof result?.orderedChoiceIndices !== "undefined" ? result.orderedChoiceIndices : baseState.orderedChoiceIndices,
    textAnswer: baseState.textAnswer,
    textSubmitted: typeof result?.textSubmitted !== "undefined" ? result.textSubmitted : baseState.textSubmitted,
    answerCorrect: typeof result?.answerCorrect !== "undefined" ? result.answerCorrect : baseState.answerCorrect
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

export function toggleExerciseChoice(scene: SceneModel, node: ExerciseNode, exerciseState: RuntimeExerciseState, choiceIndex: number) {
  const { definition, unsupportedReason } = resolveExercise(node.exercise);
  if (unsupportedReason || node.exercise.kind !== "multi_select") {
    return {
      nextState: exerciseState,
      feedback: null
    };
  }

  const selected = new Set(exerciseState.selectedAnswers);
  if (selected.has(choiceIndex)) {
    selected.delete(choiceIndex);
  } else {
    selected.add(choiceIndex);
  }

  return {
    nextState: {
      ...exerciseState,
      selectedAnswers: [...selected].sort((left, right) => left - right)
    },
    feedback: null,
    exerciseContext: buildExerciseContext(scene, node)
  };
}

export function setExerciseOrder(scene: SceneModel, node: ExerciseNode, exerciseState: RuntimeExerciseState, order: number[]) {
  const { unsupportedReason } = resolveExercise(node.exercise);
  if (unsupportedReason || node.exercise.kind !== "ordering") {
    return {
      nextState: exerciseState,
      feedback: null
    };
  }

  return {
    nextState: {
      ...exerciseState,
      orderedChoiceIndices: order
    },
    feedback: null,
    exerciseContext: buildExerciseContext(scene, node)
  };
}

export function submitExercise(scene: SceneModel, node: ExerciseNode, exerciseState: RuntimeExerciseState) {
  const { definition, unsupportedReason } = resolveExercise(node.exercise);
  if (unsupportedReason) {
    return {
      nextState: exerciseState,
      feedback: null
    };
  }

  if (definition?.submitSelection) {
    const result = definition.submitSelection({
      exercise: node.exercise,
      selectedAnswers: exerciseState.selectedAnswers,
      answerIndices: node.exercise.answerIndices,
      exerciseContext: buildExerciseContext(scene, node)
    });
    return {
      nextState: applySubmissionResult(exerciseState, result),
      feedback: result.feedback ?? null,
      exerciseContext: buildExerciseContext(scene, node)
    };
  }

  if (definition?.submitOrder) {
    const fallbackOrder = node.exercise.items.map((_, index) => index);
    const result = definition.submitOrder({
      exercise: node.exercise,
      orderedChoiceIndices: exerciseState.orderedChoiceIndices.length ? exerciseState.orderedChoiceIndices : fallbackOrder,
      answerOrder: node.exercise.answerOrder,
      exerciseContext: buildExerciseContext(scene, node)
    });
    return {
      nextState: applySubmissionResult(exerciseState, result),
      feedback: result.feedback ?? null,
      exerciseContext: buildExerciseContext(scene, node)
    };
  }

  if (!definition?.submitText) {
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
