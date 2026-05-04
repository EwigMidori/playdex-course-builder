请基于以下材料，生成一份 lesson 级 MDX 课本。

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
- 每个主要 `##` section 必须在 `sectionMap` / `coverageTrace` 中有显式 `sectionId`，并与标题对齐
- 不要输出 `## 标题 {#section-id}` 这种 heading anchor 语法；这不是稳定的 MDX
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

<QUESTION_BANK>
[
{
  "chapter_id": "lecture-5",
  "course_id": "comp7415a",
  "coverage_map": [
    {
      "coverage_tag": "cadf_test",
      "covered_by": [
        "qf_flash_cadf_steps",
        "qf_quiz_cadf_interpretation",
        "qf_long_cadf_process"
      ],
      "description": "协整增强迪基-富勒检验（CADF）的步骤和原理"
    },
    {
      "coverage_tag": "johansen_test",
      "covered_by": [
        "qf_flash_johansen_stats",
        "qf_quiz_johansen_rank",
        "qf_long_johansen_interpretation"
      ],
      "description": "约翰森检验的用途、统计量和结果解读"
    },
    {
      "coverage_tag": "hedge_ratio_determination",
      "covered_by": [
        "qf_flash_cadf_steps",
        "qf_quiz_cadf_interpretation"
      ],
      "description": "通过线性回归确定对冲比率β"
    },
    {
      "coverage_tag": "adf_test_on_spread",
      "covered_by": [
        "qf_flash_cadf_steps",
        "qf_quiz_cadf_interpretation"
      ],
      "description": "对价差序列进行ADF平稳性检验"
    },
    {
      "coverage_tag": "cointegration_rank_r",
      "covered_by": [
        "qf_flash_johansen_stats",
        "qf_quiz_johansen_rank",
        "qf_long_johansen_interpretation"
      ],
      "description": "协整关系个数r的含义（r=0 vs r>0）"
    },
    {
      "coverage_tag": "trace_statistic",
      "covered_by": [
        "qf_flash_johansen_stats",
        "qf_quiz_johansen_rank"
      ],
      "description": "迹统计量的用途：检验协整向量个数是否小于等于r"
    },
    {
      "coverage_tag": "max_eigenvalue_statistic",
      "covered_by": [
        "qf_flash_johansen_stats",
        "qf_quiz_johansen_rank"
      ],
      "description": "最大特征值统计量的用途：检验恰好有r个协整向量"
    },
    {
      "coverage_tag": "eigenvectors_as_hedge_ratios",
      "covered_by": [
        "qf_long_johansen_interpretation"
      ],
      "description": "约翰森检验的特征向量可作为多资产组合的对冲比率"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "cadf_test",
      "coverage_tags": [
        "cadf_test",
        "hedge_ratio_determination",
        "adf_test_on_spread"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_cadf_steps",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能准确回忆CADF检验的三个主要步骤及其顺序。",
      "linked_steps": [
        "step4"
      ],
      "practice_target": "学生答对后证明掌握了CADF检验的标准流程：先回归求β，再构建价差，最后做ADF检验。",
      "question_type": "short_answer",
      "retrieval_focus": "CADF检验的步骤顺序和每个步骤的核心操作。",
      "term_refs": [
        {
          "display": "协整增强迪基-富勒检验",
          "en": "Cointegrated Augmented Dickey Fuller Test (CADF)"
        },
        {
          "display": "对冲比率",
          "en": "Hedge Ratio"
        },
        {
          "display": "价差",
          "en": "Spread"
        }
      ],
      "variants": [
        {
          "back": "对两只股票的价格做线性回归，得到对冲比率β。",
          "estimated_seconds": 8,
          "explanation": "线性回归用于确定最优对冲比率，这是构建平稳组合的基础。",
          "front": "CADF检验的第一步是什么？",
          "question_id": "q_flash_cadf_steps_v1"
        },
        {
          "back": "用β构建价差序列 $z_t = y_t - \\beta x_t$。",
          "estimated_seconds": 8,
          "explanation": "价差序列是两只股票价格经对冲比率调整后的差值，用于后续的平稳性检验。",
          "front": "CADF检验中，得到对冲比率β后，下一步做什么？",
          "question_id": "q_flash_cadf_steps_v2"
        },
        {
          "back": "对价差序列进行ADF平稳性检验。",
          "estimated_seconds": 8,
          "explanation": "ADF检验用于判断价差序列是否平稳，从而推断原始序列是否协整。",
          "front": "CADF检验的最后一步是什么？",
          "question_id": "q_flash_cadf_steps_v3"
        }
      ]
    },
    {
      "concept_key": "johansen_test",
      "coverage_tags": [
        "johansen_test",
        "trace_statistic",
        "max_eigenvalue_statistic",
        "cointegration_rank_r"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_johansen_stats",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能准确回忆约翰森检验的两个统计量及其检验的假设。",
      "linked_steps": [
        "step4"
      ],
      "practice_target": "学生答对后证明掌握了迹统计量和最大特征值统计量的核心区别：一个检验≤r，一个检验=r。",
      "question_type": "short_answer",
      "retrieval_focus": "约翰森检验中两个统计量的名称和各自检验的原假设。",
      "term_refs": [
        {
          "display": "约翰森检验",
          "en": "Johansen Test"
        },
        {
          "display": "迹统计量",
          "en": "Trace Statistic"
        },
        {
          "display": "最大特征值统计量",
          "en": "Maximum Eigenvalue Statistic"
        },
        {
          "display": "协整关系个数",
          "en": "Cointegration Rank (r)"
        }
      ],
      "variants": [
        {
          "back": "协整向量的个数小于或等于r。",
          "estimated_seconds": 10,
          "explanation": "迹统计量从累积角度检验协整关系个数是否不超过某个值。",
          "front": "约翰森检验中，迹统计量检验的原假设是什么？",
          "question_id": "q_flash_johansen_stats_v1"
        },
        {
          "back": "恰好存在r个协整向量。",
          "estimated_seconds": 10,
          "explanation": "最大特征值统计量逐个检验第r+1个特征值是否为零，判断是否存在恰好r个协整关系。",
          "front": "约翰森检验中，最大特征值统计量检验的原假设是什么？",
          "question_id": "q_flash_johansen_stats_v2"
        },
        {
          "back": "没有协整关系，时间序列的任何线性组合都不平稳。",
          "estimated_seconds": 8,
          "explanation": "r=0表示无法找到平稳的线性组合，资产之间不存在长期均衡关系。",
          "front": "约翰森检验中，如果r=0意味着什么？",
          "question_id": "q_flash_johansen_stats_v3"
        }
      ]
    }
  ],
  "lesson_id": "L5",
  "longform_families": [
    {
      "concept_key": "cadf_test",
      "coverage_tags": [
        "cadf_test",
        "hedge_ratio_determination",
        "adf_test_on_spread"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_cadf_process",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能完整解释CADF检验的流程、每一步的目的以及如何根据结果判断协整性。",
      "linked_steps": [
        "step4"
      ],
      "practice_target": "学生答对后证明能系统阐述CADF检验的完整逻辑链，包括回归求β、构建价差、ADF检验和结果解读。",
      "question_type": "mechanism_trace",
      "term_refs": [
        {
          "display": "协整增强迪基-富勒检验",
          "en": "CADF"
        },
        {
          "display": "对冲比率",
          "en": "Hedge Ratio"
        },
        {
          "display": "价差",
          "en": "Spread"
        },
        {
          "display": "ADF检验",
          "en": "Augmented Dickey Fuller Test"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "第一步的目的和操作",
            "第二步的目的和操作",
            "第三步的目的和操作",
            "结果判断标准"
          ],
          "question_id": "q_long_cadf_process_v1",
          "reference_answer": [
            "第一步：对两只股票的价格序列进行线性回归，得到对冲比率β。目的是确定构建平稳组合时两种资产的最优比例。",
            "第二步：用得到的β构建价差序列 $z_t = y_t - \\beta x_t$。价差代表了两只股票价格经对冲调整后的差值。",
            "第三步：对价差序列进行ADF平稳性检验。如果p值小于0.05，拒绝'序列不平稳'的原假设，认为价差是平稳的。",
            "结论：如果价差序列平稳，则两只股票的价格序列是协整的，存在长期均衡关系。"
          ],
          "rubric_points": [
            "正确指出第一步是线性回归，目的是确定最优对冲比率β",
            "正确指出第二步是用β构建价差序列 $z_t = y_t - \\beta x_t$",
            "正确指出第三步是对价差序列进行ADF平稳性检验",
            "正确说明判断标准：p值<0.05则拒绝原假设，认为价差平稳，原始序列协整",
            "逻辑连贯，术语使用准确"
          ],
          "stem": "请详细说明协整增强迪基-富勒检验（CADF）的完整步骤，并解释每一步的目的。最后说明如何根据检验结果判断两只股票是否协整。"
        }
      ]
    },
    {
      "concept_key": "johansen_test",
      "coverage_tags": [
        "johansen_test",
        "cointegration_rank_r",
        "trace_statistic",
        "max_eigenvalue_statistic",
        "eigenvectors_as_hedge_ratios"
      ],
      "difficulty": "hard",
      "family_id": "qf_long_johansen_interpretation",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能解释约翰森检验的两个统计量的区别，以及如何利用检验结果构建多资产组合的对冲比率。",
      "linked_steps": [
        "step4"
      ],
      "practice_target": "学生答对后证明能区分迹统计量和最大特征值统计量的检验假设，并理解r>0时特征向量作为对冲比率的实际用途。",
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "约翰森检验",
          "en": "Johansen Test"
        },
        {
          "display": "迹统计量",
          "en": "Trace Statistic"
        },
        {
          "display": "最大特征值统计量",
          "en": "Maximum Eigenvalue Statistic"
        },
        {
          "display": "协整关系个数",
          "en": "Cointegration Rank (r)"
        },
        {
          "display": "特征向量",
          "en": "Eigenvectors"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "两个统计量的原假设区别",
            "r=2的实际含义",
            "特征向量的作用"
          ],
          "question_id": "q_long_johansen_interpretation_v1",
          "reference_answer": [
            "迹统计量检验原假设H0: 协整向量个数 ≤ r，最大特征值统计量检验H0: 恰好有r个协整向量。两者从不同角度判断r的值。",
            "r=2意味着在多个资产中存在2个独立的平稳线性组合，即可以构建2个不同的市场中性投资组合。",
            "特征向量就是这些平稳组合的系数，每个特征向量对应一组对冲比率。例如，对于4只股票，一个特征向量 [1, -0.5, -0.3, -0.2] 表示买入1单位股票A，同时卖出0.5单位B、0.3单位C和0.2单位D来构建平稳组合。"
          ],
          "rubric_points": [
            "正确说明迹统计量检验协整向量个数≤r，最大特征值统计量检验恰好有r个",
            "正确解释r=2意味着存在2个独立的平稳线性组合",
            "正确说明特征向量可作为各资产的对冲比率，用于构建市场中性组合",
            "逻辑清晰，术语使用准确"
          ],
          "stem": "请解释约翰森检验中迹统计量和最大特征值统计量的区别。如果检验结果显示r=2，这对构建配对交易策略意味着什么？特征向量在此过程中起什么作用？"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "cadf_test",
      "coverage_tags": [
        "cadf_test",
        "hedge_ratio_determination",
        "adf_test_on_spread"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_cadf_interpretation",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能在测验情境下正确解读CADF检验的p值结果。",
      "linked_steps": [
        "step4"
      ],
      "practice_target": "学生答对后证明能根据p值判断是否拒绝原假设，从而得出序列是否协整的结论。",
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "协整增强迪基-富勒检验",
          "en": "CADF"
        },
        {
          "display": "p值",
          "en": "p-value"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "p值0.03 < 0.05，拒绝'不平稳'的原假设，认为价差序列平稳，从而两只股票的价格序列是协整的。",
          "options": [
            "在5%显著性水平下，不能拒绝原假设，序列不平稳",
            "在5%显著性水平下，拒绝原假设，序列平稳，因此原始序列协整",
            "p值大于0.05，说明价差序列是平稳的",
            "需要重新计算对冲比率"
          ],
          "question_id": "q_quiz_cadf_interpretation_v1",
          "stem": "在CADF检验中，对价差序列进行ADF检验后得到p值为0.03。以下哪个结论是正确的？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "p值0.08 > 0.05，不能拒绝原假设，价差序列不平稳，因此两只股票不协整。",
          "options": [
            "价差序列平稳，两只股票协整",
            "价差序列不平稳，两只股票不协整",
            "需要增加样本量重新检验",
            "对冲比率计算错误"
          ],
          "question_id": "q_quiz_cadf_interpretation_v2",
          "stem": "CADF检验中，如果对价差序列的ADF检验p值为0.08，在5%显著性水平下应如何解读？"
        }
      ]
    },
    {
      "concept_key": "johansen_test",
      "coverage_tags": [
        "johansen_test",
        "cointegration_rank_r",
        "trace_statistic",
        "max_eigenvalue_statistic"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_johansen_rank",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能根据约翰森检验的结果判断协整关系个数，并理解其含义。",
      "linked_steps": [
        "step4"
      ],
      "practice_target": "学生答对后证明能根据迹统计量或最大特征值统计量与临界值的比较，确定r的值并解释其意义。",
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "约翰森检验",
          "en": "Johansen Test"
        },
        {
          "display": "协整关系个数",
          "en": "Cointegration Rank (r)"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 30,
          "explanation": "迹统计量55.2 > 47.9（r≤0被拒绝），30.1 > 29.8（r≤1被拒绝），12.3 < 15.5（r≤2不能被拒绝），因此r=2。注意：这里r表示协整向量个数，当r≤2不能被拒绝时，说明存在2个协整关系。",
          "options": [
            "0",
            "1",
            "2",
            "3"
          ],
          "question_id": "q_quiz_johansen_rank_v1",
          "stem": "约翰森检验对4只股票进行检验，迹统计量序列为[55.2, 30.1, 12.3, 3.1]，5%临界值为[47.9, 29.8, 15.5, 3.8]。协整关系个数r是多少？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "r>0表示存在r个协整关系，即r个独立的平稳线性组合，其对应的特征向量就是各资产的对冲比率。",
          "options": [
            "不存在任何协整关系",
            "存在3个独立的平稳组合，对应的特征向量可作为对冲比率",
            "所有股票价格序列都是平稳的",
            "只能构建一个投资组合"
          ],
          "question_id": "q_quiz_johansen_rank_v2",
          "stem": "约翰森检验中，如果r=3，以下哪个说法正确？"
        }
      ]
    }
  ],
  "sequence_id": "step4",
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L5/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "input source outline"
  },
  "target_language": "zh-CN"
}
,
{
  "chapter_id": "lecture-5",
  "course_id": "comp7415a",
  "coverage_map": [
    {
      "coverage_tag": "cointegration_definition",
      "covered_by": [
        "qf_flash_coint_def",
        "qf_quiz_coint_def"
      ],
      "description": "协整的定义：两个或多个非平稳时间序列的线性组合是平稳的，表明存在长期均衡关系。"
    },
    {
      "coverage_tag": "cointegration_formula",
      "covered_by": [
        "qf_flash_coint_formula",
        "qf_quiz_coint_formula"
      ],
      "description": "协整的数学公式：z_t = y_t - β x_t 是平稳的。"
    },
    {
      "coverage_tag": "hedge_ratio",
      "covered_by": [
        "qf_flash_hedge_ratio",
        "qf_quiz_hedge_ratio"
      ],
      "description": "对冲比率β的含义：构建市场中性组合时每单位X对应Y的数量。"
    },
    {
      "coverage_tag": "spread_definition",
      "covered_by": [
        "qf_flash_spread_def"
      ],
      "description": "价差的定义：配对交易中两只资产价格经对冲比率调整后的差值。"
    },
    {
      "coverage_tag": "stationarity",
      "covered_by": [
        "qf_flash_stationarity"
      ],
      "description": "平稳性的概念：时间序列的统计性质（如均值、方差）不随时间变化。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "cointegration",
      "coverage_tags": [
        "cointegration_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_coint_def",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能用自己的话准确说出协整的定义。",
      "linked_steps": [
        "step2"
      ],
      "practice_target": "学生答对后证明掌握了协整的核心定义：非平稳序列的线性组合是平稳的，代表长期均衡关系。",
      "question_type": "short_answer",
      "retrieval_focus": "协整的定义：非平稳序列的线性组合是平稳的，代表长期均衡关系。",
      "term_refs": [
        {
          "display": "协整",
          "en": "Cointegration"
        }
      ],
      "variants": [
        {
          "back": "长期均衡关系。",
          "estimated_seconds": 8,
          "explanation": "协整的核心是长期均衡：即使单个序列不平稳，它们的线性组合却是平稳的，意味着它们长期会一起运动。",
          "front": "协整描述的是两个或多个时间序列之间的什么关系？",
          "question_id": "q_flash_coint_def_v1"
        },
        {
          "back": "平稳性。",
          "estimated_seconds": 8,
          "explanation": "协整的定义就是：存在一个线性组合使得组合序列是平稳的。",
          "front": "如果两个非平稳的价格序列是协整的，它们的某个线性组合具有什么性质？",
          "question_id": "q_flash_coint_def_v2"
        }
      ]
    },
    {
      "concept_key": "cointegration_formula",
      "coverage_tags": [
        "cointegration_formula"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_coint_formula",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能写出协整的数学表达式，并理解各符号的含义。",
      "linked_steps": [
        "step2"
      ],
      "practice_target": "学生答对后证明掌握了协整的数学定义：z_t = y_t - β x_t 是平稳的。",
      "question_type": "short_answer",
      "retrieval_focus": "协整的数学表达式：z_t = y_t - β x_t 是平稳的。",
      "term_refs": [
        {
          "display": "协整公式",
          "en": "Cointegration Formula"
        }
      ],
      "variants": [
        {
          "back": "$z_t = y_t - \\beta x_t$ 是平稳的。",
          "estimated_seconds": 10,
          "explanation": "这是协整的核心公式：组合z_t是平稳的，意味着y和x存在长期均衡关系。",
          "front": "写出协整关系的数学表达式，其中y和x是两只资产的价格，β是常数。",
          "question_id": "q_flash_coint_formula_v1"
        },
        {
          "back": "平稳性。",
          "estimated_seconds": 6,
          "explanation": "z_t必须是平稳的，这是协整的定义。",
          "front": "在协整公式 $z_t = y_t - \\beta x_t$ 中，$z_t$ 必须满足什么统计性质？",
          "question_id": "q_flash_coint_formula_v2"
        }
      ]
    },
    {
      "concept_key": "hedge_ratio",
      "coverage_tags": [
        "hedge_ratio"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_hedge_ratio",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能解释对冲比率β在配对交易中的实际含义。",
      "linked_steps": [
        "step2"
      ],
      "practice_target": "学生答对后证明掌握了对冲比率的定义：每买入1股Y，应卖出β股X来构建市场中性组合。",
      "question_type": "short_answer",
      "retrieval_focus": "对冲比率β的含义：每单位Y对应多少单位X。",
      "term_refs": [
        {
          "display": "对冲比率",
          "en": "Hedge Ratio"
        }
      ],
      "variants": [
        {
          "back": "每买入1股Y，应该卖出多少股X来构建市场中性组合。",
          "estimated_seconds": 10,
          "explanation": "β决定了两种资产的比例，使得组合的市场风险被对冲掉。",
          "front": "在配对交易中，对冲比率β告诉你什么？",
          "question_id": "q_flash_hedge_ratio_v1"
        },
        {
          "back": "对冲比率。",
          "estimated_seconds": 6,
          "explanation": "β也称为协整系数，在配对交易中用于确定多空头寸的比例。",
          "front": "协整关系中的β在交易实践中被称为什么？",
          "question_id": "q_flash_hedge_ratio_v2"
        }
      ]
    },
    {
      "concept_key": "spread",
      "coverage_tags": [
        "spread_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_spread_def",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能定义价差并理解其在配对交易中的作用。",
      "linked_steps": [
        "step2"
      ],
      "practice_target": "学生答对后证明掌握了价差的定义：配对交易中两只资产价格经对冲比率调整后的差值。",
      "question_type": "short_answer",
      "retrieval_focus": "价差的定义：y_t - β x_t。",
      "term_refs": [
        {
          "display": "价差",
          "en": "Spread"
        }
      ],
      "variants": [
        {
          "back": "价差 = y_t - β x_t，其中β是对冲比率。",
          "estimated_seconds": 8,
          "explanation": "价差是两只资产价格经过对冲比率调整后的差值，当价差偏离均值时产生交易信号。",
          "front": "在配对交易中，价差（spread）是如何计算的？",
          "question_id": "q_flash_spread_def_v1"
        },
        {
          "back": "价差。",
          "estimated_seconds": 6,
          "explanation": "z_t就是价差，当它偏离历史均值太远时，就出现了交易信号。",
          "front": "协整公式中的 $z_t$ 在配对交易中通常被称为什么？",
          "question_id": "q_flash_spread_def_v2"
        }
      ]
    },
    {
      "concept_key": "stationary",
      "coverage_tags": [
        "stationarity"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_stationarity",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能解释平稳性的基本概念。",
      "linked_steps": [
        "step2"
      ],
      "practice_target": "学生答对后证明掌握了平稳性的定义：时间序列的统计性质（如均值、方差）不随时间变化。",
      "question_type": "short_answer",
      "retrieval_focus": "平稳性的定义：统计性质不随时间变化。",
      "term_refs": [
        {
          "display": "平稳性",
          "en": "Stationarity"
        }
      ],
      "variants": [
        {
          "back": "均值、方差等统计性质。",
          "estimated_seconds": 8,
          "explanation": "平稳序列的统计特征（如均值、方差）是常数，不随时间推移而改变。",
          "front": "一个平稳的时间序列，它的哪些统计性质不随时间变化？",
          "question_id": "q_flash_stationarity_v1"
        },
        {
          "back": "非平稳的。",
          "estimated_seconds": 6,
          "explanation": "大多数金融价格序列是随机游走的，即非平稳的。协整正是利用非平稳序列构建平稳组合。",
          "front": "大多数金融价格序列本身是平稳的还是非平稳的？",
          "question_id": "q_flash_stationarity_v2"
        }
      ]
    }
  ],
  "lesson_id": "L5",
  "longform_families": [
    {
      "concept_key": "cointegration",
      "coverage_tags": [
        "cointegration_definition",
        "cointegration_formula",
        "hedge_ratio",
        "spread_definition",
        "stationarity"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_coint_explain",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能用自己的语言完整解释协整的概念、公式及其在配对交易中的意义。",
      "linked_steps": [
        "step2"
      ],
      "practice_target": "学生答对后证明能综合运用协整的定义、公式、对冲比率和价差等概念，解释协整如何为配对交易提供统计基础。",
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "协整",
          "en": "Cointegration"
        },
        {
          "display": "对冲比率",
          "en": "Hedge Ratio"
        },
        {
          "display": "价差",
          "en": "Spread"
        },
        {
          "display": "平稳性",
          "en": "Stationarity"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "协整的定义",
            "数学表达式 $z_t = y_t - \\beta x_t$",
            "对冲比率β的含义",
            "价差的作用"
          ],
          "question_id": "q_long_coint_explain_v1",
          "reference_answer": [
            "协整是指两个或多个非平稳时间序列之间存在长期均衡关系，使得它们的某个线性组合是平稳的。",
            "数学上，如果存在常数β使得 $z_t = y_t - \\beta x_t$ 是平稳的，则y和x是协整的。",
            "β称为对冲比率，它告诉我们在配对交易中每买入1股Y应该卖出多少股X来构建市场中性组合。",
            "价差就是z_t，当价差偏离历史均值太远时，我们押注它会回归，从而产生交易信号。",
            "协整是配对交易的统计基础，因为它保证了价差具有均值回归的性质，使得交易策略有可盈利的预期。"
          ],
          "rubric_points": [
            "正确给出协整的定义：两个或多个非平稳时间序列的线性组合是平稳的，表明长期均衡关系。",
            "正确写出数学表达式 $z_t = y_t - \\beta x_t$，并说明z_t是平稳的。",
            "解释β是对冲比率，决定了构建市场中性组合时两种资产的比例。",
            "解释价差就是z_t，当价差偏离历史均值时产生交易信号，押注价差回归。",
            "说明协整保证了价差最终会回归均值，这是配对交易盈利的基础。"
          ],
          "stem": "请解释什么是协整（Cointegration），并说明它为什么是配对交易（Pairs Trading）的统计基础。在你的解释中，请包含以下要点：协整的定义、数学表达式、对冲比率（β）的含义、以及价差（spread）的作用。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "A和B之间的统计关系",
            "β=2的实际含义",
            "价差产生交易信号的条件"
          ],
          "question_id": "q_long_coint_explain_v2",
          "reference_answer": [
            "A和B是协整的，意味着它们之间存在长期均衡关系，即使短期偏离，最终也会回归。",
            "β=2是对冲比率，意味着每买入1股A，需要卖出2股B来对冲市场风险，使组合市场中性。",
            "价差z_t偏离历史均值时产生交易信号。通常用Z分数衡量偏离程度，当Z分数超过设定阈值（如±2）时触发交易。",
            "例如，当z_t远高于均值时，说明A相对B被高估，应卖出A、买入B；当z_t远低于均值时，说明A相对B被低估，应买入A、卖出B。"
          ],
          "rubric_points": [
            "正确指出A和B是协整的，存在长期均衡关系。",
            "解释β=2意味着每买入1股A，应卖出2股B来构建市场中性组合。",
            "说明价差z_t偏离其历史均值时产生交易信号，偏离越大信号越强。",
            "说明交易逻辑：当价差过高时卖出A买入B，价差过低时买入A卖出B，押注价差回归均值。"
          ],
          "stem": "假设你发现两只股票A和B的价格序列都是非平稳的，但它们的线性组合 $z_t = A_t - 2B_t$ 是平稳的。请解释：1）这个发现意味着A和B之间有什么统计关系？2）β=2在实际交易中意味着什么？3）价差z_t在什么情况下会产生交易信号？"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "cointegration",
      "coverage_tags": [
        "cointegration_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_coint_def",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能在选择题中准确识别协整的定义。",
      "linked_steps": [
        "step2"
      ],
      "practice_target": "学生答对后证明能区分协整与其他统计概念，准确理解协整描述的是长期均衡关系。",
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "协整",
          "en": "Cointegration"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "协整的定义是：两个或多个非平稳时间序列的线性组合是平稳的，表明它们存在长期均衡关系。高相关性不等于协整。",
          "options": [
            "两个时间序列的相关系数很高",
            "两个或多个非平稳时间序列的线性组合是平稳的",
            "两个时间序列的均值相等",
            "两个时间序列的方差相等"
          ],
          "question_id": "q_quiz_coint_def_v1",
          "stem": "以下哪个选项最准确地描述了协整（Cointegration）？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "协整意味着长期均衡关系：短期可能偏离，但长期会一起运动。单个价格序列通常是非平稳的。",
          "options": [
            "它们的价格总是完全同步变动",
            "它们的价格之间存在长期均衡关系，短期可能偏离但最终会回归",
            "它们的收益率完全正相关",
            "它们的价格序列都是平稳的"
          ],
          "question_id": "q_quiz_coint_def_v2",
          "stem": "如果两只股票的价格序列是协整的，这意味着什么？"
        }
      ]
    },
    {
      "concept_key": "cointegration_formula",
      "coverage_tags": [
        "cointegration_formula"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_coint_formula",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能正确理解协整公式中各部分的意义。",
      "linked_steps": [
        "step2"
      ],
      "practice_target": "学生答对后证明能正确解读协整公式 $z_t = y_t - \\beta x_t$ 中每个符号的含义。",
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "协整公式",
          "en": "Cointegration Formula"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "如果存在β使得z_t平稳，则y_t和x_t是协整的。单个序列y_t和x_t通常是非平稳的。",
          "options": [
            "$y_t$ 和 $x_t$ 都是平稳的",
            "$y_t$ 和 $x_t$ 是协整的",
            "$y_t$ 和 $x_t$ 完全独立",
            "$y_t$ 和 $x_t$ 的相关系数为1"
          ],
          "question_id": "q_quiz_coint_formula_v1",
          "stem": "在协整公式 $z_t = y_t - \\beta x_t$ 中，如果 $z_t$ 是平稳的，那么 $y_t$ 和 $x_t$ 是什么关系？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "β是对冲比率（协整系数），决定了构建平稳组合时两种资产的比例。",
          "options": [
            "两只股票的相关系数",
            "对冲比率",
            "价差的均值",
            "时间趋势项"
          ],
          "question_id": "q_quiz_coint_formula_v2",
          "stem": "协整公式 $z_t = y_t - \\beta x_t$ 中，β代表什么？"
        }
      ]
    },
    {
      "concept_key": "hedge_ratio",
      "coverage_tags": [
        "hedge_ratio"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_hedge_ratio",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能在实际情境中应用对冲比率的概念。",
      "linked_steps": [
        "step2"
      ],
      "practice_target": "学生答对后证明能理解对冲比率β在构建市场中性组合中的实际应用。",
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "对冲比率",
          "en": "Hedge Ratio"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 25,
          "explanation": "β=1.5，意味着每买入1股Y，应卖出1.5股X来对冲。买入100股Y，应卖出150股X。",
          "options": [
            "买入150股X",
            "卖出150股X",
            "买入100股X",
            "卖出100股X"
          ],
          "question_id": "q_quiz_hedge_ratio_v1",
          "stem": "假设两只股票Y和X的协整关系为 $z_t = y_t - 1.5 x_t$，且z_t是平稳的。如果你想买入100股Y来构建市场中性组合，你应该如何处理X？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "β决定了多空头寸的比例，使得组合的市场风险被对冲掉，实现市场中性。",
          "options": [
            "预测市场整体走势",
            "确定多空头寸的比例，使组合市场中性",
            "计算两只股票的相关系数",
            "衡量交易成本"
          ],
          "question_id": "q_quiz_hedge_ratio_v2",
          "stem": "在配对交易中，对冲比率β的主要作用是什么？"
        }
      ]
    }
  ],
  "sequence_id": "step2",
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L5/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "input source outline"
  },
  "target_language": "zh-CN"
}
,
{
  "chapter_id": "lecture-5",
  "course_id": "comp7415a",
  "coverage_map": [
    {
      "coverage_tag": "correlation_vs_cointegration",
      "covered_by": [
        "qf_flash_corr_vs_coint_01",
        "qf_quiz_corr_vs_coint_01",
        "qf_long_corr_vs_coint_01"
      ],
      "description": "区分相关性与协整：相关性衡量短期线性关系，协整衡量长期均衡关系。"
    },
    {
      "coverage_tag": "correlation_definition",
      "covered_by": [
        "qf_flash_corr_vs_coint_01"
      ],
      "description": "相关性的定义：衡量两个变量之间线性关系的强度和方向。"
    },
    {
      "coverage_tag": "cointegration_definition",
      "covered_by": [
        "qf_flash_corr_vs_coint_01"
      ],
      "description": "协整的定义：两个或多个非平稳时间序列的线性组合是平稳的，表明存在长期均衡关系。"
    },
    {
      "coverage_tag": "application_diversification_vs_trading",
      "covered_by": [
        "qf_quiz_corr_vs_coint_01",
        "qf_long_corr_vs_coint_01"
      ],
      "description": "相关性用于分散投资，协整用于寻找可交易的配对。"
    },
    {
      "coverage_tag": "short_term_vs_long_term",
      "covered_by": [
        "qf_flash_corr_vs_coint_01",
        "qf_quiz_corr_vs_coint_01"
      ],
      "description": "相关性关注短期关系，协整关注长期关系。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "correlation_vs_cointegration",
      "coverage_tags": [
        "correlation_vs_cointegration",
        "correlation_definition",
        "cointegration_definition",
        "short_term_vs_long_term"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_corr_vs_coint_01",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能准确区分相关性与协整的核心差异：时间范围（短期 vs 长期）和本质（线性关系 vs 均衡关系）。",
      "linked_steps": [
        "step3"
      ],
      "practice_target": "学生答对后证明掌握了相关性与协整在定义和时间维度上的根本区别。",
      "question_type": "core_difference",
      "retrieval_focus": "区分相关性与协整的核心维度：时间范围（短期 vs 长期）。",
      "term_refs": [
        {
          "display": "相关性",
          "en": "Correlation"
        },
        {
          "display": "协整",
          "en": "Cointegration"
        }
      ],
      "variants": [
        {
          "back": "短期线性关系。",
          "estimated_seconds": 8,
          "explanation": "相关性衡量的是两个变量之间线性关系的强度和方向，它关注的是短期内的同步变动。",
          "front": "相关性衡量的是两个变量之间的什么关系？",
          "question_id": "q_flash_corr_vs_coint_01_v1"
        },
        {
          "back": "长期均衡关系。",
          "estimated_seconds": 8,
          "explanation": "协整表明即使序列本身是非平稳的，它们的线性组合却是平稳的，意味着它们之间存在一个长期稳定的均衡关系。",
          "front": "协整衡量的是两个或多个时间序列之间的什么关系？",
          "question_id": "q_flash_corr_vs_coint_01_v2"
        },
        {
          "back": "协整性。",
          "estimated_seconds": 8,
          "explanation": "协整性保证了价差会均值回归，这是配对交易盈利的基础。高相关性不一定意味着协整。",
          "front": "在配对交易中，寻找可交易配对的关键是看高相关性还是协整性？",
          "question_id": "q_flash_corr_vs_coint_01_v3"
        }
      ]
    }
  ],
  "lesson_id": "L5",
  "longform_families": [
    {
      "concept_key": "correlation_vs_cointegration",
      "coverage_tags": [
        "correlation_vs_cointegration",
        "application_diversification_vs_trading"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_corr_vs_coint_01",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能用自己的语言解释相关性与协整的区别，并说明为什么协整对配对交易至关重要。",
      "linked_steps": [
        "step3"
      ],
      "practice_target": "学生答对后证明能够清晰阐述相关性与协整在定义、时间维度、数学含义和实际应用上的区别，并能用例子说明。",
      "question_type": "compare_and_contrast",
      "term_refs": [
        {
          "display": "相关性",
          "en": "Correlation"
        },
        {
          "display": "协整",
          "en": "Cointegration"
        },
        {
          "display": "配对交易",
          "en": "Pairs Trading"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "分别定义相关性和协整。",
            "指出两者在时间维度上的关键区别。",
            "解释为什么协整是配对交易策略的统计基础。"
          ],
          "question_id": "q_long_corr_vs_coint_01_v1",
          "reference_answer": [
            "相关性衡量的是两个变量之间短期线性关系的强度和方向，取值范围在-1到1之间。",
            "协整则表明两个或多个非平稳时间序列之间存在一个长期均衡关系，它们的线性组合是平稳的。",
            "关键区别在于时间维度：相关性关注短期同步性，而协整关注长期均衡。",
            "对于配对交易，协整比高相关性更重要，因为协整保证了价差（spread）是平稳的、均值回归的。这意味着当价差偏离历史均值时，我们有统计依据押注它会回归，从而获利。而高相关性只说明短期走势相似，但无法保证长期关系，如果两只股票只是高度相关但不协整，它们的价差可能不会回归，导致策略失败。"
          ],
          "rubric_points": [
            "正确指出相关性衡量短期线性关系（1分）。",
            "正确指出协整衡量长期均衡关系（1分）。",
            "明确指出协整保证价差均值回归，这是配对交易盈利的基础（2分）。",
            "能举例说明高相关性但非协整的资产对在配对交易中的风险（1分）。"
          ],
          "stem": "请解释相关性与协整的区别。在构建一个配对交易策略时，为什么协整比高相关性更重要？请用你自己的话说明。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "correlation_vs_cointegration",
      "coverage_tags": [
        "correlation_vs_cointegration",
        "application_diversification_vs_trading",
        "short_term_vs_long_term"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_corr_vs_coint_01",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能在测验情境下辨析相关性与协整在定义、时间范围和实际应用上的区别。",
      "linked_steps": [
        "step3"
      ],
      "practice_target": "学生答对后证明掌握了相关性与协整的本质区别，并能正确应用于分散投资和配对交易场景。",
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "相关性",
          "en": "Correlation"
        },
        {
          "display": "协整",
          "en": "Cointegration"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "相关性衡量短期线性关系，协整衡量长期均衡关系。两者本质不同。高相关性不保证协整，协整也不要求高相关性。",
          "options": [
            "高相关性意味着两个序列一定是协整的。",
            "协整关注的是长期均衡关系，而相关性关注的是短期线性关系。",
            "协整和相关性是同一个概念，只是叫法不同。",
            "协整只适用于平稳的时间序列。"
          ],
          "question_id": "q_quiz_corr_vs_coint_01_v1",
          "stem": "以下关于相关性与协整的说法，哪一个是正确的？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "协整用于寻找具有长期均衡关系的资产对，这是配对交易的基础。相关性更适合用于分散投资。",
          "options": [
            "构建一个分散化的投资组合以降低非系统性风险。",
            "寻找两只价格走势高度同步的股票进行配对交易。",
            "评估一只股票与市场指数之间的短期联动性。",
            "计算两只资产在过去一个月的价格变动相似度。"
          ],
          "question_id": "q_quiz_corr_vs_coint_01_v2",
          "stem": "在金融投资中，以下哪个应用场景最适合使用协整分析？"
        }
      ]
    }
  ],
  "sequence_id": "step3",
  "source": {
    "coverage_checklist": "L5: Statistical Arbitrage and Pairs Trading - Correlation vs Cointegration",
    "guided_story_manifest": "pipeline/3-guided_story/L5/step3/step.json",
    "lesson_map": "L5 step3: Correlation vs. Cointegration",
    "plain_text": "pipeline/1-plain/L5/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L5: Statistical Arbitrage and Pairs Trading - Correlation vs Cointegration"
  },
  "target_language": "zh-CN"
}
,
{
  "chapter_id": "lecture-5",
  "course_id": "comp7415a",
  "coverage_map": [
    {
      "coverage_tag": "implement_pairs_trading_steps",
      "covered_by": [
        "qf_flash_zscore_formula",
        "qf_flash_entry_exit_logic",
        "qf_quiz_trade_direction",
        "qf_quiz_strategy_steps",
        "qf_long_implement_strategy"
      ],
      "description": "实现配对交易的六步流程：识别资产对、协整检验、计算价差与统计量、设定阈值、执行交易、平仓。"
    },
    {
      "coverage_tag": "z_score_calculation",
      "covered_by": [
        "qf_flash_zscore_formula",
        "qf_quiz_zscore_interpretation"
      ],
      "description": "Z分数的计算公式及其在配对交易中衡量价差偏离程度的作用。"
    },
    {
      "coverage_tag": "entry_threshold_and_signal",
      "covered_by": [
        "qf_flash_entry_exit_logic",
        "qf_quiz_trade_direction",
        "qf_quiz_zscore_interpretation"
      ],
      "description": "入场阈值的概念，以及基于Z分数阈值的交易信号生成逻辑（Z > 阈值 vs Z < -阈值）。"
    },
    {
      "coverage_tag": "trade_execution_logic",
      "covered_by": [
        "qf_flash_entry_exit_logic",
        "qf_quiz_trade_direction"
      ],
      "description": "根据Z分数信号执行多空交易的具体方向：Z > 阈值时卖Y买X，Z < -阈值时买Y卖X。"
    },
    {
      "coverage_tag": "exit_condition",
      "covered_by": [
        "qf_flash_entry_exit_logic",
        "qf_long_implement_strategy"
      ],
      "description": "当价差回归均值附近时平仓离场的条件。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "z_score",
      "coverage_tags": [
        "implement_pairs_trading_steps",
        "z_score_calculation"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_zscore_formula",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能准确回忆Z分数的计算公式及其各部分的含义。",
      "linked_steps": [
        "step5"
      ],
      "practice_target": "学生答对后证明掌握了Z分数的定义式，并能区分分子（当前价差与均值的差）和分母（标准差）。",
      "question_type": "short_answer",
      "retrieval_focus": "Z分数的计算公式。",
      "term_refs": [
        {
          "display": "Z分数",
          "en": "Z-score"
        }
      ],
      "variants": [
        {
          "back": "$Z = \\frac{\\text{当前价差} - \\text{均值}}{\\text{标准差}}$",
          "estimated_seconds": 8,
          "explanation": "Z分数衡量当前价差偏离历史均值多少个标准差。",
          "front": "在配对交易中，Z分数的计算公式是什么？",
          "question_id": "q_flash_zscore_v1"
        },
        {
          "back": "当前价差与历史均值的差值（偏离量）。",
          "estimated_seconds": 6,
          "explanation": "分子 $\\text{当前价差} - \\text{均值}$ 表示价差偏离均值的绝对大小。",
          "front": "Z分数公式中的分子代表什么？",
          "question_id": "q_flash_zscore_v2"
        }
      ]
    },
    {
      "concept_key": "entry_threshold",
      "coverage_tags": [
        "implement_pairs_trading_steps",
        "entry_threshold_and_signal",
        "trade_execution_logic",
        "exit_condition"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_entry_exit_logic",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能准确回忆基于Z分数阈值的入场和出场规则。",
      "linked_steps": [
        "step5"
      ],
      "practice_target": "学生答对后证明掌握了配对交易中何时开仓（Z > 阈值或Z < -阈值）以及何时平仓（价差回归均值）的核心规则。",
      "question_type": "short_answer",
      "retrieval_focus": "基于Z分数阈值的入场和出场条件。",
      "term_refs": [
        {
          "display": "入场阈值",
          "en": "Entry Threshold"
        }
      ],
      "variants": [
        {
          "back": "卖出Y，买入X。",
          "estimated_seconds": 10,
          "explanation": "Z > 阈值意味着价差过高（Y相对X被高估），因此卖出Y、买入X，押注价差缩小。",
          "front": "在配对交易中，当Z分数大于入场阈值时，应该对资产Y和X分别执行什么操作？",
          "question_id": "q_flash_entry_exit_v1"
        },
        {
          "back": "当价差回归到历史均值附近时。",
          "estimated_seconds": 8,
          "explanation": "策略的核心是低买高卖价差，当价差回到均值，偏离消失，应平仓了结。",
          "front": "配对交易策略中，什么条件下应该平仓离场？",
          "question_id": "q_flash_entry_exit_v2"
        }
      ]
    }
  ],
  "lesson_id": "L5",
  "longform_families": [
    {
      "concept_key": "implement_pairs_trading_steps",
      "coverage_tags": [
        "implement_pairs_trading_steps",
        "entry_threshold_and_signal",
        "trade_execution_logic",
        "exit_condition"
      ],
      "difficulty": "hard",
      "family_id": "qf_long_implement_strategy",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能完整描述配对交易策略的六步流程，并解释每一步的目的和逻辑。",
      "linked_steps": [
        "step5"
      ],
      "practice_target": "学生答对后证明能连贯地阐述从资产选择到平仓的完整策略链条，包括Z分数计算、阈值设定和交易方向判断。",
      "question_type": "mechanism_trace",
      "term_refs": [
        {
          "display": "配对交易",
          "en": "Pairs Trading"
        },
        {
          "display": "Z分数",
          "en": "Z-score"
        },
        {
          "display": "入场阈值",
          "en": "Entry Threshold"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "描述计算价差和Z分数的步骤。",
            "解释入场阈值的设定逻辑。",
            "说明Z > 阈值和Z < -阈值时分别如何操作。",
            "说明平仓条件。"
          ],
          "question_id": "q_long_implement_v1",
          "reference_answer": [
            "1. 计算价差：spread = Price_A - β * Price_B，其中β通过线性回归得到。",
            "2. 计算价差的历史均值和标准差。",
            "3. 计算Z分数：Z = (当前spread - 均值) / 标准差。",
            "4. 设定入场阈值（例如 |Z| > 2）。",
            "5. 交易规则：若Z > 2，卖出A（高估）、买入B（低估）；若Z < -2，买入A、卖出B。",
            "6. 平仓条件：当Z回归到0附近（即价差回到均值）时，平掉所有头寸。"
          ],
          "rubric_points": [
            "正确写出Z分数公式：Z = (当前价差 - 均值) / 标准差。",
            "指出价差 = Price_A - β * Price_B，其中β是对冲比率。",
            "解释入场阈值（如1.0或2.0）用于判断偏离是否足够大。",
            "正确描述：Z > 阈值时卖出A、买入B；Z < -阈值时买入A、卖出B。",
            "正确描述：当Z回归到0附近（或价差回到均值）时平仓。"
          ],
          "stem": "假设你已经确认股票A和股票B是协整的，请详细描述如何构建一个完整的配对交易策略，包括如何计算Z分数、如何设定入场和出场规则，以及如何根据Z分数执行具体的买卖操作。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "解释Z分数标准化的作用。",
            "描述一个完整的交易循环中Z分数的变化路径。"
          ],
          "question_id": "q_long_implement_v2",
          "reference_answer": [
            "使用Z分数而非原始价差的原因：原始价差的数值大小取决于资产价格水平，不同资产对无法使用统一的阈值。Z分数通过减去均值再除以标准差，将偏离量标准化，使得阈值（如2.0）可以跨不同资产对通用。",
            "一个完整的交易循环：假设Z分数从0开始，随着价差扩大，Z逐渐增大。当Z超过入场阈值（如2.0）时，触发开仓信号（卖出Y、买入X）。之后价差开始回归，Z逐渐减小。当Z回到0附近（如-0.5到0.5）时，触发平仓信号，了结头寸，完成一个循环。"
          ],
          "rubric_points": [
            "指出Z分数将价差标准化为以标准差为单位的偏离，使得阈值通用。",
            "说明原始价差受价格尺度影响，不同资产对不可比。",
            "描述循环：Z从0开始 → 偏离超过阈值 → 开仓 → 价差回归 → Z回到0附近 → 平仓。"
          ],
          "stem": "请解释为什么在配对交易策略中，使用Z分数而不是直接使用价差来生成交易信号？并描述一个完整的交易循环（从开仓到平仓）中，Z分数如何变化。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "entry_threshold",
      "coverage_tags": [
        "implement_pairs_trading_steps",
        "entry_threshold_and_signal",
        "trade_execution_logic"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_trade_direction",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能在测验情境下根据Z分数信号正确判断交易方向。",
      "linked_steps": [
        "step5"
      ],
      "practice_target": "学生答对后证明能应用Z分数阈值规则，区分Z > 阈值和Z < -阈值时对应的多空操作。",
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "入场阈值",
          "en": "Entry Threshold"
        },
        {
          "display": "Z分数",
          "en": "Z-score"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "Z = -1.5 小于 -1.0（阈值），说明价差过低（Y相对X被低估），因此应买入Y、卖出X，押注价差回升。",
          "options": [
            "买入Y，卖出X",
            "卖出Y，买入X",
            "同时买入Y和X",
            "不执行任何交易"
          ],
          "question_id": "q_quiz_trade_dir_v1",
          "stem": "在配对交易策略中，如果当前Z分数为 -1.5，而入场阈值设为 1.0，你应该如何操作？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "Z = 2.3 > 2.0，价差过高，Y被高估、X被低估，应卖出Y、买入X。",
          "options": [
            "买入Y，卖出X",
            "卖出Y，买入X",
            "同时卖出Y和X",
            "等待价差回归均值再操作"
          ],
          "question_id": "q_quiz_trade_dir_v2",
          "stem": "假设你正在交易一对协整的股票（Y和X）。当前价差的Z分数为 2.3，入场阈值为 2.0。正确的第一步操作是？"
        }
      ]
    },
    {
      "concept_key": "z_score",
      "coverage_tags": [
        "z_score_calculation",
        "entry_threshold_and_signal"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_zscore_interpretation",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能正确解释Z分数的含义及其在交易信号中的作用。",
      "linked_steps": [
        "step5"
      ],
      "practice_target": "学生答对后证明理解Z分数作为标准化偏离度量的意义，并能区分其与原始价差的区别。",
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "Z分数",
          "en": "Z-score"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "Z分数将价差标准化为以标准差为单位的偏离量，使得阈值（如1.0或2.0）可以跨不同资产对通用。",
          "options": [
            "Z分数计算更简单",
            "Z分数消除了不同资产价格尺度的影响，使阈值设定标准化",
            "Z分数能预测未来价格走势",
            "Z分数可以保证策略盈利"
          ],
          "question_id": "q_quiz_zscore_int_v1",
          "stem": "在配对交易中，使用Z分数而不是原始价差来生成信号的主要原因是？"
        },
        {
          "answer": 0,
          "estimated_seconds": 10,
          "explanation": "Z = (当前价差 - 均值) / 标准差 = 0 意味着当前价差等于历史均值。",
          "options": [
            "当前价差等于历史均值",
            "当前价差为0",
            "策略应该立即平仓",
            "两只股票价格相等"
          ],
          "question_id": "q_quiz_zscore_int_v2",
          "stem": "一个Z分数为0的价差意味着什么？"
        }
      ]
    },
    {
      "concept_key": "implement_pairs_trading_steps",
      "coverage_tags": [
        "implement_pairs_trading_steps"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_strategy_steps",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能正确排列配对交易策略的六个核心步骤。",
      "linked_steps": [
        "step5"
      ],
      "practice_target": "学生答对后证明掌握了实施配对交易策略的完整流程顺序。",
      "question_type": "ordering",
      "term_refs": [
        {
          "display": "配对交易",
          "en": "Pairs Trading"
        }
      ],
      "variants": [
        {
          "answer": [
            2,
            5,
            0,
            3,
            4,
            1
          ],
          "estimated_seconds": 30,
          "explanation": "正确顺序：1. 找到资产对(C) → 2. 协整检验(F) → 3. 计算价差与统计量(A) → 4. 设定阈值生成信号(D) → 5. 执行交易(E) → 6. 价差回归平仓(B)。",
          "options": [
            "A. 计算价差及历史均值与标准差",
            "B. 当价差回归均值时平仓",
            "C. 找到一对可能相关的资产",
            "D. 设定入场阈值并生成交易信号",
            "E. 执行交易（买入/卖出）",
            "F. 用CADF或约翰森检验确认协整"
          ],
          "question_id": "q_quiz_steps_v1",
          "stem": "请将以下配对交易策略的步骤按正确顺序排列。"
        }
      ]
    }
  ],
  "sequence_id": "step5",
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L5/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "input source outline"
  },
  "target_language": "zh-CN"
}
,
{
  "chapter_id": "lecture-5",
  "course_id": "comp7415a",
  "coverage_map": [
    {
      "coverage_tag": "real_market_challenges",
      "covered_by": [
        "qf_flash_model_risk",
        "qf_flash_execution_risk",
        "qf_quiz_risk_identification",
        "qf_long_strategy_effectiveness"
      ],
      "description": "真实市场挑战：模型风险、执行风险、市场风险、监管风险"
    },
    {
      "coverage_tag": "model_risk",
      "covered_by": [
        "qf_flash_model_risk",
        "qf_quiz_risk_identification"
      ],
      "description": "模型风险：假设错误或过拟合导致策略失效"
    },
    {
      "coverage_tag": "execution_risk",
      "covered_by": [
        "qf_flash_execution_risk",
        "qf_quiz_risk_identification"
      ],
      "description": "执行风险：滑点和延迟侵蚀利润"
    },
    {
      "coverage_tag": "strategy_effectiveness",
      "covered_by": [
        "qf_long_strategy_effectiveness"
      ],
      "description": "统计套利策略的有效运行方式：选择协整资产、尝试不同观察窗口、选择距离度量、定期重新平衡"
    },
    {
      "coverage_tag": "core_philosophy",
      "covered_by": [
        "qf_quiz_core_philosophy"
      ],
      "description": "核心思想：统计套利不是预测价格，而是押注关系回归"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "model_risk",
      "coverage_tags": [
        "real_market_challenges",
        "model_risk"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_model_risk",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能准确回忆模型风险的定义和成因。",
      "linked_steps": [
        "step6"
      ],
      "practice_target": "学生答对后证明掌握了模型风险的核心含义：因假设错误或过拟合导致策略失效。",
      "question_type": "short_answer",
      "retrieval_focus": "模型风险的两个主要成因。",
      "term_refs": [
        {
          "display": "模型风险",
          "en": "Model Risk"
        }
      ],
      "variants": [
        {
          "back": "因模型假设错误或过拟合导致策略失效的风险。",
          "estimated_seconds": 8,
          "explanation": "模型风险是量化策略面临的核心风险之一，源于模型与真实市场的不匹配。",
          "front": "在统计套利中，模型风险指的是什么？",
          "question_id": "q_flash_model_risk_v1"
        },
        {
          "back": "假设错误和过拟合。",
          "estimated_seconds": 8,
          "explanation": "假设错误指模型对市场行为的假设不成立；过拟合指模型过度适应历史数据而失去泛化能力。",
          "front": "模型风险的两个主要成因是什么？",
          "question_id": "q_flash_model_risk_v2"
        }
      ]
    },
    {
      "concept_key": "execution_risk",
      "coverage_tags": [
        "real_market_challenges",
        "execution_risk"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_execution_risk",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能准确回忆执行风险的定义和两个具体来源。",
      "linked_steps": [
        "step6"
      ],
      "practice_target": "学生答对后证明掌握了执行风险的核心含义：滑点和延迟侵蚀利润。",
      "question_type": "short_answer",
      "retrieval_focus": "执行风险的两个具体来源。",
      "term_refs": [
        {
          "display": "执行风险",
          "en": "Execution Risk"
        }
      ],
      "variants": [
        {
          "back": "因滑点和延迟导致实际成交价格偏离预期，从而侵蚀利润的风险。",
          "estimated_seconds": 8,
          "explanation": "执行风险是交易层面的风险，影响策略的实际收益。",
          "front": "在统计套利中，执行风险指的是什么？",
          "question_id": "q_flash_execution_risk_v1"
        },
        {
          "back": "滑点（slippage）和延迟（latency）。",
          "estimated_seconds": 8,
          "explanation": "滑点是预期价格与实际成交价格的差异；延迟是信号产生到订单执行之间的时间差。",
          "front": "执行风险的两个主要来源是什么？",
          "question_id": "q_flash_execution_risk_v2"
        }
      ]
    }
  ],
  "lesson_id": "L5",
  "longform_families": [
    {
      "concept_key": "strategy_effectiveness",
      "coverage_tags": [
        "real_market_challenges",
        "strategy_effectiveness"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_strategy_effectiveness",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能解释统计套利策略面临的主要挑战，并说明如何有效运行该策略。",
      "linked_steps": [
        "step6"
      ],
      "practice_target": "学生答对后证明能综合理解真实市场中的风险以及提高策略有效性的方法。",
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "模型风险",
          "en": "Model Risk"
        },
        {
          "display": "执行风险",
          "en": "Execution Risk"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "风险1：",
            "缓解措施1：",
            "风险2：",
            "缓解措施2：",
            "风险3：",
            "缓解措施3："
          ],
          "question_id": "q_long_strategy_effectiveness_v1",
          "reference_answer": [
            "风险1：模型风险——假设错误或过拟合导致策略失效。缓解措施：使用样本外测试和交叉验证，避免过拟合。",
            "风险2：执行风险——滑点和延迟侵蚀利润。缓解措施：使用限价单减少滑点，优化交易基础设施降低延迟。",
            "风险3：市场风险——突发事件打破历史关系。缓解措施：设置止损，定期重新评估协整关系。"
          ],
          "rubric_points": [
            "正确识别模型风险并给出合理缓解措施（如交叉验证、简化模型）",
            "正确识别执行风险并给出合理缓解措施（如使用限价单、优化基础设施）",
            "正确识别市场风险或监管风险并给出合理缓解措施"
          ],
          "stem": "请列举统计套利策略在真实市场中面临的至少三种风险，并针对每种风险提出一个缓解措施。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "要点1：",
            "要点2：",
            "要点3：",
            "要点4："
          ],
          "question_id": "q_long_strategy_effectiveness_v2",
          "reference_answer": [
            "要点1：选择协整的资产组，确保存在长期均衡关系。",
            "要点2：尝试不同的观察窗口，找到最稳定的协整关系。",
            "要点3：选择合适的距离度量（如Z分数）来确定最优入场和出场点。",
            "要点4：定期重新平衡组合，因为对冲比率可能随时间变化。"
          ],
          "rubric_points": [
            "提到选择协整的资产组",
            "提到尝试不同的观察窗口",
            "提到选择合适的距离度量",
            "提到定期重新平衡"
          ],
          "stem": "请说明如何有效运行一个统计套利策略，至少列出四个关键要点。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "real_market_challenges",
      "coverage_tags": [
        "real_market_challenges",
        "model_risk",
        "execution_risk"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_risk_identification",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能在不同场景中区分模型风险、执行风险、市场风险和监管风险。",
      "linked_steps": [
        "step6"
      ],
      "practice_target": "学生答对后证明能识别真实市场中不同风险类型的典型表现。",
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "模型风险",
          "en": "Model Risk"
        },
        {
          "display": "执行风险",
          "en": "Execution Risk"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "使用未来数据是过拟合的一种形式，属于模型风险——模型假设错误或过拟合导致策略失效。",
          "options": [
            "模型风险",
            "执行风险",
            "市场风险",
            "监管风险"
          ],
          "question_id": "q_quiz_risk_identification_v1",
          "stem": "一个统计套利策略在回测中表现优异，但在实盘中表现不佳。经分析发现，策略在回测时使用了未来数据。这属于哪种风险？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "延迟导致实际成交价格偏离预期，这正是执行风险的表现。",
          "options": [
            "模型风险",
            "执行风险",
            "市场风险",
            "监管风险"
          ],
          "question_id": "q_quiz_risk_identification_v2",
          "stem": "一个配对交易策略在信号触发后，由于网络延迟，实际成交价比预期差了0.1%。这属于哪种风险？"
        }
      ]
    },
    {
      "concept_key": "core_philosophy",
      "coverage_tags": [
        "core_philosophy"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_core_philosophy",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能准确理解统计套利策略的核心思想。",
      "linked_steps": [
        "step6"
      ],
      "practice_target": "学生答对后证明掌握了统计套利不是预测价格，而是押注关系回归这一核心理念。",
      "question_type": "single_choice",
      "term_refs": [],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "统计套利不是预测价格，而是押注关系——当关系被打破时，它终将回归。",
          "options": [
            "预测资产价格的未来走势",
            "押注资产价格之间的历史关系在偏离后终将回归",
            "通过分散投资降低整体风险",
            "利用高频交易获取微小利润"
          ],
          "question_id": "q_quiz_core_philosophy_v1",
          "stem": "以下哪句话最准确地概括了统计套利的核心理念？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "统计套利是市场中性策略，不依赖市场整体涨跌趋势，而是依赖资产间的相对关系。",
          "options": [
            "资产价格之间存在统计关系",
            "价格偏离后倾向于回归",
            "市场整体趋势将持续上涨",
            "历史关系在未来可能仍然有效"
          ],
          "question_id": "q_quiz_core_philosophy_v2",
          "stem": "统计套利策略不依赖以下哪种假设？"
        }
      ]
    }
  ],
  "sequence_id": "step6",
  "source": {
    "coverage_checklist": "L5: Statistical Arbitrage and Pairs Trading",
    "guided_story_manifest": "pipeline/3-guided_story/L5/step6/step.json",
    "lesson_map": "L5 lesson map",
    "plain_text": "pipeline/1-plain/L5/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L5: Statistical Arbitrage and Pairs Trading"
  },
  "target_language": "zh-CN"
}
,
{
  "chapter_id": "lecture-5",
  "course_id": "comp7415a",
  "coverage_map": [
    {
      "coverage_tag": "statistical_arbitrage_definition",
      "covered_by": [
        "qf_flash_statarb_def",
        "qf_quiz_statarb_assumption"
      ],
      "description": "统计套利的定义：利用历史数据寻找资产价格的统计性错误定价，押注价格回归。"
    },
    {
      "coverage_tag": "mean_reversion_core",
      "covered_by": [
        "qf_flash_statarb_def",
        "qf_quiz_statarb_assumption"
      ],
      "description": "均值回归：统计套利本质上是一种均值回归策略，价格偏离历史均值后倾向于回归。"
    },
    {
      "coverage_tag": "pairs_trading_intro",
      "covered_by": [
        "qf_flash_pairs_trading",
        "qf_quiz_pairs_trading"
      ],
      "description": "配对交易：同时买入一只资产、卖出另一只高度相关的资产，押注价差回归。"
    },
    {
      "coverage_tag": "market_neutral_concept",
      "covered_by": [
        "qf_flash_market_neutral",
        "qf_quiz_market_neutral"
      ],
      "description": "市场中性：同时持有多头和空头，组合的市场风险敞口为零，收益来自相对价格变化。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "statistical_arbitrage",
      "coverage_tags": [
        "statistical_arbitrage_definition",
        "mean_reversion_core"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_statarb_def",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能用自己的话准确说出统计套利的定义及其核心假设。",
      "linked_steps": [
        "step1"
      ],
      "practice_target": "学生答对后证明掌握了统计套利的基本定义和均值回归假设。",
      "question_type": "short_answer",
      "retrieval_focus": "统计套利的定义和核心假设",
      "term_refs": [
        {
          "display": "统计套利",
          "en": "Statistical Arbitrage"
        },
        {
          "display": "均值回归",
          "en": "Mean Reversion"
        }
      ],
      "variants": [
        {
          "back": "价格偏离历史关系后，最终会回归（均值回归）。",
          "estimated_seconds": 8,
          "explanation": "统计套利不预测市场涨跌，而是押注价格关系会回归历史均值。",
          "front": "统计套利策略的核心假设是什么？",
          "question_id": "q_flash_statarb_def_v1"
        },
        {
          "back": "均值回归策略。",
          "estimated_seconds": 6,
          "explanation": "它利用价格偏离历史平均水平后倾向于回归的现象来获利。",
          "front": "统计套利本质上是一种什么类型的策略？",
          "question_id": "q_flash_statarb_def_v2"
        }
      ]
    },
    {
      "concept_key": "pairs_trading",
      "coverage_tags": [
        "pairs_trading_intro"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_pairs_trading",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能准确描述配对交易的基本操作方式。",
      "linked_steps": [
        "step1"
      ],
      "practice_target": "学生答对后证明掌握了配对交易的核心操作：同时买卖两只相关资产。",
      "question_type": "short_answer",
      "retrieval_focus": "配对交易的操作方式",
      "term_refs": [
        {
          "display": "配对交易",
          "en": "Pairs Trading"
        }
      ],
      "variants": [
        {
          "back": "同时买入一只资产，卖出另一只资产。",
          "estimated_seconds": 8,
          "explanation": "配对交易押注价差回归，因此同时建立多头和空头头寸。",
          "front": "配对交易中，当两只资产的价格价差偏离历史均值时，交易者会做什么操作？",
          "question_id": "q_flash_pairs_trading_v1"
        },
        {
          "back": "两只高度相关的资产。",
          "estimated_seconds": 5,
          "explanation": "经典配对交易是在两只相关资产之间进行的。",
          "front": "配对交易通常涉及几只资产？",
          "question_id": "q_flash_pairs_trading_v2"
        }
      ]
    },
    {
      "concept_key": "market_neutral",
      "coverage_tags": [
        "market_neutral_concept"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_market_neutral",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能解释市场中性策略的含义和收益来源。",
      "linked_steps": [
        "step1"
      ],
      "practice_target": "学生答对后证明掌握了市场中性的定义：组合风险敞口为零，收益来自相对价格变化。",
      "question_type": "short_answer",
      "retrieval_focus": "市场中性策略的含义",
      "term_refs": [
        {
          "display": "市场中性",
          "en": "Market Neutral"
        }
      ],
      "variants": [
        {
          "back": "两只资产之间的相对价格变化，而不是大盘涨跌。",
          "estimated_seconds": 8,
          "explanation": "因为同时持有多头和空头，大盘影响被抵消，收益来自价差变化。",
          "front": "市场中性策略的收益主要来自哪里？",
          "question_id": "q_flash_market_neutral_v1"
        },
        {
          "back": "因为它同时持有多头和空头，整体市场风险敞口为零。",
          "estimated_seconds": 8,
          "explanation": "买入一只资产的同时卖出另一只，大盘涨跌对组合的影响相互抵消。",
          "front": "为什么统计套利中的配对交易是市场中性的？",
          "question_id": "q_flash_market_neutral_v2"
        }
      ]
    }
  ],
  "lesson_id": "L5",
  "longform_families": [
    {
      "concept_key": "statistical_arbitrage",
      "coverage_tags": [
        "statistical_arbitrage_definition",
        "mean_reversion_core",
        "pairs_trading_intro",
        "market_neutral_concept"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_statarb_explain",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能连贯地解释统计套利、均值回归、配对交易和市场中性之间的关系。",
      "linked_steps": [
        "step1"
      ],
      "practice_target": "学生答对后证明能用自己的话解释统计套利策略的完整逻辑链条。",
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "统计套利",
          "en": "Statistical Arbitrage"
        },
        {
          "display": "均值回归",
          "en": "Mean Reversion"
        },
        {
          "display": "配对交易",
          "en": "Pairs Trading"
        },
        {
          "display": "市场中性",
          "en": "Market Neutral"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "先解释统计套利的定义和核心假设",
            "然后说明配对交易如何实现统计套利",
            "最后解释市场中性如何降低风险"
          ],
          "question_id": "q_long_statarb_explain_v1",
          "reference_answer": [
            "统计套利是一种利用历史数据寻找资产价格统计性错误定价的策略，它基于均值回归假设，即价格偏离历史关系后最终会回归。",
            "配对交易是统计套利最经典的做法：同时买入一只资产、卖出另一只高度相关的资产，押注两者价差回归。",
            "因为同时持有多头和空头，整个组合是市场中性的，大盘涨跌的影响被抵消，收益只来自两只资产之间的相对价格变化。"
          ],
          "rubric_points": [
            "正确指出统计套利依赖均值回归假设",
            "正确描述配对交易同时买卖两只相关资产",
            "正确解释市场中性通过多空对冲抵消大盘风险"
          ],
          "stem": "请用你自己的话解释：统计套利、均值回归、配对交易和市场中性这几个概念之间有什么关系？"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "statistical_arbitrage",
      "coverage_tags": [
        "statistical_arbitrage_definition",
        "mean_reversion_core"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_statarb_assumption",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能在选择题中识别统计套利策略的核心假设。",
      "linked_steps": [
        "step1"
      ],
      "practice_target": "学生答对后证明能区分统计套利与其他交易策略的核心假设。",
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "统计套利",
          "en": "Statistical Arbitrage"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "统计套利依赖均值回归假设：价格偏离历史关系后，倾向于回归。",
          "options": [
            "市场永远有效",
            "价格偏离后最终会回归",
            "资产价格会一直上涨",
            "两只资产完全独立"
          ],
          "question_id": "q_quiz_statarb_assumption_v1",
          "stem": "统计套利策略的核心假设是什么？"
        }
      ]
    },
    {
      "concept_key": "pairs_trading",
      "coverage_tags": [
        "pairs_trading_intro"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_pairs_trading",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能识别配对交易的基本操作。",
      "linked_steps": [
        "step1"
      ],
      "practice_target": "学生答对后证明掌握了配对交易同时买卖两只资产的核心操作。",
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "配对交易",
          "en": "Pairs Trading"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "配对交易的核心是同时建立多头和空头头寸，押注两只资产价差回归。",
          "options": [
            "只买入一只被低估的股票",
            "同时买入一只资产，卖出另一只高度相关的资产",
            "同时买入两只高度相关的资产",
            "只卖出一只被高估的股票"
          ],
          "question_id": "q_quiz_pairs_trading_v1",
          "stem": "以下哪项最准确地描述了配对交易的操作方式？"
        }
      ]
    },
    {
      "concept_key": "market_neutral",
      "coverage_tags": [
        "market_neutral_concept"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_market_neutral",
      "is_meta_about_course_or_material": false,
      "learning_goal": "学生能理解市场中性策略如何降低市场风险。",
      "linked_steps": [
        "step1"
      ],
      "practice_target": "学生答对后证明掌握了市场中性策略降低市场风险的原理。",
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "市场中性",
          "en": "Market Neutral"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "同时持有多头和空头，大盘涨跌对多头和空头的影响相互抵消，收益来自相对价格变化。",
          "options": [
            "只投资于低风险资产",
            "同时持有多头和空头，抵消大盘涨跌的影响",
            "使用高杠杆放大收益",
            "只做多不卖空"
          ],
          "question_id": "q_quiz_market_neutral_v1",
          "stem": "统计套利中的市场中性策略如何降低投资组合的整体市场风险？"
        }
      ]
    }
  ],
  "sequence_id": "step1",
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L5/plain.txt",
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

请直接输出 MDX，不要加解释。
