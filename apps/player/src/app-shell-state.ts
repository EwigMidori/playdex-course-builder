import { atom } from "jotai";
import { atomWithStorage, createJSONStorage } from "jotai/utils";

export type ViewMode = "library" | "course" | "chapter" | "learning" | "review" | "bank" | "textbook";

export type AppShellState = {
  view: ViewMode;
  activeCourseId: string | null;
  activeChapterId: string | null;
  activeStepId: string | null;
};

const APP_SHELL_STORAGE_KEY = "playdex-player-app-shell-v1";

const defaultAppShellState = (): AppShellState => ({
  view: "library",
  activeCourseId: null,
  activeChapterId: null,
  activeStepId: null
});

function normalizeAppShellState(value: unknown): AppShellState {
  if (!value || typeof value !== "object") {
    return defaultAppShellState();
  }

  const parsed = value as Partial<AppShellState>;
  return {
    view: parsed.view ?? "library",
    activeCourseId: parsed.activeCourseId ?? null,
    activeChapterId: parsed.activeChapterId ?? null,
    activeStepId: parsed.activeStepId ?? null
  };
}

const jsonStorage = createJSONStorage<unknown>(() => window.localStorage);

const appShellStorage = {
  getItem: (key: string, initialValue: AppShellState) =>
    normalizeAppShellState(jsonStorage.getItem(key, initialValue)),
  setItem: (key: string, value: AppShellState) => jsonStorage.setItem(key, value),
  removeItem: (key: string) => jsonStorage.removeItem(key)
};

export const appShellStateAtom = atomWithStorage<AppShellState>(
  APP_SHELL_STORAGE_KEY,
  defaultAppShellState(),
  appShellStorage
);

export const patchAppShellStateAtom = atom(
  null,
  (get, set, patch: Partial<AppShellState>) => {
    set(appShellStateAtom, {
      ...get(appShellStateAtom),
      ...patch
    });
  }
);
