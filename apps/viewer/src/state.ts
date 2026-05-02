import { atom } from "jotai";
import type { AssetView } from "./types";

export const selectedLessonAtom = atom<string>("");
export const selectedAssetViewAtom = atom<AssetView>("overview");
export const selectedStepAtom = atom<string>("step1");
