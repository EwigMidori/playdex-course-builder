import type { CoverageEntry, QuestionFamily, QuestionVariant } from "../../course-asset-contract";

export type {
  ChapterRecord,
  CoverageEntry,
  CourseIndex,
  CourseRecord,
  CourseStatus,
  CourseValidationError,
  Exercise,
  QuestionBank,
  QuestionFamily,
  QuestionVariant,
  StoryManifest,
  StoryManifestStep,
  StoryScreen,
  StoryStep,
  TermEntry
} from "../../course-asset-contract";

export type QuestionIndex = {
  families: Map<string, QuestionFamily>;
  questions: Map<string, { family: QuestionFamily; variant: QuestionVariant }>;
  flashcards: Array<{
    family: QuestionFamily;
    variant: QuestionVariant;
    chapterId: string;
    chapterTitle: string;
    stepId: string;
  }>;
  quizFamilies: QuestionFamily[];
  longformFamilies: QuestionFamily[];
  coverage: CoverageEntry[];
};

export type ReviewHistoryEntry = {
  seenCount: number;
  lastRating: number;
  updatedAt: number;
};

export type PlayerProgress = {
  completedSteps: Record<string, string[]>;
  lastStepByChapter: Record<string, string>;
  reviewHistory: Record<string, ReviewHistoryEntry>;
};

export type ContinueTarget = {
  chapterId: string;
  stepId: string;
};

export type CourseSummary = {
  totalSteps: number;
  completedSteps: number;
  mastery: number;
  chapters: Array<{
    chapterId: string;
    totalSteps: number;
    completedSteps: number;
    completionRatio: number;
  }>;
};
