请基于以下材料，生成一份 lesson 级 MDX 课本。

目标语言：
zh-CN

lesson_id：
L8

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
# L8: Order book and high frequency data modeling
Course Code: COMP7415
# Agenda
- Trading ideas and insights from an order book
Market impact of a large order
- Queuing theory for order book modelling
• Introduction to high frequency trading
Major types of arbitrage
- Triangular Arbitrage Strategy
- Market Making Strategy
# Revision of Order Book
- Order book is a list of buy and sell orders
- Bid book represents the demand side
- Ask book represents the supply side
- An order book can provide insights for microeconomic structure
![](images/1890d1b4de992ca5c1c0849ef2f8bf35661f3778212c240b4f34e7c584840660.jpg)
Order Book
<table><tr><td></td><td>Bid</td><td>Ask</td><td></td></tr><tr><td>3,151</td><td>9139.60</td><td>9152.80</td><td>167</td></tr><tr><td>1,574</td><td>9138.40</td><td>9153.21</td><td>1,600</td></tr><tr><td>338</td><td>9138.20</td><td>9153.60</td><td>736</td></tr><tr><td>789</td><td>9138.10</td><td>9153.71</td><td>1,602</td></tr><tr><td>800</td><td>9138.00</td><td>9154.10</td><td>1,649</td></tr><tr><td>309</td><td>9137.79</td><td>9154.21</td><td>1,965</td></tr><tr><td>2,140</td><td>9137.60</td><td>9154.50</td><td>3,984</td></tr><tr><td>48</td><td>9137.40</td><td>9154.60</td><td>763</td></tr><tr><td>144</td><td>9137.29</td><td>9154.71</td><td>157</td></tr><tr><td>136</td><td>9137.20</td><td>9154.80</td><td>56</td></tr><tr><td>528</td><td>9137.00</td><td>9155.50</td><td>320</td></tr><tr><td>1,654</td><td>9136.90</td><td>9155.71</td><td>200</td></tr><tr><td>40</td><td>9136.79</td><td>9156.10</td><td>16</td></tr><tr><td>46</td><td>9136.70</td><td>9156.21</td><td>219</td></tr><tr><td>608</td><td>9136.60</td><td>9156.30</td><td>40</td></tr><tr><td>48</td><td>9136.50</td><td>9156.40</td><td>80</td></tr></table>
# Breakdown an Order Book
![](images/be1d9dd9583cc58d1bd249941340f7a1d8d070de10531a5d686b7ca5e950ec99.jpg)
Level 1
![](images/c13567b7d72046003a7ff73888f9489e3f61d1de71c48c4a337ace79af3f9679.jpg)
Level 2
![](images/73f455bfb0c450ad79e0403359ca6fa2a0901fcad6dcad3faeb9f93f9af88efb.jpg)
Level 3
# Order Book Mechanism
- Limit orders operated under a price-time priority rule.
- Limit orders are sorted by the price and higher priority is given to the orders at the best prices
- Orders at the same price are ranked depending on when they entered the queue according to FIFO
![](images/4f370778225b8f47bb8707a3c2ae2d152a2d63c4ef06f1bcc675dbd8fce3af67.jpg)
# What an order book can tell you?
1. Supply & Demand: Insights into trader behaviour and market sentiment.
2. Support & Resistance Level: Insights from orders with large volume
3. Market Liquidity: Measure liquidity through order sizes.
4. Market Depth: Analysis of the number of buy/sell orders at various price levels.
# Trading Strategies Based on Order Book Data
# 1. Momentum Trading:
Identifying trends through order flow signals.
- Monitor the ratio of buy to sell orders. A significant imbalance can signal potential price movements.
# 2. Sniping Strategy:
- Taking advantage of large orders (eg. iceberg orders) before they get fully executed.
# 3. Liquidity Provisioning:
- Market-making strategies to earn from the bid-ask spread.
# Market Impact
# Market Impact
A large market order is likely to fill up the order book and lead to
- significant price movement
- reduced market depth and liquidity
- Immediate Impact: Price change upon execution of the order.
- Temporary Impact: Short-term fluctuations that revert after order execution.
- Permanent Impact: Lasting price changes due to order flow
![](images/e48e94161c73aa4fc59bbf27b0baf348a33f213143f2a2ebac43f58c1bd580ff.jpg)
![](images/28ff73b6a0696bb14e363a757f31d2c67fc1fb232c4f84472041cdc29241ed34.jpg)
# Factors Influencing Market Impact
# 1. Order Size:
- Larger orders typically lead to greater impact
# 2. Order Type:
Market orders vs limit orders
# 3. Market Conditions:
Less liquid markets experience higher impact
</COVERAGE_CHECKLIST>

<SOURCE_OUTLINE>
# L8: Order book and high frequency data modeling
Course Code: COMP7415
# Agenda
- Trading ideas and insights from an order book
Market impact of a large order
- Queuing theory for order book modelling
• Introduction to high frequency trading
Major types of arbitrage
- Triangular Arbitrage Strategy
- Market Making Strategy
# Revision of Order Book
- Order book is a list of buy and sell orders
- Bid book represents the demand side
- Ask book represents the supply side
- An order book can provide insights for microeconomic structure
![](images/1890d1b4de992ca5c1c0849ef2f8bf35661f3778212c240b4f34e7c584840660.jpg)
Order Book
<table><tr><td></td><td>Bid</td><td>Ask</td><td></td></tr><tr><td>3,151</td><td>9139.60</td><td>9152.80</td><td>167</td></tr><tr><td>1,574</td><td>9138.40</td><td>9153.21</td><td>1,600</td></tr><tr><td>338</td><td>9138.20</td><td>9153.60</td><td>736</td></tr><tr><td>789</td><td>9138.10</td><td>9153.71</td><td>1,602</td></tr><tr><td>800</td><td>9138.00</td><td>9154.10</td><td>1,649</td></tr><tr><td>309</td><td>9137.79</td><td>9154.21</td><td>1,965</td></tr><tr><td>2,140</td><td>9137.60</td><td>9154.50</td><td>3,984</td></tr><tr><td>48</td><td>9137.40</td><td>9154.60</td><td>763</td></tr><tr><td>144</td><td>9137.29</td><td>9154.71</td><td>157</td></tr><tr><td>136</td><td>9137.20</td><td>9154.80</td><td>56</td></tr><tr><td>528</td><td>9137.00</td><td>9155.50</td><td>320</td></tr><tr><td>1,654</td><td>9136.90</td><td>9155.71</td><td>200</td></tr><tr><td>40</td><td>9136.79</td><td>9156.10</td><td>16</td></tr><tr><td>46</td><td>9136.70</td><td>9156.21</td><td>219</td></tr><tr><td>608</td><td>9136.60</td><td>9156.30</td><td>40</td></tr><tr><td>48</td><td>9136.50</td><td>9156.40</td><td>80</td></tr></table>
# Breakdown an Order Book
![](images/be1d9dd9583cc58d1bd249941340f7a1d8d070de10531a5d686b7ca5e950ec99.jpg)
Level 1
![](images/c13567b7d72046003a7ff73888f9489e3f61d1de71c48c4a337ace79af3f9679.jpg)
Level 2
![](images/73f455bfb0c450ad79e0403359ca6fa2a0901fcad6dcad3faeb9f93f9af88efb.jpg)
Level 3
# Order Book Mechanism
- Limit orders operated under a price-time priority rule.
- Limit orders are sorted by the price and higher priority is given to the orders at the best prices
- Orders at the same price are ranked depending on when they entered the queue according to FIFO
![](images/4f370778225b8f47bb8707a3c2ae2d152a2d63c4ef06f1bcc675dbd8fce3af67.jpg)
# What an order book can tell you?
1. Supply & Demand: Insights into trader behaviour and market sentiment.
2. Support & Resistance Level: Insights from orders with large volume
3. Market Liquidity: Measure liquidity through order sizes.
4. Market Depth: Analysis of the number of buy/sell orders at various price levels.
# Trading Strategies Based on Order Book Data
# 1. Momentum Trading:
Identifying trends through order flow signals.
- Monitor the ratio of buy to sell orders. A significant imbalance can signal potential price movements.
# 2. Sniping Strategy:
- Taking advantage of large orders (eg. iceberg orders) before they get fully executed.
# 3. Liquidity Provisioning:
- Market-making strategies to earn from the bid-ask spread.
# Market Impact
# Market Impact
A large market order is likely to fill up the order book and lead to
- significant price movement
- reduced market depth and liquidity
- Immediate Impact: Price change upon execution of the order.
- Temporary Impact: Short-term fluctuations that revert after order execution.
- Permanent Impact: Lasting price changes due to order flow
![](images/e48e94161c73aa4fc59bbf27b0baf348a33f213143f2a2ebac43f58c1bd580ff.jpg)
![](images/28ff73b6a0696bb14e363a757f31d2c67fc1fb232c4f84472041cdc29241ed34.jpg)
# Factors Influencing Market Impact
# 1. Order Size:
- Larger orders typically lead to greater impact
# 2. Order Type:
Market orders vs limit orders
# 3. Market Conditions:
Less liquid markets experience higher impact
</SOURCE_OUTLINE>

<LESSON_MAP>
{
  "lesson_id": "L8",
  "mode": "guided_story",
  "steps": [
    {
      "concept": "Order Book Basics and Insights",
      "file": "research/pipeline/3-guided_story/L8/step1/step.json",
      "sequence_id": "step1"
    },
    {
      "concept": "Market Impact and Mitigation Strategies",
      "file": "research/pipeline/3-guided_story/L8/step2/step.json",
      "sequence_id": "step2"
    },
    {
      "concept": "Queuing Theory for Order Book Modeling",
      "file": "research/pipeline/3-guided_story/L8/step3/step.json",
      "sequence_id": "step3"
    },
    {
      "concept": "High Frequency Trading and Arbitrage",
      "file": "research/pipeline/3-guided_story/L8/step4/step.json",
      "sequence_id": "step4"
    }
  ]
}

</LESSON_MAP>

<GUIDED_STORY_MANIFEST>
{
  "lesson_id": "L8",
  "mode": "guided_story",
  "steps": [
    {
      "concept": "Order Book Basics and Insights",
      "file": "research/pipeline/3-guided_story/L8/step1/step.json",
      "sequence_id": "step1"
    },
    {
      "concept": "Market Impact and Mitigation Strategies",
      "file": "research/pipeline/3-guided_story/L8/step2/step.json",
      "sequence_id": "step2"
    },
    {
      "concept": "Queuing Theory for Order Book Modeling",
      "file": "research/pipeline/3-guided_story/L8/step3/step.json",
      "sequence_id": "step3"
    },
    {
      "concept": "High Frequency Trading and Arbitrage",
      "file": "research/pipeline/3-guided_story/L8/step4/step.json",
      "sequence_id": "step4"
    }
  ]
}

</GUIDED_STORY_MANIFEST>

<GUIDED_STORY_STEPS>
[
  {
    "lesson_id": "L8",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s001",
        "introduced_terms": [],
        "lines": [
          "想象一下，你面前有一块巨大的电子公告板。",
          "上面实时滚动着所有人想买和想卖的东西。"
        ],
        "type": "narration"
      },
      {
        "id": "s002",
        "introduced_terms": [
          "order_book"
        ],
        "lines": [
          "想买的人出价，想卖的人要价。",
          "这块公告板，就是市场的 <term id=\"order_book\">订单簿</term>。"
        ],
        "type": "narration"
      },
      {
        "id": "s003",
        "introduced_terms": [
          "bid_book",
          "ask_book"
        ],
        "lines": [
          "订单簿分为两边：",
          "<term id=\"bid_book\">买盘簿</term> 代表需求，<term id=\"ask_book\">卖盘簿</term> 代表供给。"
        ],
        "type": "narration"
      },
      {
        "id": "s004",
        "lines": [
          "它不只是个列表。",
          "它能告诉你市场的微观结构——谁在着急买，谁在急着卖。"
        ],
        "type": "narration"
      },
      {
        "id": "s005",
        "lines": [
          "比如，某个价格上堆了巨大的买单。",
          "那可能就是一个强力的支撑位。"
        ],
        "type": "narration"
      },
      {
        "id": "s006",
        "introduced_terms": [
          "market_liquidity"
        ],
        "lines": [
          "通过观察订单簿，你可以判断 <term id=\"market_liquidity\">市场流动性</term>。",
          "订单越多、越密集，流动性越好。"
        ],
        "type": "narration"
      },
      {
        "id": "s007",
        "introduced_terms": [
          "market_depth"
        ],
        "lines": [
          "你还能看到 <term id=\"market_depth\">市场深度</term>。",
          "深度越厚，大单对价格的冲击就越小。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "买盘簿记录了所有买入意愿，代表市场的需求方。",
          "kind": "single_choice",
          "options": [
            "卖盘簿 (Ask Book)",
            "买盘簿 (Bid Book)",
            "成交记录 (Trade History)",
            "价格深度 (Price Depth)"
          ],
          "prompt": "订单簿的哪一边代表市场的需求？"
        },
        "id": "s008",
        "lines": [
          "订单簿的哪一边代表市场的需求？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step1",
    "source": {
      "plain_text": "Order book is a list of buy and sell orders. Bid book represents the demand side. Ask book represents the supply side. An order book can provide insights for microeconomic structure. What an order book can tell you: 1. Supply & Demand: Insights into trader behaviour and market sentiment. 2. Support & Resistance Level: Insights from orders with large volume 3. Market Liquidity: Measure liquidity through order sizes. 4. Market Depth: Analysis of the number of buy/sell orders at various price levels.",
      "related": [
        "order book mechanism",
        "limit order book"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "ask_book": {
        "aliases": [
          "Ask Book",
          "Ask Side",
          "Offer Book"
        ],
        "display": "卖盘簿",
        "gloss": "订单簿中所有卖出订单的集合。"
      },
      "bid_book": {
        "aliases": [
          "Bid Book",
          "Bid Side"
        ],
        "display": "买盘簿",
        "gloss": "订单簿中所有买入订单的集合。"
      },
      "market_depth": {
        "aliases": [
          "Market Depth"
        ],
        "display": "市场深度",
        "gloss": "在不同价格水平上可交易的订单数量。"
      },
      "market_liquidity": {
        "aliases": [
          "Market Liquidity",
          "Liquidity"
        ],
        "display": "市场流动性",
        "gloss": "在不引起价格大幅波动的情况下，买卖资产的难易程度。"
      },
      "order_book": {
        "aliases": [
          "Order Book",
          "Orderbook"
        ],
        "display": "订单簿",
        "gloss": "记录所有买入和卖出挂单的电子列表。"
      }
    }
  },
  {
    "lesson_id": "L8",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s001",
        "lines": [
          "如果你要买一万股，直接下个市价单会怎样？",
          "你会一路吃掉卖盘，把价格推高。"
        ],
        "type": "narration"
      },
      {
        "id": "s002",
        "introduced_terms": [
          "market_impact"
        ],
        "lines": [
          "这种由订单本身引起的价格不利变动，叫 <term id=\"market_impact\">市场冲击</term>。",
          "大单的冲击尤其明显。"
        ],
        "type": "narration"
      },
      {
        "id": "s003",
        "introduced_terms": [
          "order_splitting"
        ],
        "lines": [
          "怎么减少冲击？第一个办法：<term id=\"order_splitting\">订单拆分</term>。",
          "把一万股拆成一百个一百股，慢慢买。"
        ],
        "type": "narration"
      },
      {
        "id": "s004",
        "lines": [
          "第二个办法：用限价单，而不是市价单。",
          "限价单能控制成交价，但可能无法立即成交。"
        ],
        "type": "narration"
      },
      {
        "id": "s005",
        "introduced_terms": [
          "twap"
        ],
        "lines": [
          "第三个办法：<term id=\"twap\">时间加权平均价格</term>。",
          "比如，在1小时内，每5分钟买入固定数量。"
        ],
        "type": "narration"
      },
      {
        "id": "s006",
        "lines": [
          "TWAP 假设价格是稳定的。",
          "如果市场波动很大，效果会打折扣。"
        ],
        "type": "narration"
      },
      {
        "id": "s007",
        "introduced_terms": [
          "vwap"
        ],
        "lines": [
          "第四个办法：<term id=\"vwap\">成交量加权平均价格</term>。",
          "VWAP 根据市场的实际成交量来安排你的订单。"
        ],
        "type": "narration"
      },
      {
        "id": "s008",
        "lines": [
          "市场成交量大的时候，你就多买一点。",
          "成交量小的时候，你就少买一点。"
        ],
        "type": "narration"
      },
      {
        "id": "s009",
        "lines": [
          "这样，你的订单就像融入了市场本身。",
          "冲击自然就小了。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "订单拆分将大额订单分解为多个小额订单，以减少对市场的即时冲击。",
          "kind": "single_choice",
          "options": [
            "使用市价单",
            "订单拆分",
            "使用止损单",
            "增加订单数量"
          ],
          "prompt": "以下哪种策略是通过将大单拆小来降低市场冲击？"
        },
        "id": "s010",
        "lines": [
          "以下哪种策略是通过将大单拆小来降低市场冲击？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step2",
    "source": {
      "plain_text": "Market Impact: A large market order is likely to fill up the order book and lead to significant price movement, reduced market depth and liquidity. Strategies to Mitigate Market Impact: 1. Order Splitting: Break large orders into smaller segments. Execute over time to reduce immediate impact. 2. Use of Limit Orders: Place limit orders to control execution price. Avoid the risks of market orders. 3. Time-weighted Average Price (TWAP): Execute orders evenly over a specified time period. Helps in minimizing market impact. 4. Volume-weighted Average Price (VWAP): Execute orders in proportion to market volume. Reduces price impact by blending into the market.",
      "related": [
        "order splitting example",
        "TWAP example",
        "VWAP example"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "market_impact": {
        "aliases": [
          "Market Impact",
          "Slippage"
        ],
        "display": "市场冲击",
        "gloss": "执行大额订单时，因订单本身导致的价格不利变动。"
      },
      "order_splitting": {
        "aliases": [
          "Order Splitting",
          "Iceberg Order"
        ],
        "display": "订单拆分",
        "gloss": "将大额订单拆分成多个小额订单，以降低市场冲击。"
      },
      "twap": {
        "aliases": [
          "TWAP",
          "Time-weighted Average Price"
        ],
        "display": "时间加权平均价格",
        "gloss": "将订单均匀分配到一段时间内执行，以降低市场冲击。"
      },
      "vwap": {
        "aliases": [
          "VWAP",
          "Volume-weighted Average Price"
        ],
        "display": "成交量加权平均价格",
        "gloss": "根据市场成交量比例执行订单，使成交价接近市场均价。"
      }
    }
  },
  {
    "lesson_id": "L8",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s001",
        "lines": [
          "订单簿里的订单，就像在排队。",
          "先来的先成交，后来的等着。"
        ],
        "type": "narration"
      },
      {
        "id": "s002",
        "introduced_terms": [
          "queuing_theory"
        ],
        "lines": [
          "我们可以用 <term id=\"queuing_theory\">排队论</term> 来分析这个排队过程。",
          "它能回答：一个订单要等多久才能成交？"
        ],
        "type": "narration"
      },
      {
        "id": "s003",
        "introduced_terms": [
          "arrival_rate",
          "service_rate"
        ],
        "lines": [
          "排队论有两个关键参数：",
          "<term id=\"arrival_rate\">到达率</term> 和 <term id=\"service_rate\">服务率</term>。"
        ],
        "type": "narration"
      },
      {
        "id": "s004",
        "lines": [
          "到达率 (λ) 是每秒有多少新订单进来。",
          "服务率 (μ) 是每秒有多少订单被成交。"
        ],
        "type": "narration"
      },
      {
        "id": "s005",
        "lines": [
          "如果到达率大于服务率 (λ > μ)，队伍会越来越长。",
          "如果到达率小于服务率 (λ < μ)，系统才能稳定。"
        ],
        "type": "narration"
      },
      {
        "id": "s006",
        "introduced_terms": [
          "traffic_intensity"
        ],
        "lines": [
          "它们的比值 ρ = λ / μ，叫 <term id=\"traffic_intensity\">交通强度</term>。",
          "ρ 越接近 1，系统越拥挤。"
        ],
        "type": "narration"
      },
      {
        "id": "s007",
        "lines": [
          "在订单簿里，一个限价单从挂单到成交，就是一次完整的排队服务。",
          "排队论能帮我们估算这个等待时间。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answers": [
            "0.67",
            "2/3"
          ],
          "explanation": "ρ = λ / μ = 10 / 15 = 0.67。系统是稳定的，因为 ρ < 1。",
          "kind": "fill_in_blank",
          "prompt": "如果订单到达率 λ = 10 单/分钟，服务率 μ = 15 单/分钟，交通强度 ρ 是多少？"
        },
        "id": "s008",
        "lines": [
          "如果订单到达率 λ = 10 单/分钟，服务率 μ = 15 单/分钟，交通强度 ρ 是多少？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step3",
    "source": {
      "plain_text": "Why Queuing Theory? To analyze how orders flow through the market and how they impact price movement. Basic Concepts: Arrival Process (λ): Rate at which an event arrives. Service Rate (μ): Rate at which an event is executed. Queue Discipline: How events are processed (e.g. FIFO). How is it related to Order Book? Incoming orders (buy/sell) can be viewed as entities arriving at a queue. The order book acts as a queue where these orders wait to be matched. Each order can be treated as a customer in a queue, where the \"service\" is the execution of trades.",
      "related": [
        "M/M/1 queue",
        "Poisson process",
        "Exponential distribution"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "arrival_rate": {
        "aliases": [
          "Arrival Rate",
          "λ"
        ],
        "display": "到达率",
        "gloss": "单位时间内新订单到达市场的平均数量。"
      },
      "queuing_theory": {
        "aliases": [
          "Queuing Theory",
          "Queueing Theory"
        ],
        "display": "排队论",
        "gloss": "研究等待队列的数学理论，用于分析订单的等待和执行时间。"
      },
      "service_rate": {
        "aliases": [
          "Service Rate",
          "μ"
        ],
        "display": "服务率",
        "gloss": "单位时间内订单被成交的平均数量。"
      },
      "traffic_intensity": {
        "aliases": [
          "Traffic Intensity",
          "ρ",
          "Utilization"
        ],
        "display": "交通强度",
        "gloss": "到达率与服务率的比值，衡量系统繁忙程度。"
      }
    }
  },
  {
    "lesson_id": "L8",
    "mode": "guided_story",
    "screens": [
      {
        "id": "s001",
        "introduced_terms": [
          "high_frequency_trading"
        ],
        "lines": [
          "当交易速度快到毫秒甚至微秒级别，就进入了 <term id=\"high_frequency_trading\">高频交易</term> 的领域。",
          "目标是抓住转瞬即逝的微小价差。"
        ],
        "type": "narration"
      },
      {
        "id": "s002",
        "introduced_terms": [
          "co_location"
        ],
        "lines": [
          "速度就是生命。",
          "为了快，交易公司会把服务器搬到交易所旁边，这叫 <term id=\"co_location\">主机托管</term>。"
        ],
        "type": "narration"
      },
      {
        "id": "s003",
        "introduced_terms": [
          "latency_arbitrage"
        ],
        "lines": [
          "比别人快几微秒，就能看到订单流，提前行动。",
          "这就是 <term id=\"latency_arbitrage\">延迟套利</term>。"
        ],
        "type": "narration"
      },
      {
        "id": "s004",
        "introduced_terms": [
          "arbitrage"
        ],
        "lines": [
          "高频交易的核心策略之一：<term id=\"arbitrage\">套利</term>。",
          "利用不同市场或资产间的价格差异赚钱。"
        ],
        "type": "narration"
      },
      {
        "id": "s005",
        "introduced_terms": [
          "triangular_arbitrage"
        ],
        "lines": [
          "比如 <term id=\"triangular_arbitrage\">三角套利</term>：",
          "用美元换欧元，欧元换英镑，英镑再换回美元。"
        ],
        "type": "narration"
      },
      {
        "id": "s006",
        "lines": [
          "如果一圈下来，钱变多了，就存在套利机会。",
          "但这需要极快的计算和执行。"
        ],
        "type": "narration"
      },
      {
        "id": "s007",
        "introduced_terms": [
          "market_making"
        ],
        "lines": [
          "另一种常见策略是 <term id=\"market_making\">做市</term>。",
          "同时挂买单和卖单，赚取买卖价差。"
        ],
        "type": "narration"
      },
      {
        "id": "s008",
        "lines": [
          "做市商为市场提供流动性，交易所有时还会给返佣。",
          "但风险是，如果市场单边暴涨暴跌，做市商可能亏钱。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "这称为主机托管，目的是将物理距离缩到最短，从而获得毫秒甚至微秒级的速度优势。",
          "kind": "single_choice",
          "options": [
            "节省电费",
            "减少网络延迟",
            "方便维护",
            "获得更多数据"
          ],
          "prompt": "高频交易公司把服务器放在交易所数据中心，主要是为了什么？"
        },
        "id": "s009",
        "lines": [
          "高频交易公司把服务器放在交易所数据中心，主要是为了什么？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step4",
    "source": {
      "plain_text": "High Frequency Trading: Rapid executions of multiple transactions followed by extremely short holding periods. Profit from small price changes and discrepancies. Require sophisticated technology, communications and computing resources. High frequency trading depends heavily on execution speed and latency. Co-location: High frequency trading firms usually rent space from market centers, such as from the major exchanges. The purpose to put their servers next to the market center's data servers is to reduce latency. Latency Arbitrage: Exploiting delays in market data transmission to execute trades before other market participants can react. Common High Frequency Strategy: 1. Market Making 2. Latency Arbitrage 3. Cross Market Arbitrage 4. Order Flow Analysis. Triangular Arbitrage: Triangular arbitrage is a trading strategy that exploits discrepancies in currency exchange rates.",
      "related": [
        "co-location example",
        "triangular arbitrage example",
        "market making strategy"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "arbitrage": {
        "aliases": [
          "Arbitrage"
        ],
        "display": "套利",
        "gloss": "在不同市场或资产间同时买卖，利用价格差异获利。"
      },
      "co_location": {
        "aliases": [
          "Co-location",
          "Colo"
        ],
        "display": "主机托管",
        "gloss": "将交易服务器放置在交易所数据中心内，以减少网络延迟。"
      },
      "high_frequency_trading": {
        "aliases": [
          "HFT",
          "High Frequency Trading"
        ],
        "display": "高频交易",
        "gloss": "利用极快速度执行大量交易，从微小价差中获利的交易方式。"
      },
      "latency_arbitrage": {
        "aliases": [
          "Latency Arbitrage"
        ],
        "display": "延迟套利",
        "gloss": "利用信息传输速度差异，比对手更快交易以获利。"
      },
      "market_making": {
        "aliases": [
          "Market Making"
        ],
        "display": "做市",
        "gloss": "同时挂出买单和卖单，通过赚取买卖价差获利。"
      },
      "triangular_arbitrage": {
        "aliases": [
          "Triangular Arbitrage"
        ],
        "display": "三角套利",
        "gloss": "利用三种货币之间的汇率不一致进行套利。"
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
      "coverage_tag": "high_frequency_trading_intro",
      "covered_by": [
        "qf_flash_hft_def",
        "qf_quiz_hft_goal",
        "qf_long_hft_landscape"
      ],
      "description": "高频交易的定义与核心特征：快速执行、极短持仓、依赖速度与延迟"
    },
    {
      "coverage_tag": "co_location",
      "covered_by": [
        "qf_flash_colo_purpose",
        "qf_quiz_colo_reason"
      ],
      "description": "主机托管：将服务器放在交易所数据中心以减少延迟"
    },
    {
      "coverage_tag": "latency_arbitrage",
      "covered_by": [
        "qf_flash_latency_arb_def",
        "qf_quiz_latency_arb_mechanism"
      ],
      "description": "延迟套利：利用信息传输速度差异抢先交易"
    },
    {
      "coverage_tag": "arbitrage_basics",
      "covered_by": [
        "qf_flash_arb_def",
        "qf_quiz_arb_example"
      ],
      "description": "套利的基本概念：利用不同市场或资产间的价格差异获利"
    },
    {
      "coverage_tag": "triangular_arbitrage",
      "covered_by": [
        "qf_flash_triangular_arb",
        "qf_quiz_triangular_arb_flow",
        "qf_long_triangular_arb_calc"
      ],
      "description": "三角套利：利用三种货币汇率不一致进行套利"
    },
    {
      "coverage_tag": "market_making",
      "covered_by": [
        "qf_flash_mm_def",
        "qf_quiz_mm_risk",
        "qf_long_mm_strategy"
      ],
      "description": "做市策略：同时挂买单和卖单赚取买卖价差"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "high_frequency_trading",
      "coverage_tags": [
        "high_frequency_trading_intro"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_hft_def",
      "learning_goal": "学生能准确说出高频交易的核心特征：快速执行、极短持仓、依赖速度。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "高频交易的两个核心特征：执行速度和持仓时间。",
      "term_refs": [
        {
          "display": "高频交易",
          "en": "High Frequency Trading (HFT)"
        }
      ],
      "variants": [
        {
          "back": "快速执行大量交易，以及极短的持仓时间。",
          "estimated_seconds": 8,
          "explanation": "HFT 依赖毫秒甚至微秒级的速度，从微小的价格差异中获利，持仓通常只有几秒甚至更短。",
          "front": "高频交易（HFT）的两个核心特征是什么？",
          "question_id": "q_flash_hft_def_v1"
        },
        {
          "back": "从微小的价格差异和价差中获利。",
          "estimated_seconds": 6,
          "explanation": "HFT 不依赖大的价格变动，而是通过大量交易累积微小的利润。",
          "front": "高频交易主要从什么中获利？",
          "question_id": "q_flash_hft_def_v2"
        }
      ]
    },
    {
      "concept_key": "co_location",
      "coverage_tags": [
        "co_location"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_colo_purpose",
      "learning_goal": "学生能说出主机托管的目的：减少网络延迟。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "主机托管的核心目的。",
      "term_refs": [
        {
          "display": "主机托管",
          "en": "Co-location"
        }
      ],
      "variants": [
        {
          "back": "减少网络延迟，获得毫秒甚至微秒级的速度优势。",
          "estimated_seconds": 8,
          "explanation": "通过将服务器放在交易所数据中心，物理距离缩短，信号传输时间大幅减少。",
          "front": "高频交易公司采用主机托管（Co-location）的主要目的是什么？",
          "question_id": "q_flash_colo_purpose_v1"
        },
        {
          "back": "将交易服务器放置在交易所数据中心内，缩短物理距离，从而减少数据传输时间。",
          "estimated_seconds": 10,
          "explanation": "光信号在光纤中每微秒约传输300米，缩短物理距离能直接降低延迟。",
          "front": "主机托管（Co-location）是如何帮助高频交易公司获得速度优势的？",
          "question_id": "q_flash_colo_purpose_v2"
        }
      ]
    },
    {
      "concept_key": "latency_arbitrage",
      "coverage_tags": [
        "latency_arbitrage"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_latency_arb_def",
      "learning_goal": "学生能定义延迟套利并说明其获利机制。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "延迟套利的定义和获利方式。",
      "term_refs": [
        {
          "display": "延迟套利",
          "en": "Latency Arbitrage"
        }
      ],
      "variants": [
        {
          "back": "利用信息传输速度差异，比对手更快看到订单流并提前交易以获利。",
          "estimated_seconds": 10,
          "explanation": "例如，HFT 公司比普通交易者早几微秒看到大额买单，抢先买入，然后以更高价格卖给该大额订单。",
          "front": "什么是延迟套利（Latency Arbitrage）？",
          "question_id": "q_flash_latency_arb_def_v1"
        },
        {
          "back": "更快的市场数据接收速度和订单执行速度。",
          "estimated_seconds": 6,
          "explanation": "这通常通过主机托管、高速网络和优化硬件实现。",
          "front": "延迟套利依赖什么核心优势？",
          "question_id": "q_flash_latency_arb_def_v2"
        }
      ]
    },
    {
      "concept_key": "arbitrage",
      "coverage_tags": [
        "arbitrage_basics"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_arb_def",
      "learning_goal": "学生能定义套利并给出一个具体例子。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "definition_with_example",
      "retrieval_focus": "套利的定义和一个典型例子。",
      "term_refs": [
        {
          "display": "套利",
          "en": "Arbitrage"
        }
      ],
      "variants": [
        {
          "back": "在不同市场或资产间同时买卖，利用价格差异获利。例子：在 Binance 以 96531.5 卖出 BTC 期货，同时在 Bybit 以 96531.0 买入，赚取 0.5 美元差价。",
          "estimated_seconds": 12,
          "explanation": "套利有助于使不同市场的价格趋于一致，维持市场效率。",
          "front": "什么是套利（Arbitrage）？请给出一个例子。",
          "question_id": "q_flash_arb_def_v1"
        },
        {
          "back": "帮助维持市场效率，使不同市场的价格趋于一致。",
          "estimated_seconds": 8,
          "explanation": "当套利者买入低估资产、卖出高估资产时，价格差异会被迅速抹平。",
          "front": "套利交易对市场有什么作用？",
          "question_id": "q_flash_arb_def_v2"
        }
      ]
    },
    {
      "concept_key": "triangular_arbitrage",
      "coverage_tags": [
        "triangular_arbitrage"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_triangular_arb",
      "learning_goal": "学生能描述三角套利的基本流程。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "三角套利的三步货币兑换流程。",
      "term_refs": [
        {
          "display": "三角套利",
          "en": "Triangular Arbitrage"
        }
      ],
      "variants": [
        {
          "back": "用货币A换货币B，用货币B换货币C，再用货币C换回货币A。如果最终金额大于初始金额，就存在套利机会。",
          "estimated_seconds": 12,
          "explanation": "例如：美元→欧元→英镑→美元，一圈下来钱变多了。",
          "front": "三角套利（Triangular Arbitrage）的基本操作流程是什么？",
          "question_id": "q_flash_triangular_arb_v1"
        },
        {
          "back": "三种货币之间的汇率存在不一致，使得循环兑换后的金额大于初始金额。",
          "estimated_seconds": 10,
          "explanation": "这需要极快的计算和执行，因为套利机会转瞬即逝。",
          "front": "三角套利成功的关键条件是什么？",
          "question_id": "q_flash_triangular_arb_v2"
        }
      ]
    },
    {
      "concept_key": "market_making",
      "coverage_tags": [
        "market_making"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_mm_def",
      "learning_goal": "学生能说出做市策略的基本操作和利润来源。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "做市商如何赚钱。",
      "term_refs": [
        {
          "display": "做市",
          "en": "Market Making"
        }
      ],
      "variants": [
        {
          "back": "通过同时挂出买单和卖单，赚取买卖价差（bid-ask spread）。",
          "estimated_seconds": 8,
          "explanation": "做市商为市场提供流动性，交易所有时还会提供返佣作为额外激励。",
          "front": "做市商（Market Maker）如何赚钱？",
          "question_id": "q_flash_mm_def_v1"
        },
        {
          "back": "市场单边暴涨暴跌时，做市商可能因持有不利头寸而亏损。",
          "estimated_seconds": 8,
          "explanation": "例如，如果市场突然大幅上涨，做市商之前卖出的空单会面临巨大亏损。",
          "front": "做市策略的主要风险是什么？",
          "question_id": "q_flash_mm_def_v2"
        }
      ]
    }
  ],
  "lesson_id": "L8",
  "longform_families": [
    {
      "concept_key": "high_frequency_trading",
      "coverage_tags": [
        "high_frequency_trading_intro"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_hft_landscape",
      "learning_goal": "学生能解释高频交易的核心要素及其相互关系。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "高频交易",
          "en": "High Frequency Trading (HFT)"
        },
        {
          "display": "主机托管",
          "en": "Co-location"
        },
        {
          "display": "延迟套利",
          "en": "Latency Arbitrage"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "HFT 的核心特征",
            "主机托管的作用",
            "延迟套利的机制"
          ],
          "question_id": "q_long_hft_landscape_v1",
          "reference_answer": [
            "高频交易（HFT）的核心特征包括：1）快速执行大量交易，通常在毫秒甚至微秒级别；2）极短的持仓时间，从几秒到几分钟不等；3）从微小的价格差异和价差中获利。",
            "主机托管（Co-location）是 HFT 的基础设施，通过将交易服务器放置在交易所数据中心内，物理距离的缩短能显著减少数据传输时间，为 HFT 公司提供毫秒甚至微秒级的速度优势。",
            "延迟套利（Latency Arbitrage）利用这种速度优势：HFT 公司比普通交易者更早看到市场数据（如大额订单），从而抢先交易。例如，HFT 看到一个大额买单后，立即以当前价格买入，然后以稍高价格卖给该大额订单，赚取差价。",
            "这三者紧密相连：主机托管提供了速度优势的基础，延迟套利是利用这种优势的具体策略，而 HFT 的整体框架则依赖于这些技术和策略的结合来捕捉微小的市场机会。"
          ],
          "rubric_points": [
            "准确描述 HFT 的两个核心特征：快速执行和极短持仓",
            "解释主机托管如何通过缩短物理距离来减少延迟",
            "说明延迟套利如何利用速度优势抢先交易获利",
            "将三者联系起来，展示它们如何共同构成 HFT 的基础"
          ],
          "stem": "请解释高频交易（HFT）的核心特征，并说明主机托管（Co-location）和延迟套利（Latency Arbitrage）如何支持 HFT 策略的实施。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "速度在 HFT 中的重要性",
            "主机托管如何提升速度",
            "延迟套利如何利用速度优势"
          ],
          "question_id": "q_long_hft_landscape_v2",
          "reference_answer": [
            "速度对 HFT 至关重要，因为 HFT 的利润来自微小的价格差异，这些差异存在的时间极短，可能只有几毫秒。谁先看到机会并执行交易，谁就能获利。",
            "主机托管就像把赛车停在起跑线旁边。HFT 公司把服务器放在交易所数据中心，信号传输时间从几十毫秒减少到几微秒，这就像在比赛中比别人早出发 0.1 秒。",
            "延迟套利就像在赛跑中比别人早看到终点线。假设一个大型基金要买入 100 万股股票，这个信息需要几毫秒才能传到普通交易者。HFT 公司因为主机托管，能早几微秒看到这个信息，立即买入股票，然后以稍高的价格卖给该基金。",
            "速度优势直接转化为利润：每笔交易赚取的差价可能很小，但 HFT 公司每天执行数百万笔交易，累积的利润非常可观。"
          ],
          "rubric_points": [
            "用通俗语言解释速度对 HFT 的关键作用",
            "用具体例子说明主机托管如何减少延迟",
            "用简单场景描述延迟套利的运作方式",
            "解释为什么速度优势能转化为利润"
          ],
          "stem": "假设你是一家 HFT 公司的策略师，请向一位非技术背景的投资者解释：为什么速度对 HFT 如此重要？请结合主机托管和延迟套利的概念进行说明。"
        }
      ]
    },
    {
      "concept_key": "triangular_arbitrage",
      "coverage_tags": [
        "triangular_arbitrage"
      ],
      "difficulty": "hard",
      "family_id": "qf_long_triangular_arb_calc",
      "learning_goal": "学生能通过具体数值计算验证三角套利机会。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "worked_example",
      "term_refs": [
        {
          "display": "三角套利",
          "en": "Triangular Arbitrage"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "确定套利路径",
            "执行货币兑换计算",
            "比较最终金额与初始金额",
            "计算净利润"
          ],
          "question_id": "q_long_triangular_arb_calc_v1",
          "reference_answer": [
            "套利路径：美元 → 欧元 → 英镑 → 美元",
            "步骤 1：用 1000 美元换欧元：1000 × 0.8631 = 863.1 欧元",
            "步骤 2：用 863.1 欧元换英镑：863.1 / 1.4600 = 591.1644 英镑",
            "步骤 3：用 591.1644 英镑换美元：591.1644 × 1.6939 = 1001.373 美元",
            "最终金额 1001.373 美元 > 初始金额 1000 美元，存在套利机会。",
            "净利润 = 1001.373 - 1000 = 1.373 美元"
          ],
          "rubric_points": [
            "正确选择套利路径（美元→欧元→英镑→美元）",
            "正确进行每一步的货币兑换计算",
            "正确比较最终金额与初始金额",
            "正确计算净利润",
            "得出是否存在套利机会的结论"
          ],
          "stem": "假设你观察到以下汇率：\nEUR/USD = 0.8631（1 美元可换 0.8631 欧元）\nEUR/GBP = 1.4600（1 英镑可换 1.4600 欧元）\nUSD/GBP = 1.6939（1 英镑可换 1.6939 美元）\n\n你初始持有 1000 美元。请通过计算说明是否存在三角套利机会。如果存在，请写出完整的操作步骤并计算净利润。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "确定套利路径",
            "执行货币兑换计算",
            "比较最终金额与初始金额",
            "计算净利润"
          ],
          "question_id": "q_long_triangular_arb_calc_v2",
          "reference_answer": [
            "套利路径：美元 → 日元 → 欧元 → 美元",
            "步骤 1：用 1000 美元换日元：1000 × 110.00 = 110,000 日元",
            "步骤 2：用 110,000 日元换欧元：110,000 / 130.00 = 846.1538 欧元",
            "步骤 3：用 846.1538 欧元换美元：846.1538 / 0.85 = 995.4751 美元",
            "最终金额 995.4751 美元 < 初始金额 1000 美元，不存在套利机会。",
            "实际上存在亏损，说明汇率是基本一致的，没有套利空间。"
          ],
          "rubric_points": [
            "正确选择套利路径（美元→日元→欧元→美元）",
            "正确进行每一步的货币兑换计算",
            "正确比较最终金额与初始金额",
            "正确计算净利润",
            "得出是否存在套利机会的结论"
          ],
          "stem": "假设你观察到以下汇率：\nUSD/JPY = 110.00（1 美元可换 110 日元）\nEUR/JPY = 130.00（1 欧元可换 130 日元）\nEUR/USD = 0.85（1 美元可换 0.85 欧元）\n\n你初始持有 1000 美元。请通过计算说明是否存在三角套利机会。如果存在，请写出完整的操作步骤并计算净利润。"
        }
      ]
    },
    {
      "concept_key": "market_making",
      "coverage_tags": [
        "market_making"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_mm_strategy",
      "learning_goal": "学生能解释做市策略的运作机制、利润来源和风险管理。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "mechanism_trace",
      "term_refs": [
        {
          "display": "做市",
          "en": "Market Making"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "做市策略的基本操作",
            "利润来源",
            "主要风险",
            "风险管理方法"
          ],
          "question_id": "q_long_mm_strategy_v1",
          "reference_answer": [
            "做市策略的基本操作：做市商同时挂出买单（bid）和卖单（ask），为市场提供流动性。例如，在 10.00 挂买单，在 10.01 挂卖单。",
            "利润来源：1）买卖价差：当两个订单都成交时，做市商赚取 0.01 的差价；2）交易所返佣：许多交易所会向提供流动性的做市商支付返佣。",
            "主要风险：库存风险。当市场出现单边行情时，做市商可能只成交了一边订单，导致持有不利头寸。例如，市场突然上涨，做市商之前卖出的空单会面临亏损。",
            "风险管理方法：1）动态调整报价：根据市场趋势调整买卖价格和数量，平衡库存；2）设置止损：当市场趋势明显时，及时平仓止损；3）只在震荡市开仓：通过技术指标判断市场是否处于震荡区间，避免在趋势行情中做市。"
          ],
          "rubric_points": [
            "准确描述做市商同时挂出买单和卖单的操作",
            "解释买卖价差如何产生利润",
            "说明交易所返佣作为额外收入来源",
            "识别单边行情带来的库存风险",
            "提出至少两种风险管理方法"
          ],
          "stem": "请解释做市策略（Market Making）的基本运作机制，包括：1）如何产生利润；2）面临的主要风险；3）如何管理这些风险。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "报价策略设计",
            "利润预期",
            "风险应对方案"
          ],
          "question_id": "q_long_mm_strategy_v2",
          "reference_answer": [
            "报价策略：在 2000.5 挂入 1 手买单，在 2001.0 挂出 1 手卖单，赚取 0.5 美元的价差。订单有效期设为 1 小时，未成交则取消重新报价。",
            "利润预期：在震荡市中，每对订单成交可赚取 0.5 美元。如果每小时成交 1 对，每天（24 小时）可赚取 12 美元。",
            "风险应对：1）设置价格监控，如果价格突破 2000-2010 区间，立即取消所有未成交订单；2）如果已经持有头寸，立即在突破方向建立对冲头寸；3）在趋势确认后，暂停做市策略，转为趋势跟踪策略。",
            "库存管理：定期检查净头寸，如果净头寸偏离零，调整报价策略使其回归中性。例如，如果持有多头，可以降低买单价格、提高卖单价格，鼓励卖出成交。"
          ],
          "rubric_points": [
            "设计合理的买卖报价（如 2000.5 买，2001.0 卖）",
            "解释如何通过价差获利",
            "说明如何设置订单有效期",
            "提出市场突破时的应对措施",
            "考虑库存管理"
          ],
          "stem": "假设你是一个做市商，当前市场处于震荡区间，黄金价格在 2000-2010 美元之间波动。请设计一个简单的做市策略，并说明：1）你的报价策略；2）如何应对市场突然突破震荡区间的情况。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "high_frequency_trading",
      "coverage_tags": [
        "high_frequency_trading_intro"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_hft_goal",
      "learning_goal": "学生能识别高频交易的核心目标。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "高频交易",
          "en": "High Frequency Trading (HFT)"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "HFT 的核心是速度，通过快速执行大量交易来捕捉微小的价格差异，持仓时间极短。",
          "options": [
            "通过长期持有优质股票获得分红收益",
            "利用极快的速度执行大量交易，从微小价差中获利",
            "通过基本面分析寻找被低估的资产",
            "通过分散投资降低整体投资组合风险"
          ],
          "question_id": "q_quiz_hft_goal_v1",
          "stem": "以下哪一项最准确地描述了高频交易（HFT）的核心目标？"
        },
        {
          "answer": 0,
          "estimated_seconds": 15,
          "explanation": "HFT 依赖算法在毫秒甚至微秒级别做出决策，交易频率极高，持仓时间通常只有几秒。",
          "options": [
            "交易频率更高，持仓时间更短",
            "交易金额更大",
            "只交易股票",
            "不需要计算机"
          ],
          "question_id": "q_quiz_hft_goal_v2",
          "stem": "高频交易（HFT）与传统交易相比，最显著的区别是什么？"
        }
      ]
    },
    {
      "concept_key": "co_location",
      "coverage_tags": [
        "co_location"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_colo_reason",
      "learning_goal": "学生能识别主机托管的主要目的。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "主机托管",
          "en": "Co-location"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "主机托管将服务器放在交易所数据中心，物理距离的缩短能显著降低数据传输时间，这是 HFT 速度优势的关键。",
          "options": [
            "节省服务器硬件成本",
            "减少网络延迟，获得速度优势",
            "方便与交易所员工交流",
            "获得更多的市场数据"
          ],
          "question_id": "q_quiz_colo_reason_v1",
          "stem": "高频交易公司采用主机托管（Co-location）的主要原因是？"
        },
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "光信号在光纤中每微秒约传输300米，将服务器放在交易所旁边能最大程度减少信号传输时间。",
          "options": [
            "它增加了网络延迟",
            "它使所有交易者获得相同速度",
            "它通过缩短物理距离来减少延迟",
            "它主要用于存储历史数据"
          ],
          "question_id": "q_quiz_colo_reason_v2",
          "stem": "以下哪个关于主机托管（Co-location）的说法是正确的？"
        }
      ]
    },
    {
      "concept_key": "latency_arbitrage",
      "coverage_tags": [
        "latency_arbitrage"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_latency_arb_mechanism",
      "learning_goal": "学生能理解延迟套利的具体运作机制。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "延迟套利",
          "en": "Latency Arbitrage"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "HFT 利用时间优势抢先买入，然后当大额订单到达时以更高价格卖出，赚取差价。",
          "options": [
            "忽略该信息，因为 10 微秒太短",
            "立即以当前价格买入，然后以稍高价格卖给该大额买单",
            "立即卖出，预期价格会下跌",
            "通知交易所暂停交易"
          ],
          "question_id": "q_quiz_latency_arb_mechanism_v1",
          "stem": "在延迟套利的一个典型场景中，HFT 公司比普通交易者早 10 微秒看到一个大额买单。HFT 公司最可能采取以下哪种行动？"
        },
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "延迟套利利用的是信息传输速度的差异，速度快的交易者能比速度慢的交易者更早看到市场数据并做出反应。",
          "options": [
            "信息在不同市场参与者之间传输速度不同",
            "不同交易所的股票价格不同",
            "同一资产在不同时间点的价格不同",
            "市场参与者的风险偏好不同"
          ],
          "question_id": "q_quiz_latency_arb_mechanism_v2",
          "stem": "延迟套利（Latency Arbitrage）主要利用的是哪种市场不完美？"
        }
      ]
    },
    {
      "concept_key": "arbitrage",
      "coverage_tags": [
        "arbitrage_basics"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_arb_example",
      "learning_goal": "学生能识别一个具体的套利场景。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "套利",
          "en": "Arbitrage"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "套利者应在价格较低的 Bybit 买入（96,531.0），在价格较高的 Binance 卖出（96,532.0），赚取 1 美元差价。",
          "options": [
            "在 Binance 买入，在 Bybit 卖出",
            "在 Bybit 买入，在 Binance 卖出",
            "同时在两个交易所买入",
            "同时在两个交易所卖出"
          ],
          "question_id": "q_quiz_arb_example_v1",
          "stem": "假设比特币在 Binance 的卖一价为 96,532.0 美元，在 Bybit 的买一价为 96,531.0 美元。一个套利者可以如何操作？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "套利者在低价市场买入、高价市场卖出，这一过程会推动低价上涨、高价下跌，最终使价格差异消失，市场变得更有效率。",
          "options": [
            "增加市场波动性",
            "使不同市场的价格趋于一致",
            "减少市场流动性",
            "增加交易成本"
          ],
          "question_id": "q_quiz_arb_example_v2",
          "stem": "套利交易对市场的主要影响是什么？"
        }
      ]
    },
    {
      "concept_key": "triangular_arbitrage",
      "coverage_tags": [
        "triangular_arbitrage"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_triangular_arb_flow",
      "learning_goal": "学生能正确排列三角套利的操作步骤。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "ordering",
      "term_refs": [
        {
          "display": "三角套利",
          "en": "Triangular Arbitrage"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "正确的顺序是：先用美元换欧元（C），再用欧元换英镑（A），最后用英镑换回美元（B）。如果最终美元多于初始，就存在套利机会。",
          "options": [
            "C → A → B",
            "A → B → C",
            "B → C → A",
            "C → B → A"
          ],
          "question_id": "q_quiz_triangular_arb_flow_v1",
          "stem": "请将以下三角套利的步骤按正确顺序排列：\nA. 用欧元换英镑\nB. 用英镑换回美元\nC. 用美元换欧元"
        },
        {
          "answer": 0,
          "estimated_seconds": 20,
          "explanation": "三角套利要求最终回到初始货币，这样才能计算净收益。选项 A 从美元出发，经过日元和欧元，最终回到美元，符合要求。",
          "options": [
            "美元 → 日元 → 欧元 → 美元",
            "美元 → 欧元 → 美元 → 英镑",
            "欧元 → 美元 → 英镑 → 欧元",
            "英镑 → 欧元 → 美元 → 英镑"
          ],
          "question_id": "q_quiz_triangular_arb_flow_v2",
          "stem": "在三角套利中，如果初始货币是美元，最终货币也应该是美元。以下哪个顺序是正确的？"
        }
      ]
    },
    {
      "concept_key": "market_making",
      "coverage_tags": [
        "market_making"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_mm_risk",
      "learning_goal": "学生能识别做市策略的主要风险。",
      "linked_steps": [
        "step4"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "做市",
          "en": "Market Making"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "在单边行情中，做市商的一边订单可能无法成交，而另一边订单导致其持有不利头寸，造成亏损。",
          "options": [
            "市场在窄幅区间震荡",
            "市场出现单边暴涨或暴跌",
            "市场流动性非常好",
            "市场波动率很低"
          ],
          "question_id": "q_quiz_mm_risk_v1",
          "stem": "做市策略在以下哪种市场条件下最容易亏损？"
        },
        {
          "answer": 0,
          "estimated_seconds": 15,
          "explanation": "许多交易所会向提供流动性的做市商支付返佣（rebate），作为对其贡献的奖励。",
          "options": [
            "交易所提供的流动性返佣",
            "向客户收取咨询费",
            "出售市场数据",
            "收取交易佣金"
          ],
          "question_id": "q_quiz_mm_risk_v2",
          "stem": "做市商除了赚取买卖价差外，还可能从以下哪项获得收入？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L8/plain.txt",
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
      "coverage_tag": "market_impact_definition",
      "covered_by": [
        "qf_flash_market_impact_def",
        "qf_quiz_market_impact_def"
      ],
      "description": "市场冲击的定义：由订单本身引起的价格不利变动。"
    },
    {
      "coverage_tag": "market_impact_mitigation_strategies",
      "covered_by": [
        "qf_flash_mitigation_strategies",
        "qf_quiz_mitigation_strategies",
        "qf_long_mitigation_strategies"
      ],
      "description": "降低市场冲击的四种策略：订单拆分、限价单、TWAP、VWAP。"
    },
    {
      "coverage_tag": "order_splitting",
      "covered_by": [
        "qf_flash_order_splitting",
        "qf_quiz_order_splitting"
      ],
      "description": "订单拆分：将大单拆分为多个小单以降低即时冲击。"
    },
    {
      "coverage_tag": "use_of_limit_orders",
      "covered_by": [
        "qf_flash_limit_orders",
        "qf_quiz_limit_orders"
      ],
      "description": "使用限价单：控制成交价，避免市价单风险。"
    },
    {
      "coverage_tag": "twap_strategy",
      "covered_by": [
        "qf_flash_twap",
        "qf_quiz_twap"
      ],
      "description": "时间加权平均价格（TWAP）：在指定时间段内均匀执行订单。"
    },
    {
      "coverage_tag": "vwap_strategy",
      "covered_by": [
        "qf_flash_vwap",
        "qf_quiz_vwap"
      ],
      "description": "成交量加权平均价格（VWAP）：根据市场成交量比例执行订单。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "market_impact",
      "coverage_tags": [
        "market_impact_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_market_impact_def",
      "learning_goal": "学生能准确回忆市场冲击的定义及其核心特征。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "市场冲击的定义和直接后果。",
      "term_refs": [
        {
          "display": "市场冲击",
          "en": "Market Impact"
        }
      ],
      "variants": [
        {
          "back": "由订单本身引起的价格不利变动。",
          "estimated_seconds": 8,
          "explanation": "市场冲击是指执行订单时，因订单本身导致的价格向不利方向变动。大额订单尤其明显。",
          "front": "什么是市场冲击？",
          "question_id": "q_flash_market_impact_def_v1"
        },
        {
          "back": "市场深度和流动性。",
          "estimated_seconds": 10,
          "explanation": "大额市价单会吃掉订单簿上的多个价位，导致市场深度减少，流动性也随之降低。",
          "front": "大额市价单执行时，除了价格变动，还会导致哪两个市场特征下降？",
          "question_id": "q_flash_market_impact_def_v2"
        }
      ]
    },
    {
      "concept_key": "market_impact_mitigation",
      "coverage_tags": [
        "market_impact_mitigation_strategies"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_mitigation_strategies",
      "learning_goal": "学生能列举并简要描述降低市场冲击的四种主要策略。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "四种降低市场冲击策略的名称。",
      "term_refs": [
        {
          "display": "降低市场冲击的策略",
          "en": "Market Impact Mitigation Strategies"
        }
      ],
      "variants": [
        {
          "back": "订单拆分、使用限价单、TWAP、VWAP。",
          "estimated_seconds": 12,
          "explanation": "这四种策略分别从订单大小、订单类型、时间分布和成交量分布的角度来减少对市场的冲击。",
          "front": "列举降低市场冲击的四种策略。",
          "question_id": "q_flash_mitigation_strategies_v1"
        },
        {
          "back": "TWAP 按时间均匀分配订单，VWAP 按市场成交量比例分配订单。",
          "estimated_seconds": 10,
          "explanation": "TWAP 假设价格稳定，VWAP 则根据实际成交量调整，使订单融入市场。",
          "front": "TWAP 和 VWAP 的核心区别是什么？",
          "question_id": "q_flash_mitigation_strategies_v2"
        }
      ]
    },
    {
      "concept_key": "order_splitting",
      "coverage_tags": [
        "order_splitting"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_order_splitting",
      "learning_goal": "学生能解释订单拆分的原理和目的。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "订单拆分的核心机制。",
      "term_refs": [
        {
          "display": "订单拆分",
          "en": "Order Splitting"
        }
      ],
      "variants": [
        {
          "back": "将大额订单拆分成多个小额订单，分批执行，减少即时冲击。",
          "estimated_seconds": 10,
          "explanation": "例如，将一万股拆成一百个一百股，慢慢买入，避免一次性吃掉所有卖单。",
          "front": "订单拆分是如何降低市场冲击的？",
          "question_id": "q_flash_order_splitting_v1"
        },
        {
          "back": "可能增加交易成本。",
          "estimated_seconds": 8,
          "explanation": "多次小额交易会产生更多的交易费用，需要在降低冲击和增加成本之间权衡。",
          "front": "订单拆分的主要缺点是什么？",
          "question_id": "q_flash_order_splitting_v2"
        }
      ]
    },
    {
      "concept_key": "limit_orders",
      "coverage_tags": [
        "use_of_limit_orders"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_limit_orders",
      "learning_goal": "学生能理解限价单在降低市场冲击中的作用及其局限性。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "限价单的优点和缺点。",
      "term_refs": [
        {
          "display": "限价单",
          "en": "Limit Order"
        }
      ],
      "variants": [
        {
          "back": "限价单可以控制成交价，避免以不利价格成交。",
          "estimated_seconds": 10,
          "explanation": "限价单指定了最高买入价或最低卖出价，从而避免了市价单可能导致的滑点。",
          "front": "使用限价单代替市价单如何降低市场冲击？",
          "question_id": "q_flash_limit_orders_v1"
        },
        {
          "back": "可能无法立即成交，甚至永远无法成交。",
          "estimated_seconds": 8,
          "explanation": "如果限价单的价格设定得过于保守，可能无法被市场接受，导致订单无法执行。",
          "front": "使用限价单的主要风险是什么？",
          "question_id": "q_flash_limit_orders_v2"
        }
      ]
    },
    {
      "concept_key": "twap",
      "coverage_tags": [
        "twap_strategy"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_twap",
      "learning_goal": "学生能解释TWAP策略的基本原理和适用条件。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "TWAP的执行方式和假设。",
      "term_refs": [
        {
          "display": "时间加权平均价格",
          "en": "Time-weighted Average Price (TWAP)"
        }
      ],
      "variants": [
        {
          "back": "在指定时间段内，按固定时间间隔均匀地执行订单。",
          "estimated_seconds": 10,
          "explanation": "例如，在1小时内，每5分钟买入固定数量，使交易时间均匀分布。",
          "front": "TWAP 策略如何执行订单？",
          "question_id": "q_flash_twap_v1"
        },
        {
          "back": "假设价格在交易期间是稳定的。",
          "estimated_seconds": 8,
          "explanation": "如果市场波动很大，TWAP 策略的效果会打折扣，因为均匀分配的时间点可能正好是价格不利的时候。",
          "front": "TWAP 策略的一个关键假设是什么？",
          "question_id": "q_flash_twap_v2"
        }
      ]
    },
    {
      "concept_key": "vwap",
      "coverage_tags": [
        "vwap_strategy"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_vwap",
      "learning_goal": "学生能解释VWAP策略的基本原理和优势。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "VWAP的执行方式和目的。",
      "term_refs": [
        {
          "display": "成交量加权平均价格",
          "en": "Volume-weighted Average Price (VWAP)"
        }
      ],
      "variants": [
        {
          "back": "根据市场的实际成交量比例来安排订单。",
          "estimated_seconds": 10,
          "explanation": "市场成交量大的时候多买，成交量小的时候少买，使订单融入市场。",
          "front": "VWAP 策略如何决定每个时间点的交易量？",
          "question_id": "q_flash_vwap_v1"
        },
        {
          "back": "减少价格冲击，使成交价接近市场均价。",
          "estimated_seconds": 8,
          "explanation": "通过跟随市场成交量分布，VWAP 策略能有效降低大额订单对市场的冲击。",
          "front": "VWAP 策略的主要优势是什么？",
          "question_id": "q_flash_vwap_v2"
        }
      ]
    }
  ],
  "lesson_id": "L8",
  "longform_families": [
    {
      "concept_key": "market_impact_mitigation",
      "coverage_tags": [
        "market_impact_mitigation_strategies"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_mitigation_strategies",
      "learning_goal": "学生能比较和对比TWAP与VWAP两种策略，并解释其适用场景。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "compare_and_contrast",
      "term_refs": [
        {
          "display": "降低市场冲击的策略",
          "en": "Market Impact Mitigation Strategies"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "执行逻辑",
            "适用市场条件",
            "优缺点"
          ],
          "question_id": "q_long_mitigation_strategies_v1",
          "reference_answer": [
            "TWAP（时间加权平均价格）策略的核心是在一个指定的时间段内，按照固定的时间间隔均匀地执行订单。它假设价格在该时间段内是稳定的。",
            "VWAP（成交量加权平均价格）策略则是根据市场的实际成交量分布来安排订单的执行。在成交量大的时候多交易，成交量小的时候少交易。",
            "TWAP适用于价格相对稳定、波动不大的市场。VWAP则更适用于成交量分布不均匀、有明显活跃期和清淡期的市场。",
            "TWAP的优点是逻辑简单、易于执行。缺点是当市场波动剧烈时，成交价格可能与预期产生较大偏差。",
            "VWAP的优点是能更好地将大订单融入市场，减少对价格的冲击，使成交价更接近市场均价。缺点是如果市场成交量突然萎缩，可能导致订单无法按计划完全执行。"
          ],
          "rubric_points": [
            "正确解释TWAP按时间均匀分配订单的逻辑。",
            "正确解释VWAP按成交量比例分配订单的逻辑。",
            "指出TWAP适用于价格稳定市场，VWAP适用于成交量分布不均的市场。",
            "指出TWAP的优点是简单，缺点是在波动市场中效果差。",
            "指出VWAP的优点是能更好地融入市场，缺点是可能无法完全执行。"
          ],
          "stem": "比较并对比TWAP和VWAP两种降低市场冲击的策略。请从执行逻辑、适用市场条件、以及各自的优缺点三个方面进行阐述。"
        },
        {
          "estimated_seconds": 150,
          "prompt_blocks": [
            "方案设计",
            "策略组合理由",
            "潜在风险与应对"
          ],
          "question_id": "q_long_mitigation_strategies_v2",
          "reference_answer": [
            "我的方案是：首先，将100万股的订单拆分成多个小订单，例如每个订单1万股。然后，使用VWAP策略，在一天内根据市场的成交量分布来执行这些小订单。同时，主要使用限价单来执行，将限价设定在VWAP价格附近的一个小范围内。",
            "选择这个方案的理由是：订单拆分是基础，可以避免一次性冲击。VWAP策略能确保我们的交易活动与市场节奏保持一致，减少被其他交易者注意到的风险。使用限价单可以控制成交价格，避免在市场波动时以过高的价格买入。",
            "潜在风险是：如果市场成交量突然下降，我们的VWAP订单可能无法按时完成。应对方法是：设置一个时间阈值，如果在一定时间内未完成，则转为使用TWAP策略，确保在收盘前完成大部分交易。另外，需要监控市场冲击成本，如果冲击过大，应暂停交易。"
          ],
          "rubric_points": [
            "方案中至少包含一种降低市场冲击的策略（如订单拆分、限价单、TWAP、VWAP）。",
            "合理解释了所选策略如何帮助降低冲击。",
            "考虑了流动性一般的市场条件。",
            "识别了方案的一个潜在风险（如执行延迟、成本增加）。",
            "提出了一个应对该风险的方法。"
          ],
          "stem": "假设你是一位基金经理，需要在一天内买入100万股某流动性一般的股票。请设计一个降低市场冲击的方案，并解释你选择该方案的理由。你的方案可以结合多种策略。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "market_impact",
      "coverage_tags": [
        "market_impact_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_market_impact_def",
      "learning_goal": "学生能在测验中准确识别市场冲击的定义和后果。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "市场冲击",
          "en": "Market Impact"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "市场冲击特指执行订单时，因订单本身导致的价格向不利方向变动。",
          "options": [
            "市场整体波动性增加",
            "由订单本身引起的价格不利变动",
            "交易佣金和手续费的总和",
            "买卖价差的扩大"
          ],
          "question_id": "q_quiz_market_impact_def_v1",
          "stem": "以下哪一项最准确地描述了“市场冲击”？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "大额市价单会消耗订单簿上的流动性，导致市场深度减少，而不是增加。",
          "options": [
            "价格显著变动",
            "市场深度增加",
            "流动性降低",
            "订单簿被部分填满"
          ],
          "question_id": "q_quiz_market_impact_def_v2",
          "stem": "大额市价单执行后，最不可能出现以下哪种情况？"
        }
      ]
    },
    {
      "concept_key": "market_impact_mitigation",
      "coverage_tags": [
        "market_impact_mitigation_strategies"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_mitigation_strategies",
      "learning_goal": "学生能区分不同降低市场冲击策略的特点。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "降低市场冲击的策略",
          "en": "Market Impact Mitigation Strategies"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 25,
          "explanation": "TWAP策略的核心是在指定时间段内按固定时间间隔均匀执行订单。",
          "options": [
            "订单拆分",
            "TWAP",
            "VWAP",
            "使用限价单"
          ],
          "question_id": "q_quiz_mitigation_strategies_v1",
          "stem": "一位交易员计划在1小时内买入1000股，他决定每5分钟买入约83股。他使用的是哪种策略？"
        },
        {
          "answer": 3,
          "estimated_seconds": 25,
          "explanation": "VWAP策略根据市场成交量比例执行订单，其目标就是使成交价接近成交量加权平均价。",
          "options": [
            "使用市价单一次性买入",
            "使用限价单在固定价格挂单",
            "TWAP 策略",
            "VWAP 策略"
          ],
          "question_id": "q_quiz_mitigation_strategies_v2",
          "stem": "以下哪种策略最有可能使大额订单的成交价接近当天的市场平均价格？"
        }
      ]
    },
    {
      "concept_key": "order_splitting",
      "coverage_tags": [
        "order_splitting"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_order_splitting",
      "learning_goal": "学生能识别订单拆分策略的应用场景和潜在问题。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "订单拆分",
          "en": "Order Splitting"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 20,
          "explanation": "订单拆分的主要目的就是通过将大单拆小来降低对市场的即时冲击。",
          "options": [
            "市场流动性极好，可以一次性成交大量订单",
            "交易员希望尽快完成交易，不考虑成本",
            "交易员需要买入大量股票，但希望最小化对价格的即时影响",
            "交易员只进行小额交易"
          ],
          "question_id": "q_quiz_order_splitting_v1",
          "stem": "以下哪种情况最适合使用订单拆分策略？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "多次小额交易会产生更多的交易费用（如佣金），从而增加总成本。",
          "options": [
            "增加了市场冲击",
            "可能导致更高的总交易成本",
            "无法控制成交价格",
            "需要更复杂的算法"
          ],
          "question_id": "q_quiz_order_splitting_v2",
          "stem": "订单拆分策略的一个潜在缺点是？"
        }
      ]
    },
    {
      "concept_key": "limit_orders",
      "coverage_tags": [
        "use_of_limit_orders"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_limit_orders",
      "learning_goal": "学生能判断关于限价单陈述的正确性。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "true_false",
      "term_refs": [
        {
          "display": "限价单",
          "en": "Limit Order"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "限价单可以控制成交价，但并不能完全消除市场冲击，尤其是当订单量很大时，仍可能影响市场。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_limit_orders_v1",
          "stem": "使用限价单可以完全消除市场冲击。"
        },
        {
          "answer": 0,
          "estimated_seconds": 15,
          "explanation": "限价单指定了价格，因此可以避免以不利价格成交，这是它相对于市价单的主要优势。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_limit_orders_v2",
          "stem": "与市价单相比，限价单的主要优势在于可以控制成交价格。"
        }
      ]
    },
    {
      "concept_key": "twap",
      "coverage_tags": [
        "twap_strategy"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_twap",
      "learning_goal": "学生能计算简单的TWAP交易量，并理解其局限性。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "时间加权平均价格",
          "en": "Time-weighted Average Price (TWAP)"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 25,
          "explanation": "执行次数 = 30分钟 / 5分钟 = 6次。每次买入量 = 600股 / 6次 = 100股。",
          "options": [
            "100股",
            "120股",
            "60股",
            "200股"
          ],
          "question_id": "q_quiz_twap_v1",
          "stem": "一个TWAP策略计划在30分钟内买入600股，每5分钟执行一次。每次应买入多少股？"
        },
        {
          "answer": 3,
          "estimated_seconds": 20,
          "explanation": "TWAP假设价格稳定，高波动市场会导致成交价格与预期产生较大偏差。",
          "options": [
            "价格稳定、低波动的市场",
            "价格持续上涨的趋势市场",
            "价格持续下跌的趋势市场",
            "价格高波动、方向不明的市场"
          ],
          "question_id": "q_quiz_twap_v2",
          "stem": "TWAP策略在哪种市场环境下效果最差？"
        }
      ]
    },
    {
      "concept_key": "vwap",
      "coverage_tags": [
        "vwap_strategy"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_vwap",
      "learning_goal": "学生能理解VWAP的计算逻辑和应用场景。",
      "linked_steps": [
        "step2"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "成交量加权平均价格",
          "en": "Volume-weighted Average Price (VWAP)"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "VWAP = Σ(价格 × 成交量) / Σ(成交量)，分子是总成交金额。",
          "options": [
            "所有交易的总成交量",
            "所有交易的总金额（价格×成交量）",
            "所有交易的价格之和",
            "所有交易的成交量之和"
          ],
          "question_id": "q_quiz_vwap_v1",
          "stem": "VWAP的计算公式中，分子代表什么？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "通过跟随市场成交量分布执行订单，VWAP策略旨在将大订单“融入”市场，从而减少对价格的冲击。",
          "options": [
            "最大化交易利润",
            "最小化市场冲击",
            "确保订单立即成交",
            "预测未来价格走势"
          ],
          "question_id": "q_quiz_vwap_v2",
          "stem": "一个交易员使用VWAP策略，其目标是使自己的成交价接近VWAP。这主要有助于："
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L8/plain.txt",
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
      "coverage_tag": "order_book_definition",
      "covered_by": [
        "qf_flash_order_book_def",
        "qf_quiz_order_book_side"
      ],
      "description": "订单簿的定义：记录所有买入和卖出挂单的电子列表。"
    },
    {
      "coverage_tag": "bid_book_ask_book",
      "covered_by": [
        "qf_flash_bid_ask_role",
        "qf_quiz_order_book_side"
      ],
      "description": "买盘簿代表需求，卖盘簿代表供给。"
    },
    {
      "coverage_tag": "order_book_insights",
      "covered_by": [
        "qf_flash_order_book_insights",
        "qf_quiz_order_book_insights",
        "qf_long_order_book_insights"
      ],
      "description": "订单簿能揭示的四种信息：供需、支撑阻力、流动性、市场深度。"
    },
    {
      "coverage_tag": "market_liquidity",
      "covered_by": [
        "qf_flash_liquidity_depth",
        "qf_quiz_order_book_insights"
      ],
      "description": "市场流动性：在不引起价格大幅波动的情况下买卖资产的难易程度。"
    },
    {
      "coverage_tag": "market_depth",
      "covered_by": [
        "qf_flash_liquidity_depth",
        "qf_quiz_order_book_insights"
      ],
      "description": "市场深度：在不同价格水平上可交易的订单数量。"
    },
    {
      "coverage_tag": "support_resistance_level",
      "covered_by": [
        "qf_flash_order_book_insights",
        "qf_long_order_book_insights"
      ],
      "description": "支撑位与阻力位：从大额订单中洞察。"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "order_book_definition",
      "coverage_tags": [
        "order_book_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_order_book_def",
      "learning_goal": "学生能准确回忆订单簿的定义及其基本构成。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "订单簿是什么，以及它记录什么内容。",
      "term_refs": [
        {
          "display": "订单簿",
          "en": "Order Book"
        }
      ],
      "variants": [
        {
          "back": "记录所有买入和卖出挂单的电子列表。",
          "estimated_seconds": 8,
          "explanation": "订单簿是市场微观结构的核心，实时显示所有未成交的买卖意愿。",
          "front": "订单簿（Order Book）是什么？",
          "question_id": "q_flash_order_book_def_v1"
        },
        {
          "back": "买入订单（买单）和卖出订单（卖单）。",
          "estimated_seconds": 6,
          "explanation": "买单挂在买盘簿（Bid Book），卖单挂在卖盘簿（Ask Book）。",
          "front": "订单簿中记录哪两类订单？",
          "question_id": "q_flash_order_book_def_v2"
        }
      ]
    },
    {
      "concept_key": "bid_book_ask_book",
      "coverage_tags": [
        "bid_book_ask_book"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_bid_ask_role",
      "learning_goal": "学生能区分买盘簿和卖盘簿分别代表市场的哪一方。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "买盘簿和卖盘簿各自代表需求还是供给。",
      "term_refs": [
        {
          "display": "买盘簿",
          "en": "Bid Book"
        },
        {
          "display": "卖盘簿",
          "en": "Ask Book"
        }
      ],
      "variants": [
        {
          "back": "需求方。",
          "estimated_seconds": 5,
          "explanation": "买盘簿记录了所有买入意愿，反映市场对资产的需求。",
          "front": "买盘簿（Bid Book）代表市场的哪一方？",
          "question_id": "q_flash_bid_ask_role_v1"
        },
        {
          "back": "供给方。",
          "estimated_seconds": 5,
          "explanation": "卖盘簿记录了所有卖出意愿，反映市场对资产的供给。",
          "front": "卖盘簿（Ask Book）代表市场的哪一方？",
          "question_id": "q_flash_bid_ask_role_v2"
        }
      ]
    },
    {
      "concept_key": "order_book_insights",
      "coverage_tags": [
        "order_book_insights",
        "support_resistance_level"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_order_book_insights",
      "learning_goal": "学生能列举订单簿可以揭示的至少两种市场信息。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "订单簿能提供的四种关键洞察。",
      "term_refs": [
        {
          "display": "支撑位",
          "en": "Support Level"
        },
        {
          "display": "阻力位",
          "en": "Resistance Level"
        }
      ],
      "variants": [
        {
          "back": "一个强力的支撑位。",
          "estimated_seconds": 10,
          "explanation": "大量买单集中在某个价格，表明该价位有强劲的买入需求，可能阻止价格进一步下跌。",
          "front": "订单簿中某个价格上堆了巨大的买单，这通常暗示什么？",
          "question_id": "q_flash_order_book_insights_v1"
        },
        {
          "back": "市场流动性和市场深度。",
          "estimated_seconds": 10,
          "explanation": "订单越多越密集，流动性越好；深度越厚，大单对价格的冲击越小。",
          "front": "通过观察订单簿，你可以判断哪两种市场特性？",
          "question_id": "q_flash_order_book_insights_v2"
        }
      ]
    },
    {
      "concept_key": "market_liquidity_and_depth",
      "coverage_tags": [
        "market_liquidity",
        "market_depth"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_liquidity_depth",
      "learning_goal": "学生能区分市场流动性和市场深度这两个相关但不同的概念。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "core_difference",
      "retrieval_focus": "市场流动性和市场深度的核心区别。",
      "term_refs": [
        {
          "display": "市场流动性",
          "en": "Market Liquidity"
        },
        {
          "display": "市场深度",
          "en": "Market Depth"
        }
      ],
      "variants": [
        {
          "back": "在不引起价格大幅波动的情况下，买卖资产的难易程度。",
          "estimated_seconds": 8,
          "explanation": "流动性好的市场，大额交易不会显著影响价格。",
          "front": "市场流动性衡量的是什么？",
          "question_id": "q_flash_liquidity_depth_v1"
        },
        {
          "back": "在不同价格水平上可交易的订单数量。",
          "estimated_seconds": 8,
          "explanation": "深度越厚，意味着在每个价位都有较多的订单，大单的冲击因此被分散。",
          "front": "市场深度衡量的是什么？",
          "question_id": "q_flash_liquidity_depth_v2"
        }
      ]
    }
  ],
  "lesson_id": "L8",
  "longform_families": [
    {
      "concept_key": "order_book_insights",
      "coverage_tags": [
        "order_book_insights",
        "support_resistance_level"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_order_book_insights",
      "learning_goal": "学生能用自己的话解释订单簿如何揭示市场的微观结构，并举例说明。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "支撑位",
          "en": "Support Level"
        },
        {
          "display": "阻力位",
          "en": "Resistance Level"
        },
        {
          "display": "市场流动性",
          "en": "Market Liquidity"
        },
        {
          "display": "市场深度",
          "en": "Market Depth"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "支撑位的定义",
            "阻力位的定义",
            "订单簿中如何识别它们",
            "背后的市场心理"
          ],
          "question_id": "q_long_order_book_insights_v1",
          "reference_answer": [
            "支撑位是指价格下跌时，由于大量买单的存在，可能阻止价格进一步下跌的价格水平。",
            "阻力位是指价格上涨时，由于大量卖单的存在，可能阻止价格进一步上涨的价格水平。",
            "在订单簿中，如果某个价格水平上堆积了巨大的买单（买盘簿），这通常意味着有强劲的买入需求，可能形成支撑位。",
            "相反，如果某个价格水平上堆积了巨大的卖单（卖盘簿），则可能形成阻力位。",
            "背后的逻辑是，这些大额订单代表了市场参与者在特定价格上的强烈供需意愿，从而对价格产生心理和实际的阻挡作用。"
          ],
          "rubric_points": [
            "正确解释支撑位是价格下跌时可能遇到买盘支撑的水平",
            "正确解释阻力位是价格上涨时可能遇到卖盘压力的水平",
            "指出订单簿中大量买单堆积处可能形成支撑位，大量卖单堆积处可能形成阻力位",
            "解释这反映了市场参与者在特定价位的集中供需意愿"
          ],
          "stem": "请解释订单簿如何帮助交易者判断市场的支撑位和阻力位，并说明其背后的逻辑。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "市场流动性的定义",
            "市场深度的定义",
            "从订单簿观察流动性的方法",
            "从订单簿观察深度的方法",
            "两者的关系"
          ],
          "question_id": "q_long_order_book_insights_v2",
          "reference_answer": [
            "市场流动性衡量的是在不引起价格大幅波动的情况下，买卖资产的难易程度。流动性好的市场，大额交易对价格影响小。",
            "市场深度是指在不同价格水平上可交易的订单数量。深度越厚，意味着在每个价位都有较多的订单等待成交。",
            "从订单簿看，如果买卖盘在各个价格水平上都有大量且密集的订单，说明市场流动性好。",
            "从订单簿看，通过分析每个价格水平上的订单数量，可以评估市场深度。例如，一个深度厚的订单簿，其买卖盘在各个价位都有显著的挂单量。",
            "市场深度是衡量市场流动性的一个重要指标。通常，市场深度越大，流动性越好，大额订单对价格的冲击也越小。"
          ],
          "rubric_points": [
            "正确解释市场流动性是资产在不引起价格大幅波动的情况下被买卖的难易程度",
            "正确解释市场深度是在不同价格水平上可交易的订单数量",
            "指出流动性可以通过订单簿中订单的密集程度来观察",
            "指出深度可以通过分析各价格水平的订单数量来观察",
            "解释深度是流动性的一个维度，深度越厚通常流动性越好"
          ],
          "stem": "请解释市场流动性和市场深度这两个概念的区别，并说明它们如何从订单簿中体现。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "bid_book_ask_book",
      "coverage_tags": [
        "order_book_definition",
        "bid_book_ask_book"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_order_book_side",
      "learning_goal": "学生能在测验中准确识别订单簿中代表需求和供给的边。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "买盘簿",
          "en": "Bid Book"
        },
        {
          "display": "卖盘簿",
          "en": "Ask Book"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "买盘簿记录了所有买入意愿，代表市场的需求方。",
          "options": [
            "卖盘簿 (Ask Book)",
            "买盘簿 (Bid Book)",
            "成交记录 (Trade History)",
            "价格深度 (Price Depth)"
          ],
          "question_id": "q_quiz_order_book_side_v1",
          "stem": "订单簿的哪一边代表市场的需求？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "卖盘簿记录了所有卖出意愿，代表市场的供给方。",
          "options": [
            "市场的需求",
            "市场的供给",
            "已成交的订单",
            "市场的波动率"
          ],
          "question_id": "q_quiz_order_book_side_v2",
          "stem": "在订单簿中，卖盘簿 (Ask Book) 代表什么？"
        }
      ]
    },
    {
      "concept_key": "order_book_insights",
      "coverage_tags": [
        "order_book_insights",
        "market_liquidity",
        "market_depth"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_order_book_insights",
      "learning_goal": "学生能识别订单簿可以提供的多种市场洞察。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "multiple_choice",
      "term_refs": [
        {
          "display": "市场流动性",
          "en": "Market Liquidity"
        },
        {
          "display": "市场深度",
          "en": "Market Depth"
        }
      ],
      "variants": [
        {
          "answer": [
            0,
            1,
            2,
            3
          ],
          "estimated_seconds": 25,
          "explanation": "订单簿可以揭示供需、支撑阻力、流动性和深度，共四种关键信息。",
          "options": [
            "市场的供需关系",
            "支撑位与阻力位",
            "市场流动性",
            "市场深度"
          ],
          "question_id": "q_quiz_order_book_insights_v1",
          "stem": "以下哪些是订单簿可以告诉我们的信息？（多选）"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "市场深度越厚，意味着在每个价位有更多订单，大额订单的影响被分散，因此冲击更小。",
          "options": [
            "深度越厚，大单对价格的冲击越大",
            "深度越厚，大单对价格的冲击越小",
            "深度与流动性无关",
            "深度只与买单数量有关"
          ],
          "question_id": "q_quiz_order_book_insights_v2",
          "stem": "关于市场深度（Market Depth），以下哪个描述是正确的？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L8/plain.txt",
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
      "coverage_tag": "queuing_theory_intro",
      "covered_by": [
        "qf_flash_queuing_intro",
        "qf_quiz_queuing_intro",
        "qf_long_queuing_analogy"
      ],
      "description": "排队论的基本概念及其在订单簿建模中的应用"
    },
    {
      "coverage_tag": "arrival_and_service_rate",
      "covered_by": [
        "qf_flash_arrival_service",
        "qf_quiz_arrival_service"
      ],
      "description": "到达率 (λ) 和服务率 (μ) 的定义与区别"
    },
    {
      "coverage_tag": "traffic_intensity",
      "covered_by": [
        "qf_flash_traffic_intensity",
        "qf_quiz_traffic_intensity",
        "qf_long_traffic_intensity_analysis"
      ],
      "description": "交通强度 (ρ = λ/μ) 的定义、计算与系统稳定性条件"
    },
    {
      "coverage_tag": "system_stability_condition",
      "covered_by": [
        "qf_flash_stability",
        "qf_quiz_stability"
      ],
      "description": "系统稳定条件 (λ < μ) 及 ρ 不同取值对系统的影响"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "queuing_theory_intro",
      "coverage_tags": [
        "queuing_theory_intro"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_queuing_intro",
      "learning_goal": "学生能说出排队论在订单簿建模中的核心用途。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "排队论用于分析订单簿中的哪个核心问题？",
      "term_refs": [
        {
          "display": "排队论",
          "en": "Queuing Theory"
        }
      ],
      "variants": [
        {
          "back": "一个订单要等多久才能成交。",
          "estimated_seconds": 8,
          "explanation": "排队论将订单视为排队等待服务的顾客，服务即成交，因此可以估算等待时间。",
          "front": "排队论在订单簿建模中主要用于回答什么问题？",
          "question_id": "q_flash_queuing_intro_v1"
        },
        {
          "back": "订单的成交（执行）。",
          "estimated_seconds": 8,
          "explanation": "每个订单被视为队列中的顾客，服务过程就是订单被匹配和执行。",
          "front": "在排队论模型中，订单簿中的“服务”指的是什么？",
          "question_id": "q_flash_queuing_intro_v2"
        }
      ]
    },
    {
      "concept_key": "arrival_and_service_rate",
      "coverage_tags": [
        "arrival_and_service_rate"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_arrival_service",
      "learning_goal": "学生能区分到达率和服务率，并给出各自的定义。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "区分到达率和服务率的定义。",
      "term_refs": [
        {
          "display": "到达率",
          "en": "Arrival Rate (λ)"
        },
        {
          "display": "服务率",
          "en": "Service Rate (μ)"
        }
      ],
      "variants": [
        {
          "back": "单位时间内新订单到达市场的平均数量。",
          "estimated_seconds": 8,
          "explanation": "例如，每秒有多少新限价单进入订单簿。",
          "front": "排队论中，到达率 (λ) 的定义是什么？",
          "question_id": "q_flash_arrival_service_v1"
        },
        {
          "back": "单位时间内订单被成交的平均数量。",
          "estimated_seconds": 8,
          "explanation": "例如，每秒有多少订单被匹配和执行。",
          "front": "排队论中，服务率 (μ) 的定义是什么？",
          "question_id": "q_flash_arrival_service_v2"
        }
      ]
    },
    {
      "concept_key": "traffic_intensity",
      "coverage_tags": [
        "traffic_intensity"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_traffic_intensity",
      "learning_goal": "学生能计算交通强度 ρ 并判断系统状态。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "micro_calc",
      "retrieval_focus": "计算给定 λ 和 μ 下的 ρ 值。",
      "term_refs": [
        {
          "display": "交通强度",
          "en": "Traffic Intensity (ρ)"
        }
      ],
      "variants": [
        {
          "back": "0.67 (或 2/3)",
          "estimated_seconds": 10,
          "explanation": "ρ = λ / μ = 10 / 15 = 0.67。系统是稳定的，因为 ρ < 1。",
          "front": "如果订单到达率 λ = 10 单/分钟，服务率 μ = 15 单/分钟，交通强度 ρ 是多少？",
          "question_id": "q_flash_traffic_intensity_v1"
        },
        {
          "back": "2",
          "estimated_seconds": 10,
          "explanation": "ρ = λ / μ = 20 / 10 = 2。系统是不稳定的，因为 ρ > 1，队列会无限增长。",
          "front": "如果订单到达率 λ = 20 单/分钟，服务率 μ = 10 单/分钟，交通强度 ρ 是多少？",
          "question_id": "q_flash_traffic_intensity_v2"
        }
      ]
    },
    {
      "concept_key": "system_stability_condition",
      "coverage_tags": [
        "system_stability_condition"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_stability",
      "learning_goal": "学生能根据 λ 和 μ 的关系判断系统是否稳定。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "cause_effect",
      "retrieval_focus": "系统稳定的条件是什么？",
      "term_refs": [
        {
          "display": "交通强度",
          "en": "Traffic Intensity (ρ)"
        }
      ],
      "variants": [
        {
          "back": "到达率小于服务率 (λ < μ)。",
          "estimated_seconds": 8,
          "explanation": "只有当 λ < μ 时，队列才不会无限增长，系统才能达到稳态。",
          "front": "在排队论模型中，系统稳定的条件是什么？",
          "question_id": "q_flash_stability_v1"
        },
        {
          "back": "系统不稳定，队列会无限增长。",
          "estimated_seconds": 8,
          "explanation": "ρ > 1 意味着到达率大于服务率，订单积压会越来越严重。",
          "front": "如果交通强度 ρ > 1，排队系统会怎样？",
          "question_id": "q_flash_stability_v2"
        }
      ]
    }
  ],
  "lesson_id": "L8",
  "longform_families": [
    {
      "concept_key": "queuing_theory_intro",
      "coverage_tags": [
        "queuing_theory_intro"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_queuing_analogy",
      "learning_goal": "学生能用排队论的术语解释订单簿中订单的等待和执行过程。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "排队论",
          "en": "Queuing Theory"
        },
        {
          "display": "到达率",
          "en": "Arrival Rate (λ)"
        },
        {
          "display": "服务率",
          "en": "Service Rate (μ)"
        },
        {
          "display": "交通强度",
          "en": "Traffic Intensity (ρ)"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "定义排队论中的三个核心概念。",
            "将每个概念映射到订单簿的具体场景。",
            "说明交通强度如何影响订单的等待时间。"
          ],
          "question_id": "q_long_queuing_analogy_v1",
          "reference_answer": [
            "排队论中，到达率 (λ) 指单位时间内新事件到达的数量，在订单簿中对应新限价单进入市场的速率。",
            "服务率 (μ) 指单位时间内事件被处理的数量，对应订单被匹配和执行的速率。",
            "交通强度 (ρ = λ/μ) 衡量系统繁忙程度。",
            "一个限价单挂单后，它进入队列等待，直到到达队列头部（排队时间），然后被成交（服务时间）。",
            "如果 ρ 接近 1（例如 λ=9, μ=10），系统非常拥挤，订单需要等待较长时间才能成交。",
            "如果 ρ 较小（例如 λ=2, μ=10），系统空闲，订单很快就能被处理。"
          ],
          "rubric_points": [
            "正确解释到达率 (λ) 是单位时间新订单到达数量。",
            "正确解释服务率 (μ) 是单位时间订单成交数量。",
            "正确解释交通强度 (ρ = λ/μ) 衡量系统繁忙程度。",
            "将 λ 映射到新限价单进入订单簿的速率。",
            "将 μ 映射到订单被匹配执行的速率。",
            "将 ρ 映射到订单簿的拥挤程度，ρ 越大等待时间越长。"
          ],
          "stem": "请用排队论的基本概念（到达率、服务率、交通强度）解释一个限价单从挂单到成交的过程。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "计算交通强度 ρ。",
            "根据 ρ 判断系统稳定性。",
            "解释对交易者的实际含义。"
          ],
          "question_id": "q_long_queuing_analogy_v2",
          "reference_answer": [
            "交通强度 ρ = λ / μ = 5 / 8 = 0.625。",
            "由于 ρ < 1，系统是稳定的，这意味着队列不会无限增长，订单最终都会被处理。",
            "对于交易者来说，这意味着市场能够有效地处理订单流。",
            "平均等待时间可以通过公式 W = 1/(μ - λ) = 1/(8-5) = 0.33 分钟 ≈ 20 秒来计算。",
            "这意味着一个限价单平均需要等待约 20 秒才能被成交，这是一个相对健康的市场状态。"
          ],
          "rubric_points": [
            "正确计算 ρ = 5/8 = 0.625。",
            "正确判断系统稳定 (ρ < 1)。",
            "解释系统稳定意味着队列不会无限增长。",
            "解释对交易者意味着订单平均等待时间有限，市场流动性较好。"
          ],
          "stem": "假设一个订单簿的到达率 λ = 5 单/分钟，服务率 μ = 8 单/分钟。请计算交通强度 ρ，并解释这个系统是否稳定，以及这对交易者意味着什么。"
        }
      ]
    },
    {
      "concept_key": "traffic_intensity",
      "coverage_tags": [
        "traffic_intensity",
        "system_stability_condition"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_traffic_intensity_analysis",
      "learning_goal": "学生能比较不同交通强度下订单簿系统的行为差异。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "compare_and_contrast",
      "term_refs": [
        {
          "display": "交通强度",
          "en": "Traffic Intensity (ρ)"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "分别计算或描述两种 ρ 值下的系统状态。",
            "分析队列长度的差异。",
            "讨论对交易者（尤其是使用限价单的交易者）的影响。"
          ],
          "question_id": "q_long_traffic_intensity_analysis_v1",
          "reference_answer": [
            "系统稳定性：两种情况下系统都是稳定的，因为 ρ < 1。",
            "队列长度：ρ=0.9 时，平均队列长度 L_q = ρ^2/(1-ρ) = 0.81/0.1 = 8.1 个订单；ρ=0.3 时，L_q = 0.09/0.7 ≈ 0.13 个订单。",
            "等待时间：ρ=0.9 时，平均等待时间 W_q = ρ/(μ(1-ρ)) 会很长；ρ=0.3 时，等待时间很短。",
            "交易者体验：在 ρ=0.3 的市场中，限价单几乎立即成交，流动性好。在 ρ=0.9 的市场中，限价单可能需要等待较长时间，交易者可能需要考虑使用市价单或调整报价。"
          ],
          "rubric_points": [
            "正确指出两种情况下系统都是稳定的 (ρ < 1)。",
            "正确指出 ρ=0.9 时系统更拥挤。",
            "正确指出 ρ=0.3 时队列短，等待时间短。",
            "正确指出 ρ=0.9 时队列长，等待时间长。",
            "解释对交易者的影响：ρ=0.3 时限价单成交快，ρ=0.9 时限价单可能长时间未成交。"
          ],
          "stem": "比较交通强度 ρ = 0.3 和 ρ = 0.9 两种情况下订单簿系统的行为。请从系统稳定性、队列长度和交易者体验三个方面进行分析。"
        },
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "计算不同 λ 下的 ρ 值。",
            "描述 ρ 增加时队列长度的变化趋势。",
            "讨论 ρ 接近 1 时系统的脆弱性。"
          ],
          "question_id": "q_long_traffic_intensity_analysis_v2",
          "reference_answer": [
            "当 λ=2 时，ρ=0.2，系统非常空闲，平均队列长度 L_q = 0.04/0.8 = 0.05。",
            "当 λ=5 时，ρ=0.5，L_q = 0.25/0.5 = 0.5。",
            "当 λ=8 时，ρ=0.8，L_q = 0.64/0.2 = 3.2。",
            "当 λ=9 时，ρ=0.9，L_q = 0.81/0.1 = 8.1。",
            "可以看到，随着 ρ 增加，队列长度加速增长。",
            "当 ρ 接近 1 时，系统非常脆弱：如果 λ 突然增加到 10.1 (ρ=1.01)，系统就会变得不稳定，队列无限增长。",
            "这意味着在高流量市场中，订单簿需要更高的服务率来维持稳定。"
          ],
          "rubric_points": [
            "正确计算 λ=2 时 ρ=0.2，λ=9 时 ρ=0.9。",
            "正确指出队列长度随 ρ 增加而加速增长。",
            "正确指出 ρ 接近 1 时，微小的 λ 增加会导致队列长度急剧增加。",
            "解释系统在 ρ 接近 1 时的脆弱性：任何波动都可能导致系统不稳定。"
          ],
          "stem": "假设一个订单簿系统，服务率 μ 固定为 10 单/分钟。请分析当到达率 λ 从 2 增加到 9 时，交通强度 ρ 和系统行为如何变化。特别关注 ρ 接近 1 时的情况。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "queuing_theory_intro",
      "coverage_tags": [
        "queuing_theory_intro"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_queuing_intro",
      "learning_goal": "学生能识别排队论在订单簿建模中的核心应用。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "排队论",
          "en": "Queuing Theory"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "排队论将订单视为排队等待服务的顾客，用于分析订单从挂单到成交的等待时间。",
          "options": [
            "预测股票价格涨跌",
            "分析订单的等待和执行时间",
            "计算交易佣金",
            "识别市场操纵行为"
          ],
          "question_id": "q_quiz_queuing_intro_v1",
          "stem": "以下哪项是排队论在订单簿建模中的主要应用？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "每个订单被视为队列中的顾客，服务过程就是订单被匹配和执行。",
          "options": [
            "顾客是交易员，服务是下单",
            "顾客是订单，服务是成交",
            "顾客是价格，服务是波动",
            "顾客是成交量，服务是流动性"
          ],
          "question_id": "q_quiz_queuing_intro_v2",
          "stem": "在排队论模型中，订单簿中的“顾客”和“服务”分别对应什么？"
        }
      ]
    },
    {
      "concept_key": "arrival_and_service_rate",
      "coverage_tags": [
        "arrival_and_service_rate"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_arrival_service",
      "learning_goal": "学生能判断关于到达率和服务率的陈述是否正确。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "true_false",
      "term_refs": [
        {
          "display": "到达率",
          "en": "Arrival Rate (λ)"
        },
        {
          "display": "服务率",
          "en": "Service Rate (μ)"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 10,
          "explanation": "服务率是单位时间内订单被成交的平均数量；到达率才是新订单到达的数量。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_arrival_service_v1",
          "stem": "在排队论中，服务率 (μ) 指的是单位时间内新订单到达市场的平均数量。"
        },
        {
          "answer": 0,
          "estimated_seconds": 10,
          "explanation": "交通强度 ρ = λ / μ，因此 λ 和 μ 必须使用相同的时间单位（如每分钟）。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_arrival_service_v2",
          "stem": "到达率 (λ) 和服务率 (μ) 的单位必须一致才能计算交通强度。"
        }
      ]
    },
    {
      "concept_key": "traffic_intensity",
      "coverage_tags": [
        "traffic_intensity"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_traffic_intensity",
      "learning_goal": "学生能根据给定的 λ 和 μ 计算 ρ 并判断系统状态。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "交通强度",
          "en": "Traffic Intensity (ρ)"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "ρ = λ / μ = 8 / 10 = 0.8。由于 ρ < 1，系统是稳定的。",
          "options": [
            "交通强度 ρ = 1.25，系统不稳定",
            "交通强度 ρ = 0.8，系统稳定",
            "交通强度 ρ = 0.8，系统不稳定",
            "交通强度 ρ = 1.25，系统稳定"
          ],
          "question_id": "q_quiz_traffic_intensity_v1",
          "stem": "一个订单簿系统，订单到达率 λ = 8 单/秒，服务率 μ = 10 单/秒。以下哪个说法是正确的？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "ρ 越接近 1，系统越拥挤，但只要 ρ < 1，系统就仍然是稳定的。",
          "options": [
            "系统变得更空闲",
            "系统变得更拥挤，但仍然是稳定的",
            "系统变得不稳定",
            "服务率增加了"
          ],
          "question_id": "q_quiz_traffic_intensity_v2",
          "stem": "如果交通强度 ρ 从 0.5 增加到 0.9，这意味着什么？"
        }
      ]
    },
    {
      "concept_key": "system_stability_condition",
      "coverage_tags": [
        "system_stability_condition"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_stability",
      "learning_goal": "学生能判断关于系统稳定性的陈述是否正确。",
      "linked_steps": [
        "step3"
      ],
      "question_type": "true_false",
      "term_refs": [
        {
          "display": "交通强度",
          "en": "Traffic Intensity (ρ)"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 10,
          "explanation": "当 ρ = 1 时，系统处于临界状态，队列会无限增长，无法达到稳态。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_stability_v1",
          "stem": "当交通强度 ρ = 1 时，排队系统处于稳定状态，队列长度保持不变。"
        },
        {
          "answer": 0,
          "estimated_seconds": 10,
          "explanation": "这是 M/M/1 队列模型的基本稳定条件。如果 λ < μ，系统最终会达到稳态。",
          "options": [
            "正确",
            "错误"
          ],
          "question_id": "q_quiz_stability_v2",
          "stem": "只要到达率小于服务率 (λ < μ)，排队系统就一定是稳定的。"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "input coverage checklist",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "input lesson map",
    "plain_text": "pipeline/1-plain/L8/plain.txt",
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
# L8: Order book and high frequency data modeling

Course Code: COMP7415

# Agenda

- Trading ideas and insights from an order book   
Market impact of a large order   
- Queuing theory for order book modelling   
• Introduction to high frequency trading   
Major types of arbitrage   
- Triangular Arbitrage Strategy   
- Market Making Strategy

# Revision of Order Book

- Order book is a list of buy and sell orders   
- Bid book represents the demand side   
- Ask book represents the supply side   
- An order book can provide insights for microeconomic structure

![](images/1890d1b4de992ca5c1c0849ef2f8bf35661f3778212c240b4f34e7c584840660.jpg)

Order Book   

<table><tr><td></td><td>Bid</td><td>Ask</td><td></td></tr><tr><td>3,151</td><td>9139.60</td><td>9152.80</td><td>167</td></tr><tr><td>1,574</td><td>9138.40</td><td>9153.21</td><td>1,600</td></tr><tr><td>338</td><td>9138.20</td><td>9153.60</td><td>736</td></tr><tr><td>789</td><td>9138.10</td><td>9153.71</td><td>1,602</td></tr><tr><td>800</td><td>9138.00</td><td>9154.10</td><td>1,649</td></tr><tr><td>309</td><td>9137.79</td><td>9154.21</td><td>1,965</td></tr><tr><td>2,140</td><td>9137.60</td><td>9154.50</td><td>3,984</td></tr><tr><td>48</td><td>9137.40</td><td>9154.60</td><td>763</td></tr><tr><td>144</td><td>9137.29</td><td>9154.71</td><td>157</td></tr><tr><td>136</td><td>9137.20</td><td>9154.80</td><td>56</td></tr><tr><td>528</td><td>9137.00</td><td>9155.50</td><td>320</td></tr><tr><td>1,654</td><td>9136.90</td><td>9155.71</td><td>200</td></tr><tr><td>40</td><td>9136.79</td><td>9156.10</td><td>16</td></tr><tr><td>46</td><td>9136.70</td><td>9156.21</td><td>219</td></tr><tr><td>608</td><td>9136.60</td><td>9156.30</td><td>40</td></tr><tr><td>48</td><td>9136.50</td><td>9156.40</td><td>80</td></tr></table>

# Breakdown an Order Book

![](images/be1d9dd9583cc58d1bd249941340f7a1d8d070de10531a5d686b7ca5e950ec99.jpg)  
Level 1

![](images/c13567b7d72046003a7ff73888f9489e3f61d1de71c48c4a337ace79af3f9679.jpg)  
Level 2

![](images/73f455bfb0c450ad79e0403359ca6fa2a0901fcad6dcad3faeb9f93f9af88efb.jpg)  
Level 3

# Order Book Mechanism

- Limit orders operated under a price-time priority rule.

- Limit orders are sorted by the price and higher priority is given to the orders at the best prices   
- Orders at the same price are ranked depending on when they entered the queue according to FIFO

![](images/4f370778225b8f47bb8707a3c2ae2d152a2d63c4ef06f1bcc675dbd8fce3af67.jpg)

# What an order book can tell you?

1. Supply & Demand: Insights into trader behaviour and market sentiment.   
2. Support & Resistance Level: Insights from orders with large volume   
3. Market Liquidity: Measure liquidity through order sizes.   
4. Market Depth: Analysis of the number of buy/sell orders at various price levels.

# Trading Strategies Based on Order Book Data

# 1. Momentum Trading:

Identifying trends through order flow signals.   
- Monitor the ratio of buy to sell orders. A significant imbalance can signal potential price movements.

# 2. Sniping Strategy:

- Taking advantage of large orders (eg. iceberg orders) before they get fully executed.

# 3. Liquidity Provisioning:

- Market-making strategies to earn from the bid-ask spread.

# Market Impact

# Market Impact

A large market order is likely to fill up the order book and lead to   
- significant price movement   
- reduced market depth and liquidity

- Immediate Impact: Price change upon execution of the order.   
- Temporary Impact: Short-term fluctuations that revert after order execution.   
- Permanent Impact: Lasting price changes due to order flow

![](images/e48e94161c73aa4fc59bbf27b0baf348a33f213143f2a2ebac43f58c1bd580ff.jpg)

![](images/28ff73b6a0696bb14e363a757f31d2c67fc1fb232c4f84472041cdc29241ed34.jpg)

# Factors Influencing Market Impact

# 1. Order Size:

- Larger orders typically lead to greater impact

# 2. Order Type:

Market orders vs limit orders

# 3. Market Conditions:

Less liquid markets experience higher impact   
- Volatility and market volume affect impact

# Strategies to Mitigate Market Impact

# 1. Order Splitting:

- Break large orders into smaller segments.   
- Execute over time to reduce immediate impact.

# 2. Use of Limit Orders:

- Place limit orders to control execution price.   
- Avoid the risks of market orders.

# 3. Time-weighted Average Price (TWAP):

- Execute orders evenly over a specified time period.   
- Helps in minimizing market impact.

# 4. Volume-weighted Average Price (VWAP):

- Execute orders in proportion to market volume.   
- Reduces price impact by blending into the market.

# 1. Order Splitting

- If informed investors' trades are the main cause of stock price changes, large trades by informed traders would reveal their information because traders can easily discern those who execute large transactions.   
- However, small trades executed repeatedly will incur excessive transactions costs.   
- Thus, informed traders will tend to fragment orders into medium-sized trades sufficiently small to disguise themselves but large enough to maintain fairly small transactions costs.

# Example: Slicing the Large Order

- Suppose a client of the XYZ brokerage firm wants to buy $X = 80,000$ shares with the best possible execution.   
These execution costs $B$ will include

order processing costs $F$ and   
- market impact (slippage) costs that are increasing in the size $c$ of each of $n$ equal-size transactions.

- Broker XYZ works with a market impact model that estimates the slippage cost $S$ of each execution of size $c = X / n$ with function $S = \nu c^{\mathfrak{m}}$ , where the coefficient $\nu$ and exponent $m$ are related to the liquidity of the security:

$$
B / n = F + v (X / n) ^ {m}
$$

# Example: Slicing the Large Order

- Suppose that broker estimated F to be \(0.03 per order, and v and m to be 0.00001 and 1.5. What are the optimal number of transactions and transaction sizes? We set up a total transactions cost function as follows:

$$
B = n F + n v \left(\frac {X}{n}\right) ^ {m} = 0. 0 3 n + 0. 0 0 0 0 1 \frac {8 0 , 0 0 0 ^ {1 . 5}}{n ^ {0 . 5}} = 0. 0 3 n + 2 2 6. 2 7 n ^ {- 0. 5}
$$

Take derivative of $B$ with respect to $n$ , and solve for $n$ as follows:

$$
\frac {d B}{d n} = \frac {d \left[ n F + n v \left(\frac {X}{n}\right) ^ {m} \right]}{d n} = F - (m - 1) v \frac {X ^ {m}}{n ^ {m}} = 0. 0 3 - 0. 5 (0. 0 0 0 0 1) \frac {X ^ {m}}{n ^ {m}} = 0. 0 3 - \frac {1 1 3 . 1 4}{n ^ {1 . 5}} = 0. 0 2
$$

$$
n = \sqrt [ m ]{\frac {(m - 1) v X ^ {m}}{F}} = \sqrt [ 1. 5 ]{\frac {0 . 5 (0 . 0 0 0 0 1) \cdot 8 0 , 0 0 0 ^ {1 . 5}}{0 . 0 3}} = 2 4 2. 2 8 \approx 2 4 2
$$

- Thus, the optimal number of stock transactions for this order is $n = 242$ , each with transaction size of $X / n$ equal to $80,000 / 242 = 330$ (subject to rounding)

# Improve the order split mechanism

- There are many factors that we can consider when designing the optimal cost or trade-slicing functions.   
- The model above provides no guidance on spacing trades through the day or time frame

- The amount of time elapsing between trades executing at time $t$ and time $t + 1$ would be inversely related to market impact costs. That is, more time elapses between the transactions, slippage will be reduced.   
- Stocks tend to be more liquid during the earliest and latest parts of the day, and least liquid in the middle. That is, one might expect liquidity during the day to be "U"-shaped, and market impact costs to resemble an upside down U.

# 2. Use of Limit Orders

- Suppose we want to buy 1000 shares

- If we directly send a market order, the whole ask book will be cleared and the purchase price can be expensive   
Instead of market order, we can place multiple limit orders at different price levels

Current order book   

<table><tr><td>Bid Size</td><td>Bid Price</td><td>Ask Price</td><td>Ask Size</td></tr><tr><td>20</td><td>9.9</td><td>10.5</td><td>100</td></tr><tr><td>30</td><td>9.6</td><td>11.0</td><td>50</td></tr><tr><td>50</td><td>8.8</td><td>11.6</td><td>300</td></tr><tr><td>50</td><td>8.1</td><td>12.3</td><td>100</td></tr><tr><td>10</td><td>7.5</td><td>12.9</td><td>120</td></tr><tr><td>50</td><td>7.3</td><td>20.0</td><td>200</td></tr><tr><td>10</td><td>7.1</td><td></td><td></td></tr><tr><td>70</td><td>6.9</td><td></td><td></td></tr><tr><td>80</td><td>6.6</td><td></td><td></td></tr></table>

![](images/180115e22b3274fefb2f1b805af5eead3f8797dc508be190c90eaf6e174e8dd9.jpg)

<table><tr><td>Bid Size</td><td>Bid Price</td><td>Ask Price</td><td>Ask Size</td></tr><tr><td>250</td><td>10</td><td>10.5</td><td>100</td></tr><tr><td>20</td><td>9.9</td><td>11.0</td><td>50</td></tr><tr><td>30</td><td>9.6</td><td>11.6</td><td>300</td></tr><tr><td>250</td><td>9.5</td><td>12.3</td><td>100</td></tr><tr><td>250</td><td>9.0</td><td>12.9</td><td>120</td></tr><tr><td>50</td><td>8.8</td><td>20.0</td><td>200</td></tr><tr><td>250</td><td>8.5</td><td></td><td></td></tr><tr><td>50</td><td>8.1</td><td></td><td></td></tr><tr><td>10</td><td>7.5</td><td></td><td></td></tr><tr><td>50</td><td>7.3</td><td></td><td></td></tr><tr><td>10</td><td>7.1</td><td></td><td></td></tr><tr><td>70</td><td>6.9</td><td></td><td></td></tr><tr><td>80</td><td>6.6</td><td></td><td></td></tr></table>

# Potential issues of using limit orders

- Wait too long for the limit order to get filled if the limit price is set too far from the top book.   
- Other market participants may discover the large order buyer/seller if there is a sudden increase in the order book's volume.

- Solution: split into smaller limit order and place slowly to the market

# 3. Time-weighted Average Price (TWAP)

- Place market orders with the same size evenly over a specified time period.   
- Parameters:

- total_order_size: Total number of shares you want to buy or sell.   
- execution_duration: Total time over which to execute the order.   
- interval: Time interval for executing parts of the order.

$$
S h a r e s p e r i n t e r v a l = R o u n d (\frac {t o t a l \_ o r d e r \_ s i z e}{e x e c u t i o n \_ d u r a t i o n / i n t e r v a l}, 0)
$$

# Example: TWAP

- Suppose we want to buy a total of 1000 shares.   
- To reduce the market impact, we decide to buy a small fraction every 5 minutes for a total duration of 1 hour.   
Thus,

- total_order_size = 1000   
- execution_duration = 60   
- interval = 5

• Shares per interval = round(1000 / (60 / 5), 0) = 83   
• Remaining shares = 1000 - 83* (60/5) = 4

# Potential issues of TWAP

- Large orders executed over time can still impact market prices, especially for illiquid markets.   
- Delays or failures in order execution at specified intervals (eg. due to market close) can disrupt the planned strategy.   
- TWAP assumes a stable price environment; high volatility can lead to significant deviations.

# 4. Volume-weighted Average Price (VWAP)

- VWAP is the average price a security has traded at throughout the day, based on both volume and price.   
- Formula

$$
V W A P = \frac {\sum (P r i c e \times V o l u m e)}{\sum V o l u m e}
$$

# Where

Price: The price at which trades occur.   
Volume: The number of shares traded at each price.

# Use of VWAP

- Benchmarking: Provides a standard for evaluating our purchase price.   
- Execution Strategy: Helps traders execute orders with minimal market impact.   
- Trend Indicator: Can signal potential reversals when the price diverges from VWAP.

# VWAP Example

- Assume the following trades occur throughout a trading day:

<table><tr><td>Time</td><td>Price</td><td>Volume</td></tr><tr><td>09:30</td><td>$100</td><td>200</td></tr><tr><td>09:35</td><td>$101</td><td>150</td></tr><tr><td>09:40</td><td>$102</td><td>250</td></tr><tr><td>09:45</td><td>$100</td><td>300</td></tr><tr><td>09:50</td><td>$103</td><td>100</td></tr></table>

- VWAP = $\frac{100 * 200 + 101 * 150 + 102 * 250 + 100 * 300 + 103 * 100}{200 + 150 + 250 + 300 + 100} = 100.95$   
- So we can execute order in smaller increments around the VWAP to avoid impacting the market   
- For example, you might place the following orders close to VWAP:

- Buy 200 shares at \(100.90   
Buy 300 shares at \(100.95   
Buy 500 shares at \(101.00

# Impact of VWAP on Execution

- By using VWAP:

- The majority of orders are executed around the VWAP price, which is $100.95.   
- The average execution price might be slightly higher than the VWAP, but it reduces the likelihood of large market movements caused by a single large order.

# Order Book Modeling with Queuing Theory

# Why Queuing Theory?

- To analyze how orders flow through the market and how they impact price movement.   
Key Questions:

How long do orders wait before being executed?   
What is the optimal order size?   
- How do market conditions (eg. volatility) affect execution time?

# Basic Concepts of Queuing Theory

# - Queue Components

- Arrival Process $(\lambda)$ : Rate at which an event arrives. (eg. customer)   
- Service Rate $(\mu)$ : Rate at which an event is executed.   
- Queue Discipline: How events are processed (e.g. FIFO).

![](images/0be7434e791c4d3578e9a90f2777940e0384ae45c8bcb5895f02b71ecc82e4ac.jpg)

# How is it related to Order Book?

- Incoming orders (buy/sell) can be viewed as entities arriving at a queue. The order book acts as a queue where these orders wait to be matched.   
- Each order can be treated as a customer in a queue, where the "service" is the execution of trades.   
- Arrival and Service Rates:

• Arrival Rate: The rate at which orders enter the order book (eg. how many orders are placed per second).   
- Service Rate: The speed at which orders are matched and executed. This is influenced by the market's liquidity and the efficiency of the trading system.

# How is it related to Order Book?

Event: arrival of a new limit order   
- Queuing time: the time to wait from placing a limit order until it reaches to the top book   
- Service time: the elapsed time for the limit order being matched/executed when it is at the top book

<table><tr><td>Bid Size</td><td>Bid Price</td><td>Ask Price</td><td>Ask Size</td></tr><tr><td>20</td><td>9.9</td><td>10.5</td><td>100</td></tr><tr><td>30</td><td>9.6</td><td>11.0</td><td>50</td></tr><tr><td>50</td><td>8.8</td><td>11.6</td><td>300</td></tr><tr><td>50</td><td>8.1</td><td>12.3</td><td>100</td></tr><tr><td>10</td><td>7.5</td><td>12.9</td><td>120</td></tr><tr><td>50</td><td>7.3</td><td>13.0</td><td>1000</td></tr><tr><td>10</td><td>7.1</td><td>13.4</td><td>50</td></tr><tr><td>70</td><td>6.9</td><td></td><td></td></tr><tr><td>80</td><td>6.6</td><td></td><td></td></tr></table>

![](images/5b8500ae14878dc14a027160ac1c8074a4fbe9e2cbc7c257706f63a33b25eee0.jpg)

<table><tr><td>Bid Size</td><td>Bid Price</td><td>Ask Price</td><td>Ask Size</td></tr><tr><td>150</td><td>12.0</td><td>13.0</td><td>1000</td></tr><tr><td>150</td><td>11.5</td><td>13.4</td><td>50</td></tr><tr><td>100</td><td>10.8</td><td>13.6</td><td>200</td></tr><tr><td>100</td><td>10.5</td><td>14.3</td><td>150</td></tr><tr><td>20</td><td>9.9</td><td>14.9</td><td>100</td></tr><tr><td>30</td><td>9.6</td><td></td><td></td></tr><tr><td>50</td><td>8.8</td><td></td><td></td></tr><tr><td>50</td><td>8.1</td><td></td><td></td></tr><tr><td>10</td><td>7.5</td><td></td><td></td></tr></table>

# Probability density function

# Exponential Distribution

Density function

$$
f (x) = \left\{ \begin{array}{l l} \frac {1}{\lambda} e ^ {\frac {- x}{\lambda}}, & x \geq 0 \\ 0, & x <   0 \end{array} \right.
$$

where $\lambda$ is the average time taken between events.

- Cumulative function

$$
F (x) = \left\{ \begin{array}{l l} 1 - e ^ {\frac {- x}{\lambda}}, & \quad x \geq 0 \\ 0, & \quad x <   0 \end{array} \right.
$$

Expected value $E(X) = \lambda$   
- Variance $\operatorname{Var}\left( X\right)  = {\lambda }^{2}$   
- Memorylessness property

$$
P (X > s + t | X > s) = P (X > t)
$$

![](images/dd5c4057f20a3e41cad7eb055105136c971066fabaa64ed8d8e29f954786d228.jpg)  
Cumulative distribution function

![](images/7a0a83da17eccc89c0c77aab0431082cd8bfe39ec709ffe794f6d0e1f88e04a0.jpg)

# Poisson Distribution

Density function

$$
f (x) = \left\{ \begin{array}{c c} \frac {\theta^ {x} e ^ {- \theta}}{x !}, & x = 0, 1, 2, \ldots \\ 0, & o t h e r w i s e s \end{array} \right.
$$

where $\theta$ is the average number of events occurred in a fixed time interval.

- Cumulative function

$$
F (x) = \sum_ {k = 0} ^ {x} \frac {\theta^ {k} e ^ {- \theta}}{k !}
$$

- $E(X) = \operatorname{Var}(X) = \theta$

![](images/500e3ee9b82188f2e7ea7b2418d0bcd37cc9f49485d854b37e978d2848df04e3.jpg)  
Probability mass function

![](images/0c064f2a280574be7f02aa64fbf21327c4f62e47b48223e646f3dfb9b800113c.jpg)  
Cumulative distribution function

# Poisson vs Exponential Distribution

- The Poisson distribution is used to count the number of events in a fixed time period, while the exponential distribution is used to model the time between these events.   
- If events occur according to a Poisson process with rate $\theta$ , the time between consecutive events follows an exponential distribution with the rate $\lambda$ . Then, we have

$$
\theta = \frac {1}{\lambda}
$$

![](images/53a40ce0460076c3dd4d48f40133fbf4c11f57bdbaeec903986ed538e1c8bb27.jpg)  
Arrival time t1   
Arrival time t2   
Exponential Distribution $f(t)$

![](images/5ae84d9484e5f7116daa354c3c879a3a96b078f6f2e298bb3fb8c08a12b8e4ce.jpg)  
Number of arrival k1   
Number of arrival k2   
Poisson Distribution $f(k)$

# One-side Queuing Model Formulation

M/M/1 Queue:

M: Memoryless arrival process (Poisson)   
M: Memoryless service process (Exponential)   
1: Single service channel (eg. one market maker)

• The model can be described as continuous time Markov chain with transition rate matrix on the state space $\{0,1,2,3,\ldots\}$

![](images/b2068c10c2e588a1c063c2c2d90ef0f92e4779930813cbde73715fdaa35b24ed.jpg)

$$
Q = \left[ \begin{array}{c c c c c} - \lambda & \lambda & & & \\ \mu & - (\mu + \lambda) & \lambda & & \\ & \mu & - (\mu + \lambda) & \lambda & \\ & & \mu & - (\mu + \lambda) & \\ & & & & \ddots \end{array} \right]
$$

![](images/f2dbb65264cb4452f140a4990c99bea6752b36937ce8d22f9634bf884a301045.jpg)

# Transition Rate Matrix

- In a typical M/M/1 queue transition rate matrix, the off-diagonal elements represent the rates of moving between states:

- $Q_{i,i + 1} = \lambda$ : Transition from state $i$ to $i + 1$ (arrival).   
- $Q_{i,i - 1} = \mu$ : Transition from state $i$ to $i - 1$ (service completion).

- The diagonal entry $Q_{ii} = -(\lambda + \mu)$ refers to the total rate of leaving state $i$ .   
- The sum of each row in the transition rate matrix equals zero, meaning that the rate of leaving state $i$ must equal the total rate of entering state $i$ from all other states.

$$
\sum_ {j} Q _ {i j} = 0
$$

# Steady-State Probabilities $(\pi)$

- To find the steady-state probabilities $\pi$ of being in each state, we solve the system of equations derived from the balance equations $\pi Q = 0$ along with the normalization condition $\sum \pi_{i} = 1$ .

- $\pi_{i}$ means the probability of having $i$ events (i.e. limit orders) in the queuing system

- From the steady-state probabilities, we can derive various useful metrics:

Average number of customers in the system (L)   
- Average time a customer spends in the system (W)   
- Utilization of the server $(\rho)$

# Queuing Model Formulation

- The model is considered stable only if $\lambda < \mu$ . If, on average, arrivals happen faster than service completions the queue will grow indefinitely long and the system will not have a stationary distribution. The stationary distribution is the limiting distribution for large values of $t$ .   
- Under stationary process and first-come first-served (FIFO) discipline, the response time (i.e. the sum of waiting and service time) has a probability density function

- i.e. the time for a new limit order entering into the order book until it is filled

$$
f (t) = \left\{ \begin{array}{l l} (\mu - \lambda) e ^ {- (\mu - \lambda) t}, & t > 0 \\ 0, & \text {o t h e r w i s e} \end{array} \right.
$$

# Traffic Intensity

- Traffic Intensity $(\rho)$ : $\rho = \frac{\lambda}{\mu}$   
- Traffic intensity $\rho$ measures the utilization of the queuing system.   
• Ranges

- $\rho < 1$ : The system is stable which can handle incoming traffic without becoming overloaded. The queue will not grow indefinitely. Customers are served on average faster than they arrive.   
- $\rho = 1$ : The system is operating at its capacity. The arrival rate equals the service rate, leading to a steady-state condition where the queue can grow without bound.   
- $\rho > 1$ : The system is unstable. The arrival rate exceeds the service rate, leading to an ever-growing queue.

# Key Results of M/M/1 Queuing Model

- Average time an order spends in the system (W):

$$
W = E (T) = \frac {1}{\mu - \lambda} = \frac {1}{\mu (1 - \rho)}
$$

Average number of orders in the system (L):

$$
L = \lambda W = \frac {\lambda}{\mu - \lambda} = \frac {\rho}{1 - \rho}
$$

- Average time an order waiting in the queue $(W_{q})$ :

$$
W _ {q} = W - \frac {1}{\mu} = \frac {\lambda}{\mu - \lambda} = \frac {\rho}{\mu (1 - \rho)}
$$

- Average number of orders in the queue $(L_{q})$ :

Here $\frac{1}{\mu}$ represents the average service time.

$$
L _ {q} = \lambda W _ {q} = \frac {\rho^ {2}}{1 - \rho}
$$

# Estimating $\lambda$ (Arrival Rate)

- In trading, an "arrival" can refer to new limit orders entering the market.   
- The arrival rate $\lambda$ can be calculated as

$$
\lambda = \frac {T o t a l n u m b e r o f a r r i v a l s}{T o t a l t i m e p e r i o d}
$$

- Example: if a trading system receives 600 limit orders over a 10-minute period, $\lambda$ would be

$$
\lambda = \frac {6 0 0}{1 0} = 6 0 o r d e r s p e r m i n u t e
$$

# Estimating $\mu$ (Service Rate)

- In trading, the "service" could refer to the execution of orders.   
- The service rate $\mu$ can be calculated as:

$$
\mu = \frac {1}{A v e r a g e s e r v i c e t i m e}
$$

Example: If the average time to execute an order is 2 seconds, then

$$
\mu = \frac {1}{2} = 0. 5 o r d e r s p e r s e c o n d = 3 0 o r d e r s p e r m i n u t e
$$

# Example

- In an order book, buy orders arrive at an average rate of 12 orders per minute, while sell orders arrive at an average rate of 10 orders per minute.   
- Assuming that both types of orders are processed immediately and that the system operates under a Poisson arrival process, what is the expected number of buy orders in the queue at any given time?   
• Answer:

- Arrival rate $\lambda$ for buy orders $= 12$   
- Service rate $\mu = 12 + 10$ (i.e. the total rate at which orders can be processed)   
Average number of orders in the system (L):

$$
L = \frac {\lambda}{\mu - \lambda} = \frac {1 2}{2 2 - 1 2} = 1. 2
$$

Average number of buy orders in the queue $(L_{q})$

$$
L _ {q} = \frac {\lambda}{\mu} L = \frac {1 2}{2 2} 1. 2 = 0. 6 5
$$

# Remarks

- Time Period: Ensure that the time period for your data collection is consistent when calculating $\lambda$ and $\mu$ .   
- Market Conditions: Be aware that both $\lambda$ and $\mu$ can vary significantly with market conditions (eg. volatility, liquidity).   
- Statistical Methods: You can use statistical methods to refine your estimates, such as calculating confidence intervals or conducting regression analysis if you have larger datasets.

# Key Assumptions of the Basic Queuing Model

- Poisson Arrivals:

- Assumes that orders arrive randomly and independently.   
Real-world order flows can exhibit clustering or trends.

- Exponential Service Times:

- Assumes a constant average execution time for orders.   
- In reality, execution times can vary significantly due to order size, market volatility, etc.

- Single Server:

- Assumes a single market maker or execution venue.   
- Multiple venues and decentralized markets can complicate execution dynamics

# Trading Strategies based on Queuing Theory

# 1. Optimal Order Placement

- Concept: Apply queuing principles to optimize order placement.   
Strategy:

- Analyze the average waiting times and queue lengths to determine the best times to place orders.   
- Use limit orders strategically when the queue is shorter (lower waiting times), and market orders when the queue is longer.   
- Adjust order sizes based on the current traffic intensity $(\rho)$ to avoid slippage.

# Trading Strategies based on Queuing Theory

# 2. Order Flow Analysis

- Concept: Use arrival rates $(\lambda)$ and service rates $(\mu)$ to analyse market liquidity.   
Strategy:

- Monitor the number of incoming orders and execution times to gauge market activity.   
- Increase trading frequency during periods of high arrival rates to capitalize on liquidity.   
- Reduce trading during low liquidity periods.

# Trading Strategies based on Queuing Theory

# 3. Queue Analysis

- Concept: Utilize queue lengths to inform directional move.   
Strategy:

- When the queue length (or the area) of bid book is much longer than that of ask book, it may imply a strong market demand and suggest a short-term upward trend   
- Pair this with indicators that signal buy or sell conditions.

![](images/82f9716bbdf26269dd93d2e07f3deae061ab9a2f39aedeeaa6925abcedf129ed.jpg)

# Backtest Order Book Imbalance

# Trading Logic:

- For each second,

- Collect the latest bid-book and ask-book data   
- Compute the total volume in both bid-book and ask-book respectively   
Market Entry:

- If last trade time is 15 minutes before,

- If bid-book volume $>2^{*}$ ask-book volume, open a BUY order   
- If ask-book volume $>2^{*}$ bid-book volume, open a SELL order

Market Exit:

- Close if previously open a BUY order, but now bid-book volume $<$ ask-book volume   
- Close if previously open a SELL order, but now ask-book volume $<$ bid-book volume

# Backtest Settings

- Backtest Period = 2024-01   
- Data Interval = Tick   
- Initial Capital = 10,000 USD   
- Leverage = 5   
Transaction Cost = 0   
- Allow Short Sell = True   
- Position Netting = False   
- Instruments = BTCUSDM

# Settings

# Strategy Name

# Order Imbalance

# Start Period

2024-01

# End Period *

2024-01

#

# Data Interval

tick

#

# Initial Capital

10000.0

# Base Currency

USD

#

# Leverage

5.0

# Transaction Cost

0.0

# Risk free Rate

0.0

# Enable Short Sell

![](images/0184a33af81ba2fa33a9d0299f09252734d9d40521a32559682819301d3c3d89.jpg)

# Position Netting

![](images/30378578eca83fe332a2241da4271d2409766d5623155d0b2380817d68d54462.jpg)

# News Data

![](images/84970d597dc2e9e8af22e18fafaf8555523902ab13f8ff611bd7f8dba7e7dfcc.jpg)

# Weather Data

![](images/0832eaafa31015b4af659614e6f096f1972e24fd0d6c2fbd28e3a8b0b93653aa.jpg)

# Economics Data

![](images/a727deaf39c23328cfd9c32ceac0385cb5e785f00b254b5d71f008d48398be8b.jpg)

# Instruments

![](images/7addec570ed93cdf7e3a14df50c799f93f4e454a771a98d8c772ee858a47145e.jpg)

# (Max. 100 items)

BTCUSD X

# Full Script

```python
from AlgoAPI import AlgoAPIUtil, AlgoAPI_Backtest from datetime import datetime, timedelta 
```

class AlgoEvent: def __init__(self): self_timer $=$ datetime(1970,1,1) self.last_trade $=$ datetime(1970,1,1) self.thre $= 2$

```python
def start(self, mEvt): self.evt = AlgoAPI_Backtest.AlgoEvtHandler(self, mEvt) self.evt.start() 
```

def on_marketdatafeed(self, md, ab): # reset grid every 1 minute if md.timestamp $> =$ self.timemr+timedelta(seconds=1): # update timer self.timemr $\equiv$ md.timestamp

```python
calculate cumulative volume for bid and ask book  
bid_volume, ask_volume = 0, 0  
for p,v in md.bidOrderBook:  
    bid_volume += v  
for p,v in md.askOrderBook:  
    ask_volume += v 
```

```python
# entry signal
if md.timestamp >= self.last_trade + timedelta(minutes=15):
    if bid_volume > self.thre*ask_volume:
        self.last_trade = md.timestamp
        order = AlgoAPIUtil.OrderObject(
            instrument=md.instrument,
            openclose='open',
            buysell=1,
            volume=0.0001
        )
        self.evt.sendOrder(order)
elif ask_volume > self.thre*bid_volume:
    self.last_trade = md.timestamp
    order = AlgoAPIUtil.OrderObject(
        instrument=md.instrument,
        openclose='open',
        buysell=-1,
        volume=0.0001
    )
    self.evt.sendOrder(order) 
```

# Backtest Result

![](images/1e290d6f6bfd3d8eb11ddfd29ba9d17204770fe9e3adb3b97dd522aa91a09866.jpg)  
Capital Usage:

![](images/89749bdc2d9a3b21d1d9fa280348b41c61adf61a67976ea508590b90a2d6f794.jpg)  
Equity Drawdown $(\%)$ :

Performance Statistics: 1   

<table><tr><td>No. of Tradable Days:</td><td>31</td><td>No. of Win Days:</td><td>16</td><td>No. of Loss Days:</td><td>14</td></tr><tr><td>Win Rate:</td><td>53.3333%</td><td>Max. Consecutive Win Day:</td><td>5</td><td>Max. Consecutive Loss Day:</td><td>3</td></tr><tr><td>Odd Ratio:</td><td>1.1429</td><td>Max. Consecutive Gains:</td><td>457.88</td><td>Max. Consecutive Loss:</td><td>-461.27</td></tr><tr><td>No. of Trades:</td><td>2919</td><td>Average Consecutive Win Day:</td><td>0.93</td><td>Average Consecutive Loss Day:</td><td>0.6</td></tr><tr><td>Total PnL:</td><td>-173.62</td><td>Average Per Trade PnL:</td><td>-0.06</td><td>Average Per Day PnL:</td><td>-5.6</td></tr><tr><td>Mean Daily Return:</td><td>-0.0462%</td><td>Median Daily Return:</td><td>0.1058%</td><td>Mean Annual Return:</td><td>-11.6511%</td></tr><tr><td>Daily Return StdDev:</td><td>1.5529%</td><td>25th percentile Daily Return:</td><td>-0.5793%</td><td>75th percentile Daily Return:</td><td>0.5763%</td></tr><tr><td>Daily Return Downside StdDev:</td><td>1.0594%</td><td>95% 1 day return VaR:</td><td>-2.9652%</td><td>99% 1 day return VaR:</td><td>-4.128%</td></tr><tr><td>Daily Sharpe Ratio:</td><td>-0.0298</td><td>Annual Sharpe Ratio:</td><td>-0.4726</td><td>Max. Drawdown Amount:</td><td>995.9</td></tr><tr><td>Daily Sortino Ratio:</td><td>-0.0436</td><td>Annual Sortino Ratio:</td><td>-0.6928</td><td>Max. Drawdown Percent:</td><td>9.2023%</td></tr><tr><td>Max. Drawdown Duration:</td><td>8</td><td>Average Drawdown Duration:</td><td>2.45</td><td>Annual Volatility:</td><td>24.5542%</td></tr><tr><td>Gross Profit:</td><td>0.0</td><td>Gross Loss:</td><td>0.0</td><td>Profit Factor:</td><td>0.0</td></tr><tr><td>Jensen Alpha:</td><td>0.0</td><td>Beta:</td><td>0.0</td><td>Information Ratio:</td><td>0.0</td></tr><tr><td>Omega Ratio:</td><td>0.0</td><td>Treynor Ratio:</td><td>0.0</td><td>Tail Ratio:</td><td>0.8897</td></tr><tr><td>Calmar Ratio:</td><td>1.2661</td><td>Average Holding Day:</td><td>14.4259</td><td>Annual Turnover Rate:</td><td>0.0%</td></tr></table>

# Limitations of the Basic M/M/1 Queuing Model

- Assumes a simple FIFO processing, while actual order execution may depend on priority rules or other factors. Eg. a trader can randomly place a limit order at any price level.   
- Market makers may keep placing and cancelling limit orders which makes the analysis more complicated.   
It ignores the size of order. A larger size is expected to take longer time to get filled (i.e. longer service time)   
- Assumes constant arrival and service rates, while these rates can fluctuate with market volatility and liquidity

# Optional Reading

- The order book as a queueing system: average depth and influence of the size of limit orders (https://hal.science/hal-01006410/document)   
- Simulating and analyzing order book data: The queue-reactive model (https://arxiv.org/pdf/1312.0563)   
- A Model for Queue Position Valuation in a Limit Order Book (https://moallemi.com/ciamac/papers/queue-value-2016.pdf)

# High Frequency Trading

![](images/2f8765a874883a6028c0203618704b9e572e4275ac66e87d0f42a4ccdc31546d.jpg)

# High Frequency Trading

- Rapid executions of multiple transactions followed by extremely short holding periods.   
Profit from small price changes and discrepancies   
- Require sophisticated technology, communications and computing resources   
- High frequency trading depends heavily on execution speed and latency

# Co-location

- High frequency trading firms usually rent space from market centers, such as from the major exchanges   
- The purpose to put their servers next to the market center's data servers is to reduce latency. It provides significant millisecond and even microseconds advantages, and enable institutions to better serve their clients.

![](images/d121d9591e8f65b4dc14955890ec69bf275a0fa4fdb1fc978dea4b9ae4716b99.jpg)

# Co-location

- Light or an electronic signal, travels approximately 300 meters in just 1 microsecond. Therefore,   
- from the NYSE to a financial institution in Hong Kong, which is around 13,000 kilometers away, it would take about 43.3 milliseconds   
- This calculation ignores the additional latency induced by routers, switches, etc.   
- An exchange provides institutions that co-locate in its facilities with cables of identical length to connect its data feeds, regardless of how many meters an office is from the feeds.

# Latency Arbitrage

- Latency refers to the speed at which quotations are displayed or orders are executed after being placed   
- Consider this example involving a high-frequency trader (HFT) that uses automated trading to leverage its informational edge:

- The initial market bid and ask price for Stock X are 50.00 and 50.01.   
- Due to its advanced technology and strategic location, HFT receives an institutional market order for Stock X. This information reaches the trader's system 10 microseconds before it does for most competing traders.   
- HFT purchases all available shares at 50.01 before other traders can react.   
- As a result, the institutional buyer is unable to buy at 50.01.   
- HFT then place a sell limit order at 50.02, and thus market ask price adjusts to 50.02.   
- The institutional buyer's market order finally execute at 50.02, locking a 0.01 profit per share for HFT

# Latency Arbitrage

- Another example highlighted in the Wall Street Journal.

- In July 2009, Intel, announced strong earnings, prompting some traders to view the semiconductor company Broadcom as a potential buying opportunity.   
- To mask their interest, many of these traders divided their orders into smaller quantities.   
- On that day, Broadcom opened trading at \(26.20. High-frequency traders (HFTs), who had quicker access to market quotes, noticed the significant interest in Broadcom.   
- Consequently, the HFTs started purchasing Broadcom shares and subsequently reselling them to slower investors at elevated prices. The share price of Broadcom swiftly climbed to $26.39 as HFTs began offering hundreds of thousands of shares for sale.   
- Slower traders ended up spending $1.4 million for around 56,000 shares, incurring an additional cost of $7,800 compared to if they had been able to act as quickly as the high-frequency traders.

# Common High Frequency Strategy

# 1. Market Making

Providing liquidity by placing buy and sell orders simultaneously to profit from the bid-ask spread   
- Some exchanges also provide rebates to market makers

# 2. Latency Arbitrage

- Exploiting delays in market data transmission to execute trades before other market participants can react   
- Co-location for rapid data feed and order execution is crucial

# 3. Cross Market Arbitrage

Identifying and exploiting price discrepancies between different asset classes, markets or exchanges. (eg. 0700.HK vs TCEHY)

# 4. Order Flow Analysis

- Monitoring the flow of buy and sell orders to predict short-term market direction.

# HFT: Is it worth?

It is an arms race to build   
- more reliable and faster execution platforms (computer engineering and science)   
- more comprehensive and accurate prediction models (mathematics, physics)   
- Reduce the latency (information engineering)   
- It is a money burning game to buy or rent   
- Faster data feed and lower latency   
- Bigger storage space and more powerful system   
- But, after you spend so much money, can you make the profit?

![](images/22afc88597c080b5099857d2607e01973f757b921c0f8aee5f692812aea0df3e.jpg)

投資組合

新闻

外幣

港股

美股

金價

油價

行情

財經日曆

地產

理财

專欄

財經大人物

媒

# 高頻交易|傳內地打壓高頻交易要求將客戶伺服器遷出交易所數據中心

BossMind

2026年1月16日

![](images/cd03ad0a6946a6b1a7d79c758cd77f489887f705a5f2bd78ae42a66e46d8f9f2.jpg)

彭博引述知情人士報道，中國監管機構要求將客戶伺服器遷出交易所運營的數據中心，料將削高頻交易機構的優勢。據報上海和廣州的大宗商品期貨交易所，已依照監管機構指示，要求本地經紀公司將客戶伺服器遷出交易所營運的數據中心。

知情人士透露，這一舉措由監管機構主導，上海期貨交易所已告知經紀商，他們需要在下個月底前將高頻客戶的設備移出，而其他客戶則需要在4月30日前完成。

這一措施影響的不僅僅是高頻交易機構，但它們受到的衝擊可能最大。除了內地本土的高頻交易公司，城堡證券（Citadel Securities）、Jane Street Group和Jump Trading等外國公司的伺服器接入亦將受到影響。

# Arbitrage

# What is arbitrage?

- Arbitrage is the simultaneous purchase and sale of related assets in different markets to profit from price discrepancies.   
- It usually requires efficient algorithms and high performing computers for execution.   
- Helps in maintaining market efficiency by aligning prices across different markets.

# Type of Arbitrage

1. Spatial Arbitrage

- Involves buying and selling the same asset in different locations.

2. Temporal Arbitrage   
- Involves buying and selling the same asset at different times.   
3. Statistical Arbitrage   
- Relies on mathematical models to identify price inefficiencies.

# Spatial Arbitrage: Big Mac Example

# Suppose

Big Mac in Hong Kong costs 30 HKD   
Big Mac in Shenzhen costs 26 RMB   
Exchange rate: 1 HKD = 0.9 RMB

# Actions:

- Buy a Big Mac in Shenzhen   
- Sell it in Hong Kong   
- Net Profit = $30^{*}0.9 - 26 = 1$ RMB

HKD 1 <> RMB 0.9

![](images/82b6b2de2ddd4c4b96bccc43a51984adfb3e3e80d467cca43dd75d4916d690f5.jpg)

HKD 30.0

![](images/0af2617b8eec4e251c40fe1be9f77e53820433a31dbf7364538e5bdca76b6feb.jpg)

![](images/c604b973daf8eb334f35304fa4b1bf6dc8985b1a1a9a32f68405a36d7d3ab397.jpg)

RMB 26.0

![](images/d566fe4ced7fc76e3aecb90503a37258c7d26a4daa4a09409330395c36b7092e.jpg)

# Spatial Arbitrage: Cross Exchanges Example

• Suppose you have US\(100,000 in both Binance and Bybit account.   
- Let's say you observe the following bitcoin future prices:

<table><tr><td>Exchange</td><td>Bid Price</td><td>Ask Price</td></tr><tr><td>Binance</td><td>96,531.5</td><td>96,532.0</td></tr><tr><td>Bybit</td><td>96,529.0</td><td>96,531.0</td></tr></table>

- Actions:

• Buy 1 contract of bitcoin future on Bybit at $96,531   
- Sell 1 contract of bitcoin future on Binance at \(96,531.5   
- Net Profit = $0.5

# Temporal Arbitrage Example

- Suppose 2 companies A (acquiring company) and B (target company) are going to merge to company C.   
- After merge, A and B will be removed from stock exchange while C will be listed.   
- Before the merge is done

- the shares value of the targeted company B will typically be lower than that of the combined company C   
- the share value of the acquiring company A will be slightly higher than that of the combined company C

- An arbitrager can buy shares from the targeted company and sells shares for the acquiring company at the same time. Upon the merger is closed, the discrepancy in shares value will be the profit.

# High Frequency Strategy 1: Triangular Arbitrage

# Triangular Arbitrage

- Triangular arbitrage is a trading strategy that exploits discrepancies in currency exchange rates.   
• Suppose you have US\(1000 and observe the current exchange rates in the market

EUR/USD = 0.8631   
• EUR/GBP = 1.4600   
- USD/GBP = 1.6939

Note:

- In market convention, the quotation XXX/YYY means the amount of XXX you can get in exchange for 1 unit of YYY

# Triangular Arbitrage

- The exchange rates do not match up. We can capture the arbitrage profit as follows:

- Sell dollars for euros: \(1000 \times 0.8631 = €863.1   
- Sell euros for pounds: €863.1/1.4600 = £591.1644   
• Sell pounds for dollars: £591.1644 x 1.6939 = $1001.373   
• Net profit = $1001.373 -$ 1000 = $1.373

![](images/a166eecd1907ba0bd6bc72e0d2f7afcbfde2e76a52af95f4e85db7f54c393e74.jpg)

# Polygonal Arbitrage

- The idea of Triangular Arbitrage can be extended to a so-called “Polygonal Arbitrage” among $N$ currencies

![](images/6ee4aa20312816bae388754c98fbb91e3f640474173e0284e532a69f75952876.jpg)

# Polygonal Arbitrage - Example

- Suppose we obtain the following snapshot at a particular time during a trading day.

<table><tr><td>Symbol</td><td>Bid</td><td>Ask</td></tr><tr><td>CAD/AUD</td><td>0.89068</td><td>0.89071</td></tr><tr><td>CNY/AUD</td><td>4.67769</td><td>4.67784</td></tr><tr><td>EUR/AUD</td><td>0.61055</td><td>0.61065</td></tr><tr><td>GBP/AUD</td><td>0.5178</td><td>0.51791</td></tr><tr><td>JPY/AUD</td><td>73.5981</td><td>73.6017</td></tr><tr><td>NZD/AUD</td><td>1.04152</td><td>1.04163</td></tr><tr><td>USD/AUD</td><td>0.66956</td><td>0.66964</td></tr><tr><td>CNY/CAD</td><td>5.24968</td><td>5.25003</td></tr><tr><td>EUR/CAD</td><td>0.68523</td><td>0.6853</td></tr><tr><td>GBP/CAD</td><td>0.58144</td><td>0.58161</td></tr><tr><td>JPY/CAD</td><td>82.5996</td><td>82.6034</td></tr><tr><td>NZD/CAD</td><td>1.16878</td><td>1.16898</td></tr><tr><td>CAD/USD</td><td>1.33033</td><td>1.33037</td></tr><tr><td>EUR/CNY</td><td>0.13051</td><td>0.13055</td></tr></table>

<table><tr><td>Symbol</td><td>Bid</td><td>Ask</td></tr><tr><td>GBP/CNY</td><td>0.11074</td><td>0.11075</td></tr><tr><td>JPY/CNY</td><td>15.7357</td><td>15.7364</td></tr><tr><td>NZD/CNY</td><td>0.22269</td><td>0.22277</td></tr><tr><td>CNY/USD</td><td>6.98578</td><td>6.98589</td></tr><tr><td>GBP/EUR</td><td>0.84851</td><td>0.84853</td></tr><tr><td>JPY/EUR</td><td>120.558</td><td>120.562</td></tr><tr><td>NZD/EUR</td><td>1.70653</td><td>1.70667</td></tr><tr><td>USD/EUR</td><td>1.09665</td><td>1.09667</td></tr><tr><td>JPY/GBP</td><td>142.092</td><td>142.113</td></tr><tr><td>NZD/GBP</td><td>2.01151</td><td>2.01182</td></tr><tr><td>USD/GBP</td><td>1.29277</td><td>1.2929</td></tr><tr><td>JPY/NZD</td><td>64.7773</td><td>64.7779</td></tr><tr><td>JPY/USD</td><td>109.924</td><td>109.931</td></tr><tr><td>USD/NZD</td><td>0.64288</td><td>0.64298</td></tr></table>

# Polygonal Arbitrage - Example

- We need to take into account of the bid-ask direction.   
- For a quotation XXX/YYY,

- Use ask price for calculation if we convert from XXX to YYY   
- Use bid price for calculation if we convert from YYY to XXX

- Eg. JPY/USD,

- If you sell 100 JPY for USD (i.e. buy USD), you will convert using ask and get $100 / 109.931 = 0.91$ USD.   
If you sell 100 USD for JPY (i.e. buy JPY), you will convert using bid and get 10,992.4 JPY

<table><tr><td>Symbol</td><td>Bid</td><td>Ask</td></tr><tr><td>JPY/USD</td><td>109.924</td><td>109.931</td></tr></table>

# Polygonal Arbitrage - Example

We are interested in the following questions

1. Is there arbitrage opportunity?   
2. What is the optimal path that can yield the highest return after a circulation?

- The most straightforward method is by using high school mathematics!!!

1. Loop for any $3,4,5,\ldots$ number of currencies (i.e. combination)   
2. For each combination, loop for different currency order (i.e. permutation)   
3. Multiply the exchange rate along the currency path   
4. Arbitrage exists if the value is greater than 1

# Polygonal Arbitrage - Example

from itertools import combinations, permutations   
print('currencies=','currencies')   
```python
def calculate_profit(rate_dict, currencies): initial_amount = 1 # Starting with 1 unit of the first currency current_amount = initial_amount action_path = [] 
```

```javascript
for i in range(len(currencies)): from_currency = currencies[i] to_currency = currencies[(i + 1) % len(currencies)] # Wrap around 
```

```txt
if (to_currency, from_currency) in rate_dict:  
    bid_price = rate_dict[(to_currency, from_currency)][0]  
    current_amount *= bid_price  
    action_path.append(f"Buy {current_amount:.5f} {to_currency} from {initial_amount:.4f} {from_currency} at bid price {bid_price:.5f}")  
    initial_amount = current_amount # Update for the next trade 
```

return current_amount, action_path   
```python
if (from_currency, to_currency) in rate_dict: ask_price = rate_dict[(from_currency, to_currency)][1] current_amount /= ask_price action_path.append(f"Sell {initial_amount:.5f} {from_currency} for {current_amount} {to_currency} at ask price {ask_price:.5f}") initial_amount = current_amount # Update for the next trade 
```

```python
def findpolygonal_arbitrage(exchangeRates): # Create a dictionary to store the exchange rates  
rate_dict = {}  
for pair, bid, ask in exchangeRates:  
    cur1, cur2 = pair.split('/')  
    rate_dict[(cur1, cur2)] = (bid, ask) 
```

```txt
Iterate through combinations of currencies (from 3 up to the number of currencies available)  
currencies = setPAIR.split('/')[0] for pair in [pair[0] for pair in exchangeRates]) 
```

```python
for r in range(3, len(currencies) + 1):  
    for currency Combination in combinations(currencies, r):  
        for currency_permutation in permutations(currency Combination):  
            value, action_path = calculate_profit(rate_dict, currency_permutation)  
            if value > 1: # More than the initial investment indicates arbitrage return True, action_path, value  
return False, [], 0 # No arbitrage opportunity found 
```

# Polygonal Arbitrage - Example

currencies= ('JPY', 'CAD', 'NZD')  
currencies= ('JPY', 'NZD', 'CAD')  
Arbitrage OpportunityExists:  
Sell 1.00000 JPY for 0.0154373636687821 NZD at ask price 64.77790  
Sell 0.01544 NZD for 0.013205840706241425 CAD at ask price 1.16898  
Buy 1.09080 JPY from 0.0132 CAD at bid price 82.59960  
Total value after circulation: 1.0908

exchangeRates $=$ [

[′CAD/AUD, 0.89068, 0.89071],

[CNY/AUD', 4.67769, 4.67784],

[EUR/AUD', 0.61055, 0.61065],

[GBP/AUD', 0.5178, 0.51791],

['JPY/AUD', 73.5981, 73.6017],

[NZD/AUD', 1.04152, 1.04163]

[USD/AUD', 0.66956, 0.66964].

[CNY/CAD', 5.24968, 5.25003]

[EUR/CAD', 0.68523, 0.6853],

[GBP/CAD', 0.58144, 0.58161]

['JPY/CAD', 82.5996, 82.6034],

[NZD/CAD', 1.16878, 1.16898]

[CAD/USD', 1.33033, 1.33037],

[EUR/CNY, 0.13051, 0.13055]

[GBP/CNY', 0.11074, 0.11075],

['JPY/CNY', 15.7357, 15.7364],

[NZD/CNY, 0.22269, 0.22277]

[CNY/USD', 6.98578, 6.98589]

[GBP/EUR', 0.84851, 0.84853]

['JPY/EUR', 120.558, 120.562],

[NZD/EUR', 1.70653, 1.70667]

[USD/EUR', 1.09665, 1.09667]

['JPY/GBP', 142.092, 142.113],

[NZD/GBP', 2.01151, 2.01182].

[USD/GBP', 1.29277, 1.2929],

['JPY/NZD', 64.7773, 64.7779],

['JPY/USD', 109.924, 109.931],

[USD/NZD', 0.64288, 0.6429]

]

has_arbitrage, action_path, value = findpolygonal_arbitrage(exchangeRates)

If has_arbitrage:

print("Arbitrage Opportunity EXISTS:")

for action in action_path:

print(action)

print(f"Total value after circulation: {value:.4f}")

else:

print("No Arbitrage Opportunity.")

# Computational Complexity

# - Combinations

• The number of combinations of $n$ currencies taken $r$ at a time is given by $C(n, r)$ which is $\frac{n!}{r!(n - r)!}$   
- This operates in $O(n^{r})$ for each combination size $r$

# Permutations

- The number of permutations of $r$ currencies is $r!$   
- For each combination, we need to consider all permutations, which adds an additional factor of $r!$

# - Total Complexity

- For each combination of currencies, we compute the profit for all permutations. Therefore, the total computational complexity can be approximated as: $O\left(\sum_{r=3}^{n} C(n, r) \cdot r!\right)$   
- This can be simplified to $O(n! \cdot 2^{n})$ in the worst case because you are iterating through all combinations and permutations of the currencies.

# Important Notes

# To successfully implement the polygonal arbitrage strategy,

Take into account of the bid-ask prices   
- Factor in transaction cost in the calculation   
- Beware of the rounding issue   
- Beware of market slippage   
- Lower chance of arbitrage if the bid-ask spread is large   
- Need a fast-computing algorithm to solve for a path with

- The highest return (global maximum), or   
- The first identified arbitrage

# Can you execute in real market?

# 1. Cash exchange physically

- Compare prices across different shops   
- Need manually input price in your program for calculation   
- Slow processing and settlement due to human operation

NO

![](images/6b3e5d4867fc03ede997e63d71eb1be8f1b853b0bce257c9eef75451730ddbae.jpg)

# Can you execute in real market?

2. Currency exchanges via banks

- Bid-ask spread is too large   
- How can you to get the real-time exchange rates and trade programmatically on the banking system?

NO

![](images/56941231d9fd8218016cbfa98ef91419ca717e81755738634de6f4ce70136bd1.jpg)

HSBC

Banking

Accounts & Services

Borrowing

Cards & Loans

Investing

Securities & Currency Exchange

Insurance

Protection & Planning

Insights

Analysis & Market Data

Offers

Latest Rewards

Exchange rates of foreign currencies against HK Dollar

<table><tr><td>Currency</td><td>Telegraphic Transfer Bank Buy</td><td>Telegraphic Transfer Bank Sell</td><td>Banknotes Bank Buy</td><td>Banknotes Bank Sell</td><td>Last Updated</td></tr><tr><td>US Dollar</td><td>7.74820</td><td>7.80980</td><td>7.71630</td><td>7.84170</td><td>as at 18 Feb 2025 13:28 HKT</td></tr><tr><td>Australian Dollar</td><td>4.90450</td><td>4.97840</td><td>4.87720</td><td>5.00560</td><td>as at 18 Feb 2025 13:28 HKT</td></tr><tr><td>Canadian Dollar</td><td>5.43200</td><td>5.51860</td><td>5.40080</td><td>5.54980</td><td>as at 18 Feb 2025 13:28 HKT</td></tr><tr><td>Euro</td><td>8.06410</td><td>8.20180</td><td>8.02470</td><td>8.24120</td><td>as at 18 Feb 2025 13:28 HKT</td></tr><tr><td>Japanese Yen</td><td>0.05076</td><td>0.05154</td><td>0.05041</td><td>0.05189</td><td>as at 18 Feb 2025 13:28 HKT</td></tr><tr><td>New Zealand Dollar</td><td>4.39730</td><td>4.47870</td><td>4.37250</td><td>4.50400</td><td>as at 18 Feb 2025 13:28 HKT</td></tr><tr><td>Pound Sterling</td><td>9.71900</td><td>9.87600</td><td>9.61600</td><td>9.97800</td><td>as at 18 Feb 2025 13:28 HKT</td></tr><tr><td>Renminbi</td><td>1.06170</td><td>1.07590</td><td>1.05570</td><td>1.08190</td><td>as at 18 Feb 2025 13:28 HKT</td></tr><tr><td>Singapore Dollar</td><td>5.74340</td><td>5.83060</td><td>5.70300</td><td>5.87100</td><td>as at 18 Feb 2025 13:28 HKT</td></tr><tr><td>Swiss Franc</td><td>8.55460</td><td>8.67190</td><td>8.50010</td><td>8.72650</td><td>as at 18 Feb 2025 13:28 HKT</td></tr><tr><td>Thai Baht</td><td>0.22660</td><td>0.23420</td><td>0.21690</td><td>0.24390</td><td>as at 18 Feb 2025 13:28 HKT</td></tr></table>

# Can you execute in real market?

3. FX brokers (eg. via MetaTrader)

It is CFD trading, not currency exchange!   
- But it might still work…

MAY BE

Order

![](images/baac06ae2e59844ea7ec5ad9be414633ac2483cec9cd86c87ff08c1d1a624716.jpg)

Symbol:

EURUSD, Euro vs US Dollar

Type:

Instant Execution

Volume:

![](images/b0e49d52f4b9cf92f7b40065baec2f422a81af45c3b2f8996e6c1a6d0af1c841.jpg)

1000EUR

Stop Loss:

0.0000

Take Profit:

0.0000

Comment:

Deviation:

0

1.04644 / 1.04651

Sell

Buy

# Triangular Arbitrage for CFD market

- Recall that triangular arbitrage exists if

$$
\frac {E U R}{U S D} \frac {U S D}{J P Y} \frac {J P Y}{E U R} > 1
$$

It can be re-written as

$$
\log \left(\frac {U S D}{E U R}\right) = \log \left(\frac {U S D}{J P Y}\right) + \log \left(\frac {J P Y}{E U R}\right) + \varepsilon , \mathrm {w h e r e} \varepsilon \mathrm {i s a n e r r o r t e r m v e r y c l o s e t o z e r o}
$$

- We can fit a multiple linear regression (without intercept).

$$
Y _ {t} = \beta_ {1} X _ {1, t} + \beta_ {2} X _ {2, t} + \varepsilon_ {t}
$$

- The estimated value of $\beta_{1}$ and $\beta_{2}$ will be very close to 1.   
- It can be traded as a mean reversion strategy based on the price discrepancies.

# Can you execute in real market?

We need a broker that supports FX Spot trading or currency exchange (eg. Interactive Brokers, Futu, etc)   
IB charges at least 2 USD commission per transaction   
- For a circulation involving 3 currencies, total cost = 6 USD.   
- Suppose our program detects $0.01\%$ arbitrage profit among 3 currencies, the breakeven capital will be $\frac{6}{0.01\%} = 60,000 \mathrm{USD}$   
• If we require the cost ratio to be $10\%$ , we need capital at least $\frac{60,000}{10\%} = 600,000$ USD

# MAY BE

![](images/c055a74ceb44bbce6ea0f55890443fef9b6f2ddf8d4404e6b5170063cbdd8308.jpg)

# InteractiveBrokers

Monthly Trade Value (USD) $^2$

≤1,000,000,000   
1,000,000,000.01 - 2,000,000,000   
2,000,000,000.01 - 5,000,000,000   
>5,000,000,000

Minimum per order

Tiered

0.20 basis point $^3$ * Trade Value $^4$

0.15 basis point $^{3*}$ Trade Value

0.10 basis point $^{3*}$ Trade Value

0.08 basis point $^{3}$ * Trade Value $^{4}$

Tier 1 - USD 2.00

Tier II - USD 1.50

Tier III - USD 1.25

Tier IV - USD 1.00

# Conclusion

- FX market is quite efficient nowadays.   
- Triangular/Polygonal arbitrage doesn’t occur very often.   
- Arbitrage profit, if any, its return won't be too high.   
- However, it requires   
- a large capital to cover the potential costs   
- an efficient algorithm (software) and fast computer (hardware) to execute the trades   
It is not wise to put so much idle money to wait for small profits   
- Thus, not many market players still use this strategy

# High Frequency Strategy 2: Market Making

# Liquidity Providing

- Some high frequency traders can make profits from order flow.   
- Exchanges usually offer rebates (eg. $0.0025 per share) to certain brokers (so called “market maker”) that place limit orders in the market.   
- The rebate is a payment to encourage brokers to provide liquidity in the market.   
• A broker places a buy limit order that filled at $10, and quickly places another sell limit order that filled again at$ 10, can still make a $0.005 profit per share from the liquidity rebate.   
- Other buy side traders may pay higher fee to exchange, because it is regarded as the cost of using liquidity in the market.

# Market Making Strategy

Market making strategy generates profits by capturing the bid-ask spread   
- The simplest execution of a market making strategy is to place a pair of bid and offer trades, and have the 2 trades filled over time.   
- When the market is not in trending, this algorithm is reasonably effective to earn the price differential.

![](images/b391ae76b398f3f9cf9615131ffbb7df73677384fee1d4ee31fa41ad53417558.jpg)

# Market Making Strategy

- However, this strategy fails if the market does not stay in a trading range, because one of the orders doesn't get fill and wait for a long time to capture the spread.   
- A logical solution to this is, if a pair of order doesn't get fill on both side, we would want to reset the algorithm and submit a new set of pairs:

- Constantly check if we have unfilled orders in the order book.   
- If no, we can simply submit a new set of pairs.   
- Otherwise, we should cancel the previous unfilled order, and submit another pair of limit orders.   
- Try to adjust the bid-ask level or quantity so as to balance the inventory.

![](images/2b646e4711fd4c66edcbf235750849925e8d80f922340e1071e15f20dc870495.jpg)

# Market Making Strategy Backtest

- Backtest Period = 2024-01 to 2024-03   
- Data Interval = 1 hour   
- Initial Capital = 100,000 USD   
- Leverage $= 1$   
Transaction Cost = 0   
- Allow Short Sell = True   
- Position Netting = True   
- Instruments = XAUUSD

from AlgoAPI import AlgoAPIUtil, AlgoAPI_Backtest from datetime import datetime, timedelta

class AlgoEvent: def __init__(self): self.lasttradetime = datetime(1970,1,1)

def start(self, mEvt): self EVT = AlgoAPI_Backtest.AlgoEvtHandler(self, mEvt) self EVT.start()

def on_marketdatafeed(self, md, ab): if md.timestamp $> =$ self.lasttradetime + timedelta(hours=1): self.lasttradetime $\equiv$ md.timestamp self.mm_order(md, 1, 'open') self.mm_order(md, -1, 'open')

def mm_order(self, md, buysell, openclose): order = AlgoAPIUtil.OrderObject( instrument = md.instrument, orderRef = 1, volume = 0.01, openclose = openclose, buysell = buysell, ordertype = 1, #0=market_order, 1=limit_order, #2=stop_order timeinforce = 60*60*24 #cancel unfilled order every day, unit in second ) if buysell $= = 1$ : order.price = md.bidPrice\*0.999 elif buysell $= = -1$ .. order.price = md.askPrice\*1.001 self.evt.sendOrder(order)

# Market Making Strategy Backtest

![](images/48c40301b73425c5a363f250cb680e85e96d4fa2cf0b2cae990352d36c795d3c.jpg)  
Equity Drawdown (%):

![](images/d53a0cf10846e8b1090c93a42afc91f70c508295113e015b3ef5894a193af18b.jpg)

Monthly Return:   

<table><tr><td>Year</td><td>Jan</td><td>Feb</td><td>Mar</td><td>Apr</td><td>May</td><td>Jun</td><td>Jul</td><td>Aug</td><td>Sep</td><td>Oct</td><td>Nov</td><td>Dec</td><td>YTD</td></tr><tr><td>2024</td><td>0.68%</td><td>0.94%</td><td>-1.6%</td><td>-</td><td>-</td><td>-</td><td>-</td><td>-</td><td>-</td><td>-</td><td>-</td><td>-</td><td>-0.0%</td></tr></table>

# Conclusion

- Even rebates can provide stable income, a market maker still need to manage the risks of taking unwanted positions.   
- A key point to implement a market making strategy successfully is to maintain the net position as close to zero as possible over time, while maximizing the number of filled pairs.   
- Some improvements to above strategy:

- Only open a set of pair trades when the market is detected to be ranging   
- Early cut loss if the market is detected to be trending in opposite direction   
- Set a wider spread if the market is less liquid; and tighter spread for liquid market   
- Set a wider spread when market is volatile; and tighter spread for less volatile market

# Key Takeaways

- Reveals order book insights through real-time market depth and liquidity dynamics.   
- Common trading ideas/strategies through order flow analysis   
- Market impact of a large order and strategies to mitigate   
- The basics of Queuing theory for modelling order execution dynamics and waiting times in an order book   
- High frequency trading focuses on rapid execution to capture small price movements.   
- Most arbitrage strategies require high frequency trading to capture price discrepancies   
- Understand the principles for some popular high frequency strategies

- Triangular Arbitrage Strategy   
Market Making Strategy
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
