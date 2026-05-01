请基于以下材料，生成一份 lesson 级 MDX 课本。

目标语言：
zh-CN

lesson_id：
L10

要求：
- 面向整节课，而不是单个 step
- 系统化组织内容，适合预习与复习
- 要能嵌入题目与题族
- 必须区分 `flashcard_families`、`quiz_families`、`longform_families` 的用途：闪卡用于主动检索，quiz 用于测验，longform 用于解释与应用
- 不要把选择题称为 flashcard，也不要把 flashcard 放在考试题语境中
- 术语与公式要可交互引用
- 术语必须带英文备注，便于考试和交流
- 不要依赖 `StepLink`
- 必须使用适量可复用组件，例如 `Definition`、`Example`、`KeyPoint`、`Pitfall`、`Checkpoint`、`Figure`
- 必须有 worked examples
- 必须在真正必要时引用原 lesson 中的图片，并写好 `alt`
- 输出必须是可直接保存的 MDX
- 内容结构应由输入中的 `coverage checklist`、`source outline`、`lesson map` 驱动
- 不要写死任何学科、固定主题列表或固定 slide bucket
- frontmatter 中必须包含 `sectionMap` 和 `coverageTrace`
- 每个主要 `##` section 必须在 `sectionMap` / `coverageTrace` 中有显式 `sectionId`，并与标题对齐
- 不要输出 `## 标题 {#section-id}` 这种 heading anchor 语法；这不是稳定的 MDX
- `coverageTrace` 中每个 coverage item 必须明确列出它落在哪些 `sectionIds`

建议 section：
- 为什么这一课重要
- 课程全景与关键问题
- 核心概念、机制、方法或模型
- 公式、图表、代码、数据或其他关键表征
- 易错点
- 复习与自测

强制覆盖：
- `coverage checklist` 中的关键覆盖项
- `source outline` 中的核心主题与子主题
- `lesson map` 中体现的重点、依赖关系与 step 落点
- 所有关键 coverage item 都必须进入 `coverageTrace`，不能只靠正文隐含覆盖

源材料：

<COVERAGE_CHECKLIST>
# L10: Machine learning use cases in algorithmic trading
Course Code: COMP7415
# Agenda
- Major categories of machine learning algorithms
- Multiple use cases of machine learning in algo-trading
- The future of smart investment systems
# Introduction to Machine Learning
# What is Machine Learning?
- A subset of artificial intelligence (AI) that enables systems to learn from data, identify patterns, and make decisions with minimal human intervention.
Key Characteristics:
Data-driven
- Adaptive learning
# Types of Machine Learning
1. Supervised Learning
2. Unsupervised Learning
3. Reinforcement Learning
# 1. Supervised Learning
- Involves training a model on a labelled dataset where the outcome is known.
- Two main category of algorithms
1. Classification
- predict categorical target variables
• Eg. SVM, Logistic Regression, Decision Tree, CNN, etc
2. Regression
- predict continuous target
- Eg. Linear regression, decision tree, random forest, etc
![](images/2c7f41adf6657f9c556f1952ede0d622004cae58e8658435abe8f3f9a25cb776.jpg)
# 1. Supervised Learning
# Pros:
- Typically delivers high accuracy if the training data is representative and of good quality.
- The presence of labelled data provides a clear objective for the model, making it easier to evaluate performance.
- Model Interpretability: Many supervised learning algorithms offer insights into feature importance and decision-making processes.
# Cons:
- Requires a large amount of labelled data, which can be expensive and time-consuming to obtain.
- The model can only predict outcomes it has been trained on, making it ineffective for novel or unrepresented scenarios.
# 2. Unsupervised Learning
- Discover patterns and relationship in data without labelling
- Two main category of algorithms
1. Clustering
- group data points into clusters based on their similarity
- Eg. K-mean clustering, PCA, etc
2. Association
- identifies rules that indicate the presence of one item implies the presence of another item
- Eg. Apriori algorithm, Eclat algorithm, etc
![](images/841e78a2dd1f0d08deed091369328a310cbb529bf0e4ab734e4ee13bda407130.jpg)
Association Rule Learning
![](images/87966d3f21cafb5bfdd01a0130ef5f8b040d729f4279727210ce3927cd75ece5.jpg)
"93% of people who purchased item A also purchased item B"
# 2. Unsupervised Learning
# Pros:
- Can work with unlabelled data, making it easier to gather and analyze large datasets without the need for extensive labelling.
- Capable of uncovering complex structures and relationships in data that may not be apparent, leading to new insights.
# - Cons:
- Results can be difficult to interpret, as there are no clear targets or labels to guide understanding of the outcomes.
- May not yield as high predictive accuracy as supervised learning methods, especially for specific tasks like classification.
- Without labelled data, there's a risk of identifying patterns that are not meaningful or relevant, leading to incorrect conclusions.
# 3. Reinforcement Learning
- An agent interacts with the environment by producing actions and discovering errors, and learns to make decisions by maximizing cumulative reward.
- Common algorithms:
Q-learning
SARSA (State-Action-Reward-State-Action)
</COVERAGE_CHECKLIST>

<SOURCE_OUTLINE>
# L10: Machine learning use cases in algorithmic trading
Course Code: COMP7415
# Agenda
- Major categories of machine learning algorithms
- Multiple use cases of machine learning in algo-trading
- The future of smart investment systems
# Introduction to Machine Learning
# What is Machine Learning?
- A subset of artificial intelligence (AI) that enables systems to learn from data, identify patterns, and make decisions with minimal human intervention.
Key Characteristics:
Data-driven
- Adaptive learning
# Types of Machine Learning
1. Supervised Learning
2. Unsupervised Learning
3. Reinforcement Learning
# 1. Supervised Learning
- Involves training a model on a labelled dataset where the outcome is known.
- Two main category of algorithms
1. Classification
- predict categorical target variables
• Eg. SVM, Logistic Regression, Decision Tree, CNN, etc
2. Regression
- predict continuous target
- Eg. Linear regression, decision tree, random forest, etc
![](images/2c7f41adf6657f9c556f1952ede0d622004cae58e8658435abe8f3f9a25cb776.jpg)
# 1. Supervised Learning
# Pros:
- Typically delivers high accuracy if the training data is representative and of good quality.
- The presence of labelled data provides a clear objective for the model, making it easier to evaluate performance.
- Model Interpretability: Many supervised learning algorithms offer insights into feature importance and decision-making processes.
# Cons:
- Requires a large amount of labelled data, which can be expensive and time-consuming to obtain.
- The model can only predict outcomes it has been trained on, making it ineffective for novel or unrepresented scenarios.
# 2. Unsupervised Learning
- Discover patterns and relationship in data without labelling
- Two main category of algorithms
1. Clustering
- group data points into clusters based on their similarity
- Eg. K-mean clustering, PCA, etc
2. Association
- identifies rules that indicate the presence of one item implies the presence of another item
- Eg. Apriori algorithm, Eclat algorithm, etc
![](images/841e78a2dd1f0d08deed091369328a310cbb529bf0e4ab734e4ee13bda407130.jpg)
Association Rule Learning
![](images/87966d3f21cafb5bfdd01a0130ef5f8b040d729f4279727210ce3927cd75ece5.jpg)
"93% of people who purchased item A also purchased item B"
# 2. Unsupervised Learning
# Pros:
- Can work with unlabelled data, making it easier to gather and analyze large datasets without the need for extensive labelling.
- Capable of uncovering complex structures and relationships in data that may not be apparent, leading to new insights.
# - Cons:
- Results can be difficult to interpret, as there are no clear targets or labels to guide understanding of the outcomes.
- May not yield as high predictive accuracy as supervised learning methods, especially for specific tasks like classification.
- Without labelled data, there's a risk of identifying patterns that are not meaningful or relevant, leading to incorrect conclusions.
# 3. Reinforcement Learning
- An agent interacts with the environment by producing actions and discovering errors, and learns to make decisions by maximizing cumulative reward.
- Common algorithms:
Q-learning
SARSA (State-Action-Reward-State-Action)
</SOURCE_OUTLINE>

<LESSON_MAP>
{
  "lesson_id": "L10",
  "mode": "guided_story",
  "steps": [
    {
      "concept": "机器学习三大类型概览",
      "file": "research/pipeline/3-guided_story/L10/step1/step.json",
      "sequence_id": "step1"
    },
    {
      "concept": "监督学习：分类与回归",
      "file": "research/pipeline/3-guided_story/L10/step2/step.json",
      "sequence_id": "step2"
    },
    {
      "concept": "无监督学习：聚类与关联",
      "file": "research/pipeline/3-guided_story/L10/step3/step.json",
      "sequence_id": "step3"
    },
    {
      "concept": "强化学习：Q学习与交易",
      "file": "research/pipeline/3-guided_story/L10/step4/step.json",
      "sequence_id": "step4"
    },
    {
      "concept": "监督学习在交易中的应用：从规则到数据",
      "file": "research/pipeline/3-guided_story/L10/step5/step.json",
      "sequence_id": "step5"
    },
    {
      "concept": "无监督学习在交易中的应用：股票聚类",
      "file": "research/pipeline/3-guided_story/L10/step6/step.json",
      "sequence_id": "step6"
    },
    {
      "concept": "强化学习在交易中的应用：Q学习交易员",
      "file": "research/pipeline/3-guided_story/L10/step7/step.json",
      "sequence_id": "step7"
    },
    {
      "concept": "挑战与未来：AI交易的边界",
      "file": "research/pipeline/3-guided_story/L10/step8/step.json",
      "sequence_id": "step8"
    }
  ]
}

</LESSON_MAP>

<GUIDED_STORY_MANIFEST>
{
  "lesson_id": "L10",
  "mode": "guided_story",
  "steps": [
    {
      "concept": "机器学习三大类型概览",
      "file": "research/pipeline/3-guided_story/L10/step1/step.json",
      "sequence_id": "step1"
    },
    {
      "concept": "监督学习：分类与回归",
      "file": "research/pipeline/3-guided_story/L10/step2/step.json",
      "sequence_id": "step2"
    },
    {
      "concept": "无监督学习：聚类与关联",
      "file": "research/pipeline/3-guided_story/L10/step3/step.json",
      "sequence_id": "step3"
    },
    {
      "concept": "强化学习：Q学习与交易",
      "file": "research/pipeline/3-guided_story/L10/step4/step.json",
      "sequence_id": "step4"
    },
    {
      "concept": "监督学习在交易中的应用：从规则到数据",
      "file": "research/pipeline/3-guided_story/L10/step5/step.json",
      "sequence_id": "step5"
    },
    {
      "concept": "无监督学习在交易中的应用：股票聚类",
      "file": "research/pipeline/3-guided_story/L10/step6/step.json",
      "sequence_id": "step6"
    },
    {
      "concept": "强化学习在交易中的应用：Q学习交易员",
      "file": "research/pipeline/3-guided_story/L10/step7/step.json",
      "sequence_id": "step7"
    },
    {
      "concept": "挑战与未来：AI交易的边界",
      "file": "research/pipeline/3-guided_story/L10/step8/step.json",
      "sequence_id": "step8"
    }
  ]
}

</GUIDED_STORY_MANIFEST>

<GUIDED_STORY_STEPS>
[
  {
    "lesson_id": "L10",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [
          "machine_learning"
        ],
        "id": "s001",
        "introduced_terms": [
          "machine_learning"
        ],
        "lines": [
          "想象一下，你有一个能自己从数据中学习的助手。",
          "这就是<term id=\"machine_learning\">机器学习</term>的核心想法。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "machine_learning"
        ],
        "id": "s002",
        "introduced_terms": [],
        "lines": [
          "根据学习方式的不同，机器学习主要分为三大类。",
          "每一类都有自己擅长的任务。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "supervised_learning"
        ],
        "id": "s003",
        "introduced_terms": [
          "supervised_learning"
        ],
        "lines": [
          "第一类：<term id=\"supervised_learning\">监督学习</term>。",
          "就像有老师给你标准答案，你学会后去预测新问题。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "unsupervised_learning"
        ],
        "id": "s004",
        "introduced_terms": [
          "unsupervised_learning"
        ],
        "lines": [
          "第二类：<term id=\"unsupervised_learning\">无监督学习</term>。",
          "没有标准答案，全靠自己发现数据中的规律。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "reinforcement_learning"
        ],
        "id": "s005",
        "introduced_terms": [
          "reinforcement_learning"
        ],
        "lines": [
          "第三类：<term id=\"reinforcement_learning\">强化学习</term>。",
          "像一个玩家，通过不断试错来学习最优策略。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 0,
          "explanation": "预测具体数值（如价格）属于回归任务，通常使用监督学习。",
          "kind": "single_choice",
          "options": [
            "监督学习",
            "无监督学习",
            "强化学习"
          ],
          "prompt": "哪种机器学习方法最适合用来预测明天的股票价格？"
        },
        "focus_terms": [
          "supervised_learning"
        ],
        "id": "s006",
        "introduced_terms": [],
        "lines": [
          "哪种机器学习方法最适合用来预测明天的股票价格？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step1",
    "source": {
      "plain_text": "Machine learning is a subset of AI that enables systems to learn from data. Three main types: supervised, unsupervised, reinforcement learning.",
      "related": [
        "L10_ML_types"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "machine_learning": {
        "aliases": [
          "Machine Learning",
          "ML"
        ],
        "display": "机器学习",
        "gloss": "让系统从数据中学习模式并做出决策的技术。"
      },
      "reinforcement_learning": {
        "aliases": [
          "Reinforcement Learning",
          "RL"
        ],
        "display": "强化学习",
        "gloss": "智能体通过与环境互动，以最大化累积奖励来学习决策。"
      },
      "supervised_learning": {
        "aliases": [
          "Supervised Learning"
        ],
        "display": "监督学习",
        "gloss": "使用有标签的数据训练模型，预测已知类型的结果。"
      },
      "unsupervised_learning": {
        "aliases": [
          "Unsupervised Learning"
        ],
        "display": "无监督学习",
        "gloss": "在没有标签的数据中发现隐藏的模式和结构。"
      }
    }
  },
  {
    "lesson_id": "L10",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [
          "classification",
          "regression"
        ],
        "id": "s007",
        "introduced_terms": [
          "classification",
          "regression"
        ],
        "lines": [
          "监督学习内部又分为两大任务。",
          "一个是<term id=\"classification\">分类</term>，一个是<term id=\"regression\">回归</term>。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "classification"
        ],
        "id": "s008",
        "introduced_terms": [],
        "lines": [
          "分类：预测结果是类别。",
          "比如判断明天股价是“涨”还是“跌”。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "regression"
        ],
        "id": "s009",
        "introduced_terms": [],
        "lines": [
          "回归：预测结果是连续数值。",
          "比如预测明天收盘价是 150.5 元。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "判断邮件类别是典型的分类问题。预测股价和销售额是回归问题。",
          "kind": "single_choice",
          "options": [
            "预测明天苹果公司的股价",
            "判断一封邮件是垃圾邮件还是正常邮件",
            "预测下个月的销售额"
          ],
          "prompt": "以下哪个是分类问题？"
        },
        "focus_terms": [
          "classification"
        ],
        "id": "s010",
        "introduced_terms": [],
        "lines": [
          "以下哪个是分类问题？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step2",
    "source": {
      "plain_text": "Supervised learning has two main categories: classification (predict categorical variables) and regression (predict continuous variables).",
      "related": [
        "L10_supervised"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "classification": {
        "aliases": [
          "Classification"
        ],
        "display": "分类",
        "gloss": "预测一个离散的类别，比如涨或跌。"
      },
      "regression": {
        "aliases": [
          "Regression"
        ],
        "display": "回归",
        "gloss": "预测一个连续的数值，比如具体价格。"
      }
    }
  },
  {
    "lesson_id": "L10",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [
          "clustering",
          "association"
        ],
        "id": "s011",
        "introduced_terms": [
          "clustering",
          "association"
        ],
        "lines": [
          "无监督学习没有标准答案，但能发现有趣的结构。",
          "它的两大任务是<term id=\"clustering\">聚类</term>和<term id=\"association\">关联规则</term>。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "clustering"
        ],
        "id": "s012",
        "introduced_terms": [],
        "lines": [
          "聚类：把相似的东西自动归为一类。",
          "比如，把走势相似的股票分到同一个组里。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "association"
        ],
        "id": "s013",
        "introduced_terms": [],
        "lines": [
          "关联规则：发现“买了A的人通常也会买B”。",
          "比如，93%买了某只科技股的人也买了另一只。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "k_means"
        ],
        "id": "s014",
        "introduced_terms": [
          "k_means"
        ],
        "lines": [
          "一个非常经典的聚类算法是<term id=\"k_means\">K均值聚类</term>。",
          "它的目标是把数据分成K个组，让组内数据尽量相似。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "k_means"
        ],
        "id": "s015",
        "introduced_terms": [],
        "lines": [
          "K均值的工作原理很简单：",
          "先随机选K个中心点，然后反复调整，直到分组稳定。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answers": [
            "K",
            "k",
            "簇的数量",
            "分组数"
          ],
          "explanation": "K均值需要用户指定要分成几个簇，即K值。",
          "kind": "fill_in_blank",
          "prompt": "K均值聚类需要用户提前指定什么参数？"
        },
        "focus_terms": [
          "k_means"
        ],
        "id": "s016",
        "introduced_terms": [],
        "lines": [
          "K均值聚类需要用户提前指定什么参数？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step3",
    "source": {
      "plain_text": "Unsupervised learning includes clustering (group similar data) and association (find rules). K-Means is a popular clustering algorithm.",
      "related": [
        "L10_unsupervised"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "association": {
        "aliases": [
          "Association Rule Learning"
        ],
        "display": "关联规则",
        "gloss": "发现数据中“如果A发生，B也经常发生”的规律。"
      },
      "clustering": {
        "aliases": [
          "Clustering"
        ],
        "display": "聚类",
        "gloss": "将数据点根据相似度自动分组。"
      },
      "k_means": {
        "aliases": [
          "K-Means",
          "K-Means Clustering"
        ],
        "display": "K均值聚类",
        "gloss": "一种常用的聚类算法，通过迭代优化将数据分成K个组。"
      }
    }
  },
  {
    "lesson_id": "L10",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [
          "reinforcement_learning"
        ],
        "id": "s017",
        "introduced_terms": [],
        "lines": [
          "强化学习里，一个智能体在环境中学习。",
          "它通过尝试不同的动作，来获得最大的累积奖励。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "state",
          "action"
        ],
        "id": "s018",
        "introduced_terms": [
          "state",
          "action"
        ],
        "lines": [
          "想象一个交易机器人：",
          "它看到市场<term id=\"state\">状态</term>，然后决定<term id=\"action\">动作</term>（买/卖/持有）。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "reward"
        ],
        "id": "s019",
        "introduced_terms": [
          "reward"
        ],
        "lines": [
          "如果这个动作让它赚钱了，它就得到一个正<term id=\"reward\">奖励</term>。",
          "如果亏钱了，就得到负奖励。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "q_learning"
        ],
        "id": "s020",
        "introduced_terms": [
          "q_learning"
        ],
        "lines": [
          "一个经典的强化学习算法是<term id=\"q_learning\">Q学习</term>。",
          "它维护一张Q值表，记录在每个状态下做每个动作的“好坏”。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "q_learning"
        ],
        "id": "s021",
        "introduced_terms": [],
        "lines": [
          "Q学习的更新公式：",
          "新Q值 = 旧Q值 + 学习率 × (奖励 + 折扣因子 × 未来最佳Q值 - 旧Q值)"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "Q(s, a) 表示在状态s下采取动作a后，所能获得的预期累积奖励。",
          "kind": "single_choice",
          "options": [
            "当前状态的好坏",
            "在某个状态下采取某个动作的预期未来总奖励",
            "立即获得的奖励"
          ],
          "prompt": "在Q学习中，Q值代表什么？"
        },
        "focus_terms": [
          "q_learning"
        ],
        "id": "s022",
        "introduced_terms": [],
        "lines": [
          "在Q学习中，Q值代表什么？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step4",
    "source": {
      "plain_text": "Reinforcement learning: agent learns by interacting with environment. Q-Learning is a key algorithm. Key components: state, action, reward.",
      "related": [
        "L10_rl"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "action": {
        "aliases": [
          "Action",
          "a"
        ],
        "display": "动作",
        "gloss": "智能体可以执行的操作。"
      },
      "q_learning": {
        "aliases": [
          "Q-Learning"
        ],
        "display": "Q学习",
        "gloss": "一种无模型的强化学习算法，通过Q值表学习最优策略。"
      },
      "reward": {
        "aliases": [
          "Reward",
          "r"
        ],
        "display": "奖励",
        "gloss": "执行动作后环境给予的即时反馈信号。"
      },
      "state": {
        "aliases": [
          "State",
          "s"
        ],
        "display": "状态",
        "gloss": "环境在某一时刻的描述。"
      }
    }
  },
  {
    "lesson_id": "L10",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [
          "trading_strategy",
          "rsi"
        ],
        "id": "s023",
        "introduced_terms": [
          "trading_strategy",
          "rsi"
        ],
        "lines": [
          "传统<term id=\"trading_strategy\">交易策略</term>是规则驱动的。",
          "比如：如果<term id=\"rsi\">相对强弱指标</term>低于30就买入，高于70就卖出。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "supervised_learning"
        ],
        "id": "s024",
        "introduced_terms": [],
        "lines": [
          "但机器学习的方法不同。",
          "它是结果驱动的：先标记出历史上哪些交易是好的，然后让模型自己学习规则。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "supervised_learning"
        ],
        "id": "s025",
        "introduced_terms": [],
        "lines": [
          "比如，你告诉模型：",
          "“在这些价格点买入是好的，在这些点卖出是好的。”"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "supervised_learning"
        ],
        "id": "s026",
        "introduced_terms": [],
        "lines": [
          "模型就会自己学习，什么样的市场特征会导致好的交易。",
          "它不再依赖固定的RSI阈值，而是从数据中发现隐藏的规律。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "传统策略由人定义规则，AI策略从标记好的数据中自动学习规则。",
          "kind": "single_choice",
          "options": [
            "传统策略更赚钱",
            "传统策略是规则驱动，AI策略是数据驱动",
            "AI策略不需要数据"
          ],
          "prompt": "传统交易策略和AI交易策略的主要区别是什么？"
        },
        "focus_terms": [
          "trading_strategy"
        ],
        "id": "s027",
        "introduced_terms": [],
        "lines": [
          "传统交易策略和AI交易策略的主要区别是什么？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step5",
    "source": {
      "plain_text": "Traditional trading strategy: define rules (e.g., RSI). AI approach: learn rules from data by labeling desired trades.",
      "related": [
        "L10_supervised_trading"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "rsi": {
        "aliases": [
          "RSI",
          "Relative Strength Index"
        ],
        "display": "相对强弱指标",
        "gloss": "一种技术指标，用于衡量价格变动的速度和变化，判断超买或超卖。"
      },
      "trading_strategy": {
        "aliases": [
          "Trading Strategy"
        ],
        "display": "交易策略",
        "gloss": "决定何时买卖、买卖多少的规则集合。"
      }
    }
  },
  {
    "lesson_id": "L10",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [
          "clustering"
        ],
        "id": "s028",
        "introduced_terms": [],
        "lines": [
          "无监督学习在交易中的一个重要应用是股票聚类。",
          "把走势相似的股票自动分到一组。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "k_means"
        ],
        "id": "s029",
        "introduced_terms": [],
        "lines": [
          "比如，用K均值聚类把7只股票分成2组。",
          "一组可能是科技股（微软、谷歌、苹果），另一组是银行股（摩根大通、美国银行）。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "euclidean_distance",
          "cosine_distance"
        ],
        "id": "s030",
        "introduced_terms": [
          "euclidean_distance",
          "cosine_distance"
        ],
        "lines": [
          "聚类时，需要定义“相似度”。",
          "常用的度量有<term id=\"euclidean_distance\">欧几里得距离</term>和<term id=\"cosine_distance\">余弦距离</term>。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "euclidean_distance",
          "cosine_distance"
        ],
        "id": "s031",
        "introduced_terms": [],
        "lines": [
          "欧几里得距离：直接计算两条收益率曲线之间的“直线”距离。",
          "余弦距离：更关注两条曲线的形状是否相似，而不是它们的绝对值。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "余弦距离关注方向（形状），对幅度不敏感，更适合这种情况。",
          "kind": "single_choice",
          "options": [
            "欧几里得距离",
            "余弦距离",
            "两者一样"
          ],
          "prompt": "如果两只股票的收益率曲线形状非常相似，但幅度不同，哪种距离度量更合适？"
        },
        "focus_terms": [
          "cosine_distance"
        ],
        "id": "s032",
        "introduced_terms": [],
        "lines": [
          "如果两只股票的收益率曲线形状非常相似，但幅度不同，哪种距离度量更合适？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step6",
    "source": {
      "plain_text": "Use K-Means to cluster stocks based on historical returns. Distance measures: Euclidean, Cosine, Correlation-based.",
      "related": [
        "L10_stock_clustering"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "cosine_distance": {
        "aliases": [
          "Cosine Distance"
        ],
        "display": "余弦距离",
        "gloss": "基于向量夹角的距离度量，关注方向而非大小。"
      },
      "euclidean_distance": {
        "aliases": [
          "Euclidean Distance"
        ],
        "display": "欧几里得距离",
        "gloss": "两点之间的直线距离，用于衡量相似度。"
      }
    }
  },
  {
    "lesson_id": "L10",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [
          "q_learning"
        ],
        "id": "s033",
        "introduced_terms": [],
        "lines": [
          "现在，把Q学习应用到交易中。",
          "状态：当前的市场情况（价格、成交量等）。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "q_learning"
        ],
        "id": "s034",
        "introduced_terms": [],
        "lines": [
          "动作：只有三个选择——买入、卖出、持有。",
          "奖励：这次交易赚了还是亏了。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "exploration_exploitation"
        ],
        "id": "s035",
        "introduced_terms": [
          "exploration_exploitation"
        ],
        "lines": [
          "Q学习智能体通过反复模拟交易来学习。",
          "它会在<term id=\"exploration_exploitation\">探索与利用</term>之间平衡。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "q_learning"
        ],
        "id": "s036",
        "introduced_terms": [],
        "lines": [
          "刚开始，它会随机探索各种交易。",
          "慢慢地，它会学会在什么状态下买入、什么状态下卖出最赚钱。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 0,
          "explanation": "初期需要更多探索，以发现各种可能的交易策略。",
          "kind": "single_choice",
          "options": [
            "探索",
            "利用",
            "两者一样"
          ],
          "prompt": "Q学习交易员在训练初期应该更偏向探索还是利用？"
        },
        "focus_terms": [
          "exploration_exploitation"
        ],
        "id": "s037",
        "introduced_terms": [],
        "lines": [
          "Q学习交易员在训练初期应该更偏向探索还是利用？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step7",
    "source": {
      "plain_text": "Apply Q-Learning to trading: define state (market conditions), action (buy/sell/hold), reward (profit/loss).",
      "related": [
        "L10_rl_trading"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "exploration_exploitation": {
        "aliases": [
          "Exploration vs Exploitation"
        ],
        "display": "探索与利用",
        "gloss": "在尝试新动作（探索）和选择已知最佳动作（利用）之间平衡。"
      }
    }
  },
  {
    "lesson_id": "L10",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [
          "non_stationary"
        ],
        "id": "s038",
        "introduced_terms": [
          "non_stationary"
        ],
        "lines": [
          "AI交易虽然强大，但也有自己的局限。",
          "首先，金融市场是<term id=\"non_stationary\">非平稳</term>的。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "non_stationary"
        ],
        "id": "s039",
        "introduced_terms": [],
        "lines": [
          "市场规律会变，去年有效的策略今年可能就失效了。",
          "模型需要不断适应，而不是学一次就一劳永逸。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "overfitting"
        ],
        "id": "s040",
        "introduced_terms": [
          "overfitting"
        ],
        "lines": [
          "其次，<term id=\"overfitting\">过拟合</term>是个大问题。",
          "模型可能在历史数据上表现完美，但一到实盘就亏钱。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "reinforcement_learning"
        ],
        "id": "s041",
        "introduced_terms": [],
        "lines": [
          "还有，交易的状态空间非常巨大。",
          "不像下围棋，市场有无数种可能的状态和动作。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s042",
        "introduced_terms": [],
        "lines": [
          "所以，未来的方向不是AI完全取代人类。",
          "而是人机协同：AI提供建议，人类做最终判断。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "explanation": "因为历史数据中的噪声和特定模式可能被模型当作普遍规律学习，导致在新数据上失效。",
          "kind": "short_reflection",
          "prompt": "为什么AI交易模型容易过拟合？"
        },
        "focus_terms": [
          "overfitting"
        ],
        "id": "s043",
        "introduced_terms": [],
        "lines": [
          "为什么AI交易模型容易过拟合？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step8",
    "source": {
      "plain_text": "Limitations of Q-learning in trading: high dimensionality, non-stationary environment, overfitting risks. Future: AI + human synthesis.",
      "related": [
        "L10_limitations_future"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "non_stationary": {
        "aliases": [
          "Non-Stationary"
        ],
        "display": "非平稳性",
        "gloss": "环境的统计特性随时间变化，比如市场规律会变。"
      },
      "overfitting": {
        "aliases": [
          "Overfitting"
        ],
        "display": "过拟合",
        "gloss": "模型在训练数据上表现很好，但在新数据上表现很差。"
      }
    }
  }
]

</GUIDED_STORY_STEPS>

<QUESTION_BANK>
[
{
  "coverage_map": [
    {
      "coverage_tag": "limitations_of_ai_trading",
      "covered_by": [
        "qf_flash_limitations_v1",
        "qf_flash_limitations_v2",
        "qf_flash_limitations_v3",
        "qf_quiz_limitations_v1",
        "qf_quiz_limitations_v2",
        "qf_long_future_v1",
        "qf_long_future_v2"
      ],
      "description": "AI交易的主要局限：非平稳市场、过拟合、高维状态空间"
    },
    {
      "coverage_tag": "non_stationary_market",
      "covered_by": [
        "qf_flash_limitations_v1",
        "qf_quiz_limitations_v1"
      ],
      "description": "金融市场的非平稳性：市场规律随时间变化，模型需要持续适应"
    },
    {
      "coverage_tag": "overfitting_risk",
      "covered_by": [
        "qf_flash_limitations_v2",
        "qf_quiz_limitations_v2"
      ],
      "description": "过拟合风险：模型在历史数据上表现完美，但在实盘中失效"
    },
    {
      "coverage_tag": "high_dimensionality",
      "covered_by": [
        "qf_flash_limitations_v3"
      ],
      "description": "高维状态空间：交易环境的状态和动作空间巨大，增加学习难度"
    },
    {
      "coverage_tag": "future_human_ai_synthesis",
      "covered_by": [
        "qf_long_future_v1",
        "qf_long_future_v2"
      ],
      "description": "未来方向：人机协同，AI提供建议，人类做最终判断"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "non_stationary_market",
      "coverage_tags": [
        "limitations_of_ai_trading",
        "non_stationary_market"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_limitations_v1",
      "learning_goal": "学生能解释金融市场非平稳性的含义及其对AI交易模型的影响。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "非平稳性的定义及其对交易策略有效性的影响。",
      "term_refs": [
        {
          "display": "非平稳性",
          "en": "Non-Stationary"
        }
      ],
      "variants": [
        {
          "back": "市场的统计特性（如规律、模式）会随时间变化。",
          "estimated_seconds": 8,
          "explanation": "非平稳性意味着过去有效的策略可能在未来失效，模型需要持续适应。",
          "front": "金融市场被称为“非平稳”环境，这意味着什么？",
          "question_id": "q_flash_limitations_v1_v1"
        },
        {
          "back": "非平稳性。",
          "estimated_seconds": 6,
          "explanation": "非平稳性意味着市场规律会变，模型不能一劳永逸。",
          "front": "如果去年有效的交易策略今年失效了，这反映了金融市场的什么特性？",
          "question_id": "q_flash_limitations_v1_v2"
        }
      ]
    },
    {
      "concept_key": "overfitting_risk",
      "coverage_tags": [
        "limitations_of_ai_trading",
        "overfitting_risk"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_limitations_v2",
      "learning_goal": "学生能定义过拟合，并说明其在AI交易中的具体表现。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "过拟合的定义及其在交易中的典型表现。",
      "term_refs": [
        {
          "display": "过拟合",
          "en": "Overfitting"
        }
      ],
      "variants": [
        {
          "back": "模型在历史数据上表现完美，但在新数据（实盘）上表现很差。",
          "estimated_seconds": 8,
          "explanation": "过拟合的模型学到了历史数据中的噪声而非普遍规律。",
          "front": "在AI交易中，“过拟合”指的是什么现象？",
          "question_id": "q_flash_limitations_v2_v1"
        },
        {
          "back": "过拟合。",
          "estimated_seconds": 6,
          "explanation": "过拟合导致模型无法泛化到新的市场数据。",
          "front": "一个AI交易模型在回测中收益惊人，但实盘交易却持续亏损，这最可能是什么问题？",
          "question_id": "q_flash_limitations_v2_v2"
        }
      ]
    },
    {
      "concept_key": "high_dimensionality",
      "coverage_tags": [
        "limitations_of_ai_trading",
        "high_dimensionality"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_limitations_v3",
      "learning_goal": "学生能理解交易环境的高维状态空间对强化学习模型的挑战。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "高维状态空间对Q学习等强化学习算法的具体挑战。",
      "term_refs": [
        {
          "display": "高维状态空间",
          "en": "High Dimensionality"
        }
      ],
      "variants": [
        {
          "back": "状态空间巨大且非有限，有无数种可能的状态和动作。",
          "estimated_seconds": 10,
          "explanation": "围棋的状态空间虽然大但有限，而市场状态是连续且无限的。",
          "front": "与下围棋相比，金融交易的状态空间有什么特点？",
          "question_id": "q_flash_limitations_v3_v1"
        },
        {
          "back": "需要巨大的计算资源和时间才能有效训练。",
          "estimated_seconds": 8,
          "explanation": "状态和动作空间过大，使得Q表难以收敛。",
          "front": "Q学习在交易中面临“高维状态空间”挑战，这会导致什么问题？",
          "question_id": "q_flash_limitations_v3_v2"
        }
      ]
    }
  ],
  "lesson_id": "L10",
  "longform_families": [
    {
      "concept_key": "future_human_ai_synthesis",
      "coverage_tags": [
        "limitations_of_ai_trading",
        "future_human_ai_synthesis"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_future_v1",
      "learning_goal": "学生能解释为什么AI交易需要人机协同，并阐述各自的角色。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "人机协同",
          "en": "Human-AI Synthesis"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "指出AI交易的两个主要局限性",
            "解释这些局限性如何影响AI的自主决策",
            "说明人类在协同中能发挥什么作用"
          ],
          "question_id": "q_long_future_v1_v1",
          "reference_answer": [
            "AI交易的主要局限包括：1）金融市场是非平稳的，模型学到的规律会随时间失效；2）模型容易过拟合，在历史数据上表现好但实盘可能亏损。",
            "这些局限性意味着AI无法完全自主地应对所有市场变化。",
            "因此，未来的方向是人机协同：AI负责快速分析大量数据、发现模式、提供交易建议，而人类负责最终判断、处理异常情况、管理风险，并不断调整和优化AI模型。"
          ],
          "rubric_points": [
            "正确指出非平稳性或过拟合等局限性（2分）",
            "解释局限性如何导致AI无法完全自主（2分）",
            "说明人类在判断、适应、风险管理等方面的价值（2分）"
          ],
          "stem": "结合本课内容，解释为什么AI交易系统的未来方向是“人机协同”而非“AI完全取代人类”。请至少从两个AI的局限性出发进行论述。"
        },
        {
          "estimated_seconds": 150,
          "prompt_blocks": [
            "指出Q学习模型在交易中的至少两个局限性",
            "解释这些局限性对实盘交易的风险",
            "描述人机协同的具体工作流程"
          ],
          "question_id": "q_long_future_v1_v2",
          "reference_answer": [
            "Q学习模型面临非平稳市场、过拟合和高维状态空间等挑战，完全依赖它风险很高。",
            "例如，模型可能在回测中表现优异，但市场风格切换后就会失效。",
            "我会设计一个人机协同流程：AI模型每天生成交易信号和风险评估报告，交易员团队审核这些信号，结合市场新闻和宏观判断做出最终决策。同时，定期用最新数据重新训练模型，并监控模型的实时表现，一旦出现异常就人工介入。"
          ],
          "rubric_points": [
            "正确指出Q学习的局限性（如非平稳性、过拟合、高维状态空间）（2分）",
            "解释这些局限性带来的实盘风险（2分）",
            "设计合理的人机协同流程，明确AI和人类的角色（2分）"
          ],
          "stem": "假设你是一个量化基金的管理者，你的团队开发了一个基于Q学习的交易模型。请解释为什么你不能完全依赖这个模型，并说明你将如何设计一个人机协同的工作流程。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "non_stationary_market",
      "coverage_tags": [
        "limitations_of_ai_trading",
        "non_stationary_market"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_limitations_v1",
      "learning_goal": "学生能辨析非平稳性对AI交易模型的具体影响。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "非平稳性",
          "en": "Non-Stationary"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "非平稳性意味着市场规律会变，模型必须持续适应，否则策略会失效。",
          "options": [
            "模型需要处理大量历史数据",
            "模型学到的规律可能随时失效，需要持续更新",
            "模型无法处理高频交易数据",
            "模型需要更复杂的神经网络结构"
          ],
          "question_id": "q_quiz_limitations_v1_v1",
          "stem": "以下哪一项最能说明金融市场的“非平稳性”对AI交易模型的挑战？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "市场风格切换体现了环境的非平稳性，导致之前学到的策略不再适用。",
          "options": [
            "过拟合",
            "非平稳性",
            "高维状态空间",
            "奖励函数设计不当"
          ],
          "question_id": "q_quiz_limitations_v1_v2",
          "stem": "一个Q学习交易员在2023年表现优异，但在2024年市场风格切换后表现糟糕。这最直接地反映了什么问题？"
        }
      ]
    },
    {
      "concept_key": "overfitting_risk",
      "coverage_tags": [
        "limitations_of_ai_trading",
        "overfitting_risk"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_limitations_v2",
      "learning_goal": "学生能识别过拟合在交易模型中的表现及其原因。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "过拟合",
          "en": "Overfitting"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "过拟合的典型特征是训练集表现好，但无法泛化到新数据。",
          "options": [
            "模型在训练集和测试集上的表现一致",
            "模型在历史数据上收益极高，但在模拟交易中表现平平",
            "模型在训练集上收益极高，但在实盘交易中持续亏损",
            "模型在训练集上收益较低，但在实盘交易中表现良好"
          ],
          "question_id": "q_quiz_limitations_v2_v1",
          "stem": "以下哪个现象最能表明一个AI交易模型可能过拟合了？"
        },
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "模型可能过度学习历史数据中的噪声和偶然模式，导致在新数据上失效。",
          "options": [
            "因为市场数据太少",
            "因为模型结构太简单",
            "因为历史数据中的噪声和特定模式可能被模型当作普遍规律学习",
            "因为交易策略本身就不稳定"
          ],
          "question_id": "q_quiz_limitations_v2_v2",
          "stem": "为什么AI交易模型容易过拟合？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L10: Machine learning use cases in algorithmic trading - Agenda: Major categories of ML algorithms, Multiple use cases of ML in algo-trading, The future of smart investment systems",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "step8: 挑战与未来：AI交易的边界",
    "plain_text": "pipeline/1-plain/L10/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L10: Machine learning use cases in algorithmic trading - Agenda: Major categories of ML algorithms, Multiple use cases of ML in algo-trading, The future of smart investment systems"
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "machine_learning_definition",
      "covered_by": [
        "qf_flash_ml_def",
        "qf_quiz_ml_def"
      ],
      "description": "机器学习的定义：AI的子集，从数据中学习模式并做出决策"
    },
    {
      "coverage_tag": "ml_three_types_overview",
      "covered_by": [
        "qf_flash_three_types",
        "qf_quiz_three_types",
        "qf_long_three_types_compare"
      ],
      "description": "机器学习三大类型：监督学习、无监督学习、强化学习"
    },
    {
      "coverage_tag": "supervised_learning_intro",
      "covered_by": [
        "qf_flash_supervised_intro",
        "qf_quiz_supervised_use_case"
      ],
      "description": "监督学习：有标签数据，像有老师指导"
    },
    {
      "coverage_tag": "unsupervised_learning_intro",
      "covered_by": [
        "qf_flash_unsupervised_intro",
        "qf_quiz_unsupervised_use_case"
      ],
      "description": "无监督学习：无标签数据，自己发现规律"
    },
    {
      "coverage_tag": "reinforcement_learning_intro",
      "covered_by": [
        "qf_flash_rl_intro",
        "qf_quiz_rl_use_case"
      ],
      "description": "强化学习：智能体通过试错与环境互动，最大化累积奖励"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "machine_learning_definition",
      "coverage_tags": [
        "machine_learning_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_ml_def",
      "learning_goal": "学生能准确说出机器学习的定义及其核心特征。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "机器学习的定义和两个关键特征。",
      "term_refs": [
        {
          "display": "机器学习",
          "en": "Machine Learning"
        }
      ],
      "variants": [
        {
          "back": "机器学习是人工智能的一个子集，它使系统能够从数据中学习、识别模式，并在最少人工干预下做出决策。",
          "estimated_seconds": 10,
          "explanation": "核心在于从数据中自动学习，而不是由人显式编程。",
          "front": "什么是机器学习？请用一句话概括其核心思想。",
          "question_id": "q_flash_ml_def_v1"
        },
        {
          "back": "数据驱动（Data-driven）和自适应学习（Adaptive learning）。",
          "estimated_seconds": 8,
          "explanation": "数据驱动意味着模型从数据中学习规律；自适应学习意味着模型能根据新数据调整自身。",
          "front": "机器学习的两大关键特征是什么？",
          "question_id": "q_flash_ml_def_v2"
        }
      ]
    },
    {
      "concept_key": "ml_three_types_overview",
      "coverage_tags": [
        "ml_three_types_overview"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_three_types",
      "learning_goal": "学生能列举机器学习的三大类型，并说出每种类型的核心比喻。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "三种学习类型的名称和核心比喻。",
      "term_refs": [
        {
          "display": "监督学习",
          "en": "Supervised Learning"
        },
        {
          "display": "无监督学习",
          "en": "Unsupervised Learning"
        },
        {
          "display": "强化学习",
          "en": "Reinforcement Learning"
        }
      ],
      "variants": [
        {
          "back": "监督学习、无监督学习和强化学习。",
          "estimated_seconds": 5,
          "explanation": "这是机器学习最基本的分类，根据学习过程中使用的数据和反馈方式划分。",
          "front": "机器学习主要分为哪三大类型？",
          "question_id": "q_flash_three_types_v1"
        },
        {
          "back": "监督学习。",
          "estimated_seconds": 6,
          "explanation": "监督学习使用有标签的数据进行训练，就像老师提供了正确答案。",
          "front": "在机器学习的比喻中，哪种学习方式像“有老师给你标准答案”？",
          "question_id": "q_flash_three_types_v2"
        },
        {
          "back": "强化学习。",
          "estimated_seconds": 6,
          "explanation": "强化学习中的智能体通过与环境互动，根据获得的奖励或惩罚来调整行为。",
          "front": "在机器学习的比喻中，哪种学习方式像“一个玩家通过不断试错来学习最优策略”？",
          "question_id": "q_flash_three_types_v3"
        },
        {
          "back": "无监督学习。",
          "estimated_seconds": 6,
          "explanation": "无监督学习处理的是没有标签的数据，目标是发现数据中隐藏的结构或模式。",
          "front": "在机器学习的比喻中，哪种学习方式像“没有标准答案，全靠自己发现数据中的规律”？",
          "question_id": "q_flash_three_types_v4"
        }
      ]
    },
    {
      "concept_key": "supervised_learning_intro",
      "coverage_tags": [
        "supervised_learning_intro"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_supervised_intro",
      "learning_goal": "学生能说出监督学习的定义和关键特征。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "监督学习的定义和核心特点。",
      "term_refs": [
        {
          "display": "监督学习",
          "en": "Supervised Learning"
        }
      ],
      "variants": [
        {
          "back": "有标签的数据集（labelled dataset）。",
          "estimated_seconds": 5,
          "explanation": "标签数据意味着每个训练样本都已知其对应的正确输出。",
          "front": "监督学习使用什么类型的数据进行训练？",
          "question_id": "q_flash_supervised_intro_v1"
        },
        {
          "back": "学习一个模型，使其能够根据输入数据预测已知类型的输出。",
          "estimated_seconds": 8,
          "explanation": "模型从有标签的数据中学习输入到输出的映射关系。",
          "front": "监督学习的主要目标是什么？",
          "question_id": "q_flash_supervised_intro_v2"
        }
      ]
    },
    {
      "concept_key": "unsupervised_learning_intro",
      "coverage_tags": [
        "unsupervised_learning_intro"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_unsupervised_intro",
      "learning_goal": "学生能说出无监督学习的定义和关键特征。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "无监督学习的定义和核心特点。",
      "term_refs": [
        {
          "display": "无监督学习",
          "en": "Unsupervised Learning"
        }
      ],
      "variants": [
        {
          "back": "没有标签的数据（unlabelled data）。",
          "estimated_seconds": 5,
          "explanation": "无监督学习不需要预先标记好的答案，直接从原始数据中学习。",
          "front": "无监督学习使用什么类型的数据进行训练？",
          "question_id": "q_flash_unsupervised_intro_v1"
        },
        {
          "back": "发现数据中隐藏的模式和结构。",
          "estimated_seconds": 8,
          "explanation": "例如，将相似的数据点分组（聚类）或发现数据项之间的关联规则。",
          "front": "无监督学习的主要目标是什么？",
          "question_id": "q_flash_unsupervised_intro_v2"
        }
      ]
    },
    {
      "concept_key": "reinforcement_learning_intro",
      "coverage_tags": [
        "reinforcement_learning_intro"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_rl_intro",
      "learning_goal": "学生能说出强化学习的定义和核心要素。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "强化学习的定义和核心要素。",
      "term_refs": [
        {
          "display": "强化学习",
          "en": "Reinforcement Learning"
        }
      ],
      "variants": [
        {
          "back": "通过与环境的互动，产生动作并发现错误，从而学习。",
          "estimated_seconds": 8,
          "explanation": "智能体在环境中尝试不同的动作，根据获得的奖励或惩罚来调整策略。",
          "front": "强化学习中的智能体通过什么方式来学习决策？",
          "question_id": "q_flash_rl_intro_v1"
        },
        {
          "back": "最大化累积奖励。",
          "estimated_seconds": 5,
          "explanation": "智能体学习的目标是选择一系列动作，使得长期累积的奖励总和最大。",
          "front": "强化学习的目标是什么？",
          "question_id": "q_flash_rl_intro_v2"
        }
      ]
    }
  ],
  "lesson_id": "L10",
  "longform_families": [
    {
      "concept_key": "ml_three_types_overview",
      "coverage_tags": [
        "ml_three_types_overview"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_three_types_compare",
      "learning_goal": "学生能比较和对比监督学习、无监督学习和强化学习在数据需求、学习方式和目标上的主要区别。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "compare_and_contrast",
      "term_refs": [
        {
          "display": "监督学习",
          "en": "Supervised Learning"
        },
        {
          "display": "无监督学习",
          "en": "Unsupervised Learning"
        },
        {
          "display": "强化学习",
          "en": "Reinforcement Learning"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "数据类型",
            "学习方式",
            "主要目标"
          ],
          "question_id": "q_long_three_types_compare_v1",
          "reference_answer": [
            "监督学习使用有标签的数据集进行训练，学习从输入到已知输出的映射关系，主要目标是预测或分类。",
            "无监督学习使用没有标签的数据，通过发现数据中的模式、结构或关系来学习，主要目标是聚类、降维或关联规则发现。",
            "强化学习不依赖静态数据集，而是让一个智能体（agent）在环境中通过执行动作、接收奖励或惩罚来学习，主要目标是学习一个能最大化长期累积奖励的策略。"
          ],
          "rubric_points": [
            "准确指出监督学习使用有标签数据，无监督学习使用无标签数据，强化学习通过与环境的互动来学习。",
            "解释监督学习是学习输入到输出的映射，无监督学习是发现数据中的隐藏结构，强化学习是通过试错来最大化累积奖励。",
            "能够清晰地对比三者的不同之处。"
          ],
          "stem": "请比较监督学习、无监督学习和强化学习这三种机器学习类型。你的回答应涵盖它们各自使用的数据类型、学习方式（如何从数据中学习）以及主要目标。"
        },
        {
          "estimated_seconds": 150,
          "prompt_blocks": [
            "问题1：预测明天是否会下雨",
            "问题2：将顾客分组",
            "问题3：训练AI下棋"
          ],
          "question_id": "q_long_three_types_compare_v2",
          "reference_answer": [
            "问题1：监督学习。因为我们可以使用历史气象数据（如温度、湿度、气压）作为特征，并以“是否下雨”作为标签来训练一个分类模型。",
            "问题2：无监督学习。因为我们没有预先定义好的顾客类别标签，目标是让算法根据消费数据的相似性自动将顾客分组（聚类）。",
            "问题3：强化学习。因为下棋是一个序列决策问题，AI（智能体）需要通过在当前棋局（状态）下选择走法（动作），并根据输赢（奖励）来学习最优策略。"
          ],
          "rubric_points": [
            "问题1选择监督学习，并解释原因（有历史标签数据，是分类问题）。",
            "问题2选择无监督学习，并解释原因（没有标签，目标是发现结构）。",
            "问题3选择强化学习，并解释原因（需要通过试错和奖励来学习策略）。"
          ],
          "stem": "假设你被要求为以下三个问题选择机器学习方法：1) 预测明天是否会下雨；2) 将一群顾客根据消费习惯分成几个小组；3) 训练一个AI玩国际象棋。请分别说明你会选择哪种机器学习类型（监督、无监督、强化学习），并解释为什么。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "machine_learning_definition",
      "coverage_tags": [
        "machine_learning_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_ml_def",
      "learning_goal": "学生能在不同表述中准确识别机器学习的定义。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "机器学习",
          "en": "Machine Learning"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "机器学习是AI的子集，核心是从数据中学习模式并做出决策，而不是依赖固定的规则。",
          "options": [
            "一种通过预定义规则进行计算的编程方法",
            "人工智能的一个子集，使系统能从数据中学习并做出决策",
            "一种专门用于图像识别的算法",
            "一个存储和处理大数据的数据库系统"
          ],
          "question_id": "q_quiz_ml_def_v1",
          "stem": "以下哪一项最准确地描述了机器学习？"
        },
        {
          "answer": 2,
          "estimated_seconds": 12,
          "explanation": "机器学习是数据驱动的，并且能够自适应地学习，即模型可以根据新数据调整自身。",
          "options": [
            "规则驱动",
            "人工干预",
            "自适应学习",
            "静态模型"
          ],
          "question_id": "q_quiz_ml_def_v2",
          "stem": "机器学习的两个关键特征是数据驱动和什么？"
        }
      ]
    },
    {
      "concept_key": "ml_three_types_overview",
      "coverage_tags": [
        "ml_three_types_overview"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_three_types",
      "learning_goal": "学生能根据任务描述判断应使用哪种机器学习类型。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "监督学习",
          "en": "Supervised Learning"
        },
        {
          "display": "无监督学习",
          "en": "Unsupervised Learning"
        },
        {
          "display": "强化学习",
          "en": "Reinforcement Learning"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "由于历史数据包含“是否违约”的标签，这是一个有监督的分类问题，适合使用监督学习。",
          "options": [
            "无监督学习",
            "强化学习",
            "监督学习",
            "关联规则学习"
          ],
          "question_id": "q_quiz_three_types_v1",
          "stem": "一家银行希望根据历史贷款数据（包含是否违约的标签）来预测新客户的违约风险。这最适合使用哪种机器学习方法？"
        },
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "发现隐藏的关联规则属于无监督学习的范畴，因为它不需要预先标记好的数据。",
          "options": [
            "监督学习",
            "强化学习",
            "无监督学习",
            "回归分析"
          ],
          "question_id": "q_quiz_three_types_v2",
          "stem": "一个电商平台希望发现用户购买行为中的隐藏模式，例如“购买了A商品的用户也经常购买B商品”，而不需要预先知道任何购买组合。这最适合使用哪种机器学习方法？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "通过试错和最大化累积奖励来学习，是强化学习的典型特征。",
          "options": [
            "监督学习",
            "无监督学习",
            "强化学习",
            "聚类分析"
          ],
          "question_id": "q_quiz_three_types_v3",
          "stem": "一个游戏AI需要通过不断尝试不同的操作，并根据游戏得分（奖励）来学习如何通关。这最适合使用哪种机器学习方法？"
        }
      ]
    },
    {
      "concept_key": "supervised_learning_intro",
      "coverage_tags": [
        "supervised_learning_intro"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_supervised_use_case",
      "learning_goal": "学生能识别监督学习的典型应用场景。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "监督学习",
          "en": "Supervised Learning"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "预测一个具体的数值（股票价格）是回归问题，属于监督学习。其他选项分别对应无监督学习（聚类、关联）和强化学习。",
          "options": [
            "将一组新闻文章自动分成几个主题类别",
            "预测明天下午3点的股票价格",
            "训练一个机器人在迷宫中找到出口",
            "发现超市商品之间的购买关联"
          ],
          "question_id": "q_quiz_supervised_use_case_v1",
          "stem": "以下哪个任务最适合使用监督学习？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "课程中明确指出，预测具体数值（如价格）属于回归任务，通常使用监督学习。",
          "options": [
            "无监督学习",
            "强化学习",
            "监督学习",
            "半监督学习"
          ],
          "question_id": "q_quiz_supervised_use_case_v2",
          "stem": "在课程中，预测明天的股票价格被用作哪种学习类型的例子？"
        }
      ]
    },
    {
      "concept_key": "unsupervised_learning_intro",
      "coverage_tags": [
        "unsupervised_learning_intro"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_unsupervised_use_case",
      "learning_goal": "学生能识别无监督学习的典型应用场景。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "无监督学习",
          "en": "Unsupervised Learning"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "将客户分组（客户细分）是典型的聚类问题，属于无监督学习。",
          "options": [
            "根据历史数据预测房价",
            "将客户根据购买行为分成不同的群体",
            "训练一个AI玩星际争霸",
            "识别图片中是否有猫"
          ],
          "question_id": "q_quiz_unsupervised_use_case_v1",
          "stem": "以下哪个任务最适合使用无监督学习？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "无监督学习可以处理无标签数据，省去了昂贵且耗时的人工标注过程。",
          "options": [
            "通常比监督学习更准确",
            "不需要大量有标签的数据",
            "结果总是很容易解释",
            "可以处理任何类型的问题"
          ],
          "question_id": "q_quiz_unsupervised_use_case_v2",
          "stem": "无监督学习的一个主要优势是什么？"
        }
      ]
    },
    {
      "concept_key": "reinforcement_learning_intro",
      "coverage_tags": [
        "reinforcement_learning_intro"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_rl_use_case",
      "learning_goal": "学生能识别强化学习的典型应用场景。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "强化学习",
          "en": "Reinforcement Learning"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "机器人走路需要通过不断试错和接收反馈（如摔倒的惩罚或前进的奖励）来学习，这是强化学习的典型应用。",
          "options": [
            "根据房屋特征预测其价格",
            "将文档自动分类",
            "训练一个机器人学会走路",
            "发现数据中的异常点"
          ],
          "question_id": "q_quiz_rl_use_case_v1",
          "stem": "以下哪个任务最适合使用强化学习？"
        },
        {
          "answer": 1,
          "estimated_seconds": 12,
          "explanation": "奖励是环境对智能体动作的即时反馈，用于指导智能体学习。",
          "options": [
            "模型预测正确的次数",
            "智能体执行动作后环境给予的即时反馈信号",
            "数据集中样本的标签",
            "算法的运行速度"
          ],
          "question_id": "q_quiz_rl_use_case_v2",
          "stem": "强化学习中的“奖励”是什么？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L10: Machine learning use cases in algorithmic trading - Agenda: Major categories of machine learning algorithms, Multiple use cases of machine learning in algo-trading, The future of smart investment systems; Introduction to Machine Learning; What is Machine Learning?; Types of Machine Learning: 1. Supervised Learning, 2. Unsupervised Learning, 3. Reinforcement Learning",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "{\"lesson_id\": \"L10\", \"mode\": \"guided_story\", \"steps\": [{\"concept\": \"机器学习三大类型概览\", \"file\": \"research/pipeline/3-guided_story/L10/step1/step.json\", \"sequence_id\": \"step1\"}, ...]}",
    "plain_text": "pipeline/1-plain/L10/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L10: Machine learning use cases in algorithmic trading - Agenda: Major categories of machine learning algorithms, Multiple use cases of machine learning in algo-trading, The future of smart investment systems; Introduction to Machine Learning; What is Machine Learning?; Types of Machine Learning: 1. Supervised Learning, 2. Unsupervised Learning, 3. Reinforcement Learning"
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "reinforcement_learning_definition",
      "covered_by": [
        "qf_flash_rl_def",
        "qf_quiz_rl_def"
      ],
      "description": "强化学习的核心定义：智能体通过与环境互动，以最大化累积奖励来学习决策。"
    },
    {
      "coverage_tag": "rl_key_components",
      "covered_by": [
        "qf_flash_rl_components",
        "qf_quiz_rl_components"
      ],
      "description": "强化学习的三个关键组件：状态(state)、动作(action)、奖励(reward)。"
    },
    {
      "coverage_tag": "q_learning_definition",
      "covered_by": [
        "qf_flash_q_learning_def",
        "qf_quiz_q_learning_def"
      ],
      "description": "Q学习的定义：一种无模型的强化学习算法，通过Q值表学习最优策略。"
    },
    {
      "coverage_tag": "q_value_meaning",
      "covered_by": [
        "qf_flash_q_value",
        "qf_quiz_q_value"
      ],
      "description": "Q值的含义：Q(s, a) 表示在状态s下采取动作a后，所能获得的预期累积奖励。"
    },
    {
      "coverage_tag": "q_learning_update_formula",
      "covered_by": [
        "qf_flash_q_formula",
        "qf_quiz_q_formula",
        "qf_long_q_formula_apply"
      ],
      "description": "Q学习的更新公式：新Q值 = 旧Q值 + 学习率 × (奖励 + 折扣因子 × 未来最佳Q值 - 旧Q值)。"
    },
    {
      "coverage_tag": "q_learning_steps",
      "covered_by": [
        "qf_flash_q_steps",
        "qf_quiz_q_steps"
      ],
      "description": "Q学习的五个步骤：初始化Q表、选择动作（探索/利用）、执行动作、更新Q值、迭代。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "reinforcement_learning_definition",
      "coverage_tags": [
        "reinforcement_learning_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_rl_def",
      "learning_goal": "学生能准确说出强化学习的核心定义。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "强化学习的核心机制：智能体、环境、互动、累积奖励。",
      "term_refs": [
        {
          "display": "强化学习",
          "en": "Reinforcement Learning"
        }
      ],
      "variants": [
        {
          "back": "通过与环境的互动（试错），以最大化累积奖励为目标。",
          "estimated_seconds": 8,
          "explanation": "强化学习的核心是智能体在环境中尝试动作，根据奖励反馈调整策略。",
          "front": "强化学习中，智能体通过什么方式学习决策？",
          "question_id": "q_flash_rl_def_v1"
        },
        {
          "back": "学习一种策略，使得在与环境互动过程中获得的累积奖励最大化。",
          "estimated_seconds": 8,
          "explanation": "智能体不是为了单次奖励，而是为了长期的总回报。",
          "front": "强化学习的目标是什么？",
          "question_id": "q_flash_rl_def_v2"
        }
      ]
    },
    {
      "concept_key": "rl_key_components",
      "coverage_tags": [
        "rl_key_components"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_rl_components",
      "learning_goal": "学生能识别并解释强化学习的三个关键组件。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "强化学习三要素：状态、动作、奖励。",
      "term_refs": [
        {
          "display": "状态",
          "en": "State"
        },
        {
          "display": "动作",
          "en": "Action"
        },
        {
          "display": "奖励",
          "en": "Reward"
        }
      ],
      "variants": [
        {
          "back": "当前的环境状态（state）。",
          "estimated_seconds": 6,
          "explanation": "状态是环境在某一时刻的描述，智能体基于状态选择动作。",
          "front": "在强化学习中，智能体根据什么来决定下一步动作？",
          "question_id": "q_flash_rl_components_v1"
        },
        {
          "back": "奖励（reward）和下一个状态（next state）。",
          "estimated_seconds": 8,
          "explanation": "奖励是即时反馈，下一个状态是环境的新描述。",
          "front": "在强化学习中，智能体执行动作后，环境会返回什么信号？",
          "question_id": "q_flash_rl_components_v2"
        },
        {
          "back": "买入、卖出或持有。",
          "estimated_seconds": 6,
          "explanation": "这是交易中最基本的三个动作选项。",
          "front": "在交易场景下，强化学习中的“动作”可以是什么？",
          "question_id": "q_flash_rl_components_v3"
        }
      ]
    },
    {
      "concept_key": "q_learning_definition",
      "coverage_tags": [
        "q_learning_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_q_learning_def",
      "learning_goal": "学生能说出Q学习的核心定义和主要特点。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "Q学习的定义：无模型、Q值表、最优策略。",
      "term_refs": [
        {
          "display": "Q学习",
          "en": "Q-Learning"
        }
      ],
      "variants": [
        {
          "back": "一种无模型（model-free）的强化学习算法。",
          "estimated_seconds": 6,
          "explanation": "无模型意味着它不试图显式地建模环境，而是直接学习最优策略。",
          "front": "Q学习是一种什么类型的强化学习算法？",
          "question_id": "q_flash_q_learning_def_v1"
        },
        {
          "back": "一张Q值表（Q-table），记录每个状态-动作对的预期累积奖励。",
          "estimated_seconds": 8,
          "explanation": "Q表是Q学习的核心记忆，它告诉智能体在某个状态下哪个动作最好。",
          "front": "Q学习通过维护什么数据结构来学习最优策略？",
          "question_id": "q_flash_q_learning_def_v2"
        }
      ]
    },
    {
      "concept_key": "q_value_meaning",
      "coverage_tags": [
        "q_value_meaning"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_q_value",
      "learning_goal": "学生能准确解释Q(s, a)的含义。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "Q值的定义：预期未来总奖励。",
      "term_refs": [
        {
          "display": "Q值",
          "en": "Q-Value"
        }
      ],
      "variants": [
        {
          "back": "s 代表当前状态（state），a 代表要采取的动作（action）。",
          "estimated_seconds": 6,
          "explanation": "Q(s, a) 是状态-动作对的价值估计。",
          "front": "Q(s, a) 中的 s 和 a 分别代表什么？",
          "question_id": "q_flash_q_value_v1"
        },
        {
          "back": "在状态 s 下采取动作 a 后，所能获得的预期累积奖励（未来总奖励）。",
          "estimated_seconds": 8,
          "explanation": "它衡量的是长期回报，而不仅仅是立即奖励。",
          "front": "Q(s, a) 的值代表什么含义？",
          "question_id": "q_flash_q_value_v2"
        }
      ]
    },
    {
      "concept_key": "q_learning_update_formula",
      "coverage_tags": [
        "q_learning_update_formula"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_q_formula",
      "learning_goal": "学生能识别Q学习更新公式中的关键组成部分及其作用。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "Q学习更新公式中的变量：学习率、折扣因子、奖励、未来最佳Q值。",
      "term_refs": [
        {
          "display": "学习率",
          "en": "Learning Rate (α)"
        },
        {
          "display": "折扣因子",
          "en": "Discount Factor (γ)"
        }
      ],
      "variants": [
        {
          "back": "控制新信息覆盖旧信息的程度（0 < α ≤ 1）。",
          "estimated_seconds": 8,
          "explanation": "学习率越大，新经验对Q值的影响越大。",
          "front": "在Q学习更新公式中，α（学习率）的作用是什么？",
          "question_id": "q_flash_q_formula_v1"
        },
        {
          "back": "平衡即时奖励和未来奖励的重要性（0 ≤ γ < 1）。",
          "estimated_seconds": 8,
          "explanation": "折扣因子越接近1，智能体越看重长期回报。",
          "front": "在Q学习更新公式中，γ（折扣因子）的作用是什么？",
          "question_id": "q_flash_q_formula_v2"
        },
        {
          "back": "在下一个状态 s' 中，所有可能动作的最大Q值，即未来最佳动作的预期价值。",
          "estimated_seconds": 10,
          "explanation": "它代表了智能体对未来最优情况的估计。",
          "front": "在Q学习更新公式中，max Q(s', a') 代表什么？",
          "question_id": "q_flash_q_formula_v3"
        }
      ]
    },
    {
      "concept_key": "q_learning_steps",
      "coverage_tags": [
        "q_learning_steps"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_q_steps",
      "learning_goal": "学生能说出Q学习的主要步骤和探索与利用的概念。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "Q学习步骤：初始化、选择动作、执行、更新、迭代。",
      "term_refs": [
        {
          "display": "探索",
          "en": "Exploration"
        },
        {
          "display": "利用",
          "en": "Exploitation"
        }
      ],
      "variants": [
        {
          "back": "初始化Q表，通常将所有状态-动作对的Q值设为0。",
          "estimated_seconds": 6,
          "explanation": "这是学习的起点，智能体从一无所知开始。",
          "front": "Q学习的第一步是什么？",
          "question_id": "q_flash_q_steps_v1"
        },
        {
          "back": "探索是随机选择一个动作；利用是选择当前Q值最高的动作。",
          "estimated_seconds": 10,
          "explanation": "探索帮助发现新策略，利用则执行已知的最佳策略。",
          "front": "在Q学习中，智能体选择动作时，'探索'和'利用'分别指什么？",
          "question_id": "q_flash_q_steps_v2"
        }
      ]
    }
  ],
  "lesson_id": "L10",
  "longform_families": [
    {
      "concept_key": "q_learning_update_formula",
      "coverage_tags": [
        "q_learning_update_formula"
      ],
      "difficulty": "hard",
      "family_id": "qf_long_q_formula_apply",
      "learning_goal": "学生能应用Q学习更新公式进行手动计算，并解释每个参数的作用。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "formula_apply",
      "term_refs": [
        {
          "display": "Q学习更新公式",
          "en": "Q-Learning Update Formula"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "写出Q学习更新公式。",
            "代入已知数值。",
            "计算更新后的Q值。"
          ],
          "question_id": "q_long_q_formula_apply_v1",
          "reference_answer": [
            "Q学习更新公式：Q(s, a) ← Q(s, a) + α [r + γ * max Q(s', a') - Q(s, a)]",
            "代入数值：Q(s, '买入') ← 10 + 0.1 * [5 + 0.9 * 12 - 10]",
            "计算：Q(s, '买入') ← 10 + 0.1 * [5 + 10.8 - 10] = 10 + 0.1 * 5.8 = 10 + 0.58 = 10.58",
            "更新后的 Q(s, '买入') 值为 10.58。"
          ],
          "rubric_points": [
            "正确写出公式 (2分)",
            "正确代入数值 (2分)",
            "计算过程正确 (2分)",
            "最终答案正确 (2分)"
          ],
          "stem": "一个交易智能体使用Q学习进行训练。当前状态s下，Q(s, '买入') = 10。智能体执行了'买入'动作，获得了即时奖励 r = 5，并进入下一个状态 s'。在状态 s' 中，所有动作的最大Q值为 max Q(s', a') = 12。假设学习率 α = 0.1，折扣因子 γ = 0.9。请计算更新后的 Q(s, '买入') 值。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "写出Q学习更新公式。",
            "代入已知数值。",
            "计算更新后的Q值。",
            "解释负奖励对Q值更新的影响。"
          ],
          "question_id": "q_long_q_formula_apply_v2",
          "reference_answer": [
            "Q学习更新公式：Q(s, a) ← Q(s, a) + α [r + γ * max Q(s', a') - Q(s, a)]",
            "代入数值：Q(s, '卖出') ← 20 + 0.5 * [-2 + 0.8 * 15 - 20]",
            "计算：Q(s, '卖出') ← 20 + 0.5 * [-2 + 12 - 20] = 20 + 0.5 * (-10) = 20 - 5 = 15",
            "负奖励（-2）表示这次卖出导致了亏损。在更新中，它降低了目标值（r + γ * max Q(s', a')），从而使得更新后的Q值从20下降到15。这告诉智能体，在当前状态下执行'卖出'动作并不如之前认为的那么好。"
          ],
          "rubric_points": [
            "正确写出公式 (2分)",
            "正确代入数值 (2分)",
            "计算过程正确 (2分)",
            "正确解释负奖励的影响 (2分)"
          ],
          "stem": "假设另一个场景中，Q(s, '卖出') = 20，智能体执行'卖出'后获得奖励 r = -2（亏损），进入状态 s'，其中 max Q(s', a') = 15。请使用学习率 α = 0.5，折扣因子 γ = 0.8，计算更新后的 Q(s, '卖出') 值，并解释为什么奖励是负的。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "reinforcement_learning_definition",
      "coverage_tags": [
        "reinforcement_learning_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_rl_def",
      "learning_goal": "学生能在不同描述中准确识别强化学习的核心特征。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "强化学习",
          "en": "Reinforcement Learning"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "强化学习的核心是智能体在环境中试错，目标是最大化累积奖励。选项A是监督学习，选项B是无监督学习。",
          "options": [
            "从有标签的数据中学习预测结果",
            "在没有标签的数据中发现隐藏模式",
            "智能体通过与环境互动，以最大化累积奖励来学习决策",
            "通过遗传算法进化出最优模型"
          ],
          "question_id": "q_quiz_rl_def_v1",
          "stem": "以下哪一项最准确地描述了强化学习？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "监督学习使用有标签的数据（正确答案），而强化学习使用奖励信号作为反馈，没有直接的正确答案。",
          "options": [
            "强化学习不需要数据",
            "强化学习没有明确的正确答案，而是通过奖励信号学习",
            "强化学习只能用于游戏",
            "监督学习比强化学习更复杂"
          ],
          "question_id": "q_quiz_rl_def_v2",
          "stem": "强化学习与监督学习的主要区别是什么？"
        }
      ]
    },
    {
      "concept_key": "rl_key_components",
      "coverage_tags": [
        "rl_key_components"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_rl_components",
      "learning_goal": "学生能将强化学习的组件与其在交易场景下的例子正确匹配。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "matching",
      "term_refs": [
        {
          "display": "状态",
          "en": "State"
        },
        {
          "display": "动作",
          "en": "Action"
        },
        {
          "display": "奖励",
          "en": "Reward"
        }
      ],
      "variants": [
        {
          "correct_mapping": [
            0,
            1,
            2
          ],
          "estimated_seconds": 30,
          "explanation": "状态是环境的描述（价格、成交量）；动作是智能体的决策（买入）；奖励是反馈信号（盈利）。",
          "left_items": [
            "状态 (State)",
            "动作 (Action)",
            "奖励 (Reward)"
          ],
          "question_id": "q_quiz_rl_components_v1",
          "right_items": [
            "当前股票价格和成交量",
            "决定买入100股",
            "交易后账户盈利50元"
          ],
          "stem": "请将左侧的强化学习组件与右侧的交易场景例子进行匹配。"
        },
        {
          "correct_mapping": [
            0,
            1,
            2
          ],
          "estimated_seconds": 20,
          "explanation": "这是强化学习三个核心组件的标准定义。",
          "left_items": [
            "状态 (State)",
            "动作 (Action)",
            "奖励 (Reward)"
          ],
          "question_id": "q_quiz_rl_components_v2",
          "right_items": [
            "环境在某一时刻的描述",
            "智能体可以执行的操作",
            "执行动作后环境给予的即时反馈信号"
          ],
          "stem": "请将左侧的强化学习组件与右侧的定义进行匹配。"
        }
      ]
    },
    {
      "concept_key": "q_learning_definition",
      "coverage_tags": [
        "q_learning_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_q_learning_def",
      "learning_goal": "学生能识别Q学习的关键特征。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "Q学习",
          "en": "Q-Learning"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "Q学习是无模型的（model-free），它不尝试显式地学习环境模型，而是直接学习最优策略。",
          "options": [
            "基于模型的算法",
            "无模型的算法",
            "基于规则的算法",
            "监督学习算法"
          ],
          "question_id": "q_quiz_q_learning_def_v1",
          "stem": "Q学习属于哪种类型的强化学习算法？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "Q表是Q学习的核心，它存储了Q(s, a)值，即智能体对在每个状态下采取每个动作的长期价值的估计。",
          "options": [
            "存储所有历史交易数据",
            "记录每个状态-动作对的预期累积奖励",
            "定义交易规则",
            "计算市场波动率"
          ],
          "question_id": "q_quiz_q_learning_def_v2",
          "stem": "Q学习算法中，Q表的作用是什么？"
        }
      ]
    },
    {
      "concept_key": "q_value_meaning",
      "coverage_tags": [
        "q_value_meaning"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_q_value",
      "learning_goal": "学生能准确判断Q值的含义。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "Q值",
          "en": "Q-Value"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "Q(s, a) 是预期未来总奖励，它考虑了立即奖励和所有未来的折扣奖励。",
          "options": [
            "在状态s下立即获得的奖励",
            "在状态s下采取动作a后，所能获得的预期累积奖励",
            "状态s的好坏程度",
            "动作a的难度"
          ],
          "question_id": "q_quiz_q_value_v1",
          "stem": "在Q学习中，Q(s, a) 的值代表什么？"
        },
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "Q值代表预期累积奖励，值越高越好。因此智能体应选择Q值最高的动作，即'买入'。",
          "options": [
            "买入",
            "卖出",
            "持有",
            "无法判断"
          ],
          "question_id": "q_quiz_q_value_v2",
          "stem": "如果一个交易智能体在状态s下，Q(s, '买入') = 100，Q(s, '卖出') = 50，Q(s, '持有') = 20，那么它应该选择哪个动作？"
        }
      ]
    },
    {
      "concept_key": "q_learning_update_formula",
      "coverage_tags": [
        "q_learning_update_formula"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_q_formula",
      "learning_goal": "学生能理解Q学习更新公式中各个部分的作用。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "Q学习更新公式",
          "en": "Q-Learning Update Formula"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "学习率α控制新信息覆盖旧信息的程度。如果α=0，则新信息对Q值没有任何影响，Q值永远不会更新。",
          "options": [
            "Q值永远不会更新",
            "Q值会快速收敛",
            "智能体只关注未来奖励",
            "智能体只关注立即奖励"
          ],
          "question_id": "q_quiz_q_formula_v1",
          "stem": "在Q学习更新公式中，如果学习率 α = 0，会发生什么？"
        },
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "折扣因子γ平衡即时和未来奖励。γ=0意味着未来奖励的权重为0，智能体变得短视，只关心立即奖励。",
          "options": [
            "只关注立即奖励，完全忽略未来奖励",
            "只关注未来奖励，完全忽略立即奖励",
            "无法学习",
            "学习速度会变慢"
          ],
          "question_id": "q_quiz_q_formula_v2",
          "stem": "在Q学习更新公式中，如果折扣因子 γ = 0，智能体会如何表现？"
        }
      ]
    },
    {
      "concept_key": "q_learning_steps",
      "coverage_tags": [
        "q_learning_steps"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_q_steps",
      "learning_goal": "学生能正确排列Q学习的核心步骤。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "ordering",
      "term_refs": [
        {
          "display": "Q学习步骤",
          "en": "Q-Learning Steps"
        }
      ],
      "variants": [
        {
          "correct_order": [
            1,
            2,
            3,
            0,
            4
          ],
          "estimated_seconds": 30,
          "explanation": "正确的顺序是：1. 初始化Q表，2. 选择并执行动作，3. 观察奖励和下一个状态，4. 更新Q值，5. 重复直到收敛。",
          "items": [
            "A. 更新Q值",
            "B. 初始化Q表",
            "C. 选择并执行动作",
            "D. 观察奖励和下一个状态",
            "E. 重复直到收敛"
          ],
          "question_id": "q_quiz_q_steps_v1",
          "stem": "请将以下Q学习的步骤按正确顺序排列。"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "更新Q值需要用到执行动作后获得的奖励和下一个状态的信息，因此必须发生在执行动作并观察结果之后。",
          "options": [
            "在选择动作之前",
            "在执行动作之后，观察到奖励和下一个状态之后",
            "在初始化Q表之前",
            "在所有迭代结束之后"
          ],
          "question_id": "q_quiz_q_steps_v2",
          "stem": "在Q学习的迭代过程中，'更新Q值'这一步发生在什么时候？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L10 coverage checklist (reinforcement learning, Q-learning, state, action, reward, Q-learning formula)",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "L10 lesson map (step4: 强化学习：Q学习与交易)",
    "plain_text": "pipeline/1-plain/L10/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L10 source outline (3. Reinforcement Learning, Q-learning, SARSA)"
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "rl_trading_application",
      "covered_by": [
        "qf_flash_rl_trading_components",
        "qf_quiz_rl_trading_action_space",
        "qf_long_rl_trading_workflow"
      ],
      "description": "将Q学习应用于交易：定义状态（市场情况）、动作（买入/卖出/持有）、奖励（盈亏）。"
    },
    {
      "coverage_tag": "exploration_vs_exploitation",
      "covered_by": [
        "qf_flash_exploration_exploitation",
        "qf_quiz_exploration_exploitation_balance"
      ],
      "description": "Q学习智能体在探索（尝试新动作）与利用（选择已知最佳动作）之间平衡。训练初期更偏向探索。"
    },
    {
      "coverage_tag": "q_learning_agent_learning_process",
      "covered_by": [
        "qf_flash_rl_learning_process",
        "qf_long_rl_trading_workflow"
      ],
      "description": "Q学习智能体通过反复模拟交易来学习，从随机探索到学会最优策略。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "rl_trading_application",
      "coverage_tags": [
        "rl_trading_application"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_rl_trading_components",
      "learning_goal": "学生能准确回忆Q学习交易员的核心三要素：状态、动作、奖励的具体定义。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "Q学习交易框架中三个核心组件的定义。",
      "term_refs": [
        {
          "display": "状态",
          "en": "State"
        },
        {
          "display": "动作",
          "en": "Action"
        },
        {
          "display": "奖励",
          "en": "Reward"
        }
      ],
      "variants": [
        {
          "back": "当前的市场情况，例如价格、成交量等。",
          "estimated_seconds": 8,
          "explanation": "状态是智能体观察到的环境信息，用于决策。",
          "front": "在Q学习交易员中，“状态”指的是什么？",
          "question_id": "q_flash_rl_trading_components_v1"
        },
        {
          "back": "买入、卖出、持有。",
          "estimated_seconds": 8,
          "explanation": "这是Q学习交易员可以执行的所有操作。",
          "front": "在Q学习交易员中，“动作”空间包含哪三个选择？",
          "question_id": "q_flash_rl_trading_components_v2"
        },
        {
          "back": "这次交易赚了还是亏了（即盈亏）。",
          "estimated_seconds": 8,
          "explanation": "奖励是环境对智能体动作的即时反馈信号。",
          "front": "在Q学习交易员中，“奖励”是如何定义的？",
          "question_id": "q_flash_rl_trading_components_v3"
        }
      ]
    },
    {
      "concept_key": "exploration_vs_exploitation",
      "coverage_tags": [
        "exploration_vs_exploitation"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_exploration_exploitation",
      "learning_goal": "学生能准确回忆探索与利用的定义及其在Q学习训练初期的侧重。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "探索与利用的定义和训练初期的策略。",
      "term_refs": [
        {
          "display": "探索与利用",
          "en": "Exploration vs Exploitation"
        }
      ],
      "variants": [
        {
          "back": "尝试随机的、未知的交易动作，以发现可能更好的策略。",
          "estimated_seconds": 10,
          "explanation": "探索有助于发现新的、可能更优的策略。",
          "front": "在Q学习中，“探索”指的是什么？",
          "question_id": "q_flash_exploration_exploitation_v1"
        },
        {
          "back": "选择当前已知的、Q值最高的动作，以最大化预期奖励。",
          "estimated_seconds": 10,
          "explanation": "利用是使用已学到的知识来获取最大回报。",
          "front": "在Q学习中，“利用”指的是什么？",
          "question_id": "q_flash_exploration_exploitation_v2"
        },
        {
          "back": "探索。",
          "estimated_seconds": 5,
          "explanation": "初期需要更多探索，以发现各种可能的交易策略。",
          "front": "Q学习交易员在训练初期应该更偏向探索还是利用？",
          "question_id": "q_flash_exploration_exploitation_v3"
        }
      ]
    },
    {
      "concept_key": "q_learning_agent_learning_process",
      "coverage_tags": [
        "q_learning_agent_learning_process"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_rl_learning_process",
      "learning_goal": "学生能描述Q学习智能体从随机探索到学会最优策略的学习过程。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "Q学习智能体学习过程的两个阶段。",
      "term_refs": [
        {
          "display": "Q学习",
          "en": "Q-Learning"
        }
      ],
      "variants": [
        {
          "back": "随机探索各种交易。",
          "estimated_seconds": 8,
          "explanation": "初期通过随机探索来收集数据。",
          "front": "Q学习智能体在训练初期是如何选择动作的？",
          "question_id": "q_flash_rl_learning_process_v1"
        },
        {
          "back": "在什么状态下买入、什么状态下卖出最赚钱。",
          "estimated_seconds": 10,
          "explanation": "智能体通过学习Q值表，最终学会最优策略。",
          "front": "经过充分训练后，Q学习智能体会学会什么？",
          "question_id": "q_flash_rl_learning_process_v2"
        }
      ]
    }
  ],
  "lesson_id": "L10",
  "longform_families": [
    {
      "concept_key": "rl_trading_application",
      "coverage_tags": [
        "rl_trading_application",
        "q_learning_agent_learning_process"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_rl_trading_workflow",
      "learning_goal": "学生能完整解释Q学习交易员的工作流程，包括其核心组件、学习过程和探索与利用的平衡。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "mechanism_trace",
      "term_refs": [
        {
          "display": "Q学习",
          "en": "Q-Learning"
        },
        {
          "display": "状态",
          "en": "State"
        },
        {
          "display": "动作",
          "en": "Action"
        },
        {
          "display": "奖励",
          "en": "Reward"
        },
        {
          "display": "探索与利用",
          "en": "Exploration vs Exploitation"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "定义Q学习交易员的三个核心组件。",
            "描述智能体从开始到学会策略的学习过程。",
            "解释“探索与利用”在这个过程中的作用。"
          ],
          "question_id": "q_long_rl_trading_workflow_v1",
          "reference_answer": [
            "Q学习交易员的核心组件包括：状态（当前市场情况，如价格、成交量）、动作（买入、卖出、持有）和奖励（交易产生的盈亏）。",
            "智能体通过反复模拟交易来学习。开始时，它会随机探索各种交易动作。每次交易后，它会根据获得的奖励更新Q值表，从而学习在特定状态下哪个动作能带来最大累积奖励。",
            "在这个过程中，智能体需要在“探索”（尝试新动作以发现更好的策略）和“利用”（选择当前已知的最佳动作以最大化收益）之间取得平衡。在训练初期，智能体更偏向于探索，以收集足够的信息。"
          ],
          "rubric_points": [
            "正确识别并解释了状态、动作、奖励在交易场景中的具体含义。",
            "描述了智能体通过反复试错、更新Q值来学习的过程。",
            "解释了探索与利用的平衡，并指出训练初期更偏向探索。"
          ],
          "stem": "请解释如何将Q学习应用于构建一个简单的交易员。你需要描述其核心组件（状态、动作、奖励）以及智能体是如何通过反复模拟交易来学习的。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "描述一个完整的学习循环（状态 -> 动作 -> 奖励 -> 更新）。",
            "解释在这个循环中，智能体如何决定是“探索”还是“利用”。",
            "说明为什么这种平衡对于最终学习到一个好的交易策略至关重要。"
          ],
          "question_id": "q_long_rl_trading_workflow_v2",
          "reference_answer": [
            "学习循环开始于智能体观察当前的市场状态（例如，价格处于高位）。然后，它根据一个策略（如ε-贪婪策略）选择一个动作：要么以概率ε进行探索（随机选择买/卖/持有），要么以概率1-ε进行利用（选择当前Q值最高的动作）。执行动作后，环境会返回一个奖励（盈利或亏损）和新的市场状态。智能体使用Q学习公式更新上一个状态-动作对的Q值。",
            "探索与利用的平衡通过调整探索概率ε来实现。训练初期，ε值较高，鼓励探索；随着训练的进行，ε值逐渐降低，智能体更多地利用已学到的知识。",
            "这种平衡至关重要：没有探索，智能体可能永远发现不了最优策略；没有利用，智能体无法将学到的知识转化为稳定的收益。"
          ],
          "rubric_points": [
            "清晰地描述了状态、动作、奖励、Q值更新的循环。",
            "解释了智能体基于探索概率来选择动作的机制。",
            "阐述了平衡探索与利用对于避免局部最优和发现全局最优策略的重要性。"
          ],
          "stem": "假设你正在训练一个Q学习交易员。请描述它的学习循环：从观察市场状态开始，到做出决策，再到获得反馈并更新知识。请特别说明“探索”与“利用”策略在这个循环中是如何被应用的。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "rl_trading_application",
      "coverage_tags": [
        "rl_trading_application"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_rl_trading_action_space",
      "learning_goal": "学生能在测验中识别Q学习交易员动作空间的具体内容。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "动作空间",
          "en": "Action Space"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "Q学习交易员的动作空间被简化为三个基本动作：买入、卖出、持有。",
          "options": [
            "开仓、平仓、止损",
            "买入、卖出、持有",
            "做多、做空、套利",
            "限价单、市价单、止损单"
          ],
          "question_id": "q_quiz_rl_trading_action_space_v1",
          "stem": "在将Q学习应用于交易时，智能体的动作空间通常包含哪三个选项？"
        },
        {
          "answer": 3,
          "estimated_seconds": 15,
          "explanation": "Q学习交易员的动作空间是买入、卖出、持有，不包括对冲。",
          "options": [
            "买入",
            "卖出",
            "持有",
            "对冲"
          ],
          "question_id": "q_quiz_rl_trading_action_space_v2",
          "stem": "以下哪一项不是Q学习交易员的标准动作？"
        }
      ]
    },
    {
      "concept_key": "exploration_vs_exploitation",
      "coverage_tags": [
        "exploration_vs_exploitation"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_exploration_exploitation_balance",
      "learning_goal": "学生能在测验情境下辨析探索与利用的平衡策略。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "探索与利用",
          "en": "Exploration vs Exploitation"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "即使找到了不错的策略，也需要保持一定的探索概率，以防有更好的策略未被发现。",
          "options": [
            "完全停止探索，只利用当前最佳策略",
            "继续以较低的概率进行探索，同时主要利用当前策略",
            "完全停止利用，只进行探索",
            "随机选择探索或利用"
          ],
          "question_id": "q_quiz_exploration_exploitation_balance_v1",
          "stem": "一个Q学习交易员在训练了1000个回合后，已经找到了一个看起来不错的策略。为了获得更高的长期收益，它接下来应该怎么做？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "初期不进行探索，智能体可能永远发现不了比当前已知动作更好的策略，从而陷入局部最优。",
          "options": [
            "它会学得很快，并找到全局最优策略",
            "它可能会陷入局部最优，错过更好的交易策略",
            "它会导致Q值表无法收敛",
            "它会使交易风险变得过高"
          ],
          "question_id": "q_quiz_exploration_exploitation_balance_v2",
          "stem": "如果一个Q学习交易员在训练初期就完全偏向“利用”，即总是选择当前Q值最高的动作，最可能发生什么问题？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L10: Machine learning use cases in algorithmic trading - Agenda: Major categories of ML algorithms, Multiple use cases of ML in algo-trading, The future of smart investment systems. Types: Supervised, Unsupervised, Reinforcement. Reinforcement Learning: agent interacts with environment, learns by maximizing cumulative reward. Common algorithms: Q-learning, SARSA.",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "{\"lesson_id\": \"L10\", \"steps\": [{\"concept\": \"强化学习在交易中的应用：Q学习交易员\", \"sequence_id\": \"step7\"}]}",
    "plain_text": "pipeline/1-plain/L10/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L10: Machine learning use cases in algorithmic trading - Agenda: Major categories of ML algorithms, Multiple use cases of ML in algo-trading, The future of smart investment systems. 3. Reinforcement Learning: agent interacts with environment, learns by maximizing cumulative reward. Common algorithms: Q-learning, SARSA."
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "supervised_learning_two_tasks",
      "covered_by": [
        "qf_flash_sl_tasks",
        "qf_quiz_sl_tasks",
        "qf_long_sl_tasks"
      ],
      "description": "监督学习的两大任务：分类与回归"
    },
    {
      "coverage_tag": "classification_definition_example",
      "covered_by": [
        "qf_flash_classification",
        "qf_quiz_classification"
      ],
      "description": "分类的定义与交易场景示例（涨/跌）"
    },
    {
      "coverage_tag": "regression_definition_example",
      "covered_by": [
        "qf_flash_regression",
        "qf_quiz_regression"
      ],
      "description": "回归的定义与交易场景示例（具体价格）"
    },
    {
      "coverage_tag": "classification_vs_regression",
      "covered_by": [
        "qf_flash_class_vs_reg",
        "qf_quiz_class_vs_reg",
        "qf_long_class_vs_reg"
      ],
      "description": "分类与回归的核心区别：离散类别 vs 连续数值"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "supervised_learning_two_tasks",
      "coverage_tags": [
        "supervised_learning_two_tasks"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_sl_tasks",
      "learning_goal": "学生能准确说出监督学习的两大任务类型。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "监督学习的两大任务名称",
      "term_refs": [
        {
          "display": "监督学习",
          "en": "Supervised Learning"
        },
        {
          "display": "分类",
          "en": "Classification"
        },
        {
          "display": "回归",
          "en": "Regression"
        }
      ],
      "variants": [
        {
          "back": "分类（Classification）和回归（Regression）。",
          "estimated_seconds": 8,
          "explanation": "分类预测离散类别，回归预测连续数值。",
          "front": "监督学习内部主要分为哪两大任务？",
          "question_id": "q_flash_sl_tasks_v1"
        },
        {
          "back": "分类（Classification）。",
          "estimated_seconds": 6,
          "explanation": "分类任务输出的是有限的类别标签。",
          "front": "在监督学习中，预测离散类别（如涨/跌）的任务叫什么？",
          "question_id": "q_flash_sl_tasks_v2"
        },
        {
          "back": "回归（Regression）。",
          "estimated_seconds": 6,
          "explanation": "回归任务输出的是一个连续的数值。",
          "front": "在监督学习中，预测连续数值（如具体股价）的任务叫什么？",
          "question_id": "q_flash_sl_tasks_v3"
        }
      ]
    },
    {
      "concept_key": "classification_definition_example",
      "coverage_tags": [
        "classification_definition_example"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_classification",
      "learning_goal": "学生能给出分类的定义并提供一个交易相关的例子。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "definition_with_example",
      "retrieval_focus": "分类的定义与交易示例",
      "term_refs": [
        {
          "display": "分类",
          "en": "Classification"
        }
      ],
      "variants": [
        {
          "back": "分类是预测离散类别。例子：判断明天股价是“涨”还是“跌”。",
          "estimated_seconds": 10,
          "explanation": "分类的输出是有限的、离散的类别标签。",
          "front": "什么是分类（Classification）？请举一个交易中的例子。",
          "question_id": "q_flash_classification_v1"
        },
        {
          "back": "分类（Classification）。",
          "estimated_seconds": 8,
          "explanation": "因为输出是“垃圾邮件”或“正常邮件”这两个离散类别。",
          "front": "在交易中，判断一封邮件是“垃圾邮件”还是“正常邮件”属于什么类型的监督学习任务？",
          "question_id": "q_flash_classification_v2"
        }
      ]
    },
    {
      "concept_key": "regression_definition_example",
      "coverage_tags": [
        "regression_definition_example"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_regression",
      "learning_goal": "学生能给出回归的定义并提供一个交易相关的例子。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "definition_with_example",
      "retrieval_focus": "回归的定义与交易示例",
      "term_refs": [
        {
          "display": "回归",
          "en": "Regression"
        }
      ],
      "variants": [
        {
          "back": "回归是预测连续数值。例子：预测明天收盘价是150.5元。",
          "estimated_seconds": 10,
          "explanation": "回归的输出是一个连续的数值，而不是类别。",
          "front": "什么是回归（Regression）？请举一个交易中的例子。",
          "question_id": "q_flash_regression_v1"
        },
        {
          "back": "回归（Regression）。",
          "estimated_seconds": 8,
          "explanation": "因为销售额是一个连续的数值。",
          "front": "预测下个月的销售额属于什么类型的监督学习任务？",
          "question_id": "q_flash_regression_v2"
        }
      ]
    },
    {
      "concept_key": "classification_vs_regression",
      "coverage_tags": [
        "classification_vs_regression"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_class_vs_reg",
      "learning_goal": "学生能清晰区分分类和回归的输出类型。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "core_difference",
      "retrieval_focus": "分类与回归的核心区别：输出类型",
      "term_refs": [
        {
          "display": "分类",
          "en": "Classification"
        },
        {
          "display": "回归",
          "en": "Regression"
        }
      ],
      "variants": [
        {
          "back": "分类预测离散类别，回归预测连续数值。",
          "estimated_seconds": 8,
          "explanation": "这是两者最根本的区别。",
          "front": "分类（Classification）和回归（Regression）的核心区别是什么？",
          "question_id": "q_flash_class_vs_reg_v1"
        },
        {
          "back": "前者是分类，后者是回归。",
          "estimated_seconds": 10,
          "explanation": "涨/跌是离散类别，收盘价是连续数值。",
          "front": "预测“股票明天是涨还是跌”和预测“股票明天收盘价是多少”分别属于什么任务？",
          "question_id": "q_flash_class_vs_reg_v2"
        }
      ]
    }
  ],
  "lesson_id": "L10",
  "longform_families": [
    {
      "concept_key": "supervised_learning_two_tasks",
      "coverage_tags": [
        "supervised_learning_two_tasks",
        "classification_vs_regression"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_sl_tasks",
      "learning_goal": "学生能用自己的话解释分类和回归的区别，并分别给出交易场景的例子。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "compare_and_contrast",
      "term_refs": [
        {
          "display": "监督学习",
          "en": "Supervised Learning"
        },
        {
          "display": "分类",
          "en": "Classification"
        },
        {
          "display": "回归",
          "en": "Regression"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "定义分类和回归",
            "指出输出类型的核心区别",
            "为分类举一个交易例子",
            "为回归举一个交易例子"
          ],
          "question_id": "q_long_sl_tasks_v1",
          "reference_answer": [
            "分类（Classification）是预测离散类别标签的任务，例如判断明天股价是“涨”还是“跌”。",
            "回归（Regression）是预测连续数值的任务，例如预测明天收盘价是150.5元。",
            "核心区别在于输出类型：分类输出是有限的、离散的类别；回归输出是无限的、连续的数值。"
          ],
          "rubric_points": [
            "准确给出分类和回归的定义",
            "明确指出分类输出离散类别，回归输出连续数值",
            "分类例子合理（如判断涨/跌、垃圾邮件等）",
            "回归例子合理（如预测具体股价、波动率等）"
          ],
          "stem": "请解释监督学习中的“分类”和“回归”任务的区别，并为每个任务各举一个在算法交易中的应用例子。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "描述一个分类任务场景",
            "描述一个回归任务场景",
            "解释为什么分类任务输出是离散的",
            "解释为什么回归任务输出是连续的"
          ],
          "question_id": "q_long_sl_tasks_v2",
          "reference_answer": [
            "分类任务：预测股票明天相对于今天收盘价是“上涨”还是“下跌”。输出是“上涨”或“下跌”这两个离散的类别。",
            "回归任务：预测股票明天收盘价的具体数值，例如150.5元。输出是一个连续的数值。",
            "它们属于不同任务，因为分类处理的是有限、无序的类别，而回归处理的是有序、连续的数值。"
          ],
          "rubric_points": [
            "分类任务场景清晰（如预测方向）",
            "回归任务场景清晰（如预测具体数值）",
            "正确解释分类输出是离散的",
            "正确解释回归输出是连续的"
          ],
          "stem": "假设你是一个量化分析师，你的任务是开发一个模型来预测某只股票的未来走势。请分别描述一个分类任务和一个回归任务，并解释为什么它们属于不同的监督学习类型。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "supervised_learning_two_tasks",
      "coverage_tags": [
        "supervised_learning_two_tasks"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_sl_tasks",
      "learning_goal": "学生能在具体场景中识别出分类和回归任务。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "监督学习",
          "en": "Supervised Learning"
        },
        {
          "display": "分类",
          "en": "Classification"
        },
        {
          "display": "回归",
          "en": "Regression"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "判断邮件类别是典型的分类问题，输出是离散类别。预测股价、销售额和温度都是回归问题。",
          "options": [
            "预测明天苹果公司的股价",
            "判断一封邮件是垃圾邮件还是正常邮件",
            "预测下个月的销售额",
            "预测未来一周的温度"
          ],
          "question_id": "q_quiz_sl_tasks_v1",
          "stem": "以下哪个是分类（Classification）问题？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "预测具体收盘价是回归问题，输出是连续数值。其他选项都是分类问题。",
          "options": [
            "判断一张图片里是猫还是狗",
            "预测明天某只股票的具体收盘价",
            "将客户分为高、中、低三个风险等级",
            "识别手写数字0-9"
          ],
          "question_id": "q_quiz_sl_tasks_v2",
          "stem": "以下哪个是回归（Regression）问题？"
        }
      ]
    },
    {
      "concept_key": "classification_definition_example",
      "coverage_tags": [
        "classification_definition_example"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_classification",
      "learning_goal": "学生能识别分类问题的特征。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "分类",
          "en": "Classification"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "判断“是否上涨”是一个二分类问题，输出是“是”或“否”的离散类别。",
          "options": [
            "回归",
            "分类",
            "聚类",
            "关联规则"
          ],
          "question_id": "q_quiz_classification_v1",
          "stem": "在交易中，使用监督学习模型判断“明天股价是否会上涨”，这属于什么任务？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "逻辑回归虽然名字有回归，但常用于二分类问题。线性回归用于回归，K均值和Apriori是无监督学习。",
          "options": [
            "线性回归",
            "逻辑回归",
            "K均值聚类",
            "Apriori算法"
          ],
          "question_id": "q_quiz_classification_v2",
          "stem": "以下哪个算法的输出最适合解决分类问题？"
        }
      ]
    },
    {
      "concept_key": "regression_definition_example",
      "coverage_tags": [
        "regression_definition_example"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_regression",
      "learning_goal": "学生能识别回归问题的特征。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "回归",
          "en": "Regression"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "波动率是一个连续的百分比数值，因此是回归问题。",
          "options": [
            "分类",
            "回归",
            "聚类",
            "关联规则"
          ],
          "question_id": "q_quiz_regression_v1",
          "stem": "一个量化基金想要预测某只股票未来5分钟的波动率（一个百分比数值），这属于什么任务？"
        },
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "线性回归是经典的回归算法，用于预测连续数值。SVM、分类决策树和KNN通常用于分类。",
          "options": [
            "支持向量机（SVM）",
            "决策树（用于分类）",
            "线性回归",
            "K近邻（KNN，用于分类）"
          ],
          "question_id": "q_quiz_regression_v2",
          "stem": "以下哪个算法的输出最适合解决回归问题？"
        }
      ]
    },
    {
      "concept_key": "classification_vs_regression",
      "coverage_tags": [
        "classification_vs_regression"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_class_vs_reg",
      "learning_goal": "学生能准确判断一个预测问题是分类还是回归。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "true_false",
      "term_refs": [
        {
          "display": "分类",
          "en": "Classification"
        },
        {
          "display": "回归",
          "en": "Regression"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "这是分类问题，因为输出是“高于”或“低于”这两个离散类别。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_class_vs_reg_v1",
          "stem": "预测“明天某只股票的价格是高于还是低于当前价格”是一个回归问题。"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "这是回归问题，因为输出是一个具体的连续数值。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_class_vs_reg_v2",
          "stem": "预测“明天某只股票的具体价格”是一个分类问题。"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L10 coverage checklist (supervised learning: classification & regression)",
    "guided_story_manifest": "pipeline/3-guided_story/L10/step2/step.json",
    "lesson_map": "L10 lesson map (step2: supervised learning: classification & regression)",
    "plain_text": "pipeline/1-plain/L10/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L10 source outline (supervised learning: classification & regression)"
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "traditional_vs_ai_approach",
      "covered_by": [
        "qf_flash_trad_vs_ai",
        "qf_quiz_trad_vs_ai",
        "qf_long_trad_vs_ai"
      ],
      "description": "传统交易策略（规则驱动）与AI交易策略（数据驱动）的核心区别"
    },
    {
      "coverage_tag": "supervised_learning_trading_application",
      "covered_by": [
        "qf_flash_sl_trading",
        "qf_quiz_sl_trading",
        "qf_long_sl_trading"
      ],
      "description": "监督学习在算法交易中的应用：通过标记历史交易来学习规则"
    },
    {
      "coverage_tag": "rsi_trading_strategy",
      "covered_by": [
        "qf_flash_rsi",
        "qf_quiz_rsi"
      ],
      "description": "RSI（相对强弱指标）作为传统规则驱动策略的典型例子"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "traditional_vs_ai_approach",
      "coverage_tags": [
        "traditional_vs_ai_approach"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_trad_vs_ai",
      "learning_goal": "学生能准确区分传统交易策略和AI交易策略的核心驱动方式。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "传统策略与AI策略在规则来源上的根本差异。",
      "term_refs": [
        {
          "display": "规则驱动",
          "en": "Rule-driven"
        },
        {
          "display": "数据驱动",
          "en": "Data-driven"
        }
      ],
      "variants": [
        {
          "back": "数据驱动（结果驱动）",
          "estimated_seconds": 8,
          "explanation": "传统策略由人预先定义规则（如RSI阈值），AI策略则从标记好的历史数据中自动学习规则。",
          "front": "传统交易策略是规则驱动的，AI交易策略是什么驱动的？",
          "question_id": "q_flash_trad_vs_ai_v1"
        },
        {
          "back": "由人预先定义",
          "estimated_seconds": 8,
          "explanation": "传统策略依赖专家设定的固定规则（如技术指标阈值），而AI策略从数据中自动发现隐藏规律。",
          "front": "AI交易策略从数据中学习规则，传统交易策略的规则来自哪里？",
          "question_id": "q_flash_trad_vs_ai_v2"
        }
      ]
    },
    {
      "concept_key": "supervised_learning_trading_application",
      "coverage_tags": [
        "supervised_learning_trading_application"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_sl_trading",
      "learning_goal": "学生能说出监督学习应用于交易的基本流程：先标记后学习。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "监督学习在交易中的核心步骤：标记期望的交易结果。",
      "term_refs": [
        {
          "display": "监督学习",
          "en": "Supervised Learning"
        },
        {
          "display": "标记数据",
          "en": "Labeled data"
        }
      ],
      "variants": [
        {
          "back": "标记历史上哪些交易是好的（标记期望的交易结果）",
          "estimated_seconds": 10,
          "explanation": "先标记出历史上好的买入点和卖出点，然后让模型从这些标记数据中学习隐藏的市场规律。",
          "front": "在监督学习交易策略中，模型学习规则之前，需要先做什么？",
          "question_id": "q_flash_sl_trading_v1"
        },
        {
          "back": "从标记好的历史数据中",
          "estimated_seconds": 8,
          "explanation": "模型通过分析标记好的历史交易数据，自动学习什么样的市场特征会导致好的交易结果。",
          "front": "监督学习交易策略不再依赖固定的RSI阈值，而是从什么中发现隐藏规律？",
          "question_id": "q_flash_sl_trading_v2"
        }
      ]
    },
    {
      "concept_key": "rsi_trading_strategy",
      "coverage_tags": [
        "rsi_trading_strategy"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_rsi",
      "learning_goal": "学生能记住RSI策略的基本规则：超买和超卖的阈值。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "RSI策略中买入和卖出的具体阈值。",
      "term_refs": [
        {
          "display": "相对强弱指标",
          "en": "Relative Strength Index (RSI)"
        }
      ],
      "variants": [
        {
          "back": "30",
          "estimated_seconds": 5,
          "explanation": "RSI低于30通常被视为超卖状态，是买入信号。",
          "front": "在RSI交易策略中，当RSI低于多少时通常被视为买入信号？",
          "question_id": "q_flash_rsi_v1"
        },
        {
          "back": "70",
          "estimated_seconds": 5,
          "explanation": "RSI高于70通常被视为超买状态，是卖出信号。",
          "front": "在RSI交易策略中，当RSI高于多少时通常被视为卖出信号？",
          "question_id": "q_flash_rsi_v2"
        }
      ]
    }
  ],
  "lesson_id": "L10",
  "longform_families": [
    {
      "concept_key": "traditional_vs_ai_approach",
      "coverage_tags": [
        "traditional_vs_ai_approach",
        "supervised_learning_trading_application"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_trad_vs_ai",
      "learning_goal": "学生能用自己的语言解释传统规则驱动策略与AI数据驱动策略在交易中的根本区别，并举例说明。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "compare_and_contrast",
      "term_refs": [
        {
          "display": "规则驱动",
          "en": "Rule-driven"
        },
        {
          "display": "数据驱动",
          "en": "Data-driven"
        },
        {
          "display": "监督学习",
          "en": "Supervised Learning"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "规则来源",
            "数据需求",
            "适应性"
          ],
          "question_id": "q_long_trad_vs_ai_v1",
          "reference_answer": [
            "规则来源：传统策略（如RSI策略）的规则由人预先定义，例如'RSI低于30买入，高于70卖出'；监督学习策略的规则从标记好的历史数据中自动学习，模型自己发现什么样的市场特征会导致好的交易。",
            "数据需求：传统策略主要依赖技术指标的计算值（如RSI值），不需要标记数据；监督学习策略需要大量标记好的历史交易数据，即需要先标记出哪些交易是好的。",
            "适应性：传统策略的规则是固定的，市场规律变化后可能失效；监督学习策略可以从新数据中持续学习，适应市场变化，发现隐藏的规律。"
          ],
          "rubric_points": [
            "明确指出传统策略的规则由人预先定义，AI策略的规则从数据中自动学习",
            "指出传统策略依赖技术指标阈值，AI策略依赖标记好的历史交易数据",
            "指出传统策略规则固定，难以适应市场变化；AI策略能从新数据中学习，适应性更强"
          ],
          "stem": "请比较传统交易策略（如RSI策略）与监督学习交易策略在规则来源、数据需求和适应性方面的主要区别。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "核心思想变化",
            "潜在优势",
            "一个具体例子"
          ],
          "question_id": "q_long_trad_vs_ai_v2",
          "reference_answer": [
            "核心思想变化：从'规则驱动'转变为'数据驱动'。传统方法是先定义规则（如RSI<30买入），然后看结果；监督学习方法是先标记期望的结果（如历史上好的买入点），然后让模型从数据中学习规则。",
            "潜在优势：监督学习能发现比固定RSI阈值更复杂的隐藏规律，模型可以同时考虑多个市场特征，而不是仅依赖一个指标。",
            "具体例子：在RSI策略中，我们固定使用30和70作为阈值；在监督学习中，我们标记出历史上所有盈利的交易点，让模型自己学习在这些点之前市场呈现什么特征，可能发现RSI、成交量、波动率等多个指标的组合规律。"
          ],
          "rubric_points": [
            "准确描述从'定义规则然后看结果'到'标记好结果然后学习规则'的思想转变",
            "指出数据驱动方法能发现更复杂的隐藏规律，而非依赖固定阈值",
            "能结合RSI例子说明：不再问'RSI低于30是否该买入'，而是问'历史上哪些市场特征导致了好的买入机会'"
          ],
          "stem": "假设你是一个量化交易员，请解释为什么从传统RSI策略转向监督学习策略可能带来优势，并指出这种转变的核心思想变化。"
        }
      ]
    },
    {
      "concept_key": "supervised_learning_trading_application",
      "coverage_tags": [
        "supervised_learning_trading_application"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_sl_trading",
      "learning_goal": "学生能描述监督学习应用于交易策略的完整流程：从数据标记到模型学习规则。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "mechanism_trace",
      "term_refs": [
        {
          "display": "监督学习",
          "en": "Supervised Learning"
        },
        {
          "display": "标记数据",
          "en": "Labeled data"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "数据准备",
            "标记过程",
            "模型学习"
          ],
          "question_id": "q_long_sl_trading_v1",
          "reference_answer": [
            "数据准备：收集历史市场数据，包括价格、成交量、技术指标等，作为模型的输入特征。",
            "标记过程：在历史数据中标记出期望的交易结果，例如标记出哪些价格点是好的买入点（后续价格上涨），哪些是好的卖出点（后续价格下跌）。这些标记就是监督学习中的'标签'。",
            "模型学习：将市场特征作为输入，交易标记作为输出，训练监督学习模型（如分类或回归模型）。模型学习什么样的市场特征组合会导致好的交易结果，从而自动发现隐藏的规律，不再依赖固定的RSI阈值。"
          ],
          "rubric_points": [
            "指出需要历史价格、成交量等市场数据",
            "准确描述如何标记期望的交易结果（好的买入点和卖出点）",
            "说明模型如何从标记数据中学习市场特征与交易结果之间的关系"
          ],
          "stem": "请描述监督学习如何应用于构建一个交易策略，从数据准备到模型学习规则的全过程。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "逻辑方向",
            "结果驱动的含义",
            "与传统方法的对比"
          ],
          "question_id": "q_long_sl_trading_v2",
          "reference_answer": [
            "逻辑方向：传统规则驱动方法的逻辑是'定义规则 → 观察结果'；监督学习结果驱动方法的逻辑是'确定期望结果 → 学习规则'。",
            "结果驱动的含义：先标记出历史上哪些交易结果是好的（如哪些买入点后续盈利），然后让模型反向学习：在这些好的交易发生之前，市场呈现什么特征？模型从结果出发，逆向推导出规则。",
            "与传统方法的对比：传统RSI策略问'如果RSI低于30，是否应该买入？'；监督学习策略问'历史上那些盈利的买入点，在发生之前市场有什么共同特征？'。前者从规则出发，后者从结果出发。"
          ],
          "rubric_points": [
            "准确指出传统方法是从规则到结果，监督学习是从结果到规则",
            "解释'结果驱动'意味着先确定期望的结果，再反向学习规则",
            "能结合RSI例子说明逻辑方向的不同"
          ],
          "stem": "解释为什么监督学习交易策略被称为'结果驱动'方法，并说明它与传统'规则驱动'方法在逻辑方向上的根本不同。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "traditional_vs_ai_approach",
      "coverage_tags": [
        "traditional_vs_ai_approach"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_trad_vs_ai",
      "learning_goal": "学生能在选择题中准确辨析传统策略与AI策略的根本区别。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "规则驱动",
          "en": "Rule-driven"
        },
        {
          "display": "数据驱动",
          "en": "Data-driven"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "传统策略由人定义规则（如RSI阈值），AI策略从标记好的历史数据中自动学习规则。",
          "options": [
            "传统策略更赚钱，AI策略更稳定",
            "传统策略是规则驱动，AI策略是数据驱动",
            "传统策略不需要数据，AI策略需要大量数据",
            "传统策略只用于股票，AI策略用于所有资产"
          ],
          "question_id": "q_quiz_trad_vs_ai_v1",
          "stem": "以下哪一项最准确地描述了传统交易策略与AI交易策略的核心区别？"
        },
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "该策略基于预先定义的固定规则（RSI阈值），属于传统的规则驱动方法，而非从数据中学习的AI方法。",
          "options": [
            "监督学习方法",
            "无监督学习方法",
            "规则驱动方法",
            "强化学习方法"
          ],
          "question_id": "q_quiz_trad_vs_ai_v2",
          "stem": "一个交易员使用“如果RSI低于30则买入，高于70则卖出”的策略。这属于哪种方法？"
        }
      ]
    },
    {
      "concept_key": "supervised_learning_trading_application",
      "coverage_tags": [
        "supervised_learning_trading_application"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_sl_trading",
      "learning_goal": "学生能理解监督学习在交易中如何从标记数据中学习。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "监督学习",
          "en": "Supervised Learning"
        },
        {
          "display": "标记数据",
          "en": "Labeled data"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "监督学习需要先标记出历史上期望的交易结果（如好的买入/卖出点），然后模型从这些标记数据中学习隐藏规律。",
          "options": [
            "定义RSI阈值",
            "标记历史上哪些交易是好的",
            "随机生成交易信号",
            "计算市场波动率"
          ],
          "question_id": "q_quiz_sl_trading_v1",
          "stem": "在监督学习交易策略中，模型学习规则的第一步是什么？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "监督学习策略能从标记好的历史数据中自动学习市场特征与交易结果之间的复杂关系，而不是依赖固定的RSI阈值。",
          "options": [
            "不需要历史数据",
            "能从数据中发现隐藏的规律，而非依赖固定阈值",
            "交易速度更快",
            "永远不会亏损"
          ],
          "question_id": "q_quiz_sl_trading_v2",
          "stem": "与传统RSI策略相比，监督学习交易策略的主要优势是什么？"
        }
      ]
    },
    {
      "concept_key": "rsi_trading_strategy",
      "coverage_tags": [
        "rsi_trading_strategy"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_rsi",
      "learning_goal": "学生能正确判断RSI策略的基本规则。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "true_false",
      "term_refs": [
        {
          "display": "相对强弱指标",
          "en": "Relative Strength Index (RSI)"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "RSI低于30通常被视为超卖状态，是买入信号，而非卖出信号。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_rsi_v1",
          "stem": "在RSI交易策略中，当RSI低于30时通常被视为卖出信号。"
        },
        {
          "answer": 0,
          "estimated_seconds": 15,
          "explanation": "RSI策略基于预先定义的固定阈值（低于30买入，高于70卖出），属于规则驱动策略。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_rsi_v2",
          "stem": "RSI交易策略是规则驱动策略的一个典型例子。"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L10: Machine learning use cases in algorithmic trading - Agenda: Major categories of ML algorithms, Multiple use cases of ML in algo-trading, The future of smart investment systems",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "{\"lesson_id\": \"L10\", \"steps\": [{\"concept\": \"机器学习三大类型概览\", \"sequence_id\": \"step1\"}, {\"concept\": \"监督学习：分类与回归\", \"sequence_id\": \"step2\"}, {\"concept\": \"无监督学习：聚类与关联\", \"sequence_id\": \"step3\"}, {\"concept\": \"强化学习：Q学习与交易\", \"sequence_id\": \"step4\"}, {\"concept\": \"监督学习在交易中的应用：从规则到数据\", \"sequence_id\": \"step5\"}, {\"concept\": \"无监督学习在交易中的应用：股票聚类\", \"sequence_id\": \"step6\"}, {\"concept\": \"强化学习在交易中的应用：Q学习交易员\", \"sequence_id\": \"step7\"}, {\"concept\": \"挑战与未来：AI交易的边界\", \"sequence_id\": \"step8\"}]}",
    "plain_text": "pipeline/1-plain/L10/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L10: Machine learning use cases in algorithmic trading - Agenda: Major categories of ML algorithms, Multiple use cases of ML in algo-trading, The future of smart investment systems"
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "unsupervised_learning_definition",
      "covered_by": [
        "qf_flash_unsup_def",
        "qf_quiz_unsup_def"
      ],
      "description": "无监督学习的定义：在没有标签的数据中发现模式和关系。"
    },
    {
      "coverage_tag": "clustering_definition",
      "covered_by": [
        "qf_flash_clustering_def",
        "qf_quiz_clustering_app"
      ],
      "description": "聚类：将数据点根据相似度自动分组。"
    },
    {
      "coverage_tag": "association_definition",
      "covered_by": [
        "qf_flash_association_def",
        "qf_quiz_association_app"
      ],
      "description": "关联规则：发现“如果A发生，B也经常发生”的规律。"
    },
    {
      "coverage_tag": "k_means_algorithm",
      "covered_by": [
        "qf_flash_kmeans_param",
        "qf_flash_kmeans_steps",
        "qf_quiz_kmeans_process",
        "qf_long_kmeans_explain"
      ],
      "description": "K均值聚类算法：工作原理、参数K、迭代过程。"
    },
    {
      "coverage_tag": "k_means_trading_example",
      "covered_by": [
        "qf_quiz_kmeans_stock"
      ],
      "description": "K均值在交易中的应用：股票聚类示例。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "unsupervised_learning",
      "coverage_tags": [
        "unsupervised_learning_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_unsup_def",
      "learning_goal": "学生能准确说出无监督学习的核心特点：无需标签，自行发现模式。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "无监督学习与监督学习的核心区别在于数据是否需要标签。",
      "term_refs": [
        {
          "display": "无监督学习",
          "en": "Unsupervised Learning"
        }
      ],
      "variants": [
        {
          "back": "无监督学习不需要带标签的数据。",
          "estimated_seconds": 8,
          "explanation": "监督学习依赖有标签的数据集，而无监督学习在没有标签的数据中发现隐藏模式。",
          "front": "无监督学习在数据要求上与监督学习最大的区别是什么？",
          "question_id": "q_flash_unsup_def_v1"
        },
        {
          "back": "聚类和关联规则。",
          "estimated_seconds": 6,
          "explanation": "聚类将相似数据分组，关联规则发现物品之间的共现规律。",
          "front": "无监督学习的两大主要任务是什么？",
          "question_id": "q_flash_unsup_def_v2"
        }
      ]
    },
    {
      "concept_key": "clustering",
      "coverage_tags": [
        "clustering_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_clustering_def",
      "learning_goal": "学生能用自己的话解释聚类的目标并给出一个交易场景的例子。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "聚类的核心是“物以类聚”，将相似对象自动分组。",
      "term_refs": [
        {
          "display": "聚类",
          "en": "Clustering"
        }
      ],
      "variants": [
        {
          "back": "将数据点根据相似度自动分成若干组（簇）。",
          "estimated_seconds": 8,
          "explanation": "聚类是无监督学习的一种，旨在发现数据中的自然分组。",
          "front": "聚类算法的核心目标是什么？",
          "question_id": "q_flash_clustering_def_v1"
        },
        {
          "back": "把走势相似的股票自动分到同一个组里。",
          "estimated_seconds": 10,
          "explanation": "例如，用K均值聚类将科技股和银行股分开，帮助构建分散化投资组合。",
          "front": "在算法交易中，聚类的一个具体应用例子是什么？",
          "question_id": "q_flash_clustering_def_v2"
        }
      ]
    },
    {
      "concept_key": "association",
      "coverage_tags": [
        "association_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_association_def",
      "learning_goal": "学生能解释关联规则的核心思想并给出一个具体例子。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "关联规则的核心是发现“如果...那么...”的共现模式。",
      "term_refs": [
        {
          "display": "关联规则",
          "en": "Association Rule Learning"
        }
      ],
      "variants": [
        {
          "back": "“如果A发生，B也经常发生”的共现规律。",
          "estimated_seconds": 8,
          "explanation": "例如，93%买了某只科技股的人也买了另一只。",
          "front": "关联规则学习要发现数据中的什么规律？",
          "question_id": "q_flash_association_def_v1"
        },
        {
          "back": "“93%购买了商品A的顾客也购买了商品B”。",
          "estimated_seconds": 6,
          "explanation": "这种规则可以用于交叉销售或推荐系统。",
          "front": "关联规则的一个经典例子是什么？",
          "question_id": "q_flash_association_def_v2"
        }
      ]
    },
    {
      "concept_key": "k_means",
      "coverage_tags": [
        "k_means_algorithm"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_kmeans_param",
      "learning_goal": "学生能记住K均值算法需要用户指定的关键参数。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "K均值需要用户提前指定簇的数量K。",
      "term_refs": [
        {
          "display": "K均值聚类",
          "en": "K-Means Clustering"
        }
      ],
      "variants": [
        {
          "back": "K值（簇的数量）。",
          "estimated_seconds": 5,
          "explanation": "K均值需要用户告诉算法要将数据分成几个组。",
          "front": "使用K均值聚类前，用户必须提前指定哪个参数？",
          "question_id": "q_flash_kmeans_param_v1"
        },
        {
          "back": "最终要形成的簇（组）的数量。",
          "estimated_seconds": 5,
          "explanation": "例如K=2表示将数据分成两组。",
          "front": "K均值聚类中，K代表什么？",
          "question_id": "q_flash_kmeans_param_v2"
        }
      ]
    },
    {
      "concept_key": "k_means",
      "coverage_tags": [
        "k_means_algorithm"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_kmeans_steps",
      "learning_goal": "学生能按顺序回忆K均值算法的核心步骤。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "next_step",
      "retrieval_focus": "K均值是一个迭代算法，核心步骤是分配和更新。",
      "term_refs": [
        {
          "display": "K均值聚类",
          "en": "K-Means Clustering"
        }
      ],
      "variants": [
        {
          "back": "将每个数据点分配到最近的中心点。",
          "estimated_seconds": 8,
          "explanation": "这是分配步骤，基于距离度量（如欧几里得距离）将点归入最近的簇。",
          "front": "K均值算法中，在随机初始化K个中心点之后，下一步做什么？",
          "question_id": "q_flash_kmeans_steps_v1"
        },
        {
          "back": "重新计算每个簇的中心点（取簇内所有点的均值）。",
          "estimated_seconds": 8,
          "explanation": "这是更新步骤，新的中心点是簇内所有数据点的平均值。",
          "front": "K均值算法中，在分配完所有数据点后，下一步做什么？",
          "question_id": "q_flash_kmeans_steps_v2"
        }
      ]
    }
  ],
  "lesson_id": "L10",
  "longform_families": [
    {
      "concept_key": "k_means",
      "coverage_tags": [
        "k_means_algorithm"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_kmeans_explain",
      "learning_goal": "学生能完整解释K均值算法的工作原理、关键参数和迭代过程。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "mechanism_trace",
      "term_refs": [
        {
          "display": "K均值聚类",
          "en": "K-Means Clustering"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "关键参数",
            "迭代步骤",
            "停止条件"
          ],
          "question_id": "q_long_kmeans_explain_v1",
          "reference_answer": [
            "K均值聚类需要用户提前指定K值，即最终要形成的簇的数量。",
            "算法首先随机初始化K个中心点。然后进入迭代：① 分配步骤：计算每个数据点到所有中心点的距离（通常用欧几里得距离），将其分配到最近的中心点所在的簇；② 更新步骤：重新计算每个簇的中心点，取簇内所有数据点的均值。",
            "重复分配和更新步骤，直到中心点不再发生变化（收敛），或者达到预设的最大迭代次数。"
          ],
          "rubric_points": [
            "正确指出K值（簇的数量）是用户必须指定的参数",
            "正确描述初始化、分配、更新三个核心步骤",
            "正确说明收敛条件（中心点不再变化或达到最大迭代次数）"
          ],
          "stem": "请解释K均值聚类算法的工作原理。你的回答应包括：① 算法需要用户指定的关键参数；② 算法的核心迭代步骤；③ 算法何时停止。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "特征构建",
            "K值选择",
            "距离度量选择"
          ],
          "question_id": "q_long_kmeans_explain_v2",
          "reference_answer": [
            "每个股票的特征向量可以是一段时间内的日收益率序列，例如过去一年的每日收益率。",
            "K值的选择可以通过肘部法则（观察不同K值下的簇内误差平方和）或基于业务需求（如希望分成几个行业板块）来确定。",
            "余弦距离更合适，因为它关注的是向量之间的方向（形状）而非大小（幅度）。即使两只股票的收益率绝对值不同，只要它们同涨同跌的模式相似，余弦距离就能捕捉到这种相似性。"
          ],
          "rubric_points": [
            "正确说明使用收益率时间序列作为特征向量",
            "提到肘部法则或领域知识作为K值选择方法",
            "正确选择余弦距离并解释原因（关注形状而非幅度）"
          ],
          "stem": "假设你有一组股票的历史日收益率数据，你想用K均值聚类将它们分组。请解释：① 你将如何构建每个股票的特征向量？② 你将如何选择K值？③ 如果两只股票的收益率曲线形状相似但幅度不同，哪种距离度量更合适？为什么？"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "unsupervised_learning",
      "coverage_tags": [
        "unsupervised_learning_definition"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_unsup_def",
      "learning_goal": "学生能在不同场景中识别出无监督学习的应用。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "无监督学习",
          "en": "Unsupervised Learning"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "将股票自动分组是聚类任务，属于无监督学习。预测涨跌是分类（监督学习），下棋是强化学习。",
          "options": [
            "根据历史数据预测明天股票是涨还是跌",
            "将一组股票根据收益率曲线自动分成几个类别",
            "训练一个机器人通过试错学习下棋"
          ],
          "question_id": "q_quiz_unsup_def_v1",
          "stem": "以下哪个任务最适合使用无监督学习？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "发现商品之间的共现规律是关联规则学习的典型应用，属于无监督学习。",
          "options": [
            "监督学习中的分类",
            "无监督学习中的关联规则",
            "强化学习"
          ],
          "question_id": "q_quiz_unsup_def_v2",
          "stem": "一家电商平台想发现“买了A商品的顾客通常也买B商品”的规律，应该使用哪种机器学习方法？"
        }
      ]
    },
    {
      "concept_key": "clustering",
      "coverage_tags": [
        "clustering_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_clustering_app",
      "learning_goal": "学生能区分聚类与其他无监督学习任务。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "聚类",
          "en": "Clustering"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "将客户分组是聚类。预测气温是回归，判断垃圾邮件是分类，两者都是监督学习。",
          "options": [
            "预测明天的气温",
            "将客户根据消费习惯分成不同的群体",
            "判断一封邮件是否为垃圾邮件"
          ],
          "question_id": "q_quiz_clustering_app_v1",
          "stem": "以下哪个是无监督学习中聚类的例子？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "聚类用于发现相似股票组。预测价格是回归，学习买卖时机是强化学习。",
          "options": [
            "预测下一分钟的价格",
            "将走势相似的股票归为一组",
            "学习最优的买卖时机"
          ],
          "question_id": "q_quiz_clustering_app_v2",
          "stem": "在算法交易中，以下哪个是聚类的直接应用？"
        }
      ]
    },
    {
      "concept_key": "association",
      "coverage_tags": [
        "association_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_association_app",
      "learning_goal": "学生能判断一个场景是否属于关联规则学习。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "true_false",
      "term_refs": [
        {
          "display": "关联规则",
          "en": "Association Rule Learning"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 10,
          "explanation": "这正是在发现“如果A则B”的共现规律，是关联规则学习的典型应用。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_association_app_v1",
          "stem": "“发现购买了科技ETF的投资者也经常购买半导体个股”属于关联规则学习的应用。"
        },
        {
          "answer": 1,
          "estimated_seconds": 10,
          "explanation": "将股票分组是聚类任务，不是关联规则。关联规则关注的是物品之间的共现关系。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_association_app_v2",
          "stem": "“根据过去一年的数据，将股票分为高波动和低波动两组”属于关联规则学习的应用。"
        }
      ]
    },
    {
      "concept_key": "k_means",
      "coverage_tags": [
        "k_means_algorithm"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_kmeans_process",
      "learning_goal": "学生能正确排列K均值算法的执行步骤。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "ordering",
      "term_refs": [
        {
          "display": "K均值聚类",
          "en": "K-Means Clustering"
        }
      ],
      "variants": [
        {
          "answer": [
            0,
            1,
            2,
            3
          ],
          "estimated_seconds": 25,
          "explanation": "K均值先初始化中心点，然后迭代进行分配和更新，直到中心点不再变化。",
          "options": [
            "A. 随机初始化K个中心点",
            "B. 将每个数据点分配到最近的中心点",
            "C. 重新计算每个簇的中心点",
            "D. 重复分配和更新直到收敛"
          ],
          "question_id": "q_quiz_kmeans_process_v1",
          "stem": "请将K均值聚类的以下步骤按正确顺序排列："
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "分配完数据点后，需要根据簇内所有点的均值更新中心点，然后判断是否收敛。",
          "options": [
            "随机初始化K个中心点",
            "重新计算每个簇的中心点",
            "判断是否收敛"
          ],
          "question_id": "q_quiz_kmeans_process_v2",
          "stem": "K均值算法中，以下哪个步骤发生在“分配数据点”之后？"
        }
      ]
    },
    {
      "concept_key": "k_means",
      "coverage_tags": [
        "k_means_trading_example"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_kmeans_stock",
      "learning_goal": "学生能理解K均值在股票聚类中的实际应用场景。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "micro_case_choice",
      "term_refs": [
        {
          "display": "K均值聚类",
          "en": "K-Means Clustering"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 25,
          "explanation": "第一组主要是大型科技公司，第二组包含特斯拉（汽车）和两家银行，反映了行业差异。",
          "options": [
            "按市值大小分组",
            "按行业或板块分组（科技 vs 非纯科技/金融）",
            "按股票代码字母顺序分组"
          ],
          "question_id": "q_quiz_kmeans_stock_v1",
          "stem": "一位量化分析师用K均值聚类将7只股票（MSFT, GOOG, AAPL, TSLA, NVDA, JPM, BAC）分成2组。结果一组是{MSFT, GOOG, AAPL, NVDA}，另一组是{TSLA, JPM, BAC}。这个结果最可能反映了什么？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "通常使用股票的历史收益率序列作为特征，因为聚类基于价格走势的相似性。",
          "options": [
            "股票代码",
            "历史收益率时间序列",
            "公司CEO的名字"
          ],
          "question_id": "q_quiz_kmeans_stock_v2",
          "stem": "使用K均值对股票进行聚类时，通常用什么作为每个股票的特征向量？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L10: Machine learning use cases in algorithmic trading - Unsupervised Learning: Clustering and Association",
    "guided_story_manifest": "pipeline/3-guided_story/L10/manifest.json",
    "lesson_map": "L10 step3: 无监督学习：聚类与关联",
    "plain_text": "pipeline/1-plain/L10/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L10: Machine learning use cases in algorithmic trading - 2. Unsupervised Learning"
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "unsupervised_learning_trading_applications",
      "covered_by": [
        "qf_flash_stock_clustering_goal",
        "qf_flash_distance_measures",
        "qf_quiz_distance_choice",
        "qf_long_clustering_application"
      ],
      "description": "无监督学习在算法交易中的应用，特别是股票聚类。"
    },
    {
      "coverage_tag": "k_means_clustering",
      "covered_by": [
        "qf_flash_stock_clustering_goal",
        "qf_flash_distance_measures",
        "qf_quiz_distance_choice",
        "qf_long_clustering_application"
      ],
      "description": "K均值聚类算法及其在股票聚类中的应用。"
    },
    {
      "coverage_tag": "distance_measures",
      "covered_by": [
        "qf_flash_distance_measures",
        "qf_quiz_distance_choice"
      ],
      "description": "欧几里得距离和余弦距离的定义、区别及适用场景。"
    },
    {
      "coverage_tag": "euclidean_distance",
      "covered_by": [
        "qf_flash_distance_measures",
        "qf_quiz_distance_choice"
      ],
      "description": "欧几里得距离：计算两点之间的直线距离。"
    },
    {
      "coverage_tag": "cosine_distance",
      "covered_by": [
        "qf_flash_distance_measures",
        "qf_quiz_distance_choice"
      ],
      "description": "余弦距离：关注向量方向（形状）而非大小。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "unsupervised_learning_trading_applications",
      "coverage_tags": [
        "unsupervised_learning_trading_applications",
        "k_means_clustering"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_stock_clustering_goal",
      "learning_goal": "学生能说出无监督学习在交易中的一个具体应用及其目的。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "无监督学习在交易中的一个核心应用及其目标。",
      "term_refs": [
        {
          "display": "股票聚类",
          "en": "Stock Clustering"
        },
        {
          "display": "无监督学习",
          "en": "Unsupervised Learning"
        }
      ],
      "variants": [
        {
          "back": "股票聚类：把走势相似的股票自动分到一组。",
          "estimated_seconds": 8,
          "explanation": "通过聚类，可以识别出具有相似价格行为的股票，用于投资组合分散或配对交易。",
          "front": "无监督学习在算法交易中的一个重要应用是什么？",
          "question_id": "q_flash_stock_clustering_goal_v1"
        },
        {
          "back": "将走势相似的股票自动分组，以发现潜在的相关性或用于投资组合分散。",
          "estimated_seconds": 10,
          "explanation": "例如，将科技股和银行股分别聚类，有助于构建更平衡的投资组合。",
          "front": "股票聚类的目的是什么？",
          "question_id": "q_flash_stock_clustering_goal_v2"
        }
      ]
    },
    {
      "concept_key": "distance_measures",
      "coverage_tags": [
        "distance_measures",
        "euclidean_distance",
        "cosine_distance"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_distance_measures",
      "learning_goal": "学生能区分欧几里得距离和余弦距离的核心差异。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "core_difference",
      "retrieval_focus": "两种距离度量在衡量股票收益率曲线相似度时的核心区别。",
      "term_refs": [
        {
          "display": "欧几里得距离",
          "en": "Euclidean Distance"
        },
        {
          "display": "余弦距离",
          "en": "Cosine Distance"
        }
      ],
      "variants": [
        {
          "back": "欧几里得距离关注曲线之间的绝对数值差异（直线距离），而余弦距离关注曲线的形状（方向）是否相似。",
          "estimated_seconds": 15,
          "explanation": "如果两条曲线形状相同但幅度不同，余弦距离会认为它们相似，而欧几里得距离则会认为它们不同。",
          "front": "欧几里得距离和余弦距离在衡量两条收益率曲线相似度时，最核心的区别是什么？",
          "question_id": "q_flash_distance_measures_v1"
        },
        {
          "back": "余弦距离。因为它关注方向（形状），对幅度不敏感。",
          "estimated_seconds": 12,
          "explanation": "余弦距离通过计算向量夹角来衡量相似性，因此对向量的绝对大小不敏感。",
          "front": "当两只股票的收益率曲线形状非常相似，但一只的波动幅度远大于另一只时，哪种距离度量更合适？",
          "question_id": "q_flash_distance_measures_v2"
        }
      ]
    }
  ],
  "lesson_id": "L10",
  "longform_families": [
    {
      "concept_key": "unsupervised_learning_trading_applications",
      "coverage_tags": [
        "unsupervised_learning_trading_applications",
        "k_means_clustering"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_clustering_application",
      "learning_goal": "学生能解释如何将K均值聚类应用于股票聚类，并说明其目的和潜在价值。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "K均值聚类",
          "en": "K-Means Clustering"
        },
        {
          "display": "股票聚类",
          "en": "Stock Clustering"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "输入数据",
            "算法步骤",
            "实际用途"
          ],
          "question_id": "q_long_clustering_application_v1",
          "reference_answer": [
            "输入数据：每只股票的历史日收益率序列，形成一个向量。例如，对于7只股票，我们有一个7×T的矩阵，其中T是时间周期数。",
            "算法步骤：首先，选择要形成的簇的数量K（例如K=2）。然后，随机初始化K个中心点。接着，重复以下两步直到收敛：1) 将每只股票的收益率向量分配给距离最近的中心点；2) 重新计算每个簇的中心点，即该簇内所有股票向量的平均值。",
            "实际用途：1) 投资组合分散化：从不同簇中选择股票，因为不同簇的股票走势相关性较低，可以降低风险。2) 配对交易：同一簇内的股票走势相似，如果它们的价格出现背离，可能产生套利机会。3) 市场细分：自动识别出科技股、银行股等不同板块，无需依赖外部标签。"
          ],
          "rubric_points": [
            "正确指出输入数据是股票的历史收益率时间序列。",
            "简要描述K均值算法的核心步骤：初始化K个中心点、分配数据点到最近中心、更新中心点、迭代至收敛。",
            "至少提到一个实际用途，例如：构建分散化投资组合（从不同簇选股）、发现配对交易机会（同一簇内股票）、或进行行业分类验证。"
          ],
          "stem": "解释如何使用K均值聚类对一组股票进行聚类。请说明输入数据是什么、算法如何工作、以及这种聚类结果对交易者有什么实际用途。"
        },
        {
          "estimated_seconds": 150,
          "prompt_blocks": [
            "数据准备",
            "距离度量选择",
            "结果解读与交易决策"
          ],
          "question_id": "q_long_clustering_application_v2",
          "reference_answer": [
            "数据准备：首先，从数据源（如Yahoo Finance）下载所有500只股票过去一年的日收盘价。然后，计算每只股票的日收益率序列。最后，处理任何缺失值（例如，删除或填充）。最终得到一个500×T的矩阵，其中T是交易日数。",
            "距离度量选择：如果我的目标是发现具有相似价格行为模式的股票（例如，同涨同跌），我会选择余弦距离，因为它对幅度不敏感，能更好地捕捉形状相似性。如果我的目标是识别具有相似风险水平（波动率）的股票，我会选择欧几里得距离。",
            "结果解读与交易决策：假设聚类结果将股票分为三组：簇1包含高增长科技股（如NVDA, AMD），簇2包含大型银行股（如JPM, BAC），簇3包含防御性公用事业股（如DUK, SO）。基于此，我可以：1) 构建一个分散化投资组合，从每个簇中各选一只股票。2) 如果我发现簇1中的两只股票（如NVDA和AMD）历史上走势高度相关，但近期出现背离，我可以考虑进行配对交易。"
          ],
          "rubric_points": [
            "正确描述数据准备过程：获取历史价格数据，计算日收益率，处理缺失值。",
            "能根据分析目标讨论距离度量的选择，例如，如果关注形状则选余弦距离，如果关注波动幅度则选欧几里得距离。",
            "能合理解读聚类结果，例如，发现一个簇全是高波动科技股，另一个簇是低波动公用事业股，并据此提出交易策略。"
          ],
          "stem": "假设你是一位量化分析师，你决定使用K均值聚类（K=3）对标准普尔500指数中的股票进行聚类。请解释你将如何准备数据、如何选择距离度量、以及如何解读聚类结果来辅助你的交易决策。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "distance_measures",
      "coverage_tags": [
        "distance_measures",
        "euclidean_distance",
        "cosine_distance"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_distance_choice",
      "learning_goal": "学生能在具体场景中选择合适的距离度量。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "欧几里得距离",
          "en": "Euclidean Distance"
        },
        {
          "display": "余弦距离",
          "en": "Cosine Distance"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 25,
          "explanation": "余弦距离关注向量的方向（形状）而非大小，因此对于形状相似但幅度不同的曲线，余弦距离会认为它们很接近。",
          "options": [
            "欧几里得距离",
            "余弦距离",
            "曼哈顿距离",
            "切比雪夫距离"
          ],
          "question_id": "q_quiz_distance_choice_v1",
          "stem": "假设你使用K均值聚类对一组股票进行聚类。你发现股票A和股票B的收益率曲线形状几乎完全一致，但股票A的波动幅度是股票B的10倍。在这种情况下，使用哪种距离度量更可能将它们分到同一个簇？"
        },
        {
          "answer": 0,
          "estimated_seconds": 25,
          "explanation": "欧几里得距离计算的是两点之间的直线距离，对数值的绝对大小敏感，因此能区分不同波动水平的股票。",
          "options": [
            "欧几里得距离",
            "余弦距离",
            "相关系数距离",
            "马氏距离"
          ],
          "question_id": "q_quiz_distance_choice_v2",
          "stem": "在股票聚类中，如果你希望将具有相似绝对收益率水平的股票（例如，两只股票每日收益率都在±0.5%附近波动）分在一起，而不仅仅是形状相似，你应该优先考虑哪种距离度量？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "# L10: Machine learning use cases in algorithmic trading\nCourse Code: COMP7415\n# Agenda\n- Major categories of machine learning algorithms\n- Multiple use cases of machine learning in algo-trading\n- The future of smart investment systems\n# Introduction to Machine Learning\n# What is Machine Learning?\n- A subset of artificial intelligence (AI) that enables systems to learn from data, identify patterns, and make decisions with minimal human intervention.\nKey Characteristics:\nData-driven\n- Adaptive learning\n# Types of Machine Learning\n1. Supervised Learning\n2. Unsupervised Learning\n3. Reinforcement Learning\n# 1. Supervised Learning\n- Involves training a model on a labelled dataset where the outcome is known.\n- Two main category of algorithms\n1. Classification\n- predict categorical target variables\n• Eg. SVM, Logistic Regression, Decision Tree, CNN, etc\n2. Regression\n- predict continuous target\n- Eg. Linear regression, decision tree, random forest, etc\n![](images/2c7f41adf6657f9c556f1952ede0d622004cae58e8658435abe8f3f9a25cb776.jpg)\n# 1. Supervised Learning\n# Pros:\n- Typically delivers high accuracy if the training data is representative and of good quality.\n- The presence of labelled data provides a clear objective for the model, making it easier to evaluate performance.\n- Model Interpretability: Many supervised learning algorithms offer insights into feature importance and decision-making processes.\n# Cons:\n- Requires a large amount of labelled data, which can be expensive and time-consuming to obtain.\n- The model can only predict outcomes it has been trained on, making it ineffective for novel or unrepresented scenarios.\n# 2. Unsupervised Learning\n- Discover patterns and relationship in data without labelling\n- Two main category of algorithms\n1. Clustering\n- group data points into clusters based on their similarity\n- Eg. K-mean clustering, PCA, etc\n2. Association\n- identifies rules that indicate the presence of one item implies the presence of another item\n- Eg. Apriori algorithm, Eclat algorithm, etc\n![](images/841e78a2dd1f0d08deed091369328a310cbb529bf0e4ab734e4ee13bda407130.jpg)\nAssociation Rule Learning\n![](images/87966d3f21cafb5bfdd01a0130ef5f8b040d729f4279727210ce3927cd75ece5.jpg)\n\"93% of people who purchased item A also purchased item B\"\n# 2. Unsupervised Learning\n# Pros:\n- Can work with unlabelled data, making it easier to gather and analyze large datasets without the need for extensive labelling.\n- Capable of uncovering complex structures and relationships in data that may not be apparent, leading to new insights.\n# - Cons:\n- Results can be difficult to interpret, as there are no clear targets or labels to guide understanding of the outcomes.\n- May not yield as high predictive accuracy as supervised learning methods, especially for specific tasks like classification.\n- Without labelled data, there's a risk of identifying patterns that are not meaningful or relevant, leading to incorrect conclusions.\n# 3. Reinforcement Learning\n- An agent interacts with the environment by producing actions and discovering errors, and learns to make decisions by maximizing cumulative reward.\n- Common algorithms:\nQ-learning\nSARSA (State-Action-Reward-State-Action)",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "{\n  \"lesson_id\": \"L10\",\n  \"mode\": \"guided_story\",\n  \"steps\": [\n    {\n      \"concept\": \"机器学习三大类型概览\",\n      \"file\": \"research/pipeline/3-guided_story/L10/step1/step.json\",\n      \"sequence_id\": \"step1\"\n    },\n    {\n      \"concept\": \"监督学习：分类与回归\",\n      \"file\": \"research/pipeline/3-guided_story/L10/step2/step.json\",\n      \"sequence_id\": \"step2\"\n    },\n    {\n      \"concept\": \"无监督学习：聚类与关联\",\n      \"file\": \"research/pipeline/3-guided_story/L10/step3/step.json\",\n      \"sequence_id\": \"step3\"\n    },\n    {\n      \"concept\": \"强化学习：Q学习与交易\",\n      \"file\": \"research/pipeline/3-guided_story/L10/step4/step.json\",\n      \"sequence_id\": \"step4\"\n    },\n    {\n      \"concept\": \"监督学习在交易中的应用：从规则到数据\",\n      \"file\": \"research/pipeline/3-guided_story/L10/step5/step.json\",\n      \"sequence_id\": \"step5\"\n    },\n    {\n      \"concept\": \"无监督学习在交易中的应用：股票聚类\",\n      \"file\": \"research/pipeline/3-guided_story/L10/step6/step.json\",\n      \"sequence_id\": \"step6\"\n    },\n    {\n      \"concept\": \"强化学习在交易中的应用：Q学习交易员\",\n      \"file\": \"research/pipeline/3-guided_story/L10/step7/step.json\",\n      \"sequence_id\": \"step7\"\n    },\n    {\n      \"concept\": \"挑战与未来：AI交易的边界\",\n      \"file\": \"research/pipeline/3-guided_story/L10/step8/step.json\",\n      \"sequence_id\": \"step8\"\n    }\n  ]\n}",
    "plain_text": "pipeline/1-plain/L10/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "# L10: Machine learning use cases in algorithmic trading\nCourse Code: COMP7415\n# Agenda\n- Major categories of machine learning algorithms\n- Multiple use cases of machine learning in algo-trading\n- The future of smart investment systems\n# Introduction to Machine Learning\n# What is Machine Learning?\n- A subset of artificial intelligence (AI) that enables systems to learn from data, identify patterns, and make decisions with minimal human intervention.\nKey Characteristics:\nData-driven\n- Adaptive learning\n# Types of Machine Learning\n1. Supervised Learning\n2. Unsupervised Learning\n3. Reinforcement Learning\n# 1. Supervised Learning\n- Involves training a model on a labelled dataset where the outcome is known.\n- Two main category of algorithms\n1. Classification\n- predict categorical target variables\n• Eg. SVM, Logistic Regression, Decision Tree, CNN, etc\n2. Regression\n- predict continuous target\n- Eg. Linear regression, decision tree, random forest, etc\n![](images/2c7f41adf6657f9c556f1952ede0d622004cae58e8658435abe8f3f9a25cb776.jpg)\n# 1. Supervised Learning\n# Pros:\n- Typically delivers high accuracy if the training data is representative and of good quality.\n- The presence of labelled data provides a clear objective for the model, making it easier to evaluate performance.\n- Model Interpretability: Many supervised learning algorithms offer insights into feature importance and decision-making processes.\n# Cons:\n- Requires a large amount of labelled data, which can be expensive and time-consuming to obtain.\n- The model can only predict outcomes it has been trained on, making it ineffective for novel or unrepresented scenarios.\n# 2. Unsupervised Learning\n- Discover patterns and relationship in data without labelling\n- Two main category of algorithms\n1. Clustering\n- group data points into clusters based on their similarity\n- Eg. K-mean clustering, PCA, etc\n2. Association\n- identifies rules that indicate the presence of one item implies the presence of another item\n- Eg. Apriori algorithm, Eclat algorithm, etc\n![](images/841e78a2dd1f0d08deed091369328a310cbb529bf0e4ab734e4ee13bda407130.jpg)\nAssociation Rule Learning\n![](images/87966d3f21cafb5bfdd01a0130ef5f8b040d729f4279727210ce3927cd75ece5.jpg)\n\"93% of people who purchased item A also purchased item B\"\n# 2. Unsupervised Learning\n# Pros:\n- Can work with unlabelled data, making it easier to gather and analyze large datasets without the need for extensive labelling.\n- Capable of uncovering complex structures and relationships in data that may not be apparent, leading to new insights.\n# - Cons:\n- Results can be difficult to interpret, as there are no clear targets or labels to guide understanding of the outcomes.\n- May not yield as high predictive accuracy as supervised learning methods, especially for specific tasks like classification.\n- Without labelled data, there's a risk of identifying patterns that are not meaningful or relevant, leading to incorrect conclusions.\n# 3. Reinforcement Learning\n- An agent interacts with the environment by producing actions and discovering errors, and learns to make decisions by maximizing cumulative reward.\n- Common algorithms:\nQ-learning\nSARSA (State-Action-Reward-State-Action)"
  },
  "target_language": "zh-CN"
}

]
</QUESTION_BANK>

<PLAIN_TEXT>
# L10: Machine learning use cases in algorithmic trading

Course Code: COMP7415

# Agenda

- Major categories of machine learning algorithms   
- Multiple use cases of machine learning in algo-trading   
- The future of smart investment systems

# Introduction to Machine Learning

# What is Machine Learning?

- A subset of artificial intelligence (AI) that enables systems to learn from data, identify patterns, and make decisions with minimal human intervention.   
Key Characteristics:

Data-driven   
- Adaptive learning

# Types of Machine Learning

1. Supervised Learning   
2. Unsupervised Learning   
3. Reinforcement Learning

# 1. Supervised Learning

- Involves training a model on a labelled dataset where the outcome is known.   
- Two main category of algorithms

1. Classification

- predict categorical target variables   
• Eg. SVM, Logistic Regression, Decision Tree, CNN, etc

2. Regression

- predict continuous target   
- Eg. Linear regression, decision tree, random forest, etc

![](images/2c7f41adf6657f9c556f1952ede0d622004cae58e8658435abe8f3f9a25cb776.jpg)

# 1. Supervised Learning

# Pros:

- Typically delivers high accuracy if the training data is representative and of good quality.   
- The presence of labelled data provides a clear objective for the model, making it easier to evaluate performance.   
- Model Interpretability: Many supervised learning algorithms offer insights into feature importance and decision-making processes.

# Cons:

- Requires a large amount of labelled data, which can be expensive and time-consuming to obtain.   
- The model can only predict outcomes it has been trained on, making it ineffective for novel or unrepresented scenarios.

# 2. Unsupervised Learning

- Discover patterns and relationship in data without labelling   
- Two main category of algorithms

1. Clustering

- group data points into clusters based on their similarity   
- Eg. K-mean clustering, PCA, etc

2. Association

- identifies rules that indicate the presence of one item implies the presence of another item   
- Eg. Apriori algorithm, Eclat algorithm, etc

![](images/841e78a2dd1f0d08deed091369328a310cbb529bf0e4ab734e4ee13bda407130.jpg)

Association Rule Learning   
![](images/87966d3f21cafb5bfdd01a0130ef5f8b040d729f4279727210ce3927cd75ece5.jpg)  
"93% of people who purchased item A also purchased item B"

# 2. Unsupervised Learning

# Pros:

- Can work with unlabelled data, making it easier to gather and analyze large datasets without the need for extensive labelling.   
- Capable of uncovering complex structures and relationships in data that may not be apparent, leading to new insights.

# - Cons:

- Results can be difficult to interpret, as there are no clear targets or labels to guide understanding of the outcomes.   
- May not yield as high predictive accuracy as supervised learning methods, especially for specific tasks like classification.   
- Without labelled data, there's a risk of identifying patterns that are not meaningful or relevant, leading to incorrect conclusions.

# 3. Reinforcement Learning

- An agent interacts with the environment by producing actions and discovering errors, and learns to make decisions by maximizing cumulative reward.   
- Common algorithms:

Q-learning   
SARSA (State-Action-Reward-State-Action)

![](images/e1b42db4f694770e010297843a9d058039ba0e5936b7c3577b7475ec30d34db9.jpg)

![](images/d70e700ed849fd1f1fd0f29d603b91cc4da9c988759b85dfb632af1b6cccf638.jpg)

# 3. Reinforcement Learning

# Pros:

- Learn optimal strategies through trial and error, minimizing the need for human intervention and supervision.   
- Can adapt to dynamic environments, continuously improving performance based on feedback and changing conditions.   
- Well-suited for complex problems with long-term decision-making, such as game playing and robotic control, where actions have delayed rewards.

# - Cons:

- Requires a large number of interactions with the environment to learn effectively, which can be time-consuming and resource-intensive.   
- Define appropriate reward functions can be challenging and may lead to unintended behaviors if not designed carefully.

# Summary

<table><tr><td></td><td>Description</td><td>Data Requirement</td><td>Common Use Cases</td></tr><tr><td>Supervised Learning</td><td>Learns from labelled data to make predictions.</td><td>Labeled data is required.</td><td>Classification, regression tasks.</td></tr><tr><td>Unsupervised Learning</td><td>Finds patterns or structures in unlabelled data.</td><td>No labelled data needed.</td><td>Clustering, dimensionality reduction.</td></tr><tr><td>Reinforcement Learning</td><td>Learns through trial and error by receiving feedback.</td><td>Interaction with environment.</td><td>Game playing, robotics, autonomous systems.</td></tr></table>

# Supervised Learning in Algo-Trading

# What is a Trading Strategy?

$$
y = f (x)
$$

# Outputs:

- action (i.e. buy/ sell/ do nothing)   
quantity   
price   
- stop loss   
- take profit

.

![](images/566516d15d40e768aaca6564ceedef4daa9a3a61cd2ee75dbdf9e1cfa3784893.jpg)

# Model / Logic:

- technical analysis   
fundamental analysis   
- pricing model   
statistical arbitrage   
- regression

![](images/3b3c7d91d439d9a576a06f5a543a9306565e48216ddd7cf3b8698c1fa583ae58.jpg)

.

# Inputs:

timeframe   
price   
volume   
order book   
- technical indicators   
fundamental data   
news   
.

![](images/f32b19a2ac430e072b7732bf7ae4a4326b543416621e5a698ba949906838f7cf.jpg)

# RSI Example

![](images/ebd387f4c66bebdf6618c8c27d2d52c9e901449be463e572a094aa42a9adfe80.jpg)

# Inputs:

- market price   
14-day RSI value

![](images/6d0969e9f1981380ae6d621c3b810d9ff9e69968fad5e72856dbd7655ed299e0.jpg)

# Logic:

- buy if RSI < 30   
- sell if RSI > 70   
- do nothing otherwise

# Output:

- buy/sell action

EUR/USD 1 hour

RSI (14, 70, 30)

![](images/e612662b9576d890cafc1173cbcc29e883d8b8fc8f4fab2d2345999e715b8bc3.jpg)

RSI 14

70

![](images/a1dc31e39740395ae33c7b2ac669aa25c431dd4e71e29370ebb1469f4cf1ea21.jpg)

30

# Traditional vs AI Approach

![](images/91d379f0174a54bead4802482a6be0764a6a66dca4da3b08d8e74912da9b4c83.jpg)

Rule driven: define a rule, then see what happens later.

![](images/dc464f60c9ded7eb6b670f7aab7873d36d73581f27e574b85dcfb732f8ae44e7.jpg)

Result driven: identify a desirable trade, then see what happened earlier.

![](images/d96ee1d2f703766e02cc0133e8cd0c0e03d1315dc62308efcf346560c91c9093.jpg)

# RSI Example

# Output:

- desire to buy at red points   
- desire to sell at yellow points

# Inputs:

- market price   
- any technical indicators

# Logic:

What should be the hidden rules?

![](images/be7d1a5c25e296d057632dd08f16ec9e03a7d7252bb8d6d1066424d4a64054af.jpg)

![](images/7baa2aee6807e68a9b97c25818210132d1b918e1851a1d52f359706a35cbd896.jpg)

# Unsupervised Learning in Algo-Trading

# Overview

- Unsupervised learning analyzes financial data without labelled outcomes   
- Applications in Trading:

- Clustering for Market Segmentation

- Grouping similar stocks or assets based on historical price movements or features to identify trends and potential investment opportunities

- Anomaly Detection

Identifying unusual market behaviors or price movements that may signal trading opportunities or risks, helping traders react to potential market shifts

- Feature Extraction

- Reducing dimensionality of financial datasets (eg. PCA) to highlight key factors influencing asset prices, aiding in more effective modeling

# Stock Clustering

- Involves grouping stocks based on their historical price movements or other financial metrics, revealing similarities that can inform trading strategies.   
- Use cases:

Identifying Investment Opportunities

- Helps in discovering stocks that are moving together, indicating potential correlations   
Further develop pair-trading/ co-integration strategies

- Portfolio Diversification

- Build diversified portfolios by selecting stocks from different clusters

# K-mean Clustering

- Objective: Minimize the variance within each cluster while maximizing the variance between clusters.   
How it works

1. Initialization: Randomly select K initial centroids from the dataset   
2. Assignment: Assign each data point to the nearest centroid based on Euclidean distance.   
3. Update: Recalculate the centroids as the mean of the assigned data points.   
4. Repeat: Iterate the assignment and update steps until convergence (i.e., centroids no longer change).

# K-mean Clustering

1. Define the number of cluster K   
2. Initialize K centroids randomly   
3. Repeat until convergence:

a. For each data point, assign it to the nearest centroid   
b. For each centroid, update the centroid to the mean of assigned points

![](images/d758f79d5f1441c8e3d1ab706030746fccb644f22e622678a01f32b56b35cdc9.jpg)  
(a)

![](images/91a67dd9dd396fbdab691c04c0e2cb53630b130c12cb1a76d7da91c0a87173de.jpg)  
(d)

![](images/c75b111cbbb83a64fecc0f6ef118f23a0e83728ec8243158c0fbfb7dfd3d26bf.jpg)  
(b)

![](images/c3ed30320926ed5e50714e124170992b3c2bd42d4c7348ece563329956b108be.jpg)  
(e)

![](images/b8e7e1ab68eb2a43feec4a1e2c9d1f0c45f6868491f6e562dde53cd7293c4960.jpg)  
(c)

![](images/a69f1a2d70fa6ce7ea4724db7ea4fae4fbb39762c22999ec5f17d57093fb6d0b.jpg)  
(f)

# K-means in Python

![](images/d6dedae7c3d84aca8a13f5b3c8927fa2d5a3db44dfe17dcddaa5ce4a88a9fb4a.jpg)

import numpy as np  
import matplotlib.pyplot as plt  
from sklearn.cluster import KMeans

Generating synthetic data  
np.random.seed(42)  
X1 = np.random.randint(50, 2) + np.array([0, 0]) # Cluster 1  
X2 = np.random.randint(50, 2) + np.array([5, 5]) # Cluster 2  
X3 = np.random.randint(50, 2) + np.array([10, 0]) # Cluster 3

Combine clusters into one dataset X = np.vstack([X1, X2, X3])

Applying K-Means kmeans $=$ KMeans(n_Clusters $\coloneqq$ 3, random_state $\coloneqq$ 0).fit(X) #Getting the cluster centers and labels centroids $=$ kmeans.clustercenters_ labels $=$ kmeans.labels

Plotting the results  
pltscatter(X(:, 0], X(:, 1], c=labels, s=100, cmap='viridis')  
pltscatter(centroids(:, 0], centroids(:, 1], c='red', s=200, alpha=0.75)  
plt.title('K-Means Clustering')  
plt.xlabel('Feature 1')  
pltylabel('Feature 2')  
plt.show()

# K-means for stock cluster

- Group the following 7 stocks into 2 clusters, based on their historical returns in the year of 2024

1. MSFT: Microsoft   
2. GOOG: Google   
3. AAPL: Apple   
4. TSLA: Tesla   
5. NVDA: Nvidia   
6. JPM: JP Morgan   
7. BAC: Bank of America

# K-means for stock cluster

import pandas as pd

from sklearn.cluster import KMeans

import yfinance as yf

Define the stock symbols

stocks = ['MSFT', 'GOOG', 'AAPL', 'TSLA', 'NVDA', 'JPM', 'BAC']

Collect historical price data for the last year

data = yf.download(stocks, start='2024-01-01', end='2024-12-31')['Close'])

Calculate daily returns

returns = data_pct_change().dropna()

Apply K-Means clustering

kmeans = KMeans(n_clusters=2, random_state=0)

kmeans.fit returns.T) #Transpose to cluster stocks instead of time periods

Get the cluster labels

labels = kmeans.labels_

Create a DataFrame to hold the results

results = pd.DataFrame({'Stock': stocks, 'Cluster': labels})

print(results)

![](images/4f571d666f727a4785af403b70df9560a5448f4de168584ccde2c8b061c5dc42.jpg)

# Stock Cluster

0 MSFT 0   
1 GOOG   
2 AAPL   
3 TSLA   
4 NVIDIA   
5 JPM   
6 BAC

# K-means for stock cluster

- By default, scikit-learn uses Euclidean distance for calculating the distance between data points (in above case, the feature vectors representing the time series of return).   
- For 2 time series,

• stock A = {rA,1, rA,2, ..., rA,T}   
• stock B = {rB,1, rB,2, ..., rB,T}

- The Euclidean distance will be

$$
d (A, B) = \sqrt {\sum_ {i = 1} ^ {T} (r _ {A , i} - r _ {B , i}) ^ {2}}
$$

![](images/b49e203b677daee6ce23e99802b5390c8cd29c3b287554a82c32e4429c5f7fff.jpg)  
Euclidean

# Cosine Distance

$\cos (\theta) = \frac{A \cdot B}{\| A \| \| B \|}$   
$\mathrm{d}(\mathrm{A}, \mathrm{B}) = 1 - \cos (\theta) = 1 - \frac{\mathrm{A} \cdot \mathrm{B}}{\|\mathrm{A}\| \|\mathrm{B}\|}$

where

$\left\| A\right\| = \sqrt{\sum_{i = 1}^{T}r_{A,i}{}^{2}}$   
$\left\| B\right\| = \sqrt{\sum_{i = 1}^{T}r_{B,i}{}^{2}}$   
$A \cdot B = \sum_{i=1}^{T} r_{A,i} \times r_{B,i}$

- $\cos (\theta)$ is ranging from -1 to 1   
- Thus $d(A, B)$ is ranging from 0 to 2   
- If $d(A, B)$ is close to 0, the 2 datasets are very similar or close to each other.

![](images/8245363cdde6e2a89196f16dd3817aee4780a1912f877b8c423b1cb77eea6997.jpg)  
Cosine

# Correlation-Based Distance

• Value is ranging from 0 to 2   
- When the distance $d$ is close to 0, the 2 datasets are very similar

$$
d (A, B) = 1 - \operatorname {c o r r} (A, B) = 1 - \frac {\operatorname {C o v} (A , B)}{\sqrt {\operatorname {V a r} (A) \operatorname {V a r} (B)}}
$$

# Limitation of above distance measures

- Assumes that the time series are aligned and of equal length.   
- If the time series have different lengths or require alignment (eg. due to different time frames), we need to preprocess the data accordingly.

# Limitation of K-means

- The results is sensitive to the initial placement of centroids, potentially leading to different cluster assignments in different runs.   
- The algorithm requires to specify the number of clusters (k) in advance, which can be challenging if the optimal number is unknown.   
- K-Means is sensitive to outliers, as they can disproportionately influence the position of the centroids, skewing the clustering results.

# Reinforcement Learning in Algo-Trading

# Overview

- A type of machine learning where an agent learns to make decisions by taking actions in an environment to maximize cumulative rewards.   
• Key components:

- Agent: The trading algorithm.   
- Environment: The stock market.   
• Actions: (eg. buy, sell, hold)   
- Rewards: Profit or loss from trades

![](images/673b7352e664de945cdb1b846afc03ca10ec83ad89f294cd8adf19332b253e8b.jpg)  
SCORE: 0

# Q-Learning Algorithm

- A model-free reinforcement learning algorithm used to find the optimal action-selection policy for an agent interacting with an environment.   
- Enable the agent to learn how to maximize cumulative rewards over time by estimating the value of taking certain actions in given states.   
- Denote:

State (s): A representation of the current situation in the environment   
- Action (a): The choices available to the agent that can change the state.   
- Reward (r): A feedback signal received after taking an action in a particular state, indicating the immediate benefit of that action.   
- Q-Value ( $Q(s, a))$ ): Represents the expected future rewards for taking action $a$ in state $s$ .   
- Policy $(\pi)$ : The agent's strategy in deciding actions based on the current state

# Q-Learning Steps

1. Initialization: Start by initializing the Q-table with zeros for all state-action pairs.   
2. Exploration: At each step, choose an action based on the current state, either by exploring (choosing a random action) or exploiting (choosing the action with the highest Q-value).   
3. Taking Action: Execute the chosen action, observe the reward and the next state.   
4. Update Q-Value: Use the Q-learning formula to update the Q-value for the state-action pair.   
5. Iterate: Repeat the process for many episodes, allowing the agent to learn the optimal policy over time.

# Q-Learning Formula

- Q-value update rule:

$$
Q (s, a) \leftarrow Q (s, a) + \alpha \left(r + \gamma \max  _ {a ^ {\prime}} Q \left(s ^ {\prime}, a ^ {\prime}\right) - Q (s, a)\right)
$$

# where

- Q(s,a): Current estimated value of taking action $a$ in state $s$ .   
- $\alpha$ : Learning rate ( $0 < \alpha \leq 1$ ), determines how much new information overrides the old information.   
• r: Immediate reward received after taking action a from state s.   
- $\gamma$ : Discount factor $(0 \leq \gamma < 1)$ , which balances the importance of immediate and future rewards.   
• s': The new state after taking action a.   
- $\pi(a) = \max Q(s', a')$ : The maximum predicted Q-value for the next state, indicating the best possible action from state $s'$ .

# Example

![](images/ce1f90369ebb787c385565b0ee01f5282e3f3291629fe727fa8b958a8b93bcc3.jpg)

![](images/1845dc159af134d09360df0caea66e6a26d86db29c1d6d159d32e87d0504d693.jpg)

# Heuristic Function

- A heuristic function is a technique used to improve the efficiency of a search algorithm.   
- It provides an estimate of the cost to reach a goal state from a given state.   
• Role in Q-Learning

- In reinforcement learning, especially Q-learning, heuristic functions can guide the agent's exploration strategy.   
Utilizing prior knowledge about the environment to prioritize actions that are more likely to lead to better rewards.

# Heuristic Function Example

- In grid-based environments, estimating the distance to the goal can guide the agent's movements.   
- In a pac-man game, we want the agent to

- Move towards food   
- Avoid moving towards ghosts

![](images/64bb74b37e0a65227e0b5db2e1d04787c60d26ebbcb3f098460bcfedae7b05fd.jpg)

# Benefits of Using Heuristic Functions

- Faster Convergence: Heuristics can speed up the learning process by focusing on promising actions.   
- Improved Performance: They can lead to higher cumulative rewards by avoiding suboptimal paths.   
- Efficient Exploration: Heuristics reduce the amount of exploration needed by directing the agent towards beneficial states.

# Apply Q-Learning to Trading

# Define the basic structure

State Space:

- Current market conditions (eg. price, volume)

- Action Space:

- Only 3 possible actions

1: buy

- 1: sell

• 0: hold or do nothing

- Reward Function:

Profit or loss from an action

# Python Example

# - Define trading environment

import numpy as np

import pandas as pd

import random

class TradingEnvironment:

def __init__(self, data):

self.data = data

self.current_step = 0

selfbalance $= 1000$ #Starting balance

selfshares $= 0$

def reset(self):

self.current_step = 0

selfbalance $= 1000$

selfshares $= 0$

return self.current_step # Return the current step as the state index

def step(self, action):

daily_return = self.data.iloc[self.current_step]

reward = 0

if action == 1: # Buy

self shares $+ = 1$

selfbalance $\text{一} =$ (1+daily_return)

elif action == 2: # Sell

long only

if self shares $>0$

selfshares $\text{一} = 1$

self.balance $\stackrel{*}{=}$ (1-daily_return)

reward = selfbalance - 1000 # Total profit

self.current_step += 1

done = self.current_step >= len(self.data) # Check if done

terminal states

if done:

next_state = None # No next state if done # non-terminal states

else:

next_state = self.current_step # Current step as next state

return next_state, reward, done # Return the next state

# - Define q-learning agent

class QLearningAgent: def __init__(self, state_size, action_size): self.q_table = np.zeros((state_size, action_size)) self-learning_rate = 0.1 selfdiscount_factor = 0.95 self.explorationprob $= 0.9$ def choose_action(self,state): if random.uniform(0,1) $<$ self.explorationprob: return random.choice(len(self.q_table[0])) # Explore else: return np.argmax(self.q_table[state]) # Exploit   
def learn(self,state,action,reward,next_state): if next_state is not None:# Only learn if next_state is not terminal best_next_action $=$ np.argmax(self.q_table[next_state]) td_target $=$ reward $^+$ self(discount_factor\*self.q_table(next_state)[best_next_action] tdOTA $=$ td_target-self.q_table[state][action] self.q_table[state][action] $+ =$ self_learning_rate\*tdOTA

# - Iterate the game for 1000 times

Sample price data   
```python
data = pd.Series([100, 101, 102, 99, 98, 100, 105])
dailyreturns = (data - data-shift(1)) / data-shift(1)
dailyreturns = dailyreturns.dropna() 
```

env = TradingEnvironment(dailyreturns) # Define the number of states and actions   
```python
state_size = len(data) # Number of steps as states  
action_size = 3 # Actions: Hold (0), Buy (1), Sell (2)  
agent = QLearningAgent(state_size, action_size) # Training the agent 
```

for episode in range(1000):   
state $=$ env.reset()   
done $=$ False   
while not done: action $=$ agent.choose_action(state) next_state, reward, done $=$ env.step(action) agentlearn(state, action, reward, next_state) state $=$ next_state if next_state is not None else state # Stay in the same state if done

```txt
Print the final policy  
final_policy = np.argmax(agent.q_table, axis=1)  
action_labels = {0: 'Hold', 1: 'Buy', 2: 'Sell'} 
```

```python
print("Final Policy:", final_policy)  
for state, action in enumerate(final_policy):  
    print(f"State {state}: {action_labels[action]}") 
```

![](images/604d31ac06c44ca5a7476f4b46a221ff422bb461fdbe7051a2160db2c795dd8e.jpg)

```txt
Final Policy: [1 1 2 2 1 0 0]  
State 0: Buy  
State 1: Buy  
State 2: Sell  
State 3: Sell  
State 4: Buy  
State 5: Hold  
State 6: Hold 
```

# Considerations

How to define a market state?

Daily returns, volume, market trend, volatility, etc?   
- Unlike Go, financial market is a non-finite and continuous environment

• How to define the action space?

- In previous Q-learning example, we set {buy, sell, do nothing}   
- It doesn’t reflect all possible trading actions in reality (eg. place a limit order at different price levels, dynamic trading size, etc)

- Given the current market condition, there are many possibilities for the future. Which one is the most likely to happen?

- Is it possible to apply Monte Carlo tree search to simulate future market states?   
- Can the trading agent self-learn from the simulated environment, like the self-playing AlphaGo?

![](images/f01b61f8c1e58fe30afd26be5e6d682485a80a01b329a6930759af0a667e9c34.jpg)

# Limitation of Q-Learning in Trading

- High Dimensionality & Computational Complexity   
- Trading environments often have large state and action spaces (eg. multiple assets, different strategies) which may require significant computational power and time to train Q-learning models.   
Non-Stationary Environment   
- Financial markets are influenced by numerous factors that change over time.   
- Q-learning assumes a stationary environment, which may lead to outdated policies as market conditions evolve.   
Overfitting Risks   
- Q-learning models may overfit to historical price data, performing well in backtesting but poorly in live trading.   
- Strategies that work in one market condition may not generalize to others.

# The Future of Investment

B BTC

$105,888.00

ETH

$3,546.05

SOL

\$166.72

#

$984.00

DOGE

$0.1794

XRP

$2.54

$ %

ALL 72H

![](images/b9a02475ebd878bce7ca3bce726e36053a4e0434ada5bb100c9aa109e88862b3.jpg)  
TOTAL ACCOUNT VALUE

# Emerging Technologies in Algo-Trading

- Sentiment Analysis & Natural Language Processing (NLP)   
Big Data Analytics   
AI and Machine Learning   
- Explainable AI (XAI)   
- Blockchain   
- Cloud & Edge Computing   
- Quantum computing

# A smart investment system should be able to

- Predict which direction the markets and assets will move   
- Detect the potential risks and their impact   
- Choose the best timing to trade   
- Optimize for transaction costs   
- Manage the position, risk and capital   
- Explain the decisions it made   
- Generate new investment strategies   
- Know what other investors think and predict the actions they will take

0

# Questions

1. Will AI or investment bot beat the best fund manager?   
2. Given that you have access to all data in the world (both real-time and historical, public and private), are you able to accurately predict a stock price 10 minutes later?   
3. In a world dominated by trading algorithms,

What will be the role of human traders?   
- Will the market become more or less volatile?

# Key Takeaways

- Understand different types of machine learning algorithms   
- Learn multiple use cases of machine learning in algo-trading

Data labelling on desired trades   
Stock segmentation   
Q-learning

- Understand the limitation of AI, so synthesis of human and AI is important
</PLAIN_TEXT>

<RELATED_IMPORTANT>
COMP7415A - Mastering the markets: Financial analytics and algorithmic trading
Semester 2, 2024-25
Professor	
Tony Lam
Syllabus	Algorithmic trading is a trending investment approach nowadays that consists of identification of trading opportunities and implementation via computer algorithms. This course will cover emerging trend in the quantitative investment field, and introduce various data analysis techniques and methodologies that are commonly employed in the industry.

The first half of the course focuses on financial data analysis and strategy implementation. The second half of the course discusses practical techniques to manage and deploy algorithmic trading strategies in real financial world.
Introduction by Professor	 
Learning Outcomes	
Course Learning Outcomes	
CLO1. A solid understanding of the nuances of algorithmic trading, design and implement algorithmic trading strategies	
CLO2. The ability to apply quantitative methods to analyze financial data and build financial models	
CLO3. The ability to formulate trading strategies, carry out backtesting, optimization, risk management and interpret investment performance	
CLO4. The ability to apply various investment theories and trading techniques to the real financial market	
CLO5. Familiar with the current trends, and understand the limitations and challenges in the field	
CLO6. Complete a capstone project that includes a full cycle of trading strategy development	
Pre-requisites	To succeed in this course, students are expected to have a foundation and basic knowledge in the following areas:
- Python programming
- Statistics and probability theory
- Mathematics and optimization theory
Compatibility	-
Topics covered	
Course Content	No. of Hours	Course Learning Outcomes
Algorithmic trading basics and financial markets	 	 
Data scraping and database management with Python	 	 
Building backtest framework and rule-based trading strategy	 	 
Statistical time series analysis for market classification	 	 
Statistical arbitrage and pairs trading	 	 
Capital and Risk Management	 	 
Performance measures and portfolio optimization	 	 
Order book and high frequency data modeling	 	 
Market practice in broker selection and account connection	 	 
Machine learning use cases in algorithmic trading	 	 
 
Assessment	
Description	Type	Weighting *	Tentative Assessment Period /
Examination Period ^	Course Learning Outcomes
Written assignment and project	Continuous Assessment	50%	-	 
Written examination covering all the taught contents in the course	Written Examination	50%	-	 
* The weighting of coursework and examination marks is subject to approval
^ The exact examination date uses to be released when all enrolments are confirmed after add/drop period by the Examinations Office.  Students are obliged to follow the examination schedule.  Students should NOT enrol in the course if they are not certain that they will be in Hong Kong during the examination period.  Absent from examination may result in failure in the course. There is no supplementary examination for all MSc curriculums in the Faculty of Engineering.
Course materials	
 

Session dates	
Date	Time	Venue	Remark
Session 1	5 Feb 2025 (Wed)	7:00pm - 10:00pm	LE-5	 
Session 2	12 Feb 2025 (Wed)	7:00pm - 10:00pm	LE-5	 
Session 3	19 Feb 2025 (Wed)	7:00pm - 10:00pm	LE-5	 
Session 4	26 Feb 2025 (Wed)	7:00pm - 10:00pm	LE-5	 
Session 5	5 Mar 2025 (Wed)	7:00pm - 10:00pm	LE-5	 
Session 6	19 Mar 2025 (Wed)	7:00pm - 10:00pm	LE-5	 
Session 7	26 Mar 2025 (Wed)	7:00pm - 10:00pm	LE-5	 
Session 8	2 Apr 2025 (Wed)	7:00pm - 10:00pm	LE-5	 
Session 9	9 Apr 2025 (Wed)	7:00pm - 10:00pm	LE-5	 
Session 10	16 Apr 2025 (Wed)	7:00pm - 10:00pm	LE-5	 
LE - Library Extension Building
Add/drop	20 January, 2025 - 12 February, 2025
Maximum class size	146
Back

</RELATED_IMPORTANT>

请直接输出 MDX，不要加解释。
