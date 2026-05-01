import { atom } from "jotai";
import type { AssetView, LessonId, LessonOption } from "./types";

const lectureNotes: Record<string, string> = {
  L0: "course introduction and market overview",
  L1: "algorithmic trading basics",
  L2: "data scraping and database management",
  L3: "backtesting framework and rule-based strategy",
  L4: "time series analysis for market classification",
  L5: "statistical arbitrage and pairs trading",
  L6: "capital and risk management",
  L7: "performance measures and portfolio optimization",
  L8: "order book and high frequency data modeling",
  L9: "broker connectivity and live deployment",
  L10: "machine learning in algorithmic trading"
};

export const lessons: LessonOption[] = Array.from({ length: 11 }, (_, index) => {
  const id = `L${index}`;
  return {
    id,
    label: `${id} - ${lectureNotes[id] ?? "lecture"}`,
    courseLabel: "COMP7415A",
    lectureLabel: `Lecture ${index}`,
    defaultStep: "step1"
  };
});

export const selectedLessonAtom = atom<LessonId>("L2");
export const selectedAssetViewAtom = atom<AssetView>("overview");
export const selectedStepAtom = atom<string>("step1");
