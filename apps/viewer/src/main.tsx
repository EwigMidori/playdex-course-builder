import React from "react";
import ReactDOM from "react-dom/client";
import ReactMarkdown from "react-markdown";
import { useAtom } from "jotai";
import remarkGfm from "remark-gfm";
import { mdxToMarkdownPreview } from "./mdxPreview";
import {
  fetchText,
  loadManifest,
  loadQuestionBank,
  loadRelevance,
  loadStep,
  textbookPath
} from "./data";
import { lessons, selectedLessonAtom, selectedStepAtom, selectedTabAtom } from "./state";
import type {
  LessonId,
  QuestionBank,
  QuestionFamily,
  RelevanceReport,
  RelevanceScore,
  StoryManifest,
  StoryScreen,
  StoryStep,
  ViewerTab
} from "./types";
import "./styles.css";

type AsyncState<T> = {
  loading: boolean;
  data: T | null;
  error: string | null;
  path?: string;
};

const emptyState = <T,>(): AsyncState<T> => ({ loading: true, data: null, error: null });

function useTextbook(lessonId: LessonId) {
  const [state, setState] = React.useState<AsyncState<string>>(emptyState);
  React.useEffect(() => {
    let alive = true;
    setState(emptyState());
    fetchText(textbookPath(lessonId)).then((result) => {
      if (alive) {
        setState({ loading: false, data: result.data, error: result.error, path: result.path });
      }
    });
    return () => {
      alive = false;
    };
  }, [lessonId]);
  return state;
}

function useStoryData(lessonId: LessonId, stepId: string) {
  const lesson = lessons.find((item) => item.id === lessonId);
  const [manifest, setManifest] = React.useState<AsyncState<StoryManifest>>(emptyState);
  const [step, setStep] = React.useState<AsyncState<StoryStep>>(emptyState);
  const [bank, setBank] = React.useState<AsyncState<QuestionBank>>(emptyState);

  React.useEffect(() => {
    let alive = true;
    setManifest(emptyState());
    loadManifest(lessonId, lesson?.legacySteps).then((result) => {
      if (alive) {
        setManifest({ loading: false, data: result.data, error: result.error, path: result.path });
      }
    });
    return () => {
      alive = false;
    };
  }, [lesson?.legacySteps, lessonId]);

  React.useEffect(() => {
    let alive = true;
    setStep(emptyState());
    setBank(emptyState());
    Promise.all([loadStep(lessonId, stepId), loadQuestionBank(lessonId, stepId)]).then(([stepResult, bankResult]) => {
      if (alive) {
        setStep({ loading: false, data: stepResult.data, error: stepResult.error, path: stepResult.path });
        setBank({ loading: false, data: bankResult.data, error: bankResult.error, path: bankResult.path });
      }
    });
    return () => {
      alive = false;
    };
  }, [lessonId, stepId]);

  return { manifest, step, bank };
}

function useRelevance(lessonId: LessonId) {
  const [state, setState] = React.useState<AsyncState<RelevanceReport>>(emptyState);
  React.useEffect(() => {
    let alive = true;
    setState(emptyState());
    loadRelevance(lessonId).then((result) => {
      if (alive) {
        setState({ loading: false, data: result.data, error: result.error, path: result.path });
      }
    });
    return () => {
      alive = false;
    };
  }, [lessonId]);
  return state;
}

function App() {
  const [lessonId, setLessonId] = useAtom(selectedLessonAtom);
  const [tab, setTab] = useAtom(selectedTabAtom);
  const [selectedStep, setSelectedStep] = useAtom(selectedStepAtom);
  const relevance = useRelevance(lessonId);
  const currentLesson = lessons.find((lesson) => lesson.id === lessonId) ?? lessons[0];

  const changeLesson = (nextLesson: LessonId) => {
    const lesson = lessons.find((item) => item.id === nextLesson) ?? lessons[0];
    setLessonId(nextLesson);
    setSelectedStep(lesson.defaultStep);
  };

  return (
    <div className="min-h-screen bg-stone-100 text-stone-950">
      <header className="border-b border-stone-300 bg-white">
        <div className="mx-auto flex max-w-[1500px] flex-wrap items-center gap-3 px-4 py-3">
          <div className="mr-2">
            <h1 className="text-base font-semibold">Pipeline Viewer</h1>
            <p className="text-xs text-stone-500">Existing research/pipeline artifacts</p>
          </div>
          <Segmented
            value={lessonId}
            options={lessons.map((lesson) => ({ value: lesson.id, label: lesson.label }))}
            onChange={(value) => changeLesson(value as LessonId)}
          />
          <Segmented
            value={tab}
            options={[
              { value: "textbook", label: "MDX textbook" },
              { value: "story", label: "Guided story" },
              { value: "relevance", label: "Relevance" }
            ]}
            onChange={(value) => setTab(value as ViewerTab)}
          />
          <div className="ml-auto flex flex-wrap gap-2 text-xs">
            <Badge label={`lesson ${currentLesson.id}`} />
            <RelevanceBadge report={relevance.data} sequenceId={selectedStep} />
          </div>
        </div>
      </header>

      <main className="mx-auto max-w-[1500px] px-4 py-4">
        {tab === "textbook" ? <TextbookView lessonId={lessonId} relevance={relevance.data} /> : null}
        {tab === "story" ? (
          <StoryView
            lessonId={lessonId}
            selectedStep={selectedStep}
            setSelectedStep={setSelectedStep}
            relevance={relevance.data}
          />
        ) : null}
        {tab === "relevance" ? <RelevanceView relevance={relevance} /> : null}
      </main>
    </div>
  );
}

function Segmented({
  value,
  options,
  onChange
}: {
  value: string;
  options: Array<{ value: string; label: string }>;
  onChange: (value: string) => void;
}) {
  return (
    <div className="flex rounded-md border border-stone-300 bg-stone-100 p-0.5">
      {options.map((option) => (
        <button
          className={[
            "h-8 whitespace-nowrap rounded px-3 text-xs font-medium",
            value === option.value ? "bg-emerald-700 text-white shadow-sm" : "text-stone-700 hover:bg-white"
          ].join(" ")}
          key={option.value}
          onClick={() => onChange(option.value)}
          type="button"
        >
          {option.label}
        </button>
      ))}
    </div>
  );
}

function TextbookView({ lessonId, relevance }: { lessonId: LessonId; relevance: RelevanceReport | null }) {
  const textbook = useTextbook(lessonId);
  const preview = React.useMemo(() => (textbook.data ? mdxToMarkdownPreview(textbook.data) : ""), [textbook.data]);

  return (
    <section className="grid gap-4 lg:grid-cols-[minmax(0,1fr)_320px]">
      <Panel title="MDX preview" subtitle={textbook.path}>
        {textbook.loading ? <Muted>Loading textbook...</Muted> : null}
        {textbook.error ? <Notice title="Textbook unavailable" detail={textbook.error} /> : null}
        {textbook.data ? (
          <article className="prose prose-stone max-w-none prose-headings:scroll-mt-4 prose-pre:bg-stone-950 prose-pre:text-stone-50">
            <ReactMarkdown remarkPlugins={[remarkGfm]}>{preview}</ReactMarkdown>
          </article>
        ) : null}
      </Panel>
      <Panel title="Textbook relevance" subtitle="section badges">
        <ScoreList
          empty="No textbook relevance report for this lesson."
          idKey="section_id"
          scores={relevance?.textbook_section_scores}
        />
      </Panel>
    </section>
  );
}

function StoryView({
  lessonId,
  selectedStep,
  setSelectedStep,
  relevance
}: {
  lessonId: LessonId;
  selectedStep: string;
  setSelectedStep: (step: string) => void;
  relevance: RelevanceReport | null;
}) {
  const { manifest, step, bank } = useStoryData(lessonId, selectedStep);
  const steps = manifest.data?.steps ?? [];

  React.useEffect(() => {
    if (steps.length > 0 && !steps.some((item) => item.sequence_id === selectedStep)) {
      setSelectedStep(steps[0]?.sequence_id ?? "step1");
    }
  }, [selectedStep, setSelectedStep, steps]);

  return (
    <section className="grid gap-4 xl:grid-cols-[270px_minmax(0,1fr)_420px]">
      <Panel title="Steps" subtitle={manifest.path}>
        {manifest.loading ? <Muted>Loading manifest...</Muted> : null}
        {manifest.error ? <Notice title="Manifest fallback" detail={manifest.error} /> : null}
        <div className="space-y-2">
          {steps.map((stepItem) => {
            const stepId = stepItem.sequence_id ?? "step1";
            return (
              <button
                className={[
                  "w-full rounded-md border px-3 py-2 text-left text-sm",
                  selectedStep === stepId
                    ? "border-emerald-700 bg-emerald-50"
                    : "border-stone-300 bg-white hover:border-stone-500"
                ].join(" ")}
                key={stepId}
                onClick={() => setSelectedStep(stepId)}
                type="button"
              >
                <div className="flex items-center justify-between gap-2">
                  <span className="font-semibold">{stepId}</span>
                  <RelevanceBadge report={relevance} sequenceId={stepId} compact />
                </div>
                <div className="mt-1 text-xs text-stone-600">{stepItem.concept ?? stepItem.file ?? "Untitled step"}</div>
              </button>
            );
          })}
          {!manifest.loading && steps.length === 0 ? <Muted>No guided story steps found.</Muted> : null}
        </div>
      </Panel>

      <Panel title={`Screens - ${selectedStep}`} subtitle={step.path}>
        {step.loading ? <Muted>Loading step...</Muted> : null}
        {step.error ? <Notice title="Step unavailable" detail={step.error} /> : null}
        {step.data?.screens?.length ? (
          <div className="space-y-3">
            {step.data.screens.map((screen, index) => (
              <ScreenCard key={`${screen.id ?? "screen"}-${index}`} screen={screen} index={index} />
            ))}
          </div>
        ) : null}
      </Panel>

      <Panel title="Question bank" subtitle={bank.path}>
        {bank.loading ? <Muted>Loading question bank...</Muted> : null}
        {bank.error ? <Notice title="Question bank unavailable" detail={bank.error} /> : null}
        {bank.data ? <QuestionBankView bank={bank.data} relevance={relevance} selectedStep={selectedStep} /> : null}
      </Panel>
    </section>
  );
}

function ScreenCard({ screen, index }: { screen: StoryScreen; index: number }) {
  const lines = screen.lines ?? (screen.body ? [screen.body] : []);
  return (
    <article className="rounded-md border border-stone-300 bg-white p-3">
      <div className="mb-2 flex flex-wrap items-center gap-2">
        <Badge label={`${index + 1}. ${screen.id ?? "screen"}`} />
        <Badge label={screen.type ?? "unknown"} tone={screen.type === "exercise" ? "amber" : "stone"} />
        {screen.focus_terms?.map((term) => <Badge key={`focus-${term}`} label={term} tone="blue" />)}
      </div>
      {screen.title ? <h3 className="mb-2 font-semibold">{screen.title}</h3> : null}
      <div className="space-y-1 text-sm leading-6">
        {lines.map((line, lineIndex) => (
          <InlineMarkup key={`${screen.id}-line-${lineIndex}`} value={line} />
        ))}
      </div>
      {screen.introduced_terms?.length ? (
        <div className="mt-3 flex flex-wrap gap-1">
          {screen.introduced_terms.map((term) => (
            <Badge key={`intro-${term}`} label={`introduces ${term}`} tone="emerald" />
          ))}
        </div>
      ) : null}
      {screen.exercise ? <ExercisePreview exercise={screen.exercise} /> : null}
      {screen.terms || screen.exercises ? (
        <pre className="mt-3 max-h-56 overflow-auto rounded bg-stone-950 p-3 text-xs text-stone-100">
          {JSON.stringify({ terms: screen.terms, exercises: screen.exercises }, null, 2)}
        </pre>
      ) : null}
    </article>
  );
}

function InlineMarkup({ value }: { value: string }) {
  const normalized = value
    .replace(/<term\b[^>]*id=["']([^"']+)["'][^>]*>(.*?)<\/term>/g, "**$2** `term:$1`")
    .replace(/<[^>]+>/g, "");
  return (
    <div className="prose prose-sm prose-stone max-w-none">
      <ReactMarkdown remarkPlugins={[remarkGfm]}>{normalized}</ReactMarkdown>
    </div>
  );
}

function ExercisePreview({ exercise }: { exercise: NonNullable<StoryScreen["exercise"]> }) {
  return (
    <div className="mt-3 rounded-md border border-amber-300 bg-amber-50 p-3 text-sm">
      <div className="mb-2 flex items-center gap-2">
        <Badge label={exercise.kind ?? "exercise"} tone="amber" />
        <span className="font-semibold">{exercise.prompt}</span>
      </div>
      {exercise.options?.length ? (
        <ol className="space-y-1">
          {exercise.options.map((option, index) => (
            <li className={index === exercise.answer ? "font-semibold text-emerald-800" : ""} key={option}>
              {index}. {option}
            </li>
          ))}
        </ol>
      ) : null}
      {exercise.explanation ? <p className="mt-2 text-xs text-stone-700">{exercise.explanation}</p> : null}
    </div>
  );
}

function QuestionBankView({
  bank,
  relevance,
  selectedStep
}: {
  bank: QuestionBank;
  relevance: RelevanceReport | null;
  selectedStep: string;
}) {
  return (
    <div className="space-y-4">
      <CoverageMap bank={bank} />
      <FamilyGroup
        title="Flashcards"
        families={bank.flashcard_families}
        relevance={relevance}
        selectedStep={selectedStep}
      />
      <FamilyGroup
        title="Longform"
        families={bank.longform_families}
        relevance={relevance}
        selectedStep={selectedStep}
      />
    </div>
  );
}

function CoverageMap({ bank }: { bank: QuestionBank }) {
  if (!bank.coverage_map?.length) {
    return null;
  }
  return (
    <div>
      <h3 className="mb-2 text-xs font-semibold uppercase tracking-wide text-stone-500">Coverage</h3>
      <div className="flex flex-wrap gap-1">
        {bank.coverage_map.map((coverage) => (
          <Badge
            key={coverage.coverage_tag ?? coverage.coverage_id}
            label={coverage.coverage_tag ?? coverage.coverage_id ?? "coverage"}
            tone="blue"
          />
        ))}
      </div>
    </div>
  );
}

function FamilyGroup({
  title,
  families = [],
  relevance,
  selectedStep
}: {
  title: string;
  families?: QuestionFamily[];
  relevance: RelevanceReport | null;
  selectedStep: string;
}) {
  const filtered = families.filter((family) => {
    if (!family.linked_steps?.length) {
      return true;
    }
    return family.linked_steps.includes(selectedStep);
  });

  return (
    <div>
      <h3 className="mb-2 text-xs font-semibold uppercase tracking-wide text-stone-500">
        {title} <span className="font-normal">({filtered.length})</span>
      </h3>
      <div className="space-y-3">
        {filtered.map((family) => (
          <QuestionFamilyCard family={family} key={family.family_id} relevance={relevance} />
        ))}
        {filtered.length === 0 ? <Muted>No families linked to this step.</Muted> : null}
      </div>
    </div>
  );
}

function QuestionFamilyCard({ family, relevance }: { family: QuestionFamily; relevance: RelevanceReport | null }) {
  const score = relevance?.question_family_scores?.find((item) => item.family_id === family.family_id);
  return (
    <article className="rounded-md border border-stone-300 bg-white p-3">
      <div className="mb-2 flex flex-wrap items-center gap-2">
        <span className="font-mono text-xs font-semibold">{family.family_id}</span>
        {family.difficulty ? <Badge label={family.difficulty} /> : null}
        {family.question_type ? <Badge label={family.question_type} tone="blue" /> : null}
        {score ? <TreatmentBadge score={score} /> : null}
      </div>
      {family.learning_goal ? <p className="mb-2 text-sm text-stone-700">{family.learning_goal}</p> : null}
      {family.coverage_tags?.length ? (
        <div className="mb-2 flex flex-wrap gap-1">
          {family.coverage_tags.map((tag) => (
            <Badge key={tag} label={tag} tone="stone" />
          ))}
        </div>
      ) : null}
      <div className="space-y-2">
        {family.variants?.map((variant) => (
          <details className="rounded border border-stone-200 bg-stone-50 p-2" key={variant.question_id}>
            <summary className="cursor-pointer text-sm font-medium">{variant.question_id ?? variant.stem}</summary>
            <div className="mt-2 text-sm">
              {variant.stem ? <p className="font-medium">{variant.stem}</p> : null}
              {variant.prompt_blocks?.length ? (
                <ul className="mt-1 list-disc pl-5">
                  {variant.prompt_blocks.map((block) => (
                    <li key={block}>{block}</li>
                  ))}
                </ul>
              ) : null}
              {variant.options?.length ? (
                <ol className="mt-2 space-y-1">
                  {variant.options.map((option, index) => (
                    <li className={index === variant.answer ? "font-semibold text-emerald-800" : ""} key={option}>
                      {index}. {option}
                    </li>
                  ))}
                </ol>
              ) : null}
              {variant.reference_answer?.length ? (
                <div className="mt-2 text-xs text-stone-700">{variant.reference_answer.join(" ")}</div>
              ) : null}
              {variant.explanation ? <p className="mt-2 text-xs text-stone-700">{variant.explanation}</p> : null}
            </div>
          </details>
        ))}
      </div>
    </article>
  );
}

function RelevanceView({ relevance }: { relevance: AsyncState<RelevanceReport> }) {
  return (
    <section className="grid gap-4 lg:grid-cols-2">
      <Panel title="Relevance report" subtitle={relevance.path}>
        {relevance.loading ? <Muted>Loading relevance report...</Muted> : null}
        {relevance.error ? <Notice title="No relevance report" detail={relevance.error} /> : null}
        {relevance.data ? (
          <div className="grid gap-4">
            <ScoreSection title="Steps" idKey="sequence_id" scores={relevance.data.step_scores} />
            <ScoreSection title="Question families" idKey="family_id" scores={relevance.data.question_family_scores} />
          </div>
        ) : null}
      </Panel>
      <Panel title="Coverage and textbook" subtitle="coverage_scores / textbook_section_scores">
        {relevance.data ? (
          <div className="grid gap-4">
            <ScoreSection title="Coverage" idKey="coverage_id" scores={relevance.data.coverage_scores} />
            <ScoreSection title="Textbook sections" idKey="section_id" scores={relevance.data.textbook_section_scores} />
          </div>
        ) : (
          <Muted>Open L2 to inspect available relevance data.</Muted>
        )}
      </Panel>
    </section>
  );
}

function ScoreSection({ title, scores, idKey }: { title: string; scores?: RelevanceScore[]; idKey: keyof RelevanceScore }) {
  return (
    <div>
      <h3 className="mb-2 text-xs font-semibold uppercase tracking-wide text-stone-500">{title}</h3>
      <ScoreList scores={scores} idKey={idKey} empty={`No ${title.toLowerCase()} scores.`} />
    </div>
  );
}

function ScoreList({
  scores,
  idKey,
  empty
}: {
  scores?: RelevanceScore[];
  idKey: keyof RelevanceScore;
  empty: string;
}) {
  if (!scores?.length) {
    return <Muted>{empty}</Muted>;
  }
  return (
    <div className="space-y-2">
      {scores.map((score, index) => (
        <article className="rounded-md border border-stone-300 bg-white p-3 text-sm" key={`${String(score[idKey])}-${index}`}>
          <div className="mb-1 flex flex-wrap items-center gap-2">
            <span className="font-mono text-xs font-semibold">{String(score[idKey] ?? score.title ?? "score")}</span>
            <TreatmentBadge score={score} />
            <Metric label="importance" value={score.importance} />
            <Metric label="course" value={score.course_relevance} />
            <Metric label="exam" value={score.exam_relevance} />
          </div>
          {score.title ? <p className="font-medium">{score.title}</p> : null}
          {score.reason ? <p className="mt-1 text-xs leading-5 text-stone-600">{score.reason}</p> : null}
        </article>
      ))}
    </div>
  );
}

function RelevanceBadge({
  report,
  sequenceId,
  compact = false
}: {
  report: RelevanceReport | null;
  sequenceId: string;
  compact?: boolean;
}) {
  const score = report?.step_scores?.find((item) => item.sequence_id === sequenceId);
  if (!score) {
    return <Badge label={compact ? "no rel" : "no relevance"} tone="stone" />;
  }
  return <TreatmentBadge score={score} compact={compact} />;
}

function TreatmentBadge({ score, compact = false }: { score: RelevanceScore; compact?: boolean }) {
  const treatment = score.recommended_treatment ?? "unknown";
  const tone = treatment === "emphasize" ? "emerald" : treatment === "condense" ? "amber" : "blue";
  const metric = typeof score.importance === "number" ? ` ${score.importance.toFixed(2)}` : "";
  return <Badge label={compact ? treatment : `${treatment}${metric}`} tone={tone} />;
}

function Metric({ label, value }: { label: string; value?: number }) {
  if (typeof value !== "number") {
    return null;
  }
  return <span className="rounded bg-stone-100 px-1.5 py-0.5 text-[11px] text-stone-600">{`${label}:${value.toFixed(2)}`}</span>;
}

function Badge({ label, tone = "stone" }: { label: string; tone?: "stone" | "emerald" | "blue" | "amber" }) {
  const classes = {
    stone: "border-stone-300 bg-stone-100 text-stone-700",
    emerald: "border-emerald-300 bg-emerald-50 text-emerald-800",
    blue: "border-sky-300 bg-sky-50 text-sky-800",
    amber: "border-amber-300 bg-amber-50 text-amber-800"
  };
  return <span className={`rounded border px-1.5 py-0.5 text-[11px] font-medium ${classes[tone]}`}>{label}</span>;
}

function Panel({ title, subtitle, children }: { title: string; subtitle?: string; children: React.ReactNode }) {
  return (
    <section className="min-w-0 rounded-md border border-stone-300 bg-stone-50">
      <div className="border-b border-stone-300 bg-white px-3 py-2">
        <h2 className="text-sm font-semibold">{title}</h2>
        {subtitle ? <p className="mt-0.5 truncate font-mono text-[11px] text-stone-500">{subtitle}</p> : null}
      </div>
      <div className="p-3">{children}</div>
    </section>
  );
}

function Notice({ title, detail }: { title: string; detail: string | null }) {
  return (
    <div className="rounded-md border border-amber-300 bg-amber-50 p-3 text-sm">
      <div className="font-semibold text-amber-900">{title}</div>
      {detail ? <div className="mt-1 font-mono text-xs text-amber-800">{detail}</div> : null}
    </div>
  );
}

function Muted({ children }: { children: React.ReactNode }) {
  return <p className="text-sm text-stone-500">{children}</p>;
}

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
