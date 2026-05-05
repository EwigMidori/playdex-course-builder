import React from "react";
import { evaluate } from "@mdx-js/mdx";
import * as runtime from "react/jsx-runtime";
import remarkGfm from "remark-gfm";
import { CourseAssetClient } from "../../platform/course-assets";
import type {
  ChapterRecord,
  CourseIndex,
  CourseRecord,
  QuestionBank,
  QuestionFamily,
  QuestionIndex,
  QuestionVariant,
  StoryManifest,
  StoryStep
} from "./player-model";
import { EMPTY_STEPS, chapterKey, isCourseReady } from "./player-utils";

export type AsyncState<T> = {
  loading: boolean;
  data: T | null;
  error: string | null;
};

export type CatalogState = AsyncState<{
  index: CourseIndex;
  manifests: Record<string, StoryManifest>;
}>;

export type PracticeState = AsyncState<QuestionIndex>;

export type TextbookState = AsyncState<string>;

const emptyState = <T,>(): AsyncState<T> => ({ loading: true, data: null, error: null });

function formatLoadFailure(path: string, error: string | null, fallback: string) {
  return `${path}: ${error ?? fallback}`;
}

export function useCatalogState(): CatalogState {
  const [state, setState] = React.useState<CatalogState>(emptyState);

  React.useEffect(() => {
    let alive = true;
    setState(emptyState());

    CourseAssetClient.loadCourseIndex().then(async (result) => {
      if (!result.data) {
        if (alive) {
          setState({ loading: false, data: null, error: result.error ?? "Failed to load course index." });
        }
        return;
      }

      const manifestResults = await Promise.all(
        result.data.courses.filter(isCourseReady).flatMap((course) =>
          course.chapters.map(async (chapter) => {
            const manifest = await CourseAssetClient.loadManifest(chapter);
            return {
              key: chapterKey(course.courseId, chapter.chapterId),
              manifest
            };
          })
        )
      );

      const failedManifest = manifestResults.find(({ manifest }) => !manifest.data);
      if (failedManifest) {
        if (alive) {
          setState({
            loading: false,
            data: null,
            error: formatLoadFailure(failedManifest.manifest.path, failedManifest.manifest.error, "Manifest data is missing.")
          });
        }
        return;
      }

      if (alive) {
        setState({
          loading: false,
          data: {
            index: result.data,
            manifests: Object.fromEntries(
              manifestResults.map(({ key, manifest }) => [key, manifest.data as StoryManifest])
            )
          },
          error: null
        });
      }
    });

    return () => {
      alive = false;
    };
  }, []);

  return state;
}

export function usePracticeIndex(course: CourseRecord | null, manifests: Record<string, StoryManifest>): PracticeState {
  const [state, setState] = React.useState<PracticeState>({ loading: false, data: null, error: null });

  React.useEffect(() => {
    if (!course) {
      setState({ loading: false, data: null, error: null });
      return;
    }

    let alive = true;
    setState(emptyState());

    Promise.all(
      course.chapters.flatMap((chapter) => {
        const manifest = manifests[chapterKey(course.courseId, chapter.chapterId)];
        const steps = manifest.steps ?? [];
        return steps.map(async (step) => {
          const stepId = step.sequence_id ?? "step1";
          const bank = await CourseAssetClient.loadQuestionBank(chapter, stepId);
          return { bank, chapter, stepId };
        });
      })
    ).then((results) => {
      if (!alive) {
        return;
      }

      const failedBank = results.find((entry) => !entry.bank.data);
      if (failedBank) {
        setState({
          loading: false,
          data: null,
          error: formatLoadFailure(failedBank.bank.path, failedBank.bank.error, "Question bank data is missing.")
        });
        return;
      }

      const families = new Map<string, QuestionFamily>();
      const questions = new Map<string, { family: QuestionFamily; variant: QuestionVariant }>();
      const flashcards: QuestionIndex["flashcards"] = [];
      const quizFamilies: QuestionFamily[] = [];
      const longformFamilies: QuestionFamily[] = [];
      const coverage = [];

      for (const entry of results) {
        const bank = entry.bank.data as QuestionBank;

        for (const item of bank.coverage_map ?? []) {
          coverage.push(item);
        }

        for (const family of bank.flashcard_families ?? []) {
          if (family.family_id) {
            families.set(family.family_id, family);
          }
          for (const variant of family.variants ?? []) {
            if (variant.question_id) {
              questions.set(variant.question_id, { family, variant });
            }
            flashcards.push({
              family,
              variant,
              chapterId: entry.chapter.chapterId,
              chapterTitle: entry.chapter.title,
              stepId: entry.stepId
            });
          }
        }

        for (const family of bank.quiz_families ?? []) {
          if (family.family_id) {
            families.set(family.family_id, family);
          }
          quizFamilies.push(family);
          for (const variant of family.variants ?? []) {
            if (variant.question_id) {
              questions.set(variant.question_id, { family, variant });
            }
          }
        }

        for (const family of bank.longform_families ?? []) {
          if (family.family_id) {
            families.set(family.family_id, family);
          }
          longformFamilies.push(family);
          for (const variant of family.variants ?? []) {
            if (variant.question_id) {
              questions.set(variant.question_id, { family, variant });
            }
          }
        }
      }

      setState({
        loading: false,
        data: { families, questions, flashcards, quizFamilies, longformFamilies, coverage },
        error: null
      });
    });

    return () => {
      alive = false;
    };
  }, [course, manifests]);

  return state;
}

export function useStoryBundle(course: CourseRecord | null, chapter: ChapterRecord | null, stepId: string | null) {
  const [state, setState] = React.useState<AsyncState<{ step: StoryStep; bank: QuestionBank }>>({
    loading: false,
    data: null,
    error: null
  });

  React.useEffect(() => {
    if (!course || !chapter || !stepId) {
      setState({ loading: false, data: null, error: null });
      return;
    }

    let alive = true;
    setState(emptyState());

    Promise.all([CourseAssetClient.loadStep(chapter, stepId), CourseAssetClient.loadQuestionBank(chapter, stepId)]).then(
      ([step, bank]) => {
        if (!alive) {
          return;
        }
      if (!step.data) {
        setState({
          loading: false,
          data: null,
          error: formatLoadFailure(step.path, step.error, "Story step data is missing.")
        });
        return;
      }
      if (!bank.data) {
        setState({
          loading: false,
          data: null,
          error: formatLoadFailure(bank.path, bank.error, "Question bank data is missing.")
        });
        return;
      }
        setState({
          loading: false,
          data: { step: step.data, bank: bank.data },
          error: null
        });
      }
    );

    return () => {
      alive = false;
    };
  }, [course, chapter, stepId]);

  return state;
}

export function useTextbookState(chapter: ChapterRecord | null): TextbookState {
  const [state, setState] = React.useState<TextbookState>({ loading: false, data: null, error: null });

  React.useEffect(() => {
    if (!chapter) {
      setState({ loading: false, data: null, error: null });
      return;
    }
    let alive = true;
    setState(emptyState());
    CourseAssetClient.loadTextbook(chapter).then((result) => {
      if (alive) {
        setState({
          loading: false,
          data: result.data,
          error: result.error
        });
      }
    });
    return () => {
      alive = false;
    };
  }, [chapter]);

  return state;
}

function normalizeMdxSource(source: string) {
  return source.replace(/^---[\s\S]*?---\s*/, "").replace(/^(#{1,6}\s+.+?)\s+\{#[^}]+}\s*$/gm, "$1");
}

export function useCompiledMdx(source: string | null) {
  const [state, setState] = React.useState<AsyncState<React.ComponentType<{ components?: Record<string, React.ComponentType<any>> }>>>({
    loading: false,
    data: null,
    error: null
  });

  React.useEffect(() => {
    if (!source) {
      setState({ loading: false, data: null, error: null });
      return;
    }
    let alive = true;
    setState(emptyState());
    evaluate(normalizeMdxSource(source), {
      ...runtime,
      remarkPlugins: [remarkGfm]
    })
      .then((compiled) => {
        if (alive) {
          setState({
            loading: false,
            data: (compiled as { default: React.ComponentType<any> }).default,
            error: null
          });
        }
      })
      .catch((error) => {
        if (alive) {
          setState({
            loading: false,
            data: null,
            error: error instanceof Error ? error.message : String(error)
          });
        }
      });
    return () => {
      alive = false;
    };
  }, [source]);

  return state;
}
