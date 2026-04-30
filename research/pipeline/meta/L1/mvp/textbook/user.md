请基于以下材料，生成一份 lesson 级 MDX 课本。

目标语言：
zh-CN

lesson_id：
L1

要求：
- 面向整节课，而不是单个 step
- 系统化组织内容，适合预习与复习
- 要能嵌入题目与题族
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
# L1: Algorithmic trading basics and financial markets
Course Code: COMP7415
# Agenda
- The basic concept of algo-trading
- The market trend of algo-trading
- Comparison of different trading approaches
• Infrastructure needed to deploy algo-trading programs
- Trading mechanism and order matching
Market jargon and terminology
# What is algo-trading?
- Algorithmic trading, also called Algo Trading, Quant Trading, Robo-Trading, Program Trading
- Use of mathematical models and computer algorithms/programs to
- generate trading signals (i.e. Decision making)
- automate trading process (i.e. Execution)
- Objectives:
- maximize profits
- control execution costs
- hedging and manage investment risks
# Market Trend for Algo-Trading
Expected to grow at CAGR $12.7\%$ to US $\$ 32$ b by 2028
- Largest market in North America
- Fastest growing in Asia Pacific
![](images/23944073ff63d8284dc99146e03872d7534460656b988edca49867c2a1823382.jpg)
![](images/553c1850eb62275b06eb4b2fa2de02aa49eb0882c33893ea01dbf6746a04d526.jpg)
# Market Trend for Algo-Trading
Algorithmic trading strategies account for approximately:
60% of all US equity volume
40% of all European equity volume
25% of all Forex transactions
20% of all US option trades
# Robo-Trading vs Robo-Advisory
<table><tr><td></td><td>Robo-Trading</td><td>Robo-Advisory</td></tr><tr><td>Decision making</td><td>✓</td><td>✓</td></tr><tr><td>Execution</td><td>✓</td><td>×</td></tr><tr><td>Automation</td><td>Fully automated</td><td>Semi-automated</td></tr><tr><td>Outputs</td><td>Execution of trading signals</td><td>• Stock advices
• Market summary
• Risk alerts</td></tr></table>
![](images/2853c1739330c29f9f1c777cb0195998cf6f596c913e4ccd557e9da5aa1bb6d5.jpg)
ALGOGENE
FinTech Co. Ltd.
JOIN US ON TELEGRAM
ALGOGENESIGNAL
FREE DAILY REAL-TIME TRADING SIGNALS
![](images/de6188289d2654bafc1a1dc3b641da604e09b47ea7d604dbc7620e6c670f49e3.jpg)
@algogene_signal
![](images/fd8bf798e299893a5e056c49cb7b184a1a3217af05382e16f6f2651b4fb021eb.jpg)
Warning: Beware of scam Telegram groups. Please note that the only official group is @algogene_signal
# Comparison of Trading Approaches
manual
automatic
<table><tr><td></td><td>Human Trading</td><td>Robo-Advisory</td><td>Robo-Trading</td></tr><tr><td>Working Hours</td><td>~9*5</td><td>~9*5</td><td>24*7</td></tr><tr><td>Execution Speed</td><td>Slow</td><td>Moderate</td><td>Immediately</td></tr><tr><td>Data Inputs</td><td>Limited</td><td>Almost unlimited</td><td>Almost unlimited</td></tr><tr><td>Trading Frequency</td><td>Low</td><td>Low - mid</td><td>Low – High</td></tr><tr><td>Scalability</td><td>Limited investment size due to stress</td><td>Moderate</td><td>High</td></tr><tr><td>Accuracy/ Discipline</td><td>Variable especially when you lose</td><td>Consistent</td><td>High</td></tr><tr><td>Customization</td><td>High</td><td>Medium</td><td>Low</td></tr><tr><td>User Control</td><td>Full</td><td>Partial</td><td>Minimal</td></tr><tr><td>Risk Management</td><td>Subjective</td><td>Algorithmsic</td><td>Algorithmsic</td></tr></table>
# What should you consider to implement a trading algo?
# Algo is the vehicle; Data is the Fuel!
- Data collection - time and effort demanding in data sourcing and data cleaning
- Data storage - high volume of unstructured data (Numeric, Text, Image, Audio, Video)
- Data management - difficult to protect and manage through traditional databases due to inconsistent file and data formats
- Data processing - feasibility and efficiency
# Other factors for infrastructure setup
Real-time datafeed
Historical data
- Computing hardware
- Exchange/broker connectivity
Network and latencies
</COVERAGE_CHECKLIST>

<SOURCE_OUTLINE>
# L1: Algorithmic trading basics and financial markets
Course Code: COMP7415
# Agenda
- The basic concept of algo-trading
- The market trend of algo-trading
- Comparison of different trading approaches
• Infrastructure needed to deploy algo-trading programs
- Trading mechanism and order matching
Market jargon and terminology
# What is algo-trading?
- Algorithmic trading, also called Algo Trading, Quant Trading, Robo-Trading, Program Trading
- Use of mathematical models and computer algorithms/programs to
- generate trading signals (i.e. Decision making)
- automate trading process (i.e. Execution)
- Objectives:
- maximize profits
- control execution costs
- hedging and manage investment risks
# Market Trend for Algo-Trading
Expected to grow at CAGR $12.7\%$ to US $\$ 32$ b by 2028
- Largest market in North America
- Fastest growing in Asia Pacific
![](images/23944073ff63d8284dc99146e03872d7534460656b988edca49867c2a1823382.jpg)
![](images/553c1850eb62275b06eb4b2fa2de02aa49eb0882c33893ea01dbf6746a04d526.jpg)
# Market Trend for Algo-Trading
Algorithmic trading strategies account for approximately:
60% of all US equity volume
40% of all European equity volume
25% of all Forex transactions
20% of all US option trades
# Robo-Trading vs Robo-Advisory
<table><tr><td></td><td>Robo-Trading</td><td>Robo-Advisory</td></tr><tr><td>Decision making</td><td>✓</td><td>✓</td></tr><tr><td>Execution</td><td>✓</td><td>×</td></tr><tr><td>Automation</td><td>Fully automated</td><td>Semi-automated</td></tr><tr><td>Outputs</td><td>Execution of trading signals</td><td>• Stock advices
• Market summary
• Risk alerts</td></tr></table>
![](images/2853c1739330c29f9f1c777cb0195998cf6f596c913e4ccd557e9da5aa1bb6d5.jpg)
ALGOGENE
FinTech Co. Ltd.
JOIN US ON TELEGRAM
ALGOGENESIGNAL
FREE DAILY REAL-TIME TRADING SIGNALS
![](images/de6188289d2654bafc1a1dc3b641da604e09b47ea7d604dbc7620e6c670f49e3.jpg)
@algogene_signal
![](images/fd8bf798e299893a5e056c49cb7b184a1a3217af05382e16f6f2651b4fb021eb.jpg)
Warning: Beware of scam Telegram groups. Please note that the only official group is @algogene_signal
# Comparison of Trading Approaches
manual
automatic
<table><tr><td></td><td>Human Trading</td><td>Robo-Advisory</td><td>Robo-Trading</td></tr><tr><td>Working Hours</td><td>~9*5</td><td>~9*5</td><td>24*7</td></tr><tr><td>Execution Speed</td><td>Slow</td><td>Moderate</td><td>Immediately</td></tr><tr><td>Data Inputs</td><td>Limited</td><td>Almost unlimited</td><td>Almost unlimited</td></tr><tr><td>Trading Frequency</td><td>Low</td><td>Low - mid</td><td>Low – High</td></tr><tr><td>Scalability</td><td>Limited investment size due to stress</td><td>Moderate</td><td>High</td></tr><tr><td>Accuracy/ Discipline</td><td>Variable especially when you lose</td><td>Consistent</td><td>High</td></tr><tr><td>Customization</td><td>High</td><td>Medium</td><td>Low</td></tr><tr><td>User Control</td><td>Full</td><td>Partial</td><td>Minimal</td></tr><tr><td>Risk Management</td><td>Subjective</td><td>Algorithmsic</td><td>Algorithmsic</td></tr></table>
# What should you consider to implement a trading algo?
# Algo is the vehicle; Data is the Fuel!
- Data collection - time and effort demanding in data sourcing and data cleaning
- Data storage - high volume of unstructured data (Numeric, Text, Image, Audio, Video)
- Data management - difficult to protect and manage through traditional databases due to inconsistent file and data formats
- Data processing - feasibility and efficiency
# Other factors for infrastructure setup
Real-time datafeed
Historical data
- Computing hardware
- Exchange/broker connectivity
Network and latencies
</SOURCE_OUTLINE>

<LESSON_MAP>
{
  "lesson_id": "L1",
  "mode": "guided_story",
  "steps": [
    {
      "concept": "MVP lesson slice",
      "file": "research/pipeline/3-guided_story/L1.step1.json",
      "sequence_id": "step1"
    }
  ]
}

</LESSON_MAP>

<GUIDED_STORY_MANIFEST>
{
  "lesson_id": "L1",
  "mode": "guided_story",
  "steps": [
    {
      "concept": "MVP lesson slice",
      "file": "research/pipeline/3-guided_story/L1.step1.json",
      "sequence_id": "step1"
    }
  ]
}

</GUIDED_STORY_MANIFEST>

<GUIDED_STORY_STEPS>
{
  "lesson_id": "L1",
  "mode": "guided_story",
  "screens": [
    {
      "focus_terms": [],
      "id": "s001",
      "introduced_terms": [],
      "lines": [
        "想象一下，你坐在电脑前，",
        "屏幕上跳出一个信号：“买入”。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s002",
      "introduced_terms": [],
      "lines": [
        "不到一秒钟，订单已经发出，",
        "交易自动完成。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "algorithmic_trading"
      ],
      "id": "s003",
      "introduced_terms": [
        "algorithmic_trading"
      ],
      "lines": [
        "这不是科幻电影，",
        "这就是**<term id=\"algorithmic_trading\">算法交易</term>**。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "algorithmic_trading"
      ],
      "id": "s004",
      "introduced_terms": [],
      "lines": [
        "它用数学模型和计算机程序，",
        "代替人类做判断和执行。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s005",
      "introduced_terms": [],
      "lines": [
        "目标很明确：",
        "最大化利润，控制成本，管理风险。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s006",
      "introduced_terms": [],
      "lines": [
        "如今，算法交易已经占据了",
        "美国股市约60%的交易量。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s007",
      "introduced_terms": [],
      "lines": [
        "在欧洲，这个数字是40%；",
        "在外汇市场，是25%。"
      ],
      "type": "narration"
    },
    {
      "exercise": {
        "answer": 1,
        "explanation": "算法交易的核心是用规则和程序来处理交易判断与执行，追求速度和纪律性，但并不能消除风险或保证盈利。",
        "kind": "single_choice",
        "options": [
          "完全避免所有投资风险",
          "用规则和程序实现快速、自动化的交易决策与执行",
          "保证每次交易都能盈利",
          "只需要一台电脑，不需要任何金融知识"
        ],
        "prompt": "以下哪一项最准确地描述了算法交易的主要优势？"
      },
      "focus_terms": [],
      "id": "s008",
      "introduced_terms": [],
      "lines": [
        "猜猜看：算法交易的核心优势是什么？"
      ],
      "type": "exercise"
    },
    {
      "focus_terms": [],
      "id": "s009",
      "introduced_terms": [],
      "lines": [
        "但算法交易不是凭空运行的。",
        "它需要一套完整的基础设施。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s010",
      "introduced_terms": [],
      "lines": [
        "数据是燃料，算法是引擎。",
        "没有干净、实时的数据，再好的策略也无用。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s011",
      "introduced_terms": [],
      "lines": [
        "你需要处理数据的采集、存储、管理，",
        "还要考虑计算硬件、网络延迟和系统安全。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s012",
      "introduced_terms": [],
      "lines": [
        "在这一切背后，",
        "是市场最基础的运作机制：订单如何匹配。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s013",
      "introduced_terms": [],
      "lines": [
        "每个卖家都有一个心理价位，",
        "每个买家也有一个。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s014",
      "introduced_terms": [],
      "lines": [
        "当买家的最高出价和卖家的最低要价相遇，",
        "交易就发生了。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s015",
      "introduced_terms": [],
      "lines": [
        "这个相遇点，就是市场均衡价格。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s016",
      "introduced_terms": [],
      "lines": [
        "但市场远不止这么简单。",
        "让我们认识几个关键术语。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "long_position",
        "short_position"
      ],
      "id": "s017",
      "introduced_terms": [
        "long_position",
        "short_position"
      ],
      "lines": [
        "**<term id=\"long_position\">多头头寸</term>**：买入资产，赌它涨。",
        "**<term id=\"short_position\">空头头寸</term>**：卖出借来的资产，赌它跌。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "short_position"
      ],
      "id": "s018",
      "introduced_terms": [],
      "lines": [
        "做空需要先借入股票，然后卖出，",
        "等价格下跌后再买回归还。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "short_position"
      ],
      "id": "s019",
      "introduced_terms": [],
      "lines": [
        "但做空的风险是无限的——",
        "如果股价一直涨，你的亏损可能没有上限。"
      ],
      "type": "narration"
    },
    {
      "exercise": {
        "answer": 1,
        "explanation": "预期价格下跌时，建立空头头寸（做空）才能从价格下跌中获利。",
        "kind": "single_choice",
        "options": [
          "多头头寸",
          "空头头寸",
          "不建立任何头寸",
          "同时建立多头和空头头寸"
        ],
        "prompt": "如果你预期某只股票价格会下跌，你应该建立什么头寸？"
      },
      "focus_terms": [],
      "id": "s020",
      "introduced_terms": [],
      "lines": [
        "来个小测试。"
      ],
      "type": "exercise"
    },
    {
      "focus_terms": [
        "order_book"
      ],
      "id": "s021",
      "introduced_terms": [
        "order_book"
      ],
      "lines": [
        "接下来，看看订单簿。",
        "它是市场的实时“记账本”。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "order_book",
        "bid_price",
        "ask_price"
      ],
      "id": "s022",
      "introduced_terms": [],
      "lines": [
        "**<term id=\"order_book\">订单簿</term>**里，",
        "一边是买单（Bid），一边是卖单（Ask）。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "bid_price",
        "ask_price"
      ],
      "id": "s023",
      "introduced_terms": [
        "bid_price",
        "ask_price"
      ],
      "lines": [
        "**<term id=\"bid_price\">买入价</term>**是买方愿意支付的最高价。",
        "**<term id=\"ask_price\">卖出价</term>**是卖方愿意接受的最低价。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "market_spread"
      ],
      "id": "s024",
      "introduced_terms": [
        "market_spread"
      ],
      "lines": [
        "卖出价减去买入价，就是**<term id=\"market_spread\">买卖价差</term>**。",
        "价差越小，市场流动性越好。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "market_spread"
      ],
      "formula": {
        "latex": "\\text{Percentage Spread} = \\frac{\\text{AskPrice} - \\text{BidPrice}}{\\text{MidPrice}} \\times 100\\%",
        "spoken": "百分比价差等于卖出价减去买入价，再除以中间价，乘以百分之百。"
      },
      "id": "s025",
      "introduced_terms": [],
      "lines": [
        "价差也可以用百分比来衡量，方便不同资产之间比较。"
      ],
      "type": "formula"
    },
    {
      "focus_terms": [
        "market_order",
        "limit_order",
        "stop_order"
      ],
      "id": "s026",
      "introduced_terms": [],
      "lines": [
        "有了订单簿，你还需要知道怎么下单。",
        "主要有三种订单类型。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "market_order"
      ],
      "id": "s027",
      "introduced_terms": [
        "market_order"
      ],
      "lines": [
        "**<term id=\"market_order\">市价单</term>**：立即以当前最优价成交。",
        "保证成交，但不保证价格。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "limit_order"
      ],
      "id": "s028",
      "introduced_terms": [
        "limit_order"
      ],
      "lines": [
        "**<term id=\"limit_order\">限价单</term>**：指定一个价格，只在该价格或更优价格成交。",
        "保证价格，但不保证成交。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "stop_order"
      ],
      "id": "s029",
      "introduced_terms": [
        "stop_order"
      ],
      "lines": [
        "**<term id=\"stop_order\">止损单</term>**：设定一个触发价，一旦触及，就变成市价单。",
        "常用于控制亏损或捕捉突破行情。"
      ],
      "type": "narration"
    },
    {
      "exercise": {
        "answer": 1,
        "explanation": "限价单允许你指定一个最高买入价（50元），确保不会以高于此价格成交。",
        "kind": "single_choice",
        "options": [
          "市价单",
          "限价单",
          "止损单",
          "以上都可以"
        ],
        "prompt": "你想以不高于50元的价格买入某只股票，应该使用哪种订单？"
      },
      "focus_terms": [],
      "id": "s030",
      "introduced_terms": [],
      "lines": [
        "现在，考考你。"
      ],
      "type": "exercise"
    },
    {
      "focus_terms": [
        "slippage"
      ],
      "id": "s031",
      "introduced_terms": [
        "slippage"
      ],
      "lines": [
        "即使下了单，实际成交价也可能和预期不同。",
        "这个差异叫**<term id=\"slippage\">滑点</term>**。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "slippage"
      ],
      "id": "s032",
      "introduced_terms": [],
      "lines": [
        "滑点通常发生在市场波动剧烈，",
        "或者订单量太大、市场深度不足的时候。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "leverage",
        "margin"
      ],
      "id": "s033",
      "introduced_terms": [
        "leverage",
        "margin"
      ],
      "lines": [
        "最后，聊聊**<term id=\"leverage\">杠杆</term>**和**<term id=\"margin\">保证金</term>**。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "leverage",
        "margin"
      ],
      "id": "s034",
      "introduced_terms": [],
      "lines": [
        "杠杆就是用借来的钱放大收益（也放大亏损）。",
        "保证金就是你自己出的那部分本金。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "leverage",
        "margin"
      ],
      "formula": {
        "latex": "\\text{Leverage Ratio} = \\frac{1}{\\text{Margin Requirement}}",
        "spoken": "杠杆比率等于一除以保证金要求。"
      },
      "id": "s035",
      "introduced_terms": [],
      "lines": [
        "杠杆比率和保证金要求之间的关系很简单。"
      ],
      "type": "formula"
    },
    {
      "focus_terms": [
        "leverage",
        "margin"
      ],
      "id": "s036",
      "introduced_terms": [],
      "lines": [
        "比如，如果保证金要求是10%，",
        "那么杠杆就是10倍。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s037",
      "introduced_terms": [],
      "lines": [
        "这意味着，用1万元本金，",
        "你可以操作价值10万元的资产。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "leverage"
      ],
      "id": "s038",
      "introduced_terms": [],
      "lines": [
        "但别忘了，杠杆是双刃剑。",
        "收益放大多少倍，亏损也放大多少倍。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s039",
      "introduced_terms": [],
      "lines": [
        "从算法交易到订单簿，从做多到做空，",
        "这些就是金融市场的“通用语言”。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s040",
      "introduced_terms": [],
      "lines": [
        "掌握它们，你就能看懂市场在说什么。"
      ],
      "type": "narration"
    }
  ],
  "sequence_id": "step1",
  "source": {
    "plain_text": "Algorithmic trading basics and financial markets. What is algo-trading? Market Trend for Algo-Trading. Comparison of Trading Approaches. Infrastructure needed. Trading mechanism and order matching. Market jargon and terminology.",
    "related": [
      "L1: Algorithmic trading basics and financial markets"
    ]
  },
  "target_language": "zh-CN",
  "term_catalog": {
    "algorithmic_trading": {
      "aliases": [
        "Algo Trading",
        "Quant Trading",
        "Robo-Trading",
        "Program Trading"
      ],
      "display": "算法交易",
      "gloss": "用数学模型和计算机程序来生成交易信号并自动执行交易的方式。"
    },
    "ask_price": {
      "aliases": [
        "Ask Price",
        "卖一价"
      ],
      "display": "卖出价",
      "gloss": "卖方愿意接受的最低价格。"
    },
    "bid_price": {
      "aliases": [
        "Bid Price",
        "买一价"
      ],
      "display": "买入价",
      "gloss": "买方愿意支付的最高价格。"
    },
    "leverage": {
      "aliases": [
        "Leverage"
      ],
      "display": "杠杆",
      "gloss": "使用借入资金放大投资回报（也放大风险）的方式。"
    },
    "limit_order": {
      "aliases": [
        "Limit Order"
      ],
      "display": "限价单",
      "gloss": "指定一个价格（限价），只在该价格或更优价格成交的订单。"
    },
    "long_position": {
      "aliases": [
        "Long Position",
        "做多"
      ],
      "display": "多头头寸",
      "gloss": "买入资产，预期价格上涨后卖出获利。"
    },
    "margin": {
      "aliases": [
        "Margin"
      ],
      "display": "保证金",
      "gloss": "投资者为获得杠杆交易而必须存入的自有资金。"
    },
    "market_order": {
      "aliases": [
        "Market Order"
      ],
      "display": "市价单",
      "gloss": "以当前市场上可获得的最佳价格立即买入或卖出的订单。"
    },
    "market_spread": {
      "aliases": [
        "Bid-Ask Spread",
        "Spread"
      ],
      "display": "买卖价差",
      "gloss": "卖出价与买入价之间的差额，衡量市场流动性的关键指标。"
    },
    "order_book": {
      "aliases": [
        "Order Book",
        "深度图"
      ],
      "display": "订单簿",
      "gloss": "实时列出所有买入和卖出挂单的电子列表，按价格排序。"
    },
    "short_position": {
      "aliases": [
        "Short Position",
        "做空"
      ],
      "display": "空头头寸",
      "gloss": "卖出借来的资产，预期价格下跌后低价买回获利。"
    },
    "slippage": {
      "aliases": [
        "Slippage"
      ],
      "display": "滑点",
      "gloss": "预期成交价格与实际成交价格之间的差异。"
    },
    "stop_order": {
      "aliases": [
        "Stop Order",
        "Stop-Loss Order"
      ],
      "display": "止损单",
      "gloss": "当价格触及指定价格（止损价）时，自动变为市价单执行的订单。"
    }
  }
}

</GUIDED_STORY_STEPS>

<QUESTION_BANK>
{
  "coverage_map": [
    {
      "coverage_tag": "algo_trading_basics",
      "covered_by": [
        "qf_flash_algo_def",
        "qf_long_algo_compare"
      ],
      "description": "算法交易的基本概念、别名、核心机制（信号生成与执行）和目标"
    },
    {
      "coverage_tag": "market_trend",
      "covered_by": [
        "qf_flash_market_trend"
      ],
      "description": "算法交易的市场趋势：增长率、区域分布、各市场占比"
    },
    {
      "coverage_tag": "trading_approaches_comparison",
      "covered_by": [
        "qf_long_algo_compare"
      ],
      "description": "人类交易、机器人投顾、机器人交易三种方式的对比"
    },
    {
      "coverage_tag": "infrastructure_needed",
      "covered_by": [
        "qf_flash_infra"
      ],
      "description": "部署算法交易所需的基础设施：数据采集、存储、管理、处理、硬件、网络等"
    },
    {
      "coverage_tag": "trading_mechanism_order_matching",
      "covered_by": [
        "qf_flash_order_book",
        "qf_long_order_matching"
      ],
      "description": "交易机制与订单匹配：供需、市场均衡、订单簿"
    },
    {
      "coverage_tag": "market_jargon_terminology",
      "covered_by": [
        "qf_flash_long_short",
        "qf_flash_bid_ask_spread",
        "qf_flash_order_types",
        "qf_flash_slippage",
        "qf_flash_leverage_margin",
        "qf_long_leverage_calc"
      ],
      "description": "市场术语：多头/空头、买入价/卖出价、价差、订单类型、滑点、杠杆与保证金"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "algorithmic_trading",
      "coverage_tags": [
        "algo_trading_basics"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_algo_def",
      "learning_goal": "学生能识别算法交易的核心定义、别名与目标。",
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
          "estimated_seconds": 10,
          "explanation": "算法交易的核心是用数学模型和程序来自动化交易决策与执行，追求速度和纪律性。",
          "options": [
            "完全由人类手动下单的交易方式",
            "使用数学模型和计算机程序生成交易信号并自动执行",
            "只用于长期投资的交易策略",
            "一种保证盈利的交易方法"
          ],
          "question_id": "q_flash_algo_def_v1",
          "stem": "以下哪一项最准确地描述了算法交易？"
        },
        {
          "answer": 2,
          "estimated_seconds": 10,
          "explanation": "算法交易的目标是最大化利润、控制成本和管理风险，但无法保证每次交易都盈利。",
          "options": [
            "最大化利润",
            "控制执行成本",
            "保证每次交易都盈利",
            "对冲和管理投资风险"
          ],
          "question_id": "q_flash_algo_def_v2",
          "stem": "算法交易的主要目标不包括以下哪一项？"
        },
        {
          "answer": 2,
          "estimated_seconds": 10,
          "explanation": "高频交易是算法交易的一种子类型，而非其别名。常见的别名包括量化交易、程序化交易和机器人交易。",
          "options": [
            "量化交易 (Quant Trading)",
            "程序化交易 (Program Trading)",
            "高频交易 (High-Frequency Trading)",
            "机器人交易 (Robo-Trading)"
          ],
          "question_id": "q_flash_algo_def_v3",
          "stem": "以下哪个不是算法交易的常见别名？"
        }
      ]
    },
    {
      "concept_key": "market_trend",
      "coverage_tags": [
        "market_trend"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_market_trend",
      "learning_goal": "学生能记住算法交易市场增长的关键数据和区域分布。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "算法交易市场趋势",
          "en": "Market Trend for Algo-Trading"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 8,
          "explanation": "根据材料，算法交易市场预计以12.7%的年复合增长率增长，到2028年达到320亿美元。",
          "options": [
            "US$ 12 billion",
            "US$ 32 billion",
            "US$ 50 billion",
            "US$ 100 billion"
          ],
          "question_id": "q_flash_market_trend_v1",
          "stem": "预计到2028年，全球算法交易市场的规模将达到多少？"
        },
        {
          "answer": 2,
          "estimated_seconds": 8,
          "explanation": "算法交易策略约占美国股票交易量的60%。",
          "options": [
            "20%",
            "40%",
            "60%",
            "80%"
          ],
          "question_id": "q_flash_market_trend_v2",
          "stem": "算法交易策略在美国股票交易量中大约占多少比例？"
        },
        {
          "answer": 2,
          "estimated_seconds": 8,
          "explanation": "北美是最大的市场，但亚太地区是增长最快的市场。",
          "options": [
            "北美",
            "欧洲",
            "亚太地区",
            "非洲"
          ],
          "question_id": "q_flash_market_trend_v3",
          "stem": "以下哪个地区是算法交易增长最快的市场？"
        }
      ]
    },
    {
      "concept_key": "infrastructure",
      "coverage_tags": [
        "infrastructure_needed"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_infra",
      "learning_goal": "学生能识别部署算法交易所需的关键基础设施组件。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "基础设施",
          "en": "Infrastructure"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 8,
          "explanation": "材料中强调“数据是燃料，算法是引擎”，说明数据是驱动算法交易的核心资源。",
          "options": [
            "算法模型",
            "计算硬件",
            "数据",
            "网络连接"
          ],
          "question_id": "q_flash_infra_v1",
          "stem": "在算法交易中，常被比喻为“燃料”的是什么？"
        },
        {
          "answer": 2,
          "estimated_seconds": 10,
          "explanation": "基础设施包括数据、硬件、网络连接等，而交易员的直觉判断属于人类交易范畴，不是算法交易的基础设施。",
          "options": [
            "实时数据源",
            "历史数据",
            "交易员的直觉判断",
            "网络延迟"
          ],
          "question_id": "q_flash_infra_v2",
          "stem": "以下哪一项不是部署算法交易程序时需要考虑的基础设施因素？"
        }
      ]
    },
    {
      "concept_key": "long_short_position",
      "coverage_tags": [
        "market_jargon_terminology"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_long_short",
      "learning_goal": "学生能区分多头头寸和空头头寸的基本概念和适用场景。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "多头头寸",
          "en": "Long Position"
        },
        {
          "display": "空头头寸",
          "en": "Short Position"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 8,
          "explanation": "预期价格上涨时，建立多头头寸（做多）才能从价格上涨中获利。",
          "options": [
            "多头头寸",
            "空头头寸",
            "不建立任何头寸",
            "同时建立多头和空头头寸"
          ],
          "question_id": "q_flash_long_short_v1",
          "stem": "如果你预期某只股票价格会上涨，你应该建立什么头寸？"
        },
        {
          "answer": 1,
          "estimated_seconds": 8,
          "explanation": "做空需要先借入股票，然后卖出，等价格下跌后再买回归还。",
          "options": [
            "卖出自己持有的股票",
            "从券商借入股票",
            "等待股价下跌",
            "买入看跌期权"
          ],
          "question_id": "q_flash_long_short_v2",
          "stem": "做空（Short Selling）的第一步是什么？"
        },
        {
          "answer": 1,
          "estimated_seconds": 8,
          "explanation": "如果股价持续上涨，做空者的亏损理论上没有上限，这是做空的主要风险。",
          "options": [
            "亏损有限",
            "亏损可能是无限的",
            "无法获得股息",
            "交易成本过高"
          ],
          "question_id": "q_flash_long_short_v3",
          "stem": "做空交易的主要风险是什么？"
        }
      ]
    },
    {
      "concept_key": "market_spread",
      "coverage_tags": [
        "market_jargon_terminology"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_bid_ask_spread",
      "learning_goal": "学生能计算简单的买卖价差，并理解其与流动性的关系。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "fill_in_blank",
      "term_refs": [
        {
          "display": "买入价",
          "en": "Bid Price"
        },
        {
          "display": "卖出价",
          "en": "Ask Price"
        },
        {
          "display": "买卖价差",
          "en": "Bid-Ask Spread"
        }
      ],
      "variants": [
        {
          "answer": "1",
          "estimated_seconds": 10,
          "explanation": "买卖价差 = 卖出价 - 买入价 = 25 - 24 = 1美元。",
          "options": [],
          "question_id": "q_flash_bid_ask_spread_v1",
          "stem": "某股票的买入价为 $24，卖出价为 $25，则买卖价差为 $____。"
        },
        {
          "answer": "0.50",
          "estimated_seconds": 10,
          "explanation": "买卖价差 = 卖出价 - 买入价 = 99.00 - 98.50 = 0.50美元。",
          "options": [],
          "question_id": "q_flash_bid_ask_spread_v2",
          "stem": "某股票的买入价为 $98.50，卖出价为 $99.00，则买卖价差为 $____。"
        }
      ]
    },
    {
      "concept_key": "order_types",
      "coverage_tags": [
        "market_jargon_terminology"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_order_types",
      "learning_goal": "学生能区分市价单、限价单和止损单的核心特征。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "市价单",
          "en": "Market Order"
        },
        {
          "display": "限价单",
          "en": "Limit Order"
        },
        {
          "display": "止损单",
          "en": "Stop Order"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 8,
          "explanation": "市价单以当前最优价立即成交，保证成交，但价格不确定。",
          "options": [
            "市价单",
            "限价单",
            "止损单",
            "以上都不对"
          ],
          "question_id": "q_flash_order_types_v1",
          "stem": "哪种订单类型保证成交，但不保证成交价格？"
        },
        {
          "answer": 1,
          "estimated_seconds": 8,
          "explanation": "限价单允许你指定一个最高买入价（50元），确保不会以高于此价格成交。",
          "options": [
            "市价单",
            "限价单",
            "止损单",
            "以上都可以"
          ],
          "question_id": "q_flash_order_types_v2",
          "stem": "你想以不高于50元的价格买入某只股票，应该使用哪种订单？"
        },
        {
          "answer": 1,
          "estimated_seconds": 8,
          "explanation": "止损单在触发后会自动变为市价单，以当前市场价成交。",
          "options": [
            "限价单",
            "市价单",
            "取消",
            "保持不变"
          ],
          "question_id": "q_flash_order_types_v3",
          "stem": "止损单在价格触及止损价后会变成什么？"
        }
      ]
    },
    {
      "concept_key": "slippage",
      "coverage_tags": [
        "market_jargon_terminology"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_slippage",
      "learning_goal": "学生能识别滑点的定义和常见发生场景。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "滑点",
          "en": "Slippage"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 8,
          "explanation": "滑点是预期成交价格与实际成交价格之间的差异。",
          "options": [
            "交易佣金",
            "预期成交价与实际成交价之间的差异",
            "买卖价差",
            "市场波动率"
          ],
          "question_id": "q_flash_slippage_v1",
          "stem": "滑点（Slippage）指的是什么？"
        },
        {
          "answer": 1,
          "estimated_seconds": 8,
          "explanation": "滑点通常发生在市场波动剧烈或订单量太大、市场深度不足的时候。",
          "options": [
            "市场流动性充足时",
            "市场波动剧烈时",
            "使用限价单时",
            "交易量很小时"
          ],
          "question_id": "q_flash_slippage_v2",
          "stem": "以下哪种情况最可能导致滑点？"
        }
      ]
    },
    {
      "concept_key": "leverage_margin",
      "coverage_tags": [
        "market_jargon_terminology"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_leverage_margin",
      "learning_goal": "学生能理解杠杆和保证金的基本概念及其关系。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "杠杆",
          "en": "Leverage"
        },
        {
          "display": "保证金",
          "en": "Margin"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 10,
          "explanation": "杠杆比率 = 1 / 保证金要求 = 1 / 0.2 = 5倍。",
          "options": [
            "2倍",
            "5倍",
            "10倍",
            "20倍"
          ],
          "question_id": "q_flash_leverage_margin_v1",
          "stem": "如果保证金要求是20%，那么杠杆比率是多少？"
        },
        {
          "answer": 1,
          "estimated_seconds": 8,
          "explanation": "杠杆是双刃剑，在放大收益的同时也会等比例放大亏损。",
          "options": [
            "只能做多不能做空",
            "亏损和收益都会被放大",
            "交易速度变慢",
            "无法使用止损单"
          ],
          "question_id": "q_flash_leverage_margin_v2",
          "stem": "杠杆交易的主要风险是什么？"
        }
      ]
    },
    {
      "concept_key": "order_book",
      "coverage_tags": [
        "trading_mechanism_order_matching"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_order_book",
      "learning_goal": "学生能识别订单簿的基本构成和功能。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "订单簿",
          "en": "Order Book"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 8,
          "explanation": "订单簿是实时列出所有买入和卖出挂单的电子列表，按价格排序。",
          "options": [
            "记录历史交易价格",
            "实时列出所有买入和卖出挂单",
            "计算投资组合的收益",
            "自动执行交易策略"
          ],
          "question_id": "q_flash_order_book_v1",
          "stem": "订单簿（Order Book）的主要功能是什么？"
        },
        {
          "answer": 1,
          "estimated_seconds": 8,
          "explanation": "订单簿中，Bid Book（买单簿）记录所有买入订单。",
          "options": [
            "所有卖单",
            "所有买单",
            "已成交的订单",
            "市场新闻"
          ],
          "question_id": "q_flash_order_book_v2",
          "stem": "在订单簿中，Bid Book记录的是什么？"
        }
      ]
    }
  ],
  "lesson_id": "L1",
  "longform_families": [
    {
      "concept_key": "trading_approaches",
      "coverage_tags": [
        "algo_trading_basics",
        "trading_approaches_comparison"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_algo_compare",
      "learning_goal": "学生能对比人类交易、机器人投顾和机器人交易三种方式在关键维度上的差异。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "compare_and_contrast",
      "term_refs": [
        {
          "display": "人类交易",
          "en": "Human Trading"
        },
        {
          "display": "机器人投顾",
          "en": "Robo-Advisory"
        },
        {
          "display": "机器人交易",
          "en": "Robo-Trading"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "执行速度",
            "可扩展性",
            "纪律性"
          ],
          "question_id": "q_long_algo_compare_v1",
          "reference_answer": [
            "执行速度：人类交易慢，机器人投顾中等，机器人交易立即执行。",
            "可扩展性：人类交易因压力限制投资规模，可扩展性有限；机器人投顾中等；机器人交易可扩展性高。",
            "纪律性：人类交易不稳定，尤其亏损时；机器人投顾一致；机器人交易高。"
          ],
          "rubric_points": [
            "正确指出人类交易执行速度慢、可扩展性有限、纪律性不稳定",
            "正确指出机器人投顾执行速度中等、可扩展性中等、纪律性一致",
            "正确指出机器人交易执行速度快、可扩展性高、纪律性高"
          ],
          "stem": "请从执行速度、可扩展性和纪律性三个维度，比较人类交易、机器人投顾和机器人交易这三种交易方式。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "工作时间",
            "用户控制"
          ],
          "question_id": "q_long_algo_compare_v2",
          "reference_answer": [
            "工作时间：人类交易和机器人投顾都约为9*5，机器人交易是24*7全天候运行。",
            "用户控制：人类交易拥有完全控制权，机器人投顾提供部分控制，机器人交易的用户控制最小。"
          ],
          "rubric_points": [
            "正确指出人类交易工作时间约为9*5，用户控制为完全控制",
            "正确指出机器人投顾工作时间约为9*5，用户控制为部分控制",
            "正确指出机器人交易工作时间是24*7，用户控制为最小控制"
          ],
          "stem": "请从工作时间和用户控制两个维度，比较人类交易、机器人投顾和机器人交易这三种交易方式。"
        }
      ]
    },
    {
      "concept_key": "order_matching_equilibrium",
      "coverage_tags": [
        "trading_mechanism_order_matching"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_order_matching",
      "learning_goal": "学生能解释订单匹配的基本机制和市场均衡价格的形成过程。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "mechanism_trace",
      "term_refs": [
        {
          "display": "市场均衡",
          "en": "Market Equilibrium"
        },
        {
          "display": "订单匹配",
          "en": "Order Matching"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "订单簿的构成",
            "交易发生的条件",
            "市场均衡价格的形成"
          ],
          "question_id": "q_long_order_matching_v1",
          "reference_answer": [
            "订单簿实时列出所有买单和卖单，买单按价格从高到低排列，卖单按价格从低到高排列。",
            "当买单中的最高出价（Bid Price）等于或高于卖单中的最低要价（Ask Price）时，交易就会发生。",
            "这个买卖双方都能接受的价格点就是市场均衡价格，此时供给和需求达到平衡。"
          ],
          "rubric_points": [
            "正确描述订单簿由买单（Bid）和卖单（Ask）组成",
            "正确指出当买方的最高出价与卖方的最低要价相遇时交易发生",
            "正确指出这个相遇点就是市场均衡价格"
          ],
          "stem": "请描述在订单簿中，一笔交易是如何发生的？市场均衡价格是如何形成的？"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "当前无交易的原因",
            "市价买单的执行过程"
          ],
          "question_id": "q_long_order_matching_v2",
          "reference_answer": [
            "当前没有交易发生，因为买方愿意支付的最高价（$9.95）低于卖方愿意接受的最低价（$10.05），价格没有交集。",
            "如果一位新买家下了一个市价买单，该订单会立即以当前最优的卖出价$10.05成交，因为市价单追求的是立即成交。"
          ],
          "rubric_points": [
            "正确指出因为最高买入价低于最低卖出价，价格不匹配",
            "正确指出市价买单会以当前最优价即$10.05成交"
          ],
          "stem": "假设某股票订单簿中，最高买入价为$9.95，最低卖出价为$10.05。请解释为什么此时没有交易发生？如果一位新买家以$10.05的价格下了一个市价买单，会发生什么？"
        }
      ]
    },
    {
      "concept_key": "leverage_margin_calculation",
      "coverage_tags": [
        "market_jargon_terminology"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_leverage_calc",
      "learning_goal": "学生能应用杠杆比率公式进行计算，并解释杠杆对收益和风险的影响。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "formula_apply",
      "term_refs": [
        {
          "display": "杠杆比率",
          "en": "Leverage Ratio"
        },
        {
          "display": "保证金要求",
          "en": "Margin Requirement"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "杠杆比率计算",
            "购买力计算",
            "盈亏计算"
          ],
          "question_id": "q_long_leverage_calc_v1",
          "reference_answer": [
            "(a) 杠杆比率 = 1 / 0.25 = 4倍。",
            "(b) 购买力 = 杠杆比率 × 本金 = 4 × $10,000 = $40,000。",
            "(c) 资产价格上涨10%：收益 = $40,000 × 10% = $4,000，收益率 = $4,000 / $10,000 = 40%。价格下跌10%：亏损 = $40,000 × 10% = $4,000，亏损率 = 40%。"
          ],
          "rubric_points": [
            "正确应用公式 Leverage Ratio = 1 / Margin Requirement 计算出4倍杠杆",
            "正确计算购买力为$40,000",
            "正确计算收益率为40%，亏损率也为40%"
          ],
          "stem": "假设某交易平台的保证金要求为25%。\n(a) 请计算杠杆比率。\n(b) 如果你投入了$10,000本金，使用最大杠杆可以操作多少价值的资产？\n(c) 如果资产价格上涨了10%，你的收益率是多少？如果价格下跌了10%，你的亏损率是多少？"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "保证金计算",
            "亏损计算",
            "风险解释"
          ],
          "question_id": "q_long_leverage_calc_v2",
          "reference_answer": [
            "(a) 保证金 = 资产价值 / 杠杆比率 = $50,000 / 10 = $5,000。",
            "(b) 亏损金额 = $50,000 × 5% = $2,500。亏损率 = $2,500 / $5,000 = 50%。",
            "(c) 杠杆交易放大了亏损，因为投资者只投入了少量本金（$5,000）却承担了全部资产价值（$50,000）的波动风险。资产价格的小幅下跌就会导致本金的巨大亏损。"
          ],
          "rubric_points": [
            "正确计算保证金为$5,000",
            "正确计算亏损金额为$2,500，亏损率为50%",
            "正确解释杠杆放大了亏损比例"
          ],
          "stem": "某投资者使用10倍杠杆买入价值$50,000的股票。\n(a) 该投资者投入的本金（保证金）是多少？\n(b) 如果股票价格下跌5%，该投资者的亏损金额和亏损率分别是多少？\n(c) 解释为什么杠杆交易的风险更高。"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L1/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "input source outline"
  },
  "target_language": "zh-CN"
}

</QUESTION_BANK>

<PLAIN_TEXT>
# L1: Algorithmic trading basics and financial markets

Course Code: COMP7415

# Agenda

- The basic concept of algo-trading   
- The market trend of algo-trading   
- Comparison of different trading approaches   
• Infrastructure needed to deploy algo-trading programs   
- Trading mechanism and order matching   
Market jargon and terminology

# What is algo-trading?

- Algorithmic trading, also called Algo Trading, Quant Trading, Robo-Trading, Program Trading   
- Use of mathematical models and computer algorithms/programs to

- generate trading signals (i.e. Decision making)   
- automate trading process (i.e. Execution)

- Objectives:

- maximize profits   
- control execution costs   
- hedging and manage investment risks

# Market Trend for Algo-Trading

Expected to grow at CAGR $12.7\%$ to US $\$ 32$ b by 2028   
- Largest market in North America   
- Fastest growing in Asia Pacific

![](images/23944073ff63d8284dc99146e03872d7534460656b988edca49867c2a1823382.jpg)

![](images/553c1850eb62275b06eb4b2fa2de02aa49eb0882c33893ea01dbf6746a04d526.jpg)

# Market Trend for Algo-Trading

Algorithmic trading strategies account for approximately:

60% of all US equity volume   
40% of all European equity volume   
25% of all Forex transactions   
20% of all US option trades

# Robo-Trading vs Robo-Advisory

<table><tr><td></td><td>Robo-Trading</td><td>Robo-Advisory</td></tr><tr><td>Decision making</td><td>✓</td><td>✓</td></tr><tr><td>Execution</td><td>✓</td><td>×</td></tr><tr><td>Automation</td><td>Fully automated</td><td>Semi-automated</td></tr><tr><td>Outputs</td><td>Execution of trading signals</td><td>• Stock advices
• Market summary
• Risk alerts</td></tr></table>

![](images/2853c1739330c29f9f1c777cb0195998cf6f596c913e4ccd557e9da5aa1bb6d5.jpg)

ALGOGENE

FinTech Co. Ltd.

JOIN US ON TELEGRAM

ALGOGENESIGNAL

FREE DAILY REAL-TIME TRADING SIGNALS

![](images/de6188289d2654bafc1a1dc3b641da604e09b47ea7d604dbc7620e6c670f49e3.jpg)  
@algogene_signal

![](images/fd8bf798e299893a5e056c49cb7b184a1a3217af05382e16f6f2651b4fb021eb.jpg)  
Warning: Beware of scam Telegram groups. Please note that the only official group is @algogene_signal

# Comparison of Trading Approaches

manual

automatic

<table><tr><td></td><td>Human Trading</td><td>Robo-Advisory</td><td>Robo-Trading</td></tr><tr><td>Working Hours</td><td>~9*5</td><td>~9*5</td><td>24*7</td></tr><tr><td>Execution Speed</td><td>Slow</td><td>Moderate</td><td>Immediately</td></tr><tr><td>Data Inputs</td><td>Limited</td><td>Almost unlimited</td><td>Almost unlimited</td></tr><tr><td>Trading Frequency</td><td>Low</td><td>Low - mid</td><td>Low – High</td></tr><tr><td>Scalability</td><td>Limited investment size due to stress</td><td>Moderate</td><td>High</td></tr><tr><td>Accuracy/ Discipline</td><td>Variable especially when you lose</td><td>Consistent</td><td>High</td></tr><tr><td>Customization</td><td>High</td><td>Medium</td><td>Low</td></tr><tr><td>User Control</td><td>Full</td><td>Partial</td><td>Minimal</td></tr><tr><td>Risk Management</td><td>Subjective</td><td>Algorithmsic</td><td>Algorithmsic</td></tr></table>

# What should you consider to implement a trading algo?

# Algo is the vehicle; Data is the Fuel!

- Data collection - time and effort demanding in data sourcing and data cleaning   
- Data storage - high volume of unstructured data (Numeric, Text, Image, Audio, Video)   
- Data management - difficult to protect and manage through traditional databases due to inconsistent file and data formats   
- Data processing - feasibility and efficiency

# Other factors for infrastructure setup

Real-time datafeed   
Historical data   
- Computing hardware   
- Exchange/broker connectivity   
Network and latencies   
- Order routing system   
- Order management system   
- Algo maintenance   
- Cybersecurity   
•

# General Trading Overview

# Types of Trading

# Fundamental:

Stock Selection   
Ratio Analysis   
Sector Analysis   
- Corporate and Executive Management

# Technical:

Chart pattern   
Trend Analysis

# Quantitative:

- Rule based   
Econometric Forecasting   
- Statistical Arbitrage   
- NLP, sentiment analysis   
Machine learning

# Types of Trading

# Time Frames:

- Long Term: Months to Years   
- Short Term: Days, Weeks, Months   
- Intraday: Seconds to Hours   
High frequency: Fractions of Seconds

# How trading works?

# Supply

# Current Stock holders

(each person holds 1 share)

![](images/e1e057b7d7fc7bd49d1922c8618de93b1532198a870066b4b588c4a6e747b863.jpg)

\$10

![](images/66e3829fdc2f9a0f1edca0c5ace158988518b86d015f6fc75ee5cefefccb670e.jpg)

\$12

![](images/d921225e2a9e350d3724bb19428a5369a223a20629e8267e8315f29e57225ff8.jpg)

\$13

![](images/a7cd152fad3ea773dfc34758a660b3b44bde991f91098ff56886415994b2c5b0.jpg)

\$15

![](images/2c15a222210bda5e996f2d424cdd93045f1743e4fc8d2e373a46b1d91ec36d2f.jpg)

\$15

![](images/07b0516b07de265bd3af8149f935a07966e05b6833e24644b9ba5380bb31193d.jpg)

§18

![](images/ea229043a908c9e6318d587100f327366a95cbfd88d28e42875c259efb8e7488.jpg)

\$21

![](images/b7433937afe87c2c5138a2b99115896ca2d0be9178a7865367cd1018684896b5.jpg)

\$24

![](images/c662c065d1d2f712b08d36d87935a02918fcc93834c64e0e658d884bb5a52cea.jpg)

\$25

![](images/675692737d29622eb03acbdd2b2a9b309f08b211b547e8d38cd23dc6a6c7c1df.jpg)

\$30

Price they would be willing to take (sell) for their shares

# Current Stock holders

(each person holds 1 share)

![](images/52ec0b281f70a1907f11cd9bcdbe75ca3025f7a016e47f91919ca77cb9916500.jpg)  
\$10

![](images/ec193921ae31a6a1b9329a744b1f29dd670608eb6067868e0a39832cf90f7fb5.jpg)  
\$12

![](images/8e3bef9a43fbaa662ef8c08f251003ed266561e796f09187c02db0e9c203e841.jpg)  
\$13

![](images/06f7364de3af917c64e52c55f661a3cfadb3e337c7d459b97f3adfd4561d292a.jpg)  
§15

![](images/c4e433c1285445be0abd10d4c5d990e18a91d14a48e16132abde1da0b7995355.jpg)  
\$15

![](images/b159aa9cfb0666b2024460c1f36e18619269c8906996bb51020078ad49d3f411.jpg)  
\$18

![](images/7d9ab41481da4def300804b68f53a9667de9caa3a135f6b4e76d99ae94b44919.jpg)  
§21

![](images/50b5782f452ccf7179c6361e3c335625a79aef32126ff58efd75da45c7863c35.jpg)  
\$24

![](images/17d0f1edeb826aea3aba36a5da72f534551146c03935ce571aaecb1f7338bf1d.jpg)  
\$25

![](images/7260e8fa9b83862c1abdc122adc83b69727c390d3221246163add8669ed38725.jpg)  
\$30

![](images/bf1c4e7961e05025587eeee299249a966b057686a903f3cf5b4099aaa6044ca1.jpg)  
Supply

<table><tr><td>Price</td><td>Supply</td></tr><tr><td>1</td><td>0</td></tr><tr><td>2</td><td>0</td></tr><tr><td>3</td><td>0</td></tr><tr><td>4</td><td>0</td></tr><tr><td>5</td><td>0</td></tr><tr><td>6</td><td>0</td></tr><tr><td>7</td><td>0</td></tr><tr><td>8</td><td>0</td></tr><tr><td>9</td><td>0</td></tr><tr><td>10</td><td>1</td></tr><tr><td>11</td><td>1</td></tr><tr><td>12</td><td>2</td></tr><tr><td>13</td><td>3</td></tr><tr><td>14</td><td>3</td></tr><tr><td>15</td><td>5</td></tr><tr><td>16</td><td>5</td></tr><tr><td>17</td><td>5</td></tr><tr><td>18</td><td>6</td></tr><tr><td>19</td><td>6</td></tr><tr><td>20</td><td>6</td></tr><tr><td>21</td><td>7</td></tr><tr><td>22</td><td>7</td></tr><tr><td>23</td><td>7</td></tr><tr><td>24</td><td>8</td></tr><tr><td>25</td><td>9</td></tr><tr><td>26</td><td>9</td></tr><tr><td>27</td><td>9</td></tr><tr><td>28</td><td>9</td></tr><tr><td>29</td><td>9</td></tr><tr><td>30</td><td>10</td></tr></table>

# Demand

# Potential Buyers

(each person wants to buy 1 share)

![](images/86360fdbfc0686ed5bb93811e5120194b63bed941a41f8ac4a700fd590f386d1.jpg)  
\$5

![](images/ff7d024e6fdbd516099aac633e061ba672cdb00e0cd864ed2a471baed275ac24.jpg)  
\$6

![](images/8f6548cbaa3e1b713e127969356e4339b881c4ee8275b8ebd5df0f79b84d7d54.jpg)  
\$8

![](images/580ed35fd6987c7196b168c9cd67feef87601c8798035b2fff33f45015c54af9.jpg)  
\$10

![](images/19c846959eada8c2a10d9b73544e81da601822ef1beaf3f4e93cfa8c3442ba1b.jpg)  
\$11

![](images/d4b978d4f62c3ceea2332c78ae9860aa33aba5e20368417b05810fb4adb5151c.jpg)  
\$12

![](images/d71b1972d6855ba25b49c3b78bb4e1e22d291978b507f04ffa2aa07400e63ef3.jpg)  
\$14

![](images/9af52572b328901eb34ae9e7dae44f14ea5f3adf7dc096877753435b7beea51a.jpg)  
\$15

Price they would be willing to pay for a share

# Potential Buyers

(each person wants to buy 1 share)

![](images/f9845d43e9a67e8bec11c308e4298e306417b8a1b810c67728463ac833f964c2.jpg)  
\$5

![](images/ea06648c52b2b8248ac449299981beabbdc442eaaf32e3f3c8b35f518283f3b3.jpg)  
\$6

![](images/1d3da68bc93fbd3118d914887ba9d65239dafab534b282ec9a303731544ba453.jpg)  
\$8

![](images/a34cb16fa73a9c47eee81913a1885913af2c50f5ad68f9d31880b4a5007bb092.jpg)  
\$10

![](images/53e2929550688514b55498c072399e99ea74d36063d4f814f662f5d7acc989e9.jpg)  
\$11

![](images/53367305a6e49e7db8201b8a03c85905647a0a1a65296357168ba45f6a1c20d0.jpg)  
\$12

![](images/97430e1e313c1f795221b38b9e4afcf8ecba246b88da7c62fe2966284a65f550.jpg)  
\$14

![](images/0fd2f01994766eed041dfda9e2cf6227c79bcf109d7c5885a8fd005b39f9fa4a.jpg)  
\$15

![](images/798449f0918ee7b311b24223b49c1897c83f94b9c422e3a97ca3de9563e95aec.jpg)  
Demand

<table><tr><td>Price</td><td>Demand</td></tr><tr><td>1</td><td>8</td></tr><tr><td>2</td><td>8</td></tr><tr><td>3</td><td>8</td></tr><tr><td>4</td><td>8</td></tr><tr><td>5</td><td>8</td></tr><tr><td>6</td><td>7</td></tr><tr><td>7</td><td>6</td></tr><tr><td>8</td><td>6</td></tr><tr><td>9</td><td>5</td></tr><tr><td>10</td><td>5</td></tr><tr><td>11</td><td>4</td></tr><tr><td>12</td><td>3</td></tr><tr><td>13</td><td>2</td></tr><tr><td>14</td><td>2</td></tr><tr><td>15</td><td>1</td></tr><tr><td>16</td><td>0</td></tr><tr><td>17</td><td>0</td></tr><tr><td>18</td><td>0</td></tr><tr><td>19</td><td>0</td></tr><tr><td>20</td><td>0</td></tr><tr><td>21</td><td>0</td></tr><tr><td>22</td><td>0</td></tr><tr><td>23</td><td>0</td></tr><tr><td>24</td><td>0</td></tr><tr><td>25</td><td>0</td></tr><tr><td>26</td><td>0</td></tr><tr><td>27</td><td>0</td></tr><tr><td>28</td><td>0</td></tr><tr><td>29</td><td>0</td></tr><tr><td>30</td><td>0</td></tr></table>

# Market Equilibrium

![](images/3627bc4390ce1dee9b9bd69fe4614da45984046abf62f9f1d460331c494cda71.jpg)  
- The market is balanced at \(12 and quantity 2

<table><tr><td>Price</td><td>Supply</td><td>Demand</td></tr><tr><td>1</td><td>0</td><td>8</td></tr><tr><td>2</td><td>0</td><td>8</td></tr><tr><td>3</td><td>0</td><td>8</td></tr><tr><td>4</td><td>0</td><td>8</td></tr><tr><td>5</td><td>0</td><td>8</td></tr><tr><td>6</td><td>0</td><td>7</td></tr><tr><td>7</td><td>0</td><td>6</td></tr><tr><td>8</td><td>0</td><td>6</td></tr><tr><td>9</td><td>0</td><td>5</td></tr><tr><td>10</td><td>1</td><td>5</td></tr><tr><td>11</td><td>1</td><td>4</td></tr><tr><td>12</td><td>2</td><td>3</td></tr><tr><td>13</td><td>3</td><td>2</td></tr><tr><td>14</td><td>3</td><td>2</td></tr><tr><td>15</td><td>5</td><td>1</td></tr><tr><td>16</td><td>5</td><td>0</td></tr><tr><td>17</td><td>5</td><td>0</td></tr><tr><td>18</td><td>6</td><td>0</td></tr><tr><td>19</td><td>6</td><td>0</td></tr><tr><td>20</td><td>6</td><td>0</td></tr><tr><td>21</td><td>7</td><td>0</td></tr><tr><td>22</td><td>7</td><td>0</td></tr><tr><td>23</td><td>7</td><td>0</td></tr><tr><td>24</td><td>8</td><td>0</td></tr><tr><td>25</td><td>9</td><td>0</td></tr><tr><td>26</td><td>9</td><td>0</td></tr><tr><td>27</td><td>9</td><td>0</td></tr><tr><td>28</td><td>9</td><td>0</td></tr><tr><td>29</td><td>9</td><td>0</td></tr><tr><td>30</td><td>10</td><td>0</td></tr></table>

# Order Matching

Current Stock holders

(each person holds 1 share)

![](images/76aa0950e4bed1acbae25a888f81f80f26e02ad4816ac50a4e5f2cda985a5a5c.jpg)  
\$10

![](images/1f357fd8c08721af06955e9d33adc38fd16d170af4534feb93ce922e9a425408.jpg)  
\$12

![](images/aea51de4de548ab864441c2f29f0929981552e505b19d8ca6f4387c9f610acc1.jpg)  
§13

![](images/49348a9f5482cff7509c96a4fae1a772326c9ab3afb5cb7a34308253ac4cc658.jpg)  
§15

![](images/851355a500a0f68a90ef3b5fad5900aca59944d9ea7b0951a7792cf1b569dda7.jpg)  
§15

![](images/3445f9a5d095f145645fe50b2af3810421817e28764d4c9f7e7a54fe02081038.jpg)  
\$18

![](images/edcd72bb496047cbb6319d697cbba23add728497dbe45be16de454c989d1b0a4.jpg)  
\$21

![](images/9c68fd8438dd884e2c4ab5d05ac10dd293f58d9b48c4eb96653921cf7114779b.jpg)  
\$24

![](images/c7b40ea70ccfd7c190613148a573cf062a3deae264c2d94bad90f3cd4c9a5344.jpg)  
\$25

![](images/6b6e4930be7a80fb5a92ec4019c83a4c51fe0310ef43f1074b8d16f04cbdff16.jpg)  
\$30

![](images/cbe70f1303278c4c79c22ed1f4ac31d586403e249a224b69dc39cf5770967f5f.jpg)  
\$5

![](images/8fd2a487e6a8152df82085f0e3bdae08bc244fdcee14e5d2a5f550a64b6580e4.jpg)  
\$6

![](images/164a700d165ac8c5b3f94eaea21ab59192e6f33d7ea0fa8f7392d76677e0f7b2.jpg)  
\$8

![](images/b07aaf00beb0d6cfc09d252c8f5a194755afe07e36f4939a5978924f6ea79fcc.jpg)  
\$10

![](images/025b679b5a14eadbc8880a50ca03f9ec5fe7460893eb7117942104fd9f7a8bba.jpg)  
\$11

![](images/2e019b398a59c07427da55d6ecd1cb7ea0de78f944c07bb2ea88de882b9cb3a0.jpg)  
\$12

![](images/e9ab0a6e59e18c44ec1f59ebfae68ba97e95c620319bd66523ea6b5bbad78f3f.jpg)  
\$14

![](images/8eb4771a0565203e595ebee6ff63810e8632a21620fcbdf627b1737938c749c7.jpg)  
\$15   
Potential Buyers   
(each person wants to buy 1 share)

# After Order Matching

![](images/2340feec59164504691d63240722a35cea786eabff0e8034687ee80be2a41dd4.jpg)

# Market Terminology

# Common market terminology

1. Long and short position   
2. Short-selling   
3. Order book   
4. Bid price, ask price   
5. Market spread   
6. Order types   
7. Slippage   
8. Mark-to-market   
9. Trading system   
10. Margin & leverage

# Understanding Long and Short Positions

- Long Position

• Definition: Buying an asset with the expectation that its price will rise.   
- Goal: Sell at a higher price to realize a profit.   
• Example: Buying 100 shares of a stock at $50, selling when it reaches$ 70.

![](images/e4a7b63262ab10be6402b5b91c04159ccb3f664d558c127eadc3d9b8df361959.jpg)  
Long

# Understanding Long and Short Positions

# - Short Position

- Definition: Selling an asset that you do not own, with the intention of buying it back later at a lower price.   
- Goal: Profit from a decline in the asset's price.   
• Example: Selling 100 shares of a stock at $70, buying back at$ 50.

![](images/46c2fd97f0af2633c54158e3594122c54c05f9f8c58c405f4d569dce6c33c804.jpg)

# How Short-Selling Works

1. Borrowing Shares: Obtain shares from a broker to sell.   
2. Selling the Borrowed Shares: Sell the shares at the current market price.   
3. Buying Back Shares: Later, buy the same number of shares at a lower price.   
4. Returning Shares: Return the borrowed shares to the broker.

![](images/f91be6e8633dfd9caaf37fd019f26e0d75c126588d6adc96dc5b9c3e030fc177.jpg)

# Risks of Short-Selling

- Unlimited Loss Potential: If the asset price rises significantly.   
- Margin Calls: Brokers may require additional funds if the trade goes against you.   
- Regulatory Risks: Short-selling can be subject to restrictions.

![](images/f0e8373ee69422c3e7de92d9b404515e2bbc07233ecf3a9f25b69838993184ec.jpg)

# Order Book

- The Order Book is a real-time, continually updating list of buy and sell orders for a specific financial instrument, sorted by price level.   
Key components

- Ask Book: the list/queue for sellers   
- Bid Book: the list/queue for buyers   
- Price: The price points at which investors are willing to trade   
Size: The number of shares, contracts, or lots that traders want to buy or sell

# How does Order Book present?

1. Order Ledger   
2. Top of the Book   
3. Cumulative Book

# Order Ledger

- The order book is listed in a sorted price ledger, which can be ascending or descending.   
- The longer the price ledger is, the more liquidity an instrument provides. It is also called "market depth".

![](images/1c4327d010822d6548850f694f7c27fde9fc3a6d0c9c3008732a3d8ee04a4aad.jpg)

# Top of the Book

- The order book is separated into bid-book and ask-book   
- Bid orders are sorted in descending price   
- Ask orders are sorted in ascending price   
- The highest bid and the lowest ask are referred to "the top of the book"

Order Book

<table><tr><td></td><td>Bid</td><td>Ask</td><td></td></tr><tr><td>3,151</td><td>9139.60</td><td>9152.80</td><td>167</td></tr><tr><td>1,574</td><td>9138.40</td><td>9153.21</td><td>1,600</td></tr><tr><td>338</td><td>9138.20</td><td>9153.60</td><td>736</td></tr><tr><td>789</td><td>9138.10</td><td>9153.71</td><td>1,602</td></tr><tr><td>800</td><td>9138.00</td><td>9154.10</td><td>1,649</td></tr><tr><td>309</td><td>9137.79</td><td>9154.21</td><td>1,965</td></tr><tr><td>2,140</td><td>9137.60</td><td>9154.50</td><td>3,984</td></tr><tr><td>48</td><td>9137.40</td><td>9154.60</td><td>763</td></tr><tr><td>144</td><td>9137.29</td><td>9154.71</td><td>157</td></tr><tr><td>136</td><td>9137.20</td><td>9154.80</td><td>56</td></tr><tr><td>528</td><td>9137.00</td><td>9155.50</td><td>320</td></tr><tr><td>1,654</td><td>9136.90</td><td>9155.71</td><td>200</td></tr><tr><td>40</td><td>9136.79</td><td>9156.10</td><td>16</td></tr><tr><td>46</td><td>9136.70</td><td>9156.21</td><td>219</td></tr><tr><td>608</td><td>9136.60</td><td>9156.30</td><td>40</td></tr><tr><td>48</td><td>9136.50</td><td>9156.40</td><td>80</td></tr></table>

# Cumulative Order Book

• For each of the bid and ask book, order size accumulates from the top of the book.

![](images/141e8b6f16ee4724150911c20de32c9481e617b6a0ffd9f405342bd4c77f037d.jpg)

<table><tr><td colspan="3">Order Book Market Trades</td></tr><tr><td>= = =</td><td rowspan="2">Qty(BTC)</td><td>0.1▼</td></tr><tr><td>Price(USDT)</td><td>Total(BTC)</td></tr><tr><td>61,284.8</td><td>11.972</td><td>54.521</td></tr><tr><td>61,284.7</td><td>7.282</td><td>42.549</td></tr><tr><td>61,284.0</td><td>2.529</td><td>35.267</td></tr><tr><td>61,283.9</td><td>1.305</td><td>32.738</td></tr><tr><td>61,283.8</td><td>1.795</td><td>31.433</td></tr><tr><td>61,283.7</td><td>4.569</td><td>29.638</td></tr><tr><td>61,283.5</td><td>5.773</td><td>25.069</td></tr><tr><td>61,283.4</td><td>1.795</td><td>19.296</td></tr><tr><td>61,283.3</td><td>3.590</td><td>17.501</td></tr><tr><td>61,283.2</td><td>13.911</td><td>13.911</td></tr><tr><td colspan="3">61,284.4 ↑ 61,291.9</td></tr><tr><td>61,282.3</td><td>12.075</td><td>12.075</td></tr><tr><td>61,282.2</td><td>0.816</td><td>12.891</td></tr><tr><td>61,282.1</td><td>2.693</td><td>15.584</td></tr><tr><td>61,282.0</td><td>4.712</td><td>20.296</td></tr><tr><td>61,281.9</td><td>9.444</td><td>29.740</td></tr><tr><td>61,281.8</td><td>1.795</td><td>31.535</td></tr><tr><td>61,281.7</td><td>3.672</td><td>35.207</td></tr><tr><td>61,281.6</td><td>5.181</td><td>40.388</td></tr><tr><td>61,278.4</td><td>15.301</td><td>55.689</td></tr><tr><td>61,267.2</td><td>15.306</td><td>70.995</td></tr></table>

My Portfolio

News

Markets

Sectors

Screeners

Personal Finance

Videos

# Summary

News

Chart

Conversations

Statistics

Historical Data

Profile

Financials

Analysis

Options

Holders

Sustainability

NasdaqGS - Nasdaq Real Time Price • USD

# Apple Inc. (AAPL)

Follow

$\rightarrow^{\dagger}$ Compare

# 207.49 -2.19 (-1.04%)

# 207.86 +0.37 (+0.18%)

At close: June 21 at 4:00 PM EDT

Pre-Market: 7:41 AM EDT

# Start Trading >>

Plus500 CFD Service. Your capital is at risk.

![](images/a92594ce88f7057bc67eae0caa630c03cf3bef9d391f01b0f85c9cfac59212dc.jpg)

<table><tr><td>Previous Close</td><td>209.68</td><td>Day&#x27;s Range</td><td>207.11 - 211.89</td><td>Market Cap (intraday)</td><td>3.182T</td><td colspan="2">Earnings Date Aug 1, 2024 - Aug 5, 2024</td></tr><tr><td>Open</td><td>210.39</td><td>52 Week Range</td><td>164.08 - 220.20</td><td>Beta (5Y Monthly)</td><td>1.25</td><td>Forward Dividend &amp; Yield</td><td>1.00 (0.48%)</td></tr><tr><td>Bid</td><td>207.01 x 3000</td><td>Volume</td><td>246,421,353</td><td>PE Ratio (TTM)</td><td>32.22</td><td>Ex-Dividend Date</td><td>May 10, 2024</td></tr><tr><td>Ask</td><td>209.94 x 200</td><td>Avg. Volume</td><td>67,794,775</td><td>EPS (TTM)</td><td>6.44</td><td>1y Target Est</td><td>207.58</td></tr></table>

# Bid Price & Bid Size

- Bid price is the highest price that a buyer (bidder) is willing to pay for a particular security.   
- Bid size represents the quantity a buyer is willing to purchase at the bid price.   
- The top price and size in a bid order book.   
• For example, if a buyer bids $207.01 x 3000 for a stock, it means they are willing to buy 3000 shares for not more than$ 207.01

# Ask Price & Ask Size

- Ask price is the lowest price a seller is willing to accept for a security.   
- Ask size represents the quantity a seller is willing to sell at the ask price.   
- The top price and size in an ask order book.   
• For example, if a seller asks $209.94 \times 200 for a stock, it implies they are willing to sell 200 shares for not less than$ 209.94

# HKEX Market Data Pricing

- Level 1 data:   
• bid/ask price: HK$500 /month   
- Level 2 data:   
- full order book: HK$5,000 /month

# HKEX

香港交易所

Historical Full Book - Securities Market (Binary Format) provides information on every order and trade of Main Board and GEM stocks in binary format. File is available on a daily basis, normally at around 8:30pm.   
Historical Full Book - Securities Market (CSV Format) provides information on every order and trade of Main Board and GEM stocks in CSV format. File is available on a daily basis, normally at around 10:00pm.   
Historical Order Book and Statistics Update - Securities Market provides intra-day order book and statistics information recorded on the HKEX's Main Board and GEM stocks in CSV format. File is available on a daily basis, normally at around 10:00pm.   
Bid and Ask Record - Main Board & GEM (Daily File) provides intra-day bid and ask information on Main Board and GEM stocks   
Bid and Ask Record - Main Board & GEM (Monthly File) provides intra-day bid and ask information on Main Board and GEM stocks   
Trade File - Securities Market (Binary Format) provides trade details, including trade time, price and quantity, of each transaction recorded on the Main Board and GEM in binary format. File is available on a daily basis, normally at around 8:30pm.   
Trade File - Securities Market (CSV) provides trade details, including trade time, price and quantity, of each transaction recorded on the Main Board and GEM in CSV format. File is available on a daily basis, normally at around 10:00pm.

![](images/a7956b2d8383c6a5aabd5022a273670e5bfa747de9a83241a184467998961936.jpg)

![](images/98ea7b18fef22b54f82498e9bb784fd403ed28bf2148ee0d1032cb3dcb7a6685.jpg)

$5,000/ Month

![](images/2ff2039e4730d63ff93056983d7837af278741d2cfba027fdfefaa102473b376.jpg)

$5,000/ Month

![](images/d74d447c92966acb21a5e9e319faf13eaa3bc6f054024cccea30129052bacebe.jpg)

$2,500/ Month

![](images/50018306927a07d1ff6b560ed36879285b09d927b0420483db02b9cff0bd5e2e.jpg)

$500/ Month

![](images/d240eec30502a7601eaf85da93182f883248a16e41530d00b09804f4265aa9d4.jpg)

$500/ Month

![](images/6878284176ca321419f78a4d6a00d0b54c72f835dc419e7490bb2b2d3915d6cc.jpg)

$2,500/ Month

![](images/07461bb1cf3e35252e64819e2ee160d6cd1b2f8ec310b750067381e98efc347d.jpg)

$2,500/ Month

# Market Spread

- The difference between the bid price and the ask price is known as the bid-ask spread.   
- The bid-ask spread is a key measure of the liquidity of an asset or security.   
- A smaller spread indicates a more liquid market, while a larger spread indicates a less liquid market.   
• For example, if the bid price for a stock is $24 and the ask price is $25, the bid-ask spread will be $1.

# Percentage Spread

- Percentage spread is often used for comparing the spread between different instruments

$$
\text{Percentage Spread} = \frac {\text {AskPrice} - \text {BidPrice}}{\text {MidPrice}} \times 100 \% = \frac {2 \times (\text {AskPrice} - \text {BidPrice})}{\text {AskPrice} + \text {BidPrice}} \times 100 \%
$$

• For example, if the bid price for a stock is $24 and the ask price is $25,

• the bid-ask spread will be (25-24) = $1   
- the percentage spread will be $2^{*}(25 - 20) / (25 + 20) = 4.08\%$

# Order Types

1. Market Order   
2. Limit Order   
3. Stop Order

![](images/1251d110ed9aaf8c1d6abc145dad130065bd8502f6bfe2e42e0b37e9673560fe.jpg)

# Market Order

- A market order is to buy or sell at the best available price in the current market.   
- That is, when opening a buy (sell) position using market order, it will try to execute at the best ask (bid) price.   
- A market order typically ensures an execution, but it does not guarantee a specified price. It is appropriate to use market orders when you want an immediate execution.

![](images/8c694ecb9c3bbd40d28b4c509ad59d0300e050c68909faaa8e81981cacd4710d.jpg)

# Limit Order

- A limit order is to buy or sell with a condition on the maximum price to pay or the minimum price to receive (the "limit price").   
- If the order is filled, it will only be at the specified limit price or better. However, there is no guarantee of execution.   
- A limit order may be appropriate when you think you can buy at a price lower than (or sell at a price higher than) the current market quote.

# Limit Order Example

![](images/b8adfb1eb68635e77d4d516ecfe0712fe1c40c9ae25ad17a09f5a1a6aa25f67c.jpg)

# Limit Order Example

# - The last trade price is roughly $138

investor who wants to buy (or sell) immediately would place a market order, which would be executed at or near the current price of \(138 (white line) provided that the market was open.   
investor who wants to buy the stock when it dropped to \(131.78 would place a buy limit order with a limit price of\)131.78 (green line). If the price falls to \(131.78 or lower, the limit order would be triggered and executed at \)131.78 or below. If the stock doesn't drop to \(131.78 or below, no execution would occur.   
investor who wants to sell the stock when it reached \(143.82 would place a sell limit order with a limit price of\)143.82 (red line). If the price rises to \(143.82 or higher, the limit order would be triggered and executed at \)143.82 or above. If the stock doesn't rise to \(143.82 or above, no execution would occur.

# Stop Order

- A stop order is to buy or sell at the market price when it has traded at or through a specified price (the "stop price").   
- If the stock reaches the stop price, the order becomes a market order and is filled at the next available market price. If it doesn't reach the stop price, the order will not be executed.   
- A stop order may be appropriate in these scenarios:

- you want to buy when a stock breaks out above a certain level, and believe that it will continue the trend   
- your holding stock has risen a lot and you want to protect the gain when it begins to fall

# Stop Order Example

![](images/e61f773bf848a8135ad8fef549b38f64214548d74db1fb87fb3099726cd18372.jpg)

# Stop Order Example

- The stop buy order will trigger when the price reaches 143.82 or above, and will execute as a market order at that current price.   
- Thus, if the price rise further after hitting the stop price, it is possible that the order could be executed at a price higher than the stop price.   
- Similarly for the stop sell order, once the stop price of $131.78 is reached, the order could be executed at a lower price

# Slippage

- Slippage refers to the difference between the expected execution price and the price it actually traded   
- It often occurs

- during periods of higher volatility when market orders are used   
- when large orders are executed when market depth is not sufficient to maintain the expected price of trade

# Slippage Example

- Let's assume you have an algorithm set to buy a stock when the price drops to $50. The algorithm detects the price drop and places a buy order.   
However, due to high demand or low liquidity, the order gets executed at $51. This$ 1 difference is the slippage.   
• This means that even if you were expecting to spend $5000 on 100 shares, you would end up spending $5100. The $100 is the cost of slippage

# Profit/Loss Calculation

- Entry: Based on a trade signal, generate order to go short or long a certain financial instrument in a certain quantity. Trade results in a certain position in this security   
- Mark-to-Market: As the price of the security changes, so does your unrealized PnL

$$
F o r l o n g o r d e r, P n L = Q u a n t i t y * (P _ {b i d} - P _ {e n t r y})
$$

$$
F o r s h o r t o r d e r, P n L = - 1 * Q u a n t i t y * (P _ {a s k} - P _ {e n t r y})
$$

- Exit: Generate order to exit the position and create a realized PnL

$$
\mathrm {P n L} = \mathrm {S i d e} * \mathrm {Q u a n t i t y} * \left(P _ {\mathrm {e x i t}} - P _ {\mathrm {e n t r y}}\right)
$$

# Mark-to-Market

- It is an accounting method to calculate the asset or portfolio value according to its current market value, rather than its book value   
- It involves determining the price at which you could immediately sell an asset or close a position   
• Example:

- Suppose you purchase 100 shares of ABC stock   
- The current bid and ask price of ABC are $9.5 and$ 10.5   
• Mark-to-market value of your stock will be $100 *$ 9.5 = $9500

# Order Management System (OMS)

# 1. Order based system

- Transactions are managed in a round order manner   
- Partial close is not supported   
- Trading platforms

MetaTrader, TradingView, MetaStock, etc

# 2. Position based system

- Transactions are independent and no linkage with each other   
- Partial close is allowed   
- Trading Platforms

- Most of the banks (eg. HSBC, SBC, BOC, etc)   
- Interactive Brokers, Binance, etc

# Position Based vs Order Based System

Suppose you open 3 offsetting trades:

#1: buy 2 shares of ABC at $123   
#2: sell 1 share of ABC at $124   
#3: sell 1 share of ABC at $125

For order based system, the unrealized PnL will be \(2^{*}(\mathrm{bid} - 123) + 1^{*}(124 - \mathrm{ask}) +\) \(1^{*}(125 - \mathrm{ask}) = \\(3 + 2^{*}(\mathrm{bid} - \mathrm{ask})\). You need to close all the 3 trades to realize the PnL.

For position based system, the orders will be offsetted based on FIFO, and PL will become realized so long as the position is net to zero. Thus, your PnL will be realized at $3.

# Leverage & Margin

# Leverage & Margin

- Leverage refers to using borrowed capital as a funding source for investment to increase the potential return

$$
\text {L e v e r a g e R a t i o} = \text {A s s e t / E q u i t y} = 1 + \text {D e b t / E q u i t y}
$$

- Margin is a way to create leverage

Margin Amount = (Account Asset Value) - (Borrowed Amount) = Equity held at broker

# Margin Requirement

# There are 2 market quotations for margin requirement

1. Fixed Margin Amount: Mostly used by stock exchanges   
2. Margin as a percentage: Mostly used by FX/Crypto exchanges

Update Date: 20251006

Please be reminded that the minimum margin rates below are for your firm's financially strongest client.

Exchange Participants should set their margin requirements according to each client's individual circumstances.

Index Futures

<table><tr><td rowspan="2">Effective Date</td><td rowspan="2">Product</td><td rowspan="2" colspan="2">HKATS
Code</td><td colspan="2">Client Margin</td><td>Clearing House
Margin</td></tr><tr><td>Initial
(HK$)</td><td>Maintenance
(HK$)</td><td>(HK$)</td></tr><tr><td rowspan="2">20251002</td><td rowspan="2">Hang Seng Index</td><td rowspan="2">HSI</td><td>Full Rate
/lot</td><td>116,774</td><td>93,419</td><td>87,800</td></tr><tr><td>Spread Rate
/spread</td><td>25,735</td><td>20,588</td><td>19,350</td></tr><tr><td rowspan="2">20251002</td><td rowspan="2">Mini-Hang Seng Index</td><td rowspan="2">MHI</td><td>Full Rate
/lot</td><td>23,354</td><td>18,683</td><td>17,560</td></tr><tr><td>Spread Rate
/spread</td><td>5,147</td><td>4,117</td><td>3,870</td></tr></table>

# How Margin is related to Leverage?

• As HSI Index Future has a price magnifier of 50, meaning that 1 index point change will lead to ±HK$50. Suppose current HSI Index Future price is 25000, and buying 1 lot of HSI Index Future require HK$116,774 as margin. Then, leverage ratio is calculated to be ($25000*50)/$116,774 = 10.7044   
• Suppose you pay $600,000 to get stocks worth $1,000,000. Thus, leverage ratio is calculated to be $1,000,000/$600,000 = 1.6667   
- As you can imagine, the lower is the margin requirement, the higher is the leverage ratio. In general,

$$
L e v e r a g e \mathrm {R a t i o} = \frac {1}{\text {M a r g i n R e q u i r e m e n t}}
$$

# Buying Power

- For leveraged trading where traders can take out a loan based on the amount of cash held in their broker account, buying power refers to the amount of money available for investors to purchase securities. Mathematically,

$$
B u y i n g P o w e r = (L e v e r a g e R a t i o) \times (I n v e s t o r ^ {\prime} s e q u i t y)
$$

- For example, suppose an investor made an initial deposit US $100,000 to a 20:1 leveraged broker account. Then, the investor would be able to purchase, at maximum, US$ 2,000,000 worth of securities. Hence, a buying power of US$2,000,000

# Key Takeaways

- Learn the basic concept of algo-trading and its market trend   
- Understand the pros and cons of different trading approaches   
- Know about the infrastructure/resources needed to develop and deploy algo-trading programs in real market   
- Understand the trading mechanism and order matching in financial market   
- Equip the knowledge about different market terminology (eg. Order book, order types, market spread, slippage, order/position based trading system, margin, etc)
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
