请审查以下候选题库中的每个 family，判断它是否可以进入正式练习题库。

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

当前 step：
step1

<CURRENT_STEP>
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
}

</CURRENT_STEP>

<CANDIDATE_QUESTION_BANK>
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
</CANDIDATE_QUESTION_BANK>

判定时只看每个 family 是否训练当前 step 的可学习内容或能力。不要因为它 JSON 合法、措辞流畅或来自输入材料就放行。

请直接输出 JSON，不要加解释。
