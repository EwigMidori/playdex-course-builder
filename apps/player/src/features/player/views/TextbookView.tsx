import React from "react";
import type { ChapterRecord, QuestionIndex } from "../player-model";
import type { TextbookState } from "../player-hooks";
import { useCompiledMdx } from "../player-hooks";
import { buildTextbookMdxComponents } from "./textbook-mdx";
import "./TextbookView.css";

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
    return buildTextbookMdxComponents({ chapter, practice });
  }, [chapter, practice]);

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
