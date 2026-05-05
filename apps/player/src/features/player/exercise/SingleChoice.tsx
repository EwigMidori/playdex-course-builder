import type { ExerciseRendererProps } from "./exercise-registry";

export function SingleChoice({
  exercise,
  answered,
  selectedAnswer,
  answerIndex,
  onSelectAnswer
}: ExerciseRendererProps) {
  return (
    <div className="exercise-options">
      {(exercise.options ?? []).map((option, index) => {
        let stateClass = "";
        if (answered && index === answerIndex) {
          stateClass = "correct";
        } else if (answered && index === selectedAnswer && index !== answerIndex) {
          stateClass = "wrong";
        }
        return (
          <button
            key={option}
            className={`exercise-option ${stateClass}`.trim()}
            disabled={answered}
            onClick={() => onSelectAnswer(index)}
          >
            {option}
          </button>
        );
      })}
    </div>
  );
}
