import { atom } from "jotai";
import { atomWithStorage, createJSONStorage } from "jotai/utils";
import type { PlayerProgress, ReviewHistoryEntry } from "./player-model";

const PROGRESS_STORAGE_KEY = "playdex-player-progress-v1";

const chapterKey = (courseId: string, chapterId: string) => `${courseId}:${chapterId}`;

const defaultProgress = (): PlayerProgress => ({
  completedSteps: {},
  lastStepByChapter: {},
  reviewHistory: {}
});

function normalizeProgress(value: unknown): PlayerProgress {
  if (!value || typeof value !== "object") {
    return defaultProgress();
  }

  const parsed = value as Partial<PlayerProgress>;
  return {
    completedSteps: parsed.completedSteps ?? {},
    lastStepByChapter: parsed.lastStepByChapter ?? {},
    reviewHistory: parsed.reviewHistory ?? {}
  };
}

const jsonStorage = createJSONStorage<unknown>(() => window.localStorage);

const progressStorage = {
  getItem: (key: string, initialValue: PlayerProgress) => normalizeProgress(jsonStorage.getItem(key, initialValue)),
  setItem: (key: string, value: PlayerProgress) => jsonStorage.setItem(key, value),
  removeItem: (key: string) => jsonStorage.removeItem(key)
};

export const playerProgressAtom = atomWithStorage<PlayerProgress>(PROGRESS_STORAGE_KEY, defaultProgress(), progressStorage);

export const markStepCompleteAtom = atom(
  null,
  (get, set, payload: { courseId: string; chapterId: string; stepId: string }) => {
    const key = chapterKey(payload.courseId, payload.chapterId);
    const current = get(playerProgressAtom);
    const completed = new Set(current.completedSteps[key] ?? []);
    completed.add(payload.stepId);
    set(playerProgressAtom, {
      ...current,
      completedSteps: {
        ...current.completedSteps,
        [key]: [...completed]
      },
      lastStepByChapter: {
        ...current.lastStepByChapter,
        [key]: payload.stepId
      }
    });
  }
);

export const rememberStepAtom = atom(
  null,
  (get, set, payload: { courseId: string; chapterId: string; stepId: string }) => {
    const key = chapterKey(payload.courseId, payload.chapterId);
    const current = get(playerProgressAtom);
    set(playerProgressAtom, {
      ...current,
      lastStepByChapter: {
        ...current.lastStepByChapter,
        [key]: payload.stepId
      }
    });
  }
);

export const rateReviewAtom = atom(
  null,
  (get, set, payload: { questionId: string; rating: number }) => {
    const current = get(playerProgressAtom);
    const previous = current.reviewHistory[payload.questionId];
    const nextEntry: ReviewHistoryEntry = {
      seenCount: (previous?.seenCount ?? 0) + 1,
      lastRating: payload.rating,
      updatedAt: Date.now()
    };
    set(playerProgressAtom, {
      ...current,
      reviewHistory: {
        ...current.reviewHistory,
        [payload.questionId]: nextEntry
      }
    });
  }
);
