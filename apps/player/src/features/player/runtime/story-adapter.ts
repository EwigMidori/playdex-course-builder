import type { ChapterRecord, CourseRecord, Exercise, StoryManifestStep, StoryScreen, StoryStep } from "../player-model";
import type { NormalizedExercise, SceneDescriptor, SceneModel, StoryNode } from "./story-runtime-types";

const TERM_TAG_PATTERN = /<term id="([^"]+)">[\s\S]*?<\/term>/g;

function normalizeNodeLines(lines: string[] | undefined, body: string | undefined, title: string | undefined) {
  const fallback = [body, title].filter((value): value is string => typeof value === "string" && value.trim().length > 0);
  const source = (lines?.length ? lines : fallback).filter((value): value is string => typeof value === "string");
  return source.length ? source : ["No scene content found."];
}

function collectTermRefs(lines: string[], explicitRefs: string[] = []) {
  const refs = new Set(explicitRefs.filter(Boolean));

  for (const line of lines) {
    let match = TERM_TAG_PATTERN.exec(line);
    while (match) {
      if (match[1]) {
        refs.add(match[1]);
      }
      match = TERM_TAG_PATTERN.exec(line);
    }
    TERM_TAG_PATTERN.lastIndex = 0;
  }

  return [...refs];
}

function deriveAnswerOrder(rawExercise: Exercise | undefined) {
  const answerOrder = rawExercise?.answer_order;
  if (Array.isArray(answerOrder) && answerOrder.every((value) => Number.isInteger(value))) {
    return answerOrder;
  }

  const items = rawExercise?.items ?? [];
  const answers = rawExercise?.answers ?? [];
  if (!items.length || !answers.length) {
    return [];
  }

  return answers
    .map((answer) => {
      const trimmed = answer.trim();
      const index = items.findIndex((item) => item === trimmed || item.startsWith(`${trimmed}.`) || item.startsWith(`${trimmed} `));
      return index >= 0 ? index : null;
    })
    .filter((value): value is number => value !== null);
}

function normalizeExercise(rawExercise: Exercise | undefined, lines: string[]): NormalizedExercise {
  const rawAnswer = rawExercise?.answer;
  return {
    kind: rawExercise?.kind ?? null,
    prompt: rawExercise?.prompt ?? lines[0] ?? null,
    options: rawExercise?.options ?? [],
    items: rawExercise?.items ?? [],
    acceptedAnswers: rawExercise?.answers ?? [],
    answerIndex: typeof rawAnswer === "number" ? rawAnswer : null,
    answerIndices: Array.isArray(rawAnswer) ? rawAnswer.filter((value): value is number => Number.isInteger(value)) : [],
    answerOrder: deriveAnswerOrder(rawExercise),
    explanation: rawExercise?.explanation ?? null,
    raw: rawExercise ?? {}
  };
}

function normalizeNode(sceneId: string, rawScreenIndex: number, screen: StoryScreen): StoryNode {
  const lines = normalizeNodeLines(screen?.lines, screen?.body, screen?.title);
  const termRefs = collectTermRefs(lines, [...(screen?.introduced_terms ?? []), ...(screen?.focus_terms ?? [])]);
  const id = screen?.id ?? `${sceneId}:node:${rawScreenIndex + 1}`;
  const isExercise = screen?.type === "exercise" || Boolean(screen?.exercise);

  if (isExercise) {
    return {
      id,
      kind: "exercise",
      lines,
      termRefs,
      rawScreenIndex,
      rawScreenType: screen?.type ?? null,
      exercise: normalizeExercise(screen?.exercise, lines)
    };
  }

  return {
    id,
    kind: "narration",
    lines,
    termRefs,
    rawScreenIndex,
    rawScreenType: screen?.type ?? null
  };
}

export function buildSceneDescriptor(
  course: CourseRecord,
  chapter: ChapterRecord,
  stepMeta: StoryManifestStep | null,
  step: StoryStep
): SceneDescriptor {
  const stepId = stepMeta?.sequence_id ?? step.sequence_id ?? "unknown-step";
  return {
    sceneId: `${course.courseId}:${chapter.chapterId}:${stepId}`,
    courseId: course.courseId,
    chapterId: chapter.chapterId,
    lessonId: step.lesson_id ?? chapter.lessonId,
    stepId,
    concept: stepMeta?.concept ?? step.sequence_id ?? stepId
  };
}

export function normalizeStoryStep(step: StoryStep, descriptor: SceneDescriptor): SceneModel {
  const rawScreens = step.screens ?? [];
  const nodes = rawScreens.map((screen, index) => normalizeNode(descriptor.sceneId, index, screen));

  return {
    ...descriptor,
    termCatalog: step.term_catalog ?? {},
    nodes: nodes.length
      ? nodes
      : [
          {
            id: `${descriptor.sceneId}:node:1`,
            kind: "narration",
            lines: ["No scene content found."],
            termRefs: [],
            rawScreenIndex: 0,
            rawScreenType: null
          }
        ]
  };
}
