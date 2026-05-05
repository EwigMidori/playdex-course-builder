import type { ExerciseRendererProps } from "./exercise-registry";

export function ShortReflection({
  answered,
  textAnswer,
  textSubmitted,
  onTextAnswerChange,
  onSubmitTextAnswer
}: ExerciseRendererProps) {
  return (
    <>
      <form
        className="exercise-text-form"
        onSubmit={(event) => {
          event.preventDefault();
          onSubmitTextAnswer();
        }}
      >
        <textarea
          className="exercise-textarea"
          disabled={answered}
          onChange={(event) => onTextAnswerChange(event.target.value)}
          placeholder="Write your reflection here"
          rows={4}
          value={textAnswer}
        />
        <div className="exercise-actions">
          <button className="pill-button primary" disabled={answered} type="submit">
            Submit Reflection
          </button>
          <span className="exercise-action-hint">A non-empty response unlocks Continue.</span>
        </div>
      </form>
      {answered ? (
        <div className="exercise-answer-status correct">
          <strong>Reflection submitted.</strong>
          <p>{textSubmitted}</p>
        </div>
      ) : null}
    </>
  );
}
