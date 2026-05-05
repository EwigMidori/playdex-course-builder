import { UnsupportedExercise } from "./UnsupportedExercise";
import { resolveExercise, type ExerciseRendererProps } from "./exercise-registry";
import "./ExerciseRenderer.css";

export function ExerciseRenderer({
  unsupportedReason,
  exerciseContext,
  ...props
}: ExerciseRendererProps & {
  unsupportedReason: string | null;
  exerciseContext: string;
}) {
  const { Component } = resolveExercise(props.exercise);

  if (!Component || unsupportedReason) {
    return <UnsupportedExercise exerciseContext={exerciseContext} unsupportedReason={unsupportedReason} />;
  }

  return <Component {...props} />;
}
