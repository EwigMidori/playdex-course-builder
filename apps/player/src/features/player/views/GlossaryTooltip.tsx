import type { ReactNode } from "react";
import type { TermEntry } from "../player-model";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "../ui/tooltip";
import "./GlossaryTooltip.css";

export function GlossaryTooltip({
  termId,
  term,
  children
}: {
  termId: string;
  term: TermEntry;
  children: ReactNode;
}) {
  const title = term.display ?? termId;
  const gloss = term.gloss ?? "No glossary note available yet.";

  return (
    <TooltipProvider delayDuration={160}>
      <Tooltip>
        <TooltipTrigger asChild>{children}</TooltipTrigger>
        <TooltipContent align="center" side="top">
          <div className="glossary-tooltip">
            <div className="glossary-tooltip-head">
              <span className="glossary-tooltip-label">Glossary Entry</span>
              <span className="glossary-tooltip-title">{title}</span>
            </div>
            <p className="glossary-tooltip-copy">{gloss}</p>
            {term.aliases?.length ? (
              <div className="glossary-tooltip-aliases">
                {term.aliases.slice(0, 3).map((alias) => (
                  <span key={alias} className="glossary-tooltip-alias">
                    {alias}
                  </span>
                ))}
              </div>
            ) : null}
          </div>
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>
  );
}
