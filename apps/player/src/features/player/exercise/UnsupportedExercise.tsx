export function UnsupportedExercise({
  unsupportedReason,
  exerciseContext
}: {
  unsupportedReason: string | null;
  exerciseContext: string;
}) {
  return (
    <div className="exercise-unsupported">
      <strong>Checkpoint cannot be answered in the player right now.</strong>
      <p>{unsupportedReason ?? "Unsupported checkpoint payload"}.</p>
      <p>{exerciseContext}</p>
    </div>
  );
}
