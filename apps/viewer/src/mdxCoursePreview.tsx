import React from "react";
import { evaluate } from "@mdx-js/mdx";
import * as runtime from "react/jsx-runtime";
import remarkGfm from "remark-gfm";
import type { QuestionFamily, QuestionVariant } from "./types";

export type MdxCourseIndex = {
  families: Map<string, QuestionFamily>;
  questions: Map<string, { family: QuestionFamily; variant: QuestionVariant }>;
};

type MdxCoursePreviewProps = {
  lessonId: string;
  source: string;
  index?: MdxCourseIndex;
};

type CompiledMdx = {
  default: React.ComponentType<{ components?: Record<string, React.ComponentType<any>> }>;
};

type PreviewState = {
  loading: boolean;
  error: string | null;
  Content: React.ComponentType<{ components?: Record<string, React.ComponentType<any>> }> | null;
};

const emptyPreviewState = (): PreviewState => ({
  loading: true,
  error: null,
  Content: null
});

const normalizeMdxSource = (source: string) =>
  source
    .replace(/^---[\s\S]*?---\s*/, "")
    .replace(/^(#{1,6}\s+.+?)\s+\{#[^}]+}\s*$/gm, "$1");

const resolveFigureSource = (lessonId: string, src?: string) => {
  if (!src) {
    return null;
  }
  if (src.startsWith("http://") || src.startsWith("https://") || src.startsWith("/")) {
    return src;
  }
  return `/research/pipeline/1-plain/${lessonId}/artifacts/${src.replace(/^\.\//, "")}`;
};

const flattenText = (value: React.ReactNode): string => {
  if (value == null || typeof value === "boolean") {
    return "";
  }
  if (typeof value === "string" || typeof value === "number") {
    return String(value);
  }
  if (Array.isArray(value)) {
    return value.map(flattenText).join("");
  }
  if (React.isValidElement(value)) {
    return flattenText(value.props.children);
  }
  return "";
};

const familyTone = (family?: QuestionFamily) => {
  const id = family?.family_id ?? "";
  if (id.includes("_quiz_")) {
    return {
      label: "Quiz",
      badge: "border-sky-300 bg-sky-50 text-sky-700",
      card: "border-sky-200 bg-sky-50/60"
    };
  }
  if (id.includes("_long_")) {
    return {
      label: "Longform",
      badge: "border-violet-300 bg-violet-50 text-violet-700",
      card: "border-violet-200 bg-violet-50/60"
    };
  }
  return {
    label: "Flashcard",
    badge: "border-emerald-300 bg-emerald-50 text-emerald-700",
    card: "border-emerald-200 bg-emerald-50/60"
  };
};

function VariantPreview({
  family,
  variant,
  compact = false
}: {
  family?: QuestionFamily;
  variant: QuestionVariant;
  compact?: boolean;
}) {
  const tone = familyTone(family);
  const prompt = variant.front ?? variant.stem ?? variant.prompt_blocks?.join(" / ") ?? variant.question_id ?? "Untitled item";
  const answer = variant.back ?? (variant.reference_answer?.length ? variant.reference_answer.join(" ") : undefined);

  return (
    <article className={`rounded-xl border ${tone.card} p-4 shadow-sm`}>
      <div className="mb-3 flex flex-wrap items-center gap-2">
        <span className={`rounded-full border px-2.5 py-1 text-[11px] font-semibold ${tone.badge}`}>{tone.label}</span>
        {family?.difficulty ? (
          <span className="rounded-full border border-stone-300 bg-white px-2.5 py-1 text-[11px] font-medium text-stone-600">
            {family.difficulty}
          </span>
        ) : null}
        {variant.question_id ? (
          <span className="ml-auto font-mono text-[11px] text-stone-500">{variant.question_id}</span>
        ) : null}
      </div>

      <div className="space-y-3">
        <div>
          <div className="text-[11px] font-semibold uppercase tracking-wide text-stone-500">Prompt</div>
          <div className="mt-1 text-sm font-medium leading-6 text-stone-900">{prompt}</div>
        </div>

        {variant.options?.length ? (
          <div className="grid gap-2">
            {variant.options.map((option, index) => {
              const highlighted =
                Array.isArray(variant.answer) ? variant.answer.includes(index) : variant.answer === index;
              return (
                <div
                  className={[
                    "rounded-lg border px-3 py-2 text-sm",
                    highlighted ? "border-stone-900 bg-stone-900 text-white" : "border-stone-200 bg-white text-stone-700"
                  ].join(" ")}
                  key={`${variant.question_id ?? prompt}-option-${index}`}
                >
                  <span className="mr-2 text-xs font-semibold opacity-70">{index + 1}</span>
                  {option}
                </div>
              );
            })}
          </div>
        ) : null}

        {variant.prompt_blocks?.length && !compact ? (
          <div className="rounded-lg border border-stone-200 bg-white p-3">
            <div className="text-[11px] font-semibold uppercase tracking-wide text-stone-500">Answer shape</div>
            <ul className="mt-2 space-y-1 text-sm text-stone-700">
              {variant.prompt_blocks.map((block) => (
                <li key={block}>• {block}</li>
              ))}
            </ul>
          </div>
        ) : null}

        {answer || variant.explanation || variant.reference_answer?.length ? (
          <details className="rounded-lg border border-stone-200 bg-white p-3">
            <summary className="cursor-pointer text-sm font-semibold text-stone-900">Reveal answer and notes</summary>
            {answer ? (
              <div className="mt-3">
                <div className="text-[11px] font-semibold uppercase tracking-wide text-stone-500">Target answer</div>
                <div className="mt-1 text-sm leading-6 text-stone-800">{answer}</div>
              </div>
            ) : null}
            {variant.reference_answer?.length && !answer ? (
              <div className="mt-3 text-sm leading-6 text-stone-800">{variant.reference_answer.join(" ")}</div>
            ) : null}
            {variant.explanation ? <div className="mt-3 text-sm leading-6 text-stone-600">{variant.explanation}</div> : null}
            {variant.rubric_points?.length ? (
              <div className="mt-3">
                <div className="text-[11px] font-semibold uppercase tracking-wide text-stone-500">Rubric</div>
                <ul className="mt-2 space-y-1 text-sm text-stone-700">
                  {variant.rubric_points.map((point) => (
                    <li key={point}>• {point}</li>
                  ))}
                </ul>
              </div>
            ) : null}
          </details>
        ) : null}
      </div>
    </article>
  );
}

function QuestionRefCard({ id, index }: { id: string; index?: MdxCourseIndex }) {
  const entry = index?.questions.get(id);
  if (!entry) {
    return (
      <div className="rounded-xl border border-dashed border-stone-300 bg-stone-50 p-4">
        <div className="text-[11px] font-semibold uppercase tracking-wide text-stone-500">Embedded practice</div>
        <div className="mt-1 font-mono text-sm text-stone-700">{id}</div>
      </div>
    );
  }
  return <VariantPreview family={entry.family} variant={entry.variant} />;
}

function QuestionFamilyCard({ familyId, index }: { familyId: string; index?: MdxCourseIndex }) {
  const family = index?.families.get(familyId);
  if (!family) {
    return (
      <div className="rounded-xl border border-dashed border-stone-300 bg-stone-50 p-4">
        <div className="text-[11px] font-semibold uppercase tracking-wide text-stone-500">Practice set</div>
        <div className="mt-1 font-mono text-sm text-stone-700">{familyId}</div>
      </div>
    );
  }

  const tone = familyTone(family);
  const previewVariants = family.variants?.slice(0, 2) ?? [];

  return (
    <section className="rounded-2xl border border-stone-200 bg-gradient-to-br from-white to-stone-50 p-5 shadow-sm">
      <div className="mb-4 flex flex-wrap items-start gap-3">
        <div>
          <div className={`inline-flex rounded-full border px-2.5 py-1 text-[11px] font-semibold ${tone.badge}`}>
            {tone.label} set
          </div>
          <h4 className="mt-2 text-base font-semibold text-stone-950">{family.learning_goal ?? family.family_id ?? "Practice set"}</h4>
        </div>
        <div className="ml-auto flex flex-wrap gap-2 text-[11px]">
          {family.question_type ? (
            <span className="rounded-full border border-stone-300 bg-white px-2.5 py-1 text-stone-600">{family.question_type}</span>
          ) : null}
          <span className="rounded-full border border-stone-300 bg-white px-2.5 py-1 text-stone-600">
            {family.variants?.length ?? 0} variants
          </span>
        </div>
      </div>

      {family.retrieval_focus ? <p className="mb-4 text-sm leading-6 text-stone-700">{family.retrieval_focus}</p> : null}

      {family.coverage_tags?.length ? (
        <div className="mb-4 flex flex-wrap gap-2">
          {family.coverage_tags.map((tag) => (
            <span className="rounded-full bg-stone-200 px-2.5 py-1 text-[11px] font-medium text-stone-700" key={tag}>
              {tag}
            </span>
          ))}
        </div>
      ) : null}

      <div className="grid gap-3">
        {previewVariants.map((variant) => (
          <VariantPreview compact family={family} key={variant.question_id ?? variant.front ?? variant.stem} variant={variant} />
        ))}
      </div>

      {(family.variants?.length ?? 0) > previewVariants.length ? (
        <div className="mt-4 text-xs text-stone-500">Additional variants stay available in the Practice viewer.</div>
      ) : null}
    </section>
  );
}

function MermaidFlow({ value }: { value: string }) {
  const nodes = Array.from(
    value.matchAll(/[A-Za-z0-9_]+\[([^\]]+)\]/g),
    (match) => match[1].replace(/<br\s*\/?>/gi, "\n").trim()
  ).filter(Boolean);
  const unique = nodes.filter((node, index) => nodes.indexOf(node) === index);

  if (!unique.length) {
    return (
      <pre className="overflow-x-auto rounded-2xl bg-stone-950 p-4 text-xs text-stone-100">
        <code>{value}</code>
      </pre>
    );
  }

  return (
    <div className="rounded-2xl border border-stone-200 bg-gradient-to-br from-stone-950 to-stone-800 p-4 text-stone-50 shadow-sm">
      <div className="mb-3 text-[11px] font-semibold uppercase tracking-[0.2em] text-stone-400">Flow</div>
      <div className="grid gap-3 md:grid-cols-2 xl:grid-cols-3">
        {unique.map((node, index) => (
          <div className="rounded-xl border border-white/10 bg-white/5 p-3" key={`${node}-${index}`}>
            <div className="mb-2 text-[10px] font-semibold uppercase tracking-wide text-stone-400">Step {index + 1}</div>
            <div className="whitespace-pre-line text-sm leading-6">{node}</div>
          </div>
        ))}
      </div>
    </div>
  );
}

function createCourseComponents(lessonId: string, index?: MdxCourseIndex): Record<string, React.ComponentType<any>> {
  return {
    h1: ({ children }: { children?: React.ReactNode }) => <h1 className="text-4xl font-semibold tracking-tight text-stone-950">{children}</h1>,
    h2: ({ children, id }: { children?: React.ReactNode; id?: string }) => (
      <section className="mt-10 scroll-mt-6" id={id}>
        <div className="mb-4 flex items-center gap-3">
          <div className="h-px flex-1 bg-stone-200" />
          <h2 className="text-2xl font-semibold text-stone-950">{children}</h2>
        </div>
      </section>
    ),
    h3: ({ children }: { children?: React.ReactNode }) => <h3 className="mt-8 text-lg font-semibold text-stone-900">{children}</h3>,
    p: ({ children }: { children?: React.ReactNode }) => <p className="text-[15px] leading-7 text-stone-700">{children}</p>,
    ul: ({ children }: { children?: React.ReactNode }) => <ul className="space-y-2 pl-5 text-[15px] leading-7 text-stone-700">{children}</ul>,
    ol: ({ children }: { children?: React.ReactNode }) => <ol className="space-y-2 pl-5 text-[15px] leading-7 text-stone-700">{children}</ol>,
    li: ({ children }: { children?: React.ReactNode }) => <li>{children}</li>,
    strong: ({ children }: { children?: React.ReactNode }) => <strong className="font-semibold text-stone-950">{children}</strong>,
    table: ({ children }: { children?: React.ReactNode }) => (
      <div className="overflow-x-auto rounded-2xl border border-stone-200 bg-white shadow-sm">
        <table className="min-w-full divide-y divide-stone-200 text-sm">{children}</table>
      </div>
    ),
    thead: ({ children }: { children?: React.ReactNode }) => <thead className="bg-stone-100 text-left text-stone-600">{children}</thead>,
    th: ({ children }: { children?: React.ReactNode }) => <th className="px-4 py-3 font-semibold">{children}</th>,
    td: ({ children }: { children?: React.ReactNode }) => <td className="border-t border-stone-100 px-4 py-3 align-top text-stone-700">{children}</td>,
    pre: ({ children }: { children?: React.ReactNode }) => {
      const childArray = React.Children.toArray(children);
      const firstChild = childArray[0];
      if (React.isValidElement(firstChild) && typeof firstChild.props.className === "string") {
        const className = firstChild.props.className;
        const code = flattenText(firstChild.props.children);
        if (className.includes("language-mermaid")) {
          return <MermaidFlow value={code} />;
        }
      }
      return <pre className="overflow-x-auto rounded-2xl bg-stone-950 p-4 text-sm text-stone-100">{children}</pre>;
    },
    code: ({ className, children }: { className?: string; children?: React.ReactNode }) => {
      if (className) {
        return <code className={className}>{children}</code>;
      }
      return <code className="rounded bg-stone-100 px-1.5 py-0.5 font-mono text-[0.92em] text-stone-900">{children}</code>;
    },
    Definition: ({ title, children }: { title?: string; children?: React.ReactNode }) => (
      <div className="rounded-2xl border border-emerald-200 bg-emerald-50/70 p-5 shadow-sm">
        <div className="mb-2 text-[11px] font-semibold uppercase tracking-[0.2em] text-emerald-700">Definition</div>
        {title ? <h4 className="mb-2 text-base font-semibold text-emerald-950">{title}</h4> : null}
        <div className="space-y-3 text-sm leading-7 text-emerald-950">{children}</div>
      </div>
    ),
    Example: ({ title, children }: { title?: string; children?: React.ReactNode }) => (
      <div className="rounded-2xl border border-sky-200 bg-sky-50/80 p-5 shadow-sm">
        <div className="mb-2 text-[11px] font-semibold uppercase tracking-[0.2em] text-sky-700">Example</div>
        {title ? <h4 className="mb-2 text-base font-semibold text-sky-950">{title}</h4> : null}
        <div className="space-y-3 text-sm leading-7 text-sky-950">{children}</div>
      </div>
    ),
    Checkpoint: ({ title, children }: { title?: string; children?: React.ReactNode }) => (
      <div className="rounded-2xl border border-amber-200 bg-amber-50/80 p-5 shadow-sm">
        <div className="mb-2 text-[11px] font-semibold uppercase tracking-[0.2em] text-amber-700">Checkpoint</div>
        {title ? <h4 className="mb-3 text-base font-semibold text-amber-950">{title}</h4> : null}
        <div className="grid gap-4">{children}</div>
      </div>
    ),
    KeyPoint: ({ children }: { children?: React.ReactNode }) => (
      <div className="rounded-2xl border border-stone-900 bg-stone-900 p-5 text-stone-50 shadow-sm">
        <div className="mb-2 text-[11px] font-semibold uppercase tracking-[0.2em] text-stone-400">Key point</div>
        <div className="space-y-3 text-sm leading-7 text-stone-100">{children}</div>
      </div>
    ),
    Pitfall: ({ children }: { children?: React.ReactNode }) => (
      <div className="rounded-2xl border border-rose-200 bg-rose-50/80 p-5 shadow-sm">
        <div className="mb-2 text-[11px] font-semibold uppercase tracking-[0.2em] text-rose-700">Pitfall</div>
        <div className="space-y-3 text-sm leading-7 text-rose-950">{children}</div>
      </div>
    ),
    Term: ({ id, en, children }: { id?: string; en?: string; children?: React.ReactNode }) => (
      <span className="inline-flex items-baseline gap-2 rounded-full border border-stone-300 bg-white px-3 py-1 shadow-sm">
        <span className="font-semibold text-stone-950">{children}</span>
        <span className="text-[11px] text-stone-500">{[id ? `term:${id}` : null, en].filter(Boolean).join(" / ")}</span>
      </span>
    ),
    Figure: ({ src, alt, children }: { src?: string; alt?: string; children?: React.ReactNode }) => {
      const resolved = resolveFigureSource(lessonId, src);
      return (
        <figure className="overflow-hidden rounded-2xl border border-stone-200 bg-white shadow-sm">
          {resolved ? <img alt={alt ?? "Course figure"} className="max-h-[420px] w-full object-cover" src={resolved} /> : null}
          <figcaption className="space-y-2 p-4">
            <div className="text-[11px] font-semibold uppercase tracking-[0.2em] text-stone-500">Figure</div>
            {alt ? <div className="text-sm font-semibold text-stone-900">{alt}</div> : null}
            <div className="text-sm leading-7 text-stone-600">{children}</div>
          </figcaption>
        </figure>
      );
    },
    QuestionRef: ({ id }: { id?: string }) => (id ? <QuestionRefCard id={id} index={index} /> : null),
    QuestionFamily: ({ familyId, id }: { familyId?: string; id?: string }) => (
      <QuestionFamilyCard familyId={familyId ?? id ?? "unknown"} index={index} />
    )
  };
}

export function MdxCoursePreview({ lessonId, source, index }: MdxCoursePreviewProps) {
  const [state, setState] = React.useState<PreviewState>(emptyPreviewState);

  React.useEffect(() => {
    let alive = true;
    setState(emptyPreviewState());

    evaluate(normalizeMdxSource(source), {
      ...runtime,
      remarkPlugins: [remarkGfm]
    })
      .then((compiled) => {
        if (!alive) {
          return;
        }
        React.startTransition(() => {
          setState({
            loading: false,
            error: null,
            Content: (compiled as CompiledMdx).default
          });
        });
      })
      .catch((error) => {
        if (!alive) {
          return;
        }
        setState({
          loading: false,
          error: error instanceof Error ? error.message : String(error),
          Content: null
        });
      });

    return () => {
      alive = false;
    };
  }, [source]);

  const components = React.useMemo(() => createCourseComponents(lessonId, index), [index, lessonId]);

  if (state.loading) {
    return <p className="text-sm text-stone-500">Rendering textbook...</p>;
  }

  if (state.error || !state.Content) {
    return (
      <div className="rounded-2xl border border-rose-200 bg-rose-50 p-4 text-sm text-rose-900">
        Failed to render MDX preview: {state.error ?? "unknown error"}
      </div>
    );
  }

  const Content = state.Content;
  return (
    <article className="space-y-6">
      <Content components={components} />
    </article>
  );
}
