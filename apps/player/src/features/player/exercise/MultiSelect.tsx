import type { ExerciseRendererProps } from "./exercise-registry";

export function MultiSelect({
  exercise,
  answered,
  selectedAnswers,
  answerIndices,
  answerCorrect,
  onToggleChoice,
  onSubmitTextAnswer
}: ExerciseRendererProps) {
  const selected = new Set(selectedAnswers);
  const correct = new Set(answerIndices);

  return (
    <>
      <div className="exercise-options">
        {(exercise.options ?? []).map((option, index) => {
          let stateClass = "";
          if (answered && correct.has(index)) {
            stateClass = "correct";
          } else if (answered && selected.has(index) && !correct.has(index)) {
            stateClass = "wrong";
          } else if (!answered && selected.has(index)) {
            stateClass = "selected";
          }

          return (
            <button
              key={option}
              className={`exercise-option ${stateClass}`.trim()}
              disabled={answered}
              onClick={() => onToggleChoice(index)}
              type="button"
            >
              {option}
            </button>
          );
        })}
      </div>
      <div className="exercise-actions">
        <button className="pill-button primary" disabled={answered} onClick={onSubmitTextAnswer} type="button">
          Submit Selections
        </button>
        <span className="exercise-action-hint">Select one or more options, then submit.</span>
      </div>
      {answered ? (
        <div className={`exercise-answer-status ${answerCorrect === false ? "wrong" : "correct"}`}>
          <strong>{answerCorrect ? "Selections accepted." : "Selections submitted."}</strong>
        </div>
      ) : null}
    </>
  );
}
