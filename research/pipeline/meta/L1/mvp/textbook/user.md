请基于以下材料，生成一份 lesson 级 MDX 课本。

目标语言：
zh-CN

lesson_id：
L1

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
      "concept": "什么是算法交易",
      "file": "research/pipeline/3-guided_story/L1/step1/step.json",
      "sequence_id": "step1"
    },
    {
      "concept": "算法交易的市场趋势与对比",
      "file": "research/pipeline/3-guided_story/L1/step2/step.json",
      "sequence_id": "step2"
    },
    {
      "concept": "交易机制与订单簿",
      "file": "research/pipeline/3-guided_story/L1/step3/step.json",
      "sequence_id": "step3"
    },
    {
      "concept": "做多、做空与盈亏计算",
      "file": "research/pipeline/3-guided_story/L1/step4/step.json",
      "sequence_id": "step4"
    },
    {
      "concept": "杠杆、保证金与交易系统",
      "file": "research/pipeline/3-guided_story/L1/step5/step.json",
      "sequence_id": "step5"
    },
    {
      "concept": "实施算法交易的基础设施",
      "file": "research/pipeline/3-guided_story/L1/step6/step.json",
      "sequence_id": "step6"
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
      "concept": "什么是算法交易",
      "file": "research/pipeline/3-guided_story/L1/step1/step.json",
      "sequence_id": "step1"
    },
    {
      "concept": "算法交易的市场趋势与对比",
      "file": "research/pipeline/3-guided_story/L1/step2/step.json",
      "sequence_id": "step2"
    },
    {
      "concept": "交易机制与订单簿",
      "file": "research/pipeline/3-guided_story/L1/step3/step.json",
      "sequence_id": "step3"
    },
    {
      "concept": "做多、做空与盈亏计算",
      "file": "research/pipeline/3-guided_story/L1/step4/step.json",
      "sequence_id": "step4"
    },
    {
      "concept": "杠杆、保证金与交易系统",
      "file": "research/pipeline/3-guided_story/L1/step5/step.json",
      "sequence_id": "step5"
    },
    {
      "concept": "实施算法交易的基础设施",
      "file": "research/pipeline/3-guided_story/L1/step6/step.json",
      "sequence_id": "step6"
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
        "introduced_terms": [],
        "lines": [
          "想象一下，你不需要一直盯着屏幕。",
          "电脑就能帮你做交易决策并自动下单。"
        ],
        "type": "narration"
      },
      {
        "id": "s002",
        "introduced_terms": [
          "algorithmic_trading"
        ],
        "lines": [
          "这就是 <term id=\"algorithmic_trading\">算法交易</term>。",
          "它用数学模型和程序来替代人的判断和操作。"
        ],
        "type": "narration"
      },
      {
        "id": "s003",
        "introduced_terms": [
          "trading_signal",
          "execution"
        ],
        "lines": [
          "算法交易主要做两件事：",
          "生成 <term id=\"trading_signal\">交易信号</term>，也就是决策；",
          "以及自动 <term id=\"execution\">执行</term>，也就是下单。"
        ],
        "type": "narration"
      },
      {
        "id": "s004",
        "lines": [
          "它的目标很明确：",
          "最大化利润、控制交易成本、对冲风险。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 0,
          "explanation": "算法交易就是用数学模型和计算机程序来生成交易信号并自动执行。",
          "kind": "single_choice",
          "options": [
            "用数学模型和程序进行决策与执行",
            "手动分析图表并下单",
            "只用于加密货币市场",
            "完全避免任何风险"
          ],
          "prompt": "算法交易的核心是什么？"
        },
        "id": "s005",
        "lines": [
          "算法交易的核心是什么？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step1",
    "source": {
      "plain_text": "Algorithmic trading, also called Algo Trading, Quant Trading, Robo-Trading, Program Trading. Use of mathematical models and computer algorithms/programs to generate trading signals (i.e. Decision making) and automate trading process (i.e. Execution). Objectives: maximize profits, control execution costs, hedging and manage investment risks.",
      "related": [
        "algorithmic trading basics"
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
      "execution": {
        "aliases": [
          "execution"
        ],
        "display": "执行",
        "gloss": "将交易指令实际发送到市场并完成买卖的过程。"
      },
      "trading_signal": {
        "aliases": [
          "trading signal"
        ],
        "display": "交易信号",
        "gloss": "根据规则或模型产生的买入或卖出指示。"
      }
    }
  },
  {
    "lesson_id": "L1",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s006",
        "lines": [
          "算法交易已经不是小众玩法了。",
          "在美国股市，超过60%的交易量都由算法完成。"
        ],
        "type": "narration"
      },
      {
        "id": "s007",
        "lines": [
          "在欧洲，这个比例是40%。",
          "在外汇市场，也有25%的交易是算法驱动的。"
        ],
        "type": "narration"
      },
      {
        "id": "s008",
        "lines": [
          "整个市场预计以每年12.7%的速度增长。",
          "北美最大，亚太地区增长最快。"
        ],
        "type": "narration"
      },
      {
        "id": "s009",
        "introduced_terms": [
          "robo_trading",
          "robo_advisory"
        ],
        "lines": [
          "不过，别把 <term id=\"robo_trading\">机器人交易</term> 和 <term id=\"robo_advisory\">机器人投顾</term> 搞混了。",
          "它们有本质区别。"
        ],
        "type": "narration"
      },
      {
        "id": "s010",
        "lines": [
          "机器人交易是全自动的：",
          "它自己做决策，自己执行交易。"
        ],
        "type": "narration"
      },
      {
        "id": "s011",
        "lines": [
          "机器人投顾是半自动的：",
          "它给你建议、市场总结和风险提醒，",
          "但下单还得你自己来。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 0,
          "explanation": "机器人交易是全自动的，包括执行；机器人投顾只提供建议，不自动执行。",
          "kind": "single_choice",
          "options": [
            "机器人交易自动执行，机器人投顾不执行",
            "机器人投顾更赚钱",
            "机器人交易只用于股票",
            "机器人投顾是全自动的"
          ],
          "prompt": "以下哪个是机器人交易和机器人投顾的关键区别？"
        },
        "id": "s012",
        "lines": [
          "以下哪个是机器人交易和机器人投顾的关键区别？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step2",
    "source": {
      "plain_text": "Algorithmic trading strategies account for approximately 60% of all US equity volume, 40% of all European equity volume, 25% of all Forex transactions, 20% of all US option trades. Robo-Trading vs Robo-Advisory: Robo-Trading is fully automated including decision making and execution; Robo-Advisory is semi-automated, providing advice and alerts but not execution.",
      "related": [
        "market trends",
        "trading approaches comparison"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "robo_advisory": {
        "aliases": [
          "Robo-Advisory"
        ],
        "display": "机器人投顾",
        "gloss": "半自动化的投资建议服务，提供建议但不自动执行交易。"
      },
      "robo_trading": {
        "aliases": [
          "Robo-Trading"
        ],
        "display": "机器人交易",
        "gloss": "完全自动化的算法交易，包括决策和执行。"
      }
    }
  },
  {
    "lesson_id": "L1",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s013",
        "introduced_terms": [
          "order_book"
        ],
        "lines": [
          "市场是怎么撮合交易的？",
          "核心就是 <term id=\"order_book\">订单簿</term>。"
        ],
        "type": "narration"
      },
      {
        "id": "s014",
        "lines": [
          "订单簿里有两边：",
          "买方出价和卖方要价。"
        ],
        "type": "narration"
      },
      {
        "id": "s015",
        "introduced_terms": [
          "bid_price",
          "ask_price"
        ],
        "lines": [
          "买方出的最高价叫 <term id=\"bid_price\">买入价</term>。",
          "卖方接受的最低价叫 <term id=\"ask_price\">卖出价</term>。"
        ],
        "type": "narration"
      },
      {
        "id": "s016",
        "introduced_terms": [
          "bid_ask_spread"
        ],
        "lines": [
          "卖出价减去买入价，就是 <term id=\"bid_ask_spread\">买卖价差</term>。",
          "价差越小，说明市场流动性越好。"
        ],
        "type": "narration"
      },
      {
        "id": "s017",
        "lines": [
          "比如，买入价是24元，卖出价是25元。",
          "价差就是1元。"
        ],
        "type": "narration"
      },
      {
        "id": "s018",
        "lines": [
          "下单方式也有讲究。",
          "最常用的是三种：市价单、限价单和止损单。"
        ],
        "type": "narration"
      },
      {
        "id": "s019",
        "introduced_terms": [
          "market_order"
        ],
        "lines": [
          "<term id=\"market_order\">市价单</term>：按当前最优价格立即成交。",
          "保证成交，但不保证价格。"
        ],
        "type": "narration"
      },
      {
        "id": "s020",
        "introduced_terms": [
          "limit_order"
        ],
        "lines": [
          "<term id=\"limit_order\">限价单</term>：指定一个价格，只有达到或更好才成交。",
          "保证价格，但不保证成交。"
        ],
        "type": "narration"
      },
      {
        "id": "s021",
        "introduced_terms": [
          "stop_order"
        ],
        "lines": [
          "<term id=\"stop_order\">止损单</term>：价格触及设定水平后，自动变成市价单。",
          "常用于止损或突破买入。"
        ],
        "type": "narration"
      },
      {
        "id": "s022",
        "introduced_terms": [
          "slippage"
        ],
        "lines": [
          "还有一个概念叫 <term id=\"slippage\">滑点</term>。",
          "就是预期成交价和实际成交价之间的差距。"
        ],
        "type": "narration"
      },
      {
        "id": "s023",
        "lines": [
          "比如你预期50元买入，但实际成交在51元。",
          "这1元的差价就是滑点。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 0,
          "explanation": "市价单以当前最优价格立即成交，保证成交但不保证价格。",
          "kind": "single_choice",
          "options": [
            "市价单",
            "限价单",
            "止损单",
            "以上都不是"
          ],
          "prompt": "哪种订单能保证立即成交，但不保证价格？"
        },
        "id": "s024",
        "lines": [
          "哪种订单能保证立即成交，但不保证价格？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step3",
    "source": {
      "plain_text": "Order Book is a real-time, continually updating list of buy and sell orders. Bid price is the highest price a buyer is willing to pay. Ask price is the lowest price a seller is willing to accept. Bid-ask spread is the difference. Market order: buy/sell at best available price. Limit order: buy/sell at a specified price or better. Stop order: becomes market order when stop price is reached. Slippage: difference between expected and actual execution price.",
      "related": [
        "trading mechanism",
        "order matching"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "ask_price": {
        "aliases": [
          "Ask Price",
          "Offer Price"
        ],
        "display": "卖出价",
        "gloss": "卖方愿意接受的最低价格。"
      },
      "bid_ask_spread": {
        "aliases": [
          "Bid-Ask Spread",
          "Market Spread"
        ],
        "display": "买卖价差",
        "gloss": "卖出价与买入价之间的差额，衡量市场流动性。"
      },
      "bid_price": {
        "aliases": [
          "Bid Price"
        ],
        "display": "买入价",
        "gloss": "买方愿意支付的最高价格。"
      },
      "limit_order": {
        "aliases": [
          "Limit Order"
        ],
        "display": "限价单",
        "gloss": "指定一个价格，只有达到或更优时才成交的订单。"
      },
      "market_order": {
        "aliases": [
          "Market Order"
        ],
        "display": "市价单",
        "gloss": "以当前市场最优价格立即成交的订单。"
      },
      "order_book": {
        "aliases": [
          "Order Book"
        ],
        "display": "订单簿",
        "gloss": "实时更新的买卖订单列表，按价格排序。"
      },
      "slippage": {
        "aliases": [
          "Slippage"
        ],
        "display": "滑点",
        "gloss": "预期成交价与实际成交价之间的差异。"
      },
      "stop_order": {
        "aliases": [
          "Stop Order"
        ],
        "display": "止损单",
        "gloss": "当价格触及指定水平时，自动变成市价单的订单。"
      }
    }
  },
  {
    "lesson_id": "L1",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s025",
        "lines": [
          "交易中最基本的两个方向：",
          "做多和做空。"
        ],
        "type": "narration"
      },
      {
        "id": "s026",
        "introduced_terms": [
          "long_position"
        ],
        "lines": [
          "<term id=\"long_position\">多头头寸</term>：先买入，等涨了再卖出。",
          "这是最熟悉的操作。"
        ],
        "type": "narration"
      },
      {
        "id": "s027",
        "introduced_terms": [
          "short_position"
        ],
        "lines": [
          "<term id=\"short_position\">空头头寸</term>：先借入股票卖出，等跌了再买回。",
          "这是从下跌中获利的方式。"
        ],
        "type": "narration"
      },
      {
        "id": "s028",
        "introduced_terms": [
          "short_selling"
        ],
        "lines": [
          "做空的具体流程叫 <term id=\"short_selling\">卖空</term>：",
          "借入股票 → 卖出 → 等跌了买回 → 归还股票。"
        ],
        "type": "narration"
      },
      {
        "id": "s029",
        "lines": [
          "做空的风险很大，因为理论上亏损没有上限。",
          "如果股价一直涨，你就得一直亏。"
        ],
        "type": "narration"
      },
      {
        "id": "s030",
        "introduced_terms": [
          "mark_to_market"
        ],
        "lines": [
          "无论做多还是做空，你的盈亏都会随着市场价格变化。",
          "这个过程叫 <term id=\"mark_to_market\">逐日盯市</term>。"
        ],
        "type": "narration"
      },
      {
        "formula": {
          "latex": "PnL = Quantity \\times (P_{current} - P_{entry})",
          "spoken": "盈亏等于数量乘以当前价减去买入价"
        },
        "id": "s031",
        "lines": [
          "做多的盈亏公式：",
          "PnL = 数量 × (当前价 - 买入价)"
        ],
        "type": "formula"
      },
      {
        "formula": {
          "latex": "PnL = -1 \\times Quantity \\times (P_{current} - P_{entry})",
          "spoken": "盈亏等于负一乘以数量再乘以当前价减去卖出价"
        },
        "id": "s032",
        "lines": [
          "做空的盈亏公式：",
          "PnL = -1 × 数量 × (当前价 - 卖出价)"
        ],
        "type": "formula"
      },
      {
        "exercise": {
          "answers": [
            "1000"
          ],
          "explanation": "做空盈亏 = -1 × 100 × (40 - 50) = 1000元，即盈利1000元。",
          "kind": "fill_in_blank",
          "prompt": "如果你以50元卖空100股，现在股价跌到40元，你的盈亏是多少？"
        },
        "id": "s033",
        "lines": [
          "如果你以50元卖空100股，现在股价跌到40元，你的盈亏是多少？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step4",
    "source": {
      "plain_text": "Long Position: Buying an asset with the expectation that its price will rise. Short Position: Selling an asset that you do not own, with the intention of buying it back later at a lower price. Short-Selling: Borrowing shares, selling them, buying back later, returning shares. Mark-to-Market: calculating asset value according to current market value. PnL formulas for long and short positions.",
      "related": [
        "trading positions",
        "profit and loss"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "long_position": {
        "aliases": [
          "Long Position",
          "做多"
        ],
        "display": "多头头寸",
        "gloss": "买入资产，预期价格上涨后卖出获利。"
      },
      "mark_to_market": {
        "aliases": [
          "Mark-to-Market"
        ],
        "display": "逐日盯市",
        "gloss": "按当前市场价值计算资产或头寸的价值。"
      },
      "short_position": {
        "aliases": [
          "Short Position",
          "做空"
        ],
        "display": "空头头寸",
        "gloss": "先借入资产卖出，预期价格下跌后买回获利。"
      },
      "short_selling": {
        "aliases": [
          "Short-Selling"
        ],
        "display": "卖空",
        "gloss": "借入股票卖出，希望未来以更低价格买回。"
      }
    }
  },
  {
    "lesson_id": "L1",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s034",
        "introduced_terms": [
          "leverage",
          "margin"
        ],
        "lines": [
          "想用小资金撬动大交易？",
          "这就涉及 <term id=\"leverage\">杠杆</term> 和 <term id=\"margin\">保证金</term>。"
        ],
        "type": "narration"
      },
      {
        "id": "s035",
        "lines": [
          "杠杆就是用借来的钱放大收益。",
          "保证金就是你放在经纪商那里的自有资金。"
        ],
        "type": "narration"
      },
      {
        "formula": {
          "latex": "Leverage\\ Ratio = \\frac{Asset}{Equity}",
          "spoken": "杠杆比率等于资产总值除以自有资金"
        },
        "id": "s036",
        "lines": [
          "杠杆比率 = 资产总值 / 自有资金",
          "比如你付60万买100万的股票，杠杆就是1.67倍。"
        ],
        "type": "formula"
      },
      {
        "id": "s037",
        "introduced_terms": [
          "buying_power"
        ],
        "lines": [
          "你的 <term id=\"buying_power\">购买力</term> 就是杠杆比率乘以你的自有资金。",
          "比如10万本金，20倍杠杆，最多可以买200万的证券。"
        ],
        "type": "narration"
      },
      {
        "id": "s038",
        "introduced_terms": [
          "order_based_system",
          "position_based_system"
        ],
        "lines": [
          "交易系统也有两种类型：",
          "<term id=\"order_based_system\">基于订单的系统</term> 和 <term id=\"position_based_system\">基于头寸的系统</term>。"
        ],
        "type": "narration"
      },
      {
        "id": "s039",
        "lines": [
          "基于订单的系统：每笔交易独立管理，不支持部分平仓。",
          "比如MetaTrader、TradingView。"
        ],
        "type": "narration"
      },
      {
        "id": "s040",
        "lines": [
          "基于头寸的系统：交易按净头寸管理，支持部分平仓。",
          "比如Interactive Brokers、Binance。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answers": [
            "50万",
            "500000"
          ],
          "explanation": "购买力 = 杠杆比率 × 本金 = 10 × 5万 = 50万元。",
          "kind": "fill_in_blank",
          "prompt": "如果你有5万元本金，使用10倍杠杆，你的最大购买力是多少？"
        },
        "id": "s041",
        "lines": [
          "如果你有5万元本金，使用10倍杠杆，你的最大购买力是多少？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step5",
    "source": {
      "plain_text": "Leverage refers to using borrowed capital to increase potential return. Margin is a way to create leverage. Buying Power = Leverage Ratio × Investor's equity. Order based system: transactions managed in a round order manner, partial close not supported. Position based system: transactions independent, partial close allowed.",
      "related": [
        "leverage and margin",
        "trading systems"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "buying_power": {
        "aliases": [
          "Buying Power"
        ],
        "display": "购买力",
        "gloss": "投资者可用于购买证券的最大资金量。"
      },
      "leverage": {
        "aliases": [
          "Leverage"
        ],
        "display": "杠杆",
        "gloss": "使用借入资金来放大投资回报的方式。"
      },
      "margin": {
        "aliases": [
          "Margin"
        ],
        "display": "保证金",
        "gloss": "投资者在经纪商处持有的自有资金，用于担保借款。"
      },
      "order_based_system": {
        "aliases": [
          "Order Based System"
        ],
        "display": "基于订单的系统",
        "gloss": "每笔交易独立管理，不支持部分平仓。"
      },
      "position_based_system": {
        "aliases": [
          "Position Based System"
        ],
        "display": "基于头寸的系统",
        "gloss": "交易按净头寸管理，支持部分平仓。"
      }
    }
  },
  {
    "lesson_id": "L1",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s042",
        "lines": [
          "算法是车，数据是燃料。",
          "没有数据，算法寸步难行。"
        ],
        "type": "narration"
      },
      {
        "id": "s043",
        "lines": [
          "数据工作包括：采集、存储、管理和处理。",
          "而且数据量巨大，格式多样。"
        ],
        "type": "narration"
      },
      {
        "id": "s044",
        "introduced_terms": [
          "data_feed"
        ],
        "lines": [
          "除了数据，还需要：",
          "实时 <term id=\"data_feed\">数据源</term>、历史数据、计算硬件。"
        ],
        "type": "narration"
      },
      {
        "id": "s045",
        "introduced_terms": [
          "order_management_system"
        ],
        "lines": [
          "还要考虑网络延迟、交易所连接、",
          "<term id=\"order_management_system\">订单管理系统</term>、网络安全等。"
        ],
        "type": "narration"
      },
      {
        "id": "s046",
        "lines": [
          "搭建一套完整的算法交易系统，",
          "远不止写代码那么简单。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 0,
          "explanation": "社交媒体账号不是算法交易必需的基础设施，而数据源、订单管理系统和计算硬件都是。",
          "kind": "single_choice",
          "options": [
            "社交媒体账号",
            "实时数据源",
            "订单管理系统",
            "计算硬件"
          ],
          "prompt": "以下哪项不是实施算法交易必需的基础设施？"
        },
        "id": "s047",
        "lines": [
          "以下哪项不是实施算法交易必需的基础设施？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step6",
    "source": {
      "plain_text": "Algo is the vehicle; Data is the Fuel! Data collection, storage, management, processing. Other factors: real-time datafeed, historical data, computing hardware, exchange/broker connectivity, network and latencies, order routing system, order management system, algo maintenance, cybersecurity.",
      "related": [
        "infrastructure",
        "data management"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "data_feed": {
        "aliases": [
          "Data Feed"
        ],
        "display": "数据源",
        "gloss": "实时或历史的市场数据流。"
      },
      "order_management_system": {
        "aliases": [
          "OMS",
          "Order Management System"
        ],
        "display": "订单管理系统",
        "gloss": "用于管理、发送和跟踪订单的软件系统。"
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
      "coverage_tag": "algo_trading_definition",
      "covered_by": [
        "qf_flash_algo_def",
        "qf_quiz_algo_core",
        "qf_long_algo_explain"
      ],
      "description": "算法交易的基本概念：使用数学模型和计算机程序生成交易信号并自动执行。"
    },
    {
      "coverage_tag": "algo_trading_aliases",
      "covered_by": [
        "qf_flash_algo_aliases"
      ],
      "description": "算法交易的别名：Algo Trading, Quant Trading, Robo-Trading, Program Trading。"
    },
    {
      "coverage_tag": "algo_trading_components",
      "covered_by": [
        "qf_flash_algo_components",
        "qf_quiz_algo_components"
      ],
      "description": "算法交易的两大核心组件：生成交易信号（决策）和自动执行（下单）。"
    },
    {
      "coverage_tag": "algo_trading_objectives",
      "covered_by": [
        "qf_flash_algo_objectives",
        "qf_quiz_algo_objectives"
      ],
      "description": "算法交易的目标：最大化利润、控制交易成本、对冲和管理投资风险。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "algo_trading_definition",
      "coverage_tags": [
        "algo_trading_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_algo_def",
      "learning_goal": "学生能用自己的话准确说出算法交易的核心定义。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "算法交易的核心定义：用数学模型和程序生成交易信号并自动执行。",
      "term_refs": [
        {
          "display": "算法交易",
          "en": "Algorithmic Trading"
        }
      ],
      "variants": [
        {
          "back": "用数学模型和计算机程序来生成交易信号并自动执行交易。",
          "estimated_seconds": 8,
          "explanation": "算法交易的核心就是用数学模型和程序替代人的判断和操作，完成决策和执行。",
          "front": "算法交易的核心是什么？",
          "question_id": "q_flash_algo_def_v1"
        },
        {
          "back": "数学模型和计算机程序（算法）。",
          "estimated_seconds": 6,
          "explanation": "算法交易依赖数学模型进行决策，依赖计算机程序自动执行。",
          "front": "算法交易用哪两样东西来替代人的判断和操作？",
          "question_id": "q_flash_algo_def_v2"
        }
      ]
    },
    {
      "concept_key": "algo_trading_aliases",
      "coverage_tags": [
        "algo_trading_aliases"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_algo_aliases",
      "learning_goal": "学生能说出算法交易的至少两个常用别名。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "算法交易的别名：Algo Trading, Quant Trading, Robo-Trading, Program Trading。",
      "term_refs": [
        {
          "display": "算法交易",
          "en": "Algorithmic Trading"
        }
      ],
      "variants": [
        {
          "back": "Algo Trading、Quant Trading、Robo-Trading、Program Trading（说出任意三个即可）。",
          "estimated_seconds": 8,
          "explanation": "这些名称都指向同一概念：用数学模型和程序进行交易。",
          "front": "除了 Algorithmic Trading，算法交易还有哪三个常见的英文别名？",
          "question_id": "q_flash_algo_aliases_v1"
        },
        {
          "back": "Quant Trading 是算法交易的一个别名，本质相同。",
          "estimated_seconds": 6,
          "explanation": "Quant Trading 强调量化模型，Algo Trading 强调算法执行，但两者常互换使用。",
          "front": "Quant Trading 和 Algo Trading 是什么关系？",
          "question_id": "q_flash_algo_aliases_v2"
        }
      ]
    },
    {
      "concept_key": "algo_trading_components",
      "coverage_tags": [
        "algo_trading_components"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_algo_components",
      "learning_goal": "学生能准确说出算法交易的两大核心组件及其含义。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "算法交易的两件事：生成交易信号（决策）和自动执行（下单）。",
      "term_refs": [
        {
          "display": "交易信号",
          "en": "Trading Signal"
        },
        {
          "display": "执行",
          "en": "Execution"
        }
      ],
      "variants": [
        {
          "back": "生成交易信号（决策）和自动执行（下单）。",
          "estimated_seconds": 6,
          "explanation": "决策由数学模型产生，执行由程序自动完成。",
          "front": "算法交易主要做哪两件事？",
          "question_id": "q_flash_algo_components_v1"
        },
        {
          "back": "决策过程（决定买入或卖出）。",
          "estimated_seconds": 5,
          "explanation": "交易信号是数学模型根据市场数据产生的买卖指示。",
          "front": "在算法交易中，'交易信号'对应什么过程？",
          "question_id": "q_flash_algo_components_v2"
        },
        {
          "back": "自动下单过程（将交易指令发送到市场）。",
          "estimated_seconds": 5,
          "explanation": "执行是算法交易中自动化程度最高的环节之一。",
          "front": "在算法交易中，'执行'对应什么过程？",
          "question_id": "q_flash_algo_components_v3"
        }
      ]
    },
    {
      "concept_key": "algo_trading_objectives",
      "coverage_tags": [
        "algo_trading_objectives"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_algo_objectives",
      "learning_goal": "学生能说出算法交易的三个主要目标。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "算法交易的三个目标：最大化利润、控制执行成本、对冲和管理投资风险。",
      "term_refs": [
        {
          "display": "利润最大化",
          "en": "Maximize Profits"
        },
        {
          "display": "执行成本控制",
          "en": "Control Execution Costs"
        },
        {
          "display": "对冲",
          "en": "Hedging"
        }
      ],
      "variants": [
        {
          "back": "最大化利润、控制执行成本、对冲和管理投资风险。",
          "estimated_seconds": 8,
          "explanation": "这三个目标分别对应收益、效率和风险控制。",
          "front": "算法交易的三个主要目标是什么？",
          "question_id": "q_flash_algo_objectives_v1"
        },
        {
          "back": "控制交易过程中产生的各种成本，如滑点、佣金等。",
          "estimated_seconds": 6,
          "explanation": "算法交易通过优化下单策略来降低执行成本。",
          "front": "算法交易中'控制执行成本'指的是控制什么？",
          "question_id": "q_flash_algo_objectives_v2"
        }
      ]
    }
  ],
  "lesson_id": "L1",
  "longform_families": [
    {
      "concept_key": "algo_trading_definition",
      "coverage_tags": [
        "algo_trading_definition",
        "algo_trading_components",
        "algo_trading_objectives"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_algo_explain",
      "learning_goal": "学生能用一段连贯的文字解释算法交易的定义、核心组件和目标。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "算法交易",
          "en": "Algorithmic Trading"
        },
        {
          "display": "交易信号",
          "en": "Trading Signal"
        },
        {
          "display": "执行",
          "en": "Execution"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "算法交易的定义",
            "两个核心组件",
            "三个主要目标"
          ],
          "question_id": "q_long_algo_explain_v1",
          "reference_answer": [
            "算法交易是一种使用数学模型和计算机程序来自动进行交易决策和执行的方式。",
            "它的核心包括两个部分：一是生成交易信号，即根据模型做出买入或卖出的决策；二是自动执行，即将交易指令自动发送到市场完成买卖。",
            "算法交易的主要目标有三个：最大化利润、控制执行成本、以及对冲和管理投资风险。"
          ],
          "rubric_points": [
            "提到使用数学模型和计算机程序",
            "提到生成交易信号（决策）和自动执行（下单）",
            "提到最大化利润、控制执行成本、对冲和管理风险"
          ],
          "stem": "请用你自己的话解释什么是算法交易，包括它的核心组件和主要目标。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "用比喻或简单语言解释算法交易",
            "说明它做的两件事",
            "说明它的三个好处"
          ],
          "question_id": "q_long_algo_explain_v2",
          "reference_answer": [
            "算法交易就是让电脑程序帮你做股票买卖的决策和操作，你不用一直盯着屏幕。",
            "程序主要做两件事：一是根据数学模型判断什么时候该买或卖，二是自动下单完成交易。",
            "这样做的好处是：可以更快抓住赚钱机会、降低交易成本、还能自动控制风险。"
          ],
          "rubric_points": [
            "解释算法交易是用电脑程序代替人做交易决策和操作",
            "说明程序会自己决定买卖时机并自动下单",
            "说明这样做可以赚更多钱、省成本、控制风险"
          ],
          "stem": "假设你要向一个不懂金融的朋友介绍算法交易，请用通俗的语言解释：什么是算法交易？它主要做什么？为什么要用它？"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "algo_trading_definition",
      "coverage_tags": [
        "algo_trading_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_algo_core",
      "learning_goal": "学生能在多个选项中准确识别算法交易的核心定义。",
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
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "算法交易的核心就是用数学模型和程序替代人的判断和操作，完成决策和执行。",
          "options": [
            "用数学模型和计算机程序进行交易决策和自动执行",
            "手动分析图表并手动下单",
            "只适用于加密货币市场的交易方式",
            "一种完全避免任何投资风险的交易方法"
          ],
          "question_id": "q_quiz_algo_core_v1",
          "stem": "以下哪项最准确地描述了算法交易？"
        },
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "算法交易的核心特征是用数学模型和程序替代人的判断和操作，实现自动化。",
          "options": [
            "交易决策和执行由计算机程序自动完成",
            "交易金额更大",
            "交易频率更低",
            "只使用技术分析"
          ],
          "question_id": "q_quiz_algo_core_v2",
          "stem": "算法交易与传统手动交易最本质的区别是什么？"
        }
      ]
    },
    {
      "concept_key": "algo_trading_components",
      "coverage_tags": [
        "algo_trading_components"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_algo_components",
      "learning_goal": "学生能区分算法交易中'决策'和'执行'两个核心组件。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "交易信号",
          "en": "Trading Signal"
        },
        {
          "display": "执行",
          "en": "Execution"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 15,
          "explanation": "交易信号是数学模型根据市场数据产生的买卖指示，属于决策过程。",
          "options": [
            "决策过程",
            "执行过程",
            "结算过程",
            "风控过程"
          ],
          "question_id": "q_quiz_algo_components_v1",
          "stem": "在算法交易中，'生成交易信号'对应的是哪个过程？"
        },
        {
          "answer": 0,
          "estimated_seconds": 15,
          "explanation": "执行是将交易指令实际发送到市场并完成买卖的过程，是算法交易自动化的关键环节。",
          "options": [
            "自动将交易指令发送到市场并完成买卖",
            "自动生成交易信号",
            "自动分析财务报表",
            "自动计算投资组合收益"
          ],
          "question_id": "q_quiz_algo_components_v2",
          "stem": "算法交易中的'自动执行'指的是什么？"
        }
      ]
    },
    {
      "concept_key": "algo_trading_objectives",
      "coverage_tags": [
        "algo_trading_objectives"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_algo_objectives",
      "learning_goal": "学生能识别算法交易的多个目标。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "multiple_choice",
      "term_refs": [
        {
          "display": "利润最大化",
          "en": "Maximize Profits"
        },
        {
          "display": "执行成本控制",
          "en": "Control Execution Costs"
        },
        {
          "display": "对冲",
          "en": "Hedging"
        }
      ],
      "variants": [
        {
          "answer": [
            0,
            1,
            3
          ],
          "estimated_seconds": 25,
          "explanation": "算法交易的目标是最大化利润、控制执行成本、对冲和管理投资风险。没有任何交易方式能保证每次盈利。",
          "options": [
            "最大化利润",
            "控制执行成本",
            "保证每次交易都盈利",
            "对冲和管理投资风险"
          ],
          "question_id": "q_quiz_algo_objectives_v1",
          "stem": "以下哪些是算法交易的主要目标？（多选）"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "算法交易可以管理和对冲风险，但无法完全消除市场风险。",
          "options": [
            "最大化利润",
            "控制执行成本",
            "消除所有市场风险",
            "对冲投资风险"
          ],
          "question_id": "q_quiz_algo_objectives_v2",
          "stem": "以下哪项不是算法交易的主要目标？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L1: Algorithmic trading basics and financial markets - Agenda: The basic concept of algo-trading, The market trend of algo-trading, Comparison of different trading approaches, Infrastructure needed to deploy algo-trading programs, Trading mechanism and order matching, Market jargon and terminology",
    "guided_story_manifest": "pipeline/3-guided_story/L1/manifest.json",
    "lesson_map": "L1 guided_story steps: step1: 什么是算法交易, step2: 算法交易的市场趋势与对比, step3: 交易机制与订单簿, step4: 做多、做空与盈亏计算, step5: 杠杆、保证金与交易系统, step6: 实施算法交易的基础设施",
    "plain_text": "pipeline/1-plain/L1/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L1: Algorithmic trading basics and financial markets - What is algo-trading?, Market Trend for Algo-Trading, Robo-Trading vs Robo-Advisory, Comparison of Trading Approaches, What should you consider to implement a trading algo?, Algo is the vehicle; Data is the Fuel!, Other factors for infrastructure setup"
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "infrastructure_data_management",
      "covered_by": [
        "qf_flash_data_work",
        "qf_quiz_data_work",
        "qf_long_infrastructure"
      ],
      "description": "实施算法交易所需的数据基础设施：数据采集、存储、管理和处理"
    },
    {
      "coverage_tag": "infrastructure_other_factors",
      "covered_by": [
        "qf_flash_other_factors",
        "qf_quiz_other_factors",
        "qf_long_infrastructure"
      ],
      "description": "实施算法交易的其他基础设施因素：实时数据源、历史数据、计算硬件、交易所连接、网络延迟、订单管理系统、网络安全等"
    },
    {
      "coverage_tag": "data_as_fuel_metaphor",
      "covered_by": [
        "qf_flash_data_fuel",
        "qf_quiz_data_fuel"
      ],
      "description": "理解“算法是车，数据是燃料”的核心比喻"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "data_as_fuel_metaphor",
      "coverage_tags": [
        "data_as_fuel_metaphor"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_data_fuel",
      "learning_goal": "学生能准确回忆并解释“算法是车，数据是燃料”这一核心比喻的含义。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "要求学生主动提取该比喻及其对算法交易的意义。",
      "term_refs": [
        {
          "display": "数据是燃料",
          "en": "Data is the Fuel"
        }
      ],
      "variants": [
        {
          "back": "算法依赖数据才能运行和决策；没有数据，算法无法工作。",
          "estimated_seconds": 8,
          "explanation": "这个比喻强调数据是算法交易的基础和驱动力。",
          "front": "在算法交易中，常说的“算法是车，数据是燃料”是什么意思？",
          "question_id": "q_flash_data_fuel_v1"
        },
        {
          "back": "因为算法需要数据来生成交易信号、执行交易、进行回测和优化。",
          "estimated_seconds": 10,
          "explanation": "数据是算法交易的输入和决策依据。",
          "front": "在算法交易中，为什么说“没有数据，算法寸步难行”？",
          "question_id": "q_flash_data_fuel_v2"
        }
      ]
    },
    {
      "concept_key": "infrastructure_data_management",
      "coverage_tags": [
        "infrastructure_data_management"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_data_work",
      "learning_goal": "学生能准确回忆算法交易中数据工作的四个主要环节。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "要求学生主动提取数据工作的四个环节。",
      "term_refs": [
        {
          "display": "数据工作",
          "en": "Data Work"
        }
      ],
      "variants": [
        {
          "back": "采集、存储、管理、处理。",
          "estimated_seconds": 8,
          "explanation": "这四个环节覆盖了从数据获取到使用的完整流程。",
          "front": "实施算法交易时，数据工作包括哪四个主要环节？",
          "question_id": "q_flash_data_work_v1"
        },
        {
          "back": "数据来源和清洗非常耗时费力。",
          "estimated_seconds": 8,
          "explanation": "数据采集是基础，但也是耗时耗力的环节。",
          "front": "在算法交易的数据工作中，“采集”环节面临的主要挑战是什么？",
          "question_id": "q_flash_data_work_v2"
        },
        {
          "back": "需要处理海量的非结构化数据（数值、文本、图像、音频、视频）。",
          "estimated_seconds": 10,
          "explanation": "数据量大且格式多样，对存储系统要求高。",
          "front": "在算法交易的数据工作中，“存储”环节面临的主要挑战是什么？",
          "question_id": "q_flash_data_work_v3"
        },
        {
          "back": "由于文件和数据格式不一致，难以通过传统数据库进行保护和管理。",
          "estimated_seconds": 10,
          "explanation": "数据格式不统一是管理的主要难点。",
          "front": "在算法交易的数据工作中，“管理”环节面临的主要挑战是什么？",
          "question_id": "q_flash_data_work_v4"
        }
      ]
    },
    {
      "concept_key": "infrastructure_other_factors",
      "coverage_tags": [
        "infrastructure_other_factors"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_other_factors",
      "learning_goal": "学生能准确回忆除数据工作外，实施算法交易所需的其他基础设施因素。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "要求学生主动提取其他基础设施因素。",
      "term_refs": [
        {
          "display": "数据源",
          "en": "Data Feed"
        },
        {
          "display": "订单管理系统",
          "en": "Order Management System"
        }
      ],
      "variants": [
        {
          "back": "实时数据源、历史数据、计算硬件、交易所/经纪商连接、网络和延迟、订单路由系统、订单管理系统、算法维护、网络安全。",
          "estimated_seconds": 15,
          "explanation": "这些因素共同构成了算法交易的运行环境。",
          "front": "除了数据工作，实施算法交易还需要考虑哪些基础设施因素？请列出至少三个。",
          "question_id": "q_flash_other_factors_v1"
        },
        {
          "back": "OMS代表订单管理系统（Order Management System），用于管理、发送和跟踪订单。",
          "estimated_seconds": 10,
          "explanation": "OMS是连接算法和交易所的关键软件。",
          "front": "在算法交易基础设施中，OMS代表什么？它的主要功能是什么？",
          "question_id": "q_flash_other_factors_v2"
        },
        {
          "back": "因为低延迟可以更快地执行交易，获得更好的价格，减少滑点。",
          "estimated_seconds": 10,
          "explanation": "在高速交易中，延迟直接影响交易结果。",
          "front": "在算法交易基础设施中，“网络和延迟”为什么是一个重要考虑因素？",
          "question_id": "q_flash_other_factors_v3"
        }
      ]
    }
  ],
  "lesson_id": "L1",
  "longform_families": [
    {
      "concept_key": "infrastructure_data_management",
      "coverage_tags": [
        "infrastructure_data_management",
        "infrastructure_other_factors"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_infrastructure",
      "learning_goal": "学生能综合解释实施算法交易所需的基础设施，并说明各组成部分的重要性。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "数据源",
          "en": "Data Feed"
        },
        {
          "display": "订单管理系统",
          "en": "Order Management System"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "数据工作包括哪些环节？每个环节有什么挑战？",
            "除了数据，还需要考虑哪些基础设施？",
            "这些因素如何共同影响算法交易系统的运行？"
          ],
          "question_id": "q_long_infrastructure_v1",
          "reference_answer": [
            "搭建算法交易系统远不止写代码，因为需要处理大量复杂的基础设施问题。",
            "首先，数据是燃料，数据工作包括采集（耗时费力）、存储（海量非结构化数据）、管理（格式不一致）和处理（可行性和效率）。",
            "其次，还需要实时数据源、历史数据、计算硬件、交易所连接、低延迟网络、订单管理系统、网络安全等。",
            "这些因素共同决定了算法能否稳定、高效地运行。"
          ],
          "rubric_points": [
            "正确指出数据工作的四个环节（采集、存储、管理、处理）并简要说明每个环节的挑战",
            "正确列出至少三个其他基础设施因素（如实时数据源、计算硬件、OMS、网络延迟等）",
            "解释这些因素如何共同构成算法交易系统的运行环境"
          ],
          "stem": "请解释为什么说“搭建一套完整的算法交易系统，远不止写代码那么简单”。在回答中，请至少涵盖数据工作和其它基础设施因素两个方面。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "数据方面需要考虑什么？",
            "硬件和网络方面需要考虑什么？",
            "系统和软件方面需要考虑什么？"
          ],
          "question_id": "q_long_infrastructure_v2",
          "reference_answer": [
            "数据方面：需要可靠的数据源（实时和历史），建立数据采集、存储、管理和处理的流程，应对数据量大、格式多样、清洗耗时等挑战。",
            "硬件和网络方面：需要足够的计算硬件来处理数据和运行算法，低延迟的网络连接对于快速执行交易至关重要。",
            "系统和软件方面：需要订单管理系统（OMS）来管理订单，确保与交易所/经纪商的稳定连接，并重视网络安全以保护系统和数据。"
          ],
          "rubric_points": [
            "涵盖数据采集、存储、管理、处理及其挑战",
            "涵盖实时数据源、历史数据、计算硬件、网络延迟",
            "涵盖交易所连接、订单管理系统、网络安全等"
          ],
          "stem": "假设你要为一个新的算法交易策略搭建基础设施，请列出你需要考虑的关键要素，并简要说明每个要素的重要性。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "data_as_fuel_metaphor",
      "coverage_tags": [
        "data_as_fuel_metaphor"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_data_fuel",
      "learning_goal": "学生能在测验情境下正确理解并应用“数据是燃料”这一概念。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "数据是燃料",
          "en": "Data is the Fuel"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "这个比喻强调数据是算法交易的基础，算法依赖数据才能运行。",
          "options": [
            "算法比数据更重要",
            "数据是算法交易的基础和驱动力",
            "数据只需要在初始阶段使用",
            "算法可以完全独立于数据运行"
          ],
          "question_id": "q_quiz_data_fuel_v1",
          "stem": "在算法交易中，“算法是车，数据是燃料”这个比喻最准确地说明了什么？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "数据是算法的输入，没有数据，算法无法进行任何决策或执行。",
          "options": [
            "没有好的算法，再多的数据也没用",
            "没有数据，算法无法生成交易信号或执行交易",
            "数据只用于回测，不用于实盘交易",
            "算法可以自己生成所需的数据"
          ],
          "question_id": "q_quiz_data_fuel_v2",
          "stem": "根据“算法是车，数据是燃料”的比喻，以下哪种说法是正确的？"
        }
      ]
    },
    {
      "concept_key": "infrastructure_data_management",
      "coverage_tags": [
        "infrastructure_data_management"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_data_work",
      "learning_goal": "学生能在测验情境下辨析数据工作的不同环节及其挑战。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "数据工作",
          "en": "Data Work"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "材料明确指出，由于文件和数据格式不一致，数据管理难以通过传统数据库进行。",
          "options": [
            "数据量太大，存储空间不足",
            "数据格式不一致，难以通过传统数据库保护和管理",
            "数据处理速度太慢",
            "数据来源太少"
          ],
          "question_id": "q_quiz_data_work_v1",
          "stem": "在算法交易的数据工作中，“数据管理”环节面临的主要挑战是什么？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "材料中明确列出数据工作包括：采集、存储、管理和处理。",
          "options": [
            "采集、清洗、分析、可视化",
            "采集、存储、管理、处理",
            "收集、整理、建模、回测",
            "获取、压缩、加密、传输"
          ],
          "question_id": "q_quiz_data_work_v2",
          "stem": "以下哪项最准确地描述了算法交易中数据工作的四个主要环节？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "材料中明确指出数据处理的关注点是可行性和效率。",
          "options": [
            "数据的来源是否可靠",
            "数据处理的可行性和效率",
            "数据的存储成本",
            "数据的可视化效果"
          ],
          "question_id": "q_quiz_data_work_v3",
          "stem": "在算法交易中，数据“处理”环节关注的核心问题是什么？"
        }
      ]
    },
    {
      "concept_key": "infrastructure_other_factors",
      "coverage_tags": [
        "infrastructure_other_factors"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_other_factors",
      "learning_goal": "学生能在测验情境下识别并区分实施算法交易所需的各种基础设施因素。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "数据源",
          "en": "Data Feed"
        },
        {
          "display": "订单管理系统",
          "en": "Order Management System"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "社交媒体账号不是算法交易必需的基础设施，而数据源、订单管理系统和计算硬件都是。",
          "options": [
            "实时数据源",
            "社交媒体账号",
            "订单管理系统",
            "计算硬件"
          ],
          "question_id": "q_quiz_other_factors_v1",
          "stem": "以下哪项不是实施算法交易必需的基础设施？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "OMS是用于管理、发送和跟踪订单的软件系统。",
          "options": [
            "生成交易信号",
            "管理、发送和跟踪订单",
            "分析市场数据",
            "管理投资组合风险"
          ],
          "question_id": "q_quiz_other_factors_v2",
          "stem": "在算法交易基础设施中，订单管理系统（OMS）的主要功能是什么？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "搭建完整的算法交易系统需要综合考虑数据、硬件、网络、订单管理系统、网络安全等多个方面。",
          "options": [
            "只需要编写好算法代码即可，不需要考虑其他因素",
            "网络延迟对交易结果没有影响",
            "需要综合考虑数据、硬件、网络、系统等多个方面",
            "历史数据对算法交易没有用处"
          ],
          "question_id": "q_quiz_other_factors_v3",
          "stem": "以下哪项关于算法交易基础设施的说法是正确的？"
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
,
{
  "coverage_map": [
    {
      "coverage_tag": "leverage_and_margin",
      "covered_by": [
        "qf_flash_leverage_margin",
        "qf_quiz_leverage_margin",
        "qf_long_leverage_margin"
      ],
      "description": "杠杆与保证金的概念、杠杆比率公式、保证金与杠杆的关系"
    },
    {
      "coverage_tag": "buying_power",
      "covered_by": [
        "qf_flash_buying_power",
        "qf_quiz_buying_power"
      ],
      "description": "购买力的定义与计算"
    },
    {
      "coverage_tag": "order_based_vs_position_based",
      "covered_by": [
        "qf_flash_trading_systems",
        "qf_quiz_trading_systems",
        "qf_long_trading_systems"
      ],
      "description": "基于订单的系统与基于头寸的系统的区别、特点与示例"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "leverage_and_margin",
      "coverage_tags": [
        "leverage_and_margin"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_leverage_margin",
      "learning_goal": "学生能准确回忆杠杆和保证金的核心定义及杠杆比率公式。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "杠杆与保证金的基本定义和杠杆比率公式。",
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
          "back": "使用借入资金来放大投资回报的方式。",
          "estimated_seconds": 8,
          "explanation": "杠杆允许交易者用较少的自有资金控制更大的头寸。",
          "front": "在交易中，“杠杆”指的是什么？",
          "question_id": "q_flash_leverage_margin_v1"
        },
        {
          "back": "杠杆比率 = 资产总值 / 自有资金",
          "estimated_seconds": 10,
          "explanation": "该比率衡量了投资者使用借入资金的程度。",
          "front": "杠杆比率的计算公式是什么？",
          "question_id": "q_flash_leverage_margin_v2"
        },
        {
          "back": "投资者在经纪商处持有的自有资金，用于担保借款。",
          "estimated_seconds": 8,
          "explanation": "保证金是杠杆交易的基础，它代表了投资者的自有资金部分。",
          "front": "在交易中，“保证金”指的是什么？",
          "question_id": "q_flash_leverage_margin_v3"
        }
      ]
    },
    {
      "concept_key": "buying_power",
      "coverage_tags": [
        "buying_power"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_buying_power",
      "learning_goal": "学生能准确回忆购买力的定义和计算公式。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "购买力的定义和计算公式。",
      "term_refs": [
        {
          "display": "购买力",
          "en": "Buying Power"
        }
      ],
      "variants": [
        {
          "back": "投资者可用于购买证券的最大资金量。",
          "estimated_seconds": 8,
          "explanation": "购买力决定了交易者最多可以买入多少价值的资产。",
          "front": "什么是“购买力”？",
          "question_id": "q_flash_buying_power_v1"
        },
        {
          "back": "购买力 = 杠杆比率 × 自有资金",
          "estimated_seconds": 10,
          "explanation": "例如，10万本金，20倍杠杆，购买力为200万。",
          "front": "购买力的计算公式是什么？",
          "question_id": "q_flash_buying_power_v2"
        }
      ]
    },
    {
      "concept_key": "order_based_vs_position_based",
      "coverage_tags": [
        "order_based_vs_position_based"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_trading_systems",
      "learning_goal": "学生能准确区分基于订单和基于头寸的交易系统的核心特征。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "core_difference",
      "retrieval_focus": "两种交易系统在交易管理和部分平仓支持上的关键区别。",
      "term_refs": [
        {
          "display": "基于订单的系统",
          "en": "Order Based System"
        },
        {
          "display": "基于头寸的系统",
          "en": "Position Based System"
        }
      ],
      "variants": [
        {
          "back": "每笔交易独立管理，不支持部分平仓。",
          "estimated_seconds": 10,
          "explanation": "例如MetaTrader和TradingView就是这种系统。",
          "front": "基于订单的交易系统在管理交易时有什么特点？",
          "question_id": "q_flash_trading_systems_v1"
        },
        {
          "back": "交易按净头寸管理，支持部分平仓。",
          "estimated_seconds": 10,
          "explanation": "例如Interactive Brokers和Binance就是这种系统。",
          "front": "基于头寸的交易系统在管理交易时有什么特点？",
          "question_id": "q_flash_trading_systems_v2"
        },
        {
          "back": "基于头寸的系统。",
          "estimated_seconds": 8,
          "explanation": "基于订单的系统不支持部分平仓，必须全部平仓。",
          "front": "哪个交易系统支持部分平仓？",
          "question_id": "q_flash_trading_systems_v3"
        }
      ]
    }
  ],
  "lesson_id": "L1",
  "longform_families": [
    {
      "concept_key": "leverage_and_margin",
      "coverage_tags": [
        "leverage_and_margin"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_leverage_margin",
      "learning_goal": "学生能解释杠杆和保证金的关系，并说明杠杆如何放大收益和风险。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_explain",
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
          "estimated_seconds": 120,
          "prompt_blocks": [
            "定义杠杆和保证金",
            "解释两者关系",
            "说明杠杆放大收益和风险的机制"
          ],
          "question_id": "q_long_leverage_margin_v1",
          "reference_answer": [
            "杠杆是指使用借入资金来放大投资回报的方式。保证金是投资者在经纪商处持有的自有资金，它是创造杠杆的基础。",
            "杠杆比率 = 资产总值 / 自有资金。例如，用60万自有资金购买100万资产，杠杆比率为1.67倍。",
            "杠杆放大了收益：如果资产价格上涨10%，投资者的自有资金回报率是16.7%（1.67倍）。",
            "杠杆也放大了风险：如果资产价格下跌10%，投资者的自有资金亏损率也是16.7%。如果价格下跌超过一定比例，投资者可能面临追加保证金或爆仓的风险。"
          ],
          "rubric_points": [
            "正确解释杠杆是使用借入资金放大回报。",
            "正确解释保证金是自有资金，是创造杠杆的基础。",
            "正确说明杠杆比率 = 资产总值 / 自有资金。",
            "正确解释杠杆放大收益：价格上涨时，由于本金较小，回报率更高。",
            "正确解释杠杆放大风险：价格下跌时，亏损也会被放大，甚至可能导致爆仓。"
          ],
          "stem": "请解释杠杆和保证金的关系，并说明为什么杠杆既能放大收益也能放大风险。"
        },
        {
          "estimated_seconds": 150,
          "prompt_blocks": [
            "计算购买力",
            "计算上涨20%后的收益和收益率",
            "计算下跌20%后的亏损和亏损率"
          ],
          "question_id": "q_long_leverage_margin_v2",
          "reference_answer": [
            "购买力 = 杠杆比率 × 本金 = 5 × 10万 = 50万元。",
            "如果股票价格上涨20%，总资产变为 50万 × (1 + 20%) = 60万元。",
            "收益 = 60万 - 50万 = 10万元。由于本金是10万元，实际收益率 = 10万 / 10万 = 100%。",
            "如果股票价格下跌20%，总资产变为 50万 × (1 - 20%) = 40万元。",
            "亏损 = 50万 - 40万 = 10万元。亏损率 = 10万 / 10万 = 100%。这表明本金全部亏完。"
          ],
          "rubric_points": [
            "正确计算购买力为50万元。",
            "正确计算上涨20%后的总资产为60万元。",
            "正确计算收益为10万元，收益率为100%。",
            "正确计算下跌20%后的总资产为40万元。",
            "正确计算亏损为10万元，亏损率为100%。"
          ],
          "stem": "假设你有10万元本金，使用5倍杠杆买入某股票。请计算你的购买力。如果该股票价格上涨20%，你的实际收益率是多少？如果下跌20%，你的亏损率是多少？"
        }
      ]
    },
    {
      "concept_key": "order_based_vs_position_based",
      "coverage_tags": [
        "order_based_vs_position_based"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_trading_systems",
      "learning_goal": "学生能对比基于订单和基于头寸的交易系统，并举例说明。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "compare_and_contrast",
      "term_refs": [
        {
          "display": "基于订单的系统",
          "en": "Order Based System"
        },
        {
          "display": "基于头寸的系统",
          "en": "Position Based System"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "交易管理方式",
            "部分平仓支持",
            "典型平台举例"
          ],
          "question_id": "q_long_trading_systems_v1",
          "reference_answer": [
            "基于订单的系统：每笔交易都被视为一个独立的订单，彼此之间没有关联。交易者需要分别管理每个订单。",
            "基于头寸的系统：交易者持有的是一种资产的总头寸，而不是多个独立的订单。新交易会与现有头寸合并。",
            "部分平仓支持：基于订单的系统不支持部分平仓，要平仓必须关闭整个订单。基于头寸的系统支持部分平仓，可以只平掉头寸的一部分。",
            "典型平台：基于订单的系统包括MetaTrader和TradingView。基于头寸的系统包括Interactive Brokers和Binance。"
          ],
          "rubric_points": [
            "正确说明基于订单的系统：每笔交易独立管理。",
            "正确说明基于头寸的系统：交易按净头寸管理。",
            "正确说明基于订单的系统不支持部分平仓。",
            "正确说明基于头寸的系统支持部分平仓。",
            "正确列举基于订单的系统示例（如MetaTrader, TradingView）。",
            "正确列举基于头寸的系统示例（如Interactive Brokers, Binance）。"
          ],
          "stem": "请对比基于订单的交易系统和基于头寸的交易系统，从交易管理方式、部分平仓支持和典型平台三个方面进行说明。"
        },
        {
          "estimated_seconds": 150,
          "prompt_blocks": [
            "描述3笔交易",
            "解释未实现盈亏的计算方式",
            "解释为什么需要同时关闭所有交易"
          ],
          "question_id": "q_long_trading_systems_v2",
          "reference_answer": [
            "交易1：买入2股，成本价123元。交易2：卖出1股，价格124元。交易3：卖出1股，价格125元。",
            "在基于订单的系统中，未实现盈亏是每笔交易独立计算的。例如，交易1的未实现盈亏是 2 × (当前价 - 123)，交易2是 1 × (124 - 当前价)，交易3是 1 × (125 - 当前价)。",
            "由于每笔交易是独立的，你不能只平掉交易2或交易3来锁定利润。",
            "要完全实现盈亏，你必须同时关闭所有3笔交易。例如，通过一个反向订单买入2股来平掉交易1，同时卖出1股和1股来平掉交易2和交易3。"
          ],
          "rubric_points": [
            "正确描述3笔交易。",
            "正确解释未实现盈亏是每笔交易独立计算的。",
            "正确解释由于交易是独立的，不能通过部分平仓来锁定某笔交易的利润。",
            "正确说明只有关闭所有3笔交易，才能实现总盈亏。"
          ],
          "stem": "假设你使用基于订单的系统，开了3笔交易：买入2股A股票，然后分别卖出1股和1股。请解释为什么你的未实现盈亏计算复杂，并且需要同时关闭所有3笔交易才能实现盈亏。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "leverage_and_margin",
      "coverage_tags": [
        "leverage_and_margin"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_leverage_margin",
      "learning_goal": "学生能在测验情境下辨析杠杆和保证金的概念及其关系。",
      "linked_steps": [
        "step5"
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
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "杠杆比率 = 资产总值 / 自有资金 = 100万 / 60万 ≈ 1.67倍。",
          "options": [
            "1.67倍",
            "0.6倍",
            "60倍",
            "1倍"
          ],
          "question_id": "q_quiz_leverage_margin_v1",
          "stem": "如果一位投资者支付60万元购买了价值100万元的股票，那么他的杠杆比率是多少？"
        },
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "保证金是投资者在经纪商处的自有资金，它允许投资者借入更多资金，从而创造杠杆。",
          "options": [
            "保证金是创造杠杆的一种方式。",
            "杠杆比率越高，保证金要求也越高。",
            "保证金是借入的资金，杠杆是自有资金。",
            "杠杆和保证金是同一个概念。"
          ],
          "question_id": "q_quiz_leverage_margin_v2",
          "stem": "关于杠杆和保证金，以下哪种说法是正确的？"
        }
      ]
    },
    {
      "concept_key": "buying_power",
      "coverage_tags": [
        "buying_power"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_buying_power",
      "learning_goal": "学生能应用购买力公式进行计算。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "购买力",
          "en": "Buying Power"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "购买力 = 杠杆比率 × 本金 = 15 × 20万 = 300万元。",
          "options": [
            "300万元",
            "35万元",
            "20万元",
            "150万元"
          ],
          "question_id": "q_quiz_buying_power_v1",
          "stem": "一个交易账户有20万元本金，使用15倍杠杆，其最大购买力是多少？"
        },
        {
          "answer": 0,
          "estimated_seconds": 25,
          "explanation": "自有资金 = 购买力 / 杠杆比率 = 200万 / 10 = 20万元。",
          "options": [
            "20万元",
            "200万元",
            "10万元",
            "2000万元"
          ],
          "question_id": "q_quiz_buying_power_v2",
          "stem": "如果交易者希望获得200万元的购买力，而他的经纪商提供10倍杠杆，他需要存入多少自有资金？"
        }
      ]
    },
    {
      "concept_key": "order_based_vs_position_based",
      "coverage_tags": [
        "order_based_vs_position_based"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_trading_systems",
      "learning_goal": "学生能辨析两种交易系统的特点并识别对应的交易平台。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "基于订单的系统",
          "en": "Order Based System"
        },
        {
          "display": "基于头寸的系统",
          "en": "Position Based System"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "Interactive Brokers是基于头寸的系统，支持部分平仓。MetaTrader、TradingView和MetaStock都是基于订单的系统。",
          "options": [
            "MetaTrader",
            "TradingView",
            "Interactive Brokers",
            "MetaStock"
          ],
          "question_id": "q_quiz_trading_systems_v1",
          "stem": "以下哪个交易平台是基于头寸的系统？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "基于头寸的系统支持部分平仓，而基于订单的系统不支持，必须全部平仓。",
          "options": [
            "基于订单的系统",
            "基于头寸的系统",
            "两种系统都允许",
            "两种系统都不允许"
          ],
          "question_id": "q_quiz_trading_systems_v2",
          "stem": "一个交易者开了3笔相互对冲的交易，如果他想只平掉其中1笔，以下哪种系统允许他这样做？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L1: Algorithmic trading basics and financial markets",
    "guided_story_manifest": "pipeline/3-guided_story/L1/manifest.json",
    "lesson_map": "L1 lesson map",
    "plain_text": "pipeline/1-plain/L1/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L1: Algorithmic trading basics and financial markets"
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "long_short_position",
      "covered_by": [
        "qf_flash_long_short_def",
        "qf_quiz_long_short_identify",
        "qf_long_long_short_compare"
      ],
      "description": "多头头寸与空头头寸的定义、方向与获利逻辑"
    },
    {
      "coverage_tag": "short_selling_process",
      "covered_by": [
        "qf_flash_short_selling_steps",
        "qf_quiz_short_selling_order"
      ],
      "description": "卖空的四个步骤：借入、卖出、买回、归还"
    },
    {
      "coverage_tag": "short_selling_risk",
      "covered_by": [
        "qf_flash_short_risk",
        "qf_quiz_short_risk_choice"
      ],
      "description": "做空的理论亏损无上限风险"
    },
    {
      "coverage_tag": "mark_to_market",
      "covered_by": [
        "qf_flash_mtm_definition",
        "qf_quiz_mtm_apply"
      ],
      "description": "逐日盯市的概念：按当前市价计算头寸价值"
    },
    {
      "coverage_tag": "pnl_formula_long",
      "covered_by": [
        "qf_flash_pnl_long_formula",
        "qf_quiz_pnl_long_calc",
        "qf_long_pnl_calc_apply"
      ],
      "description": "做多盈亏公式：PnL = Quantity × (P_current - P_entry)"
    },
    {
      "coverage_tag": "pnl_formula_short",
      "covered_by": [
        "qf_flash_pnl_short_formula",
        "qf_quiz_pnl_short_calc",
        "qf_long_pnl_calc_apply"
      ],
      "description": "做空盈亏公式：PnL = -1 × Quantity × (P_current - P_entry)"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "long_short_position",
      "coverage_tags": [
        "long_short_position"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_long_short_def",
      "learning_goal": "学生能准确回忆多头和空头的基本定义与获利方向。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "区分多头与空头的核心操作方向",
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
          "back": "先买入，预期价格上涨后卖出获利。",
          "estimated_seconds": 8,
          "explanation": "多头是买入资产，希望价格上升后卖出赚取差价。",
          "front": "多头头寸（Long Position）的操作方向是什么？预期价格如何变化才能获利？",
          "question_id": "q_flash_long_short_def_v1"
        },
        {
          "back": "先借入卖出，预期价格下跌后买回获利。",
          "estimated_seconds": 8,
          "explanation": "空头是借入资产卖出，希望价格下跌后低价买回归还。",
          "front": "空头头寸（Short Position）的操作方向是什么？预期价格如何变化才能获利？",
          "question_id": "q_flash_long_short_def_v2"
        }
      ]
    },
    {
      "concept_key": "short_selling_process",
      "coverage_tags": [
        "short_selling_process"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_short_selling_steps",
      "learning_goal": "学生能回忆出卖空的四个关键步骤。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "卖空流程的每一步",
      "term_refs": [
        {
          "display": "卖空",
          "en": "Short-Selling"
        }
      ],
      "variants": [
        {
          "back": "从券商借入股票。",
          "estimated_seconds": 6,
          "explanation": "卖空的第一步是借入股票，然后才能卖出。",
          "front": "卖空（Short-Selling）的第一步是什么？",
          "question_id": "q_flash_short_selling_steps_v1"
        },
        {
          "back": "将买回的股票归还给券商。",
          "estimated_seconds": 6,
          "explanation": "卖空完成后，必须归还借入的股票以平仓。",
          "front": "卖空（Short-Selling）的最后一步是什么？",
          "question_id": "q_flash_short_selling_steps_v2"
        }
      ]
    },
    {
      "concept_key": "short_selling_risk",
      "coverage_tags": [
        "short_selling_risk"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_short_risk",
      "learning_goal": "学生能识别出卖空的核心风险特征。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "做空与做多风险的不对称性",
      "term_refs": [
        {
          "display": "卖空",
          "en": "Short-Selling"
        }
      ],
      "variants": [
        {
          "back": "亏损没有上限（因为股价理论上可以无限上涨）。",
          "estimated_seconds": 8,
          "explanation": "做多最大亏损是本金，但做空如果股价持续上涨，亏损可以无限大。",
          "front": "做空（Short-Selling）在理论上有什么特殊的风险？",
          "question_id": "q_flash_short_risk_v1"
        },
        {
          "back": "因为做多最大亏损有限（本金），做空亏损理论上无上限。",
          "estimated_seconds": 8,
          "explanation": "股价可以涨到远高于做空价格，导致无限亏损。",
          "front": "为什么做空的风险比做多更大？",
          "question_id": "q_flash_short_risk_v2"
        }
      ]
    },
    {
      "concept_key": "mark_to_market",
      "coverage_tags": [
        "mark_to_market"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_mtm_definition",
      "learning_goal": "学生能回忆逐日盯市的基本含义。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "逐日盯市的核心操作",
      "term_refs": [
        {
          "display": "逐日盯市",
          "en": "Mark-to-Market"
        }
      ],
      "variants": [
        {
          "back": "按当前市场价格计算资产或头寸的价值。",
          "estimated_seconds": 6,
          "explanation": "逐日盯市确保盈亏随市价实时反映。",
          "front": "逐日盯市（Mark-to-Market）是什么意思？",
          "question_id": "q_flash_mtm_definition_v1"
        },
        {
          "back": "未实现盈亏（Unrealized PnL）。",
          "estimated_seconds": 6,
          "explanation": "逐日盯市计算的是当前持仓的浮动盈亏，平仓后才变成已实现盈亏。",
          "front": "逐日盯市（Mark-to-Market）影响的是已实现盈亏还是未实现盈亏？",
          "question_id": "q_flash_mtm_definition_v2"
        }
      ]
    },
    {
      "concept_key": "pnl_formula_long",
      "coverage_tags": [
        "pnl_formula_long"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_pnl_long_formula",
      "learning_goal": "学生能准确回忆做多盈亏公式。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "做多盈亏公式的变量与符号",
      "term_refs": [
        {
          "display": "做多盈亏公式",
          "en": "Long PnL Formula"
        }
      ],
      "variants": [
        {
          "back": "PnL = Quantity × (P_current - P_entry)",
          "estimated_seconds": 8,
          "explanation": "做多盈亏 = 数量 × (当前价 - 买入价)。",
          "front": "做多（Long）的盈亏公式是什么？",
          "question_id": "q_flash_pnl_long_formula_v1"
        },
        {
          "back": "盈利（正数）。",
          "estimated_seconds": 6,
          "explanation": "当前价高于买入价时，公式结果为正值，表示盈利。",
          "front": "在做多盈亏公式 PnL = Quantity × (P_current - P_entry) 中，如果 P_current > P_entry，结果是盈利还是亏损？",
          "question_id": "q_flash_pnl_long_formula_v2"
        }
      ]
    },
    {
      "concept_key": "pnl_formula_short",
      "coverage_tags": [
        "pnl_formula_short"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_pnl_short_formula",
      "learning_goal": "学生能准确回忆做空盈亏公式。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "做空盈亏公式的变量与符号",
      "term_refs": [
        {
          "display": "做空盈亏公式",
          "en": "Short PnL Formula"
        }
      ],
      "variants": [
        {
          "back": "PnL = -1 × Quantity × (P_current - P_entry)",
          "estimated_seconds": 8,
          "explanation": "做空盈亏 = -1 × 数量 × (当前价 - 卖出价)。",
          "front": "做空（Short）的盈亏公式是什么？",
          "question_id": "q_flash_pnl_short_formula_v1"
        },
        {
          "back": "盈利（正数）。",
          "estimated_seconds": 6,
          "explanation": "当前价低于卖出价时，负号乘以负差得到正值，表示盈利。",
          "front": "在做空盈亏公式 PnL = -1 × Quantity × (P_current - P_entry) 中，如果 P_current < P_entry，结果是盈利还是亏损？",
          "question_id": "q_flash_pnl_short_formula_v2"
        }
      ]
    }
  ],
  "lesson_id": "L1",
  "longform_families": [
    {
      "concept_key": "long_short_position",
      "coverage_tags": [
        "long_short_position"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_long_short_compare",
      "learning_goal": "学生能系统比较多头与空头在操作方向、获利逻辑和风险特征上的异同。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "compare_and_contrast",
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
          "estimated_seconds": 90,
          "prompt_blocks": [
            "操作方向",
            "获利逻辑",
            "风险特征"
          ],
          "question_id": "q_long_long_short_compare_v1",
          "reference_answer": [
            "多头头寸：先买入资产，预期价格上涨后卖出获利。最大亏损为全部本金。",
            "空头头寸：先借入资产卖出，预期价格下跌后买回获利。理论亏损无上限，因为股价可以无限上涨。"
          ],
          "rubric_points": [
            "操作方向：多头先买入后卖出；空头先借入卖出后买回。",
            "获利逻辑：多头预期价格上涨获利；空头预期价格下跌获利。",
            "风险特征：多头最大亏损有限（本金）；空头理论亏损无上限。"
          ],
          "stem": "请比较多头头寸（Long Position）和空头头寸（Short Position）在以下三个方面的区别：操作方向、获利逻辑、风险特征。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "做多的操作步骤与获利条件",
            "做空的操作步骤与获利条件",
            "做多的最大风险",
            "做空的最大风险"
          ],
          "question_id": "q_long_long_short_compare_v2",
          "reference_answer": [
            "做多：以当前价格买入股票，等待价格上涨后卖出。获利条件是卖出价高于买入价。最大风险是股价跌至0，损失全部本金。",
            "做空：从券商借入股票并卖出，等待价格下跌后买回归还。获利条件是买回价低于卖出价。最大风险是股价持续上涨，亏损无上限。"
          ],
          "rubric_points": [
            "做多：买入股票，价格上涨后卖出获利。最大风险是损失全部本金。",
            "做空：借入股票卖出，价格下跌后买回获利。最大风险是理论亏损无上限。"
          ],
          "stem": "假设你预期某股票价格将大幅波动，但不确定方向。请分别说明如果你选择做多和做空，你的操作步骤、获利条件以及各自面临的最大风险是什么。"
        }
      ]
    },
    {
      "concept_key": "pnl_formula_long",
      "coverage_tags": [
        "pnl_formula_long",
        "pnl_formula_short"
      ],
      "difficulty": "hard",
      "family_id": "qf_long_pnl_calc_apply",
      "learning_goal": "学生能综合运用做多和做空盈亏公式进行多步计算和解释。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "worked_example",
      "term_refs": [
        {
          "display": "做多盈亏公式",
          "en": "Long PnL Formula"
        },
        {
          "display": "做空盈亏公式",
          "en": "Short PnL Formula"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "股票A的盈亏计算",
            "股票B的盈亏计算",
            "总盈亏",
            "做空头寸的风险分析"
          ],
          "question_id": "q_long_pnl_calc_apply_v1",
          "reference_answer": [
            "股票A盈亏 = 300 × (25 - 20) = 1500元，盈利。",
            "股票B盈亏 = -1 × 200 × (45 - 50) = 1000元，盈利。",
            "总盈亏 = 1500 + 1000 = 2500元，盈利。",
            "如果股票B涨到55元，做空头寸亏损 = -1 × 200 × (55 - 50) = -1000元。做空的理论亏损无上限，股价越高亏损越大。"
          ],
          "rubric_points": [
            "股票A盈亏 = 300 × (25 - 20) = 1500元（盈利）",
            "股票B盈亏 = -1 × 200 × (45 - 50) = 1000元（盈利）",
            "总盈亏 = 1500 + 1000 = 2500元（盈利）",
            "如果股票B涨到55元，做空头寸亏损 = -1 × 200 × (55 - 50) = -1000元。若继续上涨，亏损无上限。"
          ],
          "stem": "你同时进行了两笔交易：\n1. 以每股20元买入300股股票A。\n2. 以每股50元卖空200股股票B。\n现在股票A的价格为25元，股票B的价格为45元。\n请计算：\n(1) 股票A的盈亏是多少？\n(2) 股票B的盈亏是多少？\n(3) 总盈亏是多少？\n(4) 如果股票B的价格涨到55元，你的做空头寸会面临什么风险？"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "股票X的盈亏计算",
            "股票Y的盈亏计算",
            "总盈亏",
            "多头头寸的风险分析"
          ],
          "question_id": "q_long_pnl_calc_apply_v2",
          "reference_answer": [
            "股票X盈亏 = 100 × (90 - 100) = -1000元，亏损。",
            "股票Y盈亏 = -1 × 150 × (70 - 80) = 1500元，盈利。",
            "总盈亏 = -1000 + 1500 = 500元，盈利。",
            "如果股票X跌到50元，多头头寸亏损 = 100 × (50 - 100) = -5000元。多头最大亏损为全部本金10000元（股价跌至0）。"
          ],
          "rubric_points": [
            "股票X盈亏 = 100 × (90 - 100) = -1000元（亏损）",
            "股票Y盈亏 = -1 × 150 × (70 - 80) = 1500元（盈利）",
            "总盈亏 = -1000 + 1500 = 500元（盈利）",
            "如果股票X跌到50元，多头头寸亏损 = 100 × (50 - 100) = -5000元。最大亏损为全部本金10000元。"
          ],
          "stem": "你进行了以下交易：\n1. 以每股100元买入100股股票X。\n2. 以每股80元卖空150股股票Y。\n现在股票X的价格为90元，股票Y的价格为70元。\n请计算：\n(1) 股票X的盈亏是多少？\n(2) 股票Y的盈亏是多少？\n(3) 总盈亏是多少？\n(4) 如果股票X的价格跌到50元，你的多头头寸会面临什么风险？"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "long_short_position",
      "coverage_tags": [
        "long_short_position"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_long_short_identify",
      "learning_goal": "学生能在具体情境中判断交易方向属于多头还是空头。",
      "linked_steps": [
        "step4"
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
          "estimated_seconds": 15,
          "explanation": "买入资产预期价格上涨，属于多头头寸。",
          "options": [
            "多头头寸",
            "空头头寸",
            "套利头寸",
            "对冲头寸"
          ],
          "question_id": "q_quiz_long_short_identify_v1",
          "stem": "小王预期某股票价格将上涨，于是买入100股。这属于什么类型的头寸？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "借入卖出，预期价格下跌后买回，属于空头头寸。",
          "options": [
            "多头头寸",
            "空头头寸",
            "套利头寸",
            "对冲头寸"
          ],
          "question_id": "q_quiz_long_short_identify_v2",
          "stem": "小李认为某股票价格会下跌，于是先借入股票卖出。这属于什么类型的头寸？"
        }
      ]
    },
    {
      "concept_key": "short_selling_process",
      "coverage_tags": [
        "short_selling_process"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_short_selling_order",
      "learning_goal": "学生能正确排列卖空的四个步骤。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "ordering",
      "term_refs": [
        {
          "display": "卖空",
          "en": "Short-Selling"
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
          "estimated_seconds": 20,
          "explanation": "正确的顺序是：借入股票 → 卖出股票 → 买回股票 → 归还股票。",
          "options": [
            "A. 借入股票",
            "B. 卖出股票",
            "C. 买回股票",
            "D. 归还股票"
          ],
          "question_id": "q_quiz_short_selling_order_v1",
          "stem": "请将以下卖空（Short-Selling）的步骤按正确顺序排列："
        },
        {
          "answer": [
            1,
            2,
            3,
            0
          ],
          "estimated_seconds": 20,
          "explanation": "正确的顺序是：借入股票 → 卖出股票 → 买回股票 → 归还股票。",
          "options": [
            "A. 归还股票给券商",
            "B. 从券商借入股票",
            "C. 以当前市价卖出股票",
            "D. 以更低价格买回股票"
          ],
          "question_id": "q_quiz_short_selling_order_v2",
          "stem": "请将以下卖空（Short-Selling）的步骤按正确顺序排列："
        }
      ]
    },
    {
      "concept_key": "short_selling_risk",
      "coverage_tags": [
        "short_selling_risk"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_short_risk_choice",
      "learning_goal": "学生能识别做空的核心风险特征。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "卖空",
          "en": "Short-Selling"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "做空时，如果股价持续上涨，亏损可以无限大，因此理论亏损没有上限。",
          "options": [
            "做空的最大亏损等于本金",
            "做空的理论亏损没有上限",
            "做空的风险比做多小",
            "做空不会触发追加保证金"
          ],
          "question_id": "q_quiz_short_risk_choice_v1",
          "stem": "以下关于做空（Short-Selling）风险的描述，哪一项是正确的？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "做多最大亏损是全部本金，但做空如果股价上涨，亏损可以无限大。",
          "options": [
            "因为做空需要更多本金",
            "因为做空只能在大盘下跌时操作",
            "因为做空的理论亏损无上限，而做多最大亏损有限",
            "因为做空的手续费更高"
          ],
          "question_id": "q_quiz_short_risk_choice_v2",
          "stem": "为什么做空（Short-Selling）被认为比做多风险更大？"
        }
      ]
    },
    {
      "concept_key": "mark_to_market",
      "coverage_tags": [
        "mark_to_market"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_mtm_apply",
      "learning_goal": "学生能理解逐日盯市对盈亏计算的影响。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "逐日盯市",
          "en": "Mark-to-Market"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "未实现盈亏 = 100 × (55 - 50) = 500元，为盈利。",
          "options": [
            "盈利500元",
            "亏损500元",
            "盈利5500元",
            "亏损5500元"
          ],
          "question_id": "q_quiz_mtm_apply_v1",
          "stem": "你以每股50元买入100股股票，当前市价为55元。根据逐日盯市（Mark-to-Market），你的未实现盈亏是多少？"
        },
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "未实现盈亏 = -1 × 200 × (70 - 80) = 2000元，为盈利。",
          "options": [
            "盈利2000元",
            "亏损2000元",
            "盈利14000元",
            "亏损14000元"
          ],
          "question_id": "q_quiz_mtm_apply_v2",
          "stem": "你以每股80元卖空200股股票，当前市价为70元。根据逐日盯市（Mark-to-Market），你的未实现盈亏是多少？"
        }
      ]
    },
    {
      "concept_key": "pnl_formula_long",
      "coverage_tags": [
        "pnl_formula_long"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_pnl_long_calc",
      "learning_goal": "学生能用做多盈亏公式进行简单计算。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "做多盈亏公式",
          "en": "Long PnL Formula"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "PnL = 500 × (35 - 30) = 2500元，盈利。",
          "options": [
            "盈利2500元",
            "亏损2500元",
            "盈利17500元",
            "亏损17500元"
          ],
          "question_id": "q_quiz_pnl_long_calc_v1",
          "stem": "你以每股30元买入500股股票，现在股价涨到35元。你的盈亏是多少？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "PnL = 300 × (110 - 120) = -3000元，亏损3000元。",
          "options": [
            "盈利3000元",
            "亏损3000元",
            "盈利36000元",
            "亏损36000元"
          ],
          "question_id": "q_quiz_pnl_long_calc_v2",
          "stem": "你以每股120元买入300股股票，现在股价跌到110元。你的盈亏是多少？"
        }
      ]
    },
    {
      "concept_key": "pnl_formula_short",
      "coverage_tags": [
        "pnl_formula_short"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_pnl_short_calc",
      "learning_goal": "学生能用做空盈亏公式进行简单计算。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "做空盈亏公式",
          "en": "Short PnL Formula"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "PnL = -1 × 200 × (50 - 60) = 2000元，盈利。",
          "options": [
            "盈利2000元",
            "亏损2000元",
            "盈利10000元",
            "亏损10000元"
          ],
          "question_id": "q_quiz_pnl_short_calc_v1",
          "stem": "你以每股60元卖空200股股票，现在股价跌到50元。你的盈亏是多少？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "PnL = -1 × 150 × (45 - 40) = -750元，亏损750元。",
          "options": [
            "盈利750元",
            "亏损750元",
            "盈利6000元",
            "亏损6000元"
          ],
          "question_id": "q_quiz_pnl_short_calc_v2",
          "stem": "你以每股40元卖空150股股票，现在股价涨到45元。你的盈亏是多少？"
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
,
{
  "coverage_map": [
    {
      "coverage_tag": "market_trend_algo_trading",
      "covered_by": [
        "qf_flash_market_trend",
        "qf_quiz_market_trend"
      ],
      "description": "算法交易的市场趋势：美国股市60%交易量由算法完成，欧洲40%，外汇25%，期权20%；市场CAGR 12.7%至2028年320亿美元；北美最大，亚太增长最快。"
    },
    {
      "coverage_tag": "robo_trading_vs_robo_advisory",
      "covered_by": [
        "qf_flash_robo_compare",
        "qf_quiz_robo_compare",
        "qf_long_robo_compare"
      ],
      "description": "机器人交易与机器人投顾的对比：机器人交易全自动（决策+执行），机器人投顾半自动（仅建议，不执行）。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "market_trend_algo_trading",
      "coverage_tags": [
        "market_trend_algo_trading"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_market_trend",
      "learning_goal": "学生能准确回忆算法交易在不同市场的渗透率及市场增长趋势的关键数据。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "要求学生主动提取算法交易在美国、欧洲、外汇、期权市场的交易量占比，以及市场年复合增长率和区域分布。",
      "term_refs": [
        {
          "display": "算法交易",
          "en": "Algorithmic Trading"
        }
      ],
      "variants": [
        {
          "back": "超过60%",
          "estimated_seconds": 5,
          "explanation": "材料明确指出算法交易策略占美国股市交易量的约60%。",
          "front": "算法交易在美国股市的交易量占比大约是多少？",
          "question_id": "q_flash_market_trend_v1"
        },
        {
          "back": "40%",
          "estimated_seconds": 5,
          "explanation": "材料指出算法交易占欧洲股市交易量的约40%。",
          "front": "算法交易在欧洲股市的交易量占比大约是多少？",
          "question_id": "q_flash_market_trend_v2"
        },
        {
          "back": "25%",
          "estimated_seconds": 5,
          "explanation": "材料指出算法交易占外汇市场交易量的约25%。",
          "front": "算法交易在外汇市场的交易量占比大约是多少？",
          "question_id": "q_flash_market_trend_v3"
        },
        {
          "back": "12.7%",
          "estimated_seconds": 5,
          "explanation": "材料指出市场预计以CAGR 12.7%增长至2028年的320亿美元。",
          "front": "算法交易市场预计到2028年的年复合增长率（CAGR）是多少？",
          "question_id": "q_flash_market_trend_v4"
        }
      ]
    },
    {
      "concept_key": "robo_trading_vs_robo_advisory",
      "coverage_tags": [
        "robo_trading_vs_robo_advisory"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_robo_compare",
      "learning_goal": "学生能准确区分机器人交易和机器人投顾在自动化程度和执行能力上的核心差异。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "core_difference",
      "retrieval_focus": "要求学生主动提取机器人交易和机器人投顾在决策、执行、自动化程度和输出上的关键区别。",
      "term_refs": [
        {
          "display": "机器人交易",
          "en": "Robo-Trading"
        },
        {
          "display": "机器人投顾",
          "en": "Robo-Advisory"
        }
      ],
      "variants": [
        {
          "back": "机器人交易自动执行，机器人投顾不执行。",
          "estimated_seconds": 8,
          "explanation": "材料中表格显示机器人交易有执行功能（✓），机器人投顾没有（×）。",
          "front": "机器人交易和机器人投顾在执行交易方面的关键区别是什么？",
          "question_id": "q_flash_robo_compare_v1"
        },
        {
          "back": "机器人交易是全自动的，机器人投顾是半自动的。",
          "estimated_seconds": 8,
          "explanation": "材料明确指出机器人交易是全自动的，机器人投顾是半自动的。",
          "front": "机器人交易和机器人投顾在自动化程度上的区别是什么？",
          "question_id": "q_flash_robo_compare_v2"
        },
        {
          "back": "股票建议、市场总结、风险提醒。",
          "estimated_seconds": 8,
          "explanation": "材料中表格显示机器人投顾的输出包括股票建议、市场总结和风险提醒。",
          "front": "机器人投顾的主要输出是什么？",
          "question_id": "q_flash_robo_compare_v3"
        }
      ]
    }
  ],
  "lesson_id": "L1",
  "longform_families": [
    {
      "concept_key": "robo_trading_vs_robo_advisory",
      "coverage_tags": [
        "robo_trading_vs_robo_advisory"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_robo_compare",
      "learning_goal": "学生能系统性地比较和对比机器人交易和机器人投顾，涵盖决策、执行、自动化程度和输出。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "compare_and_contrast",
      "term_refs": [
        {
          "display": "机器人交易",
          "en": "Robo-Trading"
        },
        {
          "display": "机器人投顾",
          "en": "Robo-Advisory"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "决策能力",
            "执行能力",
            "自动化程度",
            "输出内容"
          ],
          "question_id": "q_long_robo_compare_v1",
          "reference_answer": [
            "决策：两者都能进行决策。",
            "执行：机器人交易自动执行交易，机器人投顾不执行。",
            "自动化程度：机器人交易是全自动的，机器人投顾是半自动的。",
            "输出：机器人交易的输出是执行交易信号，机器人投顾的输出是股票建议、市场总结和风险提醒。"
          ],
          "rubric_points": [
            "指出两者都能做决策",
            "指出机器人交易能自动执行，机器人投顾不能",
            "指出机器人交易是全自动，机器人投顾是半自动",
            "指出机器人交易的输出是执行交易信号，机器人投顾的输出是建议、总结和提醒"
          ],
          "stem": "请比较机器人交易和机器人投顾，从决策、执行、自动化程度和输出四个方面进行说明。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "适合的投资者类型",
            "核心差异：决策与执行",
            "核心差异：自动化程度",
            "核心差异：输出形式"
          ],
          "question_id": "q_long_robo_compare_v2",
          "reference_answer": [
            "机器人交易适合希望完全自动化、无需人工干预的投资者，因为它全自动地决策和执行。",
            "机器人投顾适合需要专业建议但希望保留最终决策权的投资者，因为它提供建议但不自动执行。",
            "核心差异在于执行：机器人交易自动执行，机器人投顾不执行。",
            "自动化程度：机器人交易全自动，机器人投顾半自动。",
            "输出形式：机器人交易输出执行信号，机器人投顾输出股票建议、市场总结和风险提醒。"
          ],
          "rubric_points": [
            "指出机器人交易适合希望完全自动化交易的投资者，机器人投顾适合需要建议但希望自己决策的投资者",
            "指出机器人交易自动执行，机器人投顾不执行",
            "指出机器人交易全自动，机器人投顾半自动",
            "指出机器人交易输出执行信号，机器人投顾输出建议和提醒"
          ],
          "stem": "假设你是一位投资者，请解释为什么机器人交易和机器人投顾适合不同类型的投资者，并分别说明它们的核心差异。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "market_trend_algo_trading",
      "coverage_tags": [
        "market_trend_algo_trading"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_market_trend",
      "learning_goal": "学生能在测验情境下辨析算法交易的市场渗透率和增长趋势。",
      "linked_steps": [
        "step2"
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
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "算法交易占美国股市交易量的60%，高于欧洲（40%）、外汇（25%）和期权（20%）。",
          "options": [
            "美国股市",
            "欧洲股市",
            "外汇市场",
            "美国期权市场"
          ],
          "question_id": "q_quiz_market_trend_v1",
          "stem": "根据市场数据，算法交易在以下哪个市场的交易量占比最高？"
        },
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "材料指出北美是最大市场，亚太地区增长最快。",
          "options": [
            "北美市场最大，亚太地区增长最快",
            "欧洲市场最大，北美增长最快",
            "亚太市场最大，欧洲增长最快",
            "北美市场最大，欧洲增长最快"
          ],
          "question_id": "q_quiz_market_trend_v2",
          "stem": "关于算法交易市场的增长趋势，以下哪项描述是正确的？"
        }
      ]
    },
    {
      "concept_key": "robo_trading_vs_robo_advisory",
      "coverage_tags": [
        "robo_trading_vs_robo_advisory"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_robo_compare",
      "learning_goal": "学生能在测验情境下准确辨析机器人交易和机器人投顾的关键区别。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "机器人交易",
          "en": "Robo-Trading"
        },
        {
          "display": "机器人投顾",
          "en": "Robo-Advisory"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "机器人交易是全自动的，包括执行；机器人投顾只提供建议，不自动执行。",
          "options": [
            "机器人交易自动执行，机器人投顾不执行",
            "机器人投顾更赚钱",
            "机器人交易只用于股票",
            "机器人投顾是全自动的"
          ],
          "question_id": "q_quiz_robo_compare_v1",
          "stem": "以下哪项是机器人交易和机器人投顾的关键区别？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "机器人投顾是半自动的，提供建议、市场总结和风险提醒，但不执行交易。",
          "options": [
            "它是全自动的，包括决策和执行",
            "它提供建议，但不自动执行交易",
            "它只用于外汇市场",
            "它的输出是执行交易信号"
          ],
          "question_id": "q_quiz_robo_compare_v2",
          "stem": "关于机器人投顾，以下哪项描述是正确的？"
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
,
{
  "coverage_map": [
    {
      "coverage_tag": "order_book_concept",
      "covered_by": [
        "qf_flash_order_book",
        "qf_quiz_order_book"
      ],
      "description": "订单簿的定义与基本结构（实时更新的买卖订单列表）"
    },
    {
      "coverage_tag": "bid_ask_price",
      "covered_by": [
        "qf_flash_bid_ask",
        "qf_quiz_bid_ask"
      ],
      "description": "买入价（买方最高出价）与卖出价（卖方最低要价）的定义"
    },
    {
      "coverage_tag": "bid_ask_spread",
      "covered_by": [
        "qf_flash_spread",
        "qf_quiz_spread",
        "qf_long_spread"
      ],
      "description": "买卖价差（卖出价-买入价）及其与流动性的关系"
    },
    {
      "coverage_tag": "market_order",
      "covered_by": [
        "qf_flash_market_order",
        "qf_quiz_order_types"
      ],
      "description": "市价单：按当前最优价格立即成交，保证成交但不保证价格"
    },
    {
      "coverage_tag": "limit_order",
      "covered_by": [
        "qf_flash_limit_order",
        "qf_quiz_order_types"
      ],
      "description": "限价单：指定价格，保证价格但不保证成交"
    },
    {
      "coverage_tag": "stop_order",
      "covered_by": [
        "qf_flash_stop_order",
        "qf_quiz_order_types"
      ],
      "description": "止损单：价格触及设定水平后自动变成市价单"
    },
    {
      "coverage_tag": "slippage",
      "covered_by": [
        "qf_flash_slippage",
        "qf_quiz_slippage",
        "qf_long_slippage"
      ],
      "description": "滑点：预期成交价与实际成交价之间的差距"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "order_book_concept",
      "coverage_tags": [
        "order_book_concept"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_order_book",
      "learning_goal": "学生能准确说出订单簿的定义及其核心组成部分。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "订单簿的定义与基本结构",
      "term_refs": [
        {
          "display": "订单簿",
          "en": "Order Book"
        }
      ],
      "variants": [
        {
          "back": "实时更新的买卖订单列表，按价格排序。",
          "estimated_seconds": 8,
          "explanation": "订单簿是市场撮合交易的核心工具，实时列出所有买方和卖方的订单。",
          "front": "订单簿（Order Book）是什么？",
          "question_id": "q_flash_order_book_v1"
        },
        {
          "back": "买方（出价）和卖方（要价）。",
          "estimated_seconds": 6,
          "explanation": "订单簿分为买方队列（Bid Book）和卖方队列（Ask Book）。",
          "front": "订单簿中主要包含哪两方？",
          "question_id": "q_flash_order_book_v2"
        }
      ]
    },
    {
      "concept_key": "bid_ask_price",
      "coverage_tags": [
        "bid_ask_price"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_bid_ask",
      "learning_goal": "学生能区分买入价和卖出价的定义。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "买入价与卖出价的定义",
      "term_refs": [
        {
          "display": "买入价",
          "en": "Bid Price"
        },
        {
          "display": "卖出价",
          "en": "Ask Price"
        }
      ],
      "variants": [
        {
          "back": "买方愿意支付的最高价格。",
          "estimated_seconds": 6,
          "explanation": "买入价是买方队列中的最高出价。",
          "front": "买入价（Bid Price）指的是什么？",
          "question_id": "q_flash_bid_ask_v1"
        },
        {
          "back": "卖方愿意接受的最低价格。",
          "estimated_seconds": 6,
          "explanation": "卖出价是卖方队列中的最低要价。",
          "front": "卖出价（Ask Price）指的是什么？",
          "question_id": "q_flash_bid_ask_v2"
        }
      ]
    },
    {
      "concept_key": "bid_ask_spread",
      "coverage_tags": [
        "bid_ask_spread"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_spread",
      "learning_goal": "学生能说出买卖价差的定义及其与流动性的关系。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "买卖价差的定义与流动性关系",
      "term_refs": [
        {
          "display": "买卖价差",
          "en": "Bid-Ask Spread"
        }
      ],
      "variants": [
        {
          "back": "卖出价减去买入价。",
          "estimated_seconds": 5,
          "explanation": "价差 = Ask Price - Bid Price。",
          "front": "买卖价差（Bid-Ask Spread）如何计算？",
          "question_id": "q_flash_spread_v1"
        },
        {
          "back": "流动性越好。",
          "estimated_seconds": 5,
          "explanation": "较小的价差通常意味着市场深度好，交易成本低。",
          "front": "价差越小，说明市场流动性如何？",
          "question_id": "q_flash_spread_v2"
        }
      ]
    },
    {
      "concept_key": "market_order",
      "coverage_tags": [
        "market_order"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_market_order",
      "learning_goal": "学生能说出市价单的核心特征：保证成交，不保证价格。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "市价单的保证与风险",
      "term_refs": [
        {
          "display": "市价单",
          "en": "Market Order"
        }
      ],
      "variants": [
        {
          "back": "保证立即成交。",
          "estimated_seconds": 5,
          "explanation": "市价单以当前最优价格执行，确保订单被立即匹配。",
          "front": "市价单（Market Order）保证什么？",
          "question_id": "q_flash_market_order_v1"
        },
        {
          "back": "不保证成交价格。",
          "estimated_seconds": 5,
          "explanation": "由于市场波动，实际成交价可能与预期不同。",
          "front": "市价单（Market Order）不保证什么？",
          "question_id": "q_flash_market_order_v2"
        }
      ]
    },
    {
      "concept_key": "limit_order",
      "coverage_tags": [
        "limit_order"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_limit_order",
      "learning_goal": "学生能说出限价单的核心特征：保证价格，不保证成交。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "限价单的保证与风险",
      "term_refs": [
        {
          "display": "限价单",
          "en": "Limit Order"
        }
      ],
      "variants": [
        {
          "back": "保证成交价格（或更优）。",
          "estimated_seconds": 5,
          "explanation": "限价单只在指定价格或更优价格时成交。",
          "front": "限价单（Limit Order）保证什么？",
          "question_id": "q_flash_limit_order_v1"
        },
        {
          "back": "不保证一定成交。",
          "estimated_seconds": 5,
          "explanation": "如果市场价格未达到指定价格，订单可能不会被执行。",
          "front": "限价单（Limit Order）不保证什么？",
          "question_id": "q_flash_limit_order_v2"
        }
      ]
    },
    {
      "concept_key": "stop_order",
      "coverage_tags": [
        "stop_order"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_stop_order",
      "learning_goal": "学生能说出止损单的触发机制。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "止损单的触发与转换",
      "term_refs": [
        {
          "display": "止损单",
          "en": "Stop Order"
        }
      ],
      "variants": [
        {
          "back": "变成市价单。",
          "estimated_seconds": 6,
          "explanation": "一旦触发，止损单自动转换为市价单以确保执行。",
          "front": "止损单（Stop Order）在价格触及设定水平后会变成什么？",
          "question_id": "q_flash_stop_order_v1"
        },
        {
          "back": "止损或突破买入。",
          "estimated_seconds": 6,
          "explanation": "用于限制亏损或捕捉突破行情。",
          "front": "止损单（Stop Order）常用于什么场景？",
          "question_id": "q_flash_stop_order_v2"
        }
      ]
    },
    {
      "concept_key": "slippage",
      "coverage_tags": [
        "slippage"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_slippage",
      "learning_goal": "学生能说出滑点的定义。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "滑点的定义",
      "term_refs": [
        {
          "display": "滑点",
          "en": "Slippage"
        }
      ],
      "variants": [
        {
          "back": "预期成交价与实际成交价之间的差距。",
          "estimated_seconds": 6,
          "explanation": "滑点通常发生在市场波动大或流动性不足时。",
          "front": "滑点（Slippage）指的是什么？",
          "question_id": "q_flash_slippage_v1"
        },
        {
          "back": "1元。",
          "estimated_seconds": 5,
          "explanation": "滑点 = 实际成交价 - 预期成交价 = 51 - 50 = 1元。",
          "front": "如果预期50元买入，实际成交在51元，滑点是多少？",
          "question_id": "q_flash_slippage_v2"
        }
      ]
    }
  ],
  "lesson_id": "L1",
  "longform_families": [
    {
      "concept_key": "bid_ask_spread",
      "coverage_tags": [
        "bid_ask_spread"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_spread",
      "learning_goal": "学生能解释买卖价差的计算、含义及其与市场流动性的关系。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "买卖价差",
          "en": "Bid-Ask Spread"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "定义买卖价差",
            "给出一个计算示例",
            "解释价差大小与流动性的关系"
          ],
          "question_id": "q_long_spread_v1",
          "reference_answer": [
            "买卖价差是卖出价（Ask Price）与买入价（Bid Price）之间的差额。",
            "例如，买入价为24元，卖出价为25元，则价差为1元。",
            "价差越小，说明市场流动性越好，交易成本越低；价差越大，说明市场流动性越差，交易成本越高。"
          ],
          "rubric_points": [
            "正确说出买卖价差 = 卖出价 - 买入价",
            "能给出一个自洽的数值例子",
            "能说明价差越小流动性越好，价差越大流动性越差"
          ],
          "stem": "请解释什么是买卖价差（Bid-Ask Spread），并说明它如何反映市场流动性。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "计算价差",
            "判断流动性",
            "解释判断依据"
          ],
          "question_id": "q_long_spread_v2",
          "reference_answer": [
            "买卖价差 = 52元 - 50元 = 2元。",
            "2元的价差相对较大，表明该股票的流动性较差。",
            "较大的价差意味着买卖双方的价格分歧较大，交易成本较高，可能难以快速以理想价格成交。"
          ],
          "rubric_points": [
            "正确计算价差为2元",
            "能判断流动性相对较差",
            "能解释较大的价差意味着较高的交易成本和较低的流动性"
          ],
          "stem": "假设某股票的买入价为50元，卖出价为52元。请计算买卖价差，并分析该股票的流动性状况。"
        }
      ]
    },
    {
      "concept_key": "slippage",
      "coverage_tags": [
        "slippage"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_slippage",
      "learning_goal": "学生能解释滑点的定义、产生原因及其对交易成本的影响。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "滑点",
          "en": "Slippage"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "定义滑点",
            "列举至少两种容易发生滑点的情况",
            "解释滑点对交易成本的影响"
          ],
          "question_id": "q_long_slippage_v1",
          "reference_answer": [
            "滑点是指预期成交价与实际成交价之间的差异。",
            "滑点容易发生在市场波动剧烈时，或者当使用市价单且市场深度不足时。",
            "滑点会导致实际交易成本高于预期，例如预期50元买入，实际51元成交，多付的1元就是滑点成本。"
          ],
          "rubric_points": [
            "正确说出滑点是预期成交价与实际成交价之间的差距",
            "能提到高波动时期或使用市价单时容易发生",
            "能说明滑点会增加实际交易成本"
          ],
          "stem": "请解释什么是滑点（Slippage），并说明在什么情况下容易发生滑点。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "识别这一现象为滑点",
            "计算滑点金额",
            "分析对总交易成本的影响"
          ],
          "question_id": "q_long_slippage_v2",
          "reference_answer": [
            "这一现象是滑点：预期成交价100元，实际成交价101元。",
            "滑点金额为1元/股，总滑点成本为100股 × 1元 = 100元。",
            "原本预期花费10000元，实际花费10100元，交易成本因滑点增加了100元。"
          ],
          "rubric_points": [
            "正确识别为滑点",
            "正确计算滑点金额为1元/股，总计100元",
            "能说明滑点导致总成本增加100元"
          ],
          "stem": "假设你设置算法在股价跌至100元时买入100股，但由于市场波动，实际成交价为101元。请分析这一现象，并说明其对交易成本的影响。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "order_book_concept",
      "coverage_tags": [
        "order_book_concept"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_order_book",
      "learning_goal": "学生能在选择题中识别订单簿的核心功能。",
      "linked_steps": [
        "step3"
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
          "estimated_seconds": 15,
          "explanation": "订单簿是实时更新的买卖订单列表，用于撮合交易。",
          "options": [
            "记录历史成交价格",
            "实时列出买卖订单并撮合交易",
            "计算投资者的盈亏",
            "提供公司财务报表"
          ],
          "question_id": "q_quiz_order_book_v1",
          "stem": "订单簿（Order Book）的主要作用是什么？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "订单簿按价格排序，分为买方队列和卖方队列。",
          "options": [
            "只有卖方订单",
            "按时间顺序排列的所有订单",
            "按价格排序的买方和卖方订单列表",
            "随机排列的订单列表"
          ],
          "question_id": "q_quiz_order_book_v2",
          "stem": "以下哪项最能描述订单簿的结构？"
        }
      ]
    },
    {
      "concept_key": "bid_ask_price",
      "coverage_tags": [
        "bid_ask_price"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_bid_ask",
      "learning_goal": "学生能在具体情境中区分买入价和卖出价。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "买入价",
          "en": "Bid Price"
        },
        {
          "display": "卖出价",
          "en": "Ask Price"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "买入价是买方愿意支付的最高价格。",
          "options": [
            "卖方愿意接受的最低价格",
            "买方愿意支付的最高价格",
            "最近一笔成交的价格",
            "市场平均价格"
          ],
          "question_id": "q_quiz_bid_ask_v1",
          "stem": "在订单簿中，买入价（Bid Price）是指："
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "卖出价是卖方愿意接受的最低价格。",
          "options": [
            "买方愿意支付的最高价格",
            "卖方愿意接受的最低价格",
            "最近一笔成交的价格",
            "市场平均价格"
          ],
          "question_id": "q_quiz_bid_ask_v2",
          "stem": "在订单簿中，卖出价（Ask Price）是指："
        }
      ]
    },
    {
      "concept_key": "bid_ask_spread",
      "coverage_tags": [
        "bid_ask_spread"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_spread",
      "learning_goal": "学生能计算买卖价差并理解其与流动性的关系。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "买卖价差",
          "en": "Bid-Ask Spread"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "买卖价差 = 卖出价 - 买入价 = 25 - 24 = 1元。",
          "options": [
            "0.5元",
            "1元",
            "24元",
            "25元"
          ],
          "question_id": "q_quiz_spread_v1",
          "stem": "某股票的买入价为24元，卖出价为25元，买卖价差是多少？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "较小的价差通常意味着市场流动性更好，交易成本更低。",
          "options": [
            "价差越大，流动性越好",
            "价差越小，流动性越好",
            "价差与流动性无关",
            "价差等于成交量"
          ],
          "question_id": "q_quiz_spread_v2",
          "stem": "以下关于买卖价差的说法，哪项是正确的？"
        }
      ]
    },
    {
      "concept_key": "order_types",
      "coverage_tags": [
        "market_order",
        "limit_order",
        "stop_order"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_order_types",
      "learning_goal": "学生能根据交易需求选择合适的订单类型。",
      "linked_steps": [
        "step3"
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
          "estimated_seconds": 15,
          "explanation": "市价单保证立即成交，但不保证价格。",
          "options": [
            "市价单",
            "限价单",
            "止损单",
            "以上都不是"
          ],
          "question_id": "q_quiz_order_types_v1",
          "stem": "如果你希望立即买入股票，且对价格不敏感，应该使用哪种订单？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "限价单允许你指定一个最高买入价格，只有达到或低于该价格时才成交。",
          "options": [
            "市价单",
            "限价单",
            "止损单",
            "以上都不是"
          ],
          "question_id": "q_quiz_order_types_v2",
          "stem": "如果你希望以不高于50元的价格买入股票，应该使用哪种订单？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "止损单在触发后转换为市价单以确保执行。",
          "options": [
            "市价单",
            "限价单",
            "止损单",
            "以上都不是"
          ],
          "question_id": "q_quiz_order_types_v3",
          "stem": "哪种订单在价格触及设定水平后会自动变成市价单？"
        }
      ]
    },
    {
      "concept_key": "slippage",
      "coverage_tags": [
        "slippage"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_slippage",
      "learning_goal": "学生能识别滑点产生的原因和影响。",
      "linked_steps": [
        "step3"
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
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "高波动时使用市价单，实际成交价可能与预期有较大差距，导致滑点。",
          "options": [
            "市场流动性充足",
            "使用限价单",
            "市场波动剧烈时使用市价单",
            "交易量很小"
          ],
          "question_id": "q_quiz_slippage_v1",
          "stem": "以下哪种情况最可能导致滑点？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "滑点意味着实际成交价不如预期，增加了交易成本。",
          "options": [
            "总是增加利润",
            "可能导致实际交易成本高于预期",
            "只影响限价单",
            "不影响交易结果"
          ],
          "question_id": "q_quiz_slippage_v2",
          "stem": "滑点对交易者的影响是什么？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L1: Algorithmic trading basics and financial markets - Trading mechanism and order matching",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "L1 step3: 交易机制与订单簿",
    "plain_text": "pipeline/1-plain/L1/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L1: Algorithmic trading basics and financial markets - Trading mechanism and order matching"
  },
  "target_language": "zh-CN"
}

]
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
