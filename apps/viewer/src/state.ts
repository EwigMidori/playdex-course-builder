import { atom } from "jotai";
import type { LessonId, LessonOption, ViewerTab } from "./types";

export const lessons: LessonOption[] = [
  { id: "L2", label: "L2 - data scraping", defaultStep: "step1" },
  {
    id: "L1",
    label: "L1 - algorithmic trading",
    defaultStep: "step1",
    legacySteps: ["step1", "step2", "step3", "step4", "step5", "step6", "step7", "step8"]
  }
];

export const selectedLessonAtom = atom<LessonId>("L2");
export const selectedTabAtom = atom<ViewerTab>("textbook");
export const selectedStepAtom = atom<string>("step1");
