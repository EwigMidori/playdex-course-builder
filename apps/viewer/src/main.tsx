import React from "react";
import ReactDOM from "react-dom/client";
import ReactMarkdown from "react-markdown";
import { useAtom } from "jotai";
import remarkGfm from "remark-gfm";
import { mdxToMarkdownPreview, type MdxPreviewIndex } from "./mdxPreview";
import { loadManifest, loadQuestionBank, loadRelevance, loadStep, loadTextbook, textbookPath } from "./data";
import { lessons, selectedAssetViewAtom, selectedDataModeAtom, selectedLessonAtom, selectedStepAtom } from "./state";
import type {
  AssetView,
  DataMode,
  LessonId,
  LessonOption,
  QuestionBank,
  QuestionFamily,
  RelevanceReport,
  RelevanceScore,
  StoryManifest,
  StoryScreen,
  StoryStep
} from "./types";
import "./styles.css";

type AsyncState<T> = {
  loading: boolean;
  data: T | null;
  error: string | null;
  path?: string;
};

const emptyState = <T,>(): AsyncState<T> => ({ loading: true, data: null, error: null });

function useTextbook(lessonId: LessonId, mode: DataMode) {
  const [state, setState] = React.useState<AsyncState<string>>(emptyState);
  React.useEffect(() => {
    let alive = true;
    setState(emptyState());
    loadTextbook(lessonId, mode).then((result) => {
      if (alive) {
        setState({ loading: false, data: result.data, error: result.error, path: result.path });
      }
    });
    return () => {
      alive = false;
    };
  }, [lessonId, mode]);
  return state;
}

function useStoryData(lessonId: LessonId, stepId: string, mode: DataMode) {
  const lesson = lessons.find((item) => item.id === lessonId);
  const [manifest, setManifest] = React.useState<AsyncState<StoryManifest>>(emptyState);
  const [step, setStep] = React.useState<AsyncState<StoryStep>>(emptyState);
  const [bank, setBank] = React.useState<AsyncState<QuestionBank>>(emptyState);

  React.useEffect(() => {
    let alive = true;
    setManifest(emptyState());
    loadManifest(lessonId, lesson?.legacySteps, mode).then((result) => {
      if (alive) {
        setManifest({ loading: false, data: result.data, error: result.error, path: result.path });
      }
    });
    return () => {
      alive = false;
    };
  }, [lesson?.legacySteps, lessonId, mode]);

  React.useEffect(() => {
    let alive = true;
    setStep(emptyState());
    setBank(emptyState());
    Promise.all([loadStep(lessonId, stepId, mode), loadQuestionBank(lessonId, stepId, mode)]).then(([stepResult, bankResult]) => {
      if (alive) {
        setStep({ loading: false, data: stepResult.data, error: stepResult.error, path: stepResult.path });
        setBank({ loading: false, data: bankResult.data, error: bankResult.error, path: bankResult.path });
      }
    });
    return () => {
      alive = false;
    };
  }, [lessonId, stepId, mode]);

  return { manifest, step, bank };
}

function useRelevance(lessonId: LessonId, mode: DataMode) {
  const [state, setState] = React.useState<AsyncState<RelevanceReport>>(emptyState);
  React.useEffect(() => {
    let alive = true;
    setState(emptyState());
    loadRelevance(lessonId, mode).then((result) => {
      if (alive) {
        setState({ loading: false, data: result.data, error: result.error, path: result.path });
      }
    });
    return () => {
      alive = false;
    };
  }, [lessonId, mode]);
  return state;
}

function useQuestionIndex(lessonId: LessonId, mode: DataMode) {
  const lesson = lessons.find((item) => item.id === lessonId);
  const [state, setState] = React.useState<AsyncState<MdxPreviewIndex>>(emptyState);

  React.useEffect(() => {
    let alive = true;
    setState(emptyState());

    loadManifest(lessonId, lesson?.legacySteps, mode).then(async (manifest) => {
      const steps = manifest.data?.steps ?? [{ sequence_id: lesson?.defaultStep ?? "step1" }];
      const banks = await Promise.all(
        steps.map((step) => loadQuestionBank(lessonId, step.sequence_id ?? lesson?.defaultStep ?? "step1", mode))
      );
      const families = new Map<string, QuestionFamily>();
      const questions = new Map<string, { family: QuestionFamily; variant: NonNullable<QuestionFamily["variants"]>[number] }>();

      for (const bank of banks) {
        for (const family of [
          ...(bank.data?.flashcard_families ?? []),
          ...(bank.data?.quiz_families ?? []),
          ...(bank.data?.longform_families ?? [])
        ]) {
          if (family.family_id) {
            families.set(family.family_id, family);
          }
          for (const variant of family.variants ?? []) {
            if (variant.question_id) {
              questions.set(variant.question_id, { family, variant });
            }
          }
        }
      }

      if (alive) {
        setState({
          loading: false,
          data: { families, questions },
          error: manifest.error,
          path: manifest.path
        });
      }
    });

    return () => {
      alive = false;
    };
  }, [lesson?.defaultStep, lesson?.legacySteps, lessonId, mode]);

  return state;
}

function App() {
  const [lessonId, setLessonId] = useAtom(selectedLessonAtom);
  const [assetView, setAssetView] = useAtom(selectedAssetViewAtom);
  const [selectedStep, setSelectedStep] = useAtom(selectedStepAtom);
  const [dataMode, setDataMode] = useAtom(selectedDataModeAtom);
  const relevance = useRelevance(lessonId, dataMode);
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
            <h1 className="text-base font-semibold">Course Builder Workspace</h1>
            <p className="text-xs text-stone-500">Creator review for generated course artifacts</p>
          </div>
          <div className="ml-auto flex flex-wrap gap-2 text-xs">
            <Badge label={currentLesson.courseLabel} />
            <Badge label={`${currentLesson.lectureLabel} / ${currentLesson.id}`} />
            <DataModeToggle mode={dataMode} onChange={setDataMode} />
            <RelevanceBadge report={relevance.data} sequenceId={selectedStep} />
          </div>
        </div>
      </header>

      <main className="mx-auto grid max-w-[1500px] gap-4 px-4 py-4 lg:grid-cols-[280px_minmax(0,1fr)]">
        <WorkspaceNav
          assetView={assetView}
          lessonId={lessonId}
          onAssetViewChange={setAssetView}
          onLessonChange={changeLesson}
          dataMode={dataMode}
          onDataModeChange={setDataMode}
        />
        <div className="min-w-0">
          {assetView === "overview" ? (
            <OverviewView currentLesson={currentLesson} lessonId={lessonId} dataMode={dataMode} relevance={relevance} />
          ) : null}
          {assetView === "textbook" ? <TextbookView lessonId={lessonId} mode={dataMode} relevance={relevance.data} /> : null}
          {assetView === "story" ? (
            <StoryView
              lessonId={lessonId}
              selectedStep={selectedStep}
              setSelectedStep={setSelectedStep}
              mode={dataMode}
              relevance={relevance.data}
            />
          ) : null}
          {assetView === "questions" ? (
            <QuestionsView
              lessonId={lessonId}
              relevance={relevance.data}
              selectedStep={selectedStep}
              setSelectedStep={setSelectedStep}
              mode={dataMode}
            />
          ) : null}
          {assetView === "relevance" ? <RelevanceView relevance={relevance} /> : null}
        </div>
      </main>
    </div>
  );
}

function WorkspaceNav({
  assetView,
  lessonId,
  dataMode,
  onAssetViewChange,
  onLessonChange,
  onDataModeChange
}: {
  assetView: AssetView;
  lessonId: LessonId;
  dataMode: DataMode;
  onAssetViewChange: (value: AssetView) => void;
  onLessonChange: (value: LessonId) => void;
  onDataModeChange: (value: DataMode) => void;
}) {
  const views: Array<{ value: AssetView; label: string; detail: string }> = [
    { value: "overview", label: "Overview", detail: "creator checklist" },
    { value: "textbook", label: "MDX viewer", detail: "lesson textbook" },
    { value: "story", label: "Story viewer", detail: "guided screens" },
    { value: "questions", label: "Practice viewer", detail: "flashcards / quiz / longform" },
    { value: "relevance", label: "Relevance", detail: "sidecar scores" }
  ];

  return (
    <aside className="space-y-4">
      <Panel title="Lectures" subtitle="select generated lesson">
        <div className="space-y-2">
          {lessons.map((lesson) => (
            <button
              className={[
                "w-full rounded-md border px-3 py-2 text-left text-sm",
                lessonId === lesson.id ? "border-emerald-700 bg-emerald-50" : "border-stone-300 bg-white hover:border-stone-500"
              ].join(" ")}
              key={lesson.id}
              onClick={() => onLessonChange(lesson.id)}
              type="button"
            >
              <div className="flex items-center justify-between gap-2">
                <span className="font-semibold">{lesson.lectureLabel}</span>
                <span className="font-mono text-xs text-stone-500">{lesson.id}</span>
              </div>
              <div className="mt-1 text-xs text-stone-600">{lesson.label}</div>
            </button>
          ))}
        </div>
      </Panel>
      <Panel title="Review Surface" subtitle="choose creator workflow">
        <div className="space-y-1">
          {views.map((view) => (
            <button
              className={[
                "w-full rounded-md px-3 py-2 text-left",
                assetView === view.value ? "bg-stone-950 text-white" : "bg-white text-stone-800 hover:bg-stone-100"
              ].join(" ")}
              key={view.value}
              onClick={() => onAssetViewChange(view.value)}
              type="button"
            >
              <div className="text-sm font-semibold">{view.label}</div>
              <div className={assetView === view.value ? "text-xs text-stone-300" : "text-xs text-stone-500"}>
                {view.detail}
              </div>
            </button>
          ))}
        </div>
      </Panel>
      <Panel title="Data Mode" subtitle="toggle dataset density">
        <DataModeToggleBlock mode={dataMode} onChange={onDataModeChange} />
      </Panel>
    </aside>
  );
}

function OverviewView({
  currentLesson,
  lessonId,
  dataMode,
  relevance
}: {
  currentLesson: LessonOption;
  lessonId: LessonId;
  dataMode: DataMode;
  relevance: AsyncState<RelevanceReport>;
}) {
  return (
    <section className="grid gap-4 xl:grid-cols-3">
      <Panel title="Creator Context" subtitle={`${currentLesson.courseLabel} / ${currentLesson.lectureLabel}`}>
        <div className="space-y-3 text-sm">
          <div>
            <div className="text-xs font-semibold uppercase tracking-wide text-stone-500">Current lecture</div>
            <div className="mt-1 font-semibold">{currentLesson.label}</div>
          </div>
          <div className="grid grid-cols-2 gap-2">
            <MetricCard label="lesson id" value={lessonId} />
            <MetricCard label="default step" value={currentLesson.defaultStep} />
            <MetricCard label="data mode" value={dataMode} />
          </div>
          <Notice
            title="Authoring stance"
            detail="Flashcards are active-recall prompts for spaced repetition. Quiz items are assessment-like checks. Review them separately."
          />
        </div>
      </Panel>
      <Panel title="Primary Files" subtitle="read-only pipeline sources">
        <div className="space-y-2 text-xs">
          <FileLine path={textbookPath(lessonId)} />
          <FileLine path={`research/pipeline/3-guided_story/${lessonId}/manifest.json`} />
          <FileLine path={`research/pipeline/3-guided_story/${lessonId}/<step>/question_bank.json`} />
          <FileLine path={`research/pipeline/meta/${lessonId}/relevance/report.json`} />
        </div>
      </Panel>
      <Panel title="Quality Signals" subtitle={relevance.path}>
        {relevance.loading ? <Muted>Loading relevance report...</Muted> : null}
        {relevance.error ? <Notice title="Relevance missing" detail={relevance.error} /> : null}
        {relevance.data ? (
          <div className="grid grid-cols-2 gap-2">
            <MetricCard label="steps scored" value={String(relevance.data.step_scores?.length ?? 0)} />
            <MetricCard label="question families" value={String(relevance.data.question_family_scores?.length ?? 0)} />
            <MetricCard label="textbook sections" value={String(relevance.data.textbook_section_scores?.length ?? 0)} />
            <MetricCard label="coverage items" value={String(relevance.data.coverage_scores?.length ?? 0)} />
          </div>
        ) : null}
      </Panel>
    </section>
  );
}

function MetricCard({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-md border border-stone-300 bg-white p-2">
      <div className="text-[11px] uppercase tracking-wide text-stone-500">{label}</div>
      <div className="mt-1 truncate font-mono text-sm font-semibold">{value}</div>
    </div>
  );
}

function FileLine({ path }: { path: string }) {
  return <div className="truncate rounded border border-stone-200 bg-white px-2 py-1 font-mono text-stone-700">{path}</div>;
}

function DataModeToggle({
  mode,
  onChange
}: {
  mode: DataMode;
  onChange: (value: DataMode) => void;
}) {
  return (
    <div className="inline-flex rounded border border-stone-300 bg-white p-0.5 text-[11px]">
      {[
        { value: "real" as const, label: "Real" },
        { value: "stress" as const, label: "Stress" }
      ].map((item) => (
        <button
          className={[
            "rounded px-2 py-1 font-medium",
            mode === item.value ? "bg-stone-950 text-white" : "text-stone-600 hover:text-stone-900"
          ].join(" ")}
          key={item.value}
          onClick={() => onChange(item.value)}
          type="button"
        >
          {item.label}
        </button>
      ))}
    </div>
  );
}

function DataModeToggleBlock({ mode, onChange }: { mode: DataMode; onChange: (value: DataMode) => void }) {
  return (
    <div className="space-y-2">
      <DataModeToggle mode={mode} onChange={onChange} />
      <p className="text-xs leading-5 text-stone-600">
        Stress mode synthesizes 30-plus rows per list so you can test scrolling, density, and pagination behavior without
        changing real pipeline artifacts.
      </p>
    </div>
  );
}

function TextbookView({ lessonId, mode, relevance }: { lessonId: LessonId; mode: DataMode; relevance: RelevanceReport | null }) {
  const textbook = useTextbook(lessonId, mode);
  const questionIndex = useQuestionIndex(lessonId, mode);
  const preview = React.useMemo(
    () => (textbook.data ? mdxToMarkdownPreview(textbook.data, questionIndex.data ?? undefined) : ""),
    [questionIndex.data, textbook.data]
  );

  return (
    <section className="grid gap-4 lg:grid-cols-[minmax(0,1fr)_320px]">
      <Panel title="MDX preview" subtitle={textbook.path}>
        {textbook.loading ? <Muted>Loading textbook...</Muted> : null}
        {textbook.error ? <Notice title="Textbook unavailable" detail={textbook.error} /> : null}
        {questionIndex.error ? <Notice title="Practice lookup degraded" detail={questionIndex.error} /> : null}
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
  mode,
  relevance
}: {
  lessonId: LessonId;
  selectedStep: string;
  setSelectedStep: (step: string) => void;
  mode: DataMode;
  relevance: RelevanceReport | null;
}) {
  const { manifest, step, bank } = useStoryData(lessonId, selectedStep, mode);
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
        {bank.data ? <QuestionBankView bank={bank.data} relevance={relevance} selectedStep={selectedStep} compact /> : null}
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
  selectedStep,
  compact = false
}: {
  bank: QuestionBank;
  relevance: RelevanceReport | null;
  selectedStep: string;
  compact?: boolean;
}) {
  return (
    <div className="space-y-4">
      {!compact ? <CoverageMap bank={bank} /> : null}
      <FamilyGroup
        title="Flashcards"
        subtitle="active recall for spaced repetition"
        families={bank.flashcard_families}
        kind="flashcard"
        relevance={relevance}
        selectedStep={selectedStep}
      />
      <FamilyGroup
        title="Quiz"
        subtitle="assessment-like checks"
        families={bank.quiz_families}
        kind="quiz"
        relevance={relevance}
        selectedStep={selectedStep}
      />
      <FamilyGroup
        title="Longform"
        subtitle="explain, derive, compare, apply"
        families={bank.longform_families}
        kind="longform"
        relevance={relevance}
        selectedStep={selectedStep}
      />
    </div>
  );
}

function QuestionsView({
  lessonId,
  relevance,
  selectedStep,
  setSelectedStep,
  mode
}: {
  lessonId: LessonId;
  relevance: RelevanceReport | null;
  selectedStep: string;
  setSelectedStep: (step: string) => void;
  mode: DataMode;
}) {
  const { manifest, bank } = useStoryData(lessonId, selectedStep, mode);
  const steps = manifest.data?.steps ?? [];

  return (
    <section className="grid gap-4 xl:grid-cols-[280px_minmax(0,1fr)]">
      <Panel title="Lecture Steps" subtitle={manifest.path}>
        {manifest.loading ? <Muted>Loading manifest...</Muted> : null}
        {manifest.error ? <Notice title="Manifest fallback" detail={manifest.error} /> : null}
        <div className="space-y-2">
          {steps.map((stepItem) => (
            <button
              className={[
                "w-full rounded-md border px-3 py-2 text-left text-sm",
                selectedStep === stepItem.sequence_id ? "border-emerald-700 bg-emerald-50" : "border-stone-300 bg-white"
              ].join(" ")}
              key={stepItem.sequence_id}
              onClick={() => setSelectedStep(stepItem.sequence_id ?? "step1")}
              type="button"
            >
              <div className="font-semibold">{stepItem.sequence_id}</div>
              <div className="mt-1 text-xs text-stone-600">{stepItem.concept ?? stepItem.file}</div>
            </button>
          ))}
        </div>
      </Panel>
      <Panel title={`Practice assets - ${selectedStep}`} subtitle={bank.path}>
        {bank.loading ? <Muted>Loading question bank...</Muted> : null}
        {bank.error ? <Notice title="Question bank unavailable" detail={bank.error} /> : null}
        {bank.data ? (
          <div className="space-y-4">
            <Notice
              title="Flashcards are not quiz questions"
              detail="Review front/back cards for active recall. Keep options and distractors in Quiz, where assessment-style wording belongs."
            />
            <QuestionBankView bank={bank.data} relevance={relevance} selectedStep={selectedStep} />
          </div>
        ) : null}
      </Panel>
    </section>
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
  subtitle,
  families = [],
  kind,
  relevance,
  selectedStep
}: {
  title: string;
  subtitle: string;
  families?: QuestionFamily[];
  kind: "flashcard" | "quiz" | "longform";
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
      <p className="mb-2 text-xs text-stone-500">{subtitle}</p>
      <div className="space-y-3">
        {filtered.map((family) => (
          <QuestionFamilyCard family={family} key={family.family_id} kind={kind} relevance={relevance} />
        ))}
        {filtered.length === 0 ? <Muted>No families linked to this step.</Muted> : null}
      </div>
    </div>
  );
}

function QuestionFamilyCard({
  family,
  kind,
  relevance
}: {
  family: QuestionFamily;
  kind: "flashcard" | "quiz" | "longform";
  relevance: RelevanceReport | null;
}) {
  const score = relevance?.question_family_scores?.find((item) => item.family_id === family.family_id);
  return (
    <article className="rounded-md border border-stone-300 bg-white p-3">
      <div className="mb-2 flex flex-wrap items-center gap-2">
        <span className="font-mono text-xs font-semibold">{family.family_id}</span>
        {family.difficulty ? <Badge label={family.difficulty} /> : null}
        {family.question_type ? <Badge label={family.question_type} tone="blue" /> : null}
        <Badge label={kind === "flashcard" ? "active recall" : kind} tone={kind === "flashcard" ? "emerald" : "stone"} />
        {score ? <TreatmentBadge score={score} /> : null}
      </div>
      {family.retrieval_focus ? <p className="mb-2 text-xs text-emerald-800">{family.retrieval_focus}</p> : null}
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
            <summary className="cursor-pointer text-sm font-medium">
              {variant.question_id ?? variant.front ?? variant.stem}
            </summary>
            <div className="mt-2 text-sm">
              {variant.front ? (
                <div className="rounded border border-emerald-200 bg-emerald-50 p-2">
                  <div className="text-[11px] font-semibold uppercase tracking-wide text-emerald-800">front</div>
                  <p className="mt-1 font-medium">{variant.front}</p>
                </div>
              ) : null}
              {variant.back ? (
                <div className="mt-2 rounded border border-stone-300 bg-white p-2">
                  <div className="text-[11px] font-semibold uppercase tracking-wide text-stone-500">back</div>
                  <p className="mt-1">{variant.back}</p>
                </div>
              ) : null}
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
