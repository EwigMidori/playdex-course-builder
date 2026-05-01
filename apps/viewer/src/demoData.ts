import type {
  QuestionBank,
  QuestionFamily,
  RelevanceReport,
  StoryManifest,
  StoryScreen,
  StoryStep
} from "./types";

const STRESS_STEP_COUNT = 30;
const STRESS_FAMILY_COUNT = 30;
const STRESS_SCREEN_COUNT = 30;

const pad = (value: number) => String(value).padStart(2, "0");

const stressStepId = (index: number) => `step${index + 1}`;

const stressFamilyId = (kind: "flashcard" | "quiz" | "longform", index: number, stepId: string) =>
  `${kind}_${stepId}_${pad(index + 1)}`;

const stressQuestionId = (kind: "flashcard" | "quiz" | "longform", index: number, stepId: string) =>
  `${stressFamilyId(kind, index, stepId)}_q1`;

const stressScore = (index: number, seed: number) => Number((0.35 + ((index + seed) % 10) * 0.055).toFixed(2));

export function buildStressManifest(lessonId: string): StoryManifest {
  return {
    lesson_id: lessonId,
    mode: "guided_story",
    steps: Array.from({ length: STRESS_STEP_COUNT }, (_, index) => ({
      concept: `Synthetic lecture segment ${index + 1}`,
      file: `demo/stress/${lessonId}/step${index + 1}/step.json`,
      sequence_id: stressStepId(index)
    }))
  };
}

export function buildStressStep(lessonId: string, stepId: string): StoryStep {
  const stepIndex = Number.parseInt(stepId.replace(/^step/, ""), 10) || 1;
  const screens: StoryScreen[] = Array.from({ length: STRESS_SCREEN_COUNT }, (_, index) => {
    const screenIndex = index + 1;
    const contentIndex = `${stepIndex}.${screenIndex}`;
    const isExercise = screenIndex % 6 === 0;

    return {
      id: `${stepId}-screen-${pad(screenIndex)}`,
      type: isExercise ? "exercise" : "narration",
      title: isExercise ? `Checkpoint ${contentIndex}` : `Content block ${contentIndex}`,
      lines: isExercise
        ? [`Which synthetic idea best matches checkpoint ${contentIndex}?`]
        : [
            `Synthetic lesson ${lessonId} ${stepId} content block ${screenIndex}.`,
            `Use this row to test long scrolling and dense card rendering.`
          ],
      focus_terms: isExercise ? [`focus_${stepIndex}_${screenIndex}`] : [`topic_${stepIndex}`],
      introduced_terms: [`term_${stepIndex}_${screenIndex}`],
      exercise: isExercise
        ? {
            kind: "single_choice",
            prompt: `Which synthetic idea best matches checkpoint ${contentIndex}?`,
            options: [
              `Option A for ${contentIndex}`,
              `Option B for ${contentIndex}`,
              `Option C for ${contentIndex}`,
              `Option D for ${contentIndex}`
            ],
            answer: 1,
            explanation: `This is synthetic stress data for ${stepId}.`
          }
        : undefined
    };
  });

  return {
    lesson_id: lessonId,
    sequence_id: stepId,
    mode: "guided_story",
    source: {
      plain_text: `Synthetic stress lesson for ${lessonId} / ${stepId}.`,
      related: [`demo/stress/${lessonId}`]
    },
    term_catalog: Object.fromEntries(
      Array.from({ length: 8 }, (_, index) => {
        const termId = `term_${stepIndex}_${index + 1}`;
        return [
          termId,
          {
            display: `Term ${stepIndex}-${index + 1}`,
            aliases: [`Alias ${stepIndex}-${index + 1}`],
            gloss: `Synthetic gloss for ${stepId} term ${index + 1}.`
          }
        ];
      })
    ),
    screens
  } as StoryStep;
}

function makeFamily(kind: "flashcard" | "quiz" | "longform", lessonId: string, stepId: string, index: number): QuestionFamily {
  const familyId = stressFamilyId(kind, index, stepId);
  return {
    family_id: familyId,
    concept_key: `${kind}_${stepId}_${pad(index + 1)}`,
    difficulty: ["easy", "medium", "hard"][index % 3],
    learning_goal: `${kind} stress family ${index + 1} for ${stepId} in ${lessonId}.`,
    linked_steps: [stepId],
    coverage_tags: [`coverage_${stepId}_${pad(index + 1)}`],
    question_type: kind,
    retrieval_focus: `Recall synthetic fact ${index + 1} for ${stepId}.`,
    variants: [
      kind === "flashcard"
        ? {
            question_id: stressQuestionId(kind, index, stepId),
            front: `What does synthetic flashcard ${index + 1} test in ${stepId}?`,
            back: `Synthetic answer ${index + 1} for ${stepId}.`,
            estimated_seconds: 8
          }
        : kind === "quiz"
        ? {
            question_id: stressQuestionId(kind, index, stepId),
            stem: `Which option best matches synthetic quiz ${index + 1} in ${stepId}?`,
            options: [
              `Distractor A ${index + 1}`,
              `Correct answer ${index + 1}`,
              `Distractor C ${index + 1}`,
              `Distractor D ${index + 1}`
            ],
            answer: 1,
            explanation: `Synthetic quiz item for stress mode.`
          }
        : {
            question_id: stressQuestionId(kind, index, stepId),
            stem: `Explain synthetic longform prompt ${index + 1} in ${stepId}.`,
            prompt_blocks: [
              `State the core idea for synthetic prompt ${index + 1}.`,
              `Give one supporting detail and one constraint.`
            ],
            reference_answer: [
              `Synthetic longform answer ${index + 1} for ${stepId}.`,
              `This content is intentionally repetitive for layout stress testing.`
            ],
            rubric_points: ["Core idea", "Supporting detail", "Constraint"]
          }
    ]
  };
}

function makeFamilies(kind: "flashcard" | "quiz" | "longform", lessonId: string, stepId: string) {
  return Array.from({ length: STRESS_FAMILY_COUNT }, (_, index) => makeFamily(kind, lessonId, stepId, index));
}

export function buildStressQuestionBank(lessonId: string, stepId: string): QuestionBank {
  return {
    lesson_id: lessonId,
    coverage_map: Array.from({ length: STRESS_FAMILY_COUNT }, (_, index) => ({
      coverage_id: `coverage_${stepId}_${pad(index + 1)}`,
      coverage_tag: `coverage_${stepId}_${pad(index + 1)}`,
      description: `Synthetic coverage item ${index + 1} for ${stepId}.`,
      covered_by: [stressFamilyId("flashcard", index, stepId)]
    })),
    flashcard_families: makeFamilies("flashcard", lessonId, stepId),
    quiz_families: makeFamilies("quiz", lessonId, stepId),
    longform_families: makeFamilies("longform", lessonId, stepId)
  };
}

export function buildStressRelevance(lessonId: string): RelevanceReport {
  const steps = Array.from({ length: STRESS_STEP_COUNT }, (_, index) => {
    const stepId = stressStepId(index);
    return {
      sequence_id: stepId,
      importance: stressScore(index, 1),
      course_relevance: stressScore(index, 2),
      exam_relevance: stressScore(index, 3),
      recommended_treatment: index % 3 === 0 ? "emphasize" : index % 3 === 1 ? "keep" : "condense",
      reason: `Synthetic step score ${index + 1} for ${lessonId}.`
    };
  });

  const questionFamilies = Array.from({ length: STRESS_STEP_COUNT }, (_, index) => {
    const stepId = stressStepId(index);
    return {
      family_id: stressFamilyId("flashcard", index, stepId),
      importance: stressScore(index, 4),
      course_relevance: stressScore(index, 5),
      exam_relevance: stressScore(index, 6),
      recommended_treatment: index % 2 === 0 ? "keep" : "condense",
      reason: `Synthetic family score ${index + 1} for ${lessonId}.`
    };
  });

  const textbookSections = Array.from({ length: STRESS_STEP_COUNT }, (_, index) => ({
    section_id: `section_${pad(index + 1)}`,
    title: `Synthetic section ${index + 1}`,
    importance: stressScore(index, 7),
    course_relevance: stressScore(index, 8),
    exam_relevance: stressScore(index, 9),
    recommended_treatment: index % 2 === 0 ? "emphasize" : "keep",
    reason: `Synthetic textbook section ${index + 1} for ${lessonId}.`
  }));

  const coverageScores = Array.from({ length: STRESS_STEP_COUNT }, (_, index) => ({
    coverage_id: `coverage_step_${pad(index + 1)}`,
    title: `Synthetic coverage ${index + 1}`,
    importance: stressScore(index, 10),
    course_relevance: stressScore(index, 11),
    exam_relevance: stressScore(index, 12),
    recommended_treatment: index % 2 === 0 ? "keep" : "condense",
    reason: `Synthetic coverage score ${index + 1} for ${lessonId}.`
  }));

  return {
    lesson_id: lessonId,
    step_scores: steps,
    question_family_scores: questionFamilies,
    textbook_section_scores: textbookSections,
    coverage_scores: coverageScores
  };
}

export function buildStressTextbook(lessonId: string): string {
  const sections = Array.from({ length: STRESS_STEP_COUNT }, (_, index) => {
    const stepId = stressStepId(index);
    const familyId = stressFamilyId("flashcard", index, stepId);
    const questionId = stressQuestionId("flashcard", index, stepId);
    return `## Synthetic Section ${index + 1}

This block is repeated on purpose for dense-content testing in ${lessonId}.

<QuestionFamily familyId="${familyId}" />

<QuestionRef id="${questionId}" />
`;
  }).join("\n");

  return `---
lessonId: ${lessonId}
---

# Stress Textbook

This synthetic textbook is only for testing long lists and dense layouts.

${sections}
`;
}
