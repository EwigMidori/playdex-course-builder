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

export class CourseAssetPaths {
  static manifest(chapter: ChapterRecord) {
    return `${chapter.guidedStoryDir}/manifest.json`;
  }

  static step(chapter: ChapterRecord, stepId: string) {
    return `${chapter.guidedStoryDir}/${stepId}/step.json`;
  }

  static questionBank(chapter: ChapterRecord, stepId: string) {
    return `${chapter.guidedStoryDir}/${stepId}/question_bank.json`;
  }

  static textbook(chapter: ChapterRecord) {
    return chapter.textbookPath ?? `research/pipeline/5-textbook/${chapter.lessonId}.mdx`;
  }

  static normalize(path: string) {
    return path.startsWith("/") ? path : `/${path}`;
  }
}

export class CourseAssetClient {
  static async fetchText(path: string): Promise<Loadable<string>> {
    const url = CourseAssetPaths.normalize(path);
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

  static async fetchJson<T>(path: string): Promise<Loadable<T>> {
    const text = await this.fetchText(path);
    if (text.error || text.data == null) {
      return { data: null, error: text.error, path };
    }
    try {
      return { data: JSON.parse(text.data) as T, error: null, path };
    } catch (error) {
      return { data: null, error: error instanceof Error ? error.message : String(error), path };
    }
  }

  static loadCourseIndex() {
    return this.fetchJson<CourseIndex>(COURSE_INDEX_PATH);
  }

  static loadManifest(chapter: ChapterRecord) {
    return this.fetchJson<StoryManifest>(CourseAssetPaths.manifest(chapter));
  }

  static loadStep(chapter: ChapterRecord, stepId: string) {
    return this.fetchJson<StoryStep>(CourseAssetPaths.step(chapter, stepId));
  }

  static loadQuestionBank(chapter: ChapterRecord, stepId: string) {
    return this.fetchJson<QuestionBank>(CourseAssetPaths.questionBank(chapter, stepId));
  }

  static loadTextbook(chapter: ChapterRecord) {
    return this.fetchText(CourseAssetPaths.textbook(chapter));
  }
}
