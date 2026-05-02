# Multi-Course Learning System Spec

这份 spec 面向一个“多课程学习平台”的学习系统设计，而不是单个课程播放器。

目标不是把现有生成资产摆进一个管理面板，而是把课程资产包装成一个愿意让学生持续回来学习的系统。设计重点参考 [prototype.tsx](/Users/zijingzhang/ewig/playdex-course-builder/docs/prototype.tsx:24)，尤其是它如何把 `guided_story`、`flashcard`、`quiz`、`glossary` 这些枯燥数据，组织成一个有节奏、有情绪、有返回动力的学习过程。

## 0. Reading Guide

这份文档同时服务产品、设计、前端和另一个 AI，因此必须先明确阅读方式。

如果你只看一部分，优先顺序如下：

1. `4. Experience Principles`
2. `5. Information Architecture`
3. `6. Core Learning Loops`
4. `8. Content Model`
5. `9. Session Types`
6. `15. Technical Architecture Direction`
7. `17. Success Criteria`

其中只有下面几部分是 canonical contract，其他章节不得重新定义它们：

- `8.3 Platform Source Of Truth`
- `8.4 Canonical Identity Rules`
- `8.5 Loader Contract`
- `8.8 Canonical Package Contract`
- `9.7 Session State Contract`
- `15.4 Event Contract`

如果某章节与上述 contract 冲突，以 canonical contract 为准。

## 1. Product Thesis

这个平台的核心命题：

- 学习产品的主界面不应像后台。
- 学习资产不应先以“文件、表格、标签、筛选器”被看见。
- 学生进入系统后，首先感知到的应该是：
  - 我现在在学什么
  - 我为什么继续学
  - 我下一步做什么
  - 我刚刚变强了什么

一句话：

> 这是一个“可玩的学习系统”，不是“课程资产管理器”。

## 2. Prototype Lessons

原型有几个非常关键的方向，是这个系统必须继承的。

### 2.1 用“游戏仓库 -> 游戏 -> 路线 -> 幕”的壳，替代“课程 -> lesson -> 文件”

原型把层级重新命名为：

- Library
- Game / Course
- Chapter / Route
- Step / Scene

见 [prototype.tsx](/Users/zijingzhang/ewig/playdex-course-builder/docs/prototype.tsx:24) 与 [prototype.tsx](/Users/zijingzhang/ewig/playdex-course-builder/docs/prototype.tsx:415)。

这不是简单换皮，而是一个重要认知转移：

- 学生看到的不是“我要管理多少资料”
- 而是“我在推进哪条路线”

这能把学习对象从静态资产变成动态进程。

### 2.2 主学习流必须是沉浸式剧情推进，而不是阅读器

原型把 guided story 做成了 galgame 式 overlay：

- 全屏进入学习态
- 单屏小步推进
- 大字体对话框
- 明确的 `Click to Continue`
- 练习 screen 与 narration screen 共用同一叙事壳

见 [prototype.tsx](/Users/zijingzhang/ewig/playdex-course-builder/docs/prototype.tsx:80) 到 [prototype.tsx](/Users/zijingzhang/ewig/playdex-course-builder/docs/prototype.tsx:239)。

这背后的产品原则是：

- 学习不是“翻文档”
- 学习是“推进一段体验”

### 2.3 术语解释不是离开学习流去查资料，而是侧滑式维基

原型里 `<term>` 点击后，会打开 glossary side panel，而不是跳出当前场景。

见 [prototype.tsx](/Users/zijingzhang/ewig/playdex-course-builder/docs/prototype.tsx:97) 到 [prototype.tsx](/Users/zijingzhang/ewig/playdex-course-builder/docs/prototype.tsx:120)，以及 [prototype.tsx](/Users/zijingzhang/ewig/playdex-course-builder/docs/prototype.tsx:555)。

这说明：

- 概念解释必须是学习流的一部分
- 不是外链，不是新页面，不是资料库打断

### 2.4 复习系统必须被包装成一个 session，而不是待办表

原型没有把 FSRS 做成一堆数字，而是做成：

- `RECALL SESSION`
- 一张一张翻卡
- 进度轨道
- `Again / Hard / Good / Easy`

见 [prototype.tsx](/Users/zijingzhang/ewig/playdex-course-builder/docs/prototype.tsx:247) 到 [prototype.tsx](/Users/zijingzhang/ewig/playdex-course-builder/docs/prototype.tsx:315)。

这非常重要：

- 学生不需要先理解算法
- 学生只需要理解“我现在在完成一轮记忆巩固”

### 2.5 题库不应先以数据库对象出现，而应先以收藏馆 / 图鉴出现

原型的 `The Vault` 不是后台列表，而是题型分区的图鉴入口：

- Flashcards
- Quizzes
- Longform

见 [prototype.tsx](/Users/zijingzhang/ewig/playdex-course-builder/docs/prototype.tsx:521)。

这意味着：

- 题库是可探索的练习世界
- 不是一个平铺 JSON family 列表

### 2.6 多课程平台的入口应该是“我在玩哪些游戏”，不是“所有课程表格”

原型首页是 `Pick up your game`，并且侧栏里有 `My Games` 快捷跳转。

见 [prototype.tsx](/Users/zijingzhang/ewig/playdex-course-builder/docs/prototype.tsx:379) 到 [prototype.tsx](/Users/zijingzhang/ewig/playdex-course-builder/docs/prototype.tsx:423)。

这给多课程平台一个非常重要的壳：

- 用户不是在“管理课程”
- 用户是在“维持多个正在进行的学习世界”

## 3. Design Goals

系统必须满足以下目标。

### 3.1 Primary Goals

- 让学生愿意开始一节课
- 让学生知道下一步该做什么
- 让复习成为一个自然回路，而不是额外负担
- 让多课程并行学习仍然有秩序
- 让现有生成资产可以直接映射到前端，而不需要人工重写

### 3.2 Non-Goals

- 不做课程后台 CMS
- 不做教师管理系统
- 不做以表格、筛选、批量操作为中心的内容运营工具
- 不把学习主体验建立在 KPI 仪表板上

### 3.3 PDCA Audit Summary

用 PDCA 审核这份初稿，原始问题主要有四类：

- `Plan` 不足：
  - 有方向，但缺少优先级明确的 MVP 边界
  - 有体验原则，但缺少可验证假设
  - 有系统形态，但缺少“先做什么、后做什么”
- `Do` 不足：
  - 定义了页面和 session，但缺少跨模块编排规则
  - 定义了学习壳，但缺少平台状态与恢复策略的最小落地要求
- `Check` 不足：
  - 成功标准偏定性，缺少行为指标、埋点事件、验收门槛
  - 缺少“像后台”与“像学习系统”的判定方法
- `Act` 不足：
  - 缺少固定复盘节奏
  - 缺少问题分级与修订路径

因此，这一轮修订会补四件事：

- 明确产品假设与 MVP 交付顺序
- 明确 session orchestration 与 resume 规则
- 明确验收指标、埋点与质量门槛
- 明确复盘与纠偏机制

## 4. Experience Principles

### 4.1 Play Before Manage

默认入口是“开始一轮学习”，不是“查看资产详情”。

### 4.2 One Strong Next Action

每个页面都应该有清晰主动作，例如：

- 继续这条路线
- 完成今日复习
- 进入本章剧情
- 进行题库突击

而不是同时摆出一堆同权入口。

### 4.3 Context Over Catalog

学生应该先在上下文里遇到知识，再去查看术语、例子、练习。

### 4.4 Sessions, Not Utilities

复习、测验、阅读都应该被包装成 session，而不是裸工具。

### 4.5 Progress Must Feel Earned

进度条、 mastery、待复习数都应该紧贴学习行为出现，而不是悬浮成纯运营指标。

### 4.6 Serious Subject, Playful Container

平台可以有游戏化壳，但不能幼稚化内容。

- 金融、编程、数学等严肃主题依然要保持可信感
- “有趣”来自节奏、反馈、形式
- 不是来自花哨积分和肤浅奖励

### 4.7 System Decision Rule

任何功能如果同时满足下面两条，就优先进入 MVP：

- 它能缩短“从打开平台到进入学习态”的路径
- 它能提升“学完一次后下次还愿意回来”的概率

任何功能如果主要服务于运营可视化、资产遍历、复杂配置或批量管理，则默认不进入 learner-facing MVP。

## 4.8 Product Hypotheses

这不是纯设计稿，必须被当成一组待验证假设。

核心假设：

- H1：把课程包装成 `game / route / scene`，会提高用户开始学习的意愿
- H2：把 guided story 做成沉浸式 session，而不是阅读器，会提高 step 完成率
- H3：把 glossary 做成侧滑 panel，而不是跳页，会降低概念澄清中断成本
- H4：把 FSRS 包装成 recall session，而不是待办列表，会提高复习完成率
- H5：把题库包装成 Vault，而不是裸 family 列表，会提高题库探索率

后文所有指标与验收，都应围绕这五条假设组织。

## 5. Information Architecture

平台采用五层学习架构。

### 5.1 Layer 1: Library

面向用户的问题：

`我现在在学哪些课程世界？`

表现形式：

- 课程卡片墙
- 每张卡带课程 identity
- 显示整体 mastery
- 显示待复习数
- 显示最近学习状态

对应原型：

- `Pick up your game`
- `My Games`

### 5.2 Layer 2: Course Hub

面向用户的问题：

`这门课我有哪些路线可推进？`

表现形式：

- 大课程封面
- 课程总览
- 默认只突出一个 `继续本课程路线`
- chapter / lesson 入口网格
- 当前课程的 FSRS 复习入口
- 当前课程的 Vault 入口

对应原型：

- `game_home`

### 5.3 Layer 3: Chapter Hub

面向用户的问题：

`这一章我应该怎么学？`

表现形式：

- 章节标题
- 主按钮：开始剧情
- 次按钮：阅读课本
- scene / step 列表
- 每个 step 的状态与类型

对应原型：

- `chapter_home`

### 5.4 Layer 4: Learning Session

面向用户的问题：

`我现在只需要完成这一段学习。`

表现形式：

- 沉浸式 overlay
- 单屏叙事
- 内容与练习混排
- progress micro-bar
- back / save / log
- term click -> glossary side panel

对应原型：

- `GalgamePlayer`

### 5.5 Layer 5: Reinforcement System

面向用户的问题：

`学完之后，怎样记住并用出来？`

包含三条并行 reinforcement 流：

- FSRS Review Center
- Vault / Practice Atlas
- Textbook / Review Book

## 5.6 Layer CTA Contract

为了避免每一层都退化成功能导航，平台必须在每个层级只提供一个 strongest CTA。

### Library

- strongest CTA：
  - `继续学习` 或 `开始今日学习`
- secondary：
  - 课程卡片墙
  - 今日复习入口

### Course Hub

- strongest CTA：
  - `继续本课程路线`
- secondary：
  - chapter 浏览
  - 本课程 recall
  - Vault

### Chapter Hub

- strongest CTA：
  - `继续这一章`
- secondary：
  - Textbook
  - step list
  - 章节状态

### Story Completion

- strongest CTA：
  - `下一幕`
  - 若当前 step 已结束，则 `进入本 step 练习`
- secondary：
  - 返回章节

默认规则：

- strongest CTA 必须视觉上唯一
- secondary action 可以存在，但不能和 strongest CTA 同权
- 默认不在首屏并排展示三个以上同级行动入口
- Course Hub 默认优先展示“继续本课程路线”，FSRS 与 Vault 进入次级区域
- Chapter Hub 默认优先展示“继续这一章”，step list 可默认折叠或弱化展示
- 只有当用户明确需要选择路径时，才展开完整 step list

## 5.7 CTA Resolution Contract

`strongest CTA` 不能只是文案，它必须是 deterministic resolution 结果。

### Library Resolution

输入：

- `lastResumableSession`
- global due count
- active course recency
- unlocked chapters

输出优先级：

1. 恢复最近的 resumable story session
2. 若 due 超过阈值，进入 global review
3. 推荐最近活跃课程中的未完成 chapter
4. 推荐一个新解锁 chapter

fallback：

- 若无课程可学，显示空状态而不是功能入口墙

### Course Hub Resolution

输入：

- 当前课程内是否有 resumable story
- 当前课程 due count
- 已开始但未完成的 chapter
- 新解锁 chapter

输出优先级：

1. 恢复当前课程内的 resumable story
2. 返回最近未完成 chapter 的最近 step
3. 进入当前课程最优先的新解锁 chapter
4. 若本课程只适合复习，则进入 course-local review

同课程内 tie-breaker：

1. `paused session`
2. `currentChapterId`
3. `lastVisitedAt` 最近的未完成 chapter
4. `order` 最小的未完成 chapter
5. `order` 最小的新解锁 chapter

fallback：

- 若本课程内容不完整且不可学，则 strongest CTA 不显示开始学习，而显示不可用说明

### Chapter Hub Resolution

输入：

- 当前 chapter 是否有 resumable story
- 最近未完成 step
- chapter 是否已完成
- 是否存在 step-local practice

输出优先级：

1. 恢复当前 chapter 内的 resumable story 到 `paused screen`
2. 若无 paused story，进入最近未完成 step 的开头
3. 若 chapter 已完成且有 step-local practice，进入推荐 practice
4. 否则从 chapter 第一可学 step 开始

fallback：

- 若 chapter `blocked`，不显示 `继续这一章`
- 若 chapter `degraded`，只显示可用入口

### Story Completion Resolution

输入：

- 是否还有下一 step
- 当前 step 是否有关联 practice
- 当前 chapter 是否完成

输出优先级：

1. 若有下一 step，默认 CTA 为 `下一幕`
2. 若当前 step 已结束且 practice 可用，次级 CTA 为 `进入本 step 练习`
3. 若 chapter 已结束，进入 chapter completion landing

### Determinism Rule

- 同一输入状态必须产生同一 strongest CTA
- strongest CTA 的解析必须可埋点、可复盘、可解释
- 文案变化不能改变解析逻辑

## 6. Core Learning Loops

这是系统的核心，不是页面清单。

### 6.1 Route Loop

`选课 -> 选章 -> 进入剧情 -> 完成 step -> 返回章节 -> 继续下一个 step`

这是主线推进循环。

### 6.2 Recall Loop

`收到待复习提示 -> 进入 recall session -> 翻卡 -> 给 rating -> 结束 session -> 回到课程`

这是长期记忆循环。

### 6.3 Clarify Loop

`在剧情中遇到术语 -> 点击 term -> 打开 glossary side panel -> 读 gloss -> 返回剧情`

这是概念澄清循环。

### 6.4 Assessment Loop

`完成剧情 -> 进入 quiz / longform -> 检查理解 -> 返回课程`

这是理解验证循环。

### 6.5 Re-entry Loop

`重新打开平台 -> 看到“继续学习”与“今日复习” -> 直接恢复上次上下文`

这是多课程平台最重要的留存循环。

## 7. Multi-Course Learning System

单课程播放器不够，平台必须处理多课程并行。

### 7.1 Course Identity

每门课程都需要明确 identity：

- 标题
- 学科类别
- 品牌色
- mastery
- 待复习数
- 当前正在推进的 chapter

目的不是装饰，而是让用户能快速切换心智上下文。

### 7.2 Global Resume

平台首页必须有全局恢复能力：

- 继续上次未完成的课程
- 今日需要复习的课程
- 最近推进最快的路线
- 临近完成的章节

### 7.3 Cross-Course Review Queue

FSRS 不能只停留在课程内。

平台应同时支持：

- course-local review
- global review queue

推荐策略：

- 首页突出“今日总复习”
- 课程内保留“本课专属复习”

### 7.4 Parallel Progress Without Clutter

多课程平台最容易失败的地方，是把信息堆成仪表板。

正确做法：

- 首页只显示高信号摘要
- 细节进入课程后再展开
- 不要在首页展示过多统计、标签云、复杂筛选

### 7.5 Resume Priority

多课程平台必须定义恢复优先级，否则首页很容易退化成卡片墙。

推荐优先级：

1. 上次未完成的 story session
2. 今日 due 数最高且用户最近活跃过的课程
3. 即将完成的 chapter
4. 新解锁但尚未开始的 chapter

首页不应同时高亮多个同权 CTA。默认只给一个最强“继续”动作，其他入口放在次级区域。

## 8. Content Model

系统内容模型应分为“学习壳”和“课程资产”两层。

### 8.1 Learning Shell Entities

```ts
type Library = {
  courses: CourseShell[];
};

type CourseShell = {
  courseId: string;
  title: string;
  category?: string;
  brandColor?: string;
  mastery: number;
  itemsToReview: number;
  chapters: ChapterShell[];
};

type ChapterShell = {
  chapterId: string;
  title: string;
  progress: number;
  estimatedMinutes?: number;
  steps: StepShell[];
};

type StepShell = {
  stepId: string;
  concept: string;
  status: "locked" | "available" | "in_progress" | "completed";
  mode: "story" | "checkpoint" | "boss_quiz";
};
```

### 8.2 Course Asset Entities

这一层来自现有生成资产：

- `manifest.json`
- `step.json`
- `question_bank.json`
- `textbook.mdx`

这些原始资产经过 loader 组装后，会变成 canonical `CoursePackage`。

唯一有效定义见 `8.8 Canonical Package Contract`。

最小 raw schema 子集如下：

```ts
type StoryManifest = {
  lesson_id: string;
  mode?: string;
  steps: Array<{
    sequence_id: string;
    concept: string;
    file: string;
  }>;
};

type StoryStep = {
  lesson_id: string;
  sequence_id: string;
  target_language?: string;
  mode?: string;
  term_catalog?: Record<string, { display: string; gloss: string; aliases?: string[] }>;
  screens: Array<{
    id?: string;
    type: "narration" | "exercise" | string;
    lines: string[];
    focus_terms?: string[];
    introduced_terms?: string[];
    exercise?: {
      kind?: string;
      prompt?: string;
      options?: string[];
      answer?: number | string;
      explanation?: string;
    };
  }>;
};

type QuestionBank = {
  lesson_id: string;
  target_language?: string;
  coverage_map?: Array<{
    coverage_tag: string;
    description?: string;
    covered_by?: string[];
  }>;
  flashcard_families?: QuestionFamily[];
  quiz_families?: QuestionFamily[];
  longform_families?: QuestionFamily[];
};

type QuestionFamily = {
  family_id: string;
  question_type: string;
  linked_steps: string[];
  concept_key: string;
  coverage_tags: string[];
  learning_goal?: string;
  difficulty?: string;
  retrieval_focus?: string | null;
  term_refs?: Array<{ display: string; en?: string }>;
  variants: QuestionVariant[];
};

type QuestionVariant = {
  question_id: string;
  front?: string;
  back?: string;
  stem?: string;
  options?: string[];
  answer?: number | string;
  explanation?: string;
  estimated_seconds?: number;
  prompt_blocks?: string[];
  reference_answer?: string[];
  rubric_points?: string[];
};
```

### 8.3 Platform Source Of Truth

平台不能只靠“扫描目录后猜结构”。必须有一层平台级 source of truth。

推荐新增：

- `course-index.json`
- 可选 `course-index.schema.json`

最小 shape：

```ts
type CourseIndex = {
  version: 1;
  courses: CourseIndexEntry[];
};

type CourseIndexEntry = {
  courseId: string;
  title: string;
  category?: string;
  brandColor?: string;
  chapters: ChapterIndexEntry[];
};

type ChapterIndexEntry = {
  chapterId: string;
  title: string;
  order: number;
  lessonId: string;
  guidedStoryDir: string;
  textbookPath?: string;
  unlockRule?: "always" | "after_previous_chapter" | "manual";
};
```

这层 index 的职责：

- 定义课程发现规则
- 定义 chapter 顺序
- 建立 `courseId / chapterId / lessonId` 的映射
- 提供 learner-facing 的稳定 identity
- 避免实现方各自扫描目录并补脑

### 8.4 Canonical Identity Rules

为了防止跨课程冲突，平台必须定义 canonical ID 规则。

```ts
type CanonicalIds = {
  courseId: string;   // 平台级唯一
  chapterId: string;  // 在 course 内唯一
  lessonId: string;   // 指向 research/pipeline 里的 lesson，例如 L2
  stepId: string;     // 例如 step1
  familyId: string;   // 在平台级唯一，或通过 course/chapter 命名空间包装后唯一
  questionId: string; // 在平台级唯一，或通过 course/chapter 命名空间包装后唯一
  termId: string;     // 至少在 lesson 内唯一
};
```

推荐平台内部统一用 namespaced key：

- `courseKey = <courseId>`
- `chapterKey = <courseId>:<chapterId>`
- `stepKey = <courseId>:<chapterId>:<stepId>`
- `familyKey = <courseId>:<chapterId>:<familyId>`
- `questionKey = <courseId>:<chapterId>:<questionId>`
- `termKey = <courseId>:<chapterId>:<stepId>:<termId>`

### 8.5 Loader Contract

loader 不能只是“读文件”，它必须做 deterministic normalization。

```ts
type LoaderResult =
  | { ok: true; coursePackage: CoursePackage; warnings: LoaderWarning[] }
  | { ok: false; error: LoaderError };

type LoaderWarning = {
  code:
    | "missing_textbook"
    | "missing_term_catalog_entry"
    | "unknown_question_ref"
    | "legacy_layout_fallback";
  message: string;
  path?: string;
};

type LoaderError = {
  code:
    | "missing_course_index_entry"
    | "missing_manifest"
    | "invalid_manifest"
    | "missing_step_file"
    | "missing_question_bank"
    | "invalid_step_file"
    | "invalid_question_bank"
    | "non_unique_id";
  message: string;
  path?: string;
};
```

loader 最小责任：

1. 根据 `course-index.json` 发现课程与 chapter
2. 读取 `manifest.json`
3. 按 manifest 顺序读取 `step.json` 与 `question_bank.json`
4. 建立 `questionIndex`、`termIndex`、`reviewIndex`
5. 对缺失的 `textbookPath` 做 graceful fallback
6. 对缺失的 `question_bank.json` 视为 hard error，不允许继续 learner-facing story/practice 入口

### 8.6 Required And Optional Assets

对于每个 chapter：

- required
  - `manifest.json`
  - manifest 中声明的每个 `step.json`
  - 每个 step 对应的 `question_bank.json`
- optional
  - `textbook.mdx`
  - brand assets
  - analytics sidecar

fallback 规则：

- 缺 `textbook.mdx`：
  - chapter 进入 `degraded`
  - 主学习路径仍可用
  - 关闭 Textbook CTA
- 缺 `question_bank.json`：
  - chapter 不应暴露完整 learner-facing 入口
  - 视为 hard error
- manifest 与 step 不一致：
  - 视为 hard error
  - 不进入可学习状态

### 8.8 Canonical Package Contract

```ts
type CoursePackage = {
  courseId: string;
  title: string;
  category?: string;
  brandColor?: string;
  chapters: ChapterPackage[];
  indexes: {
    stepsByKey: Record<string, StepPackage>;
    familiesByKey: Record<string, QuestionFamily>;
    questionsByKey: Record<string, QuestionVariant>;
    termsByKey: Record<string, { display: string; gloss: string; aliases?: string[] }>;
  };
  reviewIndex: ReviewIndex;
};

type ChapterPackage = {
  chapterId: string;
  lessonId: string;
  title: string;
  order: number;
  manifest: StoryManifest;
  steps: StepPackage[];
  textbook?: TextbookPackage;
  status: "ready" | "degraded" | "blocked";
};

type StepPackage = {
  stepKey: string;
  sequenceId: string;
  concept: string;
  story: StoryStep;
  questionBank: QuestionBank;
};

type TextbookPackage = {
  path: string;
  rawMdx: string;
  sectionMap?: unknown[];
  coverageTrace?: unknown[];
};

type ReviewIndex = {
  byCourse: Record<string, string[]>;
  byChapter: Record<string, string[]>;
  dueCardIds: string[];
};
```

### 8.9 Status Resolution Matrix

`ChapterPackage.status` 必须由 loader 决定，前端不得自行猜测。

### Status Semantics

- `ready`
  - 主学习路径完整可用
- `degraded`
  - 主路径可用，但辅助资产缺失
- `blocked`
  - 主路径不可用，不可进入 learner-facing 学习

### Resolution Rules

| Condition | Status | Story CTA | Review/Vault CTA | Textbook CTA | UX Requirement |
|---|---|---|---|---|---|
| manifest + all step + all question_bank 存在；textbook 也存在 | `ready` | allowed | allowed | allowed | 正常进入 |
| 仅缺 `textbook.mdx` | `degraded` | allowed | allowed | hidden | 提示“课本稍后提供” |
| 存在 soft warning，如 `missing_term_catalog_entry`、`unknown_question_ref`，但主路径完整 | `degraded` | allowed | allowed | allowed if exists | 可学习，但带内容质量提示 |
| 缺 `question_bank.json` | `blocked` | hidden | hidden | optional hidden or read-only | 不允许暴露完整学习入口 |
| 缺 `manifest.json` / step 文件无效 / manifest-step 不一致 | `blocked` | hidden | hidden | hidden | 显示内容不可用状态 |
| 非唯一 ID 或 critical schema invalid | `blocked` | hidden | hidden | hidden | 必须阻断 learner-facing 流程 |

### Deterministic Mapping Rule

- warning 不能直接决定 `blocked`
- hard error 不能落成 `ready`
- `degraded` 必须仍保证 strongest CTA 可用
- `blocked` 不得显示误导性的 `继续学习`

### 8.7 Mapping Rule

建议映射关系：

- 一个 `course` 对应一个课程世界
- 一个 `lesson/chapter` 对应一条学习路线
- 一个 `step` 对应一个 scene
- `screens` 对应场景中的 beat
- `term_catalog` 对应术语维基数据源
- `flashcard_families / quiz_families / longform_families` 对应 reinforcement assets

## 9. Session Types

系统至少应有五种 session。

### 9.1 Story Session

目标：

- 吸收新知识
- 在叙事里推进理解

输入：

- `step.json`

关键规则：

- 一次只推进一个 screen
- narration 和 exercise 必须共享一个连续壳
- 用户感知的是“剧情推进”，不是“切换模块”

#### Story Runtime Input

```ts
type StoryRuntimeInput = {
  courseId: string;
  chapterId: string;
  stepKey: string;
  story: StoryStep;
  questionBank: QuestionBank;
};
```

### 9.2 Recall Session

目标：

- 巩固记忆

输入：

- flashcard review queue

关键规则：

- 一次一张卡
- 明确翻面动作
- 明确 rating 动作
- session 完成后要有结束反馈

### 9.3 Quiz Session

目标：

- 检查辨析与应用

输入：

- `quiz_families`

关键规则：

- 不要伪装成 flashcard
- 可用“突击模式”“章节测验”“考试模拟”

### 9.4 Longform Session

目标：

- 检查解释、比较、推导、应用能力

输入：

- `longform_families`

关键规则：

- 要支持参考答案、rubric points、延时作答
- 不应和 quick quiz 混在同一列表里

### 9.5 Textbook Session

目标：

- 预习与复习

输入：

- `textbook.mdx`

关键规则：

- 它是 supporting mode，不是主学习流
- 应能从 textbook 跳转到相关题目或 step

## 9.5B Multi-Device Learning Contract

平台必须默认 mobile-first，同时允许 desktop 增强，而不是反过来。

### Mobile Rules

- story session 默认以单列全屏阅读为准
- glossary 默认使用 bottom sheet 或全屏 sheet，而不是固定侧栏
- narrative log 默认使用上滑面板或独立 sheet
- 单屏正文必须保证单手连续点击可完成推进
- 不依赖 hover 才能发现关键交互
- 每次 story session 的推荐时长默认 2 到 6 分钟
- 关键点击热区不小于常见移动端可触达标准
- 重要 CTA 必须位于下半屏可稳定点击区域
- 不允许 glossary、log、progress 同时挤占正文主区域
- 手机端单屏正文默认目标：
  - 1 到 2 行主句
  - 避免形成需要滚动的正文块

### Desktop Rules

- 可使用 side panel 展开 glossary 或 narrative log
- 可允许 split view，但 split 只能增强，不应替代单主线阅读
- 不允许为了利用大屏而把 step 内容重新摊平成多列阅读器
- 桌面端允许 hover 提示，但 hover 不能是唯一发现机制
- 桌面端允许键盘快捷推进，但必须保留明确点击 CTA

### Cross-Device Resume Rules

- 从手机切到桌面，至少恢复：
  - `session`
  - `screen`
  - `textbook anchor`
  - `quiz/longform draft`
- 从桌面切回手机时，桌面的并行辅助面板状态可以丢失
- 但主学习位置不能丢失

### Input Rules

- 手机优先触控
- 平板兼容触控与键盘
- 桌面允许键盘快捷操作
- 任一关键动作都必须有可见点击入口，不能只靠快捷键

### Responsive Rules

- 文本密度应随屏宽调整，但节奏不变
- 大屏只能增加呼吸感与辅助面板，不能增加正文并排密度
- 小屏优先保留：
  - 主文本
  - strongest CTA
  - 当前进度
- 其余辅助信息应延后或折叠

### Per-Mode Mobile Behavior

#### Story

- 使用全屏或准全屏单列壳
- glossary / log 采用 sheet
- 恢复粒度：
  - `stepId + screenIndex + opened glossary context`
- 退出 landing：
  - 回 chapter hub 或可恢复 story

#### Review

- 一次只展示 1 张卡
- rating 区必须固定在拇指易触达区域
- 恢复粒度：
  - `queueSnapshotId + currentCardId + currentCardIndex`
- 退出 landing：
  - 回课程 strongest CTA 或全局 strongest CTA

#### Vault

- 手机端默认分区列表，不默认三列卡片墙
- 优先展示按学习意图分区，而不是复杂筛选
- 恢复粒度：
  - 最近进入的分区
- 退出 landing：
  - 回课程或 chapter

#### Quiz

- 单题或单 family 逐屏推进
- 选项区必须为触控优化布局
- 恢复粒度：
  - `familyId + variantId + answers`
- 退出 landing：
  - 回 step 或 chapter completion

#### Longform

- 草稿区优先垂直布局
- rubric / reference answer 不得压缩正文主输入区
- 恢复粒度：
  - `familyId + draft`
- 退出 landing：
  - 回 chapter 或 practice hub

#### Textbook

- 手机端默认单列阅读
- section 导航采用折叠目录或底部目录，不用永久侧栏
- 恢复粒度：
  - `sectionId/anchor`
- 退出 landing：
  - 回 chapter，或若从特定 step 跳入则回该 step

## 9.5A Learning Rhythm Contract

这是从 prototype 抽出来的核心，不是视觉建议，而是节奏约束。

### Story Beat Rules

- 默认一次只展示一个 `screen`
- `screen` 的主文本默认不超过 1 到 2 行
- 每个 `screen` 的阅读负担必须小到“点击后立刻想看下一屏”
- `exercise` screen 必须嵌在 narrative flow 中，而不是跳到独立题页
- 连续多个 explanation-heavy screen 之后，必须插入认知转换：
  - 小练习
  - 反问
  - 术语澄清
  - 小结式 punchline

### Story Mode UI Rules

- 必须有：
  - 当前 step identity
  - 微型 progress
  - back
  - exit
  - narrative log
- narrative log 必须保留已读 `screen` 历史
- term click 必须在不离开 story context 的情况下打开 glossary
- 关闭 glossary 后，必须回到原 `screen`

### CTA Rhythm Rules

- story 内主 CTA 永远只有一个：继续
- secondary CTA 只能是：
  - back
  - log
  - bookmark/save
  - exit
- 不允许在 story screen 内同时展示多个导航分支

### 9.6 Session Orchestration Rules

系统必须把各种 session 编排成一个连续学习过程，而不是彼此孤立的工具页面。

最小编排规则：

1. `story_session` 完成后，优先给出：
   - 继续下一 step
   - 进入本 step 练习
2. `review_session` 完成后，优先返回：
   - 刚才所在课程
   - 或首页继续学习入口
3. `quiz_session` 与 `longform_session` 不抢主线入口
   - 它们是 reinforcement，不是默认首页主 CTA
4. `textbook_session` 应从属于 chapter
   - 可以作为“预习 / 复习 / 查漏”的入口
   - 不应替代 story progression

## 9.6A Transition Matrix

为了保护心流，session 间切换必须遵守下面的矩阵。

### Story -> Review

- 默认不自动跳转
- 只有在 step 结束且用户主动接受时进入
- 若 due 很高，也只能在 landing 中建议，不得中途硬插

### Story -> Textbook

- 允许从 chapter hub 进入
- story 进行中不主动推荐跳 textbook

### Textbook -> Story

- 若从 chapter hub 进入 textbook，则返回 chapter hub
- 若从 step 上下文跳入 textbook 某锚点，则优先返回该 step

### Story -> Quiz / Longform

- 只在 step 结束或 chapter checkpoint 处暴露
- 不在 story 中部突然切出

### Global Review -> Story

- 完成 global review 后，优先恢复最近的 resumable story
- 若无 resumable story，则回 Library strongest CTA

### Interruptibility Rules

- `story_session`
  - 可暂停
  - 不应被系统强制打断
- `review_session`
  - 可中断
  - 但必须保存 queue snapshot
- `quiz_session`
  - 可中断并保存答案
- `longform_session`
  - 可中断并保存草稿
- `textbook_session`
  - 可中断并保存 anchor

## 9.7 Session State Contract

每个 session 不能共用一个宽泛 shape，必须使用 discriminated union。

```ts
type BaseSessionState = {
  sessionId: string;
  kind: "story" | "review" | "quiz" | "longform" | "textbook";
  courseId: string;
  chapterId?: string;
  enteredAt: string;
  updatedAt: string;
  status: "active" | "paused" | "completed" | "abandoned";
  exitTarget?: "library" | "course_hub" | "chapter_hub" | "resume_previous";
};

type StorySessionState = BaseSessionState & {
  kind: "story";
  stepId: string;
  screenIndex: number;
  seenScreenIds: string[];
  openedTermIds: string[];
  exerciseDrafts: Record<string, unknown>;
};

type ReviewSessionState = BaseSessionState & {
  kind: "review";
  scope: "course" | "global";
  queueSnapshotId: string;
  currentCardIndex: number;
  currentCardId: string;
  answeredCardIds: string[];
};

type QuizSessionState = BaseSessionState & {
  kind: "quiz";
  stepId?: string;
  familyIds: string[];
  currentFamilyId: string;
  currentVariantId?: string;
  answers: Record<string, unknown>;
};

type LongformSessionState = BaseSessionState & {
  kind: "longform";
  stepId?: string;
  familyIds: string[];
  currentFamilyId: string;
  drafts: Record<string, string>;
};

type TextbookSessionState = BaseSessionState & {
  kind: "textbook";
  anchor?: string;
  lastVisibleSectionId?: string;
};

type SessionState =
  | StorySessionState
  | ReviewSessionState
  | QuizSessionState
  | LongformSessionState
  | TextbookSessionState;
```

这能保证：

- session 可恢复
- session 可统计
- session 可复盘

### 9.8 Persistence Boundary

必须明确哪些状态落本地，哪些状态落服务端。

- local-only
  - 临时动画状态
  - glossary panel 开关
- device-persistent
  - story 当前 `screenIndex`
  - textbook 当前 section anchor
  - quiz / longform 草稿
- server-persistent
  - last resumable session
  - review queue snapshot
  - FSRS rating history
  - course/chapter/step completion

### 9.9 Resume And Abandon Rules

必须显式定义：

- `resumable`
  - 最近一次 `active` 或 `paused` 且未完成的 session
- `abandoned`
  - 连续超过设定时间未更新，且未完成
- `resume_failure_fallback`
  - story 恢复失败 -> 回到 chapter hub，并提示继续最近 step
  - review 恢复失败 -> 重新 materialize queue
  - textbook 恢复失败 -> 回到 chapter textbook 顶部

## 10. UI Patterns That Must Stay

这些是原型的高价值模式，建议保留。

### 10.1 Overlay Learning Mode

主学习流进入全屏/准全屏模式，而不是留在普通列表页里小窗阅读。

### 10.2 Strong Typography

核心内容应使用大字号、高对比、高节奏的排版，让学生有“被带着走”的感觉。

### 10.3 Motion as Transition of State

动画不是装饰，而是帮助学生感知：

- 进入学习
- 翻卡
- 打开 glossary
- 完成一个 session

### 10.4 Identity-Rich Course Cards

课程卡片要有明显 identity，而不是统一白盒子。

### 10.5 Session Framing

每个 mode 都要有强 framing：

- `Start Route`
- `Recall Session`
- `The Vault`
- `Glossary Entry`

命名有助于用户理解“我现在在做什么”。

## 10.6 Visual Language Contract

这一节不是审美建议，而是实现约束。目标是防止平台退化成“深色 SaaS 面板”。

### Density Rules

- 首页与课程页允许 grid/list，但必须保留明显主次层级
- story / review / glossary 必须是 stage-like 体验，不能做成普通卡片流
- learner-facing 首屏禁止均匀铺满信息块

### Composition Rules

- 必须有明确前景层、背景层、焦点层
- 重要 mode 必须有进入感与退出感
- 不允许用表头 + 表格 + 标签行作为默认课程浏览壳
- 不允许把首页首屏做成均匀信息栅格
- 不允许 learner-facing 首屏出现管理面板式 toolbar 主导页面

### Surface Rules

- 卡片不可全部同权同大小
- 主 CTA 区域必须与信息区有明显张力差
- 背景、模糊、阴影、边框要用于建立舞台感，而不是装饰性堆叠
- 课程卡必须有 identity 区和行动区
- story/review/glossary 的表面语气必须明显不同

### Motion Rules

- motion 只能服务于：
  - 进入 session
  - 翻卡
  - 打开 glossary/log
  - 完成一个学习动作
- 禁止无意义浮动、全局持续动效、抢正文注意力的装饰动画
- 默认使用短促、可读的 transition
- 禁止页面级大面积动画遮挡主文本

### Emotional Positioning

- story：沉浸、推进、专注
- glossary：澄清、停顿、回到主线
- review：明确、节律、轻压测
- vault：探索、分区、收藏馆
- textbook：系统、安静、可回查

### Explicit Visual Anti-Patterns

- 禁止“深色背景 + 平铺卡片 + 顶部筛选栏”作为默认学习首页
- 禁止“表格/表头/标签云”占据 learner-facing 首屏
- 禁止 story mode 中出现类似 CMS inspector 的侧信息栏
- 禁止把 Vault 做成裸 family ID 列表

## 10.7 Visual Appendix

这一节给实现者一个更直接的 mode-level 构图参考。

### Library

- 允许：
  - identity-rich course cards
  - 单一 strongest CTA + 次级课程浏览
- 禁止：
  - 首屏均匀卡片海
  - 先出现筛选栏再出现学习入口

### Course Hub

- 允许：
  - 大封面 / 大标题 / 课程 identity
  - 一个明显的继续动作
- 禁止：
  - 把 chapter、review、vault 做成同权三宫格首页

### Chapter Hub

- 允许：
  - 章节标题
  - 继续本章
  - 弱化的 step list
- 禁止：
  - 默认展开全量 step 表格式列表占据首屏

### Story

- 必须：
  - 舞台式前景层 + 焦点层
  - 大字体正文
  - 单主 CTA
- 禁止：
  - 多栏阅读器
  - 侧信息面板常驻压迫正文

### Review

- 必须：
  - 单卡焦点
  - 明确翻卡
  - 明确 rating
- 禁止：
  - 多卡并排
  - 列表式复习待办壳

### Vault

- 必须：
  - 分区感
  - 图鉴/收藏馆感
- 禁止：
  - 裸 family 列表
  - 首屏高级筛选器主导

### Textbook

- 必须：
  - 安静、系统、长阅读友好
  - 目录从属于正文，不压正文
- 禁止：
  - 管理面板式双栏工具页外观

## 11. Anti-Patterns To Avoid

这些做法会把平台拉回“后台管理面板”。

### 11.1 Asset-First Home

错误：

- 首页就是 lesson 列表
- 到处是文件路径、step id、family id

正确：

- 首页先展示课程世界、继续学习、待复习入口

### 11.2 Filter-First Practice

错误：

- 一进题库就先看到筛选器、排序、标签

正确：

- 先按学习意图分区：
  - 记忆巩固
  - 测验突击
  - 长回答训练

### 11.3 Dashboard Overload

错误：

- 首页塞满 mastery、score、retention、heatmap、coverage、streak

正确：

- 首页只保留高信号指标
- 其余进入次级页面

### 11.4 Reader-Only Story Mode

错误：

- 把 `screens` 直接串成长文页面

正确：

- 保持逐屏点击和叙事推进感

### 11.5 Glossary as Detached Encyclopedia

错误：

- 点术语后跳到新页面

正确：

- 使用侧滑 panel，在原情境中解释

## 12. Progress Model

进度不应只是一根总进度条，应分成三个维度。

### 12.1 Route Progress

- chapter 完成度
- step 完成度
- 当前停留位置

### 12.2 Memory Progress

- due cards
- review completion
- mastery trajectory

### 12.3 Capability Progress

- quiz 完成情况
- longform 覆盖情况
- 薄弱概念

平台在 UI 上不一定要同时展示三者，但系统层应分开建模。

### 12.4 Progress Display Rule

进度展示必须遵守：

- 首页看趋势，不看细账
- 课程页看路线，不看噪声
- session 内看当前推进，不看平台总览

这条规则是为了避免把学习流重新做成 dashboard。

## 13. State Model

```ts
type LearnerState = {
  activeCourseId?: string;
  activeChapterId?: string;
  activeStepId?: string;
  lastResumableSessionId?: string;
  sessions: Record<string, SessionState>;
  courseProgress: Record<string, CourseProgress>;
  reviewQueues: {
    global?: ReviewQueueState;
    byCourse: Record<string, ReviewQueueState>;
  };
};

type CourseProgress = {
  mastery: number;
  chapterProgress: Record<string, number>;
  completedSteps: string[];
  bookmarkedSteps: string[];
  lastVisitedAt?: string;
  currentChapterId?: string;
};

type ReviewQueueState = {
  snapshotId: string;
  scope: "course" | "global";
  courseId?: string;
  cardIds: string[];
  generatedAt: string;
  dueCount: number;
};
```

## 14. Platform Features

### 14.1 Must Have

- 多课程 Library
- Course Hub
- Chapter Hub
- Story Session
- Glossary Panel
- FSRS Review Session
- Vault / Practice Atlas
- Textbook Entry
- Resume Last Session

### 14.2 Should Have

- Global review queue
- Chapter checkpoint mode
- Cross-link from textbook to question family
- Cross-link from step to related practice

### 14.3 Nice to Have

- Boss quiz at chapter end
- Daily quest framing
- Adaptive recommendation for next chapter

### 14.4 MVP Sequence

MVP 不应按页面切，而应按能力切片。

`Capability Slice 1: Asset Ingestion`

- `course-index.json`
- deterministic loader
- chapter discovery
- hard/soft error handling
- canonical ID / index generation

退出标准：

- 至少 1 门课程可稳定装载为 `CoursePackage`
- 缺失文件时行为确定，不靠实现方猜测

`Capability Slice 2: Story Runtime`

- Library
- Course Hub
- Chapter Hub
- Story Session
- Glossary Panel
- Learning Rhythm Contract

退出标准：

- 用户能从 Library 进入可用 story session
- term click / log / progress / exit 都成立

`Capability Slice 3: Persistence & Resume`

- session persistence
- resume policy
- abandoned policy
- chapter-level return path

退出标准：

- story 中断后可恢复到最近 screen
- 首页 strongest CTA 可 deterministic 生成

`Capability Slice 4: Course-Local Review`

- course-local FSRS review
- review queue snapshot
- review completion flow

退出标准：

- 用户可在课程内完成一轮 recall session

`Capability Slice 5: Global Scheduler`

- global review queue
- cross-course recommendation policy
- anti-starvation rules

退出标准：

- 首页可在多课程之间生成单一 strongest CTA

`Capability Slice 6: Practice Atlas`

- Vault entry
- practice grouping
- quiz / longform 分流
- textbook cross-link

退出标准：

- step 完成后可进入正确的 practice branch

blocked items 必须显式记录，不能靠口头同步。

## 15. Technical Architecture Direction

### 15.1 Loader Layer

先做统一 loader，把课程资产转成平台级模型。

loader 负责：

- 读取 `course-index.json`
- 加载每章 `manifest / step / question_bank / textbook`
- 组装课程 identity 与学习状态
- 建立 question index / term index / review index

### 15.1A Course Package Contract

canonical `CoursePackage / ChapterPackage / StepPackage / TextbookPackage` 定义见 `8.8 Canonical Package Contract`。

这一节不再重复定义 shape，只负责说明装配职责：

- 拼装 package
- 建立 indexes
- 产出 loader warnings / errors
- 对 chapter 标记 `ready / degraded / blocked`

### 15.2 Session Engine

前端不应每个 mode 各写一套杂乱逻辑。

建议有统一 session engine：

- `story_session`
- `review_session`
- `quiz_session`
- `longform_session`
- `textbook_session`

每种 session 有：

- entry state
- current item pointer
- completion condition
- exit destination

### 15.3 Cross-Course Review Service

FSRS / review 不应绑定在单个页面组件里。

应有独立 review service：

- 生成 due queue
- 按课程聚合
- 按全局聚合
- 回写 rating 结果

### 15.3A Cross-Course Recommendation Policy

首页 strongest CTA 本质上是一个 scheduler 决策，不是 UI 文案问题。

推荐输入：

- 是否存在 `lastResumableSession`
- 全局 due count
- 每课程 due count
- 最近学习时间
- 最近完成的 chapter / step
- 新解锁 chapter
- 课程饥饿时间

推荐决策树：

1. 若存在可恢复 story session，优先恢复它
2. 否则，若全局 due 超过每日 review 阈值，优先进入 global review
3. 否则，优先推荐：
   - 已开始但未完成的 chapter
   - 且最近 7 天内有活跃
4. 若没有活跃 chapter，则推荐一个新解锁 chapter
5. 若多个课程竞争，则选择：
   - 饥饿时间更长的课程
   - 但不能连续多天压制新课程

### 15.3B Scheduling Guardrails

- 必须有每日 review 上限，避免 due board 吞掉学习主线
- 必须有 anti-starvation 规则，避免新课程永远拿不到 strongest CTA
- 必须有 stale revival 规则，允许长期未学课程重新获得曝光

### 15.4 Event Contract

这个系统如果没有统一事件 contract，很快会出现“指标定义一版、埋点实现一版、分析口径一版”。

最小事件层必须覆盖：

- 进入 Library
- 点击课程卡
- 进入 chapter
- 开始 story session
- 看到 story screen
- 完成 / 中断 step
- 打开 glossary
- 完成 screen exercise
- 开始 recall session
- 完成 recall session
- 提交 FSRS rating
- 进入 Vault
- 开始 quiz / longform
- 打开 textbook

推荐事件模型：

```ts
type LearningEvent =
  | { type: "library_opened"; at: string }
  | {
      type: "next_action_rendered";
      layer: "library" | "course" | "chapter";
      courseId?: string;
      chapterId?: string;
      targetKind: string;
      resolvedTarget?: {
        kind: "story" | "review" | "quiz" | "longform" | "textbook" | "chapter";
        courseId?: string;
        chapterId?: string;
        stepId?: string;
        sessionId?: string;
      };
      resolutionRule?:
        | "resume_story"
        | "global_review"
        | "course_review"
        | "unfinished_chapter"
        | "new_unlocked_chapter"
        | "chapter_practice"
        | "chapter_start";
      at: string;
    }
  | {
      type: "next_action_clicked";
      layer: "library" | "course" | "chapter";
      courseId?: string;
      chapterId?: string;
      targetKind: string;
      resolvedTarget?: {
        kind: "story" | "review" | "quiz" | "longform" | "textbook" | "chapter";
        courseId?: string;
        chapterId?: string;
        stepId?: string;
        sessionId?: string;
      };
      resolutionRule?:
        | "resume_story"
        | "global_review"
        | "course_review"
        | "unfinished_chapter"
        | "new_unlocked_chapter"
        | "chapter_practice"
        | "chapter_start";
      at: string;
    }
  | { type: "course_opened"; courseId: string; at: string }
  | { type: "chapter_opened"; courseId: string; chapterId: string; at: string }
  | { type: "chapter_completed"; courseId: string; chapterId: string; at: string }
  | { type: "story_started"; courseId: string; chapterId: string; stepId: string; at: string }
  | { type: "story_screen_viewed"; courseId: string; chapterId: string; stepId: string; screenIndex: number; at: string }
  | { type: "story_completed"; courseId: string; chapterId: string; stepId: string; at: string; screensSeen: number }
  | { type: "story_abandoned"; courseId: string; chapterId: string; stepId: string; at: string; screensSeen: number }
  | { type: "session_resumed"; kind: "story" | "review" | "quiz" | "longform" | "textbook"; courseId: string; chapterId?: string; stepId?: string; at: string }
  | { type: "term_opened"; courseId: string; chapterId: string; stepId: string; termId: string; at: string }
  | { type: "screen_exercise_completed"; courseId: string; chapterId: string; stepId: string; screenIndex: number; at: string }
  | { type: "review_started"; courseId?: string; queueSize: number; at: string }
  | { type: "review_completed"; courseId?: string; queueSize: number; at: string }
  | { type: "review_rated"; cardId: string; rating: "again" | "hard" | "good" | "easy"; at: string }
  | { type: "vault_opened"; courseId: string; at: string }
  | { type: "practice_started"; kind: "quiz" | "longform"; courseId: string; chapterId: string; at: string }
  | { type: "textbook_opened"; courseId: string; chapterId: string; at: string };
```

### 15.5 Analytics Layer

analytics 只能依赖 `15.4 Event Contract` 中已定义事件。

任何指标如果依赖未声明事件，视为 spec 错误。

strongest CTA / scheduler 相关指标，必须基于：

- `next_action_rendered`
- `next_action_clicked`
- `session_resumed`
- `resolvedTarget`
- `resolutionRule`

### 15.6 Quality Gates

learner-facing 上线前，至少要过四类 gate：

- 内容 gate：
  - 能正确读取 manifest / step / question bank / textbook
- 体验 gate：
  - 从首页到进入 story session 不超过 3 次主要点击
- 恢复 gate：
  - 中断后能恢复到上次 session 与 screen
- 区分 gate：
  - flashcard / quiz / longform 不可被渲染成同一种练习壳
- 调度 gate：
  - strongest CTA 的决策来源可解释、可复现、可埋点

## 16. Example User Journeys

### 16.1 New User

`进入 Library -> 选一门课程 -> 进入 Course Hub -> 选一章 -> Start Route -> 完成 step1 -> 查看术语 -> 做一个嵌入练习 -> 返回章节`

### 16.2 Returning User

`打开首页 -> 看到“继续 Web Scraping 进阶” -> 继续 chapter -> 学完一段 -> 顺手完成 6 张 due cards -> 离开`

### 16.3 Heavy Learner

`在多个课程间切换 -> 上午推进量化交易 -> 晚上做全局 FSRS review -> 周末进入 Vault 做长答训练`

## 17. Success Criteria

这个系统成功，不是因为它显示了更多数据，而是因为它提高了以下行为：

- 学生能快速进入学习状态
- 学生愿意完成更多 step
- 学生更频繁地返回做复习
- 多课程并行时仍然知道下一步
- 学生对术语与概念的中断成本更低

### 17.1 Measurable Success Metrics

为了让这份 spec 可检查，至少定义以下指标：

- `time_to_learning`
  - 从打开平台到进入第一个 story screen 的时间
- `route_start_rate`
  - 进入 chapter 后点击 `Start Route` 的比例
- `step_completion_rate`
  - 开始 story session 后完成当前 step 的比例
- `term_resolution_rate`
  - 打开 glossary 后返回并继续当前 step 的比例
- `review_completion_rate`
  - 开始 recall session 后完成整轮复习的比例
- `vault_entry_rate`
  - 完成 chapter 后进入 practice 的比例
- `7d_return_rate`
  - 7 天内再次回到平台的比例

每个指标都必须带公式口径：

- `time_to_learning`
  - 分子：首次打开后进入第一个 `story_screen_viewed` 的耗时秒数
  - 分母：成功进入 learner-facing 首页的新用户 session
- `route_start_rate`
  - 分子：进入 chapter 后触发 `story_started`
  - 分母：触发 `chapter_opened`
- `step_completion_rate`
  - 分子：触发 `story_completed`
  - 分母：触发 `story_started`
- `term_resolution_rate`
  - 分子：打开 glossary 后 60 秒内回到 story 并继续推进至少 1 屏
  - 分母：触发 `term_opened`
- `review_completion_rate`
  - 分子：触发 `review_completed`
  - 分母：触发 `review_started`
- `vault_entry_rate`
  - 分子：chapter 完成后 10 分钟内触发 `vault_opened` 或 `practice_started`
  - 分母：chapter 完成事件
- `7d_return_rate`
  - 分子：7 天内再次触发任意 learner-facing session
  - 分母：在观察窗口内首次进入平台的用户

每个假设还应配：

- baseline
- target
- falsification rule
- guardrail metric

### 17.2 MVP Acceptance Thresholds

首个可用版本建议满足以下门槛：

- `p75 time_to_learning <= 60s`
- `step_completion_rate` 不得在前 2 个 screen 出现异常高流失
- `term_resolution_rate >= 0.7`
- `review_completion_rate` 必须高于裸列表基线，且 guardrail 不恶化
- strongest CTA 必须对同一输入产生可复现输出

这些门槛不要求今天就有精确数字，但必须在试运行前被明确成可评审标准。

### 17.3 Anti-Admin Checklist

每次评审 learner-facing 首页或 chapter 页，都做 pass/fail 检查：

- 首页首屏是否出现文件路径、schema 字段名、step id、family id
- 首页首屏是否出现多个同权主 CTA
- chapter 首屏是否先出现筛选器而不是学习入口
- story session 内是否出现多余导航分支
- practice 入口是否把 flashcard / quiz / longform 混成同一列表

只要任一项失败，就说明后台化风险重新出现。

## 18. Final Decision Rule

以后评审任何页面或功能，都可以问一个简单问题：

> 它更像“学习体验的一部分”，还是更像“课程后台的一部分”？

如果答案偏向后者，就说明设计跑偏了。

这个平台的上限，不在于能展示多少 schema，而在于能否把 schema 藏到体验后面，让学生感受到的是：

- 一条路线
- 一场 session
- 一次进步

而不是一堆资产。

## 19. PDCA Operating Cadence

这份 spec 不是一次性定稿文件，而应成为每轮迭代的 operating contract。

### 19.1 Plan

每轮迭代开始前，必须明确：

- 本轮要验证哪条产品假设
- 本轮只改哪一段学习回路
- 本轮成功看哪 1 到 3 个指标

### 19.2 Do

执行阶段必须保证：

- 只实现本轮假设需要的最小功能
- 不为了“看起来完整”而同时铺太多模式
- learner-facing 改动必须可埋点、可恢复、可回滚

### 19.3 Check

每轮上线后，至少检查三类信号：

- 定量：
  - session 开始率、完成率、退出点
- 定性：
  - 用户是否觉得“像在学习”还是“像在操作系统”
- 结构性：
  - 是否出现新的后台感反模式

### 19.4 Act

复盘后，问题按三类处理：

- `fix_now`
  - 直接破坏主学习流的问题
- `next_cycle`
  - 不阻断主线，但明显降低体验的问题
- `watchlist`
  - 有风险信号，但证据不足的问题

### 19.5 Review Ritual

建议每一轮评审都用同一模板：

1. 本轮验证的假设是什么
2. 哪个学习回路被影响
3. 指标发生了什么变化
4. 用户在哪一步失去动机
5. 哪些设计重新出现了后台感
6. 下一轮删什么、保什么、补什么

这就是这份 spec 的 PDCA 闭环。
