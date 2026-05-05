import type { Exercise, TermEntry } from "../player-model";
import type { ExerciseFeedback } from "../exercise/exercise-registry";

export type SceneDescriptor = {
  sceneId: string;
  courseId: string | null;
  chapterId: string;
  lessonId: string | null;
  stepId: string;
  concept: string;
};

export type NormalizedExercise = {
  kind: string | null;
  prompt: string | null;
  options: string[];
  acceptedAnswers: string[];
  answerIndex: number | null;
  explanation: string | null;
  raw: Exercise;
};

type BaseStoryNode = {
  id: string;
  kind: "narration" | "exercise";
  lines: string[];
  termRefs: string[];
  rawScreenIndex: number;
  rawScreenType: string | null;
};

export type NarrationNode = BaseStoryNode & {
  kind: "narration";
};

export type ExerciseNode = BaseStoryNode & {
  kind: "exercise";
  exercise: NormalizedExercise;
};

export type StoryNode = NarrationNode | ExerciseNode;

export type SceneModel = SceneDescriptor & {
  termCatalog: Record<string, TermEntry>;
  nodes: StoryNode[];
};

export type StoryCursor = {
  sceneId: string;
  nodeIndex: number;
  nodeId: string;
};

export type NodeVisit = {
  cursor: StoryCursor;
  kind: StoryNode["kind"];
  lines: string[];
  enteredAt: number;
};

export type RuntimeExerciseState = {
  answered: boolean;
  selectedAnswer: number | null;
  textAnswer: string;
  textSubmitted: string | null;
  textAnswerCorrect: boolean | null;
};

export type StoryRuntimeState = {
  cursor: StoryCursor;
  history: NodeVisit[];
  exerciseStateByNode: Record<string, RuntimeExerciseState>;
  readNodeIds: Record<string, true>;
  completedSceneIds: Record<string, true>;
};

export type StoryRuntimeAction =
  | { type: "enter-scene" }
  | { type: "advance" }
  | { type: "back" }
  | { type: "select-choice"; nodeId: string; choiceIndex: number }
  | { type: "set-text-answer"; nodeId: string; value: string }
  | { type: "submit-exercise"; nodeId: string };

export type StoryRuntimeEvaluation = {
  state: StoryRuntimeState;
  blockedReason: string | null;
  feedback: ExerciseFeedback | null;
  didAdvance: boolean;
  didCompleteScene: boolean;
};
