请基于以下材料，生成一份 lesson 级 MDX 课本。

目标语言：
zh-CN

lesson_id：
L9

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
# L9: Market practice in strategy optimization, broker selection, and market tricks
Course Code: COMP7415
# Agenda
- Best practice to optimize a trading strategy
- Introduce common optimization methods
- Different type of investment funds
- The winning secret behind top tier hedge funds
Guidelines to choose a trading broker
- Algo deployment on ALGOGENE
- Market tricks by traders and investment firms
# Strategy Optimization
# What is Strategy Optimization?
- Strategy optimization involves fine-tuning trading algorithms to maximize performance metrics like returns, Sharpe ratio, etc.
- Helps in improving trading strategies based on historical data, leading to better decision-making in live trading.
$$
\underline {{y}} = f (\underline {{x}})
$$
# Strategy Optimization Process
# 1. Define the optimization objective
Commonly used performance metric
Max. annualized return
Max. Sharpe ratio
• Min. maximal drawdown
- User defined function (eg. Max. $F = \frac{(Annual\ return)\times(win\ rate)\sqrt{no.of\ trades}}{abs(VaR)}$
Performance Statistics: ①
<table><tr><td>No. of Tradable Days:</td><td>31</td><td>No. of Win Days:</td><td>16</td><td>No. of Loss Days:</td><td>14</td></tr><tr><td>Win Rate:</td><td>53.3333%</td><td>Max. Consecutive Win Day:</td><td>5</td><td>Max. Consecutive Loss Day:</td><td>3</td></tr><tr><td>Odd Ratio:</td><td>1.1429</td><td>Max. Consecutive Gains:</td><td>457.88</td><td>Max. Consecutive Loss:</td><td>-461.27</td></tr><tr><td>No. of Trades:</td><td>2919</td><td>Average Consecutive Win Day:</td><td>0.93</td><td>Average Consecutive Loss Day:</td><td>0.6</td></tr><tr><td>Total PnL:</td><td>-173.62</td><td>Average Per Trade PnL:</td><td>-0.06</td><td>Average Per Day PnL:</td><td>-5.6</td></tr><tr><td>Mean Daily Return:</td><td>-0.0462%</td><td>Median Daily Return:</td><td>0.1058%</td><td>Mean Annual Return:</td><td>-11.6511%</td></tr><tr><td>Daily Return StdDev:</td><td>1.5529%</td><td>25th percentile Daily Return:</td><td>-0.5793%</td><td>75th percentile Daily Return:</td><td>0.5763%</td></tr><tr><td>Daily Return Downside StdDev:</td><td>1.0594%</td><td>95% 1 day return VaR:</td><td>-2.9652%</td><td>99% 1 day return VaR:</td><td>-4.128%</td></tr><tr><td>Daily Sharpe Ratio:</td><td>-0.0298</td><td>Annual Sharpe Ratio:</td><td>-0.4726</td><td>Max. Drawdown Amount:</td><td>995.9</td></tr><tr><td>Daily Sortino Ratio:</td><td>-0.0436</td><td>Annual Sortino Ratio:</td><td>-0.6928</td><td>Max. Drawdown Percent:</td><td>9.2023%</td></tr><tr><td>Max. Drawdown Duration:</td><td>8</td><td>Average Drawdown Duration:</td><td>2.45</td><td>Annual Volatility:</td><td>24.5542%</td></tr><tr><td>Gross Profit:</td><td>0.0</td><td>Gross Loss:</td><td>0.0</td><td>Profit Factor:</td><td>0.0</td></tr><tr><td>Jensen Alpha:</td><td>0.0</td><td>Beta:</td><td>0.0</td><td>Information Ratio:</td><td>0.0</td></tr><tr><td>Omega Ratio:</td><td>0.0</td><td>Treynor Ratio:</td><td>0.0</td><td>Tail Ratio:</td><td>0.8897</td></tr><tr><td>Calmar Ratio:</td><td>1.2661</td><td>Average Holding Day:</td><td>14.4259</td><td>Annual Turnover Rate:</td><td>0.0%</td></tr></table>
# Strategy Optimization Process
# 2. Define the parameters
- The control set of trading parameters (eg. take profit level, stop loss level, etc) in a trading strategy
- Fewer parameters is preferred to reduce the chance of overfitting
# 3. Define optimization method
Common methods
i. Brute force search / Grid search
ii. Gradient descent
iii. Genetic algorithm
Balance between accuracy and computing time
- Some methods may not be appropriate due to violation of assumptions
# 4. Iterative backtesting and find the parameter set that achieve the optimal outcome
# Brute force search
- Brute force search is the most straightforward method that finds the best solution by checking all possible solutions.
- Example: crack the password "12345"
![](images/97ccc5d69c5b306ae7841ad125c8c4fcc9d0f318f8696910deb792d6ac7c8d35.jpg)
import itertools, time
Target password
target_password = "12345"
def brute_force Crack(target):
Start time for performance measurement
start_time = time.time()
Generate all possible combinations of digits from 0 to 9 for length in range(1, len(target) + 1): for attempt in itertools.product('0123456789', repeat=length): # Join the tuple to form the password attempt attempt_password = ".join(attempt) print(f'Trying password: {attempt_password}')
Check if the attempt matches the target password
if attempt_password == target:
end_time = time.time()
print(f'Password cracked: {attempt_password}')
print(f'Time taken: {end_time - start_time:.2f} seconds')
return attempt_password
print('Password not found.') return None
brute_force Crack(target_password)
# Gradient Descent
- Gradient descent is an optimization algorithm used to find the minimum of a function by iteratively moving toward the steepest descent.
- The update rule for gradient descent can be expressed as:
</COVERAGE_CHECKLIST>

<SOURCE_OUTLINE>
# L9: Market practice in strategy optimization, broker selection, and market tricks
Course Code: COMP7415
# Agenda
- Best practice to optimize a trading strategy
- Introduce common optimization methods
- Different type of investment funds
- The winning secret behind top tier hedge funds
Guidelines to choose a trading broker
- Algo deployment on ALGOGENE
- Market tricks by traders and investment firms
# Strategy Optimization
# What is Strategy Optimization?
- Strategy optimization involves fine-tuning trading algorithms to maximize performance metrics like returns, Sharpe ratio, etc.
- Helps in improving trading strategies based on historical data, leading to better decision-making in live trading.
$$
\underline {{y}} = f (\underline {{x}})
$$
# Strategy Optimization Process
# 1. Define the optimization objective
Commonly used performance metric
Max. annualized return
Max. Sharpe ratio
• Min. maximal drawdown
- User defined function (eg. Max. $F = \frac{(Annual\ return)\times(win\ rate)\sqrt{no.of\ trades}}{abs(VaR)}$
Performance Statistics: ①
<table><tr><td>No. of Tradable Days:</td><td>31</td><td>No. of Win Days:</td><td>16</td><td>No. of Loss Days:</td><td>14</td></tr><tr><td>Win Rate:</td><td>53.3333%</td><td>Max. Consecutive Win Day:</td><td>5</td><td>Max. Consecutive Loss Day:</td><td>3</td></tr><tr><td>Odd Ratio:</td><td>1.1429</td><td>Max. Consecutive Gains:</td><td>457.88</td><td>Max. Consecutive Loss:</td><td>-461.27</td></tr><tr><td>No. of Trades:</td><td>2919</td><td>Average Consecutive Win Day:</td><td>0.93</td><td>Average Consecutive Loss Day:</td><td>0.6</td></tr><tr><td>Total PnL:</td><td>-173.62</td><td>Average Per Trade PnL:</td><td>-0.06</td><td>Average Per Day PnL:</td><td>-5.6</td></tr><tr><td>Mean Daily Return:</td><td>-0.0462%</td><td>Median Daily Return:</td><td>0.1058%</td><td>Mean Annual Return:</td><td>-11.6511%</td></tr><tr><td>Daily Return StdDev:</td><td>1.5529%</td><td>25th percentile Daily Return:</td><td>-0.5793%</td><td>75th percentile Daily Return:</td><td>0.5763%</td></tr><tr><td>Daily Return Downside StdDev:</td><td>1.0594%</td><td>95% 1 day return VaR:</td><td>-2.9652%</td><td>99% 1 day return VaR:</td><td>-4.128%</td></tr><tr><td>Daily Sharpe Ratio:</td><td>-0.0298</td><td>Annual Sharpe Ratio:</td><td>-0.4726</td><td>Max. Drawdown Amount:</td><td>995.9</td></tr><tr><td>Daily Sortino Ratio:</td><td>-0.0436</td><td>Annual Sortino Ratio:</td><td>-0.6928</td><td>Max. Drawdown Percent:</td><td>9.2023%</td></tr><tr><td>Max. Drawdown Duration:</td><td>8</td><td>Average Drawdown Duration:</td><td>2.45</td><td>Annual Volatility:</td><td>24.5542%</td></tr><tr><td>Gross Profit:</td><td>0.0</td><td>Gross Loss:</td><td>0.0</td><td>Profit Factor:</td><td>0.0</td></tr><tr><td>Jensen Alpha:</td><td>0.0</td><td>Beta:</td><td>0.0</td><td>Information Ratio:</td><td>0.0</td></tr><tr><td>Omega Ratio:</td><td>0.0</td><td>Treynor Ratio:</td><td>0.0</td><td>Tail Ratio:</td><td>0.8897</td></tr><tr><td>Calmar Ratio:</td><td>1.2661</td><td>Average Holding Day:</td><td>14.4259</td><td>Annual Turnover Rate:</td><td>0.0%</td></tr></table>
# Strategy Optimization Process
# 2. Define the parameters
- The control set of trading parameters (eg. take profit level, stop loss level, etc) in a trading strategy
- Fewer parameters is preferred to reduce the chance of overfitting
# 3. Define optimization method
Common methods
i. Brute force search / Grid search
ii. Gradient descent
iii. Genetic algorithm
Balance between accuracy and computing time
- Some methods may not be appropriate due to violation of assumptions
# 4. Iterative backtesting and find the parameter set that achieve the optimal outcome
# Brute force search
- Brute force search is the most straightforward method that finds the best solution by checking all possible solutions.
- Example: crack the password "12345"
![](images/97ccc5d69c5b306ae7841ad125c8c4fcc9d0f318f8696910deb792d6ac7c8d35.jpg)
import itertools, time
Target password
target_password = "12345"
def brute_force Crack(target):
Start time for performance measurement
start_time = time.time()
Generate all possible combinations of digits from 0 to 9 for length in range(1, len(target) + 1): for attempt in itertools.product('0123456789', repeat=length): # Join the tuple to form the password attempt attempt_password = ".join(attempt) print(f'Trying password: {attempt_password}')
Check if the attempt matches the target password
if attempt_password == target:
end_time = time.time()
print(f'Password cracked: {attempt_password}')
print(f'Time taken: {end_time - start_time:.2f} seconds')
return attempt_password
print('Password not found.') return None
brute_force Crack(target_password)
# Gradient Descent
- Gradient descent is an optimization algorithm used to find the minimum of a function by iteratively moving toward the steepest descent.
- The update rule for gradient descent can be expressed as:
</SOURCE_OUTLINE>

<LESSON_MAP>
{
  "lesson_id": "L9",
  "mode": "guided_story",
  "steps": [
    {
      "concept": "策略优化的目标与流程",
      "file": "research/pipeline/3-guided_story/L9/step1/step.json",
      "sequence_id": "step1"
    },
    {
      "concept": "三种优化方法：暴力搜索、梯度下降、遗传算法",
      "file": "research/pipeline/3-guided_story/L9/step2/step.json",
      "sequence_id": "step2"
    },
    {
      "concept": "最优解 vs 稳健解：为什么稳健更重要",
      "file": "research/pipeline/3-guided_story/L9/step3/step.json",
      "sequence_id": "step3"
    },
    {
      "concept": "投资基金的分类与对冲基金的秘密",
      "file": "research/pipeline/3-guided_story/L9/step4/step.json",
      "sequence_id": "step4"
    },
    {
      "concept": "如何选择交易券商",
      "file": "research/pipeline/3-guided_story/L9/step5/step.json",
      "sequence_id": "step5"
    },
    {
      "concept": "警惕市场中的销售伎俩",
      "file": "research/pipeline/3-guided_story/L9/step6/step.json",
      "sequence_id": "step6"
    }
  ]
}

</LESSON_MAP>

<GUIDED_STORY_MANIFEST>
{
  "lesson_id": "L9",
  "mode": "guided_story",
  "steps": [
    {
      "concept": "策略优化的目标与流程",
      "file": "research/pipeline/3-guided_story/L9/step1/step.json",
      "sequence_id": "step1"
    },
    {
      "concept": "三种优化方法：暴力搜索、梯度下降、遗传算法",
      "file": "research/pipeline/3-guided_story/L9/step2/step.json",
      "sequence_id": "step2"
    },
    {
      "concept": "最优解 vs 稳健解：为什么稳健更重要",
      "file": "research/pipeline/3-guided_story/L9/step3/step.json",
      "sequence_id": "step3"
    },
    {
      "concept": "投资基金的分类与对冲基金的秘密",
      "file": "research/pipeline/3-guided_story/L9/step4/step.json",
      "sequence_id": "step4"
    },
    {
      "concept": "如何选择交易券商",
      "file": "research/pipeline/3-guided_story/L9/step5/step.json",
      "sequence_id": "step5"
    },
    {
      "concept": "警惕市场中的销售伎俩",
      "file": "research/pipeline/3-guided_story/L9/step6/step.json",
      "sequence_id": "step6"
    }
  ]
}

</GUIDED_STORY_MANIFEST>

<GUIDED_STORY_STEPS>
[
  {
    "lesson_id": "L9",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s001",
        "introduced_terms": [],
        "lines": [
          "一个策略在回测里表现很好，",
          "但一到实盘就失效，问题出在哪？"
        ],
        "type": "narration"
      },
      {
        "id": "s002",
        "introduced_terms": [
          "strategy_optimization"
        ],
        "lines": [
          "很可能是因为参数没有经过优化。",
          "**<term id=\"strategy_optimization\">策略优化</term>** 就是调整参数的过程。"
        ],
        "type": "narration"
      },
      {
        "id": "s003",
        "introduced_terms": [],
        "lines": [
          "目标是让策略在历史数据上表现更好，",
          "比如提高总收益或降低风险。"
        ],
        "type": "narration"
      },
      {
        "id": "s004",
        "introduced_terms": [
          "sharpe_ratio",
          "max_drawdown"
        ],
        "lines": [
          "常见的优化目标有：",
          "最大化年化收益、最大化 <term id=\"sharpe_ratio\">夏普比率</term>、最小化 <term id=\"max_drawdown\">最大回撤</term>。"
        ],
        "type": "narration"
      },
      {
        "id": "s005",
        "introduced_terms": [],
        "lines": [
          "也可以自定义目标函数，",
          "比如把胜率、交易次数和风险都考虑进去。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "首先要明确你想优化什么，比如最大化收益还是最小化回撤，这就是定义优化目标。",
          "kind": "single_choice",
          "options": [
            "运行回测",
            "定义优化目标",
            "选择参数范围",
            "部署到实盘"
          ],
          "prompt": "策略优化的第一步是什么？"
        },
        "id": "s006",
        "lines": [
          "策略优化的第一步是什么？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step1",
    "source": {
      "plain_text": "Strategy optimization involves fine-tuning trading algorithms to maximize performance metrics like returns, Sharpe ratio, etc. Helps in improving trading strategies based on historical data, leading to better decision-making in live trading.",
      "related": [
        "COMP7415 L9"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "max_drawdown": {
        "aliases": [
          "Max Drawdown"
        ],
        "display": "最大回撤",
        "gloss": "从峰值到谷底的最大亏损幅度。"
      },
      "sharpe_ratio": {
        "aliases": [
          "Sharpe Ratio"
        ],
        "display": "夏普比率",
        "gloss": "衡量每单位风险能获得多少超额回报的指标。"
      },
      "strategy_optimization": {
        "aliases": [
          "Strategy Optimization"
        ],
        "display": "策略优化",
        "gloss": "调整交易算法的参数，以最大化收益、夏普比率等绩效指标的过程。"
      }
    }
  },
  {
    "lesson_id": "L9",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s007",
        "introduced_terms": [],
        "lines": [
          "确定了优化目标后，下一步是选择优化方法。",
          "有三种常见的方法。"
        ],
        "type": "narration"
      },
      {
        "id": "s008",
        "introduced_terms": [
          "brute_force_search"
        ],
        "lines": [
          "第一种：<term id=\"brute_force_search\">暴力搜索</term>。",
          "它最简单直接——把所有可能的参数组合都试一遍。"
        ],
        "type": "narration"
      },
      {
        "id": "s009",
        "introduced_terms": [],
        "lines": [
          "就像破解密码“12345”，",
          "从“0”开始一个一个试，直到找到正确的那个。"
        ],
        "type": "narration"
      },
      {
        "id": "s010",
        "introduced_terms": [],
        "lines": [
          "暴力搜索很可靠，但参数一多，计算量会爆炸式增长。"
        ],
        "type": "narration"
      },
      {
        "id": "s011",
        "introduced_terms": [
          "gradient_descent"
        ],
        "lines": [
          "第二种：<term id=\"gradient_descent\">梯度下降</term>。",
          "它像一个下山的人，每次都沿着最陡的下坡路走一步。"
        ],
        "type": "narration"
      },
      {
        "formula": {
          "latex": "x_{n+1} = x_n - \\alpha \\cdot \\nabla f(x_n)",
          "spoken": "下一个参数值等于当前值减去学习率乘以梯度。"
        },
        "id": "s012",
        "introduced_terms": [],
        "lines": [
          "更新规则是：",
          "$x_{n+1} = x_n - \\alpha \\cdot \\nabla f(x_n)$"
        ],
        "type": "formula"
      },
      {
        "id": "s013",
        "introduced_terms": [],
        "lines": [
          "其中 $\\alpha$ 是学习率，控制每一步的大小。",
          "$\\nabla f(x_n)$ 是梯度，指向函数上升最快的方向。"
        ],
        "type": "narration"
      },
      {
        "id": "s014",
        "introduced_terms": [],
        "lines": [
          "梯度下降的缺点是：",
          "对初始值和学习率很敏感，而且可能只找到局部最优解。"
        ],
        "type": "narration"
      },
      {
        "id": "s015",
        "introduced_terms": [
          "genetic_algorithm"
        ],
        "lines": [
          "第三种：<term id=\"genetic_algorithm\">遗传算法</term>。",
          "它模拟了“物竞天择，适者生存”的进化过程。"
        ],
        "type": "narration"
      },
      {
        "id": "s016",
        "introduced_terms": [],
        "lines": [
          "先随机生成一批“参数个体”，",
          "然后通过选择、交叉、变异，一代代进化出更好的解。"
        ],
        "type": "narration"
      },
      {
        "id": "s017",
        "introduced_terms": [],
        "lines": [
          "遗传算法擅长探索广阔的解空间，",
          "但同样不保证找到全局最优解。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 0,
          "explanation": "当参数组合很少时，暴力搜索可以遍历所有可能，保证找到全局最优解。",
          "kind": "single_choice",
          "options": [
            "暴力搜索",
            "梯度下降",
            "遗传算法",
            "随机猜测"
          ],
          "prompt": "以下哪种方法最适合参数很少、且需要精确最优解的场景？"
        },
        "id": "s018",
        "lines": [
          "以下哪种方法最适合参数很少、且需要精确最优解的场景？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step2",
    "source": {
      "plain_text": "Common methods: i. Brute force search / Grid search ii. Gradient descent iii. Genetic algorithm",
      "related": [
        "COMP7415 L9"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "brute_force_search": {
        "aliases": [
          "Brute Force Search",
          "Grid Search"
        ],
        "display": "暴力搜索",
        "gloss": "遍历所有可能的参数组合来寻找最优解。"
      },
      "genetic_algorithm": {
        "aliases": [
          "Genetic Algorithm"
        ],
        "display": "遗传算法",
        "gloss": "模拟自然选择和生物进化过程的搜索启发式算法。"
      },
      "gradient_descent": {
        "aliases": [
          "Gradient Descent"
        ],
        "display": "梯度下降",
        "gloss": "通过沿着函数梯度的反方向迭代更新，来寻找函数最小值。"
      }
    }
  },
  {
    "lesson_id": "L9",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s019",
        "introduced_terms": [],
        "lines": [
          "优化完成后，你会得到一组“最优参数”。",
          "但最优解真的是最好的选择吗？"
        ],
        "type": "narration"
      },
      {
        "id": "s020",
        "introduced_terms": [
          "overfitting"
        ],
        "lines": [
          "最优解往往在历史数据上表现完美，",
          "但很可能已经 <term id=\"overfitting\">过拟合</term> 了。"
        ],
        "type": "narration"
      },
      {
        "id": "s021",
        "introduced_terms": [],
        "lines": [
          "过拟合的策略一到实盘就会失效，",
          "因为它学到的只是历史噪音，而不是真正的规律。"
        ],
        "type": "narration"
      },
      {
        "id": "s022",
        "introduced_terms": [
          "robust_solution"
        ],
        "lines": [
          "相比之下，<term id=\"robust_solution\">稳健解</term> 虽然回测收益不是最高，",
          "但在不同市场环境下都能稳定表现。"
        ],
        "type": "narration"
      },
      {
        "id": "s023",
        "introduced_terms": [],
        "lines": [
          "选择稳健策略的关键是：",
          "进行参数敏感性分析和滚动窗口验证。"
        ],
        "type": "narration"
      },
      {
        "id": "s024",
        "introduced_terms": [],
        "lines": [
          "比如，把5年数据分成5份，",
          "看看哪组参数在每一年都表现不错，而不是只在某一年特别好。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "explanation": "因为最优解容易过拟合，在实盘新数据上表现不佳；而稳健解在不同市场条件下表现更稳定，风险更低。",
          "kind": "short_reflection",
          "prompt": "为什么在实盘交易中，稳健解通常比最优解更受青睐？"
        },
        "id": "s025",
        "lines": [
          "为什么在实盘交易中，稳健解通常比最优解更受青睐？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step3",
    "source": {
      "plain_text": "Optimal vs Robust Solution. Optimal Strategy: Good performance in backtesting but likely fails in live conditions. Robust Strategy: Moderate performance but performs steadily across market conditions.",
      "related": [
        "COMP7415 L9"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "overfitting": {
        "aliases": [
          "Overfitting"
        ],
        "display": "过拟合",
        "gloss": "模型在训练数据上表现很好，但在新数据上表现很差的现象。"
      },
      "robust_solution": {
        "aliases": [
          "Robust Solution"
        ],
        "display": "稳健解",
        "gloss": "在不同市场环境下都能保持稳定表现的参数组合。"
      }
    }
  },
  {
    "lesson_id": "L9",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s026",
        "introduced_terms": [],
        "lines": [
          "了解市场中的各类基金，有助于你理解资金是如何运作的。",
          "主要有五种类型。"
        ],
        "type": "narration"
      },
      {
        "id": "s027",
        "introduced_terms": [
          "etf"
        ],
        "lines": [
          "<term id=\"etf\">交易所交易基金</term> 像股票一样在交易所买卖，",
          "通常追踪某个指数，费用低，流动性好。"
        ],
        "type": "narration"
      },
      {
        "id": "s028",
        "introduced_terms": [
          "mutual_fund"
        ],
        "lines": [
          "<term id=\"mutual_fund\">共同基金</term> 由专业经理主动管理，",
          "每天按收盘净值交易一次。"
        ],
        "type": "narration"
      },
      {
        "id": "s029",
        "introduced_terms": [],
        "lines": [
          "货币市场基金风险低，投资短期债券。",
          "私募股权基金流动性差，投资期长，但潜在回报高。"
        ],
        "type": "narration"
      },
      {
        "id": "s030",
        "introduced_terms": [
          "hedge_fund"
        ],
        "lines": [
          "最后是 <term id=\"hedge_fund\">对冲基金</term>，",
          "它们使用杠杆、做空、衍生品等复杂策略追求高回报。"
        ],
        "type": "narration"
      },
      {
        "id": "s031",
        "introduced_terms": [
          "uncorrelated_strategies"
        ],
        "lines": [
          "顶级对冲基金（如文艺复兴科技）的秘密是什么？",
          "它们同时运行成百上千个 <term id=\"uncorrelated_strategies\">不相关策略</term>。"
        ],
        "type": "narration"
      },
      {
        "id": "s032",
        "introduced_terms": [],
        "lines": [
          "每个策略的胜率可能只比50%高一点，",
          "但因为策略之间不相关，整体收益会非常稳定。"
        ],
        "type": "narration"
      },
      {
        "id": "s033",
        "introduced_terms": [],
        "lines": [
          "这就是大数定律的力量：",
          "单个策略有波动，但组合起来就能平滑收益曲线。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "大数定律指出，当独立试验次数足够多时，平均结果会趋近于期望值。不相关策略的组合正是利用了这一点。",
          "kind": "single_choice",
          "options": [
            "复利效应",
            "大数定律",
            "均值回归",
            "有效市场假说"
          ],
          "prompt": "对冲基金通过运行大量不相关策略来获得稳定收益，这主要利用了哪个原理？"
        },
        "id": "s034",
        "lines": [
          "对冲基金通过运行大量不相关策略来获得稳定收益，这主要利用了哪个原理？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step4",
    "source": {
      "plain_text": "Different Type of Investment Funds: ETFs, Mutual Funds, Money Market Funds, Private Equity Funds, Hedge Funds. Hedge funds utilize multiple uncorrelated strategies to diversify risk.",
      "related": [
        "COMP7415 L9"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "etf": {
        "aliases": [
          "ETF",
          "Exchange-Traded Fund"
        ],
        "display": "交易所交易基金",
        "gloss": "在交易所上市交易、通常追踪某个指数的基金。"
      },
      "hedge_fund": {
        "aliases": [
          "Hedge Fund"
        ],
        "display": "对冲基金",
        "gloss": "采用多种高级量化策略，追求高回报的私募投资基金。"
      },
      "mutual_fund": {
        "aliases": [
          "Mutual Fund"
        ],
        "display": "共同基金",
        "gloss": "汇集投资者资金，由专业基金经理管理的投资组合。"
      },
      "uncorrelated_strategies": {
        "aliases": [
          "Uncorrelated Strategies"
        ],
        "display": "不相关策略",
        "gloss": "彼此之间没有明显关联、不会同时涨跌的交易策略。"
      }
    }
  },
  {
    "lesson_id": "L9",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s035",
        "introduced_terms": [],
        "lines": [
          "选对券商，直接关系到策略的盈利和资金安全。",
          "需要考虑六个关键因素。"
        ],
        "type": "narration"
      },
      {
        "id": "s036",
        "introduced_terms": [],
        "lines": [
          "第一：交易成本。",
          "佣金、点差、出入金费用，高频交易下这些成本会大幅侵蚀利润。"
        ],
        "type": "narration"
      },
      {
        "id": "s037",
        "introduced_terms": [],
        "lines": [
          "第二：市场准入。",
          "券商支持哪些品种？股票、外汇、加密货币？杠杆和保证金政策如何？"
        ],
        "type": "narration"
      },
      {
        "id": "s038",
        "introduced_terms": [],
        "lines": [
          "第三：交易工具。",
          "API接口是否完善？有没有实时数据、回测平台和模拟账户？"
        ],
        "type": "narration"
      },
      {
        "id": "s039",
        "introduced_terms": [],
        "lines": [
          "第四：监管合规。",
          "确保券商受权威机构监管，比如香港的SFC、美国的SEC/FINRA。"
        ],
        "type": "narration"
      },
      {
        "id": "s040",
        "introduced_terms": [],
        "lines": [
          "第五：客户支持。",
          "遇到问题时能否及时获得帮助？有没有丰富的学习资源？"
        ],
        "type": "narration"
      },
      {
        "id": "s041",
        "introduced_terms": [],
        "lines": [
          "第六：可靠性与声誉。",
          "查看用户评价、公司历史，上市公司通常更可靠。"
        ],
        "type": "narration"
      },
      {
        "id": "s042",
        "introduced_terms": [],
        "lines": [
          "FTX的案例是一个惨痛的教训。",
          "即使是大交易所，也可能因为挪用客户资金而破产。"
        ],
        "type": "narration"
      },
      {
        "id": "s043",
        "introduced_terms": [
          "counterparty_risk"
        ],
        "lines": [
          "这就是 <term id=\"counterparty_risk\">对手方风险</term>：",
          "你赚钱了，但券商出了问题，你可能一分钱也取不出来。"
        ],
        "type": "narration"
      },
      {
        "id": "s044",
        "introduced_terms": [
          "proof_of_reserves"
        ],
        "lines": [
          "为了应对这种风险，一些平台会提供 <term id=\"proof_of_reserves\">储备金证明</term>。",
          "它用默克尔树技术，让用户能验证平台是否真的持有足够的资产。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "监管合规是资金安全的基础，确保券商在合法框架下运营，降低对手方风险。",
          "kind": "single_choice",
          "options": [
            "交易界面是否美观",
            "是否受权威机构监管",
            "开户赠金多少",
            "App下载量"
          ],
          "prompt": "以下哪个是选择券商时最重要的安全考量？"
        },
        "id": "s045",
        "lines": [
          "以下哪个是选择券商时最重要的安全考量？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step5",
    "source": {
      "plain_text": "Key Considerations: 1. Trading costs 2. Access to markets 3. Trading platform features 4. Regulatory compliance 5. Customer support 6. Reliability and reputation",
      "related": [
        "COMP7415 L9"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "counterparty_risk": {
        "aliases": [
          "Counterparty Risk"
        ],
        "display": "对手方风险",
        "gloss": "交易对手方可能违约，无法履行义务的风险。"
      },
      "proof_of_reserves": {
        "aliases": [
          "Proof-of-Reserves",
          "PoR"
        ],
        "display": "储备金证明",
        "gloss": "金融机构用来证明其持有足够资产来覆盖客户负债的方法。"
      }
    }
  },
  {
    "lesson_id": "L9",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s046",
        "introduced_terms": [],
        "lines": [
          "市场上有些机构会故意美化业绩来吸引投资。",
          "你需要学会识别这些伎俩。"
        ],
        "type": "narration"
      },
      {
        "id": "s047",
        "introduced_terms": [],
        "lines": [
          "第一个伎俩：误导性的胜率定义。",
          "有的机构把“一笔盈利的交易”算作100%胜率，即使这笔交易之前亏了4天。"
        ],
        "type": "narration"
      },
      {
        "id": "s048",
        "introduced_terms": [],
        "lines": [
          "更合理的定义是按交易日计算：",
          "5天里有1天赚钱，胜率是20%，而不是100%。"
        ],
        "type": "narration"
      },
      {
        "id": "s049",
        "introduced_terms": [
          "sharpe_ratio_manipulation"
        ],
        "lines": [
          "第二个伎俩：<term id=\"sharpe_ratio_manipulation\">夏普比率操纵</term>。",
          "用日数据、周数据还是月数据计算，结果可能差很多。"
        ],
        "type": "narration"
      },
      {
        "id": "s050",
        "introduced_terms": [],
        "lines": [
          "机构会选择对自己最有利的频率来展示。",
          "所以比较不同产品时，要确保计算方式一致。"
        ],
        "type": "narration"
      },
      {
        "id": "s051",
        "introduced_terms": [
          "fomo"
        ],
        "lines": [
          "第三个伎俩：利用 <term id=\"fomo\">错失恐惧症</term>。",
          "“限时优惠”、“仅剩最后名额”、“别人都赚了30%”……"
        ],
        "type": "narration"
      },
      {
        "id": "s052",
        "introduced_terms": [],
        "lines": [
          "这些都是在利用你的情绪，让你不做理性分析就匆忙投资。"
        ],
        "type": "narration"
      },
      {
        "id": "s053",
        "introduced_terms": [],
        "lines": [
          "记住：永远看透表面指标，",
          "关注完整的上下文、时间框架和风险度量。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "使用一个看似很高但定义有问题的指标（胜率）来掩盖整体不盈利的事实，属于误导性指标。",
          "kind": "single_choice",
          "options": [
            "选择性呈现数据",
            "误导性指标",
            "心理操纵",
            "以上都是"
          ],
          "prompt": "一家基金宣称其策略胜率高达95%，但未披露平均亏损远大于平均盈利。这属于哪种伎俩？"
        },
        "id": "s054",
        "lines": [
          "一家基金宣称其策略胜率高达95%，但未披露平均亏损远大于平均盈利。这属于哪种伎俩？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step6",
    "source": {
      "plain_text": "Misleading metrics, Selective presentation of data, Psychological manipulation. Examples: Winning rate definition, Sharpe ratio calculation, FOMO.",
      "related": [
        "COMP7415 L9"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "fomo": {
        "aliases": [
          "FOMO",
          "Fear of Missing Out"
        ],
        "display": "错失恐惧症",
        "gloss": "担心错过潜在收益而做出冲动投资决策的心理现象。"
      },
      "sharpe_ratio_manipulation": {
        "aliases": [
          "Sharpe Ratio Manipulation"
        ],
        "display": "夏普比率操纵",
        "gloss": "通过选择不同的时间频率或数据窗口，来呈现一个更高的夏普比率。"
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
      "coverage_tag": "broker_selection_overview",
      "covered_by": [
        "qf_flash_broker_overview",
        "qf_quiz_broker_overview"
      ],
      "description": "选择券商的重要性及六个关键考量因素的整体框架"
    },
    {
      "coverage_tag": "trading_costs",
      "covered_by": [
        "qf_flash_trading_costs",
        "qf_quiz_trading_costs"
      ],
      "description": "交易成本：佣金、点差、出入金费用对高频交易利润的影响"
    },
    {
      "coverage_tag": "market_access",
      "covered_by": [
        "qf_flash_market_access",
        "qf_quiz_market_access"
      ],
      "description": "市场准入：支持的品种、杠杆和保证金政策"
    },
    {
      "coverage_tag": "trading_tools",
      "covered_by": [
        "qf_flash_trading_tools",
        "qf_quiz_trading_tools"
      ],
      "description": "交易工具：API接口、实时数据、回测平台和模拟账户"
    },
    {
      "coverage_tag": "regulatory_compliance",
      "covered_by": [
        "qf_flash_regulatory_compliance",
        "qf_quiz_regulatory_compliance"
      ],
      "description": "监管合规：SFC、SEC/FINRA等权威机构监管的重要性"
    },
    {
      "coverage_tag": "customer_support",
      "covered_by": [
        "qf_flash_customer_support",
        "qf_quiz_customer_support"
      ],
      "description": "客户支持：及时帮助和学习资源"
    },
    {
      "coverage_tag": "reliability_reputation",
      "covered_by": [
        "qf_flash_reliability_reputation",
        "qf_quiz_reliability_reputation"
      ],
      "description": "可靠性与声誉：用户评价、公司历史、上市公司更可靠"
    },
    {
      "coverage_tag": "counterparty_risk",
      "covered_by": [
        "qf_flash_counterparty_risk",
        "qf_quiz_counterparty_risk",
        "qf_long_counterparty_risk"
      ],
      "description": "对手方风险：券商问题导致资金无法提取的风险"
    },
    {
      "coverage_tag": "proof_of_reserves",
      "covered_by": [
        "qf_flash_proof_of_reserves",
        "qf_quiz_proof_of_reserves",
        "qf_long_proof_of_reserves"
      ],
      "description": "储备金证明：用默克尔树技术验证平台资产"
    },
    {
      "coverage_tag": "ftx_case_study",
      "covered_by": [
        "qf_flash_ftx_case",
        "qf_quiz_ftx_case"
      ],
      "description": "FTX案例：挪用客户资金导致破产的教训"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "broker_selection_overview",
      "coverage_tags": [
        "broker_selection_overview"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_broker_overview",
      "learning_goal": "学生能列举选择券商时需要考虑的六个关键因素。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "六个关键因素的名称",
      "term_refs": [
        {
          "display": "券商选择",
          "en": "Broker Selection"
        }
      ],
      "variants": [
        {
          "back": "交易成本、市场准入",
          "estimated_seconds": 10,
          "explanation": "交易成本包括佣金、点差等；市场准入指券商支持的交易品种和杠杆政策。",
          "front": "选择券商时需要考虑的六个关键因素中，前两个是什么？",
          "question_id": "q_flash_broker_overview_v1"
        },
        {
          "back": "客户支持、可靠性与声誉",
          "estimated_seconds": 10,
          "explanation": "客户支持指能否及时获得帮助；可靠性与声誉包括用户评价和公司历史。",
          "front": "选择券商时需要考虑的六个关键因素中，最后两个是什么？",
          "question_id": "q_flash_broker_overview_v2"
        },
        {
          "back": "交易工具、监管合规",
          "estimated_seconds": 10,
          "explanation": "交易工具包括API接口和回测平台；监管合规指券商是否受权威机构监管。",
          "front": "选择券商时需要考虑的六个关键因素中，中间两个是什么？",
          "question_id": "q_flash_broker_overview_v3"
        }
      ]
    },
    {
      "concept_key": "trading_costs",
      "coverage_tags": [
        "trading_costs"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_trading_costs",
      "learning_goal": "学生能识别交易成本的构成及其对高频交易的影响。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "交易成本的三种主要形式",
      "term_refs": [
        {
          "display": "交易成本",
          "en": "Trading Costs"
        },
        {
          "display": "佣金",
          "en": "Commission"
        },
        {
          "display": "点差",
          "en": "Spread"
        }
      ],
      "variants": [
        {
          "back": "佣金是每笔交易的手续费；点差是买卖价差。",
          "estimated_seconds": 8,
          "explanation": "佣金是券商收取的固定或按比例的费用；点差是买入价和卖出价之间的差额。",
          "front": "交易成本中的“佣金”和“点差”分别指什么？",
          "question_id": "q_flash_trading_costs_v1"
        },
        {
          "back": "因为高频交易次数多，佣金、点差和出入金费用累积起来非常可观。",
          "estimated_seconds": 10,
          "explanation": "每笔交易的成本虽然小，但高频交易成千上万笔，总成本会显著降低净利润。",
          "front": "为什么高频交易下交易成本会大幅侵蚀利润？",
          "question_id": "q_flash_trading_costs_v2"
        }
      ]
    },
    {
      "concept_key": "market_access",
      "coverage_tags": [
        "market_access"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_market_access",
      "learning_goal": "学生能说明市场准入包含哪些要素。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "市场准入的三个关键方面",
      "term_refs": [
        {
          "display": "市场准入",
          "en": "Access to Markets"
        },
        {
          "display": "杠杆",
          "en": "Leverage"
        }
      ],
      "variants": [
        {
          "back": "支持的交易品种、杠杆政策、保证金政策。",
          "estimated_seconds": 8,
          "explanation": "例如股票、外汇、加密货币等品种，以及杠杆倍数和保证金要求。",
          "front": "市场准入主要考察券商的哪三个方面？",
          "question_id": "q_flash_market_access_v1"
        },
        {
          "back": "券商是否支持该品种的做空交易以及相关的保证金要求。",
          "estimated_seconds": 8,
          "explanation": "不是所有券商都支持所有品种的做空，需要提前确认。",
          "front": "如果策略需要做空某个品种，市场准入方面需要确认什么？",
          "question_id": "q_flash_market_access_v2"
        }
      ]
    },
    {
      "concept_key": "trading_tools",
      "coverage_tags": [
        "trading_tools"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_trading_tools",
      "learning_goal": "学生能列举交易工具的主要功能。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "交易工具的三个关键功能",
      "term_refs": [
        {
          "display": "交易工具",
          "en": "Trading Tools"
        },
        {
          "display": "API接口",
          "en": "API"
        }
      ],
      "variants": [
        {
          "back": "API接口用于程序化交易；实时数据用于决策；模拟账户用于测试。",
          "estimated_seconds": 10,
          "explanation": "API接口允许算法自动下单；实时数据提供最新行情；模拟账户可以在不投入真金白银的情况下测试策略。",
          "front": "交易工具中，API接口、实时数据和模拟账户分别有什么用？",
          "question_id": "q_flash_trading_tools_v1"
        },
        {
          "back": "API接口的完善程度。",
          "estimated_seconds": 8,
          "explanation": "因为量化交易依赖程序自动执行，API接口的质量直接决定了策略能否顺利部署。",
          "front": "对于量化交易者，交易工具中最重要的是哪一项？",
          "question_id": "q_flash_trading_tools_v2"
        }
      ]
    },
    {
      "concept_key": "regulatory_compliance",
      "coverage_tags": [
        "regulatory_compliance"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_regulatory_compliance",
      "learning_goal": "学生能说出几个主要的金融监管机构。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "三个主要监管机构及其所属地区",
      "term_refs": [
        {
          "display": "监管合规",
          "en": "Regulatory Compliance"
        },
        {
          "display": "SFC",
          "en": "Securities and Futures Commission"
        },
        {
          "display": "SEC",
          "en": "Securities and Exchange Commission"
        }
      ],
      "variants": [
        {
          "back": "香港SFC、美国SEC/FINRA、英国FCA。",
          "estimated_seconds": 10,
          "explanation": "SFC是香港证监会，SEC是美国证券交易委员会，FINRA是美国金融业监管局，FCA是英国金融行为监管局。",
          "front": "香港、美国和英国的金融监管机构分别是什么？",
          "question_id": "q_flash_regulatory_compliance_v1"
        },
        {
          "back": "因为受监管的券商必须在合法框架下运营，降低对手方风险。",
          "estimated_seconds": 10,
          "explanation": "监管机构会监督券商的资金隔离、财务披露等，保护投资者资金安全。",
          "front": "为什么监管合规是选择券商时最重要的安全考量？",
          "question_id": "q_flash_regulatory_compliance_v2"
        }
      ]
    },
    {
      "concept_key": "customer_support",
      "coverage_tags": [
        "customer_support"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_customer_support",
      "learning_goal": "学生能说明客户支持的两个主要方面。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "客户支持的两个关键维度",
      "term_refs": [
        {
          "display": "客户支持",
          "en": "Customer Support"
        }
      ],
      "variants": [
        {
          "back": "支持的及时性和学习资源的丰富程度。",
          "estimated_seconds": 8,
          "explanation": "24/7支持能快速解决问题；丰富的学习资源有助于提升交易技能。",
          "front": "客户支持主要考察券商的哪两个方面？",
          "question_id": "q_flash_customer_support_v1"
        },
        {
          "back": "因为遇到API或账户问题时需要及时解决，否则可能影响策略运行。",
          "estimated_seconds": 8,
          "explanation": "量化交易依赖自动化系统，一旦出现问题需要快速响应，否则可能导致损失。",
          "front": "为什么客户支持对量化交易者很重要？",
          "question_id": "q_flash_customer_support_v2"
        }
      ]
    },
    {
      "concept_key": "reliability_reputation",
      "coverage_tags": [
        "reliability_reputation"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_reliability_reputation",
      "learning_goal": "学生能说出评估券商可靠性的三个方法。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "评估可靠性的三个途径",
      "term_refs": [
        {
          "display": "可靠性与声誉",
          "en": "Reliability and Reputation"
        }
      ],
      "variants": [
        {
          "back": "查看用户评价、了解公司历史、选择上市公司。",
          "estimated_seconds": 10,
          "explanation": "用户评价反映真实体验；公司历史长的更稳定；上市公司需要定期披露财务信息，更透明。",
          "front": "评估券商可靠性与声誉的三个方法是什么？",
          "question_id": "q_flash_reliability_reputation_v1"
        },
        {
          "back": "因为上市公司有责任定期披露业务和财务状况。",
          "estimated_seconds": 8,
          "explanation": "上市公司的财务和运营信息更透明，受到更严格的监管。",
          "front": "为什么上市公司通常更可靠？",
          "question_id": "q_flash_reliability_reputation_v2"
        }
      ]
    },
    {
      "concept_key": "counterparty_risk",
      "coverage_tags": [
        "counterparty_risk"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_counterparty_risk",
      "learning_goal": "学生能定义对手方风险并举例。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "对手方风险的定义和后果",
      "term_refs": [
        {
          "display": "对手方风险",
          "en": "Counterparty Risk"
        }
      ],
      "variants": [
        {
          "back": "交易对手方可能违约，无法履行义务的风险。",
          "estimated_seconds": 8,
          "explanation": "例如你赚钱了，但券商出了问题，你可能一分钱也取不出来。",
          "front": "什么是对手方风险？",
          "question_id": "q_flash_counterparty_risk_v1"
        },
        {
          "back": "对手方风险。",
          "estimated_seconds": 8,
          "explanation": "FTX挪用客户资金导致破产，客户无法取回资金，这就是典型的对手方风险。",
          "front": "FTX案例中暴露了哪种风险？",
          "question_id": "q_flash_counterparty_risk_v2"
        }
      ]
    },
    {
      "concept_key": "proof_of_reserves",
      "coverage_tags": [
        "proof_of_reserves"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_proof_of_reserves",
      "learning_goal": "学生能说明储备金证明的目的和技术基础。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "储备金证明的目的和核心技术",
      "term_refs": [
        {
          "display": "储备金证明",
          "en": "Proof-of-Reserves"
        },
        {
          "display": "默克尔树",
          "en": "Merkle Tree"
        }
      ],
      "variants": [
        {
          "back": "证明平台持有足够资产来覆盖客户负债。",
          "estimated_seconds": 8,
          "explanation": "通过储备金证明，用户可以验证平台是否真的持有足够的资产，增强信任和透明度。",
          "front": "储备金证明的目的是什么？",
          "question_id": "q_flash_proof_of_reserves_v1"
        },
        {
          "back": "默克尔树（Merkle Tree）。",
          "estimated_seconds": 8,
          "explanation": "默克尔树是一种数据结构，允许高效验证大量数据的完整性，同时保护用户隐私。",
          "front": "储备金证明通常使用什么技术？",
          "question_id": "q_flash_proof_of_reserves_v2"
        }
      ]
    },
    {
      "concept_key": "ftx_case_study",
      "coverage_tags": [
        "ftx_case_study"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_ftx_case",
      "learning_goal": "学生能概括FTX事件的核心教训。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "FTX事件的三个教训",
      "term_refs": [
        {
          "display": "FTX",
          "en": "FTX"
        }
      ],
      "variants": [
        {
          "back": "尽职调查、重视监管、分散风险。",
          "estimated_seconds": 10,
          "explanation": "要研究券商声誉和合规性；选择受监管的券商；不要把资金都放在一个券商。",
          "front": "FTX事件给投资者带来的三个教训是什么？",
          "question_id": "q_flash_ftx_case_v1"
        },
        {
          "back": "CEO挪用客户资金到关联交易公司进行投机。",
          "estimated_seconds": 8,
          "explanation": "Sam Bankman-Fried将客户存款转移到Alameda Research，导致资金链断裂。",
          "front": "FTX破产的主要原因是什么？",
          "question_id": "q_flash_ftx_case_v2"
        }
      ]
    }
  ],
  "lesson_id": "L9",
  "longform_families": [
    {
      "concept_key": "counterparty_risk",
      "coverage_tags": [
        "counterparty_risk",
        "ftx_case_study"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_counterparty_risk",
      "learning_goal": "学生能解释对手方风险的概念，并结合FTX案例说明其危害和防范措施。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "mechanism_trace",
      "term_refs": [
        {
          "display": "对手方风险",
          "en": "Counterparty Risk"
        },
        {
          "display": "FTX",
          "en": "FTX"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "定义对手方风险",
            "FTX案例中对手方风险的具体表现",
            "投资者可以采取的防范措施"
          ],
          "question_id": "q_long_counterparty_risk_v1",
          "reference_answer": [
            "对手方风险是指交易中的另一方（如券商、交易所）可能违约，无法履行其义务（如返还资金）的风险。",
            "在FTX案例中，CEO Sam Bankman-Fried将客户存款转移到其关联公司Alameda Research进行高风险投机，导致FTX资不抵债并破产。客户虽然账户显示有资产，但实际上无法提取，这就是对手方风险的直接体现。",
            "投资者可以通过以下方式防范：1）选择受SFC、SEC等权威机构监管的券商，确保其在合法框架下运营；2）不要将所有资金放在一个券商，分散风险；3）关注平台是否提供储备金证明（PoR），以验证其资产是否足以覆盖负债。"
          ],
          "rubric_points": [
            "正确解释对手方风险是交易对手方违约的风险",
            "准确描述FTX挪用客户资金导致客户无法取回资金的过程",
            "提出至少两个具体的防范措施，如选择受监管券商、分散资金、查看储备金证明等"
          ],
          "stem": "请解释什么是“对手方风险”，并结合FTX案例说明这种风险是如何体现的，以及投资者应该如何防范。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "对比两家券商在监管、历史和透明度上的差异",
            "分析券商B的低费用可能隐藏的风险",
            "从对手方风险角度说明为什么选择券商A更安全"
          ],
          "question_id": "q_long_counterparty_risk_v2",
          "reference_answer": [
            "券商A受SFC监管，意味着它必须遵守严格的资金隔离和财务披露规定，降低了挪用客户资金的风险。10年历史表明其运营稳定。储备金证明提供了额外的透明度，让客户可以验证资产。",
            "券商B虽然费用低，但不受监管意味着没有外部监督，资金安全完全依赖其自身诚信。一旦出现问题，投资者可能无法追回资金。",
            "从对手方风险角度看，选择券商A虽然费用较高，但本金安全更有保障。对于量化基金来说，资金安全是首要考虑，节省的交易费用远不能弥补本金损失的风险。"
          ],
          "rubric_points": [
            "指出券商A受监管、历史长、有储备金证明，这些都能降低对手方风险",
            "指出券商B的低费用可能以牺牲安全为代价，如缺乏监管和资金隔离",
            "强调对手方风险可能导致本金损失，远超过节省的交易费用"
          ],
          "stem": "假设你是一个量化基金的负责人，正在评估两家券商。券商A受SFC监管，有10年历史，提供储备金证明；券商B不受监管，但交易费用低50%。请分析选择券商A的理由，重点从对手方风险的角度阐述。"
        }
      ]
    },
    {
      "concept_key": "proof_of_reserves",
      "coverage_tags": [
        "proof_of_reserves",
        "counterparty_risk"
      ],
      "difficulty": "hard",
      "family_id": "qf_long_proof_of_reserves",
      "learning_goal": "学生能解释储备金证明的工作原理及其在降低对手方风险中的作用。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "mechanism_trace",
      "term_refs": [
        {
          "display": "储备金证明",
          "en": "Proof-of-Reserves"
        },
        {
          "display": "默克尔树",
          "en": "Merkle Tree"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 150,
          "prompt_blocks": [
            "储备金证明的目的",
            "默克尔树的结构和工作原理",
            "投资者如何通过PoR验证自己的资产",
            "PoR如何降低对手方风险"
          ],
          "question_id": "q_long_proof_of_reserves_v1",
          "reference_answer": [
            "储备金证明（PoR）是一种让金融机构证明其持有足够资产来覆盖客户负债的方法，目的是增强信任和透明度。",
            "默克尔树是一种树形数据结构。首先，每个用户的余额被哈希处理，形成叶子节点。然后，相邻的叶子节点配对并再次哈希，形成父节点。这个过程重复进行，直到生成一个单一的根哈希（Merkle Root）。",
            "平台会公布这个Merkle Root。用户可以获取从自己的叶子节点到根节点的路径哈希，通过计算验证自己的余额是否被包含在树中，而无需知道其他用户的余额。",
            "PoR降低了对手方风险，因为它让平台难以伪造资产数据。如果平台实际资产不足，它无法生成一个与所有用户余额匹配的Merkle Root。投资者可以定期检查PoR，确保平台资产状况健康，从而降低因平台违约而损失资金的风险。"
          ],
          "rubric_points": [
            "正确说明PoR的目的是证明平台资产足以覆盖用户负债",
            "解释默克尔树如何将用户余额哈希成叶子节点，并逐层合并成根哈希",
            "说明用户可以通过自己的余额和路径哈希验证其是否被包含在树中",
            "指出PoR增加了透明度，使平台难以隐瞒资金缺口，从而降低对手方风险"
          ],
          "stem": "请解释储备金证明（Proof-of-Reserves）是如何工作的，以及它如何帮助投资者降低对手方风险。请特别说明默克尔树（Merkle Tree）在其中扮演的角色。"
        },
        {
          "estimated_seconds": 150,
          "prompt_blocks": [
            "传统券商的偿付能力证明方式（第三方审计）",
            "加密货币交易所的PoR方式（默克尔树）",
            "PoR相对于传统审计的优势",
            "PoR的局限性"
          ],
          "question_id": "q_long_proof_of_reserves_v2",
          "reference_answer": [
            "传统券商通常聘请第三方审计公司（如四大会计师事务所）定期审计其财务报表，以证明其偿付能力。这种审计通常每季度或每年进行一次，成本高且不及时。",
            "加密货币交易所使用PoR，通过默克尔树技术，将用户余额哈希并生成一个Merkle Root。用户可以自行验证自己的余额是否被包含在内。许多交易所每小时更新一次PoR数据。",
            "PoR的优势包括：1）更及时，可以实时或每小时更新；2）保护隐私，用户无需披露个人信息即可验证；3）用户可以独立验证，不依赖第三方。",
            "PoR的局限性在于：1）它只能证明平台在某个时间点持有一定数量的加密货币，但不能证明这些资产的质量（如是否被抵押）或流动性；2）它不能防止平台在生成PoR后立即转移资产。因此，PoR是增强透明度的工具，但不能完全替代传统审计。"
          ],
          "rubric_points": [
            "指出传统券商依赖第三方审计公司定期（如每季度）审计",
            "指出加密货币交易所使用PoR，可以更频繁（如每小时）更新",
            "说明PoR的优势：更及时、保护隐私、用户可以自行验证",
            "说明PoR的局限性：只能证明资产存在，不能证明资产质量或流动性"
          ],
          "stem": "比较传统券商（如受SEC监管的券商）和加密货币交易所（如Binance）在证明其偿付能力方面的不同做法。重点讨论储备金证明（PoR）相对于传统审计的优势和局限性。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "broker_selection_overview",
      "coverage_tags": [
        "broker_selection_overview"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_broker_overview",
      "learning_goal": "学生能在多个选项中识别出选择券商的关键考量因素。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "券商选择",
          "en": "Broker Selection"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "开户赠金是促销手段，不属于核心考量因素。六个关键因素是：交易成本、市场准入、交易工具、监管合规、客户支持、可靠性与声誉。",
          "options": [
            "交易成本",
            "市场准入",
            "开户赠金金额",
            "监管合规"
          ],
          "question_id": "q_quiz_broker_overview_v1",
          "stem": "以下哪一项不属于选择券商时需要考虑的六个关键因素？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "交易工具（如API接口、实时数据、回测平台）直接决定了策略能否被有效执行和测试。",
          "options": [
            "交易界面是否美观",
            "交易工具是否完善",
            "App下载量",
            "公司Logo设计"
          ],
          "question_id": "q_quiz_broker_overview_v2",
          "stem": "选择券商时，以下哪个因素直接关系到策略能否顺利执行？"
        }
      ]
    },
    {
      "concept_key": "trading_costs",
      "coverage_tags": [
        "trading_costs"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_trading_costs",
      "learning_goal": "学生能判断交易成本对高频交易的影响。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "交易成本",
          "en": "Trading Costs"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 25,
          "explanation": "每笔交易总成本 = 0.1 + 0.05 = 0.15美元。每天成本 = 1000 * 0.15 = 150美元。一个月成本 = 150 * 20 = 3000美元。",
          "options": [
            "300美元",
            "3000美元",
            "1500美元",
            "6000美元"
          ],
          "question_id": "q_quiz_trading_costs_v1",
          "stem": "一个高频交易策略每天执行1000笔交易，每笔交易佣金0.1美元，点差成本0.05美元。一个月（20个交易日）的总交易成本是多少？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "高频交易次数极多，每笔交易的佣金和点差会累积成巨大成本，远超一次性费用。",
          "options": [
            "一次性开户费",
            "每笔交易的佣金和点差",
            "账户管理年费",
            "出金手续费"
          ],
          "question_id": "q_quiz_trading_costs_v2",
          "stem": "以下哪种交易成本对高频交易者的利润影响最大？"
        }
      ]
    },
    {
      "concept_key": "market_access",
      "coverage_tags": [
        "market_access"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_market_access",
      "learning_goal": "学生能判断市场准入对策略实施的影响。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "市场准入",
          "en": "Access to Markets"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 15,
          "explanation": "市场准入的核心是确认券商是否支持所需的交易品种和杠杆/保证金政策。",
          "options": [
            "券商是否支持这两种资产类别以及杠杆政策",
            "券商是否有移动App",
            "券商的客户支持是否24小时",
            "券商的公司历史"
          ],
          "question_id": "q_quiz_market_access_v1",
          "stem": "一个交易策略需要同时交易美股和加密货币，并且需要使用5倍杠杆。选择券商时，市场准入方面最需要确认什么？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "API接口文档质量属于交易工具的范畴，不属于市场准入。",
          "options": [
            "支持的交易品种",
            "杠杆倍数",
            "API接口文档质量",
            "保证金政策"
          ],
          "question_id": "q_quiz_market_access_v2",
          "stem": "以下哪项不属于市场准入的考量范围？"
        }
      ]
    },
    {
      "concept_key": "trading_tools",
      "coverage_tags": [
        "trading_tools"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_trading_tools",
      "learning_goal": "学生能区分交易工具的不同功能。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "交易工具",
          "en": "Trading Tools"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "完善的API接口是量化交易者实现程序化自动交易的基础。",
          "options": [
            "移动App",
            "完善的API接口",
            "内置的新闻推送",
            "社交交易功能"
          ],
          "question_id": "q_quiz_trading_tools_v1",
          "stem": "对于量化交易者，以下哪个交易工具功能最为关键？"
        },
        {
          "answer": 0,
          "estimated_seconds": 15,
          "explanation": "模拟账户允许交易者在真实市场环境下测试策略，而无需承担资金风险。",
          "options": [
            "在不投入真实资金的情况下测试交易策略和平台",
            "获得更高的交易杠杆",
            "享受更低的交易佣金",
            "参与平台的抽奖活动"
          ],
          "question_id": "q_quiz_trading_tools_v2",
          "stem": "模拟账户（Demo Account）的主要用途是什么？"
        }
      ]
    },
    {
      "concept_key": "regulatory_compliance",
      "coverage_tags": [
        "regulatory_compliance"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_regulatory_compliance",
      "learning_goal": "学生能识别不同地区的监管机构。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "监管合规",
          "en": "Regulatory Compliance"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "SFC（Securities and Futures Commission）是香港证监会。SEC和FINRA属于美国，FCA属于英国。",
          "options": [
            "SEC",
            "FCA",
            "SFC",
            "FINRA"
          ],
          "question_id": "q_quiz_regulatory_compliance_v1",
          "stem": "以下哪个机构是香港的金融监管机构？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "监管机构要求券商遵守严格的规则，如客户资金隔离、定期财务披露等，以保护投资者。",
          "options": [
            "受监管的券商交易速度更快",
            "受监管的券商必须遵守资金隔离等规定，降低资金风险",
            "受监管的券商手续费一定更低",
            "受监管的券商提供更多交易品种"
          ],
          "question_id": "q_quiz_regulatory_compliance_v2",
          "stem": "为什么选择受监管的券商很重要？"
        }
      ]
    },
    {
      "concept_key": "customer_support",
      "coverage_tags": [
        "customer_support"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_customer_support",
      "learning_goal": "学生能判断客户支持的重要性。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "客户支持",
          "en": "Customer Support"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "API连接问题需要立即解决，24/7的实时客服能提供最及时的帮助。",
          "options": [
            "丰富的在线教程",
            "24/7的实时客服",
            "每周一次的网络研讨会",
            "详细的用户手册"
          ],
          "question_id": "q_quiz_customer_support_v1",
          "stem": "一个量化交易者在周末发现API连接出现问题，此时最需要券商的哪种客户支持？"
        },
        {
          "answer": 0,
          "estimated_seconds": 15,
          "explanation": "客户支持包括帮助解决问题和提供学习资源，但不包括保证盈利。",
          "options": [
            "提供交易策略的盈利保证",
            "提供在线教程和网络研讨会",
            "提供24小时客服",
            "提供详细的平台文档"
          ],
          "question_id": "q_quiz_customer_support_v2",
          "stem": "以下哪项不属于客户支持的范畴？"
        }
      ]
    },
    {
      "concept_key": "reliability_reputation",
      "coverage_tags": [
        "reliability_reputation"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_reliability_reputation",
      "learning_goal": "学生能判断评估券商可靠性的方法。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "可靠性与声誉",
          "en": "Reliability and Reputation"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "用户评价反映真实体验，公司历史长的通常更稳定可靠。",
          "options": [
            "查看其社交媒体粉丝数量",
            "查看其在CoinMarketCap上的排名",
            "查看用户评价和公司历史",
            "查看其广告投放量"
          ],
          "question_id": "q_quiz_reliability_reputation_v1",
          "stem": "以下哪种方法最能帮助评估一家券商的可靠性？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "上市公司受监管机构要求，必须定期公开财务和运营信息，透明度更高。",
          "options": [
            "上市公司规模一定更大",
            "上市公司需要定期披露财务信息，运营更透明",
            "上市公司交易费用更低",
            "上市公司提供更多交易品种"
          ],
          "question_id": "q_quiz_reliability_reputation_v2",
          "stem": "为什么上市公司通常被认为更可靠？"
        }
      ]
    },
    {
      "concept_key": "counterparty_risk",
      "coverage_tags": [
        "counterparty_risk"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_counterparty_risk",
      "learning_goal": "学生能识别对手方风险的场景。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "对手方风险",
          "en": "Counterparty Risk"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "对手方风险是指交易对手方（这里是券商）违约的风险。券商挪用资金后破产，客户无法取回资金，正是典型的对手方风险。",
          "options": [
            "市场突然暴跌导致策略亏损",
            "交易策略本身存在bug导致亏损",
            "券商挪用客户资金后破产，客户无法取回资金",
            "网络延迟导致订单未能成交"
          ],
          "question_id": "q_quiz_counterparty_risk_v1",
          "stem": "以下哪个场景最直接地体现了对手方风险？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "选择受监管的券商可以降低违约风险，分散资金可以避免单一券商出问题导致全部损失。",
          "options": [
            "只使用一个券商以方便管理",
            "选择受监管的券商并分散资金",
            "使用更高的杠杆",
            "只交易加密货币"
          ],
          "question_id": "q_quiz_counterparty_risk_v2",
          "stem": "为了降低对手方风险，投资者应该怎么做？"
        }
      ]
    },
    {
      "concept_key": "proof_of_reserves",
      "coverage_tags": [
        "proof_of_reserves"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_proof_of_reserves",
      "learning_goal": "学生能理解储备金证明的作用和技术。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "储备金证明",
          "en": "Proof-of-Reserves"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "PoR的目的是增强信任和透明度，证明平台有足够的资产来偿还用户。",
          "options": [
            "提高交易速度",
            "证明平台持有足够资产来覆盖用户存款",
            "降低交易手续费",
            "增加可交易品种"
          ],
          "question_id": "q_quiz_proof_of_reserves_v1",
          "stem": "储备金证明（PoR）的主要目的是什么？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "默克尔树允许用户在不泄露个人数据的情况下，验证自己的资产是否被包含在平台的储备中。",
          "options": [
            "可以增加交易量",
            "允许用户验证自己的余额是否被包含在总储备中，同时保护隐私",
            "可以降低交易延迟",
            "可以自动执行交易策略"
          ],
          "question_id": "q_quiz_proof_of_reserves_v2",
          "stem": "在加密货币交易所的储备金证明中，默克尔树技术的主要优势是什么？"
        }
      ]
    },
    {
      "concept_key": "ftx_case_study",
      "coverage_tags": [
        "ftx_case_study"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_ftx_case",
      "learning_goal": "学生能分析FTX事件的教训。",
      "linked_steps": [
        "step5"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "FTX",
          "en": "FTX"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "FTX的CEO Sam Bankman-Fried将客户存款转移到其关联交易公司Alameda Research，用于高风险投机，最终导致破产。",
          "options": [
            "交易策略失败导致亏损",
            "黑客攻击导致资金被盗",
            "CEO挪用客户资金到关联公司进行高风险投机",
            "监管机构突然关闭交易所"
          ],
          "question_id": "q_quiz_ftx_case_v1",
          "stem": "FTX破产事件中，核心问题是什么？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "FTX事件表明，即使是大型交易所也可能存在风险。投资者需要研究券商的合规性、声誉，并分散资金以降低风险。",
          "options": [
            "不要投资加密货币",
            "只使用最大的交易所",
            "进行尽职调查，选择受监管的券商，并分散资金",
            "永远不要使用杠杆"
          ],
          "question_id": "q_quiz_ftx_case_v2",
          "stem": "从FTX事件中，投资者应该吸取的最重要教训是什么？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L9: Market practice in strategy optimization, broker selection, and market tricks - Guidelines to choose a trading broker",
    "guided_story_manifest": "pipeline/3-guided_story/L9/step5/step.json",
    "lesson_map": "L9 step5: 如何选择交易券商",
    "plain_text": "pipeline/1-plain/L9/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L9: Market practice in strategy optimization, broker selection, and market tricks - Broker Selection"
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "fund_types",
      "covered_by": [
        "qf_flash_fund_types",
        "qf_quiz_fund_types"
      ],
      "description": "五种主要投资基金类型：ETF、共同基金、货币市场基金、私募股权基金、对冲基金"
    },
    {
      "coverage_tag": "hedge_fund_secret",
      "covered_by": [
        "qf_flash_hedge_secret",
        "qf_quiz_hedge_secret",
        "qf_long_hedge_secret"
      ],
      "description": "顶级对冲基金（如文艺复兴科技）的秘密：运行大量不相关策略，利用大数定律平滑收益"
    },
    {
      "coverage_tag": "uncorrelated_strategies",
      "covered_by": [
        "qf_flash_uncorrelated",
        "qf_quiz_uncorrelated"
      ],
      "description": "不相关策略的定义与作用：彼此没有明显关联，不会同时涨跌，用于分散风险"
    },
    {
      "coverage_tag": "law_of_large_numbers",
      "covered_by": [
        "qf_flash_law_large_numbers",
        "qf_quiz_law_large_numbers"
      ],
      "description": "大数定律在不相关策略组合中的应用：独立试验次数足够多时，平均结果趋近于期望值"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "fund_types",
      "coverage_tags": [
        "fund_types"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_fund_types",
      "learning_goal": "学生能说出五种主要投资基金类型，并准确回忆每种基金的核心特征。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "每种基金类型的核心特征：交易方式、流动性、风险水平。",
      "term_refs": [
        {
          "display": "交易所交易基金",
          "en": "Exchange-Traded Fund (ETF)"
        },
        {
          "display": "共同基金",
          "en": "Mutual Fund"
        },
        {
          "display": "货币市场基金",
          "en": "Money Market Fund"
        },
        {
          "display": "私募股权基金",
          "en": "Private Equity Fund"
        },
        {
          "display": "对冲基金",
          "en": "Hedge Fund"
        }
      ],
      "variants": [
        {
          "back": "交易所交易基金（ETF）",
          "estimated_seconds": 8,
          "explanation": "ETF在交易所上市交易，投资者可以像买卖股票一样随时买卖，通常被动追踪指数。",
          "front": "哪种基金像股票一样在交易所买卖，通常追踪某个指数，费用低、流动性好？",
          "question_id": "q_flash_fund_types_v1"
        },
        {
          "back": "共同基金（Mutual Fund）",
          "estimated_seconds": 8,
          "explanation": "共同基金汇集投资者资金，由基金经理管理，每天只在收盘后按净值交易一次。",
          "front": "哪种基金由专业经理主动管理，每天按收盘净值交易一次？",
          "question_id": "q_flash_fund_types_v2"
        },
        {
          "back": "货币市场基金（Money Market Fund）",
          "estimated_seconds": 8,
          "explanation": "货币市场基金投资于短期、高信用等级的债务工具，如国库券、商业票据，风险低、流动性好。",
          "front": "哪种基金风险低，主要投资短期债券，适合存放现金？",
          "question_id": "q_flash_fund_types_v3"
        },
        {
          "back": "私募股权基金（Private Equity Fund）",
          "estimated_seconds": 8,
          "explanation": "私募股权基金投资于非上市公司或收购上市公司，通常需要较长时间才能退出变现。",
          "front": "哪种基金流动性差、投资期长（5-10年），但潜在回报高？",
          "question_id": "q_flash_fund_types_v4"
        }
      ]
    },
    {
      "concept_key": "hedge_fund_secret",
      "coverage_tags": [
        "hedge_fund_secret"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_hedge_secret",
      "learning_goal": "学生能准确回忆顶级对冲基金获得稳定收益的核心机制。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "对冲基金秘密的两个关键要素：大量不相关策略 + 大数定律。",
      "term_refs": [
        {
          "display": "不相关策略",
          "en": "Uncorrelated Strategies"
        },
        {
          "display": "大数定律",
          "en": "Law of Large Numbers"
        }
      ],
      "variants": [
        {
          "back": "不相关策略（Uncorrelated Strategies）",
          "estimated_seconds": 8,
          "explanation": "这些策略彼此之间没有明显关联，不会同时涨跌，从而分散风险。",
          "front": "顶级对冲基金（如文艺复兴科技）同时运行成百上千个什么类型的策略？",
          "question_id": "q_flash_hedge_secret_v1"
        },
        {
          "back": "大数定律（Law of Large Numbers）",
          "estimated_seconds": 8,
          "explanation": "大数定律指出，当独立试验次数足够多时，平均结果会趋近于期望值。不相关策略的组合正是利用了这一点。",
          "front": "对冲基金利用哪个数学原理，通过大量不相关策略的组合来平滑收益曲线？",
          "question_id": "q_flash_hedge_secret_v2"
        }
      ]
    },
    {
      "concept_key": "uncorrelated_strategies",
      "coverage_tags": [
        "uncorrelated_strategies"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_uncorrelated",
      "learning_goal": "学生能定义不相关策略并理解其在投资组合中的作用。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "definition_with_example",
      "retrieval_focus": "不相关策略的定义和核心作用。",
      "term_refs": [
        {
          "display": "不相关策略",
          "en": "Uncorrelated Strategies"
        }
      ],
      "variants": [
        {
          "back": "不相关策略是指彼此之间没有明显关联、不会同时涨跌的交易策略。例如：一个策略在牛市中表现好，另一个策略在熊市中表现好。",
          "estimated_seconds": 12,
          "explanation": "不相关策略的核心价值在于分散风险：当一个策略亏损时，其他策略可能盈利，从而平滑整体收益。",
          "front": "什么是不相关策略？请给出一个简单的例子。",
          "question_id": "q_flash_uncorrelated_v1"
        },
        {
          "back": "分散风险，稳定整体收益，平滑收益曲线。",
          "estimated_seconds": 8,
          "explanation": "通过组合多个不相关的策略，可以降低单一策略波动对整体组合的影响。",
          "front": "对冲基金使用不相关策略的主要目的是什么？",
          "question_id": "q_flash_uncorrelated_v2"
        }
      ]
    },
    {
      "concept_key": "law_of_large_numbers",
      "coverage_tags": [
        "law_of_large_numbers"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_law_large_numbers",
      "learning_goal": "学生能解释大数定律如何帮助对冲基金获得稳定收益。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "cause_effect",
      "retrieval_focus": "大数定律在不相关策略组合中的因果逻辑。",
      "term_refs": [
        {
          "display": "大数定律",
          "en": "Law of Large Numbers"
        }
      ],
      "variants": [
        {
          "back": "因为大数定律：独立策略数量足够多时，整体胜率趋近于期望值，波动被平滑。",
          "estimated_seconds": 10,
          "explanation": "单个策略有波动，但大量不相关策略组合后，整体收益曲线会变得非常平滑和稳定。",
          "front": "为什么对冲基金运行大量不相关策略（每个策略胜率仅略高于50%）能获得稳定收益？",
          "question_id": "q_flash_law_large_numbers_v1"
        },
        {
          "back": "使大量不相关策略的平均收益趋近于期望值，从而平滑收益曲线。",
          "estimated_seconds": 8,
          "explanation": "这是对冲基金能长期保持稳定盈利的核心数学原理。",
          "front": "大数定律在对冲基金策略组合中起什么作用？",
          "question_id": "q_flash_law_large_numbers_v2"
        }
      ]
    }
  ],
  "lesson_id": "L9",
  "longform_families": [
    {
      "concept_key": "hedge_fund_secret",
      "coverage_tags": [
        "hedge_fund_secret",
        "uncorrelated_strategies",
        "law_of_large_numbers"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_hedge_secret",
      "learning_goal": "学生能完整解释对冲基金如何通过不相关策略和大数定律获得稳定收益。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "mechanism_trace",
      "term_refs": [
        {
          "display": "不相关策略",
          "en": "Uncorrelated Strategies"
        },
        {
          "display": "大数定律",
          "en": "Law of Large Numbers"
        },
        {
          "display": "对冲基金",
          "en": "Hedge Fund"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "不相关策略的定义",
            "使用不相关策略的原因",
            "大数定律的作用"
          ],
          "question_id": "q_long_hedge_secret_v1",
          "reference_answer": [
            "不相关策略是指彼此之间没有明显关联、不会同时涨跌的交易策略。例如，一个策略在牛市中表现好，另一个在熊市中表现好。",
            "使用不相关策略而不是单一策略，是因为单一策略的波动性很大，可能在某段时间表现很差。通过组合大量不相关的策略，可以分散风险：当一个策略亏损时，其他策略可能盈利，从而平滑整体收益。",
            "大数定律在这个机制中扮演核心角色。每个策略的胜率可能只比50%高一点（例如55%），但因为有成百上千个不相关的策略同时运行，根据大数定律，独立试验次数足够多时，平均结果会趋近于期望值。因此，整体收益曲线会变得非常平滑和稳定，而不是依赖单个策略的表现。"
          ],
          "rubric_points": [
            "正确解释不相关策略：彼此没有明显关联，不会同时涨跌",
            "说明使用不相关策略是为了分散风险，避免单一策略波动影响整体",
            "解释大数定律：独立试验次数足够多时，平均结果趋近于期望值",
            "将大数定律与策略组合联系起来：大量不相关策略使整体收益趋近于期望值，平滑收益曲线"
          ],
          "stem": "请解释顶级对冲基金（如文艺复兴科技）如何通过运行大量不相关策略来获得长期稳定收益。在回答中，请说明：\n1. 什么是不相关策略？\n2. 为什么使用不相关策略而不是单一策略？\n3. 大数定律在这个机制中扮演什么角色？"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "不相关的重要性",
            "大数定律的应用",
            "高度相关策略的后果"
          ],
          "question_id": "q_long_hedge_secret_v2",
          "reference_answer": [
            "策略需要是不相关的，这样它们就不会同时涨跌。如果一个策略亏损，其他策略可能盈利，从而分散风险，平滑整体收益曲线。",
            "大数定律指出，当独立试验次数足够多时，平均结果会趋近于期望值。1000个不相关策略，每个胜率52%，整体胜率会趋近于52%，并且波动被大大降低，从而获得稳定收益。",
            "如果这些策略是高度相关的，它们会同时盈利或同时亏损。在市场不利时，所有策略可能同时亏损，导致基金遭受巨大损失，无法起到分散风险的作用。"
          ],
          "rubric_points": [
            "解释不相关策略的分散风险作用",
            "正确应用大数定律解释稳定收益的来源",
            "说明高度相关策略会导致同时亏损，无法分散风险"
          ],
          "stem": "假设你是一个对冲基金的策略经理，你开发了1000个不相关的交易策略，每个策略的胜率约为52%。请解释：\n1. 为什么这些策略需要是“不相关”的？\n2. 大数定律如何帮助你的基金获得稳定收益？\n3. 如果这些策略是高度相关的，会发生什么？"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "fund_types",
      "coverage_tags": [
        "fund_types"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_fund_types",
      "learning_goal": "学生能在不同场景下区分五种投资基金类型。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "交易所交易基金",
          "en": "Exchange-Traded Fund (ETF)"
        },
        {
          "display": "共同基金",
          "en": "Mutual Fund"
        },
        {
          "display": "货币市场基金",
          "en": "Money Market Fund"
        },
        {
          "display": "私募股权基金",
          "en": "Private Equity Fund"
        },
        {
          "display": "对冲基金",
          "en": "Hedge Fund"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "ETF在交易所上市交易，可以像股票一样随时买卖，通常被动追踪指数，费用低。",
          "options": [
            "共同基金",
            "交易所交易基金（ETF）",
            "货币市场基金",
            "私募股权基金"
          ],
          "question_id": "q_quiz_fund_types_v1",
          "stem": "一位投资者希望像买卖股票一样随时交易，并且希望费用较低、追踪某个指数。以下哪种基金最适合？"
        },
        {
          "answer": 3,
          "estimated_seconds": 15,
          "explanation": "对冲基金采用多种高级量化策略，包括杠杆、做空和衍生品，追求高回报。",
          "options": [
            "货币市场基金",
            "共同基金",
            "交易所交易基金（ETF）",
            "对冲基金"
          ],
          "question_id": "q_quiz_fund_types_v2",
          "stem": "以下哪种基金通常使用杠杆、做空和衍生品等复杂策略来追求高回报？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "货币市场基金投资于短期高信用等级债务工具，风险低、流动性好，适合存放现金。",
          "options": [
            "私募股权基金",
            "对冲基金",
            "货币市场基金",
            "交易所交易基金（ETF）"
          ],
          "question_id": "q_quiz_fund_types_v3",
          "stem": "一位保守型投资者希望将短期闲置资金放在一个风险低、流动性好的地方，以下哪种基金最合适？"
        }
      ]
    },
    {
      "concept_key": "hedge_fund_secret",
      "coverage_tags": [
        "hedge_fund_secret"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_hedge_secret",
      "learning_goal": "学生能理解并应用对冲基金获得稳定收益的核心原理。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "不相关策略",
          "en": "Uncorrelated Strategies"
        },
        {
          "display": "大数定律",
          "en": "Law of Large Numbers"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "大数定律指出，当独立试验次数足够多时，平均结果会趋近于期望值。不相关策略的组合正是利用了这一点。",
          "options": [
            "复利效应",
            "大数定律",
            "均值回归",
            "有效市场假说"
          ],
          "question_id": "q_quiz_hedge_secret_v1",
          "stem": "对冲基金通过运行大量不相关策略来获得稳定收益，这主要利用了哪个原理？"
        },
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "顶级对冲基金同时运行成百上千个不相关策略，每个策略胜率可能只比50%高一点，但组合起来利用大数定律获得稳定收益。",
          "options": [
            "只投资于高收益债券",
            "使用单一的高胜率策略",
            "同时运行大量不相关的策略，利用大数定律平滑收益",
            "依靠内幕消息进行交易"
          ],
          "question_id": "q_quiz_hedge_secret_v2",
          "stem": "以下哪项最准确地描述了顶级对冲基金（如文艺复兴科技）获得长期稳定收益的秘密？"
        }
      ]
    },
    {
      "concept_key": "uncorrelated_strategies",
      "coverage_tags": [
        "uncorrelated_strategies"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_uncorrelated",
      "learning_goal": "学生能判断关于不相关策略的陈述是否正确。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "true_false",
      "term_refs": [
        {
          "display": "不相关策略",
          "en": "Uncorrelated Strategies"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 10,
          "explanation": "不相关策略的核心特征就是彼此之间没有明显关联，不会同时涨跌，从而起到分散风险的作用。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_uncorrelated_v1",
          "stem": "不相关策略意味着这些策略之间没有明显关联，不会同时涨跌。"
        },
        {
          "answer": 1,
          "estimated_seconds": 10,
          "explanation": "使用不相关策略的主要目的是分散风险、稳定整体收益，而不是提高单个策略的胜率。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_uncorrelated_v2",
          "stem": "对冲基金使用不相关策略的主要目的是为了提高单个策略的胜率。"
        }
      ]
    },
    {
      "concept_key": "law_of_large_numbers",
      "coverage_tags": [
        "law_of_large_numbers"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_law_large_numbers",
      "learning_goal": "学生能理解大数定律在不相关策略组合中的具体应用。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "大数定律",
          "en": "Law of Large Numbers"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 25,
          "explanation": "大数定律指出，当独立试验次数足够多时，平均结果会趋近于期望值。1000个不相关策略的整体胜率会趋近于55%，收益曲线被平滑。",
          "options": [
            "所有策略都会盈利",
            "整体胜率会趋近于55%，收益曲线会变得平滑",
            "每个策略的胜率都会提高到100%",
            "整体收益会变得极不稳定"
          ],
          "question_id": "q_quiz_law_large_numbers_v1",
          "stem": "假设一个对冲基金运行了1000个不相关的策略，每个策略的胜率是55%。根据大数定律，以下哪个描述最准确？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "大量不相关策略的组合，利用大数定律使整体收益趋近于期望值，从而获得稳定向上的收益曲线。",
          "options": [
            "一个策略在牛市中赚了100%，在熊市中亏了50%",
            "1000个不相关的策略，每个胜率52%，组合后整体收益稳定向上",
            "一个策略连续10次盈利后第11次必然亏损",
            "两个高度相关的策略同时盈利或同时亏损"
          ],
          "question_id": "q_quiz_law_large_numbers_v2",
          "stem": "以下哪个场景最符合大数定律在不相关策略组合中的应用？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L9/plain.txt",
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
      "coverage_tag": "market_tricks_misleading_metrics",
      "covered_by": [
        "qf_flash_winrate_def",
        "qf_quiz_winrate_trick",
        "qf_long_misleading_metrics"
      ],
      "description": "市场伎俩：误导性指标（胜率定义操纵）"
    },
    {
      "coverage_tag": "market_tricks_selective_presentation",
      "covered_by": [
        "qf_flash_sharpe_manip",
        "qf_quiz_sharpe_frequency",
        "qf_long_misleading_metrics"
      ],
      "description": "市场伎俩：选择性呈现数据（夏普比率操纵）"
    },
    {
      "coverage_tag": "market_tricks_psychological_manipulation",
      "covered_by": [
        "qf_flash_fomo",
        "qf_quiz_fomo_example",
        "qf_long_misleading_metrics"
      ],
      "description": "市场伎俩：心理操纵（错失恐惧症FOMO）"
    },
    {
      "coverage_tag": "market_tricks_defense",
      "covered_by": [
        "qf_flash_defense_advice",
        "qf_quiz_defense_approach"
      ],
      "description": "应对市场伎俩的建议：看透表面指标，关注上下文、时间框架和风险度量"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "misleading_winning_rate",
      "coverage_tags": [
        "market_tricks_misleading_metrics"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_winrate_def",
      "learning_goal": "学生能区分两种胜率定义，并指出哪种更容易被操纵。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "两种胜率定义的核心差异及其误导性。",
      "term_refs": [
        {
          "display": "误导性胜率定义",
          "en": "Misleading Winning Rate Definition"
        }
      ],
      "variants": [
        {
          "back": "20%（1天盈利 ÷ 5个交易日）",
          "estimated_seconds": 10,
          "explanation": "按交易日计算胜率更能反映策略的日常表现，而按订单计算容易被单笔大盈利扭曲。",
          "front": "一家机构宣称其策略胜率100%，因为一笔持仓5天的交易最终盈利。按交易日计算，这5天里只有1天盈利，真实胜率是多少？",
          "question_id": "q_flash_winrate_def_v1"
        },
        {
          "back": "它掩盖了交易过程中的连续亏损天数，无法反映策略的日常波动和风险。",
          "estimated_seconds": 12,
          "explanation": "这种定义让机构可以展示一个很高的胜率，但投资者看不到交易过程中的实际亏损天数。",
          "front": "按订单计算的胜率（一笔交易最终盈利即算100%胜率）有什么主要问题？",
          "question_id": "q_flash_winrate_def_v2"
        }
      ]
    },
    {
      "concept_key": "sharpe_ratio_manipulation",
      "coverage_tags": [
        "market_tricks_selective_presentation"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_sharpe_manip",
      "learning_goal": "学生能解释机构如何通过选择不同的时间频率来操纵夏普比率。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "夏普比率操纵的核心手段。",
      "term_refs": [
        {
          "display": "夏普比率操纵",
          "en": "Sharpe Ratio Manipulation"
        }
      ],
      "variants": [
        {
          "back": "选择波动率较低的时间频率（如日数据 vs 月数据），因为夏普比率 = 年化收益 / 年化波动率。",
          "estimated_seconds": 12,
          "explanation": "不同频率的数据计算出的年化波动率不同，机构会选择对自己最有利的频率来展示。",
          "front": "机构可以通过选择哪种数据频率来操纵夏普比率，使其看起来更高？",
          "question_id": "q_flash_sharpe_manip_v1"
        },
        {
          "back": "必须确保计算方式一致，特别是数据频率（日/周/月）和计算窗口相同。",
          "estimated_seconds": 10,
          "explanation": "否则机构可能选择对自己最有利的频率来展示，导致误导性的比较结果。",
          "front": "比较两个投资产品的夏普比率时，必须确保什么条件才能进行公平比较？",
          "question_id": "q_flash_sharpe_manip_v2"
        }
      ]
    },
    {
      "concept_key": "fomo",
      "coverage_tags": [
        "market_tricks_psychological_manipulation"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_fomo",
      "learning_goal": "学生能识别FOMO营销手段及其对投资决策的影响。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "FOMO的定义和典型营销话术。",
      "term_refs": [
        {
          "display": "错失恐惧症",
          "en": "Fear of Missing Out (FOMO)"
        }
      ],
      "variants": [
        {
          "back": "担心错过潜在收益而做出冲动投资决策的心理现象。",
          "estimated_seconds": 8,
          "explanation": "机构利用FOMO制造紧迫感，让投资者不做理性分析就匆忙投资。",
          "front": "什么是错失恐惧症（FOMO）？",
          "question_id": "q_flash_fomo_v1"
        },
        {
          "back": "“限时优惠”、“仅剩最后名额”、“别人都赚了30%”。",
          "estimated_seconds": 8,
          "explanation": "这些话术制造紧迫感和社交压力，促使投资者冲动决策。",
          "front": "举一个机构利用FOMO的典型营销话术例子。",
          "question_id": "q_flash_fomo_v2"
        }
      ]
    },
    {
      "concept_key": "defense_against_market_tricks",
      "coverage_tags": [
        "market_tricks_defense"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_defense_advice",
      "learning_goal": "学生能记住应对市场销售伎俩的核心建议。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "应对市场伎俩的核心建议。",
      "term_refs": [
        {
          "display": "应对市场伎俩的建议",
          "en": "Defense Against Market Tricks"
        }
      ],
      "variants": [
        {
          "back": "关注完整的上下文、时间框架和风险度量。",
          "estimated_seconds": 8,
          "explanation": "不要只看表面指标，要理解指标是如何计算的，以及它是否全面反映了风险和收益。",
          "front": "面对机构展示的业绩指标，你应该关注哪些方面来避免被误导？",
          "question_id": "q_flash_defense_advice_v1"
        },
        {
          "back": "保持理性，避免冲动投资决策，进行充分的尽职调查。",
          "estimated_seconds": 8,
          "explanation": "FOMO利用情绪驱动决策，理性分析和尽职调查是避免陷阱的关键。",
          "front": "面对“限时优惠”等营销话术，你应该怎么做？",
          "question_id": "q_flash_defense_advice_v2"
        }
      ]
    }
  ],
  "lesson_id": "L9",
  "longform_families": [
    {
      "concept_key": "market_tricks_overview",
      "coverage_tags": [
        "market_tricks_misleading_metrics",
        "market_tricks_selective_presentation",
        "market_tricks_psychological_manipulation",
        "market_tricks_defense"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_misleading_metrics",
      "learning_goal": "学生能比较三种市场销售伎俩（误导性指标、选择性呈现、心理操纵），并给出应对建议。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "compare_and_contrast",
      "term_refs": [
        {
          "display": "市场销售伎俩",
          "en": "Market Sales Tricks"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "定义每种伎俩",
            "为每种伎俩提供一个来自课程材料的例子",
            "解释每种伎俩如何误导投资者",
            "给出针对每种伎俩的具体应对建议"
          ],
          "question_id": "q_long_misleading_metrics_v1",
          "reference_answer": [
            "1. 误导性指标：使用定义有问题的指标来美化业绩。例如，按订单计算胜率（一笔交易最终盈利即算100%胜率），掩盖交易过程中的连续亏损。应对：要求按交易日计算胜率，并查看完整的盈亏分布。",
            "2. 选择性呈现数据：选择对自己最有利的数据频率或时间窗口来展示指标。例如，用周数据计算夏普比率，因为周波动率通常低于日波动率，从而得到更高的夏普比率。应对：比较不同产品时，确保计算方式（数据频率、窗口）一致。",
            "3. 心理操纵：利用投资者的情绪（如FOMO）来驱动冲动决策。例如，使用“限时优惠”、“仅剩最后名额”等话术制造紧迫感。应对：保持理性，避免在压力下做决定，进行充分的尽职调查。"
          ],
          "rubric_points": [
            "正确区分三种伎俩的定义",
            "每个例子准确对应课程内容",
            "解释清晰，指出误导机制",
            "应对建议具体且可行"
          ],
          "stem": "请比较并解释三种常见的市场销售伎俩：误导性指标、选择性呈现数据和心理操纵。对于每种伎俩，请给出一个具体例子，并说明投资者应该如何应对。"
        },
        {
          "estimated_seconds": 150,
          "prompt_blocks": [
            "分析每个指标可能被操纵的方式",
            "提出具体的验证步骤",
            "解释为什么这些步骤能帮助识别潜在问题"
          ],
          "question_id": "q_long_misleading_metrics_v2",
          "reference_answer": [
            "可能存在的误导性：",
            "- 胜率85%：可能按订单计算，未披露平均亏损大小；或者只平仓盈利交易，让亏损交易一直开着。",
            "- 夏普比率2.5：可能使用了较低频率的数据（如周或月数据），或者选择了特定的时间窗口。",
            "- 年化收益30%：可能只展示了表现最好的时间段。",
            "尽职调查计划：",
            "1. 要求提供完整的交易记录，包括每笔交易的盈亏、持仓时间、以及每日的PnL。",
            "2. 按交易日重新计算胜率，并查看盈亏分布（平均盈利 vs 平均亏损）。",
            "3. 要求提供日数据计算的夏普比率，并查看不同时间窗口（如滚动3年）的夏普比率。",
            "4. 查看完整的回测报告，包括最大回撤、收益曲线、以及不同市场环境下的表现。",
            "5. 要求提供第三方审计或独立的业绩验证。"
          ],
          "rubric_points": [
            "识别出胜率、夏普比率可能被操纵",
            "验证步骤具体且可行",
            "解释验证步骤如何揭露潜在问题"
          ],
          "stem": "假设你是一家投资机构的分析师，你的团队发现一个潜在的投资产品，其宣传材料显示“年化收益30%，夏普比率2.5，胜率85%”。请分析这些指标可能存在的误导性，并制定一个尽职调查计划来验证其真实性。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "misleading_winning_rate",
      "coverage_tags": [
        "market_tricks_misleading_metrics"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_winrate_trick",
      "learning_goal": "学生能识别并分析利用胜率定义误导投资者的案例。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "误导性胜率定义",
          "en": "Misleading Winning Rate Definition"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "使用一个看似很高但定义有问题的指标（胜率）来掩盖整体不盈利的事实，属于误导性指标。",
          "options": [
            "选择性呈现数据",
            "误导性指标",
            "心理操纵",
            "以上都是"
          ],
          "question_id": "q_quiz_winrate_trick_v1",
          "stem": "一家基金宣称其策略胜率高达95%，但未披露平均亏损远大于平均盈利。这属于哪种市场伎俩？"
        },
        {
          "answer": 1,
          "estimated_seconds": 25,
          "explanation": "按交易日计算胜率能反映策略在每一天的表现，更全面地展示策略的波动性和稳定性。",
          "options": [
            "按订单计算的胜率更能反映策略的日常表现",
            "按交易日计算的胜率更能反映策略的日常表现",
            "两种计算方式结果相同",
            "按交易日计算的胜率总是更高"
          ],
          "question_id": "q_quiz_winrate_trick_v2",
          "stem": "一个策略在10个交易日中进行了5笔交易，其中4笔盈利，1笔亏损。按订单计算胜率是80%。但如果按交易日计算，10天中有3天盈利，胜率是30%。以下哪个说法正确？"
        }
      ]
    },
    {
      "concept_key": "sharpe_ratio_manipulation",
      "coverage_tags": [
        "market_tricks_selective_presentation"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_sharpe_frequency",
      "learning_goal": "学生能理解不同数据频率对夏普比率的影响，并识别操纵手段。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "夏普比率操纵",
          "en": "Sharpe Ratio Manipulation"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 30,
          "explanation": "年化夏普比率 = (日收益均值 / 日波动率) * sqrt(252) = (0.1%/1%) * sqrt(252) ≈ 1.59。",
          "options": [
            "0.1",
            "0.1 * sqrt(252) ≈ 1.59",
            "0.1 / 1 = 0.1",
            "1 / 0.1 = 10"
          ],
          "question_id": "q_quiz_sharpe_frequency_v1",
          "stem": "假设一个策略的日收益均值为0.1%，日波动率为1%。年化夏普比率（假设无风险利率为0）约为多少？"
        },
        {
          "answer": 2,
          "estimated_seconds": 25,
          "explanation": "周数据的波动率通常低于日数据，因此用周数据计算的夏普比率可能被高估。改用日数据后，波动率更高，夏普比率通常会降低。",
          "options": [
            "保持不变",
            "变得更高",
            "变得更低",
            "无法确定"
          ],
          "question_id": "q_quiz_sharpe_frequency_v2",
          "stem": "一家机构展示其策略的夏普比率为3.0，但使用的是周数据计算。如果改用日数据计算，夏普比率最可能发生什么变化？"
        }
      ]
    },
    {
      "concept_key": "fomo",
      "coverage_tags": [
        "market_tricks_psychological_manipulation"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_fomo_example",
      "learning_goal": "学生能识别FOMO营销手段，并理解其如何影响投资决策。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "错失恐惧症",
          "en": "Fear of Missing Out (FOMO)"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "“限时优惠”和“仅剩最后名额”制造紧迫感，利用FOMO促使投资者冲动决策。",
          "options": [
            "“我们的策略在过去5年平均年化收益15%”",
            "“限时优惠：仅剩最后10个名额，错过就要等一年”",
            "“我们的基金经理拥有20年从业经验”",
            "“本基金投资于多元化的资产组合”"
          ],
          "question_id": "q_quiz_fomo_example_v1",
          "stem": "以下哪个营销话术最典型地利用了错失恐惧症（FOMO）？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "展示他人成功和奢华生活方式，激发投资者害怕错过潜在收益的心理，即FOMO。",
          "options": [
            "损失厌恶",
            "确认偏误",
            "错失恐惧症（FOMO）",
            "锚定效应"
          ],
          "question_id": "q_quiz_fomo_example_v2",
          "stem": "一家投资机构在广告中展示“别人都赚了30%”的客户 testimonials，并配以豪车、度假图片。这主要利用了哪种心理？"
        }
      ]
    },
    {
      "concept_key": "defense_against_market_tricks",
      "coverage_tags": [
        "market_tricks_defense"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_defense_approach",
      "learning_goal": "学生能选择正确的应对策略来避免被市场销售伎俩误导。",
      "linked_steps": [
        "step6"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "应对市场伎俩的建议",
          "en": "Defense Against Market Tricks"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "应该要求查看完整的上下文，包括指标的计算方式、时间框架和风险度量，避免被表面指标误导。",
          "options": [
            "立即投资，以免错过机会",
            "要求查看完整的业绩报告，包括计算方式和风险指标",
            "相信知名机构的背书",
            "只关注收益，忽略风险"
          ],
          "question_id": "q_quiz_defense_approach_v1",
          "stem": "面对一个宣称“年化收益50%，夏普比率5.0”的投资产品，最理性的第一步行动是什么？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "尽职调查和理性分析是避免被误导的关键，要理解指标背后的含义和潜在操纵空间。",
          "options": [
            "只投资朋友推荐的产品",
            "关注表面指标，如高胜率和高夏普比率",
            "进行尽职调查，理解指标的计算方式和局限性",
            "在限时优惠活动期间尽快做出决定"
          ],
          "question_id": "q_quiz_defense_approach_v2",
          "stem": "以下哪个做法最能帮助投资者避免被市场销售伎俩误导？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L9/plain.txt",
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
      "coverage_tag": "optimal_vs_robust",
      "covered_by": [
        "qf_flash_optimal_vs_robust",
        "qf_quiz_optimal_vs_robust",
        "qf_long_robust_choice"
      ],
      "description": "最优解与稳健解的核心区别：回测表现 vs 实盘稳定性"
    },
    {
      "coverage_tag": "overfitting_definition",
      "covered_by": [
        "qf_flash_overfitting",
        "qf_quiz_overfitting"
      ],
      "description": "过拟合的定义及其在策略优化中的危害"
    },
    {
      "coverage_tag": "robust_solution_definition",
      "covered_by": [
        "qf_flash_robust_solution",
        "qf_quiz_robust_solution"
      ],
      "description": "稳健解的定义及其在不同市场环境下的表现"
    },
    {
      "coverage_tag": "parameter_sensitivity_analysis",
      "covered_by": [
        "qf_flash_parameter_sensitivity",
        "qf_quiz_parameter_sensitivity"
      ],
      "description": "参数敏感性分析：测试参数变化对性能的影响，识别稳定区域"
    },
    {
      "coverage_tag": "walk_forward_validation",
      "covered_by": [
        "qf_flash_walk_forward",
        "qf_quiz_walk_forward",
        "qf_long_walk_forward_example"
      ],
      "description": "滚动窗口验证：将数据分段，选择在各段表现一致的参数"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "optimal_vs_robust",
      "coverage_tags": [
        "optimal_vs_robust"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_optimal_vs_robust",
      "learning_goal": "学生能准确区分最优解和稳健解在回测与实盘中的表现差异。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "core_difference",
      "retrieval_focus": "最优解和稳健解在回测表现和实盘稳定性上的核心差异。",
      "term_refs": [
        {
          "display": "最优解",
          "en": "Optimal Solution"
        },
        {
          "display": "稳健解",
          "en": "Robust Solution"
        }
      ],
      "variants": [
        {
          "back": "回测表现很好，但容易过拟合，在实盘中失效。",
          "estimated_seconds": 8,
          "explanation": "最优解过度拟合了历史数据中的噪音，导致在新数据上表现不佳。",
          "front": "在策略优化中，最优解在回测中表现如何？在实盘中又可能面临什么问题？",
          "question_id": "q_flash_optimal_vs_robust_v1"
        },
        {
          "back": "回测收益不是最高，但在不同市场环境下都能稳定表现。",
          "estimated_seconds": 8,
          "explanation": "稳健解牺牲了部分回测收益，换取了实盘中的稳定性和抗风险能力。",
          "front": "稳健解在回测中的表现通常如何？它在实盘中的优势是什么？",
          "question_id": "q_flash_optimal_vs_robust_v2"
        }
      ]
    },
    {
      "concept_key": "overfitting",
      "coverage_tags": [
        "overfitting_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_overfitting",
      "learning_goal": "学生能定义过拟合，并理解其在策略优化中的危害。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "definition_with_example",
      "retrieval_focus": "过拟合的定义及其导致策略实盘失效的原因。",
      "term_refs": [
        {
          "display": "过拟合",
          "en": "Overfitting"
        }
      ],
      "variants": [
        {
          "back": "模型在历史数据上表现完美，但在新数据（实盘）上表现很差的现象。",
          "estimated_seconds": 8,
          "explanation": "过拟合的策略学到了历史噪音而非真实规律，因此无法适应未来市场变化。",
          "front": "什么是策略优化中的过拟合？",
          "question_id": "q_flash_overfitting_v1"
        },
        {
          "back": "因为它学到的只是历史噪音，而不是真正的市场规律。",
          "estimated_seconds": 6,
          "explanation": "历史噪音是随机波动，不具备预测性，因此策略在实盘中无法复制回测表现。",
          "front": "过拟合的策略在实盘中为什么会失效？",
          "question_id": "q_flash_overfitting_v2"
        }
      ]
    },
    {
      "concept_key": "robust_solution",
      "coverage_tags": [
        "robust_solution_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_robust_solution",
      "learning_goal": "学生能定义稳健解，并理解其相对于最优解的价值。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "definition_with_example",
      "retrieval_focus": "稳健解的定义及其核心优势。",
      "term_refs": [
        {
          "display": "稳健解",
          "en": "Robust Solution"
        }
      ],
      "variants": [
        {
          "back": "在不同市场环境下都能保持稳定表现的参数组合。",
          "estimated_seconds": 8,
          "explanation": "稳健解不追求回测收益最大化，而是追求实盘表现的稳定性和可靠性。",
          "front": "什么是策略优化中的稳健解？",
          "question_id": "q_flash_robust_solution_v1"
        },
        {
          "back": "在不同市场条件下表现更稳定，实盘失效风险更低。",
          "estimated_seconds": 8,
          "explanation": "稳健解通过牺牲部分回测收益，换取了更强的泛化能力和抗风险能力。",
          "front": "与最优解相比，稳健解的主要优势是什么？",
          "question_id": "q_flash_robust_solution_v2"
        }
      ]
    },
    {
      "concept_key": "parameter_sensitivity_analysis",
      "coverage_tags": [
        "parameter_sensitivity_analysis"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_parameter_sensitivity",
      "learning_goal": "学生能解释参数敏感性分析的目的和做法。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "参数敏感性分析的目的：评估参数变化对性能的影响，识别稳定区域。",
      "term_refs": [
        {
          "display": "参数敏感性分析",
          "en": "Parameter Sensitivity Analysis"
        }
      ],
      "variants": [
        {
          "back": "评估策略性能如何随参数变化，并识别出能产生一致结果的稳定参数区域。",
          "estimated_seconds": 10,
          "explanation": "通过观察参数微调时性能的波动情况，可以判断策略是否对参数选择过于敏感。",
          "front": "选择稳健策略时，进行参数敏感性分析的目的是什么？",
          "question_id": "q_flash_parameter_sensitivity_v1"
        },
        {
          "back": "在一系列参数值上都能产生一致结果的参数范围。",
          "estimated_seconds": 8,
          "explanation": "稳定区域内的参数对微调不敏感，表明策略更稳健。",
          "front": "在参数敏感性分析中，我们寻找的“稳定区域”是指什么？",
          "question_id": "q_flash_parameter_sensitivity_v2"
        }
      ]
    },
    {
      "concept_key": "walk_forward_validation",
      "coverage_tags": [
        "walk_forward_validation"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_walk_forward",
      "learning_goal": "学生能描述滚动窗口验证的基本步骤和目的。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "滚动窗口验证的核心步骤：将数据分段，选择在各段表现一致的参数。",
      "term_refs": [
        {
          "display": "滚动窗口验证",
          "en": "Walk-Forward Validation"
        }
      ],
      "variants": [
        {
          "back": "选择在多个数据子集上都能取得中等以上且稳定表现的参数组合。",
          "estimated_seconds": 10,
          "explanation": "不是选择在某一段表现最好的参数，而是选择在所有段都表现不错的参数，以确保稳健性。",
          "front": "滚动窗口验证中，如何选择最终的参数组合？",
          "question_id": "q_flash_walk_forward_v1"
        },
        {
          "back": "将5年数据分成5份，每份1年，然后分别测试参数组合在各年份的表现。",
          "estimated_seconds": 8,
          "explanation": "这是课程中给出的例子，目的是检验参数在不同市场周期下的表现一致性。",
          "front": "假设你有5年数据，使用滚动窗口验证时，通常如何划分数据？",
          "question_id": "q_flash_walk_forward_v2"
        }
      ]
    }
  ],
  "lesson_id": "L9",
  "longform_families": [
    {
      "concept_key": "optimal_vs_robust",
      "coverage_tags": [
        "optimal_vs_robust",
        "overfitting_definition",
        "robust_solution_definition"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_robust_choice",
      "learning_goal": "学生能解释为什么在实盘交易中稳健解通常优于最优解，并阐述过拟合的风险。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "最优解",
          "en": "Optimal Solution"
        },
        {
          "display": "稳健解",
          "en": "Robust Solution"
        },
        {
          "display": "过拟合",
          "en": "Overfitting"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "定义最优解和稳健解",
            "解释过拟合及其危害",
            "说明稳健解如何降低过拟合风险"
          ],
          "question_id": "q_long_robust_choice_v1",
          "reference_answer": [
            "最优解是在历史回测中表现最好的参数组合，但它容易过拟合，即过度学习了历史数据中的噪音而非真实规律。",
            "过拟合的策略在实盘新数据上会表现很差，因为噪音不具有预测性。",
            "稳健解虽然回测收益不是最高，但它对参数变化不敏感，在不同市场环境下表现稳定。",
            "因此，稳健解降低了实盘失效的风险，是更可靠的选择。"
          ],
          "rubric_points": [
            "准确区分最优解和稳健解（2分）",
            "正确解释过拟合的定义（2分）",
            "阐述过拟合导致最优解实盘失效的原因（2分）",
            "说明稳健解通过牺牲部分回测收益换取实盘稳定性的逻辑（2分）",
            "逻辑清晰，表述完整（2分）"
          ],
          "stem": "请解释为什么在实盘交易中，一个稳健解通常比一个在回测中表现最好的最优解更受青睐。你的回答应包含对过拟合的解释。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "指出A组和B组分别对应最优解还是稳健解",
            "解释参数敏感性分析的结果意味着什么",
            "结合过拟合风险说明选择理由"
          ],
          "question_id": "q_long_robust_choice_v2",
          "reference_answer": [
            "A组是回测中的最优解，B组是稳健解。",
            "A组对参数微调非常敏感，说明它可能过拟合了历史数据中的特定模式，实盘风险高。",
            "B组对参数变化不敏感，表明其在不同市场条件下表现更稳定，是更稳健的选择。",
            "因此，我会选择B组进行实盘，因为它虽然牺牲了部分回测收益，但大大降低了实盘失效的风险。"
          ],
          "rubric_points": [
            "正确识别A组为最优解，B组为稳健解（2分）",
            "解释A组参数敏感意味着过拟合风险高（2分）",
            "解释B组参数稳定意味着更稳健，实盘失效风险低（2分）",
            "做出选择并给出合理理由（2分）",
            "逻辑清晰，表述完整（2分）"
          ],
          "stem": "假设你优化一个移动平均线交叉策略，得到了两组参数：A组（快线10，慢线50）回测年化收益35%，B组（快线20，慢线100）回测年化收益22%。但参数敏感性分析显示，A组收益对参数微调非常敏感，而B组则相对稳定。你会选择哪组参数实盘？请结合最优解、稳健解和过拟合的概念说明理由。"
        }
      ]
    },
    {
      "concept_key": "walk_forward_validation",
      "coverage_tags": [
        "walk_forward_validation",
        "parameter_sensitivity_analysis"
      ],
      "difficulty": "hard",
      "family_id": "qf_long_walk_forward_example",
      "learning_goal": "学生能描述滚动窗口验证的步骤，并解释其如何帮助选择稳健解。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "mechanism_trace",
      "term_refs": [
        {
          "display": "滚动窗口验证",
          "en": "Walk-Forward Validation"
        },
        {
          "display": "参数敏感性分析",
          "en": "Parameter Sensitivity Analysis"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "描述滚动窗口验证的步骤（数据划分、滚动测试）",
            "解释如何根据各段结果选择参数",
            "说明该方法如何降低过拟合风险"
          ],
          "question_id": "q_long_walk_forward_example_v1",
          "reference_answer": [
            "滚动窗口验证的步骤：首先将历史数据（如5年）分成多个连续的子集（如5个1年段）。然后，对每个参数组合，分别计算其在每个子集上的表现。",
            "选择参数时，不是选择在某个子集上表现最好的，而是选择在所有子集上都表现中等以上且收益波动最小的参数组合。",
            "这种方法通过在不同市场周期（如牛市、熊市、震荡市）上测试策略，可以检验其是否具有普适性。",
            "如果一个参数只在特定年份表现好，而在其他年份表现差，它很可能是过拟合的。滚动窗口验证能有效识别并排除这类参数，从而引导我们选择更稳健的解。"
          ],
          "rubric_points": [
            "正确描述数据划分和滚动测试的过程（3分）",
            "解释选择在各段表现一致的参数而非单段最优参数（3分）",
            "说明该方法通过在不同市场周期验证，降低了选择过拟合参数的风险（2分）",
            "逻辑清晰，表述完整（2分）"
          ],
          "stem": "描述滚动窗口验证（Walk-Forward Validation）的基本步骤，并解释它为什么能帮助交易者选择一个更稳健的策略参数组合，而不是一个可能过拟合的最优解。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "设计数据划分方案",
            "描述测试流程",
            "说明参数选择标准"
          ],
          "question_id": "q_long_walk_forward_example_v2",
          "reference_answer": [
            "我会将5年数据按年份分成5个独立的子集：Year1, Year2, ..., Year5。",
            "对于每一个候选的参数组合，我会分别在Year1到Year5上运行回测，记录每年的收益、夏普比率等关键指标。",
            "然后，我会分析每个参数组合在5年中的表现：计算平均收益和收益的标准差。",
            "最终，我会选择平均收益尚可（不一定是最高）且收益标准差最小的参数组合。这样的组合在历史上表现最稳定，最有可能在未来也保持稳健。"
          ],
          "rubric_points": [
            "设计合理的数据划分方案（如5个1年段）（2分）",
            "描述对每个参数组合在所有段上进行测试的流程（3分）",
            "解释选择在各段表现稳定且中等以上的参数组合（3分）",
            "逻辑清晰，表述完整（2分）"
          ],
          "stem": "假设你有一个5年的数据集，你想使用滚动窗口验证来优化一个策略的两个参数。请详细说明你将如何设计这个验证过程，以及你将如何根据结果选择最终的参数组合。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "optimal_vs_robust",
      "coverage_tags": [
        "optimal_vs_robust"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_optimal_vs_robust",
      "learning_goal": "学生能在具体场景中判断应选择最优解还是稳健解。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "最优解",
          "en": "Optimal Solution"
        },
        {
          "display": "稳健解",
          "en": "Robust Solution"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 30,
          "explanation": "参数敏感的最优解容易过拟合，实盘风险高。应选择对参数变化不敏感的稳健解，以确保实盘表现稳定。",
          "options": [
            "直接部署该最优参数，因为回测收益最高",
            "寻找一个在参数微调时收益变化更小的稳健参数组合",
            "增加参数数量，以进一步提高回测收益",
            "放弃该策略，因为参数敏感意味着策略无效"
          ],
          "question_id": "q_quiz_optimal_vs_robust_v1",
          "stem": "一个量化策略在回测中获得了最高的年化收益，但参数敏感性分析显示，参数微调会导致收益大幅波动。在实盘部署时，最合理的做法是："
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "稳健解牺牲了部分回测收益，以换取在不同市场环境下的稳定表现，从而降低了实盘失效的风险。",
          "options": [
            "最优解在实盘中一定比稳健解表现更好",
            "稳健解在回测中的收益通常低于最优解，但在实盘中更可靠",
            "最优解和稳健解在实盘中的表现没有区别",
            "稳健解意味着策略参数是固定不变的"
          ],
          "question_id": "q_quiz_optimal_vs_robust_v2",
          "stem": "以下关于最优解和稳健解的描述，哪一项是正确的？"
        }
      ]
    },
    {
      "concept_key": "overfitting",
      "coverage_tags": [
        "overfitting_definition"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_overfitting",
      "learning_goal": "学生能识别过拟合的迹象及其后果。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "过拟合",
          "en": "Overfitting"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 25,
          "explanation": "回测表现极好但实盘表现急剧恶化，是过拟合的典型特征。策略过度适应了历史数据中的噪音。",
          "options": [
            "市场风格发生了根本性变化",
            "策略存在过拟合，学到了历史噪音而非真实规律",
            "交易成本计算错误",
            "实盘数据质量不如回测数据"
          ],
          "question_id": "q_quiz_overfitting_v1",
          "stem": "一个策略在3年历史回测中年化收益达到45%，但在随后1年的实盘测试中亏损10%。最可能的原因是："
        },
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "在大量参数组合中反复回测并选择历史表现最好的，很容易找到恰好拟合历史噪音的参数，导致过拟合。",
          "options": [
            "使用较少的优化参数",
            "在优化时使用滚动窗口验证",
            "在大量参数组合中反复回测，选择历史表现最好的那一个",
            "优先选择稳健解而非最优解"
          ],
          "question_id": "q_quiz_overfitting_v2",
          "stem": "以下哪种做法最有可能导致策略过拟合？"
        }
      ]
    },
    {
      "concept_key": "robust_solution",
      "coverage_tags": [
        "robust_solution_definition"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_robust_solution",
      "learning_goal": "学生能理解稳健解的特征和选择理由。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "稳健解",
          "en": "Robust Solution"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "回测收益最高是最优解的特征，而非稳健解。稳健解可能牺牲部分收益以换取稳定性。",
          "options": [
            "在不同市场环境下表现稳定",
            "回测收益在所有参数组合中最高",
            "对参数微调不敏感",
            "在滚动窗口验证的各段数据上表现一致"
          ],
          "question_id": "q_quiz_robust_solution_v1",
          "stem": "在选择稳健策略时，以下哪一项不是稳健解的特征？"
        },
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "稳健解的核心优势在于其泛化能力和稳定性，能有效降低因过拟合导致的实盘失效风险。",
          "options": [
            "因为稳健解的计算速度更快",
            "因为稳健解能保证在任何市场条件下都盈利",
            "因为稳健解降低了过拟合风险，在不同市场条件下表现更稳定",
            "因为稳健解需要的参数更少"
          ],
          "question_id": "q_quiz_robust_solution_v2",
          "stem": "为什么在实盘交易中，稳健解通常比最优解更受青睐？"
        }
      ]
    },
    {
      "concept_key": "parameter_sensitivity_analysis",
      "coverage_tags": [
        "parameter_sensitivity_analysis"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_parameter_sensitivity",
      "learning_goal": "学生能理解参数敏感性分析在评估策略稳健性中的作用。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "参数敏感性分析",
          "en": "Parameter Sensitivity Analysis"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 25,
          "explanation": "收益对参数微调过于敏感，说明策略可能过拟合了特定参数值，不是一个稳健的策略。",
          "options": [
            "策略非常稳健，因为收益对参数变化敏感",
            "策略可能不稳健，因为收益对参数微调过于敏感",
            "参数 (11, 51) 是更好的选择",
            "需要增加更多参数来稳定收益"
          ],
          "question_id": "q_quiz_parameter_sensitivity_v1",
          "stem": "一个策略在参数 (10, 50) 时年化收益为20%，当参数微调为 (11, 51) 时，年化收益骤降至5%。这个现象说明："
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "参数敏感性分析旨在评估策略对参数变化的敏感程度，并找出能产生稳定结果的参数区域，这是选择稳健解的关键步骤。",
          "options": [
            "找到使回测收益最高的精确参数值",
            "评估策略性能如何随参数变化，并识别稳定区域",
            "减少策略中的参数数量",
            "计算所有参数组合的平均收益"
          ],
          "question_id": "q_quiz_parameter_sensitivity_v2",
          "stem": "参数敏感性分析的主要目的是："
        }
      ]
    },
    {
      "concept_key": "walk_forward_validation",
      "coverage_tags": [
        "walk_forward_validation"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_walk_forward",
      "learning_goal": "学生能理解滚动窗口验证的原理和目的。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "滚动窗口验证",
          "en": "Walk-Forward Validation"
        }
      ],
      "variants": [
        {
          "answer": 3,
          "estimated_seconds": 25,
          "explanation": "稳健解要求在不同市场环境下表现一致。选择每年都表现不错且波动小的参数，能更好地应对未来未知的市场状况。",
          "options": [
            "选择在第1年表现最好的参数",
            "选择在第5年表现最好的参数",
            "选择在5年中平均收益最高的参数",
            "选择在5年中每年都表现中等以上且收益波动最小的参数"
          ],
          "question_id": "q_quiz_walk_forward_v1",
          "stem": "在滚动窗口验证中，将5年数据分成5份，每份1年。以下哪种参数选择策略最符合稳健性原则？"
        },
        {
          "answer": 1,
          "estimated_seconds": 25,
          "explanation": "滚动窗口验证通过在不同时间段上测试策略，可以评估其在不同市场环境下的表现一致性，从而更有效地识别过拟合和选择稳健解。",
          "options": [
            "计算速度更快",
            "能更好地检验策略在不同市场周期下的稳健性",
            "总能找到收益更高的参数组合",
            "不需要定义优化目标"
          ],
          "question_id": "q_quiz_walk_forward_v2",
          "stem": "滚动窗口验证与仅在全部历史数据上做一次回测相比，主要优势是什么？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L9 coverage checklist (step3 focus: optimal vs robust solution, overfitting, parameter sensitivity, walk-forward validation)",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "L9 lesson map (step3: 最优解 vs 稳健解：为什么稳健更重要)",
    "plain_text": "pipeline/1-plain/L9/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L9 source outline (step3 focus: Optimal vs Robust Solution)"
  },
  "target_language": "zh-CN"
}
,
{
  "coverage_map": [
    {
      "coverage_tag": "optimization_methods_overview",
      "covered_by": [
        "qf_flash_opt_methods",
        "qf_quiz_opt_methods_choice",
        "qf_long_opt_methods_compare"
      ],
      "description": "三种常见优化方法：暴力搜索、梯度下降、遗传算法"
    },
    {
      "coverage_tag": "brute_force_search",
      "covered_by": [
        "qf_flash_brute_force",
        "qf_quiz_brute_force"
      ],
      "description": "暴力搜索：遍历所有可能组合，可靠但计算量大"
    },
    {
      "coverage_tag": "gradient_descent",
      "covered_by": [
        "qf_flash_gradient_descent",
        "qf_quiz_gradient_descent",
        "qf_long_gradient_descent_formula"
      ],
      "description": "梯度下降：沿最陡下坡方向迭代，更新规则 x_{n+1} = x_n - α·∇f(x_n)"
    },
    {
      "coverage_tag": "genetic_algorithm",
      "covered_by": [
        "qf_flash_genetic_algorithm",
        "qf_quiz_genetic_algorithm"
      ],
      "description": "遗传算法：模拟自然选择，通过选择、交叉、变异进化"
    },
    {
      "coverage_tag": "method_limitations",
      "covered_by": [
        "qf_flash_limitations",
        "qf_quiz_limitations"
      ],
      "description": "各方法的局限：暴力搜索参数多时计算爆炸；梯度下降对初始值和学习率敏感，可能局部最优；遗传算法不保证全局最优"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "optimization_methods_overview",
      "coverage_tags": [
        "optimization_methods_overview"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_opt_methods",
      "learning_goal": "学生能列举并区分三种常见的策略优化方法。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "三种优化方法的名称与核心思想",
      "term_refs": [
        {
          "display": "策略优化方法",
          "en": "Optimization Methods"
        }
      ],
      "variants": [
        {
          "back": "暴力搜索（网格搜索）、梯度下降、遗传算法。",
          "estimated_seconds": 8,
          "explanation": "这三种方法各有优劣，适用于不同场景。",
          "front": "策略优化中，三种常见的优化方法是什么？",
          "question_id": "q_flash_opt_methods_v1"
        },
        {
          "back": "暴力搜索（网格搜索）。",
          "estimated_seconds": 6,
          "explanation": "暴力搜索最直接，但参数多时计算量会爆炸式增长。",
          "front": "哪种优化方法通过遍历所有可能的参数组合来寻找最优解？",
          "question_id": "q_flash_opt_methods_v2"
        },
        {
          "back": "遗传算法。",
          "estimated_seconds": 6,
          "explanation": "遗传算法通过选择、交叉、变异等操作一代代进化出更好的解。",
          "front": "哪种优化方法模拟了“物竞天择，适者生存”的进化过程？",
          "question_id": "q_flash_opt_methods_v3"
        }
      ]
    },
    {
      "concept_key": "brute_force_search",
      "coverage_tags": [
        "brute_force_search"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_brute_force",
      "learning_goal": "学生能说出暴力搜索的定义、优点和缺点。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "暴力搜索的核心特征",
      "term_refs": [
        {
          "display": "暴力搜索",
          "en": "Brute Force Search"
        }
      ],
      "variants": [
        {
          "back": "简单直接，能保证找到全局最优解（如果解空间是离散且有限的）。",
          "estimated_seconds": 8,
          "explanation": "因为它遍历了所有可能的组合。",
          "front": "暴力搜索的主要优点是什么？",
          "question_id": "q_flash_brute_force_v1"
        },
        {
          "back": "当参数很多时，计算量会爆炸式增长，变得不可行。",
          "estimated_seconds": 8,
          "explanation": "参数每增加一个，组合数可能呈指数级增长。",
          "front": "暴力搜索的主要缺点是什么？",
          "question_id": "q_flash_brute_force_v2"
        }
      ]
    },
    {
      "concept_key": "gradient_descent",
      "coverage_tags": [
        "gradient_descent"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_gradient_descent",
      "learning_goal": "学生能说出梯度下降的更新规则、关键参数和比喻。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "梯度下降的更新公式和关键参数含义",
      "term_refs": [
        {
          "display": "梯度下降",
          "en": "Gradient Descent"
        }
      ],
      "variants": [
        {
          "back": "$x_{n+1} = x_n - \\alpha \\cdot \\nabla f(x_n)$",
          "estimated_seconds": 10,
          "explanation": "下一个参数值等于当前值减去学习率乘以梯度。",
          "front": "梯度下降的更新规则是什么？",
          "question_id": "q_flash_gradient_descent_v1"
        },
        {
          "back": "学习率，控制每一步更新的大小。",
          "estimated_seconds": 6,
          "explanation": "学习率太大可能震荡不收敛，太小则收敛慢。",
          "front": "在梯度下降中，$\\alpha$ 代表什么？",
          "question_id": "q_flash_gradient_descent_v2"
        },
        {
          "back": "梯度，指向函数上升最快的方向。",
          "estimated_seconds": 6,
          "explanation": "梯度下降沿着梯度的反方向（最陡下坡方向）移动。",
          "front": "在梯度下降中，$\\nabla f(x_n)$ 代表什么？",
          "question_id": "q_flash_gradient_descent_v3"
        },
        {
          "back": "像一个下山的人，每次都沿着最陡的下坡路走一步。",
          "estimated_seconds": 6,
          "explanation": "这个比喻形象地说明了梯度下降的迭代过程。",
          "front": "梯度下降被比喻成什么？",
          "question_id": "q_flash_gradient_descent_v4"
        }
      ]
    },
    {
      "concept_key": "genetic_algorithm",
      "coverage_tags": [
        "genetic_algorithm"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_genetic_algorithm",
      "learning_goal": "学生能说出遗传算法的核心步骤和灵感来源。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "遗传算法的核心步骤",
      "term_refs": [
        {
          "display": "遗传算法",
          "en": "Genetic Algorithm"
        }
      ],
      "variants": [
        {
          "back": "自然选择和生物进化过程（物竞天择，适者生存）。",
          "estimated_seconds": 6,
          "explanation": "它模拟了生物进化中的选择、交叉和变异。",
          "front": "遗传算法的灵感来源于什么？",
          "question_id": "q_flash_genetic_algorithm_v1"
        },
        {
          "back": "选择、交叉（杂交）、变异。",
          "estimated_seconds": 8,
          "explanation": "选择保留优秀个体，交叉组合产生后代，变异引入多样性。",
          "front": "遗传算法中，通过哪三个主要操作来产生新一代的解？",
          "question_id": "q_flash_genetic_algorithm_v2"
        }
      ]
    },
    {
      "concept_key": "method_limitations",
      "coverage_tags": [
        "method_limitations"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_limitations",
      "learning_goal": "学生能说出每种优化方法的主要局限性。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "各优化方法的局限性",
      "term_refs": [
        {
          "display": "方法局限性",
          "en": "Method Limitations"
        }
      ],
      "variants": [
        {
          "back": "1. 对初始值和学习率很敏感；2. 可能只找到局部最优解。",
          "estimated_seconds": 10,
          "explanation": "初始值和学习率选择不当会导致不收敛或陷入局部最优。",
          "front": "梯度下降的两个主要局限性是什么？",
          "question_id": "q_flash_limitations_v1"
        },
        {
          "back": "不保证找到全局最优解。",
          "estimated_seconds": 6,
          "explanation": "遗传算法擅长探索广阔的解空间，但无法保证收敛到全局最优。",
          "front": "遗传算法的主要局限性是什么？",
          "question_id": "q_flash_limitations_v2"
        }
      ]
    }
  ],
  "lesson_id": "L9",
  "longform_families": [
    {
      "concept_key": "optimization_methods_overview",
      "coverage_tags": [
        "optimization_methods_overview",
        "method_limitations"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_opt_methods_compare",
      "learning_goal": "学生能比较三种优化方法的工作原理、适用场景和局限性。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "compare_and_contrast",
      "term_refs": [
        {
          "display": "优化方法比较",
          "en": "Comparison of Optimization Methods"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "暴力搜索的工作原理、适用场景和局限性",
            "梯度下降的工作原理、适用场景和局限性",
            "遗传算法的工作原理、适用场景和局限性"
          ],
          "question_id": "q_long_opt_methods_compare_v1",
          "reference_answer": [
            "暴力搜索通过枚举所有可能的参数组合来寻找最优解。它最适用于参数数量少、解空间有限且需要精确全局最优的场景。其主要局限性是当参数增多时，计算量会呈指数级增长，变得不可行。",
            "梯度下降通过计算目标函数的梯度，并沿着梯度反方向（最陡下降方向）迭代更新参数来寻找最小值。它适用于目标函数连续可微且参数空间较大的场景。其主要局限性是对初始值和学习率的选择非常敏感，并且可能只找到局部最优解而非全局最优。",
            "遗传算法模拟自然选择和生物进化过程。它从一个随机初始化的种群开始，通过评估适应度、选择、交叉和变异等操作，一代代进化出更好的解。它适用于解空间广阔、复杂、存在多个局部最优的场景。其主要局限性是不保证找到全局最优解，且收敛速度可能较慢。"
          ],
          "rubric_points": [
            "暴力搜索：遍历所有组合；参数少且需要精确最优解时适用；参数多时计算爆炸",
            "梯度下降：沿梯度反方向迭代；连续可微函数优化时适用；对初始值和学习率敏感，可能局部最优",
            "遗传算法：模拟进化（选择、交叉、变异）；解空间广阔复杂时适用；不保证全局最优"
          ],
          "stem": "请比较暴力搜索、梯度下降和遗传算法这三种优化方法。从以下三个方面进行阐述：\n1. 每种方法的核心工作原理\n2. 每种方法最适用的场景\n3. 每种方法的主要局限性"
        },
        {
          "estimated_seconds": 150,
          "prompt_blocks": [
            "梯度下降不适合的原因",
            "推荐的方法及理由",
            "推荐方法的执行步骤"
          ],
          "question_id": "q_long_opt_methods_compare_v2",
          "reference_answer": [
            "梯度下降不适合，因为移动平均线的周期参数是离散整数，目标函数（如总收益）关于这些参数不是连续可微的，无法计算梯度。梯度下降要求目标函数连续可微。",
            "我推荐使用暴力搜索。理由：参数范围虽然看起来大，但快线有46种可能（5到50），慢线有151种可能（50到200），总共只有46×151=6946种组合。这个数量级对于现代计算机来说完全可以在合理时间内遍历完，并且暴力搜索能保证找到全局最优解。",
            "暴力搜索的执行步骤：1) 定义优化目标，例如最大化年化收益率；2) 定义参数网格，快线周期从5到50，慢线周期从50到200；3) 对于每一组参数组合（快线，慢线），运行一次回测，计算目标指标；4) 比较所有组合的结果，选择目标指标最优的那组参数。"
          ],
          "rubric_points": [
            "梯度下降不适合：参数是离散整数，函数不可微，梯度无法定义",
            "暴力搜索：参数范围有限（46*151=6946种组合），可遍历所有，保证全局最优",
            "遗传算法：如果参数范围更大或需要更快速度，遗传算法也可行，但不保证最优"
          ],
          "stem": "假设你正在优化一个移动平均线交叉策略，有两个参数：快线周期（整数，范围5-50）和慢线周期（整数，范围50-200）。\n1. 解释为什么在这种情况下使用梯度下降可能不是最佳选择。\n2. 你会推荐使用暴力搜索还是遗传算法？请说明理由。\n3. 如果使用你推荐的方法，请简述其执行步骤。"
        }
      ]
    },
    {
      "concept_key": "gradient_descent",
      "coverage_tags": [
        "gradient_descent"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_gradient_descent_formula",
      "learning_goal": "学生能应用梯度下降更新公式进行手动计算。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "formula_apply",
      "term_refs": [
        {
          "display": "梯度下降更新规则",
          "en": "Gradient Descent Update Rule"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "写出第一次迭代的计算过程",
            "写出第二次迭代的计算过程"
          ],
          "question_id": "q_long_gradient_descent_formula_v1",
          "reference_answer": [
            "第一次迭代：$x_1 = x_0 - \\alpha \\cdot \\nabla f(x_0) = 10 - 0.2 \\times (2 \\times 10) = 10 - 0.2 \\times 20 = 10 - 4 = 6$",
            "第二次迭代：$x_2 = x_1 - \\alpha \\cdot \\nabla f(x_1) = 6 - 0.2 \\times (2 \\times 6) = 6 - 0.2 \\times 12 = 6 - 2.4 = 3.6$"
          ],
          "rubric_points": [
            "正确计算 x1 = 10 - 0.2 * 20 = 6",
            "正确计算 x2 = 6 - 0.2 * 12 = 3.6"
          ],
          "stem": "给定函数 $f(x) = x^2$，其梯度为 $\\nabla f(x) = 2x$。假设初始点 $x_0 = 10$，学习率 $\\alpha = 0.2$。请使用梯度下降更新公式 $x_{n+1} = x_n - \\alpha \\cdot \\nabla f(x_n)$ 进行两次迭代，计算出 $x_1$ 和 $x_2$ 的值。"
        },
        {
          "estimated_seconds": 100,
          "prompt_blocks": [
            "写出三次迭代的计算过程",
            "描述 x 值的变化趋势"
          ],
          "question_id": "q_long_gradient_descent_formula_v2",
          "reference_answer": [
            "第一次迭代：$x_1 = 8 - 0.1 \\times (2 \\times 8) = 8 - 0.1 \\times 16 = 8 - 1.6 = 6.4$",
            "第二次迭代：$x_2 = 6.4 - 0.1 \\times (2 \\times 6.4) = 6.4 - 0.1 \\times 12.8 = 6.4 - 1.28 = 5.12$",
            "第三次迭代：$x_3 = 5.12 - 0.1 \\times (2 \\times 5.12) = 5.12 - 0.1 \\times 10.24 = 5.12 - 1.024 = 4.096$",
            "变化趋势：$x$ 的值从 8 开始，经过三次迭代后减小到 4.096，每一步都在向函数 $f(x)=x^2$ 的最小值点 $x=0$ 靠近，但步长逐渐变小。"
          ],
          "rubric_points": [
            "正确计算 x1 = 8 - 0.1 * 16 = 6.4",
            "正确计算 x2 = 6.4 - 0.1 * 12.8 = 5.12",
            "正确计算 x3 = 5.12 - 0.1 * 10.24 = 4.096",
            "观察到 x 值逐渐减小，向函数最小值点 x=0 靠近"
          ],
          "stem": "给定函数 $f(x) = x^2$，其梯度为 $\\nabla f(x) = 2x$。假设初始点 $x_0 = 8$，学习率 $\\alpha = 0.1$。请使用梯度下降更新公式 $x_{n+1} = x_n - \\alpha \\cdot \\nabla f(x_n)$ 进行三次迭代，计算出 $x_1$、$x_2$ 和 $x_3$ 的值，并观察 $x$ 的变化趋势。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "optimization_methods_overview",
      "coverage_tags": [
        "optimization_methods_overview"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_opt_methods_choice",
      "learning_goal": "学生能根据场景选择最合适的优化方法。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "优化方法选择",
          "en": "Optimization Method Selection"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "参数组合很少（25种），暴力搜索可以遍历所有可能，保证找到全局最优解。",
          "options": [
            "暴力搜索",
            "梯度下降",
            "遗传算法",
            "随机搜索"
          ],
          "question_id": "q_quiz_opt_methods_choice_v1",
          "stem": "一个交易策略只有2个参数，每个参数只有5个可能取值。你需要精确找到全局最优解。以下哪种方法最合适？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "梯度下降利用梯度信息，在连续可微的大参数空间中能高效地找到局部最优解。",
          "options": [
            "暴力搜索",
            "梯度下降",
            "遗传算法",
            "随机猜测"
          ],
          "question_id": "q_quiz_opt_methods_choice_v2",
          "stem": "一个优化问题的目标函数是连续可微的，但参数空间很大。你希望快速找到一个不错的解。以下哪种方法最合适？"
        },
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "遗传算法通过种群和变异机制，擅长在广阔复杂的解空间中探索，不易陷入局部最优。",
          "options": [
            "暴力搜索",
            "梯度下降",
            "遗传算法",
            "爬山法"
          ],
          "question_id": "q_quiz_opt_methods_choice_v3",
          "stem": "一个优化问题的参数是离散的，且解空间非常广阔、复杂，存在多个局部最优。你希望探索全局并避免过早陷入局部最优。以下哪种方法最合适？"
        }
      ]
    },
    {
      "concept_key": "brute_force_search",
      "coverage_tags": [
        "brute_force_search"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_brute_force",
      "learning_goal": "学生能判断关于暴力搜索的描述是否正确。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "true_false",
      "term_refs": [
        {
          "display": "暴力搜索",
          "en": "Brute Force Search"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "暴力搜索的计算量通常随参数数量呈指数级增长，而不是线性增长。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_brute_force_v1",
          "stem": "暴力搜索在参数数量增加时，计算量会线性增长。"
        },
        {
          "answer": 0,
          "estimated_seconds": 15,
          "explanation": "因为暴力搜索会遍历所有可能的参数组合，所以一定能找到全局最优解。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_brute_force_v2",
          "stem": "暴力搜索可以保证找到全局最优解（如果解空间是离散且有限的）。"
        }
      ]
    },
    {
      "concept_key": "gradient_descent",
      "coverage_tags": [
        "gradient_descent"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_gradient_descent",
      "learning_goal": "学生能理解梯度下降的更新规则和关键参数的作用。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "梯度下降",
          "en": "Gradient Descent"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "梯度 $\\nabla f(x_n)$ 指向函数值上升最快的方向，为了最小化函数，我们需要向反方向移动，所以用减号。",
          "options": [
            "因为梯度指向函数上升最快的方向，减号使其向反方向（下降方向）移动",
            "因为梯度指向函数下降最快的方向，减号使其加速下降",
            "因为学习率 $\\alpha$ 通常是负数",
            "这是一个数学惯例，没有特殊含义"
          ],
          "question_id": "q_quiz_gradient_descent_v1",
          "stem": "在梯度下降更新公式 $x_{n+1} = x_n - \\alpha \\cdot \\nabla f(x_n)$ 中，为什么是减号？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "学习率过大可能导致每一步跨越太大，在最优值附近来回震荡，甚至发散不收敛。",
          "options": [
            "收敛速度变慢",
            "可能震荡甚至不收敛",
            "一定能更快找到全局最优",
            "对结果没有影响"
          ],
          "question_id": "q_quiz_gradient_descent_v2",
          "stem": "如果梯度下降的学习率 $\\alpha$ 设置得过大，可能会导致什么后果？"
        }
      ]
    },
    {
      "concept_key": "genetic_algorithm",
      "coverage_tags": [
        "genetic_algorithm"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_genetic_algorithm",
      "learning_goal": "学生能正确排列遗传算法的主要步骤。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "ordering",
      "term_refs": [
        {
          "display": "遗传算法步骤",
          "en": "Genetic Algorithm Steps"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 25,
          "explanation": "正确的顺序是：初始化种群 → 评估适应度 → 选择 → 交叉 → 变异。",
          "options": [
            "B → E → C → D → A",
            "B → C → D → A → E",
            "E → B → C → D → A",
            "B → E → D → C → A"
          ],
          "question_id": "q_quiz_genetic_algorithm_v1",
          "stem": "请将遗传算法的以下步骤按正确顺序排列：\nA. 变异\nB. 初始化种群\nC. 选择\nD. 交叉\nE. 评估适应度"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "变异通过随机改变个体的某些特征，引入新的遗传信息，帮助算法探索更广阔的解空间，避免早熟收敛。",
          "options": [
            "保留当前最优解",
            "维持种群的多样性，防止过早收敛到局部最优",
            "选择适应度最高的个体",
            "组合两个父代个体的信息"
          ],
          "question_id": "q_quiz_genetic_algorithm_v2",
          "stem": "在遗传算法中，'变异'操作的主要目的是什么？"
        }
      ]
    },
    {
      "concept_key": "method_limitations",
      "coverage_tags": [
        "method_limitations"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_limitations",
      "learning_goal": "学生能识别不同优化方法的共同和特有局限性。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "multiple_choice",
      "term_refs": [
        {
          "display": "方法局限性",
          "en": "Method Limitations"
        }
      ],
      "variants": [
        {
          "answer": [
            0
          ],
          "estimated_seconds": 25,
          "explanation": "梯度下降和遗传算法都不能保证找到全局最优解。对初始值敏感是梯度下降的特点；连续可微是梯度下降的要求；计算量指数增长是暴力搜索的特点。",
          "options": [
            "不保证找到全局最优解",
            "对初始值敏感",
            "要求目标函数连续可微",
            "计算量随参数增加而指数增长"
          ],
          "question_id": "q_quiz_limitations_v1",
          "stem": "以下哪些是梯度下降和遗传算法共同的局限性？（多选）"
        },
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "遗传算法是一种启发式算法，它通过模拟进化来搜索，但不能保证收敛到全局最优解。",
          "options": [
            "暴力搜索在参数很多时仍然非常高效",
            "梯度下降适用于任何类型的函数优化",
            "遗传算法不保证找到全局最优解",
            "所有优化方法都能保证找到全局最优解"
          ],
          "question_id": "q_quiz_limitations_v2",
          "stem": "以下关于优化方法局限性的描述，哪一个是正确的？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L9/plain.txt",
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
      "coverage_tag": "strategy_optimization_definition",
      "covered_by": [
        "qf_flash_stratopt_def",
        "qf_quiz_stratopt_firststep"
      ],
      "description": "策略优化的定义：调整交易算法参数以最大化绩效指标的过程"
    },
    {
      "coverage_tag": "optimization_objective",
      "covered_by": [
        "qf_flash_opt_objective",
        "qf_quiz_opt_objective_common"
      ],
      "description": "优化目标：最大化年化收益、夏普比率、最小化最大回撤，以及自定义目标函数"
    },
    {
      "coverage_tag": "optimization_process_step1",
      "covered_by": [
        "qf_quiz_stratopt_firststep",
        "qf_flash_stratopt_def"
      ],
      "description": "策略优化流程第一步：定义优化目标"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "strategy_optimization",
      "coverage_tags": [
        "strategy_optimization_definition",
        "optimization_process_step1"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_stratopt_def",
      "learning_goal": "学生能准确说出策略优化的定义及其核心目的。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "策略优化的定义和目的",
      "term_refs": [
        {
          "display": "策略优化",
          "en": "Strategy Optimization"
        }
      ],
      "variants": [
        {
          "back": "调整交易算法的参数，以最大化收益、夏普比率等绩效指标的过程。",
          "estimated_seconds": 8,
          "explanation": "策略优化就是通过调整参数来让策略在历史数据上表现更好。",
          "front": "什么是策略优化？",
          "question_id": "q_flash_stratopt_def_v1"
        },
        {
          "back": "让策略在历史数据上表现更好，从而提高实盘决策质量。",
          "estimated_seconds": 8,
          "explanation": "优化后的策略更有可能在实盘中获得良好表现。",
          "front": "策略优化的主要目的是什么？",
          "question_id": "q_flash_stratopt_def_v2"
        }
      ]
    },
    {
      "concept_key": "optimization_objective",
      "coverage_tags": [
        "optimization_objective"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_opt_objective",
      "learning_goal": "学生能列举常见的优化目标并理解其含义。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "常见的优化目标",
      "term_refs": [
        {
          "display": "优化目标",
          "en": "Optimization Objective"
        },
        {
          "display": "夏普比率",
          "en": "Sharpe Ratio"
        },
        {
          "display": "最大回撤",
          "en": "Max Drawdown"
        }
      ],
      "variants": [
        {
          "back": "最大化年化收益、最大化夏普比率、最小化最大回撤。",
          "estimated_seconds": 10,
          "explanation": "这三个指标分别关注收益、风险调整后收益和下行风险。",
          "front": "列举三个常见的策略优化目标。",
          "question_id": "q_flash_opt_objective_v1"
        },
        {
          "back": "用户自定义函数，例如将年化收益、胜率、交易次数和风险组合成一个综合指标。",
          "estimated_seconds": 10,
          "explanation": "自定义目标函数可以更灵活地反映策略设计者的偏好。",
          "front": "除了常见指标，优化目标还可以是什么？",
          "question_id": "q_flash_opt_objective_v2"
        }
      ]
    }
  ],
  "lesson_id": "L9",
  "longform_families": [
    {
      "concept_key": "strategy_optimization",
      "coverage_tags": [
        "strategy_optimization_definition",
        "optimization_objective"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_stratopt_explain",
      "learning_goal": "学生能用自己的话解释策略优化的含义、目的和常见目标。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "策略优化",
          "en": "Strategy Optimization"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 60,
          "prompt_blocks": [
            "策略优化的定义",
            "主要目的"
          ],
          "question_id": "q_long_stratopt_explain_v1",
          "reference_answer": [
            "策略优化是调整交易算法参数的过程，目的是最大化收益、夏普比率等绩效指标。",
            "它帮助策略在历史数据上表现更好，从而提高在实盘交易中的决策质量。"
          ],
          "rubric_points": [
            "提到调整交易算法参数",
            "提到最大化绩效指标（如收益、夏普比率）",
            "提到基于历史数据改善实盘决策"
          ],
          "stem": "请解释什么是策略优化，并说明它的主要目的。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "常见优化目标",
            "每个目标的关注点"
          ],
          "question_id": "q_long_stratopt_explain_v2",
          "reference_answer": [
            "常见优化目标包括：最大化年化收益（关注总回报）、最大化夏普比率（关注每单位风险获得的超额回报）、最小化最大回撤（关注从峰值到谷底的最大亏损幅度）。",
            "也可以使用自定义目标函数，例如将年化收益、胜率、交易次数和风险组合成一个综合指标。"
          ],
          "rubric_points": [
            "列出至少两个常见目标（年化收益、夏普比率、最大回撤）",
            "说明年化收益关注总回报",
            "说明夏普比率关注风险调整后收益",
            "说明最大回撤关注下行风险"
          ],
          "stem": "请列举策略优化中常见的优化目标，并简要说明每个目标关注什么。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "optimization_process_step1",
      "coverage_tags": [
        "optimization_process_step1",
        "strategy_optimization_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_stratopt_firststep",
      "learning_goal": "学生能识别策略优化的第一步。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "策略优化流程",
          "en": "Strategy Optimization Process"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "首先要明确你想优化什么，比如最大化收益还是最小化回撤，这就是定义优化目标。",
          "options": [
            "运行回测",
            "定义优化目标",
            "选择参数范围",
            "部署到实盘"
          ],
          "question_id": "q_quiz_stratopt_firststep_v1",
          "stem": "策略优化的第一步是什么？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "必须先确定优化目标，后续的参数选择、方法选择、回测迭代才有方向。",
          "options": [
            "选择优化方法",
            "定义优化目标",
            "进行迭代回测",
            "分析优化结果"
          ],
          "question_id": "q_quiz_stratopt_firststep_v2",
          "stem": "在策略优化流程中，以下哪一项是首先需要完成的？"
        }
      ]
    },
    {
      "concept_key": "optimization_objective",
      "coverage_tags": [
        "optimization_objective"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_opt_objective_common",
      "learning_goal": "学生能识别常见的优化目标。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "multiple_choice",
      "term_refs": [
        {
          "display": "优化目标",
          "en": "Optimization Objective"
        },
        {
          "display": "夏普比率",
          "en": "Sharpe Ratio"
        },
        {
          "display": "最大回撤",
          "en": "Max Drawdown"
        }
      ],
      "variants": [
        {
          "answer": [
            0,
            1,
            2
          ],
          "estimated_seconds": 25,
          "explanation": "常见的优化目标包括最大化年化收益、最大化夏普比率和最小化最大回撤。最大化交易次数通常不是直接目标。",
          "options": [
            "最大化年化收益",
            "最大化夏普比率",
            "最小化最大回撤",
            "最大化交易次数"
          ],
          "question_id": "q_quiz_opt_objective_common_v1",
          "stem": "以下哪些是策略优化中常用的优化目标？（多选）"
        },
        {
          "answer": 3,
          "estimated_seconds": 20,
          "explanation": "平均持仓天数是一个描述性统计量，通常不作为优化的直接目标；优化目标更关注收益、风险和风险调整后收益。",
          "options": [
            "年化收益",
            "夏普比率",
            "最大回撤",
            "平均持仓天数"
          ],
          "question_id": "q_quiz_opt_objective_common_v2",
          "stem": "以下哪个指标通常不作为策略优化的直接目标？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "L9: Market practice in strategy optimization, broker selection, and market tricks",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "L9 guided_story steps",
    "plain_text": "pipeline/1-plain/L9/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L9: Market practice in strategy optimization, broker selection, and market tricks"
  },
  "target_language": "zh-CN"
}

]
</QUESTION_BANK>

<PLAIN_TEXT>
# L9: Market practice in strategy optimization, broker selection, and market tricks

Course Code: COMP7415

# Agenda

- Best practice to optimize a trading strategy   
- Introduce common optimization methods   
- Different type of investment funds   
- The winning secret behind top tier hedge funds   
Guidelines to choose a trading broker   
- Algo deployment on ALGOGENE   
- Market tricks by traders and investment firms

# Strategy Optimization

# What is Strategy Optimization?

- Strategy optimization involves fine-tuning trading algorithms to maximize performance metrics like returns, Sharpe ratio, etc.   
- Helps in improving trading strategies based on historical data, leading to better decision-making in live trading.

$$
\underline {{y}} = f (\underline {{x}})
$$

# Strategy Optimization Process

# 1. Define the optimization objective

Commonly used performance metric

Max. annualized return   
Max. Sharpe ratio   
• Min. maximal drawdown

- User defined function (eg. Max. $F = \frac{(Annual\ return)\times(win\ rate)\sqrt{no.of\ trades}}{abs(VaR)}$

Performance Statistics: ①   

<table><tr><td>No. of Tradable Days:</td><td>31</td><td>No. of Win Days:</td><td>16</td><td>No. of Loss Days:</td><td>14</td></tr><tr><td>Win Rate:</td><td>53.3333%</td><td>Max. Consecutive Win Day:</td><td>5</td><td>Max. Consecutive Loss Day:</td><td>3</td></tr><tr><td>Odd Ratio:</td><td>1.1429</td><td>Max. Consecutive Gains:</td><td>457.88</td><td>Max. Consecutive Loss:</td><td>-461.27</td></tr><tr><td>No. of Trades:</td><td>2919</td><td>Average Consecutive Win Day:</td><td>0.93</td><td>Average Consecutive Loss Day:</td><td>0.6</td></tr><tr><td>Total PnL:</td><td>-173.62</td><td>Average Per Trade PnL:</td><td>-0.06</td><td>Average Per Day PnL:</td><td>-5.6</td></tr><tr><td>Mean Daily Return:</td><td>-0.0462%</td><td>Median Daily Return:</td><td>0.1058%</td><td>Mean Annual Return:</td><td>-11.6511%</td></tr><tr><td>Daily Return StdDev:</td><td>1.5529%</td><td>25th percentile Daily Return:</td><td>-0.5793%</td><td>75th percentile Daily Return:</td><td>0.5763%</td></tr><tr><td>Daily Return Downside StdDev:</td><td>1.0594%</td><td>95% 1 day return VaR:</td><td>-2.9652%</td><td>99% 1 day return VaR:</td><td>-4.128%</td></tr><tr><td>Daily Sharpe Ratio:</td><td>-0.0298</td><td>Annual Sharpe Ratio:</td><td>-0.4726</td><td>Max. Drawdown Amount:</td><td>995.9</td></tr><tr><td>Daily Sortino Ratio:</td><td>-0.0436</td><td>Annual Sortino Ratio:</td><td>-0.6928</td><td>Max. Drawdown Percent:</td><td>9.2023%</td></tr><tr><td>Max. Drawdown Duration:</td><td>8</td><td>Average Drawdown Duration:</td><td>2.45</td><td>Annual Volatility:</td><td>24.5542%</td></tr><tr><td>Gross Profit:</td><td>0.0</td><td>Gross Loss:</td><td>0.0</td><td>Profit Factor:</td><td>0.0</td></tr><tr><td>Jensen Alpha:</td><td>0.0</td><td>Beta:</td><td>0.0</td><td>Information Ratio:</td><td>0.0</td></tr><tr><td>Omega Ratio:</td><td>0.0</td><td>Treynor Ratio:</td><td>0.0</td><td>Tail Ratio:</td><td>0.8897</td></tr><tr><td>Calmar Ratio:</td><td>1.2661</td><td>Average Holding Day:</td><td>14.4259</td><td>Annual Turnover Rate:</td><td>0.0%</td></tr></table>

# Strategy Optimization Process

# 2. Define the parameters

- The control set of trading parameters (eg. take profit level, stop loss level, etc) in a trading strategy   
- Fewer parameters is preferred to reduce the chance of overfitting

# 3. Define optimization method

Common methods

i. Brute force search / Grid search   
ii. Gradient descent   
iii. Genetic algorithm

Balance between accuracy and computing time   
- Some methods may not be appropriate due to violation of assumptions

# 4. Iterative backtesting and find the parameter set that achieve the optimal outcome

# Brute force search

- Brute force search is the most straightforward method that finds the best solution by checking all possible solutions.   
- Example: crack the password "12345"

![](images/97ccc5d69c5b306ae7841ad125c8c4fcc9d0f318f8696910deb792d6ac7c8d35.jpg)

import itertools, time

Target password

target_password = "12345"

def brute_force Crack(target):

Start time for performance measurement  
start_time = time.time()

Generate all possible combinations of digits from 0 to 9 for length in range(1, len(target) + 1): for attempt in itertools.product('0123456789', repeat=length): # Join the tuple to form the password attempt attempt_password = ".join(attempt) print(f'Trying password: {attempt_password}')

Check if the attempt matches the target password  
if attempt_password == target:  
    end_time = time.time()  
print(f'Password cracked: {attempt_password}')  
print(f'Time taken: {end_time - start_time:.2f} seconds')  
return attempt_password

print('Password not found.') return None

brute_force Crack(target_password)

# Gradient Descent

- Gradient descent is an optimization algorithm used to find the minimum of a function by iteratively moving toward the steepest descent.   
- The update rule for gradient descent can be expressed as:

$$
x _ {n + 1} = x _ {n} - \alpha \cdot \nabla f (x _ {n})
$$

where

$x_{n}$ is the current value.   
$x_{n + 1}$ is the next value.   
- $\alpha > 0$ is the learning rate   
- $\nabla f(x_{n})$ is the gradient of the function at the current value. (i.e. $1^{\mathrm{st}}$ derivative of $f$ )

- Explanation

- The gradient $\nabla f(x_{n})$ indicates the direction of the steepest ascent. We move in the opposite direction to minimize the function.   
- The learning rate $\alpha$ controls how big of a step we take.   
- For finding the maximum, we can apply gradient descent method to $-f(\cdot)$

# Gradient Descent

![](images/5cfae38c391b3f0b27778019cfd558653c879249dbdfcef52c33ee81ded011e2.jpg)

# Gradient Descent - Example

- Find the minimum of $f(x) = x^{2}$   
Thus, $\nabla f(x) = 2x$   
- Suppose

$x_{0} = 10$   
$\alpha = 0.1$   
- Error tolerance level = 1e-6

![](images/04bd8d214ba9136dd6f58effc1330318586e02494018af96e58a068adb19099c.jpg)

def gradient_descent_with_tolerance(f, df, x0, learning_rate, tolerance):

x = x0 error = float('inf') # Initialize error

while error > tolerance:

x_new = x - learning_rate * df(x) # Update rule

error = abs(x_new - x) # Calculate error

X = X_new

return x

# Function and its derivative

def f(x):

return $x^{**}2$

def df(x):

return $2^{*} \times$

example

initial_x = 10

learning_rate = 0.1

tolerance = 1e-6

result = gradient_descent_with_tolerance(f, df, initial_x, learning_rate, tolerance)

print("Minimum value at x:", result) # Output: Close to 0

# Gradient Descent - Limitation

- The final solution is sensitive to the choice of

- initial point $x_0$   
learning rate $\alpha$

- The optimization function is assumed to be continuous and differentiable   
- No guarantee for convergence to the global optimal solution

![](images/0221e362180e8f631600aed90627c5f98e3e1b4ee3c995851a105013122ece18.jpg)

# Genetic Algorithm

- Genetic algorithms are search heuristics inspired by natural selection and biological evolution that are used to solve optimization and search problems.

![](images/3c6d398b5a3a72e36f1214156b39252bd9f6bd02debc6cc759098d56736dcabd.jpg)

![](images/0aab047f5b108cb635d246a8f1a765be0445413e61851e234bcb6f50272fcb9e.jpg)

# Steps Involved in a Genetic Algorithm

1. Initialization: Generate an initial population of potential solutions randomly.   
2. Fitness Evaluation: Define a fitness function to evaluate how good each solution is.   
3. Selection: Select individuals from the current population based on their fitness. Higher fitness individuals have a higher chance of being selected.   
4. Crossover: Combine pairs of selected individuals (parents) to produce offspring (children). This introduces new solutions by mixing genetic information.   
5. Mutation: Apply random changes to some individuals to maintain genetic diversity. This helps prevent premature convergence on suboptimal solutions.   
6. Replacement: Replace the old population with the new one, often using strategies like elitism to retain the best solutions.   
7. Termination: Repeat the process through multiple generations until a stopping criterion is met (eg. a satisfactory fitness level or a maximum number of generations).

![](images/541ca2efc6cff6e7673a14f6270a8a281d4f3c66bc36f460524cdb0e7995d93e.jpg)

# Bitwise Operations

![](images/22ce41d3ef60214f7683f6110c1f10a1d66979e66ded7ef0d4e26108baa35dea.jpg)

![](images/5a3c1b5104c8a1289bfa6df516ffbc30514440c017a6fe3186546715eae629be.jpg)

- & (Bitwise AND): This operator compares each bit of two numbers. If both corresponding bits are 1, the resulting bit is set to 1; otherwise, it is set to 0.

$$
a = 5 \# (\text {b i n a r y :} 0 1 0 1)
$$

$$
b = 3 \# (b i n a r y: 0 0 1 1)
$$

$$
\text {r e s u l t} = \mathrm {a} \& \mathrm {b} \# (\text {b i n a r y : 0 0 0 1 , d e c i m a l : 1})
$$

- $\ll$ (Left Shift): This operator shifts the bits of a number to the left by a specified number of positions. Each left shift effectively multiplies the number by 2.

$$
a = 3 \# (\text {b i n a r y :} 0 0 1 1)
$$

$$
r e s u l t = a <   <   1 \# (b i n a r y: 0 1 1 0, d e c i m a l: 6)
$$

- ~ (Bitwise NOT): This operator inverts all the bits of a number. Each 0 becomes 1 and each 1 becomes 0.

$$
\begin{array}{l} a = 5 \# (\text {b i n a r y :} 0 1 0 1) \\ \text {r e s u l t} = \sim a \# (\text {b i n a r y : 1 0 1 0 , d e c i m a l : - 6 i n t w o ' s c o m p l e m e n t r e p r e s e n t a t i o n}) \\ \end{array}
$$

# Genetic Algorithm - Example

- Maximizing the function $f(x) = x^{2}$ within the range [0, 31].

import random

def fitness(x):

"Calculate the fitness of an individual."

return x\*\*2 # Fitness function (maximize x^2)

def select(population):

""Select two parents from the population based on fitness."

Select two individuals using weights determined by their fitness

return randomchoices(population, weights $\equiv$ [fitness(x) for x in population], k=2)

def crossover(parent1, parent2):

""Perform crossover between two parents to create two children."""

point = random.randint(1, 4) # Random crossover point

Create children by mixing bits from parents

child1 = (parent1 & (15 << point)) | (parent2 & ~(15 << point))

child2 = (parent2 & (15 << point)) | (parent1 & ~(15 << point))

return child1, child2

def mutate(x):

""Mutate an individual with a small probability."""

if random.random() < 0.1: # 10% mutation chance

bit = random.randint(0, 4) # Select a random bit to mutate

return x ^ (1 << bit) # Flip the selected bit

return x # Return the original if no mutation occurs

Example initialization

population = [random.randint(0, 31) for _ in range(10)] # Create a random population of integers

for_in range(10): # Run for a fixed number of generations

new_population = [] # Prepare a new population

for_in range(len(population) // 2): # Iterate for half the population size

parent1, parent2 = select(population) # Select two parents

child1, child2 = crossover(parent1, parent2) # Create two children

print('debug ... ', parent1, parent2, child1, child2)

# Mutate the children before adding them to the new population

new_populat. extend([mutate(child1), mutate(child2)]) # Update the population for the

next generation

population = new_population # Find the best solution in the final population

best_solution = max(population, key=fitness)

print("Best solution:", best_solution) # Output: Close to 31

# Genetic Algorithm: Remarks

- No Guarantee of Global Optimality   
- Factors Influencing Convergence

Population size   
Selection pressure   
Crossover and mutation rates   
- The nature of the fitness landscape (eg. presence of local optima)

- Exploration vs Exploitation

- Genetic algorithms balance exploration (searching through the solution space) and exploitation (refining existing solutions)   
- If exploration is too limited, the algorithm may converge prematurely to a local optimum

# Optimize an MA Cross

# Strategy

# MA Cross Strategy – Grid Search

- Suppose we apply a MA Cross strategy on AAPL   
• We are unsure what values of slow MA and fast MA to use (i.e. parameters = [fast_MA, slow_MA]), but we are interested in these ranges

fast_windows = [10, 20, 30, 40]   
- slow_windows = [80, 90, 100]

- We want to know which combination can achieve the highest overall return (i.e. objective = max. return)   
• As there are only 12 combinations, we can simply run all the backtests for comparison. (i.e. method = brute force)

![](images/9b09ac1246d4c6337c7cc9a2637866effb10100d557586427f141ecd69f06905.jpg)

# Optimize an MA Cross Strategy

import pandas as pd

import numpy as np

import matplotlib.pyplot as plt

import seaborn as sns

import yfinance as yf

from itertools import product

Fetch historical data

data = yf.download('AAPL', start='2020-01-01', end='2023-01-01')

Define window parameters

fast_windows = [10, 20, 30, 40]

slow_windows = [80, 90, 100]

Initialize a DataFrame to store results

results = pd.DataFrame(index=fast_windows, columns=slow_windows)

# Iterate through all combinations of fast_window and slow_window

for fast_window, slow_window in product(fast_windows, slow_windows):

Create a fresh copy of the data for each iteration

temp_data = data.copy()

Create signals

temp_data['fast_MA'] = temp_data['Close'].rollingwindow=fast_window).mean()

temp_data['slow_MA'] = temp_data['Close'].rollingwindow=slow_window).mean()

Initialize the Signal column

temp_data['Signal'] = 0

Create buy (1) and sell (-1) signals

temp_data['Signal'] = np.where(temp_data['fast_MA'] > temp_data['slow_MA'], 1, 0)

temp_data['Signal'] = np.where(temp_data['fast_MA'] < temp_data['slow_MA'], -1, temp_data['Signal'])

Calculate returns

temp_data['Market_Returns'] = temp_data['Close'].pct_change()

temp_data['Strategy_Returns'] = temp_data['Market_Returns'] * temp_data['Signal'].shift(1)

Calculate cumulative returns

total_return = (1 + temp_data['Strategy_Returns']) prod() - 1

Overall return from strategy # Store the result

results.loc[fast_window, slow_window] = total_return

Convert results to numeric type

results = results.astype(float)

Plotting the heatmap

plt.figure(figsize=(10,6))

sns_heatmap(results, annot=True, fmt=".2%", cmap='coolwarm', cbar_kws={'label': 'Overall Return'})

plt.title('Backtest Returns Heatmap')

plt xlabel('Slow Window')

plt. ylabel('Fast Window')

pltxticks(ticks=np.arange(len(slow_windows))+0.5,labels $\equiv$ slow_windows)

plt.yticks(ticks=np.arange(len(fast_windows)) + 0.5, labels=fast_windows)

plt show()

![](images/8a0d77208e20ff7e2f336e85caedaaa3cfc77b350cb4a0cc75cd686ffbd602e6.jpg)

# MA Cross Strategy – Gradient Descent

import pandas as pd

import numpy as np

import matplotlib.pyplot as plt

import yfinance as yf

data = yf.download('AAPL', start='2020-01-01', end='2023-01-01')

def compute_strategyreturns(fast_window,slow_window):

temp_data = data.copy()

temp_data['fast_MA'] = temp_data['Close'].rolling(window=fast_window).mean()

temp_data['slow_MA'] = temp_data['Close'].rolling(window=slow_window).mean()

temp_data['Signal'] = 0

temp_data['Signal'] = np.where(temp_data['fast_MA'] > temp_data['slow_MA'], 1, temp_data['Signal'])

temp_data['Signal'] = np.where(temp_data['fast_MA'] < temp_data['slow_MA'], -1, temp_data['Signal'])

temp_data['Market_Returns'] = temp_data['Close'].pct_change()

temp_data['Strategy_Returns'] = temp_data['Market_Returns'] * temp_data['Signal'].shift(1)

return (1 + temp_data['Strategy_Returns']) prod() - 1

Overall return # Gradient Descent parameters

learning_rate = 0.01

num_iterations = 1000

fast_window = 10

slow_window = 80

Gradient descent optimization

for_in range(num_iterations):

Calculate returns for current windows

current_return = compute_strategyreturns(fast_window, slow_window)

Small perturbations to estimate gradients

fast_return_plus = compute_strategyreturns(fast_window + 1, slow_window)

slow_return_plus = compute_strategyreturns(fast_window, slow_window + 1)

Estimate gradients

fast-gradient = (fast_return_plus - current_return) / 1

slow_gradients = (slow_return_plus - current.return) / 1

Update the windows

fast_window += learning_rate * fast_gradient

slow_window += learning_rate * slow_gradients

Ensure windows are within reasonable bounds

fast_window = max(1, int(fast_window))

slow_window = max(fast_window + 1, int(slow_window))

Final optimized windows

print(f'Optimized Fast Window: {fast_window}')

print(fOptimized Slow Window: {slow_window})

Calculate final returns

final_return = compute_strategyreturns(fast_window, slow_window)

print(f'Final Strategy Return: {final_return:.2%}')

Optimized Fast Window: 9

Optimized Slow Window: 79

Final Strategy Return: $30.09\%$

# Is Gradient Decent a proper method?

- For the MA Cross strategy, the parameters can only be integers.   
- Slow MA period   
- Fast MA period   
- Thus, it is a discrete function and therefore not differentiable.

# Optimal vs Robust Solution

# Optimal vs Robust Solution

- Suppose you obtain the following optimization results for a step size of 5.   
- Which parameter X will you choose?

![](images/6041ce3c3221f0f22dc5cd07d2734709d0e42f72fe64483fe6479da9da81bf62.jpg)

# Optimal vs Robust Solution

- For general data modelling,

<table><tr><td></td><td>Optimality</td><td>Robustness</td></tr><tr><td>Pros</td><td>High performance in ideal scenarios</td><td>Consistent performance in diverse environments</td></tr><tr><td>Cons</td><td>Vulnerable to overfitting; may fail under new conditions</td><td>May sacrifice maximum performance for reliability</td></tr></table>

# Optimal vs Robust Solution

- Optimal Strategy:

- Good performance in backtesting but likely fails in live conditions.

- Robust Strategy:

- Moderate performance but performs steadily across market conditions.

Choose a robust strategy!!!

![](images/32d9ed2ac97b4346c2d43fa52cc477df5a7ee5aba45f1b1a68ab55392c4adbea.jpg)  
Backtest Returns Heatmap

# Guidelines to find a robust strategy

Parameter Sensitivity Analysis

- Test Parameter Ranges: Assess how performance varies with changes in strategy parameters.   
- Identify Stability Zones: Look for parameters that yield consistent results across a range of values.

- Walk-Forward Validation

- Use a rolling window approach to optimize parameters on a subset of data while validating on the next segment.   
• For example,

- Suppose you have 5 years of data   
- Split into 5 subsets, each with 1 year of data   
- Run the strategy with different parameter combinations over the 5 data subsets   
- Choose the parameter set that yield a moderate performance among the 5 data subsets

# Strategy Optimization on

# ALGOGENE

# Example

We take the MA Cross strategy as an example   
- Trading parameters are initially set to

- Fast MA period = 7   
- Slow MA period $= 14$

- Backtest settings:

Period: 2024.01 - 2024.12   
Instrument: XAUUSD   
Data Interval: 1-day   
Initial capital: 10,000 USD   
- Enable short-selling: True   
- Leverage: 1x   
Transaction Cost: 0

# Settings

Strategy Name

MA Cross

Start Period

2024-01

End Period

2024-12

Data Interval *

1 day

Initial Capital *

10000

Base Currency

USD

Leverage

1

Transaction Cost

0

Risk free Rate

0

Enable Short Sell

![](images/d0d132021070d6af73d9ab5b2cc38ea9f5702bb8faecbaf441bb80e0ef3fc3f2.jpg)

Position Netting

News Data

Weather Data

Economics Data

Instruments

$\therefore m = \frac{3}{11}$ ;

(Max. 100 items)

>   
>   
>

# 1. Define parameters at __init__.

from AlgoAPI import AlgoAPIUtil, AlgoAPI_Backtest

from datetime import datetime, timedian

import talib, numpy

class AlgoEvent:

def __init__(self):

self.lasttradetime $=$ datetime(1970,1,1)

self.arr_close = numpy.array([])

self arr_fastMA = numpy.array([])

self arrslowMA $=$ numpy array(Π)

self fastestperiod $= 7$

self slowsperiod = 14

def start(self, mEvt):

self.myinstrument = mEvt['subscribeList'][0]

self.evt = AlgoAPI_Backtest.AlgoEvtHandler(self, mEvt)

selfevt.start()

def on_bulkdatafeed(self, isSync, bd, ab):

if bd[self.myinstrument][‘timestamp’] >= self lasttradetime + timedelta(hours=24):

self.lasttradetime = bd[self.myinstrument]['timestamp']

lastprice = bd[self.myinstrument]['lastPrice']

self.arr_close = numpy.append(self.arr_close, lastprice)

keep the most recent observations

if len(self.arr_close)>int(self.fastperiod+selfslowperiod):

self.arr_close = self.arr_close[int(self.fastperiod + selfslowperiod):]

fit SMA line

self.arr_fastMA = talib.SMA(self arr_close, timeperiod=int(self.fastperiod))

self.arr SlowMA = talib.SMA(self arr_close, timeperiod=int(self slowperiod))

# debug print result

self.evt consoleLog("arr_fastMA=", self arr_fastMA)

self.evtconsoleLog("arrslowMA=", self.arrslowMA)

check number of record is at least greater than both self.fastperiod, selfslowperiod

if not numpy.isnan(self.arr_fastMA[-1]) and not numpy.isnan(self.arr_fastMA[-2]) and not numpy.isnan(self.arr_slowMA[-1]) and not numpy.isnan(self.arr SlowMA[-2]):

send a buy order for Golden Cross

if self arr_fastMA[-1] > self arrslowMA[-1] and self arr_fastMA[-2] < self.arr SlowMA[

2]:

self.open_order(lastprice,1,'open')

send a sell order for Death Cross

if self arr_fastMA[-1] < self arrslowMA[-1] and self arr_fastMA[-2] > self arr SlowMA[

2]:

self.open_order(lastprice, -1, 'open')

def open_order(self, lastprice, buysell, openclose):

order $=$ AlgoAPIUtil.OrderObject(

instrument = self.myinstrument,

volume $= 0.01$

openclose = openclose,

buysell $=$ buysell.

ordertype $= 0$ #0=market_order,1=limit_order,2=stop_order

）

self evt.sendOrder(order)

# Procedures

1. Successfully complete the backtest to make sure your script has no bug   
2. Go to [My History], locate the strategy you want to optimize   
3. Click "Optimize" from the drop-down

![](images/781f3d052a40585a15927889a2244ede6adc13f15b29c7d073e59b4888a56f2a.jpg)

# Procedures

# 4. Setup optimization objective, method, parameters and its ranges and step size

# Strategy Optimizer

![](images/7125842bf79d444a2cd55394eeff8eac2b4a716082c3e34bdc93a825b8ce208b.jpg)

Initial estimate, only useful for gradient decent method

The range of parameters, including the start and end value

# Settings

Base Strategy ID

Objective

Method

20250221_160311_453081

Max. Return

Brule force

# Inputs

![](images/93137cab901f699633c560238e89fbb1827e1142cf533626df20ee8713a049d8.jpg)

Optimize

Stop

# Procedures

# 5. Analyze results

Summary of al

backtest iterations

Results

Summary

Charts

Performance

<table><tr><td>Pass</td><td>RuntimeID</td><td>Total Trades</td><td>Ann. Return (%)</td><td>Sharpe</td><td>Sortino</td><td>Max. Drawdown (%)</td><td>Volatility (%)</td><td>fastperiod</td><td>slowperiod</td></tr><tr><td>1</td><td>20250406_074403_077626</td><td>27</td><td>0.2558</td><td>0.8740</td><td>1.3013</td><td>0.2669</td><td>0.2915</td><td>7</td><td>12</td></tr><tr><td>2</td><td>20250406_074403_078626</td><td>24</td><td>0.1812</td><td>0.6183</td><td>0.9078</td><td>0.2904</td><td>0.2919</td><td>7</td><td>13</td></tr><tr><td>3</td><td>20250406_074403_079626</td><td>24</td><td>0.0869</td><td>0.2994</td><td>0.4332</td><td>0.3299</td><td>0.2892</td><td>7</td><td>14</td></tr><tr><td>4</td><td>20250406_074403_080626</td><td>20</td><td>0.0834</td><td>0.2862</td><td>0.4179</td><td>0.3262</td><td>0.2901</td><td>7</td><td>15</td></tr><tr><td>5</td><td>20250406_074403_081626</td><td>27</td><td>0.1848</td><td>0.6378</td><td>0.9307</td><td>0.2280</td><td>0.2885</td><td>8</td><td>12</td></tr><tr><td>6</td><td>20250406_074403_082626</td><td>24</td><td>0.2385</td><td>0.8038</td><td>1.1956</td><td>0.2525</td><td>0.2956</td><td>8</td><td>13</td></tr><tr><td>7</td><td>20250406_074403_083626</td><td>24</td><td>0.1655</td><td>0.5588</td><td>0.8253</td><td>0.2863</td><td>0.2950</td><td>8</td><td>14</td></tr><tr><td>8</td><td>20250406_074403_084626</td><td>22</td><td>0.1624</td><td>0.5511</td><td>0.8133</td><td>0.2844</td><td>0.2935</td><td>8</td><td>15</td></tr><tr><td>9</td><td>20250406_074403_085626</td><td>35</td><td>0.3263</td><td>1.0443</td><td>1.6014</td><td>0.2161</td><td>0.3112</td><td>9</td><td>12</td></tr><tr><td>10</td><td>20250406_074403_086626</td><td>28</td><td>0.1586</td><td>0.5326</td><td>0.7966</td><td>0.2422</td><td>0.2967</td><td>9</td><td>13</td></tr><tr><td>11</td><td>20250406_074403_087626</td><td>24</td><td>0.1334</td><td>0.4623</td><td>0.6817</td><td>0.2957</td><td>0.2874</td><td>9</td><td>14</td></tr><tr><td>12</td><td>20250406_074403_088626</td><td>24</td><td>0.2540</td><td>0.9278</td><td>1.4813</td><td>0.1699</td><td>0.2727</td><td>9</td><td>15</td></tr><tr><td>13</td><td>20250406_074403_089626</td><td>36</td><td>0.2768</td><td>0.8703</td><td>1.3379</td><td>0.2165</td><td>0.3168</td><td>10</td><td>12</td></tr><tr><td>14</td><td>20250406_074403_090626</td><td>32</td><td>0.1745</td><td>0.5750</td><td>0.8911</td><td>0.2471</td><td>0.3024</td><td>10</td><td>13</td></tr><tr><td>15</td><td>20250406_074403_091626</td><td>30</td><td>0.1018</td><td>0.3459</td><td>0.5297</td><td>0.2496</td><td>0.2930</td><td>10</td><td>14</td></tr><tr><td>16</td><td>20250406_074403_092626</td><td>24</td><td>0.1922</td><td>0.7062</td><td>1.1253</td><td>0.2146</td><td>0.2710</td><td>10</td><td>15</td></tr></table>

![](images/46e5659cad2f8ad7247ec0451a0703c5cafb6d8fb1fdb6b12e4456d79c59d3c2.jpg)  
Cumulative Return

# Each optimization task will generate a group of backtests with the same "GroupID"

ALGOGENE

FinTech Co. Ltd.

Search community topic...

Home

Services

Marketplace

Community

![](images/d7d1ca105898d5d538576b95a2ac5abce62eba1da6c3b213c244b07fe77d712a.jpg)

Account

#

![](images/0f674892c87690bafa0bf7b77e941d250f836da7f0274193478dc9a916bdece4.jpg)

![](images/7fbd7ea5b5e8e92f270c4fc833f8a2d84a30690d5f37721e6463600053fac57d.jpg)

![](images/f6ca3159cd3dee209d7d15dcdc3c455f26130a35998972b97e66a14aa2706f87.jpg)

![](images/a83779302dcbbc4778d44c19fa51f4975e578034abc87a90b38f78166af092e0.jpg)

![](images/038d9408f898b9fe077335570b7f1532896e0909e43f47fa33d192f46929e58d.jpg)

![](images/52fa9522d3f8638c9797ca53fd22cac6a7a57449b51ed7809f90a59f1124a247.jpg)

# Finished Backtest

Show 100

entries

Search:

<table><tr><td>Strategy</td><td>Data Interv</td><td>Start Date</td><td>End Date</td><td>Instruments</td><td>Initial Capit</td><td>Base Curren</td><td>Leverage</td><td>Trade Co</td><td>allowShortSe</td><td>RunTimeID</td><td>GroupID</td><td>Score</td><td>Status</td></tr><tr><td>MA Cross</td><td>1 day</td><td>2024-01</td><td>2024-12</td><td>XAUUSD</td><td>100000.0</td><td>USD</td><td>1.0</td><td>0.0</td><td>True</td><td>20250406_074403_092626</td><td>201944643</td><td>58.23703820831785</td><td>finished</td></tr><tr><td>MA Cross</td><td>1 day</td><td>2024-01</td><td>2024-12</td><td>XAUUSD</td><td>100000.0</td><td>USD</td><td>1.0</td><td>0.0</td><td>True</td><td>20250406_074403_091626</td><td>201944643</td><td>48.40544003782705</td><td>finished</td></tr><tr><td>MA Cross</td><td>1 day</td><td>2024-01</td><td>2024-12</td><td>XAUUSD</td><td>100000.0</td><td>USD</td><td>1.0</td><td>0.0</td><td>True</td><td>20250406_074403_090626</td><td>201944643</td><td>50.439484364713614</td><td>finished</td></tr><tr><td>MA Cross</td><td>1 day</td><td>2024-01</td><td>2024-12</td><td>XAUUSD</td><td>100000.0</td><td>USD</td><td>1.0</td><td>0.0</td><td>True</td><td>20250406_074403_089626</td><td>201944643</td><td>64.6928873169393</td><td>finished</td></tr><tr><td>MA Cross</td><td>1 day</td><td>2024-01</td><td>2024-12</td><td>XAUUSD</td><td>100000.0</td><td>USD</td><td>1.0</td><td>0.0</td><td>True</td><td>20250406_074403_088626</td><td>201944643</td><td>60.070279865724615</td><td>finished</td></tr><tr><td>MA Cross</td><td>1 day</td><td>2024-01</td><td>2024-12</td><td>XAUUSD</td><td>100000.0</td><td>USD</td><td>1.0</td><td>0.0</td><td>True</td><td>20250406_074403_087626</td><td>201944643</td><td>53.62612981795113</td><td>finished</td></tr><tr><td>MA Cross</td><td>1 day</td><td>2024-01</td><td>2024-12</td><td>XAUUSD</td><td>100000.0</td><td>USD</td><td>1.0</td><td>0.0</td><td>True</td><td>20250406_074403_086626</td><td>201944643</td><td>54.82840629873183</td><td>finished</td></tr><tr><td>MA Cross</td><td>1 day</td><td>2024-01</td><td>2024-12</td><td>XAUUSD</td><td>100000.0</td><td>USD</td><td>1.0</td><td>0.0</td><td>True</td><td>20250406_074403_085626</td><td>201944643</td><td>67.47145880699617</td><td>finished</td></tr><tr><td>MA Cross</td><td>1 day</td><td>2024-01</td><td>2024-12</td><td>XAUUSD</td><td>100000.0</td><td>USD</td><td>1.0</td><td>0.0</td><td>True</td><td>20250406_074403_084626</td><td>201944643</td><td>59.69430075307789</td><td>finished</td></tr><tr><td>MA Cross</td><td>1 day</td><td>2024-01</td><td>2024-12</td><td>XAUUSD</td><td>100000.0</td><td>USD</td><td>1.0</td><td>0.0</td><td>True</td><td>20250406_074403_083626</td><td>201944643</td><td>58.50512557168501</td><td>finished</td></tr><tr><td>MA Cross</td><td>1 day</td><td>2024-01</td><td>2024-12</td><td>XAUUSD</td><td>100000.0</td><td>USD</td><td>1.0</td><td>0.0</td><td>True</td><td>20250406_074403_082626</td><td>201944643</td><td>58.13699550147597</td><td>finished</td></tr><tr><td>MA Cross</td><td>1 day</td><td>2024-01</td><td>2024-12</td><td>XAUUSD</td><td>100000.0</td><td>USD</td><td>1.0</td><td>0.0</td><td>True</td><td>20250406_074403_081626</td><td>201944643</td><td>60.737965582580834</td><td>finished</td></tr><tr><td>MA Cross</td><td>1 day</td><td>2024-01</td><td>2024-12</td><td>XAUUSD</td><td>100000.0</td><td>USD</td><td>1.0</td><td>0.0</td><td>True</td><td>20250406_074403_080626</td><td>201944643</td><td>50.11683400886043</td><td>finished</td></tr><tr><td>MA Cross</td><td>1 day</td><td>2024-01</td><td>2024-12</td><td>XAUUSD</td><td>100000.0</td><td>USD</td><td>1.0</td><td>0.0</td><td>True</td><td>20250406_074403_079626</td><td>201944643</td><td>49.65171151339092</td><td>finished</td></tr></table>

# Investment Funds

# Different Type of Investment Funds

There are several types of funds available in the market, each catering to different investment strategies and investor needs.

1. Exchange-Traded Funds (ETFs)   
2. Mutual Funds   
3. Money Market Funds   
4. Private Equity Funds   
5. Hedge Funds

# 1. Exchange-Traded Funds (ETFs)

- ETFs are traded on stock exchanges, similar to individual stocks, and typically track an index, commodity, or a basket of assets.   
- Comprised of a diversified portfolio and can include stocks, bonds, or other securities.   
Key Features

- Liquidity: Can be bought and sold throughout the trading day at market prices.   
- Cost-Effective: Generally have lower expense ratios compared to mutual funds.   
- Flexibility: Investors can use various trading strategies, including short selling and options.

Examples:

- SPDR S&P 500 ETF Trust (SPY): Tracks the S&P 500 index.   
- Invesco QQQ Trust (QQQ): Tracks the Nasdaq-100 Index, focusing on tech stocks.

# 2. Mutual Funds

- Investment vehicles that pool money from multiple investors to purchase a diversified portfolio of stocks, bonds, or other securities.   
- Managed by professional fund managers who make investment decisions on behalf of the investors.   
Key Features

Diversification: Reduces risk by investing in a variety of assets.   
- Liquidity: Shares can be bought or sold on any business day.   
- Accessibility: Available to individual investors with relatively low minimum investments.

Examples:

- Fidelity Contrafund (FCNTX): Actively managed equity fund focusing on growth stocks.   
T. Rowe Price Dividend Growth Fund (PRDGX): Invests in companies with a strong history of dividend growth.   
Schwab Total Bond Market Fund (SWLBX): Provides exposure to a diversified portfolio of U.S. bonds.

# Differences Between ETFs and Mutual Funds

<table><tr><td></td><td>ETF</td><td>Mutual Funds</td></tr><tr><td>Trading</td><td>Traded on exchanges throughout the day</td><td>Bought/sold at end-of-day NAV</td></tr><tr><td>Management Style</td><td>Often passively managed</td><td>Can be actively or passively managed</td></tr><tr><td>Fees</td><td>Typically lower expense ratios</td><td>Higher fees, depending on management</td></tr><tr><td>Minimum Investment</td><td>No minimum (can buy 1 share)</td><td>Often requires a minimum investment</td></tr></table>

# 3. Money Market Funds

- A type of mutual fund that invests in short-term, high-quality debt instruments, such as treasury bills, commercial paper, and certificates of deposit.   
- The objective is to provide liquidity, stability, and a modest return, making them a safe place to park cash.   
Key Features

- Liquidity: Investors can typically access their funds quickly, often within a day.   
- Safety: Generally considered low-risk investments, as they invest in high-quality, short-term securities.   
- Yield: Offers higher interest rates than traditional savings accounts, though returns can vary based on market conditions.

![](images/55d8b2ef37e112607bd887e0e2b7a149d91be113b5feed3accb27afa1f1d31c0.jpg)

# 4. Private Equity Funds

- Investment funds that provide capital to private companies or buyouts of public companies, aiming for long-term capital appreciation.   
- Typically organized as limited partnerships, where the fund manager is the general partner and investors are limited partners.   
Key Features

- Investment Horizon: Generally have a longer investment period (5-10 years) before realizing returns.   
- Active Management: Fund managers often take an active role in improving the operations and value of portfolio companies.   
- High Returns: Potential for significant returns, but also higher risk compared to traditional investments.

# 5. Hedge Fund

- Investment funds that employ diverse and advanced quant strategies to achieve high returns, often using leverage, derivatives, and short selling.   
- Typically structured as limited partnerships, where fund managers are general partners and investors are limited partners.   
- Could be open ended or closed ended fund   
Key Features

- Flexibility: Can invest in a wide range of assets, including stocks, bonds, commodities, and real estate.   
- Risk Management: Utilize various strategies to hedge against market downturns and manage risk.   
- Accredited Investors: Generally only available to accredited, professional or institutional investors due to higher risks and complexity.

# Summary of Investment Funds

<table><tr><td>Fund Type</td><td>Structure</td><td>Liquidity</td><td>Minimum Investment</td><td>Investment Strategy</td><td>Risk Level</td><td>Investor Type</td></tr><tr><td>Exchange-Traded Funds (ETFs)</td><td>Traded on exchanges, like stocks</td><td>Intraday</td><td>Varies (generally low)</td><td>Tracks an index, sector, or commodity</td><td>Moderate to high</td><td>Retail and institutional investors</td></tr><tr><td>Mutual Funds</td><td>Pooled investment vehicle</td><td>Daily (end of day)</td><td>Varies (often low)</td><td>Actively or passively managed, diversified</td><td>Moderate</td><td>Retail and institutional investors</td></tr><tr><td>Money Market Funds</td><td>Pooled investment vehicle</td><td>Daily</td><td>Varies (often low)</td><td>Invests in short-term, high-quality debt</td><td>Low</td><td>Conservative investors</td></tr><tr><td>Private Equity Funds</td><td>Limited partnerships</td><td>Illiquid (5-10 years)</td><td>High</td><td>Invests in private companies, buyouts</td><td>High</td><td>Accredited investors</td></tr><tr><td>Hedge Funds</td><td>Limited partnerships</td><td>Illiquid (varies)</td><td>High</td><td>Diverse strategies, including long/short, arbitrage</td><td>High</td><td>Accredited and institutional investors</td></tr></table>

# Top Quant Trading Firms

# - Renaissance Technologies

- Founded by Jim Simons, Renaissance is renowned for its quantitative trading strategies, particularly through its Medallion Fund.   
- The Medallion Fund is known for achieving annualized returns of around $66 \%$ before fees since its inception in 1988. However, the fund is closed to outside investors.

# - Two Sigma Investments

- Founded in 2001 by John Overdeck and David Siegel, Two Sigma employs advanced technology and data science to drive its investment strategies.   
- The firm has consistently delivered positive returns, with some funds reporting annualized returns of around $15 - 20\%$ over the long term.

![](images/dba755c36dd2036ae53df064ab9d7c7294c302294df06a075bd5a8e88b85e286.jpg)  
Jim Simons

![](images/0d86f47087016d0fd3d2134bb9e7dc44e37626c8f4eb66cc28efc52fce7a6a87.jpg)  
John Overdeck, David Siegel

# Virtu's IPO: First high-frequency trading firm to go public

![](images/c245b649d22cd7ef2b830b0080335206a0619f1bf390debb15c4c224ebf92283.jpg)

Justine Underhill $\cdot$ Reporter

April 16, 2015

![](images/bd42f799af7df487e211a7799df268ead31ac27cad3efcfffbe2939aa94aa022.jpg)

![](images/b330dc8253a18af8994c8c5047d6cdb2f9c4f779273a345e1e167782c277fdcc.jpg)

A the high-frequency trading debate continues, Virtu Financial (VIRT) successfully launched the first HFT firm IPO Wednesday with strong demand from investors.

The firm priced shares at $19 on Wednesday, the upper-end of its targeted range, and raised over$ 300 million at a $2.6 billion valuation.

This vote of confidence from investors comes just a year after Virtu pulled its IPO amid acute industry scrutiny, which followed the release of Michael Lewis' book "Flash Boys."

In a SEC filing last year, Virtu revealed it had only lost money on one day since 2009. "I thought it was a good thing to disclose to the

world that the firm was profitable," said Doug Cifu, Virtu CEO, at a conference last June. "But, boy, did that backfire in my face, so I take responsibility for that." As of December 31, the 148-person firm generated net income of $190 million last year.

# The secret behind

- Hedge funds utilize multiple uncorrelated strategies to diversify risk and enhance overall performance.   
- By combining strategies that react differently to market conditions, funds can stabilize returns and increase the likelihood of winning trades.   
- Each strategy just has a winning rate slightly above $50\%$ . Due to the law of large number, it can increase the probability of overall positive returns.

# Example

import numpy as np  
import matplotlib.pyplot as plt  
import pandas as pd

# Parameters  
n_strategies = 1000 # Number of strategies  
winning_rate = 0.55 # Each strategy has a winning rate of 55%  
n_simulations = 10000 # Number of simulations  
n_day = 252 # Trading days in a year # Simulate returns for each simulation  
np.random.seed(0) # For reproducibility  
allcumulativereturns = np.zeros((n_simulations, n_day))

for sim in range(n_simulations): dailyreturns $=$ np.zeros((n_day, n_strategies)) # Generate returns for each strategy for i in range(n_strategies): if np.random.randint() $<$ winning_rate: dailyreturns[;, i] $=$ np.random.normal(0.01, 0.02, n_day) # Winning return else: dailyreturns[;, i] $=$ np.random.normal(-0.01, 0.02, n_day) # Losing return

Calculate cumulative returns for this simulation  
cumulativereturns = np.cumprod(1 + dailyreturns, axis=0)  
all_cumulativereturns[sim, :] = np.mean(cumulativereturns, axis=1)

Prepare date range dates = pd.date_range(start='2022-01-01', periods=n每一天)

Plotting the results  
plt.figure(figsize=(12, 6))  
for sim in range(n_simulations):  
    plt.plot(dates, all Cumulative Returns[sim, :, color='blue', alpha=0.01)  
    plt.title('Cumulative Return of 1000 Uncorrelated Strategies over 1 Year')  
    pltxlabel('Date')  
    pltylabel('Cumulative Return')  
    plt(axline(y=1, color='red', linestyle='--', label='Break-even Line')  
    plt legend()  
    plt.grid()  
    plt.show()

![](images/d9880b3f9c05d4474badb56865292b85ac27d13755af5d2d7171142b0dc00cf4.jpg)  
Cumulative Return of 1000 Uncorrelated Strategies over 1 Year

# Broker Selection

# Why Broker Choice Matters

Profitability of your trading strategy   
Investor protection   
Key Considerations

1. Trading costs   
2. Access to markets   
3. Trading platform features   
4. Regulatory compliance   
5. Customer support   
6. Reliability and reputation

![](images/8d4d228df4f72973426b91d50ba93b40a8a5c62f3139d712b3a642e42a7a2c7b.jpg)

![](images/6336992211cfbc02d41775b740593164eafd08085cca24489405b79a025eb228.jpg)

![](images/842f1521d7893a46ca9e93656be3c3ca4f05e4b3361b6e4a364af2945e648204.jpg)

FINTECH

fintechnews.hk

![](images/df18b39418fba90a3c36afdf5081af2f579174d0416048a6ff49fd640399ff86.jpg)  
HONG KONG FINTECH STARTUP MAP 2022

393

STARTUPS

Brennan

$\therefore m = \frac{3}{11}$

nittance (5)

Airwallex

EMQ

#

#

Travele

$\therefore m : x = 1$ 或 ${3x} + {4y} + 1 = 0$

${3x} =  - \frac{1}{2}{x}^{2} + \frac{3}{5}{x} + 4$

#

M

Y

1

![](images/091267aa44960d7947be9033ceda2acb9beb142fb1c19cf478a6c353fe9a8ca4.jpg)

![](images/aad8700376fcba902c064e669519011361076efeee6b632de8decb3c111cb1c4.jpg)

![](images/5d7c4c99edfeedb787e2df8d68e6bb5eb5ec0f6b44f5757667a1ee68a8c1b952.jpg)

Digital Banks (8)

$\left( {x - {2x}}\right) t - x{y}^{2} = \left( {x - {2x}}\right) {f}^{\prime }t$

A

airstar

NTI

airstar

Wealthtech (50)

$\therefore m = \frac{3}{11}$

$\therefore m = \frac{3}{11}$ ;

$\therefore m = \frac{3}{11}$ ;

$\therefore m = \frac{3}{11}$

.

$\therefore m = \frac{3}{11}$ ;

$\therefore m = \frac{3}{11}$

$\therefore m = \frac{3}{11}$ ;

$\therefore m = \frac{3}{11}$

$\therefore m = \frac{3}{11}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{12}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{12}$

${12}/{12}$

${12}/{14}$

$\therefore m = \frac{3}{11}$ ;

$\therefore m = \frac{3}{11}$

$\therefore m = \frac{3}{11}$

$\frac{1}{2} =$

${12}/{14}$

${12}/{12}$

${12}/{14}$

${12}/{12}$

${12}/{14}$

${12}/{12}$

${12}/{14}$

${12}/{12}$

${12}/{14}$

${12}/{12}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{12}$

${12}/{14}$

${12}/{12}$

${12}/{14}$

${12}/{12}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{12}$

${12}/{14}$

${12}/{12}$

${12}/{14}$

${12}/{12}$

${12}/{14}$

${12}/{12}$

${12}/{14}$

${12}/{12}$

${12}/{14}$

${12}/{14}$

${12}/{14}$

${12}/{12}$

${12}/{12}$

${12}/{12}$

Cryptocurrency (9)

$\frac{3}{1} + u + {4q} = 1 + {uq}$ dH

$\therefore m = \frac{3}{11}$

$\therefore m = \frac{3}{11}$

$\therefore m = \frac{3}{11}$

$\therefore m = \frac{3}{11}$

$\therefore m = \frac{3}{11}$ ;

$\therefore m = \frac{3}{11}$ ;

${12}/{14}$

$\therefore m = \frac{3}{11}$ ;

[ \Rightarrow  \;{3}^{x} =  - 2 + 2\sqrt{2}]

${13}/{14}$

$\therefore m = \frac{3}{11}$ ;

$\therefore m = \frac{3}{11}$ ;

$\therefore m = \frac{3}{11}$

$\therefore m = \frac{3}{11}$ ;

${12}/{14}$

$\therefore m = \frac{3}{11}$ ;

$\therefore m = \frac{3}{11}$

.

$\frac{1 + u}{7} = {70}\%$

.

${13}/{14}$

$\therefore m = \frac{3}{11}$ ;

$\therefore m = \frac{3}{11}$ ;

${13}/{14}$

$\therefore m = \frac{3}{11}$

$\therefore m = \frac{3}{11}$ ;

$\therefore m = \frac{3}{11}$ ;

.

$\therefore m = \frac{3}{11}$ ;

# 1. Evaluating Broker Fees

- Higher costs can erode profits, particularly for frequent traders   
- Commission Structures:

Per trade vs flat fees

- Spread Costs:   
- Fixed vs variable spreads

- Other Fees:

- Withdrawal fees, inactivity fees, data fees

![](images/ebe6db8d20ef0401cf86745825bf300f5d45fa19985ca6e38a5dfbe13cd2d385.jpg)

UNITED STATES   

<table><tr><td>Monthly Volume (shares)1,7</td><td>IBKR Pro - Tiered</td><td>IBKR Pro - Fixed</td><td>IBKR Lite</td></tr><tr><td>≤300,000</td><td>USD 0.0035</td><td rowspan="5">USD 0.005</td><td rowspan="5">USD 0.002</td></tr><tr><td>300,001 - 3,000,000</td><td>USD 0.0020</td></tr><tr><td>3,000,001 - 20,000,000</td><td>USD 0.0015</td></tr><tr><td>20,000,001 - 100,000,000</td><td>USD 0.0010</td></tr><tr><td>&gt;100,000,000</td><td>USD 0.0005</td></tr><tr><td colspan="4"></td></tr><tr><td>Minimum per order (shares)</td><td>USD 0.35</td><td>USD 1.00</td><td>USD 0.003</td></tr><tr><td>Minimum per order (fractional shares)</td><td>USD 0.01</td><td>USD 0.01</td><td>USD 0.003</td></tr><tr><td>Maximum per order</td><td>1% of Trade Value4,8</td><td>1% of Trade Value4,8</td><td>USD 0.00</td></tr><tr><td>IB SmartRoutingSM</td><td>✔</td><td>✔</td><td>✕</td></tr><tr><td>Eligibility</td><td>All</td><td>All</td><td>US Residents Only</td></tr><tr><td>Third Party Fees</td><td>• Regulatory Fees
• Exchange Fees
• Clearing Fees
• Pass-Through Fees</td><td>• Regulatory Fees</td><td>• Regulatory Fees</td></tr></table>

# 2. Access to Markets

- Variety of Instruments and Asset Classes   
- Ability to trade in various markets increases opportunities.   
- Stocks, forex, commodities, crypto, etc   
- Other things that may affects whether your strategy can be implemented   
- Account leverage   
- Account currency   
- Deposit method (eg. bank transfer, credit card, etc)

# 3. Trading Tools and Features

# - Platform Usability

- Intuitive interface can enhance trading efficiency

# - Analytical Tools

- Access to charts, indicators, and news feeds is useful for decision-making   
- API access for algorithmic trading   
Real-time data feeds   
- Demo account to test the platform before risking real money

# - Mobile App

- Flexibility to trade on-the-go   
Monitor the portfolio anytime, anywhere

# 4. Regulatory compliance

- Importance of Regulation

- Ensures broker reliability and protects investors from fraud   
- Ensures transparency and fair trading practices

- Verify that the broker is regulated by a reputable authority such as

- HKSC (Hong Kong)   
SEC, FINRA (U.S.)   
FCA (U.K.)   
ASIC (Australia)

# HKSC License Requirement

Type 1: Dealing in securities   
Type 2: Dealing in futures contracts   
Type 3: Leveraged foreign exchange trading   
Type 4: Advising on securities   
Type 5: Advising on futures contracts   
Type 6: Advising on corporate finance   
- Type 7: Providing automated trading services   
Type 8: Securities margin financing   
- Type 9: Asset management   
- Type 10: Providing credit rating services   
- Type 11: Dealing in OTC derivative products or advising on OTC derivative products   
- Type 12: Providing client clearing services for OTC derivative transactions   
- Type 13: Providing depository services for relevant CISs

# HKSC Licensed Brokers

- Reference: https://apps.sfc.hk/publicregWeb/searchByRa?locale=en

![](images/e174eb521d8f61a60e7defefb341102f5cb7a8ba44a9652605a08a2ceba13b22.jpg)

-A A+A| Eng 繁| Sitemap | C

ct us ⑦ FAQs ① Forms ④ Lodge a complaint ⑧ Media corner

Q Search

![](images/732b77d171cb0ec71d484be960a8537f0094bdbfc75eaf23fd6680d8a0b7e311.jpg)

About the SFC

Regulatory functions

Rules and standards

Published resources

News and announcements Career

![](images/c41b01948c7baa194e40680d6313d119011f0d59db6598e751558ee9fb556be6.jpg)

Alert List

# Public Register of Licensed Persons and Registered Institutions

# Print

Home

Public Register

Search by name

Search by licence type

Search by certain business activities (corporation only)

# Licence/Registration status

![](images/dc85eb5bffa68f51eb4ef8db8cfa51922fe1561d447b8bd5f9ea6ef8e3a9d618.jpg)

Active

![](images/96f3290a06c6c9324f465923f4b759922c3fbe496f0d65c42302cd4a1b80360a.jpg)

Active and inactive

# Type of licensee

![](images/a09529ff5c0db2fa6f801887b8750084932e1f79412c1ea83d6026589dbe552d.jpg)

Individual

![](images/4378605f97c674ebd9d85663a78b06bce08354c385ab8bd519f6f56529ef3a34.jpg)

Corporation

# SFO licence**

![](images/3e7b3810f8401ef00e17759de48ce02bced4ef76c5f59be31d987cc838a8a16b.jpg)

Type 1: Dealing in securities

![](images/f4df7bdec7ed657013028b97b07037f3ff00144323fc7bbff04527f05497f456.jpg)

Type 2: Dealing in futures contracts

![](images/98c199c9b4fbc073a60f055491359a4961085bf8ac0ad76e6cc540fcf65bb39f.jpg)

Type 3: Leveraged foreign exchange trading

![](images/a17887a7a231088d722c45f40c9e71283b3bdcbc433fb387d706d34abdb48f84.jpg)

Type 4: Advising on securities

![](images/139834c1fad3224f69dabc8e63b13f49db1f49d4ec062849d110138e49eff647.jpg)

Type 5: Advising on futures contracts

![](images/f3fce380e4b248286fa5d752411ff9a68082bb33b727d5b4328775d2149d7f52.jpg)

Type 6: Advising on corporate finance

![](images/941a9380e84406a36b3aee314ee9fc3885885d5d862740e9479761916f01013f.jpg)

Type 7: Providing automated trading services

![](images/e9426e24634a4f7273678ced8cb721854f2c6fa351dbc8eea57c960f599a5702.jpg)

Type 8: Securities margin financing

# AMLO licence

![](images/496521c2940f82ed986348942134a5d864dfe5e9ecae006c25d52b9a59edb899.jpg)

Virtual Asset Service: Operating a virtual asset trading platform*

# 5. Customer Support

Availability of Support

24/7 support can help resolve issues quickly

Educational Resources

- Quality brokers offer training materials, webinars, and articles to enhance trader skills.   
- Comprehensive documentation about its user interface, system functionalities, etc

# 6. Reliability and reputation

- Users review and comments on forum, social media, etc   
- The history of the brokerage firm (eg. company's age, industrial awards, number of customers, etc)   
- Listed companies are more reliable due to responsibility to regularly disclose its business and financials

# Useful Resources - CoinMarketCap

CoinMarketCap is a data platform to keep track on crypto market

Centralized Exchanges

- Spot: https://coinmarketcap.com/rankings/exchanges/   
- Derivatives: https://coinmarketcap.com/rankings/exchanges/derivatives/

- Decentralized Exchanges

- Spot: https://coinmarketcap.com/rankings/exchanges/dex/?type=spot   
Derivatives: https://coinmarketcap.com/rankings/exchanges/dex/?type=derivatives

![](images/c96b1d29aa2a6ebc4957d8fa3cb95a87489c585ed7765f459df16ff77606a99b.jpg)

# Useful Resources - MyFxBook

MyFxBook is a data platform to keep track on forex market

License: https://www.myfxbook.com/forexbrokers   
Quote: https://www.myfxbook.com/forex-broker- quotes   
- Spread: https://www.myfxbook.com/forex-broker-spreads   
Volume: https://www.myfxbook.com/forexbroker-volume   
- Swaps Fee: https://www.myfxbook.com/forexbroker-swapsv

![](images/46babc4cc08fc763b55c9a7a342bf36b89bdeeeb8e3a1c91c8e324a0909530dd.jpg)

# Useful Resources - BrokerChooser

- https://brokerchooser.com/broker-reviews

![](images/ce871bb0dabb357261d82717bdc8eaf19eb1f01c7a1e5cb03be8cecd4a0677b8.jpg)

# Best Practices in Broker Selection

- Research and Reviews:   
Use trader forums and reviews   
- Demo Accounts:   
- Test the platform before committing   
- Stay Updated:   
- Regularly review broker performance and fees

# FTX Case Study

# FTX Case Study

# • Background

- FTX was the global $3^{rd}$ largest cryptocurrency exchange in 2022.   
- High liquidity and a wide range of trading instruments.   
- Rapid growth led to large investments and user deposits.

![](images/2fe39cb485d2295873f405f6fb36c00c7be2e60d1e27431ba08cc97eea65b91f.jpg)

# FTX Case Study

# • Background

- Sam Bankman-Fried, the CEO of FTX, moved clients' deposit to his affiliated trading firm, Alameda Research for speculative cryptocurrency trading   
- This situation was reported by journalists in early 2022 Nov, and therefore a surge of customer withdrawals due to concerns over the mismanagement and lack of transparency in financial practices.   
- It pushed FTX and Alameda into bankruptcy and shook the volatile crypto market.   
- In December 2022, the U.S. government brought civil and criminal charges against Sam Bankman-Fried and top executives for misappropriating over $8 billion in customer deposits.   
- In March 2024, Bankman-Fried was convicted and sentenced to 25 years in prison for stealing $8 billion from customers.

# FTX Case Study - Counterparty risk

What is counterparty risk?

- The risk that the other party in a transaction may default on their obligations.   
• You have a profitable strategy and make lot of money, but unable to withdraw due to the broker's financing problem

Counterparty Risk Exposed

- FTX filed for bankruptcy in November 2022.   
- Customers lost billions due to misallocated funds and fraudulent practices   
- Many traders were unable to access their funds.   
- Highlighted the dangers of relying on a broker without sufficient oversight.

# FTX Case Study

![](images/762f8e38dba1f8ff885637311fdcbd06a070a7334d84c6f94f060dc660bc407c.jpg)

# Lessons Learned

# 1. Due Diligence

- Always research a broker's reputation and regulatory compliance   
- Look for transparency in operations and financial health

# 2. Regulation Matters

- Choose brokers regulated by reputable authorities to reduce risk   
- Understand the protections offered under those regulations

# 3. Diversification

- Avoid putting all money with a single broker   
- Spread risk across multiple platforms where possible

# Proof-Of-Reserves (PoR)

# Proof-Of-Reserves (PoR)

- PoR is a method used by financial institutions and exchanges to prove they hold sufficient reserves to back their liabilities.   
- The purpose is to enhance trust and transparency with investors.   
- For traditional brokerage firms, it requires third party auditing firm to verify and accredit the figures.   
- For crypto exchanges, Merkle Tree with blockchain is commonly used for PoR purpose nowadays.

# Merkle Tree

- A Merkle Tree is a data structure that allows efficient and secure verification of the integrity of large data sets.   
- Structure:

- Leaf nodes represent data (eg. transactions, positions, etc).   
Non-leaf nodes are hashes of their child nodes (eg. SHA-256)

Key Feature:

- Allows verification of data integrity without revealing the entire dataset.

![](images/d30977827f073ee5baf00751aaca6f372e0d39d7ac22332e9000b054e945fc1f.jpg)

# Merkle Tree

![](images/5c23a55f0c6cf6ea1ad911623623fd2d5dbed255491385eeabd46be0b9981774.jpg)

# How Merkle Trees Work

1. Data is hashed into leaf nodes.   
2. Leaf nodes are paired and hashed to form parent nodes.   
3. This process continues until a single hash (Merkle Root) is generated.   
4. The Merkle Root serves as a compact representation of all data.

# How Proof-of-Reserves Works

# - Audit Process:

- The institution publishes a Merkle Tree of its customer balances.   
- A third-party auditor verifies the accuracy of the Merkle Root.

# Verification:

- Customers can verify that their balance is included in the Merkle Tree without revealing sensitive information.   
- Ensures total reserves match or exceed customer liabilities.

# Reserve Ratio

- It measures the company's asset over liability (i.e. total client's deposit)   
- The ratio greater than 1 means the client's deposit is kept within the exchange   
- Reference: https://www.bitget.com/proofof-reserves

![](images/1b55c570df79864d38f3f621045345b6823dc5b79ae43f0f3bc1221e2829dc10.jpg)

![](images/39df70eee4849df5bf5c1de4c66e965eee5a18f6838a5a3602a58d1a562719b4.jpg)

Buy crypto

Markets

Trade

Futures

Copy

Bots

Earn

Web3

Launchhub

![](images/c7572b950e1b284e4912cb1a02f9c189772e5cec9b4916ac01166f36e09c267e.jpg)

Halloween

Log in

Sign up

![](images/4a852d81b4698289ffe0be0aec7779fb47c924b7dcf9eb98308fbe1e4bfd2bca.jpg)

![](images/fd38b4461c438077f71c29e41845e8f58bfb5dfb416b03ada7bfe5969f2c41b5.jpg)

# Latest reserve ratio

The reserve ratio is the platform's assets divided by the users' assets. A reserve ratio greater than or equal to $100\%$ means that the platform is capable of covering all user assets.

![](images/49e7c9977ca95e401ea6d654fbc5e0e51f3bf4525f8c0f0731d1205daf07f87a.jpg)

# Merkle root hash

The Merkle tree generated from all balances has 26 layers and 20,664,650 records.

e7d761e9426f537e

![](images/666827b5914d34a3a299b2de7dcbee8aabfedc84f20517fc32d29e902146581c.jpg)

# Total reserve ratio

Bitget holds more than $100\%$ of users' assets in BTC, ETH, USDT, and USDC.

166%

# Benefits of Using Merkle Trees in PoR

- Data Integrity: Ensures that the published data has not been tampered with.   
- Privacy: Customers can verify their holdings without exposing personal data.   
- Efficiency: Allows quick verification of large datasets.   
- Timeliness: Unlike traditional brokers which publish once per quarter, many crypto exchanges publish the data online every hour

# Algo Deployment with

# ALGOGENE

# Functionalities

- Trade and manage multiple broker accounts within a single platform   
- Seamlessly deploy an algo script to multiple broker accounts without code conversion   
Real-time data access from multiple brokers/exchanges

![](images/987a37f44c8483ea851557e232974eeb16296fb5158f6187a6ed89e511520dc2.jpg)

# Available brokers/exchanges

Supports $30+$ brokerage firms and crypto exchanges including but not limited to below   
Lifetime 5-10% commission discount if register using the partnership links

<table><tr><td>Alpaca</td><td>https://algogene.com/community/post/92</td></tr><tr><td>Binance</td><td>https://algogene.com/community/post/55</td></tr><tr><td>Bitget</td><td>https://algogene.com/community/post/99</td></tr><tr><td>Bybit</td><td>https://algogene.com/community/post/113</td></tr><tr><td>Exness</td><td>https://algogene.com/community/post/53</td></tr><tr><td>GO Markets</td><td>https://algogene.com/community/post/260</td></tr><tr><td>IG</td><td>https://algogene.com/community/post/15</td></tr><tr><td>Interactive Brokers</td><td>https://algogene.com/community/post/25</td></tr><tr><td>KuCoin</td><td>https://algogene.com/community/post/146</td></tr><tr><td>OKX</td><td>https://algogene.com/community/post/104</td></tr></table>

# Configuration steps

1. After login the platform, go to [Settings] page   
2. Go to [Trade Account] section   
3. Choose “Live Test” or “Real Trade”   
4. Click “Connect Broker”

![](images/ccbe962538c6df002398cda91db94bd93179bee2a5776419f10b9343c4e53b4e.jpg)

![](images/87b661fb219cf0a8788358bea93619b3447669ef7198f44709c2dc6385f27457.jpg)

![](images/8c68f4d52808c30e4d34b16766cd5962022af8f1ea7bfc96834cfb447981f50c.jpg)

![](images/2abe16f47779ef3330268be885cc4e993c2365a25e0b41cb56fac26201f8fa19.jpg)

![](images/0bcb92565ff98cc917373812ba6e3a7e19b6c169887ac7850d6b0953504f071d.jpg)

algogene.com/settings#divLivetrade

ALGOGENE

FinTech Co. Ltd.

![](images/8ecd5b40a4d32063b1c36075fca411a6e6ff6231099ffee72651b757e29fabe9.jpg)

Search community topic ...

![](images/ea5bfe7435b295217f1e445f6bbcd8a46b6f0f97bd48d23fda6b279e271a2f1c.jpg)

Profile

![](images/9cdedda1b2f79272bd59f62130ff0fc27fdad2b715c66b375b8803c1bd855262.jpg)

My Wallet

![](images/380cf5fd5025c3f1ddfd9fdf4a4ff7d57c9f6825852bb45ad52a3cee58f39954.jpg)

My Orders

![](images/a3346b624d3c542619554e523875e6e2c7a5aff9b3b7d4e5878f35e82b96126d.jpg)

Algo Market

![](images/6831936c9d0b916e399197c6ac68fbe8ff14b973f775ecee150e60e32879977f.jpg)

Trade Account

![](images/3bacffd1b281dea17f7fe2ba0bd54ddc97e8ef142ca6f2473f23ff1771e1a76d.jpg)

Affiliate

![](images/6f5b52ec9b1194164506fd685bd60beff556cd0c28073b203c23e81d6e7569ee.jpg)

Notification

![](images/906796c5c6a12d44bf1949fd71085e27390b3a90f6050c155a927e1678a77631.jpg)

Trading Account Management

![](images/95e989bcba7435aefde64e506b3cf3795ae8478b2e93cf309d4ba24cbe103470.jpg)

1001

INACTIVE

Broker:

Base Currency:

Leverage:

NAV

Running Script:

Subscription End

![](images/66c7086258da191bdc9131d7a827e7d77236b0d28b41d2f0973bb2ab957571d9.jpg)

![](images/076cb80ddd68c41597db0220c9ca60f4792690c5a709cb9226890168a145b3e5.jpg)

Connect Broker

![](images/42437b2e0f492203a410720346fc5ca1ba7a75f3ac60b33130d1a10c40b739b9.jpg)

Webhook Setup

![](images/72bab93b92d4c71ffd87e8524ce318abe4ad87364b0e3c1a101027d0592f0f37.jpg)

Download Statement

![](images/9c526371f1e78bff96839bb89b1b724747316d21636ab56e7c83ad4e396652fc.jpg)

Strategy Configs

![](images/aa9c1750ec7a1143b80fd5223fed7fe282076ab3a3b81477c5e3bf5f27d31aec.jpg)

![](images/8470e3c592bf1d75b3bb5e2e62f127aaae6059a1ab49982924e883f91cf66cfe.jpg)

# Configuration steps

5. Select your broker   
6. Update with your API info   
7. After successful connection, the broker's account info will be synchronized on the platform

# Real Trade Connection Setup

Account ID:

Account Currency:

Account Leverage:

NAV:

Running Script:

1001

Your Account Currency

1.0

0.0

No Running Script

Realized PnL:

Unrealized PnL:

Margin Used:

Available Balance:

Script Author:

0.0

0.0

0.0

0.0

None

Run Mode*:

Broker*:

API Key*:

Password*:

![](images/0b7ea6b12c3b1194eff1feec945b3a4d374d5a7ef3e4f2373e90280d56094ab1.jpg)

Robo-Trader ②

![](images/d412dd55d2b1814fa1df9c28ff78d5e406dd1b6ab2ff7a143bc161dccd523b17.jpg)

Robo-Advisor ②

<table><tr><td>Binance</td><td>✓</td></tr><tr><td>API Key</td><td></td></tr><tr><td>API Secret</td><td></td></tr></table>

* indicates for required field

Update

# Configuration steps

8. Click “Strategy Configs”, then “Execute My Algo” to deploy a backtest script to the connected broker account

ALGOGENE

FinTech Co. Ltd.

![](images/3cc30852188107a43bf51eb4ad8b3e5858e4c96c7a323b74335c8a6011f80f6f.jpg)

Profile

![](images/0a7eb4d1a26d73031be2bf6bcd3b7ed9e5a4818fd653edd5962208bcb36bed31.jpg)

My Wallet

![](images/911a1e8af12c68989ff95bf4f97f226a05fc8a2fb726a998851cdac77f8bb91a.jpg)

My Orders

![](images/8e5c0ac9c35e9348d1bc193ca5280f37d58261b185de73e80c95e0b005e38cea.jpg)

Algo Market

![](images/374cf029faf7c268952ee6f9f9657c7750d05d1e0a6dbef6d7f170b0f91a4830.jpg)

Trade Account

![](images/ee7036aeb54c807f5b8525697d3d08f5e235742a71505ab9a4233960e4514f47.jpg)

Affiliate

![](images/a75e9f36ceee1de4bd212a70af30d1d57ab3627d08265afbc3df316d28460616.jpg)

Notification

![](images/ad2023a65c9e6b61fe4d38c8a325f00ad39130e044b92a05e8fff478c473308c.jpg)

search community topic.

![](images/5179d3c65d86468d1a738189877f36252bc0e0809be3f9a1bc42e20ca83022e9.jpg)

Trading Account Management

Live Test

Real Trade

#

#

1001

INACTIVE

Broker:

Base Currency:

Leverage:

NAV:

Running Script:

Subscription End

2025-06-30 00:00:00

![](images/e4575145cb4dace7084b08b83361875d9108c1e1b808c000df00817086b3c6b7.jpg)

![](images/f7a6ffbf841064aeb9eae70841b09c9b91149b90a51ba57c3f11209bb1f2c25b.jpg)

Connect Broker

![](images/7157b96fa0dbd96cbee2b64f54cf2c5938f8599039ca652f17b4981f78cf29d5.jpg)

Webhook Setup

![](images/b53890f2bb16475e22e8e4a9b6189a0fed0c4a7d0c6e213433a81f63637e2710.jpg)

Download Statement

![](images/806c9d92ac568fce543b09889c0a5b2ad492b58c7214acdd0eb25f61e592af3f.jpg)

Strategy Configs

Execute My Algo

Share Signal

Copy Trade

# Market Tricks

# Understand the business world

- Some traders and investment firms may intentionally present their performance in certain ways to attract investment or increase sales   
Examples:

- Misleading metrics   
- Selective presentation of data   
- Psychological manipulation

# Misleading Metrics - Example

- Winning Rate Definition:

1. the percentage of profitable trades, measured from a round order perspective (misleading)   
2. the percentage of trading day that end up with a gain (more appropriate)

Using the misleading definition

- Tricks:

- Firms may highlight a high winning rate without disclosing average losses on losing trades.   
- Only close profitable trades but leave losing trades open to achieve a high winning rate

- Impact: a high winning rate does not necessarily indicate overall profitability.

Definition 2 is more applicable to both trade-based and position-based system

# Winning Rate - Example

- This trade has 4 consecutive loss days, but finally ends up with a big gain.   
• Using definition 1, as there is only 1 profitable trade, so winning rate is $100\%$   
• Using definition 2, the winning rate is $1 / 5 = 20\%$ .

<table><tr><td>Day</td><td>Closing Price</td><td>Daily PnL</td></tr><tr><td>0</td><td>110</td><td>0</td></tr><tr><td>1</td><td>108</td><td>-2</td></tr><tr><td>2</td><td>105</td><td>-3</td></tr><tr><td>3</td><td>104</td><td>-1</td></tr><tr><td>4</td><td>100</td><td>-4</td></tr><tr><td>5</td><td>130</td><td>30</td></tr><tr><td colspan="2">Total PnL</td><td>20</td></tr></table>

# Selective Presentation – Example 1

- The formula of Sharpe ratio is

assume risk free rate $= 0$

$$
\text {S h a r p e R a t i o} = \frac {\text {A n n u a l i z e d R e t u r n}}{\text {A n n u a l i z e d V o l a t i l i t y}}
$$

- Trick: Firms may present a high Sharpe Ratio by using short time frames with low volatility.   
- Impact: Misleading risk assessment; may not reflect long-term performance.

![](images/24d222b3f1ab902be24a4826bb97d313d965d7c69508bba593f18297a1ca3ef6.jpg)

# Selective Presentation – Example 2

The formula of Sharpe ratio is SharpeRatio = $\frac{\text{Annualized Return}}{\text{Annualized Volatility}}$   
It can be calculated using different ways.

- For daily data,

$$
S h a r p e R a t i o = \frac {D a i l y R e t u r n}{D a i l y V o l a t i l i t y} \sqrt {2 5 2}
$$

- For weekly data,

$$
S h a r p e R a t i o = \frac {W e a k l y R e t u r n}{W e a k l y V o l a t i l i t y} \sqrt {5 2}
$$

• For monthly data,

$$
S h a r p e R a t i o = \frac {M o n t h l y R e t u r n}{M o n t h l y V o l a t i l i t y} \sqrt {1 2}
$$

- Trick: Firms presents the one that produce the highest value.   
- Impact: Not an apple-to-apple comparison between different investment products

assume risk free rate $= 0$

# Psychological Manipulation: "Fear of Missing Out" (FOMO) 18:44

- Leverage the emotional response of potential investors to create urgency and drive decisions.   
Example:

- Promotional Campaign:   
- Claim: "Join the smart investors who gained $30 \%$ in just six months!"   
Tactics Used:   
- Urgency: Limited-time offers or exclusive access to "top-performing" funds.   
- Social Proof: Showcasing testimonials from satisfied clients who experienced high returns.   
- Imagery: Using visuals of luxury lifestyles (e.g., cars, vacations) to evoke desire.

- Impact:

- Investors may rush to invest out of fear of missing potential gains.   
- Emotional decisions often overshadow logical analysis, leading to investments in high-risk or unsuitable products.

![](images/a0e1877b8be0825d19a103c75b0afa4386e3d8c9a39cbe887b5aace82f511838.jpg)

# Conclusion & Advices

- Always look beyond the surface metrics.   
- Consider full context: time frames, benchmarks, and risk measures.   
- Perform due diligence and seek clarity on performance metrics.   
- Remain rational and avoid rush investment decisions driven by emotion.

# Key Takeaways

• 3 strategy optimization methods, including brute force, gradient descent, genetic algorithm   
- Choose the parameter set that yield a robust output is more appropriate than choosing the optimal solution   
- There are various type of investment funds in the market designed to cater for different investor needs.   
- Hedge funds deploy a large number of uncorrelated strategies to achieve long winning track records   
- Important things to consider when choosing a trading broker   
- Common sales tricks used by traders/ investment firms
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
