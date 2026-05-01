import { atom } from "jotai";
import type { AssetView, DataMode, LessonId, LessonOption } from "./types";

export const lessons: LessonOption[] = [
  {
    id: "L2",
    label: "L2 - data scraping",
    courseLabel: "Generated course",
    lectureLabel: "Lecture 2",
    defaultStep: "step1"
  },
  {
    id: "L1",
    label: "L1 - algorithmic trading",
    courseLabel: "Generated course",
    lectureLabel: "Lecture 1",
    defaultStep: "step1",
    legacySteps: ["step1", "step2", "step3", "step4", "step5", "step6", "step7", "step8"]
  }
];

export const selectedLessonAtom = atom<LessonId>("L2");
export const selectedAssetViewAtom = atom<AssetView>("overview");
export const selectedStepAtom = atom<string>("step1");
export const selectedDataModeAtom = atom<DataMode>("real");
