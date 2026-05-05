import type { ExerciseRendererProps } from "./exercise-registry";

export function FillInBlank({
  answered,
  textAnswer,
  textSubmitted,
  textAnswerCorrect,
  acceptedAnswers,
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
        <input
          className="exercise-text-input"
          disabled={answered}
          onChange={(event) => onTextAnswerChange(event.target.value)}
          placeholder="Type your answer here"
          value={textAnswer}
        />
        <div className="exercise-actions">
          <button className="pill-button primary" disabled={answered} type="submit">
            Submit Answer
          </button>
          <span className="exercise-action-hint">Your answer is checked after submission.</span>
        </div>
      </form>
      {answered ? (
        <div className={`exercise-answer-status ${textAnswerCorrect === false ? "wrong" : "correct"}`}>
          <strong>{textAnswerCorrect ? "Answer accepted." : "Answer submitted."}</strong>
          <p>Your answer: {textSubmitted ?? textAnswer}</p>
          {acceptedAnswers.length ? <p>Reference answer: {acceptedAnswers.join(" / ")}</p> : null}
        </div>
      ) : null}
    </>
  );
}
