# Player Runtime Architecture

这份文档定义 `apps/player` 的下一阶段演进方向，前提是：

- 不改当前落盘资产 contract
- 不回退到 lesson-level nested schema
- 不要求重写现有生成流水线

目标不是重做数据层，而是在播放器内部加一层稳定 runtime，让 UI、导航、练习、进度、历史记录各司其职。

相关现状代码：

- [apps/player/src/features/player/App.tsx](/Users/zijingzhang/ewig/playdex-course-builder/apps/player/src/features/player/App.tsx:72)
- [apps/player/src/features/player/views/StoryOverlay.tsx](/Users/zijingzhang/ewig/playdex-course-builder/apps/player/src/features/player/views/StoryOverlay.tsx:13)
- [apps/player/src/features/player/player-hooks.tsx](/Users/zijingzhang/ewig/playdex-course-builder/apps/player/src/features/player/player-hooks.tsx:21)
- [apps/player/src/features/player/player-state.ts](/Users/zijingzhang/ewig/playdex-course-builder/apps/player/src/features/player/player-state.ts:1)
- [apps/player/src/app-shell-state.ts](/Users/zijingzhang/ewig/playdex-course-builder/apps/player/src/app-shell-state.ts:1)
- [docs/course-player-spec.md](/Users/zijingzhang/ewig/playdex-course-builder/docs/course-player-spec.md:21)

## 1. Design Constraints

必须保持不变的外部 contract：

- `manifest.json` 仍然是 step 列表
- `step.json` 仍然是 guided story 内容源
- `question_bank.json` 仍然是 step-local 题库
- `textbook.mdx` 仍然是 lesson-level 补充内容

播放器内部允许变化的部分：

- raw asset 到 view model 的适配层
- runtime 状态结构
- 回退、历史记录、存档模型
- exercise gating 机制
- renderer 注册方式

一句话约束：

> 外部 schema 稳定，内部 runtime 可重构。

## 2. Current Problems

当前实现能跑通基础路径，但核心逻辑仍然偏“页面组件驱动”，不是“叙事 runtime 驱动”。

主要问题：

- `StoryOverlay` 同时负责 screen 游标、答题状态、点击推进、回退、术语弹层、进度提示
- `App.tsx` 同时负责 course/chapter/step 导航、continue target、step 完成推进
- `player-state.ts` 只记录 `completedSteps` 和 `lastStepByChapter`，没有 runtime 级 history/snapshot
- 当前前进/回退主要依赖本地 `screenIndex`，无法承载 rollback、save/load、branch、read-state
- UI 组件直接消费 raw `step.json` 细节，schema 变化会向上扩散

这些问题不需要改数据流水线，但需要在播放器内建立更清晰的职责边界。

## 3. Architectural Principle

建议把播放器拆成 4 层：

1. `Asset Layer`
   只负责加载 `manifest / step / question_bank / textbook`
2. `Adapter Layer`
   只负责把 raw JSON 变成稳定的内部模型
3. `Runtime Layer`
   只负责播放状态迁移、回退、分支、gating、snapshot
4. `Renderer Layer`
   只负责把当前 runtime 节点渲染成 UI

这 4 层之间的依赖必须单向：

`raw asset -> adapter -> runtime -> renderer`

不要反过来让 renderer 直接定义 runtime 规则。

## 4. Terminology Mapping

为了避免大改数据结构，建议只在“内部语义”上统一命名。

外部资产继续使用：

- `step`
- `screen`

内部 runtime 使用：

- `scene`
- `node`

映射规则：

- `manifest.steps[n]` -> `SceneDescriptor`
- `step.json` -> `SceneModel`
- `step.screens[n]` -> `StoryNode`
- `screen.exercise` -> `exercise` 类型的 `StoryNode`

这样保留了资产兼容性，同时避免在 runtime 层继续混用 `step / scene / screen`。

## 5. Target Module Boundaries

### 5.1 Asset Loader

建议保留现有 `player-hooks.tsx` 的加载职责，但不要让 UI 长期直接消费 raw shape。

职责：

- 加载 `CourseIndex`
- 加载 `StoryManifest`
- 加载 `StoryStep`
- 加载 `QuestionBank`
- 报告精确路径错误

不负责：

- 前进/后退
- 练习放行
- scene completion

### 5.2 Asset Adapter

建议新增一个 adapter 模块，例如：

- `apps/player/src/features/player/runtime/story-adapter.ts`

职责：

- 把 raw `StoryStep` 转成 `SceneModel`
- 规范化 `screen.type`
- 给缺失 `id` 的 screen 补稳定 node id
- 抽取术语引用
- 将 `screen.exercise` 规范化成统一 exercise node

示意类型：

```ts
export type SceneDescriptor = {
  stepId: string;
  concept: string;
  chapterId: string;
};

export type StoryNode =
  | {
      id: string;
      kind: "narration";
      lines: string[];
      termRefs: string[];
      rawScreenIndex: number;
    }
  | {
      id: string;
      kind: "exercise";
      lines: string[];
      exercise: NormalizedExercise;
      termRefs: string[];
      rawScreenIndex: number;
    };

export type SceneModel = {
  sceneId: string;
  stepId: string;
  concept: string;
  lessonId: string | null;
  termCatalog: Record<string, TermEntry>;
  nodes: StoryNode[];
};
```

适配层是 OCP 的第一道边界。未来 raw schema 有轻微漂移，只改 adapter，不改 runtime 和 renderer。

### 5.3 Story Runtime

建议新增一个纯状态模块，例如：

- `apps/player/src/features/player/runtime/story-runtime.ts`

职责：

- 保存当前 cursor
- 定义 `next / back / jump / complete`
- 处理 checkpoint gating
- 维护可回退 history
- 生成可持久化 snapshot

不负责：

- toast
- DOM 事件
- localStorage 细节
- React 样式

示意类型：

```ts
export type StoryCursor = {
  sceneId: string;
  nodeIndex: number;
};

export type NodeVisit = {
  cursor: StoryCursor;
  enteredAt: number;
};

export type RuntimeExerciseState = {
  answered: boolean;
  selectedAnswer: number | null;
  textAnswer: string;
  textSubmitted: string | null;
  textAnswerCorrect: boolean | null;
};

export type StoryRuntimeState = {
  cursor: StoryCursor;
  history: NodeVisit[];
  exerciseStateByNode: Record<string, RuntimeExerciseState>;
  readNodeIds: Record<string, true>;
  completedSceneIds: Record<string, true>;
};
```

核心 reducer 接口建议保持纯函数：

```ts
export type RuntimeAction =
  | { type: "enter-scene"; scene: SceneModel }
  | { type: "advance" }
  | { type: "back" }
  | { type: "submit-exercise"; nodeId: string; result: ExerciseSubmissionResult }
  | { type: "jump-to-node"; nodeId: string }
  | { type: "restore"; snapshot: StoryRuntimeSnapshot };

export function reduceStoryRuntime(
  scene: SceneModel,
  state: StoryRuntimeState,
  action: RuntimeAction
): StoryRuntimeState
```

### 5.4 Exercise Engine

现有 exercise registry 已经是一个不错的起点，但它仍然直接嵌在 overlay 使用流里。

建议把职责再收紧：

- `exercise-registry.tsx` 只做题型注册
- 新增 `exercise-engine.ts` 统一做：
  - 获取阻塞原因
  - 提交答案
  - 生成 runtime patch
  - 决定是否允许 `advance`

这样 runtime 不需要知道题型细节，只消费统一结果。

### 5.5 Progress Store

当前 `player-state.ts` 更像 chapter-level progress store，不是 runtime store。

建议保留其现有职责，但不要把它继续扩成“大杂烩”。

它应该只负责：

- `completedSteps`
- `lastStepByChapter`
- `reviewHistory`
- 可选的 `lastSnapshotByStep`

不应该负责：

- 当前 screen 游标的临时状态
- exercise UI 临时输入框值
- 点击推进逻辑

### 5.6 Backlog Service

Galgame 风格体验里，`Back` 和 `Scene Log` 不应只是 `screenIndex - 1` 和 `slice(0, index)`。

建议新增一个 backlog/history 模块，职责是：

- 记录已进入 node 的可展示文本
- 区分“历史查看”与“runtime rollback”
- 支持将来加入：
  - 已读/未读
  - auto/skip
  - 禁止跨 checkpoint rollback

## 6. Runtime Capability Model

在不改资产 schema 的前提下，runtime 先支持这些能力：

### 6.1 Must Have

- scene enter
- next node
- previous node
- exercise gating
- scene completion
- continue from last step
- backlog view

### 6.2 Should Have

- node-level read state
- rollback history
- step-level snapshot restore
- non-destructive resume

### 6.3 Later

- branching
- conditional nodes
- auto play
- skip read nodes
- media/effect nodes
- save slots

先把 capability 层次分清楚，才能避免为了未来分支剧情而现在一次性过度设计。

## 7. SRP Rules

为避免再次把状态堆回单个组件，建议明确以下 SRP 规则。

### 7.1 `App.tsx`

应该负责：

- 顶层 view mode 切换
- active course/chapter/step 选择
- 把加载结果传给子模块

不应该负责：

- screen 游标推进
- exercise 细节
- rollback 规则

### 7.2 `StoryOverlay`

应该负责：

- 展示当前 node
- 触发用户意图，如 next/back/select/submit/open-term

不应该负责：

- 决定 next 是否允许
- 保存 exercise 答题状态的业务规则
- 定义 history 栈结构

### 7.3 Runtime

应该负责：

- 状态迁移
- gating
- history
- completion

不应该负责：

- 视觉文案
- button 布局
- toast 展示

### 7.4 Adapter

应该负责：

- raw schema 兼容
- 规范化 node 结构

不应该负责：

- 用户进度
- 交互副作用

## 8. OCP Rules

为了让播放器能承载更丰富互动，但不反复改核心文件，建议建立两个注册面。

### 8.1 Node Renderer Registry

```ts
export type NodeRendererProps = {
  node: StoryNode;
  runtime: StoryRuntimeView;
  onAction: (action: RuntimeAction) => void;
};

export type NodeRendererRegistry = Record<
  StoryNode["kind"],
  React.ComponentType<NodeRendererProps>
>;
```

扩展方式：

- 新增 node kind
- 新增对应 renderer
- 注册到 registry

而不是继续在 `StoryOverlay` 里追加 `if (current.type === ...)`。

### 8.2 Exercise Handler Registry

```ts
export type ExerciseHandler = {
  getBlockedReason: (exercise: NormalizedExercise, state: RuntimeExerciseState) => string | null;
  submitChoice?: (...) => ExerciseSubmissionResult;
  submitText?: (...) => ExerciseSubmissionResult;
};

export type ExerciseRegistry = Record<string, ExerciseHandler>;
```

扩展新题型时，只加 handler，不改 runtime 主流程。

## 9. Recommended File Layout

建议逐步收敛到：

```text
apps/player/src/features/player/
  runtime/
    story-adapter.ts
    story-runtime.ts
    story-runtime-types.ts
    backlog.ts
    exercise-engine.ts
  exercise/
    exercise-registry.tsx
    ...
  views/
    StoryOverlay.tsx
    node-renderers/
      NarrationNode.tsx
      ExerciseNode.tsx
```

这不是一次性重写目标，而是明确未来文件职责归属。

## 10. Migration Plan

### Phase 1: Adapter First

先新增 `normalizeStoryStep()`，让 `StoryOverlay` 不再直接读 raw `screens`。

目标：

- 不改 UI 外观
- 不改数据资产
- 先建立内部 `SceneModel`

收益：

- raw schema 兼容集中化
- 为 runtime 抽离打基础

### Phase 2: Runtime Extraction

把这些状态从 `StoryOverlay` 移到 runtime reducer：

- `screenIndex`
- `answered`
- `selectedAnswer`
- `textAnswer`
- `textSubmitted`
- `textAnswerCorrect`

目标：

- `StoryOverlay` 变薄
- `next/back/submit` 全部经由 runtime action

### Phase 3: History and Snapshot

新增：

- `history`
- `readNodeIds`
- `snapshot`

目标：

- Back 不再只是本地 index 回退
- 为 resume/save/load 留接口

### Phase 4: Renderer Registration

把当前 `narration` 和 `exercise` 渲染拆成独立 node renderer。

目标：

- 让后续 media/choice/effect 节点可以按注册方式接入

### Phase 5: Progress Integration

只在 runtime 稳定后，把 snapshot 和 chapter-level progress 接起来。

目标：

- `completedSteps` 继续保持 chapter 进度职责
- 可选新增 `lastSnapshotByStep`

## 11. Near-Term Implementation Target

如果只做一轮中等规模重构，建议把目标收敛为：

1. 新增 `story-adapter.ts`
2. 新增 `story-runtime.ts`
3. 让 `StoryOverlay` 改为消费 `SceneModel + StoryRuntimeState`
4. 保留现有 `AppShellState` 与 `playerProgressAtom`
5. 不改任何 research pipeline 输出

这是“足够改善 SRP/OCP，但不触碰生成链”的最小闭环。

## 12. Non-Goals

本轮不建议一起做：

- 改 raw asset schema
- 引入 lesson-global merged story source
- 一次性上 branching authoring format
- 重做所有 UI 风格
- 把所有状态都塞进全局 store

先把 runtime 核心做清楚，再谈更像 Galgame 的表现层。

## 13. Bottom Line

最推荐的方向不是“重写数据结构”，而是：

> 保持现有资产 contract，把播放器从组件驱动页面，升级成 adapter + runtime + renderer 的分层系统。

这样做有三个直接收益：

- 生成流水线不用返工
- SRP 会显著改善
- OCP 会为后续更丰富的交互留出生长空间

如果下一步要开始落代码，优先顺序应是：

1. `story-adapter.ts`
2. `story-runtime.ts`
3. `StoryOverlay` 瘦身
4. history/snapshot
5. renderer registry
