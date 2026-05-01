你负责实现一个面向真实学生的 customer-facing course player。

你没有读取仓库文件的能力，所以我把你需要的上下文、schema、样例和约束全部内联在下面。你的任务不是再定义内容 schema，而是基于现有生成产物的真实结构，做一个正确、稳健、可扩展的播放器。

---

## 你的目标

实现一个课程播放器，服务以下真实课程资产：

- `guided_story`：手机端逐屏点击学习
- `question_bank`：step-local 练习资产
- `textbook.mdx`：lesson 级预习/复习教材

这个播放器必须：

1. 不创造第二份 source of truth
2. 保留 step-local ownership
3. 先做 normalization loader，再做 UI
4. 正确区分 flashcard / quiz / longform
5. 正确处理 `<term>` 术语交互
6. 支持 screen 内嵌练习
7. 对未来 schema 漂移保持前向兼容

---

## 学习理念

这些资产不是普通 CMS 文章，也不是普通题库列表。

- `guided_story` 是 learn mode，不是讲义摘要。
  每个 step 是一个自然学习单元，每个 screen 只推进一个很小的认知动作，适合手机端逐屏点击。
- `flashcard`、`quiz`、`longform` 是三种不同练习载体。
  - `flashcard`：主动检索，短、准、快
  - `quiz`：测验式判断，常见选择题/判断题
  - `longform`：解释、比较、推导、应用
- `textbook.mdx` 是 lesson 级复习/预习材料，不替代 guided story 主学习流。
- step-local ownership 很重要。
  每个 step 自己拥有 `step.json` 和 `question_bank.json`。

一句话：

> 这是一个“逐屏学习 + step 内练习 + lesson 级复习”的课程资产包，不是一个扁平 CMS。

---

## 最重要的真相源

真实数据源是：

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

也就是说：

- `manifest.json` 管 step 列表
- `stepN/step.json` 管 step 内容
- `stepN/question_bank.json` 管 step 题库
- `<lesson_id>.mdx` 管 lesson 级课本

不要发明第二份课程真相源。

---

## 非常关键：prompt schema 和真实落盘 schema 不一样

有一个很容易误解的点：

某些生成 prompt 里描述过 lesson-level nested 输出，像这样：

```json
{
  "lesson_id": "L1",
  "mode": "guided_story",
  "steps": [
    {
      "sequence_id": "step1",
      "concept": "topic",
      "step": {
        "...": "真实 step 内容"
      }
    }
  ]
}
```

但当前真实落盘格式不是这样。

当前真实格式是：

- `manifest.json` 只保存 step 元数据和文件路径
- 真实 step 内容单独存到 `stepN/step.json`
- 真实题库存到 `stepN/question_bank.json`

所以你的播放器必须以“分文件 step-local layout”为准，而不是以 lesson-level nested object 为准。

---

## 当前真实 manifest schema

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

真实样例：

```json
{
  "lesson_id": "L2",
  "mode": "guided_story",
  "steps": [
    {
      "sequence_id": "step1",
      "concept": "What is web scraping and why it matters for algorithmic trading",
      "file": "research/pipeline/3-guided_story/L2/step1/step.json"
    },
    {
      "sequence_id": "step2",
      "concept": "Understanding the basic structure of a website: HTML, CSS, and JavaScript",
      "file": "research/pipeline/3-guided_story/L2/step2/step.json"
    },
    {
      "sequence_id": "step3",
      "concept": "Using BeautifulSoup to parse HTML and extract data",
      "file": "research/pipeline/3-guided_story/L2/step3/step.json"
    },
    {
      "sequence_id": "step4",
      "concept": "Real-world scraping: S&P 500 list and handling anti-bot measures",
      "file": "research/pipeline/3-guided_story/L2/step4/step.json"
    },
    {
      "sequence_id": "step5",
      "concept": "Using yfinance for reliable financial data access",
      "file": "research/pipeline/3-guided_story/L2/step5/step.json"
    },
    {
      "sequence_id": "step6",
      "concept": "Storing scraped data in a SQLite database",
      "file": "research/pipeline/3-guided_story/L2/step6/step.json"
    },
    {
      "sequence_id": "step7",
      "concept": "Database design considerations for financial data",
      "file": "research/pipeline/3-guided_story/L2/step7/step.json"
    },
    {
      "sequence_id": "step8",
      "concept": "Limitations of web scraping and alternative tools",
      "file": "research/pipeline/3-guided_story/L2/step8/step.json"
    }
  ]
}
```

语义：

- `steps` 非空
- `sequence_id` 是 step 主键
- `concept` 可用于 step 卡片标题
- `file` 是 step 文件路径

---

## 当前真实 step.json schema

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

当前观测到的稳定顶层字段：

- `lesson_id`
- `sequence_id`
- `target_language`
- `mode`
- `source`
- `term_catalog`
- `screens`

当前观测到的稳定 screen type：

- `narration`
- `exercise`

真实样例片段：

```json
{
  "lesson_id": "L2",
  "sequence_id": "step1",
  "target_language": "zh-CN",
  "mode": "guided_story",
  "term_catalog": {
    "algorithmic_trading": {
      "aliases": ["Algorithmic Trading", "Algo Trading"],
      "display": "算法交易",
      "gloss": "用规则和程序处理交易判断与执行的方式。"
    },
    "web_scraping": {
      "aliases": ["Web Scraping", "Data Scraping"],
      "display": "网络爬取",
      "gloss": "从网站自动提取数据的过程。"
    }
  },
  "screens": [
    {
      "focus_terms": [],
      "id": "s001",
      "introduced_terms": [],
      "lines": [
        "想象一下，你每天需要手动查看几百只股票的价格、新闻和财报。",
        "这几乎不可能，而且很容易出错。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": ["web_scraping"],
      "id": "s002",
      "introduced_terms": ["web_scraping"],
      "lines": [
        "现在，让程序替你完成这些重复的工作。",
        "这就是 <term id=\"web_scraping\">网络爬取</term> 的核心思想。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": ["algorithmic_trading"],
      "id": "s003",
      "introduced_terms": ["algorithmic_trading"],
      "lines": [
        "在 <term id=\"algorithmic_trading\">算法交易</term> 中，数据就是燃料。",
        "没有数据，策略就是空谈。"
      ],
      "type": "narration"
    }
  ]
}
```

你必须正确理解这些字段：

- `lines` 是主内容，不要直接拼成长文段落
- `<term id="...">显示文本</term>` 不是垃圾 HTML，而是术语锚点
- `term_catalog` 是 glossary 数据源
- `focus_terms` 可用于“本屏关注术语”
- `introduced_terms` 可用于高亮“本屏首次引入的新术语”
- 若 `screen.exercise` 存在，则应在当前 screen 内完成交互

---

## 当前真实 question_bank.json schema

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

当前真实 `question_type` 里观测到的值包括：

- `short_answer`
- `single_choice`
- `true_false`
- `micro_calc`
- `core_difference`
- `mechanism_trace`
- `short_explain`
- `compare_and_contrast`

真实样例片段：

```json
{
  "lesson_id": "L2",
  "target_language": "zh-CN",
  "coverage_map": [
    {
      "coverage_tag": "web_scraping_definition",
      "covered_by": [
        "qf_flash_web_scraping_def",
        "qf_quiz_web_scraping_use_case"
      ],
      "description": "网络爬取的定义：从网站自动提取数据的过程。"
    },
    {
      "coverage_tag": "web_scraping_algo_trading_use_cases",
      "covered_by": [
        "qf_flash_web_scraping_use_cases",
        "qf_quiz_web_scraping_use_case",
        "qf_long_web_scraping_use_cases"
      ],
      "description": "网络爬取在算法交易中的常见用例：实时数据收集、市场情绪分析、公司分析、另类数据。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "web_scraping_definition",
      "coverage_tags": ["web_scraping_definition"],
      "difficulty": "easy",
      "family_id": "qf_flash_web_scraping_def",
      "learning_goal": "学生能准确说出网络爬取的定义。",
      "linked_steps": ["step1"],
      "question_type": "short_answer",
      "retrieval_focus": "网络爬取的核心定义：自动化地从网站提取数据。",
      "term_refs": [
        {
          "display": "网络爬取",
          "en": "Web Scraping"
        }
      ],
      "variants": [
        {
          "back": "从网站自动提取数据的过程。",
          "estimated_seconds": 8,
          "explanation": "网络爬取是一种自动化技术，用于从网页中提取结构化或非结构化数据。",
          "front": "网络爬取（Web Scraping）的核心定义是什么？",
          "question_id": "q_flash_web_scraping_def_v1"
        },
        {
          "back": "自动从网站提取数据，例如股价、新闻等。",
          "estimated_seconds": 8,
          "explanation": "网络爬取替代了手动收集数据的工作，提高了效率和准确性。",
          "front": "在算法交易中，网络爬取主要用于什么目的？",
          "question_id": "q_flash_web_scraping_def_v2"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "web_scraping_use_case_identification",
      "coverage_tags": [
        "web_scraping_definition",
        "web_scraping_algo_trading_use_cases"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_web_scraping_use_case",
      "learning_goal": "学生能在具体情境中判断是否应该使用网络爬取。",
      "linked_steps": ["step1"],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "网络爬取",
          "en": "Web Scraping"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "网络爬取正是为了自动从网站提取数据而设计的。手动计算、绘图和编写策略逻辑都不是爬取的核心功能。",
          "options": [
            "手动计算股票平均价格",
            "自动从财经网站收集每日收盘价",
            "用 Excel 绘制图表",
            "编写交易策略的代码逻辑"
          ],
          "question_id": "q_quiz_web_scraping_use_case_v1",
          "stem": "以下哪种情况最适合使用网络爬取？"
        }
      ]
    }
  ],
  "longform_families": [
    {
      "concept_key": "web_scraping_applications",
      "coverage_tags": ["web_scraping_algo_trading_use_cases"],
      "difficulty": "medium",
      "family_id": "qf_long_web_scraping_use_cases",
      "learning_goal": "学生能解释网络爬取在算法交易中的至少三个不同应用场景，并说明其价值。",
      "linked_steps": ["step1"],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "网络爬取",
          "en": "Web Scraping"
        },
        {
          "display": "算法交易",
          "en": "Algorithmic Trading"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "场景1：实时数据收集",
            "场景2：市场情绪分析",
            "场景3：公司分析或另类数据"
          ],
          "question_id": "q_long_web_scraping_use_cases_v1",
          "reference_answer": [
            "1. 实时数据收集：自动从财经网站抓取股票价格、经济指标等实时数据，为交易策略提供输入。",
            "2. 市场情绪分析：爬取新闻、论坛和社交媒体内容，分析市场对特定股票或行业的整体情绪，辅助判断市场走向。",
            "3. 公司分析：自动收集公司公告、财报、管理层变动等信息，用于基本面分析。或者，收集天气、网络流量等另类数据，用于预测某些行业（如农业、零售）的表现。"
          ],
          "rubric_points": [
            "正确识别并解释实时数据收集（如股价、经济指标）",
            "正确识别并解释市场情绪分析（如新闻、社交媒体）",
            "正确识别并解释公司分析（如财报、公告）或另类数据（如天气）"
          ],
          "stem": "请列举并简要解释网络爬取在算法交易中的三个不同应用场景。"
        }
      ]
    }
  ]
}
```

你必须正确理解这些字段：

- family 是一级单元
- variant 是 family 下的同构变体
- `linked_steps` 是硬约束，不只是标签
- `coverage_map` 适合做覆盖可视化，不一定直接展示给学生
- `flashcard_families`、`quiz_families`、`longform_families` 的交互必须明显区分

---

## 当前 textbook.mdx 的真实用途

它不是 JSON，但对播放器很重要。

它是 lesson 级复习/预习材料，frontmatter 里稳定存在：

- `lessonId`
- `title`
- `targetLanguage`
- `mode`
- `sectionMap`
- `coverageTrace`

当前 MDX 中会出现的组件引用包括：

- `QuestionRef`
- `QuestionFamily`
- `Definition`
- `Example`
- `KeyPoint`
- `Pitfall`
- `Checkpoint`
- `Figure`

这意味着你的 player 或 loader 至少要知道：

- textbook 本身不一定内嵌题目内容
- textbook 通过 `QuestionRef id="..."` 引用具体题目
- textbook 通过 `QuestionFamily familyId="..."` 引用题族

当前 MDX 中已经真实出现过的引用例子：

- `QuestionRef id="q_quiz_web_scraping_use_case_v1"`
- `QuestionFamily familyId="qf_quiz_web_components_role"`
- `Checkpoint title="最终自测"`
- `Figure src="images/xxx.jpg" alt="..."`

---

## 当前 viewer 的真实加载行为

当前仓库里的 viewer 基本按照下面的规则取数据：

```ts
textbookPath(lessonId) =
  `research/pipeline/5-textbook/${lessonId}.mdx`

manifestPath(lessonId) =
  `research/pipeline/3-guided_story/${lessonId}/manifest.json`

newStepPath(lessonId, stepId) =
  `research/pipeline/3-guided_story/${lessonId}/${stepId}/step.json`

newQuestionBankPath(lessonId, stepId) =
  `research/pipeline/3-guided_story/${lessonId}/${stepId}/question_bank.json`
```

还有几个重要行为：

- viewer 先读 `manifest.json`
- 再按 step 加载 `step.json` 和 `question_bank.json`
- viewer 会建立 question family / question variant 的索引
- viewer 在预览 `lines` 时，会把 `<term id="...">文本</term>` 转成可见术语标记
- viewer 在 textbook 预览时，会把 `QuestionRef` / `QuestionFamily` 解析到题库索引

你不必照搬现有 creator-review UI，但你的实现逻辑应该和这些真实行为兼容。

---

## 真实校验规则，你必须尊重

这些不是建议，而是当前系统已经显式校验的事实：

1. `manifest.json` 必须有非空 `steps` 数组
2. `manifest.steps[].file` 指向的 `step.json` 必须存在
3. 每个 `step.json` 同目录下必须存在 `question_bank.json`
4. step-local question bank 中任意 family / variant 的 `linked_steps` 都必须只等于当前 step
5. `textbook.mdx` 中的 `QuestionRef` 必须能解析到真实 `question_id`
6. `textbook.mdx` 中的 `QuestionFamily` 必须能解析到真实 `family_id`

对你的实现含义是：

- 不要把题目扁平并 lesson-global 化后丢失 step 归属
- 不要假设 textbook 自己携带完整题目内容
- 必须程序化建立 `questionIndex`

---

## 推荐内部 normalized model

不要让 UI 组件直接吃分散的 raw files。

先做一层 loader / normalizer，输出统一模型，推荐 shape：

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
    byQuestionId: Record<
      string,
      {
        stepId: string;
        familyId: string;
        familyKind: "flashcard" | "quiz" | "longform";
        family: QuestionFamily;
        variant: QuestionVariant;
      }
    >;
    byFamilyId: Record<
      string,
      {
        stepId: string;
        familyKind: "flashcard" | "quiz" | "longform";
        family: QuestionFamily;
      }
    >;
  };
};
```

这个 loader 至少应做：

1. 读取 `manifest.json`
2. 按 `manifest.steps` 逐个加载 `step.json` 与 `question_bank.json`
3. 用目录名或 `sequence_id` 校验归属一致
4. 建立 `questionIndex.byQuestionId`
5. 建立 `questionIndex.byFamilyId`
6. 可选读取 `textbook.mdx` 并解析 frontmatter
7. 忽略未知字段，但不要把它们抹掉

---

## 建议的 raw-to-normalized 映射

- `manifest.lesson_id` -> `package.lessonId`
- `manifest.steps[n].sequence_id` -> `steps[n].sequenceId`
- `manifest.steps[n].concept` -> `steps[n].concept`
- `manifest.steps[n].file` -> `steps[n].stepPath`
- `dirname(stepPath) + /question_bank.json` -> `steps[n].questionBankPath`
- `stepJson.term_catalog` -> glossary source
- `stepJson.screens` -> guided story 屏幕序列
- `questionBank.flashcard_families` -> active recall 练习
- `questionBank.quiz_families` -> assessment-like checks
- `questionBank.longform_families` -> explain/apply 型练习
- `variant.question_id` -> `questionIndex.byQuestionId`
- `family.family_id` -> `questionIndex.byFamilyId`

兼容性规则：

- `front/back` 更接近 flashcard variant
- `stem/options/answer` 更接近 quiz variant
- `prompt_blocks/reference_answer/rubric_points` 更接近 longform scaffold
- 未识别字段一律保留在原始对象上

---

## Player UX 约束

你的 customer player 应优先支持这些体验：

1. step 列表
   - 先让用户看到 lesson 中有哪些 step
   - 每个 step 显示 `sequenceId + concept`

2. 单 step 逐屏播放
   - `lines` 是核心显示单元
   - 不要把 screens 拼成一篇长文章
   - 当前 step 和当前 screen 要清晰可见

3. inline glossary
   - 点击 `<term>` 对应文本时
   - 弹出 `term_catalog[termId]`
   - 显示 `display / aliases / gloss`

4. 内容内嵌练习
   - `screen.exercise` 在当前 screen 内完成作答和反馈
   - 不要强制跳转到外部题库页

5. step-local practice
   - step 学完后，可以进入该 step 的
     - flashcards
     - quiz
     - longform

6. lesson-level textbook
   - 作为预习/复习入口
   - 不取代 guided story 主流

---

## 明确不要做的事

- 不要把 flashcard 渲染成选择题
- 不要把所有题型混成单一“题库”列表
- 不要跳过 step 结构，直接把所有 screen 扁平排序成一条超长 feed
- 不要把 textbook 作为唯一阅读入口
- 不要重新生成课程内容
- 不要把 raw data 变成手写 mock data
- 不要重新设计一套和现有产物不兼容的新 schema

---

## 推荐交付内容

你至少要交付：

1. 一个 data loader / normalizer
2. 一个 customer player 主页面
3. step 列表
4. 单 step 逐屏播放
5. glossary 交互
6. step-local practice 入口与展示
7. textbook 入口

如果你在现有代码库内实现：

- 优先复用既有加载逻辑和类型定义
- 但不要照搬 creator-review 风格 UI
- 允许新 app、新页面层或重构组件
- 但不要破坏既有 viewer 对生成产物的读取方式

---

## 你应该直接开始实现，而不是先空谈

除非遇到真实 blocker，否则按下面顺序推进：

1. loader
2. player 主页面
3. step 播放
4. glossary
5. step-local practice
6. textbook 入口

---

## 如果你要解释设计，只回答这些问题

- 你如何避免第二份 source of truth？
- 你如何处理 step-local question bank？
- 你如何把 `<term>` 渲染成 glossary 交互？
- 你如何区分 flashcard / quiz / longform？
- 你如何对未知字段保持前向兼容？

---

## 最终提醒

你不是在设计一个“课程内容生成器”，你是在实现一个“读取现有课程资产并正确播放它们的产品界面”。

最重要的原则只有四个：

1. 尊重现有真实数据结构
2. 保留 step-local ownership
3. 先 normalization，后 UI
4. 不创造第二份真相源

