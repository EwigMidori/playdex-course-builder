请基于以下材料，生成一份 lesson 级 MDX 课本。

目标语言：
zh-CN

lesson_id：
L6

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
# L6: Capital and Risk Management
Course Code: COMP7415
# Agenda
• Introduction to the risk management cycle
- Risk identification
- Risk measurement
- Volatility modelling including ARCH, GARCH
- Value-at-Risk (VaR)
- Stress testing, Scenario analysis
- Position sizing strategy
- Fixed Size
Balance Rescaling
- Dollar Risk Approach
- Kelly Criterion
# What is Risk?
![](images/5356a963e01aaccc319c2604722e58132deb56083bf83b844da77a029d6e0cba.jpg)
# What is Risk?
- Risk simply refers to uncertainty
- In finance, risk refers to the uncertainty of the investment return.
- Up-side risk: the possibility of making money
- Down-side risk: the possibility of losing money
# Risk Management Cycle
1. Risk Identification
2. Risk Assessment / Measurement
3. Risk Treatment
4. Risk Monitoring
![](images/1b25afef457cce7d0af1c1efc7bad43caa138fea188417d07cc746c7cfa3cd86.jpg)
# Risk Identification
# Major Risk Types for a financial institution
1. Market Risk
Equity Risk
- FX Risk
Commodity Risk
2. Interest rate Risk
3. Credit Risk
4. Liquidity Risk
5. Operational Risk
6. Legal Risk
7. Reputational Risk
8. Strategic Risk
Investment related risks
# Market Risk
- Market Risk is the potential for financial losses due to changes in market prices. It affects assets such as stocks, bonds, currencies, and commodities.
- Example: Suppose you own shares in a tech company. If the stock market experiences a downturn, the value of your shares may decrease regardless of the company's performance, due to overall market sentiment.
- This type of risk is inherent to all investments and can be influenced by factors like economic changes, political events, and natural disasters.
# Interest Risk
- Interest Rate Risk is the potential for investment losses due to fluctuations in interest rates. It primarily affects the value of bonds and other fixed-income securities. When interest rates rise, bond prices typically fall, and vice versa.
- Example: Imagine you own a bond with a fixed interest rate of $3\%$ . If the market interest rate rises to $4\%$ , new bonds offer better returns, making your bond less attractive. Consequently, the market value of your bond decreases.
- This risk is crucial for investors and financial institutions managing portfolios sensitive to interest rate changes.
# Credit Risk
- Credit Risk is the possibility of a loss resulting from a borrower's failure to repay a loan or meet contractual obligations. It affects lenders and investors in bonds or loans.
- Example: If a bank lends money to a business, and the business defaults on the loan, the bank faces credit risk. This risk can lead to financial loss for the bank due to the unpaid loan amount.
- Credit risk is a key consideration in lending and investing, influencing interest rates and lending terms.
# Liquidity Risk
- Liquidity Risk is the risk that an entity may not be able to quickly convert assets into cash without significant loss in value. It affects individuals, businesses, and financial institutions.
- Example: Imagine a company owns a large amount of real estate. If it suddenly needs cash to cover expenses, selling the properties quickly might force them to accept lower prices, resulting in a financial loss.
- Liquidity risk is important for managing cash flow and ensuring that obligations can be met when they come due.
# What type of risk involved in these examples?
1. A company defaults on its loan payment.
2. You need to sell a property quickly but can't find a buyer without reducing the price significantly.
</COVERAGE_CHECKLIST>

<SOURCE_OUTLINE>
# L6: Capital and Risk Management
Course Code: COMP7415
# Agenda
• Introduction to the risk management cycle
- Risk identification
- Risk measurement
- Volatility modelling including ARCH, GARCH
- Value-at-Risk (VaR)
- Stress testing, Scenario analysis
- Position sizing strategy
- Fixed Size
Balance Rescaling
- Dollar Risk Approach
- Kelly Criterion
# What is Risk?
![](images/5356a963e01aaccc319c2604722e58132deb56083bf83b844da77a029d6e0cba.jpg)
# What is Risk?
- Risk simply refers to uncertainty
- In finance, risk refers to the uncertainty of the investment return.
- Up-side risk: the possibility of making money
- Down-side risk: the possibility of losing money
# Risk Management Cycle
1. Risk Identification
2. Risk Assessment / Measurement
3. Risk Treatment
4. Risk Monitoring
![](images/1b25afef457cce7d0af1c1efc7bad43caa138fea188417d07cc746c7cfa3cd86.jpg)
# Risk Identification
# Major Risk Types for a financial institution
1. Market Risk
Equity Risk
- FX Risk
Commodity Risk
2. Interest rate Risk
3. Credit Risk
4. Liquidity Risk
5. Operational Risk
6. Legal Risk
7. Reputational Risk
8. Strategic Risk
Investment related risks
# Market Risk
- Market Risk is the potential for financial losses due to changes in market prices. It affects assets such as stocks, bonds, currencies, and commodities.
- Example: Suppose you own shares in a tech company. If the stock market experiences a downturn, the value of your shares may decrease regardless of the company's performance, due to overall market sentiment.
- This type of risk is inherent to all investments and can be influenced by factors like economic changes, political events, and natural disasters.
# Interest Risk
- Interest Rate Risk is the potential for investment losses due to fluctuations in interest rates. It primarily affects the value of bonds and other fixed-income securities. When interest rates rise, bond prices typically fall, and vice versa.
- Example: Imagine you own a bond with a fixed interest rate of $3\%$ . If the market interest rate rises to $4\%$ , new bonds offer better returns, making your bond less attractive. Consequently, the market value of your bond decreases.
- This risk is crucial for investors and financial institutions managing portfolios sensitive to interest rate changes.
# Credit Risk
- Credit Risk is the possibility of a loss resulting from a borrower's failure to repay a loan or meet contractual obligations. It affects lenders and investors in bonds or loans.
- Example: If a bank lends money to a business, and the business defaults on the loan, the bank faces credit risk. This risk can lead to financial loss for the bank due to the unpaid loan amount.
- Credit risk is a key consideration in lending and investing, influencing interest rates and lending terms.
# Liquidity Risk
- Liquidity Risk is the risk that an entity may not be able to quickly convert assets into cash without significant loss in value. It affects individuals, businesses, and financial institutions.
- Example: Imagine a company owns a large amount of real estate. If it suddenly needs cash to cover expenses, selling the properties quickly might force them to accept lower prices, resulting in a financial loss.
- Liquidity risk is important for managing cash flow and ensuring that obligations can be met when they come due.
# What type of risk involved in these examples?
1. A company defaults on its loan payment.
2. You need to sell a property quickly but can't find a buyer without reducing the price significantly.
</SOURCE_OUTLINE>

<LESSON_MAP>
{
  "lesson_id": "L6",
  "mode": "guided_story",
  "steps": [
    {
      "concept": "风险是什么？为什么需要管理风险？",
      "file": "research/pipeline/3-guided_story/L6/step1/step.json",
      "sequence_id": "step1"
    },
    {
      "concept": "金融机构面临的主要风险类型",
      "file": "research/pipeline/3-guided_story/L6/step2/step.json",
      "sequence_id": "step2"
    },
    {
      "concept": "衡量风险：从标准差到波动率模型",
      "file": "research/pipeline/3-guided_story/L6/step3/step.json",
      "sequence_id": "step3"
    },
    {
      "concept": "ARCH 与 GARCH：给波动率建模",
      "file": "research/pipeline/3-guided_story/L6/step4/step.json",
      "sequence_id": "step4"
    },
    {
      "concept": "在险价值（VaR）：一个数字告诉你最大可能亏损",
      "file": "research/pipeline/3-guided_story/L6/step5/step.json",
      "sequence_id": "step5"
    },
    {
      "concept": "压力测试与情景分析：为最坏情况做准备",
      "file": "research/pipeline/3-guided_story/L6/step6/step.json",
      "sequence_id": "step6"
    },
    {
      "concept": "仓位管理：每次交易该下多大注？",
      "file": "research/pipeline/3-guided_story/L6/step7/step.json",
      "sequence_id": "step7"
    },
    {
      "concept": "凯利公式的局限与总结",
      "file": "research/pipeline/3-guided_story/L6/step8/step.json",
      "sequence_id": "step8"
    }
  ]
}

</LESSON_MAP>

<GUIDED_STORY_MANIFEST>
{
  "lesson_id": "L6",
  "mode": "guided_story",
  "steps": [
    {
      "concept": "风险是什么？为什么需要管理风险？",
      "file": "research/pipeline/3-guided_story/L6/step1/step.json",
      "sequence_id": "step1"
    },
    {
      "concept": "金融机构面临的主要风险类型",
      "file": "research/pipeline/3-guided_story/L6/step2/step.json",
      "sequence_id": "step2"
    },
    {
      "concept": "衡量风险：从标准差到波动率模型",
      "file": "research/pipeline/3-guided_story/L6/step3/step.json",
      "sequence_id": "step3"
    },
    {
      "concept": "ARCH 与 GARCH：给波动率建模",
      "file": "research/pipeline/3-guided_story/L6/step4/step.json",
      "sequence_id": "step4"
    },
    {
      "concept": "在险价值（VaR）：一个数字告诉你最大可能亏损",
      "file": "research/pipeline/3-guided_story/L6/step5/step.json",
      "sequence_id": "step5"
    },
    {
      "concept": "压力测试与情景分析：为最坏情况做准备",
      "file": "research/pipeline/3-guided_story/L6/step6/step.json",
      "sequence_id": "step6"
    },
    {
      "concept": "仓位管理：每次交易该下多大注？",
      "file": "research/pipeline/3-guided_story/L6/step7/step.json",
      "sequence_id": "step7"
    },
    {
      "concept": "凯利公式的局限与总结",
      "file": "research/pipeline/3-guided_story/L6/step8/step.json",
      "sequence_id": "step8"
    }
  ]
}

</GUIDED_STORY_MANIFEST>

<GUIDED_STORY_STEPS>
[
  {
    "lesson_id": "L6",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s001",
        "introduced_terms": [],
        "lines": [
          "想象你刚买入一只股票。",
          "明天它的价格会涨，还是会跌？"
        ],
        "type": "narration"
      },
      {
        "id": "s002",
        "introduced_terms": [
          "risk"
        ],
        "lines": [
          "这种“不确定”的感觉，就是**<term id=\"risk\">风险</term>**。",
          "在金融世界里，风险就是投资回报的不确定性。"
        ],
        "type": "narration"
      },
      {
        "id": "s003",
        "introduced_terms": [
          "upside_risk",
          "downside_risk"
        ],
        "lines": [
          "风险不一定是坏事。",
          "赚钱的可能性叫**<term id=\"upside_risk\">上行风险</term>**，亏钱的可能性叫**<term id=\"downside_risk\">下行风险</term>**。"
        ],
        "type": "narration"
      },
      {
        "id": "s004",
        "introduced_terms": [],
        "lines": [
          "既然风险无处不在，我们该怎么对付它？",
          "这就需要一个系统性的流程。"
        ],
        "type": "narration"
      },
      {
        "id": "s005",
        "introduced_terms": [
          "risk_management_cycle"
        ],
        "lines": [
          "这个流程叫做**<term id=\"risk_management_cycle\">风险管理循环</term>**。",
          "它包含四个步骤：识别、评估、处理、监控。"
        ],
        "type": "narration"
      },
      {
        "id": "s006",
        "introduced_terms": [],
        "lines": [
          "第一步：识别风险。",
          "第二步：衡量风险有多大。",
          "第三步：采取措施。第四步：持续观察。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "风险的核心是“不确定性”，它既包含赚钱的可能，也包含亏钱的可能。",
          "kind": "single_choice",
          "options": [
            "一定会亏钱",
            "投资回报的不确定性",
            "股票价格下跌",
            "市场波动"
          ],
          "prompt": "下面哪个选项最准确地描述了金融中的“风险”？"
        },
        "id": "s007",
        "lines": [
          "下面哪个选项最准确地描述了金融中的“风险”？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step1",
    "source": {
      "plain_text": "L6: Capital and Risk Management. What is Risk? Risk simply refers to uncertainty. In finance, risk refers to the uncertainty of the investment return. Up-side risk: the possibility of making money. Down-side risk: the possibility of losing money. Risk Management Cycle: 1. Risk Identification, 2. Risk Assessment / Measurement, 3. Risk Treatment, 4. Risk Monitoring.",
      "related": [
        "risk management cycle"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "downside_risk": {
        "aliases": [
          "Downside Risk"
        ],
        "display": "下行风险",
        "gloss": "亏钱的可能性。"
      },
      "risk": {
        "aliases": [
          "Risk"
        ],
        "display": "风险",
        "gloss": "在金融中，指投资回报的不确定性。"
      },
      "risk_management_cycle": {
        "aliases": [
          "Risk Management Cycle"
        ],
        "display": "风险管理循环",
        "gloss": "一个包含识别、评估、处理、监控风险的持续过程。"
      },
      "upside_risk": {
        "aliases": [
          "Upside Risk"
        ],
        "display": "上行风险",
        "gloss": "赚钱的可能性。"
      }
    }
  },
  {
    "lesson_id": "L6",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s008",
        "introduced_terms": [],
        "lines": [
          "金融机构每天要面对好几种风险。",
          "我们先认识四个最重要的。"
        ],
        "type": "narration"
      },
      {
        "id": "s009",
        "introduced_terms": [
          "market_risk"
        ],
        "lines": [
          "第一种：**<term id=\"market_risk\">市场风险</term>**。",
          "因为股价、汇率、商品价格波动而亏钱的风险。"
        ],
        "type": "narration"
      },
      {
        "id": "s010",
        "introduced_terms": [],
        "lines": [
          "比如你持有一家科技公司的股票。",
          "即使公司本身没问题，整个市场大跌，你的股票也会跟着跌。"
        ],
        "type": "narration"
      },
      {
        "id": "s011",
        "introduced_terms": [
          "interest_rate_risk"
        ],
        "lines": [
          "第二种：**<term id=\"interest_rate_risk\">利率风险</term>**。",
          "主要影响债券。利率上升，你手里的旧债券就不值钱了。"
        ],
        "type": "narration"
      },
      {
        "id": "s012",
        "introduced_terms": [
          "credit_risk"
        ],
        "lines": [
          "第三种：**<term id=\"credit_risk\">信用风险</term>**。",
          "借钱的人不还钱了，这就是信用风险。"
        ],
        "type": "narration"
      },
      {
        "id": "s013",
        "introduced_terms": [
          "liquidity_risk"
        ],
        "lines": [
          "第四种：**<term id=\"liquidity_risk\">流动性风险</term>**。",
          "急需用钱时，手里的资产卖不掉，或者只能打折卖。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 3,
          "explanation": "无法快速将资产变现而不损失价值，正是流动性风险的核心。",
          "kind": "single_choice",
          "options": [
            "市场风险",
            "利率风险",
            "信用风险",
            "流动性风险"
          ],
          "prompt": "这主要体现了哪种风险？"
        },
        "id": "s014",
        "lines": [
          "一家公司突然需要大量现金，但它的资产主要是很难快速出手的房产。",
          "这主要体现了哪种风险？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step2",
    "source": {
      "plain_text": "Major Risk Types for a financial institution: 1. Market Risk (Equity Risk, FX Risk, Commodity Risk), 2. Interest rate Risk, 3. Credit Risk, 4. Liquidity Risk, 5. Operational Risk, 6. Legal Risk, 7. Reputational Risk, 8. Strategic Risk.",
      "related": [
        "market risk",
        "interest rate risk",
        "credit risk",
        "liquidity risk"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "credit_risk": {
        "aliases": [
          "Credit Risk"
        ],
        "display": "信用风险",
        "gloss": "借款方未能偿还贷款或履行合约义务而导致损失的风险。"
      },
      "interest_rate_risk": {
        "aliases": [
          "Interest Rate Risk"
        ],
        "display": "利率风险",
        "gloss": "因利率波动导致投资损失的风险，主要影响债券等固定收益证券。"
      },
      "liquidity_risk": {
        "aliases": [
          "Liquidity Risk"
        ],
        "display": "流动性风险",
        "gloss": "无法在不显著损失价值的情况下，快速将资产转换为现金的风险。"
      },
      "market_risk": {
        "aliases": [
          "Market Risk"
        ],
        "display": "市场风险",
        "gloss": "因市场价格变动导致损失的风险。"
      }
    }
  },
  {
    "lesson_id": "L6",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s015",
        "introduced_terms": [
          "volatility"
        ],
        "lines": [
          "识别了风险，下一步就是衡量它有多大。",
          "最常用的工具是**<term id=\"volatility\">波动率</term>**。"
        ],
        "type": "narration"
      },
      {
        "id": "s016",
        "introduced_terms": [
          "standard_deviation"
        ],
        "lines": [
          "波动率通常用**<term id=\"standard_deviation\">标准差</term>**来衡量。",
          "标准差越大，说明价格波动越剧烈，风险也越高。"
        ],
        "type": "narration"
      },
      {
        "formula": {
          "latex": "\\sigma_{t,n} = \\sqrt{\\frac{1}{n}\\sum_{i=1}^{n}(x_{t-i} - \\mu)^2}",
          "spoken": "sigma t n 等于 n 分之一乘以 x 减去 mu 的平方和，再开根号。"
        },
        "id": "s017",
        "lines": [
          "计算过去 n 天的标准差，公式是这样的："
        ],
        "type": "formula"
      },
      {
        "id": "s018",
        "introduced_terms": [],
        "lines": [
          "但简单的标准差有个问题：它给每一天的数据相同的权重。",
          "而实际上，最近发生的事情往往更重要。"
        ],
        "type": "narration"
      },
      {
        "id": "s019",
        "introduced_terms": [
          "ewma"
        ],
        "lines": [
          "于是有了**<term id=\"ewma\">指数加权移动平均（EWMA）</term>**。",
          "它给近期的数据更高的权重，反应更灵敏。"
        ],
        "type": "narration"
      },
      {
        "id": "s020",
        "introduced_terms": [
          "volatility_clustering"
        ],
        "lines": [
          "还有一个有趣的现象：**<term id=\"volatility_clustering\">波动率聚集</term>**。",
          "大的波动之后往往跟着大的波动，小的之后跟着小的。"
        ],
        "type": "narration"
      },
      {
        "id": "s021",
        "introduced_terms": [],
        "lines": [
          "比如市场崩盘时，波动率会持续很高。",
          "而在平静期，波动率会持续很低。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "EWMA 通过给近期数据更高权重，能更快地反映市场的最新变化。",
          "kind": "single_choice",
          "options": [
            "计算更简单",
            "给近期数据更高权重，反应更灵敏",
            "不需要历史数据",
            "结果总是更小"
          ],
          "prompt": "EWMA 相比于简单标准差，最大的优势是什么？"
        },
        "id": "s022",
        "lines": [
          "EWMA 相比于简单标准差，最大的优势是什么？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step3",
    "source": {
      "plain_text": "Measuring Risk: 1. Volatility Analysis (Historical estimates, EWMA, GARCH model), 2. Value at Risk (VaR), 3. Stress Testing and Scenario Analysis. Standard deviation is a statistical measure that quantifies the amount of variation or dispersion in a set of data values. Volatility is often measured using standard deviation. EWMA: exponentially weighted moving averages by putting more weight on the recent data. Volatility clustering refers to the observation that large changes in asset prices are often followed by large changes.",
      "related": [
        "volatility",
        "standard deviation",
        "EWMA",
        "volatility clustering"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "ewma": {
        "aliases": [
          "EWMA",
          "Exponentially Weighted Moving Average"
        ],
        "display": "指数加权移动平均",
        "gloss": "一种给近期数据更高权重的波动率估计方法。"
      },
      "standard_deviation": {
        "aliases": [
          "Standard Deviation"
        ],
        "display": "标准差",
        "gloss": "衡量一组数据离散程度的统计量。"
      },
      "volatility": {
        "aliases": [
          "Volatility"
        ],
        "display": "波动率",
        "gloss": "衡量资产价格变动幅度的指标，通常用标准差表示。"
      },
      "volatility_clustering": {
        "aliases": [
          "Volatility Clustering"
        ],
        "display": "波动率聚集",
        "gloss": "大的价格变化后往往跟着大的变化，小的变化后跟着小的变化的现象。"
      }
    }
  },
  {
    "lesson_id": "L6",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s023",
        "introduced_terms": [],
        "lines": [
          "波动率聚集告诉我们，波动率不是随机的，而是有规律的。",
          "过去的波动会影响未来的波动。"
        ],
        "type": "narration"
      },
      {
        "id": "s024",
        "introduced_terms": [
          "arch_effect"
        ],
        "lines": [
          "这种规律被称为**<term id=\"arch_effect\">ARCH 效应</term>**。",
          "为了捕捉这种效应，科学家们发明了专门的模型。"
        ],
        "type": "narration"
      },
      {
        "id": "s025",
        "introduced_terms": [
          "arch"
        ],
        "lines": [
          "第一个模型叫**<term id=\"arch\">ARCH 模型</term>**。",
          "它假设今天的波动率，取决于过去几天的“冲击”（即价格的大幅变动）。"
        ],
        "type": "narration"
      },
      {
        "formula": {
          "latex": "\\sigma_t^2 = \\alpha_0 + \\alpha_1 a_{t-1}^2",
          "spoken": "sigma t 平方等于 alpha 0 加上 alpha 1 乘以 a t-1 的平方。"
        },
        "id": "s026",
        "lines": [
          "最简单的 ARCH(1) 模型："
        ],
        "type": "formula"
      },
      {
        "id": "s027",
        "introduced_terms": [
          "garch"
        ],
        "lines": [
          "但 ARCH 模型有个缺点：为了准确，它需要很多个过去的“冲击”项。",
          "于是有了它的升级版：**<term id=\"garch\">GARCH 模型</term>**。"
        ],
        "type": "narration"
      },
      {
        "id": "s028",
        "introduced_terms": [],
        "lines": [
          "GARCH 模型不仅考虑过去的冲击，还考虑过去的波动率本身。",
          "这就像给波动率加上了“记忆”。"
        ],
        "type": "narration"
      },
      {
        "formula": {
          "latex": "\\sigma_t^2 = \\alpha_0 + \\alpha_1 a_{t-1}^2 + \\beta_1 \\sigma_{t-1}^2",
          "spoken": "sigma t 平方等于 alpha 0 加上 alpha 1 乘以 a t-1 的平方，再加上 beta 1 乘以 sigma t-1 的平方。"
        },
        "id": "s029",
        "lines": [
          "最常用的 GARCH(1,1) 模型："
        ],
        "type": "formula"
      },
      {
        "id": "s030",
        "introduced_terms": [],
        "lines": [
          "GARCH(1,1) 是业界最常用的波动率模型之一。",
          "它用三个参数，就能很好地描述金融市场的波动特征。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "GARCH 模型在 ARCH 的基础上，加入了过去的波动率项，使得模型更简洁、更有效。",
          "kind": "single_choice",
          "options": [
            "计算速度更快",
            "加入了波动率自身的滞后项",
            "不需要历史数据",
            "只能用于股票市场"
          ],
          "prompt": "GARCH 模型相比于 ARCH 模型，最主要的改进是什么？"
        },
        "id": "s031",
        "lines": [
          "GARCH 模型相比于 ARCH 模型，最主要的改进是什么？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step4",
    "source": {
      "plain_text": "The ARCH or GARCH model are commonly used to model time-varying volatility, which take into account of volatility clustering. When volatility is autoregressive, we call this the ARCH effect. The GARCH(m, s) model reduces to a pure ARCH(m) model if s = 0. The most commonly used GARCH model is the simplest GARCH(1,1) model.",
      "related": [
        "ARCH",
        "GARCH",
        "ARCH effect"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "arch": {
        "aliases": [
          "ARCH",
          "Autoregressive Conditional Heteroskedasticity"
        ],
        "display": "ARCH 模型",
        "gloss": "自回归条件异方差模型，用于建模随时间变化的波动率。"
      },
      "arch_effect": {
        "aliases": [
          "ARCH Effect"
        ],
        "display": "ARCH 效应",
        "gloss": "指波动率具有自相关性，即过去的波动会影响未来的波动。"
      },
      "garch": {
        "aliases": [
          "GARCH",
          "Generalized Autoregressive Conditional Heteroskedasticity"
        ],
        "display": "GARCH 模型",
        "gloss": "广义自回归条件异方差模型，是 ARCH 模型的扩展，加入了波动率自身的滞后项。"
      }
    }
  },
  {
    "lesson_id": "L6",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s032",
        "introduced_terms": [],
        "lines": [
          "波动率告诉我们风险的大小，但还不够直观。",
          "老板问：“明天最多可能亏多少钱？”"
        ],
        "type": "narration"
      },
      {
        "id": "s033",
        "introduced_terms": [
          "var"
        ],
        "lines": [
          "答案就是**<term id=\"var\">在险价值（VaR）</term>**。",
          "它用一个数字告诉你：在 95% 的置信度下，明天最多亏多少钱。"
        ],
        "type": "narration"
      },
      {
        "id": "s034",
        "introduced_terms": [],
        "lines": [
          "比如，95% VaR 是 100 万。",
          "意思是，明天有 95% 的概率，亏损不会超过 100 万。"
        ],
        "type": "narration"
      },
      {
        "id": "s035",
        "introduced_terms": [
          "parametric_var"
        ],
        "lines": [
          "计算 VaR 有多种方法。",
          "第一种：**<term id=\"parametric_var\">参数法</term>**。假设收益率符合正态分布，然后直接算。"
        ],
        "type": "narration"
      },
      {
        "id": "s036",
        "introduced_terms": [
          "historical_var"
        ],
        "lines": [
          "第二种：**<term id=\"historical_var\">历史模拟法</term>**。",
          "把过去 N 天的收益率从低到高排序，取第 5% 最差的那个。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answers": [
            "-0.8%"
          ],
          "explanation": "将收益率排序后，第 10% 分位点（即最差的第 1 个）是 -0.8%。",
          "kind": "fill_in_blank",
          "prompt": "用历史模拟法，90% 的 VaR 是多少？（答案请用百分比表示，如 -1.3%）"
        },
        "id": "s037",
        "lines": [
          "过去 10 天的收益率是：1.1%, 2.4%, -1.3%, -0.4%, 3%, 2.6%, 1.4%, -0.8%, 0.9%, -0.5%。",
          "用历史模拟法，90% 的 VaR 是多少？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step5",
    "source": {
      "plain_text": "Value at Risk (VaR). VaR is the 'maximum' loss of an asset over a time horizon with a high confidence. Parametric VaR (or Variance-Covariance method). Historical VaR. Hypothetical VaR. Monte Carlo Simulation.",
      "related": [
        "VaR",
        "Parametric VaR",
        "Historical VaR"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "historical_var": {
        "aliases": [
          "Historical VaR"
        ],
        "display": "历史模拟法 VaR",
        "gloss": "基于过去实际收益率数据，通过排序来估算 VaR。"
      },
      "parametric_var": {
        "aliases": [
          "Parametric VaR",
          "Variance-Covariance Method"
        ],
        "display": "参数法 VaR",
        "gloss": "假设收益率服从某种分布（如正态分布）来计算 VaR。"
      },
      "var": {
        "aliases": [
          "VaR",
          "Value at Risk"
        ],
        "display": "在险价值",
        "gloss": "在给定置信水平和时间范围内，资产可能的最大损失。"
      }
    }
  },
  {
    "lesson_id": "L6",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s038",
        "introduced_terms": [],
        "lines": [
          "VaR 基于“正常”市场情况。",
          "但如果金融危机重演呢？"
        ],
        "type": "narration"
      },
      {
        "id": "s039",
        "introduced_terms": [
          "stress_testing"
        ],
        "lines": [
          "这就需要**<term id=\"stress_testing\">压力测试</term>**。",
          "它问的是：“如果 2008 年金融危机再来一次，我的投资组合会怎样？”"
        ],
        "type": "narration"
      },
      {
        "id": "s040",
        "introduced_terms": [
          "scenario_analysis"
        ],
        "lines": [
          "另一种方法是**<term id=\"scenario_analysis\">情景分析</term>**。",
          "它构建各种“如果……会怎样”的场景。"
        ],
        "type": "narration"
      },
      {
        "id": "s041",
        "introduced_terms": [],
        "lines": [
          "比如：如果美联储突然加息到 50%？",
          "如果港股和美元脱钩？"
        ],
        "type": "narration"
      },
      {
        "id": "s042",
        "introduced_terms": [],
        "lines": [
          "这些极端情况虽然概率小，但一旦发生，后果可能是毁灭性的。",
          "提前做好准备，才能有备无患。"
        ],
        "type": "narration"
      }
    ],
    "sequence_id": "step6",
    "source": {
      "plain_text": "Stress VaR calculates potential loss for holding the current portfolio during a stress period. Scenario analysis consists of evaluating the portfolio under various extreme but probable states of the world.",
      "related": [
        "stress testing",
        "scenario analysis"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "scenario_analysis": {
        "aliases": [
          "Scenario Analysis"
        ],
        "display": "情景分析",
        "gloss": "通过构建各种极端但可能发生的情景来评估投资组合的风险。"
      },
      "stress_testing": {
        "aliases": [
          "Stress Testing"
        ],
        "display": "压力测试",
        "gloss": "评估投资组合在极端市场条件下的潜在损失。"
      }
    }
  },
  {
    "lesson_id": "L6",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s043",
        "introduced_terms": [
          "position_sizing"
        ],
        "lines": [
          "知道风险有多大后，下一个问题是：每次交易该投入多少？",
          "这就是**<term id=\"position_sizing\">仓位管理</term>**。"
        ],
        "type": "narration"
      },
      {
        "id": "s044",
        "introduced_terms": [
          "fixed_size"
        ],
        "lines": [
          "最简单的策略：**<term id=\"fixed_size\">固定规模法</term>**。",
          "每次都买固定数量，比如 100 股。简单，但不够灵活。"
        ],
        "type": "narration"
      },
      {
        "id": "s045",
        "introduced_terms": [
          "balance_rescaling"
        ],
        "lines": [
          "进阶一点：**<term id=\"balance_rescaling\">余额重缩放法</term>**。",
          "账户资金变多了，就多买点；变少了，就少买点。"
        ],
        "type": "narration"
      },
      {
        "id": "s046",
        "introduced_terms": [
          "dollar_risk_approach"
        ],
        "lines": [
          "更精细的：**<term id=\"dollar_risk_approach\">美元风险法</term>**。",
          "每笔交易只承担账户总资金的 1% 作为风险。"
        ],
        "type": "narration"
      },
      {
        "id": "s047",
        "introduced_terms": [
          "kelly_criterion"
        ],
        "lines": [
          "最后，还有一个数学上最优的策略：**<term id=\"kelly_criterion\">凯利公式</term>**。",
          "它告诉你，为了长期让资金增长最快，应该下多大比例。"
        ],
        "type": "narration"
      },
      {
        "formula": {
          "latex": "f^* = \\frac{bp - q}{b}",
          "spoken": "f 星等于 b 乘以 p 减去 q，再除以 b。"
        },
        "id": "s048",
        "lines": [
          "凯利公式："
        ],
        "type": "formula"
      },
      {
        "id": "s049",
        "introduced_terms": [],
        "lines": [
          "其中 p 是胜率，q 是败率，b 是赔率。",
          "如果算出来是负数，说明这个交易不值得做。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answers": [
            "0.2"
          ],
          "explanation": "f* = (1*0.6 - 0.4) / 1 = 0.2，即每次投入 20% 的资金。",
          "kind": "fill_in_blank",
          "prompt": "根据凯利公式，最优投注比例是多少？（请用小数表示，如 0.2）"
        },
        "id": "s050",
        "lines": [
          "一个交易策略胜率 60%，赔率是 1:1（即赢了赚 1 元，输了亏 1 元）。",
          "根据凯利公式，最优投注比例是多少？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step7",
    "source": {
      "plain_text": "Common position sizing strategy: 1. Fixed Size, 2. Balance Rescaling, 3. Dollar Risk Approach, 4. Kelly Criterion.",
      "related": [
        "position sizing",
        "fixed size",
        "balance rescaling",
        "dollar risk approach",
        "kelly criterion"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "balance_rescaling": {
        "aliases": [
          "Balance Rescaling"
        ],
        "display": "余额重缩放法",
        "gloss": "根据账户余额的变化，按比例调整交易规模。"
      },
      "dollar_risk_approach": {
        "aliases": [
          "Dollar Risk Approach"
        ],
        "display": "美元风险法",
        "gloss": "每笔交易只承担账户总资金一个固定百分比的风险。"
      },
      "fixed_size": {
        "aliases": [
          "Fixed Size"
        ],
        "display": "固定规模法",
        "gloss": "每次交易都使用相同的数量。"
      },
      "kelly_criterion": {
        "aliases": [
          "Kelly Criterion"
        ],
        "display": "凯利公式",
        "gloss": "用于确定最优投注比例，以最大化长期资金增长率的公式。"
      },
      "position_sizing": {
        "aliases": [
          "Position Sizing"
        ],
        "display": "仓位管理",
        "gloss": "决定每笔交易投入多少资金的方法。"
      }
    }
  },
  {
    "lesson_id": "L6",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s051",
        "introduced_terms": [],
        "lines": [
          "凯利公式看起来很完美，但它有严格的假设。",
          "它假设每次交易的结果是独立的，且胜率和赔率是固定的。"
        ],
        "type": "narration"
      },
      {
        "id": "s052",
        "introduced_terms": [],
        "lines": [
          "但在真实交易中，胜率和赔率是变化的，交易之间也可能存在相关性。",
          "所以直接套用凯利公式，结果可能并不好。"
        ],
        "type": "narration"
      },
      {
        "id": "s053",
        "introduced_terms": [],
        "lines": [
          "一个常见的改进是：只使用凯利公式计算出的比例的一部分。",
          "比如用一半，来降低风险。"
        ],
        "type": "narration"
      },
      {
        "id": "s054",
        "introduced_terms": [],
        "lines": [
          "回顾一下，我们走过了完整的风险管理之旅：",
          "从识别风险，到用波动率和 VaR 衡量风险，再到用仓位管理来控制风险。"
        ],
        "type": "narration"
      },
      {
        "id": "s055",
        "introduced_terms": [],
        "lines": [
          "记住一句话：",
          "**风险管理不是要消除风险，而是要在了解风险的基础上，做出明智的决策。**"
        ],
        "type": "narration"
      }
    ],
    "sequence_id": "step8",
    "source": {
      "plain_text": "Kelly Criterion Assumptions: Gambling vs Investment. The risk and reward are determined. Each bet has only binary outcomes. Each bet is independent. For risk averse traders, only trade at a fraction of f*. Key Takeaways.",
      "related": [
        "kelly criterion limitations"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {}
  }
]

</GUIDED_STORY_STEPS>

<QUESTION_BANK>
[
{
  "coverage_map": [
    {
      "coverage_tag": "arch_effect",
      "covered_by": [
        "qf_flash_arch_effect",
        "qf_quiz_arch_effect"
      ],
      "description": "ARCH 效应：波动率具有自相关性，过去的波动影响未来。"
    },
    {
      "coverage_tag": "arch_model",
      "covered_by": [
        "qf_flash_arch_model",
        "qf_quiz_arch_model",
        "qf_long_arch_garch"
      ],
      "description": "ARCH 模型：自回归条件异方差模型，用于建模时变波动率。"
    },
    {
      "coverage_tag": "garch_model",
      "covered_by": [
        "qf_flash_garch_model",
        "qf_quiz_garch_model",
        "qf_long_arch_garch"
      ],
      "description": "GARCH 模型：广义自回归条件异方差模型，是 ARCH 的扩展，加入波动率自身的滞后项。"
    },
    {
      "coverage_tag": "garch_vs_arch",
      "covered_by": [
        "qf_flash_garch_vs_arch",
        "qf_quiz_garch_vs_arch"
      ],
      "description": "GARCH 相比于 ARCH 的主要改进：加入了波动率自身的滞后项，模型更简洁。"
    },
    {
      "coverage_tag": "garch_formula",
      "covered_by": [
        "qf_flash_garch_formula",
        "qf_quiz_garch_formula"
      ],
      "description": "GARCH(1,1) 公式：σ_t² = α₀ + α₁ a_{t-1}² + β₁ σ_{t-1}²"
    },
    {
      "coverage_tag": "arch_formula",
      "covered_by": [
        "qf_flash_arch_formula",
        "qf_quiz_arch_formula"
      ],
      "description": "ARCH(1) 公式：σ_t² = α₀ + α₁ a_{t-1}²"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "arch_effect",
      "coverage_tags": [
        "arch_effect"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_arch_effect",
      "learning_goal": "学生能准确回忆 ARCH 效应的定义及其与波动率聚集的关系。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "ARCH 效应的核心定义：波动率具有自相关性。",
      "term_refs": [
        {
          "display": "ARCH 效应",
          "en": "ARCH Effect"
        }
      ],
      "variants": [
        {
          "back": "波动率具有自相关性，即过去的波动会影响未来的波动。",
          "estimated_seconds": 8,
          "explanation": "ARCH 效应是 Autoregressive Conditional Heteroskedasticity 的缩写，指波动率不是随机的，而是表现出聚集性和自相关性。",
          "front": "ARCH 效应指的是什么现象？",
          "question_id": "q_flash_arch_effect_v1"
        },
        {
          "back": "ARCH 效应。",
          "estimated_seconds": 6,
          "explanation": "波动率聚集（大的波动后跟大的波动，小的波动后跟小的波动）是 ARCH 效应的直观表现。",
          "front": "波动率聚集现象与哪个概念直接相关？",
          "question_id": "q_flash_arch_effect_v2"
        }
      ]
    },
    {
      "concept_key": "arch_model",
      "coverage_tags": [
        "arch_model"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_arch_model",
      "learning_goal": "学生能准确回忆 ARCH 模型的基本假设和用途。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "ARCH 模型的核心假设：今天的波动率取决于过去几天的冲击。",
      "term_refs": [
        {
          "display": "ARCH 模型",
          "en": "ARCH Model"
        }
      ],
      "variants": [
        {
          "back": "过去几天的“冲击”（即价格的大幅变动）。",
          "estimated_seconds": 8,
          "explanation": "ARCH 模型通过过去几天的冲击项（a_{t-1}², a_{t-2}², ...）来预测今天的波动率。",
          "front": "ARCH 模型假设今天的波动率取决于什么？",
          "question_id": "q_flash_arch_model_v1"
        },
        {
          "back": "自回归条件异方差模型（Autoregressive Conditional Heteroskedasticity）。",
          "estimated_seconds": 6,
          "explanation": "ARCH 模型专门用于建模随时间变化的波动率。",
          "front": "ARCH 模型的全称是什么？",
          "question_id": "q_flash_arch_model_v2"
        }
      ]
    },
    {
      "concept_key": "garch_model",
      "coverage_tags": [
        "garch_model"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_garch_model",
      "learning_goal": "学生能准确回忆 GARCH 模型的核心特征和用途。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "GARCH 模型的核心特征：不仅考虑过去的冲击，还考虑过去的波动率本身。",
      "term_refs": [
        {
          "display": "GARCH 模型",
          "en": "GARCH Model"
        }
      ],
      "variants": [
        {
          "back": "波动率自身的滞后项（过去的波动率）。",
          "estimated_seconds": 8,
          "explanation": "GARCH 模型加入了 σ_{t-1}² 项，使得模型具有“记忆”效应。",
          "front": "GARCH 模型在 ARCH 模型的基础上增加了什么？",
          "question_id": "q_flash_garch_model_v1"
        },
        {
          "back": "GARCH(1,1) 模型。",
          "estimated_seconds": 6,
          "explanation": "GARCH(1,1) 用三个参数（α₀, α₁, β₁）就能很好地描述金融市场的波动特征。",
          "front": "业界最常用的 GARCH 模型是哪个？",
          "question_id": "q_flash_garch_model_v2"
        }
      ]
    },
    {
      "concept_key": "garch_vs_arch",
      "coverage_tags": [
        "garch_vs_arch"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_garch_vs_arch",
      "learning_goal": "学生能准确区分 GARCH 和 ARCH 模型的核心差异。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "core_difference",
      "retrieval_focus": "GARCH 相比于 ARCH 的核心改进：加入了波动率自身的滞后项。",
      "term_refs": [
        {
          "display": "GARCH 模型",
          "en": "GARCH Model"
        },
        {
          "display": "ARCH 模型",
          "en": "ARCH Model"
        }
      ],
      "variants": [
        {
          "back": "加入了波动率自身的滞后项（σ_{t-1}²）。",
          "estimated_seconds": 10,
          "explanation": "这使得 GARCH 模型可以用更少的参数达到更好的拟合效果，解决了 ARCH 模型需要很多滞后项的问题。",
          "front": "GARCH 模型相比于 ARCH 模型，最主要的改进是什么？",
          "question_id": "q_flash_garch_vs_arch_v1"
        },
        {
          "back": "因为 GARCH 模型通过加入波动率自身的滞后项，减少了所需冲击项的数量。",
          "estimated_seconds": 10,
          "explanation": "ARCH 模型为了准确需要很多个过去的冲击项，而 GARCH 模型通过加入 σ_{t-1}² 项，用更少的参数就能捕捉波动率的持续性。",
          "front": "为什么 GARCH 模型比 ARCH 模型更简洁？",
          "question_id": "q_flash_garch_vs_arch_v2"
        }
      ]
    },
    {
      "concept_key": "garch_formula",
      "coverage_tags": [
        "garch_formula"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_garch_formula",
      "learning_goal": "学生能准确回忆 GARCH(1,1) 的公式并理解各参数的含义。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "micro_calc",
      "retrieval_focus": "GARCH(1,1) 公式的结构和参数含义。",
      "term_refs": [
        {
          "display": "GARCH(1,1) 模型",
          "en": "GARCH(1,1) Model"
        }
      ],
      "variants": [
        {
          "back": "σ_t² = α₀ + α₁ a_{t-1}² + β₁ σ_{t-1}²",
          "estimated_seconds": 12,
          "explanation": "其中 α₀ 是常数项，α₁ 是 ARCH 参数（对冲击的反应），β₁ 是 GARCH 参数（波动率的持续性）。",
          "front": "写出 GARCH(1,1) 模型的波动率方程。",
          "question_id": "q_flash_garch_formula_v1"
        },
        {
          "back": "GARCH 参数，表示波动率自身的持续性，即过去的波动率对当前波动率的影响程度。",
          "estimated_seconds": 10,
          "explanation": "β₁ 越大，说明冲击对条件方差的影响消退得越慢，波动率越持久。",
          "front": "在 GARCH(1,1) 模型中，参数 β₁ 代表什么含义？",
          "question_id": "q_flash_garch_formula_v2"
        }
      ]
    },
    {
      "concept_key": "arch_formula",
      "coverage_tags": [
        "arch_formula"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_arch_formula",
      "learning_goal": "学生能准确回忆 ARCH(1) 的公式并理解各参数的含义。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "micro_calc",
      "retrieval_focus": "ARCH(1) 公式的结构和参数含义。",
      "term_refs": [
        {
          "display": "ARCH(1) 模型",
          "en": "ARCH(1) Model"
        }
      ],
      "variants": [
        {
          "back": "σ_t² = α₀ + α₁ a_{t-1}²",
          "estimated_seconds": 10,
          "explanation": "其中 α₀ > 0，0 ≤ α₁ < 1，a_{t-1} 是上一期的冲击（残差）。",
          "front": "写出 ARCH(1) 模型的波动率方程。",
          "question_id": "q_flash_arch_formula_v1"
        },
        {
          "back": "ARCH 参数，表示过去冲击对当前波动率的影响程度。",
          "estimated_seconds": 8,
          "explanation": "α₁ 越大，说明过去的冲击对当前波动率的影响越大。",
          "front": "在 ARCH(1) 模型中，参数 α₁ 代表什么含义？",
          "question_id": "q_flash_arch_formula_v2"
        }
      ]
    }
  ],
  "lesson_id": "L6",
  "longform_families": [
    {
      "concept_key": "arch_garch_comparison",
      "coverage_tags": [
        "arch_model",
        "garch_model",
        "garch_vs_arch"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_arch_garch",
      "learning_goal": "学生能解释 ARCH 和 GARCH 模型的联系与区别，并说明为什么 GARCH 模型在实践中更常用。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "compare_and_contrast",
      "term_refs": [
        {
          "display": "ARCH 模型",
          "en": "ARCH Model"
        },
        {
          "display": "GARCH 模型",
          "en": "GARCH Model"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "写出 ARCH(1) 和 GARCH(1,1) 的波动率方程",
            "解释 GARCH 模型如何解决 ARCH 模型的局限性",
            "讨论 GARCH(1,1) 模型的实用优势"
          ],
          "question_id": "q_long_arch_garch_v1",
          "reference_answer": [
            "ARCH(1) 模型：σ_t² = α₀ + α₁ a_{t-1}²，假设今天的波动率只取决于昨天的冲击。",
            "GARCH(1,1) 模型：σ_t² = α₀ + α₁ a_{t-1}² + β₁ σ_{t-1}²，不仅考虑昨天的冲击，还考虑昨天的波动率本身。",
            "GARCH 的主要改进：ARCH 模型为了准确需要很多个过去的冲击项（高阶 ARCH），而 GARCH 通过加入 σ_{t-1}² 项，相当于给波动率加上了“记忆”，用更少的参数就能捕捉波动率的持续性。",
            "GARCH(1,1) 成为业界标准的原因：它用三个参数（α₀, α₁, β₁）就能很好地拟合金融市场的波动特征，包括波动率聚集、厚尾分布等，且参数含义直观（α₁ 反映对冲击的反应，β₁ 反映持续性）。"
          ],
          "rubric_points": [
            "正确写出 ARCH(1): σ_t² = α₀ + α₁ a_{t-1}²",
            "正确写出 GARCH(1,1): σ_t² = α₀ + α₁ a_{t-1}² + β₁ σ_{t-1}²",
            "指出 GARCH 加入了波动率自身的滞后项",
            "解释 ARCH 需要很多滞后项才能准确，而 GARCH 用更少的参数达到更好效果",
            "说明 GARCH(1,1) 用三个参数就能很好地描述金融市场的波动特征"
          ],
          "stem": "请比较 ARCH 模型和 GARCH 模型，说明：(1) 它们各自的核心公式；(2) GARCH 模型相对于 ARCH 模型的主要改进；(3) 为什么 GARCH(1,1) 成为业界最常用的波动率模型。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "识别现象并解释其含义",
            "选择模型并给出理由",
            "写出公式并解释参数"
          ],
          "question_id": "q_long_arch_garch_v2",
          "reference_answer": [
            "这个现象叫 ARCH 效应（或波动率聚集），说明该股票的波动率不是随机的，而是具有自相关性——大的波动后往往跟着大的波动。",
            "我会选择 GARCH 模型，最好是 GARCH(1,1)。因为 GARCH 模型比 ARCH 模型更简洁高效，用更少的参数就能捕捉波动率的持续性，而且 GARCH(1,1) 是业界最常用的模型。",
            "GARCH(1,1) 公式：σ_t² = α₀ + α₁ a_{t-1}² + β₁ σ_{t-1}²。其中 α₀ 是常数项（基础波动率水平），α₁ 是 ARCH 参数（衡量过去冲击对当前波动率的影响），β₁ 是 GARCH 参数（衡量波动率自身的持续性，即记忆效应）。"
          ],
          "rubric_points": [
            "正确识别为 ARCH 效应或波动率聚集",
            "解释说明波动率具有自相关性",
            "选择 GARCH 模型并给出合理理由",
            "正确写出 GARCH(1,1) 公式",
            "正确解释 α₀, α₁, β₁ 的含义"
          ],
          "stem": "假设你是一名量化分析师，发现某股票收益率的平方序列表现出显著的自相关性。请解释：(1) 这个现象叫什么？它说明了什么？(2) 你会选择 ARCH 还是 GARCH 模型来建模？为什么？(3) 写出你选择的模型的公式，并解释每个参数的含义。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "arch_effect",
      "coverage_tags": [
        "arch_effect"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_arch_effect",
      "learning_goal": "学生能在测验情境下识别和判断 ARCH 效应的存在。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "ARCH 效应",
          "en": "ARCH Effect"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "大的波动后跟大的波动，小的波动后跟小的波动，这正是波动率聚集的表现，也是 ARCH 效应的核心特征。",
          "options": [
            "股票价格长期呈上升趋势",
            "大的价格波动之后往往跟着大的价格波动",
            "股票收益率服从正态分布",
            "不同股票之间的相关性为常数"
          ],
          "question_id": "q_quiz_arch_effect_v1",
          "stem": "以下哪个现象最能说明 ARCH 效应的存在？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "收益率的平方序列自相关是 ARCH 效应的典型特征，因为波动率（用平方收益率近似）具有自相关性。",
          "options": [
            "市场是有效的",
            "ARCH 效应",
            "收益率是随机游走的",
            "不存在风险溢价"
          ],
          "question_id": "q_quiz_arch_effect_v2",
          "stem": "如果某资产收益率的平方序列表现出显著的自相关性，这通常表明存在什么？"
        }
      ]
    },
    {
      "concept_key": "arch_model",
      "coverage_tags": [
        "arch_model"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_arch_model",
      "learning_goal": "学生能在测验情境下理解 ARCH 模型的基本原理和适用场景。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "ARCH 模型",
          "en": "ARCH Model"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "ARCH 模型专门用于建模时变波动率，它考虑了波动率聚集现象，通过过去的冲击来预测当前的波动率。",
          "options": [
            "预测股票价格的长期趋势",
            "建模随时间变化的波动率",
            "计算投资组合的预期收益",
            "识别市场中的套利机会"
          ],
          "question_id": "q_quiz_arch_model_v1",
          "stem": "ARCH 模型的主要用途是什么？"
        },
        {
          "answer": 0,
          "estimated_seconds": 25,
          "explanation": "如果 α₁ = 0，则 σ_t² = α₀，波动率变为常数，说明不存在 ARCH 效应。",
          "options": [
            "波动率是常数",
            "波动率完全由过去的冲击决定",
            "模型无法估计",
            "收益率服从正态分布"
          ],
          "question_id": "q_quiz_arch_model_v2",
          "stem": "ARCH(1) 模型中，如果 α₁ = 0，这意味着什么？"
        }
      ]
    },
    {
      "concept_key": "garch_model",
      "coverage_tags": [
        "garch_model"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_garch_model",
      "learning_goal": "学生能在测验情境下理解 GARCH 模型的基本原理和优势。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "GARCH 模型",
          "en": "GARCH Model"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 25,
          "explanation": "β₁ 是 GARCH 参数，表示波动率的持续性。β₁ 越大，说明冲击对条件方差的影响消退得越慢，波动率越持久。",
          "options": [
            "波动率对市场冲击的反应非常剧烈",
            "波动率的冲击会持续很长时间",
            "模型无法收敛",
            "收益率序列是平稳的"
          ],
          "question_id": "q_quiz_garch_model_v1",
          "stem": "GARCH(1,1) 模型中的参数 β₁ 较大（接近 1）通常意味着什么？"
        },
        {
          "answer": 0,
          "estimated_seconds": 25,
          "explanation": "GARCH(1,1) 模型平稳的条件是 α₁ + β₁ < 1，这保证了无条件方差是有限且稳定的。",
          "options": [
            "α₁ + β₁ < 1",
            "α₁ > β₁",
            "α₀ = 0",
            "β₁ = 0"
          ],
          "question_id": "q_quiz_garch_model_v2",
          "stem": "以下哪个条件对于 GARCH(1,1) 模型的平稳性是必要的？"
        }
      ]
    },
    {
      "concept_key": "garch_vs_arch",
      "coverage_tags": [
        "garch_vs_arch"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_garch_vs_arch",
      "learning_goal": "学生能在测验情境下辨析 GARCH 和 ARCH 模型的区别。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "GARCH 模型",
          "en": "GARCH Model"
        },
        {
          "display": "ARCH 模型",
          "en": "ARCH Model"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 25,
          "explanation": "GARCH 模型通过加入波动率自身的滞后项，可以用更少的参数（如 GARCH(1,1)）达到比高阶 ARCH 模型更好的拟合效果。",
          "options": [
            "计算速度更快",
            "可以用更少的参数捕捉波动率的持续性",
            "不需要假设收益率分布",
            "只能用于高频数据"
          ],
          "question_id": "q_quiz_garch_vs_arch_v1",
          "stem": "GARCH 模型相比于 ARCH 模型最主要的优势是什么？"
        },
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "当 s = 0 时，GARCH 模型中的波动率滞后项全部消失，模型退化为纯 ARCH(m) 模型。",
          "options": [
            "模型退化为 ARCH(m) 模型",
            "模型变为常数方差模型",
            "模型无法估计",
            "模型变为 GARCH(m, 1) 模型"
          ],
          "question_id": "q_quiz_garch_vs_arch_v2",
          "stem": "当 GARCH(m, s) 模型中的 s = 0 时，会发生什么？"
        }
      ]
    },
    {
      "concept_key": "garch_formula",
      "coverage_tags": [
        "garch_formula"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_garch_formula",
      "learning_goal": "学生能在测验情境下正确识别 GARCH(1,1) 公式及其参数。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "GARCH(1,1) 模型",
          "en": "GARCH(1,1) Model"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "GARCH(1,1) 包含常数项 α₀、ARCH 项 α₁ a_{t-1}² 和 GARCH 项 β₁ σ_{t-1}²。",
          "options": [
            "σ_t² = α₀ + α₁ r_{t-1}²",
            "σ_t² = α₀ + α₁ a_{t-1}² + β₁ σ_{t-1}²",
            "σ_t² = α₀ + β₁ σ_{t-1}²",
            "σ_t² = α₁ a_{t-1}² + β₁ σ_{t-1}²"
          ],
          "question_id": "q_quiz_garch_formula_v1",
          "stem": "以下哪个公式正确表示了 GARCH(1,1) 模型？"
        },
        {
          "answer": 1,
          "estimated_seconds": 30,
          "explanation": "无条件方差 = α₀ / (1 - α₁ - β₁) = 0.00001 / (1 - 0.1 - 0.85) = 0.00001 / 0.05 = 0.0002。",
          "options": [
            "0.00002",
            "0.0002",
            "0.0001",
            "0.001"
          ],
          "question_id": "q_quiz_garch_formula_v2",
          "stem": "在 GARCH(1,1) 模型中，如果 α₁ = 0.1, β₁ = 0.85，那么无条件方差是多少？（假设 α₀ = 0.00001）"
        }
      ]
    },
    {
      "concept_key": "arch_formula",
      "coverage_tags": [
        "arch_formula"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_arch_formula",
      "learning_goal": "学生能在测验情境下正确识别 ARCH(1) 公式及其参数。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "ARCH(1) 模型",
          "en": "ARCH(1) Model"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 15,
          "explanation": "ARCH(1) 模型只包含常数项 α₀ 和一个滞后冲击项 α₁ a_{t-1}²。",
          "options": [
            "σ_t² = α₀ + α₁ a_{t-1}²",
            "σ_t² = α₀ + α₁ a_{t-1}² + β₁ σ_{t-1}²",
            "σ_t² = α₁ a_{t-1}²",
            "σ_t² = α₀ + β₁ σ_{t-1}²"
          ],
          "question_id": "q_quiz_arch_formula_v1",
          "stem": "以下哪个公式正确表示了 ARCH(1) 模型？"
        },
        {
          "answer": 0,
          "estimated_seconds": 25,
          "explanation": "无条件方差 = α₀ / (1 - α₁) = 0.0001 / (1 - 0.5) = 0.0001 / 0.5 = 0.0002。",
          "options": [
            "0.0002",
            "0.0001",
            "0.00005",
            "0.00015"
          ],
          "question_id": "q_quiz_arch_formula_v2",
          "stem": "在 ARCH(1) 模型中，如果 α₁ = 0.5，α₀ = 0.0001，那么 a_t 的无条件方差是多少？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L6/plain.txt",
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
      "coverage_tag": "kelly_criterion_limitations",
      "covered_by": [
        "qf_flash_kelly_limits",
        "qf_quiz_kelly_assumptions",
        "qf_long_kelly_limitations"
      ],
      "description": "凯利公式的严格假设及其在真实交易中的局限性"
    },
    {
      "coverage_tag": "kelly_criterion_enhancement",
      "covered_by": [
        "qf_flash_kelly_fraction",
        "qf_quiz_kelly_fraction",
        "qf_long_kelly_limitations"
      ],
      "description": "针对凯利公式局限性的常见改进方法（如使用分数比例）"
    },
    {
      "coverage_tag": "risk_management_summary",
      "covered_by": [
        "qf_flash_rm_cycle_recap",
        "qf_quiz_rm_cycle_recap"
      ],
      "description": "风险管理循环的完整回顾：识别、衡量、控制风险"
    },
    {
      "coverage_tag": "risk_management_philosophy",
      "covered_by": [
        "qf_flash_rm_philosophy",
        "qf_quiz_rm_philosophy"
      ],
      "description": "风险管理的核心理念：不是消除风险，而是明智决策"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "kelly_criterion_limitations",
      "coverage_tags": [
        "kelly_criterion_limitations"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_kelly_limits",
      "learning_goal": "学生能准确回忆凯利公式在真实交易中的两个关键假设及其与现实不符之处。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "凯利公式的两个核心假设（独立性与固定胜率/赔率）以及它们在真实交易中为何不成立。",
      "term_refs": [
        {
          "display": "凯利公式",
          "en": "Kelly Criterion"
        },
        {
          "display": "独立性假设",
          "en": "Independence Assumption"
        }
      ],
      "variants": [
        {
          "back": "相互独立",
          "estimated_seconds": 8,
          "explanation": "凯利公式假设每次交易的结果是独立的，但真实交易中可能存在序列相关性。",
          "front": "凯利公式假设每次交易的结果之间是什么关系？",
          "question_id": "q_flash_kelly_limits_v1"
        },
        {
          "back": "固定不变",
          "estimated_seconds": 8,
          "explanation": "凯利公式假设胜率和赔率是确定的常数，但真实市场中它们是变化的。",
          "front": "凯利公式假设交易的胜率和赔率是什么状态？",
          "question_id": "q_flash_kelly_limits_v2"
        },
        {
          "back": "因为真实交易的胜率/赔率会变化，且交易之间可能存在相关性",
          "estimated_seconds": 10,
          "explanation": "凯利公式的严格假设（独立、固定参数）在真实金融市场中通常不成立。",
          "front": "为什么直接套用凯利公式到真实交易中结果可能不好？",
          "question_id": "q_flash_kelly_limits_v3"
        }
      ]
    },
    {
      "concept_key": "kelly_criterion_enhancement",
      "coverage_tags": [
        "kelly_criterion_enhancement"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_kelly_fraction",
      "learning_goal": "学生能回忆针对凯利公式局限性的一个常见改进方法。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "风险厌恶交易者如何调整凯利公式的结果。",
      "term_refs": [
        {
          "display": "分数凯利",
          "en": "Fractional Kelly"
        }
      ],
      "variants": [
        {
          "back": "只使用计算出的比例的一部分（如一半）",
          "estimated_seconds": 8,
          "explanation": "使用分数凯利（Fractional Kelly）可以降低风险，提高策略的稳健性。",
          "front": "风险厌恶的交易者通常如何使用凯利公式的结果？",
          "question_id": "q_flash_kelly_fraction_v1"
        },
        {
          "back": "一部分（例如一半）",
          "estimated_seconds": 6,
          "explanation": "这被称为分数凯利策略，旨在平衡增长潜力与风险。",
          "front": "为了降低风险，一个常见的改进是只使用凯利公式计算出的比例的多少？",
          "question_id": "q_flash_kelly_fraction_v2"
        }
      ]
    },
    {
      "concept_key": "risk_management_cycle",
      "coverage_tags": [
        "risk_management_summary"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_rm_cycle_recap",
      "learning_goal": "学生能回忆风险管理循环的四个主要步骤。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "风险管理循环的四个步骤。",
      "term_refs": [
        {
          "display": "风险管理循环",
          "en": "Risk Management Cycle"
        }
      ],
      "variants": [
        {
          "back": "识别风险",
          "estimated_seconds": 5,
          "explanation": "风险管理循环包括：识别、评估/衡量、处理、监控。",
          "front": "风险管理循环的第一步是什么？",
          "question_id": "q_flash_rm_cycle_recap_v1"
        },
        {
          "back": "评估/衡量风险",
          "estimated_seconds": 5,
          "explanation": "在识别风险后，需要衡量其大小，例如使用波动率或VaR。",
          "front": "风险管理循环的第二步是什么？",
          "question_id": "q_flash_rm_cycle_recap_v2"
        },
        {
          "back": "处理风险",
          "estimated_seconds": 5,
          "explanation": "处理风险包括仓位管理等策略。",
          "front": "风险管理循环的第三步是什么？",
          "question_id": "q_flash_rm_cycle_recap_v3"
        },
        {
          "back": "监控风险",
          "estimated_seconds": 5,
          "explanation": "风险需要持续监控，以确保管理措施有效。",
          "front": "风险管理循环的第四步是什么？",
          "question_id": "q_flash_rm_cycle_recap_v4"
        }
      ]
    },
    {
      "concept_key": "risk_management_philosophy",
      "coverage_tags": [
        "risk_management_philosophy"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_rm_philosophy",
      "learning_goal": "学生能准确复述风险管理的核心理念。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "风险管理的最终目标是什么。",
      "term_refs": [
        {
          "display": "风险管理",
          "en": "Risk Management"
        }
      ],
      "variants": [
        {
          "back": "不是消除风险，而是在了解风险的基础上做出明智的决策",
          "estimated_seconds": 8,
          "explanation": "风险是投资中固有的，管理的目的是更好地理解和应对它。",
          "front": "风险管理的核心目标是什么？",
          "question_id": "q_flash_rm_philosophy_v1"
        },
        {
          "back": "在了解风险的基础上做出明智的决策",
          "estimated_seconds": 8,
          "explanation": "风险无法被完全消除，但可以被识别、衡量和管理。",
          "front": "根据课程总结，风险管理应该如何看待风险？",
          "question_id": "q_flash_rm_philosophy_v2"
        }
      ]
    }
  ],
  "lesson_id": "L6",
  "longform_families": [
    {
      "concept_key": "kelly_criterion_limitations",
      "coverage_tags": [
        "kelly_criterion_limitations",
        "kelly_criterion_enhancement"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_kelly_limitations",
      "learning_goal": "学生能解释凯利公式在赌博与投资中的假设差异，并说明分数凯利策略的原理。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "compare_and_contrast",
      "term_refs": [
        {
          "display": "凯利公式",
          "en": "Kelly Criterion"
        },
        {
          "display": "分数凯利",
          "en": "Fractional Kelly"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "凯利公式在赌博中的假设",
            "这些假设在投资中为何不成立",
            "一种改进方法及其原理"
          ],
          "question_id": "q_long_kelly_limitations_v1",
          "reference_answer": [
            "凯利公式在赌博中表现良好，因为它假设每次下注的风险回报（赔率）和胜率是确定的，并且每次下注的结果是独立的。",
            "然而，在真实投资中，这些假设通常不成立。交易的胜率和赔率会随市场变化，交易之间也可能存在相关性。",
            "一种常见的改进方法是使用分数凯利（Fractional Kelly），即只使用凯利公式计算出的最优比例的一部分（例如一半）。这样可以降低策略的波动性和风险，使策略更加稳健。"
          ],
          "rubric_points": [
            "正确指出凯利公式假设赌博中风险回报和胜率是确定的（1分）。",
            "正确指出凯利公式假设每次下注是独立的（1分）。",
            "解释在投资中，胜率和赔率是变化的，交易可能存在序列相关性（2分）。",
            "提出分数凯利（Fractional Kelly）作为改进方法（1分）。",
            "解释分数凯利通过使用计算出的比例的一部分来降低风险和波动性（1分）。"
          ],
          "stem": "请解释为什么凯利公式在赌博场景中表现良好，但在真实投资中直接套用效果不佳？并说明一种常见的改进方法及其原理。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "直接使用30%比例的风险",
            "凯利公式的假设为何不适用",
            "建议的稳健实施方案"
          ],
          "question_id": "q_long_kelly_limitations_v2",
          "reference_answer": [
            "直接使用30%的比例风险很高，因为凯利公式假设胜率和赔率是固定的，但市场是动态的。如果未来胜率下降，30%的仓位可能导致巨大亏损。",
            "一个更稳健的实施方案是使用分数凯利，例如只使用一半，即15%的仓位。这样可以降低策略对参数估计误差的敏感性，减少回撤，使资金曲线更平滑。"
          ],
          "rubric_points": [
            "指出直接使用可能因市场变化导致过度交易或巨大回撤（1分）。",
            "解释凯利公式假设参数固定，但实际胜率和赔率会变（2分）。",
            "建议使用分数凯利，例如使用15%（一半凯利）（1分）。",
            "解释分数凯利可以降低风险，提高策略的生存概率（1分）。"
          ],
          "stem": "一个交易者使用凯利公式计算出一个策略的最优仓位比例为 30%。请分析直接使用这个比例可能面临的风险，并建议一个更稳健的实施方案。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "kelly_criterion_limitations",
      "coverage_tags": [
        "kelly_criterion_limitations"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_kelly_assumptions",
      "learning_goal": "学生能辨析凯利公式的假设在赌博与投资场景下的区别。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "multiple_choice",
      "term_refs": [
        {
          "display": "凯利公式",
          "en": "Kelly Criterion"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "凯利公式假设每次下注的结果相互独立，但投资中交易可能存在序列相关性。",
          "options": [
            "每次交易的结果是独立的",
            "交易成本为零",
            "市场是有效的",
            "投资者是风险中性的"
          ],
          "question_id": "q_quiz_kelly_assumptions_v1",
          "stem": "以下哪项是凯利公式在赌博场景中的假设，但在真实投资中通常不成立？"
        },
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "投资的回报通常是连续的百分比变化，而不是赌博中固定的赢或输金额。",
          "options": [
            "投资回报是连续变量，而非二元结果",
            "投资者无法设定止损",
            "市场总是有效的",
            "交易次数总是有限的"
          ],
          "question_id": "q_quiz_kelly_assumptions_v2",
          "stem": "凯利公式假设每次交易的结果是二元的（赢或输），且风险回报是确定的。这在真实投资中面临什么问题？"
        }
      ]
    },
    {
      "concept_key": "kelly_criterion_enhancement",
      "coverage_tags": [
        "kelly_criterion_enhancement"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_kelly_fraction",
      "learning_goal": "学生能理解并应用分数凯利策略。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "分数凯利",
          "en": "Fractional Kelly"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "一半凯利即使用 f* 的一半：0.4 * 0.5 = 0.2。",
          "options": [
            "0.1",
            "0.2",
            "0.4",
            "0.8"
          ],
          "question_id": "q_quiz_kelly_fraction_v1",
          "stem": "一个交易者使用凯利公式计算出最优投注比例 f* 为 0.4。如果他是一个风险厌恶者，决定使用一半凯利，那么他实际投入的资金比例是多少？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "分数凯利通过使用更小的比例来降低风险，使策略更稳健。",
          "options": [
            "最大化长期增长率",
            "简化计算过程",
            "降低策略的风险和波动性",
            "提高交易的胜率"
          ],
          "question_id": "q_quiz_kelly_fraction_v2",
          "stem": "使用分数凯利（Fractional Kelly）的主要目的是什么？"
        }
      ]
    },
    {
      "concept_key": "risk_management_cycle",
      "coverage_tags": [
        "risk_management_summary"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_rm_cycle_recap",
      "learning_goal": "学生能正确排列风险管理循环的四个步骤。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "ordering",
      "term_refs": [
        {
          "display": "风险管理循环",
          "en": "Risk Management Cycle"
        }
      ],
      "variants": [
        {
          "answer": [
            1,
            3,
            2,
            0
          ],
          "estimated_seconds": 15,
          "explanation": "正确的顺序是：识别 -> 评估/衡量 -> 处理 -> 监控。",
          "options": [
            "风险监控",
            "风险识别",
            "风险处理",
            "风险评估/衡量"
          ],
          "question_id": "q_quiz_rm_cycle_recap_v1",
          "stem": "请将风险管理循环的四个步骤按正确顺序排列。"
        },
        {
          "answer": 2,
          "estimated_seconds": 10,
          "explanation": "识别风险后，需要评估和衡量其大小。",
          "options": [
            "风险监控",
            "风险处理",
            "风险评估/衡量",
            "风险报告"
          ],
          "question_id": "q_quiz_rm_cycle_recap_v2",
          "stem": "在风险管理循环中，在“识别风险”之后，下一步应该做什么？"
        }
      ]
    },
    {
      "concept_key": "risk_management_philosophy",
      "coverage_tags": [
        "risk_management_philosophy"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_rm_philosophy",
      "learning_goal": "学生能判断关于风险管理目标的陈述是否正确。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "true_false",
      "term_refs": [
        {
          "display": "风险管理",
          "en": "Risk Management"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 10,
          "explanation": "风险管理的目标不是消除风险，而是在了解风险的基础上做出明智的决策。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_rm_philosophy_v1",
          "stem": "风险管理的最终目标是完全消除所有投资风险。"
        },
        {
          "answer": 0,
          "estimated_seconds": 10,
          "explanation": "这是课程总结中强调的核心理念。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_rm_philosophy_v2",
          "stem": "根据课程总结，有效的风险管理意味着在了解风险的前提下做出更明智的决策。"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L6: Capital and Risk Management ... (see input)",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "L6 guided_story lesson map (see input)",
    "plain_text": "pipeline/1-plain/L6/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L6: Capital and Risk Management ... (see input)"
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "market_risk",
      "covered_by": [
        "qf_flash_market_risk",
        "qf_quiz_market_risk",
        "qf_long_risk_identification"
      ],
      "description": "市场风险：因市场价格变动导致损失的风险，包括股票、债券、货币、商品。"
    },
    {
      "coverage_tag": "interest_rate_risk",
      "covered_by": [
        "qf_flash_interest_rate_risk",
        "qf_quiz_interest_rate_risk",
        "qf_long_risk_identification"
      ],
      "description": "利率风险：因利率波动导致投资损失的风险，主要影响债券等固定收益证券。"
    },
    {
      "coverage_tag": "credit_risk",
      "covered_by": [
        "qf_flash_credit_risk",
        "qf_quiz_credit_risk",
        "qf_long_risk_identification"
      ],
      "description": "信用风险：借款方未能偿还贷款或履行合约义务而导致损失的风险。"
    },
    {
      "coverage_tag": "liquidity_risk",
      "covered_by": [
        "qf_flash_liquidity_risk",
        "qf_quiz_liquidity_risk",
        "qf_long_risk_identification"
      ],
      "description": "流动性风险：无法在不显著损失价值的情况下，快速将资产转换为现金的风险。"
    },
    {
      "coverage_tag": "risk_identification_overview",
      "covered_by": [
        "qf_flash_risk_types_overview",
        "qf_quiz_risk_types_overview"
      ],
      "description": "金融机构面临的八种主要风险类型概览。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "market_risk",
      "coverage_tags": [
        "market_risk"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_market_risk",
      "learning_goal": "学生能准确说出市场风险的定义，并给出一个具体例子。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "市场风险的核心定义和典型触发因素。",
      "term_refs": [
        {
          "display": "市场风险",
          "en": "Market Risk"
        }
      ],
      "variants": [
        {
          "back": "因市场价格（如股价、汇率、商品价格）变动导致损失的风险。例子：持有科技股，市场整体大跌导致股票价值下跌。",
          "estimated_seconds": 10,
          "explanation": "市场风险是投资中固有的风险，与公司自身表现无关。",
          "front": "什么是市场风险？请给出一个例子。",
          "question_id": "q_flash_market_risk_v1"
        },
        {
          "back": "股票、债券、货币和商品。",
          "estimated_seconds": 8,
          "explanation": "这些资产的价格都受市场整体波动影响。",
          "front": "市场风险主要影响哪些类型的资产？",
          "question_id": "q_flash_market_risk_v2"
        }
      ]
    },
    {
      "concept_key": "interest_rate_risk",
      "coverage_tags": [
        "interest_rate_risk"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_interest_rate_risk",
      "learning_goal": "学生能准确说出利率风险的定义，并解释利率与债券价格的关系。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "利率风险的定义和利率与债券价格的反向关系。",
      "term_refs": [
        {
          "display": "利率风险",
          "en": "Interest Rate Risk"
        }
      ],
      "variants": [
        {
          "back": "因利率波动导致投资损失的风险。主要影响债券等固定收益证券。",
          "estimated_seconds": 8,
          "explanation": "利率上升，旧债券吸引力下降，价格下跌。",
          "front": "什么是利率风险？它主要影响哪类资产？",
          "question_id": "q_flash_interest_rate_risk_v1"
        },
        {
          "back": "下降。因为新发行的债券提供更高收益，旧债券吸引力降低。",
          "estimated_seconds": 10,
          "explanation": "这是利率风险的核心机制。",
          "front": "当市场利率上升时，你手中固定利率债券的市场价值会如何变化？为什么？",
          "question_id": "q_flash_interest_rate_risk_v2"
        }
      ]
    },
    {
      "concept_key": "credit_risk",
      "coverage_tags": [
        "credit_risk"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_credit_risk",
      "learning_goal": "学生能准确说出信用风险的定义，并给出一个具体例子。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "信用风险的定义和典型场景。",
      "term_refs": [
        {
          "display": "信用风险",
          "en": "Credit Risk"
        }
      ],
      "variants": [
        {
          "back": "借款方未能偿还贷款或履行合约义务而导致损失的风险。例子：银行借钱给企业，企业违约不还。",
          "estimated_seconds": 10,
          "explanation": "信用风险是借贷和投资中的关键考量。",
          "front": "什么是信用风险？请给出一个例子。",
          "question_id": "q_flash_credit_risk_v1"
        },
        {
          "back": "贷款人（如银行）和债券/贷款投资者。",
          "estimated_seconds": 8,
          "explanation": "这些参与者面临借款人违约的风险。",
          "front": "信用风险主要影响哪些市场参与者？",
          "question_id": "q_flash_credit_risk_v2"
        }
      ]
    },
    {
      "concept_key": "liquidity_risk",
      "coverage_tags": [
        "liquidity_risk"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_liquidity_risk",
      "learning_goal": "学生能准确说出流动性风险的定义，并给出一个具体例子。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "流动性风险的定义和核心特征。",
      "term_refs": [
        {
          "display": "流动性风险",
          "en": "Liquidity Risk"
        }
      ],
      "variants": [
        {
          "back": "无法在不显著损失价值的情况下，快速将资产转换为现金的风险。例子：公司急需现金，但资产主要是房产，只能打折出售。",
          "estimated_seconds": 10,
          "explanation": "流动性风险关乎现金流管理。",
          "front": "什么是流动性风险？请给出一个例子。",
          "question_id": "q_flash_liquidity_risk_v1"
        },
        {
          "back": "无法快速变现，或变现时需接受大幅折价。",
          "estimated_seconds": 8,
          "explanation": "这可能导致无法满足短期债务或运营需求。",
          "front": "流动性风险的核心问题是什么？",
          "question_id": "q_flash_liquidity_risk_v2"
        }
      ]
    },
    {
      "concept_key": "risk_identification_overview",
      "coverage_tags": [
        "risk_identification_overview"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_risk_types_overview",
      "learning_goal": "学生能列举出金融机构面临的至少四种主要风险类型。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "金融机构面临的八种主要风险类型。",
      "term_refs": [
        {
          "display": "风险类型",
          "en": "Risk Types"
        }
      ],
      "variants": [
        {
          "back": "市场风险、利率风险、信用风险、流动性风险。",
          "estimated_seconds": 8,
          "explanation": "这是最核心的四种风险。",
          "front": "请列举金融机构面临的四种主要风险类型。",
          "question_id": "q_flash_risk_types_overview_v1"
        },
        {
          "back": "操作风险、法律风险、声誉风险、战略风险。",
          "estimated_seconds": 10,
          "explanation": "这些风险同样重要，可能来自内部流程、外部事件或决策失误。",
          "front": "除了市场、利率、信用、流动性风险外，金融机构还面临哪些其他风险？（说出至少两个）",
          "question_id": "q_flash_risk_types_overview_v2"
        }
      ]
    }
  ],
  "lesson_id": "L6",
  "longform_families": [
    {
      "concept_key": "risk_identification_overview",
      "coverage_tags": [
        "market_risk",
        "interest_rate_risk",
        "credit_risk",
        "liquidity_risk"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_risk_identification",
      "learning_goal": "学生能解释四种核心风险的区别，并针对给定场景进行风险识别和分类。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "风险识别",
          "en": "Risk Identification"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "四种风险的核心区别",
            "场景A的风险识别与理由",
            "场景B的风险识别与理由"
          ],
          "question_id": "q_long_risk_identification_v1",
          "reference_answer": [
            "市场风险是因市场价格（如股价、汇率）整体变动导致损失的风险。利率风险是因利率变动导致固定收益证券价值变化的风险。信用风险是交易对手方（如借款人）违约的风险。流动性风险是无法在不显著折价的情况下快速将资产变现的风险。",
            "场景A主要涉及信用风险。因为贷款给评级下调的公司，意味着该公司违约的可能性增加，银行面临无法收回贷款本息的风险。",
            "场景B同时涉及市场风险和流动性风险。市场暴跌导致基金持有的小盘股价格下跌，这是市场风险。同时，由于市场流动性枯竭，基金无法以合理价格快速卖出股票来满足赎回，这是流动性风险。"
          ],
          "rubric_points": [
            "准确区分四种风险的定义和触发因素",
            "正确识别场景A为信用风险，并解释原因（借款人信用恶化可能导致违约）",
            "正确识别场景B同时涉及市场风险和流动性风险，并解释原因（市场暴跌是市场风险，无法快速以合理价格卖出是流动性风险）"
          ],
          "stem": "请解释市场风险、利率风险、信用风险和流动性风险之间的核心区别。然后，针对以下两个场景，分别指出其主要涉及的风险类型，并简要说明理由。\n\n场景A：一家银行向一家评级下调的公司发放了大额贷款。\n场景B：一家共同基金持有大量小盘股，当市场暴跌时，它试图卖出股票来应对赎回，但发现很难找到买家，只能以远低于昨日收盘价的价格成交。"
        },
        {
          "estimated_seconds": 150,
          "prompt_blocks": [
            "识别每种投资对应的主要风险",
            "提出监控或缓解建议"
          ],
          "question_id": "q_long_risk_identification_v2",
          "reference_answer": [
            "1. 美国国债：主要风险是利率风险。建议：监控美联储利率政策预期，使用久期分析来评估利率变动对债券组合价值的影响。\n2. 科技公司股票：主要风险是市场风险。建议：设置止损点，并使用波动率指标（如历史波动率）来监控市场风险敞口。\n3. 初创企业贷款：主要风险是信用风险。建议：定期审查初创企业的财务状况和现金流，并要求其提供抵押品或担保。\n4. 商业地产项目：主要风险是流动性风险。建议：保持一定比例的现金或高流动性资产，并制定在需要时出售地产份额或寻求过桥贷款的预案。"
          ],
          "rubric_points": [
            "正确识别美国国债投资面临的主要风险是利率风险",
            "正确识别科技公司股票投资面临的主要风险是市场风险",
            "正确识别初创企业贷款面临的主要风险是信用风险",
            "正确识别商业地产项目面临的主要风险是流动性风险",
            "为每种风险提出一个合理且具体的监控或缓解建议"
          ],
          "stem": "假设你是一家小型投资公司的风险官。公司主要投资于：\n1. 美国国债（长期）\n2. 一家大型科技公司的股票\n3. 向一家初创企业提供的贷款\n4. 一个商业地产项目\n\n请分析公司当前面临的主要风险类型，并针对每种风险提出一个简要的监控或缓解建议。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "market_risk",
      "coverage_tags": [
        "market_risk"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_market_risk",
      "learning_goal": "学生能在具体情境中识别市场风险。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "市场风险",
          "en": "Market Risk"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "市场风险是因市场价格整体变动导致损失的风险。股市普遍下跌属于市场风险。",
          "options": [
            "信用风险",
            "利率风险",
            "市场风险",
            "流动性风险"
          ],
          "question_id": "q_quiz_market_risk_v1",
          "stem": "由于全球经济衰退的担忧，股市普遍下跌，导致你持有的股票组合价值缩水。这主要体现了哪种风险？"
        },
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "外汇风险是市场风险的一种，因汇率变动导致损失。",
          "options": [
            "市场风险（外汇风险）",
            "信用风险",
            "流动性风险",
            "操作风险"
          ],
          "question_id": "q_quiz_market_risk_v2",
          "stem": "一家公司持有大量外币计价的应收账款。如果该外币突然大幅贬值，公司可能面临损失。这主要属于哪种风险？"
        }
      ]
    },
    {
      "concept_key": "interest_rate_risk",
      "coverage_tags": [
        "interest_rate_risk"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_interest_rate_risk",
      "learning_goal": "学生能在具体情境中识别利率风险，并理解其对债券价格的影响。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "利率风险",
          "en": "Interest Rate Risk"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 25,
          "explanation": "市场利率上升，新发行债券提供更高收益，旧债券吸引力下降，因此价格下跌。这是利率风险。",
          "options": [
            "上升",
            "下降",
            "不变",
            "先升后降"
          ],
          "question_id": "q_quiz_interest_rate_risk_v1",
          "stem": "你持有一张面值1000元、票面利率3%的10年期债券。如果央行突然加息，市场利率上升到5%，你的债券市场价值最可能发生什么变化？"
        },
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "利率上升会导致其持有的债券价值下降，这是典型的利率风险。",
          "options": [
            "信用风险",
            "流动性风险",
            "利率风险",
            "操作风险"
          ],
          "question_id": "q_quiz_interest_rate_risk_v2",
          "stem": "一家银行持有大量长期固定利率债券。如果市场利率预期将上升，该银行面临的主要风险是什么？"
        }
      ]
    },
    {
      "concept_key": "credit_risk",
      "coverage_tags": [
        "credit_risk"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_credit_risk",
      "learning_goal": "学生能在具体情境中识别信用风险。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "信用风险",
          "en": "Credit Risk"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "信用风险是指借款方未能履行还款义务的风险。",
          "options": [
            "市场风险",
            "利率风险",
            "信用风险",
            "流动性风险"
          ],
          "question_id": "q_quiz_credit_risk_v1",
          "stem": "你购买了一家陷入财务困境的公司发行的债券。该公司最终未能支付利息和本金。你遭受的损失主要源于哪种风险？"
        },
        {
          "answer": 3,
          "estimated_seconds": 20,
          "explanation": "借款人违约导致银行无法收回贷款本金和利息，这是信用风险。",
          "options": [
            "流动性风险",
            "市场风险",
            "操作风险",
            "信用风险"
          ],
          "question_id": "q_quiz_credit_risk_v2",
          "stem": "一家银行向一个信用记录不佳的个人发放了贷款。该个人后来失业并停止还款。银行面临的主要风险是？"
        }
      ]
    },
    {
      "concept_key": "liquidity_risk",
      "coverage_tags": [
        "liquidity_risk"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_liquidity_risk",
      "learning_goal": "学生能在具体情境中识别流动性风险。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "流动性风险",
          "en": "Liquidity Risk"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 25,
          "explanation": "无法快速将资产变现而不损失价值，是流动性风险的核心。",
          "options": [
            "市场风险",
            "信用风险",
            "流动性风险",
            "操作风险"
          ],
          "question_id": "q_quiz_liquidity_risk_v1",
          "stem": "一家对冲基金持有大量非上市公司的股权。当投资者要求赎回资金时，基金无法迅速卖出这些股权来筹集现金，只能以极低的价格出售。这主要体现了哪种风险？"
        },
        {
          "answer": 3,
          "estimated_seconds": 20,
          "explanation": "急需现金时，资产无法快速以合理价格变现，是流动性风险。",
          "options": [
            "市场风险",
            "利率风险",
            "信用风险",
            "流动性风险"
          ],
          "question_id": "q_quiz_liquidity_risk_v2",
          "stem": "一家公司的大部分资产是房地产。它突然需要一笔现金来支付到期的供应商货款，但短期内找不到买家，只能以低于市场价20%的价格出售。这主要体现了哪种风险？"
        }
      ]
    },
    {
      "concept_key": "risk_identification_overview",
      "coverage_tags": [
        "risk_identification_overview"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_risk_types_overview",
      "learning_goal": "学生能将风险定义与正确的风险类型匹配。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "matching",
      "term_refs": [
        {
          "display": "风险类型",
          "en": "Risk Types"
        }
      ],
      "variants": [
        {
          "correct_mapping": {
            "借款方未能偿还贷款": "信用风险",
            "因利率波动导致债券价值损失": "利率风险",
            "因市场价格变动导致损失": "市场风险",
            "无法快速以合理价格变现资产": "流动性风险"
          },
          "estimated_seconds": 30,
          "explanation": "这些是四种核心金融风险的标准定义。",
          "left_items": [
            "因市场价格变动导致损失",
            "因利率波动导致债券价值损失",
            "借款方未能偿还贷款",
            "无法快速以合理价格变现资产"
          ],
          "question_id": "q_quiz_risk_types_overview_v1",
          "right_items": [
            "市场风险",
            "利率风险",
            "信用风险",
            "流动性风险"
          ],
          "stem": "请将左侧的风险描述与右侧正确的风险类型匹配。"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L6: Capital and Risk Management - Major Risk Types for a financial institution: Market Risk, Interest rate Risk, Credit Risk, Liquidity Risk, Operational Risk, Legal Risk, Reputational Risk, Strategic Risk",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "{\"lesson_id\":\"L6\",\"steps\":[{\"concept\":\"金融机构面临的主要风险类型\",\"sequence_id\":\"step2\"}]}",
    "plain_text": "pipeline/1-plain/L6/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L6: Capital and Risk Management - Risk Identification - Major Risk Types for a financial institution: 1. Market Risk (Equity Risk, FX Risk, Commodity Risk), 2. Interest rate Risk, 3. Credit Risk, 4. Liquidity Risk, 5. Operational Risk, 6. Legal Risk, 7. Reputational Risk, 8. Strategic Risk"
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "position_sizing",
      "covered_by": [
        "qf_flash_position_sizing",
        "qf_quiz_position_sizing"
      ],
      "description": "仓位管理的概念与目的"
    },
    {
      "coverage_tag": "fixed_size",
      "covered_by": [
        "qf_flash_fixed_size",
        "qf_quiz_fixed_size"
      ],
      "description": "固定规模法的定义、特点与局限"
    },
    {
      "coverage_tag": "balance_rescaling",
      "covered_by": [
        "qf_flash_balance_rescaling",
        "qf_quiz_balance_rescaling"
      ],
      "description": "余额重缩放法的定义、公式与特点"
    },
    {
      "coverage_tag": "dollar_risk_approach",
      "covered_by": [
        "qf_flash_dollar_risk_approach",
        "qf_quiz_dollar_risk_approach",
        "qf_long_dollar_risk_calc"
      ],
      "description": "美元风险法的定义、计算步骤与特点"
    },
    {
      "coverage_tag": "kelly_criterion",
      "covered_by": [
        "qf_flash_kelly_criterion",
        "qf_quiz_kelly_criterion",
        "qf_long_kelly_calc"
      ],
      "description": "凯利公式的定义、公式、变量含义与计算"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "position_sizing",
      "coverage_tags": [
        "position_sizing"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_position_sizing",
      "learning_goal": "学生能准确回忆仓位管理的核心问题与目的。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "仓位管理要解决的核心问题是什么？",
      "term_refs": [
        {
          "display": "仓位管理",
          "en": "Position Sizing"
        }
      ],
      "variants": [
        {
          "back": "每次交易该投入多少资金。",
          "estimated_seconds": 8,
          "explanation": "在知道风险大小后，仓位管理决定每笔交易的具体规模。",
          "front": "仓位管理要解决的核心问题是什么？",
          "question_id": "q_flash_position_sizing_v1"
        },
        {
          "back": "Position Sizing",
          "estimated_seconds": 5,
          "explanation": "Position Sizing 是决定每笔交易投入多少资金的方法。",
          "front": "仓位管理的英文术语是什么？",
          "question_id": "q_flash_position_sizing_v2"
        }
      ]
    },
    {
      "concept_key": "fixed_size",
      "coverage_tags": [
        "fixed_size"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_fixed_size",
      "learning_goal": "学生能准确回忆固定规模法的定义和主要缺点。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "固定规模法的核心特征和主要缺点是什么？",
      "term_refs": [
        {
          "display": "固定规模法",
          "en": "Fixed Size"
        }
      ],
      "variants": [
        {
          "back": "每次交易都使用相同的数量（如100股）。",
          "estimated_seconds": 8,
          "explanation": "这是最简单的仓位管理策略，不考虑账户余额变化。",
          "front": "固定规模法（Fixed Size）的核心特征是什么？",
          "question_id": "q_flash_fixed_size_v1"
        },
        {
          "back": "盈利时资金利用不足，亏损时风险暴露过高。",
          "estimated_seconds": 10,
          "explanation": "因为不随账户余额调整，所以不够灵活。",
          "front": "固定规模法的主要缺点是什么？",
          "question_id": "q_flash_fixed_size_v2"
        }
      ]
    },
    {
      "concept_key": "balance_rescaling",
      "coverage_tags": [
        "balance_rescaling"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_balance_rescaling",
      "learning_goal": "学生能准确回忆余额重缩放法的定义和调整公式。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "余额重缩放法如何调整交易规模？",
      "term_refs": [
        {
          "display": "余额重缩放法",
          "en": "Balance Rescaling"
        }
      ],
      "variants": [
        {
          "back": "根据当前账户余额与初始余额的比例来调整。",
          "estimated_seconds": 10,
          "explanation": "公式为 q_t = (B_t / B_0) * q_0，即按余额变化比例缩放。",
          "front": "余额重缩放法（Balance Rescaling）如何调整交易规模？",
          "question_id": "q_flash_balance_rescaling_v1"
        },
        {
          "back": "能动态调整仓位，保持相同的风险水平。",
          "estimated_seconds": 10,
          "explanation": "账户资金变多就多买，变少就少买，更灵活。",
          "front": "余额重缩放法相比固定规模法的主要优势是什么？",
          "question_id": "q_flash_balance_rescaling_v2"
        }
      ]
    },
    {
      "concept_key": "dollar_risk_approach",
      "coverage_tags": [
        "dollar_risk_approach"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_dollar_risk_approach",
      "learning_goal": "学生能准确回忆美元风险法的核心原则和所需输入。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "美元风险法的核心原则是什么？",
      "term_refs": [
        {
          "display": "美元风险法",
          "en": "Dollar Risk Approach"
        }
      ],
      "variants": [
        {
          "back": "每笔交易只承担账户总资金的一个固定百分比（如1%）作为风险。",
          "estimated_seconds": 10,
          "explanation": "这能防止连续亏损导致账户归零。",
          "front": "美元风险法（Dollar Risk Approach）的核心原则是什么？",
          "question_id": "q_flash_dollar_risk_approach_v1"
        },
        {
          "back": "账户余额、每笔交易的风险百分比、止损点数（或点值）。",
          "estimated_seconds": 12,
          "explanation": "例如：余额$10,000，风险1%，止损50点，则风险金额为$100，仓位规模为$100/(点值*50)。",
          "front": "使用美元风险法计算仓位规模需要哪三个输入？",
          "question_id": "q_flash_dollar_risk_approach_v2"
        }
      ]
    },
    {
      "concept_key": "kelly_criterion",
      "coverage_tags": [
        "kelly_criterion"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_kelly_criterion",
      "learning_goal": "学生能准确回忆凯利公式、变量含义及负值含义。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "凯利公式的表达式及各变量含义是什么？",
      "term_refs": [
        {
          "display": "凯利公式",
          "en": "Kelly Criterion"
        }
      ],
      "variants": [
        {
          "back": "f* = (bp - q) / b。f*是最优投注比例，b是赔率，p是胜率，q是败率(1-p)。",
          "estimated_seconds": 15,
          "explanation": "该公式用于最大化长期资金增长率。",
          "front": "写出凯利公式的表达式，并说明 f*、b、p、q 分别代表什么。",
          "question_id": "q_flash_kelly_criterion_v1"
        },
        {
          "back": "该交易不值得做，因为期望收益为负。",
          "estimated_seconds": 8,
          "explanation": "负的 f* 表示长期来看会亏损，不应参与。",
          "front": "如果凯利公式计算出的 f* 为负数，意味着什么？",
          "question_id": "q_flash_kelly_criterion_v2"
        }
      ]
    }
  ],
  "lesson_id": "L6",
  "longform_families": [
    {
      "concept_key": "dollar_risk_approach",
      "coverage_tags": [
        "dollar_risk_approach"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_dollar_risk_calc",
      "learning_goal": "学生能完整地应用美元风险法计算仓位规模。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "worked_example",
      "term_refs": [
        {
          "display": "美元风险法",
          "en": "Dollar Risk Approach"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "步骤1：计算每笔交易的最大风险金额。",
            "步骤2：计算每标准手在止损范围内的总风险金额。",
            "步骤3：计算合适的仓位规模。"
          ],
          "question_id": "q_long_dollar_risk_calc_v1",
          "reference_answer": [
            "步骤1：最大风险金额 = $20,000 * 1.5% = $300。",
            "步骤2：每标准手止损风险 = $10/点 * 40点 = $400。",
            "步骤3：仓位规模 = $300 / $400 = 0.75 标准手。"
          ],
          "rubric_points": [
            "正确计算最大风险金额 (1分)",
            "正确计算每标准手的止损风险 (1分)",
            "正确计算仓位规模 (1分)",
            "答案格式正确 (1分)"
          ],
          "stem": "你的交易账户余额为 $20,000。你决定每笔交易只承担账户余额的 1.5% 作为风险。你计划交易 EUR/USD，其标准合约大小为 100,000 单位，每点价值 $10。你的止损设置为 40 点。请计算你应该交易多少标准手（保留两位小数）。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "步骤1：计算每笔交易的最大风险金额。",
            "步骤2：计算每标准手在止损范围内的总风险金额。",
            "步骤3：计算合适的仓位规模。"
          ],
          "question_id": "q_long_dollar_risk_calc_v2",
          "reference_answer": [
            "步骤1：最大风险金额 = $50,000 * 2% = $1,000。",
            "步骤2：每标准手止损风险 = $1/点 * 100点 = $100。",
            "步骤3：仓位规模 = $1,000 / $100 = 10.00 标准手。"
          ],
          "rubric_points": [
            "正确计算最大风险金额 (1分)",
            "正确计算每标准手的止损风险 (1分)",
            "正确计算仓位规模 (1分)",
            "答案格式正确 (1分)"
          ],
          "stem": "你的交易账户余额为 $50,000。你决定每笔交易只承担账户余额的 2% 作为风险。你计划交易黄金（XAU/USD），其标准合约大小为 100 盎司，每点价值 $1。你的止损设置为 100 点。请计算你应该交易多少标准手（保留两位小数）。"
        }
      ]
    },
    {
      "concept_key": "kelly_criterion",
      "coverage_tags": [
        "kelly_criterion"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_kelly_calc",
      "learning_goal": "学生能应用凯利公式计算最优投注比例，并解释结果的含义。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "worked_example",
      "term_refs": [
        {
          "display": "凯利公式",
          "en": "Kelly Criterion"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "步骤1：计算赔率 b。",
            "步骤2：应用凯利公式计算 f*。",
            "步骤3：解释 f* 的含义。"
          ],
          "question_id": "q_long_kelly_calc_v1",
          "reference_answer": [
            "步骤1：赔率 b = 平均盈利 / 平均亏损 = 200 / 100 = 2。",
            "步骤2：f* = (b*p - q) / b = (2*0.55 - 0.45) / 2 = (1.1 - 0.45) / 2 = 0.65 / 2 = 0.325。",
            "步骤3：f* = 0.325 意味着，为了长期最大化资金增长率，每次交易应投入账户总资金的 32.5%。"
          ],
          "rubric_points": [
            "正确计算赔率 (1分)",
            "正确代入公式并计算 (1分)",
            "正确解释结果 (1分)",
            "表述清晰 (1分)"
          ],
          "stem": "一个交易策略的历史数据显示，胜率为 55%，平均盈利为 $200，平均亏损为 $100。请使用凯利公式计算最优投注比例 f*，并解释该结果的含义。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "步骤1：计算赔率 b。",
            "步骤2：应用凯利公式计算 f*。",
            "步骤3：解释 f* 的含义。"
          ],
          "question_id": "q_long_kelly_calc_v2",
          "reference_answer": [
            "步骤1：赔率 b = 平均盈利 / 平均亏损 = 300 / 100 = 3。",
            "步骤2：f* = (b*p - q) / b = (3*0.4 - 0.6) / 3 = (1.2 - 0.6) / 3 = 0.6 / 3 = 0.2。",
            "步骤3：f* = 0.2 意味着，为了长期最大化资金增长率，每次交易应投入账户总资金的 20%。"
          ],
          "rubric_points": [
            "正确计算赔率 (1分)",
            "正确代入公式并计算 (1分)",
            "正确解释结果 (1分)",
            "表述清晰 (1分)"
          ],
          "stem": "一个交易策略的历史数据显示，胜率为 40%，平均盈利为 $300，平均亏损为 $100。请使用凯利公式计算最优投注比例 f*，并解释该结果的含义。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "position_sizing",
      "coverage_tags": [
        "position_sizing"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_position_sizing",
      "learning_goal": "学生能在不同场景下识别仓位管理的核心问题。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "仓位管理",
          "en": "Position Sizing"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "仓位管理（Position Sizing）正是在衡量风险后，决定每次交易该投入多少资金。",
          "options": [
            "识别新的风险类型",
            "决定每笔交易的投入资金规模",
            "计算投资组合的预期收益",
            "选择交易标的"
          ],
          "question_id": "q_quiz_position_sizing_v1",
          "stem": "在风险管理流程中，完成风险衡量（如VaR）后，下一步通常需要考虑什么？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "仓位管理不是消除风险，而是通过控制每笔交易的资金规模来管理风险。",
          "options": [
            "消除所有投资风险",
            "最大化单次交易的利润",
            "在了解风险的基础上，控制每笔交易的风险暴露",
            "预测市场未来的走势"
          ],
          "question_id": "q_quiz_position_sizing_v2",
          "stem": "以下哪项最准确地描述了仓位管理的目的？"
        }
      ]
    },
    {
      "concept_key": "fixed_size",
      "coverage_tags": [
        "fixed_size"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_fixed_size",
      "learning_goal": "学生能辨析固定规模法的优缺点。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "固定规模法",
          "en": "Fixed Size"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 10,
          "explanation": "固定规模法的特征就是每次交易使用相同的数量。",
          "options": [
            "余额重缩放法",
            "美元风险法",
            "固定规模法",
            "凯利公式"
          ],
          "question_id": "q_quiz_fixed_size_v1",
          "stem": "一个交易员始终每次买入100股某股票，无论账户余额如何变化。他使用的是哪种仓位管理策略？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "因为不随账户余额变化，盈利时资金闲置，亏损时风险比例反而增大。",
          "options": [
            "计算过于复杂",
            "需要频繁调整仓位",
            "盈利时资金利用不足，亏损时风险过高",
            "无法用于股票交易"
          ],
          "question_id": "q_quiz_fixed_size_v2",
          "stem": "固定规模法的主要缺点是什么？"
        }
      ]
    },
    {
      "concept_key": "balance_rescaling",
      "coverage_tags": [
        "balance_rescaling"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_balance_rescaling",
      "learning_goal": "学生能理解余额重缩放法的调整逻辑。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "余额重缩放法",
          "en": "Balance Rescaling"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "新规模 = (15000/10000) * 100 = 150股。",
          "options": [
            "100股",
            "150股",
            "200股",
            "50股"
          ],
          "question_id": "q_quiz_balance_rescaling_v1",
          "stem": "某交易员初始资金$10,000，初始交易规模为100股。使用余额重缩放法，当账户资金增长到$15,000时，新的交易规模应为多少？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "余额重缩放法通过按比例调整，使风险水平与账户规模相匹配。",
          "options": [
            "考虑了交易的胜率",
            "考虑了赔率",
            "根据账户余额动态调整仓位，保持风险水平一致",
            "能完全避免亏损"
          ],
          "question_id": "q_quiz_balance_rescaling_v2",
          "stem": "余额重缩放法相比固定规模法，最主要的改进是什么？"
        }
      ]
    },
    {
      "concept_key": "dollar_risk_approach",
      "coverage_tags": [
        "dollar_risk_approach"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_dollar_risk_approach",
      "learning_goal": "学生能应用美元风险法进行简单计算和概念辨析。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "美元风险法",
          "en": "Dollar Risk Approach"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "最大亏损 = $50,000 * 2% = $1,000。",
          "options": [
            "$500",
            "$1,000",
            "$2,500",
            "$10,000"
          ],
          "question_id": "q_quiz_dollar_risk_approach_v1",
          "stem": "一个账户余额为$50,000，交易员决定每笔交易只承担2%的风险。那么每笔交易的最大亏损金额是多少？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "美元风险法需要明确的止损点来计算仓位，且不直接考虑策略的胜率。",
          "options": [
            "计算过于复杂，需要编程实现",
            "只适用于没有止损的交易策略",
            "只适用于有固定止损点的策略，且忽略了胜率",
            "无法控制单笔交易的最大亏损"
          ],
          "question_id": "q_quiz_dollar_risk_approach_v2",
          "stem": "美元风险法的主要局限性是什么？"
        }
      ]
    },
    {
      "concept_key": "kelly_criterion",
      "coverage_tags": [
        "kelly_criterion"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_kelly_criterion",
      "learning_goal": "学生能应用凯利公式进行简单计算，并理解其假设。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "凯利公式",
          "en": "Kelly Criterion"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "f* = (1*0.7 - 0.3) / 1 = 0.4。",
          "options": [
            "0.1",
            "0.2",
            "0.4",
            "0.7"
          ],
          "question_id": "q_quiz_kelly_criterion_v1",
          "stem": "一个交易策略的胜率为70%，赔率为1:1（即赢亏金额相等）。根据凯利公式，最优投注比例 f* 是多少？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "凯利公式假设每次交易的结果是独立的，且胜率和赔率是确定的，这在真实交易中很难满足。",
          "options": [
            "交易结果之间存在正相关性",
            "胜率和赔率是固定不变的",
            "市场总是有效的",
            "交易成本为零"
          ],
          "question_id": "q_quiz_kelly_criterion_v2",
          "stem": "凯利公式的一个关键假设是什么？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L6/plain.txt",
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
      "coverage_tag": "risk_definition",
      "covered_by": [
        "qf_flash_risk_definition",
        "qf_quiz_risk_definition"
      ],
      "description": "风险的定义：在金融中指投资回报的不确定性。"
    },
    {
      "coverage_tag": "upside_downside_risk",
      "covered_by": [
        "qf_flash_upside_downside",
        "qf_quiz_upside_downside"
      ],
      "description": "上行风险（赚钱的可能性）与下行风险（亏钱的可能性）的概念。"
    },
    {
      "coverage_tag": "risk_management_cycle",
      "covered_by": [
        "qf_flash_rm_cycle",
        "qf_quiz_rm_cycle",
        "qf_long_rm_cycle"
      ],
      "description": "风险管理循环的四个步骤：识别、评估、处理、监控。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "risk_definition",
      "coverage_tags": [
        "risk_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_risk_definition",
      "learning_goal": "学生能准确回忆金融中风险的核心定义。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "金融中风险的定义，即投资回报的不确定性。",
      "term_refs": [
        {
          "display": "风险",
          "en": "Risk"
        }
      ],
      "variants": [
        {
          "back": "投资回报的不确定性。",
          "estimated_seconds": 8,
          "explanation": "风险不是必然亏钱，而是结果不确定，既可能赚钱也可能亏钱。",
          "front": "在金融领域，“风险”的核心含义是什么？",
          "question_id": "q_flash_risk_definition_v1"
        },
        {
          "back": "金融风险就是投资回报的不确定性，与日常生活中的不确定性本质相同。",
          "estimated_seconds": 10,
          "explanation": "材料指出，风险就是不确定性。在金融中，这种不确定性特指投资回报。",
          "front": "金融中的风险与日常生活中的“不确定”有何关系？",
          "question_id": "q_flash_risk_definition_v2"
        }
      ]
    },
    {
      "concept_key": "upside_downside_risk",
      "coverage_tags": [
        "upside_downside_risk"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_upside_downside",
      "learning_goal": "学生能区分上行风险与下行风险。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "上行风险和下行风险分别指什么。",
      "term_refs": [
        {
          "display": "上行风险",
          "en": "Upside Risk"
        },
        {
          "display": "下行风险",
          "en": "Downside Risk"
        }
      ],
      "variants": [
        {
          "back": "赚钱的可能性。",
          "estimated_seconds": 6,
          "explanation": "上行风险是风险中好的一面，即获得收益的可能性。",
          "front": "金融中，“上行风险”指的是什么？",
          "question_id": "q_flash_upside_downside_v1"
        },
        {
          "back": "亏钱的可能性。",
          "estimated_seconds": 6,
          "explanation": "下行风险是风险中坏的一面，即遭受损失的可能性。",
          "front": "金融中，“下行风险”指的是什么？",
          "question_id": "q_flash_upside_downside_v2"
        }
      ]
    },
    {
      "concept_key": "risk_management_cycle",
      "coverage_tags": [
        "risk_management_cycle"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_rm_cycle",
      "learning_goal": "学生能回忆风险管理循环的四个步骤。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "风险管理循环的四个步骤及其顺序。",
      "term_refs": [
        {
          "display": "风险管理循环",
          "en": "Risk Management Cycle"
        }
      ],
      "variants": [
        {
          "back": "识别风险。",
          "estimated_seconds": 5,
          "explanation": "在管理风险之前，必须先识别出存在哪些风险。",
          "front": "风险管理循环的第一步是什么？",
          "question_id": "q_flash_rm_cycle_v1"
        },
        {
          "back": "评估/衡量风险。",
          "estimated_seconds": 5,
          "explanation": "识别风险后，需要量化风险的大小。",
          "front": "风险管理循环的第二步是什么？",
          "question_id": "q_flash_rm_cycle_v2"
        },
        {
          "back": "处理风险。",
          "estimated_seconds": 5,
          "explanation": "在评估风险后，采取相应的措施来应对风险。",
          "front": "风险管理循环的第三步是什么？",
          "question_id": "q_flash_rm_cycle_v3"
        },
        {
          "back": "监控风险。",
          "estimated_seconds": 5,
          "explanation": "风险是动态的，需要持续监控以确保管理措施有效。",
          "front": "风险管理循环的第四步是什么？",
          "question_id": "q_flash_rm_cycle_v4"
        }
      ]
    }
  ],
  "lesson_id": "L6",
  "longform_families": [
    {
      "concept_key": "risk_management_cycle",
      "coverage_tags": [
        "risk_management_cycle"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_rm_cycle",
      "learning_goal": "学生能用自己的语言解释风险管理循环的四个步骤及其相互关系。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "mechanism_trace",
      "term_refs": [
        {
          "display": "风险管理循环",
          "en": "Risk Management Cycle"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "列出四个步骤",
            "解释每个步骤的目的",
            "说明循环的含义"
          ],
          "question_id": "q_long_rm_cycle_v1",
          "reference_answer": [
            "风险管理循环包含四个步骤：",
            "1. 风险识别：找出可能面临的风险，例如市场风险、信用风险等。",
            "2. 风险评估/衡量：量化风险的大小，例如使用波动率、VaR等工具。",
            "3. 风险处理：根据评估结果采取措施，例如对冲、分散投资或设定止损。",
            "4. 风险监控：持续观察风险状况和管理措施的有效性。",
            "它是一个循环，因为风险环境是不断变化的。监控过程中可能发现新的风险或原有风险的变化，这需要重新进行识别和评估，从而形成一个持续改进的闭环。"
          ],
          "rubric_points": [
            "正确列出并解释识别、评估、处理、监控四个步骤。",
            "解释识别是发现风险，评估是量化风险，处理是采取措施，监控是持续观察。",
            "解释“循环”意味着风险是动态变化的，需要不断重复这个过程，例如监控后可能发现新风险，需要重新识别和评估。"
          ],
          "stem": "请解释风险管理循环的四个步骤，并说明为什么它是一个“循环”而不是一个一次性的过程。"
        },
        {
          "estimated_seconds": 150,
          "prompt_blocks": [
            "识别当前面临的风险",
            "评估该风险的大小",
            "提出处理方案",
            "说明如何监控"
          ],
          "question_id": "q_long_rm_cycle_v2",
          "reference_answer": [
            "1. 风险识别：当前面临的主要风险是市场风险，即由于市场整体波动加剧导致投资组合价值下跌的风险。",
            "2. 风险评估：我会计算投资组合的历史波动率（标准差）和95% VaR，以量化在正常市场条件下一天内可能的最大损失。",
            "3. 风险处理：根据评估结果，我可能会采取以下措施：a) 减少高风险资产的仓位；b) 买入指数看跌期权进行对冲；c) 将部分资金转移到更稳定的资产如国债。",
            "4. 风险监控：我会建立一个监控系统，每日跟踪投资组合的波动率和VaR值，并设定预警线。一旦指标超过阈值，我会重新评估并调整处理方案。"
          ],
          "rubric_points": [
            "识别出市场波动加剧带来的市场风险。",
            "评估风险：可以使用历史波动率或VaR来量化潜在损失。",
            "处理方案：例如降低仓位、买入看跌期权对冲、或分散投资到不同资产。",
            "监控：每日或每周检查投资组合的波动率和VaR值，并根据市场变化调整策略。"
          ],
          "stem": "假设你是一个小型投资基金的经理，你发现最近市场波动加剧。请结合风险管理循环的四个步骤，描述你将如何应对这一情况。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "risk_definition",
      "coverage_tags": [
        "risk_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_risk_definition",
      "learning_goal": "学生能在多个选项中准确识别金融风险的定义。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "风险",
          "en": "Risk"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "风险的核心是“不确定性”，它既包含赚钱的可能，也包含亏钱的可能。",
          "options": [
            "一定会亏钱",
            "投资回报的不确定性",
            "股票价格下跌",
            "市场波动"
          ],
          "question_id": "q_quiz_risk_definition_v1",
          "stem": "根据课程内容，金融中的“风险”最准确的定义是什么？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "风险是投资回报的不确定性，它同时包含了赚钱（上行风险）和亏钱（下行风险）的可能性。",
          "options": [
            "风险就是可能亏钱",
            "风险就是可能赚钱",
            "风险是投资结果的不确定性",
            "风险是市场价格的波动"
          ],
          "question_id": "q_quiz_risk_definition_v2",
          "stem": "以下哪个说法最能体现金融风险的本质？"
        }
      ]
    },
    {
      "concept_key": "upside_downside_risk",
      "coverage_tags": [
        "upside_downside_risk"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_upside_downside",
      "learning_goal": "学生能区分上行风险和下行风险。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "上行风险",
          "en": "Upside Risk"
        },
        {
          "display": "下行风险",
          "en": "Downside Risk"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "上行风险指的是赚钱的可能性。股价上涨带来收益，正是上行风险的体现。",
          "options": [
            "下行风险",
            "上行风险",
            "市场风险",
            "流动性风险"
          ],
          "question_id": "q_quiz_upside_downside_v1",
          "stem": "一位投资者买入股票后，股价上涨，他获得了收益。这体现了风险的哪个方面？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "这句话通常警示投资者可能亏钱，强调的是下行风险，即亏损的可能性。",
          "options": [
            "上行风险",
            "下行风险",
            "系统性风险",
            "非系统性风险"
          ],
          "question_id": "q_quiz_upside_downside_v2",
          "stem": "“投资有风险，入市需谨慎”这句话中，通常强调的是哪种风险？"
        }
      ]
    },
    {
      "concept_key": "risk_management_cycle",
      "coverage_tags": [
        "risk_management_cycle"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_rm_cycle",
      "learning_goal": "学生能正确排列风险管理循环的四个步骤。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "ordering",
      "term_refs": [
        {
          "display": "风险管理循环",
          "en": "Risk Management Cycle"
        }
      ],
      "variants": [
        {
          "answer": [
            2,
            3,
            0,
            1
          ],
          "estimated_seconds": 20,
          "explanation": "正确的顺序是：1. 识别风险，2. 评估风险，3. 处理风险，4. 监控风险。这是一个持续循环的过程。",
          "options": [
            "A. 处理风险",
            "B. 监控风险",
            "C. 识别风险",
            "D. 评估风险"
          ],
          "question_id": "q_quiz_rm_cycle_v1",
          "stem": "请将风险管理循环的四个步骤按正确顺序排列。"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "在识别出风险后，下一步是评估或衡量风险的大小，以便决定如何应对。",
          "options": [
            "处理风险",
            "监控风险",
            "评估/衡量风险",
            "忽略风险"
          ],
          "question_id": "q_quiz_rm_cycle_v2",
          "stem": "一家银行首先需要识别出可能面临的信用风险，然后才能进行下一步。请问，识别风险之后，紧接着应该做什么？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L6: Capital and Risk Management, Agenda: Introduction to the risk management cycle, Risk identification, Risk measurement, Volatility modelling including ARCH, GARCH, Value-at-Risk (VaR), Stress testing, Scenario analysis, Position sizing strategy, Fixed Size, Balance Rescaling, Dollar Risk Approach, Kelly Criterion. What is Risk? Risk simply refers to uncertainty. In finance, risk refers to the uncertainty of the investment return. Up-side risk: the possibility of making money. Down-side risk: the possibility of losing money. Risk Management Cycle: 1. Risk Identification, 2. Risk Assessment / Measurement, 3. Risk Treatment, 4. Risk Monitoring.",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "{\n  \"lesson_id\": \"L6\",\n  \"mode\": \"guided_story\",\n  \"steps\": [\n    {\n      \"concept\": \"风险是什么？为什么需要管理风险？\",\n      \"file\": \"research/pipeline/3-guided_story/L6/step1/step.json\",\n      \"sequence_id\": \"step1\"\n    },\n    {\n      \"concept\": \"金融机构面临的主要风险类型\",\n      \"file\": \"research/pipeline/3-guided_story/L6/step2/step.json\",\n      \"sequence_id\": \"step2\"\n    },\n    {\n      \"concept\": \"衡量风险：从标准差到波动率模型\",\n      \"file\": \"research/pipeline/3-guided_story/L6/step3/step.json\",\n      \"sequence_id\": \"step3\"\n    },\n    {\n      \"concept\": \"ARCH 与 GARCH：给波动率建模\",\n      \"file\": \"research/pipeline/3-guided_story/L6/step4/step.json\",\n      \"sequence_id\": \"step4\"\n    },\n    {\n      \"concept\": \"在险价值（VaR）：一个数字告诉你最大可能亏损\",\n      \"file\": \"research/pipeline/3-guided_story/L6/step5/step.json\",\n      \"sequence_id\": \"step5\"\n    },\n    {\n      \"concept\": \"压力测试与情景分析：为最坏情况做准备\",\n      \"file\": \"research/pipeline/3-guided_story/L6/step6/step.json\",\n      \"sequence_id\": \"step6\"\n    },\n    {\n      \"concept\": \"仓位管理：每次交易该下多大注？\",\n      \"file\": \"research/pipeline/3-guided_story/L6/step7/step.json\",\n      \"sequence_id\": \"step7\"\n    },\n    {\n      \"concept\": \"凯利公式的局限与总结\",\n      \"file\": \"research/pipeline/3-guided_story/L6/step8/step.json\",\n      \"sequence_id\": \"step8\"\n    }\n  ]\n}",
    "plain_text": "pipeline/1-plain/L6/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L6: Capital and Risk Management. Agenda: Introduction to the risk management cycle, Risk identification, Risk measurement, Volatility modelling including ARCH, GARCH, Value-at-Risk (VaR), Stress testing, Scenario analysis, Position sizing strategy, Fixed Size, Balance Rescaling, Dollar Risk Approach, Kelly Criterion. What is Risk? Risk simply refers to uncertainty. In finance, risk refers to the uncertainty of the investment return. Up-side risk: the possibility of making money. Down-side risk: the possibility of losing money. Risk Management Cycle: 1. Risk Identification, 2. Risk Assessment / Measurement, 3. Risk Treatment, 4. Risk Monitoring. Risk Identification. Major Risk Types for a financial institution: 1. Market Risk, Equity Risk, FX Risk, Commodity Risk, 2. Interest rate Risk, 3. Credit Risk, 4. Liquidity Risk, 5. Operational Risk, 6. Legal Risk, 7. Reputational Risk, 8. Strategic Risk. Investment related risks. Market Risk, Interest Risk, Credit Risk, Liquidity Risk."
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "stress_testing",
      "covered_by": [
        "qf_flash_stress_testing",
        "qf_quiz_stress_testing",
        "qf_long_stress_testing"
      ],
      "description": "压力测试：评估投资组合在极端市场条件下的潜在损失。"
    },
    {
      "coverage_tag": "scenario_analysis",
      "covered_by": [
        "qf_flash_scenario_analysis",
        "qf_quiz_scenario_analysis",
        "qf_long_scenario_analysis"
      ],
      "description": "情景分析：通过构建各种极端但可能发生的情景来评估投资组合的风险。"
    },
    {
      "coverage_tag": "var_limitation",
      "covered_by": [
        "qf_flash_var_limitation",
        "qf_quiz_var_limitation"
      ],
      "description": "VaR 的局限性：基于正常市场情况，无法覆盖极端事件。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "stress_testing",
      "coverage_tags": [
        "stress_testing"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_stress_testing",
      "learning_goal": "学生能准确回忆压力测试的定义和核心目的。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "压力测试的定义和它与 VaR 的关键区别。",
      "term_refs": [
        {
          "display": "压力测试",
          "en": "Stress Testing"
        }
      ],
      "variants": [
        {
          "back": "评估投资组合在极端市场条件（如金融危机重演）下的潜在损失。",
          "estimated_seconds": 8,
          "explanation": "压力测试关注的是 VaR 无法覆盖的极端但可能发生的事件。",
          "front": "压力测试（Stress Testing）的核心目的是什么？",
          "question_id": "q_flash_stress_testing_v1"
        },
        {
          "back": "VaR 基于正常市场情况，压力测试基于极端市场条件。",
          "estimated_seconds": 10,
          "explanation": "VaR 回答“正常情况下最多亏多少”，压力测试回答“如果危机重演会怎样”。",
          "front": "压力测试与 VaR 在评估风险时的主要区别是什么？",
          "question_id": "q_flash_stress_testing_v2"
        }
      ]
    },
    {
      "concept_key": "scenario_analysis",
      "coverage_tags": [
        "scenario_analysis"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_scenario_analysis",
      "learning_goal": "学生能准确回忆情景分析的定义和典型例子。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "情景分析的定义和它与压力测试的细微差别。",
      "term_refs": [
        {
          "display": "情景分析",
          "en": "Scenario Analysis"
        }
      ],
      "variants": [
        {
          "back": "构建各种“如果……会怎样”的极端但可能发生的情景，来评估投资组合的风险。",
          "estimated_seconds": 8,
          "explanation": "它不局限于历史事件，可以包含假设性情景。",
          "front": "情景分析（Scenario Analysis）的核心方法是什么？",
          "question_id": "q_flash_scenario_analysis_v1"
        },
        {
          "back": "例如：美联储突然加息到 50%，或港股与美元脱钩。",
          "estimated_seconds": 8,
          "explanation": "这些情景概率小但后果严重，情景分析用于评估其影响。",
          "front": "给出一个情景分析中可能使用的假设性情景例子。",
          "question_id": "q_flash_scenario_analysis_v2"
        }
      ]
    },
    {
      "concept_key": "var_limitation",
      "coverage_tags": [
        "var_limitation"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_var_limitation",
      "learning_goal": "学生能准确回忆 VaR 的一个关键局限性。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "VaR 无法覆盖极端事件这一核心局限。",
      "term_refs": [
        {
          "display": "VaR 的局限性",
          "en": "Limitation of VaR"
        }
      ],
      "variants": [
        {
          "back": "它基于“正常”市场情况，无法覆盖金融危机等极端事件。",
          "estimated_seconds": 8,
          "explanation": "VaR 给出的是正常市场下的最大可能损失，但极端事件可能造成远超 VaR 的损失。",
          "front": "VaR 的一个主要局限性是什么？",
          "question_id": "q_flash_var_limitation_v1"
        },
        {
          "back": "因为 VaR 无法捕捉极端但可能发生的市场事件（如金融危机）带来的风险。",
          "estimated_seconds": 10,
          "explanation": "压力测试和情景分析正是为了弥补 VaR 的这一不足。",
          "front": "为什么仅靠 VaR 进行风险管理是不够的？",
          "question_id": "q_flash_var_limitation_v2"
        }
      ]
    }
  ],
  "lesson_id": "L6",
  "longform_families": [
    {
      "concept_key": "stress_testing",
      "coverage_tags": [
        "stress_testing",
        "var_limitation"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_stress_testing",
      "learning_goal": "学生能解释压力测试与 VaR 的区别，并说明为什么两者需要结合使用。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "compare_and_contrast",
      "term_refs": [
        {
          "display": "压力测试",
          "en": "Stress Testing"
        },
        {
          "display": "在险价值",
          "en": "Value at Risk (VaR)"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "分别定义 VaR 和压力测试",
            "指出 VaR 的局限性",
            "说明压力测试如何弥补 VaR 的不足",
            "总结两者结合使用的必要性"
          ],
          "question_id": "q_long_stress_testing_v1",
          "reference_answer": [
            "VaR（在险价值）用于回答“在正常市场条件下，给定置信水平（如95%），明天最多可能亏多少钱”。它基于历史数据或统计分布，适用于日常风险管理。",
            "压力测试则用于回答“如果2008年金融危机重演，我的投资组合会怎样”。它模拟极端但可能发生的市场条件，评估投资组合在压力情景下的损失。",
            "VaR 的主要局限性在于它假设市场行为符合历史模式，无法捕捉从未发生或极少发生的极端事件。这些事件虽然概率低，但一旦发生可能造成毁灭性损失。",
            "压力测试通过重现历史危机事件（如亚洲金融风暴、全球金融危机）来评估投资组合的脆弱性，帮助金融机构提前做好准备。",
            "因此，VaR 和压力测试是互补的：VaR 提供日常风险监控，压力测试提供极端风险预警。两者结合使用，才能构建更全面的风险管理体系。"
          ],
          "rubric_points": [
            "正确解释 VaR 用于正常市场情况下的最大可能损失",
            "正确解释压力测试用于评估极端市场条件下的损失",
            "指出 VaR 无法覆盖极端事件",
            "说明压力测试通过模拟危机事件来评估尾部风险",
            "总结两者互补，构成完整的风险管理框架"
          ],
          "stem": "请解释压力测试（Stress Testing）与在险价值（VaR）在风险管理中的不同作用，并说明为什么金融机构需要同时使用这两种方法。"
        },
        {
          "estimated_seconds": 150,
          "prompt_blocks": [
            "描述你的投资组合构成",
            "选择一个合适的压力测试场景",
            "解释为什么 VaR 不足以评估该场景的风险",
            "说明压力测试可能揭示的风险"
          ],
          "question_id": "q_long_stress_testing_v2",
          "reference_answer": [
            "我的投资组合由60%的美国股票（如标普500指数基金）和40%的美国国债组成。",
            "我会选择2008年全球金融危机作为压力测试场景。当时标普500指数下跌约38%，而国债因避险需求上涨。",
            "仅靠 VaR 是不够的，因为 VaR 基于过去几年的正常市场数据，无法反映金融危机时市场同时大幅下跌、流动性枯竭等极端情况。VaR 可能会低估尾部风险。",
            "压力测试可以揭示：在金融危机重演时，我的股票部分可能损失约38%，而债券部分可能上涨约10%，整体组合可能仍面临较大亏损。此外，压力测试还能暴露流动性风险——在危机时可能无法以合理价格卖出资产。",
            "通过压力测试，我可以提前制定应对措施，如降低股票仓位、增加现金储备或使用期权对冲，从而更好地管理极端风险。"
          ],
          "rubric_points": [
            "合理描述投资组合（如60%股票，40%债券）",
            "选择一个相关的历史危机场景（如2008年全球金融危机）",
            "指出 VaR 基于正常市场假设，无法反映极端事件",
            "说明压力测试可以模拟市场同时暴跌、信用利差飙升等极端情况",
            "总结压力测试能帮助识别 VaR 无法捕捉的尾部风险"
          ],
          "stem": "假设你是一家投资银行的风险经理，你的投资组合主要持有美国股票和债券。请设计一个压力测试方案，并解释为什么仅靠 VaR 是不够的。"
        }
      ]
    },
    {
      "concept_key": "scenario_analysis",
      "coverage_tags": [
        "scenario_analysis"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_scenario_analysis",
      "learning_goal": "学生能解释情景分析的方法及其在风险管理中的价值。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "情景分析",
          "en": "Scenario Analysis"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "定义情景分析",
            "说明它与压力测试的区别",
            "给出一个假设性情景例子",
            "解释该情景如何帮助评估风险"
          ],
          "question_id": "q_long_scenario_analysis_v1",
          "reference_answer": [
            "情景分析是一种风险管理方法，通过构建各种极端但可能发生的情景来评估投资组合的风险。它不局限于历史事件，可以包含假设性情景。",
            "与压力测试不同，压力测试通常基于过去发生的危机事件（如2008年金融危机），而情景分析可以探索从未发生过的“如果……会怎样”的场景。",
            "例如，假设美联储突然将基准利率从当前水平加息到50%。这是一个极端但理论上可能的情景。",
            "通过情景分析，我可以评估我的债券投资组合在这种极端加息下的损失。由于债券价格与利率反向变动，加息50%将导致债券价格暴跌，可能造成巨大亏损。同时，股票市场也可能因利率飙升而大幅下跌。",
            "情景分析的价值在于，它帮助我识别 VaR 和压力测试可能无法覆盖的、但一旦发生后果严重的风险，从而提前制定应对策略，如减少债券久期、增加浮动利率债券配置或使用利率衍生品对冲。"
          ],
          "rubric_points": [
            "正确解释情景分析是构建极端但可能发生的情景来评估风险",
            "指出情景分析可以包含假设性事件，而压力测试通常基于历史事件",
            "给出一个合理的假设性情景（如美联储加息到50%）",
            "说明该情景对投资组合的潜在影响",
            "总结情景分析的价值在于发现 VaR 和压力测试可能遗漏的风险"
          ],
          "stem": "请解释情景分析（Scenario Analysis）在风险管理中的价值，并给出一个具体的假设性情景例子，说明如何用它来评估风险。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "分别描述两种方法的场景构建方式",
            "指出情景分析可以包含假设性事件",
            "说明情景分析的优势",
            "给出一个情景分析优于压力测试的例子"
          ],
          "question_id": "q_long_scenario_analysis_v2",
          "reference_answer": [
            "压力测试的场景通常基于过去发生的极端事件，如1997年亚洲金融风暴、2000年互联网泡沫、2008年全球金融危机等。它假设这些事件重演，评估当前投资组合的损失。",
            "情景分析的场景构建更加灵活，可以包含从未发生过的假设性事件。例如，它可以构建“如果港股与美元脱钩”或“如果美国主权信用评级被下调”等场景。",
            "情景分析的优势在于，它能够探索历史上从未发生过、但理论上可能发生的风险。这些风险可能被压力测试遗漏，因为压力测试只关注过去。",
            "例如，对于持有大量港币资产的投资者，“港股与美元脱钩”是一个重要的风险，但历史上从未发生过。压力测试无法覆盖这一场景，而情景分析可以。",
            "因此，情景分析在应对未知风险、黑天鹅事件方面具有独特优势，能够帮助投资者更全面地识别和管理风险。"
          ],
          "rubric_points": [
            "正确描述压力测试使用历史危机事件作为场景",
            "正确描述情景分析可以构建假设性事件",
            "指出情景分析能探索从未发生过的风险",
            "给出一个合理的例子（如港股与美元脱钩）",
            "总结情景分析在应对未知风险方面的优势"
          ],
          "stem": "请比较情景分析（Scenario Analysis）和压力测试（Stress Testing）在构建风险场景时的不同方法，并说明为什么情景分析在某些情况下更有优势。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "stress_testing",
      "coverage_tags": [
        "stress_testing"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_stress_testing",
      "learning_goal": "学生能在测验情境下辨析压力测试的核心特征。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "压力测试",
          "en": "Stress Testing"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "压力测试的核心是评估极端市场条件下的损失，通常基于过去发生的危机事件。",
          "options": [
            "计算投资组合在正常市场条件下的最大可能损失",
            "评估投资组合在极端历史事件（如2008年金融危机）重演时的潜在损失",
            "预测未来市场价格的走势",
            "优化投资组合的资产配置比例"
          ],
          "question_id": "q_quiz_stress_testing_v1",
          "stem": "以下哪项最准确地描述了压力测试（Stress Testing）在风险管理中的作用？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "压力测试使用过去发生的极端事件作为场景，来评估当前投资组合的脆弱性。",
          "options": [
            "假设未来一年市场波动率保持在当前水平",
            "假设2008年全球金融危机再次发生",
            "假设股票市场每天上涨0.5%",
            "假设通货膨胀率保持在2%不变"
          ],
          "question_id": "q_quiz_stress_testing_v2",
          "stem": "一家银行使用压力测试来评估其投资组合。以下哪个最可能是压力测试的场景？"
        }
      ]
    },
    {
      "concept_key": "scenario_analysis",
      "coverage_tags": [
        "scenario_analysis"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_scenario_analysis",
      "learning_goal": "学生能在测验情境下辨析情景分析的核心特征。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "情景分析",
          "en": "Scenario Analysis"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 25,
          "explanation": "情景分析可以构建假设性场景（如美联储加息到50%），而压力测试通常基于过去发生的危机事件。",
          "options": [
            "情景分析只使用历史数据，压力测试使用假设数据",
            "情景分析可以包含从未发生过的假设性事件，而压力测试通常基于历史事件",
            "情景分析只适用于债券，压力测试只适用于股票",
            "情景分析的结果更精确，压力测试的结果更模糊"
          ],
          "question_id": "q_quiz_scenario_analysis_v1",
          "stem": "情景分析（Scenario Analysis）与压力测试（Stress Testing）的主要区别是什么？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "情景分析通过构建“如果……会怎样”的假设性场景来评估风险，港股与美元脱钩就是一个典型的假设性情景。",
          "options": [
            "计算过去30天的平均收益率",
            "评估如果港股与美元脱钩，投资组合会受多大影响",
            "预测明天股票价格的涨跌",
            "计算投资组合的夏普比率"
          ],
          "question_id": "q_quiz_scenario_analysis_v2",
          "stem": "以下哪个是情景分析（Scenario Analysis）的典型应用？"
        }
      ]
    },
    {
      "concept_key": "var_limitation",
      "coverage_tags": [
        "var_limitation"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_var_limitation",
      "learning_goal": "学生能判断 VaR 局限性的相关陈述是否正确。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "true_false",
      "term_refs": [
        {
          "display": "VaR 的局限性",
          "en": "Limitation of VaR"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "VaR 基于正常市场情况，无法覆盖极端事件。压力测试和情景分析用于弥补这一不足。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_var_limitation_v1",
          "stem": "VaR 能够完全覆盖金融危机等极端事件带来的风险。"
        },
        {
          "answer": 0,
          "estimated_seconds": 15,
          "explanation": "VaR 关注正常市场，压力测试和情景分析关注极端但可能发生的事件，三者互补。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_var_limitation_v2",
          "stem": "压力测试和情景分析是为了弥补 VaR 无法覆盖极端事件的局限性而设计的。"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L6/plain.txt",
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
      "coverage_tag": "var_definition",
      "covered_by": [
        "qf_flash_var_def",
        "qf_quiz_var_interpret"
      ],
      "description": "在险价值（VaR）的定义：在给定置信水平和时间范围内，资产可能的最大损失。"
    },
    {
      "coverage_tag": "var_interpretation",
      "covered_by": [
        "qf_flash_var_def",
        "qf_quiz_var_interpret"
      ],
      "description": "VaR 的解读：例如 95% VaR 为 100 万，意味着有 95% 的概率亏损不超过 100 万。"
    },
    {
      "coverage_tag": "parametric_var",
      "covered_by": [
        "qf_flash_var_methods",
        "qf_quiz_var_methods"
      ],
      "description": "参数法 VaR：假设收益率服从正态分布来计算 VaR。"
    },
    {
      "coverage_tag": "historical_var",
      "covered_by": [
        "qf_flash_var_methods",
        "qf_quiz_var_methods",
        "qf_long_var_calc"
      ],
      "description": "历史模拟法 VaR：基于过去实际收益率数据，通过排序来估算 VaR。"
    },
    {
      "coverage_tag": "var_calculation",
      "covered_by": [
        "qf_long_var_calc"
      ],
      "description": "VaR 的计算：能够根据历史数据计算历史模拟法 VaR。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "var_definition",
      "coverage_tags": [
        "var_definition",
        "var_interpretation"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_var_def",
      "learning_goal": "学生能准确说出 VaR 的定义，并能解释一个具体的 VaR 陈述的含义。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "VaR 的核心定义和解读方式。",
      "term_refs": [
        {
          "display": "在险价值",
          "en": "Value at Risk (VaR)"
        }
      ],
      "variants": [
        {
          "back": "在给定置信水平和时间范围内，资产可能的最大损失。",
          "estimated_seconds": 8,
          "explanation": "VaR 用一个数字概括了在正常市场条件下，特定时间段内可能发生的最大损失。",
          "front": "在险价值（VaR）的定义是什么？",
          "question_id": "q_flash_var_def_v1"
        },
        {
          "back": "有 95% 的概率，明天的亏损不会超过 100 万。",
          "estimated_seconds": 10,
          "explanation": "这意味着在 100 天里，大约有 95 天的亏损会少于或等于 100 万，而有 5 天的亏损会超过 100 万。",
          "front": "如何解读“95% VaR 是 100 万”这句话？",
          "question_id": "q_flash_var_def_v2"
        }
      ]
    },
    {
      "concept_key": "var_methods",
      "coverage_tags": [
        "parametric_var",
        "historical_var"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_var_methods",
      "learning_goal": "学生能区分参数法和历史模拟法这两种 VaR 计算方法的核心思想。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "两种 VaR 计算方法的核心假设或操作步骤。",
      "term_refs": [
        {
          "display": "参数法 VaR",
          "en": "Parametric VaR"
        },
        {
          "display": "历史模拟法 VaR",
          "en": "Historical VaR"
        }
      ],
      "variants": [
        {
          "back": "假设收益率符合正态分布。",
          "estimated_seconds": 8,
          "explanation": "参数法基于收益率服从某种已知分布（通常是正态分布）的假设，利用均值和标准差等参数直接计算 VaR。",
          "front": "参数法 VaR 的核心假设是什么？",
          "question_id": "q_flash_var_methods_v1"
        },
        {
          "back": "将过去 N 天的收益率从低到高排序，取第 (1-置信度) 分位点的收益率。",
          "estimated_seconds": 12,
          "explanation": "例如，计算 95% VaR，就取排序后第 5% 最差的收益率。这种方法不依赖任何分布假设。",
          "front": "历史模拟法 VaR 的计算步骤是什么？",
          "question_id": "q_flash_var_methods_v2"
        }
      ]
    }
  ],
  "lesson_id": "L6",
  "longform_families": [
    {
      "concept_key": "var_calculation",
      "coverage_tags": [
        "historical_var",
        "var_calculation"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_var_calc",
      "learning_goal": "学生能根据给定的历史收益率数据，正确计算历史模拟法 VaR。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "worked_example",
      "term_refs": [
        {
          "display": "历史模拟法 VaR",
          "en": "Historical VaR"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "步骤 1：将收益率排序。",
            "步骤 2：确定分位点。",
            "步骤 3：找出对应的 VaR 值。"
          ],
          "question_id": "q_long_var_calc_v1",
          "reference_answer": [
            "步骤 1：将收益率从小到大排序：-2.2%, -1.8%, -1.0%, -0.5%, -0.1%, 0.2%, 0.8%, 1.5%, 2.5%, 3.0%",
            "步骤 2：90% VaR 对应第 (1-0.9) = 10% 分位点。共有 10 个数据点，第 10% 分位点对应第 1 个数据（10 * 10% = 1）。",
            "步骤 3：排序后的第 1 个收益率是 -2.2%。",
            "最终结果：90% 日 VaR 为 -2.2%（或亏损 2.2%）。"
          ],
          "rubric_points": [
            "正确排序收益率（1分）",
            "正确识别 90% VaR 对应第 10% 分位点（1分）",
            "正确找出第 10% 分位点的收益率值（1分）",
            "最终答案正确（1分）"
          ],
          "stem": "某资产过去 10 个交易日的日收益率如下（单位：%）：\n2.5, -1.0, 0.8, -2.2, 1.5, -0.5, 3.0, -1.8, 0.2, -0.1\n请使用历史模拟法计算该资产的 90% 日 VaR。请写出计算步骤和最终结果。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "步骤 1：将收益率排序。",
            "步骤 2：确定分位点。",
            "步骤 3：找出对应的 VaR 值。"
          ],
          "question_id": "q_long_var_calc_v2",
          "reference_answer": [
            "步骤 1：将收益率从小到大排序：-2.0%, -1.5%, -1.3%, -1.1%, -0.9%, -0.8%, -0.6%, -0.4%, -0.3%, -0.2%, 0.1%, 0.3%, 0.5%, 0.9%, 1.0%, 1.2%, 1.5%, 1.8%, 2.0%, 2.5%",
            "步骤 2：95% VaR 对应第 (1-0.95) = 5% 分位点。共有 20 个数据点，第 5% 分位点对应第 1 个数据（20 * 5% = 1）。",
            "步骤 3：排序后的第 1 个收益率是 -2.0%。",
            "最终结果：95% 日 VaR 为 -2.0%（或亏损 2.0%）。"
          ],
          "rubric_points": [
            "正确排序收益率（1分）",
            "正确识别 95% VaR 对应第 5% 分位点（1分）",
            "正确找出第 5% 分位点的收益率值（1分）",
            "最终答案正确（1分）"
          ],
          "stem": "某投资组合过去 20 个交易日的日收益率如下（单位：%）：\n1.2, -0.8, 0.5, -1.5, 2.0, -0.3, 0.9, -2.0, 1.8, -1.1, 0.1, -0.6, 1.5, -0.9, 0.3, -1.3, 2.5, -0.2, 1.0, -0.4\n请使用历史模拟法计算该投资组合的 95% 日 VaR。请写出计算步骤和最终结果。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "var_interpretation",
      "coverage_tags": [
        "var_definition",
        "var_interpretation"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_var_interpret",
      "learning_goal": "学生能在不同表述中准确识别 VaR 的正确含义。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "在险价值",
          "en": "Value at Risk (VaR)"
        }
      ],
      "variants": [
        {
          "answer": 3,
          "estimated_seconds": 20,
          "explanation": "VaR 表示在给定置信水平下，最大损失不会超过该数值。因此，99% VaR 为 500 万意味着有 99% 的把握，明天的亏损不会超过 500 万。",
          "options": [
            "明天有 99% 的概率亏损 500 万元。",
            "明天有 1% 的概率亏损超过 500 万元。",
            "明天最多亏损 500 万元。",
            "明天有 99% 的概率亏损不超过 500 万元。"
          ],
          "question_id": "q_quiz_var_interpret_v1",
          "stem": "一个投资组合的 99% 日 VaR 为 500 万元。以下哪个解读是正确的？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "95% VaR 意味着有 5% 的概率亏损会超过该值。因此，在 100 个交易日中，大约有 5 天（100 * 5%）的亏损会超过 200 万元。",
          "options": [
            "95 天",
            "5 天",
            "200 天",
            "无法确定"
          ],
          "question_id": "q_quiz_var_interpret_v2",
          "stem": "某基金公告称其 95% 的日 VaR 为 200 万元。这意味着在 100 个交易日中，大约有多少天的亏损会超过 200 万元？"
        }
      ]
    },
    {
      "concept_key": "var_methods",
      "coverage_tags": [
        "parametric_var",
        "historical_var"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_var_methods",
      "learning_goal": "学生能根据描述判断使用了哪种 VaR 计算方法。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "参数法 VaR",
          "en": "Parametric VaR"
        },
        {
          "display": "历史模拟法 VaR",
          "en": "Historical VaR"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "参数法（或方差-协方差法）的核心是假设收益率服从某种分布（如正态分布），然后利用分布的参数（均值和标准差）来计算 VaR。",
          "options": [
            "历史模拟法",
            "参数法",
            "蒙特卡洛模拟法",
            "压力测试法"
          ],
          "question_id": "q_quiz_var_methods_v1",
          "stem": "一位分析师通过计算过去 100 天收益率的均值和标准差，并假设收益率服从正态分布，从而计算出 VaR。他使用的是哪种方法？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "历史模拟法不依赖任何分布假设，直接使用过去实际发生的收益率数据，通过排序来找到对应分位点的损失值。",
          "options": [
            "参数法",
            "历史模拟法",
            "假设性 VaR",
            "压力 VaR"
          ],
          "question_id": "q_quiz_var_methods_v2",
          "stem": "一位交易员将过去 500 天的日收益率从低到高排序，然后选取第 5% 分位点的收益率作为 VaR 的估计值。他使用的是哪种方法？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L6: Capital and Risk Management. Agenda includes Value-at-Risk (VaR).",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "step5 concept: 在险价值（VaR）：一个数字告诉你最大可能亏损",
    "plain_text": "pipeline/1-plain/L6/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L6: Capital and Risk Management. Agenda includes Value-at-Risk (VaR)."
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "volatility_definition",
      "covered_by": [
        "qf_flash_vol_def",
        "qf_quiz_vol_def"
      ],
      "description": "波动率的定义：衡量资产价格变动幅度的指标，通常用标准差表示。"
    },
    {
      "coverage_tag": "standard_deviation_formula",
      "covered_by": [
        "qf_flash_sd_formula",
        "qf_quiz_sd_interpret"
      ],
      "description": "标准差公式：σ_{t,n} = sqrt(1/n * Σ(x_{t-i} - μ)^2)，用于计算历史波动率。"
    },
    {
      "coverage_tag": "ewma_concept",
      "covered_by": [
        "qf_flash_ewma_def",
        "qf_quiz_ewma_advantage",
        "qf_long_ewma_compare"
      ],
      "description": "EWMA（指数加权移动平均）：给近期数据更高权重的波动率估计方法。"
    },
    {
      "coverage_tag": "volatility_clustering",
      "covered_by": [
        "qf_flash_vc_def",
        "qf_quiz_vc_example",
        "qf_long_vc_explain"
      ],
      "description": "波动率聚集：大的价格变化后往往跟着大的变化，小的变化后跟着小的变化。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "volatility",
      "coverage_tags": [
        "volatility_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_vol_def",
      "learning_goal": "学生能准确说出波动率的定义及其在金融中的衡量方式。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "波动率的定义和衡量工具。",
      "term_refs": [
        {
          "display": "波动率",
          "en": "Volatility"
        }
      ],
      "variants": [
        {
          "back": "资产价格变动幅度的大小。",
          "estimated_seconds": 5,
          "explanation": "波动率是衡量资产价格波动剧烈程度的指标。",
          "front": "在金融中，“波动率”衡量的是什么？",
          "question_id": "q_flash_vol_def_v1"
        },
        {
          "back": "标准差。",
          "estimated_seconds": 5,
          "explanation": "标准差越大，说明价格波动越剧烈，风险也越高。",
          "front": "衡量波动率最常用的统计量是什么？",
          "question_id": "q_flash_vol_def_v2"
        },
        {
          "back": "剧烈；高。",
          "estimated_seconds": 5,
          "explanation": "标准差是衡量数据离散程度的统计量，数值越大表示波动越大。",
          "front": "标准差越大，说明资产价格的波动越____，风险越____。",
          "question_id": "q_flash_vol_def_v3"
        }
      ]
    },
    {
      "concept_key": "standard_deviation",
      "coverage_tags": [
        "standard_deviation_formula"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_sd_formula",
      "learning_goal": "学生能回忆标准差公式的组成部分及其含义。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "标准差公式中的符号含义。",
      "term_refs": [
        {
          "display": "标准差",
          "en": "Standard Deviation"
        }
      ],
      "variants": [
        {
          "back": "数据的均值。",
          "estimated_seconds": 8,
          "explanation": "μ 是用于计算离差平方和的平均值。",
          "front": "在标准差公式 σ_{t,n} = sqrt(1/n * Σ(x_{t-i} - μ)^2) 中，μ 代表什么？",
          "question_id": "q_flash_sd_formula_v1"
        },
        {
          "back": "观测值的数量（天数）。",
          "estimated_seconds": 5,
          "explanation": "n 是用于计算的历史数据点个数。",
          "front": "在标准差公式中，n 代表什么？",
          "question_id": "q_flash_sd_formula_v2"
        },
        {
          "back": "相同的权重。",
          "estimated_seconds": 5,
          "explanation": "这是简单标准差的一个缺点，它不能突出近期数据的重要性。",
          "front": "简单标准差给每一天的数据什么样的权重？",
          "question_id": "q_flash_sd_formula_v3"
        }
      ]
    },
    {
      "concept_key": "ewma",
      "coverage_tags": [
        "ewma_concept"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_ewma_def",
      "learning_goal": "学生能说出EWMA的核心思想及其与简单标准差的区别。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "EWMA的核心改进点。",
      "term_refs": [
        {
          "display": "指数加权移动平均",
          "en": "Exponentially Weighted Moving Average (EWMA)"
        }
      ],
      "variants": [
        {
          "back": "给近期数据更高的权重。",
          "estimated_seconds": 8,
          "explanation": "EWMA 通过指数衰减权重，使模型对近期市场变化反应更灵敏。",
          "front": "EWMA 相比于简单标准差，最大的改进是什么？",
          "question_id": "q_flash_ewma_def_v1"
        },
        {
          "back": "更灵敏（更反应近期事件）。",
          "estimated_seconds": 8,
          "explanation": "λ 越小，近期数据的权重越大，模型越容易受近期数据影响。",
          "front": "EWMA 中的 λ（衰减因子）越接近 0，波动率估计对近期事件的反应会怎样？",
          "question_id": "q_flash_ewma_def_v2"
        },
        {
          "back": "0.94。",
          "estimated_seconds": 5,
          "explanation": "这是一个常用的经验值，用于平衡近期和远期数据的影响。",
          "front": "RiskMetrics（JP Morgan）为 EWMA 的 λ 设置的默认值是多少？",
          "question_id": "q_flash_ewma_def_v3"
        }
      ]
    },
    {
      "concept_key": "volatility_clustering",
      "coverage_tags": [
        "volatility_clustering"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_vc_def",
      "learning_goal": "学生能描述波动率聚集现象及其含义。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "波动率聚集的定义。",
      "term_refs": [
        {
          "display": "波动率聚集",
          "en": "Volatility Clustering"
        }
      ],
      "variants": [
        {
          "back": "大的价格变化后往往跟着大的变化，小的变化后往往跟着小的变化。",
          "estimated_seconds": 8,
          "explanation": "这表明波动率不是随机的，而是具有自相关性。",
          "front": "什么是“波动率聚集”现象？",
          "question_id": "q_flash_vc_def_v1"
        },
        {
          "back": "随时间变化的（不是恒定的）。",
          "estimated_seconds": 5,
          "explanation": "波动率聚集现象直接反驳了波动率恒定不变的假设。",
          "front": "波动率聚集现象说明波动率是恒定的还是随时间变化的？",
          "question_id": "q_flash_vc_def_v2"
        },
        {
          "back": "持续很高。",
          "estimated_seconds": 5,
          "explanation": "这是波动率聚集的一个典型例子：大的负面冲击后，市场波动会持续一段时间。",
          "front": "市场崩盘时，波动率通常会持续很高还是低？",
          "question_id": "q_flash_vc_def_v3"
        }
      ]
    }
  ],
  "lesson_id": "L6",
  "longform_families": [
    {
      "concept_key": "ewma",
      "coverage_tags": [
        "ewma_concept",
        "standard_deviation_formula"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_ewma_compare",
      "learning_goal": "学生能比较和对比简单标准差与EWMA在波动率估计中的优缺点。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "compare_and_contrast",
      "term_refs": [
        {
          "display": "简单标准差",
          "en": "Simple Standard Deviation"
        },
        {
          "display": "指数加权移动平均",
          "en": "EWMA"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "权重分配",
            "对近期事件的反应速度",
            "幽灵特征的影响"
          ],
          "question_id": "q_long_ewma_compare_v1",
          "reference_answer": [
            "简单标准差给过去n天内的每一天数据分配相同的权重。EWMA则通过指数衰减因子λ（0<λ<1）给近期数据更高权重，远期数据权重指数递减。",
            "由于EWMA更重视近期数据，当市场发生突变时，EWMA的波动率估计值能更快地反映这一变化。简单标准差由于对所有数据一视同仁，其反应相对滞后。",
            "“幽灵特征”指一个极端的观测值会在后续一段时间内持续影响波动率估计。在简单标准差中，这个极端值会在接下来的n天内一直以相同权重影响结果，直到它被移出窗口。在EWMA中，该极端值的影响会随着时间推移按λ的幂次迅速衰减，因此“幽灵”效应更短暂、更轻微。"
          ],
          "rubric_points": [
            "明确指出简单标准差给所有数据点相同权重，而EWMA给近期数据更高权重。",
            "解释EWMA因此对近期市场变化反应更灵敏，而简单标准差反应较慢。",
            "解释简单标准差中一个极端观测值会在n天内持续影响估计值（幽灵特征），而EWMA中这种影响会随时间指数衰减。"
          ],
          "stem": "比较简单标准差和 EWMA 在估计金融资产波动率时的优缺点。请从权重分配、对近期事件的反应速度、以及“幽灵特征”（ghost features）的影响三个方面进行阐述。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "EWMA的优势",
            "简单标准差的适用场景"
          ],
          "question_id": "q_long_ewma_compare_v2",
          "reference_answer": [
            "对于高频交易策略，风险状况可能迅速变化。EWMA通过给近期数据更高权重，能更快地调整波动率估计，从而更及时地反映当前市场风险。此外，EWMA对极端值的处理更平滑，避免了简单标准差中极端值在固定窗口期内持续产生影响的“幽灵特征”。",
            "如果分析师有强有力的理论或实证证据表明资产的长期波动率是稳定的，并且希望估计值不受短期市场噪音的过度影响，那么使用一个较大时间窗口（例如n=120或252天）的简单标准差可能更合适。它能提供一个更平滑、更稳定的长期波动率基准。"
          ],
          "rubric_points": [
            "解释EWMA对市场变化反应灵敏，适合捕捉快速变化的风险。",
            "解释EWMA能更自然地处理“幽灵特征”。",
            "指出如果相信长期波动率是常数，且不希望被短期噪音干扰，使用较大n的简单标准差可能更合适。"
          ],
          "stem": "假设你是一个风险管理师，需要为一个高频交易策略选择波动率估计模型。请解释为什么 EWMA 通常比简单标准差更受青睐？在什么情况下，简单标准差可能是更合适的选择？"
        }
      ]
    },
    {
      "concept_key": "volatility_clustering",
      "coverage_tags": [
        "volatility_clustering"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_vc_explain",
      "learning_goal": "学生能用自己的语言解释波动率聚集现象及其对风险管理的重要性。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "波动率聚集",
          "en": "Volatility Clustering"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "波动率聚集的定义",
            "对风险管理的重要性",
            "市场实例"
          ],
          "question_id": "q_long_vc_explain_v1",
          "reference_answer": [
            "波动率聚集是指金融资产价格的大幅变动往往倾向于聚集在一起，即一个大的价格波动（无论是上涨还是下跌）之后，通常紧跟着另一个大的波动；同样，小的波动之后也往往跟着小的波动。",
            "这一现象对风险管理至关重要，因为它推翻了“波动率恒定”的简单假设。认识到波动率是随时间变化且具有自相关性的，风险管理者才能使用更先进的模型（如ARCH和GARCH）来动态地预测和衡量风险，而不是依赖一个静态的标准差。这有助于更准确地设定保证金要求、计算在险价值（VaR）以及制定仓位管理策略。",
            "一个典型的例子是2008年全球金融危机。危机爆发后，全球股市的波动率急剧上升，并在随后数月甚至数年内都保持在远高于危机前水平的状态，形成了明显的波动率聚集。"
          ],
          "rubric_points": [
            "准确定义波动率聚集：大的价格变化后往往跟着大的变化，小的变化后跟着小的变化。",
            "解释其重要性：它表明波动率不是常数，而是可预测的（自相关），因此可以使用ARCH/GARCH等模型进行建模和预测，从而更准确地衡量风险。",
            "给出一个实例，如2008年金融危机期间市场波动率持续高企，或一个公司发布盈利报告后股价波动加剧。"
          ],
          "stem": "请解释什么是“波动率聚集”现象，并说明为什么这一现象对金融风险管理至关重要。请至少给出一个市场实例。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "用波动率聚集解释观察到的现象",
            "对简单标准差风险评估的挑战"
          ],
          "question_id": "q_long_vc_explain_v2",
          "reference_answer": [
            "观察到的现象正是波动率聚集的典型表现。股票价格先是经历了一段高波动期（大的变化后跟着大的变化），然后进入了一段低波动期（小的变化后跟着小的变化）。这表明该股票的波动率状态发生了切换，而不是保持恒定。",
            "如果使用一个覆盖了高波动期和低波动期的简单标准差来评估当前风险，会面临严重问题。在低波动期，这个包含了高波动历史数据的标准差会高估当前的实际风险，导致资金利用不足或过度保守。反之，如果市场刚刚进入高波动期，而标准差窗口还包含之前的低波动数据，则会低估当前的风险，可能导致仓位过大而遭受损失。因此，简单标准差无法有效处理波动率状态的变化，而波动率聚集现象恰恰说明了这种状态变化是常态。"
          ],
          "rubric_points": [
            "正确地将观察到的现象归类为波动率聚集。",
            "解释高波动期和低波动期交替出现是波动率聚集的典型表现。",
            "指出如果使用包含整个时期（高波动和低波动）的简单标准差，会高估低波动期的风险，同时低估高波动期的风险，导致风险评估不准确。"
          ],
          "stem": "假设你观察到某只股票的价格在连续几天内都出现了超过2%的涨跌幅，随后又进入了连续几周波动率极低的时期。请用“波动率聚集”的概念解释这一现象，并讨论这对使用简单标准差进行风险评估可能带来的问题。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "volatility",
      "coverage_tags": [
        "volatility_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_vol_def",
      "learning_goal": "学生能在选择题中准确识别波动率的定义和衡量方式。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "波动率",
          "en": "Volatility"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "波动率衡量的是资产价格的不确定性或变动幅度，通常用标准差表示。",
          "options": [
            "资产的平均收益率",
            "资产价格变动的幅度",
            "资产的流动性",
            "资产的信用评级"
          ],
          "question_id": "q_quiz_vol_def_v1",
          "stem": "在金融风险管理中，“波动率”通常被用来衡量什么？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "标准差是衡量数据离散程度的标准统计量，常用于表示波动率。",
          "options": [
            "均值",
            "中位数",
            "标准差",
            "峰度"
          ],
          "question_id": "q_quiz_vol_def_v2",
          "stem": "以下哪个统计量最常用于量化金融资产的波动率？"
        }
      ]
    },
    {
      "concept_key": "standard_deviation",
      "coverage_tags": [
        "standard_deviation_formula"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_sd_interpret",
      "learning_goal": "学生能理解标准差公式的含义及其局限性。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "标准差",
          "en": "Standard Deviation"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "简单标准差给每一天的数据分配相同的权重，这是它的一个主要特点，也是其局限性所在。",
          "options": [
            "最近一天的数据权重最高",
            "每一天的数据权重相同",
            "过去 n 天以外的数据也会被考虑",
            "计算结果总是比 EWMA 更准确"
          ],
          "question_id": "q_quiz_sd_interpret_v1",
          "stem": "使用过去 n 天的数据计算简单标准差时，以下哪项描述是正确的？"
        },
        {
          "answer": 3,
          "estimated_seconds": 25,
          "explanation": "使用较小的 n 值可以让简单标准差更关注近期数据，而 EWMA 通过指数加权更系统地实现了这一点。两者都可以，但 EWMA 更精细。",
          "options": [
            "使用较大 n 值的简单标准差",
            "使用较小 n 值的简单标准差",
            "使用 EWMA",
            "以上两种方法（较小 n 的简单标准差和 EWMA）都可以，但 EWMA 更系统化"
          ],
          "question_id": "q_quiz_sd_interpret_v2",
          "stem": "如果一位分析师认为近期市场波动更能反映当前风险状况，他应该选择哪种波动率估计方法？"
        }
      ]
    },
    {
      "concept_key": "ewma",
      "coverage_tags": [
        "ewma_concept"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_ewma_advantage",
      "learning_goal": "学生能辨析EWMA相对于简单标准差的优势。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "指数加权移动平均",
          "en": "EWMA"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "EWMA 通过给近期数据更高权重，能更快地捕捉市场的最新变化。",
          "options": [
            "计算速度更快",
            "不需要假设数据分布",
            "对近期市场变化反应更灵敏",
            "结果总是更小"
          ],
          "question_id": "q_quiz_ewma_advantage_v1",
          "stem": "与使用相同数量历史数据的简单标准差相比，EWMA 的主要优势是什么？"
        },
        {
          "answer": 1,
          "estimated_seconds": 25,
          "explanation": "λ 越小，近期数据的权重越大，因此模型对近期市场变化的反应会更灵敏。",
          "options": [
            "对近期数据的反应变得更迟钝",
            "对近期数据的反应变得更灵敏",
            "波动率估计值会完全不变",
            "模型将不再有效"
          ],
          "question_id": "q_quiz_ewma_advantage_v2",
          "stem": "在 EWMA 模型中，如果 λ（衰减因子）从 0.94 降低到 0.80，波动率估计值会如何变化？"
        }
      ]
    },
    {
      "concept_key": "volatility_clustering",
      "coverage_tags": [
        "volatility_clustering"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_vc_example",
      "learning_goal": "学生能识别波动率聚集现象的实际例子。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "波动率聚集",
          "en": "Volatility Clustering"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "波动率聚集指大的波动后往往跟着大的波动，市场崩盘后的持续高波动是典型例子。",
          "options": [
            "股票价格长期稳定上涨",
            "市场崩盘后，波动率持续处于高位",
            "债券价格与利率呈反向变动",
            "公司盈利超预期导致股价跳空高开"
          ],
          "question_id": "q_quiz_vc_example_v1",
          "stem": "以下哪个市场现象最符合“波动率聚集”的描述？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "波动率聚集现象直接表明波动率不是恒定的，而是随时间变化的。",
          "options": [
            "该资产价格是随机游走的",
            "该资产的波动率是随时间变化的",
            "该资产没有风险",
            "该资产的收益率是正态分布的"
          ],
          "question_id": "q_quiz_vc_example_v2",
          "stem": "观察到某资产价格在一段时期内频繁出现大幅涨跌，随后进入一段相对平静期。这种现象支持以下哪个观点？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L6: Capital and Risk Management - Risk Measurement: Volatility Analysis (Historical estimates, EWMA, GARCH model)",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "L6 step3: 衡量风险：从标准差到波动率模型",
    "plain_text": "pipeline/1-plain/L6/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L6: Capital and Risk Management - Risk Measurement: Volatility Analysis (Historical estimates, EWMA, GARCH model)"
  },
  "target_language": "zh-CN"
}

]
</QUESTION_BANK>

<PLAIN_TEXT>
# L6: Capital and Risk Management

Course Code: COMP7415

# Agenda

• Introduction to the risk management cycle

- Risk identification   
- Risk measurement

- Volatility modelling including ARCH, GARCH   
- Value-at-Risk (VaR)   
- Stress testing, Scenario analysis

- Position sizing strategy

- Fixed Size   
Balance Rescaling   
- Dollar Risk Approach   
- Kelly Criterion

# What is Risk?

![](images/5356a963e01aaccc319c2604722e58132deb56083bf83b844da77a029d6e0cba.jpg)

# What is Risk?

- Risk simply refers to uncertainty   
- In finance, risk refers to the uncertainty of the investment return.

- Up-side risk: the possibility of making money   
- Down-side risk: the possibility of losing money

# Risk Management Cycle

1. Risk Identification   
2. Risk Assessment / Measurement   
3. Risk Treatment   
4. Risk Monitoring

![](images/1b25afef457cce7d0af1c1efc7bad43caa138fea188417d07cc746c7cfa3cd86.jpg)

# Risk Identification

# Major Risk Types for a financial institution

1. Market Risk

Equity Risk   
- FX Risk   
Commodity Risk

2. Interest rate Risk   
3. Credit Risk   
4. Liquidity Risk   
5. Operational Risk   
6. Legal Risk   
7. Reputational Risk   
8. Strategic Risk

Investment related risks

# Market Risk

- Market Risk is the potential for financial losses due to changes in market prices. It affects assets such as stocks, bonds, currencies, and commodities.   
- Example: Suppose you own shares in a tech company. If the stock market experiences a downturn, the value of your shares may decrease regardless of the company's performance, due to overall market sentiment.   
- This type of risk is inherent to all investments and can be influenced by factors like economic changes, political events, and natural disasters.

# Interest Risk

- Interest Rate Risk is the potential for investment losses due to fluctuations in interest rates. It primarily affects the value of bonds and other fixed-income securities. When interest rates rise, bond prices typically fall, and vice versa.   
- Example: Imagine you own a bond with a fixed interest rate of $3\%$ . If the market interest rate rises to $4\%$ , new bonds offer better returns, making your bond less attractive. Consequently, the market value of your bond decreases.   
- This risk is crucial for investors and financial institutions managing portfolios sensitive to interest rate changes.

# Credit Risk

- Credit Risk is the possibility of a loss resulting from a borrower's failure to repay a loan or meet contractual obligations. It affects lenders and investors in bonds or loans.   
- Example: If a bank lends money to a business, and the business defaults on the loan, the bank faces credit risk. This risk can lead to financial loss for the bank due to the unpaid loan amount.   
- Credit risk is a key consideration in lending and investing, influencing interest rates and lending terms.

# Liquidity Risk

- Liquidity Risk is the risk that an entity may not be able to quickly convert assets into cash without significant loss in value. It affects individuals, businesses, and financial institutions.   
- Example: Imagine a company owns a large amount of real estate. If it suddenly needs cash to cover expenses, selling the properties quickly might force them to accept lower prices, resulting in a financial loss.   
- Liquidity risk is important for managing cash flow and ensuring that obligations can be met when they come due.

# What type of risk involved in these examples?

1. A company defaults on its loan payment.   
2. You need to sell a property quickly but can't find a buyer without reducing the price significantly.   
3. The value of your bond portfolio decreases due to rising interest rates.   
4. Stock prices drop due to a sudden economic downturn.   
5. A bank is unable to meet its short-term cash obligations.   
6. An investor worries about a borrower's ability to repay a loan.   
7. The price of a commodity fluctuates widely and affects your investment.   
8. You are a fresh graduate. You worry about the house price keeps increasing and unaffordable to buy.

# Risk Measurement

# Measuring Risk

1. Volatility Analysis

Historical estimates   
- Exponentially weighted moving average (EWMA)   
GARCH model

2. Value at Risk (VaR)   
3. Stress Testing and Scenario Analysis

# Standard Derivation

- Standard derivation is a statistical measure that quantifies the amount of variation or dispersion in a set of data values

$$
\sigma_ {t, n} = \sqrt {\frac {1}{n} \sum_ {i = 1} ^ {n} (x _ {t - i} - \mu) ^ {2}}
$$

where $\sigma_{t,n}$ is standard deviation at time $t$ , $n$ is the number of observations, $x_{t - i}$ is each value, and $\mu$ is the mean.

- Volatility is often measured using standard deviation.   
- High standard deviation indicates high volatility, meaning the asset's price can vary significantly.

# Standard Derivation

- Since $n$ is fixed and the last $n$ observations are used, we also call this a moving average (MA) estimate   
• Common values for $n$ : 30, 60, 120 days, etc   
- If one believes that the long term volatility is a constant, then a larger $n$ should be used.   
- If one wants to reflect more about the current situation, then a smaller $n$ should be considered.   
- Disadvantage: extreme observations can affect the estimate for a prolonged period of time (ghost features). A small n gives more pronounced ghost features but for a shorter period of time.

# EWMA

- To avoid the problem of equally weighted averages in the moving average estimates, we alternatively use the exponentially weighted moving averages (EWMA) by putting more weight on the recent data.   
- EWMA is defined as

$$
\sigma_ {t} ^ {2} = \frac {r _ {t - 1} ^ {2} + \lambda r _ {t - 2} ^ {2} + \lambda^ {2} r _ {t - 3} ^ {2} + \cdots + \lambda^ {n - 1} r _ {t - n} ^ {2}}{1 + \lambda + \lambda^ {2} + \cdots + \lambda^ {n - 1}}
$$

where $0 < \lambda < 1$ is the discounting factor.

- Here we assume $r_t$ are mean-corrected returns. Otherwise, we should subtract estimated mean from them.

# EWMA

• Since

$$
1 + \lambda + \lambda^ {2} + \dots + \lambda^ {n - 1} \approx \frac {1}{1 - \lambda}
$$

- So

$$
\begin{array}{l} \sigma_ {t} ^ {2} \approx (1 - \lambda) (r _ {t - 1} ^ {2} + \lambda r _ {t - 2} ^ {2} + \lambda^ {2} r _ {t - 3} ^ {2} + \dots + \lambda^ {n - 1} r _ {t - n} ^ {2}) \\ \approx (1 - \lambda) \sum_ {i = 1} ^ {n} \lambda^ {i - 1} r _ {t - i} ^ {2} \\ \approx (1 - \lambda) \sum_ {i = 1} ^ {\infty} \lambda^ {i - 1} r _ {t - i} ^ {2} \\ \end{array}
$$

# EWMA

- If $\lambda$ close to 0, $\sigma_t^2$ will be more reactive to current events   
- If $\lambda$ close to $1$ , $\sigma_t^2$ will depend more on past values   
- RiskMetrics (JP Morgan) has a default value of 0.94 for $\lambda$

# Volatility Clustering

- Volatility clustering refers to the observation that large changes in asset prices are often followed by large changes, and small changes tend to be followed by small changes. This suggests that volatility is not constant over time.   
- In financial market, volatility tends to increase during

Market crash   
- Natural disasters   
- Wars   
- Company earning release   
President election due to policy uncertainty

# Volatility Clustering - SP500

import yfinance as yf

import numpy as np

import pandas as pd

import matplotlib.pyplot as plt

Download S&P 500 (^GSPC) data

data = yf.download('^GSPC', start='2000-01-01', end='2024-12-31')

Calculate daily returns

data['Daily Return'] = data['Close'].pct_change()

Calculate rolling volatility with a 30-day lookback period

data['Volatility'] = data['Daily Return'].rollingwindow=30).std() * np.sqrt(252)

Annualized volatility # Plot the volatility

plt.figure(figsize=(12,6))

plt.plot(data index, data['Volatility'], label='30-Day Rolling Volatility')

plt title('S&P 500 Volatility Clustering (2000-2024)')

plt.xlabel('Date')

plt ylabel('Annualized Volatility')

pltlegend()

plt grid(True)

plt.show()

![](images/a91a75dc7ceb2a80004445757599b6098717ad44d4709fa3ad61399798a94c2a.jpg)  
S&P 500 Volatility Clustering (2000-2024)

# ARCH/ GARCH model

- The ARCH or GARCH (Generalized Autoregressive Conditional Heteroskedasticity) model are commonly used to model time-varying volatility, which take into account of volatility clustering.   
- When volatility is autoregressive, we call this the ARCH effect.   
- To check whether $\left\{r_{t}^{2}\right\}$ is correlated, we can look at the ACF terms

$$
| \widehat {\rho_ {1}} | > \frac {2}{\sqrt {T}}
$$

$$
| \widehat {\rho_ {2}} | > \frac {2}{\sqrt {T}}
$$

···

# ARCH/ GARCH model

- For overall checking, we can use McLeod-Li test (1983)

$$
Q _ {2} (m) = T (T + 2) \sum_ {i = 1} ^ {m} \frac {\widehat {\rho_ {i}} ^ {2}}{T - i} \quad \sim \quad \chi^ {2} (m)
$$

Hypothesis:

- $H_{0}$ : ARCH effect is not present   
- $H_{1}$ : ARCH effect is present (i.e. volatility clustering effect exists)

- In McLeod-Li test, commonly used $m$ are 6, 12, 18, and 24

# ARCH/ GARCH model

- Generally the conditional variance $R_{t}$ is defined as

$$
\sigma_ {t} ^ {2} = E \left[ \left(R _ {t} - \mu_ {t}\right) ^ {2} \mid F _ {t - 1} \right]
$$

where $F_{t-1}$ is the information up to time $t-1$

- If $R_{t} = \mu_{t} + a_{t},\sigma_{t}^{2} = E\left[a_{t}{}^{2}\mid F_{t - 1}\right]$   
- In the trivial case of $\mu_t = 0$ (i.e. $R_t = a_t$ ), $\sigma_t^2 = E[R_t^2 \mid F_{t-1}]$

# The ARCH(1) model

- Since volatility is autoregressive and dependent on past squared returns, it is natural to assume, say

$$
\sigma_ {t} ^ {2} = \alpha_ {0} + \alpha_ {1} r _ {t - 1} ^ {2} \qquad \mathrm {w h e r e} \alpha_ {0} > 0 \mathrm {a n d} 0 \leq \alpha_ {1} <   1 \mathrm {a r e c o n s t a n t s}
$$

It is the so called ARCH(1) model.   
- Here we assume $\{r_t\}$ is mean corrected and serially uncorrelated. If not, we replace $r_{t - 1}$ by $\varepsilon_{t - 1}$ , where its realization $\widehat{a_{t - 1}}$ are the residuals from an AR model fitted to $r_{t - 1}$

# The ARCH(1) model

- A more formal definition of ARCH(1) model for a return series $\{r_t\}$ is defined as

$$
\left\{ \begin{array}{l} r _ {t} = \varphi_ {0} + \varphi_ {1} r _ {t - 1} + \dots + \varphi_ {p} r _ {t - p} + a _ {t} \\ \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad i. i. d. \\ a _ {t} = \sigma_ {t} \varepsilon_ {t} \qquad \mathrm {w h e r e} \{\varepsilon_ {t} \} \sim \mathrm {N} (0, 1) \\ \sigma_ {t} ^ {2} = E (\varepsilon_ {t} ^ {2} | F _ {t - 1}) = \alpha_ {0} + \alpha_ {1} a _ {t - 1} ^ {2} \end{array} \right.
$$

- The first AR(p) equation is used to estimate the conditional means, and the last one to estimate volatilities.   
- Therefore, they are usually referred to as the mean equation and the volatility equation respectively.

# Properties of the ARCH(1) model

- The AR residuals or mean-corrected returns $\{a_{t}\}$ are serially uncorrelated.

$E\left(a_{t}\right) = 0$   
$\operatorname{Var}\left(a_{t}\right) = \operatorname{E}\left(a_{t}^{2}\right) = \frac{\alpha_{0}}{1 - \alpha_{1}}$   
- The residuals $\left\{a_{t}\right\}$ are statistically independent.   
- Conditioning on previous information $F_{t - 1}, a_t$ is conditionally normal $N(0, \sigma_t^2)$

- The squared residual $\left\{a_{t}^{2}\right\}$ are serially correlated with

$$
\rho (k) = C o r r \bigl (a _ {t} ^ {2}, a _ {t - k} ^ {2} \bigr) = \alpha_ {1} ^ {k}
$$

# Properties of the ARCH(1) model

- $E\left(a_{t}^{4}\right)$ exists if and only if $\alpha_{1}< \frac{1}{\sqrt{3}}$ . Moreover, the kurtosis is $\kappa = \frac{3(1 - \alpha_{1}^{2})}{1 - 3\alpha_{1}^{2}} >3$   
- Define $\eta_{t} = a_{t}^{2} - \sigma_{t}^{2}$ . Substituting this into the volatility equation becomes

$$
a _ {t} ^ {2} = \alpha_ {0} + \alpha_ {1} a _ {t - 1} ^ {2} + \eta_ {t}
$$

which is in the form of an AR(1) model. This fact is usually used for specification of ARCH models

# The ARCH(m) model

A generalization $\mathrm{ARCH}(\mathfrak{m})$ model is

$$
\left\{ \begin{array}{l l} a _ {t} = \sigma_ {t} \varepsilon_ {t} & \{\varepsilon_ {t} \} \stackrel {\mathrm {i . i . d .}} {\sim} \mathrm {N} (0, 1) \\ \sigma_ {t} ^ {2} = \alpha_ {0} + \alpha_ {1} a _ {t - 1} ^ {2} + \dots + \alpha_ {m} a _ {t - m} ^ {2} \end{array} \right.
$$

- Here $\alpha_0 > 0$ and $\alpha_1, \ldots, \alpha_m \geq 0$ . Also, we assume for stationary that

$$
\alpha_ {1} + \dots + \alpha_ {m} <   1
$$

# Properties of ARCH(m) models

- The ARCH(m) model has similar properties as the ARCH(1). For example,

- $\{a_{t}\}$ is serially uncorrelated, conditional normal;   
• $\left\{a_{t}^{2}\right\}$ is serially correlated;   
- $a_{t}$ has a kurtosis (under more strict conditions) greater than 3

- We simple state here that

$$
V a r (a _ {t}) = \frac {\alpha_ {0}}{1 - \alpha_ {1} - \cdots - \alpha_ {m}}
$$

- Moreover, if we define $\eta_{t} = a_{t}^{2} - \sigma_{t}^{2}$ , then

$$
a _ {t} ^ {2} = \alpha_ {0} + \alpha_ {1} a _ {t - 1} ^ {2} + \dots + \alpha_ {m} a _ {t - m} ^ {2} + \eta_ {t}
$$

# Lagrange multiplier test for ARCH effects

- Lagrange multiplier test considers a multiple linear regression of $a_{t}^{2}$ on the lagged values

$$
a _ {t} ^ {2} = \beta_ {0} + \beta_ {1} a _ {t - 1} ^ {2} + \dots + \beta_ {m} a _ {t - m} ^ {2} + \varepsilon_ {t}, \qquad t = m + 1, \ldots , T
$$

where $\varepsilon_{t}$ denotes the error term, $m$ is a pre-specified positive integer, and $T$ is sample size

Hypotheses:

$$
H _ {0} \colon \beta_ {0} = \dots = \beta_ {m} = 0
$$

$H_{1}$ : Not all $\beta$ 's equal to zero

# Lagrange multiplier test

- The Lagrange multiplier test statistic is

$$
L M = (T - m) R ^ {2}
$$

where $R^2$ is the coefficient of determination of the regression

- Under $H_{0}$ , LM asymptotically follows $\chi^{2}(m)$   
- We reject $H_{0}$ if $LM > \chi_{\alpha}^{2}(m)$ at the $\alpha$ level of significance

# Model Building Steps

1. Specify a mean equation by testing for serial dependence in the data.

- For most asset return series, the serial correlations are weak.   
- If, necessary, build an economic model (simple AR or MA model) for the return series to remove any linear dependence.   
- Some explanatory variables may be employed.

2. Use the residuals of the mean equation to test for ARCH effects.   
3. Specify a volatility model if ARCH effects are statistically significant and perform a joint estimation of the mean and volatility equations   
4. Check the fitted model and refine.

# Estimating ARCH models

- Notice that the volatility equation in an ARCH(m) model is equivalent to an AR(m) model for the squared residuals from the mean equation.   
- We can separately estimate the mean equation and the volatility equation by either least-squares or maximum likelihood method.   
- As an alternative, we can jointly estimate the mean and volatility equations.

# ARCH library

- Official Doc: https://bashtage.github.io/arch/

![](images/d518a3f443ead3d436e3a0ba690558c22e2b15b2891ab4c3ee09f9d3a3028a21.jpg)

# SPY Example

# - Let's consider the SPY daily closing price in 2010-2020

import yfinance as yf

import numpy as np

import pandas as pd

import matplotlib.pyplot as plt

from statsmodels.graphics.tsaplots import plot_acf, plot_pacf

Download data

data = yf.download('SPY', start='2010-01-01', end='2020-12-31')

Calculate daily returns

data['Daily Return'] = data['Close'].pct_change()

data['Squared Return'] = data['Daily Return'] **2

Plot the return

plt.figure(figsize=(12,6))

plt.plot(data.index, data['Daily Return'], label='Daily Return')

plt.title('SPY Daily Return')

pltxlabel('Date')

pltylabel('Daily Return')

plt.legend()

plt.grid(True)

plt show()

![](images/62c44ff2b867f879ea544c36db80fd9c7c3f7627ee01c577fe5e04886f3d2b75.jpg)

# SPY Example

- ACF for returns series shown to be serially uncorrelated   
- Thus, mean equation is set to $A R(0)$

$$
\left\{ \begin{array}{l l} r _ {t} = \varphi_ {0} + a _ {t} & \\ a _ {t} = \sigma_ {t} \varepsilon_ {t} & \{\varepsilon_ {t} \} \stackrel {\mathrm {i . i . d .}} {\sim} \mathrm {N} (0, 1) \end{array} \right.
$$

Plot ACF

data1 = data['Daily Return'])

data1 = data1.dropna()

plot_acf(data1)

plt.show()

![](images/1991fb9ac9812ab565595e29928715bfe98c7a9027b64b1d16c7513772750b6c.jpg)

# SPY Example - Mean equation

- The parameters are significant, thus we got

$$
\left\{ \begin{array}{l l} r _ {t} = 0. 0 0 0 5 7 + a _ {t} & \\ & \text {i . i . d .} \\ a _ {t} = 0. 0 0 0 1 1 8   \varepsilon_ {t} \quad \text {w h e r e} \{\varepsilon_ {t} \} \sim \mathrm {N} (0, 1) \end{array} \right.
$$

import arch

$$
\begin{array}{l} \text {d a t a 1} = \text {d a t a} [ [ ^ {\prime} D a l l y R e t u r n ^ {\prime} ] ] \\ \mathsf {d a t a 1} = \mathsf {d a t a 1 . d r o p n a ()} \\ \end{array}
$$

$$
\begin{array}{l} \operatorname {a r x} = \text {a r c h . u n i v a r i a t e . A R X (y = d a t a 1 , l a g s = N o n e)} \\ \mathrm {r e s} = \operatorname {a r x . f i t ()} \\ \mathsf {p r i n t} (\mathsf {r e s}) \\ \end{array}
$$

Lag terms for mean series $r_t$

<table><tr><td colspan="6">AR - Constant Variance Model Results</td></tr><tr><td>Dep. Variable:</td><td colspan="2">(&#x27;Daily Return&#x27;, &#x27;&#x27;)</td><td>R-squared:</td><td>0.000</td><td></td></tr><tr><td>Mean Model:</td><td colspan="2">AR</td><td>Adj. R-squared:</td><td>0.000</td><td></td></tr><tr><td>Vol Model:</td><td colspan="2">Constant Variance</td><td>Log-Likelihood:</td><td>8585.64</td><td></td></tr><tr><td>Distribution:</td><td colspan="2">Normal</td><td>AIC:</td><td>-17167.3</td><td></td></tr><tr><td>Method:</td><td colspan="2">Maximum Likelihood</td><td>BIC:</td><td>-17155.4</td><td></td></tr><tr><td></td><td colspan="2"></td><td>No. Observations:</td><td>2767</td><td></td></tr><tr><td>Date:</td><td colspan="2">Sun, Oct 05 2025</td><td>Df Residuals:</td><td>2766</td><td></td></tr><tr><td>Time:</td><td colspan="2">22:05:45</td><td>Df Model:</td><td>1</td><td></td></tr><tr><td colspan="6">Mean Model</td></tr><tr><td>coef</td><td>std err</td><td>t</td><td>P&gt;|t|</td><td>95.0% Conf. Int.</td><td></td></tr><tr><td>Const</td><td>5.6800e-04</td><td>2.066e-04</td><td>2.749</td><td>5.981e-03</td><td>[1.630e-04,9.730e-04]</td></tr><tr><td></td><td></td><td colspan="4">Volatility Model</td></tr><tr><td>coef</td><td>std err</td><td>t</td><td>P&gt;|t|</td><td>95.0% Conf. Int.</td><td></td></tr><tr><td>sigma2</td><td>1.1814e-04</td><td>8.819e-06</td><td>13.397</td><td>6.306e-41</td><td>[1.009e-04,1.354e-04]</td></tr><tr><td colspan="6">Covariance estimator: White&#x27;s Heteroskedasticity Consistent Estimator</td></tr></table>

# SPY Example

# - Volatility clustering is obvious from the squared return chart

plot Squared return

plt.figure(figsize=(12,6))

plt.plot(data.index, data['Squared Return'], label='Squared Return')

plt.title('SPY Squared Return')

plt.xlabel('Date')

pltylabel('Squared Return')

pltlegend()

plt.grid(True)

plt.show()

![](images/a531e14dcfdc796253226464d0f5f2d42d3998af260ed2d909eff42da07a261a.jpg)

# SPY Example

- From squared returns' PACF, an ARCH(2) model is tentatively identified

$$
\left\{ \begin{array}{c} a _ {t} = \sigma_ {t} \varepsilon_ {t} \qquad \{\varepsilon_ {t} \} \stackrel {\mathrm {i . i . d .}} {\sim} \mathrm {N} (0, 1) \\ \sigma_ {t} ^ {2} = \alpha_ {0} + \alpha_ {1} a _ {t - 1} ^ {2} + \alpha_ {2} a _ {t - 2} ^ {2} \end{array} \right.
$$

Plot ACF and PACF  
data1 = data['Squared Return'].  
data1 = data1.dropna()  
plot_acf(data1)  
plot_pacf(data1)  
plt.show()

![](images/42b904ecd132882101228fb0b875d3e845ab19eed4adabee4ed9aca286622f33.jpg)

![](images/dc69fb702dbfc3ab61c368e2626415bcd5201ea7d61550b476a0eb7c25d9993f.jpg)

# SPY Example - Joint Estimation

- All parameters are significant, so our model will be

$$
\left\{ \begin{array}{l} r _ {t} = 0. 0 0 0 9 8 3 7 3 + a _ {t} \\ \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad i. i. d. \\ a _ {t} = \sigma_ {t} \varepsilon_ {t} \qquad \{\varepsilon_ {t} \} \sim N (0, 1) \\ \sigma_ {t} ^ {2} = 0. 0 0 0 0 4 1 3 5 + 0. 3 2 5 a _ {t - 1} ^ {2} + 0. 3 2 5 a _ {t - 2} ^ {2} \end{array} \right.
$$

import arch

data1 = data[['Daily Return'])

data1 = data1.dropna()

arm = arch.univariate.arch_model( y=data1, lags=None, mean='AR', vol='ARCH', p=2

res = arm.fit()
print(res)

![](images/0666524872d854b8d7641df8e025e553bd211d835436dc04a1e8f544d0e725b4.jpg)

# SPY Example - Final model

- The unconditional variance of $a_{t}$

$$
V a r (a _ {t}) = \frac {\alpha_ {0}}{1 - \alpha_ {1} - \cdots - \alpha_ {m}} = \frac {0 . 0 0 0 0 4 1 3 5}{1 - 0 . 3 2 5 - 0 . 3 2 5} = 0. 0 0 0 1 1 8 1 4
$$

# Model Forecast

- Forecasting of ARCH models can be obtained recursively as that of AR   
1-step ahead forecast

$$
\begin{array}{l} \sigma_ {h} ^ {2} (1) = E \big (\sigma_ {h + 1} ^ {2} \big | F _ {h} \big) \\ = E \left(\alpha_ {0} + \alpha_ {1} a _ {h} ^ {2} + \dots + \alpha_ {m} a _ {h + 1 - m} ^ {2} \mid F _ {h}\right) \\ = \alpha_ {0} + \alpha_ {1} a _ {h} ^ {2} + \dots + \alpha_ {m} a _ {h + 1 - m} ^ {2} \\ = \sigma_ {h} ^ {2} \\ \end{array}
$$

# Model Forecast

2-step ahead forecast

$$
\begin{array}{l} \sigma_ {h} ^ {2} (2) = E \big (\sigma_ {h + 2} ^ {2} \big | F _ {h} \big) \\ = E \left(\alpha_ {0} + \alpha_ {1} a _ {h + 1} ^ {2} + \dots + \alpha_ {m} a _ {h + 2 - m} ^ {2} \mid F _ {h}\right) \\ = \alpha_ {0} + \alpha_ {1} E \left(a _ {h + 1} ^ {2} \mid F _ {h}\right) + \alpha_ {2} a _ {h} ^ {2} + \dots + \alpha_ {m} a _ {h + 2 - m} ^ {2} \\ = \alpha_ {0} + \alpha_ {1} E \left(\sigma_ {h + 1} ^ {2} \varepsilon_ {h + 1} ^ {2} \mid F _ {h}\right) + \alpha_ {2} a _ {h} ^ {2} + \dots + \alpha_ {m} a _ {h + 2 - m} ^ {2} \\ = \alpha_ {0} + \alpha_ {1} \sigma_ {h + 1} ^ {2} E \left(\varepsilon_ {h + 1} ^ {2} \mid F _ {h}\right) + \alpha_ {2} a _ {h} ^ {2} + \dots + \alpha_ {m} a _ {h + 2 - m} ^ {2} \\ = \alpha_ {0} + \alpha_ {1} \sigma_ {h} ^ {2} (1) + \alpha_ {2} a _ {h} ^ {2} + \dots + \alpha_ {m} a _ {h + 2 - m} ^ {2} \\ \end{array}
$$

# ARCH Model Forecast

- k-step ahead forecast

$$
\sigma_ {h} ^ {2} (k) = E \big (\sigma_ {h + k} ^ {2} \big | F _ {h} \big) = \alpha_ {0} + \sum_ {i = 1} ^ {m} \alpha_ {i} \sigma_ {h} ^ {2} (k - i)
$$

$$
\mathrm {w h e r e} \sigma_ {h} ^ {2} (k - i) = a _ {h + k - m} ^ {2} \mathrm {i f k - i \leq 0}
$$

# GARCH Model

# GARCH model

- Bollerslev (1986) proposes a useful extension known as the generalized ARCH (GARCH) model. For a mean-corrected return series $\{a_{t}\}$ , it is said to follow a GARCH(m, s) model if

$$
\left\{ \begin{array}{c} a _ {t} = \sigma_ {t} \varepsilon_ {t} \\ \sigma_ {t} ^ {2} = \alpha_ {0} + \sum_ {i = 1} ^ {m} \alpha_ {i} a _ {t - i} ^ {2} + \sum_ {j = 1} ^ {s} \beta_ {j} \sigma_ {t - j} ^ {2} \end{array} \right.
$$

$$
\text {w h e r e} \left\{ \begin{array}{c} \quad \text {i . i . d .} \\ \{\varepsilon_ {t} \} \sim \mathrm {N} (0, 1) \\ \max  _ {i = 1} ^ {m, s} (\alpha_ {i} + \beta_ {i}) <   1 \qquad \text {i n w h i c h} \alpha_ {i} = 0 f o r i > m a n d \beta_ {j} = 0 f o r j > s \end{array} \right.
$$

# GARCH model

- The GARCH(m, s) model reduces to a pure ARCH(m) model if $s = 0$ .   
- $\alpha_{i}$ is referred to as ARCH parameters.

GARCH error coefficients.   
Large value indicate volatility reacts quite intensely to market movements.   
- Volatility tend to be more spiky.

- $\beta_{j}$ is referred to as GARCH parameters.

GARCH lag coefficient.   
Large value indicate shocks to conditional variance take a long time to die out.   
- Volatility is persistent.

# GARCH(1,1) model

- The most commonly used GARCH model is the simplest GARCH(1,1) model

$$
\sigma_ {t} ^ {2} = \alpha_ {0} + \alpha_ {1} a _ {t - 1} ^ {2} + \beta_ {1} \sigma_ {t - 1} ^ {2} \qquad \mathrm {w h e r e} 0 \leq \alpha_ {1}, \beta_ {1} \leq 1, \alpha_ {1} + \beta_ {1} <   1
$$

- It can be seen that a large $a_{t - 1}^2$ or $\sigma_{t - 1}^2$ give rise to a large $\sigma_t^2$ . This means that a large $a_{t - 1}^2$ tends to be followed by another large $a_{t}^{2}$ generating the well-know behavior of volatility clustering in financial time series.

# GARCH(1,1) model

- Tail distribution: similar to ARCH models, the tail distribution of a GARCH(1,1) process is heavier than that of a normal distribution (kurtosis $>3$ )   
- Actually, if $a_t$ is 4-th order stationary with $m_4 = a_t^4$ and $m_2 = a_t^2$ . Then,

$$
\begin{array}{l} m _ {2} = E [ E (a _ {t} ^ {2} | F _ {t - 1}) ] = E [ E (\varepsilon_ {t} ^ {2} \sigma_ {t} ^ {2} | F _ {t - 1}) ] = E [ E (\sigma_ {t} ^ {2} | F _ {t - 1}) ] \\ = E (\sigma_ {t} ^ {2}) = \frac {1}{1 - \alpha_ {1} - \beta_ {1}} \\ \end{array}
$$

# GARCH(1,1) model

$$
m _ {4} = E [ E (a _ {t} ^ {4} | F _ {t - 1}) ] = E [ E (\varepsilon_ {t} ^ {4} \sigma_ {t} ^ {4} | F _ {t - 1}) ] = E (3 \sigma_ {t} ^ {4}) = 3 E (\sigma_ {t} ^ {4})
$$

Also, $E(a_{t}^{2}\sigma_{t}^{2}) = E(E(a_{t}^{2}\sigma_{t}^{2}|F_{t - 1})) = E(\sigma_{t}^{2}E(a_{t}^{2}|F_{t - 1})) = E(\sigma_{t}^{2}\sigma_{t}^{2}) = \frac{m_{4}}{3}$

Then,

$$
\begin{array}{l} m _ {4} = 3 E (\sigma_ {t} ^ {4}) = 3 E ((\alpha_ {0} + \alpha_ {1} a _ {t - 1} ^ {2} + \beta_ {1} \sigma_ {t - 1} ^ {2}) ^ {2}) \\ = 3 E (\alpha_ {0} ^ {2} + \alpha_ {1} ^ {2} a _ {t - 1} ^ {4} + \beta_ {1} ^ {2} \sigma_ {t - 1} ^ {4} + 2 \alpha_ {0} \alpha_ {1} a _ {t - 1} ^ {2} + 2 \alpha_ {0} \beta_ {1} \sigma_ {t - 1} ^ {2} + 2 \alpha_ {1} \beta_ {1} a _ {t - 1} ^ {2} \sigma_ {t - 1} ^ {2}) \\ = 3 (\alpha_ {0} ^ {2} + \alpha_ {1} ^ {2} m _ {4} + \beta_ {1} ^ {2} m _ {4} / 3 + 2 \alpha_ {0} \alpha_ {1} m _ {2} + 2 \alpha_ {0} \beta_ {1} m _ {2} + 2 \alpha_ {1} \beta_ {1} m _ {4} / 3) \\ \end{array}
$$

$$
\Rightarrow \qquad m _ {4} = \frac {3 (\alpha_ {0} ^ {2} + 2 \alpha_ {0} (\alpha_ {1} + \beta_ {1}) m _ {2})}{1 - 3 \alpha_ {t} ^ {2} - \beta_ {t} ^ {2} - 2 \alpha_ {1} \beta_ {1}}
$$

# GARCH(1,1) model

- So Kurtosis of $a_{t}$ is

$$
K (a _ {t}) = \frac {m _ {4}}{m _ {2} ^ {2}} = \frac {3 (\alpha_ {0} ^ {2} + 2 \alpha_ {0} (\alpha_ {1} + \beta_ {1}) m _ {2})}{(1 - 3 \alpha_ {t} ^ {2} - \beta_ {t} ^ {2} - 2 \alpha_ {1} \beta_ {1}) m _ {2} ^ {2}} = \frac {3 [ 1 - (\alpha_ {1} + \beta_ {1}) ^ {2} ]}{1 - (\alpha_ {1} + \beta_ {1}) ^ {2} - 2 \alpha_ {1} ^ {2}}
$$

- $1 - (\alpha_{1} + \beta_{1})^{2} - 2\alpha_{1}^{2} > 0$ the condition for the 4-order stationarity of the GARCH(1,1) model

# Forecasting GARCH model

Consider GARCH(1,1) model $\left\{ \begin{array}{l}\sigma_t^2 = \alpha_0 + \alpha_1a_{t - 1}^2 +\beta_1\sigma_{t - 1}^2\\ a_t = \sigma_t\varepsilon_t \end{array} \right.$   
- For 1-step ahead forecast, we have

$$
\sigma_ {h} ^ {2} (1) = E \big (\alpha_ {0} + \alpha_ {1} a _ {h} ^ {2} + \beta_ {1} \sigma_ {h} ^ {2} \big | F _ {h} \big) = \alpha_ {0} + \alpha_ {1} a _ {h} ^ {2} + \beta_ {1} \sigma_ {h} ^ {2}
$$

# Forecasting GARCH model

- For 2-step ahead forecast, we use $a_{t}^{2} = \sigma_{t}^{2}\varepsilon_{t}^{2}$ and rewrite the volatility equation as

$$
\sigma_ {t} ^ {2} = \alpha_ {0} + (\alpha_ {1} + \beta_ {1}) \sigma_ {t - 1} ^ {2} + \alpha_ {1} \sigma_ {t} ^ {2} (\varepsilon_ {t} ^ {2} - 1)
$$

- When $t = h + 1$ , the equation becomes

$$
\sigma_ {h + 2} ^ {2} = \alpha_ {0} + (\alpha_ {1} + \beta_ {1}) \sigma_ {h + 1} ^ {2} + \alpha_ {1} \sigma_ {h + 1} ^ {2} (\varepsilon_ {h + 1} ^ {2} - 1)
$$

- Since $E\left( {{\varepsilon }_{h + 1}^{2} - 1 \mid  {F}_{h}}\right)  = 0$ ,the 2-step ahead volatility forecast

$$
\sigma_ {h} ^ {2} (2) = \alpha_ {0} + (\alpha_ {1} + \beta_ {1}) \sigma_ {h} ^ {2} (1)
$$

# Forecasting GARCH model

In general, we have

$$
\sigma_ {h} ^ {2} (k) = \alpha_ {0} + (\alpha_ {1} + \beta_ {1}) \sigma_ {h} ^ {2} (k - 1)
$$

- This result is the same as that of an ARCH(1) model with ARCH parameter $(\alpha_{1} + \beta_{1})$   
- By repeated substitutions, the k-step ahead forecast can be written as

$$
\sigma_ {h} ^ {2} (k) = \frac {\alpha_ {0} [ 1 - (\alpha_ {1} + \beta_ {1}) ^ {k - 1} ]}{1 - \alpha_ {1} - \beta_ {1}} + (\alpha_ {1} + \beta_ {1}) ^ {k - 1} \sigma_ {h} ^ {2} (1)
$$

- Provided that $\alpha_{1} + \beta_{1} < 1$ , it tends to $\frac{\alpha_{0}}{1 - \alpha_{1} - \beta_{1}}$ when $k$ tends to $\infty$

# Summary of ARCH/GARCH models

# Advantages

They can model heteroscedasticity.   
They can model volatility clustering.   
They can model fat tail property.   
- They can model the evolution of the volatility.

# Weaknesses

- The model assumes that positive and negative shocks have the same effects on volatility because it depends on the square of the previous shocks. In practice, it is well known that price of a financial asset responds differently to positive and negative shocks.   
- GARCH models are likely to over-predict the lower volatilities because they respond slowly to large isolated shocks to the return series.   
- Recent empirical studies of high-frequency financial time series indicate that the tail behavior of GARCH models remains too short even with standardized Student-t innovations.

# Value at Risk (VaR)

# Value at Risk (VaR)

• VaR is the “maximum” loss of an asset over a time horizon (e.g. 1 day, 10 days) with a high confidence (eg. $95\%$ , $99\%$ )   
- For example, what is the maximum daily loss of an investment at $95\%$ confidence?   
A formal definition

- Let X be the PnL distribution (loss is negative, profit is positive)   
- The VaR at level $\alpha \epsilon (0,1)$ is the smallest number $y$ such that the probability that the loss $Y := -X$ does not exceed $y$ is $1 - \alpha$

$$
V a R _ {\alpha} (X) = - \inf \{x \epsilon \mathbb {R}: F _ {X} (x) > \alpha \} = F _ {Y} ^ {- 1} (1 - \alpha)
$$

# VaR Example

Consider an investment of $W_0 = \$ 10,000 in a stock for one month, and the monthly return is R ~ N(0.05, 0.1^2).

(a) What is the probability distribution at the end of month wealth $W_{1} = W_{0}(1 + R)?$   
(b) Calculate $P\left(W_{1}< 9000\right)$   
(c) Find the \(95 \%\)value- at- risk (VaR) on the \(\$ 10,000 investment in one month. i.e. to find VaR such that \(P \left(W _ {0} - W _ {1} < V a R\right) = 0. 9 5\)

# VaR Example - answer

(a) $W_{1} = W_{0}(1 + R) \sim N(10000(1 + 0.05), 10000^{2}0.1^{2}) = N(10500, 1000^{2})$   
(b) $P\left(W_{1}< 9000\right)=P\left(Z< \frac{9000-10500}{1000}\right)=P\left(Z< -1.5\right)=0.0668$   
(c) $P\left(W_{0} - W_{1}< V a R\right) = 0.95$

$$
\begin{array}{l} P (1 0 0 0 0 - (1 0 5 0 0 + 1 0 0 0 Z) <   V a R) = 0. 9 5 \\ P \left(Z > \frac {- V a R - 5 0 0}{1 0 0 0}\right) = 0. 9 5 \\ \frac {- V a R - 5 0 0}{1 0 0 0} = - 1. 6 4 5 \\ V a R = 1 1 4 5 \\ \end{array}
$$

# Relative VaR vs Absolute VaR

![](images/4c826b75e86d7ba3e29658562efc8f113c501df25721b7262487cbf1289cbba8.jpg)  
Distribution of Portfolio Value

# Value-at-Risk (VaR) Estimation Method

- The example above is calculated based on parametric method (i.e. normal distribution)   
- Some common VaR approach used in the industry

1. Parametric VaR (or Variance-Covariance method)   
2. Historical VaR   
3. Hypothetical VaR   
4. Monte Carlo Simulation

# Parametric VaR

- In previous example, the return R is assumed to be normally distributed which is NOT a proper assumption.   
- The maximum loss of an investment cannot exceed -100%   
- In finance, it is more common to assume the log return $R$ follows normal distribution. (i.e. $e^{R}$ follows log-normal distribution)   
- Log normal distribution:

- If $X \sim N(\mu, \sigma^2)$ where $-\infty < X < \infty$   
- Then $Y = e^{X} \sim \operatorname{LogN}(\mu, \sigma^{2})$ where $0 < Y < \infty$   
$E(Y) = \exp \left(\mu +\frac{\sigma^2}{2}\right)$   
$\operatorname{Var}(Y) = \exp (2\mu + \sigma^2)\left[\exp (\sigma^2) - 1\right]$

# Parametric VaR Log-Normal Example

Consider an investment of $W_0 = \$ 10,000 in a stock for one month, and the monthly log return is R ~ N(0.05, 0.1^2).

(a) What is the probability distribution at the end of month wealth $W_{1} = W_{0} e^{R?}$ ?   
(b) Calculate $P\left(W_{1}< 9000\right)$   
(c) Find the \(95 \%\)value- at- risk (VaR) on the \(\$ 10,000 investment in one month. i.e. to find VaR such that \(P \left(W _ {0} - W _ {1} < V a R\right) = 0. 9 5\)

# Parametric VaR Log-Normal Example - answer

(a) $\operatorname{Log}\left( {W}_{1}\right)  = \operatorname{Log}\left( {W}_{0}\right)  + R \sim  N\left( {\operatorname{Log}\left( {W}_{0}\right)  + E\left( R\right) ,\operatorname{Var}\left( R\right) }\right)  = N\left( {\log \left( {{10000}}\right)  + {0.05},{0.1}^{2}}\right)  =$ $N\left( {{9.26},{0.1}^{2}}\right)$

$$
\mathrm {S o} W _ {1} \sim \operatorname {L o g N} (9. 2 6, 0. 1 ^ {2})
$$

(b) $P(W_{1} < 9000) = P(\log (W_{1}) < \log (9000)) = P\left(Z < \frac{\log(9000) - 9.26}{0.1}\right) = P(Z < -1.55) = 0.0605$   
(c) $P\left(W_{0} - W_{1}< V a R\right) = 0.95$

$$
P \left(W _ {1} > 1 0 0 0 0 - V a R\right) = 0. 9 5
$$

$$
P \left(Z > \frac {\log (1 0 0 0 0 - V a R) - 9 . 2 6}{0 . 1}\right) = 0. 9 5
$$

$$
\frac {\log (1 0 0 0 0 - V a R) - 9 . 2 6}{0 . 1} = - 1. 6 4 5
$$

![](images/7e4db99193bc550897ebe0f2b784bfc49436ebcabab70ea7114b9c4d33803066.jpg)

$V a R = 1084.9$

# Historical VaR

Question: Give the previous 10 days’ return series. What’s the $90 \%$ daily VaR?

$$
\{1.1 \%, 2.4 \%, -1.3 \%, -0.4 \%, 3 \%, 2.6 \%, 1.4 \%, -0.8 \%, 0.9 \%, -0.5 \% \}
$$

# Answer:

$$
\{-1.3 \%, -0.8 \%, -0.5 \%, -0.4 \%, 0.9 \%, 1.1 \%, 1.4 \%, 2.4 \%, 2.6 \%, 3 \% \}
$$

sort the series in ascending order   
- Find the smallest number $y$ such that the probability that the loss does not exceed $y$ is $1 - 90\%$   
So $90\%$ worst case is $-0.8\%$   
- Also, the mean is $0.84\%$ .   
- So daily VaR (relative) will be $0.84\% - (-0.8\%) = 1.64\%$ .

# Historical VaR

- For the example above, what would be the $95 \%$ VaR?   
- The worst case lose is …

- $1.3\%$   
- $0.8\%$   
• $0.5^{*}(-1.3\% - 0.8\%) = -1.05\%$ ?

# Stress VaR

- VaR measures based on recent data can miss out extreme situations that could cause severe losses.   
- Stress VaR calculates potential loss for holding the current portfolio during a stress period.   
- A stress period is an actual event happened in the past, such as

Asian Financial Crisis in 1997;   
Dot-com bubbles in 2000;   
- Global Financial crisis in 2008;   
- European Debt Crisis in 2012;   
etc

- In other words, if financial crisis happen again and I do nothing with my portfolio during the whole period, what would be my "maximum" loss?

# Hypothetical VaR

- Hypothetical VaR calculates potential losses based on hypothetical scenarios rather than actual past data.   
- It is commonly used for risk assessment on extreme scenarios that don’t have past reference.   
- In other word, it calculates the "maximum" loss for "what-if" scenarios. eg.

US Fed Fund rate increase to $50\%$ ,   
S&P500 Index drops $90\%$ ,   
HKD and USD unpegged   
0

# Monte Carlo Simulation

- For a large investment portfolio consisting of many securities (eg. stocks, options, futures, currencies, bonds, etc), it involves multiple risk factors that make the risk assessment difficult.   
- Monte Carlo can be used to simulate the portfolio value under different possible paths and movements

![](images/d52beae300afb6b2e182c83582e83eb275ed31a30abbfeb151825dc45028ee22.jpg)

# Monte Carlo example

![](images/5c696aa1c3994195ab2420225411eda0e7661b86e48a33a9049d65692eca29a8.jpg)

Mean stock price at the end of the year: 102.56

5th percentile stock price at the end of the year: 102.00

![](images/b4a63eb50702a456a75160a168e6739084dd29fe2256f813700d32121cf93d21.jpg)

import numpy as np

import matplotlib.pyplot as plt

Parameters

initial_price = 100

num_simulations = 1000

num.days = 252

Approximate trading days in a year

daily_mean = 0.0001

daily_std_dev = 0.0002

Simulate the stock prices

simulatedprices $\equiv$ np.zeros((num_simulations, num_day))

for i in range(num_simulations):

dailyreturns $=$ np.random.normal(daily_mean,daily_std_dev,num_dayse)

price_series = initial_price * np.exp(np cumsum(dailyreturns))

simulated_price[i] = price_series

Calculate mean and 5th percentile at the end of the year

final_price $=$ simulated_price[:,-1]

mean_price = np.mean(final_price)

percentile_5 = np-percentile(final_price, 5)

Output the results

print(f"Mean stock price at the end of the year: {mean_price:.2f}")

print(f"5th percentile stock price at the end of the year: {percentile_5:.2f}")

Plotting the simulations with different colors

plt.figure(figsize=(12,6))

colors = plt.cm.viridis(np.linspace(0, 1, num_simulations)) # Use a colormap for different colors

for i in range(num_simulations):

plt.plot(simulated的价格[i], color=colors[i], alpha=0.5) # Set alpha for transparency

plt title('Monte Carlo Simulations of Daily Stock Price')

plt.xlabel('Days')

plt.ylabel('Stock Price')

plt.grid()

plt show()

# Limitation of VaR

- Assumptions: Assumes normal distribution of returns, which may not hold in reality.   
- Non-linear Risks: VaR does not account for extreme market movements.   
- Time Horizon Sensitivity: VaR can change significantly with the time horizon chosen.   
- Ignore about the loss in the tail distribution

# Other Risk Measures

- Conditional Value-at-Risk(CVaR) or Expected Shortfall (ES) or Tail Value-at-Risk (TVaR) or Conditional Tail Expectation (CTE)

$$
E (L o s s \mid L o s s > V a R)
$$

• Lower Partial Movement (r is the target return)

$$
E ([ \max (0, r - R) ] ^ {2})
$$

# Scenario Analysis

# Scenario Analysis

- Scenario analysis consists of evaluating the portfolio under various extreme but probable states of the world. It includes

1. Sensitivity Testing: Evaluate the portfolio by moving key variables sequentially by a large amount (ignoring correlations).   
2. Evaluate portfolios by creating scenarios of joint (but unusual) movements in key variables.

# Scenario Analysis

# Ways to come up with scenario

1. Prospective Scenarios: hypothetical events (eg. earthquake in Tokyo, Korean reunification, etc)   
2. Factor Push Method: push up or down all risk-factors individually by, say standard deviations, and then compute the changes to the portfolio   
3. Historical scenarios: based on extreme historical event to provide guideline on joint movements   
4. Conditional Scenario Method:

- Denote extreme movements in the key variable by $\mathsf{R}^{*}$   
- Regress other variables (denote by $\mathsf{R}$ ) on $\mathsf{R}^*$ . i.e. $R_{j} = \beta_{0} + \sum_{i}\beta_{ji}R_{i}^{*} + \varepsilon_{j}$   
- Predict $R_{j}$ by $E\left(R_{j} \mid R^{*}\right)$

# Capital Management

# Position Sizing

# Common position sizing strategy

1. Fixed Size   
2. Balance Rescaling   
3. Dollar Risk Approach   
4. Kelly Criterion

# Position Sizing - Fixed Size

- Trade at a fixed quantity every time regardless of balance change   
- Pros: simple to implement   
- Cons:

- Capital will be under utilized when profit keep accumulating   
- Expose to higher risk when balance decreases due to losses

# Position Sizing – Balance Rescaling

- Suppose initial capital is $B_{0}$ and initial trade size is $q_{0}$   
- We can adjust the trade size according to latest balance

$$
q _ {t} = \frac {B _ {t}}{B _ {0}} q _ {0}
$$

- Then round to the closest Tradable lot size   
Pros: dynamic quantity with the same risk level   
- Cons: ignore the win rate of a trade

# Dollar Risk Approach

- Dollar Risk is to risk on each trade only a small percentage of your entire account. It is to prevent your account from going straight to zero in case of a streak of losing trades.   
• For example, your account balance is US $10,000. Assume you want to risk only \(1 \%$ of your balance on each trade. In other word, you want to cap the maximum loss for each individual trade to US\)100.

$$
U S\\(10,000*1\% = US\\)100
$$

- Assuming this trade is EUR/USD, a standard lot has contract size of 100,000 units and therefore every pip has a value of US$10.

$$
1 0 0, 0 0 0 * 0. 0 0 0 1 = U S \mathbb {S} 1 0
$$

# Dollar Risk Approach

• Also suppose you want the stop-loss is 50 pips from the open price. This means that 50 pips are valued US$500 for a standard lot.

$$
U S \mathbb {S} 1 0 * 5 0 p i p s = U S \mathbb {S} 5 0 0
$$

- Since you want to only risk $100, while maintaining your stop-loss 50 pips away, then your position size should be 0.2 standard lot.

$$
\$ 100 / (\$ 10 * 50) = 0.2
$$

# Dollar Risk Approach

- To summarize, we need these inputs to calculate the trade size

- Current account balance   
Percentage of risk for a single trade   
- Stop loss in pips/points/ticks   
- Pip/point/tick value

Pros: maximum loss of each trade is fixed   
- Cons: only applicable to strategy that have fixed trade-level stop-loss

# Kelly Criterion

- The Kelly Criterion is a formula used to determine the optimal size of a series of bets.   
- It aims to maximize the long term wealth, balancing risk and reward.   
- Formula:

$$
f ^ {*} = \frac {b p - q}{b}
$$

where

- $f^{*} =$ the optimal percentage of capital to put in a bet   
- $b =$ odds received on the wager (net odds)   
p = probability of winning   
• $q =$ probability of losing $(1 - p)$

- Understand the components:

• Odds (b): If you bet $1 and win$ b, you gain $b. i.e. win amount / loss amount   
- Probability (p): Your estimated chance of winning the bet.

- If $f^{*}$ is calculated to be negative, then we shouldn't bet at all

# Utility Function U(W)

- A measure of satisfaction or value derived from consumption or wealth.   
- Types of Utility Functions:

1. Linear

$U(W) \propto W$

2. Quadratic

$U(W) \propto W^{2}$

3. Logarithmic

- $U(W) \propto \log (W)$

# Log Utility

![](images/9788e04b41530746f52c0e5284890e91cc2cd9d8455c8d2c379a2308224ed4a4.jpg)

# Proof of Kelly Criterion

- Objective: maximize the log-utility function   
- Denote

W be the initial wealth   
- $f$ be the fraction of wealth bet   
W' be the wealth after the bet

- Then, $W'$ can be expressed as

$$
W ^ {\prime} = \left\{ \begin{array}{l l} W (1 + b f), & \text {w i t h p r o b a b i t y o f p} \\ W (1 - f), & \text {w i t h p r o b a b i t y o f q} \end{array} \right.
$$

Expected Log Wealth:

$$
\begin{array}{l} E (\log (W ^ {\prime})) = p \log \bigl (W (1 + b f) \bigr) + q \log \bigl (W (1 - f) \bigr) \\ = p \log (W) + p \log (1 + b f) + q \log (W) + q \log (1 - f) \\ = \log (W) + p \log (1 + b f) + q \log (1 - f) \\ \end{array}
$$

# Proof of Kelly Criterion

- Differentiate with respect to $f$ :

$$
\frac {\partial E (\log (W ^ {\prime}))}{\partial f} = \frac {p b}{1 + b f} - \frac {q}{1 - f}
$$

- Set the derivative to zero and substitute $q = 1 - p$ , then solve for $f$ ,

$$
\frac {p b}{1 + b f} = \frac {1 - p}{1 - f}
$$

$$
p b - p b f = 1 - p + b f - p b f
$$

$$
f = \frac {b p - (1 - p)}{b} = \frac {b p - q}{b}
$$

# Kelly Criterion Example

![](images/63c51beb56c279feb4c518e08aeab0b4c798e17b5744e1e63a0f17c159fdc232.jpg)

import numpy as np

import matplotlib.pyplot as plt

Parameters

np.random.seed(42) # For reproducibility

num_trades = 1000 # Number of trades

win_rate = 0.55 # Probability of winning

avg_win = 0.1 # Average win (10% return)

avg_loss = -0.05 # Average loss (-5% return) # Kelly Criterion Calculation

def calculate_kelly(p, b):

q = 1 - p

return $(\mathsf{b}^{*}\mathsf{p} - \mathsf{q}) / \mathsf{b}$

Simulate Trades

def simulate_trades(num_trades, win_rate, avg_win, avg_loss):

kelly_fraction = calculate_kelly(win_rate, avg_win / abs(avg_loss))

capital = 1.0 # Starting capital

capital_history = [capital]

for_in range(num_trades):

if np.random.randint() < win_rate: # Win

capital += capital * kelly_fraction * avg_win

else:#Loss

capital $+ =$ capital \* kelly_fraction \* avg_loss

capital_history.append(capital)

return capital_history

Run the simulation

capital_history = simulate_trades(num_trades, win_rate, avg_win, avg_loss)

Plot the results

plt.figure(figsize=(12,6))

plt.plot(capital_history, label='Capital Growth', color='blue')

plt.title('Capital Growth Simulation Using Kelly Criterion')

plt.xlabel('Number of Trades')

plt. ylabel('Capital')

plt(axhline(y=1, color='red', linestyle='---', label='Initial Capital')

pltlegend()

plt grid()

plt.show()

# Kelly Criterion Strategy

# - Backtest Setting

Instrument: SPYUS   
Data Interval: 1-day   
- Start Date: 2024-01   
End Date: 2024-07   
Initial Capital: US$100,000

from AlgoAPI import AlgoAPIUtil, AlgoAPI_Backtest

from datetime import datetime, timedelta

def calculate_kelly(p, b):

q = 1 - p

return $(\mathsf{b}^{*}\mathsf{p} - \mathsf{q}) / \mathsf{b}$

class AlgoEvent:

def __init__(self):

self timer = datetime(1970, 1, 1)

self.last_close = None

selfreturns $\equiv$ []

self.numObs = 30

def start(self, mEvt):

self EVT = AlgoAPI_Backtest.AlgoEvtHandler(self, mEvt)

get lot size

self.instrument = mEvt['subscribeList'][0]

self contractSize = self.evt.getContractSpec(self.instrument)["contractSize"]

self.evt.start()

def on_markdatafeed(self, md, ab):

if md.timestamp $> =$ self.timetherimedelta(hours=24):

self_timer = md.timestamp

update return series

if self.last_close != None:

selfreturns.append Md lastPrice/self.last_close-1)

self.last_close = md(lastPrice

keep only the recent observations

if len(selfreturns) $\rightharpoondown$ self.numObs:

selfreturns $=$ selfreturns[-self numObs:]

calculate Kelly Criterion

win_rate = sum([1 for r in selfreturns if r > 0]) / len(selfreturns)

avg_win = sum([r for r in selfreturns if r > 0]) / len(selfreturns)

avg_loss = sum([r for r in selfreturns if r < 0]) / len(selfreturns)

b = abs(avg_win/avg_loss)

kelly_fraction = calculate_kelly(win_rate, b)

if kelly_fraction<=0 or kelly_fraction>1: return

round to the closest number of lot

quantity = round(ab['availableBalance'] * kelly_fraction / (md.lastPrice * self_contractSize), 0)

if quantity>0:

order $=$ AlgoAPIUtil.OrderObject(

openclose = "open",

instrument = md.instrument,

buysell $= 1$

volume $=$ quantity,

ordertype $= 0$ # market order

holdtime $= 24^{*}60^{*}60$ # hold for 1 day only

(1) ${\mathrm{F}}_{2}$ 与黄色亲本杂交,后代有两种表现型。 现状为_____,黄色为_____;红色为_____;判断依据是：

self.evt.sendOrder(order)

# Kelly Criterion Example

Why performance not good?

![](images/9eb0a4478182df3030aeb1e4a72337c4f0ac04596c9edfb4c33f5b3c1c5ea92e.jpg)

# Kelly Criterion Assumptions

<table><tr><td>Gambling</td><td>Investment</td></tr><tr><td>The risk and reward are determined. (eg. 50% win rate for flipping a coin)</td><td>The chance of winning a trade is a variable</td></tr><tr><td>Each bet has only binary outcomes which are determined. (eg. Every time when if you win, you get $10; if you lose, you lose $10)</td><td>The outcome is return which is a continuous variable</td></tr><tr><td>Each bet is independent</td><td>There could be serial correlation</td></tr></table>

# Kelly Criterion Enhancement

- For risk averse traders, only trade at a fraction of $f^{*}$   
- Define a take-profit and stop-loss level (eg. 20 points from entry price) $\rightarrow$ fix the profit/loss amount of each trade   
Instead of applying Kelly formula directly to the original price, we can apply it to the trading signals generated from other models $\rightarrow$ ensure the trades are independent   
• There are other literature studying the applicability of Kelly Criterion in financial market. For example, this paper (https://sites.math.washington.edu/~morrow/336_20/2016papers/nikhil.pdf) suggests the optimal quantity would be

$$
f ^ {*} = \frac {\mu - r}{\sigma^ {2}}
$$

# Key Takeaways

- Understand the risk management cycle employed by financial institutions   
- Learn about commonly used risk models

ARCH and GARCH model   
- Value-at-Risk (VaR)   
- Stress testing, Scenario analysis

- Learn about common position sizing strategy for capital management

- Fixed Size   
- Balance Rescaling   
- Dollar Risk Approach   
- Kelly Criterion
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
