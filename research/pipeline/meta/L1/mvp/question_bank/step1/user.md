请基于以下 lesson 材料，生成一个结构化题库 JSON。

目标语言：
zh-CN

lesson_id：
L1

要求：
- 同时生成 `flashcard_families`、`quiz_families` 和 `longform_families`
- 题目必须只关联到当前 step：`step1`
- 所有 family 和 variant 的 `linked_steps` 都必须等于 `["step1"]`
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

<CURRENT_STEP_ID>
step1
</CURRENT_STEP_ID>

<CURRENT_STEP>
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
}

</CURRENT_STEP>

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

请直接输出 JSON，不要加解释。
