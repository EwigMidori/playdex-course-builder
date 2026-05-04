请基于以下 lesson 材料，生成一个结构化题库 JSON。

目标语言：
zh-CN

course_id：
comp7415a

course_title：
COMP7415A Algorithmic Trading

chapter_id：
lecture-5

chapter_title：
Statistical Arbitrage and Pairs Trading

lesson_id：
L5

要求：
- 同时生成 `flashcard_families`、`quiz_families` 和 `longform_families`
- 题目必须只关联到当前 step：`step2`
- 所有 family 和 variant 的 `linked_steps` 都必须等于 `["step2"]`
- 同一个合格 family 尽量给出 2 个 variants；可练习内容不足时允许更少
- 题目应覆盖当前 step 的关键内容；不要把其它 step 的内容塞进这个 step 的题库
- `flashcard` 是间隔重复用的主动检索载体，不是选择题；每张卡必须有精准 front 与短 back
- `quiz` 才承载选择题、判断题、配对题等更像考试的小题
- `longform` 要能真正检查理解与表达
- 每个 family 必须包含 `practice_target` 和 `is_meta_about_course_or_material`
- 题目必须考当前 step 的可学习内容，而不是关于课程、材料、教学过程、内容组织或题库自身的元信息
- 如果当前 step 缺少可练习内容，可以减少 family 或 variants，不要为了凑数量制造低价值题
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
L5: Statistical Arbitrage and
Pairs Trading
Course Code: COMP7415
Lecturer: Tony Lam
Agenda
• Statistical arbitrage
• Pair Trading
• Co-integration
• Correlation vs Cointegration
• Implement a cointegration strategy
• FX Market Introduction
• USD-HKD arbitrage
Introduction to Statistical Arbitrage
• A trading strategy that seeks to exploit statistical mis-pricings of one or more
assets based on historical data.
• It is a mean reversion strategy that profits from the convergence of asset prices.
• Commonly involves pairs trading, basket trading, or market-neutral strategies.
• Due to market-neutral, it reduces exposure to overall market risk of a portfolio.
Top Hedge Funds
• Renaissance Technologies: Known for
using statistical arbitrage strategies in
their Medallion Fund.
• Two Sigma: Utilizes machine learning
and statistical models to implement
market-neutral strategies.
Source: https://www.visualcapitalist.com/growth-of-100-invested-in-jim-simons-medallion-fund/#google_vignette
Pair Trading
Introduction to Pair Trading
• A market-neutral trading strategy that involves
trading two correlated assets.
• Exploits the relative price movements
between the two assets.
• Example: Buying one stock and simultaneously
selling another if their price spread deviates
from the historical mean.
Example: Coca-Cola (KO) and PepsiCo (PEP)
import pandas as pd                                                                 Price          Close      High       Low      Open   Volume
import yfinance as yf                                                               Ticker            KO        KO        KO        KO       KO
Date
1970-01-02  0.176391  0.176391  0.175051  0.176391  1104000
# Fetch historical data                                                             1970-01-05  0.173710  0.175855  0.173174  0.175855   556800
data_KO=yf.download('KO',start='1970-01-01', end='1999-12-31')                      1970-01-06  0.175319  0.175855  0.172906  0.173710   796800
1970-01-07  0.176391  0.176927  0.174247  0.175319   700800
data_PEP=yf.download('PEP',start='1970-01-01', end='1999-12-31')                    1970-01-08  0.176659  0.177999  0.175319  0.176391  1065600
Price          Close      High       Low      Open  Volume
# Display the first few rows of the dataset                                         Ticker           PEP       PEP       PEP       PEP     PEP
print(data_KO.head())                                                               Date
print(data_PEP.head())                                                              1972-06-01  0.381617  0.381617  0.377205  0.377205  318600
1972-06-02  0.380514  0.385478  0.380514  0.381617  140400
1972-06-05  0.376090  0.382174  0.368900  0.381621  469800
# Preprocess the data                                                               1972-06-06  0.372771  0.372771  0.371665  0.372771  140400
data_KO['Date'] = data_KO.index                                                     1972-06-07  0.370559  0.373324  0.369453  0.372771  178200
data_KO['Date'] = pd.to_datetime(data_KO['Date'])
data_KO.set_index('Date', inplace=True)
data_PEP['Date'] = data_PEP.index
data_PEP['Date'] = pd.to_datetime(data_PEP['Date'])
data_PEP.set_index('Date', inplace=True)
Closing Price: KO vs PEP
import matplotlib.pyplot as plt
# Create a new DataFrame with common dates
</COVERAGE_CHECKLIST>

<SOURCE_OUTLINE>
L5: Statistical Arbitrage and
Pairs Trading
Course Code: COMP7415
Lecturer: Tony Lam
Agenda
• Statistical arbitrage
• Pair Trading
• Co-integration
• Correlation vs Cointegration
• Implement a cointegration strategy
• FX Market Introduction
• USD-HKD arbitrage
Introduction to Statistical Arbitrage
• A trading strategy that seeks to exploit statistical mis-pricings of one or more
assets based on historical data.
• It is a mean reversion strategy that profits from the convergence of asset prices.
• Commonly involves pairs trading, basket trading, or market-neutral strategies.
• Due to market-neutral, it reduces exposure to overall market risk of a portfolio.
Top Hedge Funds
• Renaissance Technologies: Known for
using statistical arbitrage strategies in
their Medallion Fund.
• Two Sigma: Utilizes machine learning
and statistical models to implement
market-neutral strategies.
Source: https://www.visualcapitalist.com/growth-of-100-invested-in-jim-simons-medallion-fund/#google_vignette
Pair Trading
Introduction to Pair Trading
• A market-neutral trading strategy that involves
trading two correlated assets.
• Exploits the relative price movements
between the two assets.
• Example: Buying one stock and simultaneously
selling another if their price spread deviates
from the historical mean.
Example: Coca-Cola (KO) and PepsiCo (PEP)
import pandas as pd                                                                 Price          Close      High       Low      Open   Volume
import yfinance as yf                                                               Ticker            KO        KO        KO        KO       KO
Date
1970-01-02  0.176391  0.176391  0.175051  0.176391  1104000
# Fetch historical data                                                             1970-01-05  0.173710  0.175855  0.173174  0.175855   556800
data_KO=yf.download('KO',start='1970-01-01', end='1999-12-31')                      1970-01-06  0.175319  0.175855  0.172906  0.173710   796800
1970-01-07  0.176391  0.176927  0.174247  0.175319   700800
data_PEP=yf.download('PEP',start='1970-01-01', end='1999-12-31')                    1970-01-08  0.176659  0.177999  0.175319  0.176391  1065600
Price          Close      High       Low      Open  Volume
# Display the first few rows of the dataset                                         Ticker           PEP       PEP       PEP       PEP     PEP
print(data_KO.head())                                                               Date
print(data_PEP.head())                                                              1972-06-01  0.381617  0.381617  0.377205  0.377205  318600
1972-06-02  0.380514  0.385478  0.380514  0.381617  140400
1972-06-05  0.376090  0.382174  0.368900  0.381621  469800
# Preprocess the data                                                               1972-06-06  0.372771  0.372771  0.371665  0.372771  140400
data_KO['Date'] = data_KO.index                                                     1972-06-07  0.370559  0.373324  0.369453  0.372771  178200
data_KO['Date'] = pd.to_datetime(data_KO['Date'])
data_KO.set_index('Date', inplace=True)
data_PEP['Date'] = data_PEP.index
data_PEP['Date'] = pd.to_datetime(data_PEP['Date'])
data_PEP.set_index('Date', inplace=True)
Closing Price: KO vs PEP
import matplotlib.pyplot as plt
# Create a new DataFrame with common dates
</SOURCE_OUTLINE>

<LESSON_MAP>
{
  "chapter_id": "lecture-5",
  "chapter_title": "Statistical Arbitrage and Pairs Trading",
  "course_id": "comp7415a",
  "course_title": "COMP7415A Algorithmic Trading",
  "lesson_id": "L5",
  "mode": "guided_story",
  "steps": [
    {
      "concept": "Statistical Arbitrage and Pairs Trading Introduction",
      "file": "research/pipeline/3-guided_story/L5/step1/step.json",
      "sequence_id": "step1"
    },
    {
      "concept": "Cointegration: The Statistical Foundation",
      "file": "research/pipeline/3-guided_story/L5/step2/step.json",
      "sequence_id": "step2"
    },
    {
      "concept": "Correlation vs. Cointegration",
      "file": "research/pipeline/3-guided_story/L5/step3/step.json",
      "sequence_id": "step3"
    },
    {
      "concept": "Testing for Cointegration: CADF and Johansen Test",
      "file": "research/pipeline/3-guided_story/L5/step4/step.json",
      "sequence_id": "step4"
    },
    {
      "concept": "Implementing a Pairs Trading Strategy",
      "file": "research/pipeline/3-guided_story/L5/step5/step.json",
      "sequence_id": "step5"
    },
    {
      "concept": "Real-World Challenges and Wrap-Up",
      "file": "research/pipeline/3-guided_story/L5/step6/step.json",
      "sequence_id": "step6"
    }
  ]
}

</LESSON_MAP>

<GUIDED_STORY_MANIFEST>
{
  "chapter_id": "lecture-5",
  "chapter_title": "Statistical Arbitrage and Pairs Trading",
  "course_id": "comp7415a",
  "course_title": "COMP7415A Algorithmic Trading",
  "lesson_id": "L5",
  "mode": "guided_story",
  "steps": [
    {
      "concept": "Statistical Arbitrage and Pairs Trading Introduction",
      "file": "research/pipeline/3-guided_story/L5/step1/step.json",
      "sequence_id": "step1"
    },
    {
      "concept": "Cointegration: The Statistical Foundation",
      "file": "research/pipeline/3-guided_story/L5/step2/step.json",
      "sequence_id": "step2"
    },
    {
      "concept": "Correlation vs. Cointegration",
      "file": "research/pipeline/3-guided_story/L5/step3/step.json",
      "sequence_id": "step3"
    },
    {
      "concept": "Testing for Cointegration: CADF and Johansen Test",
      "file": "research/pipeline/3-guided_story/L5/step4/step.json",
      "sequence_id": "step4"
    },
    {
      "concept": "Implementing a Pairs Trading Strategy",
      "file": "research/pipeline/3-guided_story/L5/step5/step.json",
      "sequence_id": "step5"
    },
    {
      "concept": "Real-World Challenges and Wrap-Up",
      "file": "research/pipeline/3-guided_story/L5/step6/step.json",
      "sequence_id": "step6"
    }
  ]
}

</GUIDED_STORY_MANIFEST>

<GUIDED_STORY_STEPS>
[
  {
    "chapter_id": "lecture-5",
    "course_id": "comp7415a",
    "lesson_id": "L5",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s001",
        "introduced_terms": [],
        "lines": [
          "想象一下，你发现两只股票的价格总是像跳舞一样，你进我退。",
          "当它们暂时分开时，你赌它们会重新靠近。"
        ],
        "type": "narration"
      },
      {
        "id": "s002",
        "introduced_terms": [
          "statistical_arbitrage"
        ],
        "lines": [
          "这种思路，就是**<term id=\"statistical_arbitrage\">统计套利</term>**的核心。",
          "它不预测市场涨跌，而是寻找价格关系中出现的统计偏差。"
        ],
        "type": "narration"
      },
      {
        "id": "s003",
        "introduced_terms": [
          "mean_reversion"
        ],
        "lines": [
          "它本质上是一种**<term id=\"mean_reversion\">均值回归</term>**策略。",
          "当价格偏离历史平均水平太远时，就押注它会回来。"
        ],
        "type": "narration"
      },
      {
        "id": "s004",
        "introduced_terms": [
          "pairs_trading"
        ],
        "lines": [
          "最经典的做法是**<term id=\"pairs_trading\">配对交易</term>**：",
          "同时买入一只资产，卖出另一只高度相关的资产。"
        ],
        "type": "narration"
      },
      {
        "id": "s005",
        "introduced_terms": [
          "market_neutral"
        ],
        "lines": [
          "因为同时持有多头和空头，整个组合是**<term id=\"market_neutral\">市场中性</term>**的。",
          "大盘涨跌的影响被抵消，收益只来自两只资产之间的相对价格变化。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "统计套利依赖均值回归假设：价格偏离历史关系后，倾向于回归。",
          "kind": "single_choice",
          "options": [
            "市场永远有效",
            "价格偏离后最终会回归",
            "资产价格会一直上涨",
            "两只资产完全独立"
          ],
          "prompt": "统计套利策略的核心假设是什么？"
        },
        "id": "s006",
        "lines": [
          "统计套利策略的核心假设是什么？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step1",
    "source": {
      "plain_text": "Statistical arbitrage is a trading strategy that seeks to exploit statistical mis-pricings of one or more assets based on historical data. It is a mean reversion strategy that profits from the convergence of asset prices. Commonly involves pairs trading, basket trading, or market-neutral strategies. Due to market-neutral, it reduces exposure to overall market risk of a portfolio.",
      "related": [
        "Pairs Trading",
        "Mean Reversion"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "market_neutral": {
        "aliases": [
          "Market Neutral"
        ],
        "display": "市场中性",
        "gloss": "投资组合的整体市场风险敞口为零，收益不依赖大盘涨跌。"
      },
      "mean_reversion": {
        "aliases": [
          "Mean Reversion"
        ],
        "display": "均值回归",
        "gloss": "资产价格偏离历史均值后，倾向于回归到该均值的现象。"
      },
      "pairs_trading": {
        "aliases": [
          "Pairs Trading"
        ],
        "display": "配对交易",
        "gloss": "同时买入一只资产、卖出另一只相关资产，押注两者价差回归的交易方式。"
      },
      "statistical_arbitrage": {
        "aliases": [
          "Statistical Arbitrage",
          "Stat Arb"
        ],
        "display": "统计套利",
        "gloss": "利用历史数据寻找资产价格统计性错误定价，并押注价格回归的交易策略。"
      }
    }
  },
  {
    "chapter_id": "lecture-5",
    "course_id": "comp7415a",
    "lesson_id": "L5",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s007",
        "introduced_terms": [],
        "lines": [
          "大多数金融价格序列本身并不平稳，它们会随机游走。",
          "但两只不平稳的股票，它们的某种组合却可能是平稳的。"
        ],
        "type": "narration"
      },
      {
        "id": "s008",
        "introduced_terms": [
          "cointegration"
        ],
        "lines": [
          "这种性质叫做**<term id=\"cointegration\">协整</term>**。",
          "它描述的是两个或多个时间序列之间的长期均衡关系。"
        ],
        "type": "narration"
      },
      {
        "id": "s009",
        "introduced_terms": [
          "stationary"
        ],
        "lines": [
          "如果序列Y和X是协整的，就存在一个常数β，",
          "使得组合 $z_t = y_t - \\beta x_t$ 是**<term id=\"stationary\">平稳</term>**的。"
        ],
        "type": "narration"
      },
      {
        "id": "s010",
        "introduced_terms": [
          "hedge_ratio"
        ],
        "lines": [
          "这个β就是**<term id=\"hedge_ratio\">对冲比率</term>**。",
          "它告诉你：每买入1股Y，应该卖出多少股X来构建市场中性组合。"
        ],
        "type": "narration"
      },
      {
        "id": "s011",
        "introduced_terms": [
          "spread"
        ],
        "lines": [
          "组合 $z_t$ 就是**<term id=\"spread\">价差</term>**。",
          "当价差偏离均值太远时，就出现了交易信号。"
        ],
        "type": "narration"
      },
      {
        "formula": {
          "latex": "z_t = y_t - \\beta x_t \\text{ is stationary}",
          "spoken": "价差序列等于Y的价格减去β乘以X的价格，这个序列是平稳的。"
        },
        "id": "s012",
        "lines": [
          "协整的数学定义很简洁：",
          "$z_t = y_t - \\beta x_t$ 是平稳的。"
        ],
        "type": "formula"
      },
      {
        "exercise": {
          "answers": [
            "对冲比率"
          ],
          "explanation": "β决定了构建平稳组合时两种资产的比例，因此称为对冲比率。",
          "kind": "fill_in_blank",
          "prompt": "协整关系中的β被称为______。"
        },
        "id": "s013",
        "lines": [
          "协整关系中的β被称为什么？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step2",
    "source": {
      "plain_text": "Most financial time series are not stationary nor mean reverting. However, we can create a portfolio of individual price series so that the market value (or price) series of this portfolio is stationary. If we can find a stationary linear combination of several non-stationary price series, then these price series are called cointegrated.",
      "related": [
        "Cointegration",
        "Stationarity",
        "Hedge Ratio"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "cointegration": {
        "aliases": [
          "Cointegration"
        ],
        "display": "协整",
        "gloss": "两个或多个非平稳时间序列的线性组合是平稳的，表明它们存在长期均衡关系。"
      },
      "hedge_ratio": {
        "aliases": [
          "Hedge Ratio",
          "Cointegrating Coefficient"
        ],
        "display": "对冲比率",
        "gloss": "构建平稳组合时，每单位资产X需要对应多少单位资产Y的系数。"
      },
      "spread": {
        "aliases": [
          "Spread"
        ],
        "display": "价差",
        "gloss": "配对交易中，两只资产价格经过对冲比率调整后的差值。"
      },
      "stationary": {
        "aliases": [
          "Stationarity"
        ],
        "display": "平稳性",
        "gloss": "时间序列的统计性质（如均值、方差）不随时间变化。"
      }
    }
  },
  {
    "chapter_id": "lecture-5",
    "course_id": "comp7415a",
    "lesson_id": "L5",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s014",
        "introduced_terms": [
          "correlation"
        ],
        "lines": [
          "很多人会把协整和**<term id=\"correlation\">相关性</term>**搞混。",
          "它们听起来很像，但本质完全不同。"
        ],
        "type": "narration"
      },
      {
        "id": "s015",
        "introduced_terms": [],
        "lines": [
          "相关性衡量的是短期线性关系。",
          "两只股票过去一年高度相关，不代表它们未来不会分道扬镳。"
        ],
        "type": "narration"
      },
      {
        "id": "s016",
        "introduced_terms": [],
        "lines": [
          "协整关注的是长期均衡关系。",
          "即使短期偏离，协整的序列最终会重新走到一起。"
        ],
        "type": "narration"
      },
      {
        "id": "s017",
        "introduced_terms": [],
        "lines": [
          "一个经典例子：醉汉和他的狗。",
          "狗可能跑前跑后（短期偏离），但始终跟着主人（长期均衡）。"
        ],
        "type": "narration"
      },
      {
        "id": "s018",
        "introduced_terms": [],
        "lines": [
          "相关性用于分散投资，协整用于寻找可交易的配对。",
          "对于配对交易，协整才是关键，而不是高相关性。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "相关性衡量短期线性关系，协整衡量长期均衡关系。两者不同。",
          "kind": "single_choice",
          "options": [
            "高相关性意味着一定协整",
            "协整关注长期均衡，相关性关注短期关系",
            "协整和相关性是同一个概念",
            "协整只适用于平稳序列"
          ],
          "prompt": "以下哪个说法是正确的？"
        },
        "id": "s019",
        "lines": [
          "以下哪个说法是正确的？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step3",
    "source": {
      "plain_text": "Correlation measures the strength and direction of a linear relationship between 2 variables. Cointegration indicates a long-term equilibrium relationship between 2 or more time series, even if they are non-stationary individually. Correlation is about the short-term relationship. Cointegration focuses on the long-term relationship.",
      "related": [
        "Correlation",
        "Cointegration"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "correlation": {
        "aliases": [
          "Correlation"
        ],
        "display": "相关性",
        "gloss": "衡量两个变量之间线性关系的强度和方向，取值范围在-1到1之间。"
      }
    }
  },
  {
    "chapter_id": "lecture-5",
    "course_id": "comp7415a",
    "lesson_id": "L5",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s020",
        "introduced_terms": [
          "cadf"
        ],
        "lines": [
          "如何判断两只股票是否协整？",
          "最常用的方法是**<term id=\"cadf\">协整增强迪基-富勒检验</term>**，简称CADF。"
        ],
        "type": "narration"
      },
      {
        "id": "s021",
        "introduced_terms": [],
        "lines": [
          "第一步：对两只股票的价格做线性回归，得到对冲比率β。",
          "第二步：用β构建价差序列 $z_t = y_t - \\beta x_t$。"
        ],
        "type": "narration"
      },
      {
        "id": "s022",
        "introduced_terms": [],
        "lines": [
          "第三步：对价差序列进行ADF平稳性检验。",
          "如果p值小于0.05，就拒绝“不平稳”的原假设，认为序列协整。"
        ],
        "type": "narration"
      },
      {
        "id": "s023",
        "introduced_terms": [
          "johansen_test"
        ],
        "lines": [
          "当涉及三只或更多资产时，需要用**<term id=\"johansen_test\">约翰森检验</term>**。",
          "它可以找出存在多少个协整关系。"
        ],
        "type": "narration"
      },
      {
        "id": "s024",
        "introduced_terms": [
          "trace_statistic",
          "max_eigenvalue_statistic"
        ],
        "lines": [
          "约翰森检验给出两个统计量：**<term id=\"trace_statistic\">迹统计量</term>**和**<term id=\"max_eigenvalue_statistic\">最大特征值统计量</term>**。",
          "它们从不同角度判断协整关系的个数r。"
        ],
        "type": "narration"
      },
      {
        "id": "s025",
        "introduced_terms": [],
        "lines": [
          "如果r=0，说明没有协整关系。",
          "如果r>0，说明存在r个独立的平稳组合，对应的特征向量就是对冲比率。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answers": [
            "线性回归"
          ],
          "explanation": "先对两只股票的价格做线性回归，得到对冲比率β。",
          "kind": "fill_in_blank",
          "prompt": "CADF检验的第一步是：通过______确定最优对冲比率。"
        },
        "id": "s026",
        "lines": [
          "CADF检验的第一步是什么？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step4",
    "source": {
      "plain_text": "Cointegrated Augmented Dickey Fuller Test (CADF): We first determine the optimal hedge ratio by running a linear regression between two price series. Use this hedge ratio to form a portfolio. Then run an ADF test on this portfolio price series. Johansen Test for Cointegration: It tests for cointegration of more than 2 variables.",
      "related": [
        "CADF",
        "Johansen Test"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "cadf": {
        "aliases": [
          "CADF",
          "Cointegrated Augmented Dickey Fuller Test"
        ],
        "display": "协整增强迪基-富勒检验",
        "gloss": "先通过回归确定对冲比率，再对价差进行ADF检验，判断序列是否协整。"
      },
      "johansen_test": {
        "aliases": [
          "Johansen Test"
        ],
        "display": "约翰森检验",
        "gloss": "用于检验多个时间序列之间协整关系的个数，并给出对冲比率。"
      },
      "max_eigenvalue_statistic": {
        "aliases": [
          "Maximum Eigenvalue Statistic"
        ],
        "display": "最大特征值统计量",
        "gloss": "约翰森检验中的另一种统计量，检验恰好有r个协整向量。"
      },
      "trace_statistic": {
        "aliases": [
          "Trace Statistic"
        ],
        "display": "迹统计量",
        "gloss": "约翰森检验中的一种统计量，检验协整向量个数是否小于等于r。"
      }
    }
  },
  {
    "chapter_id": "lecture-5",
    "course_id": "comp7415a",
    "lesson_id": "L5",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s027",
        "introduced_terms": [],
        "lines": [
          "有了协整工具，就可以搭建完整的配对交易策略了。",
          "一共六步，每一步都很清晰。"
        ],
        "type": "narration"
      },
      {
        "id": "s028",
        "introduced_terms": [],
        "lines": [
          "第一步：找到一对可能相关的资产。",
          "第二步：用CADF或约翰森检验确认它们是否协整。"
        ],
        "type": "narration"
      },
      {
        "id": "s029",
        "introduced_terms": [
          "entry_threshold"
        ],
        "lines": [
          "第三步：计算价差，并求出历史均值和标准差。",
          "第四步：设定**<term id=\"entry_threshold\">入场阈值</term>**，当价差偏离超过阈值时触发信号。"
        ],
        "type": "narration"
      },
      {
        "id": "s030",
        "introduced_terms": [
          "z_score"
        ],
        "lines": [
          "通常用**<term id=\"z_score\">Z分数</term>**来衡量偏离程度：",
          "$Z = \\frac{\\text{当前价差} - \\text{均值}}{\\text{标准差}}$"
        ],
        "type": "narration"
      },
      {
        "id": "s031",
        "introduced_terms": [],
        "lines": [
          "第五步：执行交易。",
          "如果Z > 阈值，卖出Y、买入X（押注价差缩小）。",
          "如果Z < -阈值，买入Y、卖出X。"
        ],
        "type": "narration"
      },
      {
        "id": "s032",
        "introduced_terms": [],
        "lines": [
          "第六步：当价差回归到均值附近时，平仓离场。",
          "整个策略的核心就是：低买高卖价差。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 0,
          "explanation": "Z > 阈值意味着价差过高，应卖出被高估的Y，买入被低估的X。",
          "kind": "single_choice",
          "options": [
            "买入Y，卖出X",
            "买入X，卖出Y",
            "同时买入X和Y",
            "什么都不做"
          ],
          "prompt": "当Z分数大于入场阈值时，应该怎么做？"
        },
        "id": "s033",
        "lines": [
          "当Z分数大于入场阈值时，应该怎么做？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step5",
    "source": {
      "plain_text": "Steps to Implement Pair Trading: 1. Identify a pair of correlated assets. 2. Test for co-integration between the assets. 3. Calculate the spread and determine the historical mean and standard deviation. 4. Generate trading signals when the spread deviates from the mean by a certain threshold. 5. Execute trades. 6. Close positions when the spread reverts to the mean.",
      "related": [
        "Pairs Trading",
        "Z-score",
        "Entry Threshold"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "entry_threshold": {
        "aliases": [
          "Entry Threshold"
        ],
        "display": "入场阈值",
        "gloss": "触发交易信号的价差偏离程度，通常用Z分数的绝对值表示。"
      },
      "z_score": {
        "aliases": [
          "Z-score"
        ],
        "display": "Z分数",
        "gloss": "衡量当前价差偏离均值多少个标准差的标准化指标。"
      }
    }
  },
  {
    "chapter_id": "lecture-5",
    "course_id": "comp7415a",
    "lesson_id": "L5",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s034",
        "introduced_terms": [
          "model_risk"
        ],
        "lines": [
          "统计套利听起来很完美，但真实市场充满挑战。",
          "**<term id=\"model_risk\">模型风险</term>**：假设错误或过拟合，策略可能失效。"
        ],
        "type": "narration"
      },
      {
        "id": "s035",
        "introduced_terms": [
          "execution_risk"
        ],
        "lines": [
          "**<term id=\"execution_risk\">执行风险</term>**：滑点和延迟会侵蚀利润。",
          "市场风险：突发事件可能打破历史关系。",
          "监管风险：做空限制、杠杆要求变化等。"
        ],
        "type": "narration"
      },
      {
        "id": "s036",
        "introduced_terms": [],
        "lines": [
          "尽管如此，统计套利仍是许多量化基金的核心策略。",
          "从统计上看，它有很高的胜率。"
        ],
        "type": "narration"
      },
      {
        "id": "s037",
        "introduced_terms": [],
        "lines": [
          "要有效运行这个策略：",
          "选择协整的资产组，尝试不同的观察窗口，",
          "选择合适的距离度量，并定期重新平衡。"
        ],
        "type": "narration"
      },
      {
        "id": "s038",
        "introduced_terms": [],
        "lines": [
          "一句话带走：",
          "统计套利不是预测价格，而是押注关系——当关系被打破时，它终将回归。"
        ],
        "type": "narration"
      }
    ],
    "sequence_id": "step6",
    "source": {
      "plain_text": "Real Market Challenges: Model Risk, Execution Risk, Market Risk, Regulatory Risk. Statistical arbitrage is a popular strategy used by many quant trading firms that effectively make profits over years. From statistics point of view, it has a high probability of winning.",
      "related": [
        "Model Risk",
        "Execution Risk"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "execution_risk": {
        "aliases": [
          "Execution Risk"
        ],
        "display": "执行风险",
        "gloss": "因滑点、延迟等因素导致实际成交价格偏离预期的风险。"
      },
      "model_risk": {
        "aliases": [
          "Model Risk"
        ],
        "display": "模型风险",
        "gloss": "因模型假设错误或过拟合导致策略表现不佳的风险。"
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
  "chapter_id": "lecture-5",
  "course_id": "comp7415a",
  "lesson_id": "L5",
  "mode": "guided_story",
  "screens": [
    {
      "id": "s007",
      "introduced_terms": [],
      "lines": [
        "大多数金融价格序列本身并不平稳，它们会随机游走。",
        "但两只不平稳的股票，它们的某种组合却可能是平稳的。"
      ],
      "type": "narration"
    },
    {
      "id": "s008",
      "introduced_terms": [
        "cointegration"
      ],
      "lines": [
        "这种性质叫做**<term id=\"cointegration\">协整</term>**。",
        "它描述的是两个或多个时间序列之间的长期均衡关系。"
      ],
      "type": "narration"
    },
    {
      "id": "s009",
      "introduced_terms": [
        "stationary"
      ],
      "lines": [
        "如果序列Y和X是协整的，就存在一个常数β，",
        "使得组合 $z_t = y_t - \\beta x_t$ 是**<term id=\"stationary\">平稳</term>**的。"
      ],
      "type": "narration"
    },
    {
      "id": "s010",
      "introduced_terms": [
        "hedge_ratio"
      ],
      "lines": [
        "这个β就是**<term id=\"hedge_ratio\">对冲比率</term>**。",
        "它告诉你：每买入1股Y，应该卖出多少股X来构建市场中性组合。"
      ],
      "type": "narration"
    },
    {
      "id": "s011",
      "introduced_terms": [
        "spread"
      ],
      "lines": [
        "组合 $z_t$ 就是**<term id=\"spread\">价差</term>**。",
        "当价差偏离均值太远时，就出现了交易信号。"
      ],
      "type": "narration"
    },
    {
      "formula": {
        "latex": "z_t = y_t - \\beta x_t \\text{ is stationary}",
        "spoken": "价差序列等于Y的价格减去β乘以X的价格，这个序列是平稳的。"
      },
      "id": "s012",
      "lines": [
        "协整的数学定义很简洁：",
        "$z_t = y_t - \\beta x_t$ 是平稳的。"
      ],
      "type": "formula"
    },
    {
      "exercise": {
        "answers": [
          "对冲比率"
        ],
        "explanation": "β决定了构建平稳组合时两种资产的比例，因此称为对冲比率。",
        "kind": "fill_in_blank",
        "prompt": "协整关系中的β被称为______。"
      },
      "id": "s013",
      "lines": [
        "协整关系中的β被称为什么？"
      ],
      "type": "exercise"
    }
  ],
  "sequence_id": "step2",
  "source": {
    "plain_text": "Most financial time series are not stationary nor mean reverting. However, we can create a portfolio of individual price series so that the market value (or price) series of this portfolio is stationary. If we can find a stationary linear combination of several non-stationary price series, then these price series are called cointegrated.",
    "related": [
      "Cointegration",
      "Stationarity",
      "Hedge Ratio"
    ]
  },
  "target_language": "zh-CN",
  "term_catalog": {
    "cointegration": {
      "aliases": [
        "Cointegration"
      ],
      "display": "协整",
      "gloss": "两个或多个非平稳时间序列的线性组合是平稳的，表明它们存在长期均衡关系。"
    },
    "hedge_ratio": {
      "aliases": [
        "Hedge Ratio",
        "Cointegrating Coefficient"
      ],
      "display": "对冲比率",
      "gloss": "构建平稳组合时，每单位资产X需要对应多少单位资产Y的系数。"
    },
    "spread": {
      "aliases": [
        "Spread"
      ],
      "display": "价差",
      "gloss": "配对交易中，两只资产价格经过对冲比率调整后的差值。"
    },
    "stationary": {
      "aliases": [
        "Stationarity"
      ],
      "display": "平稳性",
      "gloss": "时间序列的统计性质（如均值、方差）不随时间变化。"
    }
  }
}

</CURRENT_STEP>

<PLAIN_TEXT>
         L5: Statistical Arbitrage and 
                   Pairs Trading
                     Course Code: COMP7415
  Lecturer: Tony Lam
    Agenda
    • Statistical arbitrage
      • Pair Trading
      • Co-integration
    • Correlation vs Cointegration
    • Implement a cointegration strategy
    • FX Market Introduction
    • USD-HKD arbitrage
    Introduction to Statistical Arbitrage
    • A trading strategy that seeks to exploit statistical mis-pricings of one or more 
    assets based on historical data.
    • It is a mean reversion strategy that profits from the convergence of asset prices.
    • Commonly involves pairs trading, basket trading, or market-neutral strategies.
    • Due to market-neutral, it reduces exposure to overall market risk of a portfolio.
         Top Hedge Funds
         • Renaissance Technologies: Known for 
           using statistical arbitrage strategies in 
           their Medallion Fund.
         • Two Sigma: Utilizes machine learning 
           and statistical models to implement 
           market-neutral strategies.
   Source: https://www.visualcapitalist.com/growth-of-100-invested-in-jim-simons-medallion-fund/#google_vignette
      Pair Trading
    Introduction to Pair Trading
    • A market-neutral trading strategy that involves 
    trading two correlated assets.
    • Exploits the relative price movements 
    between the two assets.
    • Example: Buying one stock and simultaneously 
    selling another if their price spread deviates 
    from the historical mean.
             Example: Coca-Cola (KO) and PepsiCo (PEP)
           import pandas as pd                                                                 Price          Close      High       Low      Open   Volume
           import yfinance as yf                                                               Ticker            KO        KO        KO        KO       KO
                                                                                               Date
                                                                                               1970-01-02  0.176391  0.176391  0.175051  0.176391  1104000
           # Fetch historical data                                                             1970-01-05  0.173710  0.175855  0.173174  0.175855   556800
           data_KO=yf.download('KO',start='1970-01-01', end='1999-12-31')                      1970-01-06  0.175319  0.175855  0.172906  0.173710   796800
                                                                                               1970-01-07  0.176391  0.176927  0.174247  0.175319   700800
           data_PEP=yf.download('PEP',start='1970-01-01', end='1999-12-31')                    1970-01-08  0.176659  0.177999  0.175319  0.176391  1065600
                                                                                               Price          Close      High       Low      Open  Volume
           # Display the first few rows of the dataset                                         Ticker           PEP       PEP       PEP       PEP     PEP
           print(data_KO.head())                                                               Date
           print(data_PEP.head())                                                              1972-06-01  0.381617  0.381617  0.377205  0.377205  318600
                                                                                               1972-06-02  0.380514  0.385478  0.380514  0.381617  140400
                                                                                               1972-06-05  0.376090  0.382174  0.368900  0.381621  469800
           # Preprocess the data                                                               1972-06-06  0.372771  0.372771  0.371665  0.372771  140400
           data_KO['Date'] = data_KO.index                                                     1972-06-07  0.370559  0.373324  0.369453  0.372771  178200
           data_KO['Date'] = pd.to_datetime(data_KO['Date'])
           data_KO.set_index('Date', inplace=True)
           data_PEP['Date'] = data_PEP.index
           data_PEP['Date'] = pd.to_datetime(data_PEP['Date'])
           data_PEP.set_index('Date', inplace=True)
       Closing Price: KO vs PEP
   import matplotlib.pyplot as plt
   # Create a new DataFrame with common dates
   common_dates=data_KO.index.intersection(data_PEP.index)
   data_combined= pd.DataFrame({
    'KO_Close': data_KO['Close'].loc[common_dates]['KO'],
    'PEP_Close': data_PEP['Close'].loc[common_dates]['PEP'] }
   )
   # Plot the closing prices
   plt.style.use('dark_background')
   plt.figure(figsize=(10, 6))
   plt.plot(data_combined.index, data_combined['KO_Close'],
   color='blue', linewidth=2, label='KO')
   plt.plot(data_combined.index, data_combined['PEP_Close'],
   color='red', linewidth=2, label='PEP')
   plt.xlabel('Date')
   plt.ylabel('Closing Price')
   plt.title('KO vs PEP')
   plt.legend()
   plt.show()
      Correlation: KO vs PEP
                                         Correlation between KO and PEP: 0.98
    # Calculate the correlation between KO and PEP
    correlation = data_combined['KO_Close'].corr(data_combined['PEP_Close'])
    print(f'Correlation between KO and PEP: {correlation:.2f}')
      Scatter Plot: KO vs PEP
    # Scatter plot between KO and PEP closing prices
    plt.figure(figsize=(10, 6))
    plt.scatter(data_combined['KO_Close'], data_combined['PEP_Close'],
    alpha=0.5, color='purple')
    plt.title('Scatter Plot of KO vs PEP Closing Prices')
    plt.xlabel('KO Closing Price')
    plt.ylabel('PEP Closing Price')
    # Add a 1:1 line for reference
    max_price= max(data_combined['KO_Close'].max(),
    data_combined['PEP_Close'].max())
    plt.plot([0, max_price], [0, max_price], color='red', linestyle='--',
    linewidth=1, label='1:1 Line')
    plt.show()
      Price Difference: KO vs PEP
  # Calculate the price difference
  price_difference = data_combined['KO_Close'] -
  data_combined['PEP_Close']
  # Calculate the mean of the price difference
  mean_difference = price_difference.mean()
  # Plotting the price difference
  plt.figure(figsize=(14, 7))
  plt.plot(price_difference.index, price_difference.values,
  label='Price Difference (KO - PEP)', color='blue')
  plt.axhline(mean_difference, color='red', linestyle='--',
  linewidth=1, label='Mean Difference') # Mean line
  plt.title('Price Difference between Coca-Cola (KO) and 
  PepsiCo (PEP)')
  plt.xlabel('Date')
  plt.ylabel('Price Difference')
  plt.legend()
  plt.grid()
  plt.show()
  Does this pair still valid nowadays? 
     Potential reasons
     • Fundamental Changes:
       • Company Performance: Changes in the operational performance, financial health, or strategic 
        direction of either KO or PEP can lead to divergence in their stock prices.
       • Market Positioning: If one company captures more market share or shifts its product strategy 
        significantly (e.g., focusing on health-conscious products), it may affect its stock price relative 
        to the other.
     • Economic Factors:
       • Economic downturns or inflation can affect consumer spending patterns, impacting KO and 
        PEP differently based on their product offerings and market strategies.
     • Investor Behavior:
       • Changes in investor sentiment towards either company (e.g., due to negative news, earnings 
        reports, or analyst downgrades) can lead to price movements that do not correspond with 
        historical patterns.
      Cointegration
   Cointegration
   • Most financial time series are not stationary nor mean reverting. 
   • However, we can create a portfolio of individual price series so that 
    the market value (or price) series of this portfolio is stationary. 
   • If we can find a stationary linear combination of several non-
    stationary price series, then these price series are called cointegrated. 
                 Definition and Formula
                 • Two time series  ÿ , Ā are cointegrated if there exists a constant β
                                                                    ���㕡          ���㕡
                     such that ā = ÿ 2βĀ is stationary
                                                 ���㕡             ���㕡              ���㕡
                 • Here β represents the cointegrating coefficient.
                 • In market practice, β is also called the hedge ratio suggesting the 
                     number of shares to long (short) Y for each share short (long) on X
     Statistical Tests for Cointegration
     • Cointegrated Augmented Dickey Fuller Test (CADF)
     • Steps: 
       • We first determine the optimal hedge ratio by running a linear regression 
        between two price series
       • Use this hedge ratio to form a portfolio
       • Then run an ADF test on this portfolio price series {ā = ÿ 2 βĀ }
                                        ���㕡 ���㕡  ���㕡
       • If the null hypothesis is rejected, we can conclude that ā is stationary, and 
        hence {Ā,ÿ}are cointegrated       ���㕡
      Python Example
                                    Cointegration test p-value: 0.9555567191241595
     import numpy as np
     import pandas as pd
     import statsmodels.api as sm
     import matplotlib.pyplot as plt
     import yfinance as yf
     start_date = '2020-01-01'
     end_date='2025-08-31'
     ko=yf.Ticker("KO").history(start=start_date, end=end_date)
     pep=yf.Ticker("PEP").history(start=start_date, end=end_date)
     # Price data 
     ko_close = ko['Close']
     pep_close= pep['Close']
     # Cointegration test 
     result = sm.tsa.stattools.coint(ko_close, pep_close)
     print('Cointegration test p-value:', result[1])
         Steps to Implement Pair Trading
         1.  Identify a pair of correlated assets.
         2.  Test for co-integration between the assets.
         3.  Calculate the spread and determine the historical mean and standard 
             deviation.
         4.  Generate trading signals when the spread deviates from the mean by a certain 
             threshold.
         5.  Execute trades: Go long on the undervalued asset and short on the overvalued 
             asset.
         6.  Close positions when the spread reverts to the mean.
  Market Entry/Exit based on the Spread’s Distance
    Key Components
    • Co-integration: A statistical property indicating a long-term 
     equilibrium relationship between two time series.
    • Spread: The difference in price between the two assets being traded.
    • Mean Reversion: The assumption that the spread will revert to its 
     historical average.
    Cointegration for multiple 
   variables (more than 2 assets)
                 Cointegrated Augmented Dickey Fuller Test
                 • Create a multiple linear regression model
                                                                   Ā =���㗽 +���㗽 ÿ +ï+���㗽 ÿ +���㔀
                                                                     ���㕡        0         1 1���㕡                    ���㕘   ���㕘���㕡       ���㕡
                 • Steps: 
                         • We first determine the optimal hedge ratios by running a multiple linear 
                             regression between the price series
                         • Use the hedge ratios {���㗽 } to form a portfolio
                                                                                 ÿ
                         • Then run an ADF test on this portfolio price series {ā = Ā 2 ���㗽 2 ���㗽 ÿ                                                                                             2
                             ï2���㗽 ÿ }                                                                                                           ���㕡          ���㕡         0           1 1���㕡
                                           ���㕘    ���㕘���㕡
                         • If null hypothesis is rejected, we can conclude that ā is stationary, and hence 
                             {Ā,ÿ ,…,ÿ }are cointegrated                                                                                        ���㕡
                                        1              ���㕘
     Limitation of above idea
     • If null hypothesis cannot be rejected, we only know that 
      {Ā,ÿ ,…,ÿ }cannot form a stationary time series. However, it is 
         1   ���㕘
      possible that a subset (eg. Ā,ÿ and ÿ ) can be cointegrated. The 
                          1    2
      CADF test doesn’t provide further insights about this. 
     • It only assumes a multiple linear relationship between the stocks, but 
      ignore other useful variables (eg. economic factors, technical 
      indicators, etc). 
           Vector Error Correction Model (VECM)
           • The concept of cointegration for multiple variables can be captured using the 
             Vector Error Correction Model (VECM) framework. 
           • General Framework:
                1. Vector Autoregressive Model (VAR):
                     Consider a vector of multiple non-stationary time series:             Ā
                                                                                            1���㕡
                                                                                            î
                                                                                   ���㖀 =
                                                                                     ���㖕
                                                                                           Ā
                                                                                            Ā���㕡
                     A VAR model can be expressed as:                        Where
                                                                             •   A: Matrix of coefficients.
                                          ���㖀 = ý���㖀       +þÿ +���㕁
                                            ���㖕      ���㖕2���㗏      ���㕡    ���㖕      •   ÿ : Exogenous variables (if any)
                                                                             •   ���㔇���㕡: Error term
                                                                                  ���㕡
      Vector Error Correction Model (VECM)
      • General Framework:
         2. Conintegration
            If the series are cointegrated, there exists a matrix ���㗽 such that:
                                    ���㕇
                                   ���㗽 Ā ~���㔼(0)
                                      ���㕡
             ���㔼(0) is a notation referring to a stationary time series without order differencing.
            This means that a linear combination of the series, given by ���㗽���㕇Ā is stationary
                                                      ���㕡
              Johansen Test for Cointegration
              • It tests for cointegration of more than 2 variables
              • Let’s generalize the discrete version of the ADF equation. 
                     • Ā are vectors representing multiple time series
                          ���㕡
                     • Λ, ý are matrices
                               ���㕘
                                                 ∆Ā =ΛĀ                  +Ā+ý∆Ā +ï+ý ∆Ā +ε
                                                     ���㕡           ���㕡21                   1      ���㕡21                    ���㕘     ���㕡2���㕘        ���㕡
              • Just in the univariate case, cointegration doesn’t exist if Λ=0
              • Denote the rank of Λ is r and the number of time series is n
                     • So testing whether Λ=0 will be equivalent to testing r=0
    Johansen Test for Cointegration
    • The number of independent portfolios that can be formed by various 
    linear combinations of the cointegrating price series is equal to r. The 
    Johansen test will calculate r in two different ways, both based on the 
    eigenvector decomposition of Λ.
     1. Trace Statistic
     2. Maximum Eigenvalue Statistic
     Johansen Test for Cointegration
     • Meaning of ���㕟:
       • It represents the number of cointegrating vectors among the time series 
        being analyzed.
         • If ���㕟=0: There are no cointegrating relationships; the time series are not stationary in any 
          linear combination.
         • If ���㕟>0: There are ���㕟 cointegrating relationships, indicating that some linear combinations 
          of the time series are stationary, suggesting long-run equilibrium relationships among 
          them.
     • As a useful by-product, the eigenvectors found can be used as our hedge ratios 
      for the individual price series to form a stationary portfolio.
             Johansen Test for Cointegration
                    1. Trace Statistic
                          • Tests the null hypothesis that the number of cointegrating vectors is less than or equal to ���㕟
                                                                                       ���㕘
                                                                                                             
                                                    ÿ���㕅            ���㕟    =2���㕇   log(12λ )
                                                         ���㕡���㕟���㕎���㕐���㕒                                           ÿ
                                                                                   ÿ=���㕟+1
                                    where
                                    • T: Sample size
                                    • k: Number of variables in the system
                                          
                                    • λ : The estimated eigenvalues from the matrix of Λ
                                          ÿ
        Johansen Test for Cointegration
           2. Maximum Eigenvalue Statistic
               • Tests the null hypothesis that there is exactly ���㕟 cointegrating vectors 
                 against the alternative of ���㕟+1
                                                            
                              ÿ���㕅    (���㕟) = 2���㕇 ���㕙���㕜���㕔(1 2 λ     )
                                 ÿ���㕎���㕥                       ���㕟+1
               • It focuses specifically on the (���㕟 + 1)���㕡/ eigenvalue
    Python Example
    • Determine whether the <Magnificent 7=stocks are cointegrated using 
     the daily closing price from 2020 to 2025
      • AAPL
      • AMZN
      • NVDA
      • MSFT
      • TSLA
      • META
      • GOOGL
        import numpy as np
        import pandas as pd                                                                                   Stock prices data:
        import yfinance as yf                                                                                  Ticker           AAPL       AMZN      GOOGL        META        MSFT      NVDA       
                 Python Example                                                                               TSLA
        from statsmodels.tsa.vector_ar.vecm import coint_johansen
                                                                                                              Date
        # Define the stock symbols                                                                            2020-01-02  72.400505  94.900497  67.920815  208.324753  152.505692  5.971076  
        stocks = ['AAPL', 'AMZN', 'NVDA', 'MSFT', 'TSLA', 'META', 'GOOGL']                                    28.684000
                                                                                                              2020-01-03  71.696617  93.748497  67.565483  207.222488  150.606720  5.875504  
        # Download the stock data                                                                             29.534000
                                                                                                              2020-01-06  72.267921  95.143997  69.366386  211.125244  150.995987  5.900144  
        df = yf.download(stocks, start='2020-01-01', end='2025-12-31')['Close']                               30.102667
                                                                                                              2020-01-07  71.928047  95.343002  69.232391  211.582031  149.619278  5.971575  
        # Display the first few rows of the data                                                              31.270666
        print("Stock prices data:\n", df.head())                                                              2020-01-08  73.085098  94.598503  69.725174  213.727051  152.002472  5.982776  
                                                                                                              32.809334
        # Perform the Johansen test                                                                           Eigenvalues:
        result = coint_johansen(df, det_order=0, k_ar_diff=1)                                                  [0.02747738 0.02070775 0.011672   0.00785587 0.00530318 0.00366997
                                                                                                               0.00042318]
        # Check the results                                                                                   Critical values (trace):
        print("\nEigenvalues:\n", result.eig)                                                                  [[120.3673 125.6185 135.9825]
        print("\nCritical values (trace):\n", result.cvt)                                                      [ 91.109   95.7542 104.9637]
        print("\nCritical values (max eigenvalue):\n", result.cvm)                                             [ 65.8202  69.8189  77.8202]
                                                                                                               [ 44.4929  47.8545  54.6815]
        # Determine the rank based on the trace statistic                                                      [ 27.0669  29.7961  35.4628]
        trace_stat = result.lr1                                                                                [ 13.4294  15.4943  19.9349]                               (90%, 95%, 99%) 
        critical_values = result.cvt[:, 1] # 5% critical values                                                [  2.7055   3.8415   6.6349]]                              confidence level
        rank = np.sum(trace_stat > critical_values)                                                           Critical values (max eigenvalue):
                                                                                                               [[43.2947 46.2299 52.3069]
                                                                                                               [37.2786 40.0763 45.8662]
        print("\ntrace_stat=",trace_stat)                                                                      [31.2379 33.8777 39.3693]
        print("\nNumber of cointegration relationships (rank):", rank)                                         [25.1236 27.5858 32.7172]
                                                                                                               [18.8928 21.1314 25.865 ]
        # Determine the rank based on the maximum eigenvalue statistic                                         [12.2971 14.2639 18.52  ]
        max_eigen_stat= result.lr2                                                                             [ 2.7055  3.8415  6.6349]]
        critical_values = result.cvm[:, 1] # 5% critical values                                               trace_stat= [117.13703785  75.20481929  43.71245014  26.0427652   14.1730027
                                                                                                                 6.17048494   0.63701792]
        rank = np.sum(max_eigen_stat > critical_values)                                                       Number of cointegration relationships (rank): 0
        print("\nmax_eigen_stat=", max_eigen_stat)                                                            max_eigen_stat= [41.93221855 31.49236916 17.66968493 11.8697625   8.00251776  
        print("\nNumber of cointegration relationships (rank):", rank)                                        5.53346703  0.63701792]
   Reference: https://www.statsmodels.org/dev/generated/statsmodels.tsa.vector_ar.vecm.coint_johansen.html    Number of cointegration relationships (rank): 0
   Cointegration vs Correlation
     Correlation
     • Measures the strength and direction of a linear relationship between 2 variables.
     • The value is ranging between -1 and 1
       • 1: Perfect positive linear relationship
       • -1: Perfect negative linear relationship
       • 0: No linear relationship
     • Limitation: Does not imply a long-term equilibrium relationship.
   Cointegration
   • Indicates a long-term equilibrium relationship between 2 or more 
    time series, even if they are non-stationary individually.
   • If two series are cointegrated, they will move together over time, 
    despite short-term deviations.
     Key Differences
     • Nature:
       • Correlation is about the short-term relationship.
       • Cointegration focuses on the long-term relationship.
     • Application:
       • Use correlation for diversification.
       • Use cointegration for identifying tradable pairs.
   Implement a Cointegration
       Strategy
     Cointegration Strategy
     • Let’s create a cointegration strategy for banking sector stocks
       • 00005HK (i.e. HSBC)
       • 00011HK (i.e. Hang Seng Bank)
       • 00939HK (i.e. China Construction Bank)
       • 02388HK (i.e. Bank of China Hong Kong)
       • 03968HK (i.e. China Merchants Bank)
     • BacktestSetup/ Assumptions
       • Instrument: 00005HK, 00011HK, 00939HK, 02388HK, 03968HK
       • Backtest period: 2021.01 – 2024.12
       • Initial capital: HK$100k
       • Short-selling are possible
       • We can trade odd lot
  Market Entry/Exit based on the Spread’s Distance
     Cointegration Script (1)
      from AlgoAPI import AlgoAPIUtil, AlgoAPI_Backtest
      import pandas as pd
      import numpy as np
      import statsmodels.api as sm
      from statsmodels.tsa.stattools import coint
      class AlgoEvent:
        def __init__(self):
         self.order_size = 100
         self.entry_threshold = 1
         self.pair_id = 0
         self.contractSize = {}
        def start(self, mEvt):
         self.evt = AlgoAPI_Backtest.AlgoEvtHandler(self, mEvt)
         for instrument in mEvt['subscribeList']:
          self.contractSize[instrument] = self.evt.getContractSpec(instrument)["contractSize"]
         self.evt.start()
               Cointegration Script (2)
       def on_bulkdatafeed(self, isSync, bd, ab):
          if not isSync:
            return
          # get historical data
          instruments = list(bd)
          data = {}
          for instrument in instruments:
            obs=self.evt.getHistoricalBar(contract={"instrument":instrument}, numOfBar=100, interval="D")
            data[instrument] = {t: obs[t]['c'] for t in obs}
          df = pd.DataFrame(data)
          df = df.fillna(method='ffill')
          df = df.dropna()
          self.evt.consoleLog("df=",df)
          # cointegration matrix and pair
          cointegrated_pairs, coint_df = find_cointegration_pair(instruments, df, threshold=0.05)
          self.evt.consoleLog("coint_df=",coint_df)                                                      def find_cointegration_pair(instruments, df, threshold=0.05):
          self.evt.consoleLog("\nCointegrated Pairs:")                                                     n = len(instruments)
          for pair in cointegrated_pairs:                                                                  coint_matrix = np.zeros((n, n))
            self.evt.consoleLog(pair)                                                                      for i in range(n):
                                                                                                              for j in range(i + 1, n):
                                                                                                                p_value = coint(df[instruments[i]], df[instruments[j]])[1]
                                                                                                                coint_matrix[i, j] = p_value
                                                                                                           cointegrated_pairs = [(instruments[i], instruments[j]) for i in range(n) for j in
                                                                                                         range(i + 1, n) if coint_matrix[i, j] < threshold]
                                                                                                           return cointegrated_pairs, coint_matrix
                  Cointegration Script (3)
                   # Find hedge ratios and trading signals
                   for stock1, stock2 in cointegrated_pairs:
                      self.pair_id += 1
                      # Fit linear regression to find hedge ratio
                      Y=df[stock1]
                      X=df[stock2]
                      X=sm.add_constant(X)
                      model = sm.OLS(Y, X).fit()
                      hedge_ratio = model.params[1]                                                                                   # long stock1 and short stock2
                      # Calculate the spread and Z-score                                                                              elif z_score[-1] < -1*self.entry_threshold:
                      spread = Y - hedge_ratio * df[stock2]                                                                             order1 = AlgoAPIUtil.OrderObject(
                      z_score = (spread - spread.mean()) / spread.std()                                                                    instrument = stock1,
                      self.evt.consoleLog('debug ... z_score=',z_score[-1])                                                                openclose = 'open',
                      # short stock1 and long stock2                                                                                       buysell = 1, #sell
                      if z_score[-1] > self.entry_threshold:                                                                               ordertype = 0, #market order
                        order1 = AlgoAPIUtil.OrderObject(                                                                                  volume=1/self.contractSize[stock1]*self.order_size, #1 share of stock 1
                           instrument = stock1,                                                                                            orderRef= self.pair_id
                           openclose = 'open',                                                                                          )
                           buysell = -1, #sell                                                                                          self.evt.sendOrder(order1)
                           ordertype = 0, #market order                                                                                 order2 = AlgoAPIUtil.OrderObject(
                           volume = 1/self.contractSize[stock1]*self.order_size, #1 share of stock1                                        instrument = stock2,
                           orderRef = self.pair_id                                                                                         openclose = 'open',
                        )                                                                                                                  buysell = -1 if hedge_ratio>0 else 1,
                        self.evt.sendOrder(order1)                                                                                         ordertype = 0, #market order
                        order2 = AlgoAPIUtil.OrderObject(                                                                                  volume=hedge_ratio/self.contractSize[stock2]*self.order_size,
                           instrument = stock2,                                                                                            orderRef= self.pair_id
                           openclose = 'open',                                                                                          )
                           buysell = 1 if hedge_ratio>0 else -1,                                                                        self.evt.sendOrder(order2)
                           ordertype = 0, #market order
                           volume = hedge_ratio/self.contractSize[stock2]*self.order_size,
                           orderRef = self.pair_id
                        )
                        self.evt.sendOrder(order2)
       Cointegration Script (4)
          # check order exit
          pairs = {}
          pos, osOrder, pendOrder = self.evt.getSystemOrders()
          for tradeID in osOrder:
           order = osOrder[tradeID]
           instrument = order['instrument']
           pair_id = order['orderRef']
           buysell = order['buysell']
           volume=order['Volume']
           openprice = order['openprice']
           pl = order[8pl']
           mktPrice = bd[instrument]['bidPrice'] if buysell==1 else bd[instrument]['askPrice']
           if pair_id not in pairs:
            pairs[pair_id] = {"entry_spread":0, "current_spread":0, "count":0, “pl":0}
           pairs[pair_id]["count"]+=1
           pairs[pair_id][“pl"]+= pl
           pairs[pair_id]["entry_spread"]+= -1*buysell*volume*openprice
           pairs[pair_id]["current_spread"]+= buysell*volume*mktPrice
          for pair_id in pairs:
           if pairs[pair_id]["count"]==2 and ( \
            (pairs[pair_id]["entry_spread"]>0 and pairs[pair_id]["current_spread"]<=0) or \
            (pairs[pair_id]["entry_spread"]<0 and pairs[pair_id]["current_spread"]>=0) \
           ) and pairs[pair_id][“pl"]>0:
            order = AlgoAPIUtil.OrderObject(
             openclose = 'close',
             orderRef = pair_id
            )
            self.evt.sendOrder(order)
                                                                                                                                      defon_bulkdatafeed(self,isSync,bd,ab):
                                                                                                                                        if not isSync:
                                                                                                                                          return
                                                                                                                                        # get historical data
                                                                                                                                        instruments= list(bd)
                                                                                                                                        data= {}
                                                                                                                                        for instrumentin instruments:
                                                                                                                                          obs=self.evt.getHistoricalBar(contract={"instrument":instrument},numOfBar=100,interval="D")
                                                                                                                                          data[instrument]= {t: obs[t]['c']for t in obs}
                                                                                                                                        df = pd.DataFrame(data)
                           Full Script                                                                                                  df = df.fillna(method='ffill')
                                                                                                                                        df = df.dropna()
                                                                                                                                        self.evt.consoleLog("df=",df)
                                                                                                                                        # cointegrationmatrix and pair
                                                                                                                                        cointegrated_pairs, coint_df= find_cointegration_pair(instruments,df, threshold=0.05)
                                                                                                                                        self.evt.consoleLog("coint_df=",coint_df)                                                                 # check order exit
                                                                                                                                        self.evt.consoleLog("\nCointegratedPairs:")                                                               pairs = {}
                                                                                                                                        for pair in cointegrated_pairs:                                                                           pos, osOrder, pendOrder = self.evt.getSystemOrders()
        from AlgoAPI import AlgoAPIUtil, AlgoAPI_Backtest                                                                                 self.evt.consoleLog(pair)                                                                               for tradeID in osOrder:
        import pandas as pd                                                                                                             # Find hedge ratios and trading signals                                                                      order = osOrder[tradeID]
        import numpy as np                                                                                                              for stock1,stock2 in cointegrated_pairs:                                                                     instrument = order['instrument']
        import statsmodels.api as sm                                                                                                      self.pair_id += 1                                                                                          pair_id = order['orderRef']
        from statsmodels.tsa.stattools                                                                                                    # Fit linear regression to find hedge ratio                                                                buysell = order['buysell']
        import coint                                                                                                                      Y=df[stock1]
                                                                                                                                          X=df[stock2]                                                                                               volume = order['Volume']
                                                                                                                                          X=sm.add_constant(X)                                                                                       openprice = order['openprice']
        def find_cointegration_pair(instruments, df, threshold=0.05):                                                                     model=sm.OLS(Y,X).fit()                                                                                    mktPrice = bd[instrument]['bidPrice'] if buysell==1 else bd[instrument]['askPrice']
           n = len(instruments)                                                                                                           hedge_ratio= model.params[1]                                                                               if pair_id not in pairs:
           coint_matrix = np.zeros((n, n))                                                                                                # Calculate the spread and Z-score                                                                            pairs[pair_id] = {"entry_spread":0, "current_spread":0, "count":0, “pl":0}
           for i in range(n):                                                                                                             spread= Y- hedge_ratio* df[stock2]                                                                         pairs[pair_id]["count"]+=1
              for j in range(i + 1, n):                                                                                                   z_score = (spread- spread.mean()) / spread.std()                                                           pairs[pair_id][“pl"]+=pl
                 p_value = coint(df[instruments[i]], df[instruments[j]])[1]                                                               self.evt.consoleLog('debug ... z_score=',z_score[-1])                                                      pairs[pair_id]["entry_spread"]+= -1*buysell*volume*openprice
                 coint_matrix[i, j] = p_value                                                                                             # short stock1 and long stock2                                                                             pairs[pair_id]["current_spread"]+= buysell*volume*mktPrice
           cointegrated_pairs = [(instruments[i], instruments[j]) for i in range(n) for j in range(i + 1, n) if                           if z_score[-1] > self.entry_threshold:                                                                  for pair_id in pairs:
        coint_matrix[i, j] < threshold]                                                                                                     order1=AlgoAPIUtil.OrderObject(                                                                          if pairs[pair_id]["count"]==2 and ( \
           return cointegrated_pairs, coint_matrix                                                                                            instrument= stock1,
                                                                                                                                              openclose='open',                                                                                         (pairs[pair_id]["entry_spread"]>0 and pairs[pair_id]["current_spread"]<=0) or \
                                                                                                                                              buysell= -1, #sell                                                                                        (pairs[pair_id]["entry_spread"]<0 and pairs[pair_id]["current_spread"]>=0) \
        class AlgoEvent:                                                                                                                      ordertype= 0, #market order                                                                            ) and pairs[pair_id][“pl"] >0:
           def __init__(self):                                                                                                                volume=1/self.contractSize[stock1]*self.order_size,#1 share of stock1                                     order = AlgoAPIUtil.OrderObject(
              self.order_size = 100                                                                                                           orderRef= self.pair_id
                                                                                                                                            )                                                                                                              openclose = 'close',
              self.entry_threshold = 1                                                                                                      self.evt.sendOrder(order1)                                                                                     orderRef = pair_id
              self.pair_id = 0                                                                                                              order2=AlgoAPIUtil.OrderObject(                                                                             )
              self.contractSize = {}                                                                                                          instrument= stock2,                                                                                       self.evt.sendOrder(order)
                                                                                                                                              openclose='open',
                                                                                                                                              buysell= 1 if hedge_ratio>0else -1,
           def start(self, mEvt):                                                                                                             ordertype= 0, #market order
              self.evt = AlgoAPI_Backtest.AlgoEvtHandler(self, mEvt)                                                                          volume=hedge_ratio/self.contractSize[stock2]*self.order_size,
              for instrument in mEvt['subscribeList']:                                                                                        orderRef= self.pair_id
                                                                                                                                            )
                 self.contractSize[instrument] = self.evt.getContractSpec(instrument)["contractSize"]                                       self.evt.sendOrder(order2)
              self.evt.start()                                                                                                            # long stock1 and short stock2
                                                                                                                                          elif z_score[-1]< -1*self.entry_threshold:
                                                                                                                                            order1=AlgoAPIUtil.OrderObject(
                                                                                                                                              instrument= stock1,
                                                                                                                                              openclose='open',
                                                                                                                                              buysell= 1, #sell
                                                                                                                                              ordertype= 0, #market order
                                                                                                                                              volume=1/self.contractSize[stock1]*self.order_size,#1 share of stock 1
                                                                                                                                              orderRef= self.pair_id
                                                                                                                                            )
                                                                                                                                            self.evt.sendOrder(order1)
                                                                                                                                            order2=AlgoAPIUtil.OrderObject(
                                                                                                                                              instrument= stock2,
                                                                                                                                              openclose='open',
                                                                                                                                              buysell= -1 if hedge_ratio>0else 1,
                                                                                                                                              ordertype= 0, #market order
                                                                                                                                              volume=hedge_ratio/self.contractSize[stock2]*self.order_size,
                                                                                                                                              orderRef= self.pair_id
                                                                                                                                            )
                                                                                                                                            self.evt.sendOrder(order2)
  Backtest Result
  Backtest Result
    Real Market Challenges
    • Model Risk: Incorrect assumptions or overfitting can lead to poor performance.
    • Execution Risk: Slippage and latency can affect the profitability of trades.
    • Market Risk: Unforeseen market events can disrupt statistical relationships.
    • Regulatory Risk: Changes in market regulations can impact strategy viability. (eg. 
     short selling, leverage requirement, etc)
    Wrap Up
    • Statistical arbitrage is a popular strategy used by many quant trading 
     firms that effectively make profits over years. 
    • From statistics point of view, it has a high probability of winning
    • To run this strategy effectively, 
     • Select the group of assets that are cointegrated
     • Try different observation period for cointegration analysis
     • Choose a distance measure to determine optimal entry and exit points
     • Data transformation of the original price series may increase significance of 
      cointegration
     • May worth to rebalance portfolio due to hedge ratio changes
    Overview of FX Market
    Foreign Exchange Market
    • The Foreign Exchange market, is also called FX or forex, is the largest 
     asset class in terms of daily trading volume
    • It has the following properties:
      1. Decentralized market
      2. Long Trading Hour
      3. High Turnover
      4. Low Spread
      5. Rollover Swap
      6. High Leverage
     Centralized vs Decentralized Market
     • FX market operates by the market itself. There is no single entity responsible for 
      central clearing. 
     • More price transparency and less market manipulation
    Long Trading Hour
    • FX market operates in 24*5.5
    • It starts trading from 21:00 GMT in Sydney on Sunday to 20:00 GMT 
     in New York on Friday
      • i.e. Monday 5am (HKT) to Saturday 4am (HKT)
    • More trading opportunities 
    • Less market gap and price jumps
    High Turnover
    • Global equity market: $3.93 
     trillion
    • Forex: $6.6 trillion
    Low Spread
    • For EURUSD, the percentage spread is 
        (1.11376 –1.11364)/((1.11376+1.11364)/2 ) = 0.0108%
    Low Spread
    • For TSLA, the percentage spread is 
     (428.05 – 426.73)/((428.05 + 426.73)/2 ) = 0.031%
    Rollover Swap
    • FX trading involves buying 1 currency and selling 
     another currency
    • You will receive interest from the currency you buy; 
     and pay interest for the currency you sell
    • If the net interest is positive, you will have positive 
     carry for holding your position overnight. 
    • The net interest rate is called <overnight swap=, or 
     simply <swap=
    • The swap rates are updating every day based on the 
     funding costs the FX brokers need to maintain its risks 
     and operations
    • For the same instrument, the swap rates can be 
     different for different FX brokers
    Swap Schedule
    • The swap payment, if any, will be 
     settled in your broker account every 
     day.  
    • To handle market closure in 
     weekends, the swap fee will be 
     calculated 3 times on Wednesday
    • That means if you have overnight 
     position on Wednesday, you will 
     receive/pay 3 times of the swap fee. 
   High Leverage
   • Compared to stock market, FX is less regulated due to the 
    decentralization property
   • Many FX brokers can allow users to trade at 100x leverage. Some 
    even supports 5000x
   • For a Hong Kong regulated FX broker, investors can only trade up to 5x 
    leverage
   Market Quotation
   • In FX market, it is usually quoted as XXXYYY
   • It means that for 1 unit of XXX, how much YYY can you get?
   Market Quotation
   • Some brokers may quote in the form of YYY/XXX 
   • It means that for 1 unit of XXX, how much YYY can you get?
    Whichcurrency do I buy?
    • For stock, a price quotation is $xxx / 1 share of yyy
    • For currency, a price is quoted in the amount of currency XXX / 1 unit 
     of currency YYY. 
      • 7.79 simple means 7.79 HKD / 1 USD
      Mostly Traded Currencies
                                               Sum of column = 200%
              Reference: https://en.wikipedia.org/wiki/Template:Most_traded_currencies
    Trading USDHKD Peg
                  Sure Win???
   What is a Currency Peg?
   • Definition: A currency peg is a policy where a country's currency 
    value is tied or fixed to another major currency.
   • Purpose: To stabilize the exchange rate and provide predictability for 
    international trade.
    The USDHKD Peg
    • In 1983, the Hong Kong Gov introduced to peg the exchange rate with 
     USD.
    • The Hong Kong Dollar (HKD) is pegged to the US Dollar (USD) at 
     approximately 7.8 HKD to 1 USD.
    • The Hong Kong Monetary Authority (HKMA) maintains this peg 
     through market interventions.
          Pros & Cons of USDHKD peg
            Pros                                               Cons
            Stability: Reduces exchange rate volatility for    Loss of Monetary Policy Autonomy: HKMA 
            businesses and investors.                          cannot freely adjust interest rates.
            Trade Facilitation: Simplifies international       Vulnerability to External Shocks: Economic 
            trade with the US                                  issues in the US can impact Hong Kong.
            Inflation Control: Helps maintain low inflation  Speculative Attacks: Potential for market 
            rates in Hong Kong                                 speculation against the peg.
    The USDHKD Peg
    • Allows fluctuation between 7.75 and 7.85
    • If the exchange rate is approaching 7.75, that means HKD is getting 
     strong (or USD is weak). HKMA will sell HKD and buy USD in the 
     market
    • If the exchange rate is approaching 7.85, that means HKD is getting 
     weak (or USD is strong). HKMA will buy HKD and sell USD in the 
     market
    How retail investors can capture the opportunities?
    • You want to buy around 7.75 and sell around 7.85 to capture the 0.1 
    profits
    • Suppose you have 1m HKD at the beginning, 
     • At 7.75, you should sell all HKD to get 1m/7.75 = 129,032 USD
     • At 7.85, you then sell all USD to get 129,032*7.85 = 1,012,903 HKD
     Is the USDHKD peg strategy safe?
     • Can international hedge funds still attack 
      the peg mechanism just like Asia Financial 
      Crisis in 1997?
     • What if HKMA run out of capital to 
      interfere the market?
                                           George Soros 
     Is the USDHKD peg strategy safe?
     • If HKD is depreciating toward 7.85, that means the market is dumping HKD. 
     • HKMA has sufficient reserve to buy back all circulating HKD in the market. 
  Reference: https://www.hkma.gov.hk/eng/news-and-media/press-releases/2026/02/20260206-3/
   Is the USDHKD peg strategy safe?
   • If the exchange rate is moving toward 7.75 (i.e. HKD is appreciating), 
    that means the market has growing demand for HKD. 
   • HKMA can easily print more HKD
    Simple USD/HKD trading strategy
    Trading Logic:
      • Get the current market price of USD/HKD
      • Entry logic:
        • If current position is zero, 
          • If market price is less than 7.76, open a buy order with all available capital
          • If market price is great than 7.84, open a sell order with all available capital
      • Exit logic:
        • If current position is non-zero, 
          • If it previously buy at 7.76 but now rebound to 7.8 or above, close order
          • If it previously sell at 7.84 but now reverse to 7.8 or below, close order
    Backtest
    • Settings
      • Instrument: USDHKD
      • Period: 2020-01 to 2024-12
      • Enable short-sell: YES
      • Leverage: 1x
      • Data Interval: 1-day
      • Transaction Cost: 0
      • Initial Capital: 100k HKD
     Backtest
     • Define variables at initialization
       class AlgoEvent:
         def __init__(self):
          self.thre_b = 7.76
          self.thre_s = 7.84
          self.thre_exit = 7.8
          self.timer = datetime(1970,1,1)
     Backtest
     • Get contract size for USDHKD
     • In forex market, 1 lot = 100,000 unit of settle currency (i.e. HKD)
       def start(self, mEvt):
         self.evt = AlgoAPI_Backtest.AlgoEvtHandler(self, mEvt)
         # get contract size of USDHKD
         instrument = mEvt['subscribeList'][0]
         spec = self.evt.getContractSpec(instrument)
         self.contractSize = spec["contractSize"]
         self.evt.start()
     Backtest
     • Get current position
       def on_marketdatafeed(self, md, ab):
        if md.timestamp>=self.timer+timedelta(hours=24):
         self.timer = md.timestamp
         # get current position
         pos, osOrder, pendOrder = self.evt.getSystemOrders()
         position = pos[md.instrument]["netVolume"]
                           if position==0:
     Backtest               # calculate the max lot can trade based on current balance
                            lotSize = ab['availableBalance']/self.contractSize/md.askPrice
                            lotSize = round( 0.01*floor(100*lotSize), 2)
     • Order Entry          if md.askPrice<=self.thre_b:
                             order = AlgoAPIUtil.OrderObject(
                              instrument = md.instrument,
                              buysell = 1,
                              ordertype = 0, #market order
                              openclose = 'open9,
                              volume = lotSize,
                              orderRef = 'Long9
                             )
                             self.evt.sendOrder(order)
                            elif md.bidPrice>=self.thre_s:
                             order = AlgoAPIUtil.OrderObject(
                              instrument = md.instrument,
                              buysell = -1,
                              ordertype = 0, #market order
                              openclose = 'open9,
                              volume = lotSize,
                              orderRef = 'Short9
                             )
                             self.evt.sendOrder(order)
     Backtest
     • Order Exit
                        # order exit
                        elif position>0 and md.bidPrice>=self.thre_exit:
                          order = AlgoAPIUtil.OrderObject(
                           openclose = 'close9,
                           orderRef = 'Long9
                          )
                          self.evt.sendOrder(order)
                        elif position<0 and md.askPrice<=self.thre_exit:
                          order = AlgoAPIUtil.OrderObject(
                           openclose = 'close9,
                           orderRef = 'Short9
                          )
                          self.evt.sendOrder(order)
                                                                 def on_marketdatafeed(self, md, ab):
                                                                  if md.timestamp>=self.timer+timedelta(hours=24):
                                                                   self.timer = md.timestamp
                                                                   # get current position
         Backtest                                                  pos, osOrder, pendOrder = self.evt.getSystemOrders()
                                                                   position = pos[md.instrument]["netVolume"] # order entry
                                                                   if position==0:
                                                                    # calculate the max lot can trade based on current balance
                                                                    lotSize = ab['availableBalance']/self.contractSize/md.askPrice
         • Full script                                              lotSize = round( 0.01*floor(100*lotSize), 2)
                                                                    if md.askPrice<=self.thre_b:
                                                                     order = AlgoAPIUtil.OrderObject(
                                                                      instrument = md.instrument,
                                                                      buysell = 1,
           from AlgoAPI import AlgoAPIUtil, AlgoAPI_Backtest          ordertype = 0, #market order
           from datetime import datetime, timedelta                   openclose= 'open9,
           from math import floor                                     volume=lotSize,
                                                                      orderRef = 'Long9
                                                                     )
           class AlgoEvent:                                          self.evt.sendOrder(order)
            def __init__(self):                                     elif md.bidPrice>=self.thre_s:
             self.thre_b = 7.76                                      order = AlgoAPIUtil.OrderObject(
             self.thre_s = 7.84                                       instrument = md.instrument,
             self.thre_exit = 7.8                                     buysell = -1,
             self.timer = datetime(1970,1,1)                          ordertype = 0, #market order
                                                                      openclose= 'open9,
            def start(self, mEvt):                                    volume=lotSize,
                                                                      orderRef = 'Short9
             self.evt = AlgoAPI_Backtest.AlgoEvtHandler(self, mEvt)  )
                                                                     self.evt.sendOrder(order)
             # get contract size of USDHKD                         # order exit
             instrument = mEvt['subscribeList'][0]                 elif position>0 and md.bidPrice>=self.thre_exit:
             spec = self.evt.getContractSpec(instrument)            order = AlgoAPIUtil.OrderObject(
             self.contractSize = spec["contractSize"]                openclose= 'close9,
             self.evt.start()                                        orderRef = 'Long9
                                                                    )
                                                                    self.evt.sendOrder(order)
                                                                   elif position<0 and md.askPrice<=self.thre_exit:
                                                                    order = AlgoAPIUtil.OrderObject(
                                                                     openclose= 'close9,
                                                                     orderRef = 'Short9
                                                                    )
                                                                    self.evt.sendOrder(order)
  Backtest Results
  Return is too small
    Some ways to improve the strategy
    • Earlier entry so that it can trade more
    • Exit earlier such that the holding period can be shorter
    • Split the capital and trade at a different price level, eg.
      • 0.1 lot at 7.77
      • 0.2 lot at 7.76
      • 0.3 lot at 7.75
    • Engage in leverage trading
      Is it executable in the market?
      • Max profit = 7.85 / 7.75 – 1 = 1.29%
      • The percentage spread = 2*(7.8246 – 7.7568)/(7.8246 + 7.7568) = 0.87% 
  Reference: https://www.hsbc.com.hk/investments/products/foreign-exchange/currency-rate/
   How about other CFD brokers?
   • The percentage spread = 2*(7.79213 - 7.78938)/(7.8938 + 7.79213) = 
    0.035% 
    High Swap Fee
    • For long position, we are paying 
     9.3/100000 = 0.0093% every day
      • So 1.29% / 0.0093% = 139 days
    • For short position, we are paying 0.0194% 
     every day
      • So 1.29% / 0.0194% = 66 days
         Wrap Up
         • Without leverage, the maximum profit you can get is 7.85/7.75 – 1 = 1.29%
         • It usually takes a few months for the price to rebound.
         • For Spot market, 
              • Due to physical cash exchange, no leverage involved.                        Not profitable
              • The bid-ask spread are also too wide for currency exchanges. 
         • For CFD market, 
              • We can take leverage 
              • Acceptable bid-ask spread                                Not as profitable as expected
              • However overnight swap fee is too high
   Final Conclusion
   • This strategy has a high winning rate.
   • But it requires a good market timing for entry 
    and exit. 
   • Should keep your holding time short, preferably 
    within a few days.
   • No free lunch in the world!!!
    Key Takeaways
    • Statistical arbitrage is a type of strategy commonly used by many hedge funds 
     and investment banks. 
    • Co-integration is used to identify the basket of suitable instruments that can 
     construct a stationary time series
    • Correlation measures short terms relation while cointegration for long terms
    • FX Market has its unique properties and features
    • Discuss the risks and opportunities of the USD-HKD arbitrage 

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
