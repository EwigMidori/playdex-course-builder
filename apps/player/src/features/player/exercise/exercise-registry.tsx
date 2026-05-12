import type { NormalizedExercise } from "../runtime/story-runtime-types";
import { normalizeExerciseText } from "../player-utils";
import { FillInBlank } from "./FillInBlank";
import { MultiSelect } from "./MultiSelect";
import { Ordering } from "./Ordering";
import { ShortReflection } from "./ShortReflection";
import { SingleChoice } from "./SingleChoice";

export type ExerciseRendererProps = {
  exercise: NormalizedExercise;
  answered: boolean;
  selectedAnswer: number | null;
  selectedAnswers: number[];
  orderedChoiceIndices: number[];
  textAnswer: string;
  textSubmitted: string | null;
  answerCorrect: boolean | null;
  acceptedAnswers: string[];
  answerIndex: number;
  answerIndices: number[];
  answerOrder: number[];
  onSelectAnswer: (index: number) => void;
  onToggleChoice: (index: number) => void;
  onSetOrder: (order: number[]) => void;
  onTextAnswerChange: (value: string) => void;
  onSubmitTextAnswer: () => void;
};

export type ExerciseFeedback = {
  level: "warning" | "message";
  title: string;
  description: string;
};

export type ExerciseSubmissionResult = {
  answered?: boolean;
  selectedAnswer?: number | null;
  selectedAnswers?: number[];
  orderedChoiceIndices?: number[];
  textSubmitted?: string | null;
  answerCorrect?: boolean | null;
  feedback?: ExerciseFeedback;
};

type ExerciseDefinition = {
  Component: React.ComponentType<ExerciseRendererProps>;
  validate: (exercise: NormalizedExercise) => string | null;
  getBlockedReason: (exercise: NormalizedExercise, answered: boolean) => string | null;
  getAnsweredNote: (exercise: NormalizedExercise) => string;
  selectChoice?: (exercise: NormalizedExercise, choiceIndex: number, answerIndex: number) => ExerciseSubmissionResult;
  submitSelection?: (args: {
    exercise: NormalizedExercise;
    selectedAnswers: number[];
    answerIndices: number[];
    exerciseContext: string;
  }) => ExerciseSubmissionResult;
  submitOrder?: (args: {
    exercise: NormalizedExercise;
    orderedChoiceIndices: number[];
    answerOrder: number[];
    exerciseContext: string;
  }) => ExerciseSubmissionResult;
  submitText?: (args: {
    exercise: NormalizedExercise;
    textAnswer: string;
    acceptedAnswers: string[];
    normalizedAcceptedAnswers: string[];
    exerciseContext: string;
  }) => ExerciseSubmissionResult;
};

function isValidChoiceIndex(index: number | null, choiceCount: number) {
  return typeof index === "number" && Number.isInteger(index) && index >= 0 && index < choiceCount;
}

function areValidChoiceIndices(indices: number[], choiceCount: number) {
  if (!indices.length) {
    return false;
  }

  const unique = new Set(indices);
  return (
    unique.size === indices.length &&
    indices.every((index) => Number.isInteger(index) && index >= 0 && index < choiceCount)
  );
}

function isValidAnswerOrder(items: string[], answerOrder: number[]) {
  return (
    items.length > 0 &&
    answerOrder.length === items.length &&
    areValidChoiceIndices(answerOrder, items.length)
  );
}

const registry: Record<string, ExerciseDefinition> = {
  single_choice: {
    Component: SingleChoice,
    validate: (exercise) => {
      if (!exercise.options.length) {
        return "single_choice exercise has no options";
      }
      return isValidChoiceIndex(exercise.answerIndex, exercise.options.length)
        ? null
        : "single_choice exercise must define a valid answer index";
    },
    getBlockedReason: (_exercise, answered) => (answered ? null : "Answer this checkpoint before continuing"),
    getAnsweredNote: (exercise) => exercise.explanation ?? "Checkpoint complete. Continue the route.",
    selectChoice: (_exercise, choiceIndex, answerIndex) => ({
      answered: true,
      selectedAnswer: choiceIndex,
      textSubmitted: null,
      answerCorrect: choiceIndex === answerIndex
    })
  },
  multi_select: {
    Component: MultiSelect,
    validate: (exercise) => {
      if (!exercise.options.length) {
        return "multi_select exercise has no options";
      }
      return areValidChoiceIndices(exercise.answerIndices, exercise.options.length)
        ? null
        : "multi_select exercise must define unique answer indices within options";
    },
    getBlockedReason: (_exercise, answered) => (answered ? null : "Submit your selections before continuing"),
    getAnsweredNote: (exercise) => exercise.explanation ?? "Checkpoint complete. Continue the route.",
    submitSelection: ({ selectedAnswers, answerIndices, exerciseContext }) => {
      if (!selectedAnswers.length) {
        return {
          feedback: {
            level: "warning",
            title: "Select at least one answer",
            description: `No options were selected. ${exerciseContext}`
          }
        };
      }

      const normalizedSelected = [...selectedAnswers].sort((left, right) => left - right);
      const normalizedAnswer = [...answerIndices].sort((left, right) => left - right);
      const isCorrect =
        normalizedSelected.length === normalizedAnswer.length &&
        normalizedSelected.every((value, index) => value === normalizedAnswer[index]);

      return {
        answered: true,
        selectedAnswers: normalizedSelected,
        answerCorrect: isCorrect
      };
    }
  },
  fill_in_blank: {
    Component: FillInBlank,
    validate: (exercise) => (exercise.acceptedAnswers.length > 0 ? null : "fill_in_blank exercise has no accepted answers"),
    getBlockedReason: (_exercise, answered) => (answered ? null : "Submit a text answer before continuing"),
    getAnsweredNote: (exercise) => exercise.explanation ?? "Checkpoint complete. Continue the route.",
    submitText: ({ textAnswer, acceptedAnswers, normalizedAcceptedAnswers, exerciseContext }) => {
      const submitted = textAnswer.trim();
      if (!submitted) {
        return {
          feedback: {
            level: "warning",
            title: "Answer required before submission",
            description: `No text was entered. ${exerciseContext}`
          }
        };
      }

      const isCorrect = normalizedAcceptedAnswers.includes(normalizeExerciseText(submitted));
      return {
        answered: true,
        textSubmitted: submitted,
        answerCorrect: isCorrect,
        feedback: isCorrect
          ? undefined
          : {
              level: "message",
              title: "Submitted answer did not match the reference",
              description: `submitted=${submitted} · accepted=${acceptedAnswers.join(" / ")} · ${exerciseContext}`
        }
      };
    }
  },
  ordering: {
    Component: Ordering,
    validate: (exercise) =>
      isValidAnswerOrder(exercise.items, exercise.answerOrder)
        ? null
        : "ordering exercise must define items and a complete answer_order permutation",
    getBlockedReason: (_exercise, answered) => (answered ? null : "Arrange the items and submit before continuing"),
    getAnsweredNote: (exercise) => exercise.explanation ?? "Checkpoint complete. Continue the route.",
    submitOrder: ({ exercise, orderedChoiceIndices, answerOrder, exerciseContext }) => {
      if (orderedChoiceIndices.length !== exercise.items.length) {
        return {
          feedback: {
            level: "warning",
            title: "Complete the ordering first",
            description: `The ordering is incomplete. ${exerciseContext}`
          }
        };
      }

      const isCorrect = orderedChoiceIndices.every((value, index) => value === answerOrder[index]);
      return {
        answered: true,
        orderedChoiceIndices,
        answerCorrect: isCorrect
      };
    }
  },
  short_reflection: {
    Component: ShortReflection,
    validate: () => null,
    getBlockedReason: (_exercise, answered) => (answered ? null : "Submit a reflection before continuing"),
    getAnsweredNote: (exercise) => exercise.explanation ?? "Reflection captured for this step. Continue when ready.",
    submitText: ({ textAnswer, exerciseContext }) => {
      const submitted = textAnswer.trim();
      if (!submitted) {
        return {
          feedback: {
            level: "warning",
            title: "Answer required before submission",
            description: `No text was entered. ${exerciseContext}`
          }
        };
      }

      return {
        answered: true,
        textSubmitted: submitted,
        answerCorrect: null
      };
    }
  }
};

export function resolveExercise(exercise: NormalizedExercise | null | undefined) {
  const kind = exercise?.kind;
  if (!kind) {
    return {
      Component: null,
      unsupportedReason: "Exercise payload is missing `kind`"
    };
  }

  const definition = registry[kind];
  if (!definition) {
    return {
      Component: null,
      unsupportedReason: `Unsupported exercise kind \`${kind}\``
    };
  }

  const unsupportedReason = definition.validate(exercise);
  return {
    definition,
    Component: unsupportedReason ? null : definition.Component,
    unsupportedReason
  };
}
