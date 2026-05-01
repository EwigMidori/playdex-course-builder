请基于以下材料，生成一份 lesson 级 MDX 课本。

目标语言：
zh-CN

lesson_id：
L0

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
- 每个主要 `##` section 必须带显式 `sectionId`，并与 `sectionMap` / `coverageTrace` 对齐
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
# Financial Analytics and
# Algorithmic Trading
Course Code: COMP7415
# Background of Lecturer
- Founder of AlgoGene (https://algogene.com)
Vice Chairman of Algo Challenge Association (https://algochallenge.org)
- Former algo developer, quant trader, risk manager, data analyst for hedge funds and banks.
- BSc in Math, Risk Management, Actuarial Science (HKU); MSc in Computer Science (HKU)
- Champion and awardee of several algo trading competitions, including
CCTV证券资讯频道《宽客天下·全球量化争霸赛》(2017/18)
- Rotman International Trading Competition (2017)
CASH Algo Trading Contest (2016)
WorldQuant Challenge (2014, 2015)
# Let’s play a game!
![](images/997b92b1751c1ce70437186759220c5d106c40ed0862fb47cf1033ee018d9aa5.jpg)
# 1. What’s your study mode?
a) Full time
b) Part time
c) Just sit in
# 2. What is your main reason for taking this course?
a) I want to start a career in algo-trading
b) I want to enhance my current job skills
c) I am interested in the topic and want to learn more
d) Just to fulfill graduation requirement
# 3. What do you expect to learn from this course?
a) Some skills for financial data analysis
b) I want to develop an automated trading system
c) Tell me some profitable trading strategies
d) Not sure yet
# 4. Do you have any prior experience in algo-trading?
a) Yes, I have extensive experience
b) Yes, I have some experience
c) No, but I have experience in other forms of trading
d) No, I am completely new to trading
# 5. Have you ever used any algo-trading platforms before?
a) Yes, I have used MetaTrader
b) Yes, I have used TradingView
c) Yes, I have used AlgoGene
d) No, I have no idea what it is
# 6. Which of the following has the largest market capitalization?
a) China stock market
b) Hong Kong stock market
c) India stock market
d) Japan stock market
# Answer
As of Dec-2025,
- China: $15 trillion
- Hong Kong: $6 trillion
- India: $5.3 trillion
- Japan: $7.6 trillion
# 7. Which of the following market has the largest daily turnover/trading volume?
a) Commodity market
b) Crypto market
c) Global Equity market
d) Forex
# Answer
Global equity market: $3.93 trillion
• Forex: $6.6 trillion
- Commodity: $126 billion
- Crypto: $139 billion
</COVERAGE_CHECKLIST>

<SOURCE_OUTLINE>
# Financial Analytics and
# Algorithmic Trading
Course Code: COMP7415
# Background of Lecturer
- Founder of AlgoGene (https://algogene.com)
Vice Chairman of Algo Challenge Association (https://algochallenge.org)
- Former algo developer, quant trader, risk manager, data analyst for hedge funds and banks.
- BSc in Math, Risk Management, Actuarial Science (HKU); MSc in Computer Science (HKU)
- Champion and awardee of several algo trading competitions, including
CCTV证券资讯频道《宽客天下·全球量化争霸赛》(2017/18)
- Rotman International Trading Competition (2017)
CASH Algo Trading Contest (2016)
WorldQuant Challenge (2014, 2015)
# Let’s play a game!
![](images/997b92b1751c1ce70437186759220c5d106c40ed0862fb47cf1033ee018d9aa5.jpg)
# 1. What’s your study mode?
a) Full time
b) Part time
c) Just sit in
# 2. What is your main reason for taking this course?
a) I want to start a career in algo-trading
b) I want to enhance my current job skills
c) I am interested in the topic and want to learn more
d) Just to fulfill graduation requirement
# 3. What do you expect to learn from this course?
a) Some skills for financial data analysis
b) I want to develop an automated trading system
c) Tell me some profitable trading strategies
d) Not sure yet
# 4. Do you have any prior experience in algo-trading?
a) Yes, I have extensive experience
b) Yes, I have some experience
c) No, but I have experience in other forms of trading
d) No, I am completely new to trading
# 5. Have you ever used any algo-trading platforms before?
a) Yes, I have used MetaTrader
b) Yes, I have used TradingView
c) Yes, I have used AlgoGene
d) No, I have no idea what it is
# 6. Which of the following has the largest market capitalization?
a) China stock market
b) Hong Kong stock market
c) India stock market
d) Japan stock market
# Answer
As of Dec-2025,
- China: $15 trillion
- Hong Kong: $6 trillion
- India: $5.3 trillion
- Japan: $7.6 trillion
# 7. Which of the following market has the largest daily turnover/trading volume?
a) Commodity market
b) Crypto market
c) Global Equity market
d) Forex
# Answer
Global equity market: $3.93 trillion
• Forex: $6.6 trillion
- Commodity: $126 billion
- Crypto: $139 billion
</SOURCE_OUTLINE>

<LESSON_MAP>
{
  "lesson_id": "L0",
  "mode": "guided_story",
  "steps": [
    {
      "concept": "什么是算法交易",
      "file": "research/pipeline/3-guided_story/L0/step1/step.json",
      "sequence_id": "step1"
    },
    {
      "concept": "课程全景：你将学到什么",
      "file": "research/pipeline/3-guided_story/L0/step2/step.json",
      "sequence_id": "step2"
    },
    {
      "concept": "市场有多大？先建立全局感",
      "file": "research/pipeline/3-guided_story/L0/step3/step.json",
      "sequence_id": "step3"
    },
    {
      "concept": "工具与语言：算法交易者的武器",
      "file": "research/pipeline/3-guided_story/L0/step4/step.json",
      "sequence_id": "step4"
    },
    {
      "concept": "课程目标与评估：你要交付什么",
      "file": "research/pipeline/3-guided_story/L0/step5/step.json",
      "sequence_id": "step5"
    }
  ]
}

</LESSON_MAP>

<GUIDED_STORY_MANIFEST>
{
  "lesson_id": "L0",
  "mode": "guided_story",
  "steps": [
    {
      "concept": "什么是算法交易",
      "file": "research/pipeline/3-guided_story/L0/step1/step.json",
      "sequence_id": "step1"
    },
    {
      "concept": "课程全景：你将学到什么",
      "file": "research/pipeline/3-guided_story/L0/step2/step.json",
      "sequence_id": "step2"
    },
    {
      "concept": "市场有多大？先建立全局感",
      "file": "research/pipeline/3-guided_story/L0/step3/step.json",
      "sequence_id": "step3"
    },
    {
      "concept": "工具与语言：算法交易者的武器",
      "file": "research/pipeline/3-guided_story/L0/step4/step.json",
      "sequence_id": "step4"
    },
    {
      "concept": "课程目标与评估：你要交付什么",
      "file": "research/pipeline/3-guided_story/L0/step5/step.json",
      "sequence_id": "step5"
    }
  ]
}

</GUIDED_STORY_MANIFEST>

<GUIDED_STORY_STEPS>
[
  {
    "lesson_id": "L1",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s001",
        "lines": [
          "想象一下，你发现了一个赚钱的机会：某只股票的价格总是在下午三点突然上涨。",
          "你手动下单，但每次点鼠标都慢了几秒。"
        ],
        "type": "narration"
      },
      {
        "id": "s002",
        "introduced_terms": [
          "algorithmic_trading"
        ],
        "lines": [
          "如果能让电脑替你盯着行情，自动下单呢？",
          "这就是 <term id=\"algorithmic_trading\">算法交易</term> 的起点。"
        ],
        "type": "narration"
      },
      {
        "id": "s003",
        "lines": [
          "它把“发现机会”和“执行交易”这两件事，都交给了计算机程序。",
          "速度更快，不受情绪干扰。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 2,
          "explanation": "算法交易的关键是自动化：从识别机会到执行下单，都由程序完成。",
          "kind": "single_choice",
          "options": [
            "用手机App看股票行情",
            "用Excel表格计算收益率",
            "用程序自动识别机会并下单",
            "每天收盘后手动复盘"
          ],
          "prompt": "下面哪个最接近算法交易的核心？"
        },
        "id": "s004",
        "lines": [
          "下面哪个最接近算法交易的核心？"
        ],
        "type": "exercise"
      },
      {
        "id": "s005",
        "lines": [
          "简单来说，算法交易就是用规则和代码，代替人的直觉和手指。",
          "接下来的内容，会一步步带你走进这个领域。"
        ],
        "type": "narration"
      }
    ],
    "sequence_id": "step1",
    "source": {
      "plain_text": "Algorithmic trading is a trending investment approach nowadays that consists of identification of trading opportunities and implementation via computer algorithms.",
      "related": [
        "COMP7415A syllabus"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "algorithmic_trading": {
        "aliases": [
          "Algorithmic Trading",
          "Algo Trading"
        ],
        "display": "算法交易",
        "gloss": "用计算机程序自动执行交易决策和下单的过程。"
      }
    }
  },
  {
    "lesson_id": "L1",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s001",
        "lines": [
          "这门课会带你走完一条完整的路径：",
          "从零开始，到部署一个真正的交易策略。"
        ],
        "type": "narration"
      },
      {
        "id": "s002",
        "introduced_terms": [
          "backtesting"
        ],
        "lines": [
          "前半段，你会学习如何获取金融数据、用Python分析它。",
          "然后搭建一个 <term id=\"backtesting\">回测</term> 框架，验证你的想法。"
        ],
        "type": "narration"
      },
      {
        "id": "s003",
        "introduced_terms": [
          "pairs_trading"
        ],
        "lines": [
          "接着，你会接触到一些经典的策略思路。",
          "比如利用统计关系做 <term id=\"pairs_trading\">配对交易</term>。"
        ],
        "type": "narration"
      },
      {
        "id": "s004",
        "introduced_terms": [
          "order_book"
        ],
        "lines": [
          "后半段，话题转向实战：如何管理风险、优化投资组合。",
          "甚至深入 <term id=\"order_book\">订单簿</term> 数据，理解高频交易的世界。"
        ],
        "type": "narration"
      },
      {
        "id": "s005",
        "lines": [
          "最后，你还会看到机器学习如何被用在交易策略中。",
          "整个课程就像一次从理论到实践的完整旅程。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 2,
          "explanation": "风险管理与投资组合优化属于实战应用部分，通常在课程后半段讨论。",
          "kind": "single_choice",
          "options": [
            "用Python抓取股票数据",
            "搭建回测框架",
            "风险管理与投资组合优化",
            "统计套利基础"
          ],
          "prompt": "以下哪个主题最可能出现在课程的后半段？"
        },
        "id": "s006",
        "lines": [
          "以下哪个主题最可能出现在课程的后半段？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step2",
    "source": {
      "plain_text": "Topics to be covered: 1. Algorithmic trading basics and financial markets 2. Data scraping and database management with Python 3. Building backtest framework and rule-based trading strategy 4. Statistical time series analysis for market classification 5. Statistical arbitrage and pairs trading 6. Capital and Risk Management 7. Performance measures and portfolio optimization 8. Order book and high frequency data modeling 9. Market practice in strategy optimization, broker selection, and market tricks 10. Machine learning use cases in algorithmic trading",
      "related": [
        "COMP7415A syllabus"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "backtesting": {
        "aliases": [
          "Backtesting"
        ],
        "display": "回测",
        "gloss": "用历史数据测试交易策略表现的方法。"
      },
      "order_book": {
        "aliases": [
          "Order Book",
          "L2 Data"
        ],
        "display": "订单簿",
        "gloss": "记录所有未成交买卖订单的电子列表。"
      },
      "pairs_trading": {
        "aliases": [
          "Pairs Trading",
          "Statistical Arbitrage"
        ],
        "display": "配对交易",
        "gloss": "利用两只相关股票价格偏离进行套利的策略。"
      }
    }
  },
  {
    "lesson_id": "L1",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s001",
        "introduced_terms": [
          "market_capitalization"
        ],
        "lines": [
          "在动手写代码之前，先看看我们面对的市场有多大。",
          "猜猜看，哪个股票市场的 <term id=\"market_capitalization\">市值</term> 最高？"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 0,
          "explanation": "中国A股市场市值约15万亿美元，远超其他选项。",
          "kind": "single_choice",
          "options": [
            "中国A股市场",
            "香港股票市场",
            "印度股票市场",
            "日本股票市场"
          ],
          "prompt": "截至2025年底，以下哪个市场的市值最大？"
        },
        "id": "s002",
        "lines": [
          "截至2025年底，以下哪个市场的市值最大？"
        ],
        "type": "exercise"
      },
      {
        "id": "s003",
        "lines": [
          "中国A股市场市值约15万亿美元，日本约7.6万亿，香港约6万亿。",
          "但市值大，不代表交易最活跃。"
        ],
        "type": "narration"
      },
      {
        "id": "s004",
        "introduced_terms": [
          "daily_turnover"
        ],
        "lines": [
          "衡量活跃度的指标是 <term id=\"daily_turnover\">日交易额</term>。",
          "全球股票市场每天交易约3.93万亿美元。"
        ],
        "type": "narration"
      },
      {
        "id": "s005",
        "introduced_terms": [
          "forex"
        ],
        "lines": [
          "但有一个市场比它大得多——<term id=\"forex\">外汇市场</term>。",
          "每天的交易额高达6.6万亿美元。"
        ],
        "type": "narration"
      },
      {
        "id": "s006",
        "lines": [
          "相比之下，加密货币市场每天只有1390亿美元。",
          "知道这些数字，能帮你理解算法交易者为什么偏爱某些市场。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "外汇市场日交易额约6.6万亿美元，是全球最大的金融市场。",
          "kind": "single_choice",
          "options": [
            "全球股票市场",
            "外汇市场",
            "商品市场",
            "加密货币市场"
          ],
          "prompt": "哪个市场的日交易额最大？"
        },
        "id": "s007",
        "lines": [
          "哪个市场的日交易额最大？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step3",
    "source": {
      "plain_text": "Which of the following has the largest market capitalization? China: $15 trillion, Hong Kong: $6 trillion, India: $5.3 trillion, Japan: $7.6 trillion. Which market has the largest daily turnover? Global equity market: $3.93 trillion, Forex: $6.6 trillion, Commodity: $126 billion, Crypto: $139 billion.",
      "related": [
        "COMP7415A lecture slides"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "daily_turnover": {
        "aliases": [
          "Daily Turnover",
          "Trading Volume"
        ],
        "display": "日交易额",
        "gloss": "一天内所有交易的总金额。"
      },
      "forex": {
        "aliases": [
          "Forex",
          "FX"
        ],
        "display": "外汇市场",
        "gloss": "全球最大的金融市场，交易各国货币。"
      },
      "market_capitalization": {
        "aliases": [
          "Market Cap"
        ],
        "display": "市值",
        "gloss": "上市公司股票总数乘以当前股价的总价值。"
      }
    }
  },
  {
    "lesson_id": "L1",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s001",
        "lines": [
          "想做算法交易，你需要两样东西：一门编程语言，和一个交易平台。"
        ],
        "type": "narration"
      },
      {
        "id": "s002",
        "lines": [
          "先问个简单的问题：哪种语言在算法交易中最流行？"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 2,
          "explanation": "Python凭借其丰富的数据分析库和易用性，成为算法交易领域的首选语言。",
          "kind": "single_choice",
          "options": [
            "C++",
            "Java",
            "Python",
            "R"
          ],
          "prompt": "以下哪种编程语言在算法交易中使用最广泛？"
        },
        "id": "s003",
        "lines": [
          "以下哪种编程语言在算法交易中使用最广泛？"
        ],
        "type": "exercise"
      },
      {
        "id": "s004",
        "introduced_terms": [
          "python"
        ],
        "lines": [
          "没错，就是 <term id=\"python\">Python</term>。",
          "它简单、灵活，而且有大量现成的库帮你处理数据、回测策略。"
        ],
        "type": "narration"
      },
      {
        "id": "s005",
        "introduced_terms": [
          "metatrader",
          "tradingview"
        ],
        "lines": [
          "至于交易平台，市面上有很多选择。",
          "比如 <term id=\"metatrader\">MetaTrader</term> 和 <term id=\"tradingview\">TradingView</term>，都支持自动化交易。"
        ],
        "type": "narration"
      },
      {
        "id": "s006",
        "lines": [
          "这门课会教你如何用Python从零搭建自己的交易系统。",
          "你不需要依赖任何特定平台，就能拥有完全的控制权。"
        ],
        "type": "narration"
      },
      {
        "id": "s007",
        "lines": [
          "一句话总结：Python是你的大脑，交易平台是你的手脚。",
          "接下来，我们开始动手。"
        ],
        "type": "narration"
      }
    ],
    "sequence_id": "step4",
    "source": {
      "plain_text": "Which programming language is most widely used in algo-trading? Python. Have you ever used any algo-trading platforms before? MetaTrader, TradingView, AlgoGene.",
      "related": [
        "COMP7415A lecture slides"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "metatrader": {
        "aliases": [
          "MT4",
          "MT5"
        ],
        "display": "MetaTrader",
        "gloss": "一款流行的外汇和差价合约交易平台，支持自动化交易。"
      },
      "python": {
        "aliases": [],
        "display": "Python",
        "gloss": "一种广泛用于数据分析和算法交易的高级编程语言。"
      },
      "tradingview": {
        "aliases": [],
        "display": "TradingView",
        "gloss": "一款基于网页的图表分析和交易平台。"
      }
    }
  },
  {
    "lesson_id": "L1",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s001",
        "lines": [
          "学完这门课，你应该能独立完成一件事：",
          "从分析数据到写出策略，再到验证它是否有效。"
        ],
        "type": "narration"
      },
      {
        "id": "s002",
        "introduced_terms": [
          "backtesting",
          "risk_management"
        ],
        "lines": [
          "具体来说，你要能进行 <term id=\"backtesting\">回测</term>、优化参数、做 <term id=\"risk_management\">风险管理</term>。",
          "并且能解释策略的表现。"
        ],
        "type": "narration"
      },
      {
        "id": "s003",
        "lines": [
          "最终，你会通过一个小组项目，完成一个完整的策略开发周期。",
          "项目占最终成绩的50%。"
        ],
        "type": "narration"
      },
      {
        "id": "s004",
        "lines": [
          "另外50%来自期末笔试，覆盖所有教过的内容。",
          "所以，理解每一个环节都很重要。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answers": [
            "50"
          ],
          "explanation": "小组项目和期末笔试各占50%。",
          "kind": "fill_in_blank",
          "prompt": "课程评估中，小组项目占______%。"
        },
        "id": "s005",
        "lines": [
          "课程评估中，小组项目占多少比例？"
        ],
        "type": "exercise"
      },
      {
        "id": "s006",
        "lines": [
          "准备好了吗？",
          "接下来的每一步，都会让你离一个真正的算法交易者更近一点。"
        ],
        "type": "narration"
      }
    ],
    "sequence_id": "step5",
    "source": {
      "plain_text": "Course Objectives: Understand the trading process, understand algorithmic trading framework, formulate trading strategies, carry out backtesting, optimization, risk management. Assessment: Group project - 50%, Written final exam - 50%.",
      "related": [
        "COMP7415A syllabus"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "backtesting": {
        "aliases": [
          "Backtesting"
        ],
        "display": "回测",
        "gloss": "用历史数据测试交易策略表现的方法。"
      },
      "risk_management": {
        "aliases": [
          "Risk Management"
        ],
        "display": "风险管理",
        "gloss": "识别、评估和控制投资风险的过程。"
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
      "coverage_tag": "algo_trading_tools_languages",
      "covered_by": [
        "qf_flash_python_popularity",
        "qf_flash_platform_examples",
        "qf_quiz_python_vs_others",
        "qf_quiz_platform_recognition",
        "qf_long_tool_analogy"
      ],
      "description": "算法交易的核心工具：编程语言（Python）和交易平台（MetaTrader, TradingView）"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "algo_trading_tools_languages",
      "coverage_tags": [
        "algo_trading_tools_languages"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_python_popularity",
      "learning_goal": "学生能准确说出算法交易中最广泛使用的编程语言及其核心优势。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "算法交易中最流行的编程语言名称及两个关键优势。",
      "term_refs": [
        {
          "display": "Python",
          "en": "Python"
        }
      ],
      "variants": [
        {
          "back": "Python。优势：简单灵活，拥有大量现成的数据分析与回测库。",
          "estimated_seconds": 8,
          "explanation": "Python 因其易用性和丰富的生态（如 pandas, numpy, backtrader）成为算法交易的首选。",
          "front": "在算法交易领域，使用最广泛的编程语言是什么？它有哪些主要优势？",
          "question_id": "q_flash_python_popularity_v1"
        },
        {
          "back": "1. 语法简单，开发效率高；2. 拥有大量现成的数据处理和回测库。",
          "estimated_seconds": 10,
          "explanation": "虽然 C++ 执行速度更快，但 Python 的开发速度和丰富的库使其在策略研究和原型开发中占主导地位。",
          "front": "为什么 Python 在算法交易中比 C++ 或 Java 更受欢迎？说出两个原因。",
          "question_id": "q_flash_python_popularity_v2"
        }
      ]
    },
    {
      "concept_key": "algo_trading_tools_languages",
      "coverage_tags": [
        "algo_trading_tools_languages"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_platform_examples",
      "learning_goal": "学生能列举出至少两个支持自动化交易的常用平台。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "课程中提到的两个支持自动化交易的平台名称。",
      "term_refs": [
        {
          "display": "MetaTrader",
          "en": "MetaTrader"
        },
        {
          "display": "TradingView",
          "en": "TradingView"
        }
      ],
      "variants": [
        {
          "back": "MetaTrader 和 TradingView。",
          "estimated_seconds": 6,
          "explanation": "这两个平台都支持用户编写和运行自动化交易策略。",
          "front": "课程中提到了哪两个支持自动化交易的平台？",
          "question_id": "q_flash_platform_examples_v1"
        },
        {
          "back": "TradingView。",
          "estimated_seconds": 6,
          "explanation": "TradingView 是一个基于网页的图表分析和交易平台，也支持自动化交易。",
          "front": "除了 MetaTrader，课程中还提到了哪个支持自动化交易的网页平台？",
          "question_id": "q_flash_platform_examples_v2"
        }
      ]
    }
  ],
  "lesson_id": "L0",
  "longform_families": [
    {
      "concept_key": "algo_trading_tools_languages",
      "coverage_tags": [
        "algo_trading_tools_languages"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_tool_analogy",
      "learning_goal": "学生能解释 Python 和交易平台在算法交易中的不同角色，并用自己的比喻进行说明。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "Python",
          "en": "Python"
        },
        {
          "display": "交易平台",
          "en": "Trading Platform"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "解释“大脑”和“手脚”分别对应什么工具及其功能。",
            "说明为什么 Python 适合作为“大脑”（至少两个理由）。",
            "简要说明交易平台作为“手脚”的作用。"
          ],
          "question_id": "q_long_tool_analogy_v1",
          "reference_answer": [
            "在这个比喻中，'大脑'指的是 Python 编程语言，它负责策略的逻辑、数据分析、回测和决策生成。'手脚'指的是交易平台（如 MetaTrader 或 TradingView），它们负责接收大脑的指令并实际执行买卖操作。",
            "Python 成为首选'大脑'的原因：1) 语法简单，开发效率高，能让交易者快速将想法转化为代码；2) 拥有大量现成的库（如 pandas, numpy, backtrader），可以高效地处理数据、进行统计分析和回测策略。",
            "交易平台作为'手脚'，其核心作用是提供市场连接、账户管理和订单执行功能，确保策略的指令能准确、快速地传递到市场。"
          ],
          "rubric_points": [
            "正确指出 Python 是大脑（负责思考、决策、策略逻辑），交易平台是手脚（负责执行下单）。",
            "给出至少两个 Python 成为首选的原因（如：简单灵活、丰富的库、开发效率高）。",
            "正确说明交易平台的作用是连接市场和执行交易。"
          ],
          "stem": "课程中将 Python 和交易平台的关系比喻为“大脑”和“手脚”。请解释这个比喻的含义，并说明为什么 Python 是算法交易者的首选“大脑”。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "提出一个你自己的比喻来描述 Python 和交易平台的关系。",
            "解释 Python 在策略开发中的具体作用。",
            "解释交易平台在策略执行中的具体作用。"
          ],
          "question_id": "q_long_tool_analogy_v2",
          "reference_answer": [
            "一个可能的比喻是：Python 就像一位厨师和他的菜谱，而交易平台就是厨房和炉灶。厨师（Python）负责研究食材（数据）、设计菜谱（策略）、并决定什么时候开始烹饪（生成交易信号）。",
            "Python 的具体作用包括：从各种来源获取金融数据、编写策略逻辑（如当价格突破某均线时买入）、用历史数据测试策略表现（回测）、以及实时计算并生成买卖指令。",
            "交易平台的具体作用包括：接收 Python 生成的买卖指令、连接到交易所或经纪商、检查账户资金和持仓、最终将订单发送到市场并报告成交结果。没有平台，Python 的指令就无法被执行。"
          ],
          "rubric_points": [
            "提出一个清晰、合理的比喻（例如：Python 是厨师和菜谱，交易平台是厨房和炉灶）。",
            "正确解释 Python 用于策略逻辑、数据分析、回测和生成交易信号。",
            "正确解释交易平台用于接收信号、连接市场、管理账户和执行订单。"
          ],
          "stem": "假设你是一个刚入门的算法交易者，你需要向一个不懂技术的朋友解释为什么你需要同时学习 Python 和使用一个交易平台。请用你自己的比喻来解释这两者的不同作用。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "algo_trading_tools_languages",
      "coverage_tags": [
        "algo_trading_tools_languages"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_python_vs_others",
      "learning_goal": "学生能在多个选项中准确识别出算法交易中最广泛使用的编程语言。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "Python",
          "en": "Python"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "Python 凭借其丰富的数据分析库和易用性，成为算法交易领域的首选语言。",
          "options": [
            "C++",
            "Java",
            "Python",
            "R"
          ],
          "question_id": "q_quiz_python_vs_others_v1",
          "stem": "以下哪种编程语言在算法交易中使用最广泛？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "Python 拥有 pandas、numpy、statsmodels 等库，非常适合快速进行数据分析和策略回测。",
          "options": [
            "C++",
            "Python",
            "JavaScript",
            "Swift"
          ],
          "question_id": "q_quiz_python_vs_others_v2",
          "stem": "一个量化团队需要快速开发并测试一个新的统计套利策略。考虑到开发效率和库支持，他们最可能选择哪种语言？"
        }
      ]
    },
    {
      "concept_key": "algo_trading_tools_languages",
      "coverage_tags": [
        "algo_trading_tools_languages"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_platform_recognition",
      "learning_goal": "学生能识别出课程中提到的、支持自动化交易的平台。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "MetaTrader",
          "en": "MetaTrader"
        },
        {
          "display": "TradingView",
          "en": "TradingView"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "课程中明确提到了 MetaTrader 和 TradingView 作为支持自动化交易的平台。",
          "options": [
            "Bloomberg Terminal",
            "MetaTrader",
            "Excel",
            "Tableau"
          ],
          "question_id": "q_quiz_platform_recognition_v1",
          "stem": "以下哪个平台在课程中被提及为支持自动化交易的选项？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "TradingView 是一个基于网页的平台，提供图表分析并支持自动化交易。",
          "options": [
            "MetaTrader",
            "TradingView",
            "AlgoGene",
            "Interactive Brokers TWS"
          ],
          "question_id": "q_quiz_platform_recognition_v2",
          "stem": "一个交易者想使用基于网页的图表分析工具并运行自动化策略，以下哪个平台最符合他的需求？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L0/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "input source outline"
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "algorithmic_trading_definition",
      "covered_by": [
        "qf_flash_algo_def",
        "qf_quiz_algo_core",
        "qf_long_algo_explain"
      ],
      "description": "算法交易的定义：用计算机程序自动执行交易决策和下单的过程。"
    },
    {
      "coverage_tag": "algorithmic_trading_components",
      "covered_by": [
        "qf_flash_algo_components",
        "qf_quiz_algo_core"
      ],
      "description": "算法交易的两个核心组成部分：发现交易机会和执行交易。"
    },
    {
      "coverage_tag": "algorithmic_trading_benefits",
      "covered_by": [
        "qf_flash_algo_benefits",
        "qf_quiz_algo_benefits"
      ],
      "description": "算法交易的优势：速度更快、不受情绪干扰。"
    },
    {
      "coverage_tag": "algorithmic_trading_vs_manual",
      "covered_by": [
        "qf_flash_algo_vs_manual",
        "qf_quiz_algo_core",
        "qf_long_algo_explain"
      ],
      "description": "算法交易与手动交易的核心区别：用规则和代码代替人的直觉和手指。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "algorithmic_trading_definition",
      "coverage_tags": [
        "algorithmic_trading_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_algo_def",
      "learning_goal": "学生能准确回忆算法交易的定义。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "算法交易的核心定义：用计算机程序自动执行交易决策和下单。",
      "term_refs": [
        {
          "display": "算法交易",
          "en": "Algorithmic Trading"
        }
      ],
      "variants": [
        {
          "back": "用计算机程序自动执行交易决策和下单的过程。",
          "estimated_seconds": 8,
          "explanation": "算法交易的核心在于自动化，将交易决策和执行都交给程序。",
          "front": "什么是算法交易？",
          "question_id": "q_flash_algo_def_v1"
        },
        {
          "back": "发现交易机会和执行交易。",
          "estimated_seconds": 10,
          "explanation": "算法交易把'发现机会'和'执行交易'这两件事都交给了计算机程序。",
          "front": "算法交易中，哪两个环节被交给了计算机程序？",
          "question_id": "q_flash_algo_def_v2"
        }
      ]
    },
    {
      "concept_key": "algorithmic_trading_components",
      "coverage_tags": [
        "algorithmic_trading_components"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_algo_components",
      "learning_goal": "学生能识别算法交易的两个核心组成部分。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "算法交易的两个核心组成部分：发现交易机会和执行交易。",
      "term_refs": [
        {
          "display": "算法交易",
          "en": "Algorithmic Trading"
        }
      ],
      "variants": [
        {
          "back": "发现交易机会和执行交易。",
          "estimated_seconds": 8,
          "explanation": "算法交易将'发现机会'和'执行交易'这两个环节都交给了计算机程序。",
          "front": "算法交易的两个核心组成部分是什么？",
          "question_id": "q_flash_algo_components_v1"
        },
        {
          "back": "识别交易机会和自动下单。",
          "estimated_seconds": 8,
          "explanation": "计算机程序负责识别交易机会并自动执行下单操作。",
          "front": "在算法交易中，计算机程序负责哪两个主要任务？",
          "question_id": "q_flash_algo_components_v2"
        }
      ]
    },
    {
      "concept_key": "algorithmic_trading_benefits",
      "coverage_tags": [
        "algorithmic_trading_benefits"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_algo_benefits",
      "learning_goal": "学生能列举算法交易相比手动交易的两个主要优势。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "算法交易的两个主要优势：速度更快和不受情绪干扰。",
      "term_refs": [
        {
          "display": "算法交易",
          "en": "Algorithmic Trading"
        }
      ],
      "variants": [
        {
          "back": "速度更快，不受情绪干扰。",
          "estimated_seconds": 8,
          "explanation": "算法交易由计算机执行，速度远快于手动操作，且不受人类情绪影响。",
          "front": "与手动交易相比，算法交易的两个主要优势是什么？",
          "question_id": "q_flash_algo_benefits_v1"
        },
        {
          "back": "因为交易决策和下单都由计算机程序自动完成。",
          "estimated_seconds": 10,
          "explanation": "计算机程序可以瞬间分析市场并执行交易，避免了手动操作的时间延迟。",
          "front": "为什么算法交易能比手动交易更快地执行？",
          "question_id": "q_flash_algo_benefits_v2"
        }
      ]
    },
    {
      "concept_key": "algorithmic_trading_vs_manual",
      "coverage_tags": [
        "algorithmic_trading_vs_manual"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_algo_vs_manual",
      "learning_goal": "学生能概括算法交易与手动交易的核心区别。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "算法交易与手动交易的核心区别：用规则和代码代替人的直觉和手指。",
      "term_refs": [
        {
          "display": "算法交易",
          "en": "Algorithmic Trading"
        }
      ],
      "variants": [
        {
          "back": "规则和代码。",
          "estimated_seconds": 8,
          "explanation": "算法交易的核心思想是用预先设定的规则和计算机代码来执行交易，取代人的直觉判断和手动操作。",
          "front": "算法交易用什么代替了人的直觉和手指？",
          "question_id": "q_flash_algo_vs_manual_v1"
        },
        {
          "back": "发现机会和手动下单。",
          "estimated_seconds": 8,
          "explanation": "手动交易中，交易者需要自己观察市场发现机会，然后手动点击鼠标下单。",
          "front": "手动交易中，交易者需要亲自完成哪两个动作？",
          "question_id": "q_flash_algo_vs_manual_v2"
        }
      ]
    }
  ],
  "lesson_id": "L0",
  "longform_families": [
    {
      "concept_key": "algorithmic_trading_definition",
      "coverage_tags": [
        "algorithmic_trading_definition",
        "algorithmic_trading_vs_manual"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_algo_explain",
      "learning_goal": "学生能用一段话解释算法交易的概念，并对比手动交易说明其优势。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "算法交易",
          "en": "Algorithmic Trading"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "算法交易的定义",
            "算法交易的两个核心组成部分",
            "与手动交易相比的两个主要优势"
          ],
          "question_id": "q_long_algo_explain_v1",
          "reference_answer": [
            "算法交易是一种用计算机程序自动执行交易决策和下单的投资方法。",
            "它将'发现交易机会'和'执行交易'这两个环节都交给了计算机程序。",
            "与手动交易相比，算法交易有两个主要优势：第一，速度更快，计算机可以在瞬间分析市场并执行交易；第二，不受情绪干扰，程序会严格按照预设的规则执行，不会因为恐惧或贪婪而做出非理性决策。"
          ],
          "rubric_points": [
            "正确给出算法交易的定义（用计算机程序自动执行交易决策和下单）",
            "指出算法交易的两个组成部分（发现机会和执行交易）",
            "正确列出至少两个优势（速度更快、不受情绪干扰）"
          ],
          "stem": "请用你自己的话解释什么是算法交易，并说明它与手动交易相比有哪些优势。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "描述如何用算法交易解决手动下单慢的问题",
            "解释算法交易的两个核心组成部分在这个场景中如何体现",
            "说明算法交易相比手动交易的优势"
          ],
          "question_id": "q_long_algo_explain_v2",
          "reference_answer": [
            "我会编写一个计算机程序，让它每天下午三点自动监控这只股票的价格，并在价格开始上涨时自动买入。",
            "在这个场景中，'发现交易机会'对应的是程序识别出下午三点股价上涨的模式，'执行交易'对应的是程序自动下单买入。",
            "算法交易的优势在于：第一，速度更快，程序可以在毫秒级别内完成下单，避免了手动点击鼠标的延迟；第二，不受情绪干扰，程序会严格按照预设规则执行，不会因为犹豫或恐惧而错过最佳时机。"
          ],
          "rubric_points": [
            "提出用程序自动在下午三点买入的解决方案",
            "指出'发现机会'（识别下午三点的上涨模式）和'执行交易'（自动下单）两个环节",
            "正确说明速度优势和不受情绪干扰的优势"
          ],
          "stem": "假设你发现了一个交易机会：某只股票的价格总是在下午三点突然上涨，但手动下单总是慢几秒。请解释你将如何利用算法交易来解决这个问题，并说明算法交易相比手动交易的优势。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "algorithmic_trading_definition",
      "coverage_tags": [
        "algorithmic_trading_definition",
        "algorithmic_trading_components",
        "algorithmic_trading_vs_manual"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_algo_core",
      "learning_goal": "学生能在多个选项中准确识别算法交易的核心特征。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "算法交易",
          "en": "Algorithmic Trading"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "算法交易的关键在于自动化：从识别机会到执行下单，都由计算机程序完成。",
          "options": [
            "用手机App查看股票行情",
            "用Excel表格计算投资收益率",
            "用计算机程序自动识别交易机会并执行下单",
            "每天收盘后手动复盘交易记录"
          ],
          "question_id": "q_quiz_algo_core_v1",
          "stem": "以下哪个选项最准确地描述了算法交易的核心？"
        },
        {
          "answer": 2,
          "estimated_seconds": 25,
          "explanation": "编写程序自动执行交易是算法交易的核心思想，可以解决手动操作速度慢的问题。",
          "options": [
            "换一个更快的网络",
            "使用更高级的鼠标",
            "编写一个程序，在下午三点自动买入该股票",
            "提前挂好限价单"
          ],
          "question_id": "q_quiz_algo_core_v2",
          "stem": "小明发现某只股票经常在下午三点突然上涨，但他手动下单总是慢几秒。以下哪种方法最能解决他的问题？"
        }
      ]
    },
    {
      "concept_key": "algorithmic_trading_benefits",
      "coverage_tags": [
        "algorithmic_trading_benefits"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_algo_benefits",
      "learning_goal": "学生能识别算法交易相比手动交易的优势。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "算法交易",
          "en": "Algorithmic Trading"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "算法交易由计算机执行，速度远快于手动操作，且不受人类情绪（如恐惧、贪婪）的影响。",
          "options": [
            "可以完全避免亏损",
            "交易速度更快，且不受情绪干扰",
            "不需要任何编程知识",
            "可以保证每次交易都盈利"
          ],
          "question_id": "q_quiz_algo_benefits_v1",
          "stem": "以下哪项是算法交易相对于手动交易的优势？"
        },
        {
          "answer": 1,
          "estimated_seconds": 25,
          "explanation": "算法交易严格按照预设的规则和代码执行，不会因为恐惧、贪婪等情绪而犹豫或改变决策。",
          "options": [
            "算法交易可以预测未来价格",
            "算法交易不受情绪影响，会严格按照预设规则执行",
            "算法交易会自动取消所有亏损的交易",
            "算法交易会建议交易者何时买入或卖出"
          ],
          "question_id": "q_quiz_algo_benefits_v2",
          "stem": "一个交易者因为害怕亏损而犹豫不决，错过了最佳卖出时机。算法交易如何帮助解决这个问题？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L0/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "input source outline"
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "course_objectives",
      "covered_by": [
        "qf_flash_course_obj",
        "qf_quiz_course_obj",
        "qf_long_course_obj"
      ],
      "description": "课程目标：理解交易流程、算法交易框架、制定策略、回测、优化、风险管理、解释表现"
    },
    {
      "coverage_tag": "assessment_structure",
      "covered_by": [
        "qf_flash_assessment",
        "qf_quiz_assessment"
      ],
      "description": "评估结构：小组项目50%，期末笔试50%"
    },
    {
      "coverage_tag": "group_project",
      "covered_by": [
        "qf_flash_assessment",
        "qf_quiz_assessment",
        "qf_long_course_obj"
      ],
      "description": "小组项目：完成完整的策略开发周期"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "course_objectives",
      "coverage_tags": [
        "course_objectives"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_course_obj",
      "learning_goal": "学生能准确回忆课程目标中要求学生掌握的核心能力项。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "课程目标中要求学生能进行的核心操作步骤。",
      "term_refs": [
        {
          "display": "回测",
          "en": "backtesting"
        },
        {
          "display": "风险管理",
          "en": "risk management"
        }
      ],
      "variants": [
        {
          "back": "回测和优化参数",
          "estimated_seconds": 8,
          "explanation": "课程目标明确要求学生能进行回测（backtesting）和参数优化（optimization）来验证策略的有效性。",
          "front": "课程目标中，学生需要能进行哪两项关键操作来验证策略？",
          "question_id": "q_flash_course_obj_v1"
        },
        {
          "back": "风险管理",
          "estimated_seconds": 8,
          "explanation": "课程目标要求学生能进行风险管理（risk management），以识别、评估和控制投资风险。",
          "front": "除了回测和优化，课程目标还要求学生能做什么来管理策略风险？",
          "question_id": "q_flash_course_obj_v2"
        },
        {
          "back": "策略的表现",
          "estimated_seconds": 8,
          "explanation": "课程目标要求学生能解释策略的表现（interpret investment performance），即分析策略的收益和风险特征。",
          "front": "完成回测、优化和风险管理后，学生还需要能解释什么？",
          "question_id": "q_flash_course_obj_v3"
        }
      ]
    },
    {
      "concept_key": "assessment_structure",
      "coverage_tags": [
        "assessment_structure",
        "group_project"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_assessment",
      "learning_goal": "学生能准确回忆课程评估的两大部分及其权重。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "课程评估的两大组成部分及其各自占比。",
      "term_refs": [
        {
          "display": "小组项目",
          "en": "group project"
        },
        {
          "display": "期末笔试",
          "en": "written final exam"
        }
      ],
      "variants": [
        {
          "back": "50%",
          "estimated_seconds": 5,
          "explanation": "小组项目占最终成绩的50%。",
          "front": "课程评估中，小组项目占最终成绩的百分之多少？",
          "question_id": "q_flash_assessment_v1"
        },
        {
          "back": "期末笔试，占50%",
          "estimated_seconds": 8,
          "explanation": "另外50%来自期末笔试，覆盖所有教过的内容。",
          "front": "课程评估中，除了小组项目，另一部分评估是什么？它占多少比例？",
          "question_id": "q_flash_assessment_v2"
        }
      ]
    }
  ],
  "lesson_id": "L0",
  "longform_families": [
    {
      "concept_key": "course_objectives",
      "coverage_tags": [
        "course_objectives",
        "group_project"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_course_obj",
      "learning_goal": "学生能用自己的话解释课程目标中要求学生掌握的核心能力，并说明这些能力如何通过小组项目体现。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "回测",
          "en": "backtesting"
        },
        {
          "display": "风险管理",
          "en": "risk management"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "列出课程目标中的核心能力",
            "解释小组项目如何让学生实践这些能力"
          ],
          "question_id": "q_long_course_obj_v1",
          "reference_answer": [
            "课程目标要求学生掌握的核心能力包括：进行回测（用历史数据测试策略）、优化参数（调整策略参数以提升表现）、进行风险管理（控制投资风险）、以及解释策略的表现（分析收益和风险特征）。",
            "小组项目要求学生完成一个完整的策略开发周期，从分析数据、制定策略、进行回测和优化，到进行风险管理和最终解释策略表现，从而全面实践这些能力。"
          ],
          "rubric_points": [
            "准确列出回测、优化参数、风险管理、解释策略表现等能力",
            "说明小组项目要求学生完成一个完整的策略开发周期，从而实践上述能力"
          ],
          "stem": "请简要解释本课程的目标要求学生掌握哪些核心能力？并说明这些能力如何通过小组项目来体现。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "描述回测的具体步骤",
            "描述风险管理的具体做法"
          ],
          "question_id": "q_long_course_obj_v2",
          "reference_answer": [
            "在小组项目中，我会先编写策略代码，然后使用历史数据运行回测，计算策略的累计收益、最大回撤、夏普比率等指标，以评估策略的有效性。",
            "在风险管理方面，我会设置止损点以限制单笔交易的最大亏损，控制每笔交易的仓位大小以避免过度集中，并考虑分散投资于不同资产以降低整体风险。"
          ],
          "rubric_points": [
            "回测：使用历史数据运行策略，计算收益、回撤等指标",
            "风险管理：设置止损、控制仓位、分散投资等"
          ],
          "stem": "假设你正在完成小组项目，请描述你将如何应用课程目标中提到的“回测”和“风险管理”两个能力。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "course_objectives",
      "coverage_tags": [
        "course_objectives"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_course_obj",
      "learning_goal": "学生能在测验情境下辨析课程目标中要求学生掌握的核心能力。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "回测",
          "en": "backtesting"
        },
        {
          "display": "风险管理",
          "en": "risk management"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "课程目标包括回测、优化参数、风险管理和解释策略表现，但不涉及开发高频交易硬件。",
          "options": [
            "进行回测",
            "优化交易策略参数",
            "开发高频交易硬件",
            "解释策略表现"
          ],
          "question_id": "q_quiz_course_obj_v1",
          "stem": "以下哪一项不是本课程目标要求学生掌握的能力？"
        },
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "课程目标要求学生能独立完成从分析数据、写出策略到验证策略有效性的完整流程。",
          "options": [
            "仅完成回测",
            "仅优化参数",
            "从分析数据到写出策略，再到验证其有效性",
            "仅进行风险管理"
          ],
          "question_id": "q_quiz_course_obj_v2",
          "stem": "根据课程目标，学生完成一个完整的策略开发周期后，最终需要能做什么？"
        }
      ]
    },
    {
      "concept_key": "assessment_structure",
      "coverage_tags": [
        "assessment_structure",
        "group_project"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_assessment",
      "learning_goal": "学生能在测验情境下准确识别课程评估的构成和权重。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "小组项目",
          "en": "group project"
        },
        {
          "display": "期末笔试",
          "en": "written final exam"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "课程评估中，小组项目占50%，期末笔试占50%。",
          "options": [
            "小组项目30%，期末笔试70%",
            "小组项目50%，期末笔试50%",
            "小组项目70%，期末笔试30%",
            "小组项目40%，期末笔试60%"
          ],
          "question_id": "q_quiz_assessment_v1",
          "stem": "本课程的评估方式中，小组项目和期末笔试的占比分别是多少？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "小组项目占最终成绩的50%，即一半。期末笔试覆盖所有教过的内容。",
          "options": [
            "只有小组项目，没有考试",
            "小组项目占最终成绩的一半",
            "期末笔试只覆盖部分内容",
            "小组项目是可选任务"
          ],
          "question_id": "q_quiz_assessment_v2",
          "stem": "关于本课程的评估，以下哪项描述是正确的？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L0/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "input source outline"
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "course_roadmap",
      "covered_by": [
        "qf_flash_roadmap",
        "qf_quiz_roadmap",
        "qf_long_roadmap"
      ],
      "description": "课程全景：从零到部署交易策略的完整路径"
    },
    {
      "coverage_tag": "front_half_topics",
      "covered_by": [
        "qf_flash_front_half",
        "qf_quiz_front_half"
      ],
      "description": "前半段主题：数据获取、Python分析、回测框架搭建"
    },
    {
      "coverage_tag": "back_half_topics",
      "covered_by": [
        "qf_flash_back_half",
        "qf_quiz_back_half",
        "qf_long_back_half"
      ],
      "description": "后半段主题：风险管理、投资组合优化、订单簿、高频数据、机器学习"
    },
    {
      "coverage_tag": "key_terms_step2",
      "covered_by": [
        "qf_flash_terms",
        "qf_quiz_terms"
      ],
      "description": "step2引入的关键术语：回测、配对交易、订单簿"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "course_roadmap",
      "coverage_tags": [
        "course_roadmap"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_roadmap",
      "learning_goal": "学生能复述课程从零到部署交易策略的完整路径。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "课程路径的起点和终点",
      "term_refs": [
        {
          "display": "课程路径",
          "en": "Course Roadmap"
        }
      ],
      "variants": [
        {
          "back": "起点：从零开始；终点：部署一个真正的交易策略。",
          "estimated_seconds": 8,
          "explanation": "课程设计为从零基础到实战部署的完整旅程。",
          "front": "这门课带你走完一条完整的路径，起点是什么？终点是什么？",
          "question_id": "q_flash_roadmap_v1"
        },
        {
          "back": "从零开始。",
          "estimated_seconds": 5,
          "explanation": "课程假设学生没有算法交易经验，从基础开始教学。",
          "front": "课程路径的终点是部署一个真正的交易策略，那么起点是什么？",
          "question_id": "q_flash_roadmap_v2"
        }
      ]
    },
    {
      "concept_key": "front_half_topics",
      "coverage_tags": [
        "front_half_topics"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_front_half",
      "learning_goal": "学生能说出课程前半段的核心活动。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "前半段的关键动作",
      "term_refs": [
        {
          "display": "回测",
          "en": "Backtesting"
        }
      ],
      "variants": [
        {
          "back": "回测框架。",
          "estimated_seconds": 6,
          "explanation": "回测框架用于用历史数据验证交易策略。",
          "front": "课程前半段，你学习获取金融数据、用Python分析后，接下来要搭建什么框架？",
          "question_id": "q_flash_front_half_v1"
        },
        {
          "back": "回测框架。",
          "estimated_seconds": 6,
          "explanation": "回测是验证策略有效性的关键步骤。",
          "front": "课程前半段的核心活动是：获取数据、用Python分析、然后搭建什么？",
          "question_id": "q_flash_front_half_v2"
        }
      ]
    },
    {
      "concept_key": "back_half_topics",
      "coverage_tags": [
        "back_half_topics"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_back_half",
      "learning_goal": "学生能说出课程后半段涉及的两个关键领域。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "后半段的两个核心主题",
      "term_refs": [
        {
          "display": "订单簿",
          "en": "Order Book"
        },
        {
          "display": "高频交易",
          "en": "High Frequency Trading"
        }
      ],
      "variants": [
        {
          "back": "订单簿数据。",
          "estimated_seconds": 7,
          "explanation": "订单簿记录所有未成交买卖订单，是高频交易的核心数据。",
          "front": "课程后半段，话题转向实战，其中深入什么数据来理解高频交易的世界？",
          "question_id": "q_flash_back_half_v1"
        },
        {
          "back": "订单簿数据。",
          "estimated_seconds": 7,
          "explanation": "订单簿数据用于理解高频交易和市场微观结构。",
          "front": "课程后半段除了风险管理、投资组合优化，还会深入什么数据？",
          "question_id": "q_flash_back_half_v2"
        }
      ]
    },
    {
      "concept_key": "key_terms_step2",
      "coverage_tags": [
        "key_terms_step2"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_terms",
      "learning_goal": "学生能给出step2引入的三个关键术语的定义。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "definition_with_example",
      "retrieval_focus": "术语定义",
      "term_refs": [
        {
          "display": "回测",
          "en": "Backtesting"
        },
        {
          "display": "配对交易",
          "en": "Pairs Trading"
        },
        {
          "display": "订单簿",
          "en": "Order Book"
        }
      ],
      "variants": [
        {
          "back": "用历史数据测试交易策略表现的方法。",
          "estimated_seconds": 6,
          "explanation": "回测是验证策略在历史数据上是否有效的过程。",
          "front": "什么是回测？",
          "question_id": "q_flash_terms_v1"
        },
        {
          "back": "利用两只相关股票价格偏离进行套利的策略。",
          "estimated_seconds": 7,
          "explanation": "配对交易基于统计关系，当价差偏离时进行买卖。",
          "front": "什么是配对交易？",
          "question_id": "q_flash_terms_v2"
        },
        {
          "back": "记录所有未成交买卖订单的电子列表。",
          "estimated_seconds": 6,
          "explanation": "订单簿显示市场深度，是高频交易分析的基础。",
          "front": "什么是订单簿？",
          "question_id": "q_flash_terms_v3"
        }
      ]
    }
  ],
  "lesson_id": "L0",
  "longform_families": [
    {
      "concept_key": "course_roadmap",
      "coverage_tags": [
        "course_roadmap"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_roadmap",
      "learning_goal": "学生能用自己的话描述课程从理论到实践的完整路径，并解释前后半段的逻辑关系。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "mechanism_trace",
      "term_refs": [
        {
          "display": "课程路径",
          "en": "Course Roadmap"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "课程路径的起点和终点",
            "前半段的核心活动",
            "后半段的核心活动",
            "前后半段的逻辑关系"
          ],
          "question_id": "q_long_roadmap_v1",
          "reference_answer": [
            "这门课从零开始，目标是部署一个真正的交易策略。",
            "前半段学习获取金融数据、用Python分析，并搭建回测框架来验证策略想法。",
            "后半段转向实战，包括风险管理、投资组合优化，以及深入订单簿数据理解高频交易，最后还会看到机器学习的应用。",
            "前后半段的逻辑是：先打好数据分析和策略验证的基础，再进入更复杂的实战和高级主题，这样学生能循序渐进地掌握从理论到实践的完整流程。"
          ],
          "rubric_points": [
            "明确指出起点是从零开始，终点是部署交易策略",
            "正确列出前半段：数据获取、Python分析、回测框架",
            "正确列出后半段：风险管理、投资组合优化、订单簿、机器学习",
            "解释前半段是基础构建，后半段是实战应用和高级主题"
          ],
          "stem": "请描述这门课的整体路径，并解释为什么前半段先学数据获取和回测，后半段才学风险管理和订单簿。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "课程路径概述",
            "前半段内容",
            "后半段内容",
            "课程安排的理由"
          ],
          "question_id": "q_long_roadmap_v2",
          "reference_answer": [
            "这门课带你从零开始，一直到能部署一个真正的交易策略。",
            "前半段先学怎么拿数据、用Python分析，然后搭一个回测框架来测试你的想法。",
            "后半段进入实战，学风险管理、优化投资组合，还会看订单簿数据理解高频交易，最后接触机器学习在交易中的应用。",
            "这样安排是因为你得先会处理数据和验证策略，才能进入更复杂的实战环节，一步步从理论走到实践。"
          ],
          "rubric_points": [
            "清晰概括从零到部署的路径",
            "准确描述前半段的数据获取、分析和回测",
            "准确描述后半段的风险管理、订单簿和机器学习",
            "解释课程安排从基础到实战的递进逻辑"
          ],
          "stem": "假设你要向一个朋友介绍这门课，请用你自己的话概括课程路径，并说明为什么课程要这样安排。"
        }
      ]
    },
    {
      "concept_key": "back_half_topics",
      "coverage_tags": [
        "back_half_topics"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_back_half",
      "learning_goal": "学生能比较课程后半段中风险管理和订单簿分析的不同关注点。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "compare_and_contrast",
      "term_refs": [
        {
          "display": "订单簿",
          "en": "Order Book"
        },
        {
          "display": "高频交易",
          "en": "High Frequency Trading"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "风险管理的关注点",
            "订单簿分析的关注点",
            "两者的区别",
            "两者如何互补"
          ],
          "question_id": "q_long_back_half_v1",
          "reference_answer": [
            "风险管理主要关注如何控制投资风险，比如设置止损、控制仓位，确保不会因为一次交易损失过大。",
            "订单簿分析则关注市场微观结构，通过分析未成交订单的深度和流动性，优化交易执行时机和成本。",
            "两者的区别在于：风险管理是宏观的资金安全层面，订单簿分析是微观的执行优化层面。",
            "它们共同服务于实战交易：风险管理确保你不会亏光，订单簿分析帮你更聪明地执行交易，两者结合才能实现稳健的算法交易。"
          ],
          "rubric_points": [
            "风险管理关注资金安全、控制损失",
            "订单簿分析关注市场微观结构和执行质量",
            "指出风险管理是宏观层面，订单簿是微观层面",
            "说明两者结合能更好地控制风险和优化执行"
          ],
          "stem": "课程后半段涉及风险管理和订单簿分析。请比较这两个主题的关注点有何不同，并说明它们如何共同服务于实战交易。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "投资组合优化解决的问题",
            "订单簿数据建模解决的问题",
            "两者的配合方式"
          ],
          "question_id": "q_long_back_half_v2",
          "reference_answer": [
            "投资组合优化解决的是如何在多个资产之间分配资金，以达到风险收益的最佳平衡。",
            "订单簿数据建模解决的是交易执行时如何减少市场冲击、降低交易成本。",
            "在实际交易中，先通过投资组合优化决定买什么、买多少，再通过订单簿分析决定怎么买、什么时候买，两者配合才能实现从决策到执行的全流程优化。"
          ],
          "rubric_points": [
            "投资组合优化解决资产配置和风险收益平衡",
            "订单簿数据建模解决交易执行和市场冲击",
            "说明两者配合：先优化组合，再优化执行"
          ],
          "stem": "课程后半段同时涉及投资组合优化和订单簿数据建模。请解释这两个主题分别解决什么问题，以及它们在实际交易中如何配合。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "course_roadmap",
      "coverage_tags": [
        "course_roadmap"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_roadmap",
      "learning_goal": "学生能识别课程路径的起点和终点。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "课程路径",
          "en": "Course Roadmap"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "课程设计为从零基础到实战部署的完整旅程。",
          "options": [
            "从基础概念到高级数学理论",
            "从零开始到部署一个真正的交易策略",
            "从手动交易到自动化交易平台使用",
            "从股票分析到外汇交易"
          ],
          "question_id": "q_quiz_roadmap_v1",
          "stem": "以下哪个选项最准确地描述了这门课的完整路径？"
        },
        {
          "answer": 2,
          "estimated_seconds": 12,
          "explanation": "课程路径的终点是部署一个真正的交易策略。",
          "options": [
            "学会使用MetaTrader平台",
            "通过期末考试",
            "部署一个真正的交易策略",
            "掌握所有金融术语"
          ],
          "question_id": "q_quiz_roadmap_v2",
          "stem": "这门课的最终目标是什么？"
        }
      ]
    },
    {
      "concept_key": "front_half_topics",
      "coverage_tags": [
        "front_half_topics"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_front_half",
      "learning_goal": "学生能区分课程前半段和后半段的主题。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "回测",
          "en": "Backtesting"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "搭建回测框架是前半段的核心内容，属于从数据到策略验证的环节。",
          "options": [
            "风险管理与投资组合优化",
            "订单簿与高频数据建模",
            "搭建回测框架",
            "机器学习在交易中的应用"
          ],
          "question_id": "q_quiz_front_half_v1",
          "stem": "以下哪个主题最可能出现在课程的前半段？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "前半段的流程是：获取数据 → Python分析 → 搭建回测框架验证策略。",
          "options": [
            "直接部署到真实市场",
            "搭建回测框架验证想法",
            "学习机器学习模型",
            "研究高频交易策略"
          ],
          "question_id": "q_quiz_front_half_v2",
          "stem": "课程前半段，你学习获取金融数据、用Python分析后，接下来要做什么？"
        }
      ]
    },
    {
      "concept_key": "back_half_topics",
      "coverage_tags": [
        "back_half_topics"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_back_half",
      "learning_goal": "学生能识别课程后半段的典型主题。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "订单簿",
          "en": "Order Book"
        },
        {
          "display": "高频交易",
          "en": "High Frequency Trading"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "风险管理与投资组合优化属于实战应用部分，通常在课程后半段讨论。",
          "options": [
            "用Python抓取股票数据",
            "搭建回测框架",
            "风险管理与投资组合优化",
            "统计套利基础"
          ],
          "question_id": "q_quiz_back_half_v1",
          "stem": "以下哪个主题最可能出现在课程的后半段？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "订单簿数据是高频交易的核心，用于理解市场微观结构和交易执行。",
          "options": [
            "学习如何下单",
            "理解高频交易的世界",
            "分析公司财务报表",
            "预测宏观经济走势"
          ],
          "question_id": "q_quiz_back_half_v2",
          "stem": "课程后半段深入订单簿数据的主要目的是什么？"
        }
      ]
    },
    {
      "concept_key": "key_terms_step2",
      "coverage_tags": [
        "key_terms_step2"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_terms",
      "learning_goal": "学生能将术语与其定义正确匹配。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "matching",
      "term_refs": [
        {
          "display": "回测",
          "en": "Backtesting"
        },
        {
          "display": "配对交易",
          "en": "Pairs Trading"
        },
        {
          "display": "订单簿",
          "en": "Order Book"
        }
      ],
      "variants": [
        {
          "answer": [
            0,
            1,
            2
          ],
          "estimated_seconds": 25,
          "explanation": "回测验证策略历史表现；配对交易利用统计套利；订单簿显示市场深度。",
          "matches": [
            "用历史数据测试交易策略表现的方法",
            "利用两只相关股票价格偏离进行套利的策略",
            "记录所有未成交买卖订单的电子列表"
          ],
          "options": [
            "回测",
            "配对交易",
            "订单簿"
          ],
          "question_id": "q_quiz_terms_v1",
          "stem": "请将以下术语与对应的定义匹配："
        },
        {
          "answer": 1,
          "estimated_seconds": 12,
          "explanation": "配对交易基于两只相关股票的价格偏离进行套利。",
          "options": [
            "用历史数据测试交易策略表现的方法",
            "利用两只相关股票价格偏离进行套利的策略",
            "记录所有未成交买卖订单的电子列表",
            "用计算机程序自动执行交易决策的过程"
          ],
          "question_id": "q_quiz_terms_v2",
          "stem": "以下哪个定义对应“配对交易”？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/L0/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L0/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "input source outline"
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "market_capitalization_comparison",
      "covered_by": [
        "qf_flash_market_cap",
        "qf_quiz_market_cap"
      ],
      "description": "比较不同股票市场的市值大小，特别是中国A股、日本、香港、印度市场的市值数据。"
    },
    {
      "coverage_tag": "daily_turnover_comparison",
      "covered_by": [
        "qf_flash_daily_turnover",
        "qf_quiz_daily_turnover"
      ],
      "description": "比较不同金融市场的日交易额，包括全球股票市场、外汇市场、商品市场和加密货币市场。"
    },
    {
      "coverage_tag": "market_cap_vs_turnover",
      "covered_by": [
        "qf_flash_market_cap_vs_turnover",
        "qf_quiz_market_cap_vs_turnover"
      ],
      "description": "区分市值和日交易额两个概念，理解市值大不等于交易活跃。"
    },
    {
      "coverage_tag": "forex_market_dominance",
      "covered_by": [
        "qf_flash_forex_dominance",
        "qf_quiz_forex_dominance",
        "qf_long_forex_dominance"
      ],
      "description": "理解外汇市场在日交易额上的主导地位，以及算法交易者偏爱外汇市场的原因。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "market_capitalization_comparison",
      "coverage_tags": [
        "market_capitalization_comparison"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_market_cap",
      "learning_goal": "学生能准确回忆并比较主要股票市场的市值大小。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "要求学生主动提取特定市场的市值数值或排名。",
      "term_refs": [
        {
          "display": "市值",
          "en": "Market Capitalization"
        }
      ],
      "variants": [
        {
          "back": "中国A股市场，约15万亿美元。",
          "estimated_seconds": 8,
          "explanation": "中国A股市场市值远超日本（7.6万亿）、香港（6万亿）和印度（5.3万亿）。",
          "front": "截至2025年底，市值最大的股票市场是哪个？其市值约为多少？",
          "question_id": "q_flash_market_cap_v1"
        },
        {
          "back": "约7.6万亿美元。",
          "estimated_seconds": 6,
          "explanation": "日本市场市值排名第二，低于中国A股市场（15万亿），但高于香港（6万亿）和印度（5.3万亿）。",
          "front": "截至2025年底，日本股票市场的市值约为多少？",
          "question_id": "q_flash_market_cap_v2"
        },
        {
          "back": "约6万亿美元。",
          "estimated_seconds": 6,
          "explanation": "香港市场市值排名第三，低于中国A股（15万亿）和日本（7.6万亿），但高于印度（5.3万亿）。",
          "front": "截至2025年底，香港股票市场的市值约为多少？",
          "question_id": "q_flash_market_cap_v3"
        }
      ]
    },
    {
      "concept_key": "daily_turnover_comparison",
      "coverage_tags": [
        "daily_turnover_comparison"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_daily_turnover",
      "learning_goal": "学生能准确回忆并比较不同金融市场的日交易额大小。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "要求学生主动提取特定市场的日交易额数值或排名。",
      "term_refs": [
        {
          "display": "日交易额",
          "en": "Daily Turnover"
        }
      ],
      "variants": [
        {
          "back": "外汇市场，约6.6万亿美元。",
          "estimated_seconds": 8,
          "explanation": "外汇市场的日交易额远超全球股票市场（3.93万亿）、加密货币市场（1390亿）和商品市场（126亿）。",
          "front": "日交易额最大的金融市场是哪个？其日交易额约为多少？",
          "question_id": "q_flash_daily_turnover_v1"
        },
        {
          "back": "约3.93万亿美元。",
          "estimated_seconds": 6,
          "explanation": "全球股票市场日交易额排名第二，低于外汇市场（6.6万亿），但高于加密货币市场（1390亿）和商品市场（126亿）。",
          "front": "全球股票市场的日交易额约为多少？",
          "question_id": "q_flash_daily_turnover_v2"
        },
        {
          "back": "约1390亿美元。",
          "estimated_seconds": 6,
          "explanation": "加密货币市场日交易额远低于外汇市场（6.6万亿）和全球股票市场（3.93万亿），但略高于商品市场（126亿）。",
          "front": "加密货币市场的日交易额约为多少？",
          "question_id": "q_flash_daily_turnover_v3"
        }
      ]
    },
    {
      "concept_key": "market_cap_vs_turnover",
      "coverage_tags": [
        "market_cap_vs_turnover"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_market_cap_vs_turnover",
      "learning_goal": "学生能区分市值和日交易额这两个概念，并理解它们衡量市场不同维度。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "core_difference",
      "retrieval_focus": "要求学生主动提取市值和日交易额的核心区别。",
      "term_refs": [
        {
          "display": "市值",
          "en": "Market Capitalization"
        },
        {
          "display": "日交易额",
          "en": "Daily Turnover"
        }
      ],
      "variants": [
        {
          "back": "市值衡量市场总规模（存量），日交易额衡量市场活跃度（流量）。",
          "estimated_seconds": 10,
          "explanation": "市值是上市公司股票总数乘以股价的总价值，反映市场大小；日交易额是一天内所有交易的总金额，反映市场流动性。",
          "front": "市值和日交易额分别衡量市场的什么维度？",
          "question_id": "q_flash_market_cap_vs_turnover_v1"
        },
        {
          "back": "不一定。市值大不代表交易活跃，因为活跃度由日交易额衡量，两者是不同概念。",
          "estimated_seconds": 12,
          "explanation": "例如中国A股市值最大，但日交易额最大的市场是外汇市场。市值大只说明市场总价值高，但不一定每天都有大量交易。",
          "front": "一个市场市值很大，是否一定意味着它的交易很活跃？为什么？",
          "question_id": "q_flash_market_cap_vs_turnover_v2"
        }
      ]
    },
    {
      "concept_key": "forex_market_dominance",
      "coverage_tags": [
        "forex_market_dominance"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_forex_dominance",
      "learning_goal": "学生能理解外汇市场在日交易额上的主导地位。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "要求学生主动提取外汇市场日交易额及其与其他市场的对比。",
      "term_refs": [
        {
          "display": "外汇市场",
          "en": "Forex"
        }
      ],
      "variants": [
        {
          "back": "因为外汇市场日交易额最大（约6.6万亿美元），流动性极高，交易机会多。",
          "estimated_seconds": 10,
          "explanation": "高流动性意味着更容易进出市场，滑点更小，是算法交易者的理想环境。",
          "front": "为什么算法交易者偏爱外汇市场？",
          "question_id": "q_flash_forex_dominance_v1"
        },
        {
          "back": "约52倍（6.6万亿美元 / 1260亿美元 ≈ 52.4）。",
          "estimated_seconds": 10,
          "explanation": "外汇市场日交易额6.6万亿美元，商品市场仅1260亿美元，差距巨大。",
          "front": "外汇市场的日交易额大约是商品市场的多少倍？",
          "question_id": "q_flash_forex_dominance_v2"
        }
      ]
    }
  ],
  "lesson_id": "L0",
  "longform_families": [
    {
      "concept_key": "forex_market_dominance",
      "coverage_tags": [
        "forex_market_dominance",
        "market_cap_vs_turnover"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_forex_dominance",
      "learning_goal": "学生能综合运用市值和日交易额的概念，解释为什么算法交易者偏爱外汇市场。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "外汇市场",
          "en": "Forex"
        },
        {
          "display": "日交易额",
          "en": "Daily Turnover"
        },
        {
          "display": "市值",
          "en": "Market Capitalization"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "比较中国A股市场和外汇市场的市值",
            "比较中国A股市场和外汇市场的日交易额",
            "解释日交易额对算法交易者的重要性"
          ],
          "question_id": "q_long_forex_dominance_v1",
          "reference_answer": [
            "中国A股市场市值约15万亿美元，是全球市值最大的股票市场。",
            "但算法交易者更看重市场的流动性，即日交易额。外汇市场日交易额高达6.6万亿美元，远超全球股票市场的3.93万亿美元。",
            "高日交易额意味着外汇市场流动性极佳，算法交易者可以快速执行大额订单而不显著影响价格，滑点成本低，交易机会更多。因此，尽管中国A股市值最大，但外汇市场因其卓越的流动性而更受算法交易者青睐。"
          ],
          "rubric_points": [
            "正确指出中国A股市值最大（15万亿美元），外汇市场不是股票市场，没有可比市值。",
            "正确指出外汇市场日交易额最大（6.6万亿美元），远高于全球股票市场（3.93万亿）。",
            "解释高日交易额意味着高流动性，算法交易者更容易进出市场，滑点更小，交易机会更多。"
          ],
          "stem": "请解释为什么算法交易者偏爱外汇市场，而不是市值最大的中国A股市场。在回答中，请使用市值和日交易额这两个概念。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "比较两个市场的日交易额",
            "讨论流动性对算法交易策略的影响",
            "考虑市场成熟度和监管环境"
          ],
          "question_id": "q_long_forex_dominance_v2",
          "reference_answer": [
            "外汇市场日交易额约6.6万亿美元，而加密货币市场仅约1390亿美元，两者相差约47倍。",
            "高流动性对外汇市场算法交易至关重要：它允许交易者快速执行订单，减少滑点，并支持更复杂的策略如统计套利。",
            "此外，外汇市场是一个成熟、受监管的全球市场，而加密货币市场相对年轻，波动性大，监管环境不明确，可能增加策略风险。因此，对于追求稳定和可预测性的算法交易者，外汇市场通常是更优选择。"
          ],
          "rubric_points": [
            "正确指出外汇市场日交易额（6.6万亿美元）远大于加密货币市场（1390亿美元）。",
            "解释高流动性使外汇市场更适合高频和大额交易，滑点更小。",
            "提及外汇市场更成熟、监管更完善，而加密货币市场波动性大、监管不确定性高。"
          ],
          "stem": "假设你是一名算法交易者，正在选择交易市场。请比较外汇市场和加密货币市场，并解释为什么外汇市场可能更适合算法交易。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "market_capitalization_comparison",
      "coverage_tags": [
        "market_capitalization_comparison"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_market_cap",
      "learning_goal": "学生能在测验情境下准确判断不同股票市场的市值排名。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "市值",
          "en": "Market Capitalization"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 15,
          "explanation": "中国A股市场市值约15万亿美元，日本约7.6万亿，香港约6万亿，印度约5.3万亿。",
          "options": [
            "中国A股市场",
            "日本股票市场",
            "香港股票市场",
            "印度股票市场"
          ],
          "question_id": "q_quiz_market_cap_v1",
          "stem": "截至2025年底，以下哪个股票市场的市值最大？"
        },
        {
          "answer": 3,
          "estimated_seconds": 15,
          "explanation": "印度股票市场市值约5.3万亿美元，是四个选项中最低的。中国A股约15万亿，日本约7.6万亿，香港约6万亿。",
          "options": [
            "中国A股市场",
            "日本股票市场",
            "香港股票市场",
            "印度股票市场"
          ],
          "question_id": "q_quiz_market_cap_v2",
          "stem": "截至2025年底，以下哪个股票市场的市值最小？"
        }
      ]
    },
    {
      "concept_key": "daily_turnover_comparison",
      "coverage_tags": [
        "daily_turnover_comparison"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_daily_turnover",
      "learning_goal": "学生能在测验情境下准确判断不同金融市场的日交易额排名。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "日交易额",
          "en": "Daily Turnover"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "外汇市场日交易额约6.6万亿美元，是全球最大的金融市场。全球股票市场约3.93万亿，加密货币市场约1390亿，商品市场约1260亿。",
          "options": [
            "全球股票市场",
            "外汇市场",
            "商品市场",
            "加密货币市场"
          ],
          "question_id": "q_quiz_daily_turnover_v1",
          "stem": "以下哪个金融市场的日交易额最大？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "商品市场日交易额约1260亿美元，是四个选项中最低的。外汇市场约6.6万亿，全球股票市场约3.93万亿，加密货币市场约1390亿。",
          "options": [
            "全球股票市场",
            "外汇市场",
            "商品市场",
            "加密货币市场"
          ],
          "question_id": "q_quiz_daily_turnover_v2",
          "stem": "以下哪个金融市场的日交易额最小？"
        }
      ]
    },
    {
      "concept_key": "market_cap_vs_turnover",
      "coverage_tags": [
        "market_cap_vs_turnover"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_market_cap_vs_turnover",
      "learning_goal": "学生能辨析市值和日交易额的区别，避免混淆。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "true_false",
      "term_refs": [
        {
          "display": "市值",
          "en": "Market Capitalization"
        },
        {
          "display": "日交易额",
          "en": "Daily Turnover"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "错误。中国A股市值最大，但日交易额最大的市场是外汇市场（6.6万亿美元）。市值和日交易额是衡量市场不同维度的指标。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_market_cap_vs_turnover_v1",
          "stem": "中国A股市场市值最大，因此它的日交易额也最大。"
        },
        {
          "answer": 0,
          "estimated_seconds": 10,
          "explanation": "正确。日交易额是衡量市场活跃度和流动性的重要指标，交易额越大，流动性通常越好。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_market_cap_vs_turnover_v2",
          "stem": "一个市场的日交易额越大，通常意味着它的流动性越好。"
        }
      ]
    },
    {
      "concept_key": "forex_market_dominance",
      "coverage_tags": [
        "forex_market_dominance"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_forex_dominance",
      "learning_goal": "学生能理解外汇市场在日交易额上的主导地位及其对算法交易者的意义。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "外汇市场",
          "en": "Forex"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "外汇市场日交易额高达6.6万亿美元，流动性极佳，是算法交易者的理想市场。",
          "options": [
            "商品市场",
            "外汇市场",
            "加密货币市场",
            "全球股票市场"
          ],
          "question_id": "q_quiz_forex_dominance_v1",
          "stem": "以下哪个市场是算法交易者最偏爱的市场之一，主要是因为其极高的日交易额？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "外汇市场日交易额6.6万亿美元，加密货币市场1390亿美元，6.6万亿 / 1390亿 ≈ 47.5倍。",
          "options": [
            "约4.7倍",
            "约47倍",
            "约470倍",
            "约4700倍"
          ],
          "question_id": "q_quiz_forex_dominance_v2",
          "stem": "外汇市场的日交易额大约是加密货币市场的多少倍？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L0/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "input source outline"
  },
  "target_language": "zh-CN"
}

]
</QUESTION_BANK>

<PLAIN_TEXT>
# Financial Analytics and

# Algorithmic Trading

Course Code: COMP7415

# Background of Lecturer

- Founder of AlgoGene (https://algogene.com)   
Vice Chairman of Algo Challenge Association (https://algochallenge.org)   
- Former algo developer, quant trader, risk manager, data analyst for hedge funds and banks.   
- BSc in Math, Risk Management, Actuarial Science (HKU); MSc in Computer Science (HKU)   
- Champion and awardee of several algo trading competitions, including

CCTV证券资讯频道《宽客天下·全球量化争霸赛》(2017/18)  
- Rotman International Trading Competition (2017)   
CASH Algo Trading Contest (2016)   
WorldQuant Challenge (2014, 2015)

# Let’s play a game!

![](images/997b92b1751c1ce70437186759220c5d106c40ed0862fb47cf1033ee018d9aa5.jpg)

# 1. What’s your study mode?

a) Full time   
b) Part time   
c) Just sit in

# 2. What is your main reason for taking this course?

a) I want to start a career in algo-trading   
b) I want to enhance my current job skills   
c) I am interested in the topic and want to learn more   
d) Just to fulfill graduation requirement

# 3. What do you expect to learn from this course?

a) Some skills for financial data analysis   
b) I want to develop an automated trading system   
c) Tell me some profitable trading strategies   
d) Not sure yet

# 4. Do you have any prior experience in algo-trading?

a) Yes, I have extensive experience   
b) Yes, I have some experience   
c) No, but I have experience in other forms of trading   
d) No, I am completely new to trading

# 5. Have you ever used any algo-trading platforms before?

a) Yes, I have used MetaTrader   
b) Yes, I have used TradingView   
c) Yes, I have used AlgoGene   
d) No, I have no idea what it is

# 6. Which of the following has the largest market capitalization?

a) China stock market   
b) Hong Kong stock market   
c) India stock market   
d) Japan stock market

# Answer

As of Dec-2025,

- China: $15 trillion   
- Hong Kong: $6 trillion   
- India: $5.3 trillion   
- Japan: $7.6 trillion

# 7. Which of the following market has the largest daily turnover/trading volume?

a) Commodity market   
b) Crypto market   
c) Global Equity market   
d) Forex

# Answer

Global equity market: $3.93 trillion   
• Forex: $6.6 trillion   
- Commodity: $126 billion   
- Crypto: $139 billion

# Reference:

- https://www.world-exchanges.org/our-work/statistics   
- https://www.bis.org/statistics/rpfx19 fx.htm   
- https://www.cftc.gov/MarketReports/CommitmentsofTraders/index.htm   
- https://www.coingecko.com/en/global charts

# 8. Which programming language is most widely used in algo-trading?

a) C++   
b) Java   
c) Python   
d) R

# 9. Which of the following market is the most popular for algo traders?

a) Commodities   
b) Cryptocurrency   
c) Equity   
d) Forex

# 10. What's the name of your teachers?

a) Tommy Lai   
b) Tony Lam   
c) Tim Leung   
d) Timothy Lau

# Course Objectives & Learning Outcomes

- Understand the trading process in financial market   
- Understand the fundamental of algorithmic trading framework for building a trading strategy   
- Able to formulate trading strategies, carry out backtesting, optimization, risk management and interpret investment performance   
- Develop practical skills in financial data analysis, trading strategy development, and deployment to real market and broker account

# Topics to be covered

1. Algorithmic trading basics and financial markets   
2. Data scraping and database management with Python   
3. Building backtest framework and rule-based trading strategy   
4. Statistical time series analysis for market classification   
5. Statistical arbitrage and pairs trading   
6. Capital and Risk Management   
7. Performance measures and portfolio optimization   
8. Order book and high frequency data modeling   
9. Market practice in strategy optimization, broker selection, and market tricks   
10. Machine learning use cases in algorithmic trading

# Assessment methods

- Group project - 50%   
• Written final exam covers all taught content in the course – 50%

# Contact

Lecturer

- Tony Lam: tonylamfm@hku.hk / tonylam@algogene.com

TA:

- Rex Tsang: trex@hku.hk
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
