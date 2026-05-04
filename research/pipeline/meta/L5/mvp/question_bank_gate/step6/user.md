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
step6

<CURRENT_STEP>
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

</CURRENT_STEP>

<CANDIDATE_QUESTION_BANK>
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
</CANDIDATE_QUESTION_BANK>

判定时只看每个 family 是否训练当前 step 的可学习内容或能力。不要因为它 JSON 合法、措辞流畅或来自输入材料就放行。

请直接输出 JSON，不要加解释。
