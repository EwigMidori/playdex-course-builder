请把下面的材料重写成一个 lesson 级 `guided_story` JSON。

目标：
- 面向手机端逐屏点击学习
- 先把 lecture 拆成多个自然 step，再为每个 step 写逐屏内容
- 不要像课件提纲
- 不要像老师讲稿
- 要像一段有推进感的字幕流

目标语言：
zh-CN

本次 step 的主题边界：
full lecture; split into 4-8 natural learning steps unless the source is very short

输出风格：
- 每个 screen 默认 1 到 2 行
- 一节复杂 lecture 通常生成 4 到 8 个 steps
- 每个 step 通常 6 到 14 个 screens，避免 25+ screens 的巨型 step
- 每个 step 只覆盖一个自然学习单元
- 顶层必须优先输出 `{ "lesson_id": ..., "mode": "guided_story", "steps": [...] }`
- 每个 `steps[]` item 必须包含 `sequence_id`、`concept`、`step`
- `steps[].step` 必须符合 guided_story step schema，并包含自己的 `screens` 与 `term_catalog`
- 每行尽量短
- 每 3 到 6 个 screen 插入一次小练习或认知拐点
- 新概念首次出现必须加粗
- 新术语必须使用 `<term id="...">主显示文本</term>` 内嵌标记
- 第二语言术语、英文原词、别名不要直接混写在正文里，放进 `term_catalog.aliases`
- 如果有关键公式，先讲直觉，再给公式
- 如果有关键图，才使用 `media`
- 正文不要写标题
- 正文和练习都不要提“原材料”“材料里”“本节”“这一步”“上一段”等元叙述

额外约束：
- 去掉广告、无关宣传、课程管理信息、教师信息
- 遇到 OCR 噪声时做最小修复
- 不要一次性引入太多新词
- 学生看完这个 step 后，应该能带走一句 punchline
- 输出必须像成品内容，而不是像“基于某份材料整理”的内容
- 填空题答案不要混放多种语言版本
- 不要生成“纯英文术语 + 下一句松散意译”这种不完整教学表达
- 输出时请生成顶层 `term_catalog`
- 如果输出 lesson 级对象，`term_catalog` 放在每个 `steps[].step` 内
- 每个首次出现的术语都要：
  - 在 `lines` 中用 `<term>` 标记
  - 在 `term_catalog` 中登记 `display`、`aliases`、`gloss`

自检清单：
- 学生只看这一份 JSON，是否能自然理解？
- 有没有任何句子泄露“这段话来自源材料”？
- 有没有“根据材料”“原文提到”“这一页”之类的幕后措辞？
- 有没有课程管理信息或备课口吻？
- 术语写法是否统一？
- 填空题答案是否只有必要的、同语言体系内的正确答案？
- 是否所有术语都通过 `<term>` + `term_catalog` 表达，而不是靠自由文本混排？

源材料：

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

<PLAIN_TEXT>
# Financial Analytics and

# Algorithmic Trading

Course Code: COMP7415

# Background of Lecturer

- Founder of AlgoGene (https://algogene.com)   
Vice Chairman of Algo Challenge Association (https://algochallenge.org)   
- Former algo developer, quant trader, risk manager, data analyst for hedge funds and banks.   
- BSc in Math, Risk Management, Actuarial Science (HKU); MSc in Computer Science (HKU)   
- Champion and awardee of several algo trading competitions, including

CCTV证券资讯频道《宽客天下·全球量化争霸赛》(2017/18)  
- Rotman International Trading Competition (2017)   
CASH Algo Trading Contest (2016)   
WorldQuant Challenge (2014, 2015)

# Let’s play a game!

![](images/997b92b1751c1ce70437186759220c5d106c40ed0862fb47cf1033ee018d9aa5.jpg)

# 1. What’s your study mode?

a) Full time   
b) Part time   
c) Just sit in

# 2. What is your main reason for taking this course?

a) I want to start a career in algo-trading   
b) I want to enhance my current job skills   
c) I am interested in the topic and want to learn more   
d) Just to fulfill graduation requirement

# 3. What do you expect to learn from this course?

a) Some skills for financial data analysis   
b) I want to develop an automated trading system   
c) Tell me some profitable trading strategies   
d) Not sure yet

# 4. Do you have any prior experience in algo-trading?

a) Yes, I have extensive experience   
b) Yes, I have some experience   
c) No, but I have experience in other forms of trading   
d) No, I am completely new to trading

# 5. Have you ever used any algo-trading platforms before?

a) Yes, I have used MetaTrader   
b) Yes, I have used TradingView   
c) Yes, I have used AlgoGene   
d) No, I have no idea what it is

# 6. Which of the following has the largest market capitalization?

a) China stock market   
b) Hong Kong stock market   
c) India stock market   
d) Japan stock market

# Answer

As of Dec-2025,

- China: $15 trillion   
- Hong Kong: $6 trillion   
- India: $5.3 trillion   
- Japan: $7.6 trillion

# 7. Which of the following market has the largest daily turnover/trading volume?

a) Commodity market   
b) Crypto market   
c) Global Equity market   
d) Forex

# Answer

Global equity market: $3.93 trillion   
• Forex: $6.6 trillion   
- Commodity: $126 billion   
- Crypto: $139 billion

# Reference:

- https://www.world-exchanges.org/our-work/statistics   
- https://www.bis.org/statistics/rpfx19 fx.htm   
- https://www.cftc.gov/MarketReports/CommitmentsofTraders/index.htm   
- https://www.coingecko.com/en/global charts

# 8. Which programming language is most widely used in algo-trading?

a) C++   
b) Java   
c) Python   
d) R

# 9. Which of the following market is the most popular for algo traders?

a) Commodities   
b) Cryptocurrency   
c) Equity   
d) Forex

# 10. What's the name of your teachers?

a) Tommy Lai   
b) Tony Lam   
c) Tim Leung   
d) Timothy Lau

# Course Objectives & Learning Outcomes

- Understand the trading process in financial market   
- Understand the fundamental of algorithmic trading framework for building a trading strategy   
- Able to formulate trading strategies, carry out backtesting, optimization, risk management and interpret investment performance   
- Develop practical skills in financial data analysis, trading strategy development, and deployment to real market and broker account

# Topics to be covered

1. Algorithmic trading basics and financial markets   
2. Data scraping and database management with Python   
3. Building backtest framework and rule-based trading strategy   
4. Statistical time series analysis for market classification   
5. Statistical arbitrage and pairs trading   
6. Capital and Risk Management   
7. Performance measures and portfolio optimization   
8. Order book and high frequency data modeling   
9. Market practice in strategy optimization, broker selection, and market tricks   
10. Machine learning use cases in algorithmic trading

# Assessment methods

- Group project - 50%   
• Written final exam covers all taught content in the course – 50%

# Contact

Lecturer

- Tony Lam: tonylamfm@hku.hk / tonylam@algogene.com

TA:

- Rex Tsang: trex@hku.hk
</PLAIN_TEXT>

可用图片：
<IMAGE_HINTS>

</IMAGE_HINTS>

请直接输出 JSON，不要加解释。
