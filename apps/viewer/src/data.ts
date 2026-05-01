import { buildStressManifest, buildStressQuestionBank, buildStressRelevance, buildStressStep, buildStressTextbook } from "./demoData";
import type { DataMode, LessonId, QuestionBank, RelevanceReport, StoryManifest, StoryStep } from "./types";

export type Loadable<T> = {
  data: T | null;
  error: string | null;
  path: string;
};

const normalizePath = (path: string) => (path.startsWith("/") ? path : `/${path}`);

export async function fetchText(path: string): Promise<Loadable<string>> {
  const url = normalizePath(path);
  try {
    const response = await fetch(url);
    if (!response.ok) {
      return { data: null, error: `${response.status} ${response.statusText}`, path };
    }
    return { data: await response.text(), error: null, path };
  } catch (error) {
    return { data: null, error: error instanceof Error ? error.message : String(error), path };
  }
}

export async function fetchJson<T>(path: string): Promise<Loadable<T>> {
  const text = await fetchText(path);
  if (text.error || text.data == null) {
    return { data: null, error: text.error, path };
  }
  try {
    return { data: JSON.parse(text.data) as T, error: null, path };
  } catch (error) {
    return { data: null, error: error instanceof Error ? error.message : String(error), path };
  }
}

export const textbookPath = (lessonId: LessonId) => `research/pipeline/5-textbook/${lessonId}.mdx`;
export const manifestPath = (lessonId: LessonId) => `research/pipeline/3-guided_story/${lessonId}/manifest.json`;
export const newStepPath = (lessonId: LessonId, stepId: string) =>
  `research/pipeline/3-guided_story/${lessonId}/${stepId}/step.json`;
export const legacyStepPath = (lessonId: LessonId, stepId: string) =>
  `research/pipeline/3-guided_story/${lessonId}.${stepId}.json`;
export const newQuestionBankPath = (lessonId: LessonId, stepId: string) =>
  `research/pipeline/3-guided_story/${lessonId}/${stepId}/question_bank.json`;
export const legacyQuestionBankPath = (lessonId: LessonId) =>
  `research/pipeline/4-question_bank/${lessonId}.question_bank.json`;
export const relevancePath = (lessonId: LessonId) => `research/pipeline/meta/${lessonId}/relevance/report.json`;

export async function loadTextbook(lessonId: LessonId, mode: DataMode) {
  if (mode === "stress") {
    return { data: buildStressTextbook(lessonId), error: null, path: `demo/stress/${lessonId}.mdx` };
  }
  return fetchText(textbookPath(lessonId));
}

export async function loadManifest(lessonId: LessonId, legacySteps: string[] = [], mode: DataMode) {
  if (mode === "stress") {
    return { data: buildStressManifest(lessonId), error: null, path: `demo/stress/${lessonId}/manifest.json` };
  }
  const manifest = await fetchJson<StoryManifest>(manifestPath(lessonId));
  if (manifest.data) {
    return manifest;
  }

  return {
    data: {
      lesson_id: lessonId,
      mode: "guided_story",
      steps: legacySteps.map((stepId) => ({
        concept: "Legacy guided story step",
        file: legacyStepPath(lessonId, stepId),
        sequence_id: stepId
      }))
    },
    error: manifest.error ? `Using legacy layout fallback: ${manifest.error}` : null,
    path: manifest.path
  };
}

export async function loadStep(lessonId: LessonId, stepId: string, mode: DataMode): Promise<Loadable<StoryStep>> {
  if (mode === "stress") {
    return { data: buildStressStep(lessonId, stepId), error: null, path: `demo/stress/${lessonId}/${stepId}/step.json` };
  }
  const current = await fetchJson<StoryStep>(newStepPath(lessonId, stepId));
  if (current.data) {
    return current;
  }
  const legacy = await fetchJson<StoryStep>(legacyStepPath(lessonId, stepId));
  if (legacy.data) {
    return legacy;
  }
  return {
    data: null,
    error: `New layout failed (${current.error}); legacy layout failed (${legacy.error})`,
    path: `${current.path} | ${legacy.path}`
  };
}

export async function loadQuestionBank(
  lessonId: LessonId,
  stepId: string,
  mode: DataMode
): Promise<Loadable<QuestionBank>> {
  if (mode === "stress") {
    return {
      data: buildStressQuestionBank(lessonId, stepId),
      error: null,
      path: `demo/stress/${lessonId}/${stepId}/question_bank.json`
    };
  }
  const current = await fetchJson<QuestionBank>(newQuestionBankPath(lessonId, stepId));
  if (current.data) {
    return current;
  }
  const legacy = await fetchJson<QuestionBank>(legacyQuestionBankPath(lessonId));
  if (legacy.data) {
    return legacy;
  }
  return {
    data: null,
    error: `Step question bank missing (${current.error}); legacy bank missing (${legacy.error})`,
    path: `${current.path} | ${legacy.path}`
  };
}

export async function loadRelevance(lessonId: LessonId, mode: DataMode): Promise<Loadable<RelevanceReport>> {
  if (mode === "stress") {
    return { data: buildStressRelevance(lessonId), error: null, path: `demo/stress/${lessonId}/relevance/report.json` };
  }
  return fetchJson<RelevanceReport>(relevancePath(lessonId));
}
