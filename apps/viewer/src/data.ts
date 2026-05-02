import type {
  ChapterIndexEntry,
  CourseCatalog,
  CourseIndex,
  CourseIndexEntry,
  LessonId,
  LessonOption,
  QuestionBank,
  RelevanceReport,
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

const normalizeRelativePath = (path: string) => path.replace(/^\/+/, "");

const chapterSelectionKey = (courseId: string, chapterId: string) => `${courseId}:${chapterId}`;

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

const toCourseLabel = (entry: CourseIndexEntry) => entry.courseId.toUpperCase();

const toChapterLabel = (chapter: ChapterIndexEntry) =>
  typeof chapter.order === "number" ? `Lecture ${chapter.order}` : chapter.chapterId;

const defaultPlainOutputDir = (lessonId: LessonId) => `research/pipeline/1-plain/${lessonId}`;
const defaultTextbookPath = (lessonId: LessonId) => `research/pipeline/5-textbook/${lessonId}.mdx`;
const defaultRelevancePath = (lessonId: LessonId) => `research/pipeline/meta/${lessonId}/relevance/report.json`;

const lessonFromIndex = (course: CourseIndexEntry, chapter: ChapterIndexEntry): LessonOption => {
  const lessonId = chapter.lessonId;
  const plainOutputDir = chapter.plainOutputDir ?? defaultPlainOutputDir(lessonId);
  return {
    id: chapterSelectionKey(course.courseId, chapter.chapterId),
    lessonId,
    courseId: course.courseId,
    chapterId: chapter.chapterId,
    courseLabel: toCourseLabel(course),
    courseTitle: course.title,
    chapterLabel: toChapterLabel(chapter),
    chapterTitle: chapter.title,
    label: `${lessonId} - ${chapter.title}`,
    defaultStep: "step1",
    paths: {
      plainOutputDir,
      guidedStoryDir: chapter.guidedStoryDir,
      textbookPath: chapter.textbookPath ?? defaultTextbookPath(lessonId),
      relevancePath: chapter.metaDir ? `${chapter.metaDir}/relevance/report.json` : defaultRelevancePath(lessonId),
      plainArtifactsDir: `${plainOutputDir}/artifacts`
    }
  };
};

const buildCatalogFromIndex = (index: CourseIndex): CourseCatalog => {
  const courses = index.courses.map((course) => {
    const chapters = [...course.chapters]
      .sort((left, right) => {
        const leftOrder = typeof left.order === "number" ? left.order : Number.MAX_SAFE_INTEGER;
        const rightOrder = typeof right.order === "number" ? right.order : Number.MAX_SAFE_INTEGER;
        return leftOrder - rightOrder || left.chapterId.localeCompare(right.chapterId);
      })
      .map((chapter) => lessonFromIndex(course, chapter));
    return {
      id: course.courseId,
      title: course.title,
      category: course.category,
      brandColor: course.brandColor,
      chapters
    };
  });

  return {
    courses,
    lessons: courses.flatMap((course) => course.chapters)
  };
};

export async function loadCourseCatalog(): Promise<Loadable<CourseCatalog>> {
  const index = await fetchJson<CourseIndex>(COURSE_INDEX_PATH);
  if (!index.data?.courses?.length) {
    return {
      data: null,
      error: index.error ?? "course-index.json did not contain any courses",
      path: index.path
    };
  }

  return {
    data: buildCatalogFromIndex(index.data),
    error: null,
    path: index.path
  };
}

export const textbookPath = (lesson: LessonOption) => lesson.paths.textbookPath;
export const manifestPath = (lesson: LessonOption) => `${lesson.paths.guidedStoryDir}/manifest.json`;
export const stepPath = (lesson: LessonOption, stepId: string) => `${lesson.paths.guidedStoryDir}/${stepId}/step.json`;
export const questionBankPath = (lesson: LessonOption, stepId: string) =>
  `${lesson.paths.guidedStoryDir}/${stepId}/question_bank.json`;
export const relevancePath = (lesson: LessonOption) => lesson.paths.relevancePath;
export const plainArtifactsDir = (lesson: LessonOption) => normalizeRelativePath(lesson.paths.plainArtifactsDir);

export const loadManifest = (lesson: LessonOption) => fetchJson<StoryManifest>(manifestPath(lesson));

export const loadStep = (lesson: LessonOption, stepId: string): Promise<Loadable<StoryStep>> =>
  fetchJson<StoryStep>(stepPath(lesson, stepId));

export const loadQuestionBank = (
  lesson: LessonOption,
  stepId: string
): Promise<Loadable<QuestionBank>> => fetchJson<QuestionBank>(questionBankPath(lesson, stepId));

export const loadRelevance = (lesson: LessonOption) => fetchJson<RelevanceReport>(relevancePath(lesson));
