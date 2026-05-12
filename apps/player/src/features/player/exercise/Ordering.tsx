import type { ExerciseRendererProps } from "./exercise-registry";

function move(order: number[], index: number, direction: -1 | 1) {
  const nextIndex = index + direction;
  if (nextIndex < 0 || nextIndex >= order.length) {
    return order;
  }
  const next = [...order];
  [next[index], next[nextIndex]] = [next[nextIndex], next[index]];
  return next;
}

export function Ordering({
  exercise,
  answered,
  orderedChoiceIndices,
  answerCorrect,
  onSetOrder,
  onSubmitTextAnswer
}: ExerciseRendererProps) {
  const currentOrder =
    orderedChoiceIndices.length === exercise.items.length
      ? orderedChoiceIndices
      : exercise.items.map((_, index) => index);

  return (
    <>
      <div className="exercise-ordering-list">
        {currentOrder.map((itemIndex, index) => (
          <div key={`${exercise.items[itemIndex]}-${index}`} className="exercise-ordering-item">
            <span className="exercise-ordering-rank">{index + 1}.</span>
            <span className="exercise-ordering-label">{exercise.items[itemIndex]}</span>
            <div className="exercise-ordering-actions">
              <button
                className="pill-button ghost"
                disabled={answered || index === 0}
                onClick={() => onSetOrder(move(currentOrder, index, -1))}
                type="button"
              >
                Up
              </button>
              <button
                className="pill-button ghost"
                disabled={answered || index === currentOrder.length - 1}
                onClick={() => onSetOrder(move(currentOrder, index, 1))}
                type="button"
              >
                Down
              </button>
            </div>
          </div>
        ))}
      </div>
      <div className="exercise-actions">
        <button className="pill-button primary" disabled={answered} onClick={onSubmitTextAnswer} type="button">
          Submit Order
        </button>
        <span className="exercise-action-hint">Reorder the items, then submit.</span>
      </div>
      {answered ? (
        <div className={`exercise-answer-status ${answerCorrect === false ? "wrong" : "correct"}`}>
          <strong>{answerCorrect ? "Order accepted." : "Order submitted."}</strong>
        </div>
      ) : null}
    </>
  );
}
