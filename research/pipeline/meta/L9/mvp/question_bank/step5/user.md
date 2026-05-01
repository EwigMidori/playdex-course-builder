请基于以下 lesson 材料，生成一个结构化题库 JSON。

目标语言：
zh-CN

lesson_id：
L9

要求：
- 同时生成 `flashcard_families`、`quiz_families` 和 `longform_families`
- 题目必须只关联到当前 step：`step5`
- 所有 family 和 variant 的 `linked_steps` 都必须等于 `["step5"]`
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

<CURRENT_STEP_ID>
step5
</CURRENT_STEP_ID>

<CURRENT_STEP>
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
}

</CURRENT_STEP>

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

请直接输出 JSON，不要加解释。
