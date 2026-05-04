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

export class ViewerAssetPaths {
  static textbook(lesson: LessonOption) {
    return lesson.paths.textbookPath;
  }

  static manifest(lesson: LessonOption) {
    return `${lesson.paths.guidedStoryDir}/manifest.json`;
  }

  static step(lesson: LessonOption, stepId: string) {
    return `${lesson.paths.guidedStoryDir}/${stepId}/step.json`;
  }

  static questionBank(lesson: LessonOption, stepId: string) {
    return `${lesson.paths.guidedStoryDir}/${stepId}/question_bank.json`;
  }

  static relevance(lesson: LessonOption) {
    return lesson.paths.relevancePath;
  }

  static plainArtifactsDir(lesson: LessonOption) {
    return this.normalizeRelative(lesson.paths.plainArtifactsDir);
  }

  static normalize(path: string) {
    return path.startsWith("/") ? path : `/${path}`;
  }

  static normalizeRelative(path: string) {
    return path.replace(/^\/+/, "");
  }

  static chapterSelectionKey(courseId: string, chapterId: string) {
    return `${courseId}:${chapterId}`;
  }

  static defaultPlainOutputDir(lessonId: LessonId) {
    return `research/pipeline/1-plain/${lessonId}`;
  }

  static defaultTextbookPath(lessonId: LessonId) {
    return `research/pipeline/5-textbook/${lessonId}.mdx`;
  }

  static defaultRelevancePath(lessonId: LessonId) {
    return `research/pipeline/meta/${lessonId}/relevance/report.json`;
  }
}

export class CourseCatalogBuilder {
  static build(index: CourseIndex): CourseCatalog {
    const courses = index.courses.map((course) => {
      const chapters = [...course.chapters]
        .sort((left, right) => {
          const leftOrder = typeof left.order === "number" ? left.order : Number.MAX_SAFE_INTEGER;
          const rightOrder = typeof right.order === "number" ? right.order : Number.MAX_SAFE_INTEGER;
          return leftOrder - rightOrder || left.chapterId.localeCompare(right.chapterId);
        })
        .map((chapter) => this.lessonFromIndex(course, chapter));
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
  }

  private static lessonFromIndex(course: CourseIndexEntry, chapter: ChapterIndexEntry): LessonOption {
    const lessonId = chapter.lessonId;
    const plainOutputDir = chapter.plainOutputDir ?? ViewerAssetPaths.defaultPlainOutputDir(lessonId);
    return {
      id: ViewerAssetPaths.chapterSelectionKey(course.courseId, chapter.chapterId),
      lessonId,
      courseId: course.courseId,
      chapterId: chapter.chapterId,
      courseLabel: this.toCourseLabel(course),
      courseTitle: course.title,
      chapterLabel: this.toChapterLabel(chapter),
      chapterTitle: chapter.title,
      label: `${lessonId} - ${chapter.title}`,
      defaultStep: "step1",
      paths: {
        plainOutputDir,
        guidedStoryDir: chapter.guidedStoryDir,
        textbookPath: chapter.textbookPath ?? ViewerAssetPaths.defaultTextbookPath(lessonId),
        relevancePath: chapter.metaDir
          ? `${chapter.metaDir}/relevance/report.json`
          : ViewerAssetPaths.defaultRelevancePath(lessonId),
        plainArtifactsDir: `${plainOutputDir}/artifacts`
      }
    };
  }

  private static toCourseLabel(entry: CourseIndexEntry) {
    return entry.courseId.toUpperCase();
  }

  private static toChapterLabel(chapter: ChapterIndexEntry) {
    return typeof chapter.order === "number" ? `Lecture ${chapter.order}` : chapter.chapterId;
  }
}

export class ViewerAssetClient {
  static async fetchText(path: string): Promise<Loadable<string>> {
    const url = ViewerAssetPaths.normalize(path);
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

  static async loadCourseCatalog(): Promise<Loadable<CourseCatalog>> {
    const index = await this.fetchJson<CourseIndex>(COURSE_INDEX_PATH);
    if (!index.data?.courses?.length) {
      return {
        data: null,
        error: index.error ?? "course-index.json did not contain any courses",
        path: index.path
      };
    }

    return {
      data: CourseCatalogBuilder.build(index.data),
      error: null,
      path: index.path
    };
  }

  static loadManifest(lesson: LessonOption) {
    return this.fetchJson<StoryManifest>(ViewerAssetPaths.manifest(lesson));
  }

  static loadStep(lesson: LessonOption, stepId: string): Promise<Loadable<StoryStep>> {
    return this.fetchJson<StoryStep>(ViewerAssetPaths.step(lesson, stepId));
  }

  static loadQuestionBank(lesson: LessonOption, stepId: string): Promise<Loadable<QuestionBank>> {
    return this.fetchJson<QuestionBank>(ViewerAssetPaths.questionBank(lesson, stepId));
  }

  static loadRelevance(lesson: LessonOption) {
    return this.fetchJson<RelevanceReport>(ViewerAssetPaths.relevance(lesson));
  }
}
