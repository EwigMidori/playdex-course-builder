import type {
  ChapterRecord,
  CourseIndex,
  QuestionBank,
  StoryManifest,
  StoryStep
} from "./types";

export type Loadable<T> = {
  data: T | null;
  error: string | null;
  path: string;
};

const COURSE_INDEX_PATH = "research/pipeline/course-index.json";

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

export const loadCourseIndex = () => fetchJson<CourseIndex>(COURSE_INDEX_PATH);

export const manifestPath = (chapter: ChapterRecord) => `${chapter.guidedStoryDir}/manifest.json`;
export const stepPath = (chapter: ChapterRecord, stepId: string) => `${chapter.guidedStoryDir}/${stepId}/step.json`;
export const questionBankPath = (chapter: ChapterRecord, stepId: string) =>
  `${chapter.guidedStoryDir}/${stepId}/question_bank.json`;
export const textbookPath = (chapter: ChapterRecord) =>
  chapter.textbookPath ?? `research/pipeline/5-textbook/${chapter.lessonId}.mdx`;

export const loadManifest = (chapter: ChapterRecord) => fetchJson<StoryManifest>(manifestPath(chapter));
export const loadStep = (chapter: ChapterRecord, stepId: string) => fetchJson<StoryStep>(stepPath(chapter, stepId));
export const loadQuestionBank = (chapter: ChapterRecord, stepId: string) =>
  fetchJson<QuestionBank>(questionBankPath(chapter, stepId));
export const loadTextbook = (chapter: ChapterRecord) => fetchText(textbookPath(chapter));
