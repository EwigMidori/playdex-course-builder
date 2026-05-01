请基于以下 lesson 材料，生成一个结构化题库 JSON。

目标语言：
zh-CN

lesson_id：
L10

要求：
- 同时生成 `flashcard_families`、`quiz_families` 和 `longform_families`
- 题目必须只关联到当前 step：`step8`
- 所有 family 和 variant 的 `linked_steps` 都必须等于 `["step8"]`
- 同一个 family 必须至少给出 2 个 variants
- 题目应覆盖当前 step 的关键内容；不要把其它 step 的内容塞进这个 step 的题库
- `flashcard` 是间隔重复用的主动检索载体，不是选择题；每张卡必须有精准 front 与短 back
- `quiz` 才承载选择题、判断题、配对题等更像考试的小题
- `longform` 要能真正检查理解与表达
- 如果涉及公式，务必保证数值自洽
- 不要生成和输入材料脱节的题
- 输出里必须包含 `coverage_map`
- 每个 family 必须带 `coverage_tags`
- 每个 family 最好带 `term_refs`，注明关键术语的英文
- `coverage_map` 必须由输入中的 `coverage checklist`、`source outline`、`lesson map` 驱动
- `coverage_tags`、`concept_key` 应尽量复用或可追踪地归并这些输入中的关键项
- 不要写死任何学科、固定主题列表或固定 slide bucket
- 请把关键覆盖落实到 `coverage_map` 里，不要只在题目里隐含覆盖
- 不要把“名词正面 + 长定义背面”的被动阅读卡当成好 flashcard
- 不要把 `single_choice` 或 `true_false` 放进 `flashcard_families`
- 如果某个知识点适合选择题，请放入 `quiz_families`

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

<CURRENT_STEP_ID>
step8
</CURRENT_STEP_ID>

<CURRENT_STEP>
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

</CURRENT_STEP>

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

请直接输出 JSON，不要加解释。
