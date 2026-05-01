请基于以下 lesson 材料，生成一个结构化题库 JSON。

目标语言：
zh-CN

lesson_id：
L0

要求：
- 同时生成 `flashcard_families`、`quiz_families` 和 `longform_families`
- 题目必须只关联到当前 step：`step2`
- 所有 family 和 variant 的 `linked_steps` 都必须等于 `["step2"]`
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

<CURRENT_STEP_ID>
step2
</CURRENT_STEP_ID>

<CURRENT_STEP>
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
}

</CURRENT_STEP>

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

请直接输出 JSON，不要加解释。
