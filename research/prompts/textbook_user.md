请基于以下材料，生成一份 lesson 级 MDX 课本。

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
- 面向整节课，而不是单个 step
- 系统化组织内容，适合预习与复习
- 要能嵌入题目与题族
- 必须区分 `flashcard_families`、`quiz_families`、`longform_families` 的用途：闪卡用于主动检索，quiz 用于测验，longform 用于解释与应用
- 不要把选择题称为 flashcard，也不要把 flashcard 放在考试题语境中
- 术语与公式要可交互引用
- 不要输出行内 `$...$` 或块级 `$$...$$` 数学语法；行内公式统一写成 `<InlineFormula latex="..." />`，块级公式统一写成 `<Formula latex="...">简短说明</Formula>`
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

<QUESTION_BANK>
{{question_bank}}
</QUESTION_BANK>

<PLAIN_TEXT>
{{plain_text}}
</PLAIN_TEXT>

<RELATED_IMPORTANT>
{{related_important}}
</RELATED_IMPORTANT>

请直接输出 MDX，不要加解释。
