import React from "react";
import { Check, ChevronDown, RotateCw, Sparkles } from "lucide-react";
import katex from "katex";
import "katex/dist/katex.min.css";
import type { ChapterRecord, QuestionIndex } from "../player-model";
import type { QuestionFamily, QuestionVariant } from "../player-model";

type MdxComponentMap = Record<string, React.ComponentType<any>>;

function dirname(path: string | undefined) {
  if (!path) {
    return "";
  }
  const lastSlash = path.lastIndexOf("/");
  return lastSlash >= 0 ? path.slice(0, lastSlash) : "";
}

function joinPath(base: string, next: string) {
  return [base.replace(/^\/+|\/+$/g, ""), next.replace(/^\/+/, "")].filter(Boolean).join("/");
}

function normalizePublicPath(path: string) {
  return `/${path.replace(/^\/+/, "")}`;
}

function resolveFigureSource(chapter: ChapterRecord, src?: string) {
  if (!src) {
    return null;
  }

  if (src.startsWith("http://") || src.startsWith("https://") || src.startsWith("/")) {
    return src;
  }

  const normalized = src.replace(/^\.\//, "");
  if (normalized.startsWith("images/") && chapter.plainOutputDir) {
    return normalizePublicPath(joinPath(`${chapter.plainOutputDir}/artifacts`, normalized));
  }

  const textbookDir = dirname(chapter.textbookPath);
  if (textbookDir) {
    return normalizePublicPath(joinPath(textbookDir, normalized));
  }

  if (chapter.plainOutputDir) {
    return normalizePublicPath(joinPath(chapter.plainOutputDir, normalized));
  }

  return normalizePublicPath(normalized);
}

function getPracticeMode(family?: QuestionFamily) {
  const familyId = family?.family_id ?? "";
  if (familyId.includes("_long_")) {
    return { label: "Deep Dive", tone: "longform" };
  }
  if (familyId.includes("_quiz_")) {
    return { label: "Quick Check", tone: "quiz" };
  }
  return { label: "Recall", tone: "flashcard" };
}

function resolvePrompt(variant?: QuestionVariant, family?: QuestionFamily) {
  return variant?.front ?? variant?.stem ?? family?.retrieval_focus ?? family?.learning_goal ?? "Practice prompt";
}

function resolveAnswer(variant?: QuestionVariant) {
  if (!variant) {
    return undefined;
  }
  if (variant.back) {
    return variant.back;
  }
  if (variant.reference_answer?.length) {
    return variant.reference_answer.join(" ");
  }
  if (typeof variant.answer === "string") {
    return variant.answer;
  }
  return variant.explanation;
}

function resolveExplanation(variant?: QuestionVariant) {
  if (!variant) {
    return undefined;
  }
  if (variant.back || variant.reference_answer?.length) {
    return variant.explanation;
  }
  return undefined;
}

function normalizeCorrectOptions(answer: QuestionVariant["answer"]) {
  if (typeof answer === "number") {
    return [answer];
  }
  if (Array.isArray(answer)) {
    return answer.filter((value): value is number => typeof value === "number");
  }
  return [];
}

function sameNumberSet(left: number[], right: number[]) {
  if (left.length !== right.length) {
    return false;
  }
  const leftSorted = [...left].sort((a, b) => a - b);
  const rightSorted = [...right].sort((a, b) => a - b);
  return leftSorted.every((value, index) => value === rightSorted[index]);
}

function FormulaBlock({ latex, en, children }: { latex?: string; en?: string; children?: React.ReactNode }) {
  const result = React.useMemo(() => {
    if (!latex) {
      return {
        html: null,
        error: "Formula is missing the required `latex` attribute."
      };
    }

    try {
      return {
        html: katex.renderToString(latex, {
          displayMode: true,
          output: "html",
          throwOnError: true,
          strict: "warn"
        }),
        error: null
      };
    } catch (error) {
      return {
        html: null,
        error: error instanceof Error ? error.message : String(error)
      };
    }
  }, [latex]);

  return (
    <figure className={`textbook-formula${result.error ? " textbook-formula--error" : ""}`}>
      <div className="textbook-formula__label">
        <Sparkles size={14} />
        <span>{en ?? "Formula"}</span>
      </div>
      {result.html ? (
        <div className="textbook-formula__rendered" dangerouslySetInnerHTML={{ __html: result.html }} />
      ) : (
        <div className="textbook-formula__error" role="alert">
          <div className="textbook-formula__error-title">Formula render failed</div>
          <p>{result.error}</p>
          {latex ? <code className="textbook-formula__expression">{latex}</code> : null}
        </div>
      )}
      {children ? <figcaption className="textbook-formula__body">{children}</figcaption> : null}
    </figure>
  );
}

function InlineFormula({ latex }: { latex?: string }) {
  const result = React.useMemo(() => {
    if (!latex) {
      return {
        html: null,
        error: "Inline formula is missing the required `latex` attribute."
      };
    }

    try {
      return {
        html: katex.renderToString(latex, {
          displayMode: false,
          output: "html",
          throwOnError: true,
          strict: "warn"
        }),
        error: null
      };
    } catch (error) {
      return {
        html: null,
        error: error instanceof Error ? error.message : String(error)
      };
    }
  }, [latex]);

  if (result.html) {
    return <span className="textbook-inline-formula" dangerouslySetInnerHTML={{ __html: result.html }} />;
  }

  return (
    <span className="textbook-inline-formula textbook-inline-formula--error" role="alert" title={result.error ?? undefined}>
      {latex ?? "inline formula"}
    </span>
  );
}

function TextbookCard({
  tone,
  label,
  title,
  chrome = "default",
  children
}: {
  tone: string;
  label: string;
  title?: string;
  chrome?: "default" | "focus";
  children?: React.ReactNode;
}) {
  return (
    <section className={`textbook-card textbook-card--${tone} textbook-card--${chrome}`}>
      <div className="textbook-card__label">{label}</div>
      {title ? <h4 className="textbook-card__title">{title}</h4> : null}
      <div className="textbook-card__body">{children}</div>
    </section>
  );
}

function PracticeCard({
  title,
  eyebrow,
  prompt,
  answer,
  explanation,
  options,
  correctOptions,
  modeTone,
  meta,
  onAdvance,
  advanceLabel
}: {
  title: string;
  eyebrow: string;
  prompt: string;
  answer?: string;
  explanation?: string;
  options?: string[];
  correctOptions?: number[];
  modeTone: string;
  meta?: string;
  onAdvance?: () => void;
  advanceLabel?: string;
}) {
  const [revealed, setRevealed] = React.useState(false);
  const [selected, setSelected] = React.useState<number[]>([]);
  const [checked, setChecked] = React.useState(false);
  const [reflectionReady, setReflectionReady] = React.useState(false);
  const hasChoices = Boolean(options?.length && correctOptions?.length);
  const multiSelect = (correctOptions?.length ?? 0) > 1;
  const isCorrect = checked && sameNumberSet(selected, correctOptions ?? []);

  React.useEffect(() => {
    setRevealed(false);
    setSelected([]);
    setChecked(false);
    setReflectionReady(false);
  }, [prompt]);

  const toggleOption = (index: number) => {
    if (!hasChoices || checked) {
      return;
    }
    setSelected((current) => {
      if (multiSelect) {
        return current.includes(index) ? current.filter((value) => value !== index) : [...current, index];
      }
      return current[0] === index ? [] : [index];
    });
  };

  const reset = () => {
    setSelected([]);
    setChecked(false);
    setReflectionReady(false);
    setRevealed(false);
  };

  const statusText = hasChoices
    ? checked
      ? isCorrect
        ? "Correct. You matched the expected answer."
        : "Not quite. Compare your choice against the expected answer below."
      : multiSelect
        ? "Select all answers that apply, then check."
        : "Choose one answer, then check."
    : revealed
      ? "Compare your thinking against the reference answer."
      : reflectionReady
        ? "Good. Now reveal the reference answer."
        : "Pause and commit to an answer before revealing.";

  return (
    <aside className={`inline-practice inline-practice--${modeTone}`}>
      <div className="inline-practice__top">
        <div className="inline-practice__head">
          <span className="inline-practice__eyebrow">{eyebrow}</span>
          <h4>{title}</h4>
        </div>
        {meta ? <span className="inline-practice__meta">{meta}</span> : null}
      </div>

      <div className="inline-practice__panel">
        <div className="inline-practice__prompt-label">Prompt</div>
        <p className="inline-practice__prompt">{prompt}</p>
        <div className="inline-practice__status">{statusText}</div>

        {options?.length ? (
          <div className="inline-practice__options" role="list">
            {options.map((option, index) => {
              const isSelected = selected.includes(index);
              const isCorrectOption = Boolean(checked && correctOptions?.includes(index));
              const isIncorrectSelection = Boolean(checked && isSelected && !isCorrectOption);
              return (
                <button
                  className={[
                    "inline-practice__option",
                    isSelected ? "is-selected" : "",
                    isCorrectOption ? "is-correct" : "",
                    isIncorrectSelection ? "is-incorrect" : "",
                    checked && !isSelected && isCorrectOption ? "is-missed" : ""
                  ]
                    .filter(Boolean)
                    .join(" ")}
                  key={`${prompt}-option-${index}`}
                  onClick={() => toggleOption(index)}
                  role="listitem"
                  type="button"
                >
                  <span className="inline-practice__option-index">{index + 1}</span>
                  <span>{option}</span>
                </button>
              );
            })}
          </div>
        ) : null}

        {((hasChoices && checked) || (!hasChoices && revealed)) && (answer || explanation) ? (
          <div className="inline-practice__answer">
            {answer ? (
              <div className="inline-practice__answer-block">
                <div className="inline-practice__answer-label">Answer</div>
                <p>{answer}</p>
              </div>
            ) : null}
            {explanation ? (
              <div className="inline-practice__answer-block">
                <div className="inline-practice__answer-label">Why it works</div>
                <p>{explanation}</p>
              </div>
            ) : null}
          </div>
        ) : null}
      </div>

      <div className="inline-practice__actions">
        {hasChoices ? (
          <>
            <button
              className="textbook-action textbook-action--primary"
              disabled={checked || selected.length === 0}
              onClick={() => setChecked(true)}
              type="button"
            >
              <Check size={15} />
              <span>Check answer</span>
            </button>
            {checked ? (
              <button className="textbook-action" onClick={reset} type="button">
                <RotateCw size={15} />
                <span>Reset</span>
              </button>
            ) : null}
          </>
        ) : !reflectionReady ? (
          <button className="textbook-action textbook-action--primary" onClick={() => setReflectionReady(true)} type="button">
            <Check size={15} />
            <span>I&apos;ve thought about it</span>
          </button>
        ) : !revealed ? (
          <button className="textbook-action textbook-action--primary" onClick={() => setRevealed(true)} type="button">
            <ChevronDown size={16} />
            <span>Reveal answer</span>
          </button>
        ) : (
          <button className="textbook-action" onClick={reset} type="button">
            <RotateCw size={15} />
            <span>Reset</span>
          </button>
        )}
        {onAdvance && ((hasChoices && checked) || (!hasChoices && revealed)) ? (
          <button className="textbook-action textbook-action--ghost" onClick={onAdvance} type="button">
            <RotateCw size={15} />
            <span>{advanceLabel ?? "Next prompt"}</span>
          </button>
        ) : null}
      </div>
    </aside>
  );
}

function QuestionRefCard({ family, variant }: { family?: QuestionFamily; variant?: QuestionVariant }) {
  const mode = getPracticeMode(family);
  const prompt = resolvePrompt(variant, family);
  const answer = resolveAnswer(variant);
  const explanation = resolveExplanation(variant);
  const options = variant?.options;
  const correctOptions = normalizeCorrectOptions(variant?.answer);

  return (
    <PracticeCard
      answer={answer}
      correctOptions={correctOptions}
      eyebrow={mode.label}
      explanation={explanation}
      meta={variant?.estimated_seconds ? `${variant.estimated_seconds}s` : undefined}
      modeTone={mode.tone}
      options={options}
      prompt={prompt}
      title="Quick Practice"
    />
  );
}

function QuestionFamilyCard({ familyId, family }: { familyId: string; family?: QuestionFamily }) {
  const variants = family?.variants ?? [];
  const [activeIndex, setActiveIndex] = React.useState(0);
  const activeVariant = variants[activeIndex] ?? variants[0];
  const mode = getPracticeMode(family);
  const prompt = resolvePrompt(activeVariant, family);
  const answer = resolveAnswer(activeVariant);
  const explanation = resolveExplanation(activeVariant);
  const meta = [`${variants.length || 1} prompt${variants.length === 1 ? "" : "s"}`, family?.difficulty]
    .filter(Boolean)
    .join(" · ");

  React.useEffect(() => {
    setActiveIndex(0);
  }, [familyId]);

  return (
    <PracticeCard
      answer={answer}
      advanceLabel="Next prompt"
      correctOptions={normalizeCorrectOptions(activeVariant?.answer)}
      eyebrow={mode.label}
      explanation={explanation}
      meta={meta || undefined}
      modeTone={mode.tone}
      onAdvance={
        variants.length > 1
          ? () => {
              setActiveIndex((index) => (index + 1) % variants.length);
            }
          : undefined
      }
      options={activeVariant?.options}
      prompt={prompt}
      title={family?.learning_goal ?? familyId}
    />
  );
}

export function buildTextbookMdxComponents({
  chapter,
  practice
}: {
  chapter: ChapterRecord;
  practice: QuestionIndex | null;
}): MdxComponentMap {
  const QuestionRef = ({ id }: { id: string }) => {
    const entry = practice?.questions.get(id);
    return <QuestionRefCard family={entry?.family} variant={entry?.variant} />;
  };

  const QuestionFamily = ({ familyId }: { familyId: string }) => {
    const family = practice?.families.get(familyId);
    return <QuestionFamilyCard family={family} familyId={familyId} />;
  };

  return {
    Checkpoint: ({ title, children }: { title?: string; children?: React.ReactNode }) => (
      <TextbookCard chrome="focus" label="Checkpoint" title={title} tone="checkpoint">
        {children}
      </TextbookCard>
    ),
    Definition: ({ title, children }: { title?: string; children?: React.ReactNode }) => (
      <TextbookCard label="Definition" title={title} tone="definition">
        {children}
      </TextbookCard>
    ),
    Example: ({ title, children }: { title?: string; children?: React.ReactNode }) => (
      <TextbookCard label="Example" title={title} tone="example">
        {children}
      </TextbookCard>
    ),
    Figure: ({ src, alt, children }: { src?: string; alt?: string; children?: React.ReactNode }) => {
      const resolved = resolveFigureSource(chapter, src);
      return (
        <figure className="textbook-figure">
          {resolved ? <img alt={alt ?? "Course figure"} className="textbook-figure__image" src={resolved} /> : null}
          <figcaption className="textbook-figure__caption">
            <div className="textbook-figure__label">Figure</div>
            {alt ? <div className="textbook-figure__title">{alt}</div> : null}
            {children ? <div className="textbook-figure__body">{children}</div> : null}
          </figcaption>
        </figure>
      );
    },
    InlineFormula: ({ latex }: { latex?: string }) => <InlineFormula latex={latex} />,
    Formula: ({ latex, en, children }: { latex?: string; en?: string; children?: React.ReactNode }) => (
      <FormulaBlock en={en} latex={latex}>
        {children}
      </FormulaBlock>
    ),
    KeyPoint: ({ children }: { children?: React.ReactNode }) => (
      <TextbookCard label="Key Point" tone="key-point">
        {children}
      </TextbookCard>
    ),
    Pitfall: ({ title, children }: { title?: string; children?: React.ReactNode }) => (
      <TextbookCard label="Pitfall" title={title} tone="pitfall">
        {children}
      </TextbookCard>
    ),
    QuestionFamily,
    QuestionRef,
    Table: ({ children }: { children?: React.ReactNode }) => <div className="textbook-table-wrap">{children}</div>,
    Term: ({ id, en, children }: { id?: string; en?: string; children?: React.ReactNode }) => (
      <span className="textbook-term" title={[id ? `term:${id}` : null, en].filter(Boolean).join(" / ")}>
        <span className="textbook-term__label">{children}</span>
        {en ? <span className="textbook-term__meta">{en}</span> : null}
      </span>
    )
  };
}
