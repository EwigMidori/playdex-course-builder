# Playdex Course Builder Research

这是一个课程内容生成系统的研究型仓库。目标不是单纯把 PDF 转成文本，而是把一份课程讲义逐步加工成一套可学习、可复习、可测验的结构化课程资产。

当前系统围绕一个 lesson 级流水线展开：
- `pipeline/0-raw/`：原始 PDF
- `pipeline/1-plain/`：通过 MinerU 等工具抽取得到的 plain text 与原始 artifacts
- `pipeline/2-related_important/`：课程背景、教学意图、补充上下文
- `pipeline/3-guided_story/`：面向 learn mode 的 step-by-step 引导式内容
- `pipeline/4-question_bank/`：题库，包含 flashcards 与 longform families
- `pipeline/5-textbook/`：lesson 级 MDX 课本与必要图片资产

项目的核心思路是：先把源材料变成结构化中间表示，再通过 prompt、校验器和 reviewer 回路，逐步生成更高层的教学内容，而不是一次性让模型直接“写完整课程”。

## 当前状态

仓库里已经有一条可验证的样例链路：
- `L1.pdf` 已跑通过一轮 plain extraction
- `L1` 的 guided story、question bank、textbook 已有样例产物
- prompt 已按“课程无关、输入驱动、可 reviewer 审查”的方向做过一轮重构
- 进度规划文件位于 `docs/progress.json`

目前这些能力主要由 Python 脚本和 prompt 组合驱动。下一阶段计划是用 Rust 重建全链路执行系统，同时保留现有产物 schema、review 机制和可审计性。

## 这个仓库解决什么问题

这个项目面向的是“把讲义变成真正能学的课程资产”这一问题，而不是单纯的 OCR 或内容摘要。具体来说，它试图同时生成三类面向学生的内容：
- 引导式学习内容：适合移动端逐屏推进的 guided story
- 题库：能和 step、lesson 关联，并支持多变体
- 课本：用于预习和复习的 lesson 级系统化教材

为了避免模型产物看起来像课程、实际上却漏掉关键考点，仓库里特别强调几件事：
- source alignment
- coverage completeness
- 结构化 schema
- cross-reference validation
- reviewer fail/pass gate

## 关键文件

- [scripts/mineru_raw_convert.py](/home/midori/playdex-course-builder/research/scripts/mineru_raw_convert.py:1)
  - 用原始 MinerU API 做一次 PDF 转换，输出到 `pipeline/1-plain/`
- [scripts/guided_story_play.py](/home/midori/playdex-course-builder/research/scripts/guided_story_play.py:1)
  - 在终端里 headless 播放一个 guided story step，便于人工 review
- [scripts/guided_story_tx.py](/home/midori/playdex-course-builder/research/scripts/guided_story_tx.py:1)
  - 给 AI 用的事务型 CLI 编辑器，用来安全修改 guided story JSON
- [scripts/check_progress_schema.py](/home/midori/playdex-course-builder/research/scripts/check_progress_schema.py:1)
  - 校验 `docs/progress.json` 的执行协议和字段结构
- [scripts/view_progress.py](/home/midori/playdex-course-builder/research/scripts/view_progress.py:1)
  - 本地查看 `docs/progress.json` 的可视化表格界面

## Prompt 设计原则

`prompts/` 下的 prompt 不是为某一门具体课程写死的，而是要求由输入包驱动。也就是说：
- 不要耦合某个学科、某个 lesson 或某组固定 slide bucket
- 关键 coverage 应由 `coverage checklist`、`source outline`、`lesson map` 输入驱动
- 输出必须能被 reviewer 审查，不允许只靠“看起来像对的”

这套原则已经应用在：
- guided story generation / review
- question bank generation / review
- textbook generation / review

## 接下来怎么发展

当前计划不是继续堆更多临时脚本，而是逐步把系统迁到 Rust，形成真正的全链路执行引擎。迁移方向大致是：
- 用 Rust 实现统一 CLI 与 stage orchestration
- 把 prompt 渲染、模型调用、schema 校验、review gate 变成正式模块
- 支持 stage 级别的 resume / retry / audit trail
- 以 `L2.pdf` 作为持续集成样本推进端到端开发

详细开发计划见：
- [docs/progress.json](/Users/zijingzhang/ewig/playdex-course-builder/docs/progress.json:1)

## 设计取向

这是一个敏捷迭代型项目，不是“先冻结所有需求再一次性实现”的瀑布流系统。原因很简单：只有真正把 lesson 跑通，才会暴露出新的题型需求、MDX 组件需求、review 规则需求和资产处理需求。

因此，这个仓库更看重：
- 可运行主线
- 可恢复的 stage
- 清晰的中间产物
- 稳定的 schema
- 可持续演化的 prompt 与 validator

而不是过早把所有未来能力编码死。
