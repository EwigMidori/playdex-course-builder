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
step4

<CURRENT_STEP>
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
}

</CURRENT_STEP>

<CANDIDATE_QUESTION_BANK>
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
</CANDIDATE_QUESTION_BANK>

判定时只看每个 family 是否训练当前 step 的可学习内容或能力。不要因为它 JSON 合法、措辞流畅或来自输入材料就放行。

请直接输出 JSON，不要加解释。
