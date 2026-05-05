import { AlertTriangle } from "lucide-react";
import type { TermEntry } from "../player-model";
import "./TermDrawer.css";

export function TermDrawer({
  selectedTerm,
  onClose
}: {
  selectedTerm: { id: string; term: TermEntry };
  onClose: () => void;
}) {
  return (
    <div className="term-drawer-scrim" onClick={onClose}>
      <aside className="term-panel" onClick={(event) => event.stopPropagation()}>
        <div className="term-drawer-head">
          <h3>{selectedTerm.term.display ?? selectedTerm.id}</h3>
          <button className="pill-button ghost" onClick={onClose}>
            <span className="button-label">
              <AlertTriangle size={15} />
              <span>Close</span>
            </span>
          </button>
        </div>
        <div className="term-panel-copy">
          <span className="pill accent">Glossary Entry</span>
          <p>{selectedTerm.term.gloss ?? "No glossary note available yet."}</p>
          {selectedTerm.term.aliases?.length ? (
            <>
              <strong>Aliases</strong>
              <div className="term-chip-row">
                {selectedTerm.term.aliases.map((alias) => (
                  <span key={alias} className="pill">{alias}</span>
                ))}
              </div>
            </>
          ) : null}
        </div>
      </aside>
    </div>
  );
}
