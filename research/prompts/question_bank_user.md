请基于以下 lesson 材料，生成一个结构化题库 JSON。

目标语言：
{{target_language}}

course_id：
{{course_id}}

course_title：
{{course_title}}

chapter_id：
{{chapter_id}}

chapter_title：
{{chapter_title}}

lesson_id：
{{lesson_id}}

要求：
- 同时生成 `flashcard_families`、`quiz_families` 和 `longform_families`
- 题目必须只关联到当前 step：`{{current_step_id}}`
- 所有 family 和 variant 的 `linked_steps` 都必须等于 `["{{current_step_id}}"]`
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
{{coverage_checklist}}
</COVERAGE_CHECKLIST>

<SOURCE_OUTLINE>
{{source_outline}}
</SOURCE_OUTLINE>

<LESSON_MAP>
{{lesson_map}}
</LESSON_MAP>

<GUIDED_STORY_MANIFEST>
{{guided_story_manifest}}
</GUIDED_STORY_MANIFEST>

<GUIDED_STORY_STEPS>
{{guided_story_steps}}
</GUIDED_STORY_STEPS>

<CURRENT_STEP_ID>
{{current_step_id}}
</CURRENT_STEP_ID>

<CURRENT_STEP>
{{current_step}}
</CURRENT_STEP>

<PLAIN_TEXT>
{{plain_text}}
</PLAIN_TEXT>

<RELATED_IMPORTANT>
{{related_important}}
</RELATED_IMPORTANT>

请直接输出 JSON，不要加解释。
