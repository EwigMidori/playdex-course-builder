# Course Player Spec

这份 spec 面向“另一个 AI / 工程师来实现 customer-facing course player”。

它基于当前仓库里真实存在的生成产物，而不是只基于 prompt 里的理想 schema。核心目标有两个：

1. 明确当前生成结果的真实 contract，避免误读。
2. 给播放器定义一层推荐的 normalized model，避免前端直接耦合分散文件和漂移字段。

依据来源：

- [apps/viewer/src/types.ts](/Users/zijingzhang/ewig/playdex-course-builder/apps/viewer/src/types.ts:1)
- [apps/viewer/src/data.ts](/Users/zijingzhang/ewig/playdex-course-builder/apps/viewer/src/data.ts:1)
- [crates/app-cli/src/validation.rs](/Users/zijingzhang/ewig/playdex-course-builder/crates/app-cli/src/validation.rs:1)
- [research/prompts/guided_story_system.md](/Users/zijingzhang/ewig/playdex-course-builder/research/prompts/guided_story_system.md:1)
- [research/prompts/question_bank_system.md](/Users/zijingzhang/ewig/playdex-course-builder/research/prompts/question_bank_system.md:1)
- [research/prompts/textbook_user.md](/Users/zijingzhang/ewig/playdex-course-builder/research/prompts/textbook_user.md:1)

## 1. Learning Philosophy

播放器不应把这些内容当成普通文章或题库列表来渲染。生成资产背后的教学理念是：

- `guided_story` 是 learn mode，不是讲义摘要。
  每个 step 是一个自然学习单元，每个 screen 只推进一个很小的认知动作，适合手机端逐屏点击。
- `flashcard`、`quiz`、`longform` 是三种不同练习载体，不应混用。
  `flashcard` 用于主动检索，`quiz` 用于测验式判断，`longform` 用于解释、比较、推导、应用。
- step-local ownership 很重要。
  每个 step 自己拥有 `step.json` 和 `question_bank.json`，不要再造一个新的 step 内容源。
- `textbook.mdx` 是 lesson 级复习/预习材料，不是 guided story 的替代品。
- source alignment 和 coverage completeness 是产品约束。
  播放器不应鼓励“自由编排”或“自动补充知识点”，而应尊重现有结构和 coverage trace。

一句话总结：

> 这是一个“逐屏学习 + step 内练习 + lesson 级复习”的课程资产包，不是一个扁平 CMS。

## 2. Canonical Raw Artifacts

当前 lesson 的真实落盘结构如下：

```text
research/pipeline/
  3-guided_story/<lesson_id>/
    manifest.json
    step1/
      step.json
      question_bank.json
    step2/
      step.json
      question_bank.json
    ...
  5-textbook/
    <lesson_id>.mdx
```

对播放器来说，最重要的 source of truth 是：

- `manifest.json`
- 每个 `stepN/step.json`
- 每个 `stepN/question_bank.json`
- 可选的 `5-textbook/<lesson_id>.mdx`

当前仓库明确要求不要再造第二份课程真相源，见 [docs/progress.json](/Users/zijingzhang/ewig/playdex-course-builder/docs/progress.json:1) 与 [research/README.md](/Users/zijingzhang/ewig/playdex-course-builder/research/README.md:1)。

## 2.1 Important Divergence: Prompt Schema vs Persisted Schema

这是最容易让另一个 AI 误判的地方：

- `guided_story` 的 prompt 里描述过一种 lesson-level 输出：
  `{"lesson_id": "...", "mode": "guided_story", "steps": [{ "sequence_id": "...", "concept": "...", "step": { ...真实 step 内容... } }]}`
- 但当前真实落盘格式不是这样。

当前真实格式是：

- `manifest.json` 只保存 step 元数据和文件路径。
- 每个 step 的真实内容单独保存在 `stepN/step.json`。
- 每个 step 的题库单独保存在 `stepN/question_bank.json`。

也就是说，customer player 必须以“分文件 step-local layout”为准，而不是以 prompt 里的 lesson-level nested object 为准。

这是当前系统已经收敛下来的 canonical layout，不应再反向退回到单文件嵌套结构。

## 3. Raw Schema

### 3.1 Manifest

稳定路径：

- `research/pipeline/3-guided_story/<lesson_id>/manifest.json`

当前真实 shape：

```ts
type StoryManifest = {
  lesson_id: string;
  mode?: "guided_story" | string;
  steps: Array<{
    sequence_id: string;
    concept: string;
    file: string;
  }>;
};
```

最小示例：

```json
{
  "lesson_id": "L2",
  "mode": "guided_story",
  "steps": [
    {
      "sequence_id": "step1",
      "concept": "What is web scraping and why it matters for algorithmic trading",
      "file": "research/pipeline/3-guided_story/L2/step1/step.json"
    }
  ]
}
```

语义说明：

- `steps` 非空。
- `sequence_id` 是 step 主键。
- `file` 是该 step 的 `step.json` 路径。
- `concept` 适合用作 step 卡片标题或概念摘要。

### 3.2 Step JSON

稳定路径：

- `research/pipeline/3-guided_story/<lesson_id>/<step_id>/step.json`

当前真实 shape：

```ts
type TermEntry = {
  display: string;
  aliases: string[];
  gloss: string;
};

type StoryExercise = {
  kind: string;
  prompt: string;
  options?: string[];
  answer?: number | string;
  answers?: string[];
  explanation?: string;
};

type StoryScreen = {
  id: string;
  type: string;
  lines: string[];
  title?: string | null;
  body?: string | null;
  focus_terms?: string[];
  introduced_terms?: string[];
  exercise?: StoryExercise | null;
  media?: unknown;
  formula?: unknown;
  terms?: unknown;
  exercises?: unknown;
};

type StoryStep = {
  lesson_id: string;
  sequence_id: string;
  target_language: string;
  mode: "guided_story" | string;
  source: {
    plain_text: string;
    related: string[];
  };
  term_catalog: Record<string, TermEntry>;
  screens: StoryScreen[];
};
```

当前观测到的稳定字段：

- 顶层稳定存在：`lesson_id`、`sequence_id`、`target_language`、`mode`、`source`、`term_catalog`、`screens`
- `screen.type` 当前样例中只出现：
  - `narration`
  - `exercise`
- `screen.lines` 是主要渲染文本。
- 术语通过内嵌 `<term id="...">显示文本</term>` 标记出现在 `lines` 中。
- `term_catalog` 提供术语弹层所需结构化解释。
- 当前样例中的小练习全部走 `screen.exercise`。

短示例：

```json
{
  "lesson_id": "L2",
  "sequence_id": "step1",
  "target_language": "zh-CN",
  "mode": "guided_story",
  "term_catalog": {
    "web_scraping": {
      "display": "网络爬取",
      "aliases": ["Web Scraping", "Data Scraping"],
      "gloss": "从网站自动提取数据的过程。"
    }
  },
  "screens": [
    {
      "id": "s002",
      "type": "narration",
      "lines": [
        "现在，让程序替你完成这些重复的工作。",
        "这就是 <term id=\"web_scraping\">网络爬取</term> 的核心思想。"
      ],
      "focus_terms": ["web_scraping"],
      "introduced_terms": ["web_scraping"]
    },
    {
      "id": "s005",
      "type": "exercise",
      "lines": ["下面哪种情况最适合使用网络爬取？"],
      "exercise": {
        "kind": "single_choice",
        "prompt": "下面哪种情况最适合使用网络爬取？",
        "options": [
          "手动计算股票平均价格",
          "自动从财经网站收集每日收盘价",
          "用 Excel 绘制图表",
          "编写交易策略的代码逻辑"
        ],
        "answer": 1,
        "explanation": "网络爬取正是为了自动从网站提取数据而设计的。"
      }
    }
  ]
}
```

播放器解释建议：

- 把 `lines` 视为主内容。
- 把 `<term>` 当成 inline glossary anchor，而不是普通 HTML。
- `focus_terms` 适合显示“本屏关注术语”。
- `introduced_terms` 适合高亮“本屏首次引入的新术语”。
- 若 `screen.exercise` 存在，则本屏是内容内嵌练习，不应跳转到单独题库页才能作答。

### 3.3 Question Bank JSON

稳定路径：

- `research/pipeline/3-guided_story/<lesson_id>/<step_id>/question_bank.json`

当前真实 shape：

```ts
type CoverageItem = {
  coverage_tag: string;
  coverage_id?: string;
  description: string;
  covered_by: string[];
};

type TermRef = {
  display: string;
  en?: string;
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

type QuestionFamily = {
  family_id: string;
  question_type: string;
  linked_steps: string[];
  concept_key: string;
  coverage_tags: string[];
  term_refs?: TermRef[];
  learning_goal?: string;
  difficulty?: string;
  retrieval_focus?: string | null;
  variants: QuestionVariant[];
};

type QuestionBank = {
  lesson_id: string;
  target_language: string;
  source: {
    guided_story_manifest: string;
    plain_text: string;
    related: string[];
    coverage_checklist: string;
    source_outline: string;
    lesson_map: string;
  };
  coverage_map: CoverageItem[];
  flashcard_families: QuestionFamily[];
  quiz_families: QuestionFamily[];
  longform_families: QuestionFamily[];
};
```

当前观测到的 `question_type` 包括：

- `short_answer`
- `single_choice`
- `true_false`
- `micro_calc`
- `core_difference`
- `mechanism_trace`
- `short_explain`
- `compare_and_contrast`

短示例：

```json
{
  "lesson_id": "L2",
  "coverage_map": [
    {
      "coverage_tag": "web_scraping_definition",
      "description": "网络爬取的定义：从网站自动提取数据的过程。",
      "covered_by": [
        "qf_flash_web_scraping_def",
        "qf_quiz_web_scraping_use_case"
      ]
    }
  ],
  "flashcard_families": [
    {
      "family_id": "qf_flash_web_scraping_def",
      "question_type": "short_answer",
      "linked_steps": ["step1"],
      "concept_key": "web_scraping_definition",
      "coverage_tags": ["web_scraping_definition"],
      "term_refs": [{ "display": "网络爬取", "en": "Web Scraping" }],
      "learning_goal": "学生能准确说出网络爬取的定义。",
      "difficulty": "easy",
      "retrieval_focus": "网络爬取的核心定义：自动化地从网站提取数据。",
      "variants": [
        {
          "question_id": "q_flash_web_scraping_def_v1",
          "front": "网络爬取（Web Scraping）的核心定义是什么？",
          "back": "从网站自动提取数据的过程。",
          "explanation": "网络爬取是一种自动化技术，用于从网页中提取结构化或非结构化数据。",
          "estimated_seconds": 8
        }
      ]
    }
  ]
}
```

播放器解释建议：

- family 是一级单元，variant 是同构变体。
- `linked_steps` 是 family 所属 step 的硬约束，不只是标签。
- `flashcard_families`、`quiz_families`、`longform_families` 的展示与交互要明显区分。
- `coverage_map` 适合用于“这一 step 覆盖了什么”的可视化，而不是直接给学生逐字阅读。

### 3.4 Textbook MDX

稳定路径：

- `research/pipeline/5-textbook/<lesson_id>.mdx`

这不是 JSON，但它对播放器很重要，因为它包含 lesson 级 section 结构和题目引用。

frontmatter 中稳定存在：

- `lessonId`
- `title`
- `targetLanguage`
- `mode`
- `sectionMap`
- `coverageTrace`

当前 MDX 会引用这些组件：

- `QuestionRef`
- `QuestionFamily`
- `Definition`
- `Example`
- `KeyPoint`
- `Pitfall`
- `Checkpoint`
- `Figure`

最关键的引用规则：

- `<QuestionRef id="...">` 必须能在任一步题库的 `question_id` 中找到。
- `<QuestionFamily familyId="...">` 必须能在任一步题库的 `family_id` 中找到。

## 4. Validation Invariants

这些不是“建议”，而是当前代码里已经显式校验的事实：

1. `manifest.json` 必须有非空 `steps` 数组，且每个 step 有 `file`。
2. `manifest.steps[].file` 指向的 `step.json` 必须存在。
3. 每个 `step.json` 同目录下都必须存在 `question_bank.json`。
4. step-local question bank 中任意 family / variant 的 `linked_steps` 都必须只等于当前 step。
5. `textbook.mdx` 中的 `QuestionRef` 必须能解析到真实 `question_id`。
6. `textbook.mdx` 中的 `QuestionFamily` 必须能解析到真实 `family_id`。

对应实现见 [crates/app-cli/src/validation.rs](/Users/zijingzhang/ewig/playdex-course-builder/crates/app-cli/src/validation.rs:8)。

对播放器的含义：

- 不要把题目当成 lesson-global 扁平池后失去 step 归属。
- 不要假设 textbook 自己携带题目内容，它只携带引用。
- 不要依赖手写映射表，应该程序化建立 question index。

## 5. Recommended Normalized Model

不建议 customer player 直接把 raw files 当组件 props 到处传。建议先做一层 loader，把 lesson 组装成一个 normalized package。

推荐内部模型：

```ts
type NormalizedCoursePackage = {
  lessonId: string;
  targetLanguage: string;
  manifest: StoryManifest;
  textbook?: {
    path: string;
    rawMdx: string;
    frontmatter: {
      lessonId: string;
      title?: string;
      targetLanguage?: string;
      mode?: string;
      sectionMap?: Array<{
        sectionId: string;
        title: string;
        coverageTags?: string[];
        linkedSteps?: string[];
      }>;
      coverageTrace?: Array<{
        coverageTag: string;
        description?: string;
        sectionIds?: string[];
        linkedSteps?: string[];
      }>;
    };
  };
  steps: Array<{
    sequenceId: string;
    concept: string;
    stepPath: string;
    questionBankPath: string;
    story: StoryStep;
    questionBank: QuestionBank;
    stats: {
      screenCount: number;
      flashcardFamilyCount: number;
      quizFamilyCount: number;
      longformFamilyCount: number;
      questionVariantCount: number;
    };
  }>;
  questionIndex: {
    byQuestionId: Record<string, {
      stepId: string;
      familyId: string;
      familyKind: "flashcard" | "quiz" | "longform";
      family: QuestionFamily;
      variant: QuestionVariant;
    }>;
    byFamilyId: Record<string, {
      stepId: string;
      familyKind: "flashcard" | "quiz" | "longform";
      family: QuestionFamily;
    }>;
  };
};
```

loader 应做的事情：

1. 读 `manifest.json`。
2. 按 `manifest.steps` 逐个加载 `step.json` 与 `question_bank.json`。
3. 用 step 目录名或 `sequence_id` 校验归属一致。
4. 建立 `questionIndex.byQuestionId` 与 `questionIndex.byFamilyId`。
5. 可选解析 `textbook.mdx` frontmatter。
6. 忽略未知字段，但不要丢掉原始字段。

这样做的原因：

- raw schema 有未来扩展位，比如 `media`、`formula`、`answers`。
- 当前 viewer 只消费了其中一部分字段，不代表 customer player 也该硬编码成最小子集。
- normalized loader 可以吸收 schema 漂移，而不是把漂移扩散到 UI 全部组件。

## 6. Player UX Implications

基于当前资产形态，customer player 应优先支持这些体验：

- 逐 step 学习。
  先看 step 列表，再进入 step 内逐屏浏览。
- 单屏阅读。
  `lines` 是核心展示单元，不要直接拼成长文章。
- inline glossary。
  点击 `<term>` 对应文本时，弹出 `term_catalog[termId]` 的 `display / aliases / gloss`。
- 内容内嵌练习。
  `screen.exercise` 应在当前 screen 内完成，而不是强制跳出。
- step-local practice。
  step 学完后可切到该 step 的 `flashcards / quiz / longform`。
- lesson-level textbook。
  作为预习/复习入口，不取代 guided story 主流。

明确不要做的事：

- 不要把 flashcard 渲染成选择题。
- 不要把所有题型混成单一“题库”列表。
- 不要跳过 step 结构，直接把所有 screen 扁平排序成一条超长 feed。
- 不要把 textbook 作为唯一阅读入口。

## 7. Raw-to-Normalized Mapping Rules

建议使用以下映射：

- `manifest.lesson_id` -> `package.lessonId`
- `manifest.steps[n].sequence_id` -> `steps[n].sequenceId`
- `manifest.steps[n].concept` -> `steps[n].concept`
- `manifest.steps[n].file` -> `steps[n].stepPath`
- `dirname(stepPath) + /question_bank.json` -> `steps[n].questionBankPath`
- `stepJson.term_catalog` -> glossary source
- `stepJson.screens` -> learn-mode screen sequence
- `questionBank.flashcard_families` -> active recall
- `questionBank.quiz_families` -> assessment-like checks
- `questionBank.longform_families` -> explain/apply tasks
- `variant.question_id` -> `questionIndex.byQuestionId`
- `family.family_id` -> `questionIndex.byFamilyId`

兼容性规则：

- 把 `front/back` 视为 flashcard-like variant。
- 把 `stem/options/answer` 视为 quiz-like variant。
- 把 `prompt_blocks/reference_answer/rubric_points` 视为 longform scaffold。
- 未识别字段保留在原始对象上，供后续扩展。

## 8. Minimal End-to-End Example

```json
{
  "lessonId": "L2",
  "targetLanguage": "zh-CN",
  "steps": [
    {
      "sequenceId": "step1",
      "concept": "What is web scraping and why it matters for algorithmic trading",
      "story": {
        "screens": [
          {
            "id": "s002",
            "type": "narration",
            "lines": [
              "现在，让程序替你完成这些重复的工作。",
              "这就是 <term id=\"web_scraping\">网络爬取</term> 的核心思想。"
            ]
          }
        ],
        "term_catalog": {
          "web_scraping": {
            "display": "网络爬取",
            "aliases": ["Web Scraping"],
            "gloss": "从网站自动提取数据的过程。"
          }
        }
      },
      "questionBank": {
        "flashcard_families": [
          {
            "family_id": "qf_flash_web_scraping_def",
            "question_type": "short_answer",
            "linked_steps": ["step1"],
            "variants": [
              {
                "question_id": "q_flash_web_scraping_def_v1",
                "front": "网络爬取的核心定义是什么？",
                "back": "从网站自动提取数据的过程。"
              }
            ]
          }
        ]
      }
    }
  ]
}
```

## 9. Build Prompt For Another AI

可直接把下面这段提示词交给另一个 AI。

见：

- [docs/course-player-builder-prompt.md](/Users/zijingzhang/ewig/playdex-course-builder/docs/course-player-builder-prompt.md:1)
