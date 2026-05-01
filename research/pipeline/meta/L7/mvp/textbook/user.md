请基于以下材料，生成一份 lesson 级 MDX 课本。

目标语言：
zh-CN

lesson_id：
L7

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
# L7: Portfolio Optimization and Performance Measures
Course Code: COMP7415
# Agenda
- Calculating risk and return of an investment portfolio
Correlation effect for diversification
- Minimum variance portfolio
- Efficiency frontier
- Tangency portfolio
- Capital Asset Pricing Model (CAPM)
- Capital Market Line (CML)
Security Market Line (SML)
Commonly used performance measures
- Stop loss and risk limit
# Risk Management Cycle
1. Risk Identification
2. Risk Assessment / Measurement
3. Risk Treatment
4. Risk Monitoring
![](images/79e05f423494a59b8f72adfb80a011ffb25c27e6f2d4fc60467d5150d464c33f.jpg)
# Investment Portfolio
# Investment Portfolio
- There are many different kinds of assets in the market (eg. stocks, commodity, forex, bonds, etc). We can pick various assets for our investment as a portfolio
- Portfolio optimization is the process of selecting the best distribution of assets to maximize returns while minimizing risk
- Investment basically involves
- asset selection
- asset allocation
- In this section, we will discuss the Markowitz Portfolio Theory (or modern portfolio theory) which is a Nobel prize theory in 1990
# Revision of Basic Statistics
- Given that $a$ and $b$ are constants and $X$ and $Y$ are random variables.
$E(a + X) = a + E(X)$
$\cdot E (a \times X) = a \times E (X)$
$\cdot E(X + Y) = E(X) + E(Y)$
- $Var(a + X) = Var(X)$
$\operatorname{Var}(a \times X) = a^{2} \times \operatorname{Var}(X)$
- $\operatorname{Var}\left( {X + Y}\right)  = \operatorname{Var}\left( X\right)  + \operatorname{Var}\left( Y\right)  + 2 \times  \operatorname{Cov}\left( {X,Y}\right)$
$\operatorname{Cov}(a, X) = 0$
$\operatorname{Cov}(a \times X, b \times Y) = a \times b \times \operatorname{Cov}(X, Y)$
# Portfolio Return and Risk
- Suppose a portfolio $P$ contains $n$ assets
- Portfolio return will be
$$
R _ {p} = \sum_ {i = 1} ^ {n} w _ {i} R _ {i}
$$
where $\begin{array}{r}\left\{ \begin{array}{ll}w_{i} = \mathrm{weight~of~the~asset~in~the~portfolio}\\ \sum_{i = 1}^{n}w_{i} = 1 \end{array} \right. \end{array}$
Expected Portfolio Return
$$
E (R _ {p}) = \sum_ {i = 1} ^ {n} w _ {i} E (R _ {i})
$$
- Portfolio Variance
$$
\sigma_ {p} ^ {2} = \sum_ {i = 1} ^ {n} w _ {i} ^ {2} \sigma_ {i} ^ {2} + \sum_ {i = 1} ^ {n} \sum_ {i \neq j} w _ {i} w _ {j} \sigma_ {i j}
$$
where
$$
\begin{array}{l} \sigma_ {p} ^ {2} = \mathrm {v a r i a n c e} \\ \sigma_ {i} ^ {2} = \mathrm {v a r i a n c e} i \\ \sigma_ {i j} = \text {c o v a r i a n c e b e t w e e n a s s e t} I \text {a n d} j \\ \end{array}
$$
# Example
Given 2 risky assets, A and B
• At time 0, \(P_{A,0} = \\)1\) and \(P_{B,0} = \$100\)
- At time 1, you expect that the asset price may go up or down according to the economic conditions
</COVERAGE_CHECKLIST>

<SOURCE_OUTLINE>
# L7: Portfolio Optimization and Performance Measures
Course Code: COMP7415
# Agenda
- Calculating risk and return of an investment portfolio
Correlation effect for diversification
- Minimum variance portfolio
- Efficiency frontier
- Tangency portfolio
- Capital Asset Pricing Model (CAPM)
- Capital Market Line (CML)
Security Market Line (SML)
Commonly used performance measures
- Stop loss and risk limit
# Risk Management Cycle
1. Risk Identification
2. Risk Assessment / Measurement
3. Risk Treatment
4. Risk Monitoring
![](images/79e05f423494a59b8f72adfb80a011ffb25c27e6f2d4fc60467d5150d464c33f.jpg)
# Investment Portfolio
# Investment Portfolio
- There are many different kinds of assets in the market (eg. stocks, commodity, forex, bonds, etc). We can pick various assets for our investment as a portfolio
- Portfolio optimization is the process of selecting the best distribution of assets to maximize returns while minimizing risk
- Investment basically involves
- asset selection
- asset allocation
- In this section, we will discuss the Markowitz Portfolio Theory (or modern portfolio theory) which is a Nobel prize theory in 1990
# Revision of Basic Statistics
- Given that $a$ and $b$ are constants and $X$ and $Y$ are random variables.
$E(a + X) = a + E(X)$
$\cdot E (a \times X) = a \times E (X)$
$\cdot E(X + Y) = E(X) + E(Y)$
- $Var(a + X) = Var(X)$
$\operatorname{Var}(a \times X) = a^{2} \times \operatorname{Var}(X)$
- $\operatorname{Var}\left( {X + Y}\right)  = \operatorname{Var}\left( X\right)  + \operatorname{Var}\left( Y\right)  + 2 \times  \operatorname{Cov}\left( {X,Y}\right)$
$\operatorname{Cov}(a, X) = 0$
$\operatorname{Cov}(a \times X, b \times Y) = a \times b \times \operatorname{Cov}(X, Y)$
# Portfolio Return and Risk
- Suppose a portfolio $P$ contains $n$ assets
- Portfolio return will be
$$
R _ {p} = \sum_ {i = 1} ^ {n} w _ {i} R _ {i}
$$
where $\begin{array}{r}\left\{ \begin{array}{ll}w_{i} = \mathrm{weight~of~the~asset~in~the~portfolio}\\ \sum_{i = 1}^{n}w_{i} = 1 \end{array} \right. \end{array}$
Expected Portfolio Return
$$
E (R _ {p}) = \sum_ {i = 1} ^ {n} w _ {i} E (R _ {i})
$$
- Portfolio Variance
$$
\sigma_ {p} ^ {2} = \sum_ {i = 1} ^ {n} w _ {i} ^ {2} \sigma_ {i} ^ {2} + \sum_ {i = 1} ^ {n} \sum_ {i \neq j} w _ {i} w _ {j} \sigma_ {i j}
$$
where
$$
\begin{array}{l} \sigma_ {p} ^ {2} = \mathrm {v a r i a n c e} \\ \sigma_ {i} ^ {2} = \mathrm {v a r i a n c e} i \\ \sigma_ {i j} = \text {c o v a r i a n c e b e t w e e n a s s e t} I \text {a n d} j \\ \end{array}
$$
# Example
Given 2 risky assets, A and B
• At time 0, \(P_{A,0} = \\)1\) and \(P_{B,0} = \$100\)
- At time 1, you expect that the asset price may go up or down according to the economic conditions
</SOURCE_OUTLINE>

<LESSON_MAP>
{
  "lesson_id": "L7",
  "mode": "guided_story",
  "steps": [
    {
      "concept": "Portfolio return and risk basics",
      "file": "research/pipeline/3-guided_story/L7/step1/step.json",
      "sequence_id": "step1"
    },
    {
      "concept": "Diversification and correlation effect",
      "file": "research/pipeline/3-guided_story/L7/step2/step.json",
      "sequence_id": "step2"
    },
    {
      "concept": "Efficient frontier and tangency portfolio",
      "file": "research/pipeline/3-guided_story/L7/step3/step.json",
      "sequence_id": "step3"
    },
    {
      "concept": "CAPM and Security Market Line",
      "file": "research/pipeline/3-guided_story/L7/step4/step.json",
      "sequence_id": "step4"
    },
    {
      "concept": "Performance measures: Sharpe, Sortino, Treynor, Jensen's Alpha",
      "file": "research/pipeline/3-guided_story/L7/step5/step.json",
      "sequence_id": "step5"
    },
    {
      "concept": "Information ratio, maximum drawdown, and Calmar ratio",
      "file": "research/pipeline/3-guided_story/L7/step6/step.json",
      "sequence_id": "step6"
    },
    {
      "concept": "Stop loss and risk limit control",
      "file": "research/pipeline/3-guided_story/L7/step7/step.json",
      "sequence_id": "step7"
    },
    {
      "concept": "Key takeaways and summary",
      "file": "research/pipeline/3-guided_story/L7/step8/step.json",
      "sequence_id": "step8"
    }
  ]
}

</LESSON_MAP>

<GUIDED_STORY_MANIFEST>
{
  "lesson_id": "L7",
  "mode": "guided_story",
  "steps": [
    {
      "concept": "Portfolio return and risk basics",
      "file": "research/pipeline/3-guided_story/L7/step1/step.json",
      "sequence_id": "step1"
    },
    {
      "concept": "Diversification and correlation effect",
      "file": "research/pipeline/3-guided_story/L7/step2/step.json",
      "sequence_id": "step2"
    },
    {
      "concept": "Efficient frontier and tangency portfolio",
      "file": "research/pipeline/3-guided_story/L7/step3/step.json",
      "sequence_id": "step3"
    },
    {
      "concept": "CAPM and Security Market Line",
      "file": "research/pipeline/3-guided_story/L7/step4/step.json",
      "sequence_id": "step4"
    },
    {
      "concept": "Performance measures: Sharpe, Sortino, Treynor, Jensen's Alpha",
      "file": "research/pipeline/3-guided_story/L7/step5/step.json",
      "sequence_id": "step5"
    },
    {
      "concept": "Information ratio, maximum drawdown, and Calmar ratio",
      "file": "research/pipeline/3-guided_story/L7/step6/step.json",
      "sequence_id": "step6"
    },
    {
      "concept": "Stop loss and risk limit control",
      "file": "research/pipeline/3-guided_story/L7/step7/step.json",
      "sequence_id": "step7"
    },
    {
      "concept": "Key takeaways and summary",
      "file": "research/pipeline/3-guided_story/L7/step8/step.json",
      "sequence_id": "step8"
    }
  ]
}

</GUIDED_STORY_MANIFEST>

<GUIDED_STORY_STEPS>
[
  {
    "lesson_id": "L7",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [
          "portfolio"
        ],
        "id": "s001",
        "introduced_terms": [
          "portfolio"
        ],
        "lines": [
          "市场上有股票、债券、外汇……",
          "你不可能只买一种资产。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "portfolio"
        ],
        "id": "s002",
        "introduced_terms": [
          "portfolio"
        ],
        "lines": [
          "把多种资产放在一起，就构成了一个<term id=\"portfolio\">投资组合</term>。",
          "关键问题：每种资产该买多少？"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "portfolio_optimization"
        ],
        "id": "s003",
        "introduced_terms": [
          "portfolio_optimization"
        ],
        "lines": [
          "这就是<term id=\"portfolio_optimization\">投资组合优化</term>——",
          "在最大化收益的同时，把风险降到最低。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "markowitz_portfolio_theory"
        ],
        "id": "s004",
        "introduced_terms": [
          "markowitz_portfolio_theory"
        ],
        "lines": [
          "这套方法的核心，是<term id=\"markowitz_portfolio_theory\">马科维茨投资组合理论</term>。",
          "它在1990年获得了诺贝尔奖。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s005",
        "introduced_terms": [],
        "lines": [
          "先看一个简单情况：组合里只有两只资产。",
          "假设你买了40%的A和60%的B。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "expected_return"
        ],
        "id": "s006",
        "introduced_terms": [
          "expected_return"
        ],
        "lines": [
          "组合的<term id=\"expected_return\">期望收益率</term>就是各资产收益率的加权平均。",
          "权重加起来等于1。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "formula": {
          "latex": "E(R_p) = w_A \\cdot E(R_A) + w_B \\cdot E(R_B)",
          "spoken": "组合的期望收益率等于资产A的权重乘它的期望收益率，加上资产B的权重乘它的期望收益率。"
        },
        "id": "s007",
        "introduced_terms": [],
        "lines": [
          "公式很简单：",
          "$E(R_p) = w_A \\cdot E(R_A) + w_B \\cdot E(R_B)$"
        ],
        "type": "formula"
      },
      {
        "focus_terms": [
          "portfolio_variance"
        ],
        "id": "s008",
        "introduced_terms": [
          "portfolio_variance"
        ],
        "lines": [
          "但风险不是简单的加权平均。",
          "组合的<term id=\"portfolio_variance\">方差</term>还取决于资产之间的相关性。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "formula": {
          "latex": "\\sigma_p^2 = w_A^2 \\sigma_A^2 + w_B^2 \\sigma_B^2 + 2 w_A w_B \\sigma_{AB}",
          "spoken": "组合方差等于A的权重平方乘A的方差，加B的权重平方乘B的方差，再加两倍的A权重乘B权重乘A和B的协方差。"
        },
        "id": "s009",
        "introduced_terms": [],
        "lines": [
          "两只资产时，组合方差为：",
          "$\\sigma_p^2 = w_A^2 \\sigma_A^2 + w_B^2 \\sigma_B^2 + 2 w_A w_B \\sigma_{AB}$"
        ],
        "type": "formula"
      },
      {
        "exercise": {
          "answers": [
            "11.25"
          ],
          "explanation": "0.5 * 15% + 0.5 * 7.5% = 11.25%。",
          "kind": "fill_in_blank",
          "prompt": "组合期望收益率 = ____ %"
        },
        "focus_terms": [],
        "id": "s010",
        "introduced_terms": [],
        "lines": [
          "如果资产A的期望收益是15%，资产B是7.5%。",
          "你各投一半，组合的期望收益是多少？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step1",
    "source": {
      "plain_text": "Portfolio optimization is the process of selecting the best distribution of assets to maximize returns while minimizing risk. Markowitz Portfolio Theory (modern portfolio theory) is a Nobel prize theory in 1990. Portfolio return: R_p = sum(w_i * R_i). Expected Portfolio Return: E(R_p) = sum(w_i * E(R_i)). Portfolio Variance: sigma_p^2 = sum(w_i^2 * sigma_i^2) + sum_{i != j} w_i w_j sigma_ij.",
      "related": [
        "Portfolio return and risk"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "expected_return": {
        "aliases": [
          "Expected Return"
        ],
        "display": "期望收益率",
        "gloss": "资产或组合未来收益的加权平均值。"
      },
      "markowitz_portfolio_theory": {
        "aliases": [
          "Markowitz Portfolio Theory",
          "Modern Portfolio Theory",
          "MPT"
        ],
        "display": "马科维茨投资组合理论",
        "gloss": "1952年由哈里·马科维茨提出的现代投资组合理论，强调通过分散投资降低风险。"
      },
      "portfolio": {
        "aliases": [
          "Portfolio"
        ],
        "display": "投资组合",
        "gloss": "由多种资产构成的集合。"
      },
      "portfolio_optimization": {
        "aliases": [
          "Portfolio Optimization"
        ],
        "display": "投资组合优化",
        "gloss": "选择最佳资产分配比例，在最大化收益的同时最小化风险的过程。"
      },
      "portfolio_variance": {
        "aliases": [
          "Portfolio Variance"
        ],
        "display": "投资组合方差",
        "gloss": "衡量投资组合收益波动性的指标。"
      }
    }
  },
  {
    "lesson_id": "L7",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [
          "correlation"
        ],
        "id": "s011",
        "introduced_terms": [
          "correlation"
        ],
        "lines": [
          "两只资产之间的<term id=\"correlation\">相关系数</term>，决定了分散风险的效果。",
          "相关系数越低，分散效果越好。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s012",
        "introduced_terms": [],
        "lines": [
          "当相关系数不是特别高时，",
          "你可以找到一个组合，收益比B高，风险却比B低。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "diversification"
        ],
        "id": "s013",
        "introduced_terms": [
          "diversification"
        ],
        "lines": [
          "这就是<term id=\"diversification\">风险分散</term>的力量。",
          "不把鸡蛋放在同一个篮子里。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s014",
        "introduced_terms": [],
        "lines": [
          "如果两只资产完全正相关（ρ=1），分散无效。",
          "如果完全负相关（ρ=-1），理论上可以把风险降到零。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "minimum_variance_portfolio"
        ],
        "id": "s015",
        "introduced_terms": [
          "minimum_variance_portfolio"
        ],
        "lines": [
          "在所有可能的组合中，风险最低的那个被称为",
          "<term id=\"minimum_variance_portfolio\">最小方差组合</term>。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s016",
        "introduced_terms": [],
        "lines": [
          "它的权重可以通过求导得到。",
          "当导数等于零时，方差最小。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "formula": {
          "latex": "w = \\frac{-\\sigma_B(\\rho \\sigma_A - \\sigma_B)}{\\sigma_A^2 + \\sigma_B^2 - 2\\rho \\sigma_A \\sigma_B}",
          "spoken": "最小方差组合中资产A的权重等于负的资产B标准差乘以相关系数乘资产A标准差减资产B标准差，除以资产A方差加资产B方差减两倍相关系数乘资产A标准差乘资产B标准差。"
        },
        "id": "s017",
        "introduced_terms": [],
        "lines": [
          "最小方差组合的权重公式：",
          "$w = \\frac{-\\sigma_B(\\rho \\sigma_A - \\sigma_B)}{\\sigma_A^2 + \\sigma_B^2 - 2\\rho \\sigma_A \\sigma_B}$"
        ],
        "type": "formula"
      },
      {
        "exercise": {
          "answer": 0,
          "explanation": "当ρ=1时，组合标准差就是各资产标准差的加权平均，没有分散效果。",
          "kind": "single_choice",
          "options": [
            "各资产标准差的加权平均",
            "小于各资产标准差的加权平均",
            "等于零",
            "无法确定"
          ],
          "prompt": "当两只资产完全正相关时，组合的标准差等于："
        },
        "focus_terms": [],
        "id": "s018",
        "introduced_terms": [],
        "lines": [
          "如果ρ=1，组合的风险会怎样？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step2",
    "source": {
      "plain_text": "When the correlation is not very high, with appropriate weightings, the portfolio return can be higher than that of asset B but with a lower portfolio variance than that of asset B. This is called risk diversification. If the 2 assets are more negatively correlated, it has better diversification effect. Minimum Variance Portfolio: the portfolio with the lowest standard deviation among all possible portfolios.",
      "related": [
        "2 assets Portfolio - Correlation Effect",
        "Minimum Variance Portfolio"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "correlation": {
        "aliases": [
          "Correlation Coefficient",
          "ρ"
        ],
        "display": "相关系数",
        "gloss": "衡量两个资产收益率之间线性关系的指标，取值范围在-1到1之间。"
      },
      "diversification": {
        "aliases": [
          "Diversification"
        ],
        "display": "风险分散",
        "gloss": "通过持有多种不完美正相关的资产来降低整体组合风险。"
      },
      "minimum_variance_portfolio": {
        "aliases": [
          "Minimum Variance Portfolio",
          "MVP",
          "Global Minimum Variance Portfolio"
        ],
        "display": "最小方差组合",
        "gloss": "在所有可能的投资组合中，风险（标准差）最低的那个组合。"
      }
    }
  },
  {
    "lesson_id": "L7",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [],
        "id": "s019",
        "introduced_terms": [],
        "lines": [
          "把所有可能的组合画在风险-收益图上，",
          "会形成一片区域——机会集。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "efficient_frontier"
        ],
        "id": "s020",
        "introduced_terms": [
          "efficient_frontier"
        ],
        "lines": [
          "机会集的上边缘，就是<term id=\"efficient_frontier\">有效前沿</term>。",
          "在相同风险下，它提供最高的收益。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s021",
        "introduced_terms": [],
        "lines": [
          "不在有效前沿上的组合，都是低效的。",
          "同样的风险，收益却更低，应该避开。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s022",
        "introduced_terms": [],
        "lines": [
          "如果市场上存在无风险资产（比如国债），",
          "情况会变得更有趣。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "tangency_portfolio"
        ],
        "id": "s023",
        "introduced_terms": [
          "tangency_portfolio"
        ],
        "lines": [
          "从无风险利率出发，画一条与有效前沿相切的直线。",
          "切点就是<term id=\"tangency_portfolio\">切点组合</term>。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "capital_market_line"
        ],
        "id": "s024",
        "introduced_terms": [
          "capital_market_line"
        ],
        "lines": [
          "这条直线被称为<term id=\"capital_market_line\">资本市场线</term>。",
          "所有理性投资者都会在这条线上选择自己的组合。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "formula": {
          "latex": "E(R_p) = R_f + \\frac{E(R_M) - R_f}{\\sigma_M} \\sigma_p",
          "spoken": "组合的期望收益等于无风险利率加上市场风险溢价除以市场标准差再乘以组合标准差。"
        },
        "id": "s025",
        "introduced_terms": [],
        "lines": [
          "资本市场线的公式：",
          "$E(R_p) = R_f + \\frac{E(R_M) - R_f}{\\sigma_M} \\sigma_p$"
        ],
        "type": "formula"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "切点组合是最大化夏普比率（(μ_p - R_f)/σ_p）得到的。",
          "kind": "single_choice",
          "options": [
            "组合收益",
            "夏普比率",
            "组合方差",
            "相关系数"
          ],
          "prompt": "求解切点组合时，目标函数是最大化："
        },
        "focus_terms": [],
        "id": "s026",
        "introduced_terms": [],
        "lines": [
          "切点组合的权重可以通过最大化什么来求解？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step3",
    "source": {
      "plain_text": "Efficient Frontier: Portfolios yielding the highest expected return for a specific variance. Tangency Portfolio: If risk-free asset is available, we can draw a tangent line between the risk-free rate and the efficient frontier. Capital Market Line (CML): For any efficient portfolio on CML, E(R_p) = R_f + (E(R_M) - R_f)/sigma_M * sigma_p.",
      "related": [
        "Efficient Frontier",
        "Tangency Portfolio",
        "Capital Market Line (CML)"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "capital_market_line": {
        "aliases": [
          "Capital Market Line",
          "CML"
        ],
        "display": "资本市场线",
        "gloss": "连接无风险利率与市场组合的直线，代表有效投资组合的风险收益关系。"
      },
      "efficient_frontier": {
        "aliases": [
          "Efficient Frontier",
          "Efficient Set"
        ],
        "display": "有效前沿",
        "gloss": "在给定风险水平下能获得最高预期收益的投资组合集合。"
      },
      "tangency_portfolio": {
        "aliases": [
          "Tangency Portfolio"
        ],
        "display": "切点组合",
        "gloss": "无风险资产与有效前沿相切处的风险资产组合，是所有理性投资者持有的最优风险组合。"
      }
    }
  },
  {
    "lesson_id": "L7",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [],
        "id": "s027",
        "introduced_terms": [],
        "lines": [
          "当资产数量很多时，计算协方差矩阵会变得非常复杂。",
          "需要一种更简洁的模型。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "capital_asset_pricing_model"
        ],
        "id": "s028",
        "introduced_terms": [
          "capital_asset_pricing_model"
        ],
        "lines": [
          "这就是<term id=\"capital_asset_pricing_model\">资本资产定价模型</term>。",
          "它用一个简单的线性关系描述资产收益。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "beta"
        ],
        "id": "s029",
        "introduced_terms": [
          "beta"
        ],
        "lines": [
          "CAPM的核心是<term id=\"beta\">贝塔系数</term>。",
          "它衡量资产对市场波动的敏感度。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "formula": {
          "latex": "E(R_i) = R_f + \\beta_i (E(R_M) - R_f)",
          "spoken": "资产的期望收益等于无风险利率加上贝塔系数乘以市场风险溢价。"
        },
        "id": "s030",
        "introduced_terms": [],
        "lines": [
          "CAPM公式：",
          "$E(R_i) = R_f + \\beta_i (E(R_M) - R_f)$"
        ],
        "type": "formula"
      },
      {
        "focus_terms": [],
        "id": "s031",
        "introduced_terms": [],
        "lines": [
          "贝塔的计算：",
          "$\\beta_i = \\frac{\\sigma_{iM}}{\\sigma_M^2}$"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s032",
        "introduced_terms": [],
        "lines": [
          "如果β > 1，资产比市场更激进。",
          "如果β < 1，资产比市场更保守。",
          "市场的β等于1。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "security_market_line"
        ],
        "id": "s033",
        "introduced_terms": [
          "security_market_line"
        ],
        "lines": [
          "CAPM的图形就是<term id=\"security_market_line\">证券市场线</term>。",
          "所有资产都应该落在这条线上。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s034",
        "introduced_terms": [],
        "lines": [
          "落在线上方，说明资产被低估（α > 0）。",
          "落在下方，说明资产被高估（α < 0）。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answers": [
            "13.5"
          ],
          "explanation": "3% + 1.5 * (10% - 3%) = 13.5%。",
          "kind": "fill_in_blank",
          "prompt": "期望收益 = ____ %"
        },
        "focus_terms": [],
        "id": "s035",
        "introduced_terms": [],
        "lines": [
          "如果无风险利率是3%，市场收益是10%，某资产的β=1.5。",
          "根据CAPM，它的期望收益是多少？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step4",
    "source": {
      "plain_text": "CAPM model: mu_i = R_f + beta_i (mu_M - R_f). Beta_i = sigma_iM / sigma_M^2. SML is graphical representation of the CAPM. Market portfolio (M) is a portfolio consisting of ALL risky assets in the economy.",
      "related": [
        "Capital Asset Pricing Model (CAPM)",
        "Security Market Line (SML)",
        "Market Portfolio"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "beta": {
        "aliases": [
          "Beta",
          "β"
        ],
        "display": "贝塔系数",
        "gloss": "衡量资产收益对市场收益敏感程度的指标。"
      },
      "capital_asset_pricing_model": {
        "aliases": [
          "Capital Asset Pricing Model",
          "CAPM"
        ],
        "display": "资本资产定价模型",
        "gloss": "描述资产预期收益与市场风险之间关系的线性模型。"
      },
      "market_portfolio": {
        "aliases": [
          "Market Portfolio",
          "M"
        ],
        "display": "市场组合",
        "gloss": "包含经济体中所有风险资产的组合，每种资产的权重按其市值比例分配。"
      },
      "security_market_line": {
        "aliases": [
          "Security Market Line",
          "SML"
        ],
        "display": "证券市场线",
        "gloss": "CAPM的图形表示，展示单个资产的预期收益与其贝塔值的关系。"
      }
    }
  },
  {
    "lesson_id": "L7",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [],
        "id": "s036",
        "introduced_terms": [],
        "lines": [
          "两个基金，一个收益高但波动大，另一个收益低但稳定。",
          "怎么比较谁更好？"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "sharpe_ratio"
        ],
        "id": "s037",
        "introduced_terms": [
          "sharpe_ratio"
        ],
        "lines": [
          "需要风险调整后的收益指标。",
          "第一个常用指标：<term id=\"sharpe_ratio\">夏普比率</term>。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "formula": {
          "latex": "SR_p = \\frac{E(R_p) - R_f}{\\sigma_p}",
          "spoken": "夏普比率等于组合期望收益减去无风险利率，再除以组合标准差。"
        },
        "id": "s038",
        "introduced_terms": [],
        "lines": [
          "夏普比率 = 超额收益 / 总风险",
          "$SR_p = \\frac{E(R_p) - R_f}{\\sigma_p}$"
        ],
        "type": "formula"
      },
      {
        "focus_terms": [],
        "id": "s039",
        "introduced_terms": [],
        "lines": [
          "夏普比率越高越好。",
          "行业里，0.5到1.5很常见，超过2.5就非常优秀了。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s040",
        "introduced_terms": [],
        "lines": [
          "但夏普比率把上涨波动和下跌波动同等对待。",
          "投资者通常更讨厌下跌。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "sortino_ratio"
        ],
        "id": "s041",
        "introduced_terms": [
          "sortino_ratio"
        ],
        "lines": [
          "<term id=\"sortino_ratio\">索提诺比率</term>只考虑下行风险。",
          "它用下行标准差代替总标准差。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "treynor_ratio"
        ],
        "id": "s042",
        "introduced_terms": [
          "treynor_ratio"
        ],
        "lines": [
          "如果组合已经充分分散，非系统性风险几乎为零。",
          "这时应该用<term id=\"treynor_ratio\">特雷诺比率</term>。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "formula": {
          "latex": "TR_p = \\frac{E(R_p) - R_f}{\\beta_p}",
          "spoken": "特雷诺比率等于组合期望收益减去无风险利率，再除以组合的贝塔系数。"
        },
        "id": "s043",
        "introduced_terms": [],
        "lines": [
          "特雷诺比率 = 超额收益 / 系统性风险",
          "$TR_p = \\frac{E(R_p) - R_f}{\\beta_p}$"
        ],
        "type": "formula"
      },
      {
        "focus_terms": [
          "jensens_alpha"
        ],
        "id": "s044",
        "introduced_terms": [
          "jensens_alpha"
        ],
        "lines": [
          "另一个经典指标是<term id=\"jensens_alpha\">詹森阿尔法</term>。",
          "它直接告诉你：基金经理有没有创造超额价值。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "formula": {
          "latex": "\\alpha_p = E(R_p) - [R_f + \\beta_p (E(R_M) - R_f)]",
          "spoken": "詹森阿尔法等于组合的实际收益减去CAPM给出的预期收益。"
        },
        "id": "s045",
        "introduced_terms": [],
        "lines": [
          "詹森阿尔法 = 实际收益 - CAPM预期收益",
          "$\\alpha_p = E(R_p) - [R_f + \\beta_p (E(R_M) - R_f)]$"
        ],
        "type": "formula"
      },
      {
        "exercise": {
          "answers": [
            "0.67"
          ],
          "explanation": "(12% - 2%) / 15% = 0.67。",
          "kind": "fill_in_blank",
          "prompt": "夏普比率 = ____"
        },
        "focus_terms": [],
        "id": "s046",
        "introduced_terms": [],
        "lines": [
          "某基金年化收益12%，无风险利率2%，标准差15%。",
          "它的夏普比率是多少？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step5",
    "source": {
      "plain_text": "Sharpe Ratio: SR_p = (E(R_p) - R_f)/sigma_p. Sortino Ratio: similar but focuses on downside risk. Treynor Ratio: (E(R_p) - R_f)/beta_p. Jensen's Alpha: alpha_p = E(R_p) - [R_f + beta_p (E(R_M) - R_f)].",
      "related": [
        "Performance Measures"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "jensens_alpha": {
        "aliases": [
          "Jensen's Alpha",
          "α"
        ],
        "display": "詹森阿尔法",
        "gloss": "实际收益与CAPM预期收益之间的差值。"
      },
      "sharpe_ratio": {
        "aliases": [
          "Sharpe Ratio",
          "SR"
        ],
        "display": "夏普比率",
        "gloss": "衡量每单位总风险所获得的超额收益。"
      },
      "sortino_ratio": {
        "aliases": [
          "Sortino Ratio"
        ],
        "display": "索提诺比率",
        "gloss": "类似夏普比率，但只考虑下行风险。"
      },
      "treynor_ratio": {
        "aliases": [
          "Treynor Ratio",
          "TR"
        ],
        "display": "特雷诺比率",
        "gloss": "衡量每单位系统性风险（贝塔）所获得的超额收益。"
      }
    }
  },
  {
    "lesson_id": "L7",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [],
        "id": "s047",
        "introduced_terms": [],
        "lines": [
          "有时我们想跟一个具体的基准比较，而不是无风险利率。",
          "比如跟沪深300指数比。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "information_ratio"
        ],
        "id": "s048",
        "introduced_terms": [
          "information_ratio"
        ],
        "lines": [
          "这时用<term id=\"information_ratio\">信息比率</term>。",
          "它衡量的是相对于基准的超额收益，除以跟踪误差。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "formula": {
          "latex": "IR = \\frac{E(R_p - R_b)}{\\sigma(R_p - R_b)}",
          "spoken": "信息比率等于组合与基准收益差值的期望，除以这个差值的标准差。"
        },
        "id": "s049",
        "introduced_terms": [],
        "lines": [
          "信息比率 = 超额收益 / 跟踪误差",
          "$IR = \\frac{E(R_p - R_b)}{\\sigma(R_p - R_b)}$"
        ],
        "type": "formula"
      },
      {
        "focus_terms": [],
        "id": "s050",
        "introduced_terms": [],
        "lines": [
          "信息比率超过0.5算不错，超过1.0算非常优秀。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "maximum_drawdown"
        ],
        "id": "s051",
        "introduced_terms": [
          "maximum_drawdown"
        ],
        "lines": [
          "另一个直观的风险指标是<term id=\"maximum_drawdown\">最大回撤</term>。",
          "它告诉你：历史上最惨的时候亏了多少。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "formula": {
          "latex": "MDD = \\frac{P_{peak} - P_{trough}}{P_{peak}}",
          "spoken": "最大回撤等于最高点减去最低点，再除以最高点。"
        },
        "id": "s052",
        "introduced_terms": [],
        "lines": [
          "最大回撤 = (峰值 - 谷值) / 峰值",
          "$MDD = \\frac{P_{peak} - P_{trough}}{P_{peak}}$"
        ],
        "type": "formula"
      },
      {
        "focus_terms": [
          "calmar_ratio"
        ],
        "id": "s053",
        "introduced_terms": [
          "calmar_ratio"
        ],
        "lines": [
          "把年化收益和最大回撤结合起来，就得到<term id=\"calmar_ratio\">卡尔玛比率</term>。",
          "它衡量每单位最大回撤能带来多少收益。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "formula": {
          "latex": "Calmar\\ Ratio = \\frac{Annualized\\ Return}{|Max.\\ Drawdown|}",
          "spoken": "卡尔玛比率等于年化收益除以最大回撤的绝对值。"
        },
        "id": "s054",
        "introduced_terms": [],
        "lines": [
          "卡尔玛比率 = 年化收益 / |最大回撤|"
        ],
        "type": "formula"
      },
      {
        "exercise": {
          "answers": [
            "46.67"
          ],
          "explanation": "(150 - 80) / 150 = 46.67%。",
          "kind": "fill_in_blank",
          "prompt": "最大回撤 = ____ %"
        },
        "focus_terms": [],
        "id": "s055",
        "introduced_terms": [],
        "lines": [
          "一个组合净值从100涨到150，然后跌到80。",
          "最大回撤是多少？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step6",
    "source": {
      "plain_text": "Information Ratio: IR = E(R_p - R_b)/sigma(R_p - R_b). Maximum Drawdown: MDD = (P_peak - P_trough)/P_peak. Calmar Ratio = Annualized Return / |Max. Drawdown|.",
      "related": [
        "Information Ratio",
        "Maximum Drawdown",
        "Calmar Ratio"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "calmar_ratio": {
        "aliases": [
          "Calmar Ratio"
        ],
        "display": "卡尔玛比率",
        "gloss": "年化收益与最大回撤的比值。"
      },
      "information_ratio": {
        "aliases": [
          "Information Ratio",
          "IR"
        ],
        "display": "信息比率",
        "gloss": "衡量组合相对于基准的主动管理能力。"
      },
      "maximum_drawdown": {
        "aliases": [
          "Maximum Drawdown",
          "MDD"
        ],
        "display": "最大回撤",
        "gloss": "从历史最高点到随后最低点的最大跌幅。"
      }
    }
  },
  {
    "lesson_id": "L7",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [],
        "id": "s056",
        "introduced_terms": [],
        "lines": [
          "策略再好，也要有风险控制。",
          "否则一次大亏损就可能出局。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "stop_loss"
        ],
        "id": "s057",
        "introduced_terms": [
          "stop_loss"
        ],
        "lines": [
          "最基本的工具是<term id=\"stop_loss\">止损</term>。",
          "设定一个价格，到了就自动平仓。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s058",
        "introduced_terms": [],
        "lines": [
          "固定止损：比如100元买入，设止损90元。",
          "最大亏损锁定在10元。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "trailing_stop_loss"
        ],
        "id": "s059",
        "introduced_terms": [
          "trailing_stop_loss"
        ],
        "lines": [
          "<term id=\"trailing_stop_loss\">移动止损</term>更聪明。",
          "价格涨，止损线也跟着上移，但不会下移。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s060",
        "introduced_terms": [],
        "lines": [
          "比如设5%的移动止损。",
          "价格从100涨到110，止损线就上移到104.5。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "portfolio_stop_loss"
        ],
        "id": "s061",
        "introduced_terms": [
          "portfolio_stop_loss"
        ],
        "lines": [
          "如果同时运行多个策略，还需要<term id=\"portfolio_stop_loss\">组合止损</term>。",
          "当总净值从高点回撤一定比例时，全部清仓。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s062",
        "introduced_terms": [],
        "lines": [
          "风控部门还会设置风险限额：",
          "日止损、周止损、月止损、VaR限额等。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 0,
          "explanation": "移动止损会随着价格上涨而上移，从而锁定已实现的利润。",
          "kind": "single_choice",
          "options": [
            "能锁定已获得的利润",
            "能承受更大的亏损",
            "不需要设置参数",
            "适用于所有市场"
          ],
          "prompt": "移动止损相比固定止损的最大优势是："
        },
        "focus_terms": [],
        "id": "s063",
        "introduced_terms": [],
        "lines": [
          "移动止损的主要好处是什么？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step7",
    "source": {
      "plain_text": "Stop Loss: minimizes losses on losing trades. Types: Fixed Stop Loss, Trailing Stop Loss, Dollar Risk Approach, Portfolio level Stop Loss. Risk Limit Stoploss: Daily, Weekly, Monthly, Quarterly, Yearly stop loss.",
      "related": [
        "Stop Loss & Risk Limit Control"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "portfolio_stop_loss": {
        "aliases": [
          "Portfolio Stop Loss"
        ],
        "display": "组合止损",
        "gloss": "基于整个投资组合净值的止损机制，作为第二层保护。"
      },
      "stop_loss": {
        "aliases": [
          "Stop Loss",
          "SL"
        ],
        "display": "止损",
        "gloss": "当价格达到预设水平时自动平仓以限制损失的指令。"
      },
      "trailing_stop_loss": {
        "aliases": [
          "Trailing Stop Loss"
        ],
        "display": "移动止损",
        "gloss": "一种动态止损，随价格上升而自动上移，锁定利润。"
      }
    }
  },
  {
    "lesson_id": "L7",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [],
        "id": "s064",
        "introduced_terms": [],
        "lines": [
          "来回顾一下这节课的核心。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s065",
        "introduced_terms": [],
        "lines": [
          "组合的收益是加权平均，但风险取决于资产间的相关性。",
          "相关系数越低，分散效果越好。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s066",
        "introduced_terms": [],
        "lines": [
          "有效前沿给出了最优的风险收益组合。",
          "加入无风险资产后，切点组合和资本市场线成为新基准。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s067",
        "introduced_terms": [],
        "lines": [
          "CAPM用贝塔系数简洁地描述了资产与市场的关系。",
          "证券市场线是它的图形化表达。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s068",
        "introduced_terms": [],
        "lines": [
          "评价一个策略好不好，不能只看收益。",
          "夏普、索提诺、特雷诺、詹森阿尔法……各有适用场景。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s069",
        "introduced_terms": [],
        "lines": [
          "最后，别忘了风险控制。",
          "止损和风险限额是保护资本的底线。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s070",
        "introduced_terms": [],
        "lines": [
          "一句话带走：",
          "投资不是选最好的资产，而是选最好的组合。"
        ],
        "type": "narration"
      }
    ],
    "sequence_id": "step8",
    "source": {
      "plain_text": "Key Takeaways: We learnt how to calculate the overall risk and return of an investment portfolio. The lower the correlation, the better the diversification effect. Efficiency Frontier represents the set of optimal and feasible solutions for asset allocation. We learnt how to derive the Minimum Variance Portfolio, and Tangency portfolio. Capital Asset Pricing Model (CAPM) is a simplified model to relate an asset and the market. Different performance measures are introduced for investment comparison. Introduce different stop loss and risk limit control mechanism.",
      "related": [
        "Key Takeaways"
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
      "coverage_tag": "capital_asset_pricing_model",
      "covered_by": [
        "qf_flash_capm_formula",
        "qf_flash_capm_interpret",
        "qf_quiz_capm_calc",
        "qf_quiz_capm_interpret",
        "qf_long_capm_apply"
      ],
      "description": "CAPM模型：E(R_i) = R_f + β_i (E(R_M) - R_f)，用于描述资产预期收益与市场风险的关系。"
    },
    {
      "coverage_tag": "beta_coefficient",
      "covered_by": [
        "qf_flash_beta_formula",
        "qf_flash_beta_interpret",
        "qf_quiz_beta_calc",
        "qf_quiz_beta_interpret"
      ],
      "description": "贝塔系数：β_i = σ_iM / σ_M^2，衡量资产对市场波动的敏感度。β>1激进，β<1保守，β=1与市场同步。"
    },
    {
      "coverage_tag": "security_market_line",
      "covered_by": [
        "qf_flash_sml_definition",
        "qf_quiz_sml_interpret",
        "qf_long_sml_analysis"
      ],
      "description": "证券市场线(SML)：CAPM的图形表示，所有资产应落在这条线上。线上方α>0（低估），下方α<0（高估）。"
    },
    {
      "coverage_tag": "market_portfolio",
      "covered_by": [
        "qf_flash_market_portfolio",
        "qf_quiz_market_portfolio"
      ],
      "description": "市场组合(M)：包含经济体中所有风险资产的组合，每种资产按市值比例分配。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "capital_asset_pricing_model",
      "coverage_tags": [
        "capital_asset_pricing_model"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_capm_formula",
      "learning_goal": "学生能准确回忆CAPM公式及其各符号含义。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "CAPM公式的结构和每个变量的含义。",
      "term_refs": [
        {
          "display": "资本资产定价模型",
          "en": "Capital Asset Pricing Model (CAPM)"
        }
      ],
      "variants": [
        {
          "back": "E(R_i) = R_f + β_i (E(R_M) - R_f)。E(R_i)是资产i的期望收益，R_f是无风险利率，β_i是资产i的贝塔系数，E(R_M)是市场组合的期望收益。",
          "estimated_seconds": 10,
          "explanation": "CAPM公式是资产定价的基础，描述了期望收益与市场风险之间的线性关系。",
          "front": "写出CAPM公式，并说明每个符号代表什么。",
          "question_id": "q_flash_capm_formula_v1"
        },
        {
          "back": "市场风险溢价，即市场组合期望收益超过无风险利率的部分。",
          "estimated_seconds": 6,
          "explanation": "市场风险溢价是投资者承担市场风险所要求的额外回报。",
          "front": "在CAPM公式中，E(R_M) - R_f 代表什么？",
          "question_id": "q_flash_capm_formula_v2"
        },
        {
          "back": "无风险利率（R_f）和风险溢价（β_i × 市场风险溢价）。",
          "estimated_seconds": 6,
          "explanation": "无风险利率是时间价值，风险溢价是对承担系统性风险的补偿。",
          "front": "根据CAPM，一个资产的期望收益由哪两部分组成？",
          "question_id": "q_flash_capm_formula_v3"
        }
      ]
    },
    {
      "concept_key": "capital_asset_pricing_model",
      "coverage_tags": [
        "capital_asset_pricing_model"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_capm_interpret",
      "learning_goal": "学生能解释CAPM中贝塔系数的含义及其对期望收益的影响。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "贝塔系数如何影响资产的期望收益。",
      "term_refs": [
        {
          "display": "资本资产定价模型",
          "en": "Capital Asset Pricing Model (CAPM)"
        }
      ],
      "variants": [
        {
          "back": "E(R_i) = R_f。因为β=0表示资产与市场无相关性，不承担系统性风险，所以只获得无风险收益。",
          "estimated_seconds": 8,
          "explanation": "β=0的资产（如国债）没有市场风险，因此期望收益等于无风险利率。",
          "front": "如果β=0，根据CAPM，资产的期望收益是多少？为什么？",
          "question_id": "q_flash_capm_interpret_v1"
        },
        {
          "back": "E(R_i) = 3% + 2 × 5% = 13%。",
          "estimated_seconds": 10,
          "explanation": "β=2意味着资产对市场波动非常敏感，因此需要更高的风险溢价补偿。",
          "front": "如果β=2，市场风险溢价为5%，无风险利率为3%，根据CAPM，该资产的期望收益是多少？",
          "question_id": "q_flash_capm_interpret_v2"
        }
      ]
    },
    {
      "concept_key": "beta_coefficient",
      "coverage_tags": [
        "beta_coefficient"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_beta_formula",
      "learning_goal": "学生能准确回忆贝塔系数的计算公式。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "贝塔系数的数学定义。",
      "term_refs": [
        {
          "display": "贝塔系数",
          "en": "Beta Coefficient (β)"
        }
      ],
      "variants": [
        {
          "back": "β_i = σ_iM / σ_M^2，其中σ_iM是资产i与市场组合的协方差，σ_M^2是市场组合的方差。",
          "estimated_seconds": 8,
          "explanation": "贝塔系数衡量资产收益对市场收益的敏感程度。",
          "front": "写出贝塔系数的计算公式。",
          "question_id": "q_flash_beta_formula_v1"
        },
        {
          "back": "分子是资产与市场的协方差（σ_iM），分母是市场的方差（σ_M^2）。",
          "estimated_seconds": 6,
          "explanation": "协方差衡量两者同向变动的程度，方差衡量市场自身的波动性。",
          "front": "贝塔系数公式中的分子和分母分别代表什么？",
          "question_id": "q_flash_beta_formula_v2"
        }
      ]
    },
    {
      "concept_key": "beta_coefficient",
      "coverage_tags": [
        "beta_coefficient"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_beta_interpret",
      "learning_goal": "学生能解释不同贝塔值对应的资产特征。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "β>1、β=1、β<1分别代表什么。",
      "term_refs": [
        {
          "display": "贝塔系数",
          "en": "Beta Coefficient (β)"
        }
      ],
      "variants": [
        {
          "back": "比市场更激进（更敏感）。市场上涨时涨得更多，市场下跌时跌得更多。",
          "estimated_seconds": 6,
          "explanation": "β>1表示资产具有更高的系统性风险，波动性大于市场。",
          "front": "β>1的资产相对于市场有什么特征？",
          "question_id": "q_flash_beta_interpret_v1"
        },
        {
          "back": "比市场更保守（更不敏感）。市场波动时，该资产的波动幅度较小。",
          "estimated_seconds": 6,
          "explanation": "β<1表示资产具有较低的系统性风险，波动性小于市场。",
          "front": "β<1的资产相对于市场有什么特征？",
          "question_id": "q_flash_beta_interpret_v2"
        },
        {
          "back": "市场的β=1。",
          "estimated_seconds": 4,
          "explanation": "市场组合与自身的协方差等于其方差，所以β=1。",
          "front": "市场的贝塔系数是多少？",
          "question_id": "q_flash_beta_interpret_v3"
        }
      ]
    },
    {
      "concept_key": "security_market_line",
      "coverage_tags": [
        "security_market_line"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_sml_definition",
      "learning_goal": "学生能准确描述证券市场线的含义和用途。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "SML的定义及其与CAPM的关系。",
      "term_refs": [
        {
          "display": "证券市场线",
          "en": "Security Market Line (SML)"
        }
      ],
      "variants": [
        {
          "back": "SML是CAPM的图形表示，横轴为贝塔系数，纵轴为期望收益，所有资产都应落在这条直线上。",
          "estimated_seconds": 8,
          "explanation": "SML直观展示了资产期望收益与其系统性风险（β）之间的线性关系。",
          "front": "什么是证券市场线（SML）？",
          "question_id": "q_flash_sml_definition_v1"
        },
        {
          "back": "说明资产被低估（α>0），实际期望收益高于CAPM给出的理论收益。",
          "estimated_seconds": 6,
          "explanation": "α>0表示资产具有正的超额收益，是投资机会。",
          "front": "如果一个资产落在SML的上方，说明什么？",
          "question_id": "q_flash_sml_definition_v2"
        },
        {
          "back": "说明资产被高估（α<0），实际期望收益低于CAPM给出的理论收益。",
          "estimated_seconds": 6,
          "explanation": "α<0表示资产表现不佳，应避免投资。",
          "front": "如果一个资产落在SML的下方，说明什么？",
          "question_id": "q_flash_sml_definition_v3"
        }
      ]
    },
    {
      "concept_key": "market_portfolio",
      "coverage_tags": [
        "market_portfolio"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_market_portfolio",
      "learning_goal": "学生能准确描述市场组合的定义和构成。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "市场组合的定义和权重计算方式。",
      "term_refs": [
        {
          "display": "市场组合",
          "en": "Market Portfolio (M)"
        }
      ],
      "variants": [
        {
          "back": "市场组合是包含经济体中所有风险资产的组合，每种资产的权重按其市值比例分配。",
          "estimated_seconds": 8,
          "explanation": "市场组合代表了整个市场的风险收益特征。",
          "front": "什么是市场组合（M）？",
          "question_id": "q_flash_market_portfolio_v1"
        },
        {
          "back": "w_i = (P_i × S_i) / Σ(P_j × S_j)，其中P_i是资产i的价格，S_i是资产i的流通股数。",
          "estimated_seconds": 10,
          "explanation": "权重等于该资产的总市值除以所有资产的总市值。",
          "front": "市场组合中资产i的权重如何计算？",
          "question_id": "q_flash_market_portfolio_v2"
        }
      ]
    }
  ],
  "lesson_id": "L7",
  "longform_families": [
    {
      "concept_key": "capital_asset_pricing_model",
      "coverage_tags": [
        "capital_asset_pricing_model",
        "beta_coefficient"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_capm_apply",
      "learning_goal": "学生能运用CAPM分析资产定价并解释结果。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "资本资产定价模型",
          "en": "Capital Asset Pricing Model (CAPM)"
        },
        {
          "display": "贝塔系数",
          "en": "Beta Coefficient (β)"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "CAPM公式",
            "理论收益计算",
            "α值计算与判断",
            "投资决策理由"
          ],
          "question_id": "q_long_capm_apply_v1",
          "reference_answer": [
            "CAPM公式：E(R_i) = R_f + β_i (E(R_M) - R_f)",
            "股票A理论收益 = 4% + 0.8 × (12% - 4%) = 4% + 0.8 × 8% = 4% + 6.4% = 10.4%",
            "股票B理论收益 = 4% + 1.5 × (12% - 4%) = 4% + 1.5 × 8% = 4% + 12% = 16%",
            "股票A的α = 10% - 10.4% = -0.4%，α<0，说明股票A被高估。",
            "股票B的α = 16% - 16% = 0%，α=0，说明股票B被合理定价。",
            "投资者应选择股票B，因为它被合理定价，而股票A被高估，实际收益低于其风险水平应有的收益。"
          ],
          "rubric_points": [
            "正确写出CAPM公式",
            "正确计算股票A的理论收益：4% + 0.8 × (12% - 4%) = 10.4%",
            "正确计算股票B的理论收益：4% + 1.5 × (12% - 4%) = 16%",
            "正确计算α值：A的α=10%-10.4%=-0.4%（高估），B的α=16%-16%=0%（合理定价）",
            "给出合理的投资建议并解释"
          ],
          "stem": "假设无风险利率为4%，市场期望收益为12%。现有两只股票：股票A的β=0.8，实际期望收益为10%；股票B的β=1.5，实际期望收益为16%。\n\n(1) 根据CAPM计算股票A和股票B的理论期望收益。\n(2) 判断每只股票是被高估还是被低估，并计算其α值。\n(3) 如果你是投资者，你会选择哪只股票？为什么？"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "CAPM公式应用",
            "理论收益与α计算",
            "α正负的含义",
            "投资选择的综合考量"
          ],
          "question_id": "q_long_capm_apply_v2",
          "reference_answer": [
            "CAPM公式：E(R_i) = R_f + β_i × 市场风险溢价",
            "基金X理论收益 = 2% + 1.2 × 6% = 2% + 7.2% = 9.2%",
            "基金Y理论收益 = 2% + 0.7 × 6% = 2% + 4.2% = 6.2%",
            "基金X的α = 11% - 9.2% = 1.8% > 0，表现优于市场。",
            "基金Y的α = 7% - 6.2% = 0.8% > 0，表现优于市场。",
            "虽然两只基金的α都为正，但不能简单认为都是好选择，因为：基金X的β更高（1.2），风险更大；基金Y虽然α较小但风险更低。投资者应根据自己的风险承受能力和投资目标选择。此外，还需考虑管理费、交易成本等因素。"
          ],
          "rubric_points": [
            "正确使用CAPM公式",
            "正确计算基金X的理论收益：2% + 1.2 × 6% = 9.2%",
            "正确计算基金Y的理论收益：2% + 0.7 × 6% = 6.2%",
            "正确计算α值：X的α=11%-9.2%=1.8%（正），Y的α=7%-6.2%=0.8%（正）",
            "解释α>0表示表现优于市场基准",
            "讨论其他因素：风险水平、投资目标、费用等"
          ],
          "stem": "假设无风险利率为2%，市场风险溢价为6%。现有两只基金：基金X的β=1.2，年化收益率为11%；基金Y的β=0.7，年化收益率为7%。\n\n(1) 根据CAPM计算每只基金的理论期望收益。\n(2) 计算每只基金的詹森阿尔法（α），并判断其表现是否优于市场。\n(3) 解释为什么即使两只基金的α都为正，也不能简单认为它们都是好的投资选择。"
        }
      ]
    },
    {
      "concept_key": "security_market_line",
      "coverage_tags": [
        "security_market_line",
        "capital_asset_pricing_model"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_sml_analysis",
      "learning_goal": "学生能比较SML和CML的异同，并解释SML在资产定价中的应用。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "compare_and_contrast",
      "term_refs": [
        {
          "display": "证券市场线",
          "en": "Security Market Line (SML)"
        },
        {
          "display": "资本市场线",
          "en": "Capital Market Line (CML)"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "SML公式与变量说明",
            "CML公式与变量说明",
            "SML与CML的图形区别",
            "SML上方资产的含义"
          ],
          "question_id": "q_long_sml_analysis_v1",
          "reference_answer": [
            "SML公式：E(R_i) = R_f + β_i (E(R_M) - R_f)，其中β_i衡量资产的系统性风险。",
            "CML公式：E(R_p) = R_f + (E(R_M)-R_f)/σ_M × σ_p，其中σ_p衡量组合的总风险。",
            "SML的横轴是贝塔系数（β），只考虑系统性风险；CML的横轴是标准差（σ），考虑总风险。",
            "SML的纵轴是期望收益，CML的纵轴也是期望收益。",
            "SML适用于单个资产或组合，CML只适用于有效组合。",
            "如果资产落在SML上方，说明α>0，资产被低估，实际期望收益高于其风险水平应有的收益，是买入机会。"
          ],
          "rubric_points": [
            "正确写出SML公式：E(R_i) = R_f + β_i (E(R_M) - R_f)",
            "正确写出CML公式：E(R_p) = R_f + (E(R_M)-R_f)/σ_M × σ_p",
            "说明SML横轴为β（系统性风险），纵轴为期望收益",
            "说明CML横轴为σ（总风险），纵轴为期望收益",
            "解释SML上方资产α>0，被低估",
            "说明投资者应买入此类资产"
          ],
          "stem": "证券市场线（SML）和资本市场线（CML）都是CAPM框架中的重要概念。\n\n(1) 分别写出SML和CML的公式，并说明每个变量的含义。\n(2) 解释SML和CML在横轴和纵轴上的区别。\n(3) 如果一个资产落在SML的上方，这对投资者意味着什么？请结合α值解释。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "CAPM理论收益计算",
            "α值计算",
            "SML图上的位置分析",
            "表现评价与原因分析"
          ],
          "question_id": "q_long_sml_analysis_v2",
          "reference_answer": [
            "理论期望收益 = 3% + 1.2 × (12% - 3%) = 3% + 1.2 × 9% = 3% + 10.8% = 13.8%",
            "詹森阿尔法α = 15% - 13.8% = 1.2% > 0",
            "在SML图上，该组合位于SML的上方，因为实际收益高于理论收益。",
            "α=1.2%>0，说明该组合的表现优于市场基准，基金经理创造了超额收益。",
            "可能的原因包括：基金经理的选股能力（选择了被低估的股票）、择时能力（在市场上涨时增加仓位）、或者运气因素。但需要注意，α的正值也可能是统计上的偶然，需要长期数据验证。"
          ],
          "rubric_points": [
            "正确计算理论收益：3% + 1.2 × (12% - 3%) = 13.8%",
            "正确计算α：15% - 13.8% = 1.2%",
            "说明α>0，组合在SML上方",
            "解释组合表现优于市场基准",
            "讨论可能原因：选股能力、择时能力、运气等"
          ],
          "stem": "假设你是一名基金经理，你管理的投资组合的β=1.2，年化收益率为15%。同期无风险利率为3%，市场指数收益率为12%。\n\n(1) 根据CAPM计算你的组合的理论期望收益。\n(2) 计算你的组合的詹森阿尔法（α），并在SML图上标出你的组合的位置。\n(3) 解释你的组合相对于市场表现如何，并讨论可能的原因。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "capital_asset_pricing_model",
      "coverage_tags": [
        "capital_asset_pricing_model"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_capm_calc",
      "learning_goal": "学生能运用CAPM公式计算资产的期望收益。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "资本资产定价模型",
          "en": "Capital Asset Pricing Model (CAPM)"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "E(R_i) = 2% + 1.2 × (8% - 2%) = 2% + 1.2 × 6% = 2% + 7.2% = 9.2%。",
          "options": [
            "7.2%",
            "8.0%",
            "9.2%",
            "11.6%"
          ],
          "question_id": "q_quiz_capm_calc_v1",
          "stem": "无风险利率为2%，市场期望收益为8%，某资产的β=1.2。根据CAPM，该资产的期望收益是多少？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "E(R_i) = 3% + 0.8 × 5% = 3% + 4% = 7%。注意市场风险溢价已经是E(R_M)-R_f。",
          "options": [
            "4.0%",
            "7.0%",
            "8.0%",
            "11.0%"
          ],
          "question_id": "q_quiz_capm_calc_v2",
          "stem": "无风险利率为3%，市场风险溢价为5%，某资产的β=0.8。根据CAPM，该资产的期望收益是多少？"
        }
      ]
    },
    {
      "concept_key": "capital_asset_pricing_model",
      "coverage_tags": [
        "capital_asset_pricing_model"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_capm_interpret",
      "learning_goal": "学生能理解CAPM中贝塔系数对期望收益的影响。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "资本资产定价模型",
          "en": "Capital Asset Pricing Model (CAPM)"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "β越大，风险溢价越高，期望收益也越高。β=1.5的资产承担更多系统性风险，因此要求更高的期望收益。",
          "options": [
            "期望收益更低",
            "期望收益更高",
            "期望收益相同",
            "无法比较"
          ],
          "question_id": "q_quiz_capm_interpret_v1",
          "stem": "根据CAPM，如果市场风险溢价为正，那么β=1.5的资产比β=0.5的资产："
        },
        {
          "answer": 3,
          "estimated_seconds": 15,
          "explanation": "β=0时，E(R_i)=R_f+0×(E(R_M)-R_f)=R_f，期望收益等于无风险利率。",
          "options": [
            "β=1.5的资产",
            "β=1.0的资产",
            "β=0.2的资产",
            "β=0的资产"
          ],
          "question_id": "q_quiz_capm_interpret_v2",
          "stem": "根据CAPM，以下哪种资产的期望收益最接近无风险利率？"
        }
      ]
    },
    {
      "concept_key": "beta_coefficient",
      "coverage_tags": [
        "beta_coefficient"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_beta_calc",
      "learning_goal": "学生能根据协方差和方差计算贝塔系数。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "贝塔系数",
          "en": "Beta Coefficient (β)"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "β = σ_iM / σ_M^2 = 0.03 / 0.02 = 1.5。",
          "options": [
            "0.67",
            "1.00",
            "1.50",
            "2.00"
          ],
          "question_id": "q_quiz_beta_calc_v1",
          "stem": "某资产与市场组合的协方差为0.03，市场组合的方差为0.02。该资产的贝塔系数是多少？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "σ_M^2 = 0.1^2 = 0.01。β = 0.01 / 0.01 = 1.0。",
          "options": [
            "0.1",
            "0.5",
            "1.0",
            "10.0"
          ],
          "question_id": "q_quiz_beta_calc_v2",
          "stem": "某资产与市场组合的协方差为0.01，市场组合的标准差为0.1。该资产的贝塔系数是多少？"
        }
      ]
    },
    {
      "concept_key": "beta_coefficient",
      "coverage_tags": [
        "beta_coefficient"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_beta_interpret",
      "learning_goal": "学生能根据贝塔值判断资产的风险特征。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "贝塔系数",
          "en": "Beta Coefficient (β)"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "β=1.8表示股票对市场敏感度是市场的1.8倍，市场上涨10%时，股票预期上涨18%。",
          "options": [
            "上涨10%",
            "上涨18%",
            "下跌10%",
            "下跌18%"
          ],
          "question_id": "q_quiz_beta_interpret_v1",
          "stem": "如果某股票的β=1.8，当市场上涨10%时，该股票最可能："
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "β=0.6的股票波动性小于市场，风险较低，适合保守型投资者。",
          "options": [
            "β=1.5的股票",
            "β=1.0的股票",
            "β=0.6的股票",
            "β=2.0的股票"
          ],
          "question_id": "q_quiz_beta_interpret_v2",
          "stem": "以下哪种股票最适合保守型投资者？"
        }
      ]
    },
    {
      "concept_key": "security_market_line",
      "coverage_tags": [
        "security_market_line"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_sml_interpret",
      "learning_goal": "学生能根据SML判断资产是否被低估或高估。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "证券市场线",
          "en": "Security Market Line (SML)"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "CAPM理论收益=3%+1.0×(10%-3%)=10%。实际收益12%>10%，α=2%>0，资产被低估。",
          "options": [
            "资产被高估，α=-1%",
            "资产被低估，α=2%",
            "资产被高估，α=2%",
            "资产被低估，α=-1%"
          ],
          "question_id": "q_quiz_sml_interpret_v1",
          "stem": "根据CAPM，某资产的β=1.0，无风险利率为3%，市场期望收益为10%。如果该资产的实际期望收益为12%，则："
        },
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "CAPM理论收益=2%+0.5×6%=5%。实际收益4%<5%，α=-1%<0，资产被高估。",
          "options": [
            "资产被高估，α=-1%",
            "资产被低估，α=1%",
            "资产被高估，α=1%",
            "资产被低估，α=-1%"
          ],
          "question_id": "q_quiz_sml_interpret_v2",
          "stem": "根据CAPM，某资产的β=0.5，无风险利率为2%，市场风险溢价为6%。如果该资产的实际期望收益为4%，则："
        }
      ]
    },
    {
      "concept_key": "market_portfolio",
      "coverage_tags": [
        "market_portfolio"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_market_portfolio",
      "learning_goal": "学生能理解市场组合的概念和构成。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "市场组合",
          "en": "Market Portfolio (M)"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "市场组合包含经济体中所有风险资产（股票、债券、房地产等），每种资产的权重按其市值比例分配。",
          "options": [
            "只包含股票",
            "包含所有风险资产，按市值加权",
            "所有资产的权重相等",
            "只包含无风险资产"
          ],
          "question_id": "q_quiz_market_portfolio_v1",
          "stem": "关于市场组合，以下哪项描述是正确的？"
        },
        {
          "answer": 0,
          "estimated_seconds": 15,
          "explanation": "在市场均衡条件下，所有投资者对风险资产的需求（切点组合）必须等于供给（市场组合），因此T=M。",
          "options": [
            "T与M完全相同",
            "T的风险比M低",
            "T的收益比M高",
            "T与M无关"
          ],
          "question_id": "q_quiz_market_portfolio_v2",
          "stem": "在CAPM假设下，市场均衡时，切点组合（T）与市场组合（M）的关系是："
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L7: Portfolio Optimization and Performance Measures - CAPM, SML, Market Portfolio",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "L7 step4: CAPM and Security Market Line",
    "plain_text": "pipeline/1-plain/L7/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L7: Portfolio Optimization and Performance Measures - Capital Asset Pricing Model (CAPM), Security Market Line (SML)"
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "correlation_effect_diversification",
      "covered_by": [
        "qf_flash_corr_effect",
        "qf_quiz_corr_extreme",
        "qf_long_diversification_mechanism"
      ],
      "description": "相关系数对分散风险效果的影响：ρ越低，分散效果越好；ρ=1时分散无效，ρ=-1时风险可降至零。"
    },
    {
      "coverage_tag": "minimum_variance_portfolio",
      "covered_by": [
        "qf_flash_mvp_def",
        "qf_quiz_mvp_weight",
        "qf_long_mvp_derivation"
      ],
      "description": "最小方差组合的定义（所有可能组合中风险最低的）及其权重公式。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "correlation_effect_diversification",
      "coverage_tags": [
        "correlation_effect_diversification"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_corr_effect",
      "learning_goal": "学生能准确回忆相关系数取值对分散风险效果的影响。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "相关系数的极端值（1和-1）对应的分散效果。",
      "term_refs": [
        {
          "display": "相关系数",
          "en": "Correlation Coefficient (ρ)"
        },
        {
          "display": "风险分散",
          "en": "Diversification"
        }
      ],
      "variants": [
        {
          "back": "完全无效。组合标准差等于各资产标准差的加权平均。",
          "estimated_seconds": 8,
          "explanation": "完全正相关意味着两只资产同涨同跌，无法通过组合降低风险。",
          "front": "当两只资产的相关系数 ρ = 1 时，分散风险的效果如何？",
          "question_id": "q_flash_corr_effect_v1"
        },
        {
          "back": "将组合风险降到零。",
          "estimated_seconds": 8,
          "explanation": "完全负相关时，可以通过调整权重使一个资产的上涨完全抵消另一个资产的下跌。",
          "front": "当两只资产的相关系数 ρ = -1 时，理论上可以做到什么？",
          "question_id": "q_flash_corr_effect_v2"
        },
        {
          "back": "好",
          "estimated_seconds": 5,
          "explanation": "相关系数越低，资产收益的同步性越弱，组合的方差就越小。",
          "front": "一般来说，两只资产的相关系数越低，分散风险的效果越____。",
          "question_id": "q_flash_corr_effect_v3"
        }
      ]
    },
    {
      "concept_key": "minimum_variance_portfolio",
      "coverage_tags": [
        "minimum_variance_portfolio"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_mvp_def",
      "learning_goal": "学生能准确说出最小方差组合的定义。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "最小方差组合在所有可能组合中的位置和特征。",
      "term_refs": [
        {
          "display": "最小方差组合",
          "en": "Minimum Variance Portfolio (MVP)"
        }
      ],
      "variants": [
        {
          "back": "最小方差组合（Minimum Variance Portfolio）。",
          "estimated_seconds": 6,
          "explanation": "它位于机会集的最左端，是所有可行组合中标准差最小的一个。",
          "front": "在所有可能的投资组合中，风险最低的那个组合叫什么？",
          "question_id": "q_flash_mvp_def_v1"
        },
        {
          "back": "求导（令方差对权重的导数等于零）。",
          "estimated_seconds": 8,
          "explanation": "组合方差是权重的二次函数，通过求导找到极值点即可得到最小方差对应的权重。",
          "front": "最小方差组合的权重通常通过什么数学方法求得？",
          "question_id": "q_flash_mvp_def_v2"
        }
      ]
    }
  ],
  "lesson_id": "L7",
  "longform_families": [
    {
      "concept_key": "correlation_effect_diversification",
      "coverage_tags": [
        "correlation_effect_diversification"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_diversification_mechanism",
      "learning_goal": "学生能解释相关系数如何影响组合风险，并说明分散化的原理。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "mechanism_trace",
      "term_refs": [
        {
          "display": "相关系数",
          "en": "Correlation Coefficient (ρ)"
        },
        {
          "display": "风险分散",
          "en": "Diversification"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "组合收益的计算方式",
            "组合方差公式中协方差项的作用",
            "相关系数对协方差项的影响",
            "举例说明"
          ],
          "question_id": "q_long_diversification_mechanism_v1",
          "reference_answer": [
            "组合的期望收益是各资产期望收益的加权平均，E(R_p) = w_A E(R_A) + w_B E(R_B)。",
            "组合方差为σ_p² = w_A²σ_A² + w_B²σ_B² + 2w_A w_B σ_AB，其中σ_AB = ρ σ_A σ_B。",
            "当ρ较低（比如接近0或为负）时，协方差项2w_A w_B ρ σ_A σ_B会变小甚至为负，从而降低整体组合方差。",
            "例如，当ρ=0.5时，通过适当权重，组合的方差可以小于两种资产方差的加权平均，实现风险分散。"
          ],
          "rubric_points": [
            "正确写出组合期望收益公式，说明收益是加权平均",
            "正确写出组合方差公式，指出协方差项2w_A w_B σ_AB",
            "解释σ_AB = ρ σ_A σ_B，ρ越低协方差项越小甚至为负",
            "举例：当ρ较低时，可以在保持收益的同时降低方差"
          ],
          "stem": "请解释为什么当两只资产的相关系数不是特别高时，可以通过构建组合获得比单独持有任一资产更好的风险收益特征。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "ρ=1时组合标准差的表达式",
            "ρ=-1时组合标准差的表达式",
            "最小方差组合的存在性",
            "实际投资中的意义"
          ],
          "question_id": "q_long_diversification_mechanism_v2",
          "reference_answer": [
            "当ρ=1时，σ_p² = (w_A σ_A + w_B σ_B)²，所以σ_p = w_A σ_A + w_B σ_B，组合风险是各资产风险的加权平均，没有分散效果。",
            "当ρ=-1时，σ_p² = (w_A σ_A - w_B σ_B)²，所以σ_p = |w_A σ_A - w_B σ_B|。当w_A = σ_B/(σ_A+σ_B)时，σ_p = 0。",
            "ρ=1时无法通过组合降低风险，ρ=-1时理论上可以将风险降至零。实际市场中ρ通常在-1到1之间，因此分散效果也介于两者之间。"
          ],
          "rubric_points": [
            "ρ=1时，σ_p = w_A σ_A + w_B σ_B，是线性组合，无分散效果",
            "ρ=-1时，σ_p = |w_A σ_A - w_B σ_B|，可以为零",
            "ρ=-1时存在权重使σ_p=0，ρ=1时不存在",
            "实际中ρ通常在-1到1之间，分散效果介于两者之间"
          ],
          "stem": "比较ρ=1和ρ=-1两种极端情况下，两只资产组合的风险特征有何不同？请用公式说明。"
        }
      ]
    },
    {
      "concept_key": "minimum_variance_portfolio",
      "coverage_tags": [
        "minimum_variance_portfolio"
      ],
      "difficulty": "hard",
      "family_id": "qf_long_mvp_derivation",
      "learning_goal": "学生能推导两只资产的最小方差组合权重，并计算其风险。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "worked_example",
      "term_refs": [
        {
          "display": "最小方差组合",
          "en": "Minimum Variance Portfolio (MVP)"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "写出组合方差公式",
            "令w_A = w, w_B = 1-w",
            "对w求导并令导数为零",
            "代入数值计算w和σ_p"
          ],
          "question_id": "q_long_mvp_derivation_v1",
          "reference_answer": [
            "组合方差：σ_p² = w²(0.3)² + (1-w)²(0.2)² + 2w(1-w)(0.3)(0.3)(0.2)",
            "= 0.09w² + 0.04(1-w)² + 0.036w(1-w)",
            "求导：dσ_p²/dw = 0.18w - 0.08(1-w) + 0.036(1-2w) = 0.18w - 0.08 + 0.08w + 0.036 - 0.072w = 0.188w - 0.044",
            "令导数为零：0.188w - 0.044 = 0 → w = 0.044/0.188 ≈ 0.234",
            "代入公式：σ_p² = 0.234²×0.09 + 0.766²×0.04 + 2×0.234×0.766×0.036",
            "= 0.00493 + 0.02347 + 0.01291 = 0.04131",
            "σ_p = √0.04131 ≈ 0.2033"
          ],
          "rubric_points": [
            "正确写出σ_p² = w²σ_A² + (1-w)²σ_B² + 2w(1-w)ρσ_Aσ_B",
            "正确求导dσ_p²/dw = 2w(σ_A²+σ_B²-2ρσ_Aσ_B) + 2σ_B(ρσ_A-σ_B)",
            "令导数为零，解得w = -σ_B(ρσ_A-σ_B)/(σ_A²+σ_B²-2ρσ_Aσ_B)",
            "代入数值计算w和σ_p"
          ],
          "stem": "已知资产A的标准差σ_A = 0.3，资产B的标准差σ_B = 0.2，两者的相关系数ρ = 0.3。请推导并计算最小方差组合中资产A的权重w_A，以及该组合的标准差。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "写出最小方差组合权重的通用公式",
            "代入数值计算w_A",
            "判断w_A是否在[0,1]区间内",
            "如果存在，计算组合标准差"
          ],
          "question_id": "q_long_mvp_derivation_v2",
          "reference_answer": [
            "公式：w = -σ_B(ρσ_A-σ_B)/(σ_A²+σ_B²-2ρσ_Aσ_B)",
            "代入：w = -0.1[(-0.5)(0.5)-0.1] / [0.25+0.01-2(-0.5)(0.5)(0.1)]",
            "= -0.1(-0.25-0.1) / [0.26+0.05] = -0.1(-0.35)/0.31 = 0.035/0.31 ≈ 0.1129",
            "w_A ≈ 0.1129，在0到1之间，最小方差组合存在。",
            "组合方差：σ_p² = (0.1129)²(0.25) + (0.8871)²(0.01) + 2(0.1129)(0.8871)(-0.5)(0.5)(0.1)",
            "= 0.00319 + 0.00787 - 0.00501 = 0.00605",
            "σ_p = √0.00605 ≈ 0.0778"
          ],
          "rubric_points": [
            "正确使用公式w = -σ_B(ρσ_A-σ_B)/(σ_A²+σ_B²-2ρσ_Aσ_B)",
            "正确代入数值计算",
            "正确判断权重是否在0到1之间",
            "如果存在，正确计算组合标准差"
          ],
          "stem": "假设资产A的标准差σ_A = 0.5，资产B的标准差σ_B = 0.1，两者的相关系数ρ = -0.5。请推导最小方差组合的权重，并判断该组合是否存在（即权重是否在0到1之间）。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "correlation_effect_diversification",
      "coverage_tags": [
        "correlation_effect_diversification"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_corr_extreme",
      "learning_goal": "学生能在测验中辨析不同相关系数下组合风险的特征。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "相关系数",
          "en": "Correlation Coefficient (ρ)"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "当ρ=1时，组合方差公式简化为(w_A σ_A + w_B σ_B)²，开方后就是加权平均，没有分散效果。",
          "options": [
            "各资产标准差的加权平均",
            "小于各资产标准差的加权平均",
            "等于零",
            "无法确定"
          ],
          "question_id": "q_quiz_corr_extreme_v1",
          "stem": "当两只资产完全正相关（ρ=1）时，组合的标准差等于："
        },
        {
          "answer": 0,
          "estimated_seconds": 30,
          "explanation": "当ρ=-1时，组合标准差为零的条件是w_A σ_A = (1-w_A)σ_B，解得w_A = σ_B/(σ_A+σ_B) = 0.2/(0.3+0.2) = 0.4。",
          "options": [
            "0.4",
            "0.6",
            "0.5",
            "0.3"
          ],
          "question_id": "q_quiz_corr_extreme_v2",
          "stem": "如果两只资产的相关系数ρ = -1，且σ_A = 0.3，σ_B = 0.2，要使组合标准差为零，资产A的权重w_A应为："
        }
      ]
    },
    {
      "concept_key": "minimum_variance_portfolio",
      "coverage_tags": [
        "minimum_variance_portfolio"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_mvp_weight",
      "learning_goal": "学生能理解最小方差组合权重的决定因素。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "最小方差组合",
          "en": "Minimum Variance Portfolio (MVP)"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "最小方差组合只关注风险最小化，因此权重只由方差-协方差矩阵决定，与期望收益无关。",
          "options": [
            "资产的期望收益率",
            "资产的方差和它们的协方差",
            "无风险利率",
            "市场组合的收益率"
          ],
          "question_id": "q_quiz_mvp_weight_v1",
          "stem": "在两只资产的情况下，最小方差组合的权重主要取决于："
        },
        {
          "answer": 1,
          "estimated_seconds": 30,
          "explanation": "min(σ_A/σ_B, σ_B/σ_A) = min(2, 0.5) = 0.5，而ρ=0.5，条件ρ ≤ 0.5成立（等号），此时最小方差组合的边界情况，权重可能为0或1。",
          "options": [
            "最小方差组合一定存在且权重在0到1之间",
            "最小方差组合不存在，因为ρ > min(σ_A/σ_B, σ_B/σ_A)",
            "最小方差组合的权重与期望收益有关",
            "最小方差组合的方差一定小于σ_B²"
          ],
          "question_id": "q_quiz_mvp_weight_v2",
          "stem": "给定σ_A = 0.4，σ_B = 0.2，ρ = 0.5，以下哪个说法正确？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L7/plain.txt",
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
      "coverage_tag": "efficient_frontier",
      "covered_by": [
        "qf_flash_eff_frontier",
        "qf_quiz_eff_frontier",
        "qf_long_eff_frontier"
      ],
      "description": "有效前沿的定义、图形含义及与机会集的关系"
    },
    {
      "coverage_tag": "tangency_portfolio",
      "covered_by": [
        "qf_flash_tangency",
        "qf_quiz_tangency",
        "qf_long_tangency"
      ],
      "description": "切点组合的定义、求解目标函数及与无风险资产的关系"
    },
    {
      "coverage_tag": "capital_market_line",
      "covered_by": [
        "qf_flash_cml",
        "qf_quiz_cml",
        "qf_long_cml"
      ],
      "description": "资本市场线的公式、图形及与有效前沿的关系"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "efficient_frontier",
      "coverage_tags": [
        "efficient_frontier"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_eff_frontier",
      "learning_goal": "学生能准确回忆有效前沿的定义及其在风险-收益图中的位置。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "有效前沿的定义和它在机会集中的位置。",
      "term_refs": [
        {
          "display": "有效前沿",
          "en": "Efficient Frontier"
        }
      ],
      "variants": [
        {
          "back": "有效前沿",
          "estimated_seconds": 5,
          "explanation": "有效前沿是机会集的上边缘，代表在相同风险下能获得的最高预期收益。",
          "front": "在风险-收益图中，机会集的上边缘叫什么？",
          "question_id": "q_flash_eff_frontier_v1"
        },
        {
          "back": "在相同风险下提供最高的预期收益",
          "estimated_seconds": 8,
          "explanation": "有效前沿上的组合是给定风险水平下收益最高的组合，不在其上的组合都是低效的。",
          "front": "有效前沿上的组合与不在其上的组合相比，有什么关键优势？",
          "question_id": "q_flash_eff_frontier_v2"
        }
      ]
    },
    {
      "concept_key": "tangency_portfolio",
      "coverage_tags": [
        "tangency_portfolio"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_tangency",
      "learning_goal": "学生能回忆切点组合的定义及其求解目标。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "切点组合的定义和求解目标函数。",
      "term_refs": [
        {
          "display": "切点组合",
          "en": "Tangency Portfolio"
        }
      ],
      "variants": [
        {
          "back": "切点组合",
          "estimated_seconds": 5,
          "explanation": "切点组合是无风险资产与有效前沿相切处的风险资产组合。",
          "front": "从无风险利率出发，与有效前沿相切的直线上的切点叫什么？",
          "question_id": "q_flash_tangency_v1"
        },
        {
          "back": "夏普比率",
          "estimated_seconds": 8,
          "explanation": "切点组合的权重通过最大化夏普比率（(μ_p - R_f)/σ_p）得到。",
          "front": "求解切点组合时，目标函数是最大化什么？",
          "question_id": "q_flash_tangency_v2"
        }
      ]
    },
    {
      "concept_key": "capital_market_line",
      "coverage_tags": [
        "capital_market_line"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_cml",
      "learning_goal": "学生能回忆资本市场线的公式及其含义。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "CML公式及其各部分的含义。",
      "term_refs": [
        {
          "display": "资本市场线",
          "en": "Capital Market Line (CML)"
        }
      ],
      "variants": [
        {
          "back": "E(R_p) = R_f + (E(R_M) - R_f)/σ_M * σ_p",
          "estimated_seconds": 10,
          "explanation": "CML公式表示有效组合的期望收益等于无风险利率加上市场风险溢价乘以组合标准差与市场标准差的比值。",
          "front": "资本市场线（CML）的公式是什么？",
          "question_id": "q_flash_cml_v1"
        },
        {
          "back": "市场风险溢价除以市场标准差，即 (E(R_M) - R_f)/σ_M",
          "estimated_seconds": 10,
          "explanation": "CML的斜率是市场组合的夏普比率，代表每单位总风险的市场价格。",
          "front": "资本市场线（CML）的斜率代表什么？",
          "question_id": "q_flash_cml_v2"
        }
      ]
    }
  ],
  "lesson_id": "L7",
  "longform_families": [
    {
      "concept_key": "efficient_frontier",
      "coverage_tags": [
        "efficient_frontier"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_eff_frontier",
      "learning_goal": "学生能解释有效前沿的概念及其在投资组合优化中的意义。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "有效前沿",
          "en": "Efficient Frontier"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "有效前沿的定义",
            "有效前沿在风险-收益图中的位置",
            "为什么不在有效前沿上的组合是低效的"
          ],
          "question_id": "q_long_eff_frontier_v1",
          "reference_answer": [
            "有效前沿是机会集的上边缘，代表在给定风险水平下能获得的最高预期收益的投资组合集合。",
            "在风险-收益图中，所有可能的组合构成一个区域（机会集），其左上边界就是有效前沿。",
            "不在有效前沿上的组合是低效的，因为存在另一个组合在相同风险下提供更高的收益，或者在相同收益下承担更低的风险。因此，理性投资者应该只考虑有效前沿上的组合。"
          ],
          "rubric_points": [
            "正确描述有效前沿是机会集的上边缘",
            "说明有效前沿上的组合在给定风险下收益最高",
            "解释不在有效前沿上的组合可以被更优的组合替代"
          ],
          "stem": "请解释什么是有效前沿，并说明为什么投资者应该只考虑有效前沿上的组合。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "机会集与有效前沿的关系",
            "有效前沿上的组合的优势",
            "机会集内部组合的劣势"
          ],
          "question_id": "q_long_eff_frontier_v2",
          "reference_answer": [
            "机会集是所有可能投资组合的集合，而有效前沿是机会集的上边缘。",
            "有效前沿上的组合在相同风险下提供最高的预期收益。",
            "机会集内部的组合是低效的，因为存在有效前沿上的组合，在相同风险下收益更高，或者在相同收益下风险更低。因此，选择机会集内部的组合意味着放弃了更好的投资机会。"
          ],
          "rubric_points": [
            "正确描述机会集和有效前沿的关系",
            "说明有效前沿上的组合在相同风险下收益更高",
            "解释机会集内部的组合可以被有效前沿上的组合替代"
          ],
          "stem": "假设你是一个投资顾问，你的客户问为什么不应该选择机会集内部的一个组合。请用有效前沿的概念解释。"
        }
      ]
    },
    {
      "concept_key": "tangency_portfolio",
      "coverage_tags": [
        "tangency_portfolio"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_tangency",
      "learning_goal": "学生能解释切点组合的求解逻辑及其在投资组合理论中的作用。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "mechanism_trace",
      "term_refs": [
        {
          "display": "切点组合",
          "en": "Tangency Portfolio"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "切点组合的定义",
            "求解目标函数的选择逻辑",
            "切点组合在分离定理中的作用"
          ],
          "question_id": "q_long_tangency_v1",
          "reference_answer": [
            "切点组合是从无风险利率出发，与有效前沿相切处的风险资产组合。",
            "求解切点组合时最大化夏普比率，因为夏普比率衡量每单位总风险获得的超额收益，最大化它意味着在给定风险下获得最高的超额收益。",
            "切点组合在投资组合理论中非常重要，因为根据分离定理，所有理性投资者都会持有无风险资产和切点组合的组合，只是比例不同。切点组合是投资者对风险资产的最优选择。"
          ],
          "rubric_points": [
            "正确描述切点组合是从无风险利率出发与有效前沿相切的点",
            "解释最大化夏普比率是因为它代表每单位总风险获得的超额收益",
            "说明切点组合是所有理性投资者持有的最优风险组合"
          ],
          "stem": "请解释为什么求解切点组合时目标函数是最大化夏普比率，并说明切点组合在投资组合理论中的重要性。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "分离定理的基本思想",
            "切点组合的确定",
            "投资者风险偏好的作用"
          ],
          "question_id": "q_long_tangency_v2",
          "reference_answer": [
            "分离定理指出，投资决策可以分为两步：首先确定最优风险资产组合（切点组合），然后根据个人风险偏好决定无风险资产和切点组合的分配比例。",
            "切点组合是通过最大化夏普比率确定的，它独立于投资者的风险偏好，只取决于市场参数。",
            "不同风险偏好的投资者会持有相同的切点组合，但会调整无风险资产的比例：风险厌恶者会持有更多无风险资产，风险偏好者会持有更多切点组合。"
          ],
          "rubric_points": [
            "正确描述分离定理",
            "说明切点组合是独立于投资者风险偏好的最优风险组合",
            "解释风险偏好只影响无风险资产和切点组合的分配比例"
          ],
          "stem": "假设市场上存在无风险资产，请解释为什么所有理性投资者都会持有相同的风险资产组合（切点组合），尽管他们对风险的偏好不同。"
        }
      ]
    },
    {
      "concept_key": "capital_market_line",
      "coverage_tags": [
        "capital_market_line"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_cml",
      "learning_goal": "学生能应用CML公式进行计算并解释其含义。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "formula_apply",
      "term_refs": [
        {
          "display": "资本市场线",
          "en": "Capital Market Line (CML)"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "CML公式",
            "斜率计算",
            "给定标准差求收益",
            "给定收益求标准差"
          ],
          "question_id": "q_long_cml_v1",
          "reference_answer": [
            "CML公式：E(R_p) = R_f + (E(R_M) - R_f)/σ_M * σ_p",
            "(1) 斜率 = (12% - 2%) / 20% = 0.5",
            "(2) E(R_p) = 2% + 0.5 * 15% = 9.5%",
            "(3) 15% = 2% + 0.5 * σ_p，解得 σ_p = 26%"
          ],
          "rubric_points": [
            "正确写出CML公式",
            "正确计算斜率为0.5",
            "正确计算标准差15%时的收益为9.5%",
            "正确计算收益15%时的标准差为26%"
          ],
          "stem": "假设无风险利率为2%，市场组合的期望收益为12%，市场组合的标准差为20%。请计算并解释：\n(1) CML的斜率是多少？\n(2) 一个标准差为15%的有效组合的期望收益是多少？\n(3) 如果某个组合的期望收益为15%，它的标准差应该是多少？"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "CML公式",
            "斜率计算",
            "给定标准差求收益",
            "给定收益求标准差"
          ],
          "question_id": "q_long_cml_v2",
          "reference_answer": [
            "CML公式：E(R_p) = R_f + (E(R_M) - R_f)/σ_M * σ_p",
            "(1) 斜率 = (15% - 3%) / 25% = 0.48",
            "(2) E(R_p) = 3% + 0.48 * 10% = 7.8%",
            "(3) 18% = 3% + 0.48 * σ_p，解得 σ_p = 31.25%"
          ],
          "rubric_points": [
            "正确写出CML公式",
            "正确计算斜率为0.48",
            "正确计算标准差10%时的收益为7.8%",
            "正确计算收益18%时的标准差为31.25%"
          ],
          "stem": "假设无风险利率为3%，市场组合的期望收益为15%，市场组合的标准差为25%。请计算并解释：\n(1) CML的斜率是多少？\n(2) 一个标准差为10%的有效组合的期望收益是多少？\n(3) 如果某个组合的期望收益为18%，它的标准差应该是多少？"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "efficient_frontier",
      "coverage_tags": [
        "efficient_frontier"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_eff_frontier",
      "learning_goal": "学生能在测验情境下辨析有效前沿的特征。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "有效前沿",
          "en": "Efficient Frontier"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "有效前沿是机会集的上边缘，代表在给定风险水平下能获得的最高预期收益。",
          "options": [
            "有效前沿上的组合在相同风险下收益最低",
            "有效前沿是机会集的下边缘",
            "有效前沿上的组合在相同风险下收益最高",
            "所有可能的组合都在有效前沿上"
          ],
          "question_id": "q_quiz_eff_frontier_v1",
          "stem": "关于有效前沿，以下哪项描述是正确的？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "不在有效前沿上的组合是低效的，因为存在另一个组合在相同风险下提供更高的收益。",
          "options": [
            "它一定是风险最低的组合",
            "存在另一个组合，在相同风险下收益更高",
            "它一定是收益最高的组合",
            "它无法通过任何资产组合构建"
          ],
          "question_id": "q_quiz_eff_frontier_v2",
          "stem": "如果一个组合不在有效前沿上，以下哪项描述最准确？"
        }
      ]
    },
    {
      "concept_key": "tangency_portfolio",
      "coverage_tags": [
        "tangency_portfolio"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_tangency",
      "learning_goal": "学生能辨析切点组合的求解目标及其与无风险资产的关系。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "切点组合",
          "en": "Tangency Portfolio"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "切点组合的权重通过最大化夏普比率（(μ_p - R_f)/σ_p）得到。",
          "options": [
            "组合收益",
            "夏普比率",
            "组合方差",
            "相关系数"
          ],
          "question_id": "q_quiz_tangency_v1",
          "stem": "求解切点组合时，目标函数是最大化："
        },
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "根据分离定理，所有理性投资者都会持有无风险资产和切点组合的组合，只是比例不同。",
          "options": [
            "只持有无风险资产",
            "只持有切点组合",
            "持有无风险资产和切点组合的组合",
            "持有所有风险资产的等权组合"
          ],
          "question_id": "q_quiz_tangency_v2",
          "stem": "当市场上存在无风险资产时，理性投资者会如何构建投资组合？"
        }
      ]
    },
    {
      "concept_key": "capital_market_line",
      "coverage_tags": [
        "capital_market_line"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_cml",
      "learning_goal": "学生能应用CML公式计算组合的期望收益。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "资本市场线",
          "en": "Capital Market Line (CML)"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 30,
          "explanation": "E(R_p) = 3% + (10% - 3%)/15% * 10% = 3% + 4.67% = 7.67%。",
          "options": [
            "7.67%",
            "8.33%",
            "9.00%",
            "10.00%"
          ],
          "question_id": "q_quiz_cml_v1",
          "stem": "假设无风险利率为3%，市场组合的期望收益为10%，市场组合的标准差为15%。根据CML，一个标准差为10%的有效组合的期望收益是多少？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "CML上的组合都是有效组合，它们由无风险资产和市场组合构成。",
          "options": [
            "所有资产都落在CML上",
            "CML上的组合都是有效组合",
            "CML的斜率等于市场组合的方差",
            "CML上的组合风险都为零"
          ],
          "question_id": "q_quiz_cml_v2",
          "stem": "根据CML，以下哪项描述是正确的？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L7/plain.txt",
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
      "coverage_tag": "information_ratio",
      "covered_by": [
        "qf_flash_ir_definition",
        "qf_flash_ir_formula",
        "qf_quiz_ir_interpretation",
        "qf_long_ir_calculation"
      ],
      "description": "信息比率：衡量组合相对于基准的主动管理能力，公式为 IR = E(R_p - R_b) / σ(R_p - R_b)。"
    },
    {
      "coverage_tag": "maximum_drawdown",
      "covered_by": [
        "qf_flash_mdd_definition",
        "qf_flash_mdd_formula",
        "qf_quiz_mdd_interpretation",
        "qf_long_mdd_calculation"
      ],
      "description": "最大回撤：从历史最高点到随后最低点的最大跌幅，公式为 MDD = (P_peak - P_trough) / P_peak。"
    },
    {
      "coverage_tag": "calmar_ratio",
      "covered_by": [
        "qf_flash_calmar_definition",
        "qf_flash_calmar_formula",
        "qf_quiz_calmar_interpretation",
        "qf_long_calmar_calculation"
      ],
      "description": "卡尔玛比率：年化收益与最大回撤的比值，衡量每单位最大回撤能带来多少收益。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "information_ratio",
      "coverage_tags": [
        "information_ratio"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_ir_definition",
      "learning_goal": "学生能准确回忆信息比率的定义及其与夏普比率的区别。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "信息比率衡量什么？分母是什么？",
      "term_refs": [
        {
          "display": "信息比率",
          "en": "Information Ratio"
        }
      ],
      "variants": [
        {
          "back": "组合相对于基准的主动管理能力，即每单位跟踪误差带来的超额收益。",
          "estimated_seconds": 8,
          "explanation": "信息比率 = E(R_p - R_b) / σ(R_p - R_b)。",
          "front": "信息比率（IR）衡量的是什么？",
          "question_id": "q_flash_ir_def_v1"
        },
        {
          "back": "跟踪误差，即组合与基准收益差值的标准差 σ(R_p - R_b)。",
          "estimated_seconds": 6,
          "explanation": "跟踪误差衡量了组合收益相对于基准的偏离程度。",
          "front": "信息比率的分母是什么？",
          "question_id": "q_flash_ir_def_v2"
        }
      ]
    },
    {
      "concept_key": "information_ratio",
      "coverage_tags": [
        "information_ratio"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_ir_formula",
      "learning_goal": "学生能准确写出信息比率的公式。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "信息比率的数学表达式",
      "term_refs": [
        {
          "display": "信息比率",
          "en": "Information Ratio"
        }
      ],
      "variants": [
        {
          "back": "IR = E(R_p - R_b) / σ(R_p - R_b)",
          "estimated_seconds": 10,
          "explanation": "分子是组合相对于基准的超额收益的期望，分母是跟踪误差。",
          "front": "写出信息比率（IR）的公式。",
          "question_id": "q_flash_ir_formula_v1"
        },
        {
          "back": "组合相对于基准的超额收益的期望值。",
          "estimated_seconds": 5,
          "explanation": "这是信息比率的分子部分。",
          "front": "信息比率中，E(R_p - R_b) 代表什么？",
          "question_id": "q_flash_ir_formula_v2"
        }
      ]
    },
    {
      "concept_key": "maximum_drawdown",
      "coverage_tags": [
        "maximum_drawdown"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_mdd_definition",
      "learning_goal": "学生能准确回忆最大回撤的定义和含义。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "最大回撤衡量什么？",
      "term_refs": [
        {
          "display": "最大回撤",
          "en": "Maximum Drawdown"
        }
      ],
      "variants": [
        {
          "back": "从历史最高点到随后最低点的最大跌幅，即最坏情况下的亏损幅度。",
          "estimated_seconds": 8,
          "explanation": "MDD = (P_peak - P_trough) / P_peak。",
          "front": "最大回撤（MDD）衡量的是什么？",
          "question_id": "q_flash_mdd_def_v1"
        },
        {
          "back": "峰值（P_peak）减去谷值（P_trough）。",
          "estimated_seconds": 5,
          "explanation": "MDD = (P_peak - P_trough) / P_peak。",
          "front": "最大回撤的分子是什么？",
          "question_id": "q_flash_mdd_def_v2"
        }
      ]
    },
    {
      "concept_key": "maximum_drawdown",
      "coverage_tags": [
        "maximum_drawdown"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_mdd_formula",
      "learning_goal": "学生能准确写出最大回撤的公式。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "最大回撤的数学表达式",
      "term_refs": [
        {
          "display": "最大回撤",
          "en": "Maximum Drawdown"
        }
      ],
      "variants": [
        {
          "back": "MDD = (P_peak - P_trough) / P_peak",
          "estimated_seconds": 8,
          "explanation": "P_peak 是历史最高净值，P_trough 是随后最低净值。",
          "front": "写出最大回撤（MDD）的公式。",
          "question_id": "q_flash_mdd_formula_v1"
        },
        {
          "back": "历史最高点（峰值）的净值。",
          "estimated_seconds": 5,
          "explanation": "MDD = (P_peak - P_trough) / P_peak。",
          "front": "最大回撤公式中，P_peak 代表什么？",
          "question_id": "q_flash_mdd_formula_v2"
        }
      ]
    },
    {
      "concept_key": "calmar_ratio",
      "coverage_tags": [
        "calmar_ratio"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_calmar_definition",
      "learning_goal": "学生能准确回忆卡尔玛比率的定义和含义。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "卡尔玛比率衡量什么？",
      "term_refs": [
        {
          "display": "卡尔玛比率",
          "en": "Calmar Ratio"
        }
      ],
      "variants": [
        {
          "back": "每单位最大回撤能带来多少年化收益。",
          "estimated_seconds": 8,
          "explanation": "Calmar Ratio = 年化收益 / |最大回撤|。",
          "front": "卡尔玛比率（Calmar Ratio）衡量的是什么？",
          "question_id": "q_flash_calmar_def_v1"
        },
        {
          "back": "最大回撤的绝对值（|Max. Drawdown|）。",
          "estimated_seconds": 5,
          "explanation": "Calmar Ratio = 年化收益 / |最大回撤|。",
          "front": "卡尔玛比率的分母是什么？",
          "question_id": "q_flash_calmar_def_v2"
        }
      ]
    },
    {
      "concept_key": "calmar_ratio",
      "coverage_tags": [
        "calmar_ratio"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_calmar_formula",
      "learning_goal": "学生能准确写出卡尔玛比率的公式。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "卡尔玛比率的数学表达式",
      "term_refs": [
        {
          "display": "卡尔玛比率",
          "en": "Calmar Ratio"
        }
      ],
      "variants": [
        {
          "back": "Calmar Ratio = 年化收益 / |最大回撤|",
          "estimated_seconds": 8,
          "explanation": "分子是年化收益，分母是最大回撤的绝对值。",
          "front": "写出卡尔玛比率（Calmar Ratio）的公式。",
          "question_id": "q_flash_calmar_formula_v1"
        },
        {
          "back": "年化收益（Annualized Return）。",
          "estimated_seconds": 5,
          "explanation": "Calmar Ratio = 年化收益 / |最大回撤|。",
          "front": "卡尔玛比率公式中，分子是什么？",
          "question_id": "q_flash_calmar_formula_v2"
        }
      ]
    }
  ],
  "lesson_id": "L7",
  "longform_families": [
    {
      "concept_key": "information_ratio",
      "coverage_tags": [
        "information_ratio"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_ir_calculation",
      "learning_goal": "学生能根据给定的组合和基准收益数据，计算信息比率并解释其含义。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "worked_example",
      "term_refs": [
        {
          "display": "信息比率",
          "en": "Information Ratio"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "计算每个月的超额收益 (R_p - R_b)",
            "计算超额收益的期望 E(R_p - R_b)",
            "计算跟踪误差 σ(R_p - R_b)",
            "计算信息比率 IR = E(R_p - R_b) / σ(R_p - R_b)",
            "解释信息比率的值"
          ],
          "question_id": "q_long_ir_calc_v1",
          "reference_answer": [
            "超额收益: [2%-1%=1%, 1%-0.5%=0.5%, -1%-0%=-1%, 3%-2%=1%]",
            "E(R_p - R_b) = (1% + 0.5% - 1% + 1%) / 4 = 0.375%",
            "跟踪误差 = sqrt( ((1%-0.375%)^2 + (0.5%-0.375%)^2 + (-1%-0.375%)^2 + (1%-0.375%)^2) / 3 ) = sqrt( (0.390625 + 0.015625 + 1.890625 + 0.390625) / 3 ) = sqrt(2.6875/3) = sqrt(0.895833) ≈ 0.946%",
            "IR = 0.375% / 0.946% ≈ 0.396",
            "信息比率约为0.396，小于0.5，表示该组合相对于基准的主动管理能力一般，超额收益不高且波动较大。"
          ],
          "rubric_points": [
            "正确计算每个月的超额收益",
            "正确计算超额收益的期望",
            "正确计算跟踪误差（样本标准差）",
            "正确计算信息比率",
            "正确解释信息比率的意义"
          ],
          "stem": "一个组合在过去4个月的收益率为 [2%, 1%, -1%, 3%]，同期基准收益率为 [1%, 0.5%, 0%, 2%]。请计算该组合的信息比率，并解释其含义。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "计算每个月的超额收益 (R_p - R_b)",
            "计算超额收益的期望 E(R_p - R_b)",
            "计算跟踪误差 σ(R_p - R_b)",
            "计算信息比率 IR = E(R_p - R_b) / σ(R_p - R_b)",
            "解释信息比率的值"
          ],
          "question_id": "q_long_ir_calc_v2",
          "reference_answer": [
            "超额收益: [3%-2%=1%, -2%-(-1%)=-1%, 4%-3%=1%, 1%-0%=1%, 2%-1%=1%]",
            "E(R_p - R_b) = (1% - 1% + 1% + 1% + 1%) / 5 = 0.6%",
            "跟踪误差 = sqrt( ((1%-0.6%)^2 + (-1%-0.6%)^2 + (1%-0.6%)^2 + (1%-0.6%)^2 + (1%-0.6%)^2) / 4 ) = sqrt( (0.16 + 2.56 + 0.16 + 0.16 + 0.16) / 4 ) = sqrt(3.2/4) = sqrt(0.8) ≈ 0.894%",
            "IR = 0.6% / 0.894% ≈ 0.671",
            "信息比率约为0.671，大于0.5，表示该组合相对于基准有较好的主动管理能力，超额收益相对稳定。"
          ],
          "rubric_points": [
            "正确计算每个月的超额收益",
            "正确计算超额收益的期望",
            "正确计算跟踪误差（样本标准差）",
            "正确计算信息比率",
            "正确解释信息比率的意义"
          ],
          "stem": "一个组合在过去5个月的收益率为 [3%, -2%, 4%, 1%, 2%]，同期基准收益率为 [2%, -1%, 3%, 0%, 1%]。请计算该组合的信息比率，并解释其含义。"
        }
      ]
    },
    {
      "concept_key": "maximum_drawdown",
      "coverage_tags": [
        "maximum_drawdown"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_mdd_calculation",
      "learning_goal": "学生能根据给定的净值序列计算最大回撤。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "worked_example",
      "term_refs": [
        {
          "display": "最大回撤",
          "en": "Maximum Drawdown"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "找出所有历史高点（峰值）",
            "对于每个峰值，找出其后的最低点（谷值）",
            "计算每个峰值到谷值的回撤",
            "找出最大回撤",
            "解释最大回撤的意义"
          ],
          "question_id": "q_long_mdd_calc_v1",
          "reference_answer": [
            "峰值: 100, 110, 120",
            "从100到90的回撤: (100-90)/100 = 10%",
            "从110到90的回撤: (110-90)/110 = 18.18%",
            "从120到90的回撤: (120-90)/120 = 25%",
            "最大回撤为25%，表示该组合历史上最坏情况下亏损了25%。"
          ],
          "rubric_points": [
            "正确识别所有峰值",
            "正确计算每个峰值后的回撤",
            "正确找出最大回撤",
            "正确解释最大回撤的意义"
          ],
          "stem": "一个组合的净值序列为 [100, 110, 105, 120, 115, 90, 130]。请计算该组合的最大回撤，并解释其含义。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "找出所有历史高点（峰值）",
            "对于每个峰值，找出其后的最低点（谷值）",
            "计算每个峰值到谷值的回撤",
            "找出最大回撤",
            "解释最大回撤的意义"
          ],
          "question_id": "q_long_mdd_calc_v2",
          "reference_answer": [
            "峰值: 100, 90, 110",
            "从100到70的回撤: (100-70)/100 = 30%",
            "从90到70的回撤: (90-70)/90 = 22.22%",
            "从110到95的回撤: (110-95)/110 = 13.64%",
            "最大回撤为30%，表示该组合历史上最坏情况下亏损了30%。"
          ],
          "rubric_points": [
            "正确识别所有峰值",
            "正确计算每个峰值后的回撤",
            "正确找出最大回撤",
            "正确解释最大回撤的意义"
          ],
          "stem": "一个组合的净值序列为 [100, 80, 90, 70, 110, 105, 95]。请计算该组合的最大回撤，并解释其含义。"
        }
      ]
    },
    {
      "concept_key": "calmar_ratio",
      "coverage_tags": [
        "calmar_ratio"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_calmar_calculation",
      "learning_goal": "学生能根据给定的年化收益和最大回撤计算卡尔玛比率，并解释其含义。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "worked_example",
      "term_refs": [
        {
          "display": "卡尔玛比率",
          "en": "Calmar Ratio"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 60,
          "prompt_blocks": [
            "回忆卡尔玛比率的公式",
            "代入年化收益和最大回撤",
            "计算卡尔玛比率",
            "解释卡尔玛比率的意义"
          ],
          "question_id": "q_long_calmar_calc_v1",
          "reference_answer": [
            "Calmar Ratio = 年化收益 / |最大回撤|",
            "Calmar Ratio = 18% / 12% = 1.5",
            "卡尔玛比率为1.5，表示每承担1%的最大回撤，可以获得1.5%的年化收益。"
          ],
          "rubric_points": [
            "正确写出卡尔玛比率公式",
            "正确代入数值",
            "正确计算卡尔玛比率",
            "正确解释卡尔玛比率的意义"
          ],
          "stem": "一个组合的年化收益为18%，最大回撤为12%。请计算该组合的卡尔玛比率，并解释其含义。"
        },
        {
          "estimated_seconds": 60,
          "prompt_blocks": [
            "回忆卡尔玛比率的公式",
            "代入年化收益和最大回撤",
            "计算卡尔玛比率",
            "解释卡尔玛比率的意义"
          ],
          "question_id": "q_long_calmar_calc_v2",
          "reference_answer": [
            "Calmar Ratio = 年化收益 / |最大回撤|",
            "Calmar Ratio = 25% / 20% = 1.25",
            "卡尔玛比率为1.25，表示每承担1%的最大回撤，可以获得1.25%的年化收益。"
          ],
          "rubric_points": [
            "正确写出卡尔玛比率公式",
            "正确代入数值",
            "正确计算卡尔玛比率",
            "正确解释卡尔玛比率的意义"
          ],
          "stem": "一个组合的年化收益为25%，最大回撤为20%。请计算该组合的卡尔玛比率，并解释其含义。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "information_ratio",
      "coverage_tags": [
        "information_ratio"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_ir_interpretation",
      "learning_goal": "学生能根据信息比率的值判断组合相对于基准的表现。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "信息比率",
          "en": "Information Ratio"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "信息比率大于0，表示组合相对于基准有正的超额收益。0.8 是一个不错的水平。",
          "options": [
            "该组合的表现优于基准",
            "该组合的表现与基准持平",
            "该组合的表现劣于基准",
            "无法判断"
          ],
          "question_id": "q_quiz_ir_int_v1",
          "stem": "一个组合的信息比率（IR）为 0.8，以下哪个说法最准确？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "信息比率为负，表示组合相对于基准有负的超额收益，表现劣于基准。",
          "options": [
            "该组合的表现优于基准",
            "该组合的表现劣于基准",
            "该组合的跟踪误差很大",
            "该组合的夏普比率一定为负"
          ],
          "question_id": "q_quiz_ir_int_v2",
          "stem": "一个组合的信息比率（IR）为 -0.3，以下哪个说法最准确？"
        }
      ]
    },
    {
      "concept_key": "maximum_drawdown",
      "coverage_tags": [
        "maximum_drawdown"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_mdd_interpretation",
      "learning_goal": "学生能根据净值序列计算或判断最大回撤。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "最大回撤",
          "en": "Maximum Drawdown"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 30,
          "explanation": "峰值是130，谷值是90，MDD = (130-90)/130 = 30.77%。",
          "options": [
            "10%",
            "23.08%",
            "30.77%",
            "35.71%"
          ],
          "question_id": "q_quiz_mdd_int_v1",
          "stem": "一个组合的净值序列为 [100, 120, 110, 130, 90, 140]，其最大回撤是多少？"
        },
        {
          "answer": 0,
          "estimated_seconds": 30,
          "explanation": "峰值是100，谷值是80，MDD = (100-80)/100 = 20%。注意，后续的120不是从100开始的峰值，但回撤是从120到95，为20.83%，但最大回撤是20%。",
          "options": [
            "20%",
            "15%",
            "25%",
            "12.5%"
          ],
          "question_id": "q_quiz_mdd_int_v2",
          "stem": "一个组合的净值序列为 [100, 80, 110, 105, 120, 95]，其最大回撤是多少？"
        }
      ]
    },
    {
      "concept_key": "calmar_ratio",
      "coverage_tags": [
        "calmar_ratio"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_calmar_interpretation",
      "learning_goal": "学生能根据卡尔玛比率的值判断组合的风险调整后收益。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "卡尔玛比率",
          "en": "Calmar Ratio"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 25,
          "explanation": "组合A的卡尔玛比率 = 15%/10% = 1.5；组合B的卡尔玛比率 = 20%/20% = 1.0。组合A的卡尔玛比率更高。",
          "options": [
            "组合A的卡尔玛比率更高",
            "组合B的卡尔玛比率更高",
            "两个组合的卡尔玛比率相同",
            "无法比较"
          ],
          "question_id": "q_quiz_calmar_int_v1",
          "stem": "组合A的年化收益为15%，最大回撤为10%；组合B的年化收益为20%，最大回撤为20%。以下哪个说法正确？"
        },
        {
          "answer": 0,
          "estimated_seconds": 25,
          "explanation": "卡尔玛比率 = 年化收益 / |最大回撤|，所以 |最大回撤| = 12% / 2.0 = 6%。",
          "options": [
            "6%",
            "12%",
            "24%",
            "2%"
          ],
          "question_id": "q_quiz_calmar_int_v2",
          "stem": "一个组合的卡尔玛比率为2.0，年化收益为12%，其最大回撤是多少？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L7/plain.txt",
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
      "coverage_tag": "portfolio_definition",
      "covered_by": [
        "qf_flash_portfolio_def",
        "qf_quiz_portfolio_def"
      ],
      "description": "投资组合的定义：由多种资产构成的集合。"
    },
    {
      "coverage_tag": "portfolio_optimization_goal",
      "covered_by": [
        "qf_flash_optimization_goal",
        "qf_quiz_optimization_goal"
      ],
      "description": "投资组合优化的目标：在最大化收益的同时最小化风险。"
    },
    {
      "coverage_tag": "markowitz_theory",
      "covered_by": [
        "qf_flash_markowitz",
        "qf_quiz_markowitz"
      ],
      "description": "马科维茨投资组合理论（现代投资组合理论），1990年诺贝尔奖。"
    },
    {
      "coverage_tag": "expected_portfolio_return",
      "covered_by": [
        "qf_flash_expected_return",
        "qf_quiz_expected_return",
        "qf_long_expected_return_calc"
      ],
      "description": "组合期望收益率是各资产期望收益率的加权平均。"
    },
    {
      "coverage_tag": "portfolio_variance_formula",
      "covered_by": [
        "qf_flash_variance_formula",
        "qf_quiz_variance_formula",
        "qf_long_variance_calc"
      ],
      "description": "组合方差公式：σ_p² = w_A²σ_A² + w_B²σ_B² + 2w_A w_B σ_AB。"
    },
    {
      "coverage_tag": "weight_constraint",
      "covered_by": [
        "qf_flash_weight_constraint",
        "qf_quiz_weight_constraint"
      ],
      "description": "组合中所有资产的权重之和必须等于1。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "portfolio_definition",
      "coverage_tags": [
        "portfolio_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_portfolio_def",
      "learning_goal": "学生能准确说出投资组合的定义。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "投资组合的核心定义：多种资产的集合。",
      "term_refs": [
        {
          "display": "投资组合",
          "en": "Portfolio"
        }
      ],
      "variants": [
        {
          "back": "由多种资产（如股票、债券、外汇等）构成的集合。",
          "estimated_seconds": 8,
          "explanation": "投资组合的核心是'多种资产'的集合，而不是单一资产。",
          "front": "什么是投资组合？",
          "question_id": "q_flash_portfolio_def_v1"
        },
        {
          "back": "一个投资组合。",
          "estimated_seconds": 5,
          "explanation": "多种不同类型的资产组合在一起就是投资组合。",
          "front": "把股票、债券和外汇放在一起，构成了什么？",
          "question_id": "q_flash_portfolio_def_v2"
        }
      ]
    },
    {
      "concept_key": "portfolio_optimization_goal",
      "coverage_tags": [
        "portfolio_optimization_goal"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_optimization_goal",
      "learning_goal": "学生能说出投资组合优化的两个核心目标。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "投资组合优化的双重目标：收益最大化和风险最小化。",
      "term_refs": [
        {
          "display": "投资组合优化",
          "en": "Portfolio Optimization"
        }
      ],
      "variants": [
        {
          "back": "最大化收益，同时最小化风险。",
          "estimated_seconds": 8,
          "explanation": "优化过程就是在收益和风险之间寻找最佳平衡。",
          "front": "投资组合优化的两个核心目标是什么？",
          "question_id": "q_flash_optimization_goal_v1"
        },
        {
          "back": "每种资产该买多少，才能在收益最大化的同时把风险降到最低。",
          "estimated_seconds": 10,
          "explanation": "权重分配是优化的关键。",
          "front": "投资组合优化要解决的核心问题是什么？",
          "question_id": "q_flash_optimization_goal_v2"
        }
      ]
    },
    {
      "concept_key": "markowitz_theory",
      "coverage_tags": [
        "markowitz_theory"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_markowitz",
      "learning_goal": "学生能说出马科维茨投资组合理论的核心内容和获奖年份。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "马科维茨理论的核心：现代投资组合理论，1990年诺贝尔奖。",
      "term_refs": [
        {
          "display": "马科维茨投资组合理论",
          "en": "Markowitz Portfolio Theory"
        }
      ],
      "variants": [
        {
          "back": "1990年。",
          "estimated_seconds": 5,
          "explanation": "马科维茨因现代投资组合理论获得1990年诺贝尔经济学奖。",
          "front": "马科维茨投资组合理论在哪一年获得了诺贝尔奖？",
          "question_id": "q_flash_markowitz_v1"
        },
        {
          "back": "现代投资组合理论。",
          "estimated_seconds": 5,
          "explanation": "Markowitz Portfolio Theory 也被称为 Modern Portfolio Theory (MPT)。",
          "front": "马科维茨投资组合理论的另一个常用名称是什么？",
          "question_id": "q_flash_markowitz_v2"
        }
      ]
    },
    {
      "concept_key": "expected_portfolio_return",
      "coverage_tags": [
        "expected_portfolio_return"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_expected_return",
      "learning_goal": "学生能说出组合期望收益率的计算方法。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "组合期望收益率 = 各资产期望收益率的加权平均。",
      "term_refs": [
        {
          "display": "期望收益率",
          "en": "Expected Return"
        }
      ],
      "variants": [
        {
          "back": "各资产期望收益率的加权平均，权重之和为1。",
          "estimated_seconds": 8,
          "explanation": "E(R_p) = w_A * E(R_A) + w_B * E(R_B) + ...",
          "front": "组合的期望收益率如何计算？",
          "question_id": "q_flash_expected_return_v1"
        },
        {
          "back": "11.25%。",
          "estimated_seconds": 10,
          "explanation": "0.5 * 15% + 0.5 * 7.5% = 11.25%。",
          "front": "资产A期望收益15%，资产B期望收益7.5%，各投一半，组合期望收益是多少？",
          "question_id": "q_flash_expected_return_v2"
        }
      ]
    },
    {
      "concept_key": "portfolio_variance_formula",
      "coverage_tags": [
        "portfolio_variance_formula"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_variance_formula",
      "learning_goal": "学生能写出两只资产组合的方差公式。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "两只资产组合方差公式的构成项。",
      "term_refs": [
        {
          "display": "投资组合方差",
          "en": "Portfolio Variance"
        }
      ],
      "variants": [
        {
          "back": "w_A²σ_A² + w_B²σ_B² + 2w_A w_B σ_AB。",
          "estimated_seconds": 12,
          "explanation": "第一项是A的方差贡献，第二项是B的方差贡献，第三项是协方差贡献。",
          "front": "两只资产组合的方差公式中，包含哪三项？",
          "question_id": "q_flash_variance_formula_v1"
        },
        {
          "back": "资产A和资产B之间的协方差对组合风险的贡献。",
          "estimated_seconds": 10,
          "explanation": "协方差项反映了资产之间的相关性对组合风险的影响。",
          "front": "组合方差公式中，第三项 2w_A w_B σ_AB 代表什么？",
          "question_id": "q_flash_variance_formula_v2"
        }
      ]
    },
    {
      "concept_key": "weight_constraint",
      "coverage_tags": [
        "weight_constraint"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_weight_constraint",
      "learning_goal": "学生能说出组合权重的约束条件。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "所有资产权重之和必须等于1。",
      "term_refs": [
        {
          "display": "权重约束",
          "en": "Weight Constraint"
        }
      ],
      "variants": [
        {
          "back": "1（或100%）。",
          "estimated_seconds": 5,
          "explanation": "∑w_i = 1 是组合权重的基本约束。",
          "front": "投资组合中所有资产的权重之和必须等于多少？",
          "question_id": "q_flash_weight_constraint_v1"
        },
        {
          "back": "权重之和等于1（0.4 + 0.6 = 1）。",
          "estimated_seconds": 5,
          "explanation": "这是组合权重的基本约束条件。",
          "front": "如果组合中资产A的权重是0.4，资产B的权重是0.6，这满足什么条件？",
          "question_id": "q_flash_weight_constraint_v2"
        }
      ]
    }
  ],
  "lesson_id": "L7",
  "longform_families": [
    {
      "concept_key": "expected_portfolio_return",
      "coverage_tags": [
        "expected_portfolio_return"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_expected_return_calc",
      "learning_goal": "学生能根据给定的资产期望收益率和权重，计算组合的期望收益率。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "worked_example",
      "term_refs": [
        {
          "display": "期望收益率",
          "en": "Expected Return"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "写出组合期望收益率的公式",
            "代入数值计算",
            "给出最终结果"
          ],
          "question_id": "q_long_expected_return_calc_v1",
          "reference_answer": [
            "公式：E(R_p) = w_A * E(R_A) + w_B * E(R_B) + w_C * E(R_C)",
            "代入：0.4 * 12% + 0.35 * 8% + 0.25 * 15%",
            "计算：4.8% + 2.8% + 3.75% = 11.35%",
            "该组合的期望收益率为11.35%。"
          ],
          "rubric_points": [
            "正确写出公式 E(R_p) = ∑w_i * E(R_i)",
            "正确代入数值：0.4*12% + 0.35*8% + 0.25*15%",
            "正确计算结果：4.8% + 2.8% + 3.75% = 11.35%"
          ],
          "stem": "一个组合包含三种资产：资产A（权重40%，期望收益12%）、资产B（权重35%，期望收益8%）、资产C（权重25%，期望收益15%）。请计算该组合的期望收益率。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "写出组合期望收益率的公式",
            "代入数值计算",
            "给出最终结果"
          ],
          "question_id": "q_long_expected_return_calc_v2",
          "reference_answer": [
            "公式：E(R_p) = w_X * E(R_X) + w_Y * E(R_Y)",
            "代入：0.7 * 18% + 0.3 * 6%",
            "计算：12.6% + 1.8% = 14.4%",
            "该组合的期望收益率为14.4%。"
          ],
          "rubric_points": [
            "正确写出公式 E(R_p) = w_X * E(R_X) + w_Y * E(R_Y)",
            "正确代入数值：0.7*18% + 0.3*6%",
            "正确计算结果：12.6% + 1.8% = 14.4%"
          ],
          "stem": "一个组合包含两只资产：资产X（权重70%，期望收益18%）、资产Y（权重30%，期望收益6%）。请计算该组合的期望收益率。"
        }
      ]
    },
    {
      "concept_key": "portfolio_variance_formula",
      "coverage_tags": [
        "portfolio_variance_formula"
      ],
      "difficulty": "hard",
      "family_id": "qf_long_variance_calc",
      "learning_goal": "学生能根据给定的资产方差、协方差和权重，计算两只资产组合的方差。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "worked_example",
      "term_refs": [
        {
          "display": "投资组合方差",
          "en": "Portfolio Variance"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "写出两只资产组合的方差公式",
            "代入数值计算每一项",
            "求和得到组合方差"
          ],
          "question_id": "q_long_variance_calc_v1",
          "reference_answer": [
            "公式：σ_p² = w_A²σ_A² + w_B²σ_B² + 2w_A w_B σ_AB",
            "第一项：0.6² * 0.04 = 0.0144",
            "第二项：0.4² * 0.09 = 0.0144",
            "第三项：2 * 0.6 * 0.4 * 0.02 = 0.0096",
            "组合方差 = 0.0144 + 0.0144 + 0.0096 = 0.0384"
          ],
          "rubric_points": [
            "正确写出公式 σ_p² = w_A²σ_A² + w_B²σ_B² + 2w_A w_B σ_AB",
            "正确计算第一项：0.6² * 0.04 = 0.36 * 0.04 = 0.0144",
            "正确计算第二项：0.4² * 0.09 = 0.16 * 0.09 = 0.0144",
            "正确计算第三项：2 * 0.6 * 0.4 * 0.02 = 0.0096",
            "正确求和：0.0144 + 0.0144 + 0.0096 = 0.0384"
          ],
          "stem": "一个组合包含两只资产：资产A（权重60%，方差0.04）、资产B（权重40%，方差0.09），A和B的协方差为0.02。请计算该组合的方差。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "写出两只资产组合的方差公式",
            "代入数值计算每一项",
            "求和得到组合方差"
          ],
          "question_id": "q_long_variance_calc_v2",
          "reference_answer": [
            "公式：σ_p² = w_P²σ_P² + w_Q²σ_Q² + 2w_P w_Q σ_PQ",
            "第一项：0.5² * 0.01 = 0.0025",
            "第二项：0.5² * 0.04 = 0.01",
            "第三项：2 * 0.5 * 0.5 * 0.005 = 0.0025",
            "组合方差 = 0.0025 + 0.01 + 0.0025 = 0.015"
          ],
          "rubric_points": [
            "正确写出公式 σ_p² = w_P²σ_P² + w_Q²σ_Q² + 2w_P w_Q σ_PQ",
            "正确计算第一项：0.5² * 0.01 = 0.25 * 0.01 = 0.0025",
            "正确计算第二项：0.5² * 0.04 = 0.25 * 0.04 = 0.01",
            "正确计算第三项：2 * 0.5 * 0.5 * 0.005 = 0.0025",
            "正确求和：0.0025 + 0.01 + 0.0025 = 0.015"
          ],
          "stem": "一个组合包含两只资产：资产P（权重50%，方差0.01）、资产Q（权重50%，方差0.04），P和Q的协方差为0.005。请计算该组合的方差。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "portfolio_definition",
      "coverage_tags": [
        "portfolio_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_portfolio_def",
      "learning_goal": "学生能在选择题中识别投资组合的正确描述。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "投资组合",
          "en": "Portfolio"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "投资组合是由多种资产（如股票、债券、外汇等）构成的集合。",
          "options": [
            "单一的一种资产",
            "多种资产构成的集合",
            "只包含股票的投资",
            "一种风险管理工具"
          ],
          "question_id": "q_quiz_portfolio_def_v1",
          "stem": "以下哪一项最准确地描述了投资组合？"
        },
        {
          "answer": 3,
          "estimated_seconds": 15,
          "explanation": "个人信用评分不是一种可交易的金融资产，不属于投资组合的构成部分。",
          "options": [
            "股票",
            "债券",
            "外汇",
            "个人信用评分"
          ],
          "question_id": "q_quiz_portfolio_def_v2",
          "stem": "以下哪一项不是投资组合中常见的资产类型？"
        }
      ]
    },
    {
      "concept_key": "portfolio_optimization_goal",
      "coverage_tags": [
        "portfolio_optimization_goal"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_optimization_goal",
      "learning_goal": "学生能识别投资组合优化的核心目标。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "投资组合优化",
          "en": "Portfolio Optimization"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "投资组合优化是在收益和风险之间寻找最佳平衡，即最大化收益的同时最小化风险。",
          "options": [
            "只追求最大收益",
            "只追求最小风险",
            "在最大化收益的同时最小化风险",
            "保证每年正收益"
          ],
          "question_id": "q_quiz_optimization_goal_v1",
          "stem": "投资组合优化的核心目标是什么？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "投资组合优化的核心是确定每种资产的最佳权重分配。",
          "options": [
            "如何选择最好的单一资产",
            "每种资产该分配多少权重",
            "如何预测市场走势",
            "如何降低交易成本"
          ],
          "question_id": "q_quiz_optimization_goal_v2",
          "stem": "投资组合优化主要解决什么问题？"
        }
      ]
    },
    {
      "concept_key": "markowitz_theory",
      "coverage_tags": [
        "markowitz_theory"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_markowitz",
      "learning_goal": "学生能判断关于马科维茨理论的陈述是否正确。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "true_false",
      "term_refs": [
        {
          "display": "马科维茨投资组合理论",
          "en": "Markowitz Portfolio Theory"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 10,
          "explanation": "马科维茨投资组合理论在1990年获得了诺贝尔奖，不是2000年。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_markowitz_v1",
          "stem": "马科维茨投资组合理论在2000年获得了诺贝尔奖。"
        },
        {
          "answer": 0,
          "estimated_seconds": 10,
          "explanation": "Markowitz Portfolio Theory 确实也被称为 Modern Portfolio Theory (MPT)。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_markowitz_v2",
          "stem": "马科维茨投资组合理论也被称为现代投资组合理论。"
        }
      ]
    },
    {
      "concept_key": "expected_portfolio_return",
      "coverage_tags": [
        "expected_portfolio_return"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_expected_return",
      "learning_goal": "学生能计算简单的组合期望收益率。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "期望收益率",
          "en": "Expected Return"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "E(R_p) = 0.3 * 20% + 0.7 * 10% = 6% + 7% = 13%。",
          "options": [
            "15%",
            "13%",
            "17%",
            "30%"
          ],
          "question_id": "q_quiz_expected_return_v1",
          "stem": "资产A的期望收益是20%，资产B的期望收益是10%。如果组合中A占30%，B占70%，组合的期望收益率是多少？"
        },
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "E(R_p) = 0.6 * 12% + 0.4 * 8% = 7.2% + 3.2% = 10.4%。",
          "options": [
            "10.4%",
            "9.6%",
            "11.2%",
            "20%"
          ],
          "question_id": "q_quiz_expected_return_v2",
          "stem": "资产X的期望收益是12%，资产Y的期望收益是8%。如果组合中X占60%，Y占40%，组合的期望收益率是多少？"
        }
      ]
    },
    {
      "concept_key": "portfolio_variance_formula",
      "coverage_tags": [
        "portfolio_variance_formula"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_variance_formula",
      "learning_goal": "学生能识别组合方差公式的正确形式。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "投资组合方差",
          "en": "Portfolio Variance"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "协方差项 2w_A w_B σ_AB 反映了资产A和B之间的相关性对组合风险的贡献。",
          "options": [
            "w_A²σ_A²",
            "w_B²σ_B²",
            "2w_A w_B σ_AB",
            "w_A σ_A + w_B σ_B"
          ],
          "question_id": "q_quiz_variance_formula_v1",
          "stem": "两只资产组合的方差公式中，哪一项反映了资产之间的相关性对风险的影响？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "两只资产组合的方差公式包含三项：各自的方差贡献和协方差贡献。",
          "options": [
            "σ_p² = w_A²σ_A² + w_B²σ_B²",
            "σ_p² = w_A²σ_A² + w_B²σ_B² + 2w_A w_B σ_AB",
            "σ_p² = w_Aσ_A + w_Bσ_B",
            "σ_p² = w_A²σ_A² + w_B²σ_B² + w_A w_B σ_AB"
          ],
          "question_id": "q_quiz_variance_formula_v2",
          "stem": "以下哪个公式是两只资产组合的正确方差公式？"
        }
      ]
    },
    {
      "concept_key": "weight_constraint",
      "coverage_tags": [
        "weight_constraint"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_weight_constraint",
      "learning_goal": "学生能判断关于权重约束的陈述是否正确。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "true_false",
      "term_refs": [
        {
          "display": "权重约束",
          "en": "Weight Constraint"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 10,
          "explanation": "所有资产的权重之和必须等于1（或100%），而不是小于1。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_weight_constraint_v1",
          "stem": "投资组合中所有资产的权重之和必须小于1。"
        },
        {
          "answer": 0,
          "estimated_seconds": 10,
          "explanation": "0.6 + 0.4 = 1，满足权重之和等于1的约束条件。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_weight_constraint_v2",
          "stem": "如果组合中资产A的权重是0.6，资产B的权重是0.4，这个组合的权重分配是有效的。"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "Calculating risk and return of an investment portfolio; Portfolio return and risk; Portfolio Variance; Expected Portfolio Return; Markowitz Portfolio Theory; Investment Portfolio; Portfolio optimization",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "Portfolio return and risk basics",
    "plain_text": "pipeline/1-plain/L7/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "Portfolio Return and Risk; Expected Portfolio Return; Portfolio Variance; Markowitz Portfolio Theory; Investment Portfolio; Portfolio optimization"
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "portfolio_return_and_risk",
      "covered_by": [
        "qf_flash_portfolio_return_risk",
        "qf_quiz_portfolio_return_risk"
      ],
      "description": "投资组合的收益是各资产收益的加权平均，风险（方差）取决于资产权重、方差和协方差。"
    },
    {
      "coverage_tag": "correlation_diversification",
      "covered_by": [
        "qf_flash_correlation_diversification",
        "qf_quiz_correlation_diversification"
      ],
      "description": "相关系数越低，分散风险效果越好；完全正相关时分散无效，完全负相关时风险可降至零。"
    },
    {
      "coverage_tag": "minimum_variance_portfolio",
      "covered_by": [
        "qf_flash_minimum_variance_portfolio",
        "qf_quiz_minimum_variance_portfolio"
      ],
      "description": "最小方差组合是所有可能组合中风险（标准差）最低的组合，其权重可通过求导得到。"
    },
    {
      "coverage_tag": "efficient_frontier",
      "covered_by": [
        "qf_flash_efficient_frontier",
        "qf_quiz_efficient_frontier"
      ],
      "description": "有效前沿是在给定风险下能提供最高预期收益的投资组合集合，位于机会集的上边缘。"
    },
    {
      "coverage_tag": "tangency_portfolio_and_cml",
      "covered_by": [
        "qf_flash_tangency_portfolio_cml",
        "qf_quiz_tangency_portfolio_cml"
      ],
      "description": "切点组合是无风险资产与有效前沿相切处的风险资产组合；资本市场线（CML）连接无风险利率与市场组合。"
    },
    {
      "coverage_tag": "capm_and_sml",
      "covered_by": [
        "qf_flash_capm_sml",
        "qf_quiz_capm_sml",
        "qf_long_capm_application"
      ],
      "description": "CAPM用贝塔系数描述资产收益与市场收益的线性关系；证券市场线（SML）是其图形化表示。"
    },
    {
      "coverage_tag": "performance_measures",
      "covered_by": [
        "qf_flash_performance_measures",
        "qf_quiz_performance_measures",
        "qf_long_performance_measures_comparison"
      ],
      "description": "夏普比率、索提诺比率、特雷诺比率、詹森阿尔法、信息比率、最大回撤、卡尔玛比率等风险调整后的收益指标。"
    },
    {
      "coverage_tag": "stop_loss_and_risk_limit",
      "covered_by": [
        "qf_flash_stop_loss_risk_limit",
        "qf_quiz_stop_loss_risk_limit"
      ],
      "description": "止损（固定止损、移动止损、组合止损）和风险限额（日/周/月止损、VaR限额等）是保护资本的底线。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "portfolio_return_and_risk",
      "coverage_tags": [
        "portfolio_return_and_risk"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_portfolio_return_risk",
      "learning_goal": "学生能准确回忆投资组合期望收益率和方差的计算公式及其关键组成部分。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "组合收益与风险计算的核心公式。",
      "term_refs": [
        {
          "display": "投资组合期望收益率",
          "en": "Expected Portfolio Return"
        },
        {
          "display": "投资组合方差",
          "en": "Portfolio Variance"
        }
      ],
      "variants": [
        {
          "back": "$E(R_p) = \\sum_{i=1}^{n} w_i E(R_i)$，其中 $w_i$ 是资产 $i$ 的权重，且 $\\sum w_i = 1$。",
          "estimated_seconds": 10,
          "explanation": "组合期望收益率是各资产期望收益率的加权平均。",
          "front": "投资组合的期望收益率 $E(R_p)$ 如何计算？请写出公式。",
          "question_id": "q_flash_portfolio_return_risk_v1"
        },
        {
          "back": "$\\sigma_p^2 = w_A^2 \\sigma_A^2 + w_B^2 \\sigma_B^2 + 2 w_A w_B \\sigma_{AB}$，其中 $\\sigma_{AB}$ 是A和B的协方差。",
          "estimated_seconds": 12,
          "explanation": "组合方差不仅取决于各资产的方差和权重，还取决于资产间的协方差。",
          "front": "对于两只资产A和B，投资组合方差 $\\sigma_p^2$ 的公式是什么？",
          "question_id": "q_flash_portfolio_return_risk_v2"
        }
      ]
    },
    {
      "concept_key": "correlation_diversification",
      "coverage_tags": [
        "correlation_diversification"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_correlation_diversification",
      "learning_goal": "学生能准确回忆相关系数对分散风险效果的影响。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "相关系数与分散效果的关系。",
      "term_refs": [
        {
          "display": "相关系数",
          "en": "Correlation Coefficient"
        },
        {
          "display": "风险分散",
          "en": "Diversification"
        }
      ],
      "variants": [
        {
          "back": "$\\rho = -1$（完全负相关），此时理论上可以将组合风险降至零。",
          "estimated_seconds": 8,
          "explanation": "完全负相关时，可以通过调整权重使组合风险为零。",
          "front": "当两只资产的相关系数 $\\rho$ 等于多少时，分散风险的效果最好？",
          "question_id": "q_flash_correlation_diversification_v1"
        },
        {
          "back": "各资产标准差的加权平均：$\\sigma_p = w_A \\sigma_A + w_B \\sigma_B$。",
          "estimated_seconds": 10,
          "explanation": "完全正相关时，分散无效，组合风险就是各资产风险的加权平均。",
          "front": "当两只资产完全正相关（$\\rho=1$）时，组合的标准差等于什么？",
          "question_id": "q_flash_correlation_diversification_v2"
        }
      ]
    },
    {
      "concept_key": "minimum_variance_portfolio",
      "coverage_tags": [
        "minimum_variance_portfolio"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_minimum_variance_portfolio",
      "learning_goal": "学生能准确回忆最小方差组合的定义和求解思路。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "最小方差组合的定义和求解条件。",
      "term_refs": [
        {
          "display": "最小方差组合",
          "en": "Minimum Variance Portfolio"
        }
      ],
      "variants": [
        {
          "back": "在所有可能的投资组合中，风险（标准差或方差）最低的那个组合。",
          "estimated_seconds": 8,
          "explanation": "MVP是机会集上风险最小的点。",
          "front": "什么是最小方差组合（MVP）？",
          "question_id": "q_flash_minimum_variance_portfolio_v1"
        },
        {
          "back": "导数等于零（$\\frac{\\partial \\sigma_p^2}{\\partial w} = 0$），此时方差取得全局最小值。",
          "estimated_seconds": 10,
          "explanation": "通过求导并令导数为零，可以找到使方差最小的权重。",
          "front": "求解最小方差组合时，对组合方差的导数应满足什么条件？",
          "question_id": "q_flash_minimum_variance_portfolio_v2"
        }
      ]
    },
    {
      "concept_key": "efficient_frontier",
      "coverage_tags": [
        "efficient_frontier"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_efficient_frontier",
      "learning_goal": "学生能准确回忆有效前沿的定义和特征。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "有效前沿的定义。",
      "term_refs": [
        {
          "display": "有效前沿",
          "en": "Efficient Frontier"
        }
      ],
      "variants": [
        {
          "back": "在给定风险水平下能提供最高预期收益的投资组合集合，位于机会集的上边缘。",
          "estimated_seconds": 10,
          "explanation": "有效前沿上的组合都是最优的，不在其上的组合是低效的。",
          "front": "有效前沿（Efficient Frontier）是什么？",
          "question_id": "q_flash_efficient_frontier_v1"
        },
        {
          "back": "它是低效的：在相同的风险下，它的收益更低；或者在相同的收益下，它的风险更高。",
          "estimated_seconds": 10,
          "explanation": "理性投资者应只考虑有效前沿上的组合。",
          "front": "如果一个组合不在有效前沿上，它有什么问题？",
          "question_id": "q_flash_efficient_frontier_v2"
        }
      ]
    },
    {
      "concept_key": "tangency_portfolio_and_cml",
      "coverage_tags": [
        "tangency_portfolio_and_cml"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_tangency_portfolio_cml",
      "learning_goal": "学生能准确回忆切点组合和资本市场线的定义。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "切点组合和CML的定义。",
      "term_refs": [
        {
          "display": "切点组合",
          "en": "Tangency Portfolio"
        },
        {
          "display": "资本市场线",
          "en": "Capital Market Line"
        }
      ],
      "variants": [
        {
          "back": "当存在无风险资产时，从无风险利率出发画一条与有效前沿相切的直线，切点对应的风险资产组合就是切点组合。",
          "estimated_seconds": 12,
          "explanation": "切点组合是所有理性投资者持有的最优风险组合。",
          "front": "什么是切点组合（Tangency Portfolio）？",
          "question_id": "q_flash_tangency_portfolio_cml_v1"
        },
        {
          "back": "有效投资组合的预期收益 $E(R_p)$ 与其总风险 $\\sigma_p$ 之间的线性关系：$E(R_p) = R_f + \\frac{E(R_M)-R_f}{\\sigma_M} \\sigma_p$。",
          "estimated_seconds": 15,
          "explanation": "CML上的所有点都是无风险资产与市场组合的有效配置。",
          "front": "资本市场线（CML）描述的是什么关系？",
          "question_id": "q_flash_tangency_portfolio_cml_v2"
        }
      ]
    },
    {
      "concept_key": "capm_and_sml",
      "coverage_tags": [
        "capm_and_sml"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_capm_sml",
      "learning_goal": "学生能准确回忆CAPM公式、贝塔系数的含义和SML的定义。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "CAPM核心公式和贝塔的含义。",
      "term_refs": [
        {
          "display": "资本资产定价模型",
          "en": "Capital Asset Pricing Model"
        },
        {
          "display": "贝塔系数",
          "en": "Beta"
        },
        {
          "display": "证券市场线",
          "en": "Security Market Line"
        }
      ],
      "variants": [
        {
          "back": "$E(R_i) = R_f + \\beta_i (E(R_M) - R_f)$",
          "estimated_seconds": 8,
          "explanation": "资产的期望收益等于无风险利率加上贝塔系数乘以市场风险溢价。",
          "front": "写出资本资产定价模型（CAPM）的公式。",
          "question_id": "q_flash_capm_sml_v1"
        },
        {
          "back": "资产 $i$ 的收益对市场收益的敏感程度，即系统性风险。",
          "estimated_seconds": 10,
          "explanation": "$\\beta_i = \\frac{\\sigma_{iM}}{\\sigma_M^2}$。$\\beta > 1$ 表示比市场更激进，$\\beta < 1$ 表示比市场更保守。",
          "front": "贝塔系数 $\\beta_i$ 衡量的是什么？",
          "question_id": "q_flash_capm_sml_v2"
        },
        {
          "back": "CAPM的图形表示，展示单个资产（或组合）的预期收益与其贝塔值之间的线性关系。",
          "estimated_seconds": 12,
          "explanation": "在SML上，所有资产都应该落在这条线上。落在线上方表示被低估（$\\alpha > 0$），下方表示被高估（$\\alpha < 0$）。",
          "front": "证券市场线（SML）是什么？",
          "question_id": "q_flash_capm_sml_v3"
        }
      ]
    },
    {
      "concept_key": "performance_measures",
      "coverage_tags": [
        "performance_measures"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_performance_measures",
      "learning_goal": "学生能准确回忆各绩效衡量指标的定义、公式和适用场景。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "各绩效指标的核心公式和关键区别。",
      "term_refs": [
        {
          "display": "夏普比率",
          "en": "Sharpe Ratio"
        },
        {
          "display": "索提诺比率",
          "en": "Sortino Ratio"
        },
        {
          "display": "特雷诺比率",
          "en": "Treynor Ratio"
        },
        {
          "display": "詹森阿尔法",
          "en": "Jensen's Alpha"
        },
        {
          "display": "信息比率",
          "en": "Information Ratio"
        },
        {
          "display": "最大回撤",
          "en": "Maximum Drawdown"
        },
        {
          "display": "卡尔玛比率",
          "en": "Calmar Ratio"
        }
      ],
      "variants": [
        {
          "back": "$SR_p = \\frac{E(R_p) - R_f}{\\sigma_p}$。它衡量每单位总风险（标准差）所获得的超额收益。",
          "estimated_seconds": 10,
          "explanation": "夏普比率越高，表示风险调整后的收益越好。",
          "front": "夏普比率（Sharpe Ratio）的公式是什么？它衡量什么？",
          "question_id": "q_flash_performance_measures_v1"
        },
        {
          "back": "索提诺比率只考虑下行风险（下行标准差），而夏普比率考虑总风险（总标准差）。",
          "estimated_seconds": 10,
          "explanation": "投资者通常更讨厌下跌，索提诺比率更符合这一偏好。",
          "front": "索提诺比率（Sortino Ratio）与夏普比率的主要区别是什么？",
          "question_id": "q_flash_performance_measures_v2"
        },
        {
          "back": "$TR_p = \\frac{E(R_p) - R_f}{\\beta_p}$。适用于组合已经充分分散、非系统性风险几乎为零的情况。",
          "estimated_seconds": 12,
          "explanation": "特雷诺比率衡量每单位系统性风险（贝塔）所获得的超额收益。",
          "front": "特雷诺比率（Treynor Ratio）的公式是什么？它适用于什么情况？",
          "question_id": "q_flash_performance_measures_v3"
        },
        {
          "back": "$\\alpha_p = E(R_p) - [R_f + \\beta_p (E(R_M) - R_f)]$。正值表示组合的实际收益超过了CAPM给出的预期收益，即基金经理创造了超额价值。",
          "estimated_seconds": 12,
          "explanation": "詹森阿尔法直接衡量基金经理的主动管理能力。",
          "front": "詹森阿尔法（Jensen's Alpha）的公式是什么？正值表示什么？",
          "question_id": "q_flash_performance_measures_v4"
        }
      ]
    },
    {
      "concept_key": "stop_loss_and_risk_limit",
      "coverage_tags": [
        "stop_loss_and_risk_limit"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_stop_loss_risk_limit",
      "learning_goal": "学生能准确回忆不同类型止损的定义和特点。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "各类止损的核心机制。",
      "term_refs": [
        {
          "display": "止损",
          "en": "Stop Loss"
        },
        {
          "display": "移动止损",
          "en": "Trailing Stop Loss"
        },
        {
          "display": "组合止损",
          "en": "Portfolio Stop Loss"
        }
      ],
      "variants": [
        {
          "back": "设定一个具体的价格水平，当资产价格跌至该水平时，自动平仓以限制最大亏损。",
          "estimated_seconds": 8,
          "explanation": "例如，100元买入，设止损90元，最大亏损锁定在10元。",
          "front": "固定止损（Fixed Stop Loss）是如何工作的？",
          "question_id": "q_flash_stop_loss_risk_limit_v1"
        },
        {
          "back": "它能随着价格上涨而自动上移止损线，从而锁定已实现的利润，同时允许价格继续上涨。",
          "estimated_seconds": 10,
          "explanation": "移动止损不会下移，因此能保护利润。",
          "front": "移动止损（Trailing Stop Loss）相比固定止损的最大优势是什么？",
          "question_id": "q_flash_stop_loss_risk_limit_v2"
        },
        {
          "back": "作为第二层保护，当整个投资组合的净值从高点回撤一定比例时，全部清仓，以保护整体资本。",
          "estimated_seconds": 10,
          "explanation": "当多个策略同时运行时，单个策略的止损可能不足，组合止损提供全局保护。",
          "front": "组合止损（Portfolio Stop Loss）的作用是什么？",
          "question_id": "q_flash_stop_loss_risk_limit_v3"
        }
      ]
    }
  ],
  "lesson_id": "L7",
  "longform_families": [
    {
      "concept_key": "capm_and_sml",
      "coverage_tags": [
        "capm_and_sml"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_capm_application",
      "learning_goal": "学生能解释CAPM的核心思想、贝塔的含义以及SML的应用。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "资本资产定价模型",
          "en": "Capital Asset Pricing Model"
        },
        {
          "display": "贝塔系数",
          "en": "Beta"
        },
        {
          "display": "证券市场线",
          "en": "Security Market Line"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "CAPM的核心思想",
            "贝塔系数的含义",
            "计算期望收益率"
          ],
          "question_id": "q_long_capm_application_v1",
          "reference_answer": [
            "CAPM的核心思想是：在均衡市场中，任何风险资产的预期收益率等于无风险利率加上一个风险溢价，该风险溢价由资产的系统性风险（贝塔）决定。",
            "贝塔系数衡量资产收益对市场收益的敏感程度。$\\beta > 1$ 表示资产比市场更激进（波动更大），$\\beta < 1$ 表示比市场更保守。",
            "期望收益率 $= 3\\% + 1.5 \\times (10\\% - 3\\%) = 3\\% + 10.5\\% = 13.5\\%$。"
          ],
          "rubric_points": [
            "正确阐述CAPM描述资产预期收益与市场风险之间的线性关系",
            "正确解释贝塔衡量资产收益对市场收益的敏感程度",
            "正确应用公式 $E(R_i) = R_f + \\beta_i (E(R_M) - R_f)$ 进行计算"
          ],
          "stem": "请解释资本资产定价模型（CAPM）的核心思想，并说明贝塔系数（$\\beta$）的含义。如果一个资产的$\\beta=1.5$，无风险利率为3%，市场期望收益率为10%，根据CAPM，该资产的期望收益率是多少？"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "SML的定义",
            "资产点在SML上方/下方的含义",
            "SML与CML的区别"
          ],
          "question_id": "q_long_capm_application_v2",
          "reference_answer": [
            "证券市场线（SML）是CAPM的图形表示，横轴是贝塔系数，纵轴是预期收益，所有资产都应落在这条直线上。",
            "如果资产点落在SML上方，说明其实际收益高于CAPM给出的预期收益，即 $\\alpha > 0$，资产被低估，是好的投资机会。如果落在下方，说明 $\\alpha < 0$，资产被高估。",
            "SML与CML的主要区别：SML适用于所有单个资产和组合，横轴是系统性风险（贝塔）；CML只适用于有效组合，横轴是总风险（标准差）。"
          ],
          "rubric_points": [
            "正确描述SML是CAPM的图形表示",
            "正确解释上方表示被低估（$\\alpha > 0$），下方表示被高估（$\\alpha < 0$）",
            "正确指出SML使用贝塔（系统性风险）作为横轴，CML使用标准差（总风险）作为横轴"
          ],
          "stem": "什么是证券市场线（SML）？请解释如果一个资产的点落在SML上方或下方分别意味着什么。并说明SML与资本市场线（CML）的主要区别。"
        }
      ]
    },
    {
      "concept_key": "performance_measures",
      "coverage_tags": [
        "performance_measures"
      ],
      "difficulty": "hard",
      "family_id": "qf_long_performance_measures_comparison",
      "learning_goal": "学生能比较不同绩效衡量指标的适用场景和优缺点。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "compare_and_contrast",
      "term_refs": [
        {
          "display": "夏普比率",
          "en": "Sharpe Ratio"
        },
        {
          "display": "特雷诺比率",
          "en": "Treynor Ratio"
        },
        {
          "display": "詹森阿尔法",
          "en": "Jensen's Alpha"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 150,
          "prompt_blocks": [
            "夏普比率和特雷诺比率的公式和含义",
            "两者的相同点",
            "两者的不同点",
            "解释可能出现的矛盾情况"
          ],
          "question_id": "q_long_performance_measures_comparison_v1",
          "reference_answer": [
            "夏普比率：$SR_p = \\frac{E(R_p)-R_f}{\\sigma_p}$，衡量每单位总风险获得的超额收益。特雷诺比率：$TR_p = \\frac{E(R_p)-R_f}{\\beta_p}$，衡量每单位系统性风险获得的超额收益。",
            "相同点：两者都是风险调整后的收益指标，数值越高越好。",
            "不同点：夏普比率使用总风险（标准差），适用于组合代表全部投资的情况；特雷诺比率使用系统性风险（贝塔），适用于组合是大型投资组合的一部分的情况。",
            "矛盾情况：如果一个组合的非系统性风险很高（即分散化不足），其总风险（$\\sigma_p$）会很大，导致夏普比率较低；但如果其贝塔较小，特雷诺比率可能较高。这说明该组合虽然整体波动大，但系统性风险控制得不错，如果它只是大型投资组合的一部分，非系统性风险可以被分散掉，因此特雷诺比率更能反映其真实表现。"
          ],
          "rubric_points": [
            "正确写出两个比率的公式",
            "指出两者都衡量超额收益与风险的比值",
            "指出夏普比率使用总风险（标准差），特雷诺比率使用系统性风险（贝塔）",
            "正确解释当组合存在非系统性风险时，可能出现夏普比率低但特雷诺比率高的情况"
          ],
          "stem": "比较夏普比率（Sharpe Ratio）和特雷诺比率（Treynor Ratio）的异同。在什么情况下，一个组合的夏普比率可能显示其表现不如市场，但特雷诺比率却显示其表现优于市场？请解释原因。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "詹森阿尔法的定义和公式",
            "如何计算詹森阿尔法",
            "正阿尔法的含义"
          ],
          "question_id": "q_long_performance_measures_comparison_v2",
          "reference_answer": [
            "詹森阿尔法（$\\alpha_p$）的公式为：$\\alpha_p = E(R_p) - [R_f + \\beta_p (E(R_M) - R_f)]$。它衡量的是组合的实际收益与CAPM模型给出的预期收益之间的差值。",
            "计算时，首先根据CAPM计算出给定贝塔下的预期收益，然后用实际收益减去这个预期收益。",
            "正的詹森阿尔法（$\\alpha_p > 0$）说明基金经理通过主动管理（如选股、择时）创造了超越市场基准的超额价值，其表现优于CAPM模型的预测。负的阿尔法则说明表现不如预期。"
          ],
          "rubric_points": [
            "正确写出詹森阿尔法的公式",
            "正确解释阿尔法是实际收益与CAPM预期收益的差值",
            "正确解释正阿尔法表示基金经理创造了超额价值"
          ],
          "stem": "请解释詹森阿尔法（Jensen's Alpha）的含义和计算方法。如果一个基金经理声称其管理的基金具有正的詹森阿尔法，这说明了什么？请结合CAPM模型进行解释。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "portfolio_return_and_risk",
      "coverage_tags": [
        "portfolio_return_and_risk"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_portfolio_return_risk",
      "learning_goal": "学生能在测验情境下计算简单的投资组合期望收益率。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "投资组合期望收益率",
          "en": "Expected Portfolio Return"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "$E(R_p) = 0.6 \\times 10\\% + 0.4 \\times 5\\% = 6\\% + 2\\% = 8\\%$。",
          "options": [
            "7.0%",
            "8.0%",
            "9.0%",
            "15.0%"
          ],
          "question_id": "q_quiz_portfolio_return_risk_v1",
          "stem": "一个投资组合由资产A和B组成，权重分别为60%和40%。资产A的期望收益率为10%，资产B的期望收益率为5%。该组合的期望收益率是多少？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "$E(R_p) = 0.3 \\times 12\\% + 0.7 \\times 8\\% = 3.6\\% + 5.6\\% = 9.2\\%$。",
          "options": [
            "8.4%",
            "9.2%",
            "10.0%",
            "20.0%"
          ],
          "question_id": "q_quiz_portfolio_return_risk_v2",
          "stem": "一个投资组合由资产X和Y组成，权重分别为30%和70%。资产X的期望收益率为12%，资产Y的期望收益率为8%。该组合的期望收益率是多少？"
        }
      ]
    },
    {
      "concept_key": "correlation_diversification",
      "coverage_tags": [
        "correlation_diversification"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_correlation_diversification",
      "learning_goal": "学生能辨析不同相关系数对组合风险的影响。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "相关系数",
          "en": "Correlation Coefficient"
        }
      ],
      "variants": [
        {
          "answer": 3,
          "estimated_seconds": 15,
          "explanation": "当 $\\rho = 1$（完全正相关）时，组合的标准差就是各资产标准差的加权平均，没有任何分散效果。",
          "options": [
            "$\\rho = -1$",
            "$\\rho = 0$",
            "$\\rho = 0.5$",
            "$\\rho = 1$"
          ],
          "question_id": "q_quiz_correlation_diversification_v1",
          "stem": "当两只资产的相关系数 $\\rho$ 为以下哪个值时，分散风险的效果最差？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "相关系数越低，资产之间的联动性越弱，分散风险的效果越好。完全负相关（$\\rho=-1$）时效果最好。",
          "options": [
            "相关系数越高，分散效果越好",
            "相关系数为0时，分散效果最好",
            "相关系数越低，分散效果越好",
            "相关系数不影响分散效果"
          ],
          "question_id": "q_quiz_correlation_diversification_v2",
          "stem": "以下关于相关系数与分散效果的描述，哪一项是正确的？"
        }
      ]
    },
    {
      "concept_key": "minimum_variance_portfolio",
      "coverage_tags": [
        "minimum_variance_portfolio"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_minimum_variance_portfolio",
      "learning_goal": "学生能识别最小方差组合的特征。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "最小方差组合",
          "en": "Minimum Variance Portfolio"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "MVP是在所有可能的投资组合中，风险（标准差或方差）最低的那个组合。",
          "options": [
            "MVP是所有组合中收益最高的",
            "MVP是所有组合中风险（标准差）最低的",
            "MVP的权重总是各占50%",
            "MVP只存在于完全负相关的情况下"
          ],
          "question_id": "q_quiz_minimum_variance_portfolio_v1",
          "stem": "关于最小方差组合（MVP），以下哪项描述是正确的？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "对组合方差关于权重求导，并令导数等于零，可以找到使方差最小的权重。",
          "options": [
            "对组合收益求导",
            "对组合方差求导并令导数为零",
            "计算协方差矩阵的行列式",
            "使用线性回归"
          ],
          "question_id": "q_quiz_minimum_variance_portfolio_v2",
          "stem": "在求解两只资产的最小方差组合时，通过什么数学方法找到最优权重？"
        }
      ]
    },
    {
      "concept_key": "efficient_frontier",
      "coverage_tags": [
        "efficient_frontier"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_efficient_frontier",
      "learning_goal": "学生能辨析有效前沿的特征。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "有效前沿",
          "en": "Efficient Frontier"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "有效前沿上的组合在给定风险水平下能提供最高的预期收益。",
          "options": [
            "在相同风险下，收益最低",
            "在相同收益下，风险最高",
            "在相同风险下，收益最高",
            "收益和风险都是所有组合中最高的"
          ],
          "question_id": "q_quiz_efficient_frontier_v1",
          "stem": "有效前沿（Efficient Frontier）上的组合具有什么特征？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "不在有效前沿上的组合是低效的，因为存在相同风险但收益更高，或相同收益但风险更低的组合。",
          "options": [
            "有效组合",
            "低效组合",
            "最小方差组合",
            "切点组合"
          ],
          "question_id": "q_quiz_efficient_frontier_v2",
          "stem": "如果一个组合位于机会集内部但不在有效前沿上，它被称为："
        }
      ]
    },
    {
      "concept_key": "tangency_portfolio_and_cml",
      "coverage_tags": [
        "tangency_portfolio_and_cml"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_tangency_portfolio_cml",
      "learning_goal": "学生能辨析切点组合和CML的关键特征。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "切点组合",
          "en": "Tangency Portfolio"
        },
        {
          "display": "资本市场线",
          "en": "Capital Market Line"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "切点组合是最大化夏普比率 $\\frac{\\mu_p - R_f}{\\sigma_p}$ 得到的。",
          "options": [
            "组合的期望收益",
            "组合的夏普比率",
            "组合的方差",
            "组合的贝塔系数"
          ],
          "question_id": "q_quiz_tangency_portfolio_cml_v1",
          "stem": "求解切点组合时，目标函数是最大化什么？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "CML上的点代表无风险资产与市场组合（切点组合）的不同比例配置。",
          "options": [
            "最小方差组合",
            "无风险资产与市场组合的混合",
            "只包含风险资产的组合",
            "只包含无风险资产的组合"
          ],
          "question_id": "q_quiz_tangency_portfolio_cml_v2",
          "stem": "资本市场线（CML）上的所有组合都是什么组合？"
        }
      ]
    },
    {
      "concept_key": "capm_and_sml",
      "coverage_tags": [
        "capm_and_sml"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_capm_sml",
      "learning_goal": "学生能应用CAPM计算期望收益，并理解贝塔的含义。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "资本资产定价模型",
          "en": "Capital Asset Pricing Model"
        },
        {
          "display": "贝塔系数",
          "en": "Beta"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 25,
          "explanation": "$E(R_i) = 2\\% + 1.2 \\times (10\\% - 2\\%) = 2\\% + 9.6\\% = 11.6\\%$。",
          "options": [
            "9.6%",
            "11.6%",
            "12.0%",
            "14.0%"
          ],
          "question_id": "q_quiz_capm_sml_v1",
          "stem": "无风险利率为2%，市场期望收益率为10%，某资产的贝塔系数为1.2。根据CAPM，该资产的期望收益率是多少？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "$\\beta < 1$ 表示该资产的收益波动小于市场，因此比市场更保守。",
          "options": [
            "比市场更激进",
            "与市场风险相同",
            "比市场更保守",
            "没有系统性风险"
          ],
          "question_id": "q_quiz_capm_sml_v2",
          "stem": "一个资产的贝塔系数为0.8，这意味着该资产："
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "落在SML上方表示资产的实际收益高于CAPM给出的预期收益，即 $\\alpha > 0$，说明资产被低估。",
          "options": [
            "该资产被高估",
            "该资产被低估",
            "该资产的贝塔为负",
            "该资产没有风险"
          ],
          "question_id": "q_quiz_capm_sml_v3",
          "stem": "在证券市场线（SML）上，如果一个资产的点落在线的上方，这意味着什么？"
        }
      ]
    },
    {
      "concept_key": "performance_measures",
      "coverage_tags": [
        "performance_measures"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_performance_measures",
      "learning_goal": "学生能应用绩效指标进行计算和辨析。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "夏普比率",
          "en": "Sharpe Ratio"
        },
        {
          "display": "信息比率",
          "en": "Information Ratio"
        },
        {
          "display": "最大回撤",
          "en": "Maximum Drawdown"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 25,
          "explanation": "$SR = (15\\% - 3\\%) / 20\\% = 12\\% / 20\\% = 0.6$。",
          "options": [
            "0.6",
            "0.75",
            "0.8",
            "1.2"
          ],
          "question_id": "q_quiz_performance_measures_v1",
          "stem": "某基金的年化收益率为15%，无风险利率为3%，年化标准差为20%。该基金的夏普比率是多少？"
        },
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "信息比率衡量的是组合相对于一个具体基准（如沪深300）的表现，而夏普比率衡量的是相对于无风险利率的表现。",
          "options": [
            "信息比率使用总风险，夏普比率使用系统性风险",
            "信息比率使用下行风险，夏普比率使用总风险",
            "信息比率使用相对于基准的超额收益，夏普比率使用相对于无风险利率的超额收益",
            "信息比率使用贝塔，夏普比率使用标准差"
          ],
          "question_id": "q_quiz_performance_measures_v2",
          "stem": "信息比率（Information Ratio）与夏普比率的主要区别是什么？"
        },
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "$MDD = (150 - 90) / 150 = 60 / 150 = 40\\%$。",
          "options": [
            "40%",
            "60%",
            "66.67%",
            "150%"
          ],
          "question_id": "q_quiz_performance_measures_v3",
          "stem": "一个投资组合的净值从峰值150跌至谷值90，其最大回撤（MDD）是多少？"
        }
      ]
    },
    {
      "concept_key": "stop_loss_and_risk_limit",
      "coverage_tags": [
        "stop_loss_and_risk_limit"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_stop_loss_risk_limit",
      "learning_goal": "学生能辨析不同类型止损的适用场景。",
      "linked_steps": [
        "step8"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "移动止损",
          "en": "Trailing Stop Loss"
        },
        {
          "display": "组合止损",
          "en": "Portfolio Stop Loss"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "移动止损会随着价格上涨而自动上移止损线，从而锁定已实现的利润。",
          "options": [
            "固定止损",
            "移动止损",
            "组合止损",
            "风险限额止损"
          ],
          "question_id": "q_quiz_stop_loss_risk_limit_v1",
          "stem": "以下哪种止损方式最适合在价格上涨时锁定利润？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "组合止损基于整个投资组合的净值，当总净值从高点回撤一定比例时全部清仓，提供第二层保护。",
          "options": [
            "每个策略的固定止损",
            "每个策略的移动止损",
            "组合止损",
            "单个策略的止盈"
          ],
          "question_id": "q_quiz_stop_loss_risk_limit_v2",
          "stem": "当一个交易员同时运行多个策略时，以下哪种风险控制机制能提供全局性的资本保护？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L7/plain.txt",
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
      "coverage_tag": "sharpe_ratio",
      "covered_by": [
        "qf_flash_sharpe",
        "qf_quiz_sharpe",
        "qf_long_sharpe"
      ],
      "description": "夏普比率的定义、公式、计算与解释"
    },
    {
      "coverage_tag": "sortino_ratio",
      "covered_by": [
        "qf_flash_sortino",
        "qf_quiz_sortino"
      ],
      "description": "索提诺比率的定义、公式与适用场景"
    },
    {
      "coverage_tag": "treynor_ratio",
      "covered_by": [
        "qf_flash_treynor",
        "qf_quiz_treynor"
      ],
      "description": "特雷诺比率的定义、公式与适用场景"
    },
    {
      "coverage_tag": "jensens_alpha",
      "covered_by": [
        "qf_flash_jensen",
        "qf_quiz_jensen",
        "qf_long_jensen"
      ],
      "description": "詹森阿尔法的定义、公式与解释"
    },
    {
      "coverage_tag": "performance_measures_comparison",
      "covered_by": [
        "qf_quiz_comparison",
        "qf_long_comparison"
      ],
      "description": "四种绩效度量指标的对比与选择"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "sharpe_ratio",
      "coverage_tags": [
        "sharpe_ratio"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_sharpe",
      "learning_goal": "学生能准确回忆夏普比率的公式、含义和典型数值范围。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "夏普比率的分子、分母、以及数值含义。",
      "term_refs": [
        {
          "display": "夏普比率",
          "en": "Sharpe Ratio"
        }
      ],
      "variants": [
        {
          "back": "组合的超额收益：E(R_p) - R_f",
          "estimated_seconds": 8,
          "explanation": "夏普比率衡量每单位总风险获得的超额收益。",
          "front": "夏普比率的分子是什么？",
          "question_id": "q_flash_sharpe_v1"
        },
        {
          "back": "组合收益的标准差 σ_p",
          "estimated_seconds": 8,
          "explanation": "标准差衡量组合的总风险。",
          "front": "夏普比率的分母是什么？",
          "question_id": "q_flash_sharpe_v2"
        },
        {
          "back": "2.5",
          "estimated_seconds": 6,
          "explanation": "行业里0.5到1.5很常见，超过2.5属于顶尖水平。",
          "front": "夏普比率超过多少被认为是极其优秀的投资？",
          "question_id": "q_flash_sharpe_v3"
        }
      ]
    },
    {
      "concept_key": "sortino_ratio",
      "coverage_tags": [
        "sortino_ratio"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_sortino",
      "learning_goal": "学生能区分夏普比率与索提诺比率在风险度量上的关键差异。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "core_difference",
      "retrieval_focus": "索提诺比率与夏普比率的核心区别。",
      "term_refs": [
        {
          "display": "索提诺比率",
          "en": "Sortino Ratio"
        }
      ],
      "variants": [
        {
          "back": "索提诺比率使用下行标准差，夏普比率使用总标准差。",
          "estimated_seconds": 10,
          "explanation": "索提诺比率只惩罚下跌波动，更符合投资者厌恶亏损的心理。",
          "front": "索提诺比率与夏普比率在分母上的关键区别是什么？",
          "question_id": "q_flash_sortino_v1"
        },
        {
          "back": "下行标准差 σ_d",
          "estimated_seconds": 8,
          "explanation": "下行标准差只计算负收益的波动。",
          "front": "索提诺比率的分母是什么？",
          "question_id": "q_flash_sortino_v2"
        }
      ]
    },
    {
      "concept_key": "treynor_ratio",
      "coverage_tags": [
        "treynor_ratio"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_treynor",
      "learning_goal": "学生能准确回忆特雷诺比率的公式和适用条件。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "特雷诺比率的公式和风险度量。",
      "term_refs": [
        {
          "display": "特雷诺比率",
          "en": "Treynor Ratio"
        }
      ],
      "variants": [
        {
          "back": "组合的贝塔系数 β_p",
          "estimated_seconds": 8,
          "explanation": "贝塔衡量系统性风险。",
          "front": "特雷诺比率的分母是什么？",
          "question_id": "q_flash_treynor_v1"
        },
        {
          "back": "已经充分分散、非系统性风险几乎为零的组合。",
          "estimated_seconds": 10,
          "explanation": "因为特雷诺比率只考虑系统性风险。",
          "front": "特雷诺比率适用于什么类型的投资组合？",
          "question_id": "q_flash_treynor_v2"
        }
      ]
    },
    {
      "concept_key": "jensens_alpha",
      "coverage_tags": [
        "jensens_alpha"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_jensen",
      "learning_goal": "学生能准确回忆詹森阿尔法的公式和含义。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "詹森阿尔法的公式和正负含义。",
      "term_refs": [
        {
          "display": "詹森阿尔法",
          "en": "Jensen's Alpha"
        }
      ],
      "variants": [
        {
          "back": "基金经理是否创造了超出CAPM预期的超额价值。",
          "estimated_seconds": 10,
          "explanation": "α = 实际收益 - CAPM预期收益。",
          "front": "詹森阿尔法衡量的是什么？",
          "question_id": "q_flash_jensen_v1"
        },
        {
          "back": "组合的实际收益高于CAPM给出的预期收益，基金经理创造了超额价值。",
          "estimated_seconds": 8,
          "explanation": "α > 0 表示组合跑赢了市场。",
          "front": "詹森阿尔法大于0意味着什么？",
          "question_id": "q_flash_jensen_v2"
        }
      ]
    }
  ],
  "lesson_id": "L7",
  "longform_families": [
    {
      "concept_key": "sharpe_ratio",
      "coverage_tags": [
        "sharpe_ratio"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_sharpe",
      "learning_goal": "学生能根据给定的月度收益数据，完整计算并解释年化夏普比率。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "worked_example",
      "term_refs": [
        {
          "display": "夏普比率",
          "en": "Sharpe Ratio"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 180,
          "prompt_blocks": [
            "计算月平均收益率",
            "计算月收益率标准差",
            "年化月平均收益率和标准差",
            "代入夏普比率公式",
            "解释结果"
          ],
          "question_id": "q_long_sharpe_v1",
          "reference_answer": [
            "月平均收益率 = (1.2+2.3-1.5+0.8+1.7-2.1+3.2+0.4+2.1-0.9+1.1+0.3)/12 = 0.7167%",
            "月标准差 = sqrt(Σ(r_i - μ)^2 / 11) ≈ 1.59%",
            "年化收益率 = 0.7167% * 12 = 8.6%",
            "年化标准差 = 1.59% * √12 ≈ 5.51%",
            "夏普比率 = (8.6% - 1%) / 5.51% ≈ 1.38",
            "该基金的夏普比率为1.38，属于良好水平，表明每承担1单位总风险可获得1.38单位的超额收益。"
          ],
          "rubric_points": [
            "正确计算月平均收益率（约0.7167%）",
            "正确计算月标准差（约1.59%）",
            "正确年化：年化收益率=月平均*12（约8.6%），年化标准差=月标准差*√12（约5.51%）",
            "正确代入公式：(8.6%-1%)/5.51% ≈ 1.38",
            "解释：夏普比率1.38属于良好水平，说明每单位总风险获得了1.38单位的超额收益"
          ],
          "stem": "某基金过去12个月的月度收益率如下（%）：[1.2, 2.3, -1.5, 0.8, 1.7, -2.1, 3.2, 0.4, 2.1, -0.9, 1.1, 0.3]。无风险年利率为1%。请计算该基金的年化夏普比率，并解释其含义。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "计算基金A的夏普比率",
            "计算基金B的夏普比率",
            "比较两个比率",
            "解释结果"
          ],
          "question_id": "q_long_sharpe_v2",
          "reference_answer": [
            "基金A夏普比率 = (15% - 2%) / 20% = 0.65",
            "基金B夏普比率 = (12% - 2%) / 12% = 0.83",
            "基金B的夏普比率(0.83)高于基金A(0.65)，说明基金B在风险调整后的表现更好。",
            "尽管基金A的绝对收益更高，但其承担的风险也更大，每单位风险获得的超额收益反而更低。"
          ],
          "rubric_points": [
            "基金A夏普比率 = (15%-2%)/20% = 0.65",
            "基金B夏普比率 = (12%-2%)/12% = 0.83",
            "基金B的夏普比率更高",
            "解释：虽然基金A的绝对收益更高，但基金B每单位风险获得的超额收益更多，风险调整后表现更好"
          ],
          "stem": "基金A的年化收益率为15%，年化标准差为20%；基金B的年化收益率为12%，年化标准差为12%。无风险利率为2%。请计算两只基金的夏普比率，并比较哪只基金表现更好，解释原因。"
        }
      ]
    },
    {
      "concept_key": "jensens_alpha",
      "coverage_tags": [
        "jensens_alpha"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_jensen",
      "learning_goal": "学生能解释詹森阿尔法的计算过程、含义及其与CAPM的关系。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "mechanism_trace",
      "term_refs": [
        {
          "display": "詹森阿尔法",
          "en": "Jensen's Alpha"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "写出CAPM公式",
            "计算CAPM给出的预期收益",
            "计算詹森阿尔法",
            "解释阿尔法的含义"
          ],
          "question_id": "q_long_jensen_v1",
          "reference_answer": [
            "CAPM公式：E(R_p) = R_f + β_p * (E(R_M) - R_f)",
            "CAPM预期收益 = 2% + 1.5 * (8% - 2%) = 2% + 9% = 11%",
            "詹森阿尔法 α = 实际收益 - CAPM预期收益 = 12% - 11% = 1%",
            "α = 1% > 0，表明该组合的实际收益超过了CAPM模型基于其系统性风险所预期的收益。",
            "这意味着基金经理通过选股或择时等主动管理，创造了1%的超额价值。"
          ],
          "rubric_points": [
            "正确写出CAPM公式：E(R_p) = R_f + β_p * (E(R_M) - R_f)",
            "正确计算CAPM预期收益：2% + 1.5*(8%-2%) = 11%",
            "正确计算詹森阿尔法：α = 12% - 11% = 1%",
            "解释：α=1% > 0，说明组合的实际收益高于CAPM的预期，基金经理创造了1%的超额价值"
          ],
          "stem": "无风险利率为2%，市场组合的预期收益率为8%。某投资组合的贝塔为1.5，实际收益率为12%。请计算该组合的詹森阿尔法，并解释这个数值的含义。"
        },
        {
          "estimated_seconds": 150,
          "prompt_blocks": [
            "写出詹森阿尔法公式",
            "解释CAPM预期收益的含义",
            "举例说明α为负但绝对收益为正的情况",
            "总结"
          ],
          "question_id": "q_long_jensen_v2",
          "reference_answer": [
            "詹森阿尔法公式：α_p = E(R_p) - [R_f + β_p(E(R_M) - R_f)]",
            "CAPM预期收益是给定系统性风险下，市场认为的'公平'收益。",
            "例如：市场收益为20%，无风险利率2%，组合β=2，则CAPM预期收益=2%+2*(20%-2%)=38%。",
            "如果组合实际收益为30%，虽然绝对收益为正，但α = 30% - 38% = -8%，为负值。",
            "这说明组合虽然赚钱，但相对于其承担的高风险，表现不如市场预期。",
            "因此，α为负只表示相对表现不佳，并不代表组合亏损。"
          ],
          "rubric_points": [
            "正确写出公式：α = 实际收益 - CAPM预期收益",
            "解释CAPM预期收益是基于组合贝塔和市场风险溢价计算的公平收益",
            "举例：市场大涨时，高贝塔组合的CAPM预期收益可能很高，即使实际收益为正，也可能低于预期",
            "总结：α为负只说明相对表现不佳，不代表亏损"
          ],
          "stem": "解释为什么詹森阿尔法为负并不意味着组合的绝对收益为负。请结合CAPM模型说明。"
        }
      ]
    },
    {
      "concept_key": "performance_measures_comparison",
      "coverage_tags": [
        "performance_measures_comparison"
      ],
      "difficulty": "hard",
      "family_id": "qf_long_comparison",
      "learning_goal": "学生能比较夏普比率、特雷诺比率和詹森阿尔法三种绩效度量指标的异同，并说明各自的适用场景。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "compare_and_contrast",
      "term_refs": [
        {
          "display": "夏普比率",
          "en": "Sharpe Ratio"
        },
        {
          "display": "特雷诺比率",
          "en": "Treynor Ratio"
        },
        {
          "display": "詹森阿尔法",
          "en": "Jensen's Alpha"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 240,
          "prompt_blocks": [
            "列出三种指标及其风险度量",
            "说明各自的适用场景",
            "解释排名不一致的原因"
          ],
          "question_id": "q_long_comparison_v1",
          "reference_answer": [
            "风险度量：",
            "- 夏普比率：总风险（标准差σ_p）",
            "- 特雷诺比率：系统性风险（贝塔β_p）",
            "- 詹森阿尔法：基于CAPM，不直接使用风险度量，而是计算实际收益与CAPM预期收益的差值",
            "",
            "适用场景：",
            "- 夏普比率：当该组合是投资者的全部风险投资时最合适",
            "- 特雷诺比率：当组合是大型多元化基金的一部分时最合适",
            "- 詹森阿尔法：当需要直接评估基金经理的主动管理能力时最合适",
            "",
            "排名不一致的原因：",
            "- 如果组合存在非系统性风险，夏普比率会因总风险较大而给出较低评分",
            "- 特雷诺比率和詹森阿尔法只关注系统性风险，不受非系统性风险影响",
            "- 因此，一个分散化不足但选股能力强的组合，可能在夏普比率上表现差，但在特雷诺比率和詹森阿尔法上表现好"
          ],
          "rubric_points": [
            "夏普比率使用总风险（标准差），特雷诺比率使用系统性风险（贝塔），詹森阿尔法基于CAPM计算超额收益",
            "夏普比率适用于组合是唯一风险投资时；特雷诺比率适用于组合是大型基金的一部分时；詹森阿尔法直接衡量主动管理价值",
            "当组合存在非系统性风险时，夏普比率会惩罚组合，而特雷诺比率和詹森阿尔法不会，导致排名不一致"
          ],
          "stem": "比较夏普比率、特雷诺比率和詹森阿尔法这三种绩效度量指标。请从以下三个方面进行阐述：(1) 各自使用的风险度量；(2) 各自的适用场景；(3) 为什么它们有时会给出不同的排名。"
        },
        {
          "estimated_seconds": 240,
          "prompt_blocks": [
            "为客户A推荐指标并解释",
            "为客户B推荐指标并解释",
            "为客户C推荐指标并解释"
          ],
          "question_id": "q_long_comparison_v2",
          "reference_answer": [
            "客户A（个人投资者）：推荐夏普比率。",
            "理由：该基金是客户的唯一风险投资，需要评估总风险（包括非系统性风险）调整后的收益。夏普比率使用标准差衡量总风险，最适合此场景。",
            "",
            "客户B（养老基金投资经理）：推荐特雷诺比率。",
            "理由：该经理管理的子组合是大型基金的一部分，非系统性风险已被整体分散。应关注系统性风险，特雷诺比率使用贝塔，最适合评估子组合的贡献。",
            "",
            "客户C（对冲基金）：推荐詹森阿尔法。",
            "理由：对冲基金需要证明其主动管理能力，即能否创造超越市场的超额收益。詹森阿尔法直接衡量实际收益与CAPM预期收益的差值，最能体现基金经理的'价值增值'。"
          ],
          "rubric_points": [
            "客户A推荐夏普比率，因为这是其全部风险投资",
            "客户B推荐特雷诺比率，因为子组合应关注系统性风险",
            "客户C推荐詹森阿尔法，因为需要展示主动管理带来的超额价值"
          ],
          "stem": "假设你是一个投资顾问，有三个客户：(A) 将所有资金投资于一只基金的个人投资者；(B) 一个大型养老基金的投资经理，该基金由多个子组合构成；(C) 一个对冲基金，希望向潜在投资者展示其主动管理能力。请分别为每个客户推荐最合适的绩效度量指标，并解释理由。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "sharpe_ratio",
      "coverage_tags": [
        "sharpe_ratio"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_sharpe",
      "learning_goal": "学生能在给定数据下计算夏普比率，并理解其含义。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "夏普比率",
          "en": "Sharpe Ratio"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 30,
          "explanation": "夏普比率 = (12% - 2%) / 15% = 0.67。",
          "options": [
            "0.50",
            "0.67",
            "0.80",
            "1.20"
          ],
          "question_id": "q_quiz_sharpe_v1",
          "stem": "某基金年化收益率为12%，无风险利率为2%，年化标准差为15%。该基金的夏普比率是多少？"
        },
        {
          "answer": 1,
          "estimated_seconds": 40,
          "explanation": "基金A的夏普比率 = (18%-3%)/25% = 0.6；基金B的夏普比率 = (10%-3%)/12% = 0.583。基金A的夏普比率更高。",
          "options": [
            "基金A",
            "基金B",
            "两者相同",
            "无法判断"
          ],
          "question_id": "q_quiz_sharpe_v2",
          "stem": "基金A的年化收益率为18%，标准差为25%；基金B的年化收益率为10%，标准差为12%。无风险利率为3%。仅从夏普比率看，哪个基金表现更好？"
        }
      ]
    },
    {
      "concept_key": "sortino_ratio",
      "coverage_tags": [
        "sortino_ratio"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_sortino",
      "learning_goal": "学生能理解索提诺比率与夏普比率的区别，并判断其适用场景。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "索提诺比率",
          "en": "Sortino Ratio"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "索提诺比率只考虑下行风险，更适合厌恶亏损的投资者。",
          "options": [
            "组合收益分布对称",
            "投资者更关注下跌风险",
            "组合已经充分分散化",
            "需要与市场基准比较"
          ],
          "question_id": "q_quiz_sortino_v1",
          "stem": "以下哪种情况最适合使用索提诺比率而非夏普比率？"
        },
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "索提诺比率使用下行标准差，只惩罚下跌波动，更符合投资者心理。",
          "options": [
            "计算更简单",
            "不依赖无风险利率",
            "只惩罚下跌波动",
            "适用于任何类型的组合"
          ],
          "question_id": "q_quiz_sortino_v2",
          "stem": "与夏普比率相比，索提诺比率的主要优势是什么？"
        }
      ]
    },
    {
      "concept_key": "treynor_ratio",
      "coverage_tags": [
        "treynor_ratio"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_treynor",
      "learning_goal": "学生能理解特雷诺比率的适用条件和含义。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "特雷诺比率",
          "en": "Treynor Ratio"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "特雷诺比率 = (E(R_p) - R_f) / β_p，使用贝塔系数度量系统性风险。",
          "options": [
            "总标准差",
            "下行标准差",
            "贝塔系数",
            "最大回撤"
          ],
          "question_id": "q_quiz_treynor_v1",
          "stem": "特雷诺比率使用什么来度量风险？"
        },
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "充分分散后，非系统性风险可忽略，特雷诺比率只考虑系统性风险，最为合适。",
          "options": [
            "夏普比率",
            "索提诺比率",
            "特雷诺比率",
            "最大回撤"
          ],
          "question_id": "q_quiz_treynor_v2",
          "stem": "一个充分分散的投资组合，其非系统性风险接近于零。此时，以下哪个绩效指标最合适？"
        }
      ]
    },
    {
      "concept_key": "jensens_alpha",
      "coverage_tags": [
        "jensens_alpha"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_jensen",
      "learning_goal": "学生能计算和解释詹森阿尔法。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "詹森阿尔法",
          "en": "Jensen's Alpha"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 40,
          "explanation": "CAPM预期收益 = 3% + 1.2*(10%-3%) = 11.4%。α = 12% - 11.4% = 0.6%。",
          "options": [
            "-2.4%",
            "0.6%",
            "1.2%",
            "2.4%"
          ],
          "question_id": "q_quiz_jensen_v1",
          "stem": "无风险利率为3%，市场预期收益率为10%，某基金的贝塔为1.2，实际收益率为12%。该基金的詹森阿尔法是多少？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "α = 实际收益 - CAPM预期收益，为负说明基金经理没有跑赢市场。",
          "options": [
            "组合的收益低于无风险利率",
            "组合的实际收益低于CAPM给出的预期收益",
            "组合的贝塔过高",
            "组合的风险太大"
          ],
          "question_id": "q_quiz_jensen_v2",
          "stem": "詹森阿尔法为负值说明什么？"
        }
      ]
    },
    {
      "concept_key": "performance_measures_comparison",
      "coverage_tags": [
        "performance_measures_comparison"
      ],
      "difficulty": "hard",
      "family_id": "qf_quiz_comparison",
      "learning_goal": "学生能根据不同的投资场景选择合适的绩效度量指标。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "绩效度量指标",
          "en": "Performance Measures"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "当组合代表全部风险投资时，夏普比率（考虑总风险）最合适。",
          "options": [
            "特雷诺比率",
            "詹森阿尔法",
            "夏普比率",
            "信息比率"
          ],
          "question_id": "q_quiz_comparison_v1",
          "stem": "如果一个投资组合是投资者唯一的风险投资，应该优先使用哪个绩效指标？"
        },
        {
          "answer": 2,
          "estimated_seconds": 25,
          "explanation": "当组合是大型基金的一部分时，应关注系统性风险，特雷诺比率或詹森阿尔法更合适。",
          "options": [
            "夏普比率",
            "索提诺比率",
            "特雷诺比率或詹森阿尔法",
            "最大回撤"
          ],
          "question_id": "q_quiz_comparison_v2",
          "stem": "一个基金经理管理的组合是大型基金中的一部分，要评估其主动管理能力，应该优先使用哪个指标？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L7/plain.txt",
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
      "coverage_tag": "stop_loss_basics",
      "covered_by": [
        "qf_flash_stop_loss_def",
        "qf_quiz_stop_loss_purpose"
      ],
      "description": "止损的基本概念：设定价格自动平仓以限制损失。"
    },
    {
      "coverage_tag": "fixed_stop_loss",
      "covered_by": [
        "qf_flash_fixed_sl",
        "qf_quiz_fixed_sl_calc"
      ],
      "description": "固定止损：设定固定价格水平，最大亏损锁定。"
    },
    {
      "coverage_tag": "trailing_stop_loss",
      "covered_by": [
        "qf_flash_trailing_sl",
        "qf_quiz_trailing_sl_advantage",
        "qf_long_trailing_sl_mechanism"
      ],
      "description": "移动止损：动态调整止损线，随价格上涨上移，锁定利润。"
    },
    {
      "coverage_tag": "portfolio_stop_loss",
      "covered_by": [
        "qf_flash_portfolio_sl",
        "qf_quiz_portfolio_sl_purpose"
      ],
      "description": "组合止损：基于整个组合净值的第二层保护，从高点回撤一定比例时清仓。"
    },
    {
      "coverage_tag": "risk_limit_stop_loss",
      "covered_by": [
        "qf_flash_risk_limit_types",
        "qf_quiz_risk_limit_example"
      ],
      "description": "风险限额止损：日/周/月/季/年止损、VaR限额等风控措施。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "stop_loss",
      "coverage_tags": [
        "stop_loss_basics"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_stop_loss_def",
      "learning_goal": "学生能准确说出止损的定义和核心作用。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "止损的定义和自动平仓机制。",
      "term_refs": [
        {
          "display": "止损",
          "en": "Stop Loss"
        }
      ],
      "variants": [
        {
          "back": "当价格达到预设水平时自动平仓，以限制单笔交易的损失。",
          "estimated_seconds": 8,
          "explanation": "止损是风险控制的最基本工具，避免情绪化决策和无限亏损。",
          "front": "止损（Stop Loss）的核心作用是什么？",
          "question_id": "q_flash_stop_loss_def_v1"
        },
        {
          "back": "当资产价格达到或跌破预设的止损价格水平时自动触发平仓。",
          "estimated_seconds": 8,
          "explanation": "止损价格在开仓时设定，一旦触发即执行，无需人工干预。",
          "front": "止损指令在什么条件下触发？",
          "question_id": "q_flash_stop_loss_def_v2"
        }
      ]
    },
    {
      "concept_key": "fixed_stop_loss",
      "coverage_tags": [
        "fixed_stop_loss"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_fixed_sl",
      "learning_goal": "学生能描述固定止损的设定方式和最大亏损锁定原理。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "固定止损的设定和最大亏损计算。",
      "term_refs": [
        {
          "display": "固定止损",
          "en": "Fixed Stop Loss"
        }
      ],
      "variants": [
        {
          "back": "10元（每股）。",
          "estimated_seconds": 6,
          "explanation": "固定止损将最大亏损锁定在买入价与止损价的差值。",
          "front": "以100元买入股票，设置固定止损价为90元，最大亏损是多少？",
          "question_id": "q_flash_fixed_sl_v1"
        },
        {
          "back": "不会，固定止损的止损价格是预先设定且不变的。",
          "estimated_seconds": 6,
          "explanation": "固定止损在开仓时设定一个固定价格，不会随市场变化调整。",
          "front": "固定止损的止损线是否会随价格上涨而移动？",
          "question_id": "q_flash_fixed_sl_v2"
        }
      ]
    },
    {
      "concept_key": "trailing_stop_loss",
      "coverage_tags": [
        "trailing_stop_loss"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_trailing_sl",
      "learning_goal": "学生能解释移动止损的运作机制和主要优势。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "移动止损的动态调整规则。",
      "term_refs": [
        {
          "display": "移动止损",
          "en": "Trailing Stop Loss"
        }
      ],
      "variants": [
        {
          "back": "止损线会随价格上涨而上移，但不会下移。",
          "estimated_seconds": 8,
          "explanation": "移动止损锁定已实现利润，同时保留进一步上涨的空间。",
          "front": "移动止损的止损线在价格上涨时会如何变化？",
          "question_id": "q_flash_trailing_sl_v1"
        },
        {
          "back": "104.5（110 - 110 × 5% = 104.5）。",
          "estimated_seconds": 10,
          "explanation": "移动止损按设定的百分比或金额从最高价向下计算。",
          "front": "设5%移动止损，价格从100涨到110，止损线会上移到多少？",
          "question_id": "q_flash_trailing_sl_v2"
        }
      ]
    },
    {
      "concept_key": "portfolio_stop_loss",
      "coverage_tags": [
        "portfolio_stop_loss"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_portfolio_sl",
      "learning_goal": "学生能说明组合止损的作用和触发条件。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "组合止损作为第二层保护的定义。",
      "term_refs": [
        {
          "display": "组合止损",
          "en": "Portfolio Stop Loss"
        }
      ],
      "variants": [
        {
          "back": "当整个投资组合的净值从历史最高点回撤达到预设比例时，全部清仓。",
          "estimated_seconds": 10,
          "explanation": "组合止损是第二层保护，防止多个策略同时亏损导致资本大幅缩水。",
          "front": "组合止损（Portfolio Stop Loss）在什么情况下触发？",
          "question_id": "q_flash_portfolio_sl_v1"
        },
        {
          "back": "组合止损基于整个组合净值，单笔止损基于单个资产价格。",
          "estimated_seconds": 8,
          "explanation": "组合止损从全局角度控制风险，单笔止损只控制单个头寸。",
          "front": "组合止损与单笔交易止损的主要区别是什么？",
          "question_id": "q_flash_portfolio_sl_v2"
        }
      ]
    },
    {
      "concept_key": "risk_limit_stop_loss",
      "coverage_tags": [
        "risk_limit_stop_loss"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_risk_limit_types",
      "learning_goal": "学生能列举常见的风险限额类型。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "风险限额的时间维度分类。",
      "term_refs": [
        {
          "display": "风险限额",
          "en": "Risk Limit"
        }
      ],
      "variants": [
        {
          "back": "日止损、周止损、月止损、季度止损、年度止损。",
          "estimated_seconds": 8,
          "explanation": "不同时间维度的止损限额用于控制不同周期内的最大亏损。",
          "front": "风控部门常用的风险限额按时间维度分为哪几种？",
          "question_id": "q_flash_risk_limit_types_v1"
        },
        {
          "back": "VaR限额、DV01限额、Delta限额等。",
          "estimated_seconds": 8,
          "explanation": "这些限额针对不同类型的风险敞口进行控制。",
          "front": "除了时间维度止损，风险限额还包括哪些类型？",
          "question_id": "q_flash_risk_limit_types_v2"
        }
      ]
    }
  ],
  "lesson_id": "L7",
  "longform_families": [
    {
      "concept_key": "trailing_stop_loss",
      "coverage_tags": [
        "trailing_stop_loss"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_trailing_sl_mechanism",
      "learning_goal": "学生能完整解释移动止损的运作机制、优势及适用场景。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "mechanism_trace",
      "term_refs": [
        {
          "display": "移动止损",
          "en": "Trailing Stop Loss"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "移动止损的定义",
            "止损线如何随价格变化",
            "与固定止损的对比",
            "一个具体数值例子"
          ],
          "question_id": "q_long_trailing_sl_mechanism_v1",
          "reference_answer": [
            "移动止损是一种动态止损指令，止损线随资产价格上涨而上移，但不会下移。",
            "例如：以100元买入，设5%移动止损，初始止损线为95元。价格涨到110元时，止损线上移到104.5元（110-110×5%）。",
            "与固定止损相比，移动止损能锁定已实现利润，同时保留继续上涨的空间。",
            "固定止损的止损线固定不变，无法捕捉上涨带来的利润保护机会。"
          ],
          "rubric_points": [
            "正确描述移动止损的动态调整规则",
            "准确说明止损线只上移不下移",
            "清晰对比固定止损的静态特性",
            "给出自洽的数值例子"
          ],
          "stem": "请解释移动止损（Trailing Stop Loss）的运作机制，并说明它与固定止损相比的主要优势。"
        },
        {
          "estimated_seconds": 100,
          "prompt_blocks": [
            "初始止损线计算",
            "价格上涨时的止损线调整",
            "价格下跌时的触发条件",
            "最终盈亏计算"
          ],
          "question_id": "q_long_trailing_sl_mechanism_v2",
          "reference_answer": [
            "初始止损线：80 × (1 - 0.08) = 73.6元。",
            "价格涨到100元时，止损线上移到100 × (1 - 0.08) = 92元。",
            "价格从100元跌到90元时，90元低于92元止损线，触发卖出。",
            "最终以92元卖出，每股盈利12元（92-80），利润被锁定。"
          ],
          "rubric_points": [
            "正确计算初始止损线",
            "正确计算每个价格点的止损线",
            "正确判断触发时机",
            "正确计算最终盈亏"
          ],
          "stem": "假设你以80元买入某股票，设置8%移动止损。请描述价格从80元涨到100元再跌到90元的过程中，止损线的变化情况，并说明最终结果。"
        }
      ]
    },
    {
      "concept_key": "risk_management_hierarchy",
      "coverage_tags": [
        "stop_loss_basics",
        "fixed_stop_loss",
        "trailing_stop_loss",
        "portfolio_stop_loss",
        "risk_limit_stop_loss"
      ],
      "difficulty": "hard",
      "family_id": "qf_long_risk_management_system",
      "learning_goal": "学生能比较不同层次的风险控制工具及其适用场景。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "compare_and_contrast",
      "term_refs": [
        {
          "display": "止损与风险限额",
          "en": "Stop Loss and Risk Limit"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "四种工具的定义",
            "作用层次（交易层面 vs 组合层面）",
            "触发条件对比",
            "适用场景举例"
          ],
          "question_id": "q_long_risk_management_system_v1",
          "reference_answer": [
            "固定止损：交易层面，设定固定价格触发，适用于单笔交易风险控制。",
            "移动止损：交易层面，随价格上涨上移，适用于趋势行情中保护利润。",
            "组合止损：组合层面，基于总净值回撤比例触发，适用于多策略组合的全局保护。",
            "风险限额止损：组合层面，按日/周/月等时间周期设定最大亏损限额，适用于风控部门的合规管理。",
            "这四层工具形成从单笔到全局、从价格到时间的完整风控体系。"
          ],
          "rubric_points": [
            "准确区分交易层面和组合层面的工具",
            "正确描述每种工具的触发条件",
            "给出合理的适用场景",
            "逻辑清晰，层次分明"
          ],
          "stem": "请比较固定止损、移动止损、组合止损和风险限额止损这四种风险控制工具，说明它们各自的作用层次、触发条件和适用场景。"
        },
        {
          "estimated_seconds": 130,
          "prompt_blocks": [
            "每个策略内部的风险控制",
            "组合层面的风险控制",
            "时间维度的风险限额",
            "各层次之间的协调关系"
          ],
          "question_id": "q_long_risk_management_system_v2",
          "reference_answer": [
            "每个策略内部：对每笔交易设置固定止损（如5%）或移动止损（如8%），控制单笔最大亏损。",
            "组合层面：设置组合止损，当总净值从高点回撤10%时全部清仓，作为第二层保护。",
            "时间维度：设置日止损3%、月止损8%，由风控部门监控，超限则暂停所有策略。",
            "各层次协调：单笔止损先触发，若多个策略同时亏损导致净值回撤，组合止损再触发；时间限额作为最后防线。"
          ],
          "rubric_points": [
            "每个策略内部使用固定或移动止损",
            "组合层面设置组合止损",
            "设置时间维度风险限额",
            "说明各层次如何互补"
          ],
          "stem": "一个量化交易团队同时运行5个策略，每个策略管理不同资产。请设计一个多层次风险控制方案，说明你会如何组合使用固定止损、移动止损、组合止损和风险限额止损。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "stop_loss",
      "coverage_tags": [
        "stop_loss_basics"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_stop_loss_purpose",
      "learning_goal": "学生能辨析止损的核心目的。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "止损",
          "en": "Stop Loss"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "止损的核心目的是在价格不利时自动平仓，将损失控制在可接受范围内。",
          "options": [
            "最大化交易利润",
            "限制单笔交易的潜在损失",
            "增加交易频率",
            "预测市场走势"
          ],
          "question_id": "q_quiz_stop_loss_purpose_v1",
          "stem": "设置止损的主要目的是什么？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "止损不能保证盈利，它只是限制亏损的工具。",
          "options": [
            "减少情绪化决策",
            "锁定最大亏损",
            "保证交易盈利",
            "实现自动化风险管理"
          ],
          "question_id": "q_quiz_stop_loss_purpose_v2",
          "stem": "以下哪一项不是止损带来的好处？"
        }
      ]
    },
    {
      "concept_key": "fixed_stop_loss",
      "coverage_tags": [
        "fixed_stop_loss"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_fixed_sl_calc",
      "learning_goal": "学生能计算固定止损下的最大亏损金额。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "固定止损",
          "en": "Fixed Stop Loss"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "每股亏损5元（50-45），共100股，总亏损500元。",
          "options": [
            "50元",
            "500元",
            "450元",
            "100元"
          ],
          "question_id": "q_quiz_fixed_sl_calc_v1",
          "stem": "以50元买入100股股票，设置固定止损价为45元。如果触发止损，最大亏损是多少？"
        },
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "每股亏损20元（200-180），共100股，总亏损2000元。",
          "options": [
            "2000元",
            "1800元",
            "200元",
            "20元"
          ],
          "question_id": "q_quiz_fixed_sl_calc_v2",
          "stem": "以200元买入1手（100股），设置固定止损价为180元。最大亏损金额是多少？"
        }
      ]
    },
    {
      "concept_key": "trailing_stop_loss",
      "coverage_tags": [
        "trailing_stop_loss"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_trailing_sl_advantage",
      "learning_goal": "学生能辨析移动止损相比固定止损的核心优势。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "移动止损",
          "en": "Trailing Stop Loss"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 15,
          "explanation": "移动止损随价格上涨而上移，锁定已实现利润，同时保留上涨空间。",
          "options": [
            "能锁定已获得的利润",
            "能承受更大的亏损",
            "不需要设置参数",
            "适用于所有市场条件"
          ],
          "question_id": "q_quiz_trailing_sl_advantage_v1",
          "stem": "移动止损相比固定止损的最大优势是什么？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "上升趋势中移动止损可以不断上移，锁定利润的同时捕捉更大涨幅。",
          "options": [
            "市场处于长期横盘震荡",
            "资产价格呈现明显上升趋势",
            "资产价格持续下跌",
            "投资者希望固定最大亏损"
          ],
          "question_id": "q_quiz_trailing_sl_advantage_v2",
          "stem": "以下哪种情况最适合使用移动止损？"
        }
      ]
    },
    {
      "concept_key": "portfolio_stop_loss",
      "coverage_tags": [
        "portfolio_stop_loss"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_portfolio_sl_purpose",
      "learning_goal": "学生能理解组合止损的定位和作用。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "组合止损",
          "en": "Portfolio Stop Loss"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "组合止损作为第二层保护，在单笔止损之上提供全局风险控制。",
          "options": [
            "第一层保护",
            "第二层保护",
            "唯一的风险控制工具",
            "替代单笔止损"
          ],
          "question_id": "q_quiz_portfolio_sl_purpose_v1",
          "stem": "组合止损（Portfolio Stop Loss）通常被描述为什么？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "组合止损触发时，系统会平掉所有头寸以保护整体资本。",
          "options": [
            "只平仓亏损最大的头寸",
            "全部清仓所有头寸",
            "增加保证金",
            "暂停交易一天"
          ],
          "question_id": "q_quiz_portfolio_sl_purpose_v2",
          "stem": "当组合止损触发时，通常执行什么操作？"
        }
      ]
    },
    {
      "concept_key": "risk_limit_stop_loss",
      "coverage_tags": [
        "risk_limit_stop_loss"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_risk_limit_example",
      "learning_goal": "学生能识别不同时间维度的风险限额。",
      "linked_steps": [
        "step7"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "风险限额",
          "en": "Risk Limit"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 12,
          "explanation": "月度止损限额是风控部门设置的时间维度风险限额之一。",
          "options": [
            "移动止损",
            "固定止损",
            "月度止损限额",
            "组合止损"
          ],
          "question_id": "q_quiz_risk_limit_example_v1",
          "stem": "以下哪一项属于风险限额（Risk Limit）？"
        },
        {
          "answer": 2,
          "estimated_seconds": 12,
          "explanation": "'m'代表monthly，即月度止损限额。",
          "options": [
            "'d'",
            "'w'",
            "'m'",
            "'y'"
          ],
          "question_id": "q_quiz_risk_limit_example_v2",
          "stem": "在ALGOGENE中，设置月度止损限额应使用哪个risk_type参数？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "Stop loss and risk limit",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "Stop loss and risk limit control",
    "plain_text": "pipeline/1-plain/L7/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "Stop loss and risk limit"
  },
  "target_language": "zh-CN"
}

]
</QUESTION_BANK>

<PLAIN_TEXT>
# L7: Portfolio Optimization and Performance Measures

Course Code: COMP7415

# Agenda

- Calculating risk and return of an investment portfolio   
Correlation effect for diversification   
- Minimum variance portfolio   
- Efficiency frontier   
- Tangency portfolio   
- Capital Asset Pricing Model (CAPM)

- Capital Market Line (CML)   
Security Market Line (SML)

Commonly used performance measures   
- Stop loss and risk limit

# Risk Management Cycle

1. Risk Identification   
2. Risk Assessment / Measurement   
3. Risk Treatment   
4. Risk Monitoring

![](images/79e05f423494a59b8f72adfb80a011ffb25c27e6f2d4fc60467d5150d464c33f.jpg)

# Investment Portfolio

# Investment Portfolio

- There are many different kinds of assets in the market (eg. stocks, commodity, forex, bonds, etc). We can pick various assets for our investment as a portfolio   
- Portfolio optimization is the process of selecting the best distribution of assets to maximize returns while minimizing risk   
- Investment basically involves

- asset selection   
- asset allocation

- In this section, we will discuss the Markowitz Portfolio Theory (or modern portfolio theory) which is a Nobel prize theory in 1990

# Revision of Basic Statistics

- Given that $a$ and $b$ are constants and $X$ and $Y$ are random variables.

$E(a + X) = a + E(X)$   
$\cdot E (a \times X) = a \times E (X)$   
$\cdot E(X + Y) = E(X) + E(Y)$   
- $Var(a + X) = Var(X)$   
$\operatorname{Var}(a \times X) = a^{2} \times \operatorname{Var}(X)$   
- $\operatorname{Var}\left( {X + Y}\right)  = \operatorname{Var}\left( X\right)  + \operatorname{Var}\left( Y\right)  + 2 \times  \operatorname{Cov}\left( {X,Y}\right)$   
$\operatorname{Cov}(a, X) = 0$   
$\operatorname{Cov}(a \times X, b \times Y) = a \times b \times \operatorname{Cov}(X, Y)$

# Portfolio Return and Risk

- Suppose a portfolio $P$ contains $n$ assets   
- Portfolio return will be

$$
R _ {p} = \sum_ {i = 1} ^ {n} w _ {i} R _ {i}
$$

where $\begin{array}{r}\left\{ \begin{array}{ll}w_{i} = \mathrm{weight~of~the~asset~in~the~portfolio}\\ \sum_{i = 1}^{n}w_{i} = 1 \end{array} \right. \end{array}$

Expected Portfolio Return

$$
E (R _ {p}) = \sum_ {i = 1} ^ {n} w _ {i} E (R _ {i})
$$

- Portfolio Variance

$$
\sigma_ {p} ^ {2} = \sum_ {i = 1} ^ {n} w _ {i} ^ {2} \sigma_ {i} ^ {2} + \sum_ {i = 1} ^ {n} \sum_ {i \neq j} w _ {i} w _ {j} \sigma_ {i j}
$$

where

$$
\begin{array}{l} \sigma_ {p} ^ {2} = \mathrm {v a r i a n c e} \\ \sigma_ {i} ^ {2} = \mathrm {v a r i a n c e} i \\ \sigma_ {i j} = \text {c o v a r i a n c e b e t w e e n a s s e t} I \text {a n d} j \\ \end{array}
$$

# Example

Given 2 risky assets, A and B   
• At time 0, \(P_{A,0} = \\)1\) and \(P_{B,0} = \$100\)   
- At time 1, you expect that the asset price may go up or down according to the economic conditions

<table><tr><td>Economic Condition</td><td>Probability</td><td>Asset Price A</td><td>Return of A</td><td>Asset Price B</td><td>Return of B</td></tr><tr><td>Bad</td><td>0.25</td><td>$0.6</td><td>(0.6-1)/1 = -40%</td><td>$70</td><td>(70-100)/100 = -30%</td></tr><tr><td>Normal</td><td>0.5</td><td>$1.0</td><td>(1-1)/1 = 0%</td><td>$110</td><td>(110-100)/100 = 10%</td></tr><tr><td>Good</td><td>0.25</td><td>$2.0</td><td>(2-1)/1 = 100%</td><td>$140</td><td>(140-100)/100 = 40%</td></tr></table>

• At time 0, you invest $40,000 in asset A and $60,000 in asset B (i.e. total investment $100,000). What is the expected return and variance of your portfolio?

# Example - Method 1

<table><tr><td>Economic Condition</td><td>Probability</td><td>Portfolio Value</td><td>Return of Portfolio</td></tr><tr><td>Bad</td><td>0.25</td><td>40000(1-40%) + 60000(1-30%) = 66000</td><td>66000/100000-1 = -34%</td></tr><tr><td>Normal</td><td>0.5</td><td>40000(1+0%) + 60000(1+10%) = 106000</td><td>106000/100000-1 = 6%</td></tr><tr><td>Good</td><td>0.25</td><td>40000(1+100%) + 60000(1+40%) = 164000</td><td>164000/100000-1 = 64%</td></tr></table>

- $\mu_{p} = 0.25\left(-34\%\right) + 0.5\left(6\%\right) + 0.25\left(64\%\right) = 10.5\%$   
- $\sigma_{P} = \sqrt{0.25(-34\% - 10.5\%)^{2} + 0.5(6\% - 10.5\%)^{2} + 0.25(64\% - 10.5\%)^{2}} = 34.94\%$

# Example - Method 2

$w_{A} = \frac{40000}{100000} = 0.4$   
$w_{B} = \frac{60000}{100000} = 0.6$   
- $\mu_{A} = E\left(R_{A}\right) = 0.25\left(-40\%\right) + 0.5\left(0\%\right) + 0.25\left(100\%\right) = 15 \%$   
- $\mu_{B} = E\left(R_{B}\right) = 0.25\left(-30\%\right) + 0.5\left(10\%\right) + 0.25\left(40\%\right) = 7.5 \%$   
- $\sigma_{A} = \sqrt{0.25(-40\% - 15\%)^{2} + 0.5(0\% - 15\%)^{2} + 0.25(100\% - 15\%)^{2}} = 0.5712$   
- $\sigma_{B} = \sqrt{0.25(-30\% - 7.5\%)^{2} + 0.5(10\% - 7.5\%)^{2} + 0.25(40\% - 7.5\%)^{2}} = 0.248747$   
- $\sigma_{AB} = 0.25(-40\% -15\%)(-30\% -7.5\%) + 0.5(0\% -15\%) (10\% -7.5\%) + 0.25(100\% -15\%) (40\% -7.5\%) = 0.11875$   
• $\rho_{AB} = \frac{0.11875}{0.5712 * 0.248747} = 0.92303$   
So, we have

$$
R _ {P} = w _ {A} R _ {A} + w _ {B} R _ {B}
$$

$$
\mu_ {P} = w _ {A} \mu_ {A} + w _ {B} \mu_ {B} = 0.4 * 15 \% + 0.6 * 7.5 \% = 10.5 \%
$$

$$
\sigma_ {P} = \sqrt {w _ {A} ^ {2} \sigma_ {A} ^ {2} + w _ {B} ^ {2} \sigma_ {B} ^ {2} + 2 w _ {A} w _ {B} \sigma_ {A B}} = 34.94 \%
$$

# 2 assets Portfolio

Using the previous example,   
- $\mu_{A} = 15 \%$   
- $\mu_{B} = 7.5\%$   
$\sigma_{A} = 0.5172$   
$\sigma_{B} = 0.248747$   
$\sigma_{AB} = 0.11875$   
$\rho_{AB} = 0.92303$

• For a general combination of A and B with weights $(w_{A}, w_{B})$ where $0 \leq w_{A}, w_{B} \leq 1$ and $w_{A} + w_{B} = 1$

![](images/802a07622e4579c5d0f8206bda9891f64260a907e1dd708d5c84764d52929d1d.jpg)  
Expected Return and Volatility of Portfolio (P) (with Correlation Coefficient $= 0.923026$

# 2 assets Portfolio - Correlation Effect

![](images/bbb5ee02bf44bdf91491bb59e88d29a2156fef48ec9df1640e3cda9d262bf887.jpg)

![](images/8b6b71e93b4395102003c45ff52787d5bf254e55efb411552e749b3d5875aa7a.jpg)

![](images/42187edb8faeae78f8a92fa7d6f81c608fe5584ffdf987ce2b9df9d32d374660.jpg)

![](images/307b6afaa851b44065e419ff530ed60d2dc60edcd843a5e884459cddd913c301.jpg)

![](images/f6d177c5275e5606329066d6f869b1c32aa2b9a93994c7cc3bf30ec400072e05.jpg)

# 2 assets Portfolio – Correlation Effect

- When the correlation is not very high, with appropriate weightings $(w_{A}$ and $w_{B})$ , the portfolio return can be higher than that of asset B but with a lower portfolio variance than that of asset B. As a risk averse investor, one would like to hold that portfolio rather than asset B alone.   
- The above diagrams illustrate that in most of the cases the overall risk of a portfolio of assets can be reduced but achieve the same expected return. This is called risk diversification   
- In general, if the 2 assets are more negatively correlated, it has better diversification effect

Expected Return (%)   
![](images/efe22f088163b82237160086bebcf5a00378a76d62a80ceda9e56344c93c8377.jpg)  
Standard Deviation $(\%)$

# Minimum Variance Portfolio - 2 assets

- Given

- $0 \leq w_{A}, w_{B} \leq 1$ and $w_{A} + w_{B} = 1$   
$R_{p} = w_{A} R_{A} + w_{B} R_{B}$   
$\mu_{p} = w_{A}\mu_{A} + w_{B}\mu_{B}$   
- $\sigma_{p}^{2}=w_{A}^{2} \sigma_{A}^{2}+w_{B}^{2} \sigma_{B}^{2}+2 w_{A} w_{B} \sigma_{A} \sigma_{B} \rho_{A B}$

- If $|\rho| \neq 1$ , let $w_A = w$ , $w_B = 1 - w$   
$\frac {\partial \mu_ {p}}{\partial w} = R _{A} - R _{B}$   
$\frac{\partial \sigma_{p}^{2}}{\partial w} = 2 w \left( \sigma_{A}^{2} + \sigma_{B}^{2} - 2 \rho \sigma_{A} \sigma_{B} \right) + 2 \sigma_{B} \left( \rho \sigma_{A} - \sigma_{B} \right)$   
$\frac{\partial^{2} \sigma_{p}^{2}}{\partial w^{2}} = 2 \left( \sigma_{A}^{2} + \sigma_{B}^{2} - 2 \rho \sigma_{A} \sigma_{B} \right) > 0$

So $\sigma_{p}^{2}$ has a global minimum at $w = \frac{-\sigma_{B}(\rho \sigma_{A} - \sigma_{B})}{\sigma_{A}^{2} + \sigma_{B}^{2} - 2\rho \sigma_{A} \sigma_{B}}$   
If $0 \leq w \leq 1$ , $\min(\sigma_p)$ exists iff $\rho \leq \min\left(\frac{\sigma_A}{\sigma_B}, \frac{\sigma_B}{\sigma_A}\right)$ .

$$
\mathrm {L e t} y = \mu_ {p}, x = \sigma_ {p}
$$

# Minimum Variance Portfolio - 2 assets

- If $\rho = 1$ ,

$\mu_{p} = w \mu_{A} + (1 - w) \mu_{B}$   
- $\sigma_{p}^{2} = w^{2}\sigma_{A}^{2} + (1 - w)^{2}\sigma_{B}^{2} + 2w(1 - w)\sigma_{A}\sigma_{B} = (w\sigma_{A} + (1 - w)\sigma_{B})^{2}$   
$\sigma_{p} = w \sigma_{A} + (1 - w) \sigma_{B}$

So we have

$\frac{\sigma_{p} - \sigma_{B}}{\sigma_{A} - \sigma_{B}} = \frac{\mu_{p} - \mu_{B}}{\mu_{A} - \mu_{B}}$

i.e. a straight line $\frac{x - \sigma_B}{\sigma_A - \sigma_B} = \frac{y - \mu_B}{\mu_A - \mu_B}$

# Minimum Variance Portfolio - 2 assets

- If $\rho = -1$ ,

$$
\cdot \sigma_ {p} ^ {2} = (w \sigma_ {A} - (1 - w) \sigma_ {B}) ^ {2}
$$

- So

$$
\sigma_ {p} = \left\{ \begin{array}{l l} (1 - w) \sigma_ {B} - w \sigma_ {A} & \mathrm {I f} w <   \frac {\sigma_ {B}}{\sigma_ {A} + \sigma_ {B}} \\ 0 & \mathrm {I f} w = \frac {\sigma_ {B}}{\sigma_ {A} + \sigma_ {B}} \\ w \sigma_ {A} - (1 - w) \sigma_ {B} & \mathrm {I f} w > \frac {\sigma_ {B}}{\sigma_ {A} + \sigma_ {B}} \end{array} \right.
$$

# 2 assets Portfolio (with short selling allowed)

- For a general combination between asset A and asset B with weightings $(w_{A}, w_{B})$ ,   
- No restriction on $w_{A}, w_{B}$ , except that $w_{A} + w_{B} = 1$

# Example:

• Given that at time 0, \(P_{A,0} = \\)1\( and \(P_{B,0} = \\(100\), and originally you have \(\$ 100\) cash.   
• To create a portfolio with $w_{A} = 200\%$ and $w_{B} = -100\%$ , you need to:

- buy 200 shares of asset A (cash outflow $200); and   
- shortsell 1 share of asset B (cash inflow $100)

- The total initial investment cost of the portfolio is $100.

![](images/c2c0bd56213b8fb45da8009c25b9bcf91160faf6850659264f948a96c2b8e377.jpg)  
Expected Return and Volatility of Portfolio (P) (with Correlation Coefficient $= -0.5$

# Efficient Frontier

# Given a set of risky assets,

- Opportunity Set: The set of all possible portfolios.   
- Minimum Variance Frontier (MVF): the set of portfolios with the lowest variance for a given expected return.   
- Minimum Variance Portfolio (MVP) (or global minimum variance portfolio): The portfolio with the lowest standard deviation among all possible portfolios.   
- Efficient Frontier (or efficient set): Portfolios yielding the highest expected return for a specific variance, represented as the upper curve of the MVF.   
- Inefficient Portfolio: Portfolios not on the efficient frontier, which should be avoided as better options exist for the same or lower risk

# Efficient Frontier

- For 2-asset portfolio

![](images/aac61be223da3ccc5ee88169a8e6bdea2cded5b3e83c82e7ef5bb0dd3fae4243.jpg)  
Expected Return and Volatility of Portfolio (P) (with Correlation Coefficient $= -0.5$ )

Risk and Return of Portfolio P

100% in Asset A

100% in Asset B

Shortselling Asset A

Shortselling Asset B

# Efficient Frontier

- For n-asset portfolio

E(Rp)

![](images/1409b733918e2f6ac6e3e6a0241bc19370a3c1ad992dc2e345568026c41bc7ec.jpg)

# Solving for Global Minimum Variance Portfolio for N assets

# Global Minimum Variance Portfolio

# Given

$\sum_{i=1}^{N} w_{i} = 1$   
$R_{p} = \sum_{i=1}^{N} w_{i} R_{i}$   
$\mu_{p} = E\left(R_{p}\right) = \sum_{i=1}^{N} w_{i} R_{i}$   
$\sigma_{p}^{2} = \sum_{i=1}^{N} w_{i}^{2} \sigma_{i}^{2} + 2 \sum_{i<j} w_{i} w_{j} \sigma_{i} \sigma_{j} \rho_{ij}$

# Denote:

$$
\begin{array}{l} \cdot \Sigma = \left( \begin{array}{c c c} \sigma_ {1 1} & \dots & \sigma_ {1 N} \\ \vdots & \ddots & \vdots \\ \sigma_ {N 1} & \dots & \sigma_ {N N} \end{array} \right) = \left( \begin{array}{c c c c} \sigma_ {1} & 0 & \dots & 0 \\ 0 & \sigma_ {2} & \dots & 0 \\ \vdots & \vdots & \ddots & \vdots \\ 0 & 0 & \dots & \sigma_ {N} \end{array} \right) \left( \begin{array}{c c c c} 1 & \rho_ {1 2} & \dots & \rho_ {1 N} \\ \rho_ {2 1} & 1 & \dots & \rho_ {2 N} \\ \vdots & \vdots & \ddots & \vdots \\ \rho_ {N 1} & \rho_ {N 2} & \dots & 1 \end{array} \right) \left( \begin{array}{c c c c} \sigma_ {1} & 0 & \dots & 0 \\ 0 & \sigma_ {2} & \dots & 0 \\ \vdots & \vdots & \ddots & \vdots \\ 0 & 0 & \dots & \sigma_ {N} \end{array} \right) \\ \cdot \underline {{\boldsymbol {w}}} = (w _ {1}, \dots , w _ {N}) ^ {T} \\ \cdot \underline {{\underline {{\mu}}}} = (\mu_ {1}, \dots , \mu_ {N}) ^ {T} \\ \end{array}
$$

# - So

$$
\begin{array}{l} \cdot \mu_ {p} = \underline {{w}} ^ {T} \underline {{\mu}} \\ \cdot \sigma_ {p} ^ {2} = \underline {{\boldsymbol {w}}} ^ {T} \Sigma \underline {{\boldsymbol {w}}} \\ \end{array}
$$

# Global Minimum Variance Portfolio

# Objective function:

• Min $\frac{1}{2} Var\left(R_{p}\right) = \frac{1}{2} \sum_{i=1}^{N} \sum_{j=1}^{N} w_{i} w_{j} \sigma_{ij}$

• Subject to   
$\sum_{i=1}^{N} w_{i} = 1$   
- Let $L = \frac{1}{2}\sum_{i=1}^{N}\sum_{j=1}^{N}w_{i}w_{j}\sigma_{ij} - \lambda\left(\sum_{i=1}^{N}w_{i} - 1\right)$   
- Then, $\frac{\partial L}{\partial w_k} = \sum_{i=1}^N w_i \sigma_{ik} - \lambda = 0 \quad \forall k = 1, \dots, N$   
So we have

$$
\left( \begin{array}{c c c} \sigma_ {1 1} & \dots & \sigma_ {1 N} \\ \vdots & \ddots & \vdots \\ \sigma_ {N 1} & \dots & \sigma_ {N N} \end{array} \right) \left( \begin{array}{c} w _ {1} \\ \vdots \\ w _ {N} \end{array} \right) = \lambda \left( \begin{array}{c} 1 \\ \vdots \\ 1 \end{array} \right)
$$

or

$$
\Sigma_ {\underline {{W}}} = \lambda_ {\underline {{I}} _ {N}}
$$

# Global Minimum Variance Portfolio

- Then

$$
\cdot \underline {{W}} = \lambda \Sigma^ {- 1} \underline {{I _ {N}}}
$$

As $\sum_{i=1}^{N} w_{i} = 1$ , we get

$$
1 = \underline {{I _ {N}}} ^ {T} \underline {{w}} = \lambda \underline {{I _ {N}}} ^ {T} \Sigma^ {- 1} \underline {{I _ {N}}}
$$

$$
\lambda = (\underline {{I _ {N}}} ^ {T} \Sigma^ {- 1} \underline {{I _ {N}}}) ^ {- 1}
$$

Thus,

$$
\underline {{\boldsymbol {w}}} = \frac {1}{\underline {{\boldsymbol {I _ {N}}}} ^ {T} \Sigma^ {- 1} \underline {{\boldsymbol {I _ {N}}}}} \Sigma^ {- 1} \underline {{\boldsymbol {I _ {N}}}}
$$

# Deriving Efficient Frontier for N assets

# Deriving Efficient Frontier for N assets

- The idea is simply to iterate for each target return, then find the corresponding minimum variance portfolio

![](images/926a01df3275ffa0d2dc58f5907e54291c1d952f9d549da9ccdbfedc45eda4ee.jpg)

# Optimal Portfolio with a target return

# Given

$\sum_{i=1}^{N} w_{i} = 1$   
$R_{p} = \sum_{i=1}^{N} w_{i} R_{i}$   
$\mu_{p} = E\left(R_{p}\right) = \sum_{i=1}^{N} w_{i} R_{i} = \mu_{0}$   
$\sigma_{p}^{2} = \sum_{i=1}^{N} w_{i}^{2} \sigma_{i}^{2} + 2 \sum_{i<j} w_{i} w_{j} \sigma_{i} \sigma_{j} \rho_{ij}$

# Denote:

$$
\cdot \Sigma = \left( \begin{array}{c c c} \sigma_ {1 1} & \dots & \sigma_ {1 N} \\ \vdots & \ddots & \vdots \\ \sigma_ {N 1} & \dots & \sigma_ {N N} \end{array} \right) = \left( \begin{array}{c c c c} \sigma_ {1} & 0 & \dots & 0 \\ 0 & \sigma_ {2} & \dots & 0 \\ \vdots & \vdots & \ddots & \vdots \\ 0 & 0 & \dots & \sigma_ {N} \end{array} \right) \left( \begin{array}{c c c c} 1 & \rho_ {1 2} & \dots & \rho_ {1 N} \\ \rho_ {2 1} & 1 & \dots & 0 \\ \vdots & \vdots & \ddots & \vdots \\ \rho_ {N 1} & \rho_ {N 2} & \dots & 1 \end{array} \right) \left( \begin{array}{c c c c} \sigma_ {1} & 0 & \dots & 0 \\ 0 & \sigma_ {2} & \dots & 0 \\ \vdots & \vdots & \ddots & \vdots \\ 0 & 0 & \dots & \sigma_ {N} \end{array} \right)
$$

$\underline{\underline{W}} = (w_{1},\dots ,w_{N})^{T}$

$\underline{\mu} = (\mu_{1},\dots,\mu_{N})^{T}$

# - So

$\mu_{p} = \underline{w}^{T}\underline{\mu} = \mu_{0}$   
$\sigma_{p}^{2} = \underline{w}^{T}\Sigma \underline{w}$

# Optimal Portfolio with a target return

Objective function:

$$
\cdot \operatorname {M i n} \frac {1}{2} \operatorname {V a r} \left(R _ {p}\right) = \frac {1}{2} \sum_ {i = 1} ^ {N} \sum_ {j = 1} ^ {N} w _ {i} w _ {j} \sigma_ {i j}
$$

• Subject to

$$
\begin{array}{l} \bullet \mu_ {p} = \sum_ {i = 1} ^ {N} w _ {i} \mu_ {i} = \mu_ {0} \\ \cdot \sum_ {i = 1} ^ {N} w _ {i} = 1 \\ \end{array}
$$

$\cdot$ Let $L = \frac{1}{2}\sum_{i=1}^{N}\sum_{j=1}^{N}w_{i}w_{j}\sigma_{ij} - \lambda_{1}\left(\sum_{i=1}^{N}w_{i}\mu_{i} - \mu_{p}\right) - \lambda_{2}\left(\sum_{i=1}^{N}w_{i} - 1\right)$   
$\frac{\partial L}{\partial w_{k}} = \sum_{i=1}^{N} w_{i} \sigma_{ik} - \lambda_{1} \mu_{k} - \lambda_{2} = 0 \quad \forall k = 1, \ldots, N$   
So we have

$$
\left( \begin{array}{c c c} \sigma_ {1 1} & \dots & \sigma_ {1 N} \\ \vdots & \ddots & \vdots \\ \sigma_ {N 1} & \dots & \sigma_ {N N} \end{array} \right) \left( \begin{array}{c} w _ {1} \\ \vdots \\ w _ {N} \end{array} \right) = \left( \begin{array}{c c} \mu_ {1} & 1 \\ \vdots & \vdots \\ \mu_ {N} & 1 \end{array} \right) \left( \begin{array}{c} \lambda_ {1} \\ \lambda_ {2} \end{array} \right) \qquad \mathrm {o r} \qquad \Sigma_ {\underline {{W}}} = \left(\underline {{\mu}} \underline {{I _ {N}}}\right) \left( \begin{array}{c} \lambda_ {1} \\ \lambda_ {2} \end{array} \right)
$$

# Optimal Portfolio with a target return

- That is

$$
\begin{array}{l} \cdot \underline {{w}} = \Sigma^ {- 1} \left( \begin{array}{c c} \underline {{\mu}} & I _ {N} \end{array} \right) \left( \begin{array}{c} \lambda_ {1} \\ \lambda_ {2} \end{array} \right) \dots \dots \dots ( ^ {*}) \\ \bullet \mu_ {p} = \sum_ {i = 1} ^ {N} w _ {i} \mu_ {i} = \mu_ {0} \\ \cdot \sum_ {i = 1} ^ {N} w _ {i} = 1 \\ \end{array}
$$

Now we will find $\lambda_{1}, \lambda_{2}$ such that

$$
\begin{array}{l} \cdot \quad 1 = \underline {{I _ {N}}} ^ {T} \underline {{w}} = \underline {{I _ {N}}} ^ {T} \Sigma^ {- 1} \left( \begin{array}{c c} \underline {{\mu}} & \underline {{I _ {N}}} \end{array} \right) \left( \begin{array}{c} \lambda_ {1} \\ \lambda_ {2} \end{array} \right) \\ \cdot \mu_ {0} = \underline {{\mu}} ^ {T} \underline {{w}} = \underline {{\mu}} ^ {T} \Sigma^ {- 1} \left( \begin{array}{c c} \underline {{\mu}} & \underline {{I _ {N}}} \end{array} \right) \left( \begin{array}{c} \lambda_ {1} \\ \lambda_ {2} \end{array} \right) \\ \end{array}
$$

![](images/554a4dab82504ce3faad016b15627d934bfdb06b2c49cdb635254eff165bcc19.jpg)

$$
\left( \begin{array}{c} \lambda_ {1} \\ \lambda_ {2} \end{array} \right) = \left(\left( \begin{array}{c} I _ {N} ^ {T} \\ \underline {{\mu}} ^ {T} \end{array} \right) \Sigma^ {- 1} \left( \begin{array}{c c} \underline {{\mu}} & I _ {N} \end{array} \right)\right) ^ {- 1} \left( \begin{array}{c} 1 \\ \mu_ {0} \end{array} \right)
$$

- Substitute back to equation $(^{*})$ , we have

$$
\underline {{\boldsymbol {w}}} = \boldsymbol {\Sigma} ^ {- 1} \left( \begin{array}{l l} \underline {{\boldsymbol {\mu}}} & \underline {{\boldsymbol {I _ {N}}}} \end{array} \right) \left(\left( \begin{array}{l} \underline {{\boldsymbol {I _ {N}}}} ^ {T} \\ \underline {{\boldsymbol {\mu}}} ^ {T} \end{array} \right) \boldsymbol {\Sigma} ^ {- 1} \left( \begin{array}{l l} \underline {{\boldsymbol {\mu}}} & \underline {{\boldsymbol {I _ {N}}}} \end{array} \right)\right) ^ {- 1} \left( \begin{array}{l} 1 \\ \boldsymbol {\mu_ {0}} \end{array} \right)
$$

# Tangency Portfolio

# Tangency Portfolio

- If risk-free asset is available, we can draw a tangent line between the risk-free rate and the efficient frontier   
- The toughing point is called tangency portfolio   
- We can create any portfolios along the tangent line by proper allocation on risk-free asset and the tangency portfolio

![](images/854d8af52585526e0e5395fae93774bfa7c87dcd9cad02e9ef2bac845d8fd37d.jpg)

# Deriving Tangency Portfolio

In order to find the tangency portfolio, we consider to maximize the slope

$$
\max _ {w} \frac {\mu_ {p} - R _ {f}}{\sigma_ {p}}
$$

subject to $\sum_{i=1}^{N} w_{i} = 1$

- Consider the Lagrangian

$$
L = \frac {\mu_ {p} - R _ {f}}{\sigma_ {p}} - \lambda \left(\sum_ {i = 1} ^ {N} w _ {i} - 1\right)
$$

$$
\frac {\partial L}{\partial w _ {k}} = \frac {\sigma_ {p} \frac {\partial (\mu_ {p} - R _ {f})}{\partial w _ {k}} - (\mu_ {p} - R _ {f}) \frac {\partial \sigma_ {p}}{\partial w _ {k}}}{\sigma_ {p} ^ {2}} - \lambda
$$

# Deriving Tangency Portfolio

As the risk-free rate $R_{f}$ is a constant

$$
\frac {\partial (\mu_ {p} - R _ {f})}{\partial w _ {k}} = \frac {\partial \sum_ {i = 1} ^ {N} w _ {i} \mu_ {i}}{\partial w _ {k}} = \mu_ {k}
$$

• Also,

$$
\sigma_ {p} ^ {2} = \sum_ {i = 1} ^ {N} \sum_ {j = 1} ^ {N} w _ {i} w _ {j} \sigma_ {i j}
$$

$$
2 \sigma_ {p} \frac {\partial \sigma_ {p}}{\partial w _ {k}} = 2 \sum_ {i = 1} ^ {N} w _ {i} \sigma_ {i k}
$$

$$
\frac {\partial \sigma_ {p}}{\partial w _ {k}} = \frac {1}{\sigma_ {p}} \sum_ {i = 1} ^ {N} w _ {i} \sigma_ {i k}
$$

# Deriving Tangency Portfolio

- Then $\frac{\partial L}{\partial w_k} = \frac{\sigma_p \frac{\partial (\mu_p - R_f)}{\partial w_k} - (\mu_p - R_f) \frac{\partial \sigma_p}{\partial w_k}}{\sigma_p^2} - \lambda = \frac{\sigma_p \mu_k - (\mu_p - R_f) \frac{1}{\sigma_p} \sum_{i=1}^{N} w_i \sigma_{ik}}{\sigma_p^2} - \lambda$   
- Setting $\frac{\partial L}{\partial w_k} = 0$ , we get $\sigma_p \mu_k - (\mu_p - R_f) \frac{1}{\sigma_p} \sum_{i=1}^{N} w_i \sigma_{ik} = \lambda \sigma_p^2$ .... (*).   
From this line, multiply $w_{k}$ to both side and sum $k$ from 1 to $N$

$$
\begin{array}{l} \sigma_ {p} \sum_ {k = 1} ^ {N} w _ {k} \mu_ {k} - (\mu_ {p} - R _ {f}) \frac {1}{\sigma_ {p}} \sum_ {i = 1} ^ {N} \sum_ {k = 1} ^ {N} w _ {i} w _ {k} \sigma_ {i k} = \lambda \sigma_ {p} ^ {2} \sum_ {k = 1} ^ {N} w _ {k} \\ \sigma_ {p} \mu_ {p} - (\mu_ {p} - R _ {f}) \sigma_ {p} = \lambda \sigma_ {p} ^ {2} \\ \lambda = \frac {R _ {f}}{\sigma_ {p}} \\ \end{array}
$$

# Deriving Tangency Portfolio

- Put it in equation $(^{*})$

$$
\sigma_ {p} \mu_ {k} - (\mu_ {p} - R _ {f}) \frac {1}{\sigma_ {p}} \sum_ {i = 1} ^ {N} w _ {i} \sigma_ {i k} = R _ {f} \sigma_ {p}
$$

$$
\mu_ {k} - R _ {f} = \frac {\mu_ {p} - R _ {f}}{\sigma_ {p} ^ {2}} \sum_ {i = 1} ^ {N} w _ {i} \sigma_ {i k} \qquad \mathrm {f o r k = 1 , . . . , N}
$$

Denote $z_{i} = \frac{\mu_{p} - R_{f}}{\sigma_{p}^{2}} w_{i}$ , so it becomes $\mu_{k} - R_{f} = \sum_{i=1}^{N} z_{i} \sigma_{ik}$ for $k = 1, \ldots, N$

$$
\left( \begin{array}{c} \mu_ {1} - R _ {f} \\ \vdots \\ \mu_ {N} - R _ {f} \end{array} \right) = \left( \begin{array}{c c c} \sigma_ {1 1} & \dots & \sigma_ {1 N} \\ \vdots & \ddots & \vdots \\ \sigma_ {N 1} & \dots & \sigma_ {N N} \end{array} \right) \left( \begin{array}{c} z _ {1} \\ \vdots \\ z _ {N} \end{array} \right)
$$

$$
\underline {{z}} = \Sigma^ {- 1} \left(\underline {{\mu}} - R _ {f} \underline {{I}}\right)
$$

# Deriving Tangency Portfolio

- The equation simply means that the weights are proportional to

$$
\underline {{w}} \propto \Sigma^ {- 1} \left(\underline {{\mu}} - R _ {f} \underline {{I}}\right)
$$

- With the condition $\sum_{i=1}^{N} w_{i} = 1$

$$
\underline {{\boldsymbol {w}}} = \frac {\boldsymbol {\Sigma} ^ {- 1} \left(\underline {{\boldsymbol {\mu}}} - R _ {f} \underline {{\boldsymbol {I}}}\right)}{\underline {{\boldsymbol {I}}} ^ {T} \boldsymbol {\Sigma} ^ {- 1} \left(\underline {{\boldsymbol {\mu}}} - R _ {f} \underline {{\boldsymbol {I}}}\right)}
$$

# Python Example

1. Download stock data for AAPL, MSFT, TSLA, GOOG from 2018-2020   
2. Calculate the minimum variance portfolio   
3. Suppose risk-free rate is $5\%$ , calculate the tangency portfolio

- PyPortfolioOpt (https://pypportfolioopt.readthedocs.io/en/latest/) can be used to solve various portfolio optimization problems

![](images/4fa30c0489a1842800aff364053417f3282e1682f39c770a3ac74c0675e6416a.jpg)

# Python Example

```txt
Minimum Variance Portfolio Weights:  
OrderedDict(['AAPL', 0.18907], ('GOOG', 0.55898), ('MSFT', 0.25195), ('TSLA', 0.0)])  
Tangency Portfolio Weights:  
OrderedDict(['AAPL', 0.45481], ('GOOG', 0.0), ('MSFT', 0.0), ('TSLA', 0.54519])  
Expected annual return: 28.9%  
Annual volatility: 29.5%  
Sharpe Ratio: 0.91  
Expected annual return: 88.2%  
Annual volatility: 46.1%  
Sharpe Ratio: 1.80 
```

```python
import yfinance as yf  
import pandas as pd  
from pypfopt import EfficientFrontier, risk_models, expectedreturns 
```

```txt
Step 1: Download stock data  
tickers = ['AAPL', 'MSFT', 'TSLA', 'GOOG']  
start_date = '2018-01-01'  
end_date = '2020-12-31' 
```

```python
data = yf.download(tickers, start=start_date, end=end_date)[['Close'] # Step 2: Calculate expected returns and covariance matrix mu = expectedreturns.mean_historical_return(data) S = risk_models.sample_cov(data) 
```

```python
Minimum Variance Portfolio ef_mv = EfficientFrontier(mu, S)  
weights_mv = ef_mv.min_volatility()  
cleanedweights_mv = ef_mv.cleanweights()  
print("Minimum Variance Portfolio Weights:")  
print(cleannedweights_mv) 
```

```txt
Step 3: Calculate Tangency Portfolio risk free rate = 0.05 # 5% 
```

```python
ef_tangency = EfficientFrontier(mu, S, weight_bounds=(0, 1))  
weights_tangency = ef_tangency.max_sharpe(risk_free_rate=risk_free_rate)  
cleanedweights_tangency = ef_tangency.cleanweights()  
print("\\nTangency Portfolio Weights:")  
print(cleannedweights_tangency) 
```

```python
Display portfolio performance  
performance_mv = ef_mvportfolio_performance(verbatim=True)  
performance_tangency = ef_tangencyportfolio_performance(verbatim=True) 
```

# ALGOGENE Example

![](images/87c2df81f8c2a18cdbc7c0be67c47301b0d2b803125a5787ff8ece5c1f4804b8.jpg)

# Capital Asset Pricing Model (CAPM)

# Background

- We learnt from previous section how to construct an efficient frontier and determine the tangency portfolio   
- When the number of assets increase, the number of parameters will increase quadratically $O(N^{2})$ . It leads to difficulty to compute for a large portfolio.   
- So a simplified financial model is preferred

# Market Portfolio

- Market portfolio (M) is a portfolio consisting of ALL risky assets in the economy.

- Note that ALL risky assets are not limited to stocks. The market portfolio include stocks, bonds, real estates, human capital etc. It includes both tradable and non-tradable assets.

Each asset is held in a proportion according to its market value:

$$
w _ {i} ^ {*} = \frac {\text {M a r k e t V a l u e o f A s s e t i}}{\text {M a r k e t V a l u e o f A l l A s s e t s}} = \frac {P _ {i} S _ {i}}{\sum_ {j = 1} ^ {N} P _ {j} S _ {j}}
$$

where

- $N =$ number of risky assets in the economy   
- $P_{i} =$ market price of asset i per unit of shares   
- $S_{i} =$ number of outstanding shares of asset i

# Market Portfolio

# A Major Conclusion under Market Equilibrium

- Recall that ALL (risk-averse) investors will hold portfolios formed by the risk-free asset and the tangency portfolio (T).   
- Under market equilibrium, the tangency portfolio (T) [Demand for risky assets] MUST be exactly the same as the market portfolio (M) [Supply for risky assets] in the economy.

# CAPM Assumptions

1. Assets are all tradable and are all infinitely divisible.   
2. All investors are risk-averse and select their portfolios in a single-period time horizon based on the mean-variance criterion (Markowitz assumptions).   
3. All investors have the same belief in expected returns, variances and covariances of all assets (homogeneous expectations).   
4. All investors can do unlimited short selling of risky assets, and borrow or lend money at the same risk-free interest rate.   
5. No taxes, no transaction costs, information is costless and available to all investors.

- Assumptions 2,3,4 imply that all investors obtain the same efficient frontier   
- Using one-fund theorem, every investor hold a portfolio consisting of the tangency portfolio (T) and risk-free asset, with different portfolio weightings

# Capital Market Line (CML)

- For any efficient portfolio on CML,

$$
E \big (R _ {p} \big) = R _ {f} + \frac {E (R _ {M}) - R _ {f}}{\sigma_ {M}} \sigma_ {p} \qquad \mathrm {o r} \qquad \mu_ {p} = R _ {f} + \frac {\mu_ {M} - R _ {f}}{\sigma_ {M}} \sigma_ {p}
$$

![](images/36f710232cdf2a68cbe7ddd9b2bf3e67f3c951149825f8fb6f17ad009249ea38.jpg)

Note: The dashed line represents the efficient frontier of portfolios which consist of all risky assets.

# Deriving CAPM (Sharpe's Approach)

- Consider a portfolio $i$ and market portfolio $M$   
Note that for every portfolio $i$ , it cannot cross the capital market line   
- For a portfolio between $i$ and $M$ , we have

$$
\begin{array}{l} \cdot \mu_ {p} = w \mu_ {i} + (1 - w) \mu_ {M} \\ \cdot \sigma_ {p} = [ w ^ {2} \sigma_ {i} ^ {2} + (1 - w) ^ {2} \sigma_ {M} ^ {2} + 2 w (1 - w) \sigma_ {i j} ] ^ {1 / 2} \\ \end{array}
$$

• Then,

$$
\begin{array}{l} \frac {\partial \mu_ {p}}{\partial w} = \mu_ {i} - \mu_ {M} \\ \frac {\partial \sigma_ {p}}{\partial w} = \frac {w \sigma_ {i} ^ {2} + (1 - 2 w) \sigma_ {i M} - (1 - w) \sigma_ {M} ^ {2}}{\sigma_ {p}} \\ \end{array}
$$

- Consider the slope at M (i.e. $w = 0$ ) $\frac{\partial \sigma_{p}}{\partial w} = \frac{\sigma_{iM} - \sigma_{M}^{2}}{\sigma_{M}}$

![](images/7ee59a3c87c147b5e45d500032ebee0b8fea9006fb3981b35df562be72c05563.jpg)

# Deriving CAPM (Sharpe's Approach)

- Note that the slope of CML at portfolio M is $\frac{\mu_{M} - R_{f}}{\sigma_{M}}$   
So we have

$$
\frac {\mu_ {M} - R _ {f}}{\sigma_ {M}} = \frac {(\mu_ {i} - \mu_ {M}) \sigma_ {M}}{\sigma_ {i M} - \sigma_ {M} ^ {2}}
$$

$$
\frac {\partial \mu_ {p}}{\partial w} / \frac {\partial w}{\partial \sigma_ {p}} \mathrm {w h e n} w = 0
$$

$$
\mu_ {i} = R _ {f} + \frac {\sigma_ {i M}}{\sigma_ {M} ^ {2}} (\mu_ {M} - R _ {f})
$$

• Define $\beta_{i} = \frac{\sigma_{i M}}{\sigma_{M}^{2}}$

CAPM model: $\mu_{i} = R_{f} + \beta_{i}\left(\mu_{M} - R_{f}\right)$

# Security Market Line (SML)

- SML is graphical representation of the CAPM   
- For any asset $i$

Expected market risk premium

$$
E (R _ {i}) = R _ {f} + \underbrace {\beta_ {i} \left(E (R _ {M}) - R _ {f}\right)} _ {\mathrm {r e s p e c i e d}} \qquad \mathrm {o r} \qquad \mu_ {i} = R _ {f} + \beta_ {i} \left(\mu_ {M} - R _ {f}\right)
$$

Expected risk premium

Properties of SML

$\beta_{M} = 1$   
$\beta_{R_f} = 0$   
- Beta of a portfolio $\beta_{p} = \sum_{i=1}^{N} w_{i} \beta_{i}$   
- Every assets lie on SML   
- For 2 assets A and B, $\frac{E(R_A) - R_f}{\beta_A} = \frac{E(R_B) - R_f}{\beta_B} = \frac{E(R_M) - R_f}{1}$

![](images/96d5ce21d4e6290212b334ce5e32404d24de56dfc38c8f5b7db3dbcd88b29b71.jpg)

# CAPM

- Consider a linear regression model

$$
(R _ {i} - R _ {f}) _ {t} = \alpha_ {i} + \beta_ {i} (R _ {M} - R _ {f}) _ {t} + e _ {i, t} \quad \mathrm {t = 1 , 2 , \ldots , T}
$$

We have

$\widehat{\beta}_{i} = \frac{\sum_{t=1}^{T}\left(R_{M,t}-\mu_{M}\right)\left(R_{i,t}-\mu_{i}\right)}{\sum_{t=1}^{T}\left(R_{M,t}-\mu_{M}\right)^{2}} = \frac{\operatorname{Cov}\left(R_{M}, R_{i}\right)}{\operatorname{Var}\left(R_{M}\right)} = \frac{\sigma_{i M}}{\sigma_{M}^{2}}$   
$R^{2} = \frac{\beta_{i}\sigma_{M}^{2}}{\sigma_{i}^{2}} = (\rho_{iM})^{2}$   
- $\sigma_{i}^{2} = \beta_{i}^{2}\sigma_{M}^{2} + \tau_{i}^{2}$

Systematic risk that is non-diversifiable

Non-systematic risk that is diversifiable

![](images/0c828c130c924fb556c21e26da651cafec6fb0ad423e5959182a87947edd5434.jpg)

# Non-systematic risk

- Consider an equally weighted portfolio

$$
R _ {p} = \sum_ {i = 1} ^ {N} \frac {R _ {i}}{N} = \frac {1}{N} \sum_ {i = 1} ^ {N} \left[ R _ {f} + \beta_ {i} (R _ {M} - R _ {f}) + e _ {i} \right] = R _ {f} + \beta_ {p} (R _ {M} - R _ {f}) + \frac {1}{N} \sum_ {i = 1} ^ {N} e _ {i}
$$

- Then,

$$
\sigma_ {p} ^ {2} = \beta_ {p} ^ {2} \sigma_ {M} ^ {2} + \frac {1}{N ^ {2}} \sum_ {i = 1} ^ {N} \tau_ {i} ^ {2}
$$

Tends to 0 if N is large

# The Beta Coefficient

- The beta coefficient is a key parameter in the capital asset pricing model (CAPM).   
- Beta (also referred to as financial elasticity or correlated relative volatility) can be regarded as a measure of the sensitivity of the asset's returns to market returns.   
- For a well diversified portfolio,

- If $\beta_{i} > 1$ , the portfolio is more aggressive than the market portfolio.   
- If $\beta_{i} = 1$ , the portfolio is as aggressive than the market portfolio   
- If $\beta_{i} < 1$ , the portfolio is less aggressive than the market portfolio

# Performance Measures

# Which investment do you prefer?

![](images/1f9fcc4c26c964729b6b81a53d4324c0a945d28090b5f926e2a714fd4c807a6c.jpg)  
Two Equal Return Investments: Which is Better?

# Reasons for Performance Evaluation

- Fund managers want to know   
- how good their portfolio performs   
- the ability to diversify a portfolio effectively   
- Investors want to   
- compare different investment products in the market   
- whether their investment advisors/ fund managers have “value added”

# Performance Measures

1. Sharpe Ratio   
2. Sortino Ratio   
3. Treynor Ratio   
4. Jensen's Alpha   
5. Information Ratio   
6. Maximum Drawdown   
7. Calmar Ratio

# Sharpe Ratio

- A risk-adjusted measure introduced by Sharpe in 1966   
- Risk is measured by standard deviations of portfolio returns   
- Sharpe ratio (SR) is the reward-to-variability ratio for a portfolio.

$$
S R _ {p} = \frac {E (R _ {p}) - R _ {f}}{\sigma_ {p}}
$$

where $E\big(R_p\big)$ and $\sigma_{p}$ are the annualized expected return and standard derivation respectively

• Aim:

- Compare $S R_{p}$ with the slope of the ex-post CML in which

- If $S{R}_{p} > S{R}_{M}$ ,our portfolio beat the market.   
- If $S{R}_{p} < S{R}_{M}$ ,our portfolio lost to the market.

We may also compare $S R_{p}$ with the slope of the peers.

# Sharpe Ratio

- An investment is considered to be sensible if Sharpe ratio is at least 1. i.e.

- for 1 unit of risk taken, you can get at least 1 unit of returns

In the asset management industry,   
It is very common to see funds having Sharpe ratio between 0.5 to 1.5   
- A fund manager that achieve Sharpe ratio between 1.5 to 2.5 is considered to be good (around top $10\%$ in the market)   
- A Sharpe ratio above 2.5 is exceptionally good investment (around the top $1 \%$

# Sharpe Ratio

- We can interpret Sharpe ratio in another way   
- Suppose risk-free rate $R_{f}$ is zero and return is normally distributed   
- Let's say we want Sharpe ratio to be greater than $z$

$$
\frac {E (R _ {p}) - R _ {f}}{\sigma_ {p}} > z \quad \Rightarrow \quad \mu_ {p} - z \sigma_ {p} > 0
$$

- For $z = 1$ , we have $0.5 + 0.6827 / 2 = 84.14\%$ probability for having positive return in a 1-year horizon.   
- For $z = 2$ , we have $0.5 + 0.9545 / 2 = 97.73\%$ probability for having positive return in a 1-year horizon.   
- For $z = 3$ , we have $0.5 + 0.9973 / 2 = 99.87\%$ probability for having positive return in a 1-year horizon.

![](images/3570ee38fdd094cba17604ae1bd71380aa04d362cfbcc7157005d77a2d4eb0ad.jpg)

# Sharpe Ratio - example

- Suppose risk-free rate is $1\%$ . You are given the following monthly returns of a fund. Calculate the Sharpe ratio.

<table><tr><td>Month</td><td>Jan-24</td><td>Feb-24</td><td>Mar-24</td><td>Apr-24</td><td>May-24</td><td>Jun-24</td><td>Jul-24</td><td>Aug-24</td><td>Sep-24</td><td>Oct-24</td><td>Nov-24</td><td>Dec-24</td></tr><tr><td>Return</td><td>1.20%</td><td>2.30%</td><td>-1.50%</td><td>0.80%</td><td>1.70%</td><td>-2.10%</td><td>3.20%</td><td>0.40%</td><td>2.10%</td><td>-0.90%</td><td>1.10%</td><td>0.30%</td></tr></table>

- Answer:

- Average monthly return $= \frac{\left( {{1.2}\%  + {2.3}\%  + \cdots  + {0.3}\% }\right) /{12}} = {0.7167}\%$   
Monthly std dev = $\sqrt{\frac{(1.2\%-0.7167\%)^2+(2.3\%-0.7167\%)^2+\cdots+(0.3\%-0.7167\%)^2}{12-1}}$ = 1.59%   
- Annualized return $= {12}^{ * }{0.7167}\%  = {8.6}\%$   
Annual std dev = 1.59%*√12 = 5.4948%   
- Sharp ratio $= \frac{8.6\% - 1\%}{5.4948\%} = 1.383$

# Sharpe Ratio - example

# Given the following info:

average annual risk-free return was $5 \%$   
average annual return on the S&P 500 was $24.1\%$   
average annual return on Fund A was $20.5\%$   
average annual return on Fund B was $11.8\%$   
- betas of Fund A and Fund B are 1.0 and 0.6, respectively   
standard deviation of S&P 500 was $14\%$   
standard deviation of Fund A was $15\%$   
standard deviation of Fund B was $10\%$

# We evaluate the performance of the two funds

• $S R_{F u n d A} = (20.5 - 5) / 15 = 1.003$   
• $S R_{F u n d B} = (11.8 - 5) / 15 = 0.68$   
• $S R_{S \& P 500} = (24.1 - 5) / 14 = 1.346$

none of the funds can beat the market

# Sharpe Ratio - example

- When the dotted lines of portfolios (Fund A and Fund B) are flatter than the solid line (the ex-post CML for S&P 500), the performance of both portfolios is worse than the market portfolio because the CML represents the efficient allocation of capital between market portfolio and risk-free asset.

![](images/ace886ecdeb3177799c8d81c0eb6277f08aa84da7ac267fb208cc7519357c1be.jpg)

# Sharpe Ratio

- Advantages

- Easy to interpret and calculate   
Useful for comparing different portfolios

- Limitations

- Sensitive to the choice of the risk-free rate and sampling period

# Sortino Ratio

- Sortino Ratio is similar to Sharpe Ratio but focuses on downside risk $\sigma_{d}$

$$
\begin{array}{l} {S o r t i n o R a t i o = \frac {E (R _ {p}) - R _ {f}}{\sigma_ {d}}} \\ {\mathrm {w h e r e}} \end{array} \left\{ \begin{array}{l} {\mu_ {d} = \frac {1}{n} \sum_ {i = 1} ^ {n} \min (0, r _ {i})} \\ {\sigma_ {d} = \sqrt {\frac {1}{n - 1} \sum_ {i = 1} ^ {n} [ \min (0 , r _ {i}) - \mu_ {d} ] ^ {2}}} \end{array} \right.
$$

- Aim: choose the portfolio with the highest Sortino Ratio

# Sortino Ratio - example

- Assume risk free rate $= 0\%$   
Given a return series $= \{1\%, 1.2\%, -0.5\%, -1.6\%, 2.1\% \}$   
- Downside return series = $\{0\%, 0\%, -0.5\%, -1.6\%, 0\%\}$

We have

$$
\begin{array}{l} \cdot E \left(R _ {p}\right) = \frac {1 \% + 1.2 \% - 0.5 \% - 1.6 \% + 2.1 \%}{5} = 0.0044 \\ \cdot \mu_ {d} = \frac {0 + 0 - 0 . 5 \% - 1 . 6 \% + 0}{5} = - 0. 0 0 4 2 \\ \cdot \sigma_ {d} = \sqrt {\frac {(0 + 0 . 0 0 4 2) ^ {2} + (0 + 0 . 0 0 4 2) ^ {2} + (- 0 . 5 \% + 0 . 0 0 4 2) ^ {2} + (- 1 . 6 \% + 0 . 0 0 4 2) ^ {2} + (0 + 0 . 0 0 4 2) ^ {2}}{4}} = 0. 0 0 6 9 4 2 6 2 \\ \cdot \text {S o r t i n o R a t i o} = \frac {0 . 0 0 4 4 - 0}{0 . 0 0 6 9 4 2 6 2} = 0. 6 3 3 8 \\ \end{array}
$$

# Sortino Ratio

Pros:   
Take into account of downside risk   
- Cons:   
- Also sensitive to the choice of risk-free rate and sampling period

# Treynor Ratio (1965)

- Similar to Sharpe Ratio but risk is measured by portfolio beta

$$
T r e y n o r R a t i o = \frac {E (R _ {p}) - R _ {f}}{\beta_ {p}}
$$

• Aim:

- Choose the portfolio with the highest Treynor Ratio   
- Compare $T R_{p}$ with the slope of the ex-post SML

If $TP_{p} > TP_{M}$ , our portfolio beat the market.   
If $TP_{p} < TP_{M}$ , our portfolio lost to the market.

We may also compare $T P_{p}$ with the slope of the peers.

# Treynor Ratio - example

# Given the following info:

average annual risk-free return was $5 \%$   
average annual return on the S&P 500 was $24.1\%$   
average annual return on Fund A was $20.5\%$   
average annual return on Fund B was $11.8\%$   
- betas of Fund A and Fund B are 1.0 and 0.6, respectively   
standard deviation of S&P 500 was $14\%$   
standard deviation of Fund A was $15\%$   
standard deviation of Fund B was $10\%$

# We evaluate the performance of the two funds

• $T R_{F u n d A} = (20.5\% - 5\%) / 1 = 0.155$   
• $T R_{F u n d B} = (11.8\% - 5\%) / 0.6 = 0.113$   
- $TR_{S\& P500} = (24.1\% - 5\%) / 1 = 0.191$

none of the funds can beat the market

# Treynor Ratio - example

- When the dotted lines of portfolios (Fund A and Fund B) are flatter than the solid line (the ex-post SML for S&P 500), the performance of both portfolios is worse than the market index.

![](images/477064c4920e9ac7d1cd5c6ef41d1e039494b3e1d4b3e5a160d919d06b75c0fb.jpg)

# Jensen's Alpha (1968)

- Jensen's Alpha (from the SML)

$$
\alpha_ {p} = E \big (R _ {p} \big) - \big \{R _ {f} + \beta_ {p} \big [ E (R _ {M}) - R _ {f} \big ] \big \}
$$

Expected return from CAPM

- $\alpha_{p}$ can be estimated from

$$
(R _ {p} - R _ {f}) _ {t} = \alpha_ {p} + \beta_ {p} (E (R _ {M}) - R _ {f}) _ {t} + e _ {p, t}
$$

We can test for $H_{0}$ : $\alpha_{0} = 0$   
If $\alpha_0 > 0$ , our portfolio beat the market   
If $\alpha_0 < 0$ , our portfolio beat the market

# Jensen's Alpha - example

# Given the following info:

average annual risk-free return was $5 \%$   
average annual return on the S&P 500 was $24.1\%$   
average annual return on Fund A was $20.5\%$   
average annual return on Fund B was $11.8\%$   
- betas of Fund A and Fund B are 1.0 and 0.6, respectively   
standard deviation of S&P 500 was $14\%$   
standard deviation of Fund A was $15\%$   
standard deviation of Fund B was $10\%$

# We evaluate the performance of the two funds

- $\alpha_{Fund A} = 20.5\% - [5\% + 1(24.1 - 5)] = -3.6\%$ .   
- $\alpha_{\text {Fund } B} = 11.8\% - [5\% + 0.6(24.1 - 5)] = -4.66\%$

none of the funds can beat the market

# Jensen's Alpha - example

- When the points of portfolios (Fund A and Fund B) are below the ex-post SML for S&P 500, the performance of both portfolios is worse than the market index.

![](images/25965b232fa8acc0ce35488e5c100a4c32db7a70938c1862c3ed9ef8626bc15d.jpg)

# Jensen's Alpha

- Note that Treynor Ratio and Jensen's Alpha can be illustrated in return-beta plane.

![](images/b33111658187dd671dc9dee73b724827f8af2938517d7723aea90d90f0d661d6.jpg)

# Comparing the Measures

• $T R_{p}$ and $\alpha_{p}$ usually give the same sign and ranking.   
- It is possible that $S{R}_{p}$ indicate our portfolio lost to the market while $T{R}_{p}$ and ${\alpha }_{p}$ indicate that our portfolio beat the market because the presence of non-systematic risk in our portfolio.   
- $SR_{p}$ can rank a set of portfolios differently than $TR_{p}$ and $\alpha_{p}$ since they use different risk measures.   
- All the measures may not be meaningful if $E(R_{M}) < R_{f}$ since both the SML and CML will be downward sloping.   
- All the measures have been criticized for the subjectivity in the choice of the market index and the risk-free rate.

# Information Ratio

- Other than comparing with the risk-free rate $R_{f}$ , we can also choose a benchmark rate of return, $R_{b}$ , for comparison.   
- Define the tracking error

$$
T E = \sigma (R _ {p} - R _ {b})
$$

- Then the information ratio (or appraisal ratio) is

$$
I R = \frac {E (R _ {p} - R _ {b})}{\sigma (R _ {p} - R _ {b})}
$$

- Aim: choose the portfolio with the largest Information Ratio

# Information Ratio

- Suppose the benchmark is the portfolio on SML with the same beta and its expected return is

$$
E (R _ {b}) = R _ {f} + \beta_ {p} [ E (R _ {p}) - R _ {f} ]
$$

Using $R_{p} - R_{f} = \alpha_{p} + \beta_{p}\big[R_{M} - R_{f}\big] + e_{p}$ with $\operatorname {E}(e_p) = 0$ and $\operatorname {Var}\bigl (e_p\bigr) = \tau_p^2$   
Therefore,

$$
\left\{\begin{array}{r l r}E (R _ {p} - R _ {b}) = \alpha_ {p}&&\rightarrow I R = \frac {\alpha_ {p}}{\tau_ {p}}\\\sigma (R _ {p} - R _ {b}) = \tau_ {p}&&\end{array}\right.
$$

# Information Ratio

- Using information ratio, evaluate the performance of the two funds:   
- Recall that

$$
\alpha_ {F u n d A} = 20.5 \% - [ 5 \% + 1 (24.1 - 5) ] = -3.6 \%
$$

$$
\alpha_ {F u n d B} = 11.8 \% - [ 5 \% + 0.6 (24.1 - 5) ] = -4.66 \%
$$

• Also, $\sigma_{p}^{2} = \beta_{p}^{2} \sigma_{M}^{2} + \tau_{p}^{2}$ . So we have

$$
\tau_ {F u n d A} = \sqrt {1 5 ^ {2} - 1 ^ {2} 1 4 ^ {2}} = 5. 3 8 5 2
$$

$$
\tau_ {F u n d B} = \sqrt {10 ^ {2} - 0.6 ^ {2} 14 ^ {2}} = 5.4259 \%
$$

Therefore,

$$
I R _ {F u n d A} = \frac {- 3 . 6}{5 . 3 8 5 2} = - 0. 6 6 8 5
$$

$$
I R _ {F u n d B} = \frac {- 4 . 6 6}{5 . 4 2 5 9} = - 0. 8 5 8 8
$$

$\rightarrow$ Fund A is better than Fund B, but both perform worse than the benchmark

# Information Ratio

# Remarks:

- If $R_{p} - R_{f} = \alpha_{p} + \beta_{p}\left[R_{M} - R_{f}\right] + e_{p}$ is fitted using monthly data, we obtain monthly $IR =$

- Annualized $IR = \frac{12\alpha_p}{\sqrt{12}\tau_p} = \sqrt{12} IR$

- Grinold and Kahn (2000, Active Portfolio Management, McGrawHill, 2nd Ed) argued that reasonable IR should range from 0.50 to 1.00.

Good: IR > 0.5   
- Exceptional: IR > 1.0

# Maximum Drawdown

- Maximum Drawdown (MDD), expressed as a percentage, measures the largest peak-to-trough decline in the value of an investment portfolio   
- Indicates potential risk and the worst-case scenario for an investor.   
- Useful for assessing the volatility and risk of an investment strategy.   
- Formula:

$$
M D D = \frac {P _ {p e a k} - P _ {t r o u g h}}{P _ {p e a k}}
$$

where $P_{peak} = \text{Maximum value (peak) of the portfolio}$ $P_{trough} = \text{Minimum value (trough) following the peak}$

# Maximum Drawdown

![](images/f6f6a101506ed9c7f2c0abccf145a0dab8d89c64546732d24bd6b9080ff7d8d5.jpg)  
Time

# Maximum Drawdown - example

- Suppose the daily portfolio value is [100, 120, 90, 110, 80, 130, 70, 150]

import numpy as np   
def calculate_mdd(data): portfolio_values $\equiv$ np.array(data) #Calculate the running maximum running_max $=$ npmaximumaccumulate.portfolio_values) #Calculate drawdowns drawdowns $=$ (running_max - portfolio_values)/running_max #Maximum Drawdown max_drawdown $=$ np.max drawnowns return max_drawdown   
mdd $=$ calculate_mdd([100, 120, 90, 110, 80, 130, 70, 150])   
print(f"Maximum Drawdown: {mdd:.}")

Maximum Drawdown: $46.15\%$

# Calmar Ratio

- The Calmar Ratio measures the performance of an investment relative to its maximum drawdown.   
- Formula:

$$
C a l m a r R a t i o = \frac {A n n u a l i z e d R e t u r n}{\left| M a x . D r a w d o w n \right|}
$$

Pros: risk adjusted metric that focus on downside risk   
- Cons: ignore the frequency of drawdown or volatility within the period

# Calmar Ratio - Example

• Given a portfolio's monthly value: [100, 120, 90, 110, 80, 130, 70, 150], what is the Calmar Ratio?   
- Answer:

- Max. Drawdown = 46.15%   
- Annualized Return = (20% - 25% + 22.22% +...+ 114.29%) / 7*12 = 206.71%   
- Thus, Calmar Ratio = 206.71%/ 46.15% = 4.48

<table><tr><td>Month</td><td>Portfolio Value</td><td>Monthly Return</td><td>Drawdown</td></tr><tr><td>1</td><td>100</td><td></td><td>0.00%</td></tr><tr><td>2</td><td>120</td><td>20.00%</td><td>0.00%</td></tr><tr><td>3</td><td>90</td><td>-25.00%</td><td>25.00%</td></tr><tr><td>4</td><td>110</td><td>22.22%</td><td>8.33%</td></tr><tr><td>5</td><td>80</td><td>-27.27%</td><td>33.33%</td></tr><tr><td>6</td><td>130</td><td>62.50%</td><td>0.00%</td></tr><tr><td>7</td><td>70</td><td>-46.15%</td><td>46.15%</td></tr><tr><td>8</td><td>150</td><td>114.29%</td><td>0.00%</td></tr></table>

# Choosing an Appropriate Measure

- The criterion to choose an appropriate performance measure depends on the investment assumptions.

- If the portfolio represents our entire risky investment fund, use Sharpe, Sortino or Calmar Ratio.   
- For an actively managed portfolio with benchmark comparison, use Information ratio   
- If the portfolio is one of many portfolios combined into a large investment fund, use the Jensen's alpha or the Treynor ratio but the Treynor ratio is more complete because it adjusts for risk.

# Inclusion of a new asset

- If we are holding a portfolio $P$ , we will add the new asset $Q$ to our portfolio if

$$
S R _ {Q} > S R _ {P} C o r r (R _ {Q}, R _ {P})
$$

![](images/9fdfd563f76da05f371b4c658cbb8cee95c7d65065ad3bb7fa7bbe262aea1a51.jpg)

# Stop Loss & Risk Limit Control

# Stop Loss

- Minimizes losses on losing trades.   
• Removes emotional decision-making during losses.   
- Types of Stop Loss

1. Fixed Stop Loss   
2. Trailing Stop Loss   
3. Dollar Risk Approach   
4. Portfolio level Stop Loss

Trade level

Already discussed in L6

# Fixed Stop Loss

- Set the stop loss at a specific price level.   
• For example, we buy 1 share of a stock at $100 and set stop loss at$ 90.

- When price drops significantly, our position will be automatically closed at \(90.   
- The maximum loss of this trade is capped at $10.

# Fixed Stop Loss

- Implement on ALGOGENE (https://algogene.com/TechDoc#PlaceOrder)   
- Eg. We set TP and SL at $10\%$ from the current mid price

```python
def on_marketdatafeed(self, md, ab):  
    sl = md.midPrice*0.9  
    tp = md.midPrice*1.1  
    order = AlgoAPIUtil.OrderObject(instrument = md.instrument, openclose = 'open', buysell = 1, #1=buy, -1=sell  
    ordertype = 0,  
    volume = 1.  
    stopLossLevel = sl,  
    takeProfitLevel = tp  
)  
self.evt.sendOrder(order) 
```

# Trailing Stop Loss

- A trailing stop loss is a dynamic order that adjusts as the price of an asset moves in your favor, locking in profits while limiting potential losses.   
- Mechanism: (assume buy order)

- Set at a specific percentage or amount below the market price.   
- As the price increases, the stop loss level moves up, but it does not move down.

- Benefits:

- Protects profits while allowing for potential gains as the price increases.   
- Reduces the need for constant monitoring.

![](images/30fa209ea2170f2dfd3fd113a7a1b9f36a46b48729156b97e212def6ec7b5d43.jpg)

# Trailing Stop Loss

Example:

- Initial Position: Buy 100 shares of XYZ stock at $50.   
- Trailing Stop Loss: Set at $5 below the highest price reached.

- Price Movement:

• If XYZ rises to $55, the trailing stop loss adjusts to $50 (i.e. $55 - $5).   
- If XYZ then rises to $60, the trailing stop loss moves up to $55.   
- If XYZ drops to $55, the order triggers a sell, locking in a $5 profit per share.

- Outcome:

- Final Sell Price: $55 (when trailing stop is triggered).   
Profit: $(\$ 55 - \$ 50) $x 100 shares = \$ 500.

# Trailing Stop Loss

- Implement on ALGOGENE (https://algogene.com/TechDoc#PlaceOrder)   
- Eg. We set trailing stop at $5 \%$ from its highest value

def on_marketdatafeed(self, md, ab):

```python
order = AlgoAPIUtil.OrderObject( instrument = md.instrument, openclose = 'open', buysell = 1, #1=buy, -1=sell ordertype = 0, volume = 1, trailingstop = 0.05  
) self.evt.sendOrder(order) 
```

# Portfolio Stop Loss

- When a portfolio involves multiple positions, instruments, or trading strategies, setting only a trade level stop loss will not be sufficient.   
- A portfolio stop loss acts as a $2^{\text{nd}}$ layer protection for overall capital.

![](images/3b0640af1b7751b9a031578e5f83b6a9ca3cad517a2c4fbdd7013725af93a344.jpg)

# Portfolio Stop Loss

- Implement on ALGOGENE

(https://algogene.com/TechDoc#UpdatePortfolioStop)

- Eg. We set portfolio stop at $10\%$ from its highest value

class AlgoEvent:

def __init__(self):

pass

def start(self, mEvt):

self.evt = AlgoAPI_Backtest.AlgoEvtHandler(self, mEvt)

self.evt.update portfolioslsl $\mathsf{sl} = 0.1$ ,resume_after $= 60^{*}60^{*}24^{*}7$

self.evt.start()

<table><tr><td>&#x27;self.evt.update portfoliosl&#x27; INPUT
PARAMETER</td><td>IS
NECESSARY</td><td>DATA
TYPE</td><td>DESCRIPTIONS</td><td>SAMPLE</td></tr><tr><td>sl</td><td>Yes</td><td>float</td><td>• the stoploss (in percentage) from a portfolio NAV&#x27;s high water mark
• value between 0 to 1. eg. 0.1 refers to cutting all positions when a portfolio NAV drops 10% from its previous highest level.
• set the value to 0 to disable this feature
• once this stoploss condition trigger, the high water level will be reset to the latest NAV</td><td>0.1</td></tr><tr><td>resume_after</td><td>No</td><td>float</td><td>• when the portfolio stoploss event trigger, this parameter refers to the &quot;cooling period&quot; before resuming the algo
• unit in second, and default is 0</td><td>60*60*24</td></tr></table>

# Risk Limit Stoploss

- Risk managers usually apply additional limits for traders

Daily stop loss   
Weekly stop loss   
Monthly stop loss   
• Quarterly stop loss   
- Yearly stop loss   
VaR Limit   
- DV01 Limit   
- Delta Limit   
0

# Risk Limit Stoploss

- Implement on ALGOGENE (https://algogene.com/TechDoc#UpdateRiskLimitStopLoss)   
- Eg. We set $5 \%$ monthly and $10 \%$ yearly stop loss

class AlgoEvent:

def __init__(self):

pass

def start(self, mEvt):

self.evt = AlgoAPI_Backtest.AlgoEvtHandler(self, mEvt)

self.evt.update_risk_limit_sl(sl=0.05, risk_type='m')  
self.evt.update_risk_limit_sl(sl=0.1, risk_type='y')

self.evt.start()

<table><tr><td>&#x27;self.evt.update_risk_limit_sI&#x27; INPUTPARAMETER</td><td>ISNECESSARY</td><td>DATATYPE</td><td>DESCRIPTIONS</td><td>SAMPLE</td></tr><tr><td>sl</td><td>Yes</td><td>float</td><td>• the stoploss (in percentage) from the portfolio NAV at a period start • value between 0 to 1. eg. 0.1 refers to cutting all positions when a portfolio NAV drops 10% from its previous highest level.• set the value to 0 to disable this feature • once this stoploss condition trigger, the high water level will be reset to the latest NAV</td><td>0.1</td></tr><tr><td>risk_type</td><td>Yes</td><td>string</td><td>• set the risk limit type • &#x27;d&#x27; for daily stop loss limit • &#x27;w&#x27; for weekly stop loss limit • &#x27;m&#x27; for monthly stop loss limit • &#x27;q&#x27; for quarterly stop loss limit • &#x27;y&#x27; for yearly stop loss limit</td><td>&#x27;m&#x27;</td></tr></table>

# Key Takeaways

We learnt how to calculate the overall risk and return of an investment portfolio   
- The lower the correlation, the better the diversification effect will be   
- Efficiency Frontier represents the set of optimal and feasible solutions for asset allocation   
We learnt how to derive the Minimum Variance Portfolio, and Tangency portfolio   
- Capital Asset Pricing Model (CAPM) is a simplified model to relate an asset and the market   
- A rational investor should allocate on the Capital Market Line (CML)   
- Security Market Line (SML) is a graphical presentation of CAPM   
- Different performance measures are introduced for investment comparison   
- Introduce different stop loss and risk limit control mechanism
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
