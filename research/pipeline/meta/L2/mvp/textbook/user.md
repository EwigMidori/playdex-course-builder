请基于以下材料，生成一份 lesson 级 MDX 课本。

目标语言：
zh-CN

lesson_id：
L2

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
# L2: Data scraping and database management with Python
Course Code: COMP7415
# Agenda
- Understand the basic website structure
- Web scraping using different python libraries
- The limitations of web scraping
- Create a simple database using python
- Understand the concept of adjusted price
- Database design for financial market data storage
# Installation of Python
- Python: https://www.python.org/downloads/
- Anaconda (suggested): https://www.anaconda.com/download/success
- easier management for different environments
- pre-installed data analysis packages (eg. numpy, pandas, scipy, etc)
# What’s Data/Web Scraping?
- Automated process to extract data from websites
- Common use cases in algo-trading:
Real-time data collection (eg. economic indicators, stock prices, etc)
- Market sentiment analysis (eg. news, forum and blogs, social media)
- Corporate analysis (eg. product announcements, annual reports, director changes, etc)
Alternative data (eg. weather, web traffic, etc)
# Basic Web structure
# A website is usually made of
1. HTML (HyperText Markup Language)
2. CSS (Cascading Style Sheets)
3. JavaScript
# HTML
- Purpose: Defines the structure and content of a webpage.
- Elements: Uses tags to create elements such as headings, paragraphs, links, images, tables, etc.
• Markup Language: Not a programming language; it’s a markup language used to structure content.
# Common HTML Tags
<table><tr><td>Tag</td><td>Description</td></tr><tr><td></td><td>Defines the root of an HTML document.</td></tr><tr><td></td><td>Contains meta-information about the HTML document, such as title, styles, and scripts.</td></tr><tr><td></td><td>Sets the title of the HTML document, displayed in the browser&#x27;s title bar or tab.</td></tr><tr><td></td><td>Contains the content of the HTML document, including text, images, links, and other elements.</td></tr><tr><td></td><td>Defines headings of different sizes, with &lt;h1&gt; being the largest and &lt;h6&gt; the smallest.</td></tr><tr><td></td><td>Defines a paragraph of text.</td></tr><tr><td></td><td>Creates a hyperlink, linking to another webpage, file, or location within the same page.</td></tr><tr><td></td><td>Inserts an image into the HTML document.</td></tr><tr><td></td><td>Defines a division or container for grouping HTML elements and applying CSS styles.</td></tr><tr><td></td><td>Defines an inline container for styling a specific portion of text or content.</td></tr><tr><td></td><td>Creates an unordered or ordered list, respectively, containing list items (&lt;li&gt;).</td></tr><tr><td></td><td>Defines a list item within an unordered or ordered list.</td></tr><tr><td></td><td>Creates a table for organizing data into rows and columns.</td></tr><tr><td></td><td>Defines a table row within a &lt;table&gt; element.</td></tr><tr><td></td><td>Defines a table cell within a &lt;tr&gt; element.</td></tr><tr><td></td><td>Defines a header cell within a &lt;tr&gt; element in a table.</td></tr><tr><td></td><td>Creates an HTML form for collecting user input.</td></tr><tr><td></td><td>Defines an input control element within a form, such as text fields, checkboxes, or buttons.</td></tr><tr><td></td><td>Associates a label with an input element, improving accessibility and usability.</td></tr><tr><td></td><td>Defines a clickable button within a form or webpage.</td></tr><tr><td></td><td>Creates a multiline text input control within a form.</td></tr><tr><td></td><td>Embeds another HTML page or external content within the current document.</td></tr><tr><td></td><td>Embeds or links to client-side scripts, such as JavaScript.</td></tr><tr><td></td><td>Contains CSS rules for styling HTML elements within the document.</td></tr></table>
# HTML Example
```html
1 <!--DOCTYPE html>
2 <html>
3 <head>
4 <meta charset="utf-8">
5 <meta name="viewport" content="width=device-width, initial-scale=1">
6 <title>Page Title</title>
7 </head>
8 <body>
9 <h1>Heading</h1>
10 <p>Paragraph</p>
11 </body>
12 </html>
13
```
![](images/9a0356d3c69aa54d62765a0ef2abff6400803e8e1063976ac685e8e80ce3ccd6.jpg)
# CSS
- Purpose: Controls the visual presentation and layout of a webpage
- Styling: Defines styles for HTML elements, such as colors, fonts, spacing, and positioning.
- Cascading Rules: Multiple styles can be applied to an element, with rules for which styles take precedence.
<!DOCTYPE html>
<html>
<head> <meta charset="utf-8"> <meta name $=$ "viewport" content $=$ "width $\equiv$ device-width, initial-scale $= 1$ titlePage Title</title> <style type $=$ "text/css"> body { font-family: Arial,sans-serif; margin:0; } h1{ color:blue; text-align:center; } p{ font-size:14px; line-height:1.5; } </style>
</head>
<body> <h1>Heading</h1> <p>Paragraph</p>
</body>
</html>
</COVERAGE_CHECKLIST>

<SOURCE_OUTLINE>
# L2: Data scraping and database management with Python
Course Code: COMP7415
# Agenda
- Understand the basic website structure
- Web scraping using different python libraries
- The limitations of web scraping
- Create a simple database using python
- Understand the concept of adjusted price
- Database design for financial market data storage
# Installation of Python
- Python: https://www.python.org/downloads/
- Anaconda (suggested): https://www.anaconda.com/download/success
- easier management for different environments
- pre-installed data analysis packages (eg. numpy, pandas, scipy, etc)
# What’s Data/Web Scraping?
- Automated process to extract data from websites
- Common use cases in algo-trading:
Real-time data collection (eg. economic indicators, stock prices, etc)
- Market sentiment analysis (eg. news, forum and blogs, social media)
- Corporate analysis (eg. product announcements, annual reports, director changes, etc)
Alternative data (eg. weather, web traffic, etc)
# Basic Web structure
# A website is usually made of
1. HTML (HyperText Markup Language)
2. CSS (Cascading Style Sheets)
3. JavaScript
# HTML
- Purpose: Defines the structure and content of a webpage.
- Elements: Uses tags to create elements such as headings, paragraphs, links, images, tables, etc.
• Markup Language: Not a programming language; it’s a markup language used to structure content.
# Common HTML Tags
<table><tr><td>Tag</td><td>Description</td></tr><tr><td></td><td>Defines the root of an HTML document.</td></tr><tr><td></td><td>Contains meta-information about the HTML document, such as title, styles, and scripts.</td></tr><tr><td></td><td>Sets the title of the HTML document, displayed in the browser&#x27;s title bar or tab.</td></tr><tr><td></td><td>Contains the content of the HTML document, including text, images, links, and other elements.</td></tr><tr><td></td><td>Defines headings of different sizes, with &lt;h1&gt; being the largest and &lt;h6&gt; the smallest.</td></tr><tr><td></td><td>Defines a paragraph of text.</td></tr><tr><td></td><td>Creates a hyperlink, linking to another webpage, file, or location within the same page.</td></tr><tr><td></td><td>Inserts an image into the HTML document.</td></tr><tr><td></td><td>Defines a division or container for grouping HTML elements and applying CSS styles.</td></tr><tr><td></td><td>Defines an inline container for styling a specific portion of text or content.</td></tr><tr><td></td><td>Creates an unordered or ordered list, respectively, containing list items (&lt;li&gt;).</td></tr><tr><td></td><td>Defines a list item within an unordered or ordered list.</td></tr><tr><td></td><td>Creates a table for organizing data into rows and columns.</td></tr><tr><td></td><td>Defines a table row within a &lt;table&gt; element.</td></tr><tr><td></td><td>Defines a table cell within a &lt;tr&gt; element.</td></tr><tr><td></td><td>Defines a header cell within a &lt;tr&gt; element in a table.</td></tr><tr><td></td><td>Creates an HTML form for collecting user input.</td></tr><tr><td></td><td>Defines an input control element within a form, such as text fields, checkboxes, or buttons.</td></tr><tr><td></td><td>Associates a label with an input element, improving accessibility and usability.</td></tr><tr><td></td><td>Defines a clickable button within a form or webpage.</td></tr><tr><td></td><td>Creates a multiline text input control within a form.</td></tr><tr><td></td><td>Embeds another HTML page or external content within the current document.</td></tr><tr><td></td><td>Embeds or links to client-side scripts, such as JavaScript.</td></tr><tr><td></td><td>Contains CSS rules for styling HTML elements within the document.</td></tr></table>
# HTML Example
```html
1 <!--DOCTYPE html>
2 <html>
3 <head>
4 <meta charset="utf-8">
5 <meta name="viewport" content="width=device-width, initial-scale=1">
6 <title>Page Title</title>
7 </head>
8 <body>
9 <h1>Heading</h1>
10 <p>Paragraph</p>
11 </body>
12 </html>
13
```
![](images/9a0356d3c69aa54d62765a0ef2abff6400803e8e1063976ac685e8e80ce3ccd6.jpg)
# CSS
- Purpose: Controls the visual presentation and layout of a webpage
- Styling: Defines styles for HTML elements, such as colors, fonts, spacing, and positioning.
- Cascading Rules: Multiple styles can be applied to an element, with rules for which styles take precedence.
<!DOCTYPE html>
<html>
<head> <meta charset="utf-8"> <meta name $=$ "viewport" content $=$ "width $\equiv$ device-width, initial-scale $= 1$ titlePage Title</title> <style type $=$ "text/css"> body { font-family: Arial,sans-serif; margin:0; } h1{ color:blue; text-align:center; } p{ font-size:14px; line-height:1.5; } </style>
</head>
<body> <h1>Heading</h1> <p>Paragraph</p>
</body>
</html>
</SOURCE_OUTLINE>

<LESSON_MAP>
{
  "lesson_id": "L2",
  "mode": "guided_story",
  "steps": [
    {
      "concept": "MVP lesson slice",
      "file": "research/pipeline/3-guided_story/L2/step1/step.json",
      "sequence_id": "step1"
    }
  ]
}

</LESSON_MAP>

<GUIDED_STORY_MANIFEST>
{
  "lesson_id": "L2",
  "mode": "guided_story",
  "steps": [
    {
      "concept": "MVP lesson slice",
      "file": "research/pipeline/3-guided_story/L2/step1/step.json",
      "sequence_id": "step1"
    }
  ]
}

</GUIDED_STORY_MANIFEST>

<GUIDED_STORY_STEPS>
{
  "lesson_id": "L2",
  "mode": "guided_story",
  "screens": [
    {
      "focus_terms": [],
      "id": "s001",
      "introduced_terms": [],
      "lines": [
        "想象一下，你需要每天手动收集几百只股票的价格。",
        "那会花掉你多少时间？"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "web_scraping"
      ],
      "id": "s002",
      "introduced_terms": [
        "web_scraping"
      ],
      "lines": [
        "其实，有一种方法可以让电脑替你完成这些重复劳动。",
        "它叫 <term id=\"web_scraping\">网络爬取</term>。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "web_scraping"
      ],
      "id": "s003",
      "introduced_terms": [],
      "lines": [
        "简单来说，就是写一段程序，自动访问网页，",
        "然后把你需要的数据提取出来。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "web_scraping"
      ],
      "id": "s004",
      "introduced_terms": [],
      "lines": [
        "在算法交易中，爬取的数据可以是实时股价、新闻标题、",
        "甚至社交媒体上的情绪。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s005",
      "introduced_terms": [],
      "lines": [
        "但网页不是一堆纯文本。",
        "它有自己的骨架。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "html"
      ],
      "id": "s006",
      "introduced_terms": [
        "html"
      ],
      "lines": [
        "这个骨架就是 <term id=\"html\">HTML</term>。",
        "它用标签来定义标题、段落、表格……"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "html"
      ],
      "id": "s007",
      "introduced_terms": [],
      "lines": [
        "比如 `<h1>` 表示一级标题，`<p>` 表示段落。",
        "爬虫的工作，就是找到这些标签，取出里面的内容。"
      ],
      "type": "narration"
    },
    {
      "exercise": {
        "answer": 1,
        "explanation": "`<table>` 标签用于在 HTML 中创建表格。",
        "kind": "single_choice",
        "options": [
          "<p>",
          "<table>",
          "<h1>",
          "<a>"
        ],
        "prompt": "下面哪个标签用于定义网页中的表格？"
      },
      "focus_terms": [
        "html"
      ],
      "id": "s008",
      "introduced_terms": [],
      "lines": [
        "下面哪个标签用于定义网页中的表格？"
      ],
      "type": "exercise"
    },
    {
      "focus_terms": [
        "beautifulsoup"
      ],
      "id": "s009",
      "introduced_terms": [
        "beautifulsoup"
      ],
      "lines": [
        "Python 里最常用的爬虫库之一，叫 <term id=\"beautifulsoup\">BeautifulSoup</term>。",
        "它能帮你轻松解析 HTML 文档。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "beautifulsoup"
      ],
      "id": "s010",
      "introduced_terms": [],
      "lines": [
        "先用 `requests` 下载网页内容，",
        "再用 BeautifulSoup 解析，就能像操作对象一样提取数据。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "beautifulsoup"
      ],
      "id": "s011",
      "introduced_terms": [],
      "lines": [
        "比如，用 `find('p')` 可以找到第一个段落，",
        "用 `find_all('a')` 可以找到所有超链接。"
      ],
      "type": "narration"
    },
    {
      "exercise": {
        "answer": 1,
        "explanation": "`find_all('table')` 会返回页面中所有 `<table>` 标签的列表。",
        "kind": "single_choice",
        "options": [
          "find('table')",
          "find_all('table')",
          "get_text()",
          "prettify()"
        ],
        "prompt": "假设你想提取网页中所有表格的数据。应该使用哪个方法？"
      },
      "focus_terms": [
        "beautifulsoup"
      ],
      "id": "s012",
      "introduced_terms": [],
      "lines": [
        "假设你想提取网页中所有表格的数据。",
        "应该使用哪个方法？"
      ],
      "type": "exercise"
    },
    {
      "focus_terms": [
        "xpath"
      ],
      "id": "s013",
      "introduced_terms": [
        "xpath"
      ],
      "lines": [
        "除了 BeautifulSoup，还有另一种定位方式：<term id=\"xpath\">XPath</term>。",
        "它像一条路径，直接指向你想要的元素。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "xpath"
      ],
      "id": "s014",
      "introduced_terms": [],
      "lines": [
        "比如 `//table[@id='constituents']` 就能选中 id 为 constituents 的表格。",
        "配合 `lxml` 库，解析速度更快。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s015",
      "introduced_terms": [],
      "lines": [
        "但有些网站的内容是动态加载的，",
        "直接下载 HTML 可能拿不到数据。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "selenium"
      ],
      "id": "s016",
      "introduced_terms": [
        "selenium"
      ],
      "lines": [
        "这时候就需要 <term id=\"selenium\">Selenium</term> 了。",
        "它能模拟真实用户打开浏览器、点击按钮、滚动页面。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "selenium"
      ],
      "id": "s017",
      "introduced_terms": [],
      "lines": [
        "Selenium 就像一个机器人，替你操作网页，",
        "等数据加载完成后，再交给 BeautifulSoup 去提取。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "yfinance"
      ],
      "id": "s018",
      "introduced_terms": [
        "yfinance"
      ],
      "lines": [
        "不过，对于金融数据，还有一个更省事的办法。",
        "直接用 <term id=\"yfinance\">yfinance</term> 库。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "yfinance"
      ],
      "id": "s019",
      "introduced_terms": [],
      "lines": [
        "它是 Yahoo Finance 的非官方 Python 接口。",
        "一行代码就能下载股票的历史数据。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "yfinance"
      ],
      "id": "s020",
      "introduced_terms": [],
      "lines": [
        "比如 `yf.download('AAPL', start='2023-01-01')`",
        "就能拿到苹果公司从 2023 年开始的所有日线数据。"
      ],
      "type": "narration"
    },
    {
      "exercise": {
        "answer": 1,
        "explanation": "调整收盘价通过回溯调整历史价格，消除了公司事件造成的价格跳跃，使长期分析更准确。",
        "kind": "single_choice",
        "options": [
          "反映股票的真实市场交易价格",
          "消除分红、拆股等事件对价格序列的影响",
          "让股价看起来更高",
          "用于计算当日的成交量"
        ],
        "prompt": "调整收盘价的主要目的是什么？"
      },
      "focus_terms": [
        "adjusted_close"
      ],
      "id": "s021",
      "introduced_terms": [
        "adjusted_close"
      ],
      "lines": [
        "yfinance 下载的数据中，有一列叫 'Adj Close'。",
        "你知道它和 'Close' 有什么区别吗？"
      ],
      "type": "exercise"
    },
    {
      "focus_terms": [],
      "id": "s022",
      "introduced_terms": [],
      "lines": [
        "爬取到的数据不能每次都重新抓。",
        "你需要一个地方把它们存起来。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "sqlite"
      ],
      "id": "s023",
      "introduced_terms": [
        "sqlite"
      ],
      "lines": [
        "Python 内置的 <term id=\"sqlite\">SQLite</term> 就是轻量级数据库的好选择。",
        "不需要安装额外软件，一个文件就是一个数据库。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "sqlite"
      ],
      "id": "s024",
      "introduced_terms": [],
      "lines": [
        "你可以用 SQL 语句创建表、插入数据、查询历史。",
        "比如 `SELECT * FROM prices WHERE symbol='AAPL'`。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s025",
      "introduced_terms": [],
      "lines": [
        "设计数据库时，要考虑一个问题：",
        "是把所有股票的数据放在一张大表里，还是按股票或日期拆分？"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s026",
      "introduced_terms": [],
      "lines": [
        "没有完美的方案。",
        "按股票拆分，回测时查询快；按日期拆分，跨资产分析方便。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s027",
      "introduced_terms": [],
      "lines": [
        "关键在于平衡存储空间、查询速度和内存占用。",
        "根据你的使用场景，选择最合适的方案。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s028",
      "introduced_terms": [],
      "lines": [
        "现在你知道了：",
        "用爬虫获取数据，用数据库管理数据。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s029",
      "introduced_terms": [],
      "lines": [
        "下一步，就是把这些数据变成交易策略。",
        "但那是另一个故事了。"
      ],
      "type": "narration"
    }
  ],
  "sequence_id": "step1",
  "source": {
    "plain_text": "L2: Data scraping and database management with Python. 包含 Web scraping 基础、BeautifulSoup、lxml、Selenium、yfinance、SQLite 数据库管理、调整收盘价概念、数据库设计等。",
    "related": [
      "COMP7415A"
    ]
  },
  "target_language": "zh-CN",
  "term_catalog": {
    "adjusted_close": {
      "aliases": [
        "Adjusted Close",
        "Adj Close"
      ],
      "display": "调整收盘价",
      "gloss": "经过分红、拆股等公司事件调整后的收盘价，用于反映股票的真实历史表现。"
    },
    "beautifulsoup": {
      "aliases": [
        "Beautiful Soup"
      ],
      "display": "BeautifulSoup",
      "gloss": "一个用于解析 HTML 和 XML 文档的 Python 库。"
    },
    "html": {
      "aliases": [
        "HyperText Markup Language"
      ],
      "display": "HTML",
      "gloss": "超文本标记语言，用于定义网页的结构和内容。"
    },
    "selenium": {
      "aliases": [],
      "display": "Selenium",
      "gloss": "一个用于模拟用户交互、执行自动化测试和网络爬取的 Web 自动化工具。"
    },
    "sqlite": {
      "aliases": [],
      "display": "SQLite",
      "gloss": "一个轻量级的、基于磁盘的关系型数据库引擎，Python 内置支持。"
    },
    "web_scraping": {
      "aliases": [
        "Web Scraping",
        "Data Scraping"
      ],
      "display": "网络爬取",
      "gloss": "从网站自动提取数据的过程。"
    },
    "xpath": {
      "aliases": [
        "XML Path Language"
      ],
      "display": "XPath",
      "gloss": "一种在 XML 文档中导航和选择节点的查询语言。"
    },
    "yfinance": {
      "aliases": [],
      "display": "yfinance",
      "gloss": "一个非官方的 Python 库，用于从 Yahoo Finance 下载金融数据。"
    }
  }
}

</GUIDED_STORY_STEPS>

<QUESTION_BANK>
[
{
  "coverage_map": [
    {
      "coverage_tag": "web_scraping_definition",
      "covered_by": [
        "qf_flash_web_scraping",
        "qf_quiz_web_scraping_use"
      ],
      "description": "网络爬取的定义与算法交易中的应用场景"
    },
    {
      "coverage_tag": "html_structure",
      "covered_by": [
        "qf_flash_html_tags",
        "qf_quiz_html_tags"
      ],
      "description": "HTML基本结构与常见标签"
    },
    {
      "coverage_tag": "beautifulsoup_usage",
      "covered_by": [
        "qf_flash_beautifulsoup",
        "qf_quiz_beautifulsoup",
        "qf_long_beautifulsoup"
      ],
      "description": "BeautifulSoup库的安装与基本用法（find, find_all, get_text, prettify）"
    },
    {
      "coverage_tag": "xpath_lxml",
      "covered_by": [
        "qf_flash_xpath",
        "qf_quiz_xpath"
      ],
      "description": "XPath语法与lxml库解析"
    },
    {
      "coverage_tag": "selenium_dynamic",
      "covered_by": [
        "qf_flash_selenium",
        "qf_quiz_selenium"
      ],
      "description": "Selenium处理动态加载页面"
    },
    {
      "coverage_tag": "yfinance_usage",
      "covered_by": [
        "qf_flash_yfinance",
        "qf_quiz_yfinance",
        "qf_long_yfinance"
      ],
      "description": "yfinance库下载金融数据"
    },
    {
      "coverage_tag": "adjusted_close_concept",
      "covered_by": [
        "qf_flash_adjusted_close",
        "qf_quiz_adjusted_close",
        "qf_long_adjusted_close"
      ],
      "description": "调整收盘价的概念与计算逻辑"
    },
    {
      "coverage_tag": "sqlite_basics",
      "covered_by": [
        "qf_flash_sqlite",
        "qf_quiz_sqlite",
        "qf_long_sqlite"
      ],
      "description": "SQLite数据库的创建、表操作与CRUD"
    },
    {
      "coverage_tag": "database_design_tradeoffs",
      "covered_by": [
        "qf_flash_db_design",
        "qf_quiz_db_design",
        "qf_long_db_design"
      ],
      "description": "金融数据库设计中的权衡（按股票/按日期分区）"
    },
    {
      "coverage_tag": "scraping_limitations",
      "covered_by": [
        "qf_flash_limitations",
        "qf_quiz_limitations"
      ],
      "description": "网络爬取的局限性（数据质量、资源消耗、反爬）"
    }
  ],
  "flashcard_families": [
    {
      "concept_key": "web_scraping_definition",
      "coverage_tags": [
        "web_scraping_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_web_scraping",
      "learning_goal": "学生能用自己的话定义网络爬取，并列举至少一个算法交易中的应用场景。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "网络爬取的定义与用途",
      "term_refs": [
        {
          "display": "网络爬取",
          "en": "web scraping"
        }
      ],
      "variants": [
        {
          "back": "从网站自动提取数据的过程。",
          "estimated_seconds": 8,
          "explanation": "网络爬取是用程序自动访问网页并提取所需数据的技术。",
          "front": "什么是网络爬取？",
          "question_id": "q_flash_web_scraping_v1"
        },
        {
          "back": "实时数据收集（如经济指标、股价）、市场情绪分析、公司分析、另类数据。",
          "estimated_seconds": 10,
          "explanation": "爬取的数据可用于实时行情、新闻情绪、财报、天气等。",
          "front": "在算法交易中，网络爬取可以用于哪一类数据收集？",
          "question_id": "q_flash_web_scraping_v2"
        }
      ]
    },
    {
      "concept_key": "html_structure",
      "coverage_tags": [
        "html_structure"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_html_tags",
      "learning_goal": "学生能识别常见HTML标签及其用途。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "HTML标签功能",
      "term_refs": [
        {
          "display": "HTML",
          "en": "HyperText Markup Language"
        }
      ],
      "variants": [
        {
          "back": "<h1>",
          "estimated_seconds": 5,
          "explanation": "<h1> 是最大的一级标题标签。",
          "front": "HTML中哪个标签用于定义一级标题？",
          "question_id": "q_flash_html_tags_v1"
        },
        {
          "back": "<a>",
          "estimated_seconds": 5,
          "explanation": "<a> 标签的 href 属性指定链接目标。",
          "front": "HTML中哪个标签用于创建超链接？",
          "question_id": "q_flash_html_tags_v2"
        },
        {
          "back": "<table>",
          "estimated_seconds": 5,
          "explanation": "<table> 标签包含行 <tr> 和单元格 <td>。",
          "front": "HTML中哪个标签用于定义表格？",
          "question_id": "q_flash_html_tags_v3"
        }
      ]
    },
    {
      "concept_key": "beautifulsoup_usage",
      "coverage_tags": [
        "beautifulsoup_usage"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_beautifulsoup",
      "learning_goal": "学生能说出BeautifulSoup的常用方法及其作用。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "BeautifulSoup方法",
      "term_refs": [
        {
          "display": "BeautifulSoup",
          "en": "BeautifulSoup"
        }
      ],
      "variants": [
        {
          "back": "find()",
          "estimated_seconds": 5,
          "explanation": "find('p') 返回第一个 <p> 标签。",
          "front": "BeautifulSoup中哪个方法用于提取第一个匹配的标签？",
          "question_id": "q_flash_beautifulsoup_v1"
        },
        {
          "back": "find_all()",
          "estimated_seconds": 5,
          "explanation": "find_all('a') 返回所有 <a> 标签的列表。",
          "front": "BeautifulSoup中哪个方法用于提取所有匹配的标签？",
          "question_id": "q_flash_beautifulsoup_v2"
        },
        {
          "back": "get_text()",
          "estimated_seconds": 5,
          "explanation": "get_text() 返回去掉HTML标签后的文本内容。",
          "front": "BeautifulSoup中哪个方法用于提取网页中的所有纯文本？",
          "question_id": "q_flash_beautifulsoup_v3"
        }
      ]
    },
    {
      "concept_key": "xpath_lxml",
      "coverage_tags": [
        "xpath_lxml"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_xpath",
      "learning_goal": "学生能理解XPath的基本语法与用途。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "XPath语法",
      "term_refs": [
        {
          "display": "XPath",
          "en": "XML Path Language"
        }
      ],
      "variants": [
        {
          "back": "从文档任意位置选择节点。",
          "estimated_seconds": 6,
          "explanation": "// 是相对路径，会搜索整个文档。",
          "front": "XPath中 '//' 表示什么？",
          "question_id": "q_flash_xpath_v1"
        },
        {
          "back": "//div[@attribute='value']",
          "estimated_seconds": 8,
          "explanation": "@ 用于筛选属性。",
          "front": "XPath中如何选择属性为 'value' 的 <div> 元素？",
          "question_id": "q_flash_xpath_v2"
        }
      ]
    },
    {
      "concept_key": "selenium_dynamic",
      "coverage_tags": [
        "selenium_dynamic"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_selenium",
      "learning_goal": "学生能说出Selenium的主要用途。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "Selenium用途",
      "term_refs": [
        {
          "display": "Selenium",
          "en": "Selenium"
        }
      ],
      "variants": [
        {
          "back": "动态加载内容的网页。",
          "estimated_seconds": 8,
          "explanation": "Selenium可以模拟浏览器操作，等待JavaScript渲染后再提取数据。",
          "front": "Selenium主要用于解决什么类型的网页爬取问题？",
          "question_id": "q_flash_selenium_v1"
        },
        {
          "back": "在后台运行浏览器，不显示界面。",
          "estimated_seconds": 6,
          "explanation": "headless 模式节省资源，适合服务器环境。",
          "front": "Selenium的 'headless' 模式有什么作用？",
          "question_id": "q_flash_selenium_v2"
        }
      ]
    },
    {
      "concept_key": "yfinance_usage",
      "coverage_tags": [
        "yfinance_usage"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_yfinance",
      "learning_goal": "学生能说出yfinance的基本用法。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "yfinance用法",
      "term_refs": [
        {
          "display": "yfinance",
          "en": "yfinance"
        }
      ],
      "variants": [
        {
          "back": "history(period) 或 yf.download()",
          "estimated_seconds": 8,
          "explanation": "例如 ticker.history(period='1mo') 或 yf.download('AAPL', start='2023-01-01')。",
          "front": "yfinance中哪个方法用于下载股票的历史数据？",
          "question_id": "q_flash_yfinance_v1"
        },
        {
          "back": "ticker.info",
          "estimated_seconds": 6,
          "explanation": "info 返回包含公司概况、财务数据等的字典。",
          "front": "yfinance中哪个方法用于获取股票的基本信息？",
          "question_id": "q_flash_yfinance_v2"
        }
      ]
    },
    {
      "concept_key": "adjusted_close_concept",
      "coverage_tags": [
        "adjusted_close_concept"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_adjusted_close",
      "learning_goal": "学生能解释调整收盘价的目的与基本调整因素。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "调整收盘价的目的",
      "term_refs": [
        {
          "display": "调整收盘价",
          "en": "Adjusted Close"
        }
      ],
      "variants": [
        {
          "back": "分红和拆股。",
          "estimated_seconds": 8,
          "explanation": "分红后股价通常下跌，拆股后股价按比例降低，调整收盘价回溯修正这些变化。",
          "front": "调整收盘价主要消除了哪些公司事件对价格序列的影响？",
          "question_id": "q_flash_adjusted_close_v1"
        },
        {
          "back": "0.5",
          "estimated_seconds": 6,
          "explanation": "2:1拆股意味着每股变成两股，价格减半，所以调整系数为0.5。",
          "front": "在2:1拆股后，调整收盘价对拆股前的价格乘以什么系数？",
          "question_id": "q_flash_adjusted_close_v2"
        }
      ]
    },
    {
      "concept_key": "sqlite_basics",
      "coverage_tags": [
        "sqlite_basics"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_sqlite",
      "learning_goal": "学生能说出SQLite的基本操作。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "SQLite基本操作",
      "term_refs": [
        {
          "display": "SQLite",
          "en": "SQLite"
        }
      ],
      "variants": [
        {
          "back": "sqlite3",
          "estimated_seconds": 5,
          "explanation": "sqlite3 是Python内置的轻量级数据库模块。",
          "front": "Python中连接SQLite数据库使用哪个模块？",
          "question_id": "q_flash_sqlite_v1"
        },
        {
          "back": "INSERT INTO table_name (columns) VALUES (values);",
          "estimated_seconds": 6,
          "explanation": "例如 INSERT INTO users (name, age) VALUES ('Alice', 30);",
          "front": "SQLite中用于插入数据的SQL语句是什么？",
          "question_id": "q_flash_sqlite_v2"
        },
        {
          "back": "SELECT columns FROM table_name WHERE condition;",
          "estimated_seconds": 6,
          "explanation": "例如 SELECT * FROM users WHERE age > 25;",
          "front": "SQLite中用于查询数据的SQL语句是什么？",
          "question_id": "q_flash_sqlite_v3"
        }
      ]
    },
    {
      "concept_key": "database_design_tradeoffs",
      "coverage_tags": [
        "database_design_tradeoffs"
      ],
      "difficulty": "medium",
      "family_id": "qf_flash_db_design",
      "learning_goal": "学生能说出金融数据库设计的两种常见分区策略及其权衡。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "数据库分区策略",
      "term_refs": [
        {
          "display": "数据库设计",
          "en": "Database Design"
        }
      ],
      "variants": [
        {
          "back": "回测时查询单只股票速度快。",
          "estimated_seconds": 8,
          "explanation": "按股票分区适合回测场景，因为回测通常只涉及少数股票。",
          "front": "按股票分区存储市场数据的主要优点是什么？",
          "question_id": "q_flash_db_design_v1"
        },
        {
          "back": "跨资产分析方便。",
          "estimated_seconds": 8,
          "explanation": "按日期分区适合需要同时分析多只股票同一时间段数据的场景。",
          "front": "按日期分区存储市场数据的主要优点是什么？",
          "question_id": "q_flash_db_design_v2"
        }
      ]
    },
    {
      "concept_key": "scraping_limitations",
      "coverage_tags": [
        "scraping_limitations"
      ],
      "difficulty": "easy",
      "family_id": "qf_flash_limitations",
      "learning_goal": "学生能列举网络爬取的主要局限性。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_answer",
      "retrieval_focus": "爬取局限性",
      "term_refs": [
        {
          "display": "网络爬取局限性",
          "en": "Web Scraping Limitations"
        }
      ],
      "variants": [
        {
          "back": "网站结构频繁变化或实施反爬措施（如IP封锁、速率限制）。",
          "estimated_seconds": 8,
          "explanation": "反爬措施会阻止爬虫正常访问。",
          "front": "网络爬取的一个技术挑战是什么？",
          "question_id": "q_flash_limitations_v1"
        },
        {
          "back": "数据可能不完整或不是最新的。",
          "estimated_seconds": 6,
          "explanation": "网站数据更新不及时或页面结构变化可能导致数据缺失。",
          "front": "网络爬取在数据质量方面可能遇到什么问题？",
          "question_id": "q_flash_limitations_v2"
        }
      ]
    }
  ],
  "lesson_id": "L2",
  "longform_families": [
    {
      "concept_key": "beautifulsoup_usage",
      "coverage_tags": [
        "beautifulsoup_usage"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_beautifulsoup",
      "learning_goal": "学生能描述使用BeautifulSoup从网页提取表格数据的基本步骤。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "BeautifulSoup",
          "en": "BeautifulSoup"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "导入库",
            "获取页面内容",
            "解析HTML",
            "定位表格",
            "提取数据"
          ],
          "question_id": "q_long_beautifulsoup_v1",
          "reference_answer": [
            "1. 导入 requests 和 bs4 库。",
            "2. 使用 requests.get(url) 获取网页内容。",
            "3. 用 BeautifulSoup(text, 'html.parser') 创建解析对象。",
            "4. 使用 soup.find_all('table') 找到所有表格。",
            "5. 遍历每个表格的行 (<tr>) 和单元格 (<td>)，提取文本。"
          ],
          "rubric_points": [
            "导入 requests 和 BeautifulSoup",
            "使用 requests.get() 下载页面内容",
            "用 BeautifulSoup(text, 'html.parser') 解析",
            "使用 find_all('table') 或 find('table') 定位表格",
            "遍历行和列提取数据"
          ],
          "stem": "请描述使用BeautifulSoup从HTML页面中提取所有表格数据的基本步骤。"
        },
        {
          "estimated_seconds": 60,
          "prompt_blocks": [
            "方法区别",
            "find() 场景",
            "find_all() 场景"
          ],
          "question_id": "q_long_beautifulsoup_v2",
          "reference_answer": [
            "find() 只返回第一个匹配的标签，适合提取页面中唯一的元素，比如标题。",
            "find_all() 返回所有匹配标签的列表，适合提取多个同类元素，比如所有超链接。"
          ],
          "rubric_points": [
            "find() 返回第一个匹配元素",
            "find_all() 返回所有匹配元素的列表",
            "find() 适合提取唯一元素（如页面标题）",
            "find_all() 适合提取多个同类元素（如所有链接）"
          ],
          "stem": "请解释 BeautifulSoup 中 find() 和 find_all() 的区别，并各举一个使用场景。"
        }
      ]
    },
    {
      "concept_key": "yfinance_usage",
      "coverage_tags": [
        "yfinance_usage"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_yfinance",
      "learning_goal": "学生能描述使用yfinance下载股票数据并存入SQLite数据库的流程。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "yfinance",
          "en": "yfinance"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "安装与导入",
            "下载数据",
            "连接数据库",
            "创建表",
            "插入数据"
          ],
          "question_id": "q_long_yfinance_v1",
          "reference_answer": [
            "1. 安装 yfinance 库，导入 yfinance 和 sqlite3。",
            "2. 使用 yf.download('AAPL', start='2023-01-01') 下载数据。",
            "3. 使用 sqlite3.connect('market.db') 连接数据库。",
            "4. 执行 CREATE TABLE 语句创建表，包含 symbol, date, open, high, low, close, volume 等字段。",
            "5. 遍历下载的数据，用 INSERT 语句逐行插入。",
            "6. 提交事务并关闭连接。"
          ],
          "rubric_points": [
            "安装 yfinance 和 sqlite3",
            "使用 yf.download() 或 Ticker.history() 下载数据",
            "使用 sqlite3.connect() 连接数据库",
            "使用 CREATE TABLE 创建表结构",
            "遍历数据行，使用 INSERT 语句插入"
          ],
          "stem": "请描述使用 yfinance 下载某只股票的历史数据，并将其存入 SQLite 数据库的基本步骤。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "易用性",
            "速度",
            "可靠性",
            "适用场景"
          ],
          "question_id": "q_long_yfinance_v2",
          "reference_answer": [
            "yfinance 优点：代码简洁、速度快、无需处理反爬。缺点：非官方库，可能因Yahoo接口变化而失效。",
            "Selenium 优点：可以处理动态加载，通用性强。缺点：速度慢、资源消耗大、需要维护WebDriver。",
            "对于标准金融数据，优先使用 yfinance；对于需要模拟用户操作的场景，使用 Selenium。"
          ],
          "rubric_points": [
            "yfinance 更简单，一行代码即可",
            "yfinance 速度更快，无需渲染页面",
            "yfinance 依赖非官方API，可能失效",
            "Selenium 更通用，但速度慢且资源消耗大"
          ],
          "stem": "比较使用 yfinance 直接下载数据与使用 Selenium 爬取 Yahoo Finance 页面的优缺点。"
        }
      ]
    },
    {
      "concept_key": "adjusted_close_concept",
      "coverage_tags": [
        "adjusted_close_concept"
      ],
      "difficulty": "hard",
      "family_id": "qf_long_adjusted_close",
      "learning_goal": "学生能解释调整收盘价的计算逻辑及其在回测中的重要性。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "mechanism_trace",
      "term_refs": [
        {
          "display": "调整收盘价",
          "en": "Adjusted Close"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "计算2月16日调整收盘价",
            "计算2月18日调整收盘价",
            "解释重要性"
          ],
          "question_id": "q_long_adjusted_close_v1",
          "reference_answer": [
            "2月16日调整收盘价 = 0.5 * 0.9968 * 46.99 = 23.42",
            "2月18日调整收盘价 = 0.9968 * 24.96 = 24.88",
            "调整收盘价通过回溯调整历史价格，消除了拆股和分红造成的价格跳跃，使价格序列连续，适合长期回测和绩效分析。"
          ],
          "rubric_points": [
            "2月16日调整收盘价 = 0.5 * (1 - 0.08/24.96) * 46.99 = 23.42",
            "2月18日调整收盘价 = (1 - 0.08/24.96) * 24.96 = 24.88",
            "调整收盘价消除了拆股和分红造成的价格跳跃",
            "使历史价格序列连续，适合长期回测"
          ],
          "stem": "某股票在2月18日发生2:1拆股，2月21日除息（每股分红$0.08）。已知2月16日收盘价为$46.99，2月17日收盘价为$48.30，2月18日拆股后收盘价为$24.96，2月19日收盘价为$24.91，2月20日收盘价为$24.95，2月21日除息后收盘价为$24.53，2月22日收盘价为$24.54。请计算2月16日和2月18日的调整收盘价，并解释调整收盘价在回测中的重要性。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "回测中使用调整收盘价的原因",
            "潜在缺点"
          ],
          "question_id": "q_long_adjusted_close_v2",
          "reference_answer": [
            "使用调整收盘价可以消除分红、拆股等事件造成的价格跳跃，使回测结果更准确地反映策略的真实表现。",
            "潜在缺点包括：调整收盘价不是实际市场交易价格，建模于调整价格可能与实际交易有偏差；需要定期更新调整序列，增加维护成本。"
          ],
          "rubric_points": [
            "调整收盘价消除了公司事件造成的价格跳跃",
            "使回测结果更准确地反映策略的真实表现",
            "缺点：调整收盘价不是实际市场交易价格",
            "缺点：需要定期更新调整序列",
            "缺点：建模于调整价格可能与实际交易有偏差"
          ],
          "stem": "请解释为什么在回测中应该使用调整收盘价而不是原始收盘价？使用调整收盘价有哪些潜在缺点？"
        }
      ]
    },
    {
      "concept_key": "sqlite_basics",
      "coverage_tags": [
        "sqlite_basics"
      ],
      "difficulty": "medium",
      "family_id": "qf_long_sqlite",
      "learning_goal": "学生能描述使用SQLite进行数据CRUD操作的基本流程。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "short_explain",
      "term_refs": [
        {
          "display": "SQLite",
          "en": "SQLite"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "创建表",
            "插入数据",
            "查询数据",
            "更新数据",
            "删除数据"
          ],
          "question_id": "q_long_sqlite_v1",
          "reference_answer": [
            "1. 使用 sqlite3.connect('example.db') 连接数据库。",
            "2. 创建游标 cursor = conn.cursor()。",
            "3. 执行 CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)。",
            "4. 使用 INSERT INTO users (name, age) VALUES ('Alice', 30) 插入数据。",
            "5. 使用 SELECT * FROM users WHERE age > 25 查询数据。",
            "6. 使用 UPDATE users SET age=31 WHERE name='Alice' 更新数据。",
            "7. 使用 DELETE FROM users WHERE name='Alice' 删除数据。",
            "8. 使用 conn.commit() 提交更改，最后关闭连接。"
          ],
          "rubric_points": [
            "使用 sqlite3.connect() 连接数据库",
            "使用 cursor.execute() 执行 CREATE TABLE",
            "使用 INSERT 插入数据",
            "使用 SELECT 查询数据",
            "使用 UPDATE 更新数据",
            "使用 DELETE 删除数据",
            "使用 conn.commit() 提交事务"
          ],
          "stem": "请描述使用Python的sqlite3模块创建一个用户表，并插入、查询、更新、删除数据的基本步骤。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "效率",
            "数据一致性",
            "查询能力"
          ],
          "question_id": "q_long_sqlite_v2",
          "reference_answer": [
            "使用数据库存储市场数据可以避免每次回测都重新爬取，节省时间和网络资源。",
            "数据库保证数据的一致性和完整性，避免因网站变化导致数据缺失。",
            "数据库支持复杂的SQL查询，可以灵活筛选时间范围、多股票联合分析，便于回测和策略开发。"
          ],
          "rubric_points": [
            "避免重复爬取，节省时间和带宽",
            "数据库保证数据完整性和一致性",
            "支持复杂查询（如时间范围、多股票联合查询）",
            "便于历史回测和数据分析"
          ],
          "stem": "请解释为什么在算法交易中应该使用数据库来存储市场数据，而不是每次都重新爬取？"
        }
      ]
    },
    {
      "concept_key": "database_design_tradeoffs",
      "coverage_tags": [
        "database_design_tradeoffs"
      ],
      "difficulty": "hard",
      "family_id": "qf_long_db_design",
      "learning_goal": "学生能比较按股票分区和按日期分区的优缺点，并根据场景选择合适的设计。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "compare_and_contrast",
      "term_refs": [
        {
          "display": "数据库设计",
          "en": "Database Design"
        }
      ],
      "variants": [
        {
          "estimated_seconds": 120,
          "prompt_blocks": [
            "按股票分区优缺点",
            "按日期分区优缺点",
            "场景选择"
          ],
          "question_id": "q_long_db_design_v1",
          "reference_answer": [
            "按股票分区：优点——回测单只股票时查询速度快，数据备份方便。缺点——跨资产分析需要跨多个表，实现复杂。",
            "按日期分区：优点——跨资产分析方便，可以快速获取所有股票在某一时间段的数据。缺点——回测单只股票需要扫描多个分区，速度慢。",
            "如果主要任务是回测少数股票，选择按股票分区；如果主要任务是全市场数据分析，选择按日期分区。"
          ],
          "rubric_points": [
            "按股票分区：回测单只股票快，但跨资产分析困难",
            "按日期分区：跨资产分析方便，但回测单只股票慢",
            "回测场景优先选择按股票分区",
            "数据分析场景优先选择按日期分区",
            "需要平衡存储、速度和内存"
          ],
          "stem": "假设你要设计一个存储全球股票市场历史数据的数据库。请比较按股票分区和按日期分区两种设计方案的优缺点，并说明在什么场景下你会选择哪种方案。"
        },
        {
          "estimated_seconds": 90,
          "prompt_blocks": [
            "存储空间",
            "查询速度",
            "内存占用",
            "使用场景"
          ],
          "question_id": "q_long_db_design_v2",
          "reference_answer": [
            "没有完美的设计方案，因为不同的使用场景对存储空间、查询速度和内存占用有不同的要求。",
            "例如，按股票分区节省了回测时的查询时间，但增加了跨资产分析的复杂度；按日期分区方便全市场分析，但回测单只股票时速度慢。",
            "设计时需要在存储空间、查询速度和内存占用之间找到平衡，根据实际使用场景选择最合适的方案。"
          ],
          "rubric_points": [
            "存储空间与查询速度的权衡",
            "内存占用与数据完整性的权衡",
            "不同使用场景需要不同的设计",
            "需要根据实际需求选择最优方案"
          ],
          "stem": "请解释为什么没有一种数据库设计方案是完美的，并说明在设计金融数据库时需要在哪些方面做出权衡。"
        }
      ]
    }
  ],
  "quiz_families": [
    {
      "concept_key": "web_scraping_definition",
      "coverage_tags": [
        "web_scraping_definition"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_web_scraping_use",
      "learning_goal": "学生能识别网络爬取在算法交易中的典型应用。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "网络爬取",
          "en": "web scraping"
        }
      ],
      "variants": [
        {
          "answer": 2,
          "estimated_seconds": 15,
          "explanation": "执行交易订单通常通过交易API完成，而不是通过网页爬取。",
          "options": [
            "实时数据收集（如经济指标）",
            "市场情绪分析（如新闻、社交媒体）",
            "执行交易订单",
            "另类数据收集（如天气、网页流量）"
          ],
          "question_id": "q_quiz_web_scraping_use_v1",
          "stem": "以下哪项不是网络爬取在算法交易中的常见用途？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "网络爬取擅长从静态HTML页面提取结构化数据。高频数据通常通过API获取。",
          "options": [
            "从Yahoo Finance下载历史股价",
            "从静态HTML页面提取公司财报数据",
            "实时监控高频交易订单簿",
            "执行复杂的统计套利策略"
          ],
          "question_id": "q_quiz_web_scraping_use_v2",
          "stem": "网络爬取最适合以下哪个场景？"
        }
      ]
    },
    {
      "concept_key": "html_structure",
      "coverage_tags": [
        "html_structure"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_html_tags",
      "learning_goal": "学生能区分常见HTML标签的用途。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "HTML",
          "en": "HyperText Markup Language"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 10,
          "explanation": "<p> 标签用于定义段落。",
          "options": [
            "<h1>",
            "<p>",
            "<a>",
            "<div>"
          ],
          "question_id": "q_quiz_html_tags_v1",
          "stem": "HTML中哪个标签用于定义段落？"
        },
        {
          "answer": 0,
          "estimated_seconds": 10,
          "explanation": "<img> 标签通过 src 属性指定图片路径。",
          "options": [
            "<img>",
            "<a>",
            "<div>",
            "<span>"
          ],
          "question_id": "q_quiz_html_tags_v2",
          "stem": "HTML中哪个标签用于嵌入图片？"
        }
      ]
    },
    {
      "concept_key": "beautifulsoup_usage",
      "coverage_tags": [
        "beautifulsoup_usage"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_beautifulsoup",
      "learning_goal": "学生能选择正确的BeautifulSoup方法完成特定提取任务。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "BeautifulSoup",
          "en": "BeautifulSoup"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "find_all('table') 会返回页面中所有 <table> 标签的列表。",
          "options": [
            "find('table')",
            "find_all('table')",
            "get_text()",
            "prettify()"
          ],
          "question_id": "q_quiz_beautifulsoup_v1",
          "stem": "假设你想提取网页中所有表格的数据，应该使用哪个方法？"
        },
        {
          "answer": 2,
          "estimated_seconds": 12,
          "explanation": "prettify() 会添加缩进和换行，使HTML结构更易读。",
          "options": [
            "get_text()",
            "find()",
            "prettify()",
            "find_all()"
          ],
          "question_id": "q_quiz_beautifulsoup_v2",
          "stem": "使用BeautifulSoup解析HTML后，哪个方法可以打印格式化的HTML结构？"
        }
      ]
    },
    {
      "concept_key": "xpath_lxml",
      "coverage_tags": [
        "xpath_lxml"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_xpath",
      "learning_goal": "学生能选择正确的XPath表达式定位元素。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "XPath",
          "en": "XML Path Language"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "// 表示任意位置，[@id='constituents'] 筛选id属性。",
          "options": [
            "选择所有 <table> 元素",
            "选择id属性为 'constituents' 的 <table> 元素",
            "选择所有包含 'constituents' 文本的元素",
            "选择第一个 <table> 元素"
          ],
          "question_id": "q_quiz_xpath_v1",
          "stem": "XPath表达式 '//table[@id=\"constituents\"]' 的作用是什么？"
        },
        {
          "answer": 1,
          "estimated_seconds": 12,
          "explanation": "[2] 表示位置索引，从1开始。",
          "options": [
            "所有 <li> 元素",
            "第二个 <li> 元素",
            "所有包含文本 '2' 的 <li> 元素",
            "前两个 <li> 元素"
          ],
          "question_id": "q_quiz_xpath_v2",
          "stem": "XPath中 '//li[2]' 会选择什么？"
        }
      ]
    },
    {
      "concept_key": "selenium_dynamic",
      "coverage_tags": [
        "selenium_dynamic"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_selenium",
      "learning_goal": "学生能判断何时需要使用Selenium。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "Selenium",
          "en": "Selenium"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "Selenium可以模拟浏览器执行JavaScript，获取动态加载的内容。",
          "options": [
            "目标网页是纯静态HTML",
            "目标网页内容通过JavaScript动态加载",
            "目标网页提供REST API",
            "目标网页只有一张图片"
          ],
          "question_id": "q_quiz_selenium_v1",
          "stem": "以下哪种情况最适合使用Selenium进行爬取？"
        },
        {
          "answer": 1,
          "estimated_seconds": 12,
          "explanation": "headless 模式不显示浏览器界面，节省系统资源。",
          "options": [
            "提高爬取速度",
            "减少内存和CPU占用",
            "绕过反爬机制",
            "自动处理Cookie"
          ],
          "question_id": "q_quiz_selenium_v2",
          "stem": "Selenium的 'headless' 模式的主要优势是什么？"
        }
      ]
    },
    {
      "concept_key": "yfinance_usage",
      "coverage_tags": [
        "yfinance_usage"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_yfinance",
      "learning_goal": "学生能正确使用yfinance下载多只股票数据。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "yfinance",
          "en": "yfinance"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "yf.download 的 tickers 参数用空格分隔多个股票代码。",
          "options": [
            "yf.download('0700.HK', 'AAPL', start='2023-01-01')",
            "yf.download('0700.HK AAPL', start='2023-01-01')",
            "yf.download(['0700.HK', 'AAPL'], start='2023-01-01')",
            "yf.download('0700.HK+AAPL', start='2023-01-01')"
          ],
          "question_id": "q_quiz_yfinance_v1",
          "stem": "使用yfinance同时下载腾讯(0700.HK)和苹果(AAPL)的历史数据，正确的调用方式是？"
        },
        {
          "answer": 2,
          "estimated_seconds": 12,
          "explanation": "ticker.actions 返回包含分红、拆股等公司行为的数据。",
          "options": [
            "ticker.info",
            "ticker.history()",
            "ticker.actions",
            "ticker.dividends"
          ],
          "question_id": "q_quiz_yfinance_v2",
          "stem": "yfinance中哪个方法可以获取股票的分红和拆股信息？"
        }
      ]
    },
    {
      "concept_key": "adjusted_close_concept",
      "coverage_tags": [
        "adjusted_close_concept"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_adjusted_close",
      "learning_goal": "学生能理解调整收盘价的目的与计算逻辑。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "调整收盘价",
          "en": "Adjusted Close"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "调整收盘价通过回溯调整历史价格，消除了公司事件造成的价格跳跃，使长期分析更准确。",
          "options": [
            "反映股票的真实市场交易价格",
            "消除分红、拆股等事件对价格序列的影响",
            "让股价看起来更高",
            "用于计算当日的成交量"
          ],
          "question_id": "q_quiz_adjusted_close_v1",
          "stem": "调整收盘价的主要目的是什么？"
        },
        {
          "answer": 1,
          "estimated_seconds": 20,
          "explanation": "2:1拆股意味着每股变成两股，价格减半，所以调整系数为0.5。",
          "options": [
            "1.0",
            "0.5",
            "2.0",
            "0.25"
          ],
          "question_id": "q_quiz_adjusted_close_v2",
          "stem": "某股票在2月18日发生2:1拆股，拆股前收盘价为$100，拆股后收盘价为$50。调整收盘价对2月17日的价格应乘以什么系数？"
        }
      ]
    },
    {
      "concept_key": "sqlite_basics",
      "coverage_tags": [
        "sqlite_basics"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_sqlite",
      "learning_goal": "学生能选择正确的SQL语句完成数据库操作。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "SQLite",
          "en": "SQLite"
        }
      ],
      "variants": [
        {
          "answer": 0,
          "estimated_seconds": 12,
          "explanation": "CREATE TABLE 是创建表的正确语法。",
          "options": [
            "CREATE TABLE users (id INTEGER, name TEXT);",
            "INSERT TABLE users (id INTEGER, name TEXT);",
            "NEW TABLE users (id INTEGER, name TEXT);",
            "ADD TABLE users (id INTEGER, name TEXT);"
          ],
          "question_id": "q_quiz_sqlite_v1",
          "stem": "在SQLite中，以下哪个SQL语句用于创建新表？"
        },
        {
          "answer": 0,
          "estimated_seconds": 12,
          "explanation": "UPDATE 语句用于修改表中已有记录。",
          "options": [
            "UPDATE users SET age=31 WHERE name='Alice';",
            "MODIFY users SET age=31 WHERE name='Alice';",
            "ALTER users SET age=31 WHERE name='Alice';",
            "CHANGE users SET age=31 WHERE name='Alice';"
          ],
          "question_id": "q_quiz_sqlite_v2",
          "stem": "在SQLite中，以下哪个SQL语句用于更新已有数据？"
        }
      ]
    },
    {
      "concept_key": "database_design_tradeoffs",
      "coverage_tags": [
        "database_design_tradeoffs"
      ],
      "difficulty": "medium",
      "family_id": "qf_quiz_db_design",
      "learning_goal": "学生能根据使用场景选择合适的分区策略。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "数据库设计",
          "en": "Database Design"
        }
      ],
      "variants": [
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "按股票分区可以快速查询单只股票的全部历史数据，适合回测。",
          "options": [
            "按日期分区",
            "按股票分区",
            "将所有数据放在一张大表中",
            "按交易所分区"
          ],
          "question_id": "q_quiz_db_design_v1",
          "stem": "如果你的主要任务是回测单只股票的策略，哪种数据库分区设计更合适？"
        },
        {
          "answer": 1,
          "estimated_seconds": 15,
          "explanation": "按日期分区后，查询单只股票需要跨多个分区，速度较慢。",
          "options": [
            "跨资产分析困难",
            "回测单只股票时查询慢",
            "数据备份困难",
            "无法进行SQL查询"
          ],
          "question_id": "q_quiz_db_design_v2",
          "stem": "按日期分区存储市场数据的主要缺点是什么？"
        }
      ]
    },
    {
      "concept_key": "scraping_limitations",
      "coverage_tags": [
        "scraping_limitations"
      ],
      "difficulty": "easy",
      "family_id": "qf_quiz_limitations",
      "learning_goal": "学生能识别网络爬取的主要局限性。",
      "linked_steps": [
        "step1"
      ],
      "question_type": "single_choice",
      "term_refs": [
        {
          "display": "网络爬取局限性",
          "en": "Web Scraping Limitations"
        }
      ],
      "variants": [
        {
          "answer": 3,
          "estimated_seconds": 12,
          "explanation": "网络爬取受限于网站的反爬措施和动态加载，并非所有数据都能实时获取。",
          "options": [
            "数据可能不完整或过时",
            "消耗大量带宽和存储",
            "网站结构可能频繁变化",
            "可以实时获取任何网站的数据"
          ],
          "question_id": "q_quiz_limitations_v1",
          "stem": "以下哪项不是网络爬取的局限性？"
        },
        {
          "answer": 1,
          "estimated_seconds": 10,
          "explanation": "IP封锁和速率限制是常见的反爬措施，用于限制爬虫访问频率。",
          "options": [
            "提供免费API",
            "IP封锁和速率限制",
            "使用纯文本格式",
            "开放所有数据"
          ],
          "question_id": "q_quiz_limitations_v2",
          "stem": "以下哪种反爬措施是网站常用的？"
        }
      ]
    }
  ],
  "source": {
    "coverage_checklist": "Understand the basic website structure; Web scraping using different python libraries; The limitations of web scraping; Create a simple database using python; Understand the concept of adjusted price; Database design for financial market data storage",
    "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
    "lesson_map": "{\"lesson_id\":\"L2\",\"mode\":\"guided_story\",\"steps\":[{\"concept\":\"MVP lesson slice\",\"file\":\"research/pipeline/3-guided_story/L2/step1/step.json\",\"sequence_id\":\"step1\"}]}",
    "plain_text": "pipeline/1-plain/L2/plain.txt",
    "related": [
      "pipeline/2-related_important/course_desc.md"
    ],
    "source_outline": "L2: Data scraping and database management with Python"
  },
  "target_language": "zh-CN"
}

]
</QUESTION_BANK>

<PLAIN_TEXT>
# L2: Data scraping and database management with Python

Course Code: COMP7415

# Agenda

- Understand the basic website structure   
- Web scraping using different python libraries   
- The limitations of web scraping   
- Create a simple database using python   
- Understand the concept of adjusted price   
- Database design for financial market data storage

# Installation of Python

- Python: https://www.python.org/downloads/   
- Anaconda (suggested): https://www.anaconda.com/download/success

- easier management for different environments   
- pre-installed data analysis packages (eg. numpy, pandas, scipy, etc)

# What’s Data/Web Scraping?

- Automated process to extract data from websites   
- Common use cases in algo-trading:

Real-time data collection (eg. economic indicators, stock prices, etc)   
- Market sentiment analysis (eg. news, forum and blogs, social media)   
- Corporate analysis (eg. product announcements, annual reports, director changes, etc)   
Alternative data (eg. weather, web traffic, etc)

# Basic Web structure

# A website is usually made of

1. HTML (HyperText Markup Language)   
2. CSS (Cascading Style Sheets)   
3. JavaScript

# HTML

- Purpose: Defines the structure and content of a webpage.   
- Elements: Uses tags to create elements such as headings, paragraphs, links, images, tables, etc.   
• Markup Language: Not a programming language; it’s a markup language used to structure content.

# Common HTML Tags

<table><tr><td>Tag</td><td>Description</td></tr><tr><td></td><td>Defines the root of an HTML document.</td></tr><tr><td></td><td>Contains meta-information about the HTML document, such as title, styles, and scripts.</td></tr><tr><td></td><td>Sets the title of the HTML document, displayed in the browser&#x27;s title bar or tab.</td></tr><tr><td></td><td>Contains the content of the HTML document, including text, images, links, and other elements.</td></tr><tr><td></td><td>Defines headings of different sizes, with &lt;h1&gt; being the largest and &lt;h6&gt; the smallest.</td></tr><tr><td></td><td>Defines a paragraph of text.</td></tr><tr><td></td><td>Creates a hyperlink, linking to another webpage, file, or location within the same page.</td></tr><tr><td></td><td>Inserts an image into the HTML document.</td></tr><tr><td></td><td>Defines a division or container for grouping HTML elements and applying CSS styles.</td></tr><tr><td></td><td>Defines an inline container for styling a specific portion of text or content.</td></tr><tr><td></td><td>Creates an unordered or ordered list, respectively, containing list items (&lt;li&gt;).</td></tr><tr><td></td><td>Defines a list item within an unordered or ordered list.</td></tr><tr><td></td><td>Creates a table for organizing data into rows and columns.</td></tr><tr><td></td><td>Defines a table row within a &lt;table&gt; element.</td></tr><tr><td></td><td>Defines a table cell within a &lt;tr&gt; element.</td></tr><tr><td></td><td>Defines a header cell within a &lt;tr&gt; element in a table.</td></tr><tr><td></td><td>Creates an HTML form for collecting user input.</td></tr><tr><td></td><td>Defines an input control element within a form, such as text fields, checkboxes, or buttons.</td></tr><tr><td></td><td>Associates a label with an input element, improving accessibility and usability.</td></tr><tr><td></td><td>Defines a clickable button within a form or webpage.</td></tr><tr><td></td><td>Creates a multiline text input control within a form.</td></tr><tr><td></td><td>Embeds another HTML page or external content within the current document.</td></tr><tr><td></td><td>Embeds or links to client-side scripts, such as JavaScript.</td></tr><tr><td></td><td>Contains CSS rules for styling HTML elements within the document.</td></tr></table>

# HTML Example

```html
1 <!--DOCTYPE html>
2 <html>
3 <head>
4 <meta charset="utf-8">
5 <meta name="viewport" content="width=device-width, initial-scale=1">
6 <title>Page Title</title>
7 </head>
8 <body>
9 <h1>Heading</h1>
10 <p>Paragraph</p>
11 </body>
12 </html>
13 
```

![](images/9a0356d3c69aa54d62765a0ef2abff6400803e8e1063976ac685e8e80ce3ccd6.jpg)

# CSS

- Purpose: Controls the visual presentation and layout of a webpage   
- Styling: Defines styles for HTML elements, such as colors, fonts, spacing, and positioning.   
- Cascading Rules: Multiple styles can be applied to an element, with rules for which styles take precedence.

<!DOCTYPE html>   
<html>   
<head> <meta charset="utf-8"> <meta name $=$ "viewport" content $=$ "width $\equiv$ device-width, initial-scale $= 1$ titlePage Title</title> <style type $=$ "text/css"> body { font-family: Arial,sans-serif; margin:0; } h1{ color:blue; text-align:center; } p{ font-size:14px; line-height:1.5; } </style>   
</head>   
<body> <h1>Heading</h1> <p>Paragraph</p>   
</body>   
</html>

![](images/c35f5d13e9683acbd934c657333f5da9f266c0c770a62cd650e50b8bd39e14cb.jpg)

# JavaScript

- Purpose: Adds interactivity and dynamic behavior to webpages.   
- Programming Language: A scripting language that runs in the browser (client-side) and/or on the server (server-side with Node.js).   
- Manipulation: Can manipulate HTML and CSS to change content and styles dynamically.   
- Event Handling: Responds to user actions like clicks, form submissions, and keyboard input.

1<!DOCTYPE html>   
2<html>   
3head>   
4 <meta charset="utf-8">   
5 <meta name $=$ "viewport" content $=$ "width $\equiv$ device-width, initial-scale $= 1$ 6 <title>Page Title</title>   
7   
8 <style type $=$ "text/css">   
9 body{   
10 font-family: Arial, sans-serif;   
11 margin:0;   
12 }   
13   
14 h1{   
15 color: blue;   
16 text-align: center;   
17 }   
18   
19 p{   
20 font-size: 14px;   
21 line-height: 1.5;   
22 }   
23 </style>   
24   
25 <script type $=$ "text/javascript">   
26 document.addEventListener("DOMContentLoaded", function(){ document.querySelector("h1").style.color $=$ "red";   
27 });   
28 });   
29 </script>   
30   
31   
32 </head>   
33 <body>   
34 <h1>Heading</h1>   
35 <p>Paragraph</p>   
36 </body>   
37 </html>

![](images/e6c5cbfbf2149c0808a26a2e790762c64c7ffc6a4a645e19a5552ff19553533d.jpg)

# Task: Scrape S&P 500 stock composites

三

![](images/b77c8a61bb89d87ef588664e634543090be902e2503b1ea022ee41d2921b091f.jpg)

WIKIPEDIA

The Free Encyclopedia

![](images/214dfa94388a03b20ce2c704651d403b3657eef3a7f9cc9e105a99c594b504f0.jpg)

Search Wikipedia

Search

Create account Log in

中

# List of S&P 500 companies

文A 5 languages

Contents

hide

Article Talk

Rea

Edit View history

Tools

Appearance

hide

(Top)

S&P 500 component stocks

Selected changes to the list of S&P 500 components

See also

References

External links

From Wikipedia, the free encyclopedia

The S&P 500 is a stock market index maintained by S&P Dow Jones Indices. It comprises 503 common stocks which are issued by 500 large-cap companies traded on American stock exchanges (including the 30 companies that compose the Dow Jones Industrial Average). The index includes about 80 percent of the American equity market by capitalization. It is weighted by free float market capitalization, so more valuable companies account for relatively more weight in the index. The index constituents and the constituent weights are updated regularly using rules published by S&P Dow Jones Indices. Although called the S&P 500, the index contains 503 stocks because it includes two share classes of stock from 3 of its component companies.[1][2]

S&P 500 component stocks [edit]   

<table><tr><td>Symbol</td><td>Security</td><td>GICS Sector</td><td>GICS Sub-Industry</td><td>Headquarters Location</td><td>Date added</td><td>CIK</td><td>Founded</td></tr><tr><td>MMM</td><td>3M</td><td>Industrials</td><td>Industrial Conglomerates</td><td>Saint Paul, Minnesota</td><td>1957-03-04</td><td>0000066740</td><td>1902</td></tr><tr><td>AOS</td><td>A. O. Smith</td><td>Industrials</td><td>Building Products</td><td>Milwaukee, Wisconsin</td><td>2017-07-26</td><td>0000091142</td><td>1916</td></tr><tr><td>ABT</td><td>Abbott</td><td>Health Care</td><td>Health Care Equipment</td><td>North Chicago, Illinois</td><td>1957-03-04</td><td>0000001800</td><td>1888</td></tr><tr><td>ABBV</td><td>AbbVie</td><td>Health Care</td><td>Biotechnology</td><td>North Chicago, Illinois</td><td>2012-12-31</td><td>0001551152</td><td>2013 (1888)</td></tr></table>

Text

$\bigcirc$ Small   
Standard   
Large   
Width   
Standard   
Wide

# Inspect the website structure

![](images/36f71ed7f6522db96b6f97ca6c123003ab1e6ac15c1e5293a629c3406d44e819.jpg)

# Web Scraping with Python

# BeautifulSoup

- Popular python library for searching and parsing HTML and XML data   
- Official Doc: https://beautiful-soup-4.readthedocs.io/en/latest/

pip install beautifulsoup4

pip install requests

# Download raw content of a web page

import requests

r = requests.get("https://google.com")

text = r.text

print(text)

![](images/1c510c9ef62a467a9b008790e6d92945b10b7b29c31e65fe8ac2912ea6c8388d.jpg)

![](images/27e340c19b2a898ca061322045116f0314e1f560fd21aaf566b575103d6c500b.jpg)

![](images/aa2d370b2d5e178a4798bce1aa2781e648a2463b39ec7224c2865a7582ba7048.jpg)

Oogle.com

![](images/d43d0c915f02591061c3ef3f1b123b1cada0b8953c59f07719f76c8b9d6b1d3d.jpg)

![](images/63663c1386f8ac65d2cec3d5d69d49c0c99d6462c5ad860cd37d075c5ad1f465.jpg)

```html
<!DOCTYPE html><html itemscope="itemtype="http://schema.org/WebPage" lang="zh-HK"><head><meta content="text/html; charset=UTF-8" http-equiv="Content-Type"><meta content="/images/branding/googleleg/1x/googleleg_STANDARD_color_128dp.png" itemprop="image"<title>Google</title><script nonce="AUM0Px-zhCjmfeAs-C9Bg">(function(){var g={'kEI:'mjCCZtn3ENS12roPtyirAs',kEXPI:'0,3700303,639,439,7,21,538636,2872,2891,8349,3405,31274,30022,1235 74,56390,2,39761,6699,41949,57734,2,2,1,24626,2006,8155,23350,22436,9779,62658,73178,3030,15816,1804,21011, 5396,686,8175,11814,1634,43464,7591,5217089,891,623,37,130,5991641,2839759,16,527,240,4,19,3,1,51,1,47,2798 1467,16672,43887,3,318,4,1281,3,2124363,23029351,7954,1,4844,8408,10903,5762,28027,36870,10511,2370,6407,13 845,10475,2478,2212,7981,200,154,21743,4,23390,4139,3181,6904,11584,2483,4272,155,1759,5,720,4400,9103,6452 ,1285,6599,2539,740,,2,,225,,539,,1643,,1449,,206,,122,,546,,2671,,4,,3004,,10417,,3480,,1694,,4082,,409,,519,,2805,,1,,6,, 159 6,,3.85,,1224,1668,1699,3936,,23,663,,2929,,2308,,1440,,1119,,3，52，22，344，1509，423，665，7170，975，709，231，3102，4，1162 , 120，1704，334，665，304，7，1，839，1714，682，288，376，14，5657，34，476，4569，215，107，716，270，6，495，1，570，205，367，50， 1 441，435，287，2，9，622，1737，15，1，1，1621，3，1272，654，258，2133，6，208，36，294，831，1948，464，436，293，3，2，573，84，4， 1 , 297，412，1574，367，3.94.572.1109.25.107.172.3.1.0.307.221.65.449.51.23.1.3.1. 10.553.883.401.190.841.610.55.504.273.1.8., 1. 340.207.537.871. 127.291. 366.714. 1212. 1693. 60. 142. 295. 11. 108. 1 8I. 1I.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII
```

# Quick Example

```html
<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><title>Web Scraping Demo</title></head><body><h1>Web Scraping Demo Page</h1></p>Welcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills.</p>Sample Table</h2> <table border="1">thead>
<tr><th>Name</th><th>Age</th><th>City</th></tr><th>Head</th><th>Body</th>
<td>Alice</td><td>30</td><td>New York</td></tr><tr><td>Bob</td><td>25</td>
<td>Los Angeles</td><td>35</td><td>Chicago</td></tr>
</table><h2>Useful Links</h2><p>Check out the following resources for more information:</p><ul><a href="https://www.python.org/"Python Official Website</a></li><a
href="https://www.crummy.com/software/BeautifulSoup/"Beautiful Soup
Documentation</a></li><a href="https://requests.readthedocs.io/"Requests:
HTTP for Humans</a></li></body></html> 
```

<!DOCTYPE html>   
<html lang="en">   
<head> <meta charset="UTF-8"> <meta name $=$ "viewport" content $=$ "width $\equiv$ device-width, initial-scale $= 1.0$ > <title>Web Scraping Demo</title> </head>   
<body> <h1>Web Scraping Demo Page</h1> <p>Welcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills.</p> <h2>Sample Table</h2> <table border $=$ "1"> <thead> <tr> <th>Name</th> <th>Age</th> <th>City</th> </tr> </thead> <tbody> <tr> <td>Alice</td> <td>30</td> <td>New York</td> </tr> <tr> <td>Bob</td> <td>25</td> <td>Los Angeles</td> </tr> <tr> <td>Charlie</td> <td>35</td> <td>Chicago</td> </tr> </tbody> </table> <h2>Useful Links</h2> <p>Check out the following resources for more information:</p> <ul> <li><a href $=$ "https://www.python.org/"Python Official Website</a></li> <li><a href $=$ "https://www.ccrummy.com/software/BeautifulSoup/"Beautiful Soup Documentation</a></li> <li><a href $=$ "https://requests.readthedocs.io/"Requests: HTTP for Humans</a></li> </ul>   
</body>   
</html>

# Print website elements in a nice format

# - prettify()

import requests

from bs4 import BeautifulSoup

```html
text ='<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><title>Web Scraping Demo</title></head><body><h1>Web Scraping Demo Page</h1></p>Welcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills.</p><h2>Contact Demo Table</h2></table border="1"><thead><tr><th>Name</th><th>Age</th><th>City</th></tr></thead><tbody><td>Alice</td><td>30</td><td>New York</td></tr></td>Bob</td><td>25</td><td>Los Angeles</td></tr><tr><td>Charlie</td><td>35</td><td>Chicago</td></tr></table><h2>Useful Links</h2></p>Check out the following resources for more information:</p><ul></a href="/https://www.python.org/"Python Official Website</a></li></a href="/https://www.crummy.com/software/BeautifulSoup/"Beautiful Soup Documentation</a></li></a href="/https://requests.readthedocs.io/"Requests: HTTP for Humans</a></li></ul></body></html'> 
```

soup = BeautifulSoup(text, 'html.parser')

print(soup.prettify())

```html
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8"/>
<meta content="width device-width, initial-scale=1.0" name="viewport"/>
<title>
Web Scraping Demo
</title>
</head>
<body>
<h1>
Web Scraping Demo Page
</h1>
<p>
Welcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills.</p>
<h2>
Sample Table
</h2>
<table border="1">
<thead>
<tr>
<th>
Name</th>
<th>
Age</th>
<th>
City</th>
</tr>
</thead>
<tbody>
<tr>
<td>
Alice</td>
<td>
30</td>
<td>
New York</td>
</table>
</body>
</table> 
```

# Extract all Text in a website

# • get_text()

import requests

from bs4 import BeautifulSoup

```html
text ='<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><title>Web Scraping Demo</title></head><body><h1>Web Scraping Demo Page</h1></p>Welcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills.</p><h2>Contact"Sample Table</h2></table border="1"><thead><tr><th>Name</th><th>Age</th><th>City</th></tr></thead><tr><td>Alice</td><td>30</td><td>New York</td></tr></td>Bob</td><td>25</td><td>Los Angeles</td></tr></td>Charlie</td></td>35</td><td>Chicago</td></tr></table><h2>Useful Links</h2></p>Check out the following resources for more information:</p><ul><a href="#">https://www.python.org/"Python Official Website</a></li><a href="#">https://www.ccrummy.com/software/BeautifulSoup/"Beautiful Soup Documentation</a></li><a href="#">https://requests.readthedocs.io/"Requests: HTTP for Humans</a></li></body></html'> 
```

soup = BeautifulSoup(text, 'html.parser')

print(soup.get_text())

Web Scraping Demo Web Scraping Demo Page Welcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills. Sample Table Name Age City Alice 30 New York Bob 25 Los Angeles Charlie 35 Chicago Useful Links Check out the following resources for more information: Python Official Website Beautiful Soup Documentation Requests: HTTP for Humans

# Extract the $1^{\text {st }}$ paragraph <p>

- find("p")

import requests from bs4 import BeautifulSoup

```html
text ='<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><title>Web Scraping Demo</title></head><body><h1>Web Scraping Demo Page</h1></p>Welcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills.</p><h2>Contact Table</h2><table border="1"><thead><tr><th>Name</th><th>Age</th><th>City</th></tr><tr><td>Alice</td><td>Alice</td></td><td>Old</td><td>New York</td></tr><tr><td>Bob</td><td>Chicago</td></td><td>Los Angeles</td></tr><tr><td>Charlie</td><td>Chicago</td></td><td>Philadelphia</td></table><td>Useful Links</h2></p>Check out the following resources for more information:</p><ul><li><a href="https://www.python.org/"Python Official Website</a></li><a href="https://www.crummy.com/software/BeautifulSoup/"Beautiful Soup Documentation</a></li><a href="https://requests.readthedocs.io/"Requests: HTTP for Humans</a></li></body></html'> 
```

soup = BeautifulSoup(text, 'html.parser')

data = soup.find('p')

print(data.text)

Welcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills.

# Extract all paragraphs <p>

# • find_all("p")

import requests from bs4 import BeautifulSoup

```html
text ='<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><title>Web Scraping Demo</title></head><body><h1>Web Scraping Demo Page</h1></p>Welcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills.</p><h2>Contact Table</h2><table border="1"><thead><tr><th>Name</th><th>Age</th><th>City</th></tr></thead><tr><td>Alice</td><td>Alice</td><td>30</td><td>New York</td></tr><tr><td>Bob</td><td>25</td><td>Los Angeles</td></tr><tr><td>Charlie</td><td>35</td><td>Chicago</td></tr></table><td>H2>Useful Links</h2></p>Check out the following resources for more information:</p><ul><li><a href="https://www.python.org/"Python Official Website</a></li><a href="https://www.crummy.com/software/BeautifulSoup/"Beautiful Soup Documentation</a></li><a href="https://requests.readthedocs.io/"Requests: HTTP for Humans</a></li></body></html>' 
```

soup = BeautifulSoup(text, 'html.parser')

data = soup.find_all('p')

for p in data: print(p.text)

Welcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills. Check out the following resources for more information:

# Extract all hyperlinks $< a>$

# • find_all("a")

import requests

from bs4 import BeautifulSoup

text ='<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><title>Web Scraping Demo</title></head><body><h1>Web Scraping Demo Page</h1></body> <p>Welcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills.</p> <h2>Sample Table</h2> <table border="1"><thead><tr>
    <th>Name</th> <th>Age</th> <th>City</th> </tr> </thead> <tbody> </td>Alice</td>
    <td>30</td> <td>New York</td> </tr> <tr> <td>Bob</td> <td>25</td> <td>Los Angeles</td>
    </tr> </td> Charlie</td> <td>35</td> <td>Chicago</td> </tr> </tbody>
    <h2>Contact Link</h2>
    <p>Check out the following resources for more information:</p>
    <li><a href="https://www.python.org/" Python Official Website</a></li><a
href="https://www.crummy.com/software/BeautifulSoup/" Beautiful Soup
Documentation</a></li><a href="https://requests.readthedocs.io/" Requests: HTTP for Humans</a></li></body></html>'

soup = BeautifulSoup(text, 'html.parser')  
data = soup.find_all('a')

for a in data: print(a) print(a.text) print("\n")

<a href="https://www.python.org/" >Python Official Website</a>  
Python Official Website

<a href="https://www.crummy.com/software/BeautifulSoup/" Beautyful Soup Documentation</a> Beautiful Soup Documentation

<a href="https://requests.readthedocs.io/"Requests: HTTP for Humans</a> Requests: HTTP for Humans

# Extract all <table>

![](images/a8866a3125635d8949f600fafc109a74455f0b0638b1066e329bb4036cfd5d4e.jpg)

```txt
Name Age City 0 Alice 30 New York 1 Bob 25 Los Angeles 2 Charlie 35 Chicago 
```

```python
import requests  
import pandas as pd  
from bs4 import BeautifulSoup 
```

```html
text ='<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><title>Web Scraping Demo</title></head><body><h1>Web Scraping Demo Page</h1></p>Welcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills.</p><h2>Sample Table</h2><table border="1"><thead><tr><th>Name</th><th>Age</th><th>City</th></tr><th>Allele</td><td>Alice</td><td>Old</td><td>New York</td></tr><tr><td>Bob</td><td>Chicago</td></tr><tr><td>Los Angeles</td></table><td>Useful Links</h2></p>Check out the following resources for more information:</p><li><a href="https://www.python.org/"Python Official Website</a></li><a href="https://www.crammy.com/software/BeautifulSoup/"Beautiful Soup Documentation</a></li><a href="https://requests.readthedocs.io/"Requests: HTTP for Humans</a></li></body></html'> 
```

```python
soup = BeautifulSoup(text, 'html.parser') 
```

```txt
Find the table  
table = soup.find("table") 
```

```python
Extract the headers  
headers = []  
for th in table.find_all('th'):  
    headers.append(th.text) 
```

Extract the rows   
rows $= \left[\right]$ for tr in table.find_all('tr'[1:]: #Skip the header row cells $=$ tr.find_all('td') row $=$ [cell.text for cell in cells] rows.append(row)

```txt
Create a DataFrame df = pd.DataFrame(columns, headers) print(df) 
```

# Scrape S&P 500 stock composites

三

![](images/ed717c804f45387ca8f195878f26adf181536b27ed3983caf82851bc1881a259.jpg)

WIKIPEDIA

The Free Encyclopedia

![](images/136ede79f45d9d9111f6704547b8b07c1ed7b888b6881627de51dac9e99b96c9.jpg)

Search Wikipedia

Search

Create account Log in

中

# List of S&P 500 companies

文A 5 languages

Contents

hide

Article Talk

Rea

Edit View history

Tools

Appearance

hide

# (Top)

S&P 500 component stocks

Selected changes to the list of S&P 500 components

# See also

References

External links

From Wikipedia, the free encyclopedia

The S&P 500 is a stock market index maintained by S&P Dow Jones Indices. It comprises 503 common stocks which are issued by 500 large-cap companies traded on American stock exchanges (including the 30 companies that compose the Dow Jones Industrial Average). The index includes about 80 percent of the American equity market by capitalization. It is weighted by free float market capitalization, so more valuable companies account for relatively more weight in the index. The index constituents and the constituent weights are updated regularly using rules published by S&P Dow Jones Indices. Although called the S&P 500, the index contains 503 stocks because it includes two share classes of stock from 3 of its component companies.[1][2]

S&P 500 component stocks [edit]   

<table><tr><td>Symbol</td><td>Security</td><td>GICS Sector</td><td>GICS Sub-Industry</td><td>Headquarters Location</td><td>Date added</td><td>CIK</td><td>Founded</td></tr><tr><td>MMM</td><td>3M</td><td>Industrials</td><td>Industrial Conglomerates</td><td>Saint Paul, Minnesota</td><td>1957-03-04</td><td>0000066740</td><td>1902</td></tr><tr><td>AOS</td><td>A. O. Smith</td><td>Industrials</td><td>Building Products</td><td>Milwaukee, Wisconsin</td><td>2017-07-26</td><td>0000091142</td><td>1916</td></tr><tr><td>ABT</td><td>Abbott</td><td>Health Care</td><td>Health Care Equipment</td><td>North Chicago, Illinois</td><td>1957-03-04</td><td>0000001800</td><td>1888</td></tr><tr><td>ABBV</td><td>AbbVie</td><td>Health Care</td><td>Biotechnology</td><td>North Chicago, Illinois</td><td>2012-12-31</td><td>0001551152</td><td>2013 (1888)</td></tr></table>

# Text

![](images/6c0d5fa71a82fb88ccb08419bbf757b1b8995976c6e85c4bf56f9f3c836be31a.jpg)

Small

![](images/8da8c36f67938a556a6311230d2044eb87373c23e75794d55fff947a0f4b1f55.jpg)

Standard

![](images/6cb6a2aa069abcdd3e5afc6dbc34a01bf989944e529939141a6a55c6dd9fcff5.jpg)

Large

# Width

![](images/28003b144c3b363d1cf26bc4a9876f6d2622eacff7bf8e6fd24ae4bbacfddd19.jpg)

Standard

![](images/d66268d18e41a2493f99f7bbaec3f683b4cc692b96ff331cbf4ac55ff7f87a37.jpg)

Wide

# Get HTML content of the website

import requests   
from bs4 import BeautifulSoup   
import pandas as pd   
url $=$ 'https://en.wikipedia.org/wiki/List_of_S%26P_500_companies'   
text $=$ requests.get(url).text   
print(text)

![](images/d03efbbf1a5be83a807912cbc7e8194efae47bfc28540fcdb7a95ba9eb4eaf82.jpg)

Please set a user-agent and respect our robot policy. https://w.wiki/4wJS. See also https://phabricator.wikimedia.org/T400119.

Why we can't get the result?

# Extract all <table>

![](images/64ecad85c9d2110134bb242b6487da9c2033aa3acbc05e7122040d75612288cf.jpg)

There are 2 tables in the wiki page. How to locate the correct table?

import requests   
from bs4 import BeautifulSoup   
import pandas as pd   
headers $=$ { "user-agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36" }   
url $=$ 'https://en.wikipedia.org/wiki/List_of_S%26P_500_com panies' text $=$ requests.get(url, headers $\equiv$ headers).text   
soup $=$ BeautifulSoup(text, 'html.parser')   
data $=$ soup.find_all('table')   
print(len(data))

![](images/50fcace5412e145f4983130c1ed4a1272a25c18f06725f9d84afd62a53665f7d.jpg)

# Extract all <table>

![](images/cd3ed0e244cb21a7c0940c524fd9c1ab6b5a85b39672959aa3f3e20d4bda2984.jpg)

```csv
Symbol Security GICS Sector ... Date added CIK Founded 0 MMM 3M Industrials ... 1957-03-04 0000066740 1902 1 AOS A. O. Smith Industrials ... 2017-07-26 0000091142 1916 2 ABT Abbott Health Care ... 1957-03-04 0000001800 3 ABBV AbbVie Health Care ... 2012-12-31 0001551152 2013 (1888) 4 ACN Accenture Information Technology ... 2011-07-06 0001467373 1989 ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...
498 XYL Xylem Inc. Industries .. 2011-11-01 0001524472 2011 499 YUM Yum! Brands Consumer Discretionary .. 1997-10-06 0001041061 1997 500 ZBRA Zebra Technologies Information Technology .. 2019-12-23 0000877212 1969 501 ZBH Zimmer Biomet Health Care .. 2001-08-07 0001136869 1927 502 ZTS Zoetis Health Care .. 2013-06-21 0001555280 1952 
```

import requests

from bs4 import BeautifulSoup

import pandas as pd

headers = {

"user-agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64)

AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0

Safari/537.36"

}

url = 'https://en.wikipedia.org/wiki/List_of_S%26P_500_companies'

text = requests.get(url, headers=headers).text

soup = BeautifulSoup(text, 'html.parser')

Find the table

table = soup.find('table', id="constituents")

Extract the headers

headers $\equiv$ []

for th in table.find_all('th')

headers append(th.text.strip())

Extract the rows

rows = []

for tr in table.find_all('tr')[1]: # Skip the header row

cells = tr.find_all('td')

row = [cell.text.strip() for cell in cells]

rows.append(row)

Create aDataFrame

df = pd.DataFrame(columns, columns=headers)

print(df)

# Other search methods

- Search by CSS class

table = soup.find_all('table', class='wikitable sortable')

- Search by tag attributes

table = soup.find_all("table", attrs={'class':'wikitable sortable'})

- CSS Selector

table = soup.select('table')
table = soup.select('table[class="wikitable sortable"', table)
table = soup.select '.wikitable'
table = soup.select '.wikitable.sortable'

# LXML

- Popular python library for parsing XPath and XML schema   
- Official Doc: https://lxml.de/

pip install lxml  
pip install requests

# What is XPath?

- XPath is a query language to navigate the elements and attributes in an XML document   
Key features:

- Node Selection: XPath allows you to select nodes or elements in an XML document. For example, selecting elements by their tag name, attribute, or position.   
- Path Expressions: Provides a way to navigate through elements and attributes in an XML document using path expressions.   
- Syntax: Uses a compact, non-XML syntax to make it easier to write and read.

# Common XPath Syntax

- Basic Path: /root/child   
- Selects the child element of the root element.   
- Relative Path: //child   
- Selects all child elements in the document.   
- Attributes: //element [@attribute='value']   
- Selects elements with a specific attribute value.   
• Functions: //element[position(   )=1]   
- Selects the first element.

# XPath Example

- Select all <li> elements: //li   
- Select <li> elements with class "item": //li [@class='item']   
- Select the second <li> element: //li[2]

```txt
<ul>
    <li class="item">Item 1</li>
    <li class="item">Item 2</li>
    <li class="item">Item 3</li>
</ul> 
```

![](images/7c7be642b2b3cfa2bc57aa8cd6abd75c49b41c233af785235e71a53b9cbd7963.jpg)

# Extract stock list

![](images/3c2b29caec8d5dd3c15cd4083dc23d27aec108c27d3deaa85b1410dc7510202c.jpg)

<table><tr><td></td><td>Symbol</td><td>Security</td><td>GICS Sector</td><td>...</td><td>Date Added</td><td>CIK</td><td>Founded</td></tr><tr><td>0</td><td>MMM</td><td>3M</td><td>Industrials</td><td>...</td><td>1957-03-04</td><td>0000066740</td><td>1902</td></tr><tr><td>1</td><td>AOS</td><td>A. O. Smith</td><td>Industrials</td><td>...</td><td>2017-07-26</td><td>0000091142</td><td>1916</td></tr><tr><td>2</td><td>ABT</td><td>Abbott</td><td>Health Care</td><td>...</td><td>1957-03-04</td><td>0000001800</td><td>1888</td></tr><tr><td>3</td><td>ABBV</td><td>AbbVie</td><td>Health Care</td><td>...</td><td>2012-12-31</td><td>0001551152</td><td>2013 (1888)</td></tr><tr><td>4</td><td>ACN</td><td>Accenture</td><td>Information Technology</td><td>...</td><td>2011-07-06</td><td>0001467373</td><td>1989</td></tr><tr><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td></tr><tr><td>498</td><td>XYL</td><td>Xylem Inc.</td><td>Industrials</td><td>...</td><td>2011-11-01</td><td>0001524472</td><td>2011</td></tr><tr><td>499</td><td>YUM</td><td>Yum! Brands</td><td>Consumer Discretionary</td><td>...</td><td>1997-10-06</td><td>0001041061</td><td>1997</td></tr><tr><td>500</td><td>ZBRA</td><td>Zebra Technologies</td><td>Information Technology</td><td>...</td><td>2019-12-23</td><td>0000877212</td><td>1969</td></tr><tr><td>501</td><td>ZBH</td><td>Zimmer Biomet</td><td>Health Care</td><td>...</td><td>2001-08-07</td><td>0001136869</td><td>1927</td></tr><tr><td>502</td><td>ZTS</td><td>Zoetis</td><td>Health Care</td><td>...</td><td>2013-06-21</td><td>0001555280</td><td>1952</td></tr></table>

from lxml import html

import requests

import pandas as pd

headers $\equiv$ {

"user-agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36"

# download web page

url = 'https://en.wikipedia.org/wiki/List_of_S%26P_500 Companies'

response = requests.get(url, headers=headers)

Check that the request was successful

text = response(content

load into html tree

tree = html.fromstring(text)

extract table rows from XPath

table_rows = tree XPath('//*[@id="constituents"/tbody/tr']')

parse the table rows

data = []

for row in table_rows[1]: # Skip header row

cells = row.xpath('td')

if len(cells) > 0:

try:

symbol $=$ cells[0].text_content() strip()

security $=$ cells[1].text_content().strip()

gics sector = cells[2].text_content() strip()

gics_sub_industry = cells[3] text_content().strip()

headquarters $=$ cells[4].text_content().strip()

date-added = cells[5].text_content() strip()

cik = cells[6].text_content().strip()

founded $=$ cells[7].text_content().strip()

data append([symbol, security, gics_section, gics_sub_industry, headquarters,

date-added, cik, founded])

except Exception as e:

pass

create a DataFrame

columns = ['Symbol', 'Security', 'GICS Sector', 'GICS Sub-Industry', 'Headquarters Location',

'Date Added', 'CIK', 'Founded']

df = pd.DataFrame(data, columns=columns)

print the DataFrame

print(df)

# Yahoo Finance

# Yahoo Finance (https://finance.yahoo.com/)

# Available datasets

Market data

Stock, ETF, Index   
Historical data (End-of-day, weekly, monthly)   
Real-time data (up to 20 min delay)

- Corporate actions (eg. dividends, splits)   
- Company Profile (eg. Industry, sector, key executives, etc)   
- Financial (eg. Income statement, Balance sheet, Cash flow)   
- Analyst Estimates (earnings, revenue, growth)

![](images/472e5f0bcde95378059087475009ef8ac58e78aac5373477f570f3f2c1832917.jpg)

![](images/3b835223530d7f96591a41f807fddcf2d3462dfb66acf560785022335a67c90a.jpg)

# Extract stock price

import requests

from bs4 import BeautifulSoup

import pandas as pd

```txt
url = 'https://finance.yahoo.com/quote/0700.HK/history/'  
text = requests.get(url).text 
```

print(text)

```python
soup = BeautifulSoup(text, 'html.parser') 
```

Find the table

```txt
table - soup.find('table', class_= 'table yf-1m2i7s2') 
```

Extract the headers

[ \text{headers} = [] ]

for th in table.find_all('th'):

headers.append(th.text.strip())

Extract the rows

rows = []

for tr in table.find_all('tr')[1]: # Skip the header row

cells = tr.find_all('td')

row = [cell.text.strip() for cell in cells]

rows.append(row)

Create a DataFrame

df = pd.DataFrame(columns, columns=headers)

print(df)

```html
<!DOCTYPE html> <html lang="en-us"><head> <meta http-equiv="content-type" content="text/html; charset=UTF-8"> <meta charset="utf-8"> <title>Yahoo</title> <meta name="viewport" content="width-device-width,initial-scale=1,minimal-ui"> <meta http-equiv="X-UA-Compatible" content="IE=edge,chrome=1"> <style> html { height: 100%; } body { background: #fafafc url(https://s.yimg.com/nn/img/sad-panda-201402200631.png) 50% 50%; background-size: cover; height: 100%; text-align: center; font: 300 18px "helvetica neue", helvetica, verdana, tahoma, arial, sans-serif; } table { height: 100%; width: 100%; table-layout: fixed; border-collapse: collapse; border-spacing: 0; border: none; } h1 { font-size: 42px; font-weight: 400; color: #40090; } p { color: #1A1A1A; } #message-1 { font-weight: bold; margin: 0; } #message-2 { display: inline-block; *display: inline; zoom: 1; max-width: 17em; _width: 17em; } </style> </script> <document.write('<img src="///geo.yahoo.com/b?s=1197757129&t='+new Date().getTime()+"&src=aws&err_url="+encodeURIComponent(documents).URL)+&err=%<pssc>&test="+encodeURIComponent("%<Bucket)cqh[:200]")+"width="0px" height="0px"/");var beacon = new Image(); beacon.src="///bcn.fp.yahoo.com/p?p?s=1197757129&t="+new Date().getTime()+"&src=aws&err_url="+encodeURIComponent(documents).URL)+&err=%<pssc>&test="+encodeURIComponent("%<Bucket)cqh[:200]"); </script> </head> </body> <!-- status code : 404 --> </-- Not Found on Server --> <table> <tbody><tr> <td> <img src="https://s.yimg.com/rz/p/yahoo_frontpage_en-US_s_f_p_205x58_frontpage.png" alt="Yahoo Logo"> <h1 style="margin-top:20px;" Will be right back...</h1> <p id="message-1" Thank you for your patience.</p> <p id="message-2">Our engineers are working quickly to resolve the issue.</p> </td> </tr> </body></table> </body></html> 
```

# Why it doesn’t work?

yahoo!

Will be right back…

Thank you for your patience.

Our engineers are working quickly to resolve the issue.

???

# yfinance

It is an unofficial library for Yahoo Finance   
- Project website: https://pypi.org/project/yfinance/

pip install yfinance

# Extract stock info

# . info()

import yfinance as yf

ticket = yf.Ticker("0700.HK")

get ticker basic info

print(ticker.info)

{address1': 'Tencent Binhai Towers', 'address2': 'No. 33 Haitian 2nd Road Nanshan District', 'city': 'Shenzhen', 'zip': '518954', 'country': 'China', 'phone': '86 75 5860 13388', 'website': 'https://www.tencent.com', 'industry': 'Internet Content & Information', 'industryKey': 'internet-content-information', 'industryDisp': 'Internet Content & Information', 'sector': 'Communication Services', 'sectorKey': 'communication-services', 'sectorDisp': 'Communication Services', 'longBusinessSummary': "Tencent Holdings Limited, an investment holding company, offers value-added services (VAS), online advertising, fintech, and business services in the People's Republic of China and internationally. It operates through VAS, Online Advertising, FinTech and Business Services, and Others segments. The company's consumers business provides communication and services, such as instant messaging and social network; digital content including online games, videos, live streaming, news, music, and literature; fintech services, which includes mobile payment, wealth management, loans, and securities trading; and various tools, such as network security management, browser, navigation, application management, email, etc. Its enterprise business comprises marketing solutions, which offers digital tools including user insight, creative management, placement strategy, and digital assets management; and cloud services, such as cloud computing, big data analytics, artificial intelligence, Internet of Things, security and other technologies for financial services, education, healthcare, retail, industry, transport, energy, and radio & television application. In addition, the company operates innovation business, which includes artificial intelligences; and discover and develops enterprise and next-generation technologies for food production, energy, and water management application. Tencent Holdings Limited was formerly known as Tencent (BVI) Limited and changed its name to Tencent Holding Limited in February 2004. The company was founded in 1998 and is headquartered in Shenzhen, the People's Republic of China.", 'fullTimeEmployees': 104787, 'companyOfficers': [{'maxAge': 1, 'name': 'Mr. Huateng Ma', 'age': 51, 'title': 'Co-Founder, Chairman & CEO', 'yearBorn': 1972, 'fiscalYear': 2023, 'totalPay': 46131830, 'exercisedValue': 0}, 'unexercisedValue': 0}, {maxAge: 1, 'name': 'Mr. Chi Ping Lau', 'age': 50, 'title': 'President', 'yearBorn': 1973, 'fiscalYear': 2023, 'totalPay': 17832844, 'exercisedValue': 0, 'unexercisedValue': 1026971520}, {'maxAge': 1, 'name': Mr. Liqing Zeng', 'age': 53, 'title': Co-Founder & Advisor Emeritus', 'yearBorn': 1970, 'fiscalYear': 2023, 'exercisedValue': 0, 'unexercisedValue': 0}, {maxAge: 1, 'name': Mr. Chenye Xu', 'age': 52, 'title': Co-Founder & Chief Information Officer', 'yearBorn': 1971, 'fiscalYear': 2023, 'exercisedValue': 0, 'unexercisedValue': 0}, {maxAge: 1, 'name': Mr. Zhidong Zhang', 'age': 51, title': Co-Founder & Advisor Emeritus', yearBorn': 1972, fiscalYear': 2023, totalPay': 17342292, 'exercisedValue': 0, 'unexercisedValue': 0}, {'maxAge': 1, 'name': Mr. Yidan Chen', 'age': 52, title': Co-Founder & Advisor Emeritus', yearBorn': 1971, fiscalYear': 2023, unexercisedValue': 0}, {'maxAge': 1, 'name': Mr. Weibiao Zhan', age': 50, title': Managing Director', yearBorn': 1973, fiscalYear': 2023, exerischedValue': 0, unexercisedValue': 0}, {'maxAge': 1, 'name': Mr. Shek Hon Lo', age': 54}, title: CFO & Senior VP', yearBorn': 1969, fiscalYear': 2023, exercisedValue': 0, unexercisedValue': 0}, {'maxAge': 1, name: Mr. Yuxin Ren', age': 47, title': COO and President of Platform & Content Group & Interactive Entertainment Group', yearBorn': 1976, fiscalYear': 2023, exercisedValue': 0, unexercisedValue': 0}, {'maxAge': 1}, {name: Mr. Shan Lu', age': 48, title': Senior EVP and President of Technology & Engineering Group', yearBorn': 1975, fiscalYear': 2023, exerischedValue': 0, unexercisedValue': 0}], {'auditRisk': 10, boardRisk': 5}, {'compensationRisk': 10, shareHolderRightsRisk': 4, overallRisk': 10, governanceEpochDate': 1719792000}, {'compensationAsOfEpochDate': 1703980800, maxAge: 86400, priceHint: 3, previousClose: 369.2, open: 366.8, dayLow: 366.8, dayHigh: 373.0, regularMarketPreviousClose: 369.2, regularMarketOpen: 366.8, regularMarketDayLow: 366.8, regularMarketDayHigh: 373.0, dividendRate: 3.4, dividendYield: 0.0092', exDividendDate: 1715904000, payoutRatio: 0.1825, fiveYearAvgDividendYield: 0.45, beta: 0.557, trailingPE' : 29.052465,'forwardPE' : 14.704717,volume' :6796150,'regularMarketVolume' :6796150,'averageVolume' :22655010,'averageVolume10days' :17443391,'averageDailyVolume10Day' :17443391,'bid' :370.6,'ask' :370.8,'marketCap' :3466620305408,'fiftyTwoWeekLow' :260.2,'fiftyTwoWeekHigh' :401.0,'priceToSalesTrailing12Months' :5.6046114,'fiftyDayAverage' :369.028,'twoHundredDayAverage' :314.743,'trailingAnnualDividendRate' :3.086,'trailingAnnualDividendYield' :0.008358613,'currency' :HKD', enterpriseValue' :3475905445888,'profitMargins' :0.21222,'floatShares' :6226935680,'sharesOutstanding' :9343989760,'heldPercentInsiders' :0.33909,'heldPercentInstitutions' :0.23399,'impliedSharesOutstanding' :9424969728,'bookValue' :90.739,'priceToBook' :4.0886497,'lastFiscalYearEnd' :1703980800,'nextFiscalYearEnd' :1735603200,'mostRecentQuarter' :1711843200,'earningsQuarterlyGrowth' :0.621,'netIncomeToCommon' :131267002368,'trailingEps' :12.77,'forwardEps' :25.23,'pegRatio' :0.65,'lastSplitFactor' :5:1,'lastSplitDate' :1400112000,'enterpriseToRevenue' :5.62,'enterpriseToEbitda' :17.515,'52WeekChange' :0.10077524,'SandP52WeekChange' :0.23886502,'lastDividendValue' :3.4,'lastDividendDate' :1715904000,'exchange' : HKG', quoteType' :EQUITY', symbol' :O7OO.HK', underlyingSymbol' :O7OO.HK', shortName' TENGENT', longName' Tencent Holdings Limited', firstTradeDateEpochUtc: 1087349400,'timeZoneFullName' Asia/Hong Kong', timeZoneShortName' HKT', uuid' :341c2e2e-93bb-3ea-f-b5b-92ea7815949e', messageBoardId' :finmb_11042136', gmtOffSetMillseconds' :28800000,'currentPrice' :371.0,'targetHighPrice' :543.11,'targetLowPrice' :304.06,'targetMeanPrice' :463.24,'targetMedianPrice' :464.09,'recommendationMean' :1.7,'recommendationKey' : buy', numberOfAnalystOpinions' :47,'totalCash' :419042983936,'totalCashPerShare' :44.808,'ebitda' :198450003968,'totalDebt' :374007988224,'quickRatio' :1.213,'currentRatio' :1.451,'totalRevenue' :618530013184,'debtToEquity' :40.796,'revenuePerShare' :65.646,'returnOnAssets' : O.7O1I , returnOnEquity' : O.15278,'freeCashflow' : 143674867712,'operatingCashflow' : 232O15OoMgSdAy , earningsGrowth' : O.662,revenueGrowth' : O.063,grossMargins' : O.49925,'ebitdaMargins' : O.32O84,'operatingsMargins' : O.3295Ooo2 , financialCurrency':'CNY','trailIngPegRatio' : O.6468

# Extract stock history

• .history(period)   
available period ['1d', '5d', '1mo', '3mo', '6mo', '1y', '2y', '5y', '10y', 'ytd', 'max']

import yfinance as yf

ticket = yf.Ticker("0700.HK")

# download historical data

data = ticker-history(period='1mo')

print(data)

Open High Low Close Volume Dividends Stock Splits

Date

<table><tr><td>2025-12-22</td><td>00:00:00+08:00</td><td>620.0</td><td>621.5</td><td>610.0</td><td>614.5</td><td>13868060</td><td>0.0</td><td>0.0</td></tr><tr><td>2025-12-23</td><td>00:00:00+08:00</td><td>613.5</td><td>614.5</td><td>601.5</td><td>602.0</td><td>15623392</td><td>0.0</td><td>0.0</td></tr><tr><td>2025-12-24</td><td>00:00:00+08:00</td><td>602.5</td><td>602.5</td><td>602.5</td><td>602.5</td><td>0</td><td>0.0</td><td>0.0</td></tr><tr><td>2025-12-29</td><td>00:00:00+08:00</td><td>606.0</td><td>611.0</td><td>596.0</td><td>596.5</td><td>18502650</td><td>0.0</td><td>0.0</td></tr><tr><td>2025-12-30</td><td>00:00:00+08:00</td><td>598.5</td><td>601.0</td><td>594.0</td><td>600.0</td><td>13582535</td><td>0.0</td><td>0.0</td></tr><tr><td>2025-12-31</td><td>00:00:00+08:00</td><td>599.0</td><td>599.0</td><td>599.0</td><td>599.0</td><td>0</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-02</td><td>00:00:00+08:00</td><td>600.5</td><td>624.5</td><td>600.5</td><td>623.0</td><td>16200058</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-05</td><td>00:00:00+08:00</td><td>624.0</td><td>628.0</td><td>615.5</td><td>624.5</td><td>19947025</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-06</td><td>00:00:00+08:00</td><td>627.0</td><td>638.5</td><td>626.0</td><td>632.5</td><td>24168431</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-07</td><td>00:00:00+08:00</td><td>627.5</td><td>629.5</td><td>615.0</td><td>624.5</td><td>21378622</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-08</td><td>00:00:00+08:00</td><td>618.5</td><td>621.0</td><td>610.0</td><td>616.0</td><td>18742539</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-09</td><td>00:00:00+08:00</td><td>616.0</td><td>617.0</td><td>610.0</td><td>611.0</td><td>17813669</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-12</td><td>00:00:00+08:00</td><td>616.0</td><td>627.5</td><td>613.5</td><td>623.0</td><td>27530129</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-13</td><td>00:00:00+08:00</td><td>637.0</td><td>639.0</td><td>622.5</td><td>627.5</td><td>24212695</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-14</td><td>00:00:00+08:00</td><td>634.0</td><td>638.5</td><td>626.0</td><td>633.0</td><td>28677291</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-15</td><td>00:00:00+08:00</td><td>631.0</td><td>632.5</td><td>618.5</td><td>622.0</td><td>26194252</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-16</td><td>00:00:00+08:00</td><td>627.0</td><td>630.0</td><td>613.5</td><td>617.5</td><td>20616793</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-19</td><td>00:00:00+08:00</td><td>613.5</td><td>614.5</td><td>608.5</td><td>610.0</td><td>13233330</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-20</td><td>00:00:00+08:00</td><td>601.0</td><td>607.0</td><td>598.0</td><td>601.0</td><td>24332228</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-21</td><td>00:00:00+08:00</td><td>598.0</td><td>605.5</td><td>598.0</td><td>600.0</td><td>11476120</td><td>0.0</td><td>0.0</td></tr></table>

# Extract multiple stocks

- yf.download(tickers, start, end)   
- tickers separated by a space

import yfinance as yf

```python
ticket = yf.Ticker("0700.HK")

download Tencent, HSBC,Apple

data = yf.download("0700.HK 0005.HK AAPL", start="2025-01-01", end="2025-06-30")

print(data)

<table><tr><td>Price
Ticker
Date</td><td>Close
0005.HK</td><td>0700.HK</td><td>AAPL</td><td>High
0005.HK</td><td>...</td><td>Open
AAPL</td><td>Volume
0005.HK</td><td>0700.HK</td><td>AAPL</td></tr><tr><td>2025-01-02</td><td>71.669708</td><td>412.400024</td><td>242.752106</td><td>71.811724</td><td>...</td><td>247.809220</td><td>15851617.0</td><td>20733037.0</td><td>55740700.0</td></tr><tr><td>2025-01-03</td><td>71.006966</td><td>410.615601</td><td>242.264313</td><td>71.717035</td><td>...</td><td>242.264313</td><td>11755463.0</td><td>16843241.0</td><td>40244100.0</td></tr><tr><td>2025-01-06</td><td>71.527695</td><td>405.857117</td><td>243.896912</td><td>71.622369</td><td>...</td><td>243.210016</td><td>12886461.0</td><td>16869318.0</td><td>45045600.0</td></tr><tr><td>2025-01-07</td><td>71.291008</td><td>376.315002</td><td>241.119492</td><td>71.859062</td><td>...</td><td>241.886014</td><td>18577225.0</td><td>142920468.0</td><td>40856000.0</td></tr><tr><td>2025-01-08</td><td>71.906395</td><td>366.005035</td><td>241.607254</td><td>71.906395</td><td>...</td><td>240.830767</td><td>18019505.0</td><td>95456188.0</td><td>37628900.0</td></tr><tr><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td></tr><tr><td>2025-06-23</td><td>90.441856</td><td>504.000000</td><td>201.076660</td><td>90.638894</td><td>...</td><td>201.206392</td><td>11873908.0</td><td>14252887.0</td><td>55814300.0</td></tr><tr><td>2025-06-24</td><td>92.707825</td><td>509.500000</td><td>199.879181</td><td>93.101908</td><td>...</td><td>202.164363</td><td>29990019.0</td><td>17768858.0</td><td>54064000.0</td></tr><tr><td>2025-06-25</td><td>94.185631</td><td>512.500000</td><td>201.136536</td><td>95.367882</td><td>...</td><td>201.026766</td><td>36558811.0</td><td>17592461.0</td><td>39525700.0</td></tr><tr><td>2025-06-26</td><td>93.988586</td><td>513.000000</td><td>200.577698</td><td>94.481189</td><td>...</td><td>201.006787</td><td>10230801.0</td><td>15000643.0</td><td>50799100.0</td></tr><tr><td>2025-06-27</td><td>93.594505</td><td>513.000000</td><td>200.657532</td><td>93.840807</td><td>...</td><td>201.465827</td><td>14778679.0</td><td>15181667.0</td><td>73188600.0</td></tr></table>

# Other attributes

import yfinance as yf

msft = yfTicker("MSFT")

get all stock info

msft.info

get historical market data

hist = msft_history(period="1mo")

show meta information about the history (requires history() to be called first)  
msft_history_metadata

show actions (dividends, splits, capital gains)

msft/actions

msft-dividends

msft.splits

msft.capital_gains

only for mutual funds & etfs # show share count

msft.get_shares_full(start="2022-01-01", end=None)

show financials:

- income statement

msft.incomestmt

msftquarterly_incomestmt

balance sheet

msft balancesheet

msftquarterly_balance sheet

-cash flow statement

msft cashflow

msftquarterly_cashflow

see `Ticker.get_incomestmt()` for more options

show holders

msft major holders

msft.institutional HOLDERS

msft.mutualfund HOLDERS

msft.insider_transactions

msft.insider_purchases

msft insider roster holders

show recommendations

msft.recommendations

msft.recommendations_summary

msft.upgrades_downgrades

Show future and historic earnings dates, returns at most next 4 quarters and last 8 quarters by default.

Note: If more are needed use msft.get_Earnings_dates limit=XX) with increased limit argument.

msft.earnings_dates

show ISIN code - *experimental*

ISIN = International Securities Identification Number

msft.isin

show options expirations

msftoptions

shownews

msft.news

get option chain for specific expiration

opt = msft.option_chain('YYYY-MM-DD')

data available via: optcalls, opt.puts

# Selenium

- Selenium is a web automation tool that allows you to simulate user interactions, perform web scraping, and run automated tests on web applications.   
- Official document: https://selenium-python.readthedocs.io/

![](images/a30f86fba08fa30f4b4f345f57f8b1423eaf2896c8fdbb0b5005bdfcff235c85.jpg)

# Webdrive download

- Chrome: https://googlechromelabs.github.io/chrome-for-testing/#stable   
- Edge: https://developer.microsoft.com/en-us/microsoft-edge/tools/webdriver?form=MA13LH   
- Firefox: https://github.com/mozilla/geckodriver/releases

# Quick Example

from selenium import webdriver

from selenium.webdriver.chrome.service import Service

chrome.service = Service executable_path='path/to/chromedriver')

driver = webdriverchrome(service=chrome.service)

driver.get('http://example.com')

print(driver.title)

driver.quit()

![](images/849155d10e110bd20730c47b8e24c18d774a16be777010d5098c6865c401f20d.jpg)

DevTools listening on ws://127.0.0.1:49236/devtools/browser/f14841af-84e0-4681-9cf4eb2b55f3e68c Example Domain

![](images/d914bcabdd67d558975e417d6aebfc5ba9c3f2346c44a9aad8388f8786bebea3.jpg)

![](images/a1bd1b5e30f199e8036f9b015ef98919f0ade8dfbde9d3095c4f1f830cd0875c.jpg)

![](images/4c822937ad176446b27cbeed6756c2c389b962efed266d17ff268e0c217e245d.jpg)

![](images/4e8d987c19722fad1998fcc97acfc90cbccd56b68dac43ca94fcaf32bba7fef2.jpg)

![](images/444bcf4fdabc545fffa5e7966859c76e1fbf404696960d8878b2bc5c78698960.jpg)

![](images/299561f5e82a48c87d6312a1dde3b19784a2cb62f8dcdc275bcf69a5833574c3.jpg)

example.com

![](images/f458f088683f4938303ef4b1e0145f759887e2ce2c49dcbbe1ca28690342f4a0.jpg)

![](images/673ae2e31321c91e97cff212d83e9eb678b36e66dd886990cbd8393591b7395b.jpg)

# Example Domain

This domain is for use in illustrative examples in documents. You may use the domain in literature without prior coordination or asking for permission.

More information...

![](images/b58395ca996d55a4a1cf76cad574556541442251fb44f6174b15d37f188c0035.jpg)

![](images/e09922e3106448df6d1c2c1b7e3ada39f3e27d73b3a034bc5d5ece6e1df4d854.jpg)

![](images/edc06ede91432de4629cf1c723952b0e991cd5585cec90586782d3fc74a78ffc.jpg)

![](images/76b4618c8e1c922f3837cf93ff096b70fa332d500b8b58642c66ea46e76da556.jpg)

![](images/50f5fe132b3c6c9b47e6372dddbf6d4369c51a097fb8262595b958bf051e5a64.jpg)

![](images/494f58d5ab359b208b103067dbb39853cb0c0a49943290e01375ab5817052c0c.jpg)

![](images/bb414033ed3fd53d7efde7af14e61ef484238007f6311ecbce46dcda39db95b1.jpg)

![](images/ad54a95d22ec02ea32b00b261bda56509dc326a36b298724283537439f9c8b84.jpg)

![](images/ae95ba9a69382262ff460a8c5e98eaa0e2b8a031efbc8f329e3f369c6bc4ef9c.jpg)

![](images/be4c6b284bea8ef0bdbe6f65a2a4a9ca668db7e990ddc603a700870a8ab4cac9.jpg)

![](images/afbdbb3a42f99a20ea5208cd4e6736ed119eb8ab74c08d711179207dc37d56e3.jpg)

![](images/c682dbed9f8be465a7b1b23b2d65ce9d05bb85d4f4875820a64af2de17183bdc.jpg)

![](images/3f1fc43c9433a091c5817817dae83344ca28e633dd9809680411c3b2666825c1.jpg)

![](images/d6efa3cf6b992d8455bcad6bca1ddaf6cf484aa00ce94290c006b425a8c04f10.jpg)

![](images/88f3e72b6a29a9e86c160123932c70c58753c5836b5742c180f3d89968e59685.jpg)

![](images/73b3b96e2ad1474db08822bea1679fe8a1bc74576a65e3d5756748ac514d7980.jpg)

![](images/923a7ee6bda0c593922145ca9af521c12b93b645d05c2786e023cc7d15f3b9c3.jpg)

# yahoo'finance

Search for news, symbols or companies

![](images/f3c3a5aa71606fe23029594fc1d87445be1c97f1e66a1f067c663c29f3582034.jpg)

News

Finance

Sports

More

My Portfolio

News

Markets

Sectors

Screeners

Personal Finance

Videos

Summary

HKSE -Delayed Quote·HKD

# Tencent Holdings Limited (0700.HK)

☆Follow

$\rightarrow^{\dagger}$ Compare

News

Chart

Conversations

Statistics

Historical Data

Profile

Financials

Analysis

Options

Holders

Sustainability

# 365.000 -7.400 (-1.99%)

As of 9:15 AM GMT+8. Market Open.

Jul 02, 2023 - Jul 02, 2024

Historical Prices

Daily

# Start Trading >>

Plus500 CFD Service Your capital is at risk.

Currency in HKD

![](images/fc7a5dcb744a9caab8c0ceba719390b532226c0b9941b8589b80bbe8b339a927.jpg)

Download

table.table.svelte-ewueuo 1012.4 x 7904   

<table><tr><td>Date</td><td>Open</td><td>High</td><td>Low</td><td>Close ①</td><td>Adj Close ①</td><td>Volume</td></tr><tr><td>Jul 2, 2024</td><td>371.600</td><td>376.000</td><td>370.800</td><td>365.000</td><td>365.000</td><td>15,561,097</td></tr><tr><td>Jun 28, 2024</td><td>371.600</td><td>376.000</td><td>370.800</td><td>372.400</td><td>372.400</td><td>15,561,097</td></tr><tr><td>Jun 27, 2024</td><td>379.600</td><td>380.000</td><td>372.200</td><td>374.400</td><td>374.400</td><td>17,513,758</td></tr><tr><td>Jul 26, 2024</td><td>379.600</td><td>380.000</td><td>377.000</td><td>382.000</td><td>382.000</td><td>11,996,997</td></tr></table>

![](images/9f407602c5ad3ab78a3f4a2488859b3d80d4e6c4ad6bab40700e69595e997049.jpg)

![](images/b17aaea813cc9065c3ef5501587224bdbc2cec3b0e57d1c66c38765a6a61c9b3.jpg)

Elements

Console

Sources

Network

Performance

Memory

Application

Lighthouse

Recorder

Performance insights

![](images/bdc5e772bc63c35625cd6ca74a70e214f377e5c0d7d6681693f2e045f5a652d0.jpg)

# Extract stock price

![](images/f28f7c3f19fb0f69774684ce61444b4e0b5ec8959da3b95d7745c48d99193bfa.jpg)

<table><tr><td></td><td></td><td>Date</td><td>Open</td><td>High</td><td>Low</td><td>Close</td><td>Adj.</td><td>Close</td><td>Volume</td></tr><tr><td>0</td><td>Jan</td><td>21, 2026</td><td>598.000</td><td>605.500</td><td>598.000</td><td>601.000</td><td>601.000</td><td>12,012,950</td><td></td></tr><tr><td>1</td><td>Jan</td><td>20, 2026</td><td>601.000</td><td>607.000</td><td>598.000</td><td>601.000</td><td>601.000</td><td>24,332,228</td><td></td></tr><tr><td>2</td><td>Jan</td><td>19, 2026</td><td>613.500</td><td>614.500</td><td>608.500</td><td>610.000</td><td>610.000</td><td>13,233,330</td><td></td></tr><tr><td>3</td><td>Jan</td><td>16, 2026</td><td>627.000</td><td>630.000</td><td>613.500</td><td>617.500</td><td>617.500</td><td>20,616,793</td><td></td></tr><tr><td>4</td><td>Jan</td><td>15, 2026</td><td>631.000</td><td>632.500</td><td>618.500</td><td>622.000</td><td>622.000</td><td>26,194,252</td><td></td></tr><tr><td>...</td><td></td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td></tr><tr><td>242</td><td>Jan</td><td>27, 2025</td><td>388.000</td><td>398.000</td><td>388.000</td><td>395.600</td><td>392.177</td><td>23,605,938</td><td></td></tr><tr><td>243</td><td>Jan</td><td>24, 2025</td><td>383.200</td><td>392.400</td><td>382.200</td><td>390.600</td><td>387.220</td><td>24,079,119</td><td></td></tr><tr><td>244</td><td>Jan</td><td>23, 2025</td><td>383.000</td><td>386.000</td><td>379.600</td><td>381.200</td><td>377.901</td><td>21,256,810</td><td></td></tr><tr><td>245</td><td>Jan</td><td>22, 2025</td><td>385.000</td><td>388.600</td><td>381.000</td><td>383.400</td><td>380.082</td><td>17,859,069</td><td></td></tr><tr><td>246</td><td>Jan</td><td>21, 2025</td><td>392.000</td><td>392.200</td><td>386.400</td><td>387.400</td><td>384.048</td><td>25,249,668</td><td></td></tr></table>

[247 rows x 7 columns]

```python
from selenium import webdriver   
from selenium.webdriver.chrome.service import Service   
from selenium.webdriver(common.by import By   
from selenium.webdriver.common.keys import Keys   
from selenium.webdriver.support.ui import WebDriverWait   
from selenium.webdriver.support import expected_conditions as EC   
from bs4 import BeautifulSoup   
import time   
import pandas as pd 
```

# Path to your WebDriver (adjust the path to where you downloaded the WebDriver)  
webdriver_path = 'D:/program/chromedriver.exe'

Set up the WebDriver   
service $=$ Service(webdriver_path)   
driver $=$ webdriver.Chrome(service $\equiv$ service)   
url $=$ 'https://finance.yahoo.com/quote/0700.HK/history/'   
driver.get(url)   
#Wait for the page to load   
WebDriverWait(driver,10).until( ECpresence_of_element Located((By.CSS_SELECTOR,'table[class $\coloneqq$ "table yf-1m2i7s2 noDI hideOnPrint"]))

```python
Scroll down to load more data if necessary  
# Adjust the range for more scrolling, if required  
for _ in range(3):  
    driver_find_element(By.TAG_NAME, 'body').send_keys(Keys.END)  
time.sleep(2) 
```

```python
Wait for the new data to load # Get the page source and parse it with BeautifulSoup  
page_source = driver.page_source  
soup = BeautifulSoup(page_source, 'html.parser') 
```

```python
Find the historical data table  
table = soup.find('table', {'class': 'table yf-1jecxey noDI hideOnPrint'})  
rows = table.find_all('tr') 
```

```python
Extract and print the data  
data = []  
for row in rows[1]: # Skip the header row  
cols = row.find_all("td")  
if len(cols) > 6:  
    date = cols[0].text  
    open_price = cols[1].text  
    high_price = cols[2].text  
    low_price = cols[3].text  
    close_price = cols[4].text  
    adj_close_price = cols[5].text  
    volume = cols[6].text  
    data.append([date, open_price, high_price, low_price, close_price, adj_close_price, volume]) 
```

```python
# crate data frame
df = pd.DataFrame(data, columns=['Date', "Open", "High", "Low", "Close", "Adj. Close", "Volume']) print(df) 
```

```txt
close driver  
driver.quit() 
```

# Selenium Options

- https://www.selenium.dev/selenium/docs/api/rb/Selenium/WebDriver/Chromium/Options.html#add argument-instance method   
- headless: open browser in background   
- add_emulation: emulate opening the browser in different device

from selenium import webdriver from selenium.webdriver.chrome/options import Options

options $=$ Options() options.add_argument('--headless $\coloneqq$ new')

driver = webdriverchrome(CHROMEDRIVER_PATH, options=options)

# Wrap up

Several Python libraries are introduced for web scraping

- BeautifulSoup   
- xml   
- Selenium   
- yfinance - specific for Yahoo Finance

# Limitation of Data Scraping

# 1. Data Quality:

- Website data may be incomplete and not up-to-dated

# 2. Resource Intensive:

- Consume lots of bandwidth and storage when scraping large amount of data   
- Time consuming to fully load a website content to extract the data

# 3. Technical Challenges:

- Website structure may be changed frequently   
- Many websites may implement anti-bot measures (eg. IP blocking, rate limit, etc)

# Database Management

# Why use a database for algo-trading?

- Efficient storage and retrieval of large amounts of historical data for strategy backtesting   
- Keep data in a local disk for future analysis instead of repeated web scraping   
Data integrity and consistency   
- Advanced querying capabilities

# SQL Syntax Overview

- SELECT: Retrieve data from a table

SELECT * FROM users;

- INSERT: Add new data to a table

INSERT INTO users (name, age) VALUES ('Amy', 18);

- UPDATE: Modify existing data in a table

UPDATE users SET age=28 WHERE name='Amy';

- DELETE: Remove data from a table

DELETE FROM users WHERE name='Amy';

# Using SQLite in Python

- SQLite is a C library that provides a lightweight, disk-based database.   
- Python has built-in support for SQLite.

![](images/9942a69e86925fd549c11c66b65a717914b9a5cab080f4d5a509d5c47b4bc56d.jpg)

# Basic Usage

import sqlite3

Connect to a database (or create one if it doesn't exist) conn = sqlite3.connect('example.db')

Create a cursor object  
cursor = conn.cursor()

Execute SQL queries using the cursor object cursor.exec('SELECT * FROM users')

Fetch all results rows = cursor].[fetchall()

Close the connection conn.close()

# Create a Table

# SQL Syntax

CREATE TABLE users (
id INTEGER PRIMARY KEY, name TEXT NOT NULL, age INTEGER);

# Python Example

import sqlite3   
conn $=$ sqlite3.connect('example.db') cursor $=$ conn.cursor()   
#Create a table cursor.execute(" CREATE TABLE users ( id INTEGER PRIMARY KEY, name TEXT NOT NULL, age INTEGER ) ") conn.commit() conn.close()

# Inserting Data

# SQL Syntax

INSERT INTO users (name, age) VALUES ('Alice', 30);

# Python Example

conn = sqlite3.connect('example.db')

cursor = conn.cursor()

Insert data

cursor.execute(

INSERT INTO users (name, age) VALUES (?, ?)

```
'', ('Alice', 30))

conn.commit()

conn.close()

# Querying Data

# SQL Syntax

SELECT * FROM users WHERE age > 25;

# Python Example

conn = sqlite3.connect('example.db')  
cursor = conn.cursor()

Query data  
cursor.exec('SELECT * FROM users WHERE age > 25')  
rows = cursor_fetchall()

for row in rows: print(row)

conn.close()

# Updating Data

# SQL Syntax

UPDATE users SET age = 31 WHERE name = 'Alice';

# Python Example

conn = sqlite3.connect('example.db')

cursor = conn.cursor()

Update data

cursor.execute(

UPDATE users SET age = ? WHERE name = ?

```
'', (31, 'Alice'))

conn.commit()

conn.close()

# Deleting Data

# SQL Syntax

DELETE FROM users WHERE name = 'Alice';

# Python Example

conn = sqlite3.connect('example.db')

cursor = conn.cursor()

Delete data

cursor.execute(

DELETE FROM users WHERE name = ?

```
'', ('Alice'),))

conn.commit()

conn.close()

# Querying Data with table join

Suppose we want to know the total transactions done by each client.

SELECT A.id, A.name, count(B.*)

FROM users A

INNER JOIN transactions B ON A.id = B.user_id

GROUP BY A.id, A.name;

# users

id (Primary Key)

name (e.g., NYSE, NASDAQ)

age

# transactions

txn_id (Primary Key)

user_id

trade_time

Instrument

buysell

price

qty

# SQLite Browser

- SQLite Browser (https://sqlitebrowser.org/) is an open sourced tool to create, edit, search and visualize SQLite database files.

![](images/a9c7cf6348381ba398b8ca365feb73e2ab37bc5427854b2d4b2b554e02ac09c9.jpg)

# Scrape Market Data into database

importsqlite3

import yfinance as yf

conn =sqlite3.connect('D:/example.db')

cursor = conn.cursor()

create table

cursor.execute(

CREATE TABLE market_candles (

symbol text,

timestamp text,

open_price float

high_price float,

low_price float,

close_price float,

volume float

）

# download data from yahoo finance

symbol = "0700.HK"

```c
ticket = yf.Tickersymbol()

data = ticker_history(period='max')

save to database

for index, row in data.iterrrows():

t = index

o = row['Open']

h = row['High']

$\mathbf{l} = \mathrm{row}[L_{\mathrm{ow}}]$

c = row['Close']

v = row['Volume']

sql = ""INSERT INTO market_candles VALUES {'symbol'},{'t'},{'o'},{'h'},{'l'},{'c'},{'v'})""."format(symbol=symbol,t=t,o=o,h=h,l=l,c=c,v=v)

cursor execute(sql)

close database

conn.commit()

conn.close()

# Querying market data from database

![](images/6729b25b1fbb0f773fce5e015831b0c2b2da9c0fffa055c9976b383dc1669f8e.jpg)

![](images/948c4c98456fede4289f0d7b50fe1f233c008d8a71485661e1369e694bc3f370.jpg)

![](images/861d6153e0ade541777e6cf0d3c9a068927e8dd99188a2af9f7e40c906ef457d.jpg)

![](images/3962a561dcfcfa7220f9ed342be6a3098bba04e1baa5349089f91ccbc23154f8.jpg)

![](images/58d5b3c2e7c12cbf2fdc9953c883272ba185abb6ac44501151766c207cfb7c48.jpg)

![](images/5a2a7f00b12130bc5741dd3cddf752aec6fe4a2a40b66d4901a6612f9a5d1c38.jpg)

![](images/0d240ce45fff12e753bdf8b551ba88620477d6abd94f7ee02179f7e067b0b66f.jpg)

```csv
('0700.HK', '2025-06-02 00:00:00+08:00', 493.0, 499.20001220703125, 489.6000061035156, 498.3999938964844, 13086586.0)  
('0700.HK', '2025-06-03 00:00:00+08:00', 504.0, 505.5, 501.0, 505.0, 14609189.0)  
('0700.HK', '2025-06-04 00:00:00+08:00', 510.0, 513.0, 507.0, 512.0, 18378499.0)  
('0700.HK', '2025-06-05 00:00:00+08:00', 517.5, 517.5, 509.0, 515.0, 19329282.0)  
('0700.HK', '2025-06-06 00:00:00+08:00', 515.5, 516.5, 511.0, 515.0, 13137101.0)  
('0700.HK', '2025-06-09 00:00:00+08:00', 520.0, 521.0, 512.5, 518.0, 18481403.0)  
('0700.HK', '2025-06-10 00:00:00+08:00', 519.5, 520.0, 508.5, 513.5, 17039003.0)  
('0700.HK', '2025-06-11 00:00:00+08:00', 517.0, 518.0, 514.5, 518.0, 14784979.0)  
('0700.HK', '2025-06-12 00:00:00+08:00', 518.0, 518.0, 508.0, 510.0, 13368O48. O)  
('O7OO.HK', '2O25-OS-13 Oo:Oo:Oo+Oo:Oo', 51O.5, 5I5.5, 5O6. O, 5IO. O, 19O8S31O.O)  
('O7OO.HK', '2O25-OS-16 Oo:Oo:Oo+Oo:Oo', 5O7. O, 5I2. O, 5O4.5, 5O9.5, 1378432O.O)  
('O7OO.HK', '2O25-OS-17 Oo:Oo:Oo+Oo:Oo', 5I4. O, 5I4. O, 5O6.5, 5I3.5, I1S24O31.O)  
('O7OO.HK', '2O25-OS-18 Oo:Oo:Oo+Oo:Oo', 5Ioo. O, 5Ii. O, 5O3.5, 5O8. O, I5I87S1S.O)  
('O7OO.HK', '2O25-OS-19 Oo:Oo:Oo+Oo:Oo', 5O3. O, 5O6. O, 496. O, 498. O, I9387767.O)  
('O7OO.HK', '2O25-OS-2O Oo:Oo:Oo+Oo:Oo', 5O4.5, 5OIS. O, 496. O, SIS. O, 2O62227S.O)  
('O7OO.HK', '2O25-OS-23 Oo:Oo:Oo+Oo:Oo', 498.2OOU122OT312S, SIO4.O, 494.6OOU61OT3S1S6, SIO4.O, I42S2887.O)  
('O7OO.HK', '2O25-OS-24 Oo:Oo:Oo+Oo:Oo', 5O8.O, 5Ii.I,O4.O, SIO9.S, I77688S8.O)  
('O7OO.HK', '2O25-OS-2S Oo:Oo:Oo+Oo:Oo', 5Ii.o., Iiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., I 
```

importsqlite3

```python
conn = sqlite3.connect('D:/example.db')  
cursor = conn.cursor() 
```

query table

```txt
sql = ""SELECT * 
```

FROM market_candles

```txt
WHERE symbol='0700.HK' AND timestamp>'='2025-06-01' AND timestamp'<='2025-06-30' 
```

ORDER BY timestamp;

```txt
for row in cursor.exec (sql): print (row) 
```

```txt
close database conn.close() 
```

# Close vs Adjusted Close

![](images/f2f1c716a05a505ac435a34535151e3a678335ba52e21bf9af46969dd402fe1e.jpg)

# Close vs Adjusted Close

- The original price series may show sudden jumps or drops due to corporate events (e.g. dividends, stock splits, reverse splits).   
- After a dividend payment, the stock price typically drops by $S' = S - D$   
- Following a stock split (e.g. 2:1 split), the stock price decreases by a factor of the split ratio (i.e. $S' = S / 2$ )   
- Adjusted close prices are backward updated to account for these corporate events

- so the values of the adjusted close series may change from time to time

# How Yahoo Finance adjust data?

![](images/d0e9e60f1c68af21f4e68f878545456865fd740a43cbab5f35e3cd0ec0439940.jpg)

# Multipliers

Split multipliers are determined by the split ratio.

For example:

In a 2 for 1 split, the pre-split data is multiplied by 0.5.   
In a 4 for 1 split, the pre-split data is multiplied by 0.25.   
- In a 1 for 5 reverse split, the pre-split data is multiplied by 5.

Dividend multipliers are calculated based on dividend as a percentage of the price, primarily to avoid negative historical pricing.

For example:

- If a $0.08 cash dividend is distributed on Feb 19 (ex- date), and the Feb 18 closing price is$ 24.96, the predividend data is multiplied by (1-0.08/24.96) = 0.9968.   
- If a $2.40 cash dividend is distributed on May 12 (ex- date), and the May 11 closing price is$ 16.51, the predividend data is multiplied by (1-2.40/16.51) = 0.8546.   
- If a $1.25 cash dividend is distributed on Jan 25 (ex-date), and the Jan 24 closing price is$ 51.20, the predividend data is multiplied by (1-1.25/51.20) = 0.9756.

Here's how split and dividend multipliers are calculated and applied to determine adjusted close prices.

The first table shows historical prices and the dates of a split and a dividend. The second table shows the calculations.

The multipliers we'll use are derived from the split and dividend:

- Split Multiplier = 0.5   
Dividend Multiplier $= 1 - (0.08 / 24.96) = 0.9968$

Using these split and dividend multipliers, adjusted close prices are calculated for dates prior to the split and prior to the dividend ex-date.

The close price for 2/16 and 2/17 are adjusted for both the split on 2/18 and the ex-dividend date, 2/21.   
The close price for 2/18 through 2/20 are adjusted for the ex-dividend date, 2/21.   
- The close price for 2/18 through 2/20 don't need adjustment for the split that occurred before close on 2/18.   
The close price for 2/21 and 2/22 aren't adjusted because they're after the split and ex-dividend dates.

<table><tr><td>Date</td><td>Close, Dividend, or Split</td></tr><tr><td>2/16</td><td>Close = 46.99</td></tr><tr><td>2/17</td><td>Close = 48.30</td></tr><tr><td>2/18</td><td>Split = 2:1
Close = 24.96</td></tr><tr><td>2/19</td><td>Close = 24.91</td></tr><tr><td>2/20</td><td>Close = 24.95</td></tr><tr><td>2/21</td><td>Dividend = 0.08 (ex-date)
Close = 24.53</td></tr><tr><td>2/22</td><td>Close = 24.54</td></tr></table>

<table><tr><td>Date</td><td>Adjusted close calculation</td></tr><tr><td>2/16</td><td>0.5 * 0.9968 * 46.99 = 23.42</td></tr><tr><td>2/17</td><td>0.5 * 0.9968 * 48.30 = 24.07</td></tr><tr><td>2/18</td><td>0.9968 * 24.96 = 24.88</td></tr><tr><td>2/19</td><td>0.9968 * 24.91 = 24.83</td></tr><tr><td>2/20</td><td>0.9968 * 24.95 = 24.87</td></tr><tr><td>2/21</td><td>24.53</td></tr><tr><td>2/22</td><td>24.54</td></tr></table>

# Any adjustments on OHL data?

![](images/ae583c8e47d60a25a2a6843a30b3a19478a511f62f8c5b40336f21e07b50b709.jpg)

finance.yahoo.com/quote/AAPL/history/?period1=345479400&period2=1738642197

# yahoo finance

Search for news, symbols or companies

![](images/38fec4f456782dc74b68eba9fdf4545a8c97efcd8403707f3bc784d88fad7101.jpg)

News

Finance

Sports

Mg

My Portfolio

News

Markets

Research

Personal Finance

Videos

Streaming Now

# AAPL

Apple Inc.

![](images/ec081348c0e94ab176e7ab788ca3e4084105f4a49ca43405f19b6cfcdb93a5d8.jpg)

228.85 +0.37% (

Summary

News

Chart

Conversations

Statistics

Historical Data

Profile

Financials

Analysis

<table><tr><td>Sep 10, 2020</td><td>120.36</td><td>120.50</td><td>112.50</td><td>113.49</td><td>110.79</td><td>182,274,400</td></tr><tr><td>Sep 9, 2020</td><td>117.26</td><td>119.14</td><td>115.26</td><td>117.32</td><td>114.52</td><td>176,940,500</td></tr><tr><td>Sep 8, 2020</td><td>113.95</td><td>118.99</td><td>112.68</td><td>112.82</td><td>110.13</td><td>231,366,600</td></tr><tr><td>Sep 4, 2020</td><td>120.07</td><td>123.70</td><td>110.89</td><td>120.96</td><td>118.08</td><td>332,607,200</td></tr><tr><td>Sep 3, 2020</td><td>126.91</td><td>128.84</td><td>120.50</td><td>120.88</td><td>118.00</td><td>257,599,600</td></tr><tr><td>Sep 2, 2020</td><td>137.59</td><td>137.98</td><td>127.00</td><td>131.40</td><td>128.27</td><td>200,119,000</td></tr><tr><td>Sep 1, 2020</td><td>132.76</td><td>134.80</td><td>130.53</td><td>134.18</td><td>130.98</td><td>151,948,100</td></tr><tr><td>Aug 31, 2020</td><td colspan="6">4:1 Stock Splits</td></tr><tr><td>Aug 31, 2020</td><td>127.58</td><td>131.00</td><td>126.00</td><td>129.04</td><td>125.97</td><td>225,702,700</td></tr><tr><td>Aug 28, 2020</td><td>126.01</td><td>126.44</td><td>124.58</td><td>124.81</td><td>121.83</td><td>187,630,000</td></tr><tr><td>Aug 27, 2020</td><td>127.14</td><td>127.49</td><td>123.83</td><td>125.01</td><td>122.03</td><td>155,552,400</td></tr></table>

# Should we keep the original or the adjusted price in database?

# Pros:

- Adjusted close provides a more accurate reflection of a stock's value over time   
- Adjusted close is ideal for long-term analysis and backtesting strategies, as it smoothes out price jumps from events   
- Ensure consistency across datasets, especially when comparing stocks with varying corporate events.

# - Cons:

- Storing both may require more space   
- Need to regularly update the whole adjusted close series for new events.   
- Adjusted close is not the actual price observed in the market. Modelling on adjusted close could be inaccurate.

# Database Design

- Suppose you are going to build a database to store all market data history from global stocks exchanges. How would you structure the database?

# Database Design (1)

![](images/578eed21aa6bba80bd01e1ba2968938dfc8439474f4e8f3359d5320438b7c70a.jpg)

# Potential issues for design 1

- The table size of “Market Data Table” will be very large as all stock data is put under the same table   
- Data backup and data query speed from "Market Data Table" could be very slow

# Database Design (2)

Split the Market Data Table into monthly or quarterly partitions based on the date field.

![](images/7c7a327696fbd2d7a195d588f8e398b32d3148a6fec77808362a25b91b1b46b7.jpg)

# Potential issues for design 2

- Difficult to conduct data aggregation/summary for cross month analysis   
- Need a function to generate dynamic SQL statements due to different market data table name

# Database Design (3)

Split the Market Data Table into partitions based on the stock_id field.

![](images/8d4b414d2659d1b0efc96d82a32e990c9fdca1eda08f9bc71d988f266d23cc8c.jpg)

# Potential issues for design 3

- Difficult for cross assets analysis   
- Need a function to generate dynamic SQL statements due to different market data table name

# What would be the best design?

![](images/0d89f10494a5432d07885e1a9957ccdf165637628352265b40c375f8ce6e3086.jpg)

# Database Design

- There is no single design that is perfect for all situations!   
- It depends on our common use cases.

- For backtest purpose, partition by instruments could be a good choice   
- For data analysis purpose, partition by date could be a better design

# Database Design

- Always need to strike a balance between

Storage Space

![](images/c4ce72002946ca513ab3ed3339b91214de8b1cb656ed49c2b2b56cf4deca6be7.jpg)

![](images/a620dde1fa17859840e7e233580433ea0febf527595c01fa3af191568b8fbc89.jpg)

Runtime Speed

![](images/05dd869a766612222592c05353018cb3ac427b9f47b01a91e061c4ec6c3d6f97.jpg)

Memory

# Key Takeaways

- Learn about the basic structure of a website (i.e. HTML, CSS, Javascript)   
- Apply various python libraries for web scraping

- BeautifulSoup   
- xml   
- Selenium   
- yfinance

- Understand the limitations of web scraping   
- Learn how to create and manage a SQLite database using python   
- Considerations for design a database for financial market data
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
