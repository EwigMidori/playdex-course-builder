export type LessonId = "L1" | "L2";
export type AssetView = "overview" | "textbook" | "story" | "questions" | "relevance";

export type LessonOption = {
  id: LessonId;
  label: string;
  courseLabel: string;
  lectureLabel: string;
  defaultStep: string;
  legacySteps?: string[];
};

export type StoryManifest = {
  lesson_id?: string;
  mode?: string;
  steps?: Array<{
    concept?: string;
    file?: string;
    sequence_id?: string;
  }>;
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
  focus_terms?: string[];
  introduced_terms?: string[];
  exercise?: Exercise;
  terms?: unknown;
  exercises?: unknown;
  [key: string]: unknown;
};

export type StoryStep = {
  lesson_id?: string;
  mode?: string;
  screens?: StoryScreen[];
  [key: string]: unknown;
};

export type QuestionVariant = {
  question_id?: string;
  front?: string;
  back?: string;
  stem?: string;
  options?: string[];
  answer?: number | string;
  explanation?: string;
  estimated_seconds?: number;
  prompt_blocks?: string[];
  reference_answer?: string[];
  rubric_points?: string[];
  [key: string]: unknown;
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
  [key: string]: unknown;
};

export type QuestionBank = {
  lesson_id?: string;
  coverage_map?: Array<{
    coverage_tag?: string;
    coverage_id?: string;
    description?: string;
    covered_by?: string[];
  }>;
  flashcard_families?: QuestionFamily[];
  quiz_families?: QuestionFamily[];
  longform_families?: QuestionFamily[];
  [key: string]: unknown;
};

export type RelevanceScore = {
  recommended_treatment?: string;
  importance?: number;
  course_relevance?: number;
  exam_relevance?: number;
  reason?: string;
  title?: string;
  sequence_id?: string;
  family_id?: string;
  coverage_id?: string;
  section_id?: string;
};

export type RelevanceReport = {
  lesson_id?: string;
  coverage_scores?: RelevanceScore[];
  question_family_scores?: RelevanceScore[];
  step_scores?: RelevanceScore[];
  textbook_section_scores?: RelevanceScore[];
  [key: string]: unknown;
};
