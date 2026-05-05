import React from "react";
import type { ChapterRecord, QuestionIndex } from "../player-model";
import type { TextbookState } from "../player-hooks";
import { useCompiledMdx } from "../player-hooks";
import "./TextbookView.css";

function flattenText(value: React.ReactNode): string {
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
    const element = value as React.ReactElement<{ children?: React.ReactNode }>;
    return flattenText(element.props.children);
  }
  return "";
}

export function TextbookView({
  chapter,
  textbook,
  practice
}: {
  chapter: ChapterRecord;
  textbook: TextbookState;
  practice: QuestionIndex | null;
}) {
  const compiled = useCompiledMdx(textbook.data);

  const mdxComponents = React.useMemo<Record<string, React.ComponentType<any>>>(() => {
    const QuestionRef = ({ id }: { id: string }) => {
      const entry = practice?.questions.get(id);
      const prompt = entry?.variant.front ?? entry?.variant.stem ?? entry?.family.learning_goal ?? "Practice prompt";
      const answer = entry?.variant.back ?? entry?.variant.reference_answer?.join(" ") ?? entry?.variant.explanation;
      return (
        <aside className="inline-practice">
          <h4>Quick Practice</h4>
          <p>{prompt}</p>
          {answer ? <p className="muted">{answer}</p> : null}
        </aside>
      );
    };

    const QuestionFamily = ({ familyId }: { familyId: string }) => {
      const family = practice?.families.get(familyId);
      return (
        <article className="inline-practice">
          <h4>{family?.learning_goal ?? familyId}</h4>
          <p>{family?.retrieval_focus ?? family?.variants?.[0]?.front ?? "Practice family"}</p>
        </article>
      );
    };

    return {
      KeyPoint: ({ children }: { children: React.ReactNode }) => (
        <aside className="inline-practice">
          <h4>Key Point</h4>
          <p>{flattenText(children)}</p>
        </aside>
      ),
      Definition: ({ title, children }: { title: string; children: React.ReactNode }) => (
        <section className="inline-practice">
          <h4>{title}</h4>
          <p>{flattenText(children)}</p>
        </section>
      ),
      Example: ({ title, children }: { title: string; children: React.ReactNode }) => (
        <section className="inline-practice">
          <h4>{title}</h4>
          <div>{children}</div>
        </section>
      ),
      QuestionRef,
      QuestionFamily
    };
  }, [practice]);

  return (
    <section className="textbook-shell">
      <div className="section-heading">
        <div>
          <h2>{chapter.title}</h2>
          <p>Textbook mode keeps the content readable while embedding the generated practice back into the reading flow.</p>
        </div>
      </div>

      {textbook.loading || compiled.loading ? <div className="loading-state">Preparing the reading room...</div> : null}
      {textbook.error ? <div className="empty-state">{textbook.error}</div> : null}
      {compiled.error ? <div className="empty-state">{compiled.error}</div> : null}
      {compiled.data ? <div className="mdx-shell">{React.createElement(compiled.data, { components: mdxComponents })}</div> : null}
    </section>
  );
}
