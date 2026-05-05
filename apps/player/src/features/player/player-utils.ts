import type {
  ContinueTarget,
  CourseRecord,
  PlayerProgress,
  StoryManifest,
  StoryManifestStep
} from "./player-model";

export const EMPTY_COURSES: CourseRecord[] = [];
export const EMPTY_MANIFESTS: Record<string, StoryManifest> = {};
export const EMPTY_STEPS: StoryManifestStep[] = [];

export const chapterKey = (courseId: string, chapterId: string) => `${courseId}:${chapterId}`;

export const safeColor = (value?: string, fallback = "#6ee7b7") => value ?? fallback;

export function isCourseReady(course: CourseRecord) {
  return course.status === "ready";
}

export function courseUnavailableMessage(course: CourseRecord) {
  return course.validationErrors?.[0]?.message ?? "Course is blocked by backend catalog validation.";
}

export function getCourseContinueTarget(
  course: CourseRecord | null,
  manifests: Record<string, StoryManifest>,
  progress: PlayerProgress
): ContinueTarget | null {
  if (!course) {
    return null;
  }

  let fallback: ContinueTarget | null = null;

  for (const chapter of course.chapters) {
    const manifest = manifests[chapterKey(course.courseId, chapter.chapterId)];
    const completed = new Set(progress.completedSteps[chapterKey(course.courseId, chapter.chapterId)] ?? []);

    for (const step of manifest?.steps ?? EMPTY_STEPS) {
      if (!step.sequence_id) {
        continue;
      }

      fallback = { chapterId: chapter.chapterId, stepId: step.sequence_id };
      if (!completed.has(step.sequence_id)) {
        return fallback;
      }
    }
  }

  return fallback;
}

export function normalizeExerciseText(value: string) {
  return value.trim().toLowerCase().split(/\s+/).filter(Boolean).join(" ");
}
