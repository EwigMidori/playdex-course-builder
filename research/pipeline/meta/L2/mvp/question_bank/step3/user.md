请基于以下 lesson 材料，生成一个结构化题库 JSON。

目标语言：
zh-CN

lesson_id：
L2

要求：
- 同时生成 `flashcard_families`、`quiz_families` 和 `longform_families`
- 题目必须只关联到当前 step：`step3`
- 所有 family 和 variant 的 `linked_steps` 都必须等于 `["step3"]`
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
      "concept": "What is web scraping and why it matters for algorithmic trading",
      "file": "research/pipeline/3-guided_story/L2/step1/step.json",
      "sequence_id": "step1"
    },
    {
      "concept": "Understanding the basic structure of a website: HTML, CSS, and JavaScript",
      "file": "research/pipeline/3-guided_story/L2/step2/step.json",
      "sequence_id": "step2"
    },
    {
      "concept": "Using BeautifulSoup to parse HTML and extract data",
      "file": "research/pipeline/3-guided_story/L2/step3/step.json",
      "sequence_id": "step3"
    },
    {
      "concept": "Real-world scraping: S&P 500 list and handling anti-bot measures",
      "file": "research/pipeline/3-guided_story/L2/step4/step.json",
      "sequence_id": "step4"
    },
    {
      "concept": "Using yfinance for reliable financial data access",
      "file": "research/pipeline/3-guided_story/L2/step5/step.json",
      "sequence_id": "step5"
    },
    {
      "concept": "Storing scraped data in a SQLite database",
      "file": "research/pipeline/3-guided_story/L2/step6/step.json",
      "sequence_id": "step6"
    },
    {
      "concept": "Database design considerations for financial data",
      "file": "research/pipeline/3-guided_story/L2/step7/step.json",
      "sequence_id": "step7"
    },
    {
      "concept": "Limitations of web scraping and alternative tools",
      "file": "research/pipeline/3-guided_story/L2/step8/step.json",
      "sequence_id": "step8"
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
      "concept": "What is web scraping and why it matters for algorithmic trading",
      "file": "research/pipeline/3-guided_story/L2/step1/step.json",
      "sequence_id": "step1"
    },
    {
      "concept": "Understanding the basic structure of a website: HTML, CSS, and JavaScript",
      "file": "research/pipeline/3-guided_story/L2/step2/step.json",
      "sequence_id": "step2"
    },
    {
      "concept": "Using BeautifulSoup to parse HTML and extract data",
      "file": "research/pipeline/3-guided_story/L2/step3/step.json",
      "sequence_id": "step3"
    },
    {
      "concept": "Real-world scraping: S&P 500 list and handling anti-bot measures",
      "file": "research/pipeline/3-guided_story/L2/step4/step.json",
      "sequence_id": "step4"
    },
    {
      "concept": "Using yfinance for reliable financial data access",
      "file": "research/pipeline/3-guided_story/L2/step5/step.json",
      "sequence_id": "step5"
    },
    {
      "concept": "Storing scraped data in a SQLite database",
      "file": "research/pipeline/3-guided_story/L2/step6/step.json",
      "sequence_id": "step6"
    },
    {
      "concept": "Database design considerations for financial data",
      "file": "research/pipeline/3-guided_story/L2/step7/step.json",
      "sequence_id": "step7"
    },
    {
      "concept": "Limitations of web scraping and alternative tools",
      "file": "research/pipeline/3-guided_story/L2/step8/step.json",
      "sequence_id": "step8"
    }
  ]
}

</GUIDED_STORY_MANIFEST>

<GUIDED_STORY_STEPS>
[
  {
    "lesson_id": "L2",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [],
        "id": "s001",
        "introduced_terms": [],
        "lines": [
          "想象一下，你每天需要手动查看几百只股票的价格、新闻和财报。",
          "这几乎不可能，而且很容易出错。"
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
          "现在，让程序替你完成这些重复的工作。",
          "这就是 <term id=\"web_scraping\">网络爬取</term> 的核心思想。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "algorithmic_trading"
        ],
        "id": "s003",
        "introduced_terms": [
          "algorithmic_trading"
        ],
        "lines": [
          "在 <term id=\"algorithmic_trading\">算法交易</term> 中，数据就是燃料。",
          "没有数据，策略就是空谈。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s004",
        "introduced_terms": [],
        "lines": [
          "爬取的数据可以用于：",
          "实时获取股价、经济指标，分析市场情绪，甚至监控天气数据。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "网络爬取正是为了自动从网站提取数据而设计的。",
          "kind": "single_choice",
          "options": [
            "手动计算股票平均价格",
            "自动从财经网站收集每日收盘价",
            "用 Excel 绘制图表",
            "编写交易策略的代码逻辑"
          ],
          "prompt": "下面哪种情况最适合使用网络爬取？"
        },
        "focus_terms": [],
        "id": "s005",
        "introduced_terms": [],
        "lines": [
          "下面哪种情况最适合使用网络爬取？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step1",
    "source": {
      "plain_text": "What's Data/Web Scraping? Automated process to extract data from websites. Common use cases in algo-trading: Real-time data collection, Market sentiment analysis, Corporate analysis, Alternative data.",
      "related": [
        "L2: Data scraping and database management with Python"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "algorithmic_trading": {
        "aliases": [
          "Algorithmic Trading",
          "Algo Trading"
        ],
        "display": "算法交易",
        "gloss": "用规则和程序处理交易判断与执行的方式。"
      },
      "web_scraping": {
        "aliases": [
          "Web Scraping",
          "Data Scraping"
        ],
        "display": "网络爬取",
        "gloss": "从网站自动提取数据的过程。"
      }
    }
  },
  {
    "lesson_id": "L2",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [],
        "id": "s006",
        "introduced_terms": [],
        "lines": [
          "要爬取数据，首先得理解网页的骨架。",
          "一个网页通常由三部分组成。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "html"
        ],
        "id": "s007",
        "introduced_terms": [
          "html"
        ],
        "lines": [
          "第一部分：<term id=\"html\">HTML</term>。",
          "它定义了网页的结构和内容，比如标题、段落、表格。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "css"
        ],
        "id": "s008",
        "introduced_terms": [
          "css"
        ],
        "lines": [
          "第二部分：<term id=\"css\">CSS</term>。",
          "它负责控制网页的样式，比如颜色、字体、布局。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "javascript"
        ],
        "id": "s009",
        "introduced_terms": [
          "javascript"
        ],
        "lines": [
          "第三部分：<term id=\"javascript\">JavaScript</term>。",
          "它让网页变得动态和可交互，比如点击按钮后弹出信息。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s010",
        "introduced_terms": [],
        "lines": [
          "爬取数据时，我们最关心的是 HTML。",
          "因为数据通常就藏在 HTML 的标签里。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 2,
          "explanation": "HTML 负责定义网页的结构和内容。",
          "kind": "single_choice",
          "options": [
            "CSS",
            "JavaScript",
            "HTML",
            "Python"
          ],
          "prompt": "网页的“骨架”和“内容”是由哪种技术定义的？"
        },
        "focus_terms": [],
        "id": "s011",
        "introduced_terms": [],
        "lines": [
          "网页的“骨架”和“内容”是由哪种技术定义的？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step2",
    "source": {
      "plain_text": "A website is usually made of 1. HTML 2. CSS 3. JavaScript. HTML: Defines the structure and content of a webpage. CSS: Controls the visual presentation and layout. JavaScript: Adds interactivity and dynamic behavior.",
      "related": [
        "L2: Data scraping and database management with Python"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "css": {
        "aliases": [
          "Cascading Style Sheets"
        ],
        "display": "CSS",
        "gloss": "控制网页视觉呈现和布局的样式语言。"
      },
      "html": {
        "aliases": [
          "HyperText Markup Language"
        ],
        "display": "HTML",
        "gloss": "定义网页结构和内容的标记语言。"
      },
      "javascript": {
        "aliases": [
          "JS"
        ],
        "display": "JavaScript",
        "gloss": "为网页添加交互和动态行为的脚本语言。"
      }
    }
  },
  {
    "lesson_id": "L2",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [
          "beautifulsoup"
        ],
        "id": "s012",
        "introduced_terms": [
          "beautifulsoup"
        ],
        "lines": [
          "Python 里最流行的爬虫库之一就是 <term id=\"beautifulsoup\">BeautifulSoup</term>。",
          "它就像一个 HTML 的“瑞士军刀”。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "requests"
        ],
        "id": "s013",
        "introduced_terms": [
          "requests"
        ],
        "lines": [
          "首先，用 <term id=\"requests\">Requests</term> 库下载网页的原始内容。",
          "就像把整个网页的源代码复制下来。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s014",
        "introduced_terms": [],
        "lines": [
          "然后，把原始内容交给 BeautifulSoup 解析。",
          "它会帮你把 HTML 整理成容易操作的树状结构。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s015",
        "introduced_terms": [],
        "lines": [
          "用 `prettify()` 可以打印出格式清晰的 HTML。",
          "用 `get_text()` 可以提取网页里所有的纯文本。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s016",
        "introduced_terms": [],
        "lines": [
          "想找第一个段落？用 `find('p')`。",
          "想找所有链接？用 `find_all('a')`。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s017",
        "introduced_terms": [],
        "lines": [
          "对于表格数据，找到 `<table>` 标签后，",
          "可以配合 pandas 轻松转换成 DataFrame。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "find_all('p') 会返回所有 <p> 标签的列表。",
          "kind": "single_choice",
          "options": [
            "soup.find('p')",
            "soup.find_all('p')",
            "soup.get_text()",
            "soup.prettify()"
          ],
          "prompt": "要从网页中提取所有段落，应该使用哪个方法？"
        },
        "focus_terms": [],
        "id": "s018",
        "introduced_terms": [],
        "lines": [
          "要从网页中提取所有段落，应该使用哪个方法？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step3",
    "source": {
      "plain_text": "BeautifulSoup: Popular python library for searching and parsing HTML and XML data. pip install beautifulsoup4. pip install requests. Download raw content of a web page: import requests; r = requests.get(\"https://google.com\"); text = r.text. Print website elements in a nice format: prettify(). Extract all Text in a website: get_text(). Extract the 1st paragraph <p>: find(\"p\"). Extract all paragraphs <p>: find_all(\"p\"). Extract all hyperlinks <a>: find_all(\"a\"). Extract all <table>: find(\"table\") + pandas.",
      "related": [
        "L2: Data scraping and database management with Python"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "beautifulsoup": {
        "aliases": [
          "bs4"
        ],
        "display": "BeautifulSoup",
        "gloss": "一个用于解析 HTML 和 XML 数据的 Python 库。"
      },
      "requests": {
        "aliases": [],
        "display": "Requests",
        "gloss": "一个用于发送 HTTP 请求的 Python 库。"
      }
    }
  },
  {
    "lesson_id": "L2",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [],
        "id": "s019",
        "introduced_terms": [],
        "lines": [
          "让我们实战一下：爬取维基百科上的 S&P 500 成分股列表。",
          "这是一个非常经典的金融数据爬取任务。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s020",
        "introduced_terms": [],
        "lines": [
          "直接用 Requests 请求，结果却返回了错误信息。",
          "为什么？"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "user_agent"
        ],
        "id": "s021",
        "introduced_terms": [
          "user_agent"
        ],
        "lines": [
          "很多网站会检查请求的 <term id=\"user_agent\">User-Agent</term>。",
          "如果它看起来不像一个真实的浏览器，网站就会拒绝访问。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s022",
        "introduced_terms": [],
        "lines": [
          "解决方法很简单：在请求头中伪装成浏览器。",
          "设置一个常见的 User-Agent 字符串即可。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s023",
        "introduced_terms": [],
        "lines": [
          "成功获取页面后，用 BeautifulSoup 找到目标表格。",
          "可以通过表格的 id 属性来精确定位。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s024",
        "introduced_terms": [],
        "lines": [
          "提取表头和数据行，就能得到一个包含所有成分股信息的 DataFrame。",
          "包含股票代码、公司名、行业、成立年份等。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "许多网站会通过 User-Agent 来识别并阻止非浏览器的请求。",
          "kind": "single_choice",
          "options": [
            "网站服务器宕机",
            "请求缺少正确的 User-Agent",
            "Python 版本太低",
            "网络连接速度太慢"
          ],
          "prompt": "为什么直接请求维基百科可能会失败？"
        },
        "focus_terms": [],
        "id": "s025",
        "introduced_terms": [],
        "lines": [
          "为什么直接请求维基百科可能会失败？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step4",
    "source": {
      "plain_text": "Scrape S&P 500 stock composites from Wikipedia. Get HTML content of the website. Why we can't get the result? Please set a user-agent and respect our robot policy. headers = { \"user-agent\": \"Mozilla/5.0 ...\" }. Extract all <table>. Find the table by id: soup.find('table', id=\"constituents\").",
      "related": [
        "L2: Data scraping and database management with Python"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "user_agent": {
        "aliases": [],
        "display": "User-Agent",
        "gloss": "HTTP 请求头中的一个字段，用于标识客户端类型。"
      }
    }
  },
  {
    "lesson_id": "L2",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [
          "yfinance"
        ],
        "id": "s026",
        "introduced_terms": [
          "yfinance"
        ],
        "lines": [
          "直接爬取 Yahoo Finance 会遇到很多反爬措施。",
          "好在有一个专门的库：<term id=\"yfinance\">yfinance</term>。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s027",
        "introduced_terms": [],
        "lines": [
          "它封装了所有复杂的请求，让你几行代码就能获取数据。",
          "比如获取腾讯控股 (0700.HK) 的基本信息。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s028",
        "introduced_terms": [],
        "lines": [
          "用 `.history(period='1mo')` 可以获取最近一个月的股价历史。",
          "包含开盘价、最高价、最低价、收盘价和成交量。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "adjusted_close"
        ],
        "id": "s029",
        "introduced_terms": [
          "adjusted_close"
        ],
        "lines": [
          "你还会看到一个叫 <term id=\"adjusted_close\">调整收盘价</term> 的字段。",
          "它和普通收盘价有什么区别？"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s030",
        "introduced_terms": [],
        "lines": [
          "当公司分红或拆股时，股价会出现非市场因素的跳变。",
          "调整收盘价就是为了消除这些“噪音”，让历史价格具有可比性。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s031",
        "introduced_terms": [],
        "lines": [
          "比如，一只股票 2:1 拆股，价格会瞬间减半。",
          "调整收盘价会把拆股前的价格也除以 2，保持连续性。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 2,
          "explanation": "调整收盘价消除了分红和拆股的影响，更适合长期分析。",
          "kind": "single_choice",
          "options": [
            "开盘价",
            "收盘价",
            "调整收盘价",
            "最高价"
          ],
          "prompt": "进行长期回测时，应该使用哪个价格？"
        },
        "focus_terms": [],
        "id": "s032",
        "introduced_terms": [],
        "lines": [
          "进行长期回测时，应该使用哪个价格？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step5",
    "source": {
      "plain_text": "yfinance: It is an unofficial library for Yahoo Finance. pip install yfinance. Extract stock info: .info(). Extract stock history: .history(period). Extract multiple stocks: yf.download(tickers, start, end). Close vs Adjusted Close. Adjusted close prices are backward updated to account for these corporate events.",
      "related": [
        "L2: Data scraping and database management with Python"
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
        "gloss": "经过分红、拆股等公司事件调整后的收盘价。"
      },
      "yfinance": {
        "aliases": [],
        "display": "yfinance",
        "gloss": "一个非官方的 Yahoo Finance Python 库，用于获取股票数据。"
      }
    }
  },
  {
    "lesson_id": "L2",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [],
        "id": "s033",
        "introduced_terms": [],
        "lines": [
          "爬取到的数据不能每次都重新爬，需要存起来。",
          "数据库就是最好的选择。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "sqlite"
        ],
        "id": "s034",
        "introduced_terms": [
          "sqlite"
        ],
        "lines": [
          "Python 内置了对 <term id=\"sqlite\">SQLite</term> 的支持。",
          "它是一个轻量级的数据库，不需要安装服务器，一个文件就是一个数据库。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "sql"
        ],
        "id": "s035",
        "introduced_terms": [
          "sql"
        ],
        "lines": [
          "使用 <term id=\"sql\">SQL</term> 语言来操作数据库。",
          "最基本的四个操作：SELECT（查询）、INSERT（插入）、UPDATE（更新）、DELETE（删除）。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s036",
        "introduced_terms": [],
        "lines": [
          "首先，创建一个表来定义数据的结构。",
          "比如，创建一个 `market_candles` 表，包含股票代码、时间戳、价格等字段。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s037",
        "introduced_terms": [],
        "lines": [
          "然后，用 INSERT 语句把爬取到的数据逐行写入数据库。",
          "这样，数据就持久化到本地磁盘了。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s038",
        "introduced_terms": [],
        "lines": [
          "之后，就可以用 SELECT 语句随时查询历史数据，",
          "而不用每次都去爬取网站。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 1,
          "explanation": "INSERT 命令用于向表中添加新数据。",
          "kind": "single_choice",
          "options": [
            "SELECT",
            "INSERT",
            "UPDATE",
            "DELETE"
          ],
          "prompt": "要将爬取的数据保存到数据库，应该使用哪个 SQL 命令？"
        },
        "focus_terms": [],
        "id": "s039",
        "introduced_terms": [],
        "lines": [
          "要将爬取的数据保存到数据库，应该使用哪个 SQL 命令？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step6",
    "source": {
      "plain_text": "Why use a database for algo-trading? Efficient storage and retrieval of large amounts of historical data. SQL Syntax Overview: SELECT, INSERT, UPDATE, DELETE. Using SQLite in Python. Basic Usage: import sqlite3; conn = sqlite3.connect('example.db'); cursor = conn.cursor(). Create a Table, Inserting Data, Querying Data, Updating Data, Deleting Data. Scrape Market Data into database.",
      "related": [
        "L2: Data scraping and database management with Python"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "sql": {
        "aliases": [
          "Structured Query Language"
        ],
        "display": "SQL",
        "gloss": "用于管理关系数据库的标准语言。"
      },
      "sqlite": {
        "aliases": [],
        "display": "SQLite",
        "gloss": "一个轻量级的、基于文件的数据库引擎。"
      }
    }
  },
  {
    "lesson_id": "L2",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [],
        "id": "s040",
        "introduced_terms": [],
        "lines": [
          "设计数据库时，一个常见的陷阱：把所有数据塞进一张大表。",
          "比如，把所有股票的所有历史数据都放在一个表里。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s041",
        "introduced_terms": [],
        "lines": [
          "这样做的后果是：表会变得非常巨大，查询和备份都会很慢。",
          "这就是“大表”问题。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s042",
        "introduced_terms": [],
        "lines": [
          "一种改进方案是按时间分区，比如每个月一个表。",
          "但这会让跨月的数据分析变得很麻烦。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s043",
        "introduced_terms": [],
        "lines": [
          "另一种方案是按股票分区，比如每只股票一个表。",
          "但这又不利于跨资产的对比分析。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s044",
        "introduced_terms": [],
        "lines": [
          "所以，没有完美的设计。",
          "关键在于根据你的主要用途来权衡。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s045",
        "introduced_terms": [],
        "lines": [
          "如果主要用于回测，按股票分区可能更好。",
          "如果主要用于数据分析，按时间分区可能更合适。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s046",
        "introduced_terms": [],
        "lines": [
          "始终要在存储空间、运行速度和内存消耗之间找到平衡。",
          "这是数据库设计的核心艺术。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 2,
          "explanation": "代码行数不是数据库设计的核心权衡因素。",
          "kind": "single_choice",
          "options": [
            "存储空间",
            "运行速度",
            "代码行数",
            "内存消耗"
          ],
          "prompt": "设计金融数据库时，以下哪个不是需要考虑的权衡因素？"
        },
        "focus_terms": [],
        "id": "s047",
        "introduced_terms": [],
        "lines": [
          "设计金融数据库时，以下哪个不是需要考虑的权衡因素？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step7",
    "source": {
      "plain_text": "Database Design. Suppose you are going to build a database to store all market data history from global stocks exchanges. How would you structure the database? Potential issues for design 1: The table size will be very large. Potential issues for design 2: Difficult to conduct cross month analysis. Potential issues for design 3: Difficult for cross assets analysis. There is no single design that is perfect for all situations! It depends on our common use cases. Always need to strike a balance between Storage Space, Runtime Speed, Memory.",
      "related": [
        "L2: Data scraping and database management with Python"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {}
  },
  {
    "lesson_id": "L2",
    "mode": "guided_story",
    "screens": [
      {
        "focus_terms": [],
        "id": "s048",
        "introduced_terms": [],
        "lines": [
          "网络爬取并非万能，它有很多局限性。",
          "首先，数据质量可能不完整或过时。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s049",
        "introduced_terms": [],
        "lines": [
          "其次，爬取大量数据会消耗大量带宽和存储。",
          "加载整个网页也很耗时。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s050",
        "introduced_terms": [],
        "lines": [
          "最大的挑战是技术层面的：网站结构经常变化，",
          "而且很多网站有反爬机制，比如 IP 封锁和频率限制。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "selenium"
        ],
        "id": "s051",
        "introduced_terms": [
          "selenium"
        ],
        "lines": [
          "对于动态加载的网页，BeautifulSoup 可能无能为力。",
          "这时需要 <term id=\"selenium\">Selenium</term>，它可以模拟真实的浏览器操作。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [
          "xpath"
        ],
        "id": "s052",
        "introduced_terms": [
          "xpath"
        ],
        "lines": [
          "另一个强大的工具是 lxml，它支持 <term id=\"xpath\">XPath</term> 查询。",
          "XPath 可以用路径表达式精确定位到任何元素。"
        ],
        "type": "narration"
      },
      {
        "focus_terms": [],
        "id": "s053",
        "introduced_terms": [],
        "lines": [
          "比如，`//table[@id='constituents']//tr` 就能选中目标表格的所有行。",
          "比 BeautifulSoup 的查找方式更灵活。"
        ],
        "type": "narration"
      },
      {
        "exercise": {
          "answer": 2,
          "explanation": "Selenium 可以模拟用户点击等交互操作。",
          "kind": "single_choice",
          "options": [
            "BeautifulSoup",
            "Requests",
            "Selenium",
            "yfinance"
          ],
          "prompt": "对于需要点击按钮才能加载更多内容的网页，应该使用哪个工具？"
        },
        "focus_terms": [],
        "id": "s054",
        "introduced_terms": [],
        "lines": [
          "对于需要点击按钮才能加载更多内容的网页，应该使用哪个工具？"
        ],
        "type": "exercise"
      }
    ],
    "sequence_id": "step8",
    "source": {
      "plain_text": "Limitation of Data Scraping: 1. Data Quality 2. Resource Intensive 3. Technical Challenges. Selenium is a web automation tool. LXML: Popular python library for parsing XPath and XML schema. What is XPath? Common XPath Syntax. Extract stock list using lxml and XPath.",
      "related": [
        "L2: Data scraping and database management with Python"
      ]
    },
    "target_language": "zh-CN",
    "term_catalog": {
      "selenium": {
        "aliases": [],
        "display": "Selenium",
        "gloss": "一个用于模拟用户交互和自动化浏览器的工具。"
      },
      "xpath": {
        "aliases": [],
        "display": "XPath",
        "gloss": "一种在 XML 文档中导航和选择节点的查询语言。"
      }
    }
  }
]

</GUIDED_STORY_STEPS>

<CURRENT_STEP_ID>
step3
</CURRENT_STEP_ID>

<CURRENT_STEP>
{
  "lesson_id": "L2",
  "mode": "guided_story",
  "screens": [
    {
      "focus_terms": [
        "beautifulsoup"
      ],
      "id": "s012",
      "introduced_terms": [
        "beautifulsoup"
      ],
      "lines": [
        "Python 里最流行的爬虫库之一就是 <term id=\"beautifulsoup\">BeautifulSoup</term>。",
        "它就像一个 HTML 的“瑞士军刀”。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [
        "requests"
      ],
      "id": "s013",
      "introduced_terms": [
        "requests"
      ],
      "lines": [
        "首先，用 <term id=\"requests\">Requests</term> 库下载网页的原始内容。",
        "就像把整个网页的源代码复制下来。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s014",
      "introduced_terms": [],
      "lines": [
        "然后，把原始内容交给 BeautifulSoup 解析。",
        "它会帮你把 HTML 整理成容易操作的树状结构。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s015",
      "introduced_terms": [],
      "lines": [
        "用 `prettify()` 可以打印出格式清晰的 HTML。",
        "用 `get_text()` 可以提取网页里所有的纯文本。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s016",
      "introduced_terms": [],
      "lines": [
        "想找第一个段落？用 `find('p')`。",
        "想找所有链接？用 `find_all('a')`。"
      ],
      "type": "narration"
    },
    {
      "focus_terms": [],
      "id": "s017",
      "introduced_terms": [],
      "lines": [
        "对于表格数据，找到 `<table>` 标签后，",
        "可以配合 pandas 轻松转换成 DataFrame。"
      ],
      "type": "narration"
    },
    {
      "exercise": {
        "answer": 1,
        "explanation": "find_all('p') 会返回所有 <p> 标签的列表。",
        "kind": "single_choice",
        "options": [
          "soup.find('p')",
          "soup.find_all('p')",
          "soup.get_text()",
          "soup.prettify()"
        ],
        "prompt": "要从网页中提取所有段落，应该使用哪个方法？"
      },
      "focus_terms": [],
      "id": "s018",
      "introduced_terms": [],
      "lines": [
        "要从网页中提取所有段落，应该使用哪个方法？"
      ],
      "type": "exercise"
    }
  ],
  "sequence_id": "step3",
  "source": {
    "plain_text": "BeautifulSoup: Popular python library for searching and parsing HTML and XML data. pip install beautifulsoup4. pip install requests. Download raw content of a web page: import requests; r = requests.get(\"https://google.com\"); text = r.text. Print website elements in a nice format: prettify(). Extract all Text in a website: get_text(). Extract the 1st paragraph <p>: find(\"p\"). Extract all paragraphs <p>: find_all(\"p\"). Extract all hyperlinks <a>: find_all(\"a\"). Extract all <table>: find(\"table\") + pandas.",
    "related": [
      "L2: Data scraping and database management with Python"
    ]
  },
  "target_language": "zh-CN",
  "term_catalog": {
    "beautifulsoup": {
      "aliases": [
        "bs4"
      ],
      "display": "BeautifulSoup",
      "gloss": "一个用于解析 HTML 和 XML 数据的 Python 库。"
    },
    "requests": {
      "aliases": [],
      "display": "Requests",
      "gloss": "一个用于发送 HTTP 请求的 Python 库。"
    }
  }
}

</CURRENT_STEP>

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

请直接输出 JSON，不要加解释。
