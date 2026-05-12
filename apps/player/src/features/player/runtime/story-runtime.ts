import {
  createRuntimeExerciseState,
  getExerciseBlockedReason,
  getRuntimeExerciseState,
  setExerciseOrder,
  submitExerciseChoice,
  submitExercise,
  toggleExerciseChoice
} from "./exercise-engine";
import type {
  ExerciseNode,
  NodeVisit,
  SceneModel,
  StoryCursor,
  StoryNode,
  StoryRuntimeAction,
  StoryRuntimeEvaluation,
  StoryRuntimeState
} from "./story-runtime-types";

function createCursor(scene: SceneModel, nodeIndex: number): StoryCursor {
  const node = scene.nodes[nodeIndex];
  return {
    sceneId: scene.sceneId,
    nodeIndex,
    nodeId: node?.id ?? `${scene.sceneId}:node:missing`
  };
}

function createNodeVisit(scene: SceneModel, nodeIndex: number): NodeVisit {
  const node = scene.nodes[nodeIndex];
  return {
    cursor: createCursor(scene, nodeIndex),
    kind: node?.kind ?? "narration",
    lines: node?.lines ?? ["No scene content found."],
    enteredAt: Date.now()
  };
}

function patchExerciseState(
  state: StoryRuntimeState,
  nodeId: string,
  nextExerciseState: StoryRuntimeState["exerciseStateByNode"][string]
) {
  return {
    ...state,
    exerciseStateByNode: {
      ...state.exerciseStateByNode,
      [nodeId]: nextExerciseState
    }
  };
}

export function createStoryRuntimeState(scene: SceneModel): StoryRuntimeState {
  const cursor = createCursor(scene, 0);
  return {
    cursor,
    history: [createNodeVisit(scene, 0)],
    exerciseStateByNode: {},
    readNodeIds: {
      [cursor.nodeId]: true
    },
    completedSceneIds: {}
  };
}

export function getCurrentStoryNode(scene: SceneModel, state: StoryRuntimeState): StoryNode | null {
  return scene.nodes[state.cursor.nodeIndex] ?? null;
}

export function getRuntimeBacklog(state: StoryRuntimeState) {
  return state.history.slice(0, -1);
}

export function getRuntimeProgress(scene: SceneModel, state: StoryRuntimeState) {
  return scene.nodes.length === 0 ? 0 : ((state.cursor.nodeIndex + 1) / scene.nodes.length) * 100;
}

function getExerciseNode(scene: SceneModel, nodeId: string): ExerciseNode | null {
  const node = scene.nodes.find((entry) => entry.id === nodeId);
  return node?.kind === "exercise" ? node : null;
}

export function evaluateStoryRuntimeAction(
  scene: SceneModel,
  state: StoryRuntimeState,
  action: StoryRuntimeAction
): StoryRuntimeEvaluation {
  switch (action.type) {
    case "enter-scene":
      return {
        state: createStoryRuntimeState(scene),
        blockedReason: null,
        feedback: null,
        didAdvance: false,
        didCompleteScene: false
      };
    case "back": {
      if (state.history.length <= 1) {
        return {
          state,
          blockedReason: null,
          feedback: null,
          didAdvance: false,
          didCompleteScene: false
        };
      }

      const nextHistory = state.history.slice(0, -1);
      const previousVisit = nextHistory[nextHistory.length - 1];
      return {
        state: {
          ...state,
          cursor: previousVisit.cursor,
          history: nextHistory
        },
        blockedReason: null,
        feedback: null,
        didAdvance: false,
        didCompleteScene: false
      };
    }
    case "advance": {
      const currentNode = getCurrentStoryNode(scene, state);
      if (!currentNode) {
        return {
          state,
          blockedReason: "No scene content loaded",
          feedback: null,
          didAdvance: false,
          didCompleteScene: false
        };
      }

      if (currentNode.kind === "exercise") {
        const blockedReason = getExerciseBlockedReason(
          currentNode,
          getRuntimeExerciseState(state.exerciseStateByNode, currentNode.id)
        );
        if (blockedReason) {
          return {
            state,
            blockedReason,
            feedback: null,
            didAdvance: false,
            didCompleteScene: false
          };
        }
      }

      if (state.cursor.nodeIndex >= scene.nodes.length - 1) {
        return {
          state: {
            ...state,
            completedSceneIds: {
              ...state.completedSceneIds,
              [scene.sceneId]: true
            }
          },
          blockedReason: null,
          feedback: null,
          didAdvance: false,
          didCompleteScene: true
        };
      }

      const nextIndex = state.cursor.nodeIndex + 1;
      const nextVisit = createNodeVisit(scene, nextIndex);
      return {
        state: {
          ...state,
          cursor: nextVisit.cursor,
          history: [...state.history, nextVisit],
          readNodeIds: {
            ...state.readNodeIds,
            [nextVisit.cursor.nodeId]: true
          }
        },
        blockedReason: null,
        feedback: null,
        didAdvance: true,
        didCompleteScene: false
      };
    }
    case "set-text-answer": {
      const currentExerciseState = getRuntimeExerciseState(state.exerciseStateByNode, action.nodeId);
      if (currentExerciseState.answered) {
        return {
          state,
          blockedReason: null,
          feedback: null,
          didAdvance: false,
          didCompleteScene: false
        };
      }

      const nextExerciseState = {
        ...currentExerciseState,
        textAnswer: action.value
      };
      return {
        state: patchExerciseState(state, action.nodeId, nextExerciseState),
        blockedReason: null,
        feedback: null,
        didAdvance: false,
        didCompleteScene: false
      };
    }
    case "select-choice": {
      const node = getExerciseNode(scene, action.nodeId);
      if (!node) {
        return {
          state,
          blockedReason: null,
          feedback: null,
          didAdvance: false,
          didCompleteScene: false
        };
      }

      const currentExerciseState = getRuntimeExerciseState(state.exerciseStateByNode, action.nodeId);
      if (currentExerciseState.answered) {
        return {
          state,
          blockedReason: null,
          feedback: null,
          didAdvance: false,
          didCompleteScene: false
        };
      }

      const result = submitExerciseChoice(scene, node, currentExerciseState, action.choiceIndex);
      return {
        state: patchExerciseState(state, action.nodeId, result.nextState),
        blockedReason: null,
        feedback: result.feedback,
        didAdvance: false,
        didCompleteScene: false
      };
    }
    case "toggle-choice": {
      const node = getExerciseNode(scene, action.nodeId);
      if (!node) {
        return {
          state,
          blockedReason: null,
          feedback: null,
          didAdvance: false,
          didCompleteScene: false
        };
      }

      const currentExerciseState = getRuntimeExerciseState(state.exerciseStateByNode, action.nodeId);
      if (currentExerciseState.answered) {
        return {
          state,
          blockedReason: null,
          feedback: null,
          didAdvance: false,
          didCompleteScene: false
        };
      }

      const result = toggleExerciseChoice(scene, node, currentExerciseState, action.choiceIndex);
      return {
        state: patchExerciseState(state, action.nodeId, result.nextState),
        blockedReason: null,
        feedback: result.feedback,
        didAdvance: false,
        didCompleteScene: false
      };
    }
    case "set-order": {
      const node = getExerciseNode(scene, action.nodeId);
      if (!node) {
        return {
          state,
          blockedReason: null,
          feedback: null,
          didAdvance: false,
          didCompleteScene: false
        };
      }

      const currentExerciseState = getRuntimeExerciseState(state.exerciseStateByNode, action.nodeId);
      if (currentExerciseState.answered) {
        return {
          state,
          blockedReason: null,
          feedback: null,
          didAdvance: false,
          didCompleteScene: false
        };
      }

      const result = setExerciseOrder(scene, node, currentExerciseState, action.order);
      return {
        state: patchExerciseState(state, action.nodeId, result.nextState),
        blockedReason: null,
        feedback: result.feedback,
        didAdvance: false,
        didCompleteScene: false
      };
    }
    case "submit-exercise": {
      const node = getExerciseNode(scene, action.nodeId);
      if (!node) {
        return {
          state,
          blockedReason: null,
          feedback: null,
          didAdvance: false,
          didCompleteScene: false
        };
      }

      const currentExerciseState = getRuntimeExerciseState(state.exerciseStateByNode, action.nodeId);
      if (currentExerciseState.answered) {
        return {
          state,
          blockedReason: null,
          feedback: null,
          didAdvance: false,
          didCompleteScene: false
        };
      }

      const result = submitExercise(scene, node, currentExerciseState);
      return {
        state: patchExerciseState(state, action.nodeId, result.nextState),
        blockedReason: null,
        feedback: result.feedback,
        didAdvance: false,
        didCompleteScene: false
      };
    }
  }
}

export function reduceStoryRuntime(scene: SceneModel, state: StoryRuntimeState, action: StoryRuntimeAction) {
  return evaluateStoryRuntimeAction(scene, state, action).state;
}
