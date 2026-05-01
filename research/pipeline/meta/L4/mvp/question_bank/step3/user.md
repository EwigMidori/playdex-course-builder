请基于以下 lesson 材料，生成一个结构化题库 JSON。

目标语言：
zh-CN

lesson_id：
L4

要求：
- 同时生成 `flashcard_families`、`quiz_families` 和 `longform_families`
- 题目必须只关联到当前 step：`step3`
- 所有 family 和 variant 的 `linked_steps` 都必须等于 `["step3"]`
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
# L4: Statistical time series analysis for market classification
Course Code: COMP7415
# Agenda
- Common classification for financial market
- Market prediction with statistical models
- Simple Linear Regression
- Multiple Linear Regression
ARIMA
- Hurst Exponent
- Martingale Strategy
# Market Classification
The financial market can generally be classified into
1. Trending (Momentum)
2. Mean Reversion
3. Random Walk
# Trending (Momentum)
- Momentum trading involves
- buying securities that are trending upwards and selling them when they appear to have peaked, or
- shorting securities that are trending downwards and covering them when they appear to have bottomed out.
![](images/09778c2b01cda28bb5b71d28e6d1e2b93355193e35bfb0b0d56d9a47a92629d4.jpg)
# Mean Reversion
- Mean reversion suggests that asset prices and historical returns eventually revert to their long-term mean or average level.
- Based on the idea that extreme price movements are temporary and will revert to the mean.
![](images/9c30145270dc07b2f7ff723423144bade20fb1a716cbf144b29fa02a9777accd.jpg)
![](images/617b59e8209c742aca5c4e4cc6bb0330d7ac0dc2c81306ba24c7f3696a914c01.jpg)
![](images/f95be69cdc7c9db8c708fd963500e1b8ea92a63e227516a3406fedbaf1683ccf.jpg)
# Trending and Mean Reversion can occur at the same time
![](images/f60784d53f2cbaa56f4be7f24b9d773bc6ce6fccd06fac8de18ebbf656f11f28.jpg)
# Random Walk
- It implies that price changes are random and do not follow any patterns or trends.
- Efficient Market Hypothesis (EMH): Underpins the idea that all known information is already reflected in stock prices, making it impossible to consistently achieve higher returns than average market returns.
- Independence: Each price change is independent of previous price changes.
- Unpredictability: Future price movements cannot be predicted based on past price movements.
- If the market is random, can you think of a trading strategy that can be profitable?
# Time series Analysis
# Introduction to Time Series Analysis
- Time series analysis involves examining data points collected or recorded at specific time intervals.
- It aims to identify patterns, trends, and other characteristics in the data.
Examples:
Stock prices over time
Monthly unemployment rates
Daily temperature readings
Quarterly GDP growth rates
# Simple Linear Regression
For market trend identification
# Simple Linear Regression Model
- Linear regression is a statistical method for modeling the linear relationship between a dependent variable and independent variable.
Key Concepts:
- Dependent Variable (Y): The variable we are trying to predict or explain.
- Independent Variable (X): The variable we use to make predictions.
- Linear Equation:
$$
Y = \beta_ {0} + \beta_ {1} X + \epsilon
$$
$\beta_0$ : Intercept
$\beta_{1}$ : Slope
$\epsilon$ : Error term
# Why Use Linear Regression?
- Simplicity: Easy to understand and implement.
- Interpretability: Coefficients provide insights into the relationship between variables.
</COVERAGE_CHECKLIST>

<SOURCE_OUTLINE>
# L4: Statistical time series analysis for market classification
Course Code: COMP7415
# Agenda
- Common classification for financial market
- Market prediction with statistical models
- Simple Linear Regression
- Multiple Linear Regression
ARIMA
- Hurst Exponent
- Martingale Strategy
# Market Classification
The financial market can generally be classified into
1. Trending (Momentum)
2. Mean Reversion
3. Random Walk
# Trending (Momentum)
- Momentum trading involves
- buying securities that are trending upwards and selling them when they appear to have peaked, or
- shorting securities that are trending downwards and covering them when they appear to have bottomed out.
![](images/09778c2b01cda28bb5b71d28e6d1e2b93355193e35bfb0b0d56d9a47a92629d4.jpg)
# Mean Reversion
- Mean reversion suggests that asset prices and historical returns eventually revert to their long-term mean or average level.
- Based on the idea that extreme price movements are temporary and will revert to the mean.
![](images/9c30145270dc07b2f7ff723423144bade20fb1a716cbf144b29fa02a9777accd.jpg)
![](images/617b59e8209c742aca5c4e4cc6bb0330d7ac0dc2c81306ba24c7f3696a914c01.jpg)
![](images/f95be69cdc7c9db8c708fd963500e1b8ea92a63e227516a3406fedbaf1683ccf.jpg)
# Trending and Mean Reversion can occur at the same time
![](images/f60784d53f2cbaa56f4be7f24b9d773bc6ce6fccd06fac8de18ebbf656f11f28.jpg)
# Random Walk
- It implies that price changes are random and do not follow any patterns or trends.
- Efficient Market Hypothesis (EMH): Underpins the idea that all known information is already reflected in stock prices, making it impossible to consistently achieve higher returns than average market returns.
- Independence: Each price change is independent of previous price changes.
- Unpredictability: Future price movements cannot be predicted based on past price movements.
- If the market is random, can you think of a trading strategy that can be profitable?
# Time series Analysis
# Introduction to Time Series Analysis
- Time series analysis involves examining data points collected or recorded at specific time intervals.
- It aims to identify patterns, trends, and other characteristics in the data.
Examples:
Stock prices over time
Monthly unemployment rates
Daily temperature readings
Quarterly GDP growth rates
# Simple Linear Regression
For market trend identification
# Simple Linear Regression Model
- Linear regression is a statistical method for modeling the linear relationship between a dependent variable and independent variable.
Key Concepts:
- Dependent Variable (Y): The variable we are trying to predict or explain.
- Independent Variable (X): The variable we use to make predictions.
- Linear Equation:
$$
Y = \beta_ {0} + \beta_ {1} X + \epsilon
$$
$\beta_0$ : Intercept
$\beta_{1}$ : Slope
$\epsilon$ : Error term
# Why Use Linear Regression?
- Simplicity: Easy to understand and implement.
- Interpretability: Coefficients provide insights into the relationship between variables.
</SOURCE_OUTLINE>

<LESSON_MAP>
{
  "lesson_id": "L4",
  "mode": "guided_story",
  "steps": [
    {
      "concept": "三种市场状态：趋势、均值回归、随机游走",
      "file": "research/pipeline/3-guided_story/L4/step1/step.json",
      "sequence_id": "step1"
    },
    {
      "concept": "用简单线性回归识别趋势",
      "file": "research/pipeline/3-guided_story/L4/step2/step.json",
      "sequence_id": "step2"
    },
    {
      "concept": "用Python实现线性回归交易策略",
      "file": "research/pipeline/3-guided_story/L4/step3/step.json",
      "sequence_id": "step3"
    },
    {
      "concept": "多元线性回归：用多个因素预测价格",
      "file": "research/pipeline/3-guided_story/L4/step4/step.json",
      "sequence_id": "step4"
    },
    {
      "concept": "ARIMA模型：处理时间序列的依赖关系",
      "file": "research/pipeline/3-guided_story/L4/step5/step.json",
      "sequence_id": "step5"
    },
    {
      "concept": "Hurst指数：量化市场状态",
      "file": "research/pipeline/3-guided_story/L4/step6/step.json",
      "sequence_id": "step6"
    },
    {
      "concept": "Martingale策略：在随机市场中赚钱",
      "file": "research/pipeline/3-guided_story/L4/step7/step.json",
      "sequence_id": "step7"
    },
    {
      "concept": "总结：选择正确的模型匹配市场状态",
      "file": "research/pipeline/3-guided_story/L4/step8/step.json",
      "sequence_id": "step8"
    }
  ]
}

</LESSON_MAP>

<GUIDED_STORY_MANIFEST>
{
  "lesson_id": "L4",
  "mode": "guided_story",
  "steps": [
    {
      "concept": "三种市场状态：趋势、均值回归、随机游走",
      "file": "research/pipeline/3-guided_story/L4/step1/step.json",
      "sequence_id": "step1"
    },
    {
      "concept": "用简单线性回归识别趋势",
      "file": "research/pipeline/3-guided_story/L4/step2/step.json",
      "sequence_id": "step2"
    },
    {
      "concept": "用Python实现线性回归交易策略",
      "file": "research/pipeline/3-guided_story/L4/step3/step.json",
      "sequence_id": "step3"
    },
    {
      "concept": "多元线性回归：用多个因素预测价格",
      "file": "research/pipeline/3-guided_story/L4/step4/step.json",
      "sequence_id": "step4"
    },
    {
      "concept": "ARIMA模型：处理时间序列的依赖关系",
      "file": "research/pipeline/3-guided_story/L4/step5/step.json",
      "sequence_id": "step5"
    },
    {
      "concept": "Hurst指数：量化市场状态",
      "file": "research/pipeline/3-guided_story/L4/step6/step.json",
      "sequence_id": "step6"
    },
    {
      "concept": "Martingale策略：在随机市场中赚钱",
      "file": "research/pipeline/3-guided_story/L4/step7/step.json",
      "sequence_id": "step7"
    },
    {
      "concept": "总结：选择正确的模型匹配市场状态",
      "file": "research/pipeline/3-guided_story/L4/step8/step.json",
      "sequence_id": "step8"
    }
  ]
}

</GUIDED_STORY_MANIFEST>

<GUIDED_STORY_STEPS>
[
  {
    "lesson_id": "L4",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [],
        "id": "s001",
        "introduced_terms": [],
        "lines": [
          "金融市场并非只有一种面孔。",
          "它至少会呈现出三种截然不同的状态。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "trending_market"
        ],
        "id": "s002",
        "introduced_terms": [
          "trending_market"
        ],
        "lines": [
          "第一种：<term id=\"trending_market\">趋势市场</term>。",
          "价格像有惯性一样，持续朝一个方向移动。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "mean_reversion"
        ],
        "id": "s003",
        "introduced_terms": [
          "mean_reversion"
        ],
        "lines": [
          "第二种：<term id=\"mean_reversion\">均值回归</term>。",
          "价格涨得过高会跌回来，跌得过深会弹回去。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "random_walk"
        ],
        "id": "s004",
        "introduced_terms": [
          "random_walk"
        ],
        "lines": [
          "第三种：<term id=\"random_walk\">随机游走</term>。",
          "价格变化完全随机，没有规律可循。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "价格回到之前的水平，正是均值回归的典型表现。",
          "kind": "single_choice",
          "options": [
            "趋势市场",
            "均值回归",
            "随机游走"
          ],
          "prompt": "价格今天涨了，明天又跌回原价，这最接近哪种市场？"
        },
        "focus_terms": [
          "mean_reversion"
        ],
        "id": "s005",
        "introduced_terms": [],
        "lines": [
          "如果价格今天涨了，明天又跌回原价，这最接近哪种市场？"
        ],
        "type": "exercise"
      },
      {
        "focus_terms": [
          "trending_market",
          "mean_reversion"
        ],
        "id": "s006",
        "introduced_terms": [],
        "lines": [
          "有趣的是，趋势和均值回归可以同时存在。",
          "短时间看是趋势，长时间看可能是在回归。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "random_walk"
        ],
        "id": "s007",
        "introduced_terms": [],
        "lines": [
          "如果市场是随机游走，任何技术分析都无效。",
          "但有一种策略反而能赚钱——我们后面会讲到。"
        ],
        "type": "narration"
      }
    ],
    "sequence_id": "step1",
    "source": {
      "plain_text": "Market Classification: Trending, Mean Reversion, Random Walk",
      "related": []
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "mean_reversion": {
        "aliases": [
          "Mean Reversion"
        ],
        "display": "均值回归",
        "gloss": "价格在极端波动后倾向于回到长期平均水平的现象。"
      },
      "random_walk": {
        "aliases": [
          "Random Walk"
        ],
        "display": "随机游走",
        "gloss": "价格变化不可预测，未来走势与过去无关。"
      },
      "trending_market": {
        "aliases": [
          "Trending Market",
          "Momentum"
        ],
        "display": "趋势市场",
        "gloss": "价格朝一个方向持续移动的市场状态。"
      }
    }
  },
  {
    "lesson_id": "L4",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [],
        "id": "s008",
        "introduced_terms": [],
        "lines": [
          "如何判断市场是趋势还是随机？",
          "一个简单的方法：用直线去拟合价格。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "simple_linear_regression"
        ],
        "id": "s009",
        "introduced_terms": [
          "simple_linear_regression"
        ],
        "lines": [
          "这就是<term id=\"simple_linear_regression\">简单线性回归</term>。",
          "它用一条直线来描述价格和时间的关系。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "simple_linear_regression"
        ],
        "formula": {
          "latex": "Y = \\beta_0 + \\beta_1 X + \\epsilon",
          "spoken": "Y等于截距加上斜率乘以X，再加上误差项。"
        },
        "id": "s010",
        "introduced_terms": [],
        "lines": [
          "公式很简单：",
          "Y = β₀ + β₁X + ε"
        ],
        "type": "formula"
      },
      {
        "focus_terms": [
          "simple_linear_regression"
        ],
        "id": "s011",
        "introduced_terms": [],
        "lines": [
          "Y 是价格，X 是时间。",
          "β₁ 是斜率——如果为正，价格在上涨；为负则在下跌。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "residual"
        ],
        "id": "s012",
        "introduced_terms": [
          "residual"
        ],
        "lines": [
          "模型会给出一个预测值。",
          "实际价格与预测值的差，叫做<term id=\"residual\">残差</term>。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "residual"
        ],
        "id": "s013",
        "introduced_terms": [],
        "lines": [
          "残差越小，说明直线拟合得越好。",
          "但怎么量化“拟合得好不好”呢？"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "r_squared"
        ],
        "id": "s014",
        "introduced_terms": [
          "r_squared"
        ],
        "lines": [
          "用<term id=\"r_squared\">R²</term>。",
          "它告诉你模型能解释多少价格变动。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "r_squared"
        ],
        "id": "s015",
        "introduced_terms": [],
        "lines": [
          "R² = 1 表示完美拟合，价格完全沿着直线走。",
          "R² = 0 表示直线毫无解释力。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "R²越接近1，模型拟合越好，趋势越明显。",
          "kind": "single_choice",
          "options": [
            "模型完全没用",
            "模型解释了96%的价格变动",
            "价格是随机游走"
          ],
          "prompt": "如果R² = 0.96，说明什么？"
        },
        "focus_terms": [
          "r_squared"
        ],
        "id": "s016",
        "introduced_terms": [],
        "lines": [
          "如果R² = 0.96，说明什么？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step2",
    "source": {
      "plain_text": "Simple Linear Regression for market trend identification",
      "related": []
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "r_squared": {
        "aliases": [
          "R-squared",
          "Coefficient of Determination"
        ],
        "display": "R²",
        "gloss": "衡量模型解释了多少数据变动的比例，范围0到1。"
      },
      "residual": {
        "aliases": [
          "Residual"
        ],
        "display": "残差",
        "gloss": "实际值与模型预测值之间的差异。"
      },
      "simple_linear_regression": {
        "aliases": [
          "Simple Linear Regression",
          "SLR"
        ],
        "display": "简单线性回归",
        "gloss": "用一个自变量预测一个因变量的线性模型。"
      }
    }
  },
  {
    "lesson_id": "L4",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [],
        "id": "s017",
        "introduced_terms": [],
        "lines": [
          "理论讲完了，来实战。",
          "我们用 Python 对微软股票跑一个线性回归。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s018",
        "introduced_terms": [],
        "lines": [
          "先获取数据：用 yfinance 下载 MSFT 2023年的日线数据。",
          "然后构造特征：用“一年中的第几天”作为 X。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s019",
        "introduced_terms": [],
        "lines": [
          "用 sklearn 的 LinearRegression 拟合模型。",
          "得到截距和斜率。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "r_squared"
        ],
        "id": "s020",
        "introduced_terms": [],
        "lines": [
          "然后评估：R² = 0.46。",
          "说明直线只能解释不到一半的价格变动。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "linear_regression_strategy"
        ],
        "id": "s021",
        "introduced_terms": [
          "linear_regression_strategy"
        ],
        "lines": [
          "但这不妨碍我们用它来交易。",
          "策略很简单：如果预测未来价格高于当前，就买入。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "linear_regression_strategy"
        ],
        "id": "s022",
        "introduced_terms": [],
        "lines": [
          "回测结果显示：年化收益约3.6%，胜率58.8%。",
          "不算惊艳，但证明了线性模型确实能捕捉到一些趋势。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "线性回归假设价格与时间存在线性关系，即趋势。",
          "kind": "single_choice",
          "options": [
            "价格会随机波动",
            "价格存在可预测的线性趋势",
            "价格永远回归均值"
          ],
          "prompt": "线性回归策略的核心假设是什么？"
        },
        "focus_terms": [
          "linear_regression_strategy"
        ],
        "id": "s023",
        "introduced_terms": [],
        "lines": [
          "线性回归策略的核心假设是什么？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step3",
    "source": {
      "plain_text": "Python Example: fit LR model, backtest LR strategy",
      "related": []
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "linear_regression_strategy": {
        "aliases": [
          "LR Trading Strategy"
        ],
        "display": "线性回归交易策略",
        "gloss": "利用线性回归模型预测价格，当实际价格偏离预测值时进行交易。"
      }
    }
  },
  {
    "lesson_id": "L4",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [],
        "id": "s024",
        "introduced_terms": [],
        "lines": [
          "只用时间做预测太简单了。",
          "如果能加入更多信息，预测会更准。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "multiple_linear_regression"
        ],
        "id": "s025",
        "introduced_terms": [
          "multiple_linear_regression"
        ],
        "lines": [
          "这就是<term id=\"multiple_linear_regression\">多元线性回归</term>。",
          "Y = β₀ + β₁X₁ + β₂X₂ + ... + βₚXₚ + ε"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "multiple_linear_regression"
        ],
        "id": "s026",
        "introduced_terms": [],
        "lines": [
          "比如预测电力公司的股价，",
          "可以考虑GDP、利率、煤价、气温等因素。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "multicollinearity"
        ],
        "id": "s027",
        "introduced_terms": [
          "multicollinearity"
        ],
        "lines": [
          "但要注意：不能什么因素都往里塞。",
          "如果两个因素高度相关，就会出现<term id=\"multicollinearity\">多重共线性</term>。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "multicollinearity"
        ],
        "id": "s028",
        "introduced_terms": [],
        "lines": [
          "多重共线性会让模型系数变得不可靠。",
          "比如GDP和利率如果高度相关，你很难分清谁在起作用。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "因素要有逻辑关联，同时要避免多重共线性。",
          "kind": "single_choice",
          "options": [
            "因素越多越好",
            "确保因素之间有逻辑关联，且避免高度相关",
            "只选一个因素"
          ],
          "prompt": "以下哪个是选择因素时的正确做法？"
        },
        "focus_terms": [
          "multicollinearity"
        ],
        "id": "s029",
        "introduced_terms": [],
        "lines": [
          "以下哪个是选择因素时的正确做法？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step4",
    "source": {
      "plain_text": "Multiple Linear Regression Model",
      "related": []
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "multicollinearity": {
        "aliases": [
          "Multicollinearity"
        ],
        "display": "多重共线性",
        "gloss": "自变量之间高度相关，导致模型估计不稳定。"
      },
      "multiple_linear_regression": {
        "aliases": [
          "Multiple Linear Regression",
          "MLR"
        ],
        "display": "多元线性回归",
        "gloss": "用多个自变量预测一个因变量的线性模型。"
      }
    }
  },
  {
    "lesson_id": "L4",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [],
        "id": "s030",
        "introduced_terms": [],
        "lines": [
          "线性回归假设数据独立。",
          "但金融数据往往前后依赖——今天的价格和昨天有关。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "arima"
        ],
        "id": "s031",
        "introduced_terms": [
          "arima"
        ],
        "lines": [
          "ARIMA模型就是专门处理这种依赖关系的。",
          "AR = 自回归，用过去的值预测现在。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "arima",
          "stationarity"
        ],
        "id": "s032",
        "introduced_terms": [
          "stationarity"
        ],
        "lines": [
          "I = 差分，让数据变得<term id=\"stationarity\">平稳</term>。",
          "MA = 移动平均，用过去的预测误差来修正。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "arima"
        ],
        "id": "s033",
        "introduced_terms": [],
        "lines": [
          "ARIMA(p,d,q) 有三个参数：",
          "p是自回归阶数，d是差分次数，q是移动平均阶数。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "adf_test"
        ],
        "id": "s034",
        "introduced_terms": [
          "adf_test"
        ],
        "lines": [
          "怎么判断数据是否平稳？",
          "用<term id=\"adf_test\">ADF检验</term>。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "adf_test",
          "stationarity"
        ],
        "id": "s035",
        "introduced_terms": [],
        "lines": [
          "ADF检验的p值小于0.05，说明数据平稳。",
          "否则就需要差分，直到平稳为止。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "I代表Integrated，即通过差分使序列平稳。",
          "kind": "single_choice",
          "options": [
            "自回归",
            "差分",
            "移动平均"
          ],
          "prompt": "ARIMA中的“I”代表什么？"
        },
        "focus_terms": [
          "arima"
        ],
        "id": "s036",
        "introduced_terms": [],
        "lines": [
          "ARIMA中的“I”代表什么？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step5",
    "source": {
      "plain_text": "ARIMA Model, ADF Test, Stationarity",
      "related": []
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "adf_test": {
        "aliases": [
          "Augmented Dickey-Fuller Test"
        ],
        "display": "ADF检验",
        "gloss": "检验时间序列是否平稳的统计方法。"
      },
      "arima": {
        "aliases": [
          "AutoRegressive Integrated Moving Average",
          "ARIMA"
        ],
        "display": "ARIMA模型",
        "gloss": "结合自回归、差分和移动平均的时间序列预测模型。"
      },
      "stationarity": {
        "aliases": [
          "Stationarity"
        ],
        "display": "平稳性",
        "gloss": "时间序列的统计性质（均值、方差）不随时间变化。"
      }
    }
  },
  {
    "lesson_id": "L4",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [
          "hurst_exponent"
        ],
        "id": "s037",
        "introduced_terms": [
          "hurst_exponent"
        ],
        "lines": [
          "有没有一个数字，能直接告诉我们市场是什么状态？",
          "有——<term id=\"hurst_exponent\">Hurst指数</term>。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "hurst_exponent"
        ],
        "id": "s038",
        "introduced_terms": [],
        "lines": [
          "H = 0.5 → 随机游走",
          "H < 0.5 → 均值回归",
          "H > 0.5 → 趋势"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "hurst_exponent"
        ],
        "id": "s039",
        "introduced_terms": [],
        "lines": [
          "H越接近0，均值回归越强；",
          "H越接近1，趋势越强。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "half_life"
        ],
        "id": "s040",
        "introduced_terms": [
          "half_life"
        ],
        "lines": [
          "对于均值回归策略，还需要知道一个关键数字：",
          "<term id=\"half_life\">半衰期</term>。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "half_life"
        ],
        "id": "s041",
        "introduced_terms": [],
        "lines": [
          "半衰期 = ln(2) / κ",
          "κ是均值回归的速度。κ越大，回归越快。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "half_life"
        ],
        "id": "s042",
        "introduced_terms": [],
        "lines": [
          "如果半衰期是20天，",
          "那么用5天的均线来做均值回归就不太合适。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "H < 0.5 表示均值回归，越接近0回归越强。",
          "kind": "single_choice",
          "options": [
            "强趋势",
            "均值回归",
            "随机游走"
          ],
          "prompt": "Hurst指数为0.3，说明市场处于什么状态？"
        },
        "focus_terms": [
          "hurst_exponent"
        ],
        "id": "s043",
        "introduced_terms": [],
        "lines": [
          "Hurst指数为0.3，说明市场处于什么状态？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step6",
    "source": {
      "plain_text": "Hurst Exponent, Half Life of Mean Reversion",
      "related": []
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "half_life": {
        "aliases": [
          "Half Life"
        ],
        "display": "半衰期",
        "gloss": "价格偏离均值后恢复到一半所需的时间。"
      },
      "hurst_exponent": {
        "aliases": [
          "Hurst Exponent",
          "H"
        ],
        "display": "Hurst指数",
        "gloss": "衡量时间序列趋势或均值回归强度的指标，范围0到1。"
      }
    }
  },
  {
    "lesson_id": "L4",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [
          "martingale_strategy"
        ],
        "id": "s044",
        "introduced_terms": [
          "martingale_strategy"
        ],
        "lines": [
          "如果市场真的是随机游走，还能赚钱吗？",
          "有一种古老的策略可以——<term id=\"martingale_strategy\">Martingale策略</term>。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "martingale_strategy"
        ],
        "id": "s045",
        "introduced_terms": [],
        "lines": [
          "原理很简单：每次输了就加倍下注。",
          "只要赢一次，就能把之前所有亏损赚回来，还多赚一点。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "martingale_strategy"
        ],
        "id": "s046",
        "introduced_terms": [],
        "lines": [
          "赌场例子：第一局押1元，输了押2元，再输押4元……",
          "直到赢了，净赚1元。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "martingale_strategy"
        ],
        "id": "s047",
        "introduced_terms": [],
        "lines": [
          "数学上，连续输n次的概率是0.5ⁿ。",
          "连续输10次的概率只有0.1%，看起来几乎不可能。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "martingale_strategy"
        ],
        "id": "s048",
        "introduced_terms": [],
        "lines": [
          "但风险在于：你需要无限的资金。",
          "如果连续输10次，第11次需要押1024元。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "martingale_strategy"
        ],
        "id": "s049",
        "introduced_terms": [],
        "lines": [
          "所以Martingale策略在理论上可行，",
          "但实践中必须严格控制仓位和资金管理。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "连续亏损时赌注指数增长，资金不足就会爆仓。",
          "kind": "single_choice",
          "options": [
            "胜率太低",
            "需要无限资金，可能爆仓",
            "交易成本太高"
          ],
          "prompt": "Martingale策略的核心风险是什么？"
        },
        "focus_terms": [
          "martingale_strategy"
        ],
        "id": "s050",
        "introduced_terms": [],
        "lines": [
          "Martingale策略的核心风险是什么？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step7",
    "source": {
      "plain_text": "Martingale Strategy for random walk market",
      "related": []
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "martingale_strategy": {
        "aliases": [
          "Martingale Strategy"
        ],
        "display": "Martingale策略",
        "gloss": "每次亏损后加倍下注，直到盈利后回到初始赌注的策略。"
      }
    }
  },
  {
    "lesson_id": "L4",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [],
        "id": "s051",
        "introduced_terms": [],
        "lines": [
          "回顾一下：市场有三种状态。",
          "每种状态都有对应的分析工具。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s052",
        "introduced_terms": [],
        "lines": [
          "趋势市场 → 线性回归、ARIMA",
          "均值回归 → Hurst指数、半衰期",
          "随机游走 → Martingale策略"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s053",
        "introduced_terms": [],
        "lines": [
          "关键不是哪个模型更高级，",
          "而是先判断当前市场属于哪种状态。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s054",
        "introduced_terms": [],
        "lines": [
          "用错模型比不用模型更危险。",
          "在趋势市场做均值回归，会亏得很惨。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s055",
        "introduced_terms": [],
        "lines": [
          "一句话带走：",
          "先分类，再建模，最后交易。"
        ],
        "type": "narration"
      }
    ],
    "sequence_id": "step8",
    "source": {
      "plain_text": "Key Takeaways",
      "related": []
    },
    "target_language": "zh-CN",
    "term_catalog": {}
  }
]

</GUIDED_STORY_STEPS>

<CURRENT_STEP_ID>
step3
</CURRENT_STEP_ID>

<CURRENT_STEP>
{
  "lesson_id": "L4",
  "mode": "guided_story",
  "screens": [
    {
      "focus_terms": [],
      "id": "s017",
      "introduced_terms": [],
      "lines": [
        "理论讲完了，来实战。",
        "我们用 Python 对微软股票跑一个线性回归。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s018",
      "introduced_terms": [],
      "lines": [
        "先获取数据：用 yfinance 下载 MSFT 2023年的日线数据。",
        "然后构造特征：用“一年中的第几天”作为 X。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s019",
      "introduced_terms": [],
      "lines": [
        "用 sklearn 的 LinearRegression 拟合模型。",
        "得到截距和斜率。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "r_squared"
      ],
      "id": "s020",
      "introduced_terms": [],
      "lines": [
        "然后评估：R² = 0.46。",
        "说明直线只能解释不到一半的价格变动。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "linear_regression_strategy"
      ],
      "id": "s021",
      "introduced_terms": [
        "linear_regression_strategy"
      ],
      "lines": [
        "但这不妨碍我们用它来交易。",
        "策略很简单：如果预测未来价格高于当前，就买入。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "linear_regression_strategy"
      ],
      "id": "s022",
      "introduced_terms": [],
      "lines": [
        "回测结果显示：年化收益约3.6%，胜率58.8%。",
        "不算惊艳，但证明了线性模型确实能捕捉到一些趋势。"
      ],
      "type": "narration"
    },
    {
      "exercise": {
        "answer": 1,
        "explanation": "线性回归假设价格与时间存在线性关系，即趋势。",
        "kind": "single_choice",
        "options": [
          "价格会随机波动",
          "价格存在可预测的线性趋势",
          "价格永远回归均值"
        ],
        "prompt": "线性回归策略的核心假设是什么？"
      },
      "focus_terms": [
        "linear_regression_strategy"
      ],
      "id": "s023",
      "introduced_terms": [],
      "lines": [
        "线性回归策略的核心假设是什么？"
      ],
      "type": "exercise"
    }
  ],
  "sequence_id": "step3",
  "source": {
    "plain_text": "Python Example: fit LR model, backtest LR strategy",
    "related": []
  },
  "target_language": "zh-CN",
  "term_catalog": {
    "linear_regression_strategy": {
      "aliases": [
        "LR Trading Strategy"
      ],
      "display": "线性回归交易策略",
      "gloss": "利用线性回归模型预测价格，当实际价格偏离预测值时进行交易。"
    }
  }
}

</CURRENT_STEP>

<PLAIN_TEXT>
# L4: Statistical time series analysis for market classification

Course Code: COMP7415

# Agenda

- Common classification for financial market   
- Market prediction with statistical models

- Simple Linear Regression   
- Multiple Linear Regression   
ARIMA   
- Hurst Exponent

- Martingale Strategy

# Market Classification

The financial market can generally be classified into

1. Trending (Momentum)   
2. Mean Reversion   
3. Random Walk

# Trending (Momentum)

- Momentum trading involves

- buying securities that are trending upwards and selling them when they appear to have peaked, or   
- shorting securities that are trending downwards and covering them when they appear to have bottomed out.

![](images/09778c2b01cda28bb5b71d28e6d1e2b93355193e35bfb0b0d56d9a47a92629d4.jpg)

# Mean Reversion

- Mean reversion suggests that asset prices and historical returns eventually revert to their long-term mean or average level.   
- Based on the idea that extreme price movements are temporary and will revert to the mean.

![](images/9c30145270dc07b2f7ff723423144bade20fb1a716cbf144b29fa02a9777accd.jpg)

![](images/617b59e8209c742aca5c4e4cc6bb0330d7ac0dc2c81306ba24c7f3696a914c01.jpg)

![](images/f95be69cdc7c9db8c708fd963500e1b8ea92a63e227516a3406fedbaf1683ccf.jpg)

# Trending and Mean Reversion can occur at the same time

![](images/f60784d53f2cbaa56f4be7f24b9d773bc6ce6fccd06fac8de18ebbf656f11f28.jpg)

# Random Walk

- It implies that price changes are random and do not follow any patterns or trends.

- Efficient Market Hypothesis (EMH): Underpins the idea that all known information is already reflected in stock prices, making it impossible to consistently achieve higher returns than average market returns.   
- Independence: Each price change is independent of previous price changes.   
- Unpredictability: Future price movements cannot be predicted based on past price movements.

- If the market is random, can you think of a trading strategy that can be profitable?

# Time series Analysis

# Introduction to Time Series Analysis

- Time series analysis involves examining data points collected or recorded at specific time intervals.   
- It aims to identify patterns, trends, and other characteristics in the data.   
Examples:

Stock prices over time   
Monthly unemployment rates   
Daily temperature readings   
Quarterly GDP growth rates

# Simple Linear Regression

For market trend identification

# Simple Linear Regression Model

- Linear regression is a statistical method for modeling the linear relationship between a dependent variable and independent variable.   
Key Concepts:

- Dependent Variable (Y): The variable we are trying to predict or explain.   
- Independent Variable (X): The variable we use to make predictions.   
- Linear Equation:

$$
Y = \beta_ {0} + \beta_ {1} X + \epsilon
$$

$\beta_0$ : Intercept

$\beta_{1}$ : Slope

$\epsilon$ : Error term

# Why Use Linear Regression?

- Simplicity: Easy to understand and implement.   
- Interpretability: Coefficients provide insights into the relationship between variables.   
- Efficiency: Computationally efficient for small to medium-sized datasets.   
- Foundation: Basis for more complex models.

# Example

![](images/110b8e6f7754a2e3714d694bfdc882fb107e384a63b8f024a1406b094465c3b8.jpg)  
Reported happiness as a function of income

# Parameter Estimation

- Define $y_{i}$ is the actual value

$\hat{y}_i$ is the predicted value

N is the Number of observation

$\overline{X}, \overline{Y}$ are the mean of $\{x_{i}\}$ and $\{y_{i}\}$ respectively

- Residual is the difference between actual and model estimated value

$$
e _ {i} = y _ {i} - \hat {y} _ {i}
$$

- Mean squared error represents the average squared residual

$$
M S E = \frac {\sum e _ {i} ^ {2}}{n - 2} = \frac {\sum \left(y _ {i} - \hat {y} _ {i}\right) ^ {2}}{n - 2} \quad \leftarrow \quad \text {d e g r e e o f f r e e d o m}
$$

![](images/c8f974b3fa6c361c77236942f6e29aeac57f59e1c34dd10a81f5c5f0506b6cb6.jpg)

# Parameter Estimation

- Predicted value: $\widehat{y}_i = \beta_0 + \beta_1 x_i$   
Sum of Squared Error (SSE):

$$
S S E = \sum (y _ {i} - \hat {y} _ {i}) ^ {2} = \sum \bigl (y _ {i} - (\beta_ {0} + \beta_ {1} x _ {i}) \bigr) ^ {2}
$$

- Take partial derivatives, and set to zero to solve for $\beta_0$ and $\beta_1$

$$
\begin{array}{l} \frac {\partial S S E}{\partial \beta_ {0}} = - 2 \sum (y _ {i} - \beta_ {0} - \beta_ {1} x _ {i}) = 0 \\ \frac {\partial S S E}{\partial \beta_ {1}} = - 2 \sum (y _ {i} - \beta_ {0} - \beta_ {1} x _ {i}) x _ {i} = 0 \\ \end{array}
$$

$$
\begin{array}{l} \beta_ {0} = \bar {Y} - \beta_ {1} \bar {X} \\ \beta_ {1} = \frac {\sum (x _ {i} - \bar {X}) (y _ {i} - \bar {Y})}{\sum (x _ {i} - \bar {X}) ^ {2}} \\ \end{array}
$$

# Distribution of $\beta_0, \beta_1$

- Assume error term follows identical independent normal distribution (i.i.d.)

$$
\in \sim N (0, \sigma^ {2})
$$

- Unbiased estimate of $\sigma^2$ is

$$
s ^ {2} = M S E = \frac {S S E}{n - 2} = \frac {\sum (y _ {i} - \hat {y} _ {i}) ^ {2}}{n - 2}
$$

- Standard error of $\beta_{1}$

$$
S E (\beta_ {1}) = \sqrt {\frac {s ^ {2}}{\sum (x _ {i} - \bar {X}) ^ {2}}}
$$

- Standard error of $\beta_0$

$$
S E (\beta_ {0}) = s \sqrt {\frac {1}{n} + \frac {\bar {X} ^ {2}}{\sum (x _ {i} - \bar {X}) ^ {2}}}
$$

# Confidence Interval

- A $100(1 - \alpha) \%$ confidence interval for $\beta_{1}$

$$
\widehat {\beta_ {1}} \pm t _ {\alpha / 2, n - 2} \sqrt {\frac {s ^ {2}}{\sum (x _ {i} - \bar {X}) ^ {2}}}
$$

- A $100(1-\alpha) \%$ confidence interval for $\beta_0$

$$
\widehat {\beta_ {0}} \pm t _ {\alpha / 2, n - 2} \times s \sqrt {\frac {1}{n} + \frac {\bar {X} ^ {2}}{\sum (x _ {i} - \bar {X}) ^ {2}}}
$$

# Model Assumptions

1. Linear Relationship: the existence of a linear relationship between the dependent variable and the independent variables. This linearity can be visually inspected using scatterplots, which should reveal a straight-line relationship rather than a curvilinear one.   
2. Multivariate Normality: it assumes that the residuals are normally distributed. This assumption can be assessed by examining histograms or Q-Q plots of the residuals, or through statistical tests such as the Kolmogorov-Smirnov test.   
3. Independence: each observation is independent of the others   
4. No Multicollinearity: the independent variables $\{X_1, X_2, \ldots\}$ are not highly correlated with each other. This can be checked using correlation matrices.   
5. Homoscedasticity: The variance of error terms (residuals) should be consistent across all levels of the independent variables. A scatterplot of residuals versus predicted values should not display any discernible pattern, such as a cone-shaped distribution, which would indicate heteroscedasticity

# Model Assumptions

# 1. Linearity

(Linear relationship between Y and each X)

![](images/6f3f214a6a0b27a129376959b43b7db84b2cd6f200c47df0791906dee4497e18.jpg)

![](images/60b7ca546c56351b9cb21be440ee2b08632bb71b547c851ff09b42d15abc9e57.jpg)

# 2. Multivariate Normality

(Normality of error distribution)

![](images/2799e10369cca728468751d5840ead944852f1d06910fcbea3c3e77b7847368b.jpg)

![](images/6f3cdd8f0977d9f160826d8005d61503d481227e06458a56ea9d377a3c6e20b4.jpg)

# 3. Independence

(of observations. Includes "no autocorrelation")

![](images/db1db0813dbcb546694e06d346ed1916b3f3cc00f2040ff9d4bec0d9d0681db0.jpg)

![](images/7a8db2be8a902ab055f08ad2991cdf8d85599599077a867a0acea38ae8c7d608.jpg)

# 4. Lack of Multicollinearity

(Predictors are not correlated with each other)

$$
X _ {1} \neq X _ {2}
$$

$$
X _ {1} \sim X _ {2}
$$

# 5. Homoscedasticity

(Equal variance)

![](images/b3f7da3fffeec0c22001f6608bedcc88928bc74ebe696c5c5583afb41146d05d.jpg)

# Model Effectiveness

R-Squared (Coefficient of Determination)

- represents the proportion of the variance in the dependent variable that is predictable from the independent variable(s).   
- Indicates how well the data points fit a regression line.

• Interpretation

- $R^{2}$ ranges from 0 to 1.   
- $R^2 = 1$ : Perfect fit (the model explains all the variability of the response data around its mean).   
- $R^2 = 0$ : The model does not explain any of the variability of the response data around its mean.

- Formula

$$
R ^ {2} = 1 - \frac {S S _ {r e s}}{S S _ {t o t}} \qquad \begin{array}{l} S S _ {r e s} (\text {S u m o f s q u a r e d r e s i d u a l s}) = \sum (y _ {i} - \hat {y} _ {i}) ^ {2} \\ S S _ {t o t} (\text {T o t a l S u m o f S q u a r e s}) = \sum (y _ {i} - \overline {{Y}}) ^ {2} \end{array}
$$

# Breakdown of Regression Errors

![](images/0b83a2e7b7437553477a0faf01b502b757efba31f40f11394b425cc2b3387dec.jpg)

# $R^{2}$ Example

- Suppose we have the following data points:

Actual values $(Y)$ : [3, 4, 5, 6, 7]   
Predicted values $(\widehat{Y})$ : [2.8, 4.1, 5.3, 6.2, 7.4]

• $S S_{r e s} = (3 - 2.8)^{\wedge} 2 + (4 - 4.1)^{\wedge} 2 + (5 - 5.3)^{\wedge} 2 + (6 - 6.2)^{\wedge} 2 + (7 - 7.4)^{\wedge} 2 = 0.37$   
• $S S_{tot} = (3 - 5)^{\wedge}2 + (4 - 5)^{\wedge}2 + (5 - 5)^{\wedge}2 + (6 - 5)^{\wedge}2 + (7 - 5)^{\wedge}2 = 10$

$$
R ^ {2} = 1 - \frac {S S _ {r e s}}{S S _ {t o t}} = 1 - \frac {0 . 3 7}{1 0} = 0. 9 6 3
$$

![](images/703b99e5cda9f9eff67e11c68d161451295e7a30ccbfed88fa1abb4215e6acdc.jpg)

# Scikit-learn

- Official website (https://scikit-learn.org/stable/api/index.html)

![](images/72631ba8d0a443bab08e8aa7555150a0cecbce902c7f50888474003d3c3267f7.jpg)

pip install scikit-learn

# Python Example (1): download MSFT data

import pandas as pd

import yfinance as yf

Fetch historical data for Microsoft (MSFT)

msft_data = yf.download('MSFT', start='2023-01-01', end='2023-12-31')

Display the first few rows of the dataset

print(msft_data.head())

Preprocess the data

msft_data['Date'] = msft_data.index

msft_data['Date'] = pd.to datetime(msft_data['Date'])

msft_data.set_index('Date', inplace=True)

<table><tr><td>Price</td><td>Close</td><td>High</td><td>Low</td><td>Open</td><td>Volume</td></tr><tr><td>Ticker</td><td>MSFT</td><td>MSFT</td><td>MSFT</td><td>MSFT</td><td>MSFT</td></tr><tr><td>Date</td><td></td><td></td><td></td><td></td><td></td></tr><tr><td>2023-01-03</td><td>233.985687</td><td>240.011613</td><td>231.856584</td><td>237.403960</td><td>25740000</td></tr><tr><td>2023-01-04</td><td>223.750366</td><td>227.432323</td><td>220.683688</td><td>226.856104</td><td>50623400</td></tr><tr><td>2023-01-05</td><td>217.118896</td><td>222.236544</td><td>216.581736</td><td>221.894711</td><td>39585600</td></tr><tr><td>2023-01-06</td><td>219.677750</td><td>220.488370</td><td>214.228059</td><td>217.792823</td><td>43613600</td></tr><tr><td>2023-01-09</td><td>221.816589</td><td>225.840395</td><td>221.123177</td><td>221.162236</td><td>27369800</td></tr></table>

# Python Example (2): fit LR model

from sklearn.linear_model import LinearRegression   
```python
Prepare the features and target variable  
msft_data['Day'] = msft_data.index.dayofyear  
X = msft_data['Day'])  
y = msft_data['Close'] 
```

```txt
Split the data into training and testing sets  
n_train = int(len(X)*0.8)  
n_test = len(X) - n_train 
```

```txt
X_train, X_test = X[0:n_train], X[-n_test:]  
y_train, y_test = y[0:n_train], y[-n_test:] 
```

```txt
Initialize and train the linear regression model  
model = LinearRegression()  
model.fit(X_train, y_train) 
```

```txt
Display the coefficients  
print(f"Intercept: {model.intercept}")  
print(f"Slope: {model.coef[0]}") 
```

```txt
Intercept: [241.94576113]  
Slope: [0.36464911] 
```

# Python Example (3): model evaluation

from sklearn.metrics import mean_squared_error, r2_score

Make predictions on the test set

y_pred0 = model.predict(X_train)

y_pred = model.predict(X_test)

Calculate evaluation metrics

mse = mean_squared_error(y_test, y_pred)

r2 = r2_score(y_test, y_pred)

print(f"Mean Squared Error: {mse}")

print(f'R-squared:{r2}')

Plot the results

import matplotlib.pyplot as plt

plt.style.use('darkbackground')

Plot the results (training set)

plt.figure(figsize=(10,6))

pltscatter(X_train，y_train，color $\equiv$ 'blue'，label $=$ 'Actual Prices')

plt.plot(X_train.to_numpy(), y_pred0, color='red', linewidth=2, label='Predicted Prices')

plt.xlabel('Day of Year')

plt.ylabel('Closing Price (USD)')

plt.title('MSFT Stock Price Prediction (Training Dataset)')

pltlegend()

plt.show()

Plot the results (test set)

plt.figure(figsize=(10,6))

pltscatter(X_test,y_test,color $\equiv$ 'blue',label $=$ Actual Prices')

plt.plot(X_test.to_numpy(), y_pred, color='red', linewidth=2, label='Predicted Prices')

plt.xlabel('Day of Year')

plt.ylabel('Closing Price (USD)')

plt.title('MSFT Stock Price Prediction (Test Dataset)')

pltlegend()

plt show()

Mean Squared Error: 144.25272079512234

R-squared: 0.46490145270565975

![](images/65b2347273238c6826a0f4fbba54408b60349e3918a24b00960c28320b0ec292.jpg)

![](images/b48a4b0dc090959f074f86d7e160fb7c684c613a2189966af51ef5ebafd9134b.jpg)

# Python Example (4): future price forecast

import numpy as np

Generate future days for forecasting

future每一天 = np.array(range(1, 366)).reshape(-1, 1)

Predict future prices

futurePrices = model.predict(future每一天)

Plot the forecast

plt.figure(figsize=(10,6))

plt.plot(future_days, futurePrices, color='green', linewidth=2, label='Forecasted Prices')

plt.xlabel('Day of Year')

plt.ylabel('Closing Price (USD)')

plt.title('MSFT Stock Price Forecast')

plt.legend()

plt.show()

![](images/021a7cccbe19284374f6b0fcd08d03857969faaad2158e9d61e5f05a95fb1240.jpg)

# Linear Regression Trading Strategy

- Collect the previous $N$ observations and fit a linear regression model   
Strategy 1:

Take LR model's predicted values as the fair/reference market price.   
- If current market price is much lower than the fair price, open LONG position   
- If current market price is much higher than the fair price, open SHORT position

Strategy 2:

- Project the future prices using the LR model   
- If the future price is projected to be higher than the current price, open LONG position   
- If the future price is projected to be lower than the current price, open SHORT position

# Backtest LR Strategy

```python
from AlgoAPI import AlgoAPIUtil, AlgoAPI_Backtest  
import pandas as pd  
import numpy as np  
from sklearn.linear_model import LinearRegression  
from sklearn.metrics import mean_squared_error, r2_score 
```

- Instrument: MSFT   
- Initial capital: 1000 USD   
Period: 2023.01.01 - 2023.12.31   
- Data Interval: 1 day   
- Short-selling allowed: No

```python
class AlgoEvent: def __init__(self): self.numObs = 30 def start(self, mEvt): self.evt = AlgoAPI_Backtest.AlgoEvtHandler(self, mEvt) self.evt.start() 
```

```python
def on_marketdatafeed(self, md, ab): # get previous 100 days closing price obs = self.evt.getHistoricalBar contract={'instrument":"MSFT"}, numOfBar = self.numObs, interval="D", timestamp=md.timESTAMP) Y = [obs[t][c] for t in obs] X = np.array(range(0, self.numObs)).reshape(-1, 1) 
```

```python
# fit ARIMA model  
model = LinearRegression()  
model.fit(X, Y)  
# evaluate model effectiveness  
Y_pred = model.predict(X)  
mse = mean_squared_error(Y, Y_pred)  
r2 = r2_score(Y, Y_pred)  
self.evt consoleLog('mse, r-square = ', mse, r2) 
```

```python
# entry if e-square > 0.5
if r2 > 0.5:
    # 5 day forecast
future_days = np.array(range(1, 5)).reshape(-1, 1)
forecast = model.predict(future_day)
future_price = forecast[-1]
self.evt consoleLog(forecast)
# open buy order if future price is higher than current price
if future_price > md.askPrice:
    order = AlgoAPIUtil.OrderObject(
        instrument = md.instrument,
        openclose = 'open',
        buysell = 1, #buy
        ordertype = 0, #market order
        volume = 1, #1 lot
        holdtime = 24*60*60*5 #holding period for 5 days
) 
```

# Backtest LR Strategy

Monthly Return:   

<table><tr><td></td><td>Year</td><td>Jan</td><td>Feb</td><td>Mar</td><td>Apr</td><td>May</td><td>Jun</td><td>Jul</td><td>Aug</td><td>Sep</td><td>Oct</td><td>Nov</td><td>Dec</td><td>YTD</td></tr><tr><td></td><td>2023</td><td>4.52%</td><td>0.0%</td><td>0.0%</td><td>0.0%</td><td>0.0%</td><td>0.0%</td><td>0.0%</td><td>-1.61%</td><td>-0.01%</td><td>0.97%</td><td>0.0%</td><td>0.0%</td><td>3.82%</td></tr><tr><td colspan="15">Performance Statistics:</td></tr><tr><td colspan="4">No. of Tradable Days:</td><td>272</td><td colspan="4">No. of Win Days:</td><td>20</td><td colspan="4">No. of Loss Days:</td><td>14</td></tr><tr><td colspan="4">Win Rate:</td><td>58.8235%</td><td colspan="4">Max. Consecutive Win Day:</td><td>7</td><td colspan="4">Max. Consecutive Loss Day:</td><td>4</td></tr><tr><td colspan="4">Odd Ratio:</td><td>1.4286</td><td colspan="4">Max. Consecutive Gains:</td><td>42.54</td><td colspan="4">Max. Consecutive Loss:</td><td>-37.05</td></tr><tr><td colspan="4">No. of Trades:</td><td>32</td><td colspan="4">Average Consecutive Win Day:</td><td>1.93</td><td colspan="4">Average Consecutive Loss Day:</td><td>0.1</td></tr><tr><td colspan="4">Total PnL:</td><td>38.23</td><td colspan="4">Average Per Trade PnL:</td><td>1.19</td><td colspan="4">Average Per Day PnL:</td><td>0.14</td></tr><tr><td colspan="4">Mean Daily Return:</td><td>0.0144%</td><td colspan="4">Median Daily Return:</td><td>0.0%</td><td colspan="4">Mean Annual Return:</td><td>3.6324%</td></tr><tr><td colspan="4">Daily Return StdDev:</td><td>0.3043%</td><td colspan="4">25th percentile Daily Return:</td><td>0.0%</td><td colspan="4">75th percentile Daily Return:</td><td>0.0%</td></tr><tr><td colspan="4">Daily Return Downside StdDev:</td><td>0.2275%</td><td colspan="4">95% 1 day return VaR:</td><td>-0.0055%</td><td colspan="4">99% 1 day return VaR:</td><td>-0.4191%</td></tr><tr><td colspan="4">Daily Sharpe Ratio:</td><td>0.0474</td><td colspan="4">Annual Sharpe Ratio:</td><td>0.7519</td><td colspan="4">Max. Drawdown Amount:</td><td>37.05</td></tr><tr><td colspan="4">Daily Sortino Ratio:</td><td>0.0634</td><td colspan="4">Annual Sortino Ratio:</td><td>1.0057</td><td colspan="4">Max. Drawdown Percent:</td><td>3.5215%</td></tr><tr><td colspan="4">Max. Drawdown Duration:</td><td>97</td><td colspan="4">Average Drawdown Duration:</td><td>17.73</td><td colspan="4">Annual Volatility:</td><td>4.8115%</td></tr><tr><td colspan="4">Gross Profit:</td><td>61.479</td><td colspan="4">Gross Loss:</td><td>-23.931</td><td colspan="4">Profit Factor:</td><td>2.569</td></tr><tr><td colspan="4">Jensen Alpha:</td><td>0.0</td><td colspan="4">Beta:</td><td>0.0</td><td colspan="4">Information Ratio:</td><td>0.0</td></tr><tr><td colspan="4">Omega Ratio:</td><td>0.0</td><td colspan="4">Treynor Ratio:</td><td>0.0</td><td colspan="4">Tail Ratio:</td><td>28.0459</td></tr><tr><td colspan="4">Calmar Ratio:</td><td>1.0315</td><td colspan="4">Average Holding Day:</td><td>5.5625</td><td colspan="4">Annual Turnover Rate:</td><td>403.8137%</td></tr></table>

# Multiple Linear Regression Model

# MLR Model

- It is an extension of the simple linear regression model.   
- Dependent Variable (Y): The outcome we are trying to predict (eg. stock price).   
- Independent Variables $(X_{1}, X_{2}, \ldots, X_{p})$ : The predictors or features that influence Y (eg. economic indicators, technical indicators).

$$
Y = \beta_ {0} + \beta_ {1} X _ {1} + \beta_ {2} X _ {2} + \dots + \beta_ {p} X _ {p} + \epsilon
$$

$\beta_0$ : Intercept

$\beta_{1}, \beta_{2}, \ldots, \beta_{p}$ : coefficients

$\epsilon$ : Error term

# MLR Model

- For $p$ independent variables and $n$ observations,   
In matrix presentation,

$$
\underline {{\mathbf {Y}}} = X \underline {{\boldsymbol {\beta}}} + \underline {{\epsilon}}
$$

where

$$
\underline {{Y}} = \left( \begin{array}{c} y _ {1} \\ y _ {2} \\ \vdots \\ y _ {n} \end{array} \right)
$$

$$
\boldsymbol {X} = \left( \begin{array}{l l l l} 1 & x _ {1 1} & \dots & x _ {1 p} \\ \vdots & & \ddots & \vdots \\ 1 & x _ {n 1} & \dots & x _ {n p} \end{array} \right)
$$

$$
\underline {{\boldsymbol {\beta}}} = \left( \begin{array}{c} \beta_ {0} \\ \beta_ {1} \\ \vdots \\ \beta_ {p} \end{array} \right)
$$

$$
\underline {{\epsilon}} = \left( \begin{array}{c} \epsilon_ {1} \\ \epsilon_ {2} \\ \vdots \\ \epsilon_ {n} \end{array} \right)
$$

# Regression Plane

- Find a linear plane such that the total distance between the data points and the plane is the smallest

![](images/a1d4dec39f0cfa434b8e23147e6f92df71bcc19da8f2bf8c8911fbf7fe7be86d.jpg)

![](images/d2d170e4f5cb4442e7a2357adf93fdc818371969b26cc41319a5d4b46dff3df9.jpg)

# Least Square Estimation of $\beta$

Fitted value

$$
\widehat {Y} = X \beta
$$

- Residual

$$
e = Y - \widehat {Y}
$$

Sum of Squared Error

$$
S S E = \sum_ {i = 1} ^ {n} (y _ {i} - \widehat {y} _ {i}) ^ {2} = \left(\boldsymbol {Y} - \widehat {\boldsymbol {Y}}\right) ^ {T} \left(\boldsymbol {Y} - \widehat {\boldsymbol {Y}}\right) = (\boldsymbol {Y} - X \beta) ^ {T} (\boldsymbol {Y} - X \beta)
$$

# Least Square Estimation of $\beta$

- The estimator $\beta$ satisfies

$$
\frac {\partial}{\partial \boldsymbol {\beta}} \left(\left(\boldsymbol {Y} - \boldsymbol {X} \boldsymbol {\beta}\right) ^ {T} \left(\boldsymbol {Y} - \boldsymbol {X} \boldsymbol {\beta}\right)\right) = 0
$$

- Assume $X$ is full ranked and $X^T X$ is invertible, then

$$
\begin{array}{l} - 2 X ^ {T} (Y - X \boldsymbol {\beta}) = \mathbf {0} \\ - X ^ {T} Y + \left(X ^ {T} X\right) \beta = 0 \\ \boldsymbol {\beta} = \left(\boldsymbol {X} ^ {T} \boldsymbol {X}\right) ^ {- 1} \boldsymbol {X} ^ {T} \boldsymbol {Y} \\ \end{array}
$$

# Distribution of $\beta$

Expected value

$$
E (\pmb {\beta}) = E \left(\left(\boldsymbol {X} ^ {T} \boldsymbol {X}\right) ^ {- 1} \boldsymbol {X} ^ {T} \boldsymbol {Y}\right) = \left(\boldsymbol {X} ^ {T} \boldsymbol {X}\right) ^ {- 1} \boldsymbol {X} ^ {T} E (\boldsymbol {Y}) = \left(\boldsymbol {X} ^ {T} \boldsymbol {X}\right) ^ {- 1} \boldsymbol {X} ^ {T} \boldsymbol {X} \pmb {\beta} = \pmb {\beta}
$$

- Variance

$$
\begin{array}{l} V a r (\pmb {\beta}) = V a r \left(\left(\pmb {X} ^ {T} \pmb {X}\right) ^ {- 1} \pmb {X} ^ {T} \pmb {Y}\right) \\ = \left(X ^ {T} X\right) ^ {- 1} X ^ {T} V a r (Y) X \left(X ^ {T} X\right) ^ {- 1} \\ = \left(X ^ {T} X\right) ^ {- 1} X ^ {T} \sigma^ {2} I _ {n} X \left(X ^ {T} X\right) ^ {- 1} \\ = \sigma^ {2} \left(X ^ {T} X\right) ^ {- 1} \\ \end{array}
$$

# Distribution of $\beta$

- Assume error term follows identical independent normal distribution (i.i.d.)

$$
\mathbf {\sigma} \in \sim N (0, \sigma^ {2} I _ {n})
$$

- $\beta$ is a linear transformation of $Y$ and therefore

$$
\beta \sim N \left(\hat {\beta}, \sigma^ {2} \left(X ^ {T} X\right) ^ {- 1}\right)
$$

- Unbiased estimate of $\sigma^2$ is

$$
s ^ {2} = M S E = \frac {S S E}{n - p}
$$

- Standard error of $\beta_{j}$ will be the j-th diagonal element of $s^{2}\left(X^{T}X\right)^{-1}$

# Determine if a factor is significant

Hypothesis Test

$$
H _ {0} \colon \beta_ {j} = 0
$$

$$
H _ {1} \colon \beta_ {j} \neq 0
$$

Test statistics:

$$
\frac {\beta_ {j} - \widehat {\beta} _ {j}}{S E (\beta_ {j})} \sim t _ {n - p}
$$

- Interval estimate at a confidence level $1 - \alpha$ (eg. $\alpha = 95\%$ )

$$
\widehat {\beta} _ {j} \pm t _ {\alpha / 2, n - p} S E (\beta_ {j})
$$

# Python Example

- Suppose a stock price is related to economic factors, such as inflation rate, GDP, and interest rate.

<table><tr><td>Time</td><td>Stock Price Y</td><td>GDP X1</td><td>Interest Rate X2</td><td>Inflation Rate X3</td></tr><tr><td>1</td><td>100</td><td>2.5</td><td>3.5</td><td>1.2</td></tr><tr><td>2</td><td>105</td><td>3</td><td>3</td><td>1.5</td></tr><tr><td>3</td><td>110</td><td>3.5</td><td>2.5</td><td>1.3</td></tr><tr><td>4</td><td>120</td><td>4</td><td>2</td><td>1</td></tr><tr><td>5</td><td>130</td><td>4.5</td><td>1.5</td><td>0.8</td></tr><tr><td>6</td><td>140</td><td>5</td><td>1</td><td>0.6</td></tr><tr><td>7</td><td>150</td><td>5.5</td><td>0.5</td><td>0.4</td></tr><tr><td>8</td><td>160</td><td>6</td><td>0</td><td>0.2</td></tr></table>

# statsmodels

- Official website (https://www.statsmodels.org/stable/index.html)

![](images/d4ee2a8dba148b81b00c553a6e3793912edbbd80075e4cd9c218317a22279eb6.jpg)

# Python Example

![](images/333f47c5f93f76e117a499ecd829838c0201ab89943d5deef37cc513d57a2998.jpg)

warnings.warn("kurtosistest only valid for n>=20 ... continuing " OLS Regression Results   
```txt
Dep. Variable: Stock_Price R-squared: 0.995  
Model: OLS Adj. R-squared: 0.993  
Method: Least Squares F-statistic: 493.9  
Date: Fri, 14 Feb 2025 Prob (F-statistic): 1.80e-06  
Time: 20:05:00 Log-Likelihood: -14.332  
No. Observations: 8 AIC: 34.66  
Df Residuals: 5 BIC: 34.90  
Df Model: 2  
Covariance Type: nonrobust  
coef std err t P>|t| [0.025 0.975]  
const 6.4238 0.331 19.418 0.000 5.573 7.274  
GDP 25.8167 0.249 103.635 0.000 25.176 26.457  
Interest_Rate 12.7261 1.858 6.849 0.001 7.950 17.503  
Inflation_Rate -13.1890 4.722 -2.793 0.038 -25.327 -1.051  
Omnibus: 0.549 Durbin-Watson: 1.556  
Prob(Omnibus): 0.760 Jarque-Bera (JB): 0.454  
Skew: 0.441 Prob(JB): 0.797  
Kurtosis: 2.236 Cond. No. 5.40e+16 
```

# Notes:

[1] Standard Errors assume that the covariance matrix of the errors is correctly specified.   
[2] The smallest eigenvalue is 6.36e-32. This might indicate that there are strong multicollinearity problems or that the design matrix is singular.

import pandas as pd

import numpy as np

import statsmodels api as sm

from sklearn.model_selection import train_test_split

from sklearn.metrics import mean_squared_error, r2_score

Sample Data

data = {

'GDP': [2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 5.5, 6.0],

'Interest_Rate': [3.5, 3.0, 2.5, 2.0, 1.5, 1.0, 0.5, 0.0],

"Inflation_Rate": [1.2, 1.5, 1.3, 1.0, 0.8, 0.6, 0.4, 0.2],

'Stock_Price': [100, 105, 110, 120, 130, 140, 150, 160]

}

df = pd.DataFrame(data)

Features (independent variables)

X = df[['GDP', 'Interest_Rate', 'Inflation_Rate'])

Target variable (dependent variable)

y = df['Stock_Price']

Add a constant to the model (for the intercept)

X = sm.add_constant(X)

Creating and fitting the model using statsmodels

model $=$ sm.OLS(y,X).fit()

print(model summary())

# Python Example

Mean Squared Error: 2.106914370078738

R-squared: 0.9949638648110163

Actual Predicted

0 100 99.680118   
1 105 102.268701   
2 110 111.451772   
3 120 121.953740   
4 130 131.136811   
5 140 140.319882   
6 150 149.502953   
7 160 158.686024

Making predictions

$$
y \_ p r e d = m o d e l. p r e d i c t (X)
$$

Evaluating the model

$$
\begin{array}{l} m s e = m e a n \_ s q u a r e d \_ e r r o r (y, y _ {p r e d}) \\ r 2 = r 2 \_ s c o r e (y, y \_ p r e d) \\ \end{array}
$$

Outputting the results

print("Mean Squared Error:", mse)

print("R-squared:", r2)

Display predicted vs actual values

results = pd.DataFrame({'Actual': y, 'Predicted': y_pred})  
print(results)

# Exercise

• Suppose you want to predict the stock price of an electricity company (eg. 0002.HK, 2638.HK)   
What factors can you think of? (i.e. X1, X2, X3, ...)

![](images/d683b206e5a32a20e90d5ab7c02c553fd2f04ecc005a25fc103258f0dc18d639.jpg)

![](images/3f5d738024eab2dffb74586f77cd99412d4ce0f444ccfa0616cbc2f1f32d9796.jpg)

# Exercise

1. Negative factors that could impact operating cost/supply:

- Coal price – the raw material used to produce electricity   
- Inflation Rate – impact on operational costs and pricing power   
•

2. Positive factors that could impact income/demand:

Population size   
• Temperature   
GDP grow rate   
- Gas price - alternative energy   
0

# Guidelines for choosing factors

1. Ensure factors have a logical connection to the outcome you want to predict.   
2. Use statistical tests (eg. p-values) to assess the significance of each factor.   
3. Avoid multicollinearity

- If some of the independent variables $X_{i}, X_{j}$ are highly correlated, $X$ will not be full ranked and thus $X^{T}X$ is not invertible

4. Data availability

- Many economic factors such as GDP only release every quarter/year   
- There is time lag in data release   
- May not be suitable for short term trading

5. Simplicity and Interpretability

- Choose a manageable number of factors to avoid overfitting.   
- Prioritize easily interpretable factors for better insights.   
- Rule of thumb: at least 30 observations for estimating a parameter

# Guidelines for choosing factors

# 6. Data transformation if necessary

• Y and X has no linear relationship. The coefficient may show insignificant in hypothesis test   
However, after log transformation, Y and $\log (X)$ can show a strong linear relationship

![](images/be5c65c362c3e287308fb1c615266e158941b1fdae9e9831735d8c746a1b789c.jpg)

# ARIMA Model

# Introduction to ARIMA Model

ARIMA stands for AutoRegressive Integrated Moving Average

- AR (AutoRegressive): A model that uses the dependency between an observation and a number of lagged observations (previous time steps).   
- I (Integrated): Involves differencing the observations to make the time series stationary.   
- MA (Moving Average): A model that uses dependency between an observation and a residual error from a moving average model applied to lagged observations.

# ARIMA(p,d,q) Model

- ARMA(p, q) is denoted as

$$
\begin{array}{l} y _ {t} = \phi_ {0} + \phi_ {1} y _ {t - 1} + \phi_ {2} y _ {t - 2} + \dots + \phi_ {p} y _ {t - p} + \theta_ {1} e _ {t - 1} + \theta_ {2} e _ {t - 2} + \dots + \theta_ {q} e _ {t - q} + e _ {t - q}, \\ \mathrm {o r} \qquad \left(1 - \sum_ {i = 1} ^ {p} \phi_ {i} L ^ {i}\right) y _ {t} = \phi_ {0} + \left(1 - \sum_ {i = 1} ^ {q} \theta_ {i} L ^ {i}\right) e _ {t} \\ \end{array}
$$

where

p: number of lag observations (AR part).   
d: number of differencing of the original time series (I part).   
q: the number of moving average terms (MA part)   
$\phi$ : Coefficients for the autoregressive terms   
• $\theta$ : Coefficients for the moving average terms   
- $e_t$ : Error term at time t; also called white noise with zero mean and constant variance   
- $L$ : lag operator   
$\Delta y_{t} = y_{t} - y_{t - 1}$

# Model Assumptions

1. The time series is stationary (i.e. mean, variance, and autocorrelation are constant over time).   
2. The residuals (errors) are normally distributed.   
3. The residuals are uncorrelated (no autocorrelation).

# Terminologies

• Sample moment is defined as

$$
E (X ^ {r}) = \frac {1}{n} \sum_ {i = 1} ^ {n} x _ {i} ^ {r} \qquad \mathrm {f o r r = 1 , 2 , \ldots}
$$

- Auto-covariance is defined as

$$
\gamma_ {k} = C o v (X _ {t}, X _ {t - k}) = \frac {1}{n} \sum_ {i = 1} ^ {n} \bigl (X _ {t - i} - E (X _ {t}) \bigr) \bigl (X _ {t - k - i} - E (X _ {t - k}) \bigr) \qquad \mathrm {f o r l a g k = 1 , 2 , . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .}.
$$

- Auto-correlation function (ACF) is defined as

$$
\rho_ {k} = \frac {C o v (X _ {t} , X _ {t - k})}{\sqrt {V a r (X _ {t})} \sqrt {V a r (X _ {t - k})}} \qquad \mathrm {f o r l a g k = 1 , 2 , \ldots}
$$

# Terminologies

- For a general time series $\{z_{t}\}$ , Partial Auto-correlation function (PACF) is defined as

$$
\Phi_ {1, 1} = C o r r (z _ {t + 1}, z _ {t}) \qquad \mathrm {f o r k = 1}
$$

$$
\Phi_ {k, k} = C o r r (z _ {t + k} - z _ {t + k} ^ {\prime}, z _ {t} - z _ {t} ^ {\prime})
$$

$$
\mathrm {f o r k \geq 2}
$$

$z_{t+k}^{\prime}, z_{t}^{\prime}$ are linear combinations of $\{z_{t+1}, \ldots, z_{t+k-1}\}$ that minimize MSE of $z_{t+k}, z_{t}$ respectively

- By Durbin-Levinson Algorithm,

$$
\left\{ \begin{array}{r l} & {\Phi_ {n, n} = \frac {\rho_ {n} - \sum_ {k = 1} ^ {n - 1} \Phi_ {n - 1 , k} \rho_ {n - k}}{1 - \sum_ {k = 1} ^ {n - 1} \Phi_ {n - 1 , k} \rho_ {k}}} \\ & {\Phi_ {n, k} = \Phi_ {n - 1, k} - \Phi_ {n, n} \Phi_ {n - 1, n - k} \qquad , f o r 1 \leq k \leq n - 1} \end{array} \right.
$$

# Parameter Estimation – AR model

- Consider AR(1): $y_{t} = \phi_{0} + \phi_{1} y_{t - 1} + e_{t}$   
- There are 2 parameters and need 2 equations to solve

$\mathsf{E}(y_{t}) = E(\phi_{0} + \phi_{1}y_{t - 1} + e_{t}) = \phi_{0} + \phi_{1}\mathsf{E}(y_{t - 1})$   
$\operatorname{Cov}\left(y_{t}, y_{t-1}\right)=\operatorname{Cov}\left(\phi_{0}+\phi_{1} y_{t-1}+e_{t}, y_{t-1}\right)=\phi_{1} \operatorname{Var}\left(y_{t-1}\right)$

![](images/926b29a19572602a4efb97296d91d431b2852fe30ce3f2be5191f618ae0655d4.jpg)

$$
\widehat {\phi} _ {1} = \widehat {\rho_ {1}}
$$

$$
\widehat {\phi} _ {0} = \left(1 - \widehat {\rho_ {1}}\right) \mu
$$

# Parameter Estimation – AR model

For a general AR(p) model,

$$
y _ {t} = \phi_ {0} + \phi_ {1} y _ {t - 1} + \phi_ {2} y _ {t - 2} + \dots + \phi_ {p} y _ {t - p} + e _ {t}
$$

Yule-Walker Equations

$$
\left\{ \begin{array}{c} \rho_ {1} = \phi_ {1} \rho_ {0} + \phi_ {2} \rho_ {1} + \dots + \phi_ {p} \rho_ {p - 1} \\ \rho_ {2} = \phi_ {1} \rho_ {1} + \phi_ {2} \rho_ {0} + \dots + \phi_ {p} \rho_ {p - 2} \\ \dots \\ \rho_ {p} = \phi_ {1} \rho_ {p - 1} + \phi_ {2} \rho_ {p - 2} + \dots + \phi_ {p} \rho_ {0} \end{array} \right.
$$

$$
\mathrm {F i n a l l y ,} \qquad \phi_ {0} = (1 - \phi_ {1} - \phi_ {2} - \dots - \phi_ {p}) \mu
$$

![](images/4838f4881f262dfbe9568e1c0fa60d39dc926a019a31bfb2030aa28504c86642.jpg)

Solving the system of linear equations, we can get $\phi_1, \phi_2, \dots, \phi_p$

# Parameter Estimation – MA model

- Consider MA(1): $y_{t} = \theta_{1} e_{t - 1} + e_{t}$

$\operatorname{Var}(y_t) = \operatorname{Cov}(y_t, y_t) = \operatorname{Cov}(\theta_1 e_{t-1} + e_t, \theta_1 e_{t-1} + e_t) = (\theta_1^2 + 1) \operatorname{Var}(e_t)$   
$\operatorname{Cov}(y_t, y_{t-1}) = \operatorname{Cov}(\theta_1 e_{t-1} + e_t, y_{t-1}) = \theta_1 \operatorname{Cov}(e_{t-1}, y_{t-1}) = \theta_1 \operatorname{Cov}(e_{t-1}, \theta_1 e_{t-2} + e_{t-1}) = \theta_1 \operatorname{Var}(e_t)$

- We got a quadratic equation in $\theta_{1}$

$$
\rho_ {1} = \frac {\mathrm {C o v} (y _ {t} , y _ {t - 1})}{\mathrm {V a r} (y _ {t})} = \frac {\theta_ {1}}{\theta_ {1} ^ {2} + 1}
$$

- In general $\mathrm{MA(q)}$ model, the parameters for $\{\theta_k\}$ are non linear which could only be solved numerically

# Unit Root

• For ARMA(p,q),

$$
\left(1 - \sum_ {i = 1} ^ {p} \phi_ {i} L ^ {i}\right) y _ {t} = \phi_ {0} + \left(1 - \sum_ {i = 1} ^ {q} \theta_ {i} L ^ {i}\right) e _ {t}
$$

- A unit root is present if the polynomial $(1 - \phi_1 x - \phi_2 x^2 - \dots - \phi_p x^p)$ has a root equal to 1   
- When unit root exists, the time series is highly persistent over time, meaning that shocks or changes to the series have a long-lasting effect, and autocorrelation decays to zero very slowly.   
- A time series is non-stationary if it contains a unit root but the reverse is not true

# Unit Root

- Consider the AR(1) process

$$
y _ {t} = \phi_ {0} + \phi_ {1} y _ {t - 1} + e _ {t}
$$

- It has unit root if $\phi_1 = 1$   
- In this case,

$$
y _ {t} = \phi_ {0} t + y _ {0} + e _ {1} + e _ {2} + \dots + e _ {t}
$$

- So the mean will be a function of $t$ , and hence the time series is non-stationary

$$
E (y _ {t}) = \phi_ {0} t + y _ {0}
$$

# Random Walk Process

- Consider the random walk process which is a special case of AR(1) model

$$
y _ {t} = y _ {t - 1} + e _ {t}
$$

In this case,

$$
y _ {t} = y _ {0} + e _ {1} + e _ {2} + \dots + e _ {t}
$$

- The mean is constant but the variance depends on time, and hence non-stationary

$$
\left\{ \begin{array}{r l} & E (y _ {t}) = y _ {0} \\ & V a r (y _ {t}) = V a r (e _ {1}) * t \end{array} \right.
$$

# Augmented Dickey Fuller (ADF) Test

- If a price series is mean reverting, then the current price level will tell us something about what the price's next move will be:

- If the price level is higher than the mean, the next move will be a downward move;   
- if the price level is lower than the mean, the next move will be an upward move.

- The ADF test is based on this observation. We can describe the price changes using a linear model:

$$
\Delta y _ {t} = \lambda y _ {t - 1} + \mu + \beta t + \alpha_ {1} \Delta y _ {t - 1} + \dots + \alpha_ {k} \Delta y _ {t - k} + \varepsilon_ {t}
$$

# Augmented Dickey Fuller (ADF) Test

- When $\lambda = 0$ , the equation becomes

$$
\Delta y _ {t} = \mu + \beta t + \alpha_ {1} \Delta y _ {t - 1} + \dots + \alpha_ {k} \Delta y _ {t - k} + \varepsilon_ {t}
$$

- This indicates that the change in the series $\Delta y_{t}$ is not dependent on its previous value $y_{t - 1}$   
- In other words, past values do not predict future changes, which suggests that shocks to the series have permanent effects. This is characteristic of a unit root process (non-stationary).

- The ADF test will find out if there is unit root (i.e. $H_0$ : $\lambda = 0$ ) by looking at the test statistics $\lambda / \sigma_{\lambda}$

- If $\lambda / \sigma_{\lambda}$ is significantly negative, the time series tends to be mean reverting   
- If $\lambda / \sigma_{\lambda}$ is significantly positive, the time series tends to be trending

# Stationary

- ACF and PACF assume stationarity of the underlying time series. Stationarity can be checked by performing an ADF test:

- p-value $>0.05$ : Fail to reject the null hypothesis (H0), the data has a unit root and is non-stationary.   
- p-value <= 0.05: Reject the null hypothesis (H0), the data does not have a unit root and is stationary

from statsmodels.tsa.stattools import adfuller

def check_stationarity(data):

result = adfuller(data)

print('ADF Statistic: %f % result[0])

print('p-value: %f % result[1])

print('Critical Values:')

for key, value in result[4].items():

print('t%s:%.3f % (key, value))

if (result[1] <= 0.05):

print("Stationary")

else:

print("Non-stationary")

import random

data = [random.random() for i in range(0, 100)]

check_stationarity(data)

ADF Statistic: -8.888879

p-value: 0.000000

Critical Values:

1%: -3.498

5%：-2.891

10%: -2.583

Stationary

# How to determine $(p, d, q)$ ?

- The order $p$ and $q$ can be visualized and determined using ACF and PACF charts

<table><tr><td>Example</td><td>PACF</td></tr><tr><td>White noise</td><td>The partial autocorrelation is 0 for all lags.</td></tr><tr><td>Autoregressive model</td><td>The partial autocorrelation for an AR(p) model is nonzero for lags less than or equal to p and 0 for lags greater than p.</td></tr><tr><td>Moving-average model</td><td>• If φ1,1 &gt; 0, the partial autocorrelation oscillates to 0.
• If φ1,1 &lt; 0, the partial autocorrelation geometrically decays to 0.</td></tr><tr><td>Autoregressive-moving-average model</td><td>An ARMA(p, q) model&#x27;s partial autocorrelation geometrically decays to 0 but only after lags greater than p.</td></tr></table>

# PACF graph for AR(1)

![](images/e76019f4d88dc6671df599f91166efc0c08b36bdcea0fbd47eb2b54ab684723b.jpg)

# General rules to determine $(p, d, q)$

<table><tr><td>Rules</td><td>Identifying the order of differencing and the constant</td></tr><tr><td>1</td><td>If the series has positive autocorrelations out to a high number of lags (say, 10 or more), then it probably needs a higher order of differencing</td></tr><tr><td>2</td><td>If the lag-1 autocorrelation is zero or negative, or the autocorrelations are all small and pattern-less, then the series does not need a higher order of differencing. If the lag-1 autocorrelation is -0.5 or more negative, the series may be over-differenced</td></tr><tr><td>3</td><td>The optimal order of differencing is often the order of differencing at which the standard deviation is lowest. (Not always, though. Slightly too much or slightly too little differencing can also be corrected with AR or MA terms. See rules 6 and 7.)</td></tr><tr><td>4</td><td>A model with no orders of differencing assumes that the original series is stationary. A model with one order of differencing assumes that the original series has a constant average trend. A model with two orders of total differencing assumes that the original series has a time-varying trend.</td></tr><tr><td>5</td><td>A model with no orders of differencing normally includes a constant term (which allows for a non-zero mean value). A model with two orders of total differencing normally does not include a constant term. In a model with one order of total differencing, a constant term should be included if the series has a non-zero average trend.</td></tr></table>

# General rules to determine $(p, d, q)$

<table><tr><td>Rules</td><td>Identifying the numbers of AR and MA terms:</td></tr><tr><td>6</td><td>If PACF of the differenced series displays a sharp cutoff and/or the lag-1 autocorrelation is positive, then consider adding one or more AR terms to the model. The lag beyond which the PACF cuts off is the indicated number of AR terms.</td></tr><tr><td>7</td><td>If ACF of the differenced series displays a sharp cutoff and/or the lag-1 autocorrelation is negative, then consider adding an MA term to the model. The lag beyond which the ACF cuts off is the indicated number of MA terms.</td></tr><tr><td>8</td><td>It is possible for an AR term and an MA term to cancel each other&#x27;s effects, so if a mixed AR-MA model seems to fit the data, also try a model with one fewer AR term and one fewer MA term (particularly if the parameter estimates in the original model require more than 10 iterations to converge). BEWARE OF USING MULTIPLE AR TERMS AND MULTIPLE MA TERMS IN THE SAME MODEL.</td></tr><tr><td>9</td><td>If there is a unit root in the AR part of the model (i.e. if the sum of the AR coefficients is almost exactly 1), you should reduce the number of AR terms by one and increase the order of differencing by one.</td></tr><tr><td>10</td><td>If there is a unit root in the MA part of the model (i.e. if the sum of the MA coefficients is almost exactly 1), you should reduce the number of MA terms by one and reduce the order of differencing by one.</td></tr><tr><td>11</td><td>If the long-term forecasts appear unstable, there may be a unit root in the AR or MA coefficients</td></tr></table>

# General rules to determine $(p, d, q)$

<table><tr><td>Rules</td><td>Identifying the seasonal part of the model:</td></tr><tr><td>12</td><td>If the series has a strong and consistent seasonal pattern, then you must use an order of seasonal differencing (otherwise the model assumes that the seasonal pattern will fade away over time). However, never use more than one order of seasonal differencing or more than 2 orders of total differencing (seasonal + nonseasonal).</td></tr><tr><td>13</td><td>If the autocorrelation of the appropriately differenced series is positive at lag s, where s is the number of periods in a season, then consider adding an SAR term to the model. If the autocorrelation of the differenced series is negative at lag s, consider adding an SMA term to the model. The latter situation is likely to occur if a seasonal difference has been used, which should be done if the data has a stable and logical seasonal pattern. The former is likely to occur if a seasonal difference has not been used, which would only be appropriate if the seasonal pattern is not stable over time. You should try to avoid using more than one or two seasonal parameters (SAR+SMA) in the same model, as this is likely to lead to overfitting of the data and/or problems in estimation.</td></tr></table>

# What are the $(p, d, q)$ in this example?

![](images/a1ab3215f0d6f4392a6c0a0116b11763cba12dff56ad484cef2ad5a9b5facf5a.jpg)  
ACF - data   
PACF - data

![](images/e73c4e92b966081020d5f4c8e7d6071e08ac27d66781d175e270ef00dace1e3c.jpg)

![](images/51e846dbe98c94c83e82539d8516ada60ce5a168ea05d37345cf8fb581ea8574.jpg)  
ACF - Differenced data   
PACF - Differenced data

![](images/691b0ba24e956ce8ea893bfa1dff3e03d8b9640c070016fca18d0df7286ecebd.jpg)

# What are the $(p, d, q)$ in this example?

# Possible choices

Non-seasonal:

(1, 1, 0) or (2, 0, 0)   
(0, 1, 1) or (0, 0, 2)   
(1, 1, 1)

- Seasonal difference at lag 24

(1, 1, 0)   
(0, 1, 1)   
(1, 1, 1)

# Python Implementation

import yfinance as yf

import pandas as pd

import numpy as np

import matplotlib.pyplot as plt

from statsmodels.tsa.arima.model import ARIMA

from statsmodels.graphics.tsaplots import plot_acf, plot_pacf

plt.style.use('darkbackground')

# Fetch historical data for Microsoft (MSFT)

data = yf.download('MSFT', start='2023-01-01', end='2023-12-31')

data = data['Close']

Visualize the data

plt.figure(figsize=(10,6))

plt.plot(data)

plt.title('Time Series Data')

plt show()

Plot ACF and PACF to determine p and q

plot_acf(data)

plot_pacf(data)

plt show()

# differencing

data_diff = data(diff().dropna()

plot_acf(data_diff)

plot_pacf(data_diff)

plt.show()

![](images/5ab197745839be5d210f6104bc7101e3000d8b7f26bb602ef9cac1a84e55a048.jpg)  
d=0   
d=1

![](images/cfc173d15fbb49c64c8235f3cf6de9235ac9081ca4824f40c5eda8776297ced4.jpg)

![](images/0395ae47a47d6fb91cfc68acdcfe130a452d1d36eba827fd256b6a76d9ce14eb.jpg)

![](images/1a3a0a10228c6ccd4618095e05aeed2f1a44abf2fd8295809b544fb8bfd38a0f.jpg)

# Python Implementation

$\mathrm{p} = 1$

$\mathrm{d} = 0$

$q = 1$

Fit ARIMA model

model = ARIMA(data, order=(p, d, q))

model_fit = model.fit()

Summary of the model

print(model_fit.summary())

Forecasting

forecast = model_fit.forecast(steps=10)

print( forecast)

<table><tr><td colspan="7">SARIMAX Results</td></tr><tr><td>Dep. Variable:</td><td colspan="2">Close_MSFT</td><td colspan="2">No. Observations:</td><td colspan="2">250</td></tr><tr><td>Model:</td><td colspan="2">ARIMA(1, 0, 1)</td><td colspan="2">Log Likelihood</td><td colspan="2">-743.796</td></tr><tr><td>Date:</td><td colspan="2">Thu, 05 Feb 2026</td><td colspan="2">AIC</td><td colspan="2">1495.592</td></tr><tr><td>Time:</td><td colspan="2">22:46:24</td><td colspan="2">BIC</td><td colspan="2">1509.678</td></tr><tr><td>Sample:</td><td colspan="2">0</td><td colspan="2">HQIC</td><td colspan="2">1501.261</td></tr><tr><td>Covariance Type:</td><td colspan="2">-250</td><td></td><td></td><td></td><td></td></tr><tr><td>Coeff</td><td>std err</td><td>z</td><td>P&gt;|z|</td><td colspan="3">[0.025, 0.975]</td></tr><tr><td>const</td><td>308.0829</td><td>50.418</td><td>6.111</td><td>0.000</td><td>209.265</td><td>406.900</td></tr><tr><td>ar.L1</td><td>0.9970</td><td>0.007</td><td>143.205</td><td>0.000</td><td>0.983</td><td>1.011</td></tr><tr><td>ma.L1</td><td>-0.0427</td><td>0.056</td><td>-0.761</td><td>0.446</td><td>-0.153</td><td>0.067</td></tr><tr><td>sigma2</td><td>22.0274</td><td>1.710</td><td>12.885</td><td>0.000</td><td>18.677</td><td>25.378</td></tr><tr><td colspan="2">Ljung-Box (L1) (Q):</td><td>0.00</td><td colspan="2">Jarque-Bera (JB):</td><td colspan="2">7.39</td></tr><tr><td colspan="2">Prob(Q):</td><td>0.99</td><td colspan="2">Prob(JB):</td><td colspan="2">0.02</td></tr><tr><td colspan="2">Heteroskedasticity (H):</td><td>0.71</td><td colspan="2">Skew:</td><td colspan="2">0.19</td></tr><tr><td colspan="2">Prob(H) (two-sided):</td><td>0.13</td><td colspan="2">Kurtosis:</td><td colspan="2">3.75</td></tr><tr><td colspan="7">Warnings:</td></tr><tr><td colspan="7">[1] Covariance matrix calculated using the outer product of gradients (complex-step).</td></tr><tr><td>250</td><td colspan="2">370.289148</td><td></td><td></td><td></td><td></td></tr><tr><td>251</td><td colspan="2">370.105472</td><td></td><td></td><td></td><td></td></tr><tr><td>252</td><td colspan="2">369.922339</td><td></td><td></td><td></td><td></td></tr><tr><td>253</td><td colspan="2">369.739747</td><td></td><td></td><td></td><td></td></tr><tr><td>254</td><td colspan="2">369.557694</td><td></td><td></td><td></td><td></td></tr><tr><td>255</td><td colspan="2">369.376179</td><td></td><td></td><td></td><td></td></tr><tr><td>256</td><td colspan="2">369.195199</td><td></td><td></td><td></td><td></td></tr><tr><td>257</td><td colspan="2">369.014754</td><td></td><td></td><td></td><td></td></tr><tr><td>258</td><td colspan="2">368.834842</td><td></td><td></td><td></td><td></td></tr><tr><td>259</td><td colspan="2">368.655460</td><td></td><td></td><td></td><td></td></tr><tr><td colspan="7">Name: predicted_mean, dtype: float64</td></tr></table>

# ARIMA Trading Strategy

- Construct a stationary time series and fit an ARIMA model   
Strategy 1:

Take ARIMA model's predicted values as the fair/reference market price.   
- If current market price is much lower than the fair price, open LONG position   
- If current market price is much higher than the fair price, open SHORT position

Strategy 2:

- For AR(p) model, project for the future prices   
- If the future price is projected to be higher than the current price, open LONG position   
- If the future price is projected to be lower than the current price, open SHORT position

# Backtest ARIMA Strategy

from AlgoAPI import AlgoAPIUtil, AlgoAPI_Backtest

import pandas as pd

from statsmodels.tsa.arima.model import ARIMA

- Instrument: MSFT   
Initial capital: 10k USD   
Period: 2023.01.01 - 2023.12.31   
- Data Interval: 1 day   
- Short-selling: No

class AlgoEvent:

def __init__(self):

self.p,self.d,self.q=1,0,1

def start(self, mEvt):

self.evt = AlgoAPI_Backtest.AlgoEvtHandler(self, mEvt)

self.evt.start()

def on_marketdatafeed(self, md, ab):

get previous 100 days closing price

obs = self.evt.getHistoricalBar contract={'instrument":"MSFT"}, numOfBar=100, interval="D", timestamp=md.timestamp)

arr_close = [obs[t][c'] for t in obs]

put data into dataframe

data = pd.DataFrame(arr_close)

fit ARIMA model

model = ARIMA(data, order=(self.p, self.d, self.q))

model_fit = model.fit()

5 day forecast

forecast $=$ model_fit forecast(steps=5)

future_price = forecast.iloc[-1]

self.evt consoleLog( forecast)

open buy order if future price is higher than current price

if future_price>md.askPrice:

order $=$ AlgoAPIUtil.OrderObject(

instrument = md.instrument,

openclose $=$ 'open',

buysell = 1, #buy

ordertype $= 0$ #market order

volume $= 1$ #1 lot

holdtime $= 24^{*}60^{*}60^{*}5$ #holding period for 5 days

）

self.evt.sendOrder(order)

# Backtest ARIMA Strategy

Monthly Return:   

<table><tr><td></td><td>Year</td><td>Jan</td><td>Feb</td><td>Mar</td><td>Apr</td><td>May</td><td>Jun</td><td>Jul</td><td>Aug</td><td>Sep</td><td>Oct</td><td>Nov</td><td>Dec</td><td>YTD</td></tr><tr><td></td><td>2023</td><td>0.16%</td><td>0.0%</td><td>0.0%</td><td>0.0%</td><td>0.0%</td><td>0.0%</td><td>0.0%</td><td>0.09%</td><td>0.05%</td><td>0.81%</td><td>0.24%</td><td>0.0%</td><td>1.35%</td></tr><tr><td colspan="15">Performance Statistics:</td></tr><tr><td colspan="4">No. of Tradable Days:</td><td>272</td><td colspan="4">No. of Win Days:</td><td>30</td><td colspan="4">No. of Loss Days:</td><td>32</td></tr><tr><td colspan="4">Win Rate:</td><td>48.3871%</td><td colspan="4">Max. Consecutive Win Day:</td><td>8</td><td colspan="4">Max. Consecutive Loss Day:</td><td>6</td></tr><tr><td colspan="4">Odd Ratio:</td><td>0.9375</td><td colspan="4">Max. Consecutive Gains:</td><td>55.48</td><td colspan="4">Max. Consecutive Loss:</td><td>-42.67</td></tr><tr><td colspan="4">No. of Trades:</td><td>82</td><td colspan="4">Average Consecutive Win Day:</td><td>0.54</td><td colspan="4">Average Consecutive Loss Day:</td><td>0.25</td></tr><tr><td colspan="4">Total PnL:</td><td>133.94</td><td colspan="4">Average Per Trade PnL:</td><td>1.63</td><td colspan="4">Average Per Day PnL:</td><td>0.49</td></tr><tr><td colspan="4">Mean Daily Return:</td><td>0.0049%</td><td colspan="4">Median Daily Return:</td><td>0.0%</td><td colspan="4">Mean Annual Return:</td><td>1.2426%</td></tr><tr><td colspan="4">Daily Return StdDev:</td><td>0.0649%</td><td colspan="4">25th percentile Daily Return:</td><td>0.0%</td><td colspan="4">75th percentile Daily Return:</td><td>0.0%</td></tr><tr><td colspan="4">Daily Return Downside StdDev:</td><td>0.0335%</td><td colspan="4">95% 1 day return VaR:</td><td>-0.0423%</td><td colspan="4">99% 1 day return VaR:</td><td>-0.1868%</td></tr><tr><td colspan="4">Daily Sharpe Ratio:</td><td>0.076</td><td colspan="4">Annual Sharpe Ratio:</td><td>1.2065</td><td colspan="4">Max. Drawdown Amount:</td><td>42.67</td></tr><tr><td colspan="4">Daily Sortino Ratio:</td><td>0.1471</td><td colspan="4">Annual Sortino Ratio:</td><td>2.3352</td><td colspan="4">Max. Drawdown Percent:</td><td>0.4267%</td></tr><tr><td colspan="4">Max. Drawdown Duration:</td><td>160</td><td colspan="4">Average Drawdown Duration:</td><td>47.95</td><td colspan="4">Annual Volatility:</td><td>1.0258%</td></tr><tr><td colspan="4">Gross Profit:</td><td>213.495</td><td colspan="4">Gross Loss:</td><td>-79.554</td><td colspan="4">Profit Factor:</td><td>2.6836</td></tr><tr><td colspan="4">Jensen Alpha:</td><td>0.0</td><td colspan="4">Beta:</td><td>0.0</td><td colspan="4">Information Ratio:</td><td>0.0</td></tr><tr><td colspan="4">Omega Ratio:</td><td>0.0</td><td colspan="4">Tregnor Ratio:</td><td>0.0</td><td colspan="4">Tail Ratio:</td><td>1.7807</td></tr><tr><td colspan="4">Calmar Ratio:</td><td>2.9123</td><td colspan="4">Average Holding Day:</td><td>5.2439</td><td colspan="4">Annual Turnover Rate:</td><td>110.6951%</td></tr></table>

# Hurst Exponent

# Hurst Exponent

- The speed of diffusion can be characterized by the variance

$$
V a r (\tau) = \langle | z (t + \tau) - z (t) | ^ {2} \rangle
$$

where

• $z = \log (y)$ is the log price   
- $\tau$ is an arbitrary time lag   
• $\langle | \dots |\rangle$ is an average over all t

- For a geometric random walk, we know the variance is proportional to $\tau$

$$
\langle | z (t + \tau) - z (t) | ^ {2} \rangle \sim \tau
$$

- This equation won't hold for mean reverting or trending series.

# Hurst Exponent

- We can re-write it as

$$
\langle | z (t + \tau) - z (t) | ^ {2} \rangle \sim \tau^ {2 H}
$$

- This is the definition of Hurst Exponent. The series exhibit as

- geometric random walk If $H = 0.5$   
- mean reverting if $H < 0.5$   
trending if $H > 0.5$

- H can be an indicator for the degree of mean reverting or trending

- If $H$ is close to 0, it will be more mean reverting   
- If $H$ is close to 1, it will be more trending

# Variance Ratio Test

Hypothesis test:

H0: $H = 0.5$ (i.e. random walk)   
H1: $H \neq 0.5$

- It simply tests whether below equal to 1.

$$
\frac {V a r \big (z (t) - z (t - \tau) \big)}{\tau V a r \big (z (t) - z (t - 1) \big)} \sim \tau^ {2 H - 1}
$$

import numpy as np from arch-unitroot import VarianceRatio

Generate some sample data  
np.random.seed(42)  
data = np cumsum(np.random.randint(200)) + 100

Perform the Variance Ratio test  
vr_test = VarianceRatio(y=data, lags=2)  
# Print the test results  
print(vr_test.summary())

# Mean Reversion Process

- In practice, it is important to know how quick a time series revert to its mean   
- Let's consider the Ornstein Uhlenbeck formula for a mean reverting process

$$
\mathrm {d} y _ {t} = \kappa (\theta - y _ {t}) d t + \sigma d \varepsilon
$$

- $\kappa > 0$ is the rate of mean reversion   
$\theta$ is the long terms mean   
- $\sigma > 0$ is the volatility of the Brownian process

- Solution: for any $0 < s < t$ ,

$$
y _ {t} = \theta + (y _ {s} - \theta) e ^ {- \kappa (t - s)} + \sigma \int_ {s} ^ {t} e ^ {- \kappa (t - u)} d W _ {u}
$$

$$
E (y _ {t}) = \theta + (y _ {s} - \theta) e ^ {- \kappa (t - s)}
$$

$$
\operatorname {V a r} (y _ {t}) = \frac {\sigma^ {2}}{2 \kappa} (1 - e ^ {- 2 \kappa (t - s)})
$$

# Half Life of Mean Reversion

Note that the expected value delay exponentially to $\theta$

$$
E (y _ {t}) = \theta + (y _ {s} - \theta) e ^ {- \kappa (t - s)}
$$

Consider

$$
\frac {1}{2} (y _ {0} - \theta) = (y _ {0} - \theta) e ^ {- \kappa t}
$$

Half life will be

$$
t = \frac {\log (2)}{\kappa}
$$

# Parameter Estimation

- Consider the Ornstein Uhlenbeck in a discrete form

$$
\mathrm {d} y _ {t} = \kappa (\theta - y _ {t}) d t + \sigma d \varepsilon
$$

$$
\Delta y _ {t} = \kappa \theta - \kappa y _ {t - 1} + \varepsilon_ {t}
$$

- We can fit a linear regression model $\Delta y_{t}$ against $y_{t - 1}$ where $-\kappa$ is the slope and $\kappa \theta$ is the constant term

# How to discover Mean Reversion?

- We can base on Hurst Exponent and Variance ratio test to determine whether a time series is mean reverting   
- The connection of the coefficient $\kappa$ and half life is also very useful

- If $\kappa$ is estimated to be negative, that means the time series is not mean reverting at all and we shouldn’t attempt to apply a mean reversion strategy to trade   
- If $\kappa$ is very close to zero, that means the half life is very long and a mean reverting strategy will not be very profitable as we can't complete many round trades in a given period   
- If $\kappa$ is a natural time scale. For example, if the half life is 20 days, we shouldn't use a lookback of 5 days to compute a moving average or standard deviation for a mean-reversion strategy. Often, setting the lookback to equal a small multiple of the half-life is close to optimal, and doing so will allow us to avoid brute-force optimization of a free parameter based on the performance of a trading strategy

# How to discover Momentum?

- Momentum means that past returns are positively correlated with future returns. Thus we can just calculate the correlation coefficient of the returns together with its p-value   
- Sometimes the most positive correlations are between returns of different lags. For example, 1-day returns might show negative correlations, while the correlation between past 20-day return with the future 40-day return might be very positive. We should find the optimal pair of past and future periods that gives the highest positive correlation and use that as our look-back and holding period for our momentum strategy.   
- We can test for the correlations between the signs of past and future returns. This is appropriate when all we want to know is that an up move will be followed by another up move, and we don’t care whether the magnitudes of the moves are similar   
- If we are interested instead in finding out whether there is long-term trending behavior without regard to specific time frames, we can calculate the Hurst exponent together with the Variance Ratio test to rule out the null hypothesis of a random walk

# Backtest Mean Reversion Strategy

- Fit a linear regression model according to the discrete form of Ornstein Uhlenbeck formula   
- Check model fitness based on $R^2$ (for example, assume the model is good if $R^2 > 0.5$ )   
• Compute the theoretical half-life $t_{0.5} = \log (2) / \kappa$   
- We can assume it is a proper mean reversion period if $t_{0.5}$ is between 1 to 10   
Take the regression's predicted values as the fair/reference market price

- If current market price is much lower than the fair price, open LONG position   
- If current market price is much higher than the fair price, open SHORT position

- Also set holding period to $t_{0.5}$

# Backtest Mean Reversion Strategy

- Instrument: MSFT   
- Initial capital: 10k USD   
Period: 2023.01.01 - 2023.12.31   
- Data Interval: 1 day   
- Short-selling : No

```python
from AlgoAPI import AlgoAPIUtil, AlgoAPI_Backtest  
import pandas as pd  
import numpy as np  
from sklearn.linear_model import LinearRegression  
from sklearn.metrics import mean_squared_error, r2  
from math import log 
```

```python
class AlgoEvent: def __init__(self): pass def start(self, mEvt): self.evt = AlgoAPI_Backtest.AlgoEvtHandler(self, mEvt) self.evt.start() 
```

def on_marketdatafeed(self, md, ab): # get previous 100 days closing price obs = self.evt.getHistoricalBar contract={'instrument":"MSFT", numOfBar=10, interval="D", timestamp=md.timestamp) close $=$ np.array([obs[t][c'] for t in obs]) #Y(t)-Y(t-1) $= a + b^{*}Y(t - 1)$ Y $=$ close[1:] - close[-1] X $=$ close[-1] #Reshape for sklearn which expects a 2D array for features $\mathbf{X} = \mathbf{X}$ reshape(-1, 1) # fit ARIMA model model $=$ LinearRegression() model fit(X, Y) #evaluate model effectiveness Y_pred $=$ model.predict(X) msec $=$ mean_squared_error(Y, Y_pred) r2 $=$ r2_score(Y, Y_pred) self.evt consoleLog('mse, r-square $=$ ' , msec, r2) # skip entry if e-square too small if r2<0.5: return # compute half life b0 $=$ model.intercept_ b1 $=$ model.coef_[0] tHALflife $=$ -log(2)/b1 self.evt consoleLog('half-life $=$ ' , tHALflife) #skip entry if half life is negative or too large if t_halflife $<= 1$ or t_halflife>10: return # open buy order if current ask price is lower than model price if md.askPrice $<  <   Y_{\cdot}$ pred[-1]+close[-1]: order $=$ AlgoAPIUtil.OrderObject( instrument $=$ md.instrument, openclose $=$ 'open', buysell $= 1$ #buy ordertype $= 0$ #market order volume $= 1$ #0.01 lot holdtime $= 24^{*}60^{*}60^{*}t$ halflife #unit in second ) self.evt.sendOrder(order)

# Backtest Mean Reversion Strategy

Monthly Return:   

<table><tr><td>Year</td><td>Jan</td><td>Feb</td><td>Mar</td><td>Apr</td><td>May</td><td>Jun</td><td>Jul</td><td>Aug</td><td>Sep</td><td>Oct</td><td>Nov</td><td>Dec</td><td>YTD</td></tr><tr><td>2023</td><td>0.0%</td><td>-0.02%</td><td>0.04%</td><td>0.0%</td><td>0.0%</td><td>0.04%</td><td>0.0%</td><td>0.0%</td><td>0.0%</td><td>0.0%</td><td>0.0%</td><td>0.0%</td><td>0.06%</td></tr></table>

Performance Statistics: ①   

<table><tr><td>No. of Tradable Days:</td><td>272</td><td>No. of Win Days:</td><td>3</td><td>No. of Loss Days:</td><td>6</td></tr><tr><td>Win Rate:</td><td>33.3333%</td><td>Max. Consecutive Win Day:</td><td>2</td><td>Max. Consecutive Loss Day:</td><td>4</td></tr><tr><td>Odd Ratio:</td><td>0.5</td><td>Max. Consecutive Gains:</td><td>5.83</td><td>Max. Consecutive Loss:</td><td>-4.04</td></tr><tr><td>No. of Trades:</td><td>6</td><td>Average Consecutive Win Day:</td><td>1.33</td><td>Average Consecutive Loss Day:</td><td>0.06</td></tr><tr><td>Total PnL:</td><td>5.85</td><td>Average Per Trade PnL:</td><td>0.98</td><td>Average Per Day PnL:</td><td>0.02</td></tr><tr><td>Mean Daily Return:</td><td>0.0003%</td><td>Median Daily Return:</td><td>0.0%</td><td>Mean Annual Return:</td><td>0.065%</td></tr><tr><td>Daily Return StdDev:</td><td>0.005%</td><td>25th percentile Daily Return:</td><td>0.0%</td><td>75th percentile Daily Return:</td><td>0.0%</td></tr><tr><td>Daily Return Downside StdDev:</td><td>0.0019%</td><td>95% 1 day return VaR:</td><td>0.0%</td><td>99% 1 day return VaR:</td><td>-0.0076%</td></tr><tr><td>Daily Sharpe Ratio:</td><td>0.0516</td><td>Annual Sharpe Ratio:</td><td>0.819</td><td>Max. Drawdown Amount:</td><td>4.04</td></tr><tr><td>Daily Sortino Ratio:</td><td>0.1377</td><td>Annual Sortino Ratio:</td><td>2.1853</td><td>Max. Drawdown Percent:</td><td>0.0404%</td></tr><tr><td>Max. Drawdown Duration:</td><td>4</td><td>Average Drawdown Duration:</td><td>0.06</td><td>Annual Volatility:</td><td>0.0791%</td></tr><tr><td>Gross Profit:</td><td>9.878</td><td>Gross Loss:</td><td>-4.023</td><td>Profit Factor:</td><td>2.4554</td></tr><tr><td>Jensen Alpha:</td><td>0.0</td><td>Beta:</td><td>0.0</td><td>Information Ratio:</td><td>0.0</td></tr><tr><td>Omega Ratio:</td><td>0.0</td><td>Treynor Ratio:</td><td>0.0</td><td>Tail Ratio:</td><td>0.0</td></tr><tr><td>Calmar Ratio:</td><td>1.608</td><td>Average Holding Day:</td><td>2.6667</td><td>Annual Turnover Rate:</td><td>7.7609%</td></tr></table>

# Martingale Strategy

For random walk market

# Introduction to Martingale Strategy

It is a betting system that originated in 18th century France and was popularized in the 19th century.   
- The system was named after a casino owner in London named John Henry Martindale, who encouraged players to use the strategy to win at his casino.

![](images/7410fffcabc1a0b4c107d5b44b86d1ab890d6804fc764b67ac8c785ee9324538.jpg)

# Casino Example

# - Betting process:

- The player places a $1 bet on red. If the ball lands on black, he lose the bet.   
- The player doubles their bet to $2 and places it on red again. If the ball lands on black again, he lose$ 2.   
- The player doubles their bet to $4 and places it on red again. If the ball lands on black again, he lose$ 4.   
• The player doubles their bet to $8 and places it on red again. If the ball lands on red this time, he win $8 and have recouped the previous losses and made a $1 profit.

<table><tr><td>Bet</td><td>Outcome</td><td>Total Profit/Loss</td></tr><tr><td>$1</td><td>Loss</td><td>-$1</td></tr><tr><td>$2</td><td>Loss</td><td>-$3</td></tr><tr><td>$4</td><td>Loss</td><td>-$7</td></tr><tr><td>$8</td><td>Win</td><td>+$1</td></tr></table>

# The mathematics

- Assume initial bet size is \(1, the betting sequence will be

$$
\{1, 2, 4, 8, 1 6, \dots , 2 ^ {\wedge} n \}
$$

- Suppose the player loses for $n$ consecutive rounds and win at $n + 1$ round,

• Sum of all previous losses $= 1 + 2 + 4 + \ldots + 2^{\Lambda} (n - 1) = 2^{\Lambda} n - 1$   
- Win at $(n + 1)$ th round $= 2^{\wedge} n$   
So the total profit/loss will be $+1$

- The probability of losing n consecutive rounds is $0.5^{\wedge} n$ . The probability of losing 10 consecutive rounds is only 0.000976 which means that this strategy is highly likely to succeed in short term

# Strategy Assumptions & Limitations

• Only 2 possible outcomes (i.e. win or lose)   
- The probability of win and lose are equal   
- Each round's outcome is independent   
It requires unlimited capital in long run

# Martingale Strategy Example

- Instrument: XAUUSD   
- Backtest period: 2023.01 - 2023.12   
Initial Capital: US$50k   
- Open 0.01 lot buy order every time (for XAUUSD, 1 lot = 100 shares)   
- Set TP and SL to be $10

from AlgoAPI import AlgoAPIUtil, AlgoAPI_Backtest

# class AlgoEvent:

def __init__(self):

self.bet_amt = 10

self bet_size_base = 0.01

self.bet_size = self.bet_size_base

self.entry_price = None

def start(self, mEvt):

self.evt = AlgoAPI_Backtest.AlgoEvtHandler(self, mEvt)  
self.evt.start()

def on_marketdatafeed(self, md, ab):

open new trade if no position

pos, $-, - =$ self.evt.getSystemOrders()

if pos[md.instrument]["netVolume"] == 0:

order = AlgoAPIUtil.OrderObject(

instrument = md.instrument,

openclose = 'open',

buysell = 1, # buy

ordertype $= 0$ # market order

volume = self.bet_size,

takeProfitLevel = md.askPrice + self.bet_amt,

stopLossLevel = md.askPrice - self.bet_amt

）

self EVT.sendOrder(order)

def on_orderfeed(self, of):

if of status $= =$ "success":

if of.openclose $= =$ "open":

self.entry_price = of(fill_price

elif of.openclose $= =$ "close":

reset to base size if win

if of.fill_price > self.entry_price:

self.bet_size = self.bet_size_base

double bet size if lose

else:

self.bet_size\*=2

# Martingale Strategy Example

- This strategy is effective in making stable profit

![](images/c62ccf28e49acdbb3086a7faaa8df2f45c8d2d1b0efd0fa2442a8ed2c3d01c0a.jpg)

# Martingale Strategy Example

- But need to carefully manage the capital in case of consecutive losses

![](images/cbcf4d68c43fe3a72f8357c1d7c5624af33f05afac0108b95ab1446a49057717.jpg)  
Capital Usage:

# Ways to enhance Martingale Strategy

- Entry only when the current market is detected to be random   
- Rather than always opening a buy order,

- Enter long position if we detect a short term up trend   
- Enter short position if we detect a short term down trend

- TL/SL size adjustable according to market liquidity

# Key Takeaways

- Market is generally classified into

Trending (Momentum)   
- Mean Reversion   
- Random Walk

- We discussed the following statistical models/ trading strategies

- Simple Linear Regression   
- Multiple Linear Regression   
ARIMA   
- Hurst Exponent (a good indicator for market classification)   
- Martingale (random walk)
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
