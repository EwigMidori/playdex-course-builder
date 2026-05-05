import type { Exercise } from "../player-model";
import { normalizeExerciseText } from "../player-utils";
import { FillInBlank } from "./FillInBlank";
import { ShortReflection } from "./ShortReflection";
import { SingleChoice } from "./SingleChoice";

export type ExerciseRendererProps = {
  exercise: Exercise;
  answered: boolean;
  selectedAnswer: number | null;
  textAnswer: string;
  textSubmitted: string | null;
  textAnswerCorrect: boolean | null;
  acceptedAnswers: string[];
  answerIndex: number;
  onSelectAnswer: (index: number) => void;
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
  textSubmitted?: string | null;
  textAnswerCorrect?: boolean | null;
  feedback?: ExerciseFeedback;
};

type ExerciseDefinition = {
  Component: React.ComponentType<ExerciseRendererProps>;
  validate: (exercise: Exercise) => string | null;
  getBlockedReason: (exercise: Exercise, answered: boolean) => string | null;
  getAnsweredNote: (exercise: Exercise) => string;
  selectChoice?: (exercise: Exercise, choiceIndex: number, answerIndex: number) => ExerciseSubmissionResult;
  submitText?: (args: {
    exercise: Exercise;
    textAnswer: string;
    acceptedAnswers: string[];
    normalizedAcceptedAnswers: string[];
    exerciseContext: string;
  }) => ExerciseSubmissionResult;
};

const registry: Record<string, ExerciseDefinition> = {
  single_choice: {
    Component: SingleChoice,
    validate: (exercise) => ((exercise.options?.length ?? 0) > 0 ? null : "single_choice exercise has no options"),
    getBlockedReason: (_exercise, answered) => (answered ? null : "Answer this checkpoint before continuing"),
    getAnsweredNote: (exercise) => exercise.explanation ?? "Checkpoint complete. Continue the route.",
    selectChoice: (_exercise, choiceIndex, answerIndex) => ({
      answered: true,
      selectedAnswer: choiceIndex,
      textSubmitted: null,
      textAnswerCorrect: choiceIndex === answerIndex
    })
  },
  fill_in_blank: {
    Component: FillInBlank,
    validate: (exercise) => ((exercise.answers?.length ?? 0) > 0 ? null : "fill_in_blank exercise has no accepted answers"),
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
        textAnswerCorrect: isCorrect,
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
        textAnswerCorrect: null
      };
    }
  }
};

export function resolveExercise(exercise: Exercise | undefined) {
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

  const unsupportedReason = definition.validate(exercise ?? {});
  return {
    definition,
    Component: unsupportedReason ? null : definition.Component,
    unsupportedReason
  };
}
