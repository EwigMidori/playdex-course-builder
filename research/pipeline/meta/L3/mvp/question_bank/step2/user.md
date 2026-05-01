请基于以下 lesson 材料，生成一个结构化题库 JSON。

目标语言：
zh-CN

lesson_id：
L3

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
# L3: Building backtest framework and rule-based trading strategy
Course Code: COMP7415
# Agenda
• Introduction to Backtesting
- Common Candlestick Patterns
Data Cleaning Techniques
- Implement an MA Cross Strategy on
- Excel
- Python
ALGOGENE
- Common Backtest Pitfalls
Exercise: RSI Strategy
# What is Backtesting?
• Definition: Simulating a trading strategy using historical data
- Purpose: Validate the effectiveness of a trading strategy before live trading
Key Procedures:
1. Data Collection
2. Data Cleaning
3. Strategy Implementation
4. Performance Evaluation
# Backtesting is just like…
![](images/525dbb9750f148aaf4318653112473213222fd1da61b1114dd87086efdcffd3e.jpg)
# Why data cleaning?
- Remove data entry errors or system bugs from data provider
- Increase data quality and consistency
- Easier for data management
- Produces a more accurate statistical model
- Better trade decision making
- Potential data issues:
- Missing values
- Duplicated records
Data contains incorrect logics
# Algo Trading Lifecycle
![](images/eadd6f08094a3cade84ca188af3f43415558df388d1de9774ac77f29374cda95.jpg)
# Candlestick
# Introduction to Candlestick
- A candlestick chart (also called K-line) is a type of financial chart that displays the price movement of an asset over time.
It summarizes OHLC data in a single chart
- Open: The price at the start of the time period.
- Close: The price at the end of the time period.
High: The highest price during the period.
- Low: The lowest price during the period.
- OHLC data is aggregated in different time intervals (eg. 1-min, 1 hour, 1-day)
- In Technical Analysis, a candlestick pattern could provide indication on the future direction.
# Understanding Candlestick Components
# - Components:
- Body: The area between the opening and closing prices.
- Wicks: Lines extending above and below the body, representing the high and low prices.
# - Visual display:
- Bullish Candle: Close > Open (often colored green or white)
• Bearish Candle: Close < Open (often colored red or black)
![](images/a6fccb6cb2b229b6494f629e95a068ae057b8742065c1b4687634aa1d4a2283d.jpg)
![](images/32aa7f44ab285e94b336d6d3a900f142f6de7fb7a2825f72524baa70e58182da.jpg)
# Common Candlestick Patterns
1. Doji
2. Hammer
3. Shooting Star
4. Double Bottom
5. Double Top
6. Head and Shoulders
</COVERAGE_CHECKLIST>

<SOURCE_OUTLINE>
# L3: Building backtest framework and rule-based trading strategy
Course Code: COMP7415
# Agenda
• Introduction to Backtesting
- Common Candlestick Patterns
Data Cleaning Techniques
- Implement an MA Cross Strategy on
- Excel
- Python
ALGOGENE
- Common Backtest Pitfalls
Exercise: RSI Strategy
# What is Backtesting?
• Definition: Simulating a trading strategy using historical data
- Purpose: Validate the effectiveness of a trading strategy before live trading
Key Procedures:
1. Data Collection
2. Data Cleaning
3. Strategy Implementation
4. Performance Evaluation
# Backtesting is just like…
![](images/525dbb9750f148aaf4318653112473213222fd1da61b1114dd87086efdcffd3e.jpg)
# Why data cleaning?
- Remove data entry errors or system bugs from data provider
- Increase data quality and consistency
- Easier for data management
- Produces a more accurate statistical model
- Better trade decision making
- Potential data issues:
- Missing values
- Duplicated records
Data contains incorrect logics
# Algo Trading Lifecycle
![](images/eadd6f08094a3cade84ca188af3f43415558df388d1de9774ac77f29374cda95.jpg)
# Candlestick
# Introduction to Candlestick
- A candlestick chart (also called K-line) is a type of financial chart that displays the price movement of an asset over time.
It summarizes OHLC data in a single chart
- Open: The price at the start of the time period.
- Close: The price at the end of the time period.
High: The highest price during the period.
- Low: The lowest price during the period.
- OHLC data is aggregated in different time intervals (eg. 1-min, 1 hour, 1-day)
- In Technical Analysis, a candlestick pattern could provide indication on the future direction.
# Understanding Candlestick Components
# - Components:
- Body: The area between the opening and closing prices.
- Wicks: Lines extending above and below the body, representing the high and low prices.
# - Visual display:
- Bullish Candle: Close > Open (often colored green or white)
• Bearish Candle: Close < Open (often colored red or black)
![](images/a6fccb6cb2b229b6494f629e95a068ae057b8742065c1b4687634aa1d4a2283d.jpg)
![](images/32aa7f44ab285e94b336d6d3a900f142f6de7fb7a2825f72524baa70e58182da.jpg)
# Common Candlestick Patterns
1. Doji
2. Hammer
3. Shooting Star
4. Double Bottom
5. Double Top
6. Head and Shoulders
</SOURCE_OUTLINE>

<LESSON_MAP>
{
  "lesson_id": "L3",
  "mode": "guided_story",
  "steps": [
    {
      "concept": "什么是回测",
      "file": "research/pipeline/3-guided_story/L3/step1/step.json",
      "sequence_id": "step1"
    },
    {
      "concept": "数据清洗：让数据可信",
      "file": "research/pipeline/3-guided_story/L3/step2/step.json",
      "sequence_id": "step2"
    },
    {
      "concept": "K线图与常见形态",
      "file": "research/pipeline/3-guided_story/L3/step3/step.json",
      "sequence_id": "step3"
    },
    {
      "concept": "移动平均线策略",
      "file": "research/pipeline/3-guided_story/L3/step4/step.json",
      "sequence_id": "step4"
    },
    {
      "concept": "用Excel实现MA交叉策略",
      "file": "research/pipeline/3-guided_story/L3/step5/step.json",
      "sequence_id": "step5"
    },
    {
      "concept": "回测的陷阱与假设",
      "file": "research/pipeline/3-guided_story/L3/step6/step.json",
      "sequence_id": "step6"
    },
    {
      "concept": "用Python和专用库回测",
      "file": "research/pipeline/3-guided_story/L3/step7/step.json",
      "sequence_id": "step7"
    },
    {
      "concept": "RSI策略练习",
      "file": "research/pipeline/3-guided_story/L3/step8/step.json",
      "sequence_id": "step8"
    }
  ]
}

</LESSON_MAP>

<GUIDED_STORY_MANIFEST>
{
  "lesson_id": "L3",
  "mode": "guided_story",
  "steps": [
    {
      "concept": "什么是回测",
      "file": "research/pipeline/3-guided_story/L3/step1/step.json",
      "sequence_id": "step1"
    },
    {
      "concept": "数据清洗：让数据可信",
      "file": "research/pipeline/3-guided_story/L3/step2/step.json",
      "sequence_id": "step2"
    },
    {
      "concept": "K线图与常见形态",
      "file": "research/pipeline/3-guided_story/L3/step3/step.json",
      "sequence_id": "step3"
    },
    {
      "concept": "移动平均线策略",
      "file": "research/pipeline/3-guided_story/L3/step4/step.json",
      "sequence_id": "step4"
    },
    {
      "concept": "用Excel实现MA交叉策略",
      "file": "research/pipeline/3-guided_story/L3/step5/step.json",
      "sequence_id": "step5"
    },
    {
      "concept": "回测的陷阱与假设",
      "file": "research/pipeline/3-guided_story/L3/step6/step.json",
      "sequence_id": "step6"
    },
    {
      "concept": "用Python和专用库回测",
      "file": "research/pipeline/3-guided_story/L3/step7/step.json",
      "sequence_id": "step7"
    },
    {
      "concept": "RSI策略练习",
      "file": "research/pipeline/3-guided_story/L3/step8/step.json",
      "sequence_id": "step8"
    }
  ]
}

</GUIDED_STORY_MANIFEST>

<GUIDED_STORY_STEPS>
[
  {
    "lesson_id": "L3",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s001",
        "introduced_terms": [],
        "lines": [
          "想象一下，你设计了一套交易规则。",
          "在投入真金白银之前，你想知道它是否有效。"
        ],
        "type": "narration"
      },
      {
        "id": "s002",
        "introduced_terms": [
          "backtesting"
        ],
        "lines": [
          "这就是<term id=\"backtesting\">回测</term>要做的事。",
          "用历史数据来模拟你的策略，看看它过去表现如何。"
        ],
        "type": "narration"
      },
      {
        "id": "s003",
        "introduced_terms": [],
        "lines": [
          "回测的核心流程很简单：",
          "收集数据 → 清洗数据 → 运行策略 → 评估表现。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 2,
          "explanation": "回测的核心就是用历史数据来检验交易策略是否有效，而不是预测未来。",
          "kind": "single_choice",
          "options": [
            "预测未来股价",
            "在真实市场中赚钱",
            "用历史数据验证策略的有效性",
            "计算交易成本"
          ],
          "prompt": "回测的主要目的是什么？"
        },
        "id": "s004",
        "lines": [
          "回测的主要目的是什么？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step1",
    "source": {
      "plain_text": "回测是用历史数据模拟交易策略，验证其有效性。关键步骤包括数据收集、数据清洗、策略实施和绩效评估。",
      "related": [
        "backtesting"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "backtesting": {
        "aliases": [
          "Backtesting",
          "Backtest"
        ],
        "display": "回测",
        "gloss": "用历史数据模拟交易策略，验证其有效性。"
      }
    }
  },
  {
    "lesson_id": "L3",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s005",
        "introduced_terms": [],
        "lines": [
          "拿到数据后，不能直接使用。",
          "原始数据里常常有各种问题。"
        ],
        "type": "narration"
      },
      {
        "id": "s006",
        "introduced_terms": [
          "data_cleaning"
        ],
        "lines": [
          "这个过程叫<term id=\"data_cleaning\">数据清洗</term>。",
          "目的是让数据更准确、一致，为后续分析打好基础。"
        ],
        "type": "narration"
      },
      {
        "id": "s007",
        "introduced_terms": [],
        "lines": [
          "常见的问题有三种：",
          "缺失值、重复记录、逻辑错误。"
        ],
        "type": "narration"
      },
      {
        "id": "s008",
        "introduced_terms": [],
        "lines": [
          "比如，某一天的收盘价数据缺失了。",
          "你可以从其他来源补全，或者用相邻数据填充，甚至直接删除这一行。"
        ],
        "type": "narration"
      },
      {
        "id": "s009",
        "introduced_terms": [],
        "lines": [
          "再比如，同一天出现了两条完全一样的数据。",
          "通常保留一条就够了。"
        ],
        "type": "narration"
      },
      {
        "id": "s010",
        "introduced_terms": [],
        "lines": [
          "更隐蔽的是逻辑错误。",
          "比如收盘价高于当日最高价，这在逻辑上是不可能的。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 2,
          "explanation": "收盘价不可能高于当日最高价，这违反了基本的价格逻辑。",
          "kind": "single_choice",
          "options": [
            "某天没有交易数据",
            "某天的开盘价和收盘价数据完全相同",
            "某天的收盘价高于当日最高价",
            "某天的交易量特别大"
          ],
          "prompt": "以下哪种情况属于数据中的逻辑错误？"
        },
        "id": "s011",
        "lines": [
          "以下哪种情况属于数据中的逻辑错误？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step2",
    "source": {
      "plain_text": "数据清洗是为了去除数据录入错误或系统错误，提高数据质量和一致性。常见问题包括缺失值、重复记录和逻辑错误。",
      "related": [
        "data_cleaning"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "data_cleaning": {
        "aliases": [
          "Data Cleaning",
          "Data Cleansing"
        ],
        "display": "数据清洗",
        "gloss": "识别并修正数据中的错误、缺失、重复或不一致，以提高数据质量。"
      }
    }
  },
  {
    "lesson_id": "L3",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s012",
        "introduced_terms": [
          "candlestick"
        ],
        "lines": [
          "在分析价格时，有一种非常直观的工具：<term id=\"candlestick\">K线图</term>。",
          "它把一天的价格变化浓缩成一根蜡烛。"
        ],
        "type": "narration"
      },
      {
        "id": "s013",
        "introduced_terms": [],
        "lines": [
          "每根K线包含四个关键信息：",
          "开盘价、收盘价、最高价、最低价。"
        ],
        "type": "narration"
      },
      {
        "id": "s014",
        "introduced_terms": [],
        "lines": [
          "如果收盘价高于开盘价，通常是阳线（绿色或白色）。",
          "反之，则是阴线（红色或黑色）。"
        ],
        "type": "narration"
      },
      {
        "id": "s015",
        "introduced_terms": [
          "doji"
        ],
        "lines": [
          "K线的实体和影线能告诉我们很多信息。",
          "比如，一种叫<term id=\"doji\">十字星</term>的形态。"
        ],
        "type": "narration"
      },
      {
        "id": "s016",
        "introduced_terms": [],
        "lines": [
          "十字星的开盘价和收盘价几乎一样。",
          "它意味着市场多空双方力量均衡，犹豫不决，可能预示着趋势反转。"
        ],
        "type": "narration"
      },
      {
        "id": "s017",
        "introduced_terms": [
          "hammer"
        ],
        "lines": [
          "再看<term id=\"hammer\">锤子线</term>。",
          "它出现在下跌趋势后，下影线很长，说明价格被压低后又被买盘拉回。"
        ],
        "type": "narration"
      },
      {
        "id": "s018",
        "introduced_terms": [
          "shooting_star"
        ],
        "lines": [
          "锤子线通常被视为一个看涨反转信号。",
          "与之相反的是<term id=\"shooting_star\">射击之星</term>。"
        ],
        "type": "narration"
      },
      {
        "id": "s019",
        "introduced_terms": [],
        "lines": [
          "射击之星出现在上涨趋势后，上影线很长。",
          "说明价格冲高后遭遇抛压，可能预示着下跌。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "锤子线出现在下跌趋势后，长长的下影线表明买方力量开始介入，是潜在的看涨反转信号。",
          "kind": "single_choice",
          "options": [
            "十字星",
            "锤子线",
            "射击之星",
            "以上都不是"
          ],
          "prompt": "以下哪种K线形态通常被视为看涨反转信号？"
        },
        "id": "s020",
        "lines": [
          "以下哪种K线形态通常被视为看涨反转信号？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step3",
    "source": {
      "plain_text": "K线图（又称蜡烛图）是一种展示资产价格波动的图表。它包含开盘价、收盘价、最高价和最低价。常见的K线形态有十字星、锤子线、射击之星等，它们可以提供未来价格方向的信号。",
      "related": [
        "candlestick",
        "doji",
        "hammer",
        "shooting_star"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "candlestick": {
        "aliases": [
          "Candlestick Chart",
          "K线",
          "蜡烛图"
        ],
        "display": "K线图",
        "gloss": "一种展示资产价格波动的图表，包含开盘价、收盘价、最高价和最低价。"
      },
      "doji": {
        "aliases": [
          "Doji"
        ],
        "display": "十字星",
        "gloss": "开盘价和收盘价几乎相等的K线形态，代表市场犹豫不决。"
      },
      "hammer": {
        "aliases": [
          "Hammer"
        ],
        "display": "锤子线",
        "gloss": "下影线很长、实体很小的K线，出现在下跌趋势后，可能预示反转上涨。"
      },
      "shooting_star": {
        "aliases": [
          "Shooting Star"
        ],
        "display": "射击之星",
        "gloss": "上影线很长、实体很小的K线，出现在上涨趋势后，可能预示反转下跌。"
      }
    }
  },
  {
    "lesson_id": "L3",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s021",
        "introduced_terms": [
          "moving_average"
        ],
        "lines": [
          "现在，我们来看一个经典策略：移动平均线交叉。",
          "首先，什么是<term id=\"moving_average\">移动平均线</term>？"
        ],
        "type": "narration"
      },
      {
        "id": "s022",
        "introduced_terms": [],
        "lines": [
          "它就是把过去N天的收盘价加起来，再除以N。",
          "比如，5日移动平均线就是最近5天收盘价的平均值。"
        ],
        "type": "narration"
      },
      {
        "formula": {
          "latex": "\\mathrm{MA}(n) = \\frac{\\sum_{t=1}^{n} P_t}{n}",
          "spoken": "N日移动平均线等于过去N天的收盘价之和除以N。"
        },
        "id": "s023",
        "introduced_terms": [
          "simple_moving_average"
        ],
        "lines": [
          "这是<term id=\"simple_moving_average\">简单移动平均线</term>的公式："
        ],
        "type": "formula"
      },
      {
        "id": "s024",
        "introduced_terms": [],
        "lines": [
          "移动平均线可以平滑价格，让我们更容易看清趋势。",
          "我们通常使用两条不同周期的均线：一条快线，一条慢线。"
        ],
        "type": "narration"
      },
      {
        "id": "s025",
        "introduced_terms": [
          "golden_cross"
        ],
        "lines": [
          "当快线上穿慢线时，称为<term id=\"golden_cross\">黄金交叉</term>。",
          "这是一个买入信号。"
        ],
        "type": "narration"
      },
      {
        "id": "s026",
        "introduced_terms": [
          "death_cross"
        ],
        "lines": [
          "当快线下穿慢线时，称为<term id=\"death_cross\">死亡交叉</term>。",
          "这是一个卖出信号。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "黄金交叉是短期移动平均线上穿长期移动平均线，通常被视为看涨信号。",
          "kind": "single_choice",
          "options": [
            "价格创出新高",
            "短期均线上穿长期均线",
            "长期均线上穿短期均线",
            "交易量放大"
          ],
          "prompt": "黄金交叉是指什么？"
        },
        "id": "s027",
        "lines": [
          "黄金交叉是指什么？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step4",
    "source": {
      "plain_text": "移动平均线（MA）用于平滑价格数据，识别趋势。简单移动平均线（SMA）是过去N个周期的平均价格。MA交叉策略使用快慢两条均线，当快线上穿慢线时买入（黄金交叉），下穿时卖出（死亡交叉）。",
      "related": [
        "moving_average",
        "simple_moving_average",
        "golden_cross",
        "death_cross"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "death_cross": {
        "aliases": [
          "Death Cross"
        ],
        "display": "死亡交叉",
        "gloss": "短期移动平均线下穿长期移动平均线，通常被视为卖出信号。"
      },
      "golden_cross": {
        "aliases": [
          "Golden Cross"
        ],
        "display": "黄金交叉",
        "gloss": "短期移动平均线上穿长期移动平均线，通常被视为买入信号。"
      },
      "moving_average": {
        "aliases": [
          "Moving Average",
          "MA"
        ],
        "display": "移动平均线",
        "gloss": "对一定时期内的价格取平均值，用于平滑短期波动，显示长期趋势。"
      },
      "simple_moving_average": {
        "aliases": [
          "Simple Moving Average",
          "SMA"
        ],
        "display": "简单移动平均线",
        "gloss": "对过去N个周期的价格直接取算术平均值。"
      }
    }
  },
  {
    "lesson_id": "L3",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s028",
        "introduced_terms": [],
        "lines": [
          "理论讲完了，我们来动手实现。",
          "先从Excel开始，因为它最直观。"
        ],
        "type": "narration"
      },
      {
        "id": "s029",
        "introduced_terms": [],
        "lines": [
          "假设我们下载了汇丰控股（0005.HK）的历史数据。",
          "第一步：计算两条移动平均线。"
        ],
        "type": "narration"
      },
      {
        "id": "s030",
        "introduced_terms": [],
        "lines": [
          "用7日作为快线，14日作为慢线。",
          "在Excel里，用AVERAGE函数就能轻松算出。"
        ],
        "type": "narration"
      },
      {
        "id": "s031",
        "introduced_terms": [],
        "lines": [
          "第二步：识别交叉信号。",
          "当MA(7)上穿MA(14)时，标记为买入信号（1）。"
        ],
        "type": "narration"
      },
      {
        "id": "s032",
        "introduced_terms": [],
        "lines": [
          "当MA(7)下穿MA(14)时，标记为卖出信号（-1）。",
          "否则为0。"
        ],
        "type": "narration"
      },
      {
        "id": "s033",
        "introduced_terms": [],
        "lines": [
          "第三步：计算每日持仓。",
          "有买入信号且当前空仓时，持仓变为1。"
        ],
        "type": "narration"
      },
      {
        "id": "s034",
        "introduced_terms": [],
        "lines": [
          "第四步：计算每日盈亏。",
          "持仓 * (今日收盘价 - 昨日收盘价)。"
        ],
        "type": "narration"
      },
      {
        "id": "s035",
        "introduced_terms": [],
        "lines": [
          "最后，把每日盈亏累加起来，就是累计盈亏。",
          "这样，一个简单的回测就完成了。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answers": [
            "AVERAGE"
          ],
          "explanation": "AVERAGE函数用于计算一组数值的平均值，非常适合计算简单移动平均线。",
          "kind": "fill_in_blank",
          "prompt": "在Excel中，计算7日移动平均线应该使用哪个函数？"
        },
        "id": "s036",
        "lines": [
          "在Excel中，计算7日移动平均线应该使用哪个函数？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step5",
    "source": {
      "plain_text": "在Excel中实现MA交叉策略的步骤：1. 计算MA(7)和MA(14)；2. 识别交叉信号；3. 计算持仓；4. 计算每日盈亏；5. 计算累计盈亏。",
      "related": []
    },
    "target_language": "zh-CN",
    "term_catalog": {}
  },
  {
    "lesson_id": "L3",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s037",
        "introduced_terms": [],
        "lines": [
          "回测结果看起来很漂亮？先别急着高兴。",
          "回测中有很多陷阱，一不小心就会掉进去。"
        ],
        "type": "narration"
      },
      {
        "id": "s038",
        "introduced_terms": [
          "overfitting"
        ],
        "lines": [
          "第一个陷阱：<term id=\"overfitting\">过度拟合</term>。",
          "策略被调整得过于贴合历史数据，但一到新市场就失效。"
        ],
        "type": "narration"
      },
      {
        "id": "s039",
        "introduced_terms": [
          "look_ahead_bias"
        ],
        "lines": [
          "第二个陷阱：<term id=\"look_ahead_bias\">前视偏差</term>。",
          "在回测中不小心使用了未来的数据，比如用今天的收盘价来决定昨天的交易。"
        ],
        "type": "narration"
      },
      {
        "id": "s040",
        "introduced_terms": [
          "survivorship_bias"
        ],
        "lines": [
          "第三个陷阱：<term id=\"survivorship_bias\">幸存者偏差</term>。",
          "只测试了现在还活着的股票，那些已经退市的失败者被忽略了。"
        ],
        "type": "narration"
      },
      {
        "id": "s041",
        "introduced_terms": [],
        "lines": [
          "还有，我们做回测时总会做一些假设。",
          "比如：没有交易成本、可以按收盘价成交、允许卖空。"
        ],
        "type": "narration"
      },
      {
        "id": "s042",
        "introduced_terms": [],
        "lines": [
          "但在真实市场中，这些假设往往不成立。",
          "交易成本、滑点、流动性限制都会影响最终收益。"
        ],
        "type": "narration"
      },
      {
        "id": "s043",
        "introduced_terms": [],
        "lines": [
          "所以，回测结果好，不代表实盘也能赚钱。",
          "关键是要用现实的假设，并在样本外数据上验证。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "在计算交易信号时使用当天的收盘价，意味着你提前知道了当天收盘时的信息，这在真实交易中是不可能的。",
          "kind": "single_choice",
          "options": [
            "使用过去10年的数据测试策略",
            "在计算信号时，使用了当天的收盘价",
            "只测试了目前仍在交易的股票",
            "在回测中忽略了交易成本"
          ],
          "prompt": "以下哪种行为属于前视偏差？"
        },
        "id": "s044",
        "lines": [
          "以下哪种行为属于前视偏差？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step6",
    "source": {
      "plain_text": "回测中常见的陷阱包括：过度拟合、前视偏差、幸存者偏差、数据窥探偏差和忽略假设。最佳实践包括稳健验证、现实假设和持续监控。",
      "related": [
        "overfitting",
        "look_ahead_bias",
        "survivorship_bias"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "look_ahead_bias": {
        "aliases": [
          "Look-ahead Bias"
        ],
        "display": "前视偏差",
        "gloss": "在回测中使用了当时无法获得的未来信息。"
      },
      "overfitting": {
        "aliases": [
          "Overfitting"
        ],
        "display": "过度拟合",
        "gloss": "策略过于贴合历史数据，导致在新数据上表现不佳。"
      },
      "survivorship_bias": {
        "aliases": [
          "Survivorship Bias"
        ],
        "display": "幸存者偏差",
        "gloss": "只考虑存活至今的资产，忽略了已经退市的，导致高估策略表现。"
      }
    }
  },
  {
    "lesson_id": "L3",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s045",
        "introduced_terms": [],
        "lines": [
          "Excel虽然方便，但处理复杂策略和数据量时力不从心。",
          "这时候，Python就派上用场了。"
        ],
        "type": "narration"
      },
      {
        "id": "s046",
        "introduced_terms": [
          "talib"
        ],
        "lines": [
          "首先，<term id=\"talib\">TA-Lib</term>是一个宝藏库。",
          "它内置了120多种技术指标，比如移动平均线、RSI、布林带等。"
        ],
        "type": "narration"
      },
      {
        "id": "s047",
        "introduced_terms": [],
        "lines": [
          "用TA-Lib计算移动平均线，一行代码就够了。",
          "`real = MA(close, timeperiod=30)`"
        ],
        "type": "narration"
      },
      {
        "id": "s048",
        "introduced_terms": [
          "backtesting_py"
        ],
        "lines": [
          "另一个好用的工具是<term id=\"backtesting_py\">Backtesting.py</term>。",
          "它是一个轻量级的回测框架，代码非常简洁。"
        ],
        "type": "narration"
      },
      {
        "id": "s049",
        "introduced_terms": [],
        "lines": [
          "你只需要定义一个策略类，在`init`方法中初始化指标，",
          "在`next`方法中编写交易逻辑。"
        ],
        "type": "narration"
      },
      {
        "id": "s050",
        "introduced_terms": [],
        "lines": [
          "然后，把数据和策略传给`Backtest`对象，运行即可。",
          "它会自动计算各种绩效指标，并生成图表。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answers": [
            "next"
          ],
          "explanation": "`next`方法会在每个新的K线数据到来时被调用，是编写买卖条件的地方。",
          "kind": "fill_in_blank",
          "prompt": "在Backtesting.py中，交易逻辑应该写在哪个方法里？"
        },
        "id": "s051",
        "lines": [
          "在Backtesting.py中，交易逻辑应该写在哪个方法里？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step7",
    "source": {
      "plain_text": "Python中有很多强大的回测工具。TA-Lib提供了120多种技术指标。Backtesting.py是一个轻量级的回测框架，可以快速实现和测试策略。",
      "related": [
        "talib",
        "backtesting_py"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "backtesting_py": {
        "aliases": [
          "backtesting.py"
        ],
        "display": "Backtesting.py",
        "gloss": "一个轻量级的Python回测框架，可以快速测试交易策略。"
      },
      "talib": {
        "aliases": [
          "Technical Analysis Library"
        ],
        "display": "TA-Lib",
        "gloss": "一个开源的Python库，包含120多种技术指标的计算函数。"
      }
    }
  },
  {
    "lesson_id": "L3",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s052",
        "introduced_terms": [
          "rsi"
        ],
        "lines": [
          "最后，我们来挑战一个练习：RSI策略。",
          "<term id=\"rsi\">相对强弱指标</term>是一个常用的动量指标。"
        ],
        "type": "narration"
      },
      {
        "id": "s053",
        "introduced_terms": [],
        "lines": [
          "RSI的值在0到100之间。",
          "通常认为，RSI超过70时，市场处于超买状态，可能回调。"
        ],
        "type": "narration"
      },
      {
        "id": "s054",
        "introduced_terms": [],
        "lines": [
          "当RSI低于30时，市场处于超卖状态，可能反弹。",
          "RSI策略就是基于这个逻辑。"
        ],
        "type": "narration"
      },
      {
        "id": "s055",
        "introduced_terms": [],
        "lines": [
          "策略规则：",
          "当RSI(14) < 30时，买入。",
          "当RSI(14) > 70时，卖出。"
        ],
        "type": "narration"
      },
      {
        "id": "s056",
        "introduced_terms": [],
        "lines": [
          "你可以尝试在Excel、Backtesting.py或ALGOGENE上实现这个策略。",
          "用SPY（标普500 ETF）从2020年到2025年的日线数据来测试。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 0,
          "explanation": "RSI低于30被视为超卖，是潜在的买入信号。",
          "kind": "single_choice",
          "options": [
            "买入",
            "卖出",
            "持有不动",
            "无法判断"
          ],
          "prompt": "根据RSI策略，当RSI(14)的值为25时，应该怎么做？"
        },
        "id": "s057",
        "lines": [
          "根据RSI策略，当RSI(14)的值为25时，应该怎么做？"
        ],
        "type": "exercise"
      },
      {
        "id": "s058",
        "introduced_terms": [],
        "lines": [
          "记住，没有完美的策略。",
          "回测只是第一步，实盘交易还需要考虑更多现实因素。"
        ],
        "type": "narration"
      }
    ],
    "sequence_id": "step8",
    "source": {
      "plain_text": "RSI（相对强弱指标）是一个动量指标，范围0-100。通常RSI>70为超买，RSI<30为超卖。RSI策略：当RSI<30时买入，当RSI>70时卖出。",
      "related": [
        "rsi"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "rsi": {
        "aliases": [
          "Relative Strength Index",
          "RSI"
        ],
        "display": "相对强弱指标",
        "gloss": "一种动量指标，用于衡量价格变动的速度和幅度，范围在0到100之间。"
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
  "lesson_id": "L3",
  "mode": "guided_story",
  "screens": [
    {
      "id": "s005",
      "introduced_terms": [],
      "lines": [
        "拿到数据后，不能直接使用。",
        "原始数据里常常有各种问题。"
      ],
      "type": "narration"
    },
    {
      "id": "s006",
      "introduced_terms": [
        "data_cleaning"
      ],
      "lines": [
        "这个过程叫<term id=\"data_cleaning\">数据清洗</term>。",
        "目的是让数据更准确、一致，为后续分析打好基础。"
      ],
      "type": "narration"
    },
    {
      "id": "s007",
      "introduced_terms": [],
      "lines": [
        "常见的问题有三种：",
        "缺失值、重复记录、逻辑错误。"
      ],
      "type": "narration"
    },
    {
      "id": "s008",
      "introduced_terms": [],
      "lines": [
        "比如，某一天的收盘价数据缺失了。",
        "你可以从其他来源补全，或者用相邻数据填充，甚至直接删除这一行。"
      ],
      "type": "narration"
    },
    {
      "id": "s009",
      "introduced_terms": [],
      "lines": [
        "再比如，同一天出现了两条完全一样的数据。",
        "通常保留一条就够了。"
      ],
      "type": "narration"
    },
    {
      "id": "s010",
      "introduced_terms": [],
      "lines": [
        "更隐蔽的是逻辑错误。",
        "比如收盘价高于当日最高价，这在逻辑上是不可能的。"
      ],
      "type": "narration"
    },
    {
      "exercise": {
        "answer": 2,
        "explanation": "收盘价不可能高于当日最高价，这违反了基本的价格逻辑。",
        "kind": "single_choice",
        "options": [
          "某天没有交易数据",
          "某天的开盘价和收盘价数据完全相同",
          "某天的收盘价高于当日最高价",
          "某天的交易量特别大"
        ],
        "prompt": "以下哪种情况属于数据中的逻辑错误？"
      },
      "id": "s011",
      "lines": [
        "以下哪种情况属于数据中的逻辑错误？"
      ],
      "type": "exercise"
    }
  ],
  "sequence_id": "step2",
  "source": {
    "plain_text": "数据清洗是为了去除数据录入错误或系统错误，提高数据质量和一致性。常见问题包括缺失值、重复记录和逻辑错误。",
    "related": [
      "data_cleaning"
    ]
  },
  "target_language": "zh-CN",
  "term_catalog": {
    "data_cleaning": {
      "aliases": [
        "Data Cleaning",
        "Data Cleansing"
      ],
      "display": "数据清洗",
      "gloss": "识别并修正数据中的错误、缺失、重复或不一致，以提高数据质量。"
    }
  }
}

</CURRENT_STEP>

<PLAIN_TEXT>
# L3: Building backtest framework and rule-based trading strategy

Course Code: COMP7415

# Agenda

• Introduction to Backtesting   
- Common Candlestick Patterns   
Data Cleaning Techniques   
- Implement an MA Cross Strategy on

- Excel   
- Python   
ALGOGENE

- Common Backtest Pitfalls   
Exercise: RSI Strategy

# What is Backtesting?

• Definition: Simulating a trading strategy using historical data   
- Purpose: Validate the effectiveness of a trading strategy before live trading   
Key Procedures:

1. Data Collection   
2. Data Cleaning   
3. Strategy Implementation   
4. Performance Evaluation

# Backtesting is just like…

![](images/525dbb9750f148aaf4318653112473213222fd1da61b1114dd87086efdcffd3e.jpg)

# Why data cleaning?

- Remove data entry errors or system bugs from data provider   
- Increase data quality and consistency

- Easier for data management   
- Produces a more accurate statistical model   
- Better trade decision making

- Potential data issues:

- Missing values   
- Duplicated records   
Data contains incorrect logics

# Algo Trading Lifecycle

![](images/eadd6f08094a3cade84ca188af3f43415558df388d1de9774ac77f29374cda95.jpg)

# Candlestick

# Introduction to Candlestick

- A candlestick chart (also called K-line) is a type of financial chart that displays the price movement of an asset over time.   
It summarizes OHLC data in a single chart

- Open: The price at the start of the time period.   
- Close: The price at the end of the time period.   
High: The highest price during the period.   
- Low: The lowest price during the period.

- OHLC data is aggregated in different time intervals (eg. 1-min, 1 hour, 1-day)   
- In Technical Analysis, a candlestick pattern could provide indication on the future direction.

# Understanding Candlestick Components

# - Components:

- Body: The area between the opening and closing prices.   
- Wicks: Lines extending above and below the body, representing the high and low prices.

# - Visual display:

- Bullish Candle: Close > Open (often colored green or white)   
• Bearish Candle: Close < Open (often colored red or black)

![](images/a6fccb6cb2b229b6494f629e95a068ae057b8742065c1b4687634aa1d4a2283d.jpg)

![](images/32aa7f44ab285e94b336d6d3a900f142f6de7fb7a2825f72524baa70e58182da.jpg)

# Common Candlestick Patterns

1. Doji   
2. Hammer   
3. Shooting Star   
4. Double Bottom   
5. Double Top   
6. Head and Shoulders   
7. Bull/Bear Flag   
8. Bull/Bear Pennant   
9. Ascending/Descending Triangle

# Common Candlestick Patterns

# 1. Doji

Description: A Doji candlestick forms when the open and close prices are nearly equal, creating a small body.   
- Market Insight: The Doji indicates indecision in the market. Traders are uncertain about price direction, which can precede a reversal or continuation of the trend.   
- Example: If a Doji appears after a bullish trend, it may signal a potential reversal, prompting traders to reevaluate their positions.

![](images/4098e61f0f1c3712c0190099a3bd383ae922bf4b310fbd774d6bc3ff0b3b9742.jpg)

Neutral: Dojis form when the opening and closing prices are virtually equal. Alone, dojis are neutral patterns.

![](images/f09466aa39246287840b7cc09675db9e5a63cdaa10fdd45dd99444a5527e6a69.jpg)

Long-Legged: This doji reflects a great amount of indecision about the future direction of the underlying asset.

![](images/8d336c77819833205499f5ceefb119e2062fe50408846472930f6fe0435665c1.jpg)

# Common Candlestick Patterns

# 2. Hammer

Description: The Hammer pattern consists of a long lower wick and a small body at the top, appearing after a downtrend.   
- Market Insight: This pattern suggests a bullish reversal. The long lower wick indicates that sellers pushed prices down, but buyers stepped in, driving the price back up.   
- Example: When a Hammer forms at a support level, it can indicate strong buying interest, signaling traders to consider long positions.

![](images/98d8bf4ccbf56e9ce8744a18d9a539f0d6aafb933941183a95bb507cd94b19fa.jpg)

![](images/726985b93f86c2f72cd338e0e983af2783e6de493ff6b835fe9cb229cf677251.jpg)

# Common Candlestick Patterns

# 3. Shooting Star

- Description: The Shooting Star features a long upper wick with a small body at the bottom, forming after an uptrend.   
- Market Insight: This pattern indicates a potential bearish reversal. The long upper wick shows that buyers pushed prices higher, but sellers took control, pushing the price back down.   
- Example: If a Shooting Star appears at a resistance level, it may signal traders to consider short positions.

![](images/3be96a4b25aa70d8ba0fae6e08fcca40a570d3fb78897e0d47f430681d89a9b0.jpg)

![](images/242d3e29b324c0bb0eee9b819a65dd26522189d8a063bd568cb281efc726eafd.jpg)

# Common Candlestick Patterns

# 4. Double Bottom

# Description:

- A Double Bottom is a bullish reversal pattern that occurs after a downtrend.   
It consists of two troughs at approximately the same price level, with a peak in between.

# Market Insight:

- The first trough represents strong selling pressure, while the second trough indicates a potential reversal as buyers begin to emerge.   
- A breakout above the peak confirms the pattern, signaling potential price appreciation.

- Example: Traders often look for confirmation with increased volume during the breakout to enhance reliability.

![](images/7f9ef7720e2c612ef8b45f1c38b865f56060825e127590970d27f1add93b4bd3.jpg)

# Common Candlestick Patterns

# 5. Double Top

# Description:

- A Double Top is a bearish reversal pattern that occurs after an uptrend.   
- It consists of two peaks at roughly the same price level, with a trough in between.

# Market Insight:

- The first peak indicates strong buying pressure, but the second peak shows weakening momentum as sellers begin to take control.   
- A breakout below the trough confirms the pattern, signaling a potential price decline.

- Example: As with the Double Bottom, increased volume during the breakout can enhance the pattern's validity.

![](images/f293e8836df585b09a516e987904c969e332de979be854d169d32de2e8fd5895.jpg)

# There are so many other patterns…

![](images/91345338d1f7ea20ad2e03de5a8a1472782eef191b46667e618819653ed9a4a1.jpg)  
Head and Shoulders

![](images/b4e06a16dff55d4bae6bf75fe98965ddb1d7891d8e8f7b3fb5c5adc5cc8cdd99.jpg)  
Ascending Triangle

![](images/1fe512e60e8ce305c6a4d8a797134a629bdd7ff1fd3fe87f16be7f0fdbd408f9.jpg)  
Pennant

![](images/a20d27d4a8f0f500276363145919ebc1f291ad5dcf358f912ff8b7a51d6a2676.jpg)  
Flag

# Backtest with Excel

# Why use Excel for backtest?

- Accessibility: Widely available and user-friendly   
- Visualization: Easy to create charts   
- Simplicity: No programming knowledge required for basic tasks

# Download Data

https://finance.yahoo.com/quote/0005.HK/history/?period1=1546300800&period2=1703980800

- Instrument: 0005.HK   
Period: 2010.01.01 – 2023.12.31

![](images/a86fe02595f5e56d0429b3d348cee7335c9a02acd3dee70484658dd1126e4051.jpg)

# Import Data to Excel

![](images/007cc3be3c9b05cba8c2285ea74f94c2e2c637bf83c2cd3de7e31c35f0b2eff5.jpg)

![](images/f18d8f2c04a2a004049ea756b3e7713ec30b2bccc841de49cec6c4c38c8cf905.jpg)

<table><tr><td></td><td>A</td><td>B</td><td>C</td><td>D</td><td>E</td><td>F</td><td>G</td><td>H</td><td>I</td><td></td></tr><tr><td>1</td><td>Date</td><td>Open</td><td>High</td><td>Low</td><td>Close</td><td>Adj Close</td><td>Volume</td><td></td><td></td><td></td></tr><tr><td>2</td><td>04/01/2010</td><td>89.4</td><td>89.9</td><td>88.8</td><td>89.25</td><td>44.81234</td><td>10381759</td><td></td><td></td><td></td></tr><tr><td>3</td><td>05/01/2010</td><td>90.2</td><td>90.65</td><td>90.1</td><td>90.45</td><td>45.41485</td><td>16914189</td><td></td><td></td><td></td></tr><tr><td>4</td><td>06/01/2010</td><td>91.3</td><td>92</td><td>91.25</td><td>91.6</td><td>45.99229</td><td>20033996</td><td></td><td></td><td></td></tr><tr><td>5</td><td>07/01/2010</td><td>91.7</td><td>91.75</td><td>90.85</td><td>91.25</td><td>45.81654</td><td>11163952</td><td></td><td></td><td></td></tr><tr><td>6</td><td>08/01/2010</td><td>91.5</td><td>92.15</td><td>91.35</td><td>91.7</td><td>46.04247</td><td>15683744</td><td></td><td></td><td></td></tr><tr><td>7</td><td>11/01/2010</td><td>92</td><td>92.8</td><td>92</td><td>92.4</td><td>46.39394</td><td>15729515</td><td></td><td></td><td></td></tr><tr><td>8</td><td>12/01/2010</td><td>91.8</td><td>92.4</td><td>91.05</td><td>91.65</td><td>46.01737</td><td>14005577</td><td></td><td></td><td></td></tr><tr><td>9</td><td>13/01/2010</td><td>90.5</td><td>90.8</td><td>89.95</td><td>90.2</td><td>45.28933</td><td>17142233</td><td></td><td></td><td></td></tr><tr><td>10</td><td>14/01/2010</td><td>90.65</td><td>91.2</td><td>90.65</td><td>90.85</td><td>45.61569</td><td>11379186</td><td></td><td></td><td></td></tr><tr><td>11</td><td>15/01/2010</td><td>91</td><td>90.3</td><td>90.75</td><td>91.05</td><td>45.71613</td><td>13190937</td><td></td><td></td><td></td></tr><tr><td>12</td><td>18/01/2010</td><td>89.05</td><td>89.6</td><td>88.6</td><td>89.25</td><td>44.81234</td><td>16529101</td><td></td><td></td><td></td></tr></table>

# Data Cleaning (1): missing data

# Objective:

- Search for rows with missing value

# How to solve?

- Check from other data sources (if available)   
- Fill in the missing value based on adjacent observations (eg. average, smoothing, etc)   
- Delete the rows

![](images/0fa5b8318994888d1ac0009c3ae3f3b7ae8a8f83904457bed1374ec9f48c10ca.jpg)

# Data Cleaning (2): duplicated records

# Objective:

- Define our key (eg. Date)   
- Count the occurrence of key

# How to solve?

- if the duplicated records are completely identical,   
- Keep only 1 of them   
- If NOT identical,   
- Check from other data sources to determine which one is correct (if available)   
- Delete all duplicated records   
Take average of them

<table><tr><td colspan="2">H2</td><td>:</td><td colspan="2">× √ f(x)</td><td colspan="6">=COUNTIF(A:A,A2)</td></tr><tr><td colspan="2">A</td><td>B</td><td colspan="2">C</td><td>D</td><td>E</td><td>F</td><td>G</td><td colspan="2">H</td></tr><tr><td>1</td><td>Date</td><td>Open</td><td>High</td><td>Low</td><td>Low</td><td>Close</td><td>Adj Clos</td><td>Volume</td><td colspan="2">count</td></tr><tr><td>2</td><td>1/4/2010</td><td>89.4</td><td>89.9</td><td>88.8</td><td>89.25</td><td>44.81234</td><td>10381759</td><td colspan="3">1</td></tr><tr><td>3</td><td>1/5/2010</td><td>90.2</td><td>90.65</td><td>90.1</td><td>90.45</td><td>45.41485</td><td>16914189</td><td colspan="3">1</td></tr><tr><td>4</td><td>1/6/2010</td><td>91.3</td><td>92</td><td>91.25</td><td>91.6</td><td>45.99229</td><td>20033996</td><td colspan="3">1</td></tr><tr><td>5</td><td>1/7/2010</td><td>91.7</td><td>91.75</td><td>90.85</td><td>91.25</td><td>45.81654</td><td>11163952</td><td colspan="3">1</td></tr><tr><td>6</td><td>1/8/2010</td><td>91.5</td><td>92.15</td><td>91.35</td><td>91.7</td><td>46.04247</td><td>15683744</td><td colspan="3">1</td></tr><tr><td>7</td><td>1/11/2010</td><td>92</td><td>92.8</td><td>92</td><td>92.4</td><td>46.39394</td><td>15729515</td><td colspan="3">1</td></tr><tr><td>8</td><td>1/12/2010</td><td>91.8</td><td>92.4</td><td>91.05</td><td>91.65</td><td>46.01737</td><td>14005577</td><td colspan="3">1</td></tr><tr><td>9</td><td>1/13/2010</td><td>90.5</td><td>90.8</td><td>89.95</td><td>90.2</td><td>45.28933</td><td>17142233</td><td colspan="3">1</td></tr><tr><td>10</td><td>1/14/2010</td><td>90.65</td><td>91.2</td><td>90.65</td><td>90.85</td><td>45.61569</td><td>11379186</td><td colspan="3">1</td></tr><tr><td>11</td><td>1/15/2010</td><td>91</td><td>90.3</td><td>90.75</td><td>91.05</td><td>45.71613</td><td>13190937</td><td colspan="3">1</td></tr><tr><td>12</td><td>1/18/2010</td><td>89.05</td><td>89.6</td><td>88.6</td><td>89.25</td><td>44.81234</td><td>16529101</td><td colspan="3">1</td></tr><tr><td>13</td><td>1/19/2010</td><td>89.05</td><td>88.4</td><td>88.7</td><td>89.3</td><td>44.83743</td><td>17485152</td><td colspan="3">1</td></tr></table>

# Data Cleaning (3): incorrect logics

# Objective:

A valid row should satisfy the following conditions

- Close <= High and Close >= Low   
- Open <= High and Open >= Low

<table><tr><td colspan="2">H2</td><td>:</td><td colspan="2">× √ f(x)</td><td colspan="7">=AND(E2&lt;=C2, E2&gt;=D2)</td></tr><tr><td></td><td>A</td><td>B</td><td>C</td><td>D</td><td>E</td><td>F</td><td>G</td><td>H</td><td>I</td><td></td><td></td></tr><tr><td>1</td><td>Date</td><td>Open</td><td>High</td><td>Low</td><td>Close</td><td>Adj Clos</td><td>Volume</td><td>C&lt;=H and C&gt;</td><td>O&lt;=H and O&gt;</td><td></td><td></td></tr><tr><td>2</td><td>1/4/2010</td><td>89.4</td><td>89.9</td><td>88.8</td><td>89.25</td><td>44.81234</td><td>10381759</td><td>TRUE</td><td>TRUE</td><td></td><td></td></tr><tr><td>3</td><td>1/5/2010</td><td>90.2</td><td>90.65</td><td>90.1</td><td>90.45</td><td>45.41485</td><td>16914189</td><td>TRUE</td><td>TRUE</td><td></td><td></td></tr><tr><td>4</td><td>1/6/2010</td><td>91.3</td><td>92</td><td>91.25</td><td>91.6</td><td>45.99229</td><td>20033996</td><td>TRUE</td><td>TRUE</td><td></td><td></td></tr><tr><td>5</td><td>1/7/2010</td><td>91.7</td><td>91.75</td><td>90.85</td><td>91.25</td><td>45.81654</td><td>11163952</td><td>TRUE</td><td>TRUE</td><td></td><td></td></tr><tr><td>6</td><td>1/8/2010</td><td>91.5</td><td>92.15</td><td>91.35</td><td>91.7</td><td>46.04247</td><td>15683744</td><td>TRUE</td><td>TRUE</td><td></td><td></td></tr><tr><td>7</td><td>1/11/2010</td><td>92</td><td>92.8</td><td>92</td><td>92.4</td><td>46.39394</td><td>15729515</td><td>TRUE</td><td>TRUE</td><td></td><td></td></tr><tr><td>8</td><td>1/12/2010</td><td>91.8</td><td>92.4</td><td>91.05</td><td>91.65</td><td>46.01737</td><td>14005577</td><td>TRUE</td><td>TRUE</td><td></td><td></td></tr></table>

# How to solve?

- Check from other data sources (if available)   
- Delete the rows   
- Cap and floor the value with High and Low respectively

<table><tr><td></td><td colspan="2">A</td><td colspan="2">B</td><td colspan="2">C</td><td colspan="2">D</td><td colspan="2">E</td><td colspan="2">F</td><td colspan="2">G</td><td colspan="2">H</td><td colspan="2">I</td></tr><tr><td>1</td><td>Date</td><td>✓</td><td>Open</td><td>✓</td><td>High</td><td>✓</td><td>Low</td><td>✓</td><td>Close</td><td>✓</td><td>Adj Clos</td><td>✓</td><td>Volume</td><td>✓</td><td>C&lt;=H and C&gt;</td><td>✓</td><td>O&lt;=H and O&gt;</td><td>✓</td></tr><tr><td>11</td><td colspan="2">1/15/2010</td><td colspan="2">91</td><td colspan="2">90.3</td><td colspan="2">90.75</td><td colspan="2">91.05</td><td colspan="2">45.71613</td><td colspan="2">13190937</td><td colspan="2">FALSE</td><td colspan="2">FALSE</td></tr><tr><td>13</td><td colspan="2">1/19/2010</td><td colspan="2">89.05</td><td colspan="2">88.4</td><td colspan="2">88.7</td><td colspan="2">89.3</td><td colspan="2">44.83743</td><td colspan="2">17485152</td><td colspan="2">FALSE</td><td colspan="2">FALSE</td></tr><tr><td>314</td><td colspan="2">4/4/2011</td><td colspan="2">81.55</td><td colspan="2">82.45</td><td colspan="2">81.65</td><td colspan="2">82.2</td><td colspan="2">41.51557</td><td colspan="2">29888316</td><td colspan="2">TRUE</td><td colspan="2">FALSE</td></tr><tr><td>3452</td><td colspan="2"></td><td colspan="2"></td><td colspan="2"></td><td colspan="2"></td><td colspan="2"></td><td colspan="2"></td><td colspan="2"></td><td colspan="2"></td><td colspan="2"></td></tr></table>

# Moving Average Strategy

# Moving Average

- Definition: A moving average (MA) is a statistical calculation used to analyze data points by creating a series of averages of different subsets of the full data set.   
- Purpose: To smooth out short-term fluctuations and highlight longer-term trends in data.   
- Types of Moving Averages:

- Simple Moving Average (SMA)   
- Exponential Moving Average (EMA)

# Simple Moving Average (SMA)

- The average price over a specific number of periods $n$ .   
- Formula:

$$
\mathrm {M A} (n) = \frac {\sum_ {t = 1} ^ {n} P _ {t}}{n}
$$

• Example:

Given Prices: [10, 12, 14, 16, 18]   
5-Day SMA Calculation:

$$
S M A = \frac {1 0 + 1 2 + 1 4 + 1 6 + 1 8}{5} = 1 4
$$

# Exponential Moving Average (EMA)

- Gives more weight to recent prices, making it more responsive to new information   
- Formula:

$$
E M A _ {t} = P _ {t} \times k + (1 - k) \times E M A _ {t - 1}
$$

Commonly chosen

$$
\left\{ \begin{array}{l} k = \frac {2}{n + 1} \\ E M A _ {0} = P _ {0} \end{array} \right.
$$

# Simple MA Strategy

![](images/24256d8b2bcbc9066dc1ab9350faeed6131db6e736b52b2e9c79ecb5bc3a0ad2.jpg)  
Moving Averages

Trading Logic:

1. if Open $<$ MA and Close $>$ MA, then BUY   
2. If Open > MA and Close < MA, then SELL

![](images/a591712f1f7f3b1a9f0366ece2a16ed6dde8518a5431fba50a9d1428124ca08f.jpg)

# MA Cross Strategy

![](images/60e7fc5a614b32e13d1088f66b5f01c23952e9cdaa053567460ff3718f1e428c.jpg)

# MA Cross Strategy

- Define

fast MA as 7-day moving average   
- slow MA as 14-day moving average

- Based on a sliding window approach to collect the previous 14 closing price

Calculate fast MA and slow MA values   
- Open order conditions:

Golden Cross: BUY if fast MA(t-1) < slow MA(t-1) AND fast MA(t) > slow MA(t), OR   
• Death Cross: SELL if fast $\mathrm{MA}(t - 1) > \text{slow MA}(t - 1)$ AND fast $\mathrm{MA}(t) < \text{slow MA}(t)$

- Close order conditions:

- If previous BUY and now Death Cross appears, OR   
If previous SELL and now Golden Cross appears

- Repeat the same process until the backtest period ends

# Backtest Assumptions

1. No transaction cost   
2. Buy/sell 1 share each time   
3. Order entry/exit at market close   
4. Short selling is NOT allowed

# MA Cross Strategy in Excel

# 1. Calculate MA(7) and MA(14)

$$
\boxed {S M A _ {k} = \frac {p _ {n - k + 1} + p _ {n - k + 2} \cdots + p _ {n}}{k}}
$$

<table><tr><td colspan="2">SUM</td><td>:</td><td colspan="2">× √ f x</td><td colspan="7">=AVERAGE(F2:F8)</td></tr><tr><td></td><td>A</td><td>B</td><td>C</td><td>D</td><td>E</td><td>F</td><td>G</td><td>H</td><td>I</td><td>J</td><td></td></tr><tr><td>1</td><td>Date</td><td>Open</td><td>High</td><td>Low</td><td>Close</td><td>Adj Clos</td><td>Volume</td><td>MA(7)</td><td>MA(14)</td><td></td><td></td></tr><tr><td>2</td><td>1/4/2010</td><td>89.4</td><td>89.9</td><td>88.8</td><td>89.25</td><td>44.81234</td><td>10381759</td><td></td><td></td><td></td><td></td></tr><tr><td>3</td><td>1/5/2010</td><td>90.2</td><td>90.65</td><td>90.1</td><td>90.45</td><td>45.41485</td><td>16914189</td><td></td><td></td><td></td><td></td></tr><tr><td>4</td><td>1/6/2010</td><td>91.3</td><td>92</td><td>91.25</td><td>91.6</td><td>45.99229</td><td>20033996</td><td></td><td></td><td></td><td></td></tr><tr><td>5</td><td>1/7/2010</td><td>91.7</td><td>91.75</td><td>90.85</td><td>91.25</td><td>45.81654</td><td>11163952</td><td></td><td></td><td></td><td></td></tr><tr><td>6</td><td>1/8/2010</td><td>91.5</td><td>92.15</td><td>91.35</td><td>91.7</td><td>46.04247</td><td>15683744</td><td></td><td></td><td></td><td></td></tr><tr><td>7</td><td>1/11/2010</td><td>92</td><td>92.8</td><td>92</td><td>92.4</td><td>46.39394</td><td>15729515</td><td></td><td></td><td></td><td></td></tr><tr><td>8</td><td>1/12/2010</td><td>91.8</td><td>92.4</td><td>91.05</td><td>91.65</td><td>46.01737</td><td>14005577</td><td colspan="2">=AVERAGE(F2:F8)</td><td></td><td></td></tr><tr><td>9</td><td>1/13/2010</td><td>90.5</td><td>90.8</td><td>89.95</td><td>90.2</td><td>45.28933</td><td>17142233</td><td>45.8524</td><td></td><td></td><td></td></tr><tr><td>10</td><td>1/14/2010</td><td>90.65</td><td>91.2</td><td>90.65</td><td>90.85</td><td>45.61569</td><td>11379186</td><td>45.88109</td><td></td><td></td><td></td></tr><tr><td>11</td><td>1/18/2010</td><td>89.05</td><td>89.6</td><td>88.6</td><td>89.25</td><td>44.81234</td><td>16529101</td><td>45.71253</td><td></td><td></td><td></td></tr><tr><td>12</td><td>1/20/2010</td><td>88</td><td>88.45</td><td>87.65</td><td>87.85</td><td>44.10939</td><td>30098548</td><td>45.46865</td><td></td><td></td><td></td></tr><tr><td>13</td><td>1/21/2010</td><td>87.6</td><td>88.2</td><td>87</td><td>87.05</td><td>43.70772</td><td>20718144</td><td>45.13511</td><td></td><td></td><td></td></tr><tr><td>14</td><td>1/22/2010</td><td>85.5</td><td>86.05</td><td>85</td><td>85.7</td><td>43.02989</td><td>37434508</td><td>44.65453</td><td></td><td></td><td></td></tr><tr><td>15</td><td>1/25/2010</td><td>84</td><td>84.35</td><td>83.7</td><td>84.3</td><td>42.32695</td><td>27485326</td><td>44.12733</td><td>44.95579</td><td></td><td></td></tr><tr><td>16</td><td>1/26/2010</td><td>84.6</td><td>85.15</td><td>83.5</td><td>84.1</td><td>42.22653</td><td>24292298</td><td>43.68979</td><td>44.77109</td><td></td><td></td></tr><tr><td>17</td><td>1/27/2010</td><td>84</td><td>85.6</td><td>83.8</td><td>84.05</td><td>42.20142</td><td>19620776</td><td>43.20203</td><td>44.54156</td><td></td><td></td></tr><tr><td>18</td><td>1/28/2010</td><td>84.35</td><td>85.65</td><td>84.35</td><td>85.05</td><td>42.70354</td><td>18700737</td><td>42.90078</td><td>44.30665</td><td></td><td></td></tr><tr><td>19</td><td>1/29/2010</td><td>83.2</td><td>84.4</td><td>83.1</td><td>83.6</td><td>41.97548</td><td>22798052</td><td>42.59593</td><td>44.03229</td><td></td><td></td></tr><tr><td>20</td><td>2/1/2010</td><td>83</td><td>84.25</td><td>83</td><td>83.85</td><td>42.10099</td><td>15452820</td><td>42.3664</td><td>43.75076</td><td></td><td></td></tr><tr><td>21</td><td>2/2/2010</td><td>84.4</td><td>85.05</td><td>84.15</td><td>84.55</td><td>43.45348</td><td>13507341</td><td>43.38391</td><td>43.46933</td><td></td><td></td></tr></table>

![](images/694365fc56f4c60d3e01b05c7fda7e20adbfe9b1921b99f918bb53507454d504.jpg)  
Simple Moving Average

# MA Cross Strategy in Excel

2. Identify MA cross signal   
Buy signal   
• MA(7,t) > MA(14,t) and MA(7,t-1) < MA(14,t-1)   
Sell signal   
• $MA(7, t) < MA(14, t)$ and $MA(7, t - 1) > MA(14, t - 1)$ - $MA(7, t) < MA(14, t)$ and $MA(7, t - 1) > MA(14, t - 1)$

<table><tr><td colspan="2">J16</td><td>:</td><td colspan="2">× √ fx</td><td colspan="7">=IF(AND(H16&gt;I16,H15&lt;I15), 1, IF(AND(H16&lt;I16,H15&gt;I15), -1, 0))</td></tr><tr><td></td><td>A</td><td>B</td><td>C</td><td>D</td><td>E</td><td>F</td><td>G</td><td>H</td><td>I</td><td>J</td><td></td></tr><tr><td>1</td><td>Date</td><td>Open</td><td>High</td><td>Low</td><td>Close</td><td>Adj Clos</td><td>Volume</td><td>MA(7)</td><td>MA(14)</td><td>signal</td><td></td></tr><tr><td>2</td><td>1/4/2010</td><td>89.4</td><td>89.9</td><td>88.8</td><td>89.25</td><td>44.81234</td><td>10381759</td><td></td><td></td><td></td><td></td></tr><tr><td>3</td><td>1/5/2010</td><td>90.2</td><td>90.65</td><td>90.1</td><td>90.45</td><td>45.41485</td><td>16914189</td><td></td><td></td><td></td><td></td></tr><tr><td>4</td><td>1/6/2010</td><td>91.3</td><td>92</td><td>91.25</td><td>91.6</td><td>45.99229</td><td>20033996</td><td></td><td></td><td></td><td></td></tr><tr><td>5</td><td>1/7/2010</td><td>91.7</td><td>91.75</td><td>90.85</td><td>91.25</td><td>45.81654</td><td>11163952</td><td></td><td></td><td></td><td></td></tr><tr><td>6</td><td>1/8/2010</td><td>91.5</td><td>92.15</td><td>91.35</td><td>91.7</td><td>46.04247</td><td>15683744</td><td></td><td></td><td></td><td></td></tr><tr><td>7</td><td>1/11/2010</td><td>92</td><td>92.8</td><td>92</td><td>92.4</td><td>46.39394</td><td>15729515</td><td></td><td></td><td></td><td></td></tr><tr><td>8</td><td>1/12/2010</td><td>91.8</td><td>92.4</td><td>91.05</td><td>91.65</td><td>46.01737</td><td>14005577</td><td>45.78426</td><td></td><td></td><td></td></tr><tr><td>9</td><td>1/13/2010</td><td>90.5</td><td>90.8</td><td>89.95</td><td>90.2</td><td>45.28933</td><td>17142233</td><td>45.8524</td><td></td><td></td><td></td></tr><tr><td>10</td><td>1/14/2010</td><td>90.65</td><td>91.2</td><td>90.65</td><td>90.85</td><td>45.61569</td><td>11379186</td><td>45.88109</td><td></td><td></td><td></td></tr><tr><td>11</td><td>1/18/2010</td><td>89.05</td><td>89.6</td><td>88.6</td><td>89.25</td><td>44.81234</td><td>16529101</td><td>45.71253</td><td></td><td></td><td></td></tr><tr><td>12</td><td>1/20/2010</td><td>88</td><td>88.45</td><td>87.65</td><td>87.85</td><td>44.10939</td><td>30098548</td><td>45.46865</td><td></td><td></td><td></td></tr><tr><td>13</td><td>1/21/2010</td><td>87.6</td><td>88.2</td><td>87</td><td>87.05</td><td>43.70772</td><td>20718144</td><td>45.13511</td><td></td><td></td><td></td></tr><tr><td>14</td><td>1/22/2010</td><td>85.5</td><td>86.05</td><td>85</td><td>85.7</td><td>43.02989</td><td>37434508</td><td>44.65453</td><td></td><td></td><td></td></tr><tr><td>15</td><td>1/25/2010</td><td>84</td><td>84.35</td><td>83.7</td><td>84.3</td><td>42.32695</td><td>27485326</td><td>44.12733</td><td>44.95579</td><td></td><td></td></tr><tr><td>16</td><td>1/26/2010</td><td>84.6</td><td>85.15</td><td>83.5</td><td>84.1</td><td>42.22653</td><td>24292298</td><td>43.68979</td><td>44.77109</td><td>0</td><td></td></tr><tr><td>17</td><td>1/27/2010</td><td>84</td><td>85.6</td><td>83.8</td><td>84.05</td><td>42.20142</td><td>19620776</td><td>43.20203</td><td>44.54156</td><td>0</td><td></td></tr><tr><td>18</td><td>1/28/2010</td><td>84.35</td><td>85.65</td><td>84.35</td><td>85.05</td><td>42.70354</td><td>18700737</td><td>42.90078</td><td>44.30665</td><td>0</td><td></td></tr><tr><td>19</td><td>1/29/2010</td><td>83.2</td><td>84.4</td><td>83.1</td><td>83.6</td><td>41.97548</td><td>22798052</td><td>42.59593</td><td>44.03229</td><td>0</td><td></td></tr><tr><td>20</td><td>2/1/2010</td><td>83</td><td>84.25</td><td>83</td><td>83.85</td><td>42.10099</td><td>15452820</td><td>42.3664</td><td>43.75076</td><td>0</td><td></td></tr></table>

# MA Cross Strategy in Excel

# 3. Calculate day-end holding

<table><tr><td colspan="2">K3</td><td colspan="2">: √ √ f(x)</td><td colspan="9">=K2+IF(AND(K2=0,J3=-1), 0, J3)</td></tr><tr><td></td><td>A</td><td>B</td><td>C</td><td>D</td><td>E</td><td>F</td><td>G</td><td>H</td><td>I</td><td>J</td><td>K</td><td></td></tr><tr><td>1</td><td>Date</td><td>Open</td><td>High</td><td>Low</td><td>Close</td><td>Adj Clos</td><td>Volume</td><td>MA(7)</td><td>MA(14)</td><td>signal</td><td>Position</td><td></td></tr><tr><td>2</td><td>1/4/2010</td><td>89.4</td><td>89.9</td><td>88.8</td><td>89.25</td><td>44.81234</td><td>10381759</td><td></td><td></td><td></td><td>0</td><td></td></tr><tr><td>3</td><td>1/5/2010</td><td>90.2</td><td>90.65</td><td>90.1</td><td>90.45</td><td>45.41485</td><td>16914189</td><td></td><td></td><td></td><td>0</td><td></td></tr><tr><td>4</td><td>1/6/2010</td><td>91.3</td><td>92</td><td>91.25</td><td>91.6</td><td>45.99229</td><td>20033996</td><td></td><td></td><td></td><td>0</td><td></td></tr><tr><td>5</td><td>1/7/2010</td><td>91.7</td><td>91.75</td><td>90.85</td><td>91.25</td><td>45.81654</td><td>11163952</td><td></td><td></td><td></td><td>0</td><td></td></tr><tr><td>6</td><td>1/8/2010</td><td>91.5</td><td>92.15</td><td>91.35</td><td>91.7</td><td>46.04247</td><td>15683744</td><td></td><td></td><td></td><td>0</td><td></td></tr><tr><td>7</td><td>1/11/2010</td><td>92</td><td>92.8</td><td>92</td><td>92.4</td><td>46.39394</td><td>15729515</td><td></td><td></td><td></td><td>0</td><td></td></tr><tr><td>8</td><td>1/12/2010</td><td>91.8</td><td>92.4</td><td>91.05</td><td>91.65</td><td>46.01737</td><td>14005577</td><td>45.78426</td><td></td><td></td><td>0</td><td></td></tr><tr><td>9</td><td>1/13/2010</td><td>90.5</td><td>90.8</td><td>89.95</td><td>90.2</td><td>45.28933</td><td>17142233</td><td>45.8524</td><td></td><td></td><td>0</td><td></td></tr><tr><td>10</td><td>1/14/2010</td><td>90.65</td><td>91.2</td><td>90.65</td><td>90.85</td><td>45.61569</td><td>11379186</td><td>45.88109</td><td></td><td></td><td>0</td><td></td></tr><tr><td>11</td><td>1/18/2010</td><td>89.05</td><td>89.6</td><td>88.6</td><td>89.25</td><td>44.81234</td><td>16529101</td><td>45.71253</td><td></td><td></td><td>0</td><td></td></tr><tr><td>12</td><td>1/20/2010</td><td>88</td><td>88.45</td><td>87.65</td><td>87.85</td><td>44.10939</td><td>30098548</td><td>45.46865</td><td></td><td></td><td>0</td><td></td></tr><tr><td>13</td><td>1/21/2010</td><td>87.6</td><td>88.2</td><td>87</td><td>87.05</td><td>43.70772</td><td>20718144</td><td>45.13511</td><td></td><td></td><td>0</td><td></td></tr><tr><td>14</td><td>1/22/2010</td><td>85.5</td><td>86.05</td><td>85</td><td>85.7</td><td>43.02989</td><td>37434508</td><td>44.65453</td><td></td><td></td><td>0</td><td></td></tr><tr><td>15</td><td>1/25/2010</td><td>84</td><td>84.35</td><td>83.7</td><td>84.3</td><td>42.32695</td><td>27485326</td><td>44.12733</td><td>44.95579</td><td></td><td>0</td><td></td></tr><tr><td>16</td><td>1/26/2010</td><td>84.6</td><td>85.15</td><td>83.5</td><td>84.1</td><td>42.22653</td><td>24292298</td><td>43.68979</td><td>44.77109</td><td>0</td><td>0</td><td></td></tr><tr><td>17</td><td>1/27/2010</td><td>84</td><td>85.6</td><td>83.8</td><td>84.05</td><td>42.20142</td><td>19620776</td><td>43.20203</td><td>44.54156</td><td>0</td><td>0</td><td></td></tr><tr><td>18</td><td>1/28/2010</td><td>84.35</td><td>85.65</td><td>84.35</td><td>85.05</td><td>42.70354</td><td>18700737</td><td>42.90078</td><td>44.30665</td><td>0</td><td>0</td><td></td></tr><tr><td>19</td><td>1/29/2010</td><td>83.2</td><td>84.4</td><td>83.1</td><td>83.6</td><td>41.97548</td><td>22798052</td><td>42.59593</td><td>44.03229</td><td>0</td><td>0</td><td></td></tr></table>

# MA Cross Strategy in Excel

# 4. Calculate daily PnL

<table><tr><td colspan="2">L3</td><td>:</td><td colspan="2">X √ f(x)</td><td colspan="10">=K2*(F3-F2)</td></tr><tr><td></td><td>A</td><td>B</td><td>C</td><td>D</td><td>E</td><td>F</td><td>G</td><td>H</td><td>I</td><td>J</td><td>K</td><td>L</td><td>M</td><td></td></tr><tr><td>1</td><td>Date</td><td>Open</td><td>High</td><td>Low</td><td>Close</td><td>Adj Clos</td><td>Volume</td><td>MA(7)</td><td>MA(14)</td><td>signal</td><td>Position</td><td>Daily PnL</td><td>Cum PnL</td><td></td></tr><tr><td>2</td><td>1/4/2010</td><td>89.4</td><td>89.9</td><td>88.8</td><td>89.25</td><td>44.81234</td><td>10381759</td><td></td><td></td><td></td><td>0</td><td></td><td></td><td></td></tr><tr><td>3</td><td>1/5/2010</td><td>90.2</td><td>90.65</td><td>90.1</td><td>90.45</td><td>45.41485</td><td>16914189</td><td></td><td></td><td></td><td>0</td><td>0</td><td></td><td></td></tr><tr><td>4</td><td>1/6/2010</td><td>91.3</td><td>92</td><td>91.25</td><td>91.6</td><td>45.99229</td><td>20033996</td><td></td><td></td><td></td><td>0</td><td>0</td><td></td><td></td></tr><tr><td>5</td><td>1/7/2010</td><td>91.7</td><td>91.75</td><td>90.85</td><td>91.25</td><td>45.81654</td><td>11163952</td><td></td><td></td><td></td><td>0</td><td>0</td><td></td><td></td></tr><tr><td>6</td><td>1/8/2010</td><td>91.5</td><td>92.15</td><td>91.35</td><td>91.7</td><td>46.04247</td><td>15683744</td><td></td><td></td><td></td><td>0</td><td>0</td><td></td><td></td></tr><tr><td>7</td><td>1/11/2010</td><td>92</td><td>92.8</td><td>92</td><td>92.4</td><td>46.39394</td><td>15729515</td><td></td><td></td><td></td><td>0</td><td>0</td><td></td><td></td></tr><tr><td>8</td><td>1/12/2010</td><td>91.8</td><td>92.4</td><td>91.05</td><td>91.65</td><td>46.01737</td><td>14005577</td><td>45.78426</td><td></td><td></td><td>0</td><td>0</td><td></td><td></td></tr></table>

# 5. Calculate cumulative PnL

<table><tr><td colspan="2">M3</td><td>:</td><td colspan="2">X √ f(x)</td><td colspan="10">=M2+L3</td></tr><tr><td></td><td>A</td><td>B</td><td>C</td><td>D</td><td>E</td><td>F</td><td>G</td><td>H</td><td>I</td><td>J</td><td>K</td><td>L</td><td>M</td><td></td></tr><tr><td>1</td><td>Date</td><td>Open</td><td>High</td><td>Low</td><td>Close</td><td>Adj Clos</td><td>Volume</td><td>MA(7)</td><td>MA(14)</td><td>signal</td><td>Position</td><td>Daily PnL</td><td>Cum PnL</td><td></td></tr><tr><td>2</td><td>1/4/2010</td><td>89.4</td><td>89.9</td><td>88.8</td><td>89.25</td><td>44.81234</td><td>10381759</td><td></td><td></td><td></td><td>0</td><td></td><td>0</td><td></td></tr><tr><td>3</td><td>1/5/2010</td><td>90.2</td><td>90.65</td><td>90.1</td><td>90.45</td><td>45.41485</td><td>16914189</td><td></td><td></td><td></td><td>0</td><td>0</td><td>0</td><td></td></tr><tr><td>4</td><td>1/6/2010</td><td>91.3</td><td>92</td><td>91.25</td><td>91.6</td><td>45.99229</td><td>20033996</td><td></td><td></td><td></td><td>0</td><td>0</td><td>0</td><td></td></tr><tr><td>5</td><td>1/7/2010</td><td>91.7</td><td>91.75</td><td>90.85</td><td>91.25</td><td>45.81654</td><td>11163952</td><td></td><td></td><td></td><td>0</td><td>0</td><td>0</td><td></td></tr><tr><td>6</td><td>1/8/2010</td><td>91.5</td><td>92.15</td><td>91.35</td><td>91.7</td><td>46.04247</td><td>15683744</td><td></td><td></td><td></td><td>0</td><td>0</td><td>0</td><td></td></tr><tr><td>7</td><td>1/11/2010</td><td>92</td><td>92.8</td><td>92</td><td>92.4</td><td>46.39394</td><td>15729515</td><td></td><td></td><td></td><td>0</td><td>0</td><td>0</td><td></td></tr><tr><td>8</td><td>1/12/2010</td><td>91.8</td><td>92.4</td><td>91.05</td><td>91.65</td><td>46.01737</td><td>14005577</td><td>45.78426</td><td></td><td></td><td>0</td><td>0</td><td>0</td><td></td></tr><tr><td>9</td><td>1/13/2010</td><td>90.5</td><td>90.8</td><td>89.95</td><td>90.2</td><td>45.28933</td><td>17142233</td><td>45.8524</td><td></td><td></td><td>0</td><td>0</td><td>0</td><td></td></tr></table>

# Got a profitable strategy?

![](images/2dcc365ae691c7b5281959a18e9c7817c9c45e92dcbe9346833d7fed7c03172c.jpg)

# Does our assumptions make sense?

1. No transaction cost

- in fact, there is transaction cost in reality (eg. commission, swap fee, funding cost, tax, etc)

2. Buy/sell 1 share each time

3. Order entry/exit at market close

- can we really trade at the last moment before market close? Why not execute at market open?

4. Short selling is NOT allowed

- short-selling stocks are not easily executable in the market due to

- Regulation requirements   
Some brokers require extra cash as collateral

![](images/a15a043627453641a772d591d601dcb905b70dfb6f71171a43571574a9cee510.jpg)

# MA Cross Strategy in Excel

- Adjust for realistic assumptions

- Bid ask spread for 0005.HK is usually around 0.1   
HK stock stamp duty = 0.1%   
Commission around $0.02\%$

Market price is around $100 so stamp duty and commission is equivalent to 100 \%$ (0.1% + 0.02%) = 0.12 point value. Adding the 0.1 spread cost will be 0.22

Let's assume 0.25 price point for conservative measure

![](images/fd3286d264678ce0f972700118323145e6d93e87c25cf9d5b2047c6940a59e29.jpg)

# MA Cross Strategy in Excel

<table><tr><td colspan="2">L3</td><td>:</td><td colspan="2">X √ f x</td><td colspan="10">=K2*(F3-F2) -IF(K2&lt;&gt;K3,0.25,0)</td></tr><tr><td></td><td>A</td><td>B</td><td>C</td><td>D</td><td>E</td><td>F</td><td>G</td><td>H</td><td>I</td><td>J</td><td>K</td><td>L</td><td>M</td><td></td></tr><tr><td>1</td><td>Date</td><td>Open</td><td>High</td><td>Low</td><td>Close</td><td>Adj Clos</td><td>Volume</td><td>MA(7)</td><td>MA(14)</td><td>signal</td><td>Position</td><td>Daily PnL</td><td>Cum PnL</td><td></td></tr><tr><td>2</td><td>4/1/2010</td><td>89.4</td><td>89.9</td><td>88.8</td><td>89.25</td><td>44.81234</td><td>10381759</td><td></td><td></td><td></td><td>0</td><td></td><td>0</td><td></td></tr><tr><td>3</td><td>5/1/2010</td><td>90.2</td><td>90.65</td><td>90.1</td><td>90.45</td><td>45.41485</td><td>16914189</td><td></td><td></td><td></td><td>0</td><td>0</td><td>0</td><td></td></tr><tr><td>4</td><td>6/1/2010</td><td>91.3</td><td>92</td><td>91.25</td><td>91.6</td><td>45.99229</td><td>20033996</td><td></td><td></td><td></td><td>0</td><td>0</td><td>0</td><td></td></tr><tr><td>5</td><td>7/1/2010</td><td>91.7</td><td>91.75</td><td>90.85</td><td>91.25</td><td>45.81654</td><td>11163952</td><td></td><td></td><td></td><td>0</td><td>0</td><td>0</td><td></td></tr><tr><td>6</td><td>8/1/2010</td><td>91.5</td><td>92.15</td><td>91.35</td><td>91.7</td><td>46.04247</td><td>15683744</td><td></td><td></td><td></td><td>0</td><td>0</td><td>0</td><td></td></tr><tr><td>7</td><td>11/1/2010</td><td>92</td><td>92.8</td><td>92</td><td>92.4</td><td>46.39394</td><td>15729515</td><td></td><td></td><td></td><td>0</td><td>0</td><td>0</td><td></td></tr><tr><td>8</td><td>12/1/2010</td><td>91.8</td><td>92.4</td><td>91.05</td><td>91.65</td><td>46.01737</td><td>14005577</td><td>45.78426</td><td></td><td></td><td>0</td><td>0</td><td>0</td><td></td></tr><tr><td>9</td><td>13/1/2010</td><td>90.5</td><td>90.8</td><td>89.95</td><td>90.2</td><td>45.28933</td><td>17142233</td><td>45.8524</td><td></td><td></td><td>0</td><td>0</td><td>0</td><td></td></tr><tr><td>10</td><td>14/1/2010</td><td>90.65</td><td>91.2</td><td>90.65</td><td>90.85</td><td>45.61569</td><td>11379186</td><td>45.88109</td><td></td><td></td><td>0</td><td>0</td><td>0</td><td></td></tr><tr><td>11</td><td>18/1/2010</td><td>89.05</td><td>89.6</td><td>88.6</td><td>89.25</td><td>44.81234</td><td>16529101</td><td>45.71253</td><td></td><td></td><td>0</td><td>0</td><td>0</td><td></td></tr><tr><td>12</td><td>20/1/2010</td><td>88</td><td>88.45</td><td>87.65</td><td>87.85</td><td>44.10939</td><td>30098548</td><td>45.46865</td><td></td><td></td><td>0</td><td>0</td><td>0</td><td></td></tr><tr><td>13</td><td>21/1/2010</td><td>87.6</td><td>88.2</td><td>87</td><td>87.05</td><td>43.70772</td><td>20718144</td><td>45.13511</td><td></td><td></td><td>0</td><td>0</td><td>0</td><td></td></tr><tr><td>14</td><td>22/1/2010</td><td>85.5</td><td>86.05</td><td>85</td><td>85.7</td><td>43.02989</td><td>37434508</td><td>44.65453</td><td></td><td></td><td>0</td><td>0</td><td>0</td><td></td></tr><tr><td>15</td><td>25/1/2010</td><td>84</td><td>84.35</td><td>83.7</td><td>84.3</td><td>42.32695</td><td>27485326</td><td>44.12733</td><td>44.95579</td><td></td><td>0</td><td>0</td><td>0</td><td></td></tr><tr><td>16</td><td>26/1/2010</td><td>84.6</td><td>85.15</td><td>83.5</td><td>84.1</td><td>42.22653</td><td>24292298</td><td>43.68979</td><td>44.77109</td><td>0</td><td>0</td><td>0</td><td>0</td><td></td></tr><tr><td>17</td><td>27/1/2010</td><td>84</td><td>85.6</td><td>83.8</td><td>84.05</td><td>42.20142</td><td>19620776</td><td>43.20203</td><td>44.54156</td><td>0</td><td>0</td><td>0</td><td>0</td><td></td></tr><tr><td>18</td><td>28/1/2010</td><td>84.35</td><td>85.65</td><td>84.35</td><td>85.05</td><td>42.70354</td><td>18700737</td><td>42.90078</td><td>44.30665</td><td>0</td><td>0</td><td>0</td><td>0</td><td></td></tr><tr><td>19</td><td>29/1/2010</td><td>83.2</td><td>84.4</td><td>83.1</td><td>83.6</td><td>41.97548</td><td>22798052</td><td>42.59593</td><td>44.03229</td><td>0</td><td>0</td><td>0</td><td>0</td><td></td></tr></table>

# Result not good after factoring in costs

![](images/99f0eda4befca83ed79ab8fab6d811f54bb16db56c1f1dc2e299810c18a6ff20.jpg)  
Cum PnL

# Backtest with Python

# Technical Indicator

# TA-LIB

Official Document: https://pypi.org/project/TA-Lib/   
- An open source library with 120+ technical indicators, such as MA, RSI, Bollinger Bands, etc

![](images/d90802b3a4adabfbeea07da74eb1152752157fb3a96fc803470d8e51292ba694.jpg)

pip install Ta-Lib

# MA - Moving average

real $=$ MA(close，timeperiod=30，matype $\equiv 0$

# RSI - Relative Strength Index

NOTE: The RSI function has an unstable period.

real $=$ RSI(close，timeperiod=14)

# BBANDS - Bollinger Bands

upperband, middleband, lowerband = BBANDS(close, timeperiod=5, nbdevup=2, nbdevdn=2, matype:

# Alternative Installation Method

- Download the wheel file that match with your python version https://github.com/cgohlke/talib-build/releases (eg. ta_lib-0.6.3-cp39-cp39-win_amd64.whl)   
- Then install from your downloaded directory

![](images/b5f99233625ccea822c39091c3c67aee734bcfc045ec1c3e276ee3d7cb6cbe07.jpg)

pip install ta_lib-0.6.3-cp39-cp39-win_amd64.whl

# Example

• Given a list of price series [10, 11, 12, 15, 14, 13, 16, 17, 19, 17] and MA period = 5

from talib import MA

import numpy as np

close = [10.0, 11.0, 12.0, 15.0, 14.0, 13.0, 16.0, 17.0, 19.0, 17.0]

def my_MA(arr, period):

out $\equiv$ []

for i in range(0,len(arr)):

tmp = arr[max(0,i+1-period):i+1]

if len(tmp) < period:

out.append(None)

else:

value = sum(tmp)/period

out append(value)

return out

ma1 = my_MA(close, 5)

ma2 = MA(np.array(close), timeperiod=5)

print("ma1="，ma1)

print("ma2="，ma2)

ma1= [None, None, None, None, 12.4, 13.0, 14.0, 15.0, 15.8, 16.4]  
ma2= [nan nan nan nan nan 12.4 13. 14. 15. 15.8 16.4]

# backtesting.py

- Open sourced python library for backtesting strategy   
- Official Doc: https://github.com/kernc/backtesting.py/tree/master

![](images/1cddc1ff545455ba599a055ce31267f583fef5b1e918a5a3e6c0719c9aeef6eb.jpg)

# Example

from backtesting import Backtest, Strategy

from backtesting.lib import crossover

from backtesting.test import SMA, GOOG

class SmaCross(Strategy):

definit(self):

price = self.data.Close

self.ma1 = self.I(SMA, price, 10)

self.ma2 = self.I(SMA, price, 20)

def next(self):

if crossover(self.ma1, self.ma2): self.buy()

elif crossover(self.ma2, self.ma1):
    selfsell()

bt = Backtest(GOOG, SmaCross, cash=10000, commission=0.002, exclusive Orders=True)

```c
stats = bt.run()

print/stats)

bt.plot()

Import CrossOver function

Retrieve GOOG stock data

Import SMA function

Open buy when 10-day MA cross and $>20$ -day MA

Open sell when 20-day MA cross and >10-day MA

Initial cash = 10000

Commission = 0.002%

exclusive_order: True will close previous order when new signal appears

<table><tr><td>Start</td><td>2004-08-19 00:00:00</td></tr><tr><td>End</td><td>2013-03-01 00:00:00</td></tr><tr><td>Duration</td><td>3116 days 00:00:00</td></tr><tr><td>Exposure Time [%]</td><td>97.067039</td></tr><tr><td>Equity Final[$]</td><td>68221.96986</td></tr><tr><td>Equity Peak[$]</td><td>68991.21986</td></tr><tr><td>Return [%]</td><td>582.219699</td></tr><tr><td>Buy &amp; Hold Return [%]</td><td>703.458242</td></tr><tr><td>Return (Ann.) [%]</td><td>25.266427</td></tr><tr><td>Volatility (Ann.) [%]</td><td>38.383008</td></tr><tr><td>Sharpe Ratio</td><td>0.658271</td></tr><tr><td>Sortino Ratio</td><td>1.288779</td></tr><tr><td>Calmar Ratio</td><td>0.763748</td></tr><tr><td>Max. Drawdown [%]</td><td>-33.082172</td></tr><tr><td>Avg. Drawdown [%]</td><td>-5.581506</td></tr><tr><td>Max. Drawdown Duration</td><td>688 days 00:00:00</td></tr><tr><td>Avg. Drawdown Duration # Trades</td><td>41 days 00:00:00</td></tr><tr><td>Win Rate [%]</td><td>54.255319</td></tr><tr><td>Best Trade [%]</td><td>57.11931</td></tr><tr><td>Worst Trade [%]</td><td>-16.629898</td></tr><tr><td>Avg. Trade [%]</td><td>2.074326</td></tr><tr><td>Max. Trade Duration</td><td>121 days 00:00:00</td></tr><tr><td>Avg. Trade Duration Profit Factor</td><td>33 days 00:00:00</td></tr><tr><td>Expectancy [%]</td><td>2.190805</td></tr><tr><td>SQN</td><td>2.606294</td></tr><tr><td>Strategy</td><td>1.990216</td></tr><tr><td>_equityOUStrades</td><td>SmaCross</td></tr><tr><td>Size</td><td>EntryB...</td></tr></table>

![](images/9b843694b02a9d57ad67c47760184c11f303568cddd0d8796e505f7337f65a16.jpg)

# Remarks

- Only GOOG and EURUSD data are preinstalled   
- Need to maintain your data sources   
- .csv columns need to be in the order of [Date,Open,High,Low,Close,Volume]

![](images/e3e6320ec46ffe7136d53851c7a4187a0200dde804ab4829da13bb5bdae7bf5f.jpg)

# backtesting.py with custom data

from backtesting import Backtest, Strategy

from backtesting.lib import crossover

from backtesting.test import SMA#, GOOG

import yfinance as yf

from datetime import datetime

import pandas as pd

class SmaCross(Strategy):

def init(self):

price = self.data.Close

self.ma1 = self.I(SMA, price, 10)

self.ma2 = self.I(SMA, price, 20)

def next(self):

if crossover(self.ma1, self.ma2): self.buy()

elif crossover(self.ma2, self.ma1):
    selfsell()

# download data from yahoo finance

```python
tstart = datetime(2020,1,1)

tend = datetime(2025,6,30)

data = yf.download("0005.HK", start=tstart, end=tend)

Flatten the Multilndex columns if necessary

if isinstance(data.columns, pd.MultiIndex):

data.columns = [col[0].strip() for col in data.columns.values]

bt = Backtest(data, SmaCross, commission=0.002, exclusive Orders=True)

```python
stats = bt.run()

print/stats)

bt.plot()

<table><tr><td>Start</td><td>2020-01-02 00:00:00</td></tr><tr><td>End</td><td>2025-06-27 00:00:00</td></tr><tr><td>Duration</td><td>2003 days 00:00:00</td></tr><tr><td>Exposure Time [%]</td><td>97.33136</td></tr><tr><td>Equity Final [\$]</td><td>9999.42246</td></tr><tr><td>Equity Peak [\$]</td><td>17202.56084</td></tr><tr><td>Commissions [\$]</td><td>2981.18669</td></tr><tr><td>Return [%]</td><td>-0.00578</td></tr><tr><td>Buy &amp; Hold Return [%]</td><td>117.68325</td></tr><tr><td>Return (Ann.) [%]</td><td>-0.00108</td></tr><tr><td>Volatility (Ann.) [%]</td><td>25.43186</td></tr><tr><td>CAGR [%]</td><td>-0.00073</td></tr><tr><td>Sharpe Ratio</td><td>-0.00004</td></tr><tr><td>Sortino Ratio</td><td>-0.00006</td></tr><tr><td>Calmar Ratio</td><td>-0.00002</td></tr><tr><td>Max. Drawdown [%]</td><td>-46.52673</td></tr><tr><td>Avg. Drawdown [%]</td><td>-8.84256</td></tr><tr><td>Max. Drawdown Duration</td><td>1666 days 00:00:00</td></tr><tr><td>Avg. Drawdown Duration</td><td>161 days 00:00:00</td></tr><tr><td># Trades</td><td>59</td></tr><tr><td>Win Rate [%]</td><td>30.50847</td></tr><tr><td>Best Trade [%]</td><td>35.58777</td></tr><tr><td>Worst Trade [%]</td><td>-17.75267</td></tr><tr><td>Avg. Trade [%]</td><td>0.47075</td></tr><tr><td>Max. Trade Duration</td><td>140 days 00:00:00</td></tr><tr><td>Avg. Trade Duration</td><td>34 days 00:00:00</td></tr><tr><td>Profit Factor</td><td>1.33072</td></tr><tr><td>Expectancy [%]</td><td>0.93555</td></tr><tr><td>SQN</td><td>0.37152</td></tr><tr><td>Kelly Criterion</td><td>0.04138</td></tr><tr><td>Strategy</td><td>SmaCross</td></tr><tr><td>_equity_trace</td><td>...</td></tr><tr><td>_trades</td><td>Size EntryB...</td></tr></table>

![](images/bc0910083dd7a224a386d34891ba23d45ec3b5a143e4ba443dbbefe256fe1282.jpg)

# backtesting.py with custom strategy

from backtesting import Backtest, Strategy

import yfinance as yf

from datetime import datetime

import pandas as pd

class my_MACross(Strategy):

def init(self):

super().init()

def next(self):

if self data signal $= = 1$

self.buy()

elif self.data_signal==-1:

self sell()

download data

```python
tstart = datetime(2020,1,1)

tend = datetime(2025,6,30)

df = yf.download("0005.HK", start=tstart, end=tend)

Flatten the Multilndex columns if necessary

if isinstance(df.columns, pd.MultiIndex):

df.columns = [col[0].strip() for col in df.columns.values]

create our trading singal

df['MA_fast'] = df['Close'].rolling(7).mean()

df['MA Slow'] = df['Close'].rolling(14).mean()

drop NA records

df.dropna(inplace=True)

Create the signal column

df['signal'] = 0

df.loc[(df['MA_fast'] > df['MAslow'])& (df['MA_fast'].shift(1) <= df['MA Slow'].shift(1)), 'signal'] = 1 # Golden Cross

df.loc[(df['MA_fast'] < df['MA Slow']) & (df['MA_fast'].shift(1) >= df['MA_fast'].shift(1)), 'signal'] = -1 # Death Cross

print('df=',df)

bt = Backtest(df, my MacOSCross, commission=0.002, exclusive Orders=True)

[ \text{stats} = \text{bt.run()} ]

print/stats)

bt.plot()

<table><tr><td>Start</td><td>2020-01-21 00:00:00</td></tr><tr><td>End</td><td>2025-06-27 00:00:00</td></tr><tr><td>Duration</td><td>1984 days 00:00:00</td></tr><tr><td>Exposure Time [%]</td><td>98.87725</td></tr><tr><td>Equity Final[$]</td><td>11348.48556</td></tr><tr><td>Equity Peak[$]</td><td>17775.8496</td></tr><tr><td>Commissions[$]</td><td>4884.52945</td></tr><tr><td>Return [%]</td><td>13.48486</td></tr><tr><td>Buy &amp; Hold Return [%]</td><td>109.21881</td></tr><tr><td>Return (Ann.) [%]</td><td>2.41476</td></tr><tr><td>Volatility (Ann.) [%]</td><td>26.03221</td></tr><tr><td>CAGR [%]</td><td>1.61972</td></tr><tr><td>Sharpe Ratio</td><td>0.09276</td></tr><tr><td>Sortino Ratio</td><td>0.13548</td></tr><tr><td>Calmar Ratio</td><td>0.0646</td></tr><tr><td>Max. Drawdown [%]</td><td>-37.38035</td></tr><tr><td>Avg. Drawdown [%]</td><td>-6.68342</td></tr><tr><td>Max. Drawdown Duration</td><td>659 days 00:00:00</td></tr><tr><td>Avg. Drawdown Duration</td><td>77 days 00:00:00</td></tr><tr><td># Trades</td><td>94</td></tr><tr><td>Win Rate [%]</td><td>40.42553</td></tr><tr><td>Best Trade [%]</td><td>31.33929</td></tr><tr><td>Worst Trade [%]</td><td>-17.41497</td></tr><tr><td>Avg. Trade [%]</td><td>0.53888</td></tr><tr><td>Max. Trade Duration</td><td>111 days 00:00:00</td></tr><tr><td>Avg. Trade Duration</td><td>21 days 00:00:00</td></tr><tr><td>Profit Factor</td><td>1.3844</td></tr><tr><td>Expectancy [%]</td><td>0.83564</td></tr><tr><td>SQN</td><td>0.67682</td></tr><tr><td>Kelly Criterion</td><td>0.07645</td></tr><tr><td>_strategy</td><td>my MacOS cross</td></tr><tr><td>_equityOUS</td><td>...</td></tr><tr><td>_trades</td><td>Size EntryB...</td></tr></table>

![](images/ed47a5eb9be5d81b0754cbe07c4c437a90e52ef212d13de3eacdc3b0c677d702.jpg)

# Backtest with ALGOGENE

https://algogene.com/

# What can ALGOGENE do?

# It supports the whole algo-trading life cycle

![](images/b1b4dbc8cccc7d7eb85c01bf80676f73a2886a5b362e2b26f3ee2cd3841af45f.jpg)

20+ years market data (eg. Crypto, Commodity, FX, Equity, etc)   
Alternative datasets (eg. news, economic statistics, etc)

Powerful Python framework   
Dynamic portfolio level backtest   
External data/library plugin

Real-time simulation   
- Connect to broker's demo account   
No code revise and quick deploy

- Connect and execute to $30+$ real broker accounts   
24x7 cloud hosting

Multi-broker, multiaccount, multi-algo management   
Multi-dimensional analytics dashboard

# Data Exploration

- Go to [My History] > [Jupyter Notebook]

![](images/7a7ef9aebed0f34effcfe2fda6753ebc01160a3f01d55e6e6d390262f43bf7d8.jpg)

![](images/1e7224dbb875215725483dab4948ffaeb500201bc22d1d9312f97421edfd6415.jpg)

# Get historical data

# Data query on cloud

# (https://algogene.com/TechDoc#QueryHistory)

from AlgoAPI.AlgoAPIUtil import getHistoricalBar

data = getHistoricalBar(

contract={'instrument":"00005HK"},

numOfBar=100,

interval $\vDash$ 'D'

timestamp="2024-06-28"

）

for t in data:

print(t, data[t])

File

Edit

View

Insert

Cell

Kernel

Widgets

Help

Not Trusted

Python 3

![](images/724653dec85a19fea927a62ce1be3793099f78ef51bd271a754052d237e34d33.jpg)

In [1]: from AlgoAPI.AlgoAPIUtil import getHistoricalBar

```python
data = getHistoricalBar(
    contract={'instrument':'00005HK'},
    numOfBar=100,
    interval='D',
    timestamp='2024-06-28' 
```

```txt
for t in data: print(t, data[t]) 
```

```csv
2024-02-01 00:00:00 {'t': '2024-02-01 00:00:00', 'o': 61.0, 'h': 61.7, 'l': 61.0, 'c': 61.25, 'b': 61.23, 'a': 61.27, 'm': 61.25, 'v': 321.0, 'instrument': '00'  
005HK', 'expiry': '', 'right': '', 'strike': 0, 'symbol': '00005HK', 'bb': [],  
'ab': []}  
2024-02-02 00:00:00 {'t': '2024-02-02 00:00:00', 'o': 60.45, 'h': 61.400000000006, 'l': 60.2, 'c': 60.9, 'b': 60.88, 'a': 60.92, 'm': 60.9, 'v': 469.0, 'instrument': '00005HK', 'expiry': '', 'right': '', 'strike': 0, 'symbol': '00005HK',  
'bb': [], 'ab': []}  
2024-02-05 00:00:00 {'t': '2024-02-05 00:00:00', 'o': 61.2, 'h': 61.6, 'l': 60.75, 'c': 61.355, 'b': 61.33, 'a': 61.38, 'm': 61.355, 'v': 507.0, 'instrument':  
'00005HK', 'expiry': '', 'right': '', 'strike': 0, 'symbol': '00005HK', 'bb': [],  
'ab': []}  
2024-02-06 00:00:00 {'t': '2024-02-06 00:00:00', 'o': 61.2, 'h': 61.6, 'l': 60.475, 'c': 61.045, 'b': 61.02, 'a': 61.07, 'm': 61.045, 'v': 930.0, 'instrument':  
'00005HK', 'expiry': '', 'right': '', 'strike': 0, 'symbol': '00005HK', 'bb': 
```

# Format data with DataFrame

import pandas as pd

df = pd.DataFrame(data).T.reset_index.drop=True)  
df.drop(['strike','right','expiry'], axis=1)

print(df)

File

Edit

View

Insert

Cell

Kernel

Widgets

Help

Not Trusted

Python 3

![](images/a59c1a87617a424b142ec0e44eba5416ac26a91cdddaed72dcb18ad0bfe79168.jpg)

In [2]:

import pandas as pd

df = pd.DataFrame(data).T.reset_index.drop=True)

df = df.drop(['strike', 'right', 'expiry'], axis=1)

print(df)

<table><tr><td></td><td>t</td><td>o</td><td>h</td><td>l</td><td>c</td><td>b</td><td>a</td><td>m</td></tr><tr><td>0</td><td>2024-02-01</td><td>00:00:00</td><td>61.0</td><td>61.7</td><td>61.0</td><td>61.25</td><td>61.23</td><td>61.27</td></tr><tr><td>1</td><td>2024-02-02</td><td>00:00:00</td><td>60.45</td><td>61.4</td><td>60.2</td><td>60.9</td><td>60.88</td><td>60.92</td></tr><tr><td>2</td><td>2024-02-05</td><td>00:00:00</td><td>61.2</td><td>61.6</td><td>60.75</td><td>61.355</td><td>61.33</td><td>61.38</td></tr><tr><td>3</td><td>2024-02-06</td><td>00:00:00</td><td>61.2</td><td>61.6</td><td>60.475</td><td>61.045</td><td>61.02</td><td>61.07</td></tr><tr><td>4</td><td>2024-02-07</td><td>00:00:00</td><td>61.0</td><td>61.825</td><td>60.2</td><td>61.705</td><td>61.68</td><td>61.73</td></tr><tr><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td></tr><tr><td>95</td><td>2024-06-24</td><td>00:00:00</td><td>68.55</td><td>68.775</td><td>67.975</td><td>68.15</td><td>68.15</td><td>68.15</td></tr><tr><td>96</td><td>2024-06-25</td><td>00:00:00</td><td>68.55</td><td>68.775</td><td>67.225</td><td>67.85</td><td>67.82</td><td>67.88</td></tr><tr><td>97</td><td>2024-06-26</td><td>00:00:00</td><td>68.2</td><td>68.575</td><td>67.95</td><td>68.4</td><td>68.4</td><td>68.4</td></tr><tr><td>98</td><td>2024-06-27</td><td>00:00:00</td><td>69.5</td><td>69.5</td><td>67.5</td><td>68.3</td><td>68.27</td><td>68.33</td></tr><tr><td>99</td><td>2024-06-28</td><td>00:00:00</td><td>64.75</td><td>68.625</td><td>64.75</td><td>68.3</td><td>68.27</td><td>68.33</td></tr></table>

# Get historical data via REST API

- Go to [Settings] > [Profile]   
- Get your [UserID]   
- [Generate] a REST API Key

![](images/84c0f9dc08cb9e6cd447738bdce6c9b41aa219059e8ff58418fc410e30c9a624.jpg)

# Get historical data via REST API

https://algogene.com/RestDoc#/operations/get-query marketprice

import requests

```hcl
url = "https://algogene.com/rest/v1/history_price"  
params = {  
    "user":"XXX",  
    "api_key":"XXX",  
    "count":"100",  
    "interval":"D",  
    "instrument":"00005HK",  
    "timestamp":"2024-06-28"  
} 
```

headers = {"Content-Type": "application/json"}

response = requests.request("GET", url, headers=headers, params=params)  
print(response.text)

```txt
{"count":100,"res":[{a":60.726463,"ab": [],b":60.686818,"bb": [],c":60.70664,"expiry":"","h":61.152648,"instrument":"00005HK","l":60.458858,"m":60.70664,"o":60.458858,"right":"","strike":0,"symbol":"00005HK","t":"2024-02-01"  
00:00:00","v":321.0},{"a":60.379568,"ab": [],b":60.339923,"bb": [],c":60.359745,"expiry":"","h":60.85531,"instrument":"00005HK","l":59.665955,"m":60.359745,"o":59.913737,"right":"","strike":0,"symbol":"00005HK","t":"2024-02-02"  
00:00:00","v":469.0},{"a":60.835487,"ab": [],b":60.78593,"bb": [],c":60.810709,"expiry":"","h":61.053535,"instrument":"00005HK","l":60.211076,"m":60.810709,"o":60.657084,"right":"","strike":0,"symbol":"00005HK","t":"2024-02-05"  
00:00:00","v":507.0},{"a":60.528237,"ab": [],b":60.478681,"bb": [],c":60.503459,"expiry":"","h":61.053535,"instrument":"00005HK","l":59.938515,"m":60.503459,"o":60.657084,"right":"","strike":0,"symbol":"00005HK","t":"2024-02-06"  
00:00:00","v":930.0},{"a":61.182382,"ab": [],b":61.132826,"bb": [],c":61.157604,"expiry":"","h":61.276539,"instrument":"00005HK","l":59.665955,"m":61.157604,"o":60.458858,"right":"","strike":0,"symbol":"00005HK","t":"2024-02-07"  
00:00:00","v":447.0},{"a":61.618479,"ab": [],b":61.568922,"bb": [],c":61.5937,"expiry":"","h":61.945551,"instrument":"00005HK","l":61.202205,"m":61.5937,"o":61.202205,"right":"","strike":0,"symbol":"00005HK","t":"2 2Q24-02-08"  
O:O:O:O","v":46O.O,{"a":61.122914,"ab": [],b":61.073358,"bb": [],c":61.098136,"expiry":"","h":61.623434," instrument":"  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' ' 
O:O:O:O:"v":461. O,{"a":6O.776O19,ab:"[]",b":6O.726463,bb:"[]",c':6O.751241,expiry":"","h":61.449987, instrument":"  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  "  " 
O:O:O:O:"v":2O4.O,{"a":6O.28O455,ab:"[]",b":6O.23O898,bb:"[]",c':6O.255677,expiry":"","h":61.449987, instrument":"  "  "  "  "  "  "  "  "  " 
O:O:O:O:"v":521.O,{"a":6O.835487,ab:"[]",b":6O.78593,bb:"[]",c':6O.81O7O9,expiry":"","h":61. O28757,i nstrument":"  "  "  "  "  " 
```

```jsonl
00:00:00", "v":369.0}, {"a":61.430164, "ab": [], "b":61.380608, "bb": [], "c":61.405386, "expiry": "", "h":62.193333, "instrument": "00005HK", "l":59.963294, "m":61.405386, "o":60.954422, "right": "", "strike": 0, "symbol": "00005HK", "t": "2024-02-19
00:00:00", "v":503.0}, {"a":61.846438, "ab": [], "b":61.846438, "bb": [], "c":61.846438, "expiry": "", "h":62.193333, "instrument": "00005HK", "l":59.963294, "m":61.846438, "o":60.954422, "right": "", "strike": 0, "symbol": "00005HK", "t": "2024-02-20
00:00:00", "v":1127.0}, {"a":62.114043, "ab": [], "b":62.064487, "bb": [], "c":62.089265, "expiry": "", "h":62.267668, "instrument": "00005HK", "l":59.963294, "m":62.089265, "o":62.24289, "right": "", "strike": 0, "symbol": "00005HK", "t": "2024-02-21
00:00:00", "v":344.0}, {"a":59.715511, "ab": [], "b":59.715511, "bb": [], "c":59.715511, "expiry": "", "h":62.892079, "instrument": "00005HK", "l":59.51233, "m":59.715511, "o":62.441116, "right": "", "strike": 0, "symbol": "00005HK", "t": "2024-02-22
00:00:00", "v":776.0}, {"a":59.0911, "ab": [], "b":59.051455, "bb": [], "c":59.071278, "expiry": "", "h":59.418173, "in 
```

# Visualize Data with Matplotlib

- Matplotlib is a comprehensive Python library for creating static, animated, and interactive charts.   
- It is highly customizable and integrates well with other libraries like NumPy and Pandas.   
- Official Doc: https://matplotlib.org/stable/tutorials/pyplot.html

![](images/009e59acec15d925b97ccea12a5dfbde6df0baf5e6e4b8c9a10875359a0abee0.jpg)

# Create Line Plots

import matplotlib.pyplot as plt  
import numpy as np

$\mathrm{x} = \mathrm{np}$ .linspace(0, 10, 100)  
y1 = np.sin(x)  
y2 = np.cos(x)

plt.plot(x, y1, label='sin(x)')  
plt.plot(x, y2, label='cos(x)')

plt.title('Line Plots of sin(x) and cos(x)')  
plt.xlabel('X-axis')  
pltylabel('Y-axis')

plt.legend()  
plt.show()

![](images/ff3313fbceea621020eedce4a39b470fd59f7e1fa2ad889489f676a9842bf866.jpg)

# Visualize Stock Data

%%matplotlib inline import matplotlib

from matplotlib import pyplot as plt

$$
x = d f [ ^ {\prime} t ^ {\prime} ]
$$

$$
y = d f [ ^ {\prime} c ^ {\prime} ]
$$

title = plt.title('Closing Price')

plt.plot(x, y, '-', label="00005HK")

plt.style.use('darkbackground')

plt.xicks(rotation=90, fontweight='light', fontsize='x-small')

plt.show()

![](images/b374e7fdcf90d8f076ad8e71aff8118b9377e0b755dfade1d31a42fb551d857b.jpg)

![](images/1e8f7e61e9b6e589c5cbc8a0e13477f6df0caa1f71572c62476e10fa7bca7909.jpg)

# Backtest Interface

3. Technical Doc

# 4. Backtest results

![](images/006356d42dfe257be05b3037ef54fd3639bf454a658371cebbc8f772c1b67d2c.jpg)

# Backtest Framework

```python
1 from AlgoAPI import AlgoAPIUtil, AlgoAPI_Backtest   
2   
3 class AlgoEvent: def __init__(self): pass   
4   
5   
6   
7 def start(self, mEvt): self.evt = AlgoAPI_Backtest.AlgoEvtHandler(self, mEvt) self.evt.start()   
8   
9   
10   
11 def on_bulkdatafeed(self, isSync, bd, ab): pass   
12   
13   
14 def on_marketdatafeed(self, md, ab): pass   
15   
16   
17 def on_newsdatafeed(self, nd): pass   
18   
19   
20 def on/weatherdatafeed(self, wd): pass   
21   
22   
23 def on_econsdatafeed(self, ed): pass   
24   
25   
26 def on_corpAnnouncement(self, ca): pass   
27   
28   
29 def on_orderfeed(self, of): pass   
30   
31   
32 def on_dailylplfeed(self, pl): pass   
33   
34   
35 def on_openPositionfeed(self, op, oo, uo): pass 
```

![](images/95274d2cd5a5f01b2b7bd787cc770ceae0d7f8e7f3ee18c8d4eac444d1076d42.jpg)

# How data is processed on ALGOGENE?

# Event Stream Architecture

Data feed   
Market data   
News   
Economic statistics

- Order status update   
- Position changes   
.

Stock A:

bid price @10.0

ask price @10.5

Stock B:

bid price @53.5

ask price @56.5

Stock A:

bid price @10.4

ask price @11.3

Stock A:

bid price @11.2

ask price @11.9

Stock B:

bid price @54.8

ask price @57.0

Walt Disney World is reopening after its

closure due to the Covid 19 pandemic.

U.S. GDP for 2020 1Q

YoY Change: -12.1%

Order filled for buy 1 lot

of Stock A @11.9

The Fed signals it may

extend the Quantitative

Easing Program

# MA Cross Strategy

![](images/0475b391a6f451a57bf8db35adb7239f699e57cb34935ccf8957809fbbe54022.jpg)

# MA Cross Strategy

# Backtest setup

- Data interval = 1 day   
- Base currency = HKD   
- Leverage $= 1 \times$   
- Enable short shell = False   
- Instrument = 00005HK

# Settings

Strategy Name

Start Period

End Period

Data Interval

Initial Capital

Base Currency

Leverage

Transaction Cost

Risk free Rate

Enable Short Sell

Position Netting

News Data

Weather Data

Economics Data

Instruments 1

(Max. 100 items)

TEST

2024-01

2024-12

1 day

100000

HKD

$\therefore m = \frac{3}{11}$

0

0   
  
  
  
  
  
  
Select...   
00005HKx

#

#

#

#

$\frac{1}{2} =$

$\therefore m = \frac{3}{11}$

# MA Cross Strategy

# 1. Initialize variable

from AlgoAPI import AlgoAPIUtil, AlgoAPI_Backtest

from datetime import datetime, timeDelta

import talib, numpy

class AlgoEvent:

def __init__(self):

self.last_time = datetime(1970,1,1)

self.arr_close = numpy.array([])

self.arr_fastMA = numpy.array([])

self.arrslowMA=numpy.array([])

self.fastperiod = 7

selfslowperiod = 14

Import library

Define our variables under def __init__

# MA Cross Strategy

# 2. Calculate MA value

```python
def on_marketdatafeed(self, md, ab): # retrieve recent observations res = self.evt.getHistoricalBar( contract = {"instrument": md.instrument}, numOfBar = self.fastperiod + selfslowperiod, interval = "D" 
```

get closing price data

self.arr_close = numpy.array([res[t]['c'] for t in res])

fit SMA line

self.arr_fastMA = talib.SMA(self.arr_close, timeperiod=int(self.fastperiod))

self.arr SlowMA = talib.SMA(self.arr_close, timeperiod=int(self slowspeed))

# debug print result

self.evtconsoleLog("arr_fastMA=", self.arr_fastMA)

self.evt consoleLog("arrslowMA=", self.arrslowMA)

```csv
[2023-07-01 00:00:00] Backtest started!  
[2023-07-01 00:00:00] arr_fastMA=, [nan nan nan nan nan nan nan  
59.05714286 59.43571429 59.67857143 59.80714286 59.95 60.13571429  
60.34285714 60.55714286 60.74285714 60.71428571 60.65 60.71428571  
60.69285714 60.61428571 60.54285714]  
[2023-07-01 00:00:00] arr SlowMA=, [nan nan nan nan nan nan  
nan nan nan nan nan nan  
nan 59.80714286 60.08928571 60.19642857 60.22857143 60.33214286  
60.41428571 60.47857143 60.55 ]  
[2023-07-04 00:00:00] arr_fastMA=, [nan nan nan nan nan nan  
59.43571429 59.67857143 59.80714286 59.95 60.13571429 60.34285714  
60.55714286 60.74285714 60.71428571 60.65 60.71428571 60.69285714  
60.61428571 60.54285714 60.63571429]  
[2023-07-04 00:00:00] arr SlowMA=, [nan nan nan nan nan nan  
nan nan nan nan nan nan  
nan 60.08928571 60.19642857 60.22857143 60.33214286 60.41428571  
60.47857143 60.55 60.68928571]  
[2023-07-05 00:00:00] arr_fastMA=, [nan nan nan nan nan nan  
59.67857143 59.80714286 59.95 60.13571429 60.34285714 60.55714286  
60.74285714 60.71428571 60.65 60.71428571 60.69285714 60.61428571  
60.54285714 60.63571429 61.]  
[2023-07-05 00:00:00] arr SlowMA=, [nan nan nan nan nan nan  
nan nan nan nan nan nan  
nan 60.19642857 60.22857143 60.33214286 60.41428571 60.47857143  
60.55 60.68928571 60.85714286] 
```

# MA Cross Strategy

# 3. Query order inventory

https://algogene.com/TechDoc#QueryServerOrder

```python
def on_marketdatafeed(self, md, ab): # retrieve recent observations res = self.evt.getHistoricalBar( contract = {"instrument": md.instrument}, numOfBar = self.fastperiod + selfslowperiod, interval = "D" ) 
```

```python
get closing price data  
self.arr_close = numpy.array([res[t]['c'] for t in res]) 
```

```python
# fit SMA line
self.arr_fastMA = talib.SMA(self.arr_close, timeperiod=int(self.fastperiod))
self.arr SlowMA = talib.SMA(self.arr_close, timeperiod=int(self Slowperiod)) 
```

```python
# debug print result
self.evt consoleLog("arr_fastMA=", self.arr_fastMA)
self.evt consoleLog("arrslowMA=", self.arr SlowMA) 
```

```python
check number of record is at least greater than both self.fastperiod, selfslowperiod if numpy.isnan(self.arr_fastMA[-1]) or numpy.isnan(self.arr_fastMA[-2]) or \numpy.isnan(self.arr_slowMA[-1]) or numpy.isnan(self.arr SlowMA[-2]): return 
```

```python
# query outstanding position
positions, osOrder,PENDOrder = self.evt.getSystemOrders()
pos = positions[md.instrument]['netVolume'] 
```

# MA Cross Strategy

# 4. Order Entry Condition

def on_marketdatafeed(self, md, ab):

retrieve recent observations

res = self.evt.getHistoricalBar

contract = {"instrument": md.instrument}

numOfBar = self.fastperiod + selfslowperiod,

interval $=$ "D"

）

get closing price data

self.arr_close = numpy.array([res[t][c'] for t in res])

fit SMA line

self.arr_fastMA = talib.SMA(self.arr_close, timeperiod=int(self.fastperiod))

self arr SlowMA = talib.SMA(self.arr_close, timeperiod=int(self Slowperiod))

# debug print result

self.evt consoleLog("arr_fastMA=", self.arr_fastMA)

self.evt consoleLog("arrslowMA=", self.arrslowMA)

check number of record is at least greater than both self.fastperiod, selfslowperiod

if numpy.isnan(self.arr_fastMA[-1]) or numpy.isnan(self.arr_fastMA[-2]) or numpy.isnan(self.arr_slowMA[-1]) or numpy.isnan(self.arr SlowMA[-2]): return

# query outstanding position

positions, osOrder, pendOrder = self.evt.getSystemOrders()

pos = positions[md.instrument]['netVolume']

Open a buy order for golden cross

send a buy order for Golden Cross

if self.arr_fastMA[-1] > self.arr_slowMA[-1] and self.arr_fastMA[-2] < self.arrslowMA[-2]:

order = AlgoAPIUtil.OrderObject(

instrument = md.i

buysell $= 1$

volume $= 1$

openclose = "open",

ordertype $= 0$ #0 $\equiv$ market_order,1 $\equiv$ limit_order,2 $\equiv$ stop_order

）

self.evt.sendOrder(order)

# MA Cross Strategy

# 5. Order Exit Condition

Close previous buy order when death cross appears

```python
# query outstanding position  
positions, osOrder,PENDOrder = self.evt.getSystemOrders()  
pos = positions[md.instrument]['netVolume']  
# send a buy order for Golden Cross  
if self.arr_fastMA[-1] > self.arrslowMA[-1] and self.arr_fastMA[-2] < self.arrslowMA[-2]:  
    order = AlgoAPIUtil.OrderObject(  
        instrument = md.instrument,  
        buysell = 1,  
        volume = 1,  
        openclose = "open",  
        ordertype = 0 #0=market_order, 1=limit_order, 2=stop_order)  
    )  
self.evt.sendOrder(order) 
```

send a sell order for Death Cross   
if self.arr_fastMA[-1] $<  <$ self.arrslowMA[-1] and self.arr_fastMA[-2] $>$ self.arr SlowMA[-2]: #close all outstanding position   
for tradeID in list(osOrder): order $=$ AlgoAPIUtil.OrderObject( tradeID $=$ tradeID, openclose $=$ "close" ) self.evt.sendOrder(order)

# MA Cross Strategy

# Full script

from AlgoAPI import AlgoAPIUtil, AlgoAPI_Backtest from datetime import datetime, timedelta import talib, numpy

# class AlgoEvent:

def init(self):  
    self.last_day = datetime(1970, 1, 1)  
    self.arr_close = numpy.array([])  
    self.arr_fastMA = numpy.array([])  
    self.arr_slowMA = numpy.array([])  
    self.fastperiod = 7  
    self Slowperiod = 14

def start(self, mEvt):  
    self EVT = AlgoAPI_Backtest.AlgoEvtHandler(self, mEvt)  
    self EVT start()

# def on_markdatafeed(self, md, ab):

```python
# retrieve recent observations
res = self.evt.getHistoricalBar(
    contract = {"instrument": md.instrument},
    numOfBar = self.fastperiod + selfslowperiod,
    interval = "D"
)   
get closing price data
self.arr_close = numpy.array([res[t][c'] for t in res])   
```java
# fit SMA line
    self.arr_fastMA = talib(SMA(self.arr_close, timeperiod=int(self.fastperiod))
    self.arr SlowMA = talib(SMA(self.arrClose, timeperiod=int(selfslowperiod))   
```java
# debug print result
    self EVTconsoleLog("arr_fastMA=", "self arr_fastMA")
    self EVTconsoleLog("arrslowMA=", "self arrslowMA")   
# check number of record is at least greater than both self.fastperiod, selfslowperiod if numpy.isnan(self.arr_fastMA[-1]) or numpy.isnan(self.arr_fastMA[-2]) or numpy.isnan(self.arr_fastMA[-1]) or numpy.isnan(self.arr_fastMA[-2]): return   
```c
# query outstanding position
positions, osOrder, pendOrder = self EVT_SYSTEMOrders()
pos = positions[mdInstrument][['netVolume']]   
send a buy order for Golden Cross  
if self.arr_fastMA[-1] > self.arr SlowMA[-1] and self.arr_fastMA[-2] < self.arr SlowMA[-2]:  
    order = AlgoAPIUtil.OrderObject(  
        instrument = md.instrument,  
        buysell = 1,  
        volume = 1,  
        openclose = "open",  
        ordertype = 0 #0=market_order, 1=limit_order, 2=stop_order)  
    )  
self EVT sendOrder(order)   
send a sell order for Death Cross   
if self.arr_fastMA[-1] $<  <$ self.arr_slowMA[-1] and self arr_fastMA[-2] $>$ self.arr_slowMA[-2]: #close all outstanding position   
for tradeID in osOrder: order $=$ AlgoAPIUtil.OrderObject( tradeID $=$ tradeID, openclose $=$ "close" ) self evt sendOrder(order)

# MA Cross Strategy

<table><tr><td>No. of Tradable Days:</td><td>248</td><td>No. of Win Days:</td><td>76</td><td>No. of Loss Days:</td><td>61</td></tr><tr><td>Win Rate:</td><td>55.4745%</td><td>Max. Consecutive Win Day:</td><td>7</td><td>Max. Consecutive Loss Day:</td><td>3</td></tr><tr><td>Odd Ratio:</td><td>1.2459</td><td>Max. Consecutive Gains:</td><td>2368.0</td><td>Max. Consecutive Loss:</td><td>-2484.0</td></tr><tr><td>No. of Trades:</td><td>25</td><td>Average Consecutive Win Day:</td><td>0.92</td><td>Average Consecutive Loss Day:</td><td>0.41</td></tr><tr><td>Total PnL:</td><td>1238.4</td><td>Average Per Trade PnL:</td><td>49.54</td><td>Average Per Day PnL:</td><td>4.99</td></tr><tr><td>Mean Daily Return:</td><td>0.006%</td><td>Median Daily Return:</td><td>0.0%</td><td>Mean Annual Return:</td><td>1.5048%</td></tr><tr><td>Daily Return StdDev:</td><td>0.3312%</td><td>25th percentile Daily Return:</td><td>-0.02%</td><td>75th percentile Daily Return:</td><td>0.0937%</td></tr><tr><td>Daily Return Downside StdDev:</td><td>0.2367%</td><td>95% 1 day return VaR:</td><td>-0.3948%</td><td>99% 1 day return VaR:</td><td>-1.0166%</td></tr><tr><td>Daily Sharpe Ratio:</td><td>0.018</td><td>Annual Sharpe Ratio:</td><td>0.2862</td><td>Max. Drawdown Amount:</td><td>5376.0</td></tr><tr><td>Daily Sortino Ratio:</td><td>0.0252</td><td>Annual Sortino Ratio:</td><td>0.4004</td><td>Max. Drawdown Percent:</td><td>5.24%</td></tr><tr><td>Max. Drawdown Duration:</td><td>159</td><td>Average Drawdown Duration:</td><td>58.58</td><td>Annual Volatility:</td><td>5.2363%</td></tr><tr><td>Gross Profit:</td><td>3500.0</td><td>Gross Loss:</td><td>-4680.0</td><td>Profit Factor:</td><td>0.7479</td></tr><tr><td>Jensen Alpha:</td><td>0.0</td><td>Beta:</td><td>0.0</td><td>Information Ratio:</td><td>0.0</td></tr><tr><td>Omega Ratio:</td><td>0.0</td><td>Treynor Ratio:</td><td>0.0</td><td>Tail Ratio:</td><td>0.9591</td></tr><tr><td>Calmar Ratio:</td><td>0.2872</td><td>Average Holding Day:</td><td>14.9231</td><td>Annual Turnover Rate:</td><td>319.708%</td></tr></table>

# Wrap Up

<table><tr><td></td><td>Pros</td><td>Cons</td></tr><tr><td>Excel</td><td>·Quick to test simple ideas
·Easy to create charts and adjust formulas</td><td>·Need to manage your data sources
·Lacks of feasibility to scale and extend to complicated strategies
·Unable to load large amount of data into spreadsheet</td></tr><tr><td>Backtesting.py</td><td>·Free and simple python code
·Can extend to other open-sourced libraries for different technical indicators, statistical models, etc</td><td>·Need to manage your data sources
·Limited build-in functionalities
·Lack of supports for real trading deployment
·Limited memory to load large amount of data into dataframe</td></tr><tr><td>AlgoGene</td><td>·Python environment with open-sourced libraries available
·Ready-to-use datasets
·Seamless deployment from backtest to live test, real trading without code amendment</td><td>·May need some time to get familiar with the system</td></tr></table>

# Common Backtest Pitfalls

# Common Pitfalls

1. Overfitting   
2. Look-ahead bias   
3. Survivorship Bias   
4. Data-Snooping Bias   
5. Ignore assumptions you made

# Overfitting

• Definition: Tailoring a strategy too closely to historical data, making it less likely to perform well on new, unseen data.   
- Symptoms: Exceptional past performance without sound reasoning.   
- Avoidance: Use out-of-sample testing and cross-validation.

# Look-ahead bias

- Definition: Using future data that would not have been available at the time of the trade decision.   
- Symptoms: Unrealistically high profits.   
- Avoidance: Ensure all data used for decisions is available up to the point in time being simulated.

# Survivorship Bias

• Definition: Only considering assets that have survived until the end of the period, ignoring those that have failed.   
- Symptoms: Overestimation of strategy performance.   
- Avoidance: Include delisted or bankrupt companies in the dataset.

# Data-Snooping Bias

• Definition: Repeatedly testing multiple strategies on the same dataset until one works, which may be due to chance.   
• Symptoms: High chance of false positives.   
- Avoidance: Validate the strategy on different datasets or time periods.

# Ignore Assumptions

# Examples:

- Failing to account for costs such as commissions, slippage, financing costs, and taxes   
- Short selling on certain stocks cannot be traded in real market   
- Exceptionally high leverage is not doable in practice   
- Potential delay between signal calculation and execution (eg. during market closure, using a complicated trading model, etc)

# - Avoidance:

- Incorporate realistic estimates of transaction costs into the backtest.   
- Do some market research to check if your assumptions can be implemented in real market

# Best Practice

- Robust Validation: Use out-of-sample testing and walk-forward analysis.   
- Realistic Assumptions: Incorporate transaction costs and market impact.   
- Continuous Monitoring: Regularly update and review the strategy.

# Exercise

RSI Strategy

# Relative Strength Index (RSI)

- a momentum oscillator that measures the speed and change of price movements   
- typically used to identify overbought or oversold conditions in a market   
It ranges from 0 to 100

- Overbought: RSI > 70, indicating that the asset may be overvalued and a price correction could occur.   
- Oversold: RSI $< 30$ , suggesting that the asset may be undervalued and a price increase could occur.

# Relative Strength Index (RSI)

# Define:

- Average Gain: Sum of gains over a specified period divided by the number of periods.   
- Average Loss: Sum of losses over the same period divided by the number of periods.   
Period: Commonly 14 days.

Formula:

$$
R S = A v e r a g e G a i n / A v e r a g e L o s s
$$

$$
\mathrm {R S I} = 1 0 0 - (1 0 0 / (1 + \mathrm {R S}))
$$

# Example

- Calculate RSI(7) for the following daily stock price

<table><tr><td>t</td><td>1</td><td>2</td><td>3</td><td>4</td><td>5</td><td>6</td><td>7</td><td>8</td><td>9</td><td>10</td><td>11</td><td>12</td><td>13</td><td>14</td><td>15</td><td>16</td><td>17</td><td>18</td><td>19</td><td>20</td></tr><tr><td>Price</td><td>44.34</td><td>44.09</td><td>44.15</td><td>43.61</td><td>44.33</td><td>45.32</td><td>45.93</td><td>45.45</td><td>46.25</td><td>46.92</td><td>47.25</td><td>46.8</td><td>46</td><td>46.25</td><td>46.78</td><td>47.1</td><td>46.9</td><td>47.45</td><td>48.1</td><td>48.5</td></tr></table>

- Calculate the daily gain and loss

<table><tr><td>t</td><td>1</td><td>2</td><td>3</td><td>4</td><td>5</td><td>6</td><td>7</td><td>8</td><td>9</td><td>10</td><td>11</td><td>12</td><td>13</td><td>14</td><td>15</td><td>16</td><td>17</td><td>18</td><td>19</td><td>20</td></tr><tr><td>Price</td><td>44.34</td><td>44.09</td><td>44.15</td><td>43.61</td><td>44.33</td><td>45.32</td><td>45.93</td><td>45.45</td><td>46.25</td><td>46.92</td><td>47.25</td><td>46.8</td><td>46</td><td>46.25</td><td>46.78</td><td>47.1</td><td>46.9</td><td>47.45</td><td>48.1</td><td>48.5</td></tr><tr><td>Diff</td><td></td><td>-0.25</td><td>0.06</td><td>-0.54</td><td>0.72</td><td>0.99</td><td>0.61</td><td>-0.48</td><td>0.8</td><td>0.67</td><td>0.33</td><td>-0.45</td><td>-0.8</td><td>0.25</td><td>0.53</td><td>0.32</td><td>-0.2</td><td>0.55</td><td>0.65</td><td>0.4</td></tr><tr><td>Gain</td><td></td><td>0</td><td>0.06</td><td>0</td><td>0.72</td><td>0.99</td><td>0.61</td><td>0</td><td>0.8</td><td>0.67</td><td>0.33</td><td>0</td><td>0</td><td>0.25</td><td>0.53</td><td>0.32</td><td>0</td><td>0.55</td><td>0.65</td><td>0.4</td></tr><tr><td>Loss</td><td></td><td>0.25</td><td>0</td><td>0.54</td><td>0</td><td>0</td><td>0</td><td>0.48</td><td>0</td><td>0</td><td>0</td><td>0.45</td><td>0.8</td><td>0</td><td>0</td><td>0</td><td>0.2</td><td>0</td><td>0</td><td>0</td></tr><tr><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td></td></tr><tr><td>Avg Gain</td><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td>0.34</td><td>0.454286</td><td>0.541429</td><td>0.588571</td><td>0.485714</td><td>0.344286</td><td>0.292857</td><td>0.368571</td><td>0.3</td><td>0.204286</td><td>0.235714</td><td>0.328571</td><td>0.385714</td></tr><tr><td>Avg Loss</td><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td>0.181429</td><td>0.145714</td><td>0.145714</td><td>0.068571</td><td>0.132857</td><td>0.247143</td><td>0.247143</td><td>0.178571</td><td>0.178571</td><td>0.207143</td><td>0.207143</td><td>0.142857</td><td>0.028571</td></tr><tr><td>RS</td><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td>1.874016</td><td>3.117647</td><td>3.715686</td><td>8.583333</td><td>3.655914</td><td>1.393064</td><td>1.184971</td><td>2.064</td><td>1.68</td><td>0.986207</td><td>1.137931</td><td>2.3</td><td>13.5</td></tr><tr><td>RSI</td><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td>65.20548</td><td>75.71429</td><td>78.79418</td><td>89.56522</td><td>78.52194</td><td>58.21256</td><td>54.2328</td><td>67.36292</td><td>62.68657</td><td>49.65278</td><td>53.22581</td><td>69.69697</td><td>93.10345</td></tr></table>

# Example

import pandas as pd

def calculate_rsi(data, window=7):

Calculate the Relative Strength Index (RSI) for a given dataset.

Parameters:

data (pd.Series): A pandas Series of closing prices.

window (int): The number of periods to use for the RSI calculation.

Returns:

pd.Series: A pandas Series containing the RSI values.

1

delta = data.diff()

gain $=$ (delta.where(delta $\geq 0$ ,0)).rollingwindow $\equiv$ window).mean()

loss $=$ (-delta.where(delta $<  0$ ,0)).rollingwindow $\equiv$ window).mean()

rs = gain / loss

$\mathrm{rsi} = 100 - (100 / (1 + \mathrm{rs}))$

return rsi

df = pd.DataFrame({'Close': [44.34, 44.09, 44.15, 43.61, 44.33, 45.32, 45.93, 45.45, 46.25, 46.92, 47.25, 46.8, 46, 46.25, 46.78, 47.1, 46.9, 47.45, 48.1, 48.5]})

df['RSI'] = calculate_rsi(df,7)

print(df)

![](images/8f646973da3221999008df08960705d0455e69a26a230040a3f0288fdd4a0db2.jpg)

Close RSI   
44.34 Na   
1 44.09 NaN   
2 44.15 Na   
3 43.61 NaN   
4 44.33 NaN   
5 45.32 NaN   
6 45.93 75.078864   
7 45.45 65.205479   
8 46.25 75.714286   
9 46.92 78.794179   
10 47.25 89.565217   
11 46.80 78.521940   
12 46.00 58.212560   
13 46.25 54.232804   
14 46.78 67.362924   
15 47.10 62.686567   
16 46.90 49.652778   
17 47.45 53.225806   
18 48.10 69.696970   
19 48.50 93.103448

# Example - TALIB

import numpy as np

from talib import RSI

arr = [44.34, 44.09, 44.15, 43.61, 44.33, 45.32,

45.93, 45.45, 46.25, 46.92, 47.25, 46.8, 46, 46.25,

46.78, 47.1, 46.9, 47.45, 48.1, 48.5]

data_RSI = RSI(np.array(arr), 7)

print('data_RSI=',data_RSI)

![](images/812710c579bdf80ab6b505d3fbc3f782d89e91c3090668053c0e4b310581cd01.jpg)

```txt
data_RSI= [ nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan nan 
```

![](images/f52aa0bb3c68f8d2cd929985bf23953f8279f95dc0bc58dbdea83cadf51755f5.jpg)

Why the result is not the same as our previous calculation?

# Example - RSI with EMA smoothing

```python
data_RSI = [None, None, None, None, None, None, None, 72.29090909090911, 76.88924834453827, 78.90122060961261, 69.30271677999245, 55.33965830526435, 58.395739674188725, 64.41795674477854, 67.71028899952013, 63.43072748987339, 69.59602992193464, 75.33052761587143, 78.27267706693819] 
```

Now it matches with TA-LIB results

```python
def my_RSI(data, n, isEMA=True):  
    m = len(data) - 1  
    k = 1 / float(n)  
    RSI = [None] * m  
# error handling for insufficient data  
if m < n:  
    return RSI  
# compute 1 step price difference  
delta = [data[i + 1] - data[i] for i in range(0, m)]  
# create the positive gains (up) and negative gains (down) series  
gain = [abs(i) if i > 0 else 0 for i in delta]  
loss = [abs(i) if i < 0 else 0 for i in delta]  
avg_gain = sum(gain[0:n]) / float(n)  
avg_loss = sum(loss[0:n]) / float(n)  
for i in range(n, m):  
    # Exponential Moving Average  
    if isEMA:  
        avg_gain = gain[i] * k + avg_gain*(1-k)  
        avg_loss = loss[i] * k + avg_loss*(1-k)  
    # Simple Moving Average  
    else:  
        avg_gain = (avg_gain*n - gain[i-n] + gain[i]) * k  
        avg_loss = (avg_loss*n - loss[i-n] + loss[i]) * k  
    # calculate RSI value  
    if avg_loss == 0:  
        RSI[i] = 100.0  
    else:  
        RSI[i] = 100.0 - (100.0 / (1.0 + avg_gain/avg_loss))  
return RSI 
```

```python
import numpy as np  
arr = [44.34, 44.09, 44.15, 43.61, 44.33, 45.32, 45.93, 45.45, 46.25, 46.92, 47.25, 46.8, 46, 46.25, 46.78, 47.1, 46.9, 47.45, 48.1, 48.5]  
data_RSI = my_RSI(np.array(arr), 7)  
print('data_RSI=', data_RSI) 
```

# RSI Trading Logic

- Collect daily closing price   
Calculate the latest RSI(14) value  
- Order open conditions:

- if we have NO outstanding position,

- if RSI value $< 30$ , we open a buy order   
- if RSI value $>70$ , we open a sell order

- Order close conditions:   
- if we have outstanding position,   
- if we previously submit a buy order and RSI value reverses back to or above 50, then close the buy order   
- if we previously submit a sell order and RSI value reverses back to or below 50, then close the sell order   
- Repeat the process until the backtest period ends

![](images/9cf4d62b56fbc3fdaedf1d8d469df7e507aec724daa5263f8476c95dc5aa6bab.jpg)

# RSI Strategy

# Backtest setup

- Instrument: SPY (https://finance.yahoo.com/quote/SPY/)   
Period: 2020.01.01 - 2025.12.31   
- Data Interval: daily data   
- Long only

Please try to implement this RSI strategy in

1. Excel   
2. Backtesting.py   
3. ALGOGENE

# Key Takeaways

- Algo trading life cycle and backtesting process   
- Candlestick charts and common patterns   
Data cleaning techniques   
- Implement trading strategies on Excel and Python   
MA Cross Strategy   
- Common Pitfalls in Backtesting   
Exercise - RSI Strategy
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
