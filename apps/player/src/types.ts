export type CourseIndex = {
  version: number;
  courses: CourseRecord[];
};

export type CourseRecord = {
  courseId: string;
  title: string;
  category?: string;
  brandColor?: string;
  chapters: ChapterRecord[];
};

export type ChapterRecord = {
  chapterId: string;
  title: string;
  order?: number;
  lessonId: string;
  guidedStoryDir: string;
  textbookPath?: string;
  plainOutputDir?: string;
  metaDir?: string;
};

export type StoryManifest = {
  lesson_id?: string;
  course_id?: string;
  chapter_id?: string;
  course_title?: string;
  chapter_title?: string;
  mode?: string;
  steps?: StoryManifestStep[];
};

export type StoryManifestStep = {
  concept?: string;
  file?: string;
  sequence_id?: string;
};

export type Exercise = {
  kind?: string;
  prompt?: string;
  options?: string[];
  answer?: number | string;
  explanation?: string;
};

export type StoryScreen = {
  id?: string;
  type?: string;
  title?: string;
  body?: string;
  lines?: string[];
  introduced_terms?: string[];
  focus_terms?: string[];
  exercise?: Exercise;
  [key: string]: unknown;
};

export type TermEntry = {
  display?: string;
  gloss?: string;
  aliases?: string[];
};

export type StoryStep = {
  lesson_id?: string;
  course_id?: string;
  chapter_id?: string;
  mode?: string;
  sequence_id?: string;
  source?: {
    plain_text?: string;
    related?: string[];
  };
  target_language?: string;
  term_catalog?: Record<string, TermEntry>;
  screens?: StoryScreen[];
};

export type QuestionVariant = {
  question_id?: string;
  front?: string;
  back?: string;
  stem?: string;
  options?: string[];
  answer?: number | string | number[];
  explanation?: string;
  estimated_seconds?: number;
  prompt_blocks?: string[];
  reference_answer?: string[];
  rubric_points?: string[];
};

export type QuestionFamily = {
  family_id?: string;
  concept_key?: string;
  difficulty?: string;
  learning_goal?: string;
  linked_steps?: string[];
  coverage_tags?: string[];
  question_type?: string;
  retrieval_focus?: string;
  term_refs?: Array<{ display?: string; en?: string }>;
  variants?: QuestionVariant[];
};

export type CoverageEntry = {
  coverage_tag?: string;
  coverage_id?: string;
  description?: string;
  covered_by?: string[];
};

export type QuestionBank = {
  lesson_id?: string;
  course_id?: string;
  chapter_id?: string;
  coverage_map?: CoverageEntry[];
  flashcard_families?: QuestionFamily[];
  quiz_families?: QuestionFamily[];
  longform_families?: QuestionFamily[];
};

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
