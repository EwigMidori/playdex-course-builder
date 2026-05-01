请基于以下材料，生成一份 lesson 级 MDX 课本。

目标语言：
zh-CN

lesson_id：
L3

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

<QUESTION_BANK>
[
{
  "coverage_map": [
    {
      "coverage_tag": "backtesting_definition",
      "covered_by": [
        "qf_flash_backtest_def",
        "qf_quiz_backtest_purpose"
      ],
      "description": "回测的定义：用历史数据模拟交易策略，验证其有效性。"
    },
    {
      "coverage_tag": "backtesting_purpose",
      "covered_by": [
        "qf_flash_backtest_def",
        "qf_quiz_backtest_purpose"
      ],
      "description": "回测的目的：在实盘交易前验证策略的有效性。"
    },
    {
      "coverage_tag": "backtesting_procedures",
      "covered_by": [
        "qf_flash_backtest_steps",
        "qf_quiz_backtest_steps_order"
      ],
      "description": "回测的关键步骤：数据收集、数据清洗、策略实施、绩效评估。"
    },
    {
      "coverage_tag": "backtesting_analogy",
      "covered_by": [
        "qf_long_backtest_analogy"
      ],
      "description": "回测的类比：像用历史数据做模拟实验。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "backtesting_definition",
      "coverage_tags": [
        "backtesting_definition",
        "backtesting_purpose"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_backtest_def",
      "learning_goal": "学生能准确说出回测的定义和核心目的。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "回测是什么，以及为什么要做回测。",
      "term_refs": [
        {
          "display": "回测",
          "en": "backtesting"
        }
      ],
      "variants": [
        {
          "back": "用历史数据模拟交易策略，验证其有效性。",
          "estimated_seconds": 8,
          "explanation": "回测的核心就是利用过去的数据来检验一个交易策略是否有效，而不是预测未来。",
          "front": "回测（backtesting）的定义是什么？",
          "question_id": "q_flash_backtest_def_v1"
        },
        {
          "back": "在投入真金白银之前，验证交易策略的有效性。",
          "estimated_seconds": 8,
          "explanation": "回测的目的是在实盘交易前，通过历史数据检验策略，避免盲目投资。",
          "front": "进行回测的主要目的是什么？",
          "question_id": "q_flash_backtest_def_v2"
        }
      ]
    },
    {
      "concept_key": "backtesting_procedures",
      "coverage_tags": [
        "backtesting_procedures"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_backtest_steps",
      "learning_goal": "学生能按顺序回忆回测的四个关键步骤。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "回测流程中的四个主要环节。",
      "term_refs": [
        {
          "display": "回测流程",
          "en": "backtesting procedures"
        }
      ],
      "variants": [
        {
          "back": "清洗数据。",
          "estimated_seconds": 6,
          "explanation": "回测流程是：收集数据 → 清洗数据 → 运行策略 → 评估表现。",
          "front": "回测的核心流程中，在“收集数据”之后，下一步是什么？",
          "question_id": "q_flash_backtest_steps_v1"
        },
        {
          "back": "评估表现。",
          "estimated_seconds": 6,
          "explanation": "回测流程是：收集数据 → 清洗数据 → 运行策略 → 评估表现。",
          "front": "回测流程中，在“运行策略”之后，最后一步是什么？",
          "question_id": "q_flash_backtest_steps_v2"
        }
      ]
    }
  ],
  "lesson_id": "L3",
  "longform_families": [
    {
      "concept_key": "backtesting_analogy",
      "coverage_tags": [
        "backtesting_analogy"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_backtest_analogy",
      "learning_goal": "学生能用类比的方式解释回测的概念和重要性。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "回测",
          "en": "backtesting"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "回测的定义",
            "回测的目的",
            "一个简单的类比（例如，试飞、彩排等）"
          ],
          "question_id": "q_long_backtest_analogy_v1",
          "reference_answer": [
            "回测就是用过去的数据来模拟运行一个交易策略，看看它历史上表现如何。",
            "它的重要性在于，在投入真金白银之前，我们可以先检验策略是否有效，避免盲目亏损。",
            "这就像电影上映前的试映会，通过小范围观众的反馈来预测电影是否受欢迎，而不是直接大规模上映。"
          ],
          "rubric_points": [
            "准确说出回测是用历史数据模拟策略。",
            "说明回测的目的是验证策略有效性，降低实盘风险。",
            "给出一个合理的类比，并能解释类比与回测的对应关系。"
          ],
          "stem": "请用你自己的话，解释什么是回测，并说明为什么在实盘交易前进行回测很重要。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "回测的定义",
            "回测的局限性（历史不代表未来）",
            "回测的真正价值"
          ],
          "question_id": "q_long_backtest_analogy_v2",
          "reference_answer": [
            "我不同意。回测是用历史数据模拟策略，它只能告诉我们策略在过去的表现。",
            "市场是变化的，过去有效的策略未来可能失效。",
            "回测的真正价值是帮助我们筛选和优化策略，降低风险，但不能保证未来盈利。"
          ],
          "rubric_points": [
            "明确表示不同意。",
            "引用回测的定义，指出它只是模拟历史。",
            "指出历史表现不能保证未来结果，回测只是提供参考和信心。"
          ],
          "stem": "假设你的朋友说：“回测结果好，就代表这个策略一定能赚钱。” 你同意吗？请结合回测的定义和目的，解释你的观点。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "backtesting_purpose",
      "coverage_tags": [
        "backtesting_definition",
        "backtesting_purpose"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_backtest_purpose",
      "learning_goal": "学生能在多个选项中准确识别回测的主要目的。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "回测",
          "en": "backtesting"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "回测的核心就是用历史数据来检验交易策略是否有效，而不是预测未来。",
          "options": [
            "预测未来股价",
            "在真实市场中赚钱",
            "用历史数据验证策略的有效性",
            "计算交易成本"
          ],
          "question_id": "q_quiz_backtest_purpose_v1",
          "stem": "回测的主要目的是什么？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "回测是模拟策略在历史数据上的表现，不能保证未来盈利，但能提供参考。",
          "options": [
            "保证策略在未来盈利",
            "模拟策略在历史数据上的表现",
            "自动执行交易",
            "分析市场新闻"
          ],
          "question_id": "q_quiz_backtest_purpose_v2",
          "stem": "以下哪一项最准确地描述了回测的功能？"
        }
      ]
    },
    {
      "concept_key": "backtesting_procedures",
      "coverage_tags": [
        "backtesting_procedures"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_backtest_steps_order",
      "learning_goal": "学生能正确排列回测的四个关键步骤。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "ordering",
      "term_refs": [
        {
          "display": "回测流程",
          "en": "backtesting procedures"
        }
      ],
      "variants": [
        {
          "answer": [
            1,
            3,
            0,
            2
          ],
          "estimated_seconds": 25,
          "explanation": "正确的顺序是：数据收集 → 数据清洗 → 策略实施 → 绩效评估。",
          "options": [
            "A. 策略实施",
            "B. 数据收集",
            "C. 绩效评估",
            "D. 数据清洗"
          ],
          "question_id": "q_quiz_backtest_steps_order_v1",
          "stem": "请将以下回测步骤按正确顺序排列："
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "在运行策略之前，必须先完成数据收集和数据清洗，确保数据质量。",
          "options": [
            "A. 绩效评估",
            "B. 数据清洗",
            "C. 实盘交易",
            "D. 策略优化"
          ],
          "question_id": "q_quiz_backtest_steps_order_v2",
          "stem": "在回测流程中，以下哪个步骤应该在“策略实施”之前完成？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L3: Building backtest framework and rule-based trading strategy",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "L3 lesson map",
    "plain_text": "pipeline/1-plain/L3/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L3: Building backtest framework and rule-based trading strategy"
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "candlestick_intro",
      "covered_by": [
        "qf_flash_candlestick_components",
        "qf_quiz_candlestick_definition"
      ],
      "description": "K线图（蜡烛图）的定义、用途：展示资产价格波动，汇总OHLC数据。"
    },
    {
      "coverage_tag": "ohlc_components",
      "covered_by": [
        "qf_flash_candlestick_components",
        "qf_quiz_ohlc_meaning"
      ],
      "description": "K线包含开盘价、收盘价、最高价、最低价四个关键信息。"
    },
    {
      "coverage_tag": "bullish_bearish_candle",
      "covered_by": [
        "qf_flash_bullish_bearish",
        "qf_quiz_bullish_bearish_identification"
      ],
      "description": "阳线（收盘价>开盘价）和阴线（收盘价<开盘价）的视觉特征。"
    },
    {
      "coverage_tag": "candlestick_body_wick",
      "covered_by": [
        "qf_flash_candlestick_components"
      ],
      "description": "K线的实体（开盘价与收盘价之间）和影线（最高/最低价延伸线）。"
    },
    {
      "coverage_tag": "doji_pattern",
      "covered_by": [
        "qf_flash_doji",
        "qf_quiz_doji_meaning",
        "qf_long_doji_interpretation"
      ],
      "description": "十字星形态：开盘价与收盘价几乎相等，代表市场犹豫不决，可能预示趋势反转。"
    },
    {
      "coverage_tag": "hammer_pattern",
      "covered_by": [
        "qf_flash_hammer",
        "qf_quiz_hammer_identification",
        "qf_long_hammer_interpretation"
      ],
      "description": "锤子线形态：出现在下跌趋势后，下影线很长，实体很小，是看涨反转信号。"
    },
    {
      "coverage_tag": "shooting_star_pattern",
      "covered_by": [
        "qf_flash_shooting_star",
        "qf_quiz_shooting_star_identification",
        "qf_long_shooting_star_interpretation"
      ],
      "description": "射击之星形态：出现在上涨趋势后，上影线很长，实体很小，是看跌反转信号。"
    },
    {
      "coverage_tag": "pattern_trend_reversal",
      "covered_by": [
        "qf_quiz_pattern_reversal_signal",
        "qf_long_compare_reversal_patterns"
      ],
      "description": "K线形态可提供未来价格方向的信号，尤其是反转信号。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "candlestick_components",
      "coverage_tags": [
        "candlestick_intro",
        "ohlc_components",
        "candlestick_body_wick"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_candlestick_components",
      "learning_goal": "学生能准确回忆K线图的定义、四个关键价格以及实体和影线的含义。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "K线图的基本构成要素和定义。",
      "term_refs": [
        {
          "display": "K线图",
          "en": "Candlestick Chart"
        },
        {
          "display": "开盘价",
          "en": "Open"
        },
        {
          "display": "收盘价",
          "en": "Close"
        },
        {
          "display": "最高价",
          "en": "High"
        },
        {
          "display": "最低价",
          "en": "Low"
        },
        {
          "display": "实体",
          "en": "Body"
        },
        {
          "display": "影线",
          "en": "Wick"
        }
      ],
      "variants": [
        {
          "back": "开盘价、收盘价、最高价、最低价（OHLC）。",
          "estimated_seconds": 8,
          "explanation": "OHLC分别代表Open（开盘价）、High（最高价）、Low（最低价）、Close（收盘价）。",
          "front": "K线图（蜡烛图）中，一根K线包含哪四个关键价格信息？",
          "question_id": "q_flash_candlestick_components_v1"
        },
        {
          "back": "开盘价和收盘价之间的区域。",
          "estimated_seconds": 6,
          "explanation": "实体的大小反映了开盘与收盘之间的价格变动幅度。",
          "front": "K线图中的“实体”指的是什么？",
          "question_id": "q_flash_candlestick_components_v2"
        },
        {
          "back": "从实体上下延伸的线条，代表最高价和最低价。",
          "estimated_seconds": 6,
          "explanation": "上影线代表最高价，下影线代表最低价。",
          "front": "K线图中的“影线”代表什么？",
          "question_id": "q_flash_candlestick_components_v3"
        }
      ]
    },
    {
      "concept_key": "bullish_bearish_candle",
      "coverage_tags": [
        "bullish_bearish_candle"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_bullish_bearish",
      "learning_goal": "学生能区分阳线和阴线，并知道其颜色惯例。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "definition_with_example",
      "retrieval_focus": "阳线和阴线的定义及视觉特征。",
      "term_refs": [
        {
          "display": "阳线",
          "en": "Bullish Candle"
        },
        {
          "display": "阴线",
          "en": "Bearish Candle"
        }
      ],
      "variants": [
        {
          "back": "绿色或白色，称为阳线（Bullish Candle）。",
          "estimated_seconds": 6,
          "explanation": "阳线表示价格上涨，收盘价高于开盘价。",
          "front": "当收盘价高于开盘价时，K线通常是什么颜色？这种K线叫什么？",
          "question_id": "q_flash_bullish_bearish_v1"
        },
        {
          "back": "红色或黑色，称为阴线（Bearish Candle）。",
          "estimated_seconds": 6,
          "explanation": "阴线表示价格下跌，收盘价低于开盘价。",
          "front": "当收盘价低于开盘价时，K线通常是什么颜色？这种K线叫什么？",
          "question_id": "q_flash_bullish_bearish_v2"
        }
      ]
    },
    {
      "concept_key": "doji_pattern",
      "coverage_tags": [
        "doji_pattern"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_doji",
      "learning_goal": "学生能识别十字星形态并解释其市场含义。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "十字星的定义和市场含义。",
      "term_refs": [
        {
          "display": "十字星",
          "en": "Doji"
        }
      ],
      "variants": [
        {
          "back": "开盘价和收盘价几乎相等，实体非常小。",
          "estimated_seconds": 6,
          "explanation": "十字星的实体极小，表明多空双方力量均衡。",
          "front": "十字星（Doji）形态最显著的特征是什么？",
          "question_id": "q_flash_doji_v1"
        },
        {
          "back": "市场犹豫不决，多空双方力量均衡，可能预示着趋势反转。",
          "estimated_seconds": 8,
          "explanation": "十字星出现在趋势末端时，反转信号更强。",
          "front": "十字星（Doji）形态通常向市场传递什么信号？",
          "question_id": "q_flash_doji_v2"
        }
      ]
    },
    {
      "concept_key": "hammer_pattern",
      "coverage_tags": [
        "hammer_pattern"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_hammer",
      "learning_goal": "学生能识别锤子线形态并解释其看涨反转含义。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "锤子线的形态特征和看涨反转信号。",
      "term_refs": [
        {
          "display": "锤子线",
          "en": "Hammer"
        }
      ],
      "variants": [
        {
          "back": "下跌趋势之后。",
          "estimated_seconds": 5,
          "explanation": "锤子线是下跌趋势后的潜在看涨反转信号。",
          "front": "锤子线（Hammer）形态通常出现在什么趋势之后？",
          "question_id": "q_flash_hammer_v1"
        },
        {
          "back": "下影线很长，实体很小。暗示价格被卖盘压低后，被买盘强势拉回。",
          "estimated_seconds": 10,
          "explanation": "长长的下影线表明买方力量开始介入并推高价格。",
          "front": "锤子线（Hammer）形态最显著的特征是什么？它暗示了什么市场行为？",
          "question_id": "q_flash_hammer_v2"
        }
      ]
    },
    {
      "concept_key": "shooting_star_pattern",
      "coverage_tags": [
        "shooting_star_pattern"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_shooting_star",
      "learning_goal": "学生能识别射击之星形态并解释其看跌反转含义。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "射击之星的形态特征和看跌反转信号。",
      "term_refs": [
        {
          "display": "射击之星",
          "en": "Shooting Star"
        }
      ],
      "variants": [
        {
          "back": "上涨趋势之后。",
          "estimated_seconds": 5,
          "explanation": "射击之星是上涨趋势后的潜在看跌反转信号。",
          "front": "射击之星（Shooting Star）形态通常出现在什么趋势之后？",
          "question_id": "q_flash_shooting_star_v1"
        },
        {
          "back": "上影线很长，实体很小。暗示价格冲高后遭遇强大抛压。",
          "estimated_seconds": 10,
          "explanation": "长长的上影线表明卖方力量开始占据主导。",
          "front": "射击之星（Shooting Star）形态最显著的特征是什么？它暗示了什么市场行为？",
          "question_id": "q_flash_shooting_star_v2"
        }
      ]
    }
  ],
  "lesson_id": "L3",
  "longform_families": [
    {
      "concept_key": "doji_pattern",
      "coverage_tags": [
        "doji_pattern"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_doji_interpretation",
      "learning_goal": "学生能解释十字星形态的形成原因、市场含义及其在趋势分析中的作用。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "十字星",
          "en": "Doji"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "描述十字星的视觉特征（开盘价、收盘价、实体）。",
            "解释其代表的市场心理（多空力量对比）。",
            "说明其在趋势末端出现时的含义。"
          ],
          "question_id": "q_long_doji_interpretation_v1",
          "reference_answer": [
            "十字星形态的特征是开盘价和收盘价几乎相同，导致K线实体非常小，甚至没有实体。",
            "这表明在交易时段内，买卖双方的力量基本均衡，价格在开盘价附近来回拉锯后，最终收盘价又回到了开盘价附近。市场处于一种犹豫不决的状态。",
            "当十字星出现在一段明确的上涨或下跌趋势之后时，它可能意味着推动原有趋势的力量正在减弱，多空双方开始出现分歧。这种力量的均衡状态往往是趋势即将发生转变或进入盘整的前兆，因此被交易者视为潜在的反转信号。"
          ],
          "rubric_points": [
            "正确描述十字星的开盘价与收盘价几乎相等。",
            "正确解释这代表市场犹豫不决，多空力量均衡。",
            "正确指出在强劲趋势后出现，可能意味着原有趋势动力衰竭，存在反转风险。"
          ],
          "stem": "请解释十字星（Doji）K线形态是如何形成的，它向交易者传递了什么市场信息？为什么它常被视为趋势可能反转的信号？"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "分析当前市场背景（连续上涨）。",
            "解释十字星在此背景下的含义。",
            "提出一个基于此信号的交易思路。"
          ],
          "question_id": "q_long_doji_interpretation_v2",
          "reference_answer": [
            "在连续5天上涨后出现十字星，这是一个值得警惕的信号。之前的上涨趋势表明买方一直占据主导。",
            "十字星的出现意味着在当天，卖方力量开始与买方抗衡，使得价格无法继续创出新高，最终收盘价与开盘价持平。这暗示着上涨动力可能正在衰竭，市场情绪从一致看多转为分歧。",
            "作为交易者，我不会立即卖出，但会变得非常谨慎。我可能会考虑部分减仓以锁定利润，或者设置更紧的止损位。我会等待后续K线的确认：如果下一根K线是阴线且收盘价低于十字星，则反转的可能性增加；如果继续收阳，则可能只是上涨过程中的短暂休整。"
          ],
          "rubric_points": [
            "识别出当前处于上涨趋势中。",
            "解释十字星表明买方力量减弱，多空力量趋于平衡。",
            "提出谨慎看多或考虑减仓/观望，等待进一步确认信号。"
          ],
          "stem": "假设你看到某只股票在连续5天上涨后，出现了一根十字星K线。请分析：这根十字星可能意味着什么？作为交易者，你会如何考虑后续操作？"
        }
      ]
    },
    {
      "concept_key": "hammer_pattern",
      "coverage_tags": [
        "hammer_pattern"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_hammer_interpretation",
      "learning_goal": "学生能解释锤子线形态背后的多空博弈过程及其看涨反转含义。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "mechanism_trace",
      "term_refs": [
        {
          "display": "锤子线",
          "en": "Hammer"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "描述锤子线的视觉特征（位置、下影线、实体）。",
            "描述交易时段内价格的变化过程（开盘、下跌、拉回、收盘）。",
            "分析这个过程反映的多空力量变化。"
          ],
          "question_id": "q_long_hammer_interpretation_v1",
          "reference_answer": [
            "锤子线通常出现在一段下跌趋势之后。它的特征是有一个很小的实体位于K线的顶部，以及一条很长的下影线。",
            "在交易时段内，价格开盘后可能延续了之前的下跌趋势，遭到卖盘的猛烈打压，价格大幅下跌，创出当日新低（形成长下影线的低点）。然而，随后买方力量开始介入并逐渐增强，他们相信价格已经跌得足够多，开始积极买入。强大的买盘将价格从低点大幅推回，最终收盘价回到了开盘价附近，形成了一个很小的实体。",
            "这个过程清晰地展示了多空力量的转换：卖方在盘中一度占据绝对优势，但未能守住战果。买方在低位成功阻击并反攻，显示出更强的承接意愿和力量。因此，锤子线被视为下跌趋势可能结束、上涨趋势即将开始的看涨反转信号。"
          ],
          "rubric_points": [
            "正确指出锤子线出现在下跌趋势之后。",
            "正确描述价格在交易中先被大幅压低（形成长下影线），然后被买盘拉回至接近开盘价的位置（形成小实体）。",
            "正确解释这个过程表明卖方力量一度强大，但最终被买方力量击败，买方开始掌握控制权。"
          ],
          "stem": "请描述锤子线（Hammer）K线形态的形成过程。它为什么被视为一个看涨反转信号？请从多空双方力量博弈的角度进行解释。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "分析锤子线出现的市场背景（长期下跌 vs 短期下跌）。",
            "解释背景对形态信号强度的影响。",
            "总结交易员应如何评估这个信号。"
          ],
          "question_id": "q_long_hammer_interpretation_v2",
          "reference_answer": [
            "锤子线在长期下跌后出现，其信号意义远强于在下跌初期出现。经过一个月的下跌，市场情绪通常非常悲观，卖方力量可能已经得到充分释放，甚至开始衰竭。",
            "在这种情况下，一根锤子线的出现意义重大。长长的下影线表明，尽管卖方在当天仍然试图将价格打压至新低，但买方在低位展现了强大的购买意愿和实力，成功将价格拉回。这不仅仅是简单的盘中反弹，而可能代表着市场主导权从卖方到买方的根本性转变。",
            "相比之下，在下跌初期出现的锤子线，可能只是下跌过程中的一次短暂抵抗，卖方力量依然强大，价格很可能继续下跌。因此，交易员会给予在长期下跌趋势末端出现的锤子线更高的权重，将其视为一个更可靠的潜在反转信号，并可能据此考虑建立多头仓位。"
          ],
          "rubric_points": [
            "指出在长期下跌后，市场超卖，卖方力量可能衰竭。",
            "解释此时出现的锤子线，其长下影线更可能代表卖方力量的最后宣泄和买方的决定性反击。",
            "指出在下跌初期，锤子线可能只是短暂反弹，趋势仍可能继续。"
          ],
          "stem": "一个交易员在分析日线图时，发现某股票在经历了一个月的下跌后，今天出现了一根锤子线。请解释为什么这个形态比在下跌初期出现的锤子线更具意义？"
        }
      ]
    },
    {
      "concept_key": "shooting_star_pattern",
      "coverage_tags": [
        "shooting_star_pattern"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_shooting_star_interpretation",
      "learning_goal": "学生能解释射击之星形态背后的多空博弈过程及其看跌反转含义。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "compare_and_contrast",
      "term_refs": [
        {
          "display": "射击之星",
          "en": "Shooting Star"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "描述射击之星的视觉特征（位置、上影线、实体）。",
            "描述交易时段内价格的变化过程（开盘、冲高、回落、收盘）。",
            "分析这个过程反映的多空力量变化。"
          ],
          "question_id": "q_long_shooting_star_interpretation_v1",
          "reference_answer": [
            "射击之星通常出现在一段上涨趋势之后。它的特征是有一个很小的实体位于K线的底部，以及一条很长的上影线。",
            "在交易时段内，价格开盘后延续了之前的上涨趋势，受到买盘的强力推动，价格大幅上涨，创出当日新高（形成长上影线的高点）。然而，随后卖方力量开始介入并逐渐增强，他们选择在高位获利了结或开始做空。强大的卖盘将价格从高点大幅打压回来，最终收盘价回到了开盘价附近，形成了一个很小的实体。",
            "这个过程清晰地展示了多空力量的转换：买方在盘中一度占据绝对优势，但未能守住战果。卖方在高位成功阻击并反攻，显示出更强的抛售意愿和力量。因此，射击之星被视为上涨趋势可能结束、下跌趋势即将开始的看跌反转信号。"
          ],
          "rubric_points": [
            "正确指出射击之星出现在上涨趋势之后。",
            "正确描述价格在交易中先被大幅拉高（形成长上影线），然后被卖盘打压回至接近开盘价的位置（形成小实体）。",
            "正确解释这个过程表明买方力量一度强大，但最终被卖方力量压制，卖方开始掌握控制权。"
          ],
          "stem": "请描述射击之星（Shooting Star）K线形态的形成过程。它为什么被视为一个看跌反转信号？请从多空双方力量博弈的角度进行解释。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "分别描述两种形态的视觉特征（实体、影线）。",
            "分别说明它们通常出现的趋势位置。",
            "对比它们预示的反转方向。"
          ],
          "question_id": "q_long_shooting_star_interpretation_v2",
          "reference_answer": [
            "锤子线和射击之星在视觉上是镜像关系。锤子线有一个很小的实体位于K线的上端，并带有很长的下影线；而射击之星有一个很小的实体位于K线的下端，并带有很长的上影线。",
            "它们出现的位置也完全相反。锤子线通常出现在一段下跌趋势之后，表明卖方力量衰竭，买方开始反击。射击之星则出现在一段上涨趋势之后，表明买方力量衰竭，卖方开始反扑。",
            "因此，它们预示的市场方向也截然不同。锤子线被视为潜在的看涨反转信号，预示着价格可能从下跌转为上涨。射击之星则被视为潜在的看跌反转信号，预示着价格可能从上涨转为下跌。理解这两种形态的对比，有助于交易者快速识别市场潜在的趋势转折点。"
          ],
          "rubric_points": [
            "正确指出锤子线有长下影线、小实体在顶部；射击之星有长上影线、小实体在底部。",
            "正确指出锤子线出现在下跌趋势后；射击之星出现在上涨趋势后。",
            "正确指出锤子线是看涨反转信号；射击之星是看跌反转信号。"
          ],
          "stem": "请比较锤子线和射击之星这两种K线形态。它们在视觉特征、出现位置以及所预示的市场反转方向上有什么不同？"
        }
      ]
    },
    {
      "concept_key": "pattern_trend_reversal",
      "coverage_tags": [
        "pattern_trend_reversal",
        "doji_pattern",
        "hammer_pattern",
        "shooting_star_pattern"
      ],
      "difficulty": "hard",
      "family_id": "qf_long_compare_reversal_patterns",
      "learning_goal": "学生能比较十字星、锤子线和射击之星这三种常见反转形态的异同。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "compare_and_contrast",
      "term_refs": [
        {
          "display": "十字星",
          "en": "Doji"
        },
        {
          "display": "锤子线",
          "en": "Hammer"
        },
        {
          "display": "射击之星",
          "en": "Shooting Star"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 150,
          "prompt_blocks": [
            "分别描述三种形态的视觉特征。",
            "分别说明它们通常出现的趋势位置。",
            "对比它们预示的反转方向。"
          ],
          "question_id": "q_long_compare_reversal_patterns_v1",
          "reference_answer": [
            "视觉特征上：十字星的实体非常小，开盘价和收盘价几乎相同，影线可长可短。锤子线有一个小实体位于K线上端，并有一条很长的下影线。射击之星则有一个小实体位于K线下端，并有一条很长的上影线。",
            "出现的趋势背景：十字星可以出现在任何趋势中，但当它出现在一段明确的趋势末端时，其反转意义更强。锤子线必须出现在下跌趋势之后。射击之星必须出现在上涨趋势之后。",
            "预示的反转方向：十字星本身是中性的，它只代表犹豫不决，需要结合后续K线来确认反转方向。锤子线是看涨反转信号，预示下跌趋势可能结束，价格将上涨。射击之星是看跌反转信号，预示上涨趋势可能结束，价格将下跌。"
          ],
          "rubric_points": [
            "正确描述十字星：实体极小，开盘收盘价接近。锤子线：小实体在顶部，长下影线。射击之星：小实体在底部，长上影线。",
            "正确指出十字星可出现在任何趋势后，但反转意义在趋势末端更强。锤子线在下跌趋势后。射击之星在上涨趋势后。",
            "正确指出十字星是中性反转信号，需后续确认。锤子线是看涨反转。射击之星是看跌反转。"
          ],
          "stem": "十字星、锤子线和射击之星都是常见的K线反转形态。请从以下三个方面对它们进行比较：1) 视觉特征；2) 出现的典型趋势背景；3) 所预示的反转方向。"
        },
        {
          "estimated_seconds": 150,
          "prompt_blocks": [
            "分析每种情况下的市场背景。",
            "解释该K线形态在此背景下的具体含义。",
            "提出一个基于此信号的交易思路。"
          ],
          "question_id": "q_long_compare_reversal_patterns_v2",
          "reference_answer": [
            "情况A（上涨后出现射击之星）：这是一个强烈的警告信号，表明上涨动力衰竭，卖方开始主导市场。我会考虑平掉部分多头仓位以锁定利润，或者建立空头仓位。我会将止损位设置在射击之星上影线的上方。",
            "情况B（下跌后出现锤子线）：这是一个积极的信号，表明下跌趋势可能结束，买方开始反击。我会考虑建立多头仓位。我会将止损位设置在锤子线下影线的下方。",
            "情况C（盘整中出现十字星）：十字星出现在盘整行情中，其意义不大，因为它只是确认了市场原有的犹豫不决状态。它没有提供明确的方向性信号。在这种情况下，最好的策略是继续观望，等待价格突破盘整区间后再做决策。"
          ],
          "rubric_points": [
            "情况A：正确识别为看跌反转信号，考虑减仓或做空。",
            "情况B：正确识别为看涨反转信号，考虑买入或加仓。",
            "情况C：正确识别为市场犹豫不决，趋势不明，建议观望。"
          ],
          "stem": "假设你是一个交易员，在分析图表时发现了以下三种情况。请分别解释每种情况下K线形态的含义，并说明你会如何利用这些信息。\n情况A：在长期上涨后出现一根射击之星。\n情况B：在长期下跌后出现一根锤子线。\n情况C：在一段盘整行情中出现一根十字星。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "candlestick_intro",
      "coverage_tags": [
        "candlestick_intro"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_candlestick_definition",
      "learning_goal": "学生能准确选择K线图的定义和用途。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "K线图",
          "en": "Candlestick Chart"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "K线图是一种金融图表，用于直观展示资产在特定时间段内的价格变动，包括开盘、收盘、最高和最低价。",
          "options": [
            "预测未来股价的精确数值",
            "展示资产在一段时间内的价格波动",
            "计算交易策略的盈亏",
            "确定买卖股票的最佳数量"
          ],
          "question_id": "q_quiz_candlestick_definition_v1",
          "stem": "K线图（蜡烛图）的主要作用是什么？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "标准的单根K线图只包含OHLC（开盘、最高、最低、收盘）四个价格信息，不直接显示成交量。",
          "options": [
            "开盘价",
            "收盘价",
            "成交量",
            "最高价"
          ],
          "question_id": "q_quiz_candlestick_definition_v2",
          "stem": "以下哪项是K线图无法直接展示的信息？"
        }
      ]
    },
    {
      "concept_key": "ohlc_components",
      "coverage_tags": [
        "ohlc_components"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_ohlc_meaning",
      "learning_goal": "学生能将OHLC四个缩写与其含义正确匹配。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "matching",
      "term_refs": [
        {
          "display": "开盘价",
          "en": "Open"
        },
        {
          "display": "收盘价",
          "en": "Close"
        },
        {
          "display": "最高价",
          "en": "High"
        },
        {
          "display": "最低价",
          "en": "Low"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 20,
          "explanation": "O=Open(开盘价), H=High(最高价), L=Low(最低价), C=Close(收盘价)。",
          "pairs": [
            {
              "left": "O",
              "right": "时间段开始时的价格"
            },
            {
              "left": "H",
              "right": "时间段内的最高价格"
            },
            {
              "left": "L",
              "right": "时间段内的最低价格"
            },
            {
              "left": "C",
              "right": "时间段结束时的价格"
            }
          ],
          "question_id": "q_quiz_ohlc_meaning_v1",
          "stem": "请将左侧的OHLC缩写与右侧的正确描述配对。"
        }
      ]
    },
    {
      "concept_key": "bullish_bearish_candle",
      "coverage_tags": [
        "bullish_bearish_candle"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_bullish_bearish_identification",
      "learning_goal": "学生能根据开盘价和收盘价的关系判断K线是阳线还是阴线。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "阳线",
          "en": "Bullish Candle"
        },
        {
          "display": "阴线",
          "en": "Bearish Candle"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 10,
          "explanation": "收盘价(105) > 开盘价(100)，因此是阳线。",
          "options": [
            "阳线",
            "阴线",
            "十字星",
            "无法判断"
          ],
          "question_id": "q_quiz_bullish_bearish_identification_v1",
          "stem": "某日股票开盘价为100元，收盘价为105元。这根K线是？"
        },
        {
          "answer": 2,
          "estimated_seconds": 10,
          "explanation": "收盘价(48) < 开盘价(50)，是阴线，通常用红色或黑色表示。",
          "options": [
            "绿色",
            "白色",
            "红色",
            "蓝色"
          ],
          "question_id": "q_quiz_bullish_bearish_identification_v2",
          "stem": "某日股票开盘价为50元，收盘价为48元。这根K线通常是什么颜色？"
        }
      ]
    },
    {
      "concept_key": "doji_pattern",
      "coverage_tags": [
        "doji_pattern"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_doji_meaning",
      "learning_goal": "学生能理解十字星形态的市场含义。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "十字星",
          "en": "Doji"
        }
      ],
      "variants": [
        {
          "answer": 3,
          "estimated_seconds": 15,
          "explanation": "十字星的开盘价和收盘价几乎相等，表明买卖双方力量暂时平衡，市场处于犹豫状态。",
          "options": [
            "买方力量绝对占优",
            "卖方力量绝对占优",
            "市场趋势将加速",
            "市场多空双方力量均衡，犹豫不决"
          ],
          "question_id": "q_quiz_doji_meaning_v1",
          "stem": "当市场出现十字星（Doji）形态时，通常意味着什么？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "上涨趋势后的十字星表明买方力量减弱，多空力量趋于平衡，可能预示着上涨趋势的暂停或反转。",
          "options": [
            "趋势将继续加速上涨",
            "趋势可能即将反转或进入盘整",
            "这是一个强烈的买入信号",
            "市场情绪非常乐观"
          ],
          "question_id": "q_quiz_doji_meaning_v2",
          "stem": "在一段强劲的上涨趋势后出现十字星，这通常被解读为什么信号？"
        }
      ]
    },
    {
      "concept_key": "hammer_pattern",
      "coverage_tags": [
        "hammer_pattern"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_hammer_identification",
      "learning_goal": "学生能识别锤子线形态并理解其看涨含义。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "锤子线",
          "en": "Hammer"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 10,
          "explanation": "锤子线的典型特征是下影线很长，实体很小，且通常出现在下跌趋势后。",
          "options": [
            "实体很长，没有影线",
            "上影线很长，实体很小",
            "下影线很长，实体很小",
            "开盘价和收盘价几乎相等"
          ],
          "question_id": "q_quiz_hammer_identification_v1",
          "stem": "以下哪种K线形态最符合锤子线的特征？"
        },
        {
          "answer": 1,
          "estimated_seconds": 10,
          "explanation": "锤子线出现在下跌趋势后，长长的下影线表明买方开始介入，是潜在的看涨反转信号。",
          "options": [
            "看跌反转信号",
            "看涨反转信号",
            "趋势持续信号",
            "中性信号"
          ],
          "question_id": "q_quiz_hammer_identification_v2",
          "stem": "锤子线形态通常被视为什么信号？"
        }
      ]
    },
    {
      "concept_key": "shooting_star_pattern",
      "coverage_tags": [
        "shooting_star_pattern"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_shooting_star_identification",
      "learning_goal": "学生能识别射击之星形态并理解其看跌含义。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "射击之星",
          "en": "Shooting Star"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 10,
          "explanation": "射击之星的特征是出现在上涨趋势后，上影线很长，实体很小。",
          "options": [
            "出现在下跌趋势后，下影线很长",
            "出现在上涨趋势后，上影线很长",
            "实体很长，上下影线都很短",
            "开盘价和收盘价完全相同"
          ],
          "question_id": "q_quiz_shooting_star_identification_v1",
          "stem": "以下哪种K线形态最符合射击之星的描述？"
        },
        {
          "answer": 1,
          "estimated_seconds": 10,
          "explanation": "射击之星的上影线表明价格冲高后遭遇抛压，是潜在的看跌反转信号。",
          "options": [
            "价格将继续上涨",
            "价格可能即将下跌",
            "市场非常稳定",
            "是强烈的买入信号"
          ],
          "question_id": "q_quiz_shooting_star_identification_v2",
          "stem": "射击之星形态通常预示着什么？"
        }
      ]
    },
    {
      "concept_key": "pattern_trend_reversal",
      "coverage_tags": [
        "pattern_trend_reversal"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_pattern_reversal_signal",
      "learning_goal": "学生能区分锤子线和射击之星所代表的不同反转方向。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "锤子线",
          "en": "Hammer"
        },
        {
          "display": "射击之星",
          "en": "Shooting Star"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "锤子线是看涨反转信号，射击之星是看跌反转信号。",
          "options": [
            "都预示上涨",
            "都预示下跌",
            "锤子线预示上涨，射击之星预示下跌",
            "锤子线预示下跌，射击之星预示上涨"
          ],
          "question_id": "q_quiz_pattern_reversal_signal_v1",
          "stem": "在下跌趋势后出现锤子线，在上涨趋势后出现射击之星。这两个形态分别预示什么？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "在下跌趋势后出现下影线很长的K线，符合锤子线的特征，是潜在的看涨反转信号。",
          "options": [
            "继续下跌的信号",
            "潜在的看涨反转信号",
            "市场完全中性的信号",
            "强烈的看跌信号"
          ],
          "question_id": "q_quiz_pattern_reversal_signal_v2",
          "stem": "一个交易员观察到某股票在连续下跌后出现了一根下影线很长的K线。这最可能是什么信号？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L3: Building backtest framework and rule-based trading strategy - Common Candlestick Patterns",
    "guided_story_manifest": "pipeline/3-guided_story/L3/step3/step.json",
    "lesson_map": "L3 step3: K线图与常见形态",
    "plain_text": "pipeline/1-plain/L3/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L3: Building backtest framework and rule-based trading strategy - Candlestick - Introduction to Candlestick - Understanding Candlestick Components - Common Candlestick Patterns"
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "data_cleaning_definition",
      "covered_by": [
        "qf_flash_data_cleaning_def",
        "qf_quiz_data_cleaning_purpose"
      ],
      "description": "数据清洗的定义、目的和重要性"
    },
    {
      "coverage_tag": "data_cleaning_common_issues",
      "covered_by": [
        "qf_flash_data_issues",
        "qf_quiz_data_issues_identify",
        "qf_long_data_issues_handling"
      ],
      "description": "数据清洗中常见的三种问题：缺失值、重复记录、逻辑错误"
    },
    {
      "coverage_tag": "data_cleaning_missing_values",
      "covered_by": [
        "qf_flash_missing_handling",
        "qf_quiz_missing_handling"
      ],
      "description": "缺失值的处理方法：补全、填充、删除"
    },
    {
      "coverage_tag": "data_cleaning_duplicates",
      "covered_by": [
        "qf_flash_duplicate_handling",
        "qf_quiz_duplicate_handling"
      ],
      "description": "重复记录的处理方法：保留一条、核对后处理"
    },
    {
      "coverage_tag": "data_cleaning_logic_errors",
      "covered_by": [
        "qf_flash_logic_error_example",
        "qf_quiz_logic_error_identify",
        "qf_long_logic_error_handling"
      ],
      "description": "逻辑错误的识别与处理方法"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "data_cleaning_definition",
      "coverage_tags": [
        "data_cleaning_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_data_cleaning_def",
      "learning_goal": "学生能准确说出数据清洗的定义和核心目的。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "数据清洗的定义和目的",
      "term_refs": [
        {
          "display": "数据清洗",
          "en": "Data Cleaning"
        }
      ],
      "variants": [
        {
          "back": "让数据更准确、一致，为后续分析打好基础。",
          "estimated_seconds": 8,
          "explanation": "数据清洗旨在去除数据中的错误、缺失、重复和不一致，提高数据质量。",
          "front": "数据清洗的核心目的是什么？",
          "question_id": "q_flash_data_cleaning_def_v1"
        },
        {
          "back": "数据收集之后。",
          "estimated_seconds": 6,
          "explanation": "回测的标准流程是：收集数据 → 清洗数据 → 运行策略 → 评估表现。",
          "front": "在回测流程中，数据清洗处于哪个步骤之后？",
          "question_id": "q_flash_data_cleaning_def_v2"
        }
      ]
    },
    {
      "concept_key": "data_cleaning_common_issues",
      "coverage_tags": [
        "data_cleaning_common_issues"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_data_issues",
      "learning_goal": "学生能列举出数据清洗中常见的三种问题类型。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "数据清洗的三大常见问题",
      "term_refs": [
        {
          "display": "缺失值",
          "en": "Missing Values"
        },
        {
          "display": "重复记录",
          "en": "Duplicated Records"
        },
        {
          "display": "逻辑错误",
          "en": "Incorrect Logics"
        }
      ],
      "variants": [
        {
          "back": "缺失值、重复记录、逻辑错误。",
          "estimated_seconds": 8,
          "explanation": "这三种问题会严重影响数据质量和后续分析的准确性。",
          "front": "原始数据中常见的三种问题是什么？",
          "question_id": "q_flash_data_issues_v1"
        },
        {
          "back": "逻辑错误。",
          "estimated_seconds": 7,
          "explanation": "逻辑错误，如收盘价高于最高价，违反了基本的价格逻辑，需要被识别和修正。",
          "front": "除了缺失值和重复记录，数据清洗还需要处理哪一类更隐蔽的问题？",
          "question_id": "q_flash_data_issues_v2"
        }
      ]
    },
    {
      "concept_key": "data_cleaning_missing_values",
      "coverage_tags": [
        "data_cleaning_missing_values"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_missing_handling",
      "learning_goal": "学生能说出处理缺失值的三种常见方法。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "缺失值的处理方法",
      "term_refs": [
        {
          "display": "缺失值",
          "en": "Missing Values"
        }
      ],
      "variants": [
        {
          "back": "用相邻数据填充，或者直接删除这一行。",
          "estimated_seconds": 10,
          "explanation": "这三种方法各有优劣，选择取决于数据的重要性和缺失程度。",
          "front": "处理缺失值的一种方法是“从其他来源补全”，请说出另外两种。",
          "question_id": "q_flash_missing_handling_v1"
        },
        {
          "back": "用相邻几天的平均值填充。",
          "estimated_seconds": 8,
          "explanation": "这是一种常见的基于相邻观测值的填充方法，可以保持数据序列的连续性。",
          "front": "如果某天的收盘价数据缺失，且无法从其他来源获取，一种简单的处理方式是什么？",
          "question_id": "q_flash_missing_handling_v2"
        }
      ]
    },
    {
      "concept_key": "data_cleaning_duplicates",
      "coverage_tags": [
        "data_cleaning_duplicates"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_duplicate_handling",
      "learning_goal": "学生能说出处理完全相同的重复记录的基本方法。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "重复记录的处理方法",
      "term_refs": [
        {
          "display": "重复记录",
          "en": "Duplicated Records"
        }
      ],
      "variants": [
        {
          "back": "只保留其中一条。",
          "estimated_seconds": 6,
          "explanation": "完全相同的重复记录不提供额外信息，保留一条即可。",
          "front": "对于完全相同的重复记录，通常的处理方法是什么？",
          "question_id": "q_flash_duplicate_handling_v1"
        },
        {
          "back": "从其他数据源核对，或删除所有重复记录，或取它们的平均值。",
          "estimated_seconds": 10,
          "explanation": "对于不完全相同的重复记录，需要更谨慎地处理，以确定哪条数据是正确的。",
          "front": "如果同一天出现两条数据，但内容不完全相同，应该怎么处理？",
          "question_id": "q_flash_duplicate_handling_v2"
        }
      ]
    },
    {
      "concept_key": "data_cleaning_logic_errors",
      "coverage_tags": [
        "data_cleaning_logic_errors"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_logic_error_example",
      "learning_goal": "学生能举出一个数据中逻辑错误的典型例子。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "逻辑错误的典型例子",
      "term_refs": [
        {
          "display": "逻辑错误",
          "en": "Incorrect Logics"
        }
      ],
      "variants": [
        {
          "back": "收盘价高于当日最高价。",
          "estimated_seconds": 7,
          "explanation": "收盘价不可能高于当日最高价，这违反了基本的价格逻辑。",
          "front": "请举出一个数据中逻辑错误的例子。",
          "question_id": "q_flash_logic_error_example_v1"
        },
        {
          "back": "开盘价低于当日最低价。",
          "estimated_seconds": 7,
          "explanation": "开盘价必须在当日最高价和最低价之间，否则就是逻辑错误。",
          "front": "除了收盘价高于最高价，还有哪种情况属于逻辑错误？",
          "question_id": "q_flash_logic_error_example_v2"
        }
      ]
    }
  ],
  "lesson_id": "L3",
  "longform_families": [
    {
      "concept_key": "data_cleaning_common_issues",
      "coverage_tags": [
        "data_cleaning_common_issues",
        "data_cleaning_missing_values",
        "data_cleaning_duplicates",
        "data_cleaning_logic_errors"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_data_issues_handling",
      "learning_goal": "学生能解释数据清洗中三种常见问题的具体表现和相应的处理方法。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "数据清洗",
          "en": "Data Cleaning"
        },
        {
          "display": "缺失值",
          "en": "Missing Values"
        },
        {
          "display": "重复记录",
          "en": "Duplicated Records"
        },
        {
          "display": "逻辑错误",
          "en": "Incorrect Logics"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "缺失值的表现与例子",
            "重复记录的表现与例子",
            "逻辑错误的表现与例子"
          ],
          "question_id": "q_long_data_issues_handling_v1",
          "reference_answer": [
            "缺失值：数据集中某些字段没有值。例如，某一天的收盘价数据缺失，单元格为空。",
            "重复记录：数据集中存在完全相同的行。例如，2023年1月5日的所有价格和交易量数据出现了两次。",
            "逻辑错误：数据违反了基本的业务或物理逻辑。例如，某天的收盘价是105，但当日最高价是100，这在逻辑上是不可能的。"
          ],
          "rubric_points": [
            "正确解释缺失值的表现（如某字段为空）并给出恰当例子（如某天收盘价缺失）。",
            "正确解释重复记录的表现（如完全相同的行）并给出恰当例子（如同一天出现两次相同数据）。",
            "正确解释逻辑错误的表现（如数据违反基本规则）并给出恰当例子（如收盘价高于最高价）。"
          ],
          "stem": "请分别说明数据清洗中“缺失值”、“重复记录”和“逻辑错误”这三种问题的具体表现，并为每种问题各举一个例子。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "处理缺失值的方法与理由",
            "处理重复记录的方法与理由",
            "处理逻辑错误的方法与理由"
          ],
          "question_id": "q_long_data_issues_handling_v2",
          "reference_answer": [
            "缺失值：可以用前后两天开盘价的平均值来填充。理由是这种方法能利用相邻数据点，保持时间序列的平滑性。",
            "重复记录：直接删除其中一条，只保留一条。理由是两条完全相同的记录不提供额外信息，保留一条即可。",
            "逻辑错误：删除该行数据。理由是收盘价低于最低价是明显的错误，该数据不可靠，删除是最直接的方法。"
          ],
          "rubric_points": [
            "为缺失值提出合理方法（如用相邻数据填充）并给出理由（如保持数据连续性）。",
            "为重复记录提出合理方法（如保留一条）并给出理由（如不提供额外信息）。",
            "为逻辑错误提出合理方法（如删除该行）并给出理由（如数据不可靠）。"
          ],
          "stem": "假设你在进行回测时，发现数据集中存在以下三种问题：1) 某天开盘价缺失；2) 同一天出现两条完全相同的记录；3) 某天收盘价低于当日最低价。请分别为每种问题提出一种可行的处理方法，并简要说明理由。"
        }
      ]
    },
    {
      "concept_key": "data_cleaning_logic_errors",
      "coverage_tags": [
        "data_cleaning_logic_errors"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_logic_error_handling",
      "learning_goal": "学生能解释在Excel中如何通过逻辑公式来识别数据中的逻辑错误。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "mechanism_trace",
      "term_refs": [
        {
          "display": "逻辑错误",
          "en": "Incorrect Logics"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "写出用于判断的Excel公式",
            "解释公式中每个部分的作用"
          ],
          "question_id": "q_long_logic_error_handling_v1",
          "reference_answer": [
            "公式：=AND(E2<=C2, E2>=D2, B2<=C2, B2>=D2)",
            "解释：AND函数要求所有条件都为TRUE时，结果才为TRUE。E2<=C2检查收盘价是否小于等于最高价；E2>=D2检查收盘价是否大于等于最低价；B2<=C2检查开盘价是否小于等于最高价；B2>=D2检查开盘价是否大于等于最低价。只有当这四个条件都满足时，该行数据才被视为有效。"
          ],
          "rubric_points": [
            "正确写出公式，如 =AND(E2<=C2, E2>=D2, B2<=C2, B2>=D2)。",
            "正确解释AND函数用于同时满足多个条件。",
            "正确解释每个比较条件（如E2<=C2表示收盘价小于等于最高价）的含义。"
          ],
          "stem": "在Excel中，要检查某一行数据是否有效，需要验证收盘价和开盘价是否都在最高价和最低价之间。请写出一个Excel公式（假设数据在A到E列，分别为日期、开盘价、最高价、最低价、收盘价），用于判断第2行数据是否有效，并解释该公式的逻辑。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "方法一：删除该行",
            "方法二：将收盘价修正为最高价"
          ],
          "question_id": "q_long_logic_error_handling_v2",
          "reference_answer": [
            "方法一：删除该行。优点是操作简单，能彻底排除错误数据。缺点是会丢失该天的所有数据，如果错误数据较多，会导致样本量减少。",
            "方法二：将收盘价修正为最高价。优点是保留了该天的数据，且修正后的值在逻辑上成立。缺点是修正后的数据并非真实值，可能会对分析结果引入偏差。",
            "方法三：从其他数据源核对。优点是能获得最准确的数据。缺点是耗时，且不一定能找到其他数据源。"
          ],
          "rubric_points": [
            "正确描述删除方法的优缺点（优点：简单直接，缺点：丢失数据）。",
            "正确描述修正方法的优缺点（优点：保留数据，缺点：可能引入偏差）。",
            "能提出其他合理方法（如从其他数据源核对）并说明优缺点。"
          ],
          "stem": "假设你发现某行数据中，收盘价高于最高价。请列出至少两种处理这种逻辑错误的方法，并分别说明每种方法的优缺点。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "data_cleaning_definition",
      "coverage_tags": [
        "data_cleaning_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_data_cleaning_purpose",
      "learning_goal": "学生能在多个选项中准确识别数据清洗的主要目的。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "数据清洗",
          "en": "Data Cleaning"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "数据清洗的目的是提高数据质量，为后续分析打好基础，而不是直接预测未来。",
          "options": [
            "去除数据录入错误",
            "提高数据质量",
            "预测未来股价",
            "让数据更一致"
          ],
          "question_id": "q_quiz_data_cleaning_purpose_v1",
          "stem": "以下哪项不是数据清洗的主要目的？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "通过清洗数据，可以提高数据质量，从而产生更准确的统计模型，但无法保证盈利或消除风险。",
          "options": [
            "增加数据量",
            "产生更准确的统计模型",
            "保证策略一定盈利",
            "消除所有交易风险"
          ],
          "question_id": "q_quiz_data_cleaning_purpose_v2",
          "stem": "数据清洗能带来以下哪些好处？"
        }
      ]
    },
    {
      "concept_key": "data_cleaning_common_issues",
      "coverage_tags": [
        "data_cleaning_common_issues"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_data_issues_identify",
      "learning_goal": "学生能根据描述判断数据问题属于缺失值、重复记录还是逻辑错误。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "缺失值",
          "en": "Missing Values"
        },
        {
          "display": "重复记录",
          "en": "Duplicated Records"
        },
        {
          "display": "逻辑错误",
          "en": "Incorrect Logics"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 15,
          "explanation": "数据全部为空，属于典型的缺失值问题。",
          "options": [
            "缺失值",
            "重复记录",
            "逻辑错误",
            "数据格式错误"
          ],
          "question_id": "q_quiz_data_issues_identify_v1",
          "stem": "某天的开盘价、最高价、最低价、收盘价数据全部为空，这属于哪种数据问题？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "同一天出现两条完全一样的数据，属于重复记录。",
          "options": [
            "缺失值",
            "重复记录",
            "逻辑错误",
            "数据漂移"
          ],
          "question_id": "q_quiz_data_issues_identify_v2",
          "stem": "数据表中，2023年1月5日的数据出现了两次，且所有字段都相同，这属于哪种数据问题？"
        }
      ]
    },
    {
      "concept_key": "data_cleaning_missing_values",
      "coverage_tags": [
        "data_cleaning_missing_values"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_missing_handling",
      "learning_goal": "学生能根据场景选择合适的缺失值处理方法。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "缺失值",
          "en": "Missing Values"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "用相邻数据填充是一种常见且合理的处理方法，可以保持数据序列的连续性。",
          "options": [
            "用当天的收盘价代替",
            "用前后两天的开盘价平均值填充",
            "删除整只股票的所有数据",
            "忽略该问题，继续使用数据"
          ],
          "question_id": "q_quiz_missing_handling_v1",
          "stem": "如果某只股票的历史数据中，只有一天的开盘价缺失，而其他数据源也无法提供，最直接的处理方法是？"
        },
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "当数据缺失严重时，填充可能会引入较大偏差，直接删除该行是更稳妥的做法。",
          "options": [
            "用0填充所有缺失字段",
            "用该列的平均值填充",
            "直接删除该行数据",
            "从其他数据源复制一行数据"
          ],
          "question_id": "q_quiz_missing_handling_v2",
          "stem": "当数据集中某一行的大部分字段都缺失时，最合适的处理方法是？"
        }
      ]
    },
    {
      "concept_key": "data_cleaning_duplicates",
      "coverage_tags": [
        "data_cleaning_duplicates"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_duplicate_handling",
      "learning_goal": "学生能判断处理重复记录的正确做法。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "重复记录",
          "en": "Duplicated Records"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "COUNTIF函数可以统计某个值在指定范围内出现的次数，从而快速识别重复项。",
          "options": [
            "SUM",
            "AVERAGE",
            "COUNTIF",
            "IF"
          ],
          "question_id": "q_quiz_duplicate_handling_v1",
          "stem": "在Excel中，要快速找出重复的日期记录，可以使用哪个函数？"
        },
        {
          "answer": 3,
          "estimated_seconds": 20,
          "explanation": "随机保留一条会引入不确定性，可能导致数据失真，是最不可取的方法。",
          "options": [
            "从其他数据源核对哪个是正确的",
            "删除所有重复记录",
            "取两条记录的平均值",
            "随机保留其中一条"
          ],
          "question_id": "q_quiz_duplicate_handling_v2",
          "stem": "对于两个日期相同但价格数据不同的重复记录，以下哪种处理方式最不可取？"
        }
      ]
    },
    {
      "concept_key": "data_cleaning_logic_errors",
      "coverage_tags": [
        "data_cleaning_logic_errors"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_logic_error_identify",
      "learning_goal": "学生能识别出数据中违反基本逻辑的错误。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "逻辑错误",
          "en": "Incorrect Logics"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "最高价(98)低于开盘价(100)，这在逻辑上是不可能的。",
          "options": [
            "开盘价: 100, 最高价: 105, 最低价: 95, 收盘价: 102",
            "开盘价: 100, 最高价: 102, 最低价: 98, 收盘价: 99",
            "开盘价: 100, 最高价: 98, 最低价: 95, 收盘价: 97",
            "开盘价: 100, 最高价: 103, 最低价: 99, 收盘价: 101"
          ],
          "question_id": "q_quiz_logic_error_identify_v1",
          "stem": "以下哪条数据记录很可能包含逻辑错误？"
        },
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "一个有效的收盘价必须同时满足小于等于最高价和大于等于最低价两个条件。",
          "options": [
            "收盘价 <= 最高价 AND 收盘价 >= 最低价",
            "收盘价 >= 最高价 OR 收盘价 <= 最低价",
            "收盘价 = 最高价 AND 收盘价 = 最低价",
            "收盘价 <> 最高价 AND 收盘价 <> 最低价"
          ],
          "question_id": "q_quiz_logic_error_identify_v2",
          "stem": "在Excel中，要检查收盘价是否在最高价和最低价之间，可以使用什么逻辑判断？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/L3/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L3/plain.txt",
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
      "coverage_tag": "excel_ma_cross_steps",
      "covered_by": [
        "qf_flash_ma_calc",
        "qf_flash_signal_logic",
        "qf_flash_position_logic",
        "qf_flash_pnl_calc",
        "qf_quiz_signal_identification",
        "qf_quiz_position_logic",
        "qf_long_excel_workflow"
      ],
      "description": "在Excel中实现MA交叉策略的完整步骤：计算MA(7)和MA(14)、识别交叉信号、计算持仓、计算每日盈亏、计算累计盈亏。"
    },
    {
      "coverage_tag": "excel_ma_calc_function",
      "covered_by": [
        "qf_flash_ma_calc",
        "qf_quiz_ma_function"
      ],
      "description": "在Excel中使用AVERAGE函数计算移动平均线。"
    },
    {
      "coverage_tag": "excel_signal_identification",
      "covered_by": [
        "qf_flash_signal_logic",
        "qf_quiz_signal_identification"
      ],
      "description": "在Excel中识别黄金交叉（买入信号1）和死亡交叉（卖出信号-1）的逻辑。"
    },
    {
      "coverage_tag": "excel_position_calculation",
      "covered_by": [
        "qf_flash_position_logic",
        "qf_quiz_position_logic"
      ],
      "description": "在Excel中根据信号和当前持仓状态计算每日持仓的逻辑。"
    },
    {
      "coverage_tag": "excel_daily_pnl_calculation",
      "covered_by": [
        "qf_flash_pnl_calc",
        "qf_quiz_pnl_calc"
      ],
      "description": "在Excel中计算每日盈亏的公式：持仓 * (今日收盘价 - 昨日收盘价)。"
    },
    {
      "coverage_tag": "excel_cumulative_pnl_calculation",
      "covered_by": [
        "qf_flash_pnl_calc",
        "qf_long_excel_workflow"
      ],
      "description": "在Excel中通过累加每日盈亏计算累计盈亏。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "excel_ma_calc_function",
      "coverage_tags": [
        "excel_ma_cross_steps",
        "excel_ma_calc_function"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_ma_calc",
      "learning_goal": "学生能准确回忆在Excel中计算移动平均线所使用的函数和参数范围。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "在Excel中计算特定周期移动平均线的函数名称和参数范围。",
      "term_refs": [
        {
          "display": "移动平均线",
          "en": "Moving Average"
        },
        {
          "display": "AVERAGE函数",
          "en": "AVERAGE function"
        }
      ],
      "variants": [
        {
          "back": "AVERAGE",
          "estimated_seconds": 5,
          "explanation": "AVERAGE函数用于计算一组数值的平均值，非常适合计算简单移动平均线。",
          "front": "在Excel中，要计算7日移动平均线，应该使用哪个函数？",
          "question_id": "q_flash_ma_calc_v1"
        },
        {
          "back": "收盘价（Close）列",
          "estimated_seconds": 8,
          "explanation": "移动平均线通常基于收盘价计算，因此AVERAGE函数应引用收盘价列的数据。",
          "front": "在Excel中，要计算MA(14)，AVERAGE函数应该引用哪一列数据？",
          "question_id": "q_flash_ma_calc_v2"
        },
        {
          "back": "7行",
          "estimated_seconds": 5,
          "explanation": "MA(7)是过去7个周期的平均价格，因此需要引用最近的7行收盘价数据。",
          "front": "在Excel中，计算MA(7)时，AVERAGE函数需要引用多少行收盘价数据？",
          "question_id": "q_flash_ma_calc_v3"
        }
      ]
    },
    {
      "concept_key": "excel_signal_identification",
      "coverage_tags": [
        "excel_ma_cross_steps",
        "excel_signal_identification"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_signal_logic",
      "learning_goal": "学生能准确回忆在Excel中识别黄金交叉和死亡交叉的条件。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "在Excel中，黄金交叉和死亡交叉的IF条件判断逻辑。",
      "term_refs": [
        {
          "display": "黄金交叉",
          "en": "Golden Cross"
        },
        {
          "display": "死亡交叉",
          "en": "Death Cross"
        }
      ],
      "variants": [
        {
          "back": "1（买入信号）",
          "estimated_seconds": 8,
          "explanation": "MA(7)上穿MA(14)是黄金交叉，产生买入信号，标记为1。",
          "front": "在Excel中，当MA(7)上穿MA(14)时，信号列应标记为什么值？",
          "question_id": "q_flash_signal_logic_v1"
        },
        {
          "back": "-1（卖出信号）",
          "estimated_seconds": 8,
          "explanation": "MA(7)下穿MA(14)是死亡交叉，产生卖出信号，标记为-1。",
          "front": "在Excel中，当MA(7)下穿MA(14)时，信号列应标记为什么值？",
          "question_id": "q_flash_signal_logic_v2"
        },
        {
          "back": "0",
          "estimated_seconds": 5,
          "explanation": "没有交叉信号时，信号列标记为0，表示无操作。",
          "front": "在Excel中，如果MA(7)和MA(14)没有发生交叉，信号列应标记为什么值？",
          "question_id": "q_flash_signal_logic_v3"
        }
      ]
    },
    {
      "concept_key": "excel_position_calculation",
      "coverage_tags": [
        "excel_ma_cross_steps",
        "excel_position_calculation"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_position_logic",
      "learning_goal": "学生能准确回忆在Excel中计算每日持仓的逻辑。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "在Excel中，根据信号和当前持仓状态计算新持仓的规则。",
      "term_refs": [
        {
          "display": "持仓",
          "en": "Position"
        }
      ],
      "variants": [
        {
          "back": "1",
          "estimated_seconds": 8,
          "explanation": "当空仓且出现买入信号时，持仓变为1。",
          "front": "在Excel中，如果当前持仓为0，且当日信号为1，那么新的持仓是多少？",
          "question_id": "q_flash_position_logic_v1"
        },
        {
          "back": "0",
          "estimated_seconds": 8,
          "explanation": "当持仓为1且出现卖出信号时，持仓变为0。",
          "front": "在Excel中，如果当前持仓为1，且当日信号为-1，那么新的持仓是多少？",
          "question_id": "q_flash_position_logic_v2"
        },
        {
          "back": "1",
          "estimated_seconds": 8,
          "explanation": "当持仓为1且无新信号时，持仓保持不变。",
          "front": "在Excel中，如果当前持仓为1，且当日信号为0，那么新的持仓是多少？",
          "question_id": "q_flash_position_logic_v3"
        }
      ]
    },
    {
      "concept_key": "excel_daily_pnl_calculation",
      "coverage_tags": [
        "excel_ma_cross_steps",
        "excel_daily_pnl_calculation",
        "excel_cumulative_pnl_calculation"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_pnl_calc",
      "learning_goal": "学生能准确回忆在Excel中计算每日盈亏和累计盈亏的公式。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "在Excel中，每日盈亏和累计盈亏的计算公式。",
      "term_refs": [
        {
          "display": "每日盈亏",
          "en": "Daily PnL"
        },
        {
          "display": "累计盈亏",
          "en": "Cumulative PnL"
        }
      ],
      "variants": [
        {
          "back": "持仓 * (今日收盘价 - 昨日收盘价)",
          "estimated_seconds": 8,
          "explanation": "每日盈亏等于持仓数量乘以当日价格变动。",
          "front": "在Excel中，计算每日盈亏的公式是什么？",
          "question_id": "q_flash_pnl_calc_v1"
        },
        {
          "back": "上一日累计盈亏 + 当日每日盈亏",
          "estimated_seconds": 8,
          "explanation": "累计盈亏是每日盈亏的累加和。",
          "front": "在Excel中，计算累计盈亏的公式是什么？",
          "question_id": "q_flash_pnl_calc_v2"
        }
      ]
    }
  ],
  "lesson_id": "L3",
  "longform_families": [
    {
      "concept_key": "excel_ma_cross_steps",
      "coverage_tags": [
        "excel_ma_cross_steps",
        "excel_cumulative_pnl_calculation"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_excel_workflow",
      "learning_goal": "学生能完整描述在Excel中实现MA交叉策略的五个步骤，并解释每一步的目的。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "mechanism_trace",
      "term_refs": [
        {
          "display": "移动平均线交叉策略",
          "en": "Moving Average Cross Strategy"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "步骤一：计算移动平均线",
            "步骤二：识别交叉信号",
            "步骤三：计算每日持仓",
            "步骤四：计算每日盈亏",
            "步骤五：计算累计盈亏"
          ],
          "question_id": "q_long_excel_workflow_v1",
          "reference_answer": [
            "步骤一：计算移动平均线。使用AVERAGE函数，分别计算过去7天和14天的收盘价平均值，得到MA(7)和MA(14)两列数据。",
            "步骤二：识别交叉信号。使用IF函数判断：如果MA(7)上穿MA(14)（即MA(7) > MA(14)且前一天MA(7) < MA(14)），则信号为1（买入）；如果MA(7)下穿MA(14)（即MA(7) < MA(14)且前一天MA(7) > MA(14)），则信号为-1（卖出）；否则为0。",
            "步骤三：计算每日持仓。根据前一天的持仓和当天的信号更新持仓：如果前一日持仓为0且信号为1，则持仓变为1；如果前一日持仓为1且信号为-1，则持仓变为0；否则持仓保持不变。",
            "步骤四：计算每日盈亏。每日盈亏 = 持仓 * (今日收盘价 - 昨日收盘价)。",
            "步骤五：计算累计盈亏。累计盈亏 = 上一日累计盈亏 + 当日每日盈亏。"
          ],
          "rubric_points": [
            "正确指出使用AVERAGE函数计算MA(7)和MA(14)。",
            "正确解释黄金交叉（买入信号1）和死亡交叉（卖出信号-1）的判断逻辑。",
            "正确解释根据信号和当前持仓状态更新持仓的逻辑。",
            "正确给出每日盈亏的计算公式：持仓 * (今日收盘价 - 昨日收盘价)。",
            "正确解释累计盈亏是每日盈亏的累加和。"
          ],
          "stem": "请详细描述在Excel中实现MA(7)和MA(14)交叉策略的完整步骤，并解释每一步的作用。"
        },
        {
          "estimated_seconds": 150,
          "prompt_blocks": [
            "步骤一：计算移动平均线",
            "步骤二：识别交叉信号",
            "步骤三：计算每日持仓",
            "步骤四：计算每日盈亏",
            "步骤五：计算累计盈亏"
          ],
          "question_id": "q_long_excel_workflow_v2",
          "reference_answer": [
            "步骤一：计算移动平均线。在Excel中，使用AVERAGE函数。例如，在MA(7)列的第一个有效单元格中输入 =AVERAGE(F2:F8)，其中F列为收盘价。然后向下拖动填充公式。同样地，计算MA(14)。",
            "步骤二：识别交叉信号。在信号列使用IF函数。例如，=IF(AND(H16>I16, H15<I15), 1, IF(AND(H16<I16, H15>I15), -1, 0))。其中H列为MA(7)，I列为MA(14)。",
            "步骤三：计算每日持仓。在持仓列，根据前一日持仓和当日信号更新。例如，=K2+IF(AND(K2=0, J3=-1), 0, J3)。其中K列为持仓，J列为信号。",
            "步骤四：计算每日盈亏。在每日盈亏列，使用公式 =K2*(F3-F2)。其中K列为持仓，F列为收盘价。",
            "步骤五：计算累计盈亏。在累计盈亏列，使用公式 =M2+L3。其中M列为累计盈亏，L列为每日盈亏。"
          ],
          "rubric_points": [
            "正确指出使用AVERAGE函数，并说明引用的数据范围。",
            "正确解释使用IF函数判断交叉的条件。",
            "正确解释持仓更新的逻辑。",
            "正确给出每日盈亏的计算公式。",
            "正确解释累计盈亏的计算方法。"
          ],
          "stem": "假设你已经在Excel中下载了汇丰控股（0005.HK）的历史数据，请写出实现MA(7)和MA(14)交叉策略回测的五个关键步骤，并说明每个步骤在Excel中如何操作。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "excel_ma_calc_function",
      "coverage_tags": [
        "excel_ma_calc_function"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_ma_function",
      "learning_goal": "学生能在测验情境下正确选择用于计算移动平均线的Excel函数。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "AVERAGE函数",
          "en": "AVERAGE function"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "AVERAGE函数用于计算一组数值的平均值，适合计算简单移动平均线。",
          "options": [
            "SUM",
            "AVERAGE",
            "MEDIAN",
            "COUNT"
          ],
          "question_id": "q_quiz_ma_function_v1",
          "stem": "在Excel中，要计算一组收盘价的7日简单移动平均线，应该使用哪个函数？"
        },
        {
          "answer": 3,
          "estimated_seconds": 15,
          "explanation": "移动平均线通常基于收盘价计算。",
          "options": [
            "开盘价（Open）",
            "最高价（High）",
            "最低价（Low）",
            "收盘价（Close）"
          ],
          "question_id": "q_quiz_ma_function_v2",
          "stem": "在Excel中，要计算MA(14)，AVERAGE函数应该引用哪一列数据？"
        }
      ]
    },
    {
      "concept_key": "excel_signal_identification",
      "coverage_tags": [
        "excel_ma_cross_steps",
        "excel_signal_identification"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_signal_identification",
      "learning_goal": "学生能在测验情境下正确识别黄金交叉和死亡交叉的条件。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "黄金交叉",
          "en": "Golden Cross"
        },
        {
          "display": "死亡交叉",
          "en": "Death Cross"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "MA(7)上穿MA(14)是黄金交叉，产生买入信号。",
          "options": [
            "MA(7) > MA(14) 且 MA(7)前一天 < MA(14)前一天",
            "MA(7) < MA(14) 且 MA(7)前一天 > MA(14)前一天",
            "MA(7) > MA(14) 且 MA(7)前一天 > MA(14)前一天",
            "MA(7) < MA(14) 且 MA(7)前一天 < MA(14)前一天"
          ],
          "question_id": "q_quiz_signal_identification_v1",
          "stem": "在Excel的MA交叉策略中，以下哪种情况会产生买入信号（标记为1）？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "MA(7)下穿MA(14)是死亡交叉，产生卖出信号。",
          "options": [
            "MA(7) > MA(14) 且 MA(7)前一天 < MA(14)前一天",
            "MA(7) < MA(14) 且 MA(7)前一天 > MA(14)前一天",
            "MA(7) > MA(14) 且 MA(7)前一天 > MA(14)前一天",
            "MA(7) < MA(14) 且 MA(7)前一天 < MA(14)前一天"
          ],
          "question_id": "q_quiz_signal_identification_v2",
          "stem": "在Excel的MA交叉策略中，以下哪种情况会产生卖出信号（标记为-1）？"
        }
      ]
    },
    {
      "concept_key": "excel_position_calculation",
      "coverage_tags": [
        "excel_ma_cross_steps",
        "excel_position_calculation"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_position_logic",
      "learning_goal": "学生能在测验情境下正确推导出在给定信号和持仓状态下的新持仓。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "持仓",
          "en": "Position"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "空仓时出现买入信号，持仓变为1。",
          "options": [
            "0",
            "1",
            "-1",
            "无法确定"
          ],
          "question_id": "q_quiz_position_logic_v1",
          "stem": "在Excel的MA交叉策略中，如果前一天的持仓为0，当天的信号为1，那么当天的持仓应该是多少？"
        },
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "持仓为1时出现卖出信号，持仓变为0。",
          "options": [
            "0",
            "1",
            "-1",
            "无法确定"
          ],
          "question_id": "q_quiz_position_logic_v2",
          "stem": "在Excel的MA交叉策略中，如果前一天的持仓为1，当天的信号为-1，那么当天的持仓应该是多少？"
        }
      ]
    },
    {
      "concept_key": "excel_daily_pnl_calculation",
      "coverage_tags": [
        "excel_ma_cross_steps",
        "excel_daily_pnl_calculation"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_pnl_calc",
      "learning_goal": "学生能在测验情境下正确计算每日盈亏。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "每日盈亏",
          "en": "Daily PnL"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "每日盈亏 = 持仓 * (今日收盘价 - 昨日收盘价) = 1 * (102 - 100) = 2。",
          "options": [
            "-2",
            "0",
            "2",
            "102"
          ],
          "question_id": "q_quiz_pnl_calc_v1",
          "stem": "在Excel的MA交叉策略中，如果当天的持仓为1，昨天的收盘价是100，今天的收盘价是102，那么当天的每日盈亏是多少？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "每日盈亏 = 持仓 * (今日收盘价 - 昨日收盘价) = 0 * (102 - 100) = 0。",
          "options": [
            "-2",
            "0",
            "2",
            "102"
          ],
          "question_id": "q_quiz_pnl_calc_v2",
          "stem": "在Excel的MA交叉策略中，如果当天的持仓为0，昨天的收盘价是100，今天的收盘价是102，那么当天的每日盈亏是多少？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L3: Building backtest framework and rule-based trading strategy - Implement an MA Cross Strategy on Excel",
    "guided_story_manifest": "pipeline/3-guided_story/L3/manifest.json",
    "lesson_map": "L3 step5: 用Excel实现MA交叉策略",
    "plain_text": "pipeline/1-plain/L3/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L3: Building backtest framework and rule-based trading strategy - Implement an MA Cross Strategy on Excel"
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "moving_average_definition",
      "covered_by": [
        "qf_flash_ma_def",
        "qf_quiz_ma_calc"
      ],
      "description": "移动平均线的定义、目的和计算方式"
    },
    {
      "coverage_tag": "simple_moving_average_formula",
      "covered_by": [
        "qf_flash_ma_def",
        "qf_quiz_ma_calc"
      ],
      "description": "简单移动平均线（SMA）的公式与计算"
    },
    {
      "coverage_tag": "ma_cross_strategy",
      "covered_by": [
        "qf_flash_golden_death",
        "qf_quiz_cross_signal",
        "qf_long_ma_strategy"
      ],
      "description": "MA交叉策略：快慢均线、黄金交叉与死亡交叉"
    },
    {
      "coverage_tag": "golden_cross",
      "covered_by": [
        "qf_flash_golden_death",
        "qf_quiz_cross_signal"
      ],
      "description": "黄金交叉的定义与交易信号"
    },
    {
      "coverage_tag": "death_cross",
      "covered_by": [
        "qf_flash_golden_death",
        "qf_quiz_cross_signal"
      ],
      "description": "死亡交叉的定义与交易信号"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "moving_average_definition",
      "coverage_tags": [
        "moving_average_definition",
        "simple_moving_average_formula"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_ma_def",
      "learning_goal": "学生能准确回忆移动平均线的定义、目的和简单移动平均线的计算公式。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "这组闪卡要求学生主动提取移动平均线的核心定义、目的和SMA公式。",
      "term_refs": [
        {
          "display": "移动平均线",
          "en": "Moving Average (MA)"
        },
        {
          "display": "简单移动平均线",
          "en": "Simple Moving Average (SMA)"
        }
      ],
      "variants": [
        {
          "back": "平滑价格数据，识别和显示长期趋势。",
          "estimated_seconds": 8,
          "explanation": "移动平均线通过计算一定时期内的平均价格，消除短期波动，帮助交易者看清价格的整体走向。",
          "front": "移动平均线（MA）的主要目的是什么？",
          "question_id": "q_flash_ma_def_v1"
        },
        {
          "back": "MA(n) = (P₁ + P₂ + ... + Pₙ) / n",
          "estimated_seconds": 10,
          "explanation": "SMA是对过去N个周期的收盘价直接取算术平均值。",
          "front": "写出N日简单移动平均线（SMA）的计算公式。",
          "question_id": "q_flash_ma_def_v2"
        },
        {
          "back": "14",
          "estimated_seconds": 12,
          "explanation": "SMA = (10 + 12 + 14 + 16 + 18) / 5 = 70 / 5 = 14。",
          "front": "如果过去5天的收盘价分别为10, 12, 14, 16, 18，5日SMA是多少？",
          "question_id": "q_flash_ma_def_v3"
        }
      ]
    },
    {
      "concept_key": "ma_cross_signals",
      "coverage_tags": [
        "ma_cross_strategy",
        "golden_cross",
        "death_cross"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_golden_death",
      "learning_goal": "学生能准确区分黄金交叉和死亡交叉的定义及其对应的交易信号。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "这组闪卡要求学生主动提取黄金交叉和死亡交叉的核心定义及交易方向。",
      "term_refs": [
        {
          "display": "黄金交叉",
          "en": "Golden Cross"
        },
        {
          "display": "死亡交叉",
          "en": "Death Cross"
        }
      ],
      "variants": [
        {
          "back": "短期移动平均线上穿长期移动平均线，发出买入信号。",
          "estimated_seconds": 8,
          "explanation": "黄金交叉被视为看涨信号，表明上升趋势可能开始。",
          "front": "什么是黄金交叉（Golden Cross）？它发出什么交易信号？",
          "question_id": "q_flash_golden_death_v1"
        },
        {
          "back": "短期移动平均线下穿长期移动平均线，发出卖出信号。",
          "estimated_seconds": 8,
          "explanation": "死亡交叉被视为看跌信号，表明下降趋势可能开始。",
          "front": "什么是死亡交叉（Death Cross）？它发出什么交易信号？",
          "question_id": "q_flash_golden_death_v2"
        },
        {
          "back": "一条快线（短期）和一条慢线（长期）。",
          "estimated_seconds": 6,
          "explanation": "例如，7日移动平均线作为快线，14日移动平均线作为慢线。",
          "front": "在MA交叉策略中，我们通常使用哪两条不同周期的均线？",
          "question_id": "q_flash_golden_death_v3"
        }
      ]
    }
  ],
  "lesson_id": "L3",
  "longform_families": [
    {
      "concept_key": "ma_cross_strategy_mechanism",
      "coverage_tags": [
        "ma_cross_strategy",
        "golden_cross",
        "death_cross"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_ma_strategy",
      "learning_goal": "学生能完整解释MA交叉策略的原理，包括快慢均线的选择、交叉信号的识别以及交易逻辑。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "mechanism_trace",
      "term_refs": [
        {
          "display": "移动平均线交叉策略",
          "en": "Moving Average Crossover Strategy"
        },
        {
          "display": "黄金交叉",
          "en": "Golden Cross"
        },
        {
          "display": "死亡交叉",
          "en": "Death Cross"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "为什么使用两条不同周期的移动平均线？",
            "什么是黄金交叉和死亡交叉？",
            "基于这两个交叉，策略的交易规则是什么？"
          ],
          "question_id": "q_long_ma_strategy_v1",
          "reference_answer": [
            "MA交叉策略使用两条不同周期的移动平均线：一条快线（如7日MA）和一条慢线（如14日MA）。快线对价格变化更敏感，能更快地反映趋势变化；慢线更平滑，代表长期趋势。",
            "当快线上穿慢线时，形成黄金交叉，这被视为一个看涨信号，表明上升趋势可能开始，策略会发出买入指令。",
            "当快线下穿慢线时，形成死亡交叉，这被视为一个看跌信号，表明下降趋势可能开始，策略会发出卖出指令。",
            "该策略的核心逻辑是利用均线的滞后性来捕捉趋势的转变点。"
          ],
          "rubric_points": [
            "正确解释快线（短期）和慢线（长期）的作用：快线对价格变化更敏感，慢线更平滑，用于识别趋势。",
            "准确定义黄金交叉（快线上穿慢线）和死亡交叉（快线下穿慢线）。",
            "正确说明交易规则：黄金交叉买入，死亡交叉卖出。"
          ],
          "stem": "请解释移动平均线（MA）交叉策略的基本原理。在你的解释中，需要包含以下要点："
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "你如何计算这两条移动平均线？",
            "在什么具体条件下，你会开仓买入？",
            "在什么具体条件下，你会开仓卖出？"
          ],
          "question_id": "q_long_ma_strategy_v2",
          "reference_answer": [
            "MA(5)是过去5个交易日收盘价的平均值，MA(20)是过去20个交易日收盘价的平均值。",
            "买入条件（黄金交叉）：当今天的MA(5)值大于MA(20)值，并且昨天的MA(5)值小于或等于MA(20)值时，表明快线刚刚上穿慢线，此时开仓买入。",
            "卖出条件（死亡交叉）：当今天的MA(5)值小于MA(20)值，并且昨天的MA(5)值大于或等于MA(20)值时，表明快线刚刚下穿慢线，此时开仓卖出。",
            "如果没有任何交叉发生，则保持当前持仓不变。"
          ],
          "rubric_points": [
            "正确描述SMA的计算方法：过去N天收盘价的平均值。",
            "准确描述买入条件：今天MA(5) > MA(20) 且 昨天MA(5) <= MA(20)。",
            "准确描述卖出条件：今天MA(5) < MA(20) 且 昨天MA(5) >= MA(20)。"
          ],
          "stem": "假设你正在使用MA(5)和MA(20)构建一个简单的MA交叉策略。请描述："
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "simple_moving_average_calculation",
      "coverage_tags": [
        "simple_moving_average_formula"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_ma_calc",
      "learning_goal": "学生能在给定价格序列的情况下，正确计算简单移动平均线的值。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "简单移动平均线",
          "en": "Simple Moving Average (SMA)"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "SMA = (20 + 22 + 24 + 26) / 4 = 92 / 4 = 23。",
          "options": [
            "22",
            "23",
            "24",
            "25"
          ],
          "question_id": "q_quiz_ma_calc_v1",
          "stem": "某股票过去4天的收盘价分别为：20, 22, 24, 26。请问这4天的简单移动平均线（SMA）是多少？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "SMA = (50 + 55 + 60) / 3 = 165 / 3 = 55。",
          "options": [
            "50",
            "55",
            "60",
            "65"
          ],
          "question_id": "q_quiz_ma_calc_v2",
          "stem": "某股票过去3天的收盘价分别为：50, 55, 60。请问这3天的简单移动平均线（SMA）是多少？"
        }
      ]
    },
    {
      "concept_key": "ma_cross_signal_identification",
      "coverage_tags": [
        "ma_cross_strategy",
        "golden_cross",
        "death_cross"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_cross_signal",
      "learning_goal": "学生能根据快慢均线的相对位置，正确识别黄金交叉和死亡交叉信号。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "黄金交叉",
          "en": "Golden Cross"
        },
        {
          "display": "死亡交叉",
          "en": "Death Cross"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "短期均线上穿长期均线是黄金交叉，通常被视为买入信号。",
          "options": [
            "买入",
            "卖出",
            "持有不动",
            "无法判断"
          ],
          "question_id": "q_quiz_cross_signal_v1",
          "stem": "某交易员使用MA(5)和MA(10)进行交易。今天，MA(5)从下方上穿MA(10)。根据MA交叉策略，他应该怎么做？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "短期均线下穿长期均线是死亡交叉，通常被视为卖出信号。",
          "options": [
            "买入",
            "卖出",
            "持有不动",
            "无法判断"
          ],
          "question_id": "q_quiz_cross_signal_v2",
          "stem": "某交易员使用MA(10)和MA(30)进行交易。今天，MA(10)从上方下穿MA(30)。根据MA交叉策略，他应该怎么做？"
        },
        {
          "answer": 0,
          "estimated_seconds": 25,
          "explanation": "买入信号（黄金交叉）发生在短期均线从下方上穿长期均线的那一刻，即今天MA(7) > MA(14)而昨天MA(7) < MA(14)。",
          "options": [
            "MA(7) > MA(14)，且前一天MA(7) < MA(14)",
            "MA(7) < MA(14)，且前一天MA(7) > MA(14)",
            "MA(7) 和 MA(14) 同时上升",
            "MA(7) 和 MA(14) 同时下降"
          ],
          "question_id": "q_quiz_cross_signal_v3",
          "stem": "以下哪种情况构成了MA交叉策略中的买入信号？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L3/plain.txt",
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
      "coverage_tag": "overfitting",
      "covered_by": [
        "qf_flash_overfitting",
        "qf_quiz_overfitting",
        "qf_long_overfitting"
      ],
      "description": "过度拟合：策略过于贴合历史数据，导致在新数据上表现不佳。"
    },
    {
      "coverage_tag": "look_ahead_bias",
      "covered_by": [
        "qf_flash_look_ahead_bias",
        "qf_quiz_look_ahead_bias",
        "qf_long_look_ahead_bias"
      ],
      "description": "前视偏差：在回测中使用了当时无法获得的未来信息。"
    },
    {
      "coverage_tag": "survivorship_bias",
      "covered_by": [
        "qf_flash_survivorship_bias",
        "qf_quiz_survivorship_bias",
        "qf_long_survivorship_bias"
      ],
      "description": "幸存者偏差：只考虑存活至今的资产，忽略了已经退市的，导致高估策略表现。"
    },
    {
      "coverage_tag": "backtest_assumptions",
      "covered_by": [
        "qf_flash_backtest_assumptions",
        "qf_quiz_backtest_assumptions",
        "qf_long_backtest_assumptions"
      ],
      "description": "回测假设：无交易成本、按收盘价成交、允许卖空等，在真实市场中往往不成立。"
    },
    {
      "coverage_tag": "best_practices",
      "covered_by": [
        "qf_flash_best_practices",
        "qf_quiz_best_practices",
        "qf_long_best_practices"
      ],
      "description": "最佳实践：用现实的假设，并在样本外数据上验证。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "overfitting",
      "coverage_tags": [
        "overfitting"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_overfitting",
      "learning_goal": "学生能准确回忆过度拟合的定义及其后果。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "过度拟合的定义和核心问题。",
      "term_refs": [
        {
          "display": "过度拟合",
          "en": "Overfitting"
        }
      ],
      "variants": [
        {
          "back": "策略被调整得过于贴合历史数据，导致在新市场或新数据上表现不佳。",
          "estimated_seconds": 8,
          "explanation": "过度拟合的策略在历史数据上表现优异，但缺乏泛化能力。",
          "front": "过度拟合（Overfitting）在回测中指的是什么？",
          "question_id": "q_flash_overfitting_v1"
        },
        {
          "back": "策略在样本外数据（新市场）上失效。",
          "estimated_seconds": 6,
          "explanation": "过度拟合导致策略无法适应新的市场环境。",
          "front": "过度拟合的一个主要后果是什么？",
          "question_id": "q_flash_overfitting_v2"
        }
      ]
    },
    {
      "concept_key": "look_ahead_bias",
      "coverage_tags": [
        "look_ahead_bias"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_look_ahead_bias",
      "learning_goal": "学生能准确回忆前视偏差的定义和典型例子。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "前视偏差的定义和常见例子。",
      "term_refs": [
        {
          "display": "前视偏差",
          "en": "Look-ahead Bias"
        }
      ],
      "variants": [
        {
          "back": "在回测中不小心使用了当时无法获得的未来信息。",
          "estimated_seconds": 8,
          "explanation": "例如，用今天的收盘价来决定昨天的交易。",
          "front": "前视偏差（Look-ahead Bias）在回测中指的是什么？",
          "question_id": "q_flash_look_ahead_bias_v1"
        },
        {
          "back": "用当天的收盘价来计算交易信号，并假设在当天开盘时执行。",
          "estimated_seconds": 8,
          "explanation": "这相当于提前知道了收盘价，在真实交易中是不可能的。",
          "front": "请举一个前视偏差的典型例子。",
          "question_id": "q_flash_look_ahead_bias_v2"
        }
      ]
    },
    {
      "concept_key": "survivorship_bias",
      "coverage_tags": [
        "survivorship_bias"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_survivorship_bias",
      "learning_goal": "学生能准确回忆幸存者偏差的定义及其对回测结果的影响。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "幸存者偏差的定义和影响。",
      "term_refs": [
        {
          "display": "幸存者偏差",
          "en": "Survivorship Bias"
        }
      ],
      "variants": [
        {
          "back": "只测试了现在还活着的股票，忽略了那些已经退市的失败者。",
          "estimated_seconds": 8,
          "explanation": "这会导致回测结果被高估，因为退市的股票通常表现不佳。",
          "front": "幸存者偏差（Survivorship Bias）在回测中指的是什么？",
          "question_id": "q_flash_survivorship_bias_v1"
        },
        {
          "back": "高估。",
          "estimated_seconds": 5,
          "explanation": "因为忽略了表现差的退市股票，只保留了表现好的。",
          "front": "幸存者偏差会导致回测结果被高估还是低估？",
          "question_id": "q_flash_survivorship_bias_v2"
        }
      ]
    },
    {
      "concept_key": "backtest_assumptions",
      "coverage_tags": [
        "backtest_assumptions"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_backtest_assumptions",
      "learning_goal": "学生能列举出常见的回测假设，并理解其与现实市场的差异。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "常见的回测假设及其不现实之处。",
      "term_refs": [
        {
          "display": "回测假设",
          "en": "Backtest Assumptions"
        }
      ],
      "variants": [
        {
          "back": "1. 没有交易成本；2. 可以按收盘价成交；3. 允许卖空。",
          "estimated_seconds": 10,
          "explanation": "真实市场中存在佣金、滑点、流动性限制和卖空限制。",
          "front": "回测中常见的三个不现实的假设是什么？",
          "question_id": "q_flash_backtest_assumptions_v1"
        },
        {
          "back": "因为存在滑点，即实际成交价与预期价格之间的差异。",
          "estimated_seconds": 8,
          "explanation": "滑点可能由市场波动、流动性不足或大额订单引起。",
          "front": "在真实市场中，为什么“按收盘价成交”这个假设通常不成立？",
          "question_id": "q_flash_backtest_assumptions_v2"
        }
      ]
    },
    {
      "concept_key": "best_practices",
      "coverage_tags": [
        "best_practices"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_best_practices",
      "learning_goal": "学生能回忆出提高回测可靠性的关键最佳实践。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "提高回测可靠性的两个关键最佳实践。",
      "term_refs": [
        {
          "display": "最佳实践",
          "en": "Best Practices"
        }
      ],
      "variants": [
        {
          "back": "1. 使用现实的假设（如考虑交易成本）；2. 在样本外数据上验证。",
          "estimated_seconds": 10,
          "explanation": "这有助于避免过度拟合和确保策略的泛化能力。",
          "front": "提高回测可靠性的两个关键最佳实践是什么？",
          "question_id": "q_flash_best_practices_v1"
        },
        {
          "back": "使用在策略开发过程中未使用过的数据来测试策略的表现。",
          "estimated_seconds": 8,
          "explanation": "这是检验策略是否过度拟合的有效方法。",
          "front": "什么是“样本外验证”？",
          "question_id": "q_flash_best_practices_v2"
        }
      ]
    }
  ],
  "lesson_id": "L3",
  "longform_families": [
    {
      "concept_key": "overfitting",
      "coverage_tags": [
        "overfitting"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_overfitting",
      "learning_goal": "学生能用自己的语言解释过度拟合，并说明其危害和避免方法。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "过度拟合",
          "en": "Overfitting"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "定义过度拟合",
            "解释其危害",
            "提出至少两种避免方法"
          ],
          "question_id": "q_long_overfitting_v1",
          "reference_answer": [
            "过度拟合是指交易策略被调整得过于贴合历史数据，以至于它学习到了数据中的随机噪音而非真实的市场规律。",
            "其危害在于，策略在历史回测中表现优异，但一旦应用于新的、未见过的市场数据，其表现会急剧下降，导致实盘交易亏损。",
            "避免过度拟合的方法包括：1）使用样本外数据（Out-of-Sample Data）进行验证，即用策略开发过程中未使用的数据来测试；2）保持策略的简洁性，避免使用过多参数；3）使用交叉验证（Cross-Validation）等技术。"
          ],
          "rubric_points": [
            "正确解释过度拟合是策略过于贴合历史数据中的噪音",
            "指出其危害是策略在新数据上表现不佳，缺乏泛化能力",
            "提出至少两种方法，如：使用样本外验证、简化模型、交叉验证等"
          ],
          "stem": "请解释什么是回测中的“过度拟合”，它为什么有害，以及如何避免？"
        },
        {
          "estimated_seconds": 150,
          "prompt_blocks": [
            "指出可能存在的问题",
            "解释为什么会有这些问题",
            "提出具体的改进建议"
          ],
          "question_id": "q_long_overfitting_v2",
          "reference_answer": [
            "这个策略很可能存在严重的过度拟合问题。10个参数提供了极大的灵活性，使得策略可以完美地匹配过去5年数据中的每一个波动，但这些波动很多是随机的。",
            "问题在于，这些参数组合可能只是恰好捕捉到了历史数据中的噪音，而不是真正的市场规律。因此，策略在未来数据上几乎肯定会失效。",
            "改进建议：1）大幅减少参数数量，只保留最核心的1-2个；2）将数据分为训练集和测试集，只在训练集上优化参数，然后在测试集上验证；3）进行样本外测试，比如用过去5年数据开发，用未来1年数据验证。"
          ],
          "rubric_points": [
            "指出过度拟合的可能性很高",
            "解释参数过多容易拟合噪音",
            "提出简化模型、样本外验证、增加数据量等建议"
          ],
          "stem": "假设你开发了一个有10个参数的复杂策略，它在过去5年的数据上表现完美。请分析这个策略可能存在的问题，并给出改进建议。"
        }
      ]
    },
    {
      "concept_key": "look_ahead_bias",
      "coverage_tags": [
        "look_ahead_bias"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_look_ahead_bias",
      "learning_goal": "学生能识别并解释前视偏差，并能提出避免方法。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "前视偏差",
          "en": "Look-ahead Bias"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "定义前视偏差",
            "给出一个包含前视偏差的伪代码或逻辑示例",
            "说明如何修正这个错误"
          ],
          "question_id": "q_long_look_ahead_bias_v1",
          "reference_answer": [
            "前视偏差是指在回测过程中，无意中使用了在决策时点尚未知晓的未来数据。",
            "例如，一个错误的逻辑是：`if close_today > MA_20_today: buy_at_open_today`。这里，`close_today` 是当日收盘价，但在开盘时是无法知道的。",
            "修正方法：将信号延迟一天，使用前一日的数据来做决策。正确的逻辑是：`if close_yesterday > MA_20_yesterday: buy_at_open_today`。这样，所有用于决策的数据在开盘时都是已知的。"
          ],
          "rubric_points": [
            "正确解释前视偏差是使用了未来信息",
            "给出的示例能清晰展示问题，例如用当日收盘价计算开盘信号",
            "修正方法正确，例如使用前一日收盘价或延迟信号"
          ],
          "stem": "请解释什么是“前视偏差”，并给出一个具体的回测代码示例来说明它如何产生，以及如何修正。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "列举其他形式的前视偏差",
            "解释每种形式如何产生"
          ],
          "question_id": "q_long_look_ahead_bias_v2",
          "reference_answer": [
            "1）使用未来发布的财务数据：例如，在回测中使用公司下一季度财报的净利润来计算估值指标，但财报数据在回测时间点尚未公布。",
            "2）使用未来的指数成分股调整：例如，在回测中假设某股票当前就是标普500成分股，并使用其历史数据，但该股票可能是在回测时间点之后才被纳入指数的。",
            "3）使用未来的拆分或分红数据：在计算复权价格时，如果使用了未来的拆分因子，也会引入前视偏差。"
          ],
          "rubric_points": [
            "能列举出除价格外的其他形式，如使用未来财务数据、未来指数成分股调整等",
            "解释清晰，说明为什么这些信息在当时不可得"
          ],
          "stem": "除了使用未来价格数据，前视偏差还可能以哪些形式出现？请列举至少两种并解释。"
        }
      ]
    },
    {
      "concept_key": "survivorship_bias",
      "coverage_tags": [
        "survivorship_bias"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_survivorship_bias",
      "learning_goal": "学生能解释幸存者偏差如何影响回测，并说明如何构建一个无偏的数据集。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "幸存者偏差",
          "en": "Survivorship Bias"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "解释幸存者偏差的定义",
            "说明它如何导致回测结果被高估",
            "提出避免方法"
          ],
          "question_id": "q_long_survivorship_bias_v1",
          "reference_answer": [
            "幸存者偏差是指在回测中只使用当前仍然存在的资产（如股票）的历史数据，而忽略了那些因为破产、被收购等原因已经退市的资产。",
            "由于退市的资产通常表现不佳，忽略它们会使回测结果看起来比实际情况好得多，给人一种“随便买一只股票都能赚钱”的错觉。",
            "要避免幸存者偏差，必须使用一个“无幸存者偏差”的数据集，该数据集包含了在回测期间所有曾经存在过的资产的历史数据，无论它们当前是否还在交易。"
          ],
          "rubric_points": [
            "正确解释幸存者偏差是只考虑存活资产",
            "说明忽略退市资产会高估收益，因为退市资产通常表现差",
            "提出使用包含所有历史资产（包括已退市）的数据集"
          ],
          "stem": "请解释“幸存者偏差”如何导致回测结果失真，并说明在构建回测数据集时应如何避免它。"
        },
        {
          "estimated_seconds": 150,
          "prompt_blocks": [
            "指出会有什么偏差",
            "解释偏差产生的原因",
            "描述一个更准确的回测方法"
          ],
          "question_id": "q_long_survivorship_bias_v2",
          "reference_answer": [
            "回测结果会被严重高估，这主要是由幸存者偏差导致的。",
            "纳斯达克历史上有很多公司最终退市，尤其是在2000年互联网泡沫破裂时期。这些公司的股价跌至接近零，给投资者带来了巨大损失。如果只测试当前成分股，就相当于自动排除了这些失败案例，使得“买入并持有”策略的历史表现看起来异常优秀。",
            "一个更准确的回测方法是使用纳斯达克综合指数本身的历史数据，或者获取一个包含所有历史上在纳斯达克上市公司的完整数据集，并动态调整持仓，模拟真实的指数构成变化。"
          ],
          "rubric_points": [
            "指出结果会被高估（幸存者偏差）",
            "解释因为退市的公司（如互联网泡沫时期的公司）表现极差，忽略它们会拉高平均收益",
            "提出应使用包含所有曾上市公司的动态指数数据或历史成分股列表"
          ],
          "stem": "假设你想回测一个“买入并持有所有纳斯达克股票”的策略。如果你只使用当前纳斯达克成分股的历史数据，你的回测结果会有什么偏差？请详细解释。"
        }
      ]
    },
    {
      "concept_key": "backtest_assumptions",
      "coverage_tags": [
        "backtest_assumptions"
      ],
      "difficulty": "hard",
      "family_id": "qf_long_backtest_assumptions",
      "learning_goal": "学生能批判性地评估回测假设，并解释其现实影响。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "回测假设",
          "en": "Backtest Assumptions"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 150,
          "prompt_blocks": [
            "分析“无交易成本”假设的影响",
            "分析“按收盘价成交”假设的影响",
            "提出调整回测的具体方法"
          ],
          "question_id": "q_long_backtest_assumptions_v1",
          "reference_answer": [
            "“无交易成本”假设会显著高估策略的净收益。在真实市场中，每次交易都涉及佣金、印花税、买卖价差等成本。对于交易频繁的策略，这些成本会累积成一笔巨大的开支，甚至可能将盈利策略变为亏损。",
            "“按收盘价成交”假设忽略了滑点。滑点是指预期成交价与实际成交价之间的差异，通常由市场波动、流动性不足或大额订单引起。在真实市场中，尤其是在市场收盘前，价格波动可能很大，无法保证以收盘价成交。",
            "为了使回测更现实，可以：1）在每日盈亏计算中扣除一个固定的交易成本（如每笔交易0.25点）；2）使用开盘价或下一根K线的开盘价来代替收盘价作为成交价；3）引入滑点模型，根据市场波动率和订单规模动态计算滑点。"
          ],
          "rubric_points": [
            "指出无交易成本会高估收益，尤其是对高频策略",
            "指出按收盘价成交忽略了滑点和市场冲击",
            "提出加入佣金、滑点模型、使用开盘价或限价单等方法"
          ],
          "stem": "一个回测策略假设“无交易成本”和“可以按收盘价成交”。请分析这两个假设在真实市场中会如何影响策略的实际表现，并说明如何调整回测以使其更现实。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "列举两个其他常见假设",
            "解释每个假设如何影响结果"
          ],
          "question_id": "q_long_backtest_assumptions_v2",
          "reference_answer": [
            "1）允许卖空（Short Selling）：很多回测假设可以无限制地卖空任何股票。但在真实市场中，卖空受到严格监管，很多股票难以借到，且需要支付融券费用。忽略这些限制会高估策略的潜在收益。",
            "2）无限流动性（Infinite Liquidity）：回测通常假设可以按当前价格买卖任意数量的股票。但在真实市场中，大额订单会推动价格向不利方向移动，产生巨大的市场冲击成本。忽略这一点会高估策略的容量和收益。"
          ],
          "rubric_points": [
            "能列举出如“允许卖空”、“无限流动性”、“无市场冲击”等假设",
            "解释清晰，例如允许卖空可能高估收益，因为很多股票难以做空"
          ],
          "stem": "除了交易成本和成交价，回测中还有哪些常见的、可能不现实的假设？请列举两个并解释它们如何影响回测结果。"
        }
      ]
    },
    {
      "concept_key": "best_practices",
      "coverage_tags": [
        "best_practices"
      ],
      "difficulty": "hard",
      "family_id": "qf_long_best_practices",
      "learning_goal": "学生能综合运用最佳实践来设计一个更可靠的回测流程。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "最佳实践",
          "en": "Best Practices"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 180,
          "prompt_blocks": [
            "描述数据准备步骤",
            "描述策略开发步骤",
            "描述验证和评估步骤"
          ],
          "question_id": "q_long_best_practices_v1",
          "reference_answer": [
            "1. 数据准备：获取包含所有历史资产（包括已退市）的完整数据集。进行严格的数据清洗，处理缺失值、重复记录和逻辑错误。将数据分为训练集（如70%）、验证集（15%）和测试集（15%）。",
            "2. 策略开发：仅在训练集上开发策略。从简单的规则开始，逐步增加复杂度，但始终保持警惕，避免过度拟合。使用验证集来微调参数。",
            "3. 验证与评估：在从未使用过的测试集上进行最终评估。在回测中加入现实的假设，如交易成本、滑点和流动性限制。计算多个绩效指标（如夏普比率、最大回撤），并进行敏感性分析，测试策略在不同市场条件下的表现。"
          ],
          "rubric_points": [
            "数据准备：使用无幸存者偏差的数据，进行彻底的数据清洗",
            "策略开发：在训练集上开发，保持模型简洁",
            "验证评估：使用样本外测试、考虑现实假设、进行敏感性分析"
          ],
          "stem": "请描述一个你认为最可靠的回测流程，包括从数据准备到结果评估的各个步骤，并解释每一步如何帮助你避免常见的回测陷阱。"
        },
        {
          "estimated_seconds": 150,
          "prompt_blocks": [
            "提出关于数据的问题",
            "提出关于假设的问题",
            "提出关于验证的问题"
          ],
          "question_id": "q_long_best_practices_v2",
          "reference_answer": [
            "1. “你的回测数据是否包含了所有股票，包括那些已经退市的？”（检验幸存者偏差）",
            "2. “你在回测中假设的交易成本是多少？是否考虑了滑点和市场冲击？”（检验假设的现实性）",
            "3. “这个策略是在整个数据集上开发的，还是使用了样本外数据来验证？”（检验过度拟合）",
            "4. “这个策略的参数是固定的，还是经过大量优化后选出的最佳组合？”（检验数据窥探偏差）"
          ],
          "rubric_points": [
            "问题应针对数据（如是否有幸存者偏差）",
            "问题应针对假设（如是否考虑了交易成本）",
            "问题应针对验证（如是否做过样本外测试）"
          ],
          "stem": "你的同事向你展示了一个回测结果，声称他的新策略年化收益高达80%，夏普比率3.0。在相信这个结果之前，你会向他提出哪些关键问题来检验其可靠性？请列出至少三个问题并解释为什么这些问题重要。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "overfitting",
      "coverage_tags": [
        "overfitting"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_overfitting",
      "learning_goal": "学生能在具体情境中识别过度拟合的迹象。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "过度拟合",
          "en": "Overfitting"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 25,
          "explanation": "历史表现异常优异但实盘表现糟糕，是过度拟合的典型症状。策略可能被过度优化以匹配历史数据中的噪音。",
          "options": [
            "市场发生了不可预见的黑天鹅事件",
            "策略存在过度拟合，过于贴合历史数据",
            "交易平台出现了技术故障",
            "策略的交易频率过高"
          ],
          "question_id": "q_quiz_overfitting_v1",
          "stem": "一个交易策略在历史回测中取得了年化50%的惊人回报，但在随后一年的实盘交易中却亏损了20%。以下哪个原因最可能解释这一现象？"
        },
        {
          "answer": 1,
          "estimated_seconds": 25,
          "explanation": "反复调整复杂参数以追求历史最优表现，是过度拟合的常见原因。这会使策略学习到历史数据中的噪音而非真实规律。",
          "options": [
            "使用10年的历史数据进行测试",
            "在策略中加入多个复杂的参数，并反复调整直到历史表现最优",
            "在回测中考虑0.1%的交易成本",
            "使用样本外数据验证策略"
          ],
          "question_id": "q_quiz_overfitting_v2",
          "stem": "以下哪种做法最有可能导致回测中的过度拟合？"
        }
      ]
    },
    {
      "concept_key": "look_ahead_bias",
      "coverage_tags": [
        "look_ahead_bias"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_look_ahead_bias",
      "learning_goal": "学生能识别出包含前视偏差的回测代码或逻辑。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "前视偏差",
          "en": "Look-ahead Bias"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 25,
          "explanation": "在当日交易信号计算中使用当日最高价或最低价，意味着你提前知道了当日价格范围，这在真实交易中是不可能的。",
          "options": [
            "使用前一天的收盘价计算移动平均线",
            "在计算当日交易信号时，使用了当日的最高价和最低价",
            "在计算当日交易信号时，仅使用前一天的收盘价",
            "在计算当日交易信号时，使用了前一天的成交量"
          ],
          "question_id": "q_quiz_look_ahead_bias_v1",
          "stem": "在编写回测代码时，以下哪种做法会引入前视偏差？"
        },
        {
          "answer": 1,
          "estimated_seconds": 25,
          "explanation": "在开盘时，当日的收盘价是未知的。使用当日收盘价来决定开盘时的交易，相当于使用了未来信息。",
          "options": [
            "没有问题，这是一个标准的趋势跟踪策略",
            "存在前视偏差，因为开盘时无法知道当日的收盘价",
            "存在幸存者偏差",
            "存在过度拟合"
          ],
          "question_id": "q_quiz_look_ahead_bias_v2",
          "stem": "一个回测策略的规则是：如果当日收盘价高于20日移动平均线，则在当日开盘时买入。这个规则存在什么问题？"
        }
      ]
    },
    {
      "concept_key": "survivorship_bias",
      "coverage_tags": [
        "survivorship_bias"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_survivorship_bias",
      "learning_goal": "学生能理解幸存者偏差如何导致回测结果失真。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "幸存者偏差",
          "en": "Survivorship Bias"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "这引入了幸存者偏差。退市的公司通常表现不佳，忽略它们会使回测结果看起来比实际情况更好。",
          "options": [
            "回测结果会被低估",
            "回测结果会被高估",
            "不会产生任何偏差",
            "会导致前视偏差"
          ],
          "question_id": "q_quiz_survivorship_bias_v1",
          "stem": "如果一个回测只测试了当前标普500指数中的成分股，而忽略了历史上曾经存在但现已退市的公司，这会导致什么问题？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "包含已退市股票的数据集能更真实地反映市场的全貌，避免幸存者偏差。",
          "options": [
            "当前所有上市公司的股票数据",
            "过去20年所有曾经交易过的股票数据，包括已退市的",
            "标普500指数当前成分股的历史数据",
            "过去10年表现最好的100只股票数据"
          ],
          "question_id": "q_quiz_survivorship_bias_v2",
          "stem": "以下哪个数据集最有可能避免幸存者偏差？"
        }
      ]
    },
    {
      "concept_key": "backtest_assumptions",
      "coverage_tags": [
        "backtest_assumptions"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_backtest_assumptions",
      "learning_goal": "学生能识别不现实的回测假设及其影响。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "回测假设",
          "en": "Backtest Assumptions"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "忽略交易成本（佣金、滑点等）会高估策略的净收益，尤其是在高频交易策略中。",
          "options": [
            "使回测结果更保守",
            "使回测结果更乐观，可能高估策略收益",
            "对回测结果没有影响",
            "会导致前视偏差"
          ],
          "question_id": "q_quiz_backtest_assumptions_v1",
          "stem": "一个回测假设“没有交易成本”，这个假设会导致什么后果？"
        },
        {
          "answer": 1,
          "estimated_seconds": 25,
          "explanation": "对于流动性较差的股票或大额订单，市场冲击成本和滑点会非常显著，无法按收盘价成交。",
          "options": [
            "收盘价可能不准确",
            "存在流动性限制，大额订单可能无法以理想价格成交",
            "交易所有可能拒绝订单",
            "没有太大问题，可以近似模拟"
          ],
          "question_id": "q_quiz_backtest_assumptions_v2",
          "stem": "在回测中假设“可以按收盘价无限制地买卖任意数量股票”，这个假设在真实市场中最大的问题是什么？"
        }
      ]
    },
    {
      "concept_key": "best_practices",
      "coverage_tags": [
        "best_practices"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_best_practices",
      "learning_goal": "学生能选择正确的回测最佳实践。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "最佳实践",
          "en": "Best Practices"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "样本外验证是检验策略是否过度拟合、是否具有泛化能力的关键步骤。",
          "options": [
            "使用尽可能多的历史数据，直到策略表现最优",
            "在策略开发完成后，用一段全新的、未使用过的数据来验证",
            "忽略交易成本，因为回测主要是看趋势",
            "只测试当前表现最好的股票"
          ],
          "question_id": "q_quiz_best_practices_v1",
          "stem": "以下哪项是提高回测结果可靠性的最佳实践？"
        },
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "使用现实的假设，如交易成本、滑点和流动性限制，是获得可靠回测结果的基础。",
          "options": [
            "假设没有交易成本，简化计算",
            "假设可以无限卖空，增加策略灵活性",
            "根据真实市场数据，估算并加入合理的交易成本和滑点",
            "假设所有订单都能以收盘价成交"
          ],
          "question_id": "q_quiz_best_practices_v2",
          "stem": "为了确保回测假设更贴近现实，你应该怎么做？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/L3/step6/step.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L3/plain.txt",
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
      "coverage_tag": "python_backtest_tools",
      "covered_by": [
        "qf_flash_talib",
        "qf_flash_backtesting_py",
        "qf_quiz_talib_usage",
        "qf_quiz_backtesting_py_usage",
        "qf_long_backtesting_py_workflow"
      ],
      "description": "Python回测工具：TA-Lib和Backtesting.py的用途与基本用法"
    },
    {
      "coverage_tag": "talib_overview",
      "covered_by": [
        "qf_flash_talib",
        "qf_quiz_talib_usage"
      ],
      "description": "TA-Lib：120+技术指标库，一行代码计算指标"
    },
    {
      "coverage_tag": "backtesting_py_framework",
      "covered_by": [
        "qf_flash_backtesting_py",
        "qf_quiz_backtesting_py_usage",
        "qf_long_backtesting_py_workflow"
      ],
      "description": "Backtesting.py：轻量级回测框架，init/next方法，自动绩效与图表"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "talib_overview",
      "coverage_tags": [
        "python_backtest_tools",
        "talib_overview"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_talib",
      "learning_goal": "学生能回忆TA-Lib的核心功能：它是一个包含120多种技术指标的开源Python库。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "TA-Lib是什么，以及它的主要特点。",
      "term_refs": [
        {
          "display": "TA-Lib",
          "en": "TA-Lib (Technical Analysis Library)"
        }
      ],
      "variants": [
        {
          "back": "TA-Lib是一个开源的技术分析库，包含120多种技术指标。",
          "estimated_seconds": 8,
          "explanation": "TA-Lib（Technical Analysis Library）提供了如移动平均线、RSI、布林带等常用指标的一行代码计算。",
          "front": "TA-Lib是什么类型的Python库？它大约包含多少种技术指标？",
          "question_id": "q_flash_talib_v1"
        },
        {
          "back": "`real = MA(close, timeperiod=30)`",
          "estimated_seconds": 10,
          "explanation": "TA-Lib的MA函数用于计算移动平均线，timeperiod参数指定周期。",
          "front": "在TA-Lib中，计算30日简单移动平均线的函数调用是什么？",
          "question_id": "q_flash_talib_v2"
        },
        {
          "back": "RSI（相对强弱指标）和BBANDS（布林带）。",
          "estimated_seconds": 10,
          "explanation": "TA-Lib包含RSI、布林带、MACD、随机指标等大量技术指标。",
          "front": "除了移动平均线，请列举两个TA-Lib可以计算的技术指标。",
          "question_id": "q_flash_talib_v3"
        }
      ]
    },
    {
      "concept_key": "backtesting_py_framework",
      "coverage_tags": [
        "python_backtest_tools",
        "backtesting_py_framework"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_backtesting_py",
      "learning_goal": "学生能回忆Backtesting.py的核心概念：它是一个轻量级回测框架，通过定义策略类中的init和next方法来实现回测。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "Backtesting.py是什么，以及它的两个核心方法。",
      "term_refs": [
        {
          "display": "Backtesting.py",
          "en": "Backtesting.py"
        }
      ],
      "variants": [
        {
          "back": "一个轻量级的回测框架，用于快速测试交易策略。",
          "estimated_seconds": 8,
          "explanation": "Backtesting.py可以自动计算绩效指标并生成图表，代码简洁。",
          "front": "Backtesting.py是一个什么类型的Python库？",
          "question_id": "q_flash_backtesting_py_v1"
        },
        {
          "back": "`init`方法。",
          "estimated_seconds": 8,
          "explanation": "`init`方法在回测开始时被调用一次，用于设置指标。",
          "front": "在Backtesting.py的策略类中，应该在哪个方法里初始化技术指标？",
          "question_id": "q_flash_backtesting_py_v2"
        },
        {
          "back": "`next`方法。",
          "estimated_seconds": 8,
          "explanation": "`next`方法在每个新的K线数据到来时被调用，是编写交易逻辑的地方。",
          "front": "在Backtesting.py的策略类中，应该在哪个方法里编写买卖条件？",
          "question_id": "q_flash_backtesting_py_v3"
        }
      ]
    }
  ],
  "lesson_id": "L3",
  "longform_families": [
    {
      "concept_key": "backtesting_py_framework",
      "coverage_tags": [
        "python_backtest_tools",
        "backtesting_py_framework"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_backtesting_py_workflow",
      "learning_goal": "学生能解释使用Backtesting.py进行回测的完整工作流程，包括策略定义、数据准备和运行回测。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "mechanism_trace",
      "term_refs": [
        {
          "display": "Backtesting.py",
          "en": "Backtesting.py"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "策略定义",
            "数据准备",
            "运行回测"
          ],
          "question_id": "q_long_backtesting_py_workflow_v1",
          "reference_answer": [
            "1. 定义策略类：创建一个继承自`backtesting.Strategy`的类，例如`SmaCross`。",
            "2. 初始化指标：在类的`init`方法中，通过`self.data.Close`获取收盘价数据，并使用`self.I(SMA, price, 10)`和`self.I(SMA, price, 20)`分别计算10日和20日简单移动平均线。",
            "3. 编写交易逻辑：在`next`方法中，使用`crossover(self.ma1, self.ma2)`判断是否发生黄金交叉（快线上穿慢线），如果是则调用`self.buy()`；使用`crossover(self.ma2, self.ma1)`判断是否发生死亡交叉，如果是则调用`self.sell()`。",
            "4. 准备数据：获取历史OHLCV数据，确保列名顺序为`Date, Open, High, Low, Close, Volume`。",
            "5. 运行回测：创建`Backtest`对象，传入数据、策略类、初始资金和佣金等参数，然后调用`bt.run()`执行回测，最后可以通过`bt.plot()`查看图表。"
          ],
          "rubric_points": [
            "正确指出需要定义一个继承自`Strategy`的策略类",
            "正确指出在`init`方法中初始化移动平均线指标",
            "正确指出在`next`方法中使用`crossover`函数判断交叉信号并调用`self.buy()`或`self.sell()`",
            "正确指出需要准备OHLCV格式的历史数据",
            "正确指出创建`Backtest`对象并传入数据和策略，调用`run()`方法执行回测"
          ],
          "stem": "请描述使用Backtesting.py库对一个简单的移动平均线交叉策略进行回测的完整步骤。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "导入库",
            "定义策略类",
            "在init中计算RSI",
            "在next中编写交易逻辑",
            "运行回测"
          ],
          "question_id": "q_long_backtesting_py_workflow_v2",
          "reference_answer": [
            "```python",
            "from backtesting import Backtest, Strategy",
            "import talib",
            "",
            "class RsiStrategy(Strategy):",
            "    def init(self):",
            "        price = self.data.Close",
            "        self.rsi = self.I(talib.RSI, price, 14)",
            "",
            "    def next(self):",
            "        if self.rsi[-1] < 30 and not self.position:",
            "            self.buy()",
            "        elif self.rsi[-1] > 70 and self.position:",
            "            self.position.close()",
            "",
            "bt = Backtest(df, RsiStrategy, cash=10000, commission=0.002)",
            "stats = bt.run()",
            "print(stats)",
            "bt.plot()",
            "```"
          ],
          "rubric_points": [
            "正确导入`Backtest`, `Strategy`以及`talib`",
            "正确在`init`方法中使用`talib.RSI`计算RSI指标",
            "正确在`next`方法中判断RSI值并调用`self.buy()`或`self.sell()`",
            "正确创建`Backtest`对象并运行"
          ],
          "stem": "假设你有一个包含`Date, Open, High, Low, Close, Volume`列的DataFrame `df`，请写出使用Backtesting.py实现一个基于TA-Lib计算的RSI策略（RSI<30买入，RSI>70卖出）的核心代码框架。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "talib_overview",
      "coverage_tags": [
        "python_backtest_tools",
        "talib_overview"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_talib_usage",
      "learning_goal": "学生能在测验中识别TA-Lib的功能和用法。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "TA-Lib",
          "en": "TA-Lib"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "TA-Lib是一个开源的技术分析库，包含120多种技术指标，如MA、RSI、布林带等。",
          "options": [
            "TA-Lib是一个用于数据可视化的Python库",
            "TA-Lib内置了120多种技术指标的计算函数",
            "TA-Lib只能用于计算移动平均线",
            "TA-Lib是Backtesting.py的一个子模块"
          ],
          "question_id": "q_quiz_talib_usage_v1",
          "stem": "以下哪个关于TA-Lib的描述是正确的？"
        },
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "TA-Lib中计算RSI的函数是`RSI(close, timeperiod=14)`。",
          "options": [
            "`RSI(close, timeperiod=14)`",
            "`MA(close, timeperiod=14)`",
            "`BBANDS(close, timeperiod=14)`",
            "`SMA(close, timeperiod=14)`"
          ],
          "question_id": "q_quiz_talib_usage_v2",
          "stem": "在Python中使用TA-Lib计算14日RSI，正确的函数调用是？"
        }
      ]
    },
    {
      "concept_key": "backtesting_py_framework",
      "coverage_tags": [
        "python_backtest_tools",
        "backtesting_py_framework"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_backtesting_py_usage",
      "learning_goal": "学生能在测验中识别Backtesting.py的使用方法和核心概念。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "Backtesting.py",
          "en": "Backtesting.py"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "`next`方法会在每个新的K线数据到来时被调用，是编写买卖条件的地方。",
          "options": [
            "`init`",
            "`start`",
            "`next`",
            "`run`"
          ],
          "question_id": "q_quiz_backtesting_py_usage_v1",
          "stem": "在Backtesting.py中，以下哪个方法用于在每个新的K线数据到来时执行交易逻辑？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "Backtesting.py要求用户定义一个策略类，在`init`中初始化指标，在`next`中编写交易逻辑，然后传入`Backtest`对象运行。",
          "options": [
            "手动计算每日盈亏",
            "定义策略类并实现`init`和`next`方法",
            "使用TA-Lib计算所有指标",
            "将数据保存为Excel文件"
          ],
          "question_id": "q_quiz_backtesting_py_usage_v2",
          "stem": "使用Backtesting.py进行回测时，以下哪个步骤是必需的？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L3: Building backtest framework and rule-based trading strategy",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "L3 guided_story steps",
    "plain_text": "pipeline/1-plain/L3/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L3: Building backtest framework and rule-based trading strategy"
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "rsi_definition",
      "covered_by": [
        "qf_flash_rsi_def",
        "qf_quiz_rsi_meaning"
      ],
      "description": "RSI（相对强弱指标）的定义：一个动量指标，范围0-100，衡量价格变动的速度和幅度。"
    },
    {
      "coverage_tag": "rsi_overbought_oversold",
      "covered_by": [
        "qf_flash_rsi_thresholds",
        "qf_quiz_rsi_action"
      ],
      "description": "RSI超买（>70）和超卖（<30）的概念及其市场含义。"
    },
    {
      "coverage_tag": "rsi_trading_logic",
      "covered_by": [
        "qf_flash_rsi_rule",
        "qf_quiz_rsi_rule_apply",
        "qf_long_rsi_strategy_explain"
      ],
      "description": "RSI策略的交易规则：RSI<30买入，RSI>70卖出。"
    },
    {
      "coverage_tag": "rsi_strategy_implementation",
      "covered_by": [
        "qf_long_rsi_strategy_explain"
      ],
      "description": "在Excel、Backtesting.py或ALGOGENE上实现RSI策略的练习。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "rsi_definition",
      "coverage_tags": [
        "rsi_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_rsi_def",
      "learning_goal": "学生能准确回忆RSI的定义和基本属性。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "RSI是什么类型的指标，它的取值范围是多少。",
      "term_refs": [
        {
          "display": "相对强弱指标",
          "en": "Relative Strength Index (RSI)"
        }
      ],
      "variants": [
        {
          "back": "动量指标；0到100。",
          "estimated_seconds": 8,
          "explanation": "RSI衡量价格变动的速度和幅度，值在0到100之间。",
          "front": "RSI（相对强弱指标）是一种什么类型的指标？它的取值范围是多少？",
          "question_id": "q_flash_rsi_def_v1"
        },
        {
          "back": "Relative Strength Index（相对强弱指标）；衡量价格变动的速度和幅度。",
          "estimated_seconds": 8,
          "explanation": "RSI是一个动量振荡器，用于识别超买或超卖状态。",
          "front": "RSI的全称是什么？它主要用于衡量什么？",
          "question_id": "q_flash_rsi_def_v2"
        }
      ]
    },
    {
      "concept_key": "rsi_overbought_oversold",
      "coverage_tags": [
        "rsi_overbought_oversold"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_rsi_thresholds",
      "learning_goal": "学生能准确回忆RSI的超买和超卖阈值。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "RSI值超过多少被视为超买？低于多少被视为超卖？",
      "term_refs": [
        {
          "display": "超买",
          "en": "Overbought"
        },
        {
          "display": "超卖",
          "en": "Oversold"
        }
      ],
      "variants": [
        {
          "back": "70。",
          "estimated_seconds": 6,
          "explanation": "RSI > 70 通常表示市场处于超买状态，可能面临回调。",
          "front": "在RSI策略中，RSI值超过多少被视为超买状态？",
          "question_id": "q_flash_rsi_thresholds_v1"
        },
        {
          "back": "30。",
          "estimated_seconds": 6,
          "explanation": "RSI < 30 通常表示市场处于超卖状态，可能迎来反弹。",
          "front": "在RSI策略中，RSI值低于多少被视为超卖状态？",
          "question_id": "q_flash_rsi_thresholds_v2"
        }
      ]
    },
    {
      "concept_key": "rsi_trading_logic",
      "coverage_tags": [
        "rsi_trading_logic"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_rsi_rule",
      "learning_goal": "学生能准确回忆基于RSI的简单交易规则。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "RSI策略中买入和卖出的具体条件。",
      "term_refs": [
        {
          "display": "RSI策略",
          "en": "RSI Strategy"
        }
      ],
      "variants": [
        {
          "back": "买入。",
          "estimated_seconds": 6,
          "explanation": "RSI低于30被视为超卖，是潜在的买入信号。",
          "front": "根据RSI策略，当RSI(14)低于30时，应该执行什么操作？",
          "question_id": "q_flash_rsi_rule_v1"
        },
        {
          "back": "卖出。",
          "estimated_seconds": 6,
          "explanation": "RSI高于70被视为超买，是潜在的卖出信号。",
          "front": "根据RSI策略，当RSI(14)高于70时，应该执行什么操作？",
          "question_id": "q_flash_rsi_rule_v2"
        }
      ]
    }
  ],
  "lesson_id": "L3",
  "longform_families": [
    {
      "concept_key": "rsi_trading_logic",
      "coverage_tags": [
        "rsi_trading_logic",
        "rsi_strategy_implementation"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_rsi_strategy_explain",
      "learning_goal": "学生能解释RSI策略的逻辑、交易规则，并描述在Excel或Python中实现的基本步骤。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "相对强弱指标",
          "en": "Relative Strength Index (RSI)"
        },
        {
          "display": "RSI策略",
          "en": "RSI Strategy"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "RSI的定义和取值范围",
            "超买和超卖的含义",
            "具体的买入和卖出条件",
            "在Excel中实现的主要步骤（如数据准备、计算RSI、生成信号、计算盈亏）"
          ],
          "question_id": "q_long_rsi_strategy_explain_v1",
          "reference_answer": [
            "RSI（相对强弱指标）是一个动量指标，取值范围在0到100之间。",
            "通常，RSI超过70表示市场处于超买状态，价格可能回调；RSI低于30表示市场处于超卖状态，价格可能反弹。",
            "基于此的简单RSI策略规则是：当RSI(14) < 30时买入，当RSI(14) > 70时卖出。",
            "在Excel中实现该策略，大致需要以下步骤：1. 获取历史价格数据；2. 使用公式或插件计算RSI(14)值；3. 根据RSI值生成交易信号（买入/卖出/持有）；4. 根据信号计算每日持仓；5. 计算每日盈亏和累计盈亏。"
          ],
          "rubric_points": [
            "正确解释RSI是动量指标，范围0-100。",
            "正确说明RSI>70为超买，RSI<30为超卖。",
            "准确描述交易规则：RSI<30买入，RSI>70卖出。",
            "合理描述在Excel中实现的步骤，如获取价格数据、使用公式计算RSI、根据阈值生成交易信号、计算持仓和盈亏。"
          ],
          "stem": "请解释RSI（相对强弱指标）策略的基本原理和交易规则。并简要说明在Excel中实现该策略需要哪些关键步骤。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "RSI策略的交易规则",
            "Backtesting.py中`init`方法的作用",
            "Backtesting.py中`next`方法的作用",
            "如何生成交易信号并执行买卖"
          ],
          "question_id": "q_long_rsi_strategy_explain_v2",
          "reference_answer": [
            "RSI策略的核心逻辑是：当RSI(14)低于30时，认为市场超卖，发出买入信号；当RSI(14)高于70时，认为市场超买，发出卖出信号。",
            "在Backtesting.py中，我们需要定义一个继承自`Strategy`的策略类。",
            "在`init`方法中，我们使用TA-Lib或自定义函数计算RSI指标，并将其作为策略的一部分。",
            "在`next`方法中，我们编写具体的交易逻辑：检查当前RSI值，如果满足买入条件且无持仓，则调用`self.buy()`；如果满足卖出条件且有持仓，则调用`self.sell()`。"
          ],
          "rubric_points": [
            "准确描述RSI策略规则：RSI<30买入，RSI>70卖出。",
            "正确指出`init`方法用于初始化指标（如计算RSI）。",
            "正确指出`next`方法在每个新K线到来时被调用，用于编写交易逻辑。",
            "合理说明如何在`next`方法中检查RSI值并调用`self.buy()`或`self.sell()`。"
          ],
          "stem": "假设你要用Python的Backtesting.py库来测试一个RSI策略。请描述该策略的核心逻辑，并说明在Backtesting.py中你需要定义哪些关键方法以及它们的作用。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "rsi_definition",
      "coverage_tags": [
        "rsi_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_rsi_meaning",
      "learning_goal": "学生能在测验中正确识别RSI指标的类型和用途。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "相对强弱指标",
          "en": "Relative Strength Index (RSI)"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "RSI是一个动量振荡器，用于衡量价格变动的速度和幅度，范围在0到100之间。",
          "options": [
            "一种基于成交量的趋势跟踪指标",
            "一种衡量价格变动速度和幅度的动量指标",
            "一种预测未来价格走势的回归模型",
            "一种计算移动平均线的统计方法"
          ],
          "question_id": "q_quiz_rsi_meaning_v1",
          "stem": "以下哪个选项最准确地描述了RSI（相对强弱指标）？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "RSI通常用于识别市场是否处于超买（RSI>70）或超卖（RSI<30）状态，从而辅助交易决策。",
          "options": [
            "计算交易成本",
            "识别市场的超买或超卖状态",
            "预测公司的盈利情况",
            "衡量市场波动率"
          ],
          "question_id": "q_quiz_rsi_meaning_v2",
          "stem": "RSI指标的主要用途是什么？"
        }
      ]
    },
    {
      "concept_key": "rsi_overbought_oversold",
      "coverage_tags": [
        "rsi_overbought_oversold"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_rsi_action",
      "learning_goal": "学生能根据RSI值判断市场状态并选择正确的交易动作。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "超买",
          "en": "Overbought"
        },
        {
          "display": "超卖",
          "en": "Oversold"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "RSI低于30被视为超卖，是潜在的买入信号。",
          "options": [
            "买入",
            "卖出",
            "持有不动",
            "无法判断"
          ],
          "question_id": "q_quiz_rsi_action_v1",
          "stem": "假设你正在回测一个RSI策略，当前RSI(14)的值为25。根据标准RSI策略，你应该怎么做？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "RSI高于70被视为超买，是潜在的卖出信号。",
          "options": [
            "买入",
            "卖出",
            "持有不动",
            "无法判断"
          ],
          "question_id": "q_quiz_rsi_action_v2",
          "stem": "假设你正在回测一个RSI策略，当前RSI(14)的值为85。根据标准RSI策略，你应该怎么做？"
        }
      ]
    },
    {
      "concept_key": "rsi_trading_logic",
      "coverage_tags": [
        "rsi_trading_logic"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_rsi_rule_apply",
      "learning_goal": "学生能在具体数值场景中应用RSI策略规则。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "RSI策略",
          "en": "RSI Strategy"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 25,
          "explanation": "RSI(14)=28 < 30，满足买入条件，且当前无持仓，应执行买入。",
          "options": [
            "立即买入",
            "立即卖出",
            "等待RSI回到50再操作",
            "什么都不做"
          ],
          "question_id": "q_quiz_rsi_rule_apply_v1",
          "stem": "一个简单的RSI策略规定：当RSI(14) < 30时买入，当RSI(14) > 70时卖出。如果当前没有持仓，且RSI(14) = 28，你应该："
        },
        {
          "answer": 1,
          "estimated_seconds": 25,
          "explanation": "RSI(14)=75 > 70，满足卖出条件，应平仓卖出。",
          "options": [
            "加仓买入",
            "平仓卖出",
            "继续持有",
            "无法判断"
          ],
          "question_id": "q_quiz_rsi_rule_apply_v2",
          "stem": "一个简单的RSI策略规定：当RSI(14) < 30时买入，当RSI(14) > 70时卖出。如果你当前持有多头仓位，且RSI(14) = 75，你应该："
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L3: Building backtest framework and rule-based trading strategy - Exercise: RSI Strategy",
    "guided_story_manifest": "pipeline/3-guided_story/L3/manifest.json",
    "lesson_map": "L3 step8: RSI策略练习",
    "plain_text": "pipeline/1-plain/L3/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L3: Building backtest framework and rule-based trading strategy - Exercise: RSI Strategy"
  },
  "target_language": "zh-CN"
}

]
</QUESTION_BANK>

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

请直接输出 MDX，不要加解释。
