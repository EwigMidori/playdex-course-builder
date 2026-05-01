Score asset-level relevance for the following lesson inputs. Output JSON only.

<SCORER_INPUT_JSON>
{
  "course_description": "COMP7415A - Mastering the markets: Financial analytics and algorithmic trading\nSemester 2, 2024-25\nProfessor\t\nTony Lam\nSyllabus\tAlgorithmic trading is a trending investment approach nowadays that consists of identification of trading opportunities and implementation via computer algorithms. This course will cover emerging trend in the quantitative investment field, and introduce various data analysis techniques and methodologies that are commonly employed in the industry.\n\nThe first half of the course focuses on financial data analysis and strategy implementation. The second half of the course discusses practical techniques to manage and deploy algorithmic trading strategies in real financial world.\nIntroduction by Professor\t \nLearning Outcomes\t\nCourse Learning Outcomes\t\nCLO1. A solid understanding of the nuances of algorithmic trading, design and implement algorithmic trading strategies\t\nCLO2. The ability to apply quantitative methods to analyze financial data and build financial models\t\nCLO3. The ability to formulate trading strategies, carry out backtesting, optimization, risk management and interpret investment performance\t\nCLO4. The ability to apply various investment theories and trading techniques to the real financial market\t\nCLO5. Familiar with the current trends, and understand the limitations and challenges in the field\t\nCLO6. Complete a capstone project that includes a full cycle of trading strategy development\t\nPre-requisites\tTo succeed in this course, students are expected to have a foundation and basic knowledge in the following areas:\n- Python programming\n- Statistics and probability theory\n- Mathematics and optimization theory\nCompatibility\t-\nTopics covered\t\nCourse Content\tNo. of Hours\tCourse Learning Outcomes\nAlgorithmic trading basics and financial markets\t \t \nData scraping and database management with Python\t \t \nBuilding backtest framework and rule-based trading strategy\t \t \nStatistical time series analysis for market classification\t \t \nStatistical arbitrage and pairs trading\t \t \nCapital and Risk Management\t \t \nPerformance measures and portfolio optimization\t \t \nOrder book and high frequency data modeling\t \t \nMarket practice in broker selection and account connection\t \t \nMachine learning use cases in algorithmic trading\t \t \n \nAssessment\t\nDescription\tType\tWeighting *\tTentative Assessment Period /\nExamination Period ^\tCourse Learning Outcomes\nWritten assignment and project\tContinuous Assessment\t50%\t-\t \nWritten examination covering all the taught contents in the course\tWritten Examination\t50%\t-\t \n* The weighting of coursework and examination marks is subject to approval\n^ The exact examination date uses to be released when all enrolments are confirmed after add/drop period by the Examinations Office.  Students are obliged to follow the examination schedule.  Students should NOT enrol in the course if they are not certain that they will be in Hong Kong during the examination period.  Absent from examination may result in failure in the course. There is no supplementary examination for all MSc curriculums in the Faculty of Engineering.\nCourse materials\t\n \n\nSession dates\t\nDate\tTime\tVenue\tRemark\nSession 1\t5 Feb 2025 (Wed)\t7:00pm - 10:00pm\tLE-5\t \nSession 2\t12 Feb 2025 (Wed)\t7:00pm - 10:00pm\tLE-5\t \nSession 3\t19 Feb 2025 (Wed)\t7:00pm - 10:00pm\tLE-5\t \nSession 4\t26 Feb 2025 (Wed)\t7:00pm - 10:00pm\tLE-5\t \nSession 5\t5 Mar 2025 (Wed)\t7:00pm - 10:00pm\tLE-5\t \nSession 6\t19 Mar 2025 (Wed)\t7:00pm - 10:00pm\tLE-5\t \nSession 7\t26 Mar 2025 (Wed)\t7:00pm - 10:00pm\tLE-5\t \nSession 8\t2 Apr 2025 (Wed)\t7:00pm - 10:00pm\tLE-5\t \nSession 9\t9 Apr 2025 (Wed)\t7:00pm - 10:00pm\tLE-5\t \nSession 10\t16 Apr 2025 (Wed)\t7:00pm - 10:00pm\tLE-5\t \nLE - Library Extension Building\nAdd/drop\t20 January, 2025 - 12 February, 2025\nMaximum class size\t146\nBack\n",
  "exam_signal": {
    "notes": [
      "exam_pdf_text_unavailable"
    ],
    "other_files": [],
    "path": "research/pipeline/0-raw/exam",
    "pdf_files": [
      {
        "file": "5787b1e26c2d8fb8b3e8babf76f60eff.pdf",
        "note": "exam_pdf_text_unavailable",
        "path": "research/pipeline/0-raw/exam/5787b1e26c2d8fb8b3e8babf76f60eff.pdf"
      },
      {
        "file": "5e64a400db844f6beea9e335ded8a49f.pdf",
        "note": "exam_pdf_text_unavailable",
        "path": "research/pipeline/0-raw/exam/5e64a400db844f6beea9e335ded8a49f.pdf"
      }
    ],
    "text_files": []
  },
  "guided_story_manifest": {
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
  },
  "lesson_id": "L2",
  "plain_text": "# L2: Data scraping and database management with Python\n\nCourse Code: COMP7415\n\n# Agenda\n\n- Understand the basic website structure   \n- Web scraping using different python libraries   \n- The limitations of web scraping   \n- Create a simple database using python   \n- Understand the concept of adjusted price   \n- Database design for financial market data storage\n\n# Installation of Python\n\n- Python: https://www.python.org/downloads/   \n- Anaconda (suggested): https://www.anaconda.com/download/success\n\n- easier management for different environments   \n- pre-installed data analysis packages (eg. numpy, pandas, scipy, etc)\n\n# What’s Data/Web Scraping?\n\n- Automated process to extract data from websites   \n- Common use cases in algo-trading:\n\nReal-time data collection (eg. economic indicators, stock prices, etc)   \n- Market sentiment analysis (eg. news, forum and blogs, social media)   \n- Corporate analysis (eg. product announcements, annual reports, director changes, etc)   \nAlternative data (eg. weather, web traffic, etc)\n\n# Basic Web structure\n\n# A website is usually made of\n\n1. HTML (HyperText Markup Language)   \n2. CSS (Cascading Style Sheets)   \n3. JavaScript\n\n# HTML\n\n- Purpose: Defines the structure and content of a webpage.   \n- Elements: Uses tags to create elements such as headings, paragraphs, links, images, tables, etc.   \n• Markup Language: Not a programming language; it’s a markup language used to structure content.\n\n# Common HTML Tags\n\n<table><tr><td>Tag</td><td>Description</td></tr><tr><td></td><td>Defines the root of an HTML document.</td></tr><tr><td></td><td>Contains meta-information about the HTML document, such as title, styles, and scripts.</td></tr><tr><td></td><td>Sets the title of the HTML document, displayed in the browser&#x27;s title bar or tab.</td></tr><tr><td></td><td>Contains the content of the HTML document, including text, images, links, and other elements.</td></tr><tr><td></td><td>Defines headings of different sizes, with &lt;h1&gt; being the largest and &lt;h6&gt; the smallest.</td></tr><tr><td></td><td>Defines a paragraph of text.</td></tr><tr><td></td><td>Creates a hyperlink, linking to another webpage, file, or location within the same page.</td></tr><tr><td></td><td>Inserts an image into the HTML document.</td></tr><tr><td></td><td>Defines a division or container for grouping HTML elements and applying CSS styles.</td></tr><tr><td></td><td>Defines an inline container for styling a specific portion of text or content.</td></tr><tr><td></td><td>Creates an unordered or ordered list, respectively, containing list items (&lt;li&gt;).</td></tr><tr><td></td><td>Defines a list item within an unordered or ordered list.</td></tr><tr><td></td><td>Creates a table for organizing data into rows and columns.</td></tr><tr><td></td><td>Defines a table row within a &lt;table&gt; element.</td></tr><tr><td></td><td>Defines a table cell within a &lt;tr&gt; element.</td></tr><tr><td></td><td>Defines a header cell within a &lt;tr&gt; element in a table.</td></tr><tr><td></td><td>Creates an HTML form for collecting user input.</td></tr><tr><td></td><td>Defines an input control element within a form, such as text fields, checkboxes, or buttons.</td></tr><tr><td></td><td>Associates a label with an input element, improving accessibility and usability.</td></tr><tr><td></td><td>Defines a clickable button within a form or webpage.</td></tr><tr><td></td><td>Creates a multiline text input control within a form.</td></tr><tr><td></td><td>Embeds another HTML page or external content within the current document.</td></tr><tr><td></td><td>Embeds or links to client-side scripts, such as JavaScript.</td></tr><tr><td></td><td>Contains CSS rules for styling HTML elements within the document.</td></tr></table>\n\n# HTML Example\n\n```html\n1 <!--DOCTYPE html>\n2 <html>\n3 <head>\n4 <meta charset=\"utf-8\">\n5 <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n6 <title>Page Title</title>\n7 </head>\n8 <body>\n9 <h1>Heading</h1>\n10 <p>Paragraph</p>\n11 </body>\n12 </html>\n13 \n```\n\n![](images/9a0356d3c69aa54d62765a0ef2abff6400803e8e1063976ac685e8e80ce3ccd6.jpg)\n\n# CSS\n\n- Purpose: Controls the visual presentation and layout of a webpage   \n- Styling: Defines styles for HTML elements, such as colors, fonts, spacing, and positioning.   \n- Cascading Rules: Multiple styles can be applied to an element, with rules for which styles take precedence.\n\n<!DOCTYPE html>   \n<html>   \n<head> <meta charset=\"utf-8\"> <meta name $=$ \"viewport\" content $=$ \"width $\\equiv$ device-width, initial-scale $= 1$ titlePage Title</title> <style type $=$ \"text/css\"> body { font-family: Arial,sans-serif; margin:0; } h1{ color:blue; text-align:center; } p{ font-size:14px; line-height:1.5; } </style>   \n</head>   \n<body> <h1>Heading</h1> <p>Paragraph</p>   \n</body>   \n</html>\n\n![](images/c35f5d13e9683acbd934c657333f5da9f266c0c770a62cd650e50b8bd39e14cb.jpg)\n\n# JavaScript\n\n- Purpose: Adds interactivity and dynamic behavior to webpages.   \n- Programming Language: A scripting language that runs in the browser (client-side) and/or on the server (server-side with Node.js).   \n- Manipulation: Can manipulate HTML and CSS to change content and styles dynamically.   \n- Event Handling: Responds to user actions like clicks, form submissions, and keyboard input.\n\n1<!DOCTYPE html>   \n2<html>   \n3head>   \n4 <meta charset=\"utf-8\">   \n5 <meta name $=$ \"viewport\" content $=$ \"width $\\equiv$ device-width, initial-scale $= 1$ 6 <title>Page Title</title>   \n7   \n8 <style type $=$ \"text/css\">   \n9 body{   \n10 font-family: Arial, sans-serif;   \n11 margin:0;   \n12 }   \n13   \n14 h1{   \n15 color: blue;   \n16 text-align: center;   \n17 }   \n18   \n19 p{   \n20 font-size: 14px;   \n21 line-height: 1.5;   \n22 }   \n23 </style>   \n24   \n25 <script type $=$ \"text/javascript\">   \n26 document.addEventListener(\"DOMContentLoaded\", function(){ document.querySelector(\"h1\").style.color $=$ \"red\";   \n27 });   \n28 });   \n29 </script>   \n30   \n31   \n32 </head>   \n33 <body>   \n34 <h1>Heading</h1>   \n35 <p>Paragraph</p>   \n36 </body>   \n37 </html>\n\n![](images/e6c5cbfbf2149c0808a26a2e790762c64c7ffc6a4a645e19a5552ff19553533d.jpg)\n\n# Task: Scrape S&P 500 stock composites\n\n三\n\n![](images/b77c8a61bb89d87ef588664e634543090be902e2503b1ea022ee41d2921b091f.jpg)\n\nWIKIPEDIA\n\nThe Free Encyclopedia\n\n![](images/214dfa94388a03b20ce2c704651d403b3657eef3a7f9cc9e105a99c594b504f0.jpg)\n\nSearch Wikipedia\n\nSearch\n\nCreate account Log in\n\n中\n\n# List of S&P 500 companies\n\n文A 5 languages\n\nContents\n\nhide\n\nArticle Talk\n\nRea\n\nEdit View history\n\nTools\n\nAppearance\n\nhide\n\n(Top)\n\nS&P 500 component stocks\n\nSelected changes to the list of S&P 500 components\n\nSee also\n\nReferences\n\nExternal links\n\nFrom Wikipedia, the free encyclopedia\n\nThe S&P 500 is a stock market index maintained by S&P Dow Jones Indices. It comprises 503 common stocks which are issued by 500 large-cap companies traded on American stock exchanges (including the 30 companies that compose the Dow Jones Industrial Average). The index includes about 80 percent of the American equity market by capitalization. It is weighted by free float market capitalization, so more valuable companies account for relatively more weight in the index. The index constituents and the constituent weights are updated regularly using rules published by S&P Dow Jones Indices. Although called the S&P 500, the index contains 503 stocks because it includes two share classes of stock from 3 of its component companies.[1][2]\n\nS&P 500 component stocks [edit]   \n\n<table><tr><td>Symbol</td><td>Security</td><td>GICS Sector</td><td>GICS Sub-Industry</td><td>Headquarters Location</td><td>Date added</td><td>CIK</td><td>Founded</td></tr><tr><td>MMM</td><td>3M</td><td>Industrials</td><td>Industrial Conglomerates</td><td>Saint Paul, Minnesota</td><td>1957-03-04</td><td>0000066740</td><td>1902</td></tr><tr><td>AOS</td><td>A. O. Smith</td><td>Industrials</td><td>Building Products</td><td>Milwaukee, Wisconsin</td><td>2017-07-26</td><td>0000091142</td><td>1916</td></tr><tr><td>ABT</td><td>Abbott</td><td>Health Care</td><td>Health Care Equipment</td><td>North Chicago, Illinois</td><td>1957-03-04</td><td>0000001800</td><td>1888</td></tr><tr><td>ABBV</td><td>AbbVie</td><td>Health Care</td><td>Biotechnology</td><td>North Chicago, Illinois</td><td>2012-12-31</td><td>0001551152</td><td>2013 (1888)</td></tr></table>\n\nText\n\n$\\bigcirc$ Small   \nStandard   \nLarge   \nWidth   \nStandard   \nWide\n\n# Inspect the website structure\n\n![](images/36f71ed7f6522db96b6f97ca6c123003ab1e6ac15c1e5293a629c3406d44e819.jpg)\n\n# Web Scraping with Python\n\n# BeautifulSoup\n\n- Popular python library for searching and parsing HTML and XML data   \n- Official Doc: https://beautiful-soup-4.readthedocs.io/en/latest/\n\npip install beautifulsoup4\n\npip install requests\n\n# Download raw content of a web page\n\nimport requests\n\nr = requests.get(\"https://google.com\")\n\ntext = r.text\n\nprint(text)\n\n![](images/1c510c9ef62a467a9b008790e6d92945b10b7b29c31e65fe8ac2912ea6c8388d.jpg)\n\n![](images/27e340c19b2a898ca061322045116f0314e1f560fd21aaf566b575103d6c500b.jpg)\n\n![](images/aa2d370b2d5e178a4798bce1aa2781e648a2463b39ec7224c2865a7582ba7048.jpg)\n\nOogle.com\n\n![](images/d43d0c915f02591061c3ef3f1b123b1cada0b8953c59f07719f76c8b9d6b1d3d.jpg)\n\n![](images/63663c1386f8ac65d2cec3d5d69d49c0c99d6462c5ad860cd37d075c5ad1f465.jpg)\n\n```html\n<!DOCTYPE html><html itemscope=\"itemtype=\"http://schema.org/WebPage\" lang=\"zh-HK\"><head><meta content=\"text/html; charset=UTF-8\" http-equiv=\"Content-Type\"><meta content=\"/images/branding/googleleg/1x/googleleg_STANDARD_color_128dp.png\" itemprop=\"image\"<title>Google</title><script nonce=\"AUM0Px-zhCjmfeAs-C9Bg\">(function(){var g={'kEI:'mjCCZtn3ENS12roPtyirAs',kEXPI:'0,3700303,639,439,7,21,538636,2872,2891,8349,3405,31274,30022,1235 74,56390,2,39761,6699,41949,57734,2,2,1,24626,2006,8155,23350,22436,9779,62658,73178,3030,15816,1804,21011, 5396,686,8175,11814,1634,43464,7591,5217089,891,623,37,130,5991641,2839759,16,527,240,4,19,3,1,51,1,47,2798 1467,16672,43887,3,318,4,1281,3,2124363,23029351,7954,1,4844,8408,10903,5762,28027,36870,10511,2370,6407,13 845,10475,2478,2212,7981,200,154,21743,4,23390,4139,3181,6904,11584,2483,4272,155,1759,5,720,4400,9103,6452 ,1285,6599,2539,740,,2,,225,,539,,1643,,1449,,206,,122,,546,,2671,,4,,3004,,10417,,3480,,1694,,4082,,409,,519,,2805,,1,,6,, 159 6,,3.85,,1224,1668,1699,3936,,23,663,,2929,,2308,,1440,,1119,,3，52，22，344，1509，423，665，7170，975，709，231，3102，4，1162 , 120，1704，334，665，304，7，1，839，1714，682，288，376，14，5657，34，476，4569，215，107，716，270，6，495，1，570，205，367，50， 1 441，435，287，2，9，622，1737，15，1，1，1621，3，1272，654，258，2133，6，208，36，294，831，1948，464，436，293，3，2，573，84，4， 1 , 297，412，1574，367，3.94.572.1109.25.107.172.3.1.0.307.221.65.449.51.23.1.3.1. 10.553.883.401.190.841.610.55.504.273.1.8., 1. 340.207.537.871. 127.291. 366.714. 1212. 1693. 60. 142. 295. 11. 108. 1 8I. 1I.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II.II I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I I IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II II III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III III IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII\n```\n\n# Quick Example\n\n```html\n<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Web Scraping Demo</title></head><body><h1>Web Scraping Demo Page</h1></p>Welcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills.</p>Sample Table</h2> <table border=\"1\">thead>\n<tr><th>Name</th><th>Age</th><th>City</th></tr><th>Head</th><th>Body</th>\n<td>Alice</td><td>30</td><td>New York</td></tr><tr><td>Bob</td><td>25</td>\n<td>Los Angeles</td><td>35</td><td>Chicago</td></tr>\n</table><h2>Useful Links</h2><p>Check out the following resources for more information:</p><ul><a href=\"https://www.python.org/\"Python Official Website</a></li><a\nhref=\"https://www.crummy.com/software/BeautifulSoup/\"Beautiful Soup\nDocumentation</a></li><a href=\"https://requests.readthedocs.io/\"Requests:\nHTTP for Humans</a></li></body></html> \n```\n\n<!DOCTYPE html>   \n<html lang=\"en\">   \n<head> <meta charset=\"UTF-8\"> <meta name $=$ \"viewport\" content $=$ \"width $\\equiv$ device-width, initial-scale $= 1.0$ > <title>Web Scraping Demo</title> </head>   \n<body> <h1>Web Scraping Demo Page</h1> <p>Welcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills.</p> <h2>Sample Table</h2> <table border $=$ \"1\"> <thead> <tr> <th>Name</th> <th>Age</th> <th>City</th> </tr> </thead> <tbody> <tr> <td>Alice</td> <td>30</td> <td>New York</td> </tr> <tr> <td>Bob</td> <td>25</td> <td>Los Angeles</td> </tr> <tr> <td>Charlie</td> <td>35</td> <td>Chicago</td> </tr> </tbody> </table> <h2>Useful Links</h2> <p>Check out the following resources for more information:</p> <ul> <li><a href $=$ \"https://www.python.org/\"Python Official Website</a></li> <li><a href $=$ \"https://www.ccrummy.com/software/BeautifulSoup/\"Beautiful Soup Documentation</a></li> <li><a href $=$ \"https://requests.readthedocs.io/\"Requests: HTTP for Humans</a></li> </ul>   \n</body>   \n</html>\n\n# Print website elements in a nice format\n\n# - prettify()\n\nimport requests\n\nfrom bs4 import BeautifulSoup\n\n```html\ntext ='<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Web Scraping Demo</title></head><body><h1>Web Scraping Demo Page</h1></p>Welcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills.</p><h2>Contact Demo Table</h2></table border=\"1\"><thead><tr><th>Name</th><th>Age</th><th>City</th></tr></thead><tbody><td>Alice</td><td>30</td><td>New York</td></tr></td>Bob</td><td>25</td><td>Los Angeles</td></tr><tr><td>Charlie</td><td>35</td><td>Chicago</td></tr></table><h2>Useful Links</h2></p>Check out the following resources for more information:</p><ul></a href=\"/https://www.python.org/\"Python Official Website</a></li></a href=\"/https://www.crummy.com/software/BeautifulSoup/\"Beautiful Soup Documentation</a></li></a href=\"/https://requests.readthedocs.io/\"Requests: HTTP for Humans</a></li></ul></body></html'> \n```\n\nsoup = BeautifulSoup(text, 'html.parser')\n\nprint(soup.prettify())\n\n```html\n<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\"/>\n<meta content=\"width device-width, initial-scale=1.0\" name=\"viewport\"/>\n<title>\nWeb Scraping Demo\n</title>\n</head>\n<body>\n<h1>\nWeb Scraping Demo Page\n</h1>\n<p>\nWelcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills.</p>\n<h2>\nSample Table\n</h2>\n<table border=\"1\">\n<thead>\n<tr>\n<th>\nName</th>\n<th>\nAge</th>\n<th>\nCity</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td>\nAlice</td>\n<td>\n30</td>\n<td>\nNew York</td>\n</table>\n</body>\n</table> \n```\n\n# Extract all Text in a website\n\n# • get_text()\n\nimport requests\n\nfrom bs4 import BeautifulSoup\n\n```html\ntext ='<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Web Scraping Demo</title></head><body><h1>Web Scraping Demo Page</h1></p>Welcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills.</p><h2>Contact\"Sample Table</h2></table border=\"1\"><thead><tr><th>Name</th><th>Age</th><th>City</th></tr></thead><tr><td>Alice</td><td>30</td><td>New York</td></tr></td>Bob</td><td>25</td><td>Los Angeles</td></tr></td>Charlie</td></td>35</td><td>Chicago</td></tr></table><h2>Useful Links</h2></p>Check out the following resources for more information:</p><ul><a href=\"#\">https://www.python.org/\"Python Official Website</a></li><a href=\"#\">https://www.ccrummy.com/software/BeautifulSoup/\"Beautiful Soup Documentation</a></li><a href=\"#\">https://requests.readthedocs.io/\"Requests: HTTP for Humans</a></li></body></html'> \n```\n\nsoup = BeautifulSoup(text, 'html.parser')\n\nprint(soup.get_text())\n\nWeb Scraping Demo Web Scraping Demo Page Welcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills. Sample Table Name Age City Alice 30 New York Bob 25 Los Angeles Charlie 35 Chicago Useful Links Check out the following resources for more information: Python Official Website Beautiful Soup Documentation Requests: HTTP for Humans\n\n# Extract the $1^{\\text {st }}$ paragraph <p>\n\n- find(\"p\")\n\nimport requests from bs4 import BeautifulSoup\n\n```html\ntext ='<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Web Scraping Demo</title></head><body><h1>Web Scraping Demo Page</h1></p>Welcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills.</p><h2>Contact Table</h2><table border=\"1\"><thead><tr><th>Name</th><th>Age</th><th>City</th></tr><tr><td>Alice</td><td>Alice</td></td><td>Old</td><td>New York</td></tr><tr><td>Bob</td><td>Chicago</td></td><td>Los Angeles</td></tr><tr><td>Charlie</td><td>Chicago</td></td><td>Philadelphia</td></table><td>Useful Links</h2></p>Check out the following resources for more information:</p><ul><li><a href=\"https://www.python.org/\"Python Official Website</a></li><a href=\"https://www.crummy.com/software/BeautifulSoup/\"Beautiful Soup Documentation</a></li><a href=\"https://requests.readthedocs.io/\"Requests: HTTP for Humans</a></li></body></html'> \n```\n\nsoup = BeautifulSoup(text, 'html.parser')\n\ndata = soup.find('p')\n\nprint(data.text)\n\nWelcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills.\n\n# Extract all paragraphs <p>\n\n# • find_all(\"p\")\n\nimport requests from bs4 import BeautifulSoup\n\n```html\ntext ='<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Web Scraping Demo</title></head><body><h1>Web Scraping Demo Page</h1></p>Welcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills.</p><h2>Contact Table</h2><table border=\"1\"><thead><tr><th>Name</th><th>Age</th><th>City</th></tr></thead><tr><td>Alice</td><td>Alice</td><td>30</td><td>New York</td></tr><tr><td>Bob</td><td>25</td><td>Los Angeles</td></tr><tr><td>Charlie</td><td>35</td><td>Chicago</td></tr></table><td>H2>Useful Links</h2></p>Check out the following resources for more information:</p><ul><li><a href=\"https://www.python.org/\"Python Official Website</a></li><a href=\"https://www.crummy.com/software/BeautifulSoup/\"Beautiful Soup Documentation</a></li><a href=\"https://requests.readthedocs.io/\"Requests: HTTP for Humans</a></li></body></html>' \n```\n\nsoup = BeautifulSoup(text, 'html.parser')\n\ndata = soup.find_all('p')\n\nfor p in data: print(p.text)\n\nWelcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills. Check out the following resources for more information:\n\n# Extract all hyperlinks $< a>$\n\n# • find_all(\"a\")\n\nimport requests\n\nfrom bs4 import BeautifulSoup\n\ntext ='<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Web Scraping Demo</title></head><body><h1>Web Scraping Demo Page</h1></body> <p>Welcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills.</p> <h2>Sample Table</h2> <table border=\"1\"><thead><tr>\n    <th>Name</th> <th>Age</th> <th>City</th> </tr> </thead> <tbody> </td>Alice</td>\n    <td>30</td> <td>New York</td> </tr> <tr> <td>Bob</td> <td>25</td> <td>Los Angeles</td>\n    </tr> </td> Charlie</td> <td>35</td> <td>Chicago</td> </tr> </tbody>\n    <h2>Contact Link</h2>\n    <p>Check out the following resources for more information:</p>\n    <li><a href=\"https://www.python.org/\" Python Official Website</a></li><a\nhref=\"https://www.crummy.com/software/BeautifulSoup/\" Beautiful Soup\nDocumentation</a></li><a href=\"https://requests.readthedocs.io/\" Requests: HTTP for Humans</a></li></body></html>'\n\nsoup = BeautifulSoup(text, 'html.parser')  \ndata = soup.find_all('a')\n\nfor a in data: print(a) print(a.text) print(\"\\n\")\n\n<a href=\"https://www.python.org/\" >Python Official Website</a>  \nPython Official Website\n\n<a href=\"https://www.crummy.com/software/BeautifulSoup/\" Beautyful Soup Documentation</a> Beautiful Soup Documentation\n\n<a href=\"https://requests.readthedocs.io/\"Requests: HTTP for Humans</a> Requests: HTTP for Humans\n\n# Extract all <table>\n\n![](images/a8866a3125635d8949f600fafc109a74455f0b0638b1066e329bb4036cfd5d4e.jpg)\n\n```txt\nName Age City 0 Alice 30 New York 1 Bob 25 Los Angeles 2 Charlie 35 Chicago \n```\n\n```python\nimport requests  \nimport pandas as pd  \nfrom bs4 import BeautifulSoup \n```\n\n```html\ntext ='<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Web Scraping Demo</title></head><body><h1>Web Scraping Demo Page</h1></p>Welcome to the web scraping demo page. Here, you will find a simple table and some links to practice your scraping skills.</p><h2>Sample Table</h2><table border=\"1\"><thead><tr><th>Name</th><th>Age</th><th>City</th></tr><th>Allele</td><td>Alice</td><td>Old</td><td>New York</td></tr><tr><td>Bob</td><td>Chicago</td></tr><tr><td>Los Angeles</td></table><td>Useful Links</h2></p>Check out the following resources for more information:</p><li><a href=\"https://www.python.org/\"Python Official Website</a></li><a href=\"https://www.crammy.com/software/BeautifulSoup/\"Beautiful Soup Documentation</a></li><a href=\"https://requests.readthedocs.io/\"Requests: HTTP for Humans</a></li></body></html'> \n```\n\n```python\nsoup = BeautifulSoup(text, 'html.parser') \n```\n\n```txt\nFind the table  \ntable = soup.find(\"table\") \n```\n\n```python\nExtract the headers  \nheaders = []  \nfor th in table.find_all('th'):  \n    headers.append(th.text) \n```\n\nExtract the rows   \nrows $= \\left[\\right]$ for tr in table.find_all('tr'[1:]: #Skip the header row cells $=$ tr.find_all('td') row $=$ [cell.text for cell in cells] rows.append(row)\n\n```txt\nCreate a DataFrame df = pd.DataFrame(columns, headers) print(df) \n```\n\n# Scrape S&P 500 stock composites\n\n三\n\n![](images/ed717c804f45387ca8f195878f26adf181536b27ed3983caf82851bc1881a259.jpg)\n\nWIKIPEDIA\n\nThe Free Encyclopedia\n\n![](images/136ede79f45d9d9111f6704547b8b07c1ed7b888b6881627de51dac9e99b96c9.jpg)\n\nSearch Wikipedia\n\nSearch\n\nCreate account Log in\n\n中\n\n# List of S&P 500 companies\n\n文A 5 languages\n\nContents\n\nhide\n\nArticle Talk\n\nRea\n\nEdit View history\n\nTools\n\nAppearance\n\nhide\n\n# (Top)\n\nS&P 500 component stocks\n\nSelected changes to the list of S&P 500 components\n\n# See also\n\nReferences\n\nExternal links\n\nFrom Wikipedia, the free encyclopedia\n\nThe S&P 500 is a stock market index maintained by S&P Dow Jones Indices. It comprises 503 common stocks which are issued by 500 large-cap companies traded on American stock exchanges (including the 30 companies that compose the Dow Jones Industrial Average). The index includes about 80 percent of the American equity market by capitalization. It is weighted by free float market capitalization, so more valuable companies account for relatively more weight in the index. The index constituents and the constituent weights are updated regularly using rules published by S&P Dow Jones Indices. Although called the S&P 500, the index contains 503 stocks because it includes two share classes of stock from 3 of its component companies.[1][2]\n\nS&P 500 component stocks [edit]   \n\n<table><tr><td>Symbol</td><td>Security</td><td>GICS Sector</td><td>GICS Sub-Industry</td><td>Headquarters Location</td><td>Date added</td><td>CIK</td><td>Founded</td></tr><tr><td>MMM</td><td>3M</td><td>Industrials</td><td>Industrial Conglomerates</td><td>Saint Paul, Minnesota</td><td>1957-03-04</td><td>0000066740</td><td>1902</td></tr><tr><td>AOS</td><td>A. O. Smith</td><td>Industrials</td><td>Building Products</td><td>Milwaukee, Wisconsin</td><td>2017-07-26</td><td>0000091142</td><td>1916</td></tr><tr><td>ABT</td><td>Abbott</td><td>Health Care</td><td>Health Care Equipment</td><td>North Chicago, Illinois</td><td>1957-03-04</td><td>0000001800</td><td>1888</td></tr><tr><td>ABBV</td><td>AbbVie</td><td>Health Care</td><td>Biotechnology</td><td>North Chicago, Illinois</td><td>2012-12-31</td><td>0001551152</td><td>2013 (1888)</td></tr></table>\n\n# Text\n\n![](images/6c0d5fa71a82fb88ccb08419bbf757b1b8995976c6e85c4bf56f9f3c836be31a.jpg)\n\nSmall\n\n![](images/8da8c36f67938a556a6311230d2044eb87373c23e75794d55fff947a0f4b1f55.jpg)\n\nStandard\n\n![](images/6cb6a2aa069abcdd3e5afc6dbc34a01bf989944e529939141a6a55c6dd9fcff5.jpg)\n\nLarge\n\n# Width\n\n![](images/28003b144c3b363d1cf26bc4a9876f6d2622eacff7bf8e6fd24ae4bbacfddd19.jpg)\n\nStandard\n\n![](images/d66268d18e41a2493f99f7bbaec3f683b4cc692b96ff331cbf4ac55ff7f87a37.jpg)\n\nWide\n\n# Get HTML content of the website\n\nimport requests   \nfrom bs4 import BeautifulSoup   \nimport pandas as pd   \nurl $=$ 'https://en.wikipedia.org/wiki/List_of_S%26P_500_companies'   \ntext $=$ requests.get(url).text   \nprint(text)\n\n![](images/d03efbbf1a5be83a807912cbc7e8194efae47bfc28540fcdb7a95ba9eb4eaf82.jpg)\n\nPlease set a user-agent and respect our robot policy. https://w.wiki/4wJS. See also https://phabricator.wikimedia.org/T400119.\n\nWhy we can't get the result?\n\n# Extract all <table>\n\n![](images/64ecad85c9d2110134bb242b6487da9c2033aa3acbc05e7122040d75612288cf.jpg)\n\nThere are 2 tables in the wiki page. How to locate the correct table?\n\nimport requests   \nfrom bs4 import BeautifulSoup   \nimport pandas as pd   \nheaders $=$ { \"user-agent\": \"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36\" }   \nurl $=$ 'https://en.wikipedia.org/wiki/List_of_S%26P_500_com panies' text $=$ requests.get(url, headers $\\equiv$ headers).text   \nsoup $=$ BeautifulSoup(text, 'html.parser')   \ndata $=$ soup.find_all('table')   \nprint(len(data))\n\n![](images/50fcace5412e145f4983130c1ed4a1272a25c18f06725f9d84afd62a53665f7d.jpg)\n\n# Extract all <table>\n\n![](images/cd3ed0e244cb21a7c0940c524fd9c1ab6b5a85b39672959aa3f3e20d4bda2984.jpg)\n\n```csv\nSymbol Security GICS Sector ... Date added CIK Founded 0 MMM 3M Industrials ... 1957-03-04 0000066740 1902 1 AOS A. O. Smith Industrials ... 2017-07-26 0000091142 1916 2 ABT Abbott Health Care ... 1957-03-04 0000001800 3 ABBV AbbVie Health Care ... 2012-12-31 0001551152 2013 (1888) 4 ACN Accenture Information Technology ... 2011-07-06 0001467373 1989 ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ...\n498 XYL Xylem Inc. Industries .. 2011-11-01 0001524472 2011 499 YUM Yum! Brands Consumer Discretionary .. 1997-10-06 0001041061 1997 500 ZBRA Zebra Technologies Information Technology .. 2019-12-23 0000877212 1969 501 ZBH Zimmer Biomet Health Care .. 2001-08-07 0001136869 1927 502 ZTS Zoetis Health Care .. 2013-06-21 0001555280 1952 \n```\n\nimport requests\n\nfrom bs4 import BeautifulSoup\n\nimport pandas as pd\n\nheaders = {\n\n\"user-agent\": \"Mozilla/5.0 (Windows NT 10.0; Win64; x64)\n\nAppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0\n\nSafari/537.36\"\n\n}\n\nurl = 'https://en.wikipedia.org/wiki/List_of_S%26P_500_companies'\n\ntext = requests.get(url, headers=headers).text\n\nsoup = BeautifulSoup(text, 'html.parser')\n\nFind the table\n\ntable = soup.find('table', id=\"constituents\")\n\nExtract the headers\n\nheaders $\\equiv$ []\n\nfor th in table.find_all('th')\n\nheaders append(th.text.strip())\n\nExtract the rows\n\nrows = []\n\nfor tr in table.find_all('tr')[1]: # Skip the header row\n\ncells = tr.find_all('td')\n\nrow = [cell.text.strip() for cell in cells]\n\nrows.append(row)\n\nCreate aDataFrame\n\ndf = pd.DataFrame(columns, columns=headers)\n\nprint(df)\n\n# Other search methods\n\n- Search by CSS class\n\ntable = soup.find_all('table', class='wikitable sortable')\n\n- Search by tag attributes\n\ntable = soup.find_all(\"table\", attrs={'class':'wikitable sortable'})\n\n- CSS Selector\n\ntable = soup.select('table')\ntable = soup.select('table[class=\"wikitable sortable\"', table)\ntable = soup.select '.wikitable'\ntable = soup.select '.wikitable.sortable'\n\n# LXML\n\n- Popular python library for parsing XPath and XML schema   \n- Official Doc: https://lxml.de/\n\npip install lxml  \npip install requests\n\n# What is XPath?\n\n- XPath is a query language to navigate the elements and attributes in an XML document   \nKey features:\n\n- Node Selection: XPath allows you to select nodes or elements in an XML document. For example, selecting elements by their tag name, attribute, or position.   \n- Path Expressions: Provides a way to navigate through elements and attributes in an XML document using path expressions.   \n- Syntax: Uses a compact, non-XML syntax to make it easier to write and read.\n\n# Common XPath Syntax\n\n- Basic Path: /root/child   \n- Selects the child element of the root element.   \n- Relative Path: //child   \n- Selects all child elements in the document.   \n- Attributes: //element [@attribute='value']   \n- Selects elements with a specific attribute value.   \n• Functions: //element[position(   )=1]   \n- Selects the first element.\n\n# XPath Example\n\n- Select all <li> elements: //li   \n- Select <li> elements with class \"item\": //li [@class='item']   \n- Select the second <li> element: //li[2]\n\n```txt\n<ul>\n    <li class=\"item\">Item 1</li>\n    <li class=\"item\">Item 2</li>\n    <li class=\"item\">Item 3</li>\n</ul> \n```\n\n![](images/7c7be642b2b3cfa2bc57aa8cd6abd75c49b41c233af785235e71a53b9cbd7963.jpg)\n\n# Extract stock list\n\n![](images/3c2b29caec8d5dd3c15cd4083dc23d27aec108c27d3deaa85b1410dc7510202c.jpg)\n\n<table><tr><td></td><td>Symbol</td><td>Security</td><td>GICS Sector</td><td>...</td><td>Date Added</td><td>CIK</td><td>Founded</td></tr><tr><td>0</td><td>MMM</td><td>3M</td><td>Industrials</td><td>...</td><td>1957-03-04</td><td>0000066740</td><td>1902</td></tr><tr><td>1</td><td>AOS</td><td>A. O. Smith</td><td>Industrials</td><td>...</td><td>2017-07-26</td><td>0000091142</td><td>1916</td></tr><tr><td>2</td><td>ABT</td><td>Abbott</td><td>Health Care</td><td>...</td><td>1957-03-04</td><td>0000001800</td><td>1888</td></tr><tr><td>3</td><td>ABBV</td><td>AbbVie</td><td>Health Care</td><td>...</td><td>2012-12-31</td><td>0001551152</td><td>2013 (1888)</td></tr><tr><td>4</td><td>ACN</td><td>Accenture</td><td>Information Technology</td><td>...</td><td>2011-07-06</td><td>0001467373</td><td>1989</td></tr><tr><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td></tr><tr><td>498</td><td>XYL</td><td>Xylem Inc.</td><td>Industrials</td><td>...</td><td>2011-11-01</td><td>0001524472</td><td>2011</td></tr><tr><td>499</td><td>YUM</td><td>Yum! Brands</td><td>Consumer Discretionary</td><td>...</td><td>1997-10-06</td><td>0001041061</td><td>1997</td></tr><tr><td>500</td><td>ZBRA</td><td>Zebra Technologies</td><td>Information Technology</td><td>...</td><td>2019-12-23</td><td>0000877212</td><td>1969</td></tr><tr><td>501</td><td>ZBH</td><td>Zimmer Biomet</td><td>Health Care</td><td>...</td><td>2001-08-07</td><td>0001136869</td><td>1927</td></tr><tr><td>502</td><td>ZTS</td><td>Zoetis</td><td>Health Care</td><td>...</td><td>2013-06-21</td><td>0001555280</td><td>1952</td></tr></table>\n\nfrom lxml import html\n\nimport requests\n\nimport pandas as pd\n\nheaders $\\equiv$ {\n\n\"user-agent\": \"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36\"\n\n# download web page\n\nurl = 'https://en.wikipedia.org/wiki/List_of_S%26P_500 Companies'\n\nresponse = requests.get(url, headers=headers)\n\nCheck that the request was successful\n\ntext = response(content\n\nload into html tree\n\ntree = html.fromstring(text)\n\nextract table rows from XPath\n\ntable_rows = tree XPath('//*[@id=\"constituents\"/tbody/tr']')\n\nparse the table rows\n\ndata = []\n\nfor row in table_rows[1]: # Skip header row\n\ncells = row.xpath('td')\n\nif len(cells) > 0:\n\ntry:\n\nsymbol $=$ cells[0].text_content() strip()\n\nsecurity $=$ cells[1].text_content().strip()\n\ngics sector = cells[2].text_content() strip()\n\ngics_sub_industry = cells[3] text_content().strip()\n\nheadquarters $=$ cells[4].text_content().strip()\n\ndate-added = cells[5].text_content() strip()\n\ncik = cells[6].text_content().strip()\n\nfounded $=$ cells[7].text_content().strip()\n\ndata append([symbol, security, gics_section, gics_sub_industry, headquarters,\n\ndate-added, cik, founded])\n\nexcept Exception as e:\n\npass\n\ncreate a DataFrame\n\ncolumns = ['Symbol', 'Security', 'GICS Sector', 'GICS Sub-Industry', 'Headquarters Location',\n\n'Date Added', 'CIK', 'Founded']\n\ndf = pd.DataFrame(data, columns=columns)\n\nprint the DataFrame\n\nprint(df)\n\n# Yahoo Finance\n\n# Yahoo Finance (https://finance.yahoo.com/)\n\n# Available datasets\n\nMarket data\n\nStock, ETF, Index   \nHistorical data (End-of-day, weekly, monthly)   \nReal-time data (up to 20 min delay)\n\n- Corporate actions (eg. dividends, splits)   \n- Company Profile (eg. Industry, sector, key executives, etc)   \n- Financial (eg. Income statement, Balance sheet, Cash flow)   \n- Analyst Estimates (earnings, revenue, growth)\n\n![](images/472e5f0bcde95378059087475009ef8ac58e78aac5373477f570f3f2c1832917.jpg)\n\n![](images/3b835223530d7f96591a41f807fddcf2d3462dfb66acf560785022335a67c90a.jpg)\n\n# Extract stock price\n\nimport requests\n\nfrom bs4 import BeautifulSoup\n\nimport pandas as pd\n\n```txt\nurl = 'https://finance.yahoo.com/quote/0700.HK/history/'  \ntext = requests.get(url).text \n```\n\nprint(text)\n\n```python\nsoup = BeautifulSoup(text, 'html.parser') \n```\n\nFind the table\n\n```txt\ntable - soup.find('table', class_= 'table yf-1m2i7s2') \n```\n\nExtract the headers\n\n[ \\text{headers} = [] ]\n\nfor th in table.find_all('th'):\n\nheaders.append(th.text.strip())\n\nExtract the rows\n\nrows = []\n\nfor tr in table.find_all('tr')[1]: # Skip the header row\n\ncells = tr.find_all('td')\n\nrow = [cell.text.strip() for cell in cells]\n\nrows.append(row)\n\nCreate a DataFrame\n\ndf = pd.DataFrame(columns, columns=headers)\n\nprint(df)\n\n```html\n<!DOCTYPE html> <html lang=\"en-us\"><head> <meta http-equiv=\"content-type\" content=\"text/html; charset=UTF-8\"> <meta charset=\"utf-8\"> <title>Yahoo</title> <meta name=\"viewport\" content=\"width-device-width,initial-scale=1,minimal-ui\"> <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge,chrome=1\"> <style> html { height: 100%; } body { background: #fafafc url(https://s.yimg.com/nn/img/sad-panda-201402200631.png) 50% 50%; background-size: cover; height: 100%; text-align: center; font: 300 18px \"helvetica neue\", helvetica, verdana, tahoma, arial, sans-serif; } table { height: 100%; width: 100%; table-layout: fixed; border-collapse: collapse; border-spacing: 0; border: none; } h1 { font-size: 42px; font-weight: 400; color: #40090; } p { color: #1A1A1A; } #message-1 { font-weight: bold; margin: 0; } #message-2 { display: inline-block; *display: inline; zoom: 1; max-width: 17em; _width: 17em; } </style> </script> <document.write('<img src=\"///geo.yahoo.com/b?s=1197757129&t='+new Date().getTime()+\"&src=aws&err_url=\"+encodeURIComponent(documents).URL)+&err=%<pssc>&test=\"+encodeURIComponent(\"%<Bucket)cqh[:200]\")+\"width=\"0px\" height=\"0px\"/\");var beacon = new Image(); beacon.src=\"///bcn.fp.yahoo.com/p?p?s=1197757129&t=\"+new Date().getTime()+\"&src=aws&err_url=\"+encodeURIComponent(documents).URL)+&err=%<pssc>&test=\"+encodeURIComponent(\"%<Bucket)cqh[:200]\"); </script> </head> </body> <!-- status code : 404 --> </-- Not Found on Server --> <table> <tbody><tr> <td> <img src=\"https://s.yimg.com/rz/p/yahoo_frontpage_en-US_s_f_p_205x58_frontpage.png\" alt=\"Yahoo Logo\"> <h1 style=\"margin-top:20px;\" Will be right back...</h1> <p id=\"message-1\" Thank you for your patience.</p> <p id=\"message-2\">Our engineers are working quickly to resolve the issue.</p> </td> </tr> </body></table> </body></html> \n```\n\n# Why it doesn’t work?\n\nyahoo!\n\nWill be right back…\n\nThank you for your patience.\n\nOur engineers are working quickly to resolve the issue.\n\n???\n\n# yfinance\n\nIt is an unofficial library for Yahoo Finance   \n- Project website: https://pypi.org/project/yfinance/\n\npip install yfinance\n\n# Extract stock info\n\n# . info()\n\nimport yfinance as yf\n\nticket = yf.Ticker(\"0700.HK\")\n\nget ticker basic info\n\nprint(ticker.info)\n\n{address1': 'Tencent Binhai Towers', 'address2': 'No. 33 Haitian 2nd Road Nanshan District', 'city': 'Shenzhen', 'zip': '518954', 'country': 'China', 'phone': '86 75 5860 13388', 'website': 'https://www.tencent.com', 'industry': 'Internet Content & Information', 'industryKey': 'internet-content-information', 'industryDisp': 'Internet Content & Information', 'sector': 'Communication Services', 'sectorKey': 'communication-services', 'sectorDisp': 'Communication Services', 'longBusinessSummary': \"Tencent Holdings Limited, an investment holding company, offers value-added services (VAS), online advertising, fintech, and business services in the People's Republic of China and internationally. It operates through VAS, Online Advertising, FinTech and Business Services, and Others segments. The company's consumers business provides communication and services, such as instant messaging and social network; digital content including online games, videos, live streaming, news, music, and literature; fintech services, which includes mobile payment, wealth management, loans, and securities trading; and various tools, such as network security management, browser, navigation, application management, email, etc. Its enterprise business comprises marketing solutions, which offers digital tools including user insight, creative management, placement strategy, and digital assets management; and cloud services, such as cloud computing, big data analytics, artificial intelligence, Internet of Things, security and other technologies for financial services, education, healthcare, retail, industry, transport, energy, and radio & television application. In addition, the company operates innovation business, which includes artificial intelligences; and discover and develops enterprise and next-generation technologies for food production, energy, and water management application. Tencent Holdings Limited was formerly known as Tencent (BVI) Limited and changed its name to Tencent Holding Limited in February 2004. The company was founded in 1998 and is headquartered in Shenzhen, the People's Republic of China.\", 'fullTimeEmployees': 104787, 'companyOfficers': [{'maxAge': 1, 'name': 'Mr. Huateng Ma', 'age': 51, 'title': 'Co-Founder, Chairman & CEO', 'yearBorn': 1972, 'fiscalYear': 2023, 'totalPay': 46131830, 'exercisedValue': 0}, 'unexercisedValue': 0}, {maxAge: 1, 'name': 'Mr. Chi Ping Lau', 'age': 50, 'title': 'President', 'yearBorn': 1973, 'fiscalYear': 2023, 'totalPay': 17832844, 'exercisedValue': 0, 'unexercisedValue': 1026971520}, {'maxAge': 1, 'name': Mr. Liqing Zeng', 'age': 53, 'title': Co-Founder & Advisor Emeritus', 'yearBorn': 1970, 'fiscalYear': 2023, 'exercisedValue': 0, 'unexercisedValue': 0}, {maxAge: 1, 'name': Mr. Chenye Xu', 'age': 52, 'title': Co-Founder & Chief Information Officer', 'yearBorn': 1971, 'fiscalYear': 2023, 'exercisedValue': 0, 'unexercisedValue': 0}, {maxAge: 1, 'name': Mr. Zhidong Zhang', 'age': 51, title': Co-Founder & Advisor Emeritus', yearBorn': 1972, fiscalYear': 2023, totalPay': 17342292, 'exercisedValue': 0, 'unexercisedValue': 0}, {'maxAge': 1, 'name': Mr. Yidan Chen', 'age': 52, title': Co-Founder & Advisor Emeritus', yearBorn': 1971, fiscalYear': 2023, unexercisedValue': 0}, {'maxAge': 1, 'name': Mr. Weibiao Zhan', age': 50, title': Managing Director', yearBorn': 1973, fiscalYear': 2023, exerischedValue': 0, unexercisedValue': 0}, {'maxAge': 1, 'name': Mr. Shek Hon Lo', age': 54}, title: CFO & Senior VP', yearBorn': 1969, fiscalYear': 2023, exercisedValue': 0, unexercisedValue': 0}, {'maxAge': 1, name: Mr. Yuxin Ren', age': 47, title': COO and President of Platform & Content Group & Interactive Entertainment Group', yearBorn': 1976, fiscalYear': 2023, exercisedValue': 0, unexercisedValue': 0}, {'maxAge': 1}, {name: Mr. Shan Lu', age': 48, title': Senior EVP and President of Technology & Engineering Group', yearBorn': 1975, fiscalYear': 2023, exerischedValue': 0, unexercisedValue': 0}], {'auditRisk': 10, boardRisk': 5}, {'compensationRisk': 10, shareHolderRightsRisk': 4, overallRisk': 10, governanceEpochDate': 1719792000}, {'compensationAsOfEpochDate': 1703980800, maxAge: 86400, priceHint: 3, previousClose: 369.2, open: 366.8, dayLow: 366.8, dayHigh: 373.0, regularMarketPreviousClose: 369.2, regularMarketOpen: 366.8, regularMarketDayLow: 366.8, regularMarketDayHigh: 373.0, dividendRate: 3.4, dividendYield: 0.0092', exDividendDate: 1715904000, payoutRatio: 0.1825, fiveYearAvgDividendYield: 0.45, beta: 0.557, trailingPE' : 29.052465,'forwardPE' : 14.704717,volume' :6796150,'regularMarketVolume' :6796150,'averageVolume' :22655010,'averageVolume10days' :17443391,'averageDailyVolume10Day' :17443391,'bid' :370.6,'ask' :370.8,'marketCap' :3466620305408,'fiftyTwoWeekLow' :260.2,'fiftyTwoWeekHigh' :401.0,'priceToSalesTrailing12Months' :5.6046114,'fiftyDayAverage' :369.028,'twoHundredDayAverage' :314.743,'trailingAnnualDividendRate' :3.086,'trailingAnnualDividendYield' :0.008358613,'currency' :HKD', enterpriseValue' :3475905445888,'profitMargins' :0.21222,'floatShares' :6226935680,'sharesOutstanding' :9343989760,'heldPercentInsiders' :0.33909,'heldPercentInstitutions' :0.23399,'impliedSharesOutstanding' :9424969728,'bookValue' :90.739,'priceToBook' :4.0886497,'lastFiscalYearEnd' :1703980800,'nextFiscalYearEnd' :1735603200,'mostRecentQuarter' :1711843200,'earningsQuarterlyGrowth' :0.621,'netIncomeToCommon' :131267002368,'trailingEps' :12.77,'forwardEps' :25.23,'pegRatio' :0.65,'lastSplitFactor' :5:1,'lastSplitDate' :1400112000,'enterpriseToRevenue' :5.62,'enterpriseToEbitda' :17.515,'52WeekChange' :0.10077524,'SandP52WeekChange' :0.23886502,'lastDividendValue' :3.4,'lastDividendDate' :1715904000,'exchange' : HKG', quoteType' :EQUITY', symbol' :O7OO.HK', underlyingSymbol' :O7OO.HK', shortName' TENGENT', longName' Tencent Holdings Limited', firstTradeDateEpochUtc: 1087349400,'timeZoneFullName' Asia/Hong Kong', timeZoneShortName' HKT', uuid' :341c2e2e-93bb-3ea-f-b5b-92ea7815949e', messageBoardId' :finmb_11042136', gmtOffSetMillseconds' :28800000,'currentPrice' :371.0,'targetHighPrice' :543.11,'targetLowPrice' :304.06,'targetMeanPrice' :463.24,'targetMedianPrice' :464.09,'recommendationMean' :1.7,'recommendationKey' : buy', numberOfAnalystOpinions' :47,'totalCash' :419042983936,'totalCashPerShare' :44.808,'ebitda' :198450003968,'totalDebt' :374007988224,'quickRatio' :1.213,'currentRatio' :1.451,'totalRevenue' :618530013184,'debtToEquity' :40.796,'revenuePerShare' :65.646,'returnOnAssets' : O.7O1I , returnOnEquity' : O.15278,'freeCashflow' : 143674867712,'operatingCashflow' : 232O15OoMgSdAy , earningsGrowth' : O.662,revenueGrowth' : O.063,grossMargins' : O.49925,'ebitdaMargins' : O.32O84,'operatingsMargins' : O.3295Ooo2 , financialCurrency':'CNY','trailIngPegRatio' : O.6468\n\n# Extract stock history\n\n• .history(period)   \navailable period ['1d', '5d', '1mo', '3mo', '6mo', '1y', '2y', '5y', '10y', 'ytd', 'max']\n\nimport yfinance as yf\n\nticket = yf.Ticker(\"0700.HK\")\n\n# download historical data\n\ndata = ticker-history(period='1mo')\n\nprint(data)\n\nOpen High Low Close Volume Dividends Stock Splits\n\nDate\n\n<table><tr><td>2025-12-22</td><td>00:00:00+08:00</td><td>620.0</td><td>621.5</td><td>610.0</td><td>614.5</td><td>13868060</td><td>0.0</td><td>0.0</td></tr><tr><td>2025-12-23</td><td>00:00:00+08:00</td><td>613.5</td><td>614.5</td><td>601.5</td><td>602.0</td><td>15623392</td><td>0.0</td><td>0.0</td></tr><tr><td>2025-12-24</td><td>00:00:00+08:00</td><td>602.5</td><td>602.5</td><td>602.5</td><td>602.5</td><td>0</td><td>0.0</td><td>0.0</td></tr><tr><td>2025-12-29</td><td>00:00:00+08:00</td><td>606.0</td><td>611.0</td><td>596.0</td><td>596.5</td><td>18502650</td><td>0.0</td><td>0.0</td></tr><tr><td>2025-12-30</td><td>00:00:00+08:00</td><td>598.5</td><td>601.0</td><td>594.0</td><td>600.0</td><td>13582535</td><td>0.0</td><td>0.0</td></tr><tr><td>2025-12-31</td><td>00:00:00+08:00</td><td>599.0</td><td>599.0</td><td>599.0</td><td>599.0</td><td>0</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-02</td><td>00:00:00+08:00</td><td>600.5</td><td>624.5</td><td>600.5</td><td>623.0</td><td>16200058</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-05</td><td>00:00:00+08:00</td><td>624.0</td><td>628.0</td><td>615.5</td><td>624.5</td><td>19947025</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-06</td><td>00:00:00+08:00</td><td>627.0</td><td>638.5</td><td>626.0</td><td>632.5</td><td>24168431</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-07</td><td>00:00:00+08:00</td><td>627.5</td><td>629.5</td><td>615.0</td><td>624.5</td><td>21378622</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-08</td><td>00:00:00+08:00</td><td>618.5</td><td>621.0</td><td>610.0</td><td>616.0</td><td>18742539</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-09</td><td>00:00:00+08:00</td><td>616.0</td><td>617.0</td><td>610.0</td><td>611.0</td><td>17813669</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-12</td><td>00:00:00+08:00</td><td>616.0</td><td>627.5</td><td>613.5</td><td>623.0</td><td>27530129</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-13</td><td>00:00:00+08:00</td><td>637.0</td><td>639.0</td><td>622.5</td><td>627.5</td><td>24212695</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-14</td><td>00:00:00+08:00</td><td>634.0</td><td>638.5</td><td>626.0</td><td>633.0</td><td>28677291</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-15</td><td>00:00:00+08:00</td><td>631.0</td><td>632.5</td><td>618.5</td><td>622.0</td><td>26194252</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-16</td><td>00:00:00+08:00</td><td>627.0</td><td>630.0</td><td>613.5</td><td>617.5</td><td>20616793</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-19</td><td>00:00:00+08:00</td><td>613.5</td><td>614.5</td><td>608.5</td><td>610.0</td><td>13233330</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-20</td><td>00:00:00+08:00</td><td>601.0</td><td>607.0</td><td>598.0</td><td>601.0</td><td>24332228</td><td>0.0</td><td>0.0</td></tr><tr><td>2026-01-21</td><td>00:00:00+08:00</td><td>598.0</td><td>605.5</td><td>598.0</td><td>600.0</td><td>11476120</td><td>0.0</td><td>0.0</td></tr></table>\n\n# Extract multiple stocks\n\n- yf.download(tickers, start, end)   \n- tickers separated by a space\n\nimport yfinance as yf\n\n```python\nticket = yf.Ticker(\"0700.HK\")\n\ndownload Tencent, HSBC,Apple\n\ndata = yf.download(\"0700.HK 0005.HK AAPL\", start=\"2025-01-01\", end=\"2025-06-30\")\n\nprint(data)\n\n<table><tr><td>Price\nTicker\nDate</td><td>Close\n0005.HK</td><td>0700.HK</td><td>AAPL</td><td>High\n0005.HK</td><td>...</td><td>Open\nAAPL</td><td>Volume\n0005.HK</td><td>0700.HK</td><td>AAPL</td></tr><tr><td>2025-01-02</td><td>71.669708</td><td>412.400024</td><td>242.752106</td><td>71.811724</td><td>...</td><td>247.809220</td><td>15851617.0</td><td>20733037.0</td><td>55740700.0</td></tr><tr><td>2025-01-03</td><td>71.006966</td><td>410.615601</td><td>242.264313</td><td>71.717035</td><td>...</td><td>242.264313</td><td>11755463.0</td><td>16843241.0</td><td>40244100.0</td></tr><tr><td>2025-01-06</td><td>71.527695</td><td>405.857117</td><td>243.896912</td><td>71.622369</td><td>...</td><td>243.210016</td><td>12886461.0</td><td>16869318.0</td><td>45045600.0</td></tr><tr><td>2025-01-07</td><td>71.291008</td><td>376.315002</td><td>241.119492</td><td>71.859062</td><td>...</td><td>241.886014</td><td>18577225.0</td><td>142920468.0</td><td>40856000.0</td></tr><tr><td>2025-01-08</td><td>71.906395</td><td>366.005035</td><td>241.607254</td><td>71.906395</td><td>...</td><td>240.830767</td><td>18019505.0</td><td>95456188.0</td><td>37628900.0</td></tr><tr><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td></tr><tr><td>2025-06-23</td><td>90.441856</td><td>504.000000</td><td>201.076660</td><td>90.638894</td><td>...</td><td>201.206392</td><td>11873908.0</td><td>14252887.0</td><td>55814300.0</td></tr><tr><td>2025-06-24</td><td>92.707825</td><td>509.500000</td><td>199.879181</td><td>93.101908</td><td>...</td><td>202.164363</td><td>29990019.0</td><td>17768858.0</td><td>54064000.0</td></tr><tr><td>2025-06-25</td><td>94.185631</td><td>512.500000</td><td>201.136536</td><td>95.367882</td><td>...</td><td>201.026766</td><td>36558811.0</td><td>17592461.0</td><td>39525700.0</td></tr><tr><td>2025-06-26</td><td>93.988586</td><td>513.000000</td><td>200.577698</td><td>94.481189</td><td>...</td><td>201.006787</td><td>10230801.0</td><td>15000643.0</td><td>50799100.0</td></tr><tr><td>2025-06-27</td><td>93.594505</td><td>513.000000</td><td>200.657532</td><td>93.840807</td><td>...</td><td>201.465827</td><td>14778679.0</td><td>15181667.0</td><td>73188600.0</td></tr></table>\n\n# Other attributes\n\nimport yfinance as yf\n\nmsft = yfTicker(\"MSFT\")\n\nget all stock info\n\nmsft.info\n\nget historical market data\n\nhist = msft_history(period=\"1mo\")\n\nshow meta information about the history (requires history() to be called first)  \nmsft_history_metadata\n\nshow actions (dividends, splits, capital gains)\n\nmsft/actions\n\nmsft-dividends\n\nmsft.splits\n\nmsft.capital_gains\n\nonly for mutual funds & etfs # show share count\n\nmsft.get_shares_full(start=\"2022-01-01\", end=None)\n\nshow financials:\n\n- income statement\n\nmsft.incomestmt\n\nmsftquarterly_incomestmt\n\nbalance sheet\n\nmsft balancesheet\n\nmsftquarterly_balance sheet\n\n-cash flow statement\n\nmsft cashflow\n\nmsftquarterly_cashflow\n\nsee `Ticker.get_incomestmt()` for more options\n\nshow holders\n\nmsft major holders\n\nmsft.institutional HOLDERS\n\nmsft.mutualfund HOLDERS\n\nmsft.insider_transactions\n\nmsft.insider_purchases\n\nmsft insider roster holders\n\nshow recommendations\n\nmsft.recommendations\n\nmsft.recommendations_summary\n\nmsft.upgrades_downgrades\n\nShow future and historic earnings dates, returns at most next 4 quarters and last 8 quarters by default.\n\nNote: If more are needed use msft.get_Earnings_dates limit=XX) with increased limit argument.\n\nmsft.earnings_dates\n\nshow ISIN code - *experimental*\n\nISIN = International Securities Identification Number\n\nmsft.isin\n\nshow options expirations\n\nmsftoptions\n\nshownews\n\nmsft.news\n\nget option chain for specific expiration\n\nopt = msft.option_chain('YYYY-MM-DD')\n\ndata available via: optcalls, opt.puts\n\n# Selenium\n\n- Selenium is a web automation tool that allows you to simulate user interactions, perform web scraping, and run automated tests on web applications.   \n- Official document: https://selenium-python.readthedocs.io/\n\n![](images/a30f86fba08fa30f4b4f345f57f8b1423eaf2896c8fdbb0b5005bdfcff235c85.jpg)\n\n# Webdrive download\n\n- Chrome: https://googlechromelabs.github.io/chrome-for-testing/#stable   \n- Edge: https://developer.microsoft.com/en-us/microsoft-edge/tools/webdriver?form=MA13LH   \n- Firefox: https://github.com/mozilla/geckodriver/releases\n\n# Quick Example\n\nfrom selenium import webdriver\n\nfrom selenium.webdriver.chrome.service import Service\n\nchrome.service = Service executable_path='path/to/chromedriver')\n\ndriver = webdriverchrome(service=chrome.service)\n\ndriver.get('http://example.com')\n\nprint(driver.title)\n\ndriver.quit()\n\n![](images/849155d10e110bd20730c47b8e24c18d774a16be777010d5098c6865c401f20d.jpg)\n\nDevTools listening on ws://127.0.0.1:49236/devtools/browser/f14841af-84e0-4681-9cf4eb2b55f3e68c Example Domain\n\n![](images/d914bcabdd67d558975e417d6aebfc5ba9c3f2346c44a9aad8388f8786bebea3.jpg)\n\n![](images/a1bd1b5e30f199e8036f9b015ef98919f0ade8dfbde9d3095c4f1f830cd0875c.jpg)\n\n![](images/4c822937ad176446b27cbeed6756c2c389b962efed266d17ff268e0c217e245d.jpg)\n\n![](images/4e8d987c19722fad1998fcc97acfc90cbccd56b68dac43ca94fcaf32bba7fef2.jpg)\n\n![](images/444bcf4fdabc545fffa5e7966859c76e1fbf404696960d8878b2bc5c78698960.jpg)\n\n![](images/299561f5e82a48c87d6312a1dde3b19784a2cb62f8dcdc275bcf69a5833574c3.jpg)\n\nexample.com\n\n![](images/f458f088683f4938303ef4b1e0145f759887e2ce2c49dcbbe1ca28690342f4a0.jpg)\n\n![](images/673ae2e31321c91e97cff212d83e9eb678b36e66dd886990cbd8393591b7395b.jpg)\n\n# Example Domain\n\nThis domain is for use in illustrative examples in documents. You may use the domain in literature without prior coordination or asking for permission.\n\nMore information...\n\n![](images/b58395ca996d55a4a1cf76cad574556541442251fb44f6174b15d37f188c0035.jpg)\n\n![](images/e09922e3106448df6d1c2c1b7e3ada39f3e27d73b3a034bc5d5ece6e1df4d854.jpg)\n\n![](images/edc06ede91432de4629cf1c723952b0e991cd5585cec90586782d3fc74a78ffc.jpg)\n\n![](images/76b4618c8e1c922f3837cf93ff096b70fa332d500b8b58642c66ea46e76da556.jpg)\n\n![](images/50f5fe132b3c6c9b47e6372dddbf6d4369c51a097fb8262595b958bf051e5a64.jpg)\n\n![](images/494f58d5ab359b208b103067dbb39853cb0c0a49943290e01375ab5817052c0c.jpg)\n\n![](images/bb414033ed3fd53d7efde7af14e61ef484238007f6311ecbce46dcda39db95b1.jpg)\n\n![](images/ad54a95d22ec02ea32b00b261bda56509dc326a36b298724283537439f9c8b84.jpg)\n\n![](images/ae95ba9a69382262ff460a8c5e98eaa0e2b8a031efbc8f329e3f369c6bc4ef9c.jpg)\n\n![](images/be4c6b284bea8ef0bdbe6f65a2a4a9ca668db7e990ddc603a700870a8ab4cac9.jpg)\n\n![](images/afbdbb3a42f99a20ea5208cd4e6736ed119eb8ab74c08d711179207dc37d56e3.jpg)\n\n![](images/c682dbed9f8be465a7b1b23b2d65ce9d05bb85d4f4875820a64af2de17183bdc.jpg)\n\n![](images/3f1fc43c9433a091c5817817dae83344ca28e633dd9809680411c3b2666825c1.jpg)\n\n![](images/d6efa3cf6b992d8455bcad6bca1ddaf6cf484aa00ce94290c006b425a8c04f10.jpg)\n\n![](images/88f3e72b6a29a9e86c160123932c70c58753c5836b5742c180f3d89968e59685.jpg)\n\n![](images/73b3b96e2ad1474db08822bea1679fe8a1bc74576a65e3d5756748ac514d7980.jpg)\n\n![](images/923a7ee6bda0c593922145ca9af521c12b93b645d05c2786e023cc7d15f3b9c3.jpg)\n\n# yahoo'finance\n\nSearch for news, symbols or companies\n\n![](images/f3c3a5aa71606fe23029594fc1d87445be1c97f1e66a1f067c663c29f3582034.jpg)\n\nNews\n\nFinance\n\nSports\n\nMore\n\nMy Portfolio\n\nNews\n\nMarkets\n\nSectors\n\nScreeners\n\nPersonal Finance\n\nVideos\n\nSummary\n\nHKSE -Delayed Quote·HKD\n\n# Tencent Holdings Limited (0700.HK)\n\n☆Follow\n\n$\\rightarrow^{\\dagger}$ Compare\n\nNews\n\nChart\n\nConversations\n\nStatistics\n\nHistorical Data\n\nProfile\n\nFinancials\n\nAnalysis\n\nOptions\n\nHolders\n\nSustainability\n\n# 365.000 -7.400 (-1.99%)\n\nAs of 9:15 AM GMT+8. Market Open.\n\nJul 02, 2023 - Jul 02, 2024\n\nHistorical Prices\n\nDaily\n\n# Start Trading >>\n\nPlus500 CFD Service Your capital is at risk.\n\nCurrency in HKD\n\n![](images/fc7a5dcb744a9caab8c0ceba719390b532226c0b9941b8589b80bbe8b339a927.jpg)\n\nDownload\n\ntable.table.svelte-ewueuo 1012.4 x 7904   \n\n<table><tr><td>Date</td><td>Open</td><td>High</td><td>Low</td><td>Close ①</td><td>Adj Close ①</td><td>Volume</td></tr><tr><td>Jul 2, 2024</td><td>371.600</td><td>376.000</td><td>370.800</td><td>365.000</td><td>365.000</td><td>15,561,097</td></tr><tr><td>Jun 28, 2024</td><td>371.600</td><td>376.000</td><td>370.800</td><td>372.400</td><td>372.400</td><td>15,561,097</td></tr><tr><td>Jun 27, 2024</td><td>379.600</td><td>380.000</td><td>372.200</td><td>374.400</td><td>374.400</td><td>17,513,758</td></tr><tr><td>Jul 26, 2024</td><td>379.600</td><td>380.000</td><td>377.000</td><td>382.000</td><td>382.000</td><td>11,996,997</td></tr></table>\n\n![](images/9f407602c5ad3ab78a3f4a2488859b3d80d4e6c4ad6bab40700e69595e997049.jpg)\n\n![](images/b17aaea813cc9065c3ef5501587224bdbc2cec3b0e57d1c66c38765a6a61c9b3.jpg)\n\nElements\n\nConsole\n\nSources\n\nNetwork\n\nPerformance\n\nMemory\n\nApplication\n\nLighthouse\n\nRecorder\n\nPerformance insights\n\n![](images/bdc5e772bc63c35625cd6ca74a70e214f377e5c0d7d6681693f2e045f5a652d0.jpg)\n\n# Extract stock price\n\n![](images/f28f7c3f19fb0f69774684ce61444b4e0b5ec8959da3b95d7745c48d99193bfa.jpg)\n\n<table><tr><td></td><td></td><td>Date</td><td>Open</td><td>High</td><td>Low</td><td>Close</td><td>Adj.</td><td>Close</td><td>Volume</td></tr><tr><td>0</td><td>Jan</td><td>21, 2026</td><td>598.000</td><td>605.500</td><td>598.000</td><td>601.000</td><td>601.000</td><td>12,012,950</td><td></td></tr><tr><td>1</td><td>Jan</td><td>20, 2026</td><td>601.000</td><td>607.000</td><td>598.000</td><td>601.000</td><td>601.000</td><td>24,332,228</td><td></td></tr><tr><td>2</td><td>Jan</td><td>19, 2026</td><td>613.500</td><td>614.500</td><td>608.500</td><td>610.000</td><td>610.000</td><td>13,233,330</td><td></td></tr><tr><td>3</td><td>Jan</td><td>16, 2026</td><td>627.000</td><td>630.000</td><td>613.500</td><td>617.500</td><td>617.500</td><td>20,616,793</td><td></td></tr><tr><td>4</td><td>Jan</td><td>15, 2026</td><td>631.000</td><td>632.500</td><td>618.500</td><td>622.000</td><td>622.000</td><td>26,194,252</td><td></td></tr><tr><td>...</td><td></td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td><td>...</td></tr><tr><td>242</td><td>Jan</td><td>27, 2025</td><td>388.000</td><td>398.000</td><td>388.000</td><td>395.600</td><td>392.177</td><td>23,605,938</td><td></td></tr><tr><td>243</td><td>Jan</td><td>24, 2025</td><td>383.200</td><td>392.400</td><td>382.200</td><td>390.600</td><td>387.220</td><td>24,079,119</td><td></td></tr><tr><td>244</td><td>Jan</td><td>23, 2025</td><td>383.000</td><td>386.000</td><td>379.600</td><td>381.200</td><td>377.901</td><td>21,256,810</td><td></td></tr><tr><td>245</td><td>Jan</td><td>22, 2025</td><td>385.000</td><td>388.600</td><td>381.000</td><td>383.400</td><td>380.082</td><td>17,859,069</td><td></td></tr><tr><td>246</td><td>Jan</td><td>21, 2025</td><td>392.000</td><td>392.200</td><td>386.400</td><td>387.400</td><td>384.048</td><td>25,249,668</td><td></td></tr></table>\n\n[247 rows x 7 columns]\n\n```python\nfrom selenium import webdriver   \nfrom selenium.webdriver.chrome.service import Service   \nfrom selenium.webdriver(common.by import By   \nfrom selenium.webdriver.common.keys import Keys   \nfrom selenium.webdriver.support.ui import WebDriverWait   \nfrom selenium.webdriver.support import expected_conditions as EC   \nfrom bs4 import BeautifulSoup   \nimport time   \nimport pandas as pd \n```\n\n# Path to your WebDriver (adjust the path to where you downloaded the WebDriver)  \nwebdriver_path = 'D:/program/chromedriver.exe'\n\nSet up the WebDriver   \nservice $=$ Service(webdriver_path)   \ndriver $=$ webdriver.Chrome(service $\\equiv$ service)   \nurl $=$ 'https://finance.yahoo.com/quote/0700.HK/history/'   \ndriver.get(url)   \n#Wait for the page to load   \nWebDriverWait(driver,10).until( ECpresence_of_element Located((By.CSS_SELECTOR,'table[class $\\coloneqq$ \"table yf-1m2i7s2 noDI hideOnPrint\"]))\n\n```python\nScroll down to load more data if necessary  \n# Adjust the range for more scrolling, if required  \nfor _ in range(3):  \n    driver_find_element(By.TAG_NAME, 'body').send_keys(Keys.END)  \ntime.sleep(2) \n```\n\n```python\nWait for the new data to load # Get the page source and parse it with BeautifulSoup  \npage_source = driver.page_source  \nsoup = BeautifulSoup(page_source, 'html.parser') \n```\n\n```python\nFind the historical data table  \ntable = soup.find('table', {'class': 'table yf-1jecxey noDI hideOnPrint'})  \nrows = table.find_all('tr') \n```\n\n```python\nExtract and print the data  \ndata = []  \nfor row in rows[1]: # Skip the header row  \ncols = row.find_all(\"td\")  \nif len(cols) > 6:  \n    date = cols[0].text  \n    open_price = cols[1].text  \n    high_price = cols[2].text  \n    low_price = cols[3].text  \n    close_price = cols[4].text  \n    adj_close_price = cols[5].text  \n    volume = cols[6].text  \n    data.append([date, open_price, high_price, low_price, close_price, adj_close_price, volume]) \n```\n\n```python\n# crate data frame\ndf = pd.DataFrame(data, columns=['Date', \"Open\", \"High\", \"Low\", \"Close\", \"Adj. Close\", \"Volume']) print(df) \n```\n\n```txt\nclose driver  \ndriver.quit() \n```\n\n# Selenium Options\n\n- https://www.selenium.dev/selenium/docs/api/rb/Selenium/WebDriver/Chromium/Options.html#add argument-instance method   \n- headless: open browser in background   \n- add_emulation: emulate opening the browser in different device\n\nfrom selenium import webdriver from selenium.webdriver.chrome/options import Options\n\noptions $=$ Options() options.add_argument('--headless $\\coloneqq$ new')\n\ndriver = webdriverchrome(CHROMEDRIVER_PATH, options=options)\n\n# Wrap up\n\nSeveral Python libraries are introduced for web scraping\n\n- BeautifulSoup   \n- xml   \n- Selenium   \n- yfinance - specific for Yahoo Finance\n\n# Limitation of Data Scraping\n\n# 1. Data Quality:\n\n- Website data may be incomplete and not up-to-dated\n\n# 2. Resource Intensive:\n\n- Consume lots of bandwidth and storage when scraping large amount of data   \n- Time consuming to fully load a website content to extract the data\n\n# 3. Technical Challenges:\n\n- Website structure may be changed frequently   \n- Many websites may implement anti-bot measures (eg. IP blocking, rate limit, etc)\n\n# Database Management\n\n# Why use a database for algo-trading?\n\n- Efficient storage and retrieval of large amounts of historical data for strategy backtesting   \n- Keep data in a local disk for future analysis instead of repeated web scraping   \nData integrity and consistency   \n- Advanced querying capabilities\n\n# SQL Syntax Overview\n\n- SELECT: Retrieve data from a table\n\nSELECT * FROM users;\n\n- INSERT: Add new data to a table\n\nINSERT INTO users (name, age) VALUES ('Amy', 18);\n\n- UPDATE: Modify existing data in a table\n\nUPDATE users SET age=28 WHERE name='Amy';\n\n- DELETE: Remove data from a table\n\nDELETE FROM users WHERE name='Amy';\n\n# Using SQLite in Python\n\n- SQLite is a C library that provides a lightweight, disk-based database.   \n- Python has built-in support for SQLite.\n\n![](images/9942a69e86925fd549c11c66b65a717914b9a5cab080f4d5a509d5c47b4bc56d.jpg)\n\n# Basic Usage\n\nimport sqlite3\n\nConnect to a database (or create one if it doesn't exist) conn = sqlite3.connect('example.db')\n\nCreate a cursor object  \ncursor = conn.cursor()\n\nExecute SQL queries using the cursor object cursor.exec('SELECT * FROM users')\n\nFetch all results rows = cursor].[fetchall()\n\nClose the connection conn.close()\n\n# Create a Table\n\n# SQL Syntax\n\nCREATE TABLE users (\nid INTEGER PRIMARY KEY, name TEXT NOT NULL, age INTEGER);\n\n# Python Example\n\nimport sqlite3   \nconn $=$ sqlite3.connect('example.db') cursor $=$ conn.cursor()   \n#Create a table cursor.execute(\" CREATE TABLE users ( id INTEGER PRIMARY KEY, name TEXT NOT NULL, age INTEGER ) \") conn.commit() conn.close()\n\n# Inserting Data\n\n# SQL Syntax\n\nINSERT INTO users (name, age) VALUES ('Alice', 30);\n\n# Python Example\n\nconn = sqlite3.connect('example.db')\n\ncursor = conn.cursor()\n\nInsert data\n\ncursor.execute(\n\nINSERT INTO users (name, age) VALUES (?, ?)\n\n```\n'', ('Alice', 30))\n\nconn.commit()\n\nconn.close()\n\n# Querying Data\n\n# SQL Syntax\n\nSELECT * FROM users WHERE age > 25;\n\n# Python Example\n\nconn = sqlite3.connect('example.db')  \ncursor = conn.cursor()\n\nQuery data  \ncursor.exec('SELECT * FROM users WHERE age > 25')  \nrows = cursor_fetchall()\n\nfor row in rows: print(row)\n\nconn.close()\n\n# Updating Data\n\n# SQL Syntax\n\nUPDATE users SET age = 31 WHERE name = 'Alice';\n\n# Python Example\n\nconn = sqlite3.connect('example.db')\n\ncursor = conn.cursor()\n\nUpdate data\n\ncursor.execute(\n\nUPDATE users SET age = ? WHERE name = ?\n\n```\n'', (31, 'Alice'))\n\nconn.commit()\n\nconn.close()\n\n# Deleting Data\n\n# SQL Syntax\n\nDELETE FROM users WHERE name = 'Alice';\n\n# Python Example\n\nconn = sqlite3.connect('example.db')\n\ncursor = conn.cursor()\n\nDelete data\n\ncursor.execute(\n\nDELETE FROM users WHERE name = ?\n\n```\n'', ('Alice'),))\n\nconn.commit()\n\nconn.close()\n\n# Querying Data with table join\n\nSuppose we want to know the total transactions done by each client.\n\nSELECT A.id, A.name, count(B.*)\n\nFROM users A\n\nINNER JOIN transactions B ON A.id = B.user_id\n\nGROUP BY A.id, A.name;\n\n# users\n\nid (Primary Key)\n\nname (e.g., NYSE, NASDAQ)\n\nage\n\n# transactions\n\ntxn_id (Primary Key)\n\nuser_id\n\ntrade_time\n\nInstrument\n\nbuysell\n\nprice\n\nqty\n\n# SQLite Browser\n\n- SQLite Browser (https://sqlitebrowser.org/) is an open sourced tool to create, edit, search and visualize SQLite database files.\n\n![](images/a9c7cf6348381ba398b8ca365feb73e2ab37bc5427854b2d4b2b554e02ac09c9.jpg)\n\n# Scrape Market Data into database\n\nimportsqlite3\n\nimport yfinance as yf\n\nconn =sqlite3.connect('D:/example.db')\n\ncursor = conn.cursor()\n\ncreate table\n\ncursor.execute(\n\nCREATE TABLE market_candles (\n\nsymbol text,\n\ntimestamp text,\n\nopen_price float\n\nhigh_price float,\n\nlow_price float,\n\nclose_price float,\n\nvolume float\n\n）\n\n# download data from yahoo finance\n\nsymbol = \"0700.HK\"\n\n```c\nticket = yf.Tickersymbol()\n\ndata = ticker_history(period='max')\n\nsave to database\n\nfor index, row in data.iterrrows():\n\nt = index\n\no = row['Open']\n\nh = row['High']\n\n$\\mathbf{l} = \\mathrm{row}[L_{\\mathrm{ow}}]$\n\nc = row['Close']\n\nv = row['Volume']\n\nsql = \"\"INSERT INTO market_candles VALUES {'symbol'},{'t'},{'o'},{'h'},{'l'},{'c'},{'v'})\"\".\"format(symbol=symbol,t=t,o=o,h=h,l=l,c=c,v=v)\n\ncursor execute(sql)\n\nclose database\n\nconn.commit()\n\nconn.close()\n\n# Querying market data from database\n\n![](images/6729b25b1fbb0f773fce5e015831b0c2b2da9c0fffa055c9976b383dc1669f8e.jpg)\n\n![](images/948c4c98456fede4289f0d7b50fe1f233c008d8a71485661e1369e694bc3f370.jpg)\n\n![](images/861d6153e0ade541777e6cf0d3c9a068927e8dd99188a2af9f7e40c906ef457d.jpg)\n\n![](images/3962a561dcfcfa7220f9ed342be6a3098bba04e1baa5349089f91ccbc23154f8.jpg)\n\n![](images/58d5b3c2e7c12cbf2fdc9953c883272ba185abb6ac44501151766c207cfb7c48.jpg)\n\n![](images/5a2a7f00b12130bc5741dd3cddf752aec6fe4a2a40b66d4901a6612f9a5d1c38.jpg)\n\n![](images/0d240ce45fff12e753bdf8b551ba88620477d6abd94f7ee02179f7e067b0b66f.jpg)\n\n```csv\n('0700.HK', '2025-06-02 00:00:00+08:00', 493.0, 499.20001220703125, 489.6000061035156, 498.3999938964844, 13086586.0)  \n('0700.HK', '2025-06-03 00:00:00+08:00', 504.0, 505.5, 501.0, 505.0, 14609189.0)  \n('0700.HK', '2025-06-04 00:00:00+08:00', 510.0, 513.0, 507.0, 512.0, 18378499.0)  \n('0700.HK', '2025-06-05 00:00:00+08:00', 517.5, 517.5, 509.0, 515.0, 19329282.0)  \n('0700.HK', '2025-06-06 00:00:00+08:00', 515.5, 516.5, 511.0, 515.0, 13137101.0)  \n('0700.HK', '2025-06-09 00:00:00+08:00', 520.0, 521.0, 512.5, 518.0, 18481403.0)  \n('0700.HK', '2025-06-10 00:00:00+08:00', 519.5, 520.0, 508.5, 513.5, 17039003.0)  \n('0700.HK', '2025-06-11 00:00:00+08:00', 517.0, 518.0, 514.5, 518.0, 14784979.0)  \n('0700.HK', '2025-06-12 00:00:00+08:00', 518.0, 518.0, 508.0, 510.0, 13368O48. O)  \n('O7OO.HK', '2O25-OS-13 Oo:Oo:Oo+Oo:Oo', 51O.5, 5I5.5, 5O6. O, 5IO. O, 19O8S31O.O)  \n('O7OO.HK', '2O25-OS-16 Oo:Oo:Oo+Oo:Oo', 5O7. O, 5I2. O, 5O4.5, 5O9.5, 1378432O.O)  \n('O7OO.HK', '2O25-OS-17 Oo:Oo:Oo+Oo:Oo', 5I4. O, 5I4. O, 5O6.5, 5I3.5, I1S24O31.O)  \n('O7OO.HK', '2O25-OS-18 Oo:Oo:Oo+Oo:Oo', 5Ioo. O, 5Ii. O, 5O3.5, 5O8. O, I5I87S1S.O)  \n('O7OO.HK', '2O25-OS-19 Oo:Oo:Oo+Oo:Oo', 5O3. O, 5O6. O, 496. O, 498. O, I9387767.O)  \n('O7OO.HK', '2O25-OS-2O Oo:Oo:Oo+Oo:Oo', 5O4.5, 5OIS. O, 496. O, SIS. O, 2O62227S.O)  \n('O7OO.HK', '2O25-OS-23 Oo:Oo:Oo+Oo:Oo', 498.2OOU122OT312S, SIO4.O, 494.6OOU61OT3S1S6, SIO4.O, I42S2887.O)  \n('O7OO.HK', '2O25-OS-24 Oo:Oo:Oo+Oo:Oo', 5O8.O, 5Ii.I,O4.O, SIO9.S, I77688S8.O)  \n('O7OO.HK', '2O25-OS-2S Oo:Oo:Oo+Oo:Oo', 5Ii.o., Iiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., Ioiio.o., I \n```\n\nimportsqlite3\n\n```python\nconn = sqlite3.connect('D:/example.db')  \ncursor = conn.cursor() \n```\n\nquery table\n\n```txt\nsql = \"\"SELECT * \n```\n\nFROM market_candles\n\n```txt\nWHERE symbol='0700.HK' AND timestamp>'='2025-06-01' AND timestamp'<='2025-06-30' \n```\n\nORDER BY timestamp;\n\n```txt\nfor row in cursor.exec (sql): print (row) \n```\n\n```txt\nclose database conn.close() \n```\n\n# Close vs Adjusted Close\n\n![](images/f2f1c716a05a505ac435a34535151e3a678335ba52e21bf9af46969dd402fe1e.jpg)\n\n# Close vs Adjusted Close\n\n- The original price series may show sudden jumps or drops due to corporate events (e.g. dividends, stock splits, reverse splits).   \n- After a dividend payment, the stock price typically drops by $S' = S - D$   \n- Following a stock split (e.g. 2:1 split), the stock price decreases by a factor of the split ratio (i.e. $S' = S / 2$ )   \n- Adjusted close prices are backward updated to account for these corporate events\n\n- so the values of the adjusted close series may change from time to time\n\n# How Yahoo Finance adjust data?\n\n![](images/d0e9e60f1c68af21f4e68f878545456865fd740a43cbab5f35e3cd0ec0439940.jpg)\n\n# Multipliers\n\nSplit multipliers are determined by the split ratio.\n\nFor example:\n\nIn a 2 for 1 split, the pre-split data is multiplied by 0.5.   \nIn a 4 for 1 split, the pre-split data is multiplied by 0.25.   \n- In a 1 for 5 reverse split, the pre-split data is multiplied by 5.\n\nDividend multipliers are calculated based on dividend as a percentage of the price, primarily to avoid negative historical pricing.\n\nFor example:\n\n- If a $0.08 cash dividend is distributed on Feb 19 (ex- date), and the Feb 18 closing price is$ 24.96, the predividend data is multiplied by (1-0.08/24.96) = 0.9968.   \n- If a $2.40 cash dividend is distributed on May 12 (ex- date), and the May 11 closing price is$ 16.51, the predividend data is multiplied by (1-2.40/16.51) = 0.8546.   \n- If a $1.25 cash dividend is distributed on Jan 25 (ex-date), and the Jan 24 closing price is$ 51.20, the predividend data is multiplied by (1-1.25/51.20) = 0.9756.\n\nHere's how split and dividend multipliers are calculated and applied to determine adjusted close prices.\n\nThe first table shows historical prices and the dates of a split and a dividend. The second table shows the calculations.\n\nThe multipliers we'll use are derived from the split and dividend:\n\n- Split Multiplier = 0.5   \nDividend Multiplier $= 1 - (0.08 / 24.96) = 0.9968$\n\nUsing these split and dividend multipliers, adjusted close prices are calculated for dates prior to the split and prior to the dividend ex-date.\n\nThe close price for 2/16 and 2/17 are adjusted for both the split on 2/18 and the ex-dividend date, 2/21.   \nThe close price for 2/18 through 2/20 are adjusted for the ex-dividend date, 2/21.   \n- The close price for 2/18 through 2/20 don't need adjustment for the split that occurred before close on 2/18.   \nThe close price for 2/21 and 2/22 aren't adjusted because they're after the split and ex-dividend dates.\n\n<table><tr><td>Date</td><td>Close, Dividend, or Split</td></tr><tr><td>2/16</td><td>Close = 46.99</td></tr><tr><td>2/17</td><td>Close = 48.30</td></tr><tr><td>2/18</td><td>Split = 2:1\nClose = 24.96</td></tr><tr><td>2/19</td><td>Close = 24.91</td></tr><tr><td>2/20</td><td>Close = 24.95</td></tr><tr><td>2/21</td><td>Dividend = 0.08 (ex-date)\nClose = 24.53</td></tr><tr><td>2/22</td><td>Close = 24.54</td></tr></table>\n\n<table><tr><td>Date</td><td>Adjusted close calculation</td></tr><tr><td>2/16</td><td>0.5 * 0.9968 * 46.99 = 23.42</td></tr><tr><td>2/17</td><td>0.5 * 0.9968 * 48.30 = 24.07</td></tr><tr><td>2/18</td><td>0.9968 * 24.96 = 24.88</td></tr><tr><td>2/19</td><td>0.9968 * 24.91 = 24.83</td></tr><tr><td>2/20</td><td>0.9968 * 24.95 = 24.87</td></tr><tr><td>2/21</td><td>24.53</td></tr><tr><td>2/22</td><td>24.54</td></tr></table>\n\n# Any adjustments on OHL data?\n\n![](images/ae583c8e47d60a25a2a6843a30b3a19478a511f62f8c5b40336f21e07b50b709.jpg)\n\nfinance.yahoo.com/quote/AAPL/history/?period1=345479400&period2=1738642197\n\n# yahoo finance\n\nSearch for news, symbols or companies\n\n![](images/38fec4f456782dc74b68eba9fdf4545a8c97efcd8403707f3bc784d88fad7101.jpg)\n\nNews\n\nFinance\n\nSports\n\nMg\n\nMy Portfolio\n\nNews\n\nMarkets\n\nResearch\n\nPersonal Finance\n\nVideos\n\nStreaming Now\n\n# AAPL\n\nApple Inc.\n\n![](images/ec081348c0e94ab176e7ab788ca3e4084105f4a49ca43405f19b6cfcdb93a5d8.jpg)\n\n228.85 +0.37% (\n\nSummary\n\nNews\n\nChart\n\nConversations\n\nStatistics\n\nHistorical Data\n\nProfile\n\nFinancials\n\nAnalysis\n\n<table><tr><td>Sep 10, 2020</td><td>120.36</td><td>120.50</td><td>112.50</td><td>113.49</td><td>110.79</td><td>182,274,400</td></tr><tr><td>Sep 9, 2020</td><td>117.26</td><td>119.14</td><td>115.26</td><td>117.32</td><td>114.52</td><td>176,940,500</td></tr><tr><td>Sep 8, 2020</td><td>113.95</td><td>118.99</td><td>112.68</td><td>112.82</td><td>110.13</td><td>231,366,600</td></tr><tr><td>Sep 4, 2020</td><td>120.07</td><td>123.70</td><td>110.89</td><td>120.96</td><td>118.08</td><td>332,607,200</td></tr><tr><td>Sep 3, 2020</td><td>126.91</td><td>128.84</td><td>120.50</td><td>120.88</td><td>118.00</td><td>257,599,600</td></tr><tr><td>Sep 2, 2020</td><td>137.59</td><td>137.98</td><td>127.00</td><td>131.40</td><td>128.27</td><td>200,119,000</td></tr><tr><td>Sep 1, 2020</td><td>132.76</td><td>134.80</td><td>130.53</td><td>134.18</td><td>130.98</td><td>151,948,100</td></tr><tr><td>Aug 31, 2020</td><td colspan=\"6\">4:1 Stock Splits</td></tr><tr><td>Aug 31, 2020</td><td>127.58</td><td>131.00</td><td>126.00</td><td>129.04</td><td>125.97</td><td>225,702,700</td></tr><tr><td>Aug 28, 2020</td><td>126.01</td><td>126.44</td><td>124.58</td><td>124.81</td><td>121.83</td><td>187,630,000</td></tr><tr><td>Aug 27, 2020</td><td>127.14</td><td>127.49</td><td>123.83</td><td>125.01</td><td>122.03</td><td>155,552,400</td></tr></table>\n\n# Should we keep the original or the adjusted price in database?\n\n# Pros:\n\n- Adjusted close provides a more accurate reflection of a stock's value over time   \n- Adjusted close is ideal for long-term analysis and backtesting strategies, as it smoothes out price jumps from events   \n- Ensure consistency across datasets, especially when comparing stocks with varying corporate events.\n\n# - Cons:\n\n- Storing both may require more space   \n- Need to regularly update the whole adjusted close series for new events.   \n- Adjusted close is not the actual price observed in the market. Modelling on adjusted close could be inaccurate.\n\n# Database Design\n\n- Suppose you are going to build a database to store all market data history from global stocks exchanges. How would you structure the database?\n\n# Database Design (1)\n\n![](images/578eed21aa6bba80bd01e1ba2968938dfc8439474f4e8f3359d5320438b7c70a.jpg)\n\n# Potential issues for design 1\n\n- The table size of “Market Data Table” will be very large as all stock data is put under the same table   \n- Data backup and data query speed from \"Market Data Table\" could be very slow\n\n# Database Design (2)\n\nSplit the Market Data Table into monthly or quarterly partitions based on the date field.\n\n![](images/7c7a327696fbd2d7a195d588f8e398b32d3148a6fec77808362a25b91b1b46b7.jpg)\n\n# Potential issues for design 2\n\n- Difficult to conduct data aggregation/summary for cross month analysis   \n- Need a function to generate dynamic SQL statements due to different market data table name\n\n# Database Design (3)\n\nSplit the Market Data Table into partitions based on the stock_id field.\n\n![](images/8d4b414d2659d1b0efc96d82a32e990c9fdca1eda08f9bc71d988f266d23cc8c.jpg)\n\n# Potential issues for design 3\n\n- Difficult for cross assets analysis   \n- Need a function to generate dynamic SQL statements due to different market data table name\n\n# What would be the best design?\n\n![](images/0d89f10494a5432d07885e1a9957ccdf165637628352265b40c375f8ce6e3086.jpg)\n\n# Database Design\n\n- There is no single design that is perfect for all situations!   \n- It depends on our common use cases.\n\n- For backtest purpose, partition by instruments could be a good choice   \n- For data analysis purpose, partition by date could be a better design\n\n# Database Design\n\n- Always need to strike a balance between\n\nStorage Space\n\n![](images/c4ce72002946ca513ab3ed3339b91214de8b1cb656ed49c2b2b56cf4deca6be7.jpg)\n\n![](images/a620dde1fa17859840e7e233580433ea0febf527595c01fa3af191568b8fbc89.jpg)\n\nRuntime Speed\n\n![](images/05dd869a766612222592c05353018cb3ac427b9f47b01a91e061c4ec6c3d6f97.jpg)\n\nMemory\n\n# Key Takeaways\n\n- Learn about the basic structure of a website (i.e. HTML, CSS, Javascript)   \n- Apply various python libraries for web scraping\n\n- BeautifulSoup   \n- xml   \n- Selenium   \n- yfinance\n\n- Understand the limitations of web scraping   \n- Learn how to create and manage a SQLite database using python   \n- Considerations for design a database for financial market data",
  "step_assets": [
    {
      "question_bank": {
        "coverage_map": [
          {
            "coverage_tag": "web_scraping_definition",
            "covered_by": [
              "qf_flash_web_scraping_def",
              "qf_quiz_web_scraping_use_case"
            ],
            "description": "网络爬取的定义：从网站自动提取数据的过程。"
          },
          {
            "coverage_tag": "web_scraping_algo_trading_use_cases",
            "covered_by": [
              "qf_flash_web_scraping_use_cases",
              "qf_quiz_web_scraping_use_case",
              "qf_long_web_scraping_use_cases"
            ],
            "description": "网络爬取在算法交易中的常见用例：实时数据收集、市场情绪分析、公司分析、另类数据。"
          },
          {
            "coverage_tag": "data_as_fuel_for_algo_trading",
            "covered_by": [
              "qf_flash_data_fuel",
              "qf_quiz_data_fuel"
            ],
            "description": "数据是算法交易的燃料，没有数据策略就是空谈。"
          }
        ],
        "flashcard_families": [
          {
            "concept_key": "web_scraping_definition",
            "coverage_tags": [
              "web_scraping_definition"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_web_scraping_def",
            "learning_goal": "学生能准确说出网络爬取的定义。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "网络爬取的核心定义：自动化地从网站提取数据。",
            "term_refs": [
              {
                "display": "网络爬取",
                "en": "Web Scraping"
              }
            ],
            "variants": [
              {
                "back": "从网站自动提取数据的过程。",
                "estimated_seconds": 8,
                "explanation": "网络爬取是一种自动化技术，用于从网页中提取结构化或非结构化数据。",
                "front": "网络爬取（Web Scraping）的核心定义是什么？",
                "question_id": "q_flash_web_scraping_def_v1"
              },
              {
                "back": "自动从网站提取数据，例如股价、新闻等。",
                "estimated_seconds": 8,
                "explanation": "网络爬取替代了手动收集数据的工作，提高了效率和准确性。",
                "front": "在算法交易中，网络爬取主要用于什么目的？",
                "question_id": "q_flash_web_scraping_def_v2"
              }
            ]
          },
          {
            "concept_key": "web_scraping_use_cases",
            "coverage_tags": [
              "web_scraping_algo_trading_use_cases"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_web_scraping_use_cases",
            "learning_goal": "学生能列举网络爬取在算法交易中的至少两个应用场景。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "网络爬取在算法交易中的具体应用场景。",
            "term_refs": [
              {
                "display": "算法交易",
                "en": "Algorithmic Trading"
              }
            ],
            "variants": [
              {
                "back": "市场情绪（新闻、论坛、社交媒体）。",
                "estimated_seconds": 10,
                "explanation": "通过爬取新闻和社交媒体数据，可以分析市场情绪，辅助交易决策。",
                "front": "除了实时股价，网络爬取在算法交易中还能用于分析什么？",
                "question_id": "q_flash_web_scraping_use_cases_v1"
              },
              {
                "back": "天气数据、网络流量数据等。",
                "estimated_seconds": 10,
                "explanation": "另类数据是指非传统金融数据，如天气、网络流量，可用于预测市场趋势。",
                "front": "网络爬取可以收集哪一类“另类数据”用于算法交易？",
                "question_id": "q_flash_web_scraping_use_cases_v2"
              }
            ]
          },
          {
            "concept_key": "data_as_fuel",
            "coverage_tags": [
              "data_as_fuel_for_algo_trading"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_data_fuel",
            "learning_goal": "学生能理解数据在算法交易中的核心地位。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "数据在算法交易中的比喻性描述。",
            "term_refs": [
              {
                "display": "算法交易",
                "en": "Algorithmic Trading"
              }
            ],
            "variants": [
              {
                "back": "燃料。没有数据，策略就是空谈。",
                "estimated_seconds": 6,
                "explanation": "这个比喻强调了数据是驱动算法交易策略的基础。",
                "front": "在算法交易中，数据被比喻成什么？",
                "question_id": "q_flash_data_fuel_v1"
              },
              {
                "back": "因为算法交易依赖数据来识别机会、回测策略和执行交易。",
                "estimated_seconds": 10,
                "explanation": "数据是算法交易决策的依据，缺乏数据则无法进行有效的分析和执行。",
                "front": "为什么说“没有数据，策略就是空谈”？",
                "question_id": "q_flash_data_fuel_v2"
              }
            ]
          }
        ],
        "lesson_id": "L2",
        "longform_families": [
          {
            "concept_key": "web_scraping_applications",
            "coverage_tags": [
              "web_scraping_algo_trading_use_cases"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_web_scraping_use_cases",
            "learning_goal": "学生能解释网络爬取在算法交易中的至少三个不同应用场景，并说明其价值。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "short_explain",
            "term_refs": [
              {
                "display": "网络爬取",
                "en": "Web Scraping"
              },
              {
                "display": "算法交易",
                "en": "Algorithmic Trading"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "场景1：实时数据收集",
                  "场景2：市场情绪分析",
                  "场景3：公司分析或另类数据"
                ],
                "question_id": "q_long_web_scraping_use_cases_v1",
                "reference_answer": [
                  "1. 实时数据收集：自动从财经网站抓取股票价格、经济指标等实时数据，为交易策略提供输入。",
                  "2. 市场情绪分析：爬取新闻、论坛和社交媒体内容，分析市场对特定股票或行业的整体情绪，辅助判断市场走向。",
                  "3. 公司分析：自动收集公司公告、财报、管理层变动等信息，用于基本面分析。或者，收集天气、网络流量等另类数据，用于预测某些行业（如农业、零售）的表现。"
                ],
                "rubric_points": [
                  "正确识别并解释实时数据收集（如股价、经济指标）",
                  "正确识别并解释市场情绪分析（如新闻、社交媒体）",
                  "正确识别并解释公司分析（如财报、公告）或另类数据（如天气）"
                ],
                "stem": "请列举并简要解释网络爬取在算法交易中的三个不同应用场景。"
              },
              {
                "estimated_seconds": 150,
                "prompt_blocks": [
                  "数据来源",
                  "爬取内容",
                  "数据如何用于策略"
                ],
                "question_id": "q_long_web_scraping_use_cases_v2",
                "reference_answer": [
                  "我会使用网络爬虫定期（例如每5分钟）从Twitter和几个主要财经新闻网站爬取数据。",
                  "爬取的内容将包括包含特定股票代码（如AAPL、TSLA）的推文和新闻标题。",
                  "然后，我会使用一个简单的自然语言处理模型对这些文本进行情感分析，计算正面和负面词汇的比例。如果某只股票的正面情绪比例在短时间内急剧上升并超过一个预设阈值，我的策略就会生成一个买入信号。反之，如果负面情绪占主导，则可能生成卖出信号。"
                ],
                "rubric_points": [
                  "明确指出数据来源（如Twitter、财经新闻网站、Reddit论坛）",
                  "描述爬取的具体内容（如包含特定股票代码的推文、新闻标题、评论）",
                  "解释如何将爬取的数据转化为交易信号（如通过情感分析计算正面/负面比例，当正面情绪超过阈值时买入）"
                ],
                "stem": "假设你正在开发一个基于市场情绪的短线交易策略，请描述你将如何使用网络爬取来获取所需数据，并解释这些数据如何驱动你的策略。"
              }
            ]
          }
        ],
        "quiz_families": [
          {
            "concept_key": "web_scraping_use_case_identification",
            "coverage_tags": [
              "web_scraping_definition",
              "web_scraping_algo_trading_use_cases"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_web_scraping_use_case",
            "learning_goal": "学生能在具体情境中判断是否应该使用网络爬取。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "网络爬取",
                "en": "Web Scraping"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 20,
                "explanation": "网络爬取正是为了自动从网站提取数据而设计的。手动计算、绘图和编写策略逻辑都不是爬取的核心功能。",
                "options": [
                  "手动计算股票平均价格",
                  "自动从财经网站收集每日收盘价",
                  "用 Excel 绘制图表",
                  "编写交易策略的代码逻辑"
                ],
                "question_id": "q_quiz_web_scraping_use_case_v1",
                "stem": "以下哪种情况最适合使用网络爬取？"
              },
              {
                "answer": 1,
                "estimated_seconds": 25,
                "explanation": "网络爬虫可以自动化地、高效地从多个来源提取数据，是处理此类重复性任务的理想工具。",
                "options": [
                  "每天手动阅读所有博客",
                  "编写一个网络爬虫自动抓取博客内容并分析关键词",
                  "订阅付费的新闻摘要邮件",
                  "使用 Excel 手动记录每篇博客的观点"
                ],
                "question_id": "q_quiz_web_scraping_use_case_v2",
                "stem": "一个量化分析师需要每天监控10个财经博客的市场情绪，最有效的方法是？"
              }
            ]
          },
          {
            "concept_key": "data_importance",
            "coverage_tags": [
              "data_as_fuel_for_algo_trading"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_data_fuel",
            "learning_goal": "学生能判断关于数据在算法交易中重要性的陈述是否正确。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "true_false",
            "term_refs": [
              {
                "display": "算法交易",
                "en": "Algorithmic Trading"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "数据是算法交易的燃料。没有高质量的数据，再好的策略也无法被有效验证和执行。两者同等重要。",
                "options": [
                  "正确",
                  "错误"
                ],
                "question_id": "q_quiz_data_fuel_v1",
                "stem": "在算法交易中，拥有一个优秀的交易策略比拥有高质量的数据更重要。"
              },
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "网络爬取是一种重要的数据获取方式，但并非唯一。还可以通过API、数据库、数据供应商等途径获取数据。",
                "options": [
                  "正确",
                  "错误"
                ],
                "question_id": "q_quiz_data_fuel_v2",
                "stem": "网络爬取是算法交易中获取数据的唯一途径。"
              }
            ]
          }
        ],
        "source": {
          "coverage_checklist": "L2: Data scraping and database management with Python - Agenda: Understand the basic website structure, Web scraping using different python libraries, The limitations of web scraping, Create a simple database using python, Understand the concept of adjusted price, Database design for financial market data storage",
          "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
          "lesson_map": "L2 guided_story steps: step1: What is web scraping and why it matters for algorithmic trading",
          "plain_text": "pipeline/1-plain/L2/plain.txt",
          "related": [
            "pipeline/2-related_important/course_desc.md"
          ],
          "source_outline": "L2: Data scraping and database management with Python - Agenda: Understand the basic website structure, Web scraping using different python libraries, The limitations of web scraping, Create a simple database using python, Understand the concept of adjusted price, Database design for financial market data storage"
        },
        "target_language": "zh-CN"
      },
      "question_bank_path": "research/pipeline/3-guided_story/L2/step1/question_bank.json",
      "question_families": [
        {
          "family_id": "qf_flash_web_scraping_def",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "web_scraping_definition",
            "coverage_tags": [
              "web_scraping_definition"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_web_scraping_def",
            "learning_goal": "学生能准确说出网络爬取的定义。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "网络爬取的核心定义：自动化地从网站提取数据。",
            "term_refs": [
              {
                "display": "网络爬取",
                "en": "Web Scraping"
              }
            ],
            "variants": [
              {
                "back": "从网站自动提取数据的过程。",
                "estimated_seconds": 8,
                "explanation": "网络爬取是一种自动化技术，用于从网页中提取结构化或非结构化数据。",
                "front": "网络爬取（Web Scraping）的核心定义是什么？",
                "question_id": "q_flash_web_scraping_def_v1"
              },
              {
                "back": "自动从网站提取数据，例如股价、新闻等。",
                "estimated_seconds": 8,
                "explanation": "网络爬取替代了手动收集数据的工作，提高了效率和准确性。",
                "front": "在算法交易中，网络爬取主要用于什么目的？",
                "question_id": "q_flash_web_scraping_def_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_web_scraping_use_cases",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "web_scraping_use_cases",
            "coverage_tags": [
              "web_scraping_algo_trading_use_cases"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_web_scraping_use_cases",
            "learning_goal": "学生能列举网络爬取在算法交易中的至少两个应用场景。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "网络爬取在算法交易中的具体应用场景。",
            "term_refs": [
              {
                "display": "算法交易",
                "en": "Algorithmic Trading"
              }
            ],
            "variants": [
              {
                "back": "市场情绪（新闻、论坛、社交媒体）。",
                "estimated_seconds": 10,
                "explanation": "通过爬取新闻和社交媒体数据，可以分析市场情绪，辅助交易决策。",
                "front": "除了实时股价，网络爬取在算法交易中还能用于分析什么？",
                "question_id": "q_flash_web_scraping_use_cases_v1"
              },
              {
                "back": "天气数据、网络流量数据等。",
                "estimated_seconds": 10,
                "explanation": "另类数据是指非传统金融数据，如天气、网络流量，可用于预测市场趋势。",
                "front": "网络爬取可以收集哪一类“另类数据”用于算法交易？",
                "question_id": "q_flash_web_scraping_use_cases_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_data_fuel",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "data_as_fuel",
            "coverage_tags": [
              "data_as_fuel_for_algo_trading"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_data_fuel",
            "learning_goal": "学生能理解数据在算法交易中的核心地位。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "数据在算法交易中的比喻性描述。",
            "term_refs": [
              {
                "display": "算法交易",
                "en": "Algorithmic Trading"
              }
            ],
            "variants": [
              {
                "back": "燃料。没有数据，策略就是空谈。",
                "estimated_seconds": 6,
                "explanation": "这个比喻强调了数据是驱动算法交易策略的基础。",
                "front": "在算法交易中，数据被比喻成什么？",
                "question_id": "q_flash_data_fuel_v1"
              },
              {
                "back": "因为算法交易依赖数据来识别机会、回测策略和执行交易。",
                "estimated_seconds": 10,
                "explanation": "数据是算法交易决策的依据，缺乏数据则无法进行有效的分析和执行。",
                "front": "为什么说“没有数据，策略就是空谈”？",
                "question_id": "q_flash_data_fuel_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_web_scraping_use_case",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "web_scraping_use_case_identification",
            "coverage_tags": [
              "web_scraping_definition",
              "web_scraping_algo_trading_use_cases"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_web_scraping_use_case",
            "learning_goal": "学生能在具体情境中判断是否应该使用网络爬取。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "网络爬取",
                "en": "Web Scraping"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 20,
                "explanation": "网络爬取正是为了自动从网站提取数据而设计的。手动计算、绘图和编写策略逻辑都不是爬取的核心功能。",
                "options": [
                  "手动计算股票平均价格",
                  "自动从财经网站收集每日收盘价",
                  "用 Excel 绘制图表",
                  "编写交易策略的代码逻辑"
                ],
                "question_id": "q_quiz_web_scraping_use_case_v1",
                "stem": "以下哪种情况最适合使用网络爬取？"
              },
              {
                "answer": 1,
                "estimated_seconds": 25,
                "explanation": "网络爬虫可以自动化地、高效地从多个来源提取数据，是处理此类重复性任务的理想工具。",
                "options": [
                  "每天手动阅读所有博客",
                  "编写一个网络爬虫自动抓取博客内容并分析关键词",
                  "订阅付费的新闻摘要邮件",
                  "使用 Excel 手动记录每篇博客的观点"
                ],
                "question_id": "q_quiz_web_scraping_use_case_v2",
                "stem": "一个量化分析师需要每天监控10个财经博客的市场情绪，最有效的方法是？"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_data_fuel",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "data_importance",
            "coverage_tags": [
              "data_as_fuel_for_algo_trading"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_data_fuel",
            "learning_goal": "学生能判断关于数据在算法交易中重要性的陈述是否正确。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "true_false",
            "term_refs": [
              {
                "display": "算法交易",
                "en": "Algorithmic Trading"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "数据是算法交易的燃料。没有高质量的数据，再好的策略也无法被有效验证和执行。两者同等重要。",
                "options": [
                  "正确",
                  "错误"
                ],
                "question_id": "q_quiz_data_fuel_v1",
                "stem": "在算法交易中，拥有一个优秀的交易策略比拥有高质量的数据更重要。"
              },
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "网络爬取是一种重要的数据获取方式，但并非唯一。还可以通过API、数据库、数据供应商等途径获取数据。",
                "options": [
                  "正确",
                  "错误"
                ],
                "question_id": "q_quiz_data_fuel_v2",
                "stem": "网络爬取是算法交易中获取数据的唯一途径。"
              }
            ]
          }
        },
        {
          "family_id": "qf_long_web_scraping_use_cases",
          "kind": "longform_families",
          "summary": {
            "concept_key": "web_scraping_applications",
            "coverage_tags": [
              "web_scraping_algo_trading_use_cases"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_web_scraping_use_cases",
            "learning_goal": "学生能解释网络爬取在算法交易中的至少三个不同应用场景，并说明其价值。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "short_explain",
            "term_refs": [
              {
                "display": "网络爬取",
                "en": "Web Scraping"
              },
              {
                "display": "算法交易",
                "en": "Algorithmic Trading"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "场景1：实时数据收集",
                  "场景2：市场情绪分析",
                  "场景3：公司分析或另类数据"
                ],
                "question_id": "q_long_web_scraping_use_cases_v1",
                "reference_answer": [
                  "1. 实时数据收集：自动从财经网站抓取股票价格、经济指标等实时数据，为交易策略提供输入。",
                  "2. 市场情绪分析：爬取新闻、论坛和社交媒体内容，分析市场对特定股票或行业的整体情绪，辅助判断市场走向。",
                  "3. 公司分析：自动收集公司公告、财报、管理层变动等信息，用于基本面分析。或者，收集天气、网络流量等另类数据，用于预测某些行业（如农业、零售）的表现。"
                ],
                "rubric_points": [
                  "正确识别并解释实时数据收集（如股价、经济指标）",
                  "正确识别并解释市场情绪分析（如新闻、社交媒体）",
                  "正确识别并解释公司分析（如财报、公告）或另类数据（如天气）"
                ],
                "stem": "请列举并简要解释网络爬取在算法交易中的三个不同应用场景。"
              },
              {
                "estimated_seconds": 150,
                "prompt_blocks": [
                  "数据来源",
                  "爬取内容",
                  "数据如何用于策略"
                ],
                "question_id": "q_long_web_scraping_use_cases_v2",
                "reference_answer": [
                  "我会使用网络爬虫定期（例如每5分钟）从Twitter和几个主要财经新闻网站爬取数据。",
                  "爬取的内容将包括包含特定股票代码（如AAPL、TSLA）的推文和新闻标题。",
                  "然后，我会使用一个简单的自然语言处理模型对这些文本进行情感分析，计算正面和负面词汇的比例。如果某只股票的正面情绪比例在短时间内急剧上升并超过一个预设阈值，我的策略就会生成一个买入信号。反之，如果负面情绪占主导，则可能生成卖出信号。"
                ],
                "rubric_points": [
                  "明确指出数据来源（如Twitter、财经新闻网站、Reddit论坛）",
                  "描述爬取的具体内容（如包含特定股票代码的推文、新闻标题、评论）",
                  "解释如何将爬取的数据转化为交易信号（如通过情感分析计算正面/负面比例，当正面情绪超过阈值时买入）"
                ],
                "stem": "假设你正在开发一个基于市场情绪的短线交易策略，请描述你将如何使用网络爬取来获取所需数据，并解释这些数据如何驱动你的策略。"
              }
            ]
          }
        }
      ],
      "sequence_id": "step1",
      "step": {
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
      "step_path": "research/pipeline/3-guided_story/L2/step1/step.json"
    },
    {
      "question_bank": {
        "coverage_map": [
          {
            "coverage_tag": "basic_web_structure_three_components",
            "covered_by": [
              "qf_flash_web_components",
              "qf_quiz_web_components_role"
            ],
            "description": "理解网页由 HTML、CSS 和 JavaScript 三部分组成"
          },
          {
            "coverage_tag": "html_purpose_and_nature",
            "covered_by": [
              "qf_flash_html_purpose",
              "qf_quiz_html_nature"
            ],
            "description": "HTML 定义网页的结构和内容，是一种标记语言而非编程语言"
          },
          {
            "coverage_tag": "css_purpose",
            "covered_by": [
              "qf_flash_css_purpose",
              "qf_quiz_css_function"
            ],
            "description": "CSS 控制网页的视觉呈现和布局"
          },
          {
            "coverage_tag": "javascript_purpose",
            "covered_by": [
              "qf_flash_javascript_purpose",
              "qf_quiz_javascript_function"
            ],
            "description": "JavaScript 为网页添加交互和动态行为"
          },
          {
            "coverage_tag": "scraping_focus_html",
            "covered_by": [
              "qf_flash_scraping_focus_html",
              "qf_quiz_why_html_for_scraping"
            ],
            "description": "爬取数据时最关心 HTML，因为数据通常藏在 HTML 标签中"
          }
        ],
        "flashcard_families": [
          {
            "concept_key": "basic_web_structure_three_components",
            "coverage_tags": [
              "basic_web_structure_three_components"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_web_components",
            "learning_goal": "学生能准确说出构成网页的三个核心技术及其核心职责。",
            "linked_steps": [
              "step2"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "网页三要素的名称和各自的核心功能。",
            "term_refs": [
              {
                "display": "HTML",
                "en": "HyperText Markup Language"
              },
              {
                "display": "CSS",
                "en": "Cascading Style Sheets"
              },
              {
                "display": "JavaScript",
                "en": "JavaScript"
              }
            ],
            "variants": [
              {
                "back": "HTML、CSS 和 JavaScript。",
                "estimated_seconds": 8,
                "explanation": "HTML 负责结构，CSS 负责样式，JavaScript 负责交互。",
                "front": "一个网页通常由哪三种技术共同构成？",
                "question_id": "q_flash_web_components_v1"
              },
              {
                "back": "HTML（超文本标记语言）。",
                "estimated_seconds": 6,
                "explanation": "HTML 使用标签来创建标题、段落、表格等元素。",
                "front": "在网页三要素中，哪一种技术负责定义网页的结构和内容？",
                "question_id": "q_flash_web_components_v2"
              },
              {
                "back": "CSS（层叠样式表）。",
                "estimated_seconds": 6,
                "explanation": "CSS 定义颜色、字体、间距和定位等样式。",
                "front": "在网页三要素中，哪一种技术负责控制网页的视觉呈现和布局？",
                "question_id": "q_flash_web_components_v3"
              }
            ]
          },
          {
            "concept_key": "html_purpose_and_nature",
            "coverage_tags": [
              "html_purpose_and_nature"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_html_purpose",
            "learning_goal": "学生能准确描述 HTML 的用途和本质。",
            "linked_steps": [
              "step2"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "HTML 的核心定义和分类。",
            "term_refs": [
              {
                "display": "HTML",
                "en": "HyperText Markup Language"
              }
            ],
            "variants": [
              {
                "back": "HyperText Markup Language（超文本标记语言）。",
                "estimated_seconds": 5,
                "explanation": "HTML 是一种标记语言，用于结构化网页内容。",
                "front": "HTML 的全称是什么？",
                "question_id": "q_flash_html_purpose_v1"
              },
              {
                "back": "不是。它是一种标记语言（Markup Language）。",
                "estimated_seconds": 6,
                "explanation": "HTML 使用标签来定义内容结构，而不是编写逻辑。",
                "front": "HTML 是一种编程语言吗？如果不是，它是什么？",
                "question_id": "q_flash_html_purpose_v2"
              }
            ]
          },
          {
            "concept_key": "css_purpose",
            "coverage_tags": [
              "css_purpose"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_css_purpose",
            "learning_goal": "学生能准确说出 CSS 的核心用途。",
            "linked_steps": [
              "step2"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "CSS 的功能定义。",
            "term_refs": [
              {
                "display": "CSS",
                "en": "Cascading Style Sheets"
              }
            ],
            "variants": [
              {
                "back": "控制网页的视觉呈现和布局（如颜色、字体、间距）。",
                "estimated_seconds": 6,
                "explanation": "CSS 负责美化网页，让内容以特定样式展示。",
                "front": "CSS 在网页中的主要作用是什么？",
                "question_id": "q_flash_css_purpose_v1"
              },
              {
                "back": "Cascading Style Sheets（层叠样式表）。",
                "estimated_seconds": 5,
                "explanation": "层叠规则决定了多个样式冲突时的优先级。",
                "front": "CSS 的全称是什么？",
                "question_id": "q_flash_css_purpose_v2"
              }
            ]
          },
          {
            "concept_key": "javascript_purpose",
            "coverage_tags": [
              "javascript_purpose"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_javascript_purpose",
            "learning_goal": "学生能准确说出 JavaScript 的核心用途。",
            "linked_steps": [
              "step2"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "JavaScript 的功能定义。",
            "term_refs": [
              {
                "display": "JavaScript",
                "en": "JavaScript"
              }
            ],
            "variants": [
              {
                "back": "为网页添加交互和动态行为（如点击按钮弹出信息）。",
                "estimated_seconds": 6,
                "explanation": "JavaScript 是一种脚本语言，可以操作 HTML 和 CSS。",
                "front": "JavaScript 在网页中的主要作用是什么？",
                "question_id": "q_flash_javascript_purpose_v1"
              },
              {
                "back": "JavaScript。",
                "estimated_seconds": 5,
                "explanation": "JavaScript 可以响应用户操作，如点击、输入等。",
                "front": "网页三要素中，哪一种技术让网页变得“动态”和“可交互”？",
                "question_id": "q_flash_javascript_purpose_v2"
              }
            ]
          },
          {
            "concept_key": "scraping_focus_html",
            "coverage_tags": [
              "scraping_focus_html"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_scraping_focus_html",
            "learning_goal": "学生能理解为什么爬取数据时最关注 HTML。",
            "linked_steps": [
              "step2"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "爬虫关注 HTML 的原因。",
            "term_refs": [
              {
                "display": "HTML",
                "en": "HyperText Markup Language"
              }
            ],
            "variants": [
              {
                "back": "HTML。因为数据通常就藏在 HTML 的标签里。",
                "estimated_seconds": 8,
                "explanation": "CSS 和 JavaScript 主要控制样式和交互，而实际内容在 HTML 中。",
                "front": "爬取网页数据时，我们最关心网页三要素中的哪一个？为什么？",
                "question_id": "q_flash_scraping_focus_html_v1"
              },
              {
                "back": "因为网页的结构化数据（如文本、表格、链接）都定义在 HTML 标签中。",
                "estimated_seconds": 8,
                "explanation": "CSS 和 JavaScript 不直接包含目标数据。",
                "front": "为什么网络爬虫主要解析 HTML 而不是 CSS 或 JavaScript？",
                "question_id": "q_flash_scraping_focus_html_v2"
              }
            ]
          }
        ],
        "lesson_id": "L2",
        "longform_families": [
          {
            "concept_key": "basic_web_structure_three_components",
            "coverage_tags": [
              "basic_web_structure_three_components",
              "html_purpose_and_nature",
              "css_purpose",
              "javascript_purpose",
              "scraping_focus_html"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_web_architecture_explain",
            "learning_goal": "学生能用自己的语言解释网页三要素的各自角色以及它们与网络爬取的关系。",
            "linked_steps": [
              "step2"
            ],
            "question_type": "short_explain",
            "term_refs": [
              {
                "display": "HTML",
                "en": "HyperText Markup Language"
              },
              {
                "display": "CSS",
                "en": "Cascading Style Sheets"
              },
              {
                "display": "JavaScript",
                "en": "JavaScript"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "列出三种技术",
                  "分别描述每种技术的作用",
                  "解释爬虫关注特定技术的原因"
                ],
                "question_id": "q_long_web_architecture_explain_v1",
                "reference_answer": [
                  "一个网页通常由 HTML、CSS 和 JavaScript 三种技术构成。",
                  "HTML（超文本标记语言）负责定义网页的结构和内容，比如标题、段落、表格等。",
                  "CSS（层叠样式表）负责控制网页的视觉呈现和布局，比如颜色、字体、间距。",
                  "JavaScript 负责为网页添加交互和动态行为，比如点击按钮后弹出信息。",
                  "进行网络爬取时，我们最关心 HTML，因为网页中的目标数据（如股票价格、新闻文本）通常直接包含在 HTML 的标签里。CSS 和 JavaScript 主要控制样式和交互，不直接承载数据。"
                ],
                "rubric_points": [
                  "正确列出 HTML、CSS、JavaScript（1分）",
                  "准确描述 HTML 定义结构和内容（1分）",
                  "准确描述 CSS 控制样式和布局（1分）",
                  "准确描述 JavaScript 提供交互和动态行为（1分）",
                  "解释爬虫关注 HTML 是因为数据通常包含在 HTML 标签中（1分）"
                ],
                "stem": "请简要解释一个网页通常由哪三种核心技术构成，并分别说明它们的主要作用。最后，解释为什么在进行网络爬取时，我们最关心其中的哪一种技术。"
              },
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "用比喻或通俗语言解释三种技术的角色",
                  "说明爬虫为何聚焦于 HTML"
                ],
                "question_id": "q_long_web_architecture_explain_v2",
                "reference_answer": [
                  "可以把网页想象成一栋房子：HTML 是房子的结构和骨架（墙壁、房间），CSS 是房子的装修和风格（墙纸颜色、家具摆放），JavaScript 是房子里的智能设备（感应灯、自动门）。",
                  "爬虫的核心任务是提取数据，而数据就像是房子里的物品，这些物品都放在由 HTML 定义的“房间”（标签）里。",
                  "CSS 和 JavaScript 虽然重要，但它们不直接“存放”数据，而是决定数据如何展示或如何交互。",
                  "因此，解析 HTML 是爬虫的核心任务，因为我们需要从 HTML 标签中提取出我们想要的信息。"
                ],
                "rubric_points": [
                  "正确识别三种技术（1分）",
                  "对 HTML 的解释准确（1分）",
                  "对 CSS 的解释准确（1分）",
                  "对 JavaScript 的解释准确（1分）",
                  "清晰阐述爬虫关注 HTML 的原因（1分）"
                ],
                "stem": "假设你是一个刚入门的爬虫工程师，需要向团队新人解释网页的基本结构。请用你自己的话，说明 HTML、CSS 和 JavaScript 分别扮演什么角色，并解释为什么在编写爬虫时，解析 HTML 是核心任务。"
              }
            ]
          }
        ],
        "quiz_families": [
          {
            "concept_key": "basic_web_structure_three_components",
            "coverage_tags": [
              "basic_web_structure_three_components"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_web_components_role",
            "learning_goal": "学生能在测验中辨析三种网页技术的核心职责。",
            "linked_steps": [
              "step2"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "HTML",
                "en": "HyperText Markup Language"
              },
              {
                "display": "CSS",
                "en": "Cascading Style Sheets"
              },
              {
                "display": "JavaScript",
                "en": "JavaScript"
              }
            ],
            "variants": [
              {
                "answer": 2,
                "estimated_seconds": 15,
                "explanation": "HTML 负责定义网页的结构和内容，如标题、段落、表格。",
                "options": [
                  "CSS",
                  "JavaScript",
                  "HTML",
                  "Python"
                ],
                "question_id": "q_quiz_web_components_role_v1",
                "stem": "网页的“骨架”和“内容”是由哪种技术定义的？"
              },
              {
                "answer": 2,
                "estimated_seconds": 15,
                "explanation": "CSS（层叠样式表）专门用于控制网页的视觉呈现和布局。",
                "options": [
                  "HTML",
                  "JavaScript",
                  "CSS",
                  "SQL"
                ],
                "question_id": "q_quiz_web_components_role_v2",
                "stem": "以下哪种技术主要负责控制网页中元素的颜色、字体和布局？"
              },
              {
                "answer": 2,
                "estimated_seconds": 15,
                "explanation": "JavaScript 负责处理用户交互事件，如点击、输入等。",
                "options": [
                  "HTML",
                  "CSS",
                  "JavaScript",
                  "HTTP"
                ],
                "question_id": "q_quiz_web_components_role_v3",
                "stem": "当你在网页上点击一个按钮后，弹出一个信息框，这最可能是哪种技术的作用？"
              }
            ]
          },
          {
            "concept_key": "html_purpose_and_nature",
            "coverage_tags": [
              "html_purpose_and_nature"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_html_nature",
            "learning_goal": "学生能正确判断 HTML 的本质属性。",
            "linked_steps": [
              "step2"
            ],
            "question_type": "true_false",
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
                "explanation": "HTML 是一种标记语言，用于结构化内容，不是编程语言。",
                "options": [
                  "正确",
                  "错误"
                ],
                "question_id": "q_quiz_html_nature_v1",
                "stem": "HTML 是一种编程语言，因为它可以编写逻辑和算法。"
              },
              {
                "answer": 0,
                "estimated_seconds": 10,
                "explanation": "HTML 使用标签来创建标题、段落、链接等元素，定义网页骨架。",
                "options": [
                  "正确",
                  "错误"
                ],
                "question_id": "q_quiz_html_nature_v2",
                "stem": "HTML 的主要用途是定义网页的结构和内容。"
              }
            ]
          },
          {
            "concept_key": "css_purpose",
            "coverage_tags": [
              "css_purpose"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_css_function",
            "learning_goal": "学生能识别 CSS 的具体功能。",
            "linked_steps": [
              "step2"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "CSS",
                "en": "Cascading Style Sheets"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "CSS 负责样式控制，如字体、颜色、间距等。",
                "options": [
                  "定义网页中的超链接",
                  "设置段落的字体大小和颜色",
                  "处理用户点击事件",
                  "创建数据库表"
                ],
                "question_id": "q_quiz_css_function_v1",
                "stem": "以下哪个是 CSS 的典型功能？"
              },
              {
                "answer": 1,
                "estimated_seconds": 20,
                "explanation": "层叠规则决定了当多个样式冲突时哪个样式生效。",
                "options": [
                  "样式可以像瀑布一样流动",
                  "多个样式规则可以应用于同一元素，有优先级规则",
                  "CSS 只能用于网页的顶部区域",
                  "CSS 文件必须按顺序加载"
                ],
                "question_id": "q_quiz_css_function_v2",
                "stem": "CSS 中的“Cascading”指的是什么？"
              }
            ]
          },
          {
            "concept_key": "javascript_purpose",
            "coverage_tags": [
              "javascript_purpose"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_javascript_function",
            "learning_goal": "学生能识别 JavaScript 的具体功能。",
            "linked_steps": [
              "step2"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "JavaScript",
                "en": "JavaScript"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "JavaScript 可以处理表单提交事件并动态更新页面内容。",
                "options": [
                  "网页标题显示为蓝色",
                  "用户提交表单后，页面显示“提交成功”的提示",
                  "网页中包含一个表格",
                  "网页背景颜色为白色"
                ],
                "question_id": "q_quiz_javascript_function_v1",
                "stem": "以下哪个场景最可能使用了 JavaScript？"
              },
              {
                "answer": 2,
                "estimated_seconds": 15,
                "explanation": "JavaScript 最初是客户端脚本，但现在也可通过 Node.js 在服务器端运行。",
                "options": [
                  "仅在服务器端",
                  "仅在浏览器端（客户端）",
                  "既可以在浏览器端也可以在服务器端（如 Node.js）",
                  "仅在数据库中"
                ],
                "question_id": "q_quiz_javascript_function_v2",
                "stem": "JavaScript 可以在哪里运行？"
              }
            ]
          },
          {
            "concept_key": "scraping_focus_html",
            "coverage_tags": [
              "scraping_focus_html"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_why_html_for_scraping",
            "learning_goal": "学生能理解爬虫优先解析 HTML 的原因。",
            "linked_steps": [
              "step2"
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
                "estimated_seconds": 20,
                "explanation": "网页的结构化数据通常以文本形式嵌入 HTML 标签内，CSS 和 JavaScript 不直接承载这些数据。",
                "options": [
                  "因为 HTML 文件体积最小，下载最快",
                  "因为目标数据（如股票价格、新闻标题）通常直接包含在 HTML 标签中",
                  "因为 CSS 和 JavaScript 无法被 Python 解析",
                  "因为 HTML 是唯一可以包含文本的技术"
                ],
                "question_id": "q_quiz_why_html_for_scraping_v1",
                "stem": "进行网络爬取时，为什么我们主要关注 HTML 而不是 CSS 或 JavaScript？"
              },
              {
                "answer": 1,
                "estimated_seconds": 25,
                "explanation": "动态加载的数据最终会更新到 HTML DOM 中，但可能需要使用 Selenium 等工具先执行 JavaScript。",
                "options": [
                  "HTML 对于爬虫来说完全不重要",
                  "爬虫需要额外处理 JavaScript 渲染的内容，但数据最终仍会出现在 HTML DOM 中",
                  "应该放弃爬取该网页",
                  "CSS 比 HTML 更适合爬取数据"
                ],
                "question_id": "q_quiz_why_html_for_scraping_v2",
                "stem": "如果一个网页的数据是通过 JavaScript 动态加载的，直接解析 HTML 可能无法获取数据。这说明了什么？"
              }
            ]
          }
        ],
        "source": {
          "coverage_checklist": "L2: Data scraping and database management with Python - Understand the basic website structure",
          "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
          "lesson_map": "L2 - step2: Understanding the basic structure of a website: HTML, CSS, and JavaScript",
          "plain_text": "pipeline/1-plain/L2/plain.txt",
          "related": [
            "pipeline/2-related_important/course_desc.md"
          ],
          "source_outline": "L2: Data scraping and database management with Python - Basic Web structure - A website is usually made of 1. HTML 2. CSS 3. JavaScript"
        },
        "target_language": "zh-CN"
      },
      "question_bank_path": "research/pipeline/3-guided_story/L2/step2/question_bank.json",
      "question_families": [
        {
          "family_id": "qf_flash_web_components",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "basic_web_structure_three_components",
            "coverage_tags": [
              "basic_web_structure_three_components"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_web_components",
            "learning_goal": "学生能准确说出构成网页的三个核心技术及其核心职责。",
            "linked_steps": [
              "step2"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "网页三要素的名称和各自的核心功能。",
            "term_refs": [
              {
                "display": "HTML",
                "en": "HyperText Markup Language"
              },
              {
                "display": "CSS",
                "en": "Cascading Style Sheets"
              },
              {
                "display": "JavaScript",
                "en": "JavaScript"
              }
            ],
            "variants": [
              {
                "back": "HTML、CSS 和 JavaScript。",
                "estimated_seconds": 8,
                "explanation": "HTML 负责结构，CSS 负责样式，JavaScript 负责交互。",
                "front": "一个网页通常由哪三种技术共同构成？",
                "question_id": "q_flash_web_components_v1"
              },
              {
                "back": "HTML（超文本标记语言）。",
                "estimated_seconds": 6,
                "explanation": "HTML 使用标签来创建标题、段落、表格等元素。",
                "front": "在网页三要素中，哪一种技术负责定义网页的结构和内容？",
                "question_id": "q_flash_web_components_v2"
              },
              {
                "back": "CSS（层叠样式表）。",
                "estimated_seconds": 6,
                "explanation": "CSS 定义颜色、字体、间距和定位等样式。",
                "front": "在网页三要素中，哪一种技术负责控制网页的视觉呈现和布局？",
                "question_id": "q_flash_web_components_v3"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_html_purpose",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "html_purpose_and_nature",
            "coverage_tags": [
              "html_purpose_and_nature"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_html_purpose",
            "learning_goal": "学生能准确描述 HTML 的用途和本质。",
            "linked_steps": [
              "step2"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "HTML 的核心定义和分类。",
            "term_refs": [
              {
                "display": "HTML",
                "en": "HyperText Markup Language"
              }
            ],
            "variants": [
              {
                "back": "HyperText Markup Language（超文本标记语言）。",
                "estimated_seconds": 5,
                "explanation": "HTML 是一种标记语言，用于结构化网页内容。",
                "front": "HTML 的全称是什么？",
                "question_id": "q_flash_html_purpose_v1"
              },
              {
                "back": "不是。它是一种标记语言（Markup Language）。",
                "estimated_seconds": 6,
                "explanation": "HTML 使用标签来定义内容结构，而不是编写逻辑。",
                "front": "HTML 是一种编程语言吗？如果不是，它是什么？",
                "question_id": "q_flash_html_purpose_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_css_purpose",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "css_purpose",
            "coverage_tags": [
              "css_purpose"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_css_purpose",
            "learning_goal": "学生能准确说出 CSS 的核心用途。",
            "linked_steps": [
              "step2"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "CSS 的功能定义。",
            "term_refs": [
              {
                "display": "CSS",
                "en": "Cascading Style Sheets"
              }
            ],
            "variants": [
              {
                "back": "控制网页的视觉呈现和布局（如颜色、字体、间距）。",
                "estimated_seconds": 6,
                "explanation": "CSS 负责美化网页，让内容以特定样式展示。",
                "front": "CSS 在网页中的主要作用是什么？",
                "question_id": "q_flash_css_purpose_v1"
              },
              {
                "back": "Cascading Style Sheets（层叠样式表）。",
                "estimated_seconds": 5,
                "explanation": "层叠规则决定了多个样式冲突时的优先级。",
                "front": "CSS 的全称是什么？",
                "question_id": "q_flash_css_purpose_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_javascript_purpose",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "javascript_purpose",
            "coverage_tags": [
              "javascript_purpose"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_javascript_purpose",
            "learning_goal": "学生能准确说出 JavaScript 的核心用途。",
            "linked_steps": [
              "step2"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "JavaScript 的功能定义。",
            "term_refs": [
              {
                "display": "JavaScript",
                "en": "JavaScript"
              }
            ],
            "variants": [
              {
                "back": "为网页添加交互和动态行为（如点击按钮弹出信息）。",
                "estimated_seconds": 6,
                "explanation": "JavaScript 是一种脚本语言，可以操作 HTML 和 CSS。",
                "front": "JavaScript 在网页中的主要作用是什么？",
                "question_id": "q_flash_javascript_purpose_v1"
              },
              {
                "back": "JavaScript。",
                "estimated_seconds": 5,
                "explanation": "JavaScript 可以响应用户操作，如点击、输入等。",
                "front": "网页三要素中，哪一种技术让网页变得“动态”和“可交互”？",
                "question_id": "q_flash_javascript_purpose_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_scraping_focus_html",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "scraping_focus_html",
            "coverage_tags": [
              "scraping_focus_html"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_scraping_focus_html",
            "learning_goal": "学生能理解为什么爬取数据时最关注 HTML。",
            "linked_steps": [
              "step2"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "爬虫关注 HTML 的原因。",
            "term_refs": [
              {
                "display": "HTML",
                "en": "HyperText Markup Language"
              }
            ],
            "variants": [
              {
                "back": "HTML。因为数据通常就藏在 HTML 的标签里。",
                "estimated_seconds": 8,
                "explanation": "CSS 和 JavaScript 主要控制样式和交互，而实际内容在 HTML 中。",
                "front": "爬取网页数据时，我们最关心网页三要素中的哪一个？为什么？",
                "question_id": "q_flash_scraping_focus_html_v1"
              },
              {
                "back": "因为网页的结构化数据（如文本、表格、链接）都定义在 HTML 标签中。",
                "estimated_seconds": 8,
                "explanation": "CSS 和 JavaScript 不直接包含目标数据。",
                "front": "为什么网络爬虫主要解析 HTML 而不是 CSS 或 JavaScript？",
                "question_id": "q_flash_scraping_focus_html_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_web_components_role",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "basic_web_structure_three_components",
            "coverage_tags": [
              "basic_web_structure_three_components"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_web_components_role",
            "learning_goal": "学生能在测验中辨析三种网页技术的核心职责。",
            "linked_steps": [
              "step2"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "HTML",
                "en": "HyperText Markup Language"
              },
              {
                "display": "CSS",
                "en": "Cascading Style Sheets"
              },
              {
                "display": "JavaScript",
                "en": "JavaScript"
              }
            ],
            "variants": [
              {
                "answer": 2,
                "estimated_seconds": 15,
                "explanation": "HTML 负责定义网页的结构和内容，如标题、段落、表格。",
                "options": [
                  "CSS",
                  "JavaScript",
                  "HTML",
                  "Python"
                ],
                "question_id": "q_quiz_web_components_role_v1",
                "stem": "网页的“骨架”和“内容”是由哪种技术定义的？"
              },
              {
                "answer": 2,
                "estimated_seconds": 15,
                "explanation": "CSS（层叠样式表）专门用于控制网页的视觉呈现和布局。",
                "options": [
                  "HTML",
                  "JavaScript",
                  "CSS",
                  "SQL"
                ],
                "question_id": "q_quiz_web_components_role_v2",
                "stem": "以下哪种技术主要负责控制网页中元素的颜色、字体和布局？"
              },
              {
                "answer": 2,
                "estimated_seconds": 15,
                "explanation": "JavaScript 负责处理用户交互事件，如点击、输入等。",
                "options": [
                  "HTML",
                  "CSS",
                  "JavaScript",
                  "HTTP"
                ],
                "question_id": "q_quiz_web_components_role_v3",
                "stem": "当你在网页上点击一个按钮后，弹出一个信息框，这最可能是哪种技术的作用？"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_html_nature",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "html_purpose_and_nature",
            "coverage_tags": [
              "html_purpose_and_nature"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_html_nature",
            "learning_goal": "学生能正确判断 HTML 的本质属性。",
            "linked_steps": [
              "step2"
            ],
            "question_type": "true_false",
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
                "explanation": "HTML 是一种标记语言，用于结构化内容，不是编程语言。",
                "options": [
                  "正确",
                  "错误"
                ],
                "question_id": "q_quiz_html_nature_v1",
                "stem": "HTML 是一种编程语言，因为它可以编写逻辑和算法。"
              },
              {
                "answer": 0,
                "estimated_seconds": 10,
                "explanation": "HTML 使用标签来创建标题、段落、链接等元素，定义网页骨架。",
                "options": [
                  "正确",
                  "错误"
                ],
                "question_id": "q_quiz_html_nature_v2",
                "stem": "HTML 的主要用途是定义网页的结构和内容。"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_css_function",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "css_purpose",
            "coverage_tags": [
              "css_purpose"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_css_function",
            "learning_goal": "学生能识别 CSS 的具体功能。",
            "linked_steps": [
              "step2"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "CSS",
                "en": "Cascading Style Sheets"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "CSS 负责样式控制，如字体、颜色、间距等。",
                "options": [
                  "定义网页中的超链接",
                  "设置段落的字体大小和颜色",
                  "处理用户点击事件",
                  "创建数据库表"
                ],
                "question_id": "q_quiz_css_function_v1",
                "stem": "以下哪个是 CSS 的典型功能？"
              },
              {
                "answer": 1,
                "estimated_seconds": 20,
                "explanation": "层叠规则决定了当多个样式冲突时哪个样式生效。",
                "options": [
                  "样式可以像瀑布一样流动",
                  "多个样式规则可以应用于同一元素，有优先级规则",
                  "CSS 只能用于网页的顶部区域",
                  "CSS 文件必须按顺序加载"
                ],
                "question_id": "q_quiz_css_function_v2",
                "stem": "CSS 中的“Cascading”指的是什么？"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_javascript_function",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "javascript_purpose",
            "coverage_tags": [
              "javascript_purpose"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_javascript_function",
            "learning_goal": "学生能识别 JavaScript 的具体功能。",
            "linked_steps": [
              "step2"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "JavaScript",
                "en": "JavaScript"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "JavaScript 可以处理表单提交事件并动态更新页面内容。",
                "options": [
                  "网页标题显示为蓝色",
                  "用户提交表单后，页面显示“提交成功”的提示",
                  "网页中包含一个表格",
                  "网页背景颜色为白色"
                ],
                "question_id": "q_quiz_javascript_function_v1",
                "stem": "以下哪个场景最可能使用了 JavaScript？"
              },
              {
                "answer": 2,
                "estimated_seconds": 15,
                "explanation": "JavaScript 最初是客户端脚本，但现在也可通过 Node.js 在服务器端运行。",
                "options": [
                  "仅在服务器端",
                  "仅在浏览器端（客户端）",
                  "既可以在浏览器端也可以在服务器端（如 Node.js）",
                  "仅在数据库中"
                ],
                "question_id": "q_quiz_javascript_function_v2",
                "stem": "JavaScript 可以在哪里运行？"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_why_html_for_scraping",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "scraping_focus_html",
            "coverage_tags": [
              "scraping_focus_html"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_why_html_for_scraping",
            "learning_goal": "学生能理解爬虫优先解析 HTML 的原因。",
            "linked_steps": [
              "step2"
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
                "estimated_seconds": 20,
                "explanation": "网页的结构化数据通常以文本形式嵌入 HTML 标签内，CSS 和 JavaScript 不直接承载这些数据。",
                "options": [
                  "因为 HTML 文件体积最小，下载最快",
                  "因为目标数据（如股票价格、新闻标题）通常直接包含在 HTML 标签中",
                  "因为 CSS 和 JavaScript 无法被 Python 解析",
                  "因为 HTML 是唯一可以包含文本的技术"
                ],
                "question_id": "q_quiz_why_html_for_scraping_v1",
                "stem": "进行网络爬取时，为什么我们主要关注 HTML 而不是 CSS 或 JavaScript？"
              },
              {
                "answer": 1,
                "estimated_seconds": 25,
                "explanation": "动态加载的数据最终会更新到 HTML DOM 中，但可能需要使用 Selenium 等工具先执行 JavaScript。",
                "options": [
                  "HTML 对于爬虫来说完全不重要",
                  "爬虫需要额外处理 JavaScript 渲染的内容，但数据最终仍会出现在 HTML DOM 中",
                  "应该放弃爬取该网页",
                  "CSS 比 HTML 更适合爬取数据"
                ],
                "question_id": "q_quiz_why_html_for_scraping_v2",
                "stem": "如果一个网页的数据是通过 JavaScript 动态加载的，直接解析 HTML 可能无法获取数据。这说明了什么？"
              }
            ]
          }
        },
        {
          "family_id": "qf_long_web_architecture_explain",
          "kind": "longform_families",
          "summary": {
            "concept_key": "basic_web_structure_three_components",
            "coverage_tags": [
              "basic_web_structure_three_components",
              "html_purpose_and_nature",
              "css_purpose",
              "javascript_purpose",
              "scraping_focus_html"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_web_architecture_explain",
            "learning_goal": "学生能用自己的语言解释网页三要素的各自角色以及它们与网络爬取的关系。",
            "linked_steps": [
              "step2"
            ],
            "question_type": "short_explain",
            "term_refs": [
              {
                "display": "HTML",
                "en": "HyperText Markup Language"
              },
              {
                "display": "CSS",
                "en": "Cascading Style Sheets"
              },
              {
                "display": "JavaScript",
                "en": "JavaScript"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "列出三种技术",
                  "分别描述每种技术的作用",
                  "解释爬虫关注特定技术的原因"
                ],
                "question_id": "q_long_web_architecture_explain_v1",
                "reference_answer": [
                  "一个网页通常由 HTML、CSS 和 JavaScript 三种技术构成。",
                  "HTML（超文本标记语言）负责定义网页的结构和内容，比如标题、段落、表格等。",
                  "CSS（层叠样式表）负责控制网页的视觉呈现和布局，比如颜色、字体、间距。",
                  "JavaScript 负责为网页添加交互和动态行为，比如点击按钮后弹出信息。",
                  "进行网络爬取时，我们最关心 HTML，因为网页中的目标数据（如股票价格、新闻文本）通常直接包含在 HTML 的标签里。CSS 和 JavaScript 主要控制样式和交互，不直接承载数据。"
                ],
                "rubric_points": [
                  "正确列出 HTML、CSS、JavaScript（1分）",
                  "准确描述 HTML 定义结构和内容（1分）",
                  "准确描述 CSS 控制样式和布局（1分）",
                  "准确描述 JavaScript 提供交互和动态行为（1分）",
                  "解释爬虫关注 HTML 是因为数据通常包含在 HTML 标签中（1分）"
                ],
                "stem": "请简要解释一个网页通常由哪三种核心技术构成，并分别说明它们的主要作用。最后，解释为什么在进行网络爬取时，我们最关心其中的哪一种技术。"
              },
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "用比喻或通俗语言解释三种技术的角色",
                  "说明爬虫为何聚焦于 HTML"
                ],
                "question_id": "q_long_web_architecture_explain_v2",
                "reference_answer": [
                  "可以把网页想象成一栋房子：HTML 是房子的结构和骨架（墙壁、房间），CSS 是房子的装修和风格（墙纸颜色、家具摆放），JavaScript 是房子里的智能设备（感应灯、自动门）。",
                  "爬虫的核心任务是提取数据，而数据就像是房子里的物品，这些物品都放在由 HTML 定义的“房间”（标签）里。",
                  "CSS 和 JavaScript 虽然重要，但它们不直接“存放”数据，而是决定数据如何展示或如何交互。",
                  "因此，解析 HTML 是爬虫的核心任务，因为我们需要从 HTML 标签中提取出我们想要的信息。"
                ],
                "rubric_points": [
                  "正确识别三种技术（1分）",
                  "对 HTML 的解释准确（1分）",
                  "对 CSS 的解释准确（1分）",
                  "对 JavaScript 的解释准确（1分）",
                  "清晰阐述爬虫关注 HTML 的原因（1分）"
                ],
                "stem": "假设你是一个刚入门的爬虫工程师，需要向团队新人解释网页的基本结构。请用你自己的话，说明 HTML、CSS 和 JavaScript 分别扮演什么角色，并解释为什么在编写爬虫时，解析 HTML 是核心任务。"
              }
            ]
          }
        }
      ],
      "sequence_id": "step2",
      "step": {
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
      "step_path": "research/pipeline/3-guided_story/L2/step2/step.json"
    },
    {
      "question_bank": {
        "coverage_map": [
          {
            "coverage_tag": "beautifulsoup_intro_and_setup",
            "covered_by": [
              "qf_flash_bs4_purpose",
              "qf_flash_bs4_workflow",
              "qf_quiz_bs4_workflow"
            ],
            "description": "BeautifulSoup 库的用途、安装方式以及与 Requests 配合使用的基本流程"
          },
          {
            "coverage_tag": "requests_download_raw_content",
            "covered_by": [
              "qf_flash_bs4_workflow",
              "qf_quiz_bs4_workflow"
            ],
            "description": "使用 Requests 库下载网页原始 HTML 内容"
          },
          {
            "coverage_tag": "beautifulsoup_parse_and_pretty_print",
            "covered_by": [
              "qf_flash_bs4_pretty",
              "qf_quiz_bs4_methods"
            ],
            "description": "BeautifulSoup 解析 HTML 为树状结构，prettify() 格式化输出"
          },
          {
            "coverage_tag": "beautifulsoup_extract_text",
            "covered_by": [
              "qf_flash_bs4_get_text",
              "qf_quiz_bs4_methods"
            ],
            "description": "使用 get_text() 提取网页中所有纯文本"
          },
          {
            "coverage_tag": "beautifulsoup_find_and_find_all",
            "covered_by": [
              "qf_flash_bs4_find",
              "qf_flash_bs4_find_all",
              "qf_quiz_bs4_find_vs_findall",
              "qf_long_bs4_extract_table"
            ],
            "description": "使用 find() 和 find_all() 定位特定 HTML 标签"
          },
          {
            "coverage_tag": "beautifulsoup_table_to_dataframe",
            "covered_by": [
              "qf_flash_bs4_table_pandas",
              "qf_long_bs4_extract_table"
            ],
            "description": "使用 BeautifulSoup 找到 <table> 标签后配合 pandas 转换为 DataFrame"
          }
        ],
        "flashcard_families": [
          {
            "concept_key": "beautifulsoup_intro_and_setup",
            "coverage_tags": [
              "beautifulsoup_intro_and_setup"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_bs4_purpose",
            "learning_goal": "学生能准确说出 BeautifulSoup 库的核心用途。",
            "linked_steps": [
              "step3"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "BeautifulSoup 库的核心功能是什么？",
            "term_refs": [
              {
                "display": "BeautifulSoup",
                "en": "BeautifulSoup"
              }
            ],
            "variants": [
              {
                "back": "解析 HTML 和 XML 数据，将其整理成可操作的树状结构。",
                "estimated_seconds": 8,
                "explanation": "BeautifulSoup 就像一个 HTML 的“瑞士军刀”，专门用于搜索和解析网页标记语言。",
                "front": "BeautifulSoup 库在 Python 爬虫中的主要作用是什么？",
                "question_id": "q_flash_bs4_purpose_v1"
              },
              {
                "back": "Requests 库。",
                "estimated_seconds": 6,
                "explanation": "先用 Requests 下载网页原始内容，再交给 BeautifulSoup 解析。",
                "front": "在 Python 爬虫中，BeautifulSoup 通常与哪个库配合使用来下载网页内容？",
                "question_id": "q_flash_bs4_purpose_v2"
              }
            ]
          },
          {
            "concept_key": "beautifulsoup_parse_and_pretty_print",
            "coverage_tags": [
              "beautifulsoup_parse_and_pretty_print"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_bs4_pretty",
            "learning_goal": "学生能记住 prettify() 方法的功能。",
            "linked_steps": [
              "step3"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "prettify() 方法的作用",
            "term_refs": [
              {
                "display": "prettify()",
                "en": "prettify()"
              }
            ],
            "variants": [
              {
                "back": "prettify()",
                "estimated_seconds": 5,
                "explanation": "prettify() 方法将解析后的 HTML 树以更易读的格式输出。",
                "front": "BeautifulSoup 对象的哪个方法可以打印出格式清晰、缩进良好的 HTML 代码？",
                "question_id": "q_flash_bs4_pretty_v1"
              },
              {
                "back": "格式化的 HTML 字符串。",
                "estimated_seconds": 6,
                "explanation": "它把杂乱的 HTML 源代码整理成带有缩进的、结构清晰的文本。",
                "front": "使用 BeautifulSoup 解析 HTML 后，调用 `soup.prettify()` 会返回什么？",
                "question_id": "q_flash_bs4_pretty_v2"
              }
            ]
          },
          {
            "concept_key": "beautifulsoup_extract_text",
            "coverage_tags": [
              "beautifulsoup_extract_text"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_bs4_get_text",
            "learning_goal": "学生能记住 get_text() 方法的功能。",
            "linked_steps": [
              "step3"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "get_text() 方法的作用",
            "term_refs": [
              {
                "display": "get_text()",
                "en": "get_text()"
              }
            ],
            "variants": [
              {
                "back": "get_text()",
                "estimated_seconds": 5,
                "explanation": "get_text() 会提取网页中所有可见的文本，忽略 HTML 标签。",
                "front": "要从一个 BeautifulSoup 对象中提取网页里所有的纯文本内容（去掉所有 HTML 标签），应该调用哪个方法？",
                "question_id": "q_flash_bs4_get_text_v1"
              },
              {
                "back": "一个字符串，包含网页中所有文本内容。",
                "estimated_seconds": 5,
                "explanation": "它把所有标签内的文本拼接成一个字符串返回。",
                "front": "`soup.get_text()` 返回的是什么类型的数据？",
                "question_id": "q_flash_bs4_get_text_v2"
              }
            ]
          },
          {
            "concept_key": "beautifulsoup_find_and_find_all",
            "coverage_tags": [
              "beautifulsoup_find_and_find_all"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_bs4_find",
            "learning_goal": "学生能准确说出 find() 方法的用途和返回值。",
            "linked_steps": [
              "step3"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "find() 方法的功能",
            "term_refs": [
              {
                "display": "find()",
                "en": "find()"
              }
            ],
            "variants": [
              {
                "back": "find('p')",
                "estimated_seconds": 5,
                "explanation": "find() 方法返回匹配的第一个标签对象。",
                "front": "要提取网页中第一个 `<p>` 标签的内容，应该使用 BeautifulSoup 对象的哪个方法？",
                "question_id": "q_flash_bs4_find_v1"
              },
              {
                "back": "网页中第一个 `<a>` 标签对象。",
                "estimated_seconds": 5,
                "explanation": "find() 只返回第一个匹配的元素。",
                "front": "`soup.find('a')` 返回的是什么？",
                "question_id": "q_flash_bs4_find_v2"
              }
            ]
          },
          {
            "concept_key": "beautifulsoup_find_and_find_all",
            "coverage_tags": [
              "beautifulsoup_find_and_find_all"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_bs4_find_all",
            "learning_goal": "学生能准确说出 find_all() 方法的用途和返回值。",
            "linked_steps": [
              "step3"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "find_all() 方法的功能",
            "term_refs": [
              {
                "display": "find_all()",
                "en": "find_all()"
              }
            ],
            "variants": [
              {
                "back": "find_all('a')",
                "estimated_seconds": 5,
                "explanation": "find_all() 返回一个包含所有匹配标签的列表。",
                "front": "要提取网页中所有超链接（`<a>` 标签），应该使用 BeautifulSoup 对象的哪个方法？",
                "question_id": "q_flash_bs4_find_all_v1"
              },
              {
                "back": "一个列表，包含网页中所有 `<p>` 标签对象。",
                "estimated_seconds": 5,
                "explanation": "find_all() 返回所有匹配元素的列表，即使只有一个元素。",
                "front": "`soup.find_all('p')` 返回的是什么？",
                "question_id": "q_flash_bs4_find_all_v2"
              }
            ]
          },
          {
            "concept_key": "beautifulsoup_table_to_dataframe",
            "coverage_tags": [
              "beautifulsoup_table_to_dataframe"
            ],
            "difficulty": "medium",
            "family_id": "qf_flash_bs4_table_pandas",
            "learning_goal": "学生能记住将 HTML 表格转换为 DataFrame 的基本思路。",
            "linked_steps": [
              "step3"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "将 HTML 表格转换为 DataFrame 的步骤",
            "term_refs": [
              {
                "display": "pandas DataFrame",
                "en": "pandas DataFrame"
              }
            ],
            "variants": [
              {
                "back": "pandas",
                "estimated_seconds": 5,
                "explanation": "找到表格后，提取表头和数据行，然后创建 pandas DataFrame。",
                "front": "用 BeautifulSoup 找到 `<table>` 标签后，通常配合哪个 Python 库将其转换为 DataFrame？",
                "question_id": "q_flash_bs4_table_pandas_v1"
              },
              {
                "back": "表格的表头（列名）。",
                "estimated_seconds": 6,
                "explanation": "<th> 标签定义了表格的标题单元格。",
                "front": "在提取表格数据时，`find_all('th')` 通常用于提取什么？",
                "question_id": "q_flash_bs4_table_pandas_v2"
              }
            ]
          },
          {
            "concept_key": "beautifulsoup_intro_and_setup",
            "coverage_tags": [
              "beautifulsoup_intro_and_setup",
              "requests_download_raw_content"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_bs4_workflow",
            "learning_goal": "学生能记住使用 BeautifulSoup 爬取网页的标准两步流程。",
            "linked_steps": [
              "step3"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "BeautifulSoup 爬虫的标准工作流程",
            "term_refs": [
              {
                "display": "Requests",
                "en": "Requests"
              },
              {
                "display": "BeautifulSoup",
                "en": "BeautifulSoup"
              }
            ],
            "variants": [
              {
                "back": "用 Requests 库下载网页的原始 HTML 内容。",
                "estimated_seconds": 6,
                "explanation": "例如：`r = requests.get(url)`，然后获取 `r.text`。",
                "front": "使用 BeautifulSoup 爬取网页时，第一步应该做什么？",
                "question_id": "q_flash_bs4_workflow_v1"
              },
              {
                "back": "将下载的原始 HTML 内容交给 BeautifulSoup 解析。",
                "estimated_seconds": 6,
                "explanation": "例如：`soup = BeautifulSoup(text, 'html.parser')`。",
                "front": "使用 BeautifulSoup 爬取网页时，第二步应该做什么？",
                "question_id": "q_flash_bs4_workflow_v2"
              }
            ]
          }
        ],
        "lesson_id": "L2",
        "longform_families": [
          {
            "concept_key": "beautifulsoup_table_to_dataframe",
            "coverage_tags": [
              "beautifulsoup_find_and_find_all",
              "beautifulsoup_table_to_dataframe"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_bs4_extract_table",
            "learning_goal": "学生能描述使用 BeautifulSoup 提取 HTML 表格并转换为 pandas DataFrame 的完整步骤。",
            "linked_steps": [
              "step3"
            ],
            "question_type": "mechanism_trace",
            "term_refs": [
              {
                "display": "BeautifulSoup",
                "en": "BeautifulSoup"
              },
              {
                "display": "pandas DataFrame",
                "en": "pandas DataFrame"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "1. 如何找到这个表格？",
                  "2. 如何提取表头？",
                  "3. 如何提取数据行？",
                  "4. 如何创建 DataFrame？"
                ],
                "question_id": "q_long_bs4_extract_table_v1",
                "reference_answer": [
                  "1. 使用 `table = soup.find('table')` 找到第一个表格，或通过 id/class 精确定位。",
                  "2. 使用 `headers = [th.text for th in table.find_all('th')]` 提取所有表头文本。",
                  "3. 使用 `for tr in table.find_all('tr')[1:]:` 跳过表头行，然后对每一行使用 `cells = tr.find_all('td')` 提取单元格，再用列表推导式获取文本。",
                  "4. 使用 `df = pd.DataFrame(data, columns=headers)` 创建 DataFrame，其中 data 是行数据的列表。"
                ],
                "rubric_points": [
                  "使用 find('table') 或 find_all('table') 定位到目标表格",
                  "使用 find_all('th') 提取表头文本并存入列表",
                  "遍历除表头外的所有行 (find_all('tr')[1:])，对每行使用 find_all('td') 提取单元格文本",
                  "使用 pandas 的 DataFrame 构造函数，传入数据行列表和列名列表"
                ],
                "stem": "假设你有一个包含股票数据的 HTML 页面，其中有一个 `<table>` 标签，表头包含“Symbol”、“Price”、“Volume”。请描述使用 BeautifulSoup 和 pandas 将这个表格提取为 DataFrame 的步骤。"
              },
              {
                "estimated_seconds": 100,
                "prompt_blocks": [
                  "1. 如何定位到 id 为 constituents 的表格？",
                  "2. 如何提取表头和数据？",
                  "3. 如何处理提取到的数据？"
                ],
                "question_id": "q_long_bs4_extract_table_v2",
                "reference_answer": [
                  "1. 使用 `table = soup.find('table', id='constituents')` 直接定位到目标表格。",
                  "2. 使用 `table.find_all('th')` 提取表头，使用 `table.find_all('tr')[1:]` 遍历数据行，每行用 `find_all('td')` 提取单元格。",
                  "3. 将每行数据存入列表，最后用 `pd.DataFrame(data, columns=headers)` 创建 DataFrame。"
                ],
                "rubric_points": [
                  "使用 soup.find('table', id='constituents') 精确定位",
                  "使用 find_all('th') 和 find_all('tr') 提取表头和数据行",
                  "将提取的数据整理成列表，最后转换为 DataFrame 或进行其他处理"
                ],
                "stem": "假设一个网页中有多个表格，你只想提取 id 为 \"constituents\" 的那个表格。请描述如何使用 BeautifulSoup 定位并提取该表格的数据。"
              }
            ]
          }
        ],
        "quiz_families": [
          {
            "concept_key": "beautifulsoup_intro_and_setup",
            "coverage_tags": [
              "beautifulsoup_intro_and_setup",
              "requests_download_raw_content"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_bs4_workflow",
            "learning_goal": "学生能在测验中正确识别使用 BeautifulSoup 的标准工作流程。",
            "linked_steps": [
              "step3"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "Requests",
                "en": "Requests"
              },
              {
                "display": "BeautifulSoup",
                "en": "BeautifulSoup"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "标准流程是先用 Requests 下载网页原始内容，然后交给 BeautifulSoup 解析。",
                "options": [
                  "先用 BeautifulSoup 解析，再用 Requests 下载",
                  "先用 Requests 下载，再用 BeautifulSoup 解析",
                  "只用 BeautifulSoup 即可完成下载和解析",
                  "先用 pandas 读取，再用 BeautifulSoup 格式化"
                ],
                "question_id": "q_quiz_bs4_workflow_v1",
                "stem": "以下哪个选项正确描述了使用 BeautifulSoup 爬取网页的标准步骤？"
              },
              {
                "answer": 1,
                "estimated_seconds": 20,
                "explanation": "先通过 requests.get(url).text 获取原始 HTML 字符串，再创建 BeautifulSoup 对象。",
                "options": [
                  "soup = BeautifulSoup(url); text = soup.get_text()",
                  "text = requests.get(url).text; soup = BeautifulSoup(text, 'html.parser')",
                  "soup = requests.get(url); text = soup.prettify()",
                  "text = pandas.read_html(url); soup = BeautifulSoup(text)"
                ],
                "question_id": "q_quiz_bs4_workflow_v2",
                "stem": "要从网页 `https://example.com` 获取内容并用 BeautifulSoup 解析，正确的代码顺序是？"
              }
            ]
          },
          {
            "concept_key": "beautifulsoup_parse_and_pretty_print",
            "coverage_tags": [
              "beautifulsoup_parse_and_pretty_print",
              "beautifulsoup_extract_text"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_bs4_methods",
            "learning_goal": "学生能区分 prettify() 和 get_text() 的不同用途。",
            "linked_steps": [
              "step3"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "prettify()",
                "en": "prettify()"
              },
              {
                "display": "get_text()",
                "en": "get_text()"
              }
            ],
            "variants": [
              {
                "answer": 2,
                "estimated_seconds": 15,
                "explanation": "prettify() 会以带缩进的格式打印 HTML，方便检查结构。",
                "options": [
                  "get_text()",
                  "find('p')",
                  "prettify()",
                  "find_all('a')"
                ],
                "question_id": "q_quiz_bs4_methods_v1",
                "stem": "你想查看网页的 HTML 结构是否完整，标签是否正确嵌套，应该使用 BeautifulSoup 的哪个方法？"
              },
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "get_text() 会提取所有标签内的文本，并移除所有 HTML 标记。",
                "options": [
                  "prettify()",
                  "get_text()",
                  "find('html')",
                  "find_all('body')"
                ],
                "question_id": "q_quiz_bs4_methods_v2",
                "stem": "你只关心网页中的文字内容，不需要任何 HTML 标签，应该使用哪个方法？"
              }
            ]
          },
          {
            "concept_key": "beautifulsoup_find_and_find_all",
            "coverage_tags": [
              "beautifulsoup_find_and_find_all"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_bs4_find_vs_findall",
            "learning_goal": "学生能辨析 find() 和 find_all() 的区别并正确选择。",
            "linked_steps": [
              "step3"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "find()",
                "en": "find()"
              },
              {
                "display": "find_all()",
                "en": "find_all()"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "find('div') 只返回第一个匹配的 <div> 标签。",
                "options": [
                  "soup.find_all('div')",
                  "soup.find('div')",
                  "soup.get_text('div')",
                  "soup.prettify('div')"
                ],
                "question_id": "q_quiz_bs4_find_vs_findall_v1",
                "stem": "一个网页中有多个 `<div>` 标签，你只想获取第一个 `<div>` 的内容，应该使用哪个方法？"
              },
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "find_all('a') 返回一个包含所有 <a> 标签的列表，可以遍历。",
                "options": [
                  "soup.find('a')",
                  "soup.find_all('a')",
                  "soup.get_text('a')",
                  "soup.select_one('a')"
                ],
                "question_id": "q_quiz_bs4_find_vs_findall_v2",
                "stem": "你需要遍历网页中所有的超链接（`<a>` 标签）来提取每个链接的 URL，应该使用哪个方法？"
              }
            ]
          }
        ],
        "source": {
          "coverage_checklist": "L2: Data scraping and database management with Python - Agenda: Web scraping using different python libraries",
          "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
          "lesson_map": "L2 step3: Using BeautifulSoup to parse HTML and extract data",
          "plain_text": "pipeline/1-plain/L2/plain.txt",
          "related": [
            "pipeline/2-related_important/course_desc.md"
          ],
          "source_outline": "L2: Data scraping and database management with Python - BeautifulSoup: Popular python library for searching and parsing HTML and XML data"
        },
        "target_language": "zh-CN"
      },
      "question_bank_path": "research/pipeline/3-guided_story/L2/step3/question_bank.json",
      "question_families": [
        {
          "family_id": "qf_flash_bs4_purpose",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "beautifulsoup_intro_and_setup",
            "coverage_tags": [
              "beautifulsoup_intro_and_setup"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_bs4_purpose",
            "learning_goal": "学生能准确说出 BeautifulSoup 库的核心用途。",
            "linked_steps": [
              "step3"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "BeautifulSoup 库的核心功能是什么？",
            "term_refs": [
              {
                "display": "BeautifulSoup",
                "en": "BeautifulSoup"
              }
            ],
            "variants": [
              {
                "back": "解析 HTML 和 XML 数据，将其整理成可操作的树状结构。",
                "estimated_seconds": 8,
                "explanation": "BeautifulSoup 就像一个 HTML 的“瑞士军刀”，专门用于搜索和解析网页标记语言。",
                "front": "BeautifulSoup 库在 Python 爬虫中的主要作用是什么？",
                "question_id": "q_flash_bs4_purpose_v1"
              },
              {
                "back": "Requests 库。",
                "estimated_seconds": 6,
                "explanation": "先用 Requests 下载网页原始内容，再交给 BeautifulSoup 解析。",
                "front": "在 Python 爬虫中，BeautifulSoup 通常与哪个库配合使用来下载网页内容？",
                "question_id": "q_flash_bs4_purpose_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_bs4_pretty",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "beautifulsoup_parse_and_pretty_print",
            "coverage_tags": [
              "beautifulsoup_parse_and_pretty_print"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_bs4_pretty",
            "learning_goal": "学生能记住 prettify() 方法的功能。",
            "linked_steps": [
              "step3"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "prettify() 方法的作用",
            "term_refs": [
              {
                "display": "prettify()",
                "en": "prettify()"
              }
            ],
            "variants": [
              {
                "back": "prettify()",
                "estimated_seconds": 5,
                "explanation": "prettify() 方法将解析后的 HTML 树以更易读的格式输出。",
                "front": "BeautifulSoup 对象的哪个方法可以打印出格式清晰、缩进良好的 HTML 代码？",
                "question_id": "q_flash_bs4_pretty_v1"
              },
              {
                "back": "格式化的 HTML 字符串。",
                "estimated_seconds": 6,
                "explanation": "它把杂乱的 HTML 源代码整理成带有缩进的、结构清晰的文本。",
                "front": "使用 BeautifulSoup 解析 HTML 后，调用 `soup.prettify()` 会返回什么？",
                "question_id": "q_flash_bs4_pretty_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_bs4_get_text",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "beautifulsoup_extract_text",
            "coverage_tags": [
              "beautifulsoup_extract_text"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_bs4_get_text",
            "learning_goal": "学生能记住 get_text() 方法的功能。",
            "linked_steps": [
              "step3"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "get_text() 方法的作用",
            "term_refs": [
              {
                "display": "get_text()",
                "en": "get_text()"
              }
            ],
            "variants": [
              {
                "back": "get_text()",
                "estimated_seconds": 5,
                "explanation": "get_text() 会提取网页中所有可见的文本，忽略 HTML 标签。",
                "front": "要从一个 BeautifulSoup 对象中提取网页里所有的纯文本内容（去掉所有 HTML 标签），应该调用哪个方法？",
                "question_id": "q_flash_bs4_get_text_v1"
              },
              {
                "back": "一个字符串，包含网页中所有文本内容。",
                "estimated_seconds": 5,
                "explanation": "它把所有标签内的文本拼接成一个字符串返回。",
                "front": "`soup.get_text()` 返回的是什么类型的数据？",
                "question_id": "q_flash_bs4_get_text_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_bs4_find",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "beautifulsoup_find_and_find_all",
            "coverage_tags": [
              "beautifulsoup_find_and_find_all"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_bs4_find",
            "learning_goal": "学生能准确说出 find() 方法的用途和返回值。",
            "linked_steps": [
              "step3"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "find() 方法的功能",
            "term_refs": [
              {
                "display": "find()",
                "en": "find()"
              }
            ],
            "variants": [
              {
                "back": "find('p')",
                "estimated_seconds": 5,
                "explanation": "find() 方法返回匹配的第一个标签对象。",
                "front": "要提取网页中第一个 `<p>` 标签的内容，应该使用 BeautifulSoup 对象的哪个方法？",
                "question_id": "q_flash_bs4_find_v1"
              },
              {
                "back": "网页中第一个 `<a>` 标签对象。",
                "estimated_seconds": 5,
                "explanation": "find() 只返回第一个匹配的元素。",
                "front": "`soup.find('a')` 返回的是什么？",
                "question_id": "q_flash_bs4_find_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_bs4_find_all",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "beautifulsoup_find_and_find_all",
            "coverage_tags": [
              "beautifulsoup_find_and_find_all"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_bs4_find_all",
            "learning_goal": "学生能准确说出 find_all() 方法的用途和返回值。",
            "linked_steps": [
              "step3"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "find_all() 方法的功能",
            "term_refs": [
              {
                "display": "find_all()",
                "en": "find_all()"
              }
            ],
            "variants": [
              {
                "back": "find_all('a')",
                "estimated_seconds": 5,
                "explanation": "find_all() 返回一个包含所有匹配标签的列表。",
                "front": "要提取网页中所有超链接（`<a>` 标签），应该使用 BeautifulSoup 对象的哪个方法？",
                "question_id": "q_flash_bs4_find_all_v1"
              },
              {
                "back": "一个列表，包含网页中所有 `<p>` 标签对象。",
                "estimated_seconds": 5,
                "explanation": "find_all() 返回所有匹配元素的列表，即使只有一个元素。",
                "front": "`soup.find_all('p')` 返回的是什么？",
                "question_id": "q_flash_bs4_find_all_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_bs4_table_pandas",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "beautifulsoup_table_to_dataframe",
            "coverage_tags": [
              "beautifulsoup_table_to_dataframe"
            ],
            "difficulty": "medium",
            "family_id": "qf_flash_bs4_table_pandas",
            "learning_goal": "学生能记住将 HTML 表格转换为 DataFrame 的基本思路。",
            "linked_steps": [
              "step3"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "将 HTML 表格转换为 DataFrame 的步骤",
            "term_refs": [
              {
                "display": "pandas DataFrame",
                "en": "pandas DataFrame"
              }
            ],
            "variants": [
              {
                "back": "pandas",
                "estimated_seconds": 5,
                "explanation": "找到表格后，提取表头和数据行，然后创建 pandas DataFrame。",
                "front": "用 BeautifulSoup 找到 `<table>` 标签后，通常配合哪个 Python 库将其转换为 DataFrame？",
                "question_id": "q_flash_bs4_table_pandas_v1"
              },
              {
                "back": "表格的表头（列名）。",
                "estimated_seconds": 6,
                "explanation": "<th> 标签定义了表格的标题单元格。",
                "front": "在提取表格数据时，`find_all('th')` 通常用于提取什么？",
                "question_id": "q_flash_bs4_table_pandas_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_bs4_workflow",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "beautifulsoup_intro_and_setup",
            "coverage_tags": [
              "beautifulsoup_intro_and_setup",
              "requests_download_raw_content"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_bs4_workflow",
            "learning_goal": "学生能记住使用 BeautifulSoup 爬取网页的标准两步流程。",
            "linked_steps": [
              "step3"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "BeautifulSoup 爬虫的标准工作流程",
            "term_refs": [
              {
                "display": "Requests",
                "en": "Requests"
              },
              {
                "display": "BeautifulSoup",
                "en": "BeautifulSoup"
              }
            ],
            "variants": [
              {
                "back": "用 Requests 库下载网页的原始 HTML 内容。",
                "estimated_seconds": 6,
                "explanation": "例如：`r = requests.get(url)`，然后获取 `r.text`。",
                "front": "使用 BeautifulSoup 爬取网页时，第一步应该做什么？",
                "question_id": "q_flash_bs4_workflow_v1"
              },
              {
                "back": "将下载的原始 HTML 内容交给 BeautifulSoup 解析。",
                "estimated_seconds": 6,
                "explanation": "例如：`soup = BeautifulSoup(text, 'html.parser')`。",
                "front": "使用 BeautifulSoup 爬取网页时，第二步应该做什么？",
                "question_id": "q_flash_bs4_workflow_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_bs4_workflow",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "beautifulsoup_intro_and_setup",
            "coverage_tags": [
              "beautifulsoup_intro_and_setup",
              "requests_download_raw_content"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_bs4_workflow",
            "learning_goal": "学生能在测验中正确识别使用 BeautifulSoup 的标准工作流程。",
            "linked_steps": [
              "step3"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "Requests",
                "en": "Requests"
              },
              {
                "display": "BeautifulSoup",
                "en": "BeautifulSoup"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "标准流程是先用 Requests 下载网页原始内容，然后交给 BeautifulSoup 解析。",
                "options": [
                  "先用 BeautifulSoup 解析，再用 Requests 下载",
                  "先用 Requests 下载，再用 BeautifulSoup 解析",
                  "只用 BeautifulSoup 即可完成下载和解析",
                  "先用 pandas 读取，再用 BeautifulSoup 格式化"
                ],
                "question_id": "q_quiz_bs4_workflow_v1",
                "stem": "以下哪个选项正确描述了使用 BeautifulSoup 爬取网页的标准步骤？"
              },
              {
                "answer": 1,
                "estimated_seconds": 20,
                "explanation": "先通过 requests.get(url).text 获取原始 HTML 字符串，再创建 BeautifulSoup 对象。",
                "options": [
                  "soup = BeautifulSoup(url); text = soup.get_text()",
                  "text = requests.get(url).text; soup = BeautifulSoup(text, 'html.parser')",
                  "soup = requests.get(url); text = soup.prettify()",
                  "text = pandas.read_html(url); soup = BeautifulSoup(text)"
                ],
                "question_id": "q_quiz_bs4_workflow_v2",
                "stem": "要从网页 `https://example.com` 获取内容并用 BeautifulSoup 解析，正确的代码顺序是？"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_bs4_methods",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "beautifulsoup_parse_and_pretty_print",
            "coverage_tags": [
              "beautifulsoup_parse_and_pretty_print",
              "beautifulsoup_extract_text"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_bs4_methods",
            "learning_goal": "学生能区分 prettify() 和 get_text() 的不同用途。",
            "linked_steps": [
              "step3"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "prettify()",
                "en": "prettify()"
              },
              {
                "display": "get_text()",
                "en": "get_text()"
              }
            ],
            "variants": [
              {
                "answer": 2,
                "estimated_seconds": 15,
                "explanation": "prettify() 会以带缩进的格式打印 HTML，方便检查结构。",
                "options": [
                  "get_text()",
                  "find('p')",
                  "prettify()",
                  "find_all('a')"
                ],
                "question_id": "q_quiz_bs4_methods_v1",
                "stem": "你想查看网页的 HTML 结构是否完整，标签是否正确嵌套，应该使用 BeautifulSoup 的哪个方法？"
              },
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "get_text() 会提取所有标签内的文本，并移除所有 HTML 标记。",
                "options": [
                  "prettify()",
                  "get_text()",
                  "find('html')",
                  "find_all('body')"
                ],
                "question_id": "q_quiz_bs4_methods_v2",
                "stem": "你只关心网页中的文字内容，不需要任何 HTML 标签，应该使用哪个方法？"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_bs4_find_vs_findall",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "beautifulsoup_find_and_find_all",
            "coverage_tags": [
              "beautifulsoup_find_and_find_all"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_bs4_find_vs_findall",
            "learning_goal": "学生能辨析 find() 和 find_all() 的区别并正确选择。",
            "linked_steps": [
              "step3"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "find()",
                "en": "find()"
              },
              {
                "display": "find_all()",
                "en": "find_all()"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "find('div') 只返回第一个匹配的 <div> 标签。",
                "options": [
                  "soup.find_all('div')",
                  "soup.find('div')",
                  "soup.get_text('div')",
                  "soup.prettify('div')"
                ],
                "question_id": "q_quiz_bs4_find_vs_findall_v1",
                "stem": "一个网页中有多个 `<div>` 标签，你只想获取第一个 `<div>` 的内容，应该使用哪个方法？"
              },
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "find_all('a') 返回一个包含所有 <a> 标签的列表，可以遍历。",
                "options": [
                  "soup.find('a')",
                  "soup.find_all('a')",
                  "soup.get_text('a')",
                  "soup.select_one('a')"
                ],
                "question_id": "q_quiz_bs4_find_vs_findall_v2",
                "stem": "你需要遍历网页中所有的超链接（`<a>` 标签）来提取每个链接的 URL，应该使用哪个方法？"
              }
            ]
          }
        },
        {
          "family_id": "qf_long_bs4_extract_table",
          "kind": "longform_families",
          "summary": {
            "concept_key": "beautifulsoup_table_to_dataframe",
            "coverage_tags": [
              "beautifulsoup_find_and_find_all",
              "beautifulsoup_table_to_dataframe"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_bs4_extract_table",
            "learning_goal": "学生能描述使用 BeautifulSoup 提取 HTML 表格并转换为 pandas DataFrame 的完整步骤。",
            "linked_steps": [
              "step3"
            ],
            "question_type": "mechanism_trace",
            "term_refs": [
              {
                "display": "BeautifulSoup",
                "en": "BeautifulSoup"
              },
              {
                "display": "pandas DataFrame",
                "en": "pandas DataFrame"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "1. 如何找到这个表格？",
                  "2. 如何提取表头？",
                  "3. 如何提取数据行？",
                  "4. 如何创建 DataFrame？"
                ],
                "question_id": "q_long_bs4_extract_table_v1",
                "reference_answer": [
                  "1. 使用 `table = soup.find('table')` 找到第一个表格，或通过 id/class 精确定位。",
                  "2. 使用 `headers = [th.text for th in table.find_all('th')]` 提取所有表头文本。",
                  "3. 使用 `for tr in table.find_all('tr')[1:]:` 跳过表头行，然后对每一行使用 `cells = tr.find_all('td')` 提取单元格，再用列表推导式获取文本。",
                  "4. 使用 `df = pd.DataFrame(data, columns=headers)` 创建 DataFrame，其中 data 是行数据的列表。"
                ],
                "rubric_points": [
                  "使用 find('table') 或 find_all('table') 定位到目标表格",
                  "使用 find_all('th') 提取表头文本并存入列表",
                  "遍历除表头外的所有行 (find_all('tr')[1:])，对每行使用 find_all('td') 提取单元格文本",
                  "使用 pandas 的 DataFrame 构造函数，传入数据行列表和列名列表"
                ],
                "stem": "假设你有一个包含股票数据的 HTML 页面，其中有一个 `<table>` 标签，表头包含“Symbol”、“Price”、“Volume”。请描述使用 BeautifulSoup 和 pandas 将这个表格提取为 DataFrame 的步骤。"
              },
              {
                "estimated_seconds": 100,
                "prompt_blocks": [
                  "1. 如何定位到 id 为 constituents 的表格？",
                  "2. 如何提取表头和数据？",
                  "3. 如何处理提取到的数据？"
                ],
                "question_id": "q_long_bs4_extract_table_v2",
                "reference_answer": [
                  "1. 使用 `table = soup.find('table', id='constituents')` 直接定位到目标表格。",
                  "2. 使用 `table.find_all('th')` 提取表头，使用 `table.find_all('tr')[1:]` 遍历数据行，每行用 `find_all('td')` 提取单元格。",
                  "3. 将每行数据存入列表，最后用 `pd.DataFrame(data, columns=headers)` 创建 DataFrame。"
                ],
                "rubric_points": [
                  "使用 soup.find('table', id='constituents') 精确定位",
                  "使用 find_all('th') 和 find_all('tr') 提取表头和数据行",
                  "将提取的数据整理成列表，最后转换为 DataFrame 或进行其他处理"
                ],
                "stem": "假设一个网页中有多个表格，你只想提取 id 为 \"constituents\" 的那个表格。请描述如何使用 BeautifulSoup 定位并提取该表格的数据。"
              }
            ]
          }
        }
      ],
      "sequence_id": "step3",
      "step": {
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
      "step_path": "research/pipeline/3-guided_story/L2/step3/step.json"
    },
    {
      "question_bank": {
        "coverage_map": [
          {
            "coverage_tag": "real_world_scraping_sp500",
            "covered_by": [
              "qf_flash_sp500_goal",
              "qf_flash_user_agent",
              "qf_quiz_user_agent_fail",
              "qf_quiz_table_id",
              "qf_long_sp500_process"
            ],
            "description": "Real-world scraping: S&P 500 list from Wikipedia"
          },
          {
            "coverage_tag": "handling_anti_bot_measures",
            "covered_by": [
              "qf_flash_user_agent",
              "qf_quiz_user_agent_fail"
            ],
            "description": "Handling anti-bot measures: User-Agent"
          },
          {
            "coverage_tag": "beautifulsoup_table_extraction",
            "covered_by": [
              "qf_flash_sp500_goal",
              "qf_quiz_table_id",
              "qf_long_sp500_process"
            ],
            "description": "Using BeautifulSoup to find and extract table data by id"
          },
          {
            "coverage_tag": "dataframe_creation_from_scraped_data",
            "covered_by": [
              "qf_flash_sp500_goal",
              "qf_long_sp500_process"
            ],
            "description": "Creating a pandas DataFrame from scraped table rows"
          }
        ],
        "flashcard_families": [
          {
            "concept_key": "anti_bot_user_agent",
            "coverage_tags": [
              "handling_anti_bot_measures"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_user_agent",
            "learning_goal": "学生能准确回忆 User-Agent 的作用和设置方法，理解反爬机制的基本原理。",
            "linked_steps": [
              "step4"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "User-Agent 是什么、为什么需要设置、如何设置。",
            "term_refs": [
              {
                "display": "User-Agent",
                "en": "User-Agent"
              }
            ],
            "variants": [
              {
                "back": "因为缺少正确的 User-Agent，网站会认为请求来自非浏览器客户端而拒绝访问。",
                "estimated_seconds": 10,
                "explanation": "许多网站通过检查 HTTP 请求头中的 User-Agent 字段来识别客户端类型。",
                "front": "为什么直接用 `requests.get(url)` 请求维基百科会失败？",
                "question_id": "q_flash_user_agent_v1"
              },
              {
                "back": "在 `headers` 字典中设置 `\"user-agent\": \"Mozilla/5.0 ...\"`，然后传给 `requests.get(url, headers=headers)`。",
                "estimated_seconds": 12,
                "explanation": "常见的 User-Agent 字符串模仿主流浏览器，例如 Chrome 或 Firefox。",
                "front": "在 Python 中，如何为 Requests 请求设置 User-Agent 来伪装成浏览器？",
                "question_id": "q_flash_user_agent_v2"
              },
              {
                "back": "字段名是 `user-agent`，用于标识客户端类型（如浏览器、爬虫）。",
                "estimated_seconds": 8,
                "explanation": "服务器可以根据 User-Agent 决定是否提供服务或返回不同内容。",
                "front": "User-Agent 是 HTTP 请求头中的哪个字段？它的作用是什么？",
                "question_id": "q_flash_user_agent_v3"
              }
            ]
          },
          {
            "concept_key": "sp500_scraping_process",
            "coverage_tags": [
              "real_world_scraping_sp500",
              "beautifulsoup_table_extraction",
              "dataframe_creation_from_scraped_data"
            ],
            "difficulty": "medium",
            "family_id": "qf_flash_sp500_goal",
            "learning_goal": "学生能回忆爬取 S&P 500 成分股列表的核心步骤和关键代码。",
            "linked_steps": [
              "step4"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "爬取 S&P 500 的完整流程：请求、解析、定位表格、提取数据、创建 DataFrame。",
            "term_refs": [
              {
                "display": "BeautifulSoup",
                "en": "BeautifulSoup"
              },
              {
                "display": "DataFrame",
                "en": "DataFrame"
              }
            ],
            "variants": [
              {
                "back": "使用 `soup.find('table', id='constituents')`，通过表格的 id 属性精确定位。",
                "estimated_seconds": 10,
                "explanation": "维基百科页面上有多个表格，`id='constituents'` 唯一标识了成分股表格。",
                "front": "爬取维基百科 S&P 500 成分股列表时，如何用 BeautifulSoup 定位到目标表格？",
                "question_id": "q_flash_sp500_goal_v1"
              },
              {
                "back": "pandas DataFrame。",
                "estimated_seconds": 6,
                "explanation": "DataFrame 可以方便地处理表格数据，支持列名、索引和多种数据分析操作。",
                "front": "成功提取 S&P 500 表格后，通常用什么 Python 数据结构来存储和操作这些数据？",
                "question_id": "q_flash_sp500_goal_v2"
              },
              {
                "back": "`df = pd.DataFrame(rows, columns=headers)`",
                "estimated_seconds": 8,
                "explanation": "`rows` 是数据行列表，`headers` 是表头列表。",
                "front": "爬取 S&P 500 列表时，提取表头和数据行后，创建 DataFrame 的典型代码是什么？",
                "question_id": "q_flash_sp500_goal_v3"
              }
            ]
          }
        ],
        "lesson_id": "L2",
        "longform_families": [
          {
            "concept_key": "sp500_scraping_process",
            "coverage_tags": [
              "real_world_scraping_sp500",
              "beautifulsoup_table_extraction",
              "dataframe_creation_from_scraped_data"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_sp500_process",
            "learning_goal": "学生能完整描述爬取 S&P 500 成分股列表的步骤，包括处理反爬、解析 HTML、定位表格和创建 DataFrame。",
            "linked_steps": [
              "step4"
            ],
            "question_type": "mechanism_trace",
            "term_refs": [
              {
                "display": "User-Agent",
                "en": "User-Agent"
              },
              {
                "display": "BeautifulSoup",
                "en": "BeautifulSoup"
              },
              {
                "display": "DataFrame",
                "en": "DataFrame"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "处理反爬机制",
                  "获取页面内容",
                  "解析 HTML",
                  "定位目标表格",
                  "提取数据并创建 DataFrame"
                ],
                "question_id": "q_long_sp500_process_v1",
                "reference_answer": [
                  "1. 设置 User-Agent：在 headers 字典中添加 'user-agent': 'Mozilla/5.0 ...'，避免被维基百科的反爬机制拦截。",
                  "2. 发送请求：使用 requests.get(url, headers=headers) 获取页面 HTML 内容。",
                  "3. 解析 HTML：用 BeautifulSoup(text, 'html.parser') 创建 soup 对象，将原始 HTML 转换为可操作的树结构。",
                  "4. 定位表格：使用 soup.find('table', id='constituents') 找到 id 为 'constituents' 的目标表格。",
                  "5. 提取数据：遍历表头 <th> 获取列名，遍历数据行 <tr> 获取每行数据。",
                  "6. 创建 DataFrame：用 pd.DataFrame(rows, columns=headers) 将数据组织成表格形式，便于后续分析。"
                ],
                "rubric_points": [
                  "指出需要设置 User-Agent 来伪装成浏览器",
                  "使用 requests.get(url, headers=headers) 获取页面",
                  "用 BeautifulSoup 解析 HTML 文本",
                  "通过 soup.find('table', id='constituents') 定位表格",
                  "提取表头和数据行，用 pd.DataFrame(rows, columns=headers) 创建 DataFrame"
                ],
                "stem": "请描述使用 Python 从维基百科爬取 S&P 500 成分股列表的完整步骤，并解释每一步的目的。"
              },
              {
                "estimated_seconds": 150,
                "prompt_blocks": [
                  "导入必要的库",
                  "设置请求头",
                  "发送请求并获取响应",
                  "解析 HTML",
                  "定位并提取表格",
                  "创建 DataFrame"
                ],
                "question_id": "q_long_sp500_process_v2",
                "reference_answer": [
                  "```python",
                  "import requests",
                  "from bs4 import BeautifulSoup",
                  "import pandas as pd",
                  "",
                  "# 1. 设置请求头，伪装成浏览器",
                  "headers = {",
                  "    'user-agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36'",
                  "}",
                  "",
                  "# 2. 发送 GET 请求",
                  "url = 'https://en.wikipedia.org/wiki/List_of_S%26P_500_companies'",
                  "response = requests.get(url, headers=headers)",
                  "",
                  "# 3. 解析 HTML",
                  "soup = BeautifulSoup(response.text, 'html.parser')",
                  "",
                  "# 4. 定位目标表格",
                  "table = soup.find('table', id='constituents')",
                  "",
                  "# 5. 提取表头",
                  "headers_list = [th.text.strip() for th in table.find_all('th')]",
                  "",
                  "# 6. 提取数据行",
                  "rows = []",
                  "for tr in table.find_all('tr')[1:]:  # 跳过表头行",
                  "    cells = tr.find_all('td')",
                  "    row = [cell.text.strip() for cell in cells]",
                  "    rows.append(row)",
                  "",
                  "# 7. 创建 DataFrame",
                  "df = pd.DataFrame(rows, columns=headers_list)",
                  "print(df)",
                  "```",
                  "",
                  "关键部分解释：",
                  "- headers 字典中的 User-Agent 用于绕过反爬虫检测。",
                  "- find('table', id='constituents') 通过 id 精确定位表格，避免提取到其他无关表格。",
                  "- 使用列表推导式提取表头和数据行，代码简洁高效。",
                  "- pd.DataFrame 将数据组织成结构化表格，便于后续分析。"
                ],
                "rubric_points": [
                  "导入 requests, BeautifulSoup, pandas",
                  "定义 headers 字典包含 User-Agent",
                  "使用 requests.get 并传入 headers",
                  "用 BeautifulSoup 解析 response.text",
                  "使用 find 方法按 id 定位表格",
                  "提取表头和数据行，创建 DataFrame"
                ],
                "stem": "假设你需要爬取一个类似维基百科 S&P 500 页面的表格数据，但该网站有反爬虫机制。请写出完整的 Python 代码框架，并解释关键部分的作用。"
              }
            ]
          }
        ],
        "quiz_families": [
          {
            "concept_key": "anti_bot_user_agent",
            "coverage_tags": [
              "handling_anti_bot_measures"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_user_agent_fail",
            "learning_goal": "学生能辨析直接请求维基百科失败的根本原因。",
            "linked_steps": [
              "step4"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "User-Agent",
                "en": "User-Agent"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "许多网站通过检查 User-Agent 来识别并阻止非浏览器请求。设置一个常见的浏览器 User-Agent 即可解决。",
                "options": [
                  "维基百科服务器暂时宕机",
                  "请求缺少正确的 User-Agent 头",
                  "Python 的 requests 库版本过低",
                  "网络连接速度太慢导致超时"
                ],
                "question_id": "q_quiz_user_agent_fail_v1",
                "stem": "在爬取维基百科 S&P 500 页面时，直接使用 `requests.get(url)` 返回了错误信息。最可能的原因是什么？"
              },
              {
                "answer": 1,
                "estimated_seconds": 10,
                "explanation": "User-Agent 字段告诉服务器请求来自什么客户端（如浏览器、爬虫）。",
                "options": [
                  "Accept-Language",
                  "User-Agent",
                  "Content-Type",
                  "Cookie"
                ],
                "question_id": "q_quiz_user_agent_fail_v2",
                "stem": "以下哪个 HTTP 请求头字段用于标识客户端类型，常被网站用于反爬虫检测？"
              }
            ]
          },
          {
            "concept_key": "beautifulsoup_table_extraction",
            "coverage_tags": [
              "beautifulsoup_table_extraction"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_table_id",
            "learning_goal": "学生能选择正确的 BeautifulSoup 方法来定位具有特定 id 的表格。",
            "linked_steps": [
              "step4"
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
                "explanation": "`find('table', id='constituents')` 通过标签名和 id 属性精确定位到目标表格。",
                "options": [
                  "soup.find_all('table')",
                  "soup.find('table', id='constituents')",
                  "soup.select('table.constituents')",
                  "soup.find('table', class_='constituents')"
                ],
                "question_id": "q_quiz_table_id_v1",
                "stem": "维基百科 S&P 500 页面有多个表格，要提取 id 为 'constituents' 的表格，应该使用哪个 BeautifulSoup 方法？"
              },
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "`find` 返回第一个匹配的元素，`id` 是唯一的，所以 `find('table', id='data-table')` 是正确且高效的方式。",
                "options": [
                  "soup.find_all('table', id='data-table')",
                  "soup.find('table', id='data-table')",
                  "soup.find('table', class_='data-table')",
                  "soup.select('table#data-table')[0]"
                ],
                "question_id": "q_quiz_table_id_v2",
                "stem": "假设一个网页中有多个 `<table>`，但只有一个表格的 id 是 'data-table'。以下哪段代码能正确提取该表格？"
              }
            ]
          }
        ],
        "source": {
          "coverage_checklist": "L2: Data scraping and database management with Python - Agenda: Understand the basic website structure, Web scraping using different python libraries, The limitations of web scraping, Create a simple database using python, Understand the concept of adjusted price, Database design for financial market data storage",
          "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
          "lesson_map": "L2 guided_story steps: step1: What is web scraping and why it matters for algorithmic trading, step2: Understanding the basic structure of a website: HTML, CSS, and JavaScript, step3: Using BeautifulSoup to parse HTML and extract data, step4: Real-world scraping: S&P 500 list and handling anti-bot measures, step5: Using yfinance for reliable financial data access, step6: Storing scraped data in a SQLite database, step7: Database design considerations for financial data, step8: Limitations of web scraping and alternative tools",
          "plain_text": "pipeline/1-plain/L2/plain.txt",
          "related": [
            "pipeline/2-related_important/course_desc.md"
          ],
          "source_outline": "L2: Data scraping and database management with Python - Agenda: Understand the basic website structure, Web scraping using different python libraries, The limitations of web scraping, Create a simple database using python, Understand the concept of adjusted price, Database design for financial market data storage"
        },
        "target_language": "zh-CN"
      },
      "question_bank_path": "research/pipeline/3-guided_story/L2/step4/question_bank.json",
      "question_families": [
        {
          "family_id": "qf_flash_user_agent",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "anti_bot_user_agent",
            "coverage_tags": [
              "handling_anti_bot_measures"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_user_agent",
            "learning_goal": "学生能准确回忆 User-Agent 的作用和设置方法，理解反爬机制的基本原理。",
            "linked_steps": [
              "step4"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "User-Agent 是什么、为什么需要设置、如何设置。",
            "term_refs": [
              {
                "display": "User-Agent",
                "en": "User-Agent"
              }
            ],
            "variants": [
              {
                "back": "因为缺少正确的 User-Agent，网站会认为请求来自非浏览器客户端而拒绝访问。",
                "estimated_seconds": 10,
                "explanation": "许多网站通过检查 HTTP 请求头中的 User-Agent 字段来识别客户端类型。",
                "front": "为什么直接用 `requests.get(url)` 请求维基百科会失败？",
                "question_id": "q_flash_user_agent_v1"
              },
              {
                "back": "在 `headers` 字典中设置 `\"user-agent\": \"Mozilla/5.0 ...\"`，然后传给 `requests.get(url, headers=headers)`。",
                "estimated_seconds": 12,
                "explanation": "常见的 User-Agent 字符串模仿主流浏览器，例如 Chrome 或 Firefox。",
                "front": "在 Python 中，如何为 Requests 请求设置 User-Agent 来伪装成浏览器？",
                "question_id": "q_flash_user_agent_v2"
              },
              {
                "back": "字段名是 `user-agent`，用于标识客户端类型（如浏览器、爬虫）。",
                "estimated_seconds": 8,
                "explanation": "服务器可以根据 User-Agent 决定是否提供服务或返回不同内容。",
                "front": "User-Agent 是 HTTP 请求头中的哪个字段？它的作用是什么？",
                "question_id": "q_flash_user_agent_v3"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_sp500_goal",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "sp500_scraping_process",
            "coverage_tags": [
              "real_world_scraping_sp500",
              "beautifulsoup_table_extraction",
              "dataframe_creation_from_scraped_data"
            ],
            "difficulty": "medium",
            "family_id": "qf_flash_sp500_goal",
            "learning_goal": "学生能回忆爬取 S&P 500 成分股列表的核心步骤和关键代码。",
            "linked_steps": [
              "step4"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "爬取 S&P 500 的完整流程：请求、解析、定位表格、提取数据、创建 DataFrame。",
            "term_refs": [
              {
                "display": "BeautifulSoup",
                "en": "BeautifulSoup"
              },
              {
                "display": "DataFrame",
                "en": "DataFrame"
              }
            ],
            "variants": [
              {
                "back": "使用 `soup.find('table', id='constituents')`，通过表格的 id 属性精确定位。",
                "estimated_seconds": 10,
                "explanation": "维基百科页面上有多个表格，`id='constituents'` 唯一标识了成分股表格。",
                "front": "爬取维基百科 S&P 500 成分股列表时，如何用 BeautifulSoup 定位到目标表格？",
                "question_id": "q_flash_sp500_goal_v1"
              },
              {
                "back": "pandas DataFrame。",
                "estimated_seconds": 6,
                "explanation": "DataFrame 可以方便地处理表格数据，支持列名、索引和多种数据分析操作。",
                "front": "成功提取 S&P 500 表格后，通常用什么 Python 数据结构来存储和操作这些数据？",
                "question_id": "q_flash_sp500_goal_v2"
              },
              {
                "back": "`df = pd.DataFrame(rows, columns=headers)`",
                "estimated_seconds": 8,
                "explanation": "`rows` 是数据行列表，`headers` 是表头列表。",
                "front": "爬取 S&P 500 列表时，提取表头和数据行后，创建 DataFrame 的典型代码是什么？",
                "question_id": "q_flash_sp500_goal_v3"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_user_agent_fail",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "anti_bot_user_agent",
            "coverage_tags": [
              "handling_anti_bot_measures"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_user_agent_fail",
            "learning_goal": "学生能辨析直接请求维基百科失败的根本原因。",
            "linked_steps": [
              "step4"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "User-Agent",
                "en": "User-Agent"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "许多网站通过检查 User-Agent 来识别并阻止非浏览器请求。设置一个常见的浏览器 User-Agent 即可解决。",
                "options": [
                  "维基百科服务器暂时宕机",
                  "请求缺少正确的 User-Agent 头",
                  "Python 的 requests 库版本过低",
                  "网络连接速度太慢导致超时"
                ],
                "question_id": "q_quiz_user_agent_fail_v1",
                "stem": "在爬取维基百科 S&P 500 页面时，直接使用 `requests.get(url)` 返回了错误信息。最可能的原因是什么？"
              },
              {
                "answer": 1,
                "estimated_seconds": 10,
                "explanation": "User-Agent 字段告诉服务器请求来自什么客户端（如浏览器、爬虫）。",
                "options": [
                  "Accept-Language",
                  "User-Agent",
                  "Content-Type",
                  "Cookie"
                ],
                "question_id": "q_quiz_user_agent_fail_v2",
                "stem": "以下哪个 HTTP 请求头字段用于标识客户端类型，常被网站用于反爬虫检测？"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_table_id",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "beautifulsoup_table_extraction",
            "coverage_tags": [
              "beautifulsoup_table_extraction"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_table_id",
            "learning_goal": "学生能选择正确的 BeautifulSoup 方法来定位具有特定 id 的表格。",
            "linked_steps": [
              "step4"
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
                "explanation": "`find('table', id='constituents')` 通过标签名和 id 属性精确定位到目标表格。",
                "options": [
                  "soup.find_all('table')",
                  "soup.find('table', id='constituents')",
                  "soup.select('table.constituents')",
                  "soup.find('table', class_='constituents')"
                ],
                "question_id": "q_quiz_table_id_v1",
                "stem": "维基百科 S&P 500 页面有多个表格，要提取 id 为 'constituents' 的表格，应该使用哪个 BeautifulSoup 方法？"
              },
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "`find` 返回第一个匹配的元素，`id` 是唯一的，所以 `find('table', id='data-table')` 是正确且高效的方式。",
                "options": [
                  "soup.find_all('table', id='data-table')",
                  "soup.find('table', id='data-table')",
                  "soup.find('table', class_='data-table')",
                  "soup.select('table#data-table')[0]"
                ],
                "question_id": "q_quiz_table_id_v2",
                "stem": "假设一个网页中有多个 `<table>`，但只有一个表格的 id 是 'data-table'。以下哪段代码能正确提取该表格？"
              }
            ]
          }
        },
        {
          "family_id": "qf_long_sp500_process",
          "kind": "longform_families",
          "summary": {
            "concept_key": "sp500_scraping_process",
            "coverage_tags": [
              "real_world_scraping_sp500",
              "beautifulsoup_table_extraction",
              "dataframe_creation_from_scraped_data"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_sp500_process",
            "learning_goal": "学生能完整描述爬取 S&P 500 成分股列表的步骤，包括处理反爬、解析 HTML、定位表格和创建 DataFrame。",
            "linked_steps": [
              "step4"
            ],
            "question_type": "mechanism_trace",
            "term_refs": [
              {
                "display": "User-Agent",
                "en": "User-Agent"
              },
              {
                "display": "BeautifulSoup",
                "en": "BeautifulSoup"
              },
              {
                "display": "DataFrame",
                "en": "DataFrame"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "处理反爬机制",
                  "获取页面内容",
                  "解析 HTML",
                  "定位目标表格",
                  "提取数据并创建 DataFrame"
                ],
                "question_id": "q_long_sp500_process_v1",
                "reference_answer": [
                  "1. 设置 User-Agent：在 headers 字典中添加 'user-agent': 'Mozilla/5.0 ...'，避免被维基百科的反爬机制拦截。",
                  "2. 发送请求：使用 requests.get(url, headers=headers) 获取页面 HTML 内容。",
                  "3. 解析 HTML：用 BeautifulSoup(text, 'html.parser') 创建 soup 对象，将原始 HTML 转换为可操作的树结构。",
                  "4. 定位表格：使用 soup.find('table', id='constituents') 找到 id 为 'constituents' 的目标表格。",
                  "5. 提取数据：遍历表头 <th> 获取列名，遍历数据行 <tr> 获取每行数据。",
                  "6. 创建 DataFrame：用 pd.DataFrame(rows, columns=headers) 将数据组织成表格形式，便于后续分析。"
                ],
                "rubric_points": [
                  "指出需要设置 User-Agent 来伪装成浏览器",
                  "使用 requests.get(url, headers=headers) 获取页面",
                  "用 BeautifulSoup 解析 HTML 文本",
                  "通过 soup.find('table', id='constituents') 定位表格",
                  "提取表头和数据行，用 pd.DataFrame(rows, columns=headers) 创建 DataFrame"
                ],
                "stem": "请描述使用 Python 从维基百科爬取 S&P 500 成分股列表的完整步骤，并解释每一步的目的。"
              },
              {
                "estimated_seconds": 150,
                "prompt_blocks": [
                  "导入必要的库",
                  "设置请求头",
                  "发送请求并获取响应",
                  "解析 HTML",
                  "定位并提取表格",
                  "创建 DataFrame"
                ],
                "question_id": "q_long_sp500_process_v2",
                "reference_answer": [
                  "```python",
                  "import requests",
                  "from bs4 import BeautifulSoup",
                  "import pandas as pd",
                  "",
                  "# 1. 设置请求头，伪装成浏览器",
                  "headers = {",
                  "    'user-agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36'",
                  "}",
                  "",
                  "# 2. 发送 GET 请求",
                  "url = 'https://en.wikipedia.org/wiki/List_of_S%26P_500_companies'",
                  "response = requests.get(url, headers=headers)",
                  "",
                  "# 3. 解析 HTML",
                  "soup = BeautifulSoup(response.text, 'html.parser')",
                  "",
                  "# 4. 定位目标表格",
                  "table = soup.find('table', id='constituents')",
                  "",
                  "# 5. 提取表头",
                  "headers_list = [th.text.strip() for th in table.find_all('th')]",
                  "",
                  "# 6. 提取数据行",
                  "rows = []",
                  "for tr in table.find_all('tr')[1:]:  # 跳过表头行",
                  "    cells = tr.find_all('td')",
                  "    row = [cell.text.strip() for cell in cells]",
                  "    rows.append(row)",
                  "",
                  "# 7. 创建 DataFrame",
                  "df = pd.DataFrame(rows, columns=headers_list)",
                  "print(df)",
                  "```",
                  "",
                  "关键部分解释：",
                  "- headers 字典中的 User-Agent 用于绕过反爬虫检测。",
                  "- find('table', id='constituents') 通过 id 精确定位表格，避免提取到其他无关表格。",
                  "- 使用列表推导式提取表头和数据行，代码简洁高效。",
                  "- pd.DataFrame 将数据组织成结构化表格，便于后续分析。"
                ],
                "rubric_points": [
                  "导入 requests, BeautifulSoup, pandas",
                  "定义 headers 字典包含 User-Agent",
                  "使用 requests.get 并传入 headers",
                  "用 BeautifulSoup 解析 response.text",
                  "使用 find 方法按 id 定位表格",
                  "提取表头和数据行，创建 DataFrame"
                ],
                "stem": "假设你需要爬取一个类似维基百科 S&P 500 页面的表格数据，但该网站有反爬虫机制。请写出完整的 Python 代码框架，并解释关键部分的作用。"
              }
            ]
          }
        }
      ],
      "sequence_id": "step4",
      "step": {
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
      "step_path": "research/pipeline/3-guided_story/L2/step4/step.json"
    },
    {
      "question_bank": {
        "coverage_map": [
          {
            "coverage_tag": "yfinance_intro",
            "covered_by": [
              "qf_flash_yfinance_purpose",
              "qf_quiz_yfinance_advantage"
            ],
            "description": "yfinance 库的基本介绍：它是 Yahoo Finance 的非官方 Python 库，用于获取股票数据，封装了复杂的请求。"
          },
          {
            "coverage_tag": "yfinance_basic_usage",
            "covered_by": [
              "qf_flash_yfinance_history",
              "qf_flash_yfinance_download",
              "qf_quiz_yfinance_history_period"
            ],
            "description": "yfinance 的基本用法：.info() 获取基本信息，.history(period) 获取历史数据，yf.download() 获取多只股票数据。"
          },
          {
            "coverage_tag": "adjusted_close_concept",
            "covered_by": [
              "qf_flash_adjusted_close_purpose",
              "qf_flash_adjusted_close_example",
              "qf_quiz_adjusted_close_usage",
              "qf_long_adjusted_close_mechanism"
            ],
            "description": "调整收盘价的概念：消除分红、拆股等公司事件对历史价格的干扰，使价格具有可比性。"
          },
          {
            "coverage_tag": "close_vs_adjusted_close",
            "covered_by": [
              "qf_flash_close_vs_adjusted",
              "qf_quiz_adjusted_close_usage"
            ],
            "description": "收盘价与调整收盘价的区别：调整收盘价向后修正历史数据以反映公司事件的影响。"
          }
        ],
        "flashcard_families": [
          {
            "concept_key": "yfinance_intro",
            "coverage_tags": [
              "yfinance_intro"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_yfinance_purpose",
            "learning_goal": "学生能说出 yfinance 是什么以及为什么用它代替直接爬取 Yahoo Finance。",
            "linked_steps": [
              "step5"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "yfinance 库的核心定位和优势。",
            "term_refs": [
              {
                "display": "yfinance",
                "en": "yfinance"
              }
            ],
            "variants": [
              {
                "back": "yfinance 是一个非官方的 Yahoo Finance Python 库，用于获取股票数据。它封装了复杂的请求，避免了直接爬取 Yahoo Finance 时遇到的反爬措施。",
                "estimated_seconds": 10,
                "explanation": "直接爬取 Yahoo Finance 会遇到很多反爬措施，yfinance 封装了这些复杂性。",
                "front": "yfinance 是什么？它主要用来解决什么问题？",
                "question_id": "q_flash_yfinance_purpose_v1"
              },
              {
                "back": "因为 Yahoo Finance 有严格的反爬措施，yfinance 封装了所有复杂请求，几行代码就能获取数据。",
                "estimated_seconds": 8,
                "explanation": "yfinance 是专门为 Yahoo Finance 设计的非官方库。",
                "front": "为什么在算法交易中推荐使用 yfinance 而不是直接用 Requests 爬取 Yahoo Finance？",
                "question_id": "q_flash_yfinance_purpose_v2"
              }
            ]
          },
          {
            "concept_key": "yfinance_basic_usage",
            "coverage_tags": [
              "yfinance_basic_usage"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_yfinance_history",
            "learning_goal": "学生能回忆 yfinance 获取历史数据的方法和可用周期参数。",
            "linked_steps": [
              "step5"
            ],
            "question_type": "short_answer",
            "retrieval_focus": ".history(period) 方法的用途和常用 period 参数。",
            "term_refs": [
              {
                "display": "yfinance",
                "en": "yfinance"
              },
              {
                "display": ".history()",
                "en": ".history()"
              }
            ],
            "variants": [
              {
                "back": "ticker.history(period='1mo')",
                "estimated_seconds": 6,
                "explanation": ".history(period) 方法用于获取历史数据，'1mo' 表示最近一个月。",
                "front": "用 yfinance 获取腾讯控股 (0700.HK) 最近一个月的历史数据，应该调用哪个方法？",
                "question_id": "q_flash_yfinance_history_v1"
              },
              {
                "back": "'1d', '5d', '1mo', '3mo', '6mo', '1y', '2y', '5y', '10y', 'ytd', 'max'",
                "estimated_seconds": 10,
                "explanation": "period 参数支持从 1 天到最大历史范围的各种选项。",
                "front": "yfinance 的 .history() 方法中，period 参数可以设置为哪些值？请列举至少 4 个。",
                "question_id": "q_flash_yfinance_history_v2"
              }
            ]
          },
          {
            "concept_key": "yfinance_basic_usage",
            "coverage_tags": [
              "yfinance_basic_usage"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_yfinance_download",
            "learning_goal": "学生能回忆用 yfinance 同时获取多只股票数据的方法。",
            "linked_steps": [
              "step5"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "yf.download() 函数的用途和参数。",
            "term_refs": [
              {
                "display": "yfinance",
                "en": "yfinance"
              },
              {
                "display": "yf.download()",
                "en": "yf.download()"
              }
            ],
            "variants": [
              {
                "back": "yf.download(\"0700.HK AAPL\", start=\"2025-01-01\", end=\"2025-06-30\")",
                "estimated_seconds": 8,
                "explanation": "yf.download() 可以同时下载多只股票，tickers 用空格分隔。",
                "front": "用 yfinance 同时下载腾讯 (0700.HK) 和苹果 (AAPL) 从 2025-01-01 到 2025-06-30 的数据，应该使用哪个函数？",
                "question_id": "q_flash_yfinance_download_v1"
              },
              {
                "back": "用空格分隔，例如 \"0700.HK 0005.HK AAPL\"",
                "estimated_seconds": 5,
                "explanation": "yf.download() 接受一个字符串，多个 ticker 之间用空格隔开。",
                "front": "yf.download() 函数中，多个股票代码应该如何分隔？",
                "question_id": "q_flash_yfinance_download_v2"
              }
            ]
          },
          {
            "concept_key": "adjusted_close_concept",
            "coverage_tags": [
              "adjusted_close_concept"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_adjusted_close_purpose",
            "learning_goal": "学生能说出调整收盘价的目的和核心作用。",
            "linked_steps": [
              "step5"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "调整收盘价存在的根本原因。",
            "term_refs": [
              {
                "display": "调整收盘价",
                "en": "Adjusted Close"
              }
            ],
            "variants": [
              {
                "back": "消除分红、拆股等公司事件对历史价格的非市场因素干扰，使历史价格具有可比性。",
                "estimated_seconds": 8,
                "explanation": "公司事件会导致股价出现非市场因素的跳变，调整收盘价就是为了消除这些噪音。",
                "front": "调整收盘价的主要目的是什么？",
                "question_id": "q_flash_adjusted_close_purpose_v1"
              },
              {
                "back": "因为调整收盘价消除了分红和拆股的影响，能更准确地反映股票的真实长期表现。",
                "estimated_seconds": 8,
                "explanation": "普通收盘价会因公司事件出现非市场跳变，不适合长期分析。",
                "front": "为什么进行长期回测时应该使用调整收盘价而不是普通收盘价？",
                "question_id": "q_flash_adjusted_close_purpose_v2"
              }
            ]
          },
          {
            "concept_key": "adjusted_close_concept",
            "coverage_tags": [
              "adjusted_close_concept"
            ],
            "difficulty": "medium",
            "family_id": "qf_flash_adjusted_close_example",
            "learning_goal": "学生能通过具体例子理解拆股对调整收盘价的影响。",
            "linked_steps": [
              "step5"
            ],
            "question_type": "micro_calc",
            "retrieval_focus": "拆股场景下调整收盘价的计算逻辑。",
            "term_refs": [
              {
                "display": "调整收盘价",
                "en": "Adjusted Close"
              },
              {
                "display": "拆股",
                "en": "Stock Split"
              }
            ],
            "variants": [
              {
                "back": "50 元（100 / 2 = 50）",
                "estimated_seconds": 10,
                "explanation": "2:1 拆股意味着每 1 股变成 2 股，价格减半。调整收盘价会把拆股前的价格也除以 2，保持连续性。",
                "front": "一只股票在拆股前一天的收盘价是 100 元，第二天进行 2:1 拆股。拆股后，调整收盘价会把拆股前的价格调整为多少？",
                "question_id": "q_flash_adjusted_close_example_v1"
              },
              {
                "back": "50 元（200 / 4 = 50）",
                "estimated_seconds": 10,
                "explanation": "4:1 拆股，价格变为原来的 1/4。调整收盘价会把拆股前的价格也除以 4。",
                "front": "一只股票在拆股前一天的收盘价是 200 元，第二天进行 4:1 拆股。调整收盘价会把拆股前的价格调整为多少？",
                "question_id": "q_flash_adjusted_close_example_v2"
              }
            ]
          },
          {
            "concept_key": "close_vs_adjusted_close",
            "coverage_tags": [
              "close_vs_adjusted_close"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_close_vs_adjusted",
            "learning_goal": "学生能清晰区分收盘价和调整收盘价的核心差异。",
            "linked_steps": [
              "step5"
            ],
            "question_type": "core_difference",
            "retrieval_focus": "两者在是否受公司事件影响上的区别。",
            "term_refs": [
              {
                "display": "收盘价",
                "en": "Close"
              },
              {
                "display": "调整收盘价",
                "en": "Adjusted Close"
              }
            ],
            "variants": [
              {
                "back": "收盘价是当日实际交易的最后价格；调整收盘价是对历史收盘价进行向后修正，以消除分红、拆股等公司事件的影响。",
                "estimated_seconds": 10,
                "explanation": "调整收盘价让历史价格序列具有连续性，适合长期分析。",
                "front": "收盘价和调整收盘价的核心区别是什么？",
                "question_id": "q_flash_close_vs_adjusted_v1"
              },
              {
                "back": "收盘价通常会下跌（除息）。调整收盘价会向后修正分红前的价格，乘以 (1 - 分红/价格) 的因子。",
                "estimated_seconds": 12,
                "explanation": "分红后股价下跌是正常的除息现象，调整收盘价通过乘数修正来保持历史可比性。",
                "front": "当一只股票发放现金分红后，其收盘价通常会如何变化？调整收盘价如何处理这种变化？",
                "question_id": "q_flash_close_vs_adjusted_v2"
              }
            ]
          }
        ],
        "lesson_id": "L2",
        "longform_families": [
          {
            "concept_key": "adjusted_close_concept",
            "coverage_tags": [
              "adjusted_close_concept",
              "close_vs_adjusted_close"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_adjusted_close_mechanism",
            "learning_goal": "学生能解释调整收盘价的计算机制，并能通过具体数值例子说明拆股和分红如何影响调整。",
            "linked_steps": [
              "step5"
            ],
            "question_type": "mechanism_trace",
            "term_refs": [
              {
                "display": "调整收盘价",
                "en": "Adjusted Close"
              },
              {
                "display": "拆股",
                "en": "Stock Split"
              },
              {
                "display": "分红",
                "en": "Dividend"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "定义调整收盘价",
                  "说明它解决了什么问题",
                  "举例说明拆股场景下的调整逻辑",
                  "说明分红场景下的调整逻辑"
                ],
                "question_id": "q_long_adjusted_close_mechanism_v1",
                "reference_answer": [
                  "调整收盘价是对历史收盘价进行向后修正，以消除分红、拆股等公司事件对价格序列的影响。",
                  "当公司发生分红或拆股时，股价会出现非市场因素的跳变，导致历史价格失去可比性。调整收盘价就是为了消除这些噪音。",
                  "例如，一只股票 2:1 拆股，拆股前收盘价 100 元，拆股后价格变为 50 元。调整收盘价会把拆股前的价格也调整为 50 元，保持连续性。",
                  "对于分红，例如分红 0.08 元，除息前收盘价 24.96 元，调整因子为 (1 - 0.08/24.96) = 0.9968，分红前的价格会乘以这个因子。",
                  "在长期回测中，使用调整收盘价能更准确地反映股票的真实长期表现，避免公司事件造成的虚假收益或损失。"
                ],
                "rubric_points": [
                  "正确解释调整收盘价是经过公司事件修正后的收盘价",
                  "指出它消除了分红、拆股等非市场因素对价格序列的干扰",
                  "能正确说明 2:1 拆股时，拆股前的价格会除以 2",
                  "能正确说明分红时，分红前的价格会乘以 (1 - 分红/价格) 的因子"
                ],
                "stem": "请解释调整收盘价（Adjusted Close）的概念，并说明它为什么对长期回测很重要。"
              },
              {
                "estimated_seconds": 150,
                "prompt_blocks": [
                  "确定需要应用的调整因子",
                  "计算拆股因子",
                  "计算分红因子",
                  "对每个日期应用正确的因子组合",
                  "解释为什么不同日期的调整因子不同"
                ],
                "question_id": "q_long_adjusted_close_mechanism_v2",
                "reference_answer": [
                  "首先确定调整因子：拆股因子 = 0.5（2:1 拆股），分红因子 = 1 - 0.08/24.96 = 0.9968。",
                  "2/16 的调整收盘价：0.5 * 0.9968 * 46.99 = 23.42。因为 2/16 在拆股和除息之前，需要同时调整。",
                  "2/17 的调整收盘价：0.5 * 0.9968 * 48.30 = 24.07。同样在拆股和除息之前。",
                  "2/18 的调整收盘价：0.9968 * 24.96 = 24.88。2/18 是拆股日，拆股发生在收盘前，所以不需要拆股调整，但需要为未来的除息做准备。",
                  "2/21 的调整收盘价：24.53。2/21 是除息日，之后的价格已经是调整后的价格，不需要再调整。"
                ],
                "rubric_points": [
                  "正确计算拆股因子为 0.5",
                  "正确计算分红因子为 (1 - 0.08/24.96) = 0.9968",
                  "2/16 和 2/17 的调整收盘价需要同时应用拆股和分红因子：0.5 * 0.9968 * 原价",
                  "2/18 的调整收盘价只需要应用分红因子：0.9968 * 24.96",
                  "2/21 的调整收盘价等于当日收盘价 24.53，因为它是除息日之后，不需要调整"
                ],
                "stem": "假设某股票在 2/16 的收盘价为 46.99 元，2/17 收盘价为 48.30 元。2/18 发生 2:1 拆股，当日收盘价为 24.96 元。2/21 为除息日，分红 0.08 元，当日收盘价为 24.53 元。请计算 2/16、2/17、2/18 和 2/21 的调整收盘价，并解释每一步的计算逻辑。"
              }
            ]
          }
        ],
        "quiz_families": [
          {
            "concept_key": "yfinance_intro",
            "coverage_tags": [
              "yfinance_intro"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_yfinance_advantage",
            "learning_goal": "学生能理解 yfinance 相对于直接爬取的优势。",
            "linked_steps": [
              "step5"
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
                "explanation": "直接爬取 Yahoo Finance 会遇到很多反爬措施，yfinance 封装了所有复杂请求，让用户几行代码就能获取数据。",
                "options": [
                  "yfinance 可以免费获取实时数据",
                  "yfinance 封装了复杂的请求，避免了反爬措施",
                  "yfinance 是 Yahoo 官方提供的库",
                  "yfinance 不需要安装任何依赖"
                ],
                "question_id": "q_quiz_yfinance_advantage_v1",
                "stem": "以下哪个是使用 yfinance 而不是直接爬取 Yahoo Finance 的主要原因？"
              },
              {
                "answer": 2,
                "estimated_seconds": 12,
                "explanation": "yfinance 是一个非官方的 Yahoo Finance Python 库，用于获取股票数据。",
                "options": [
                  "yfinance 是 Yahoo 官方维护的 Python 库",
                  "yfinance 只能获取美股数据",
                  "yfinance 是一个非官方的 Yahoo Finance Python 库",
                  "yfinance 需要付费才能使用"
                ],
                "question_id": "q_quiz_yfinance_advantage_v2",
                "stem": "关于 yfinance，以下哪个说法是正确的？"
              }
            ]
          },
          {
            "concept_key": "yfinance_basic_usage",
            "coverage_tags": [
              "yfinance_basic_usage"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_yfinance_history_period",
            "learning_goal": "学生能正确选择 yfinance 获取历史数据的方法和参数。",
            "linked_steps": [
              "step5"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "yfinance",
                "en": "yfinance"
              },
              {
                "display": ".history()",
                "en": ".history()"
              }
            ],
            "variants": [
              {
                "answer": 0,
                "estimated_seconds": 15,
                "explanation": ".history(period='5d') 用于获取最近 5 天的历史数据。",
                "options": [
                  "ticker.history(period='5d')",
                  "ticker.history(period='5')",
                  "ticker.info(period='5d')",
                  "yf.download(ticker, period='5d')"
                ],
                "question_id": "q_quiz_yfinance_history_period_v1",
                "stem": "使用 yfinance 获取某只股票最近 5 天的历史数据，应该使用哪个代码？"
              },
              {
                "answer": 1,
                "estimated_seconds": 12,
                "explanation": ".history() 返回的数据包含 Open, High, Low, Close, Volume 等字段。",
                "options": [
                  "仅包含收盘价",
                  "开盘价、最高价、最低价、收盘价和成交量",
                  "仅包含调整收盘价",
                  "开盘价、收盘价和市盈率"
                ],
                "question_id": "q_quiz_yfinance_history_period_v2",
                "stem": "yfinance 的 .history() 方法返回的数据包含以下哪些字段？"
              }
            ]
          },
          {
            "concept_key": "close_vs_adjusted_close",
            "coverage_tags": [
              "close_vs_adjusted_close",
              "adjusted_close_concept"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_adjusted_close_usage",
            "learning_goal": "学生能判断在什么场景下使用调整收盘价。",
            "linked_steps": [
              "step5"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "调整收盘价",
                "en": "Adjusted Close"
              },
              {
                "display": "收盘价",
                "en": "Close"
              }
            ],
            "variants": [
              {
                "answer": 2,
                "estimated_seconds": 12,
                "explanation": "调整收盘价消除了分红和拆股的影响，更适合长期分析和回测。",
                "options": [
                  "开盘价",
                  "收盘价",
                  "调整收盘价",
                  "最高价"
                ],
                "question_id": "q_quiz_adjusted_close_usage_v1",
                "stem": "进行长期回测时，应该使用哪个价格？"
              },
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "调整收盘价消除了公司事件的影响，适合用于长期回报率的计算和比较。",
                "options": [
                  "计算当日的日内波动率",
                  "比较一只股票过去 5 年的总回报率",
                  "判断今日的收盘价是否高于开盘价",
                  "计算当日的成交量"
                ],
                "question_id": "q_quiz_adjusted_close_usage_v2",
                "stem": "以下哪个场景最适合使用调整收盘价？"
              }
            ]
          }
        ],
        "source": {
          "coverage_checklist": "L2: Data scraping and database management with Python; Understand the concept of adjusted price",
          "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
          "lesson_map": "L2 step5: Using yfinance for reliable financial data access",
          "plain_text": "pipeline/1-plain/L2/plain.txt",
          "related": [
            "pipeline/2-related_important/course_desc.md"
          ],
          "source_outline": "L2: Data scraping and database management with Python; Understand the concept of adjusted price"
        },
        "target_language": "zh-CN"
      },
      "question_bank_path": "research/pipeline/3-guided_story/L2/step5/question_bank.json",
      "question_families": [
        {
          "family_id": "qf_flash_yfinance_purpose",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "yfinance_intro",
            "coverage_tags": [
              "yfinance_intro"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_yfinance_purpose",
            "learning_goal": "学生能说出 yfinance 是什么以及为什么用它代替直接爬取 Yahoo Finance。",
            "linked_steps": [
              "step5"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "yfinance 库的核心定位和优势。",
            "term_refs": [
              {
                "display": "yfinance",
                "en": "yfinance"
              }
            ],
            "variants": [
              {
                "back": "yfinance 是一个非官方的 Yahoo Finance Python 库，用于获取股票数据。它封装了复杂的请求，避免了直接爬取 Yahoo Finance 时遇到的反爬措施。",
                "estimated_seconds": 10,
                "explanation": "直接爬取 Yahoo Finance 会遇到很多反爬措施，yfinance 封装了这些复杂性。",
                "front": "yfinance 是什么？它主要用来解决什么问题？",
                "question_id": "q_flash_yfinance_purpose_v1"
              },
              {
                "back": "因为 Yahoo Finance 有严格的反爬措施，yfinance 封装了所有复杂请求，几行代码就能获取数据。",
                "estimated_seconds": 8,
                "explanation": "yfinance 是专门为 Yahoo Finance 设计的非官方库。",
                "front": "为什么在算法交易中推荐使用 yfinance 而不是直接用 Requests 爬取 Yahoo Finance？",
                "question_id": "q_flash_yfinance_purpose_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_yfinance_history",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "yfinance_basic_usage",
            "coverage_tags": [
              "yfinance_basic_usage"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_yfinance_history",
            "learning_goal": "学生能回忆 yfinance 获取历史数据的方法和可用周期参数。",
            "linked_steps": [
              "step5"
            ],
            "question_type": "short_answer",
            "retrieval_focus": ".history(period) 方法的用途和常用 period 参数。",
            "term_refs": [
              {
                "display": "yfinance",
                "en": "yfinance"
              },
              {
                "display": ".history()",
                "en": ".history()"
              }
            ],
            "variants": [
              {
                "back": "ticker.history(period='1mo')",
                "estimated_seconds": 6,
                "explanation": ".history(period) 方法用于获取历史数据，'1mo' 表示最近一个月。",
                "front": "用 yfinance 获取腾讯控股 (0700.HK) 最近一个月的历史数据，应该调用哪个方法？",
                "question_id": "q_flash_yfinance_history_v1"
              },
              {
                "back": "'1d', '5d', '1mo', '3mo', '6mo', '1y', '2y', '5y', '10y', 'ytd', 'max'",
                "estimated_seconds": 10,
                "explanation": "period 参数支持从 1 天到最大历史范围的各种选项。",
                "front": "yfinance 的 .history() 方法中，period 参数可以设置为哪些值？请列举至少 4 个。",
                "question_id": "q_flash_yfinance_history_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_yfinance_download",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "yfinance_basic_usage",
            "coverage_tags": [
              "yfinance_basic_usage"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_yfinance_download",
            "learning_goal": "学生能回忆用 yfinance 同时获取多只股票数据的方法。",
            "linked_steps": [
              "step5"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "yf.download() 函数的用途和参数。",
            "term_refs": [
              {
                "display": "yfinance",
                "en": "yfinance"
              },
              {
                "display": "yf.download()",
                "en": "yf.download()"
              }
            ],
            "variants": [
              {
                "back": "yf.download(\"0700.HK AAPL\", start=\"2025-01-01\", end=\"2025-06-30\")",
                "estimated_seconds": 8,
                "explanation": "yf.download() 可以同时下载多只股票，tickers 用空格分隔。",
                "front": "用 yfinance 同时下载腾讯 (0700.HK) 和苹果 (AAPL) 从 2025-01-01 到 2025-06-30 的数据，应该使用哪个函数？",
                "question_id": "q_flash_yfinance_download_v1"
              },
              {
                "back": "用空格分隔，例如 \"0700.HK 0005.HK AAPL\"",
                "estimated_seconds": 5,
                "explanation": "yf.download() 接受一个字符串，多个 ticker 之间用空格隔开。",
                "front": "yf.download() 函数中，多个股票代码应该如何分隔？",
                "question_id": "q_flash_yfinance_download_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_adjusted_close_purpose",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "adjusted_close_concept",
            "coverage_tags": [
              "adjusted_close_concept"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_adjusted_close_purpose",
            "learning_goal": "学生能说出调整收盘价的目的和核心作用。",
            "linked_steps": [
              "step5"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "调整收盘价存在的根本原因。",
            "term_refs": [
              {
                "display": "调整收盘价",
                "en": "Adjusted Close"
              }
            ],
            "variants": [
              {
                "back": "消除分红、拆股等公司事件对历史价格的非市场因素干扰，使历史价格具有可比性。",
                "estimated_seconds": 8,
                "explanation": "公司事件会导致股价出现非市场因素的跳变，调整收盘价就是为了消除这些噪音。",
                "front": "调整收盘价的主要目的是什么？",
                "question_id": "q_flash_adjusted_close_purpose_v1"
              },
              {
                "back": "因为调整收盘价消除了分红和拆股的影响，能更准确地反映股票的真实长期表现。",
                "estimated_seconds": 8,
                "explanation": "普通收盘价会因公司事件出现非市场跳变，不适合长期分析。",
                "front": "为什么进行长期回测时应该使用调整收盘价而不是普通收盘价？",
                "question_id": "q_flash_adjusted_close_purpose_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_adjusted_close_example",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "adjusted_close_concept",
            "coverage_tags": [
              "adjusted_close_concept"
            ],
            "difficulty": "medium",
            "family_id": "qf_flash_adjusted_close_example",
            "learning_goal": "学生能通过具体例子理解拆股对调整收盘价的影响。",
            "linked_steps": [
              "step5"
            ],
            "question_type": "micro_calc",
            "retrieval_focus": "拆股场景下调整收盘价的计算逻辑。",
            "term_refs": [
              {
                "display": "调整收盘价",
                "en": "Adjusted Close"
              },
              {
                "display": "拆股",
                "en": "Stock Split"
              }
            ],
            "variants": [
              {
                "back": "50 元（100 / 2 = 50）",
                "estimated_seconds": 10,
                "explanation": "2:1 拆股意味着每 1 股变成 2 股，价格减半。调整收盘价会把拆股前的价格也除以 2，保持连续性。",
                "front": "一只股票在拆股前一天的收盘价是 100 元，第二天进行 2:1 拆股。拆股后，调整收盘价会把拆股前的价格调整为多少？",
                "question_id": "q_flash_adjusted_close_example_v1"
              },
              {
                "back": "50 元（200 / 4 = 50）",
                "estimated_seconds": 10,
                "explanation": "4:1 拆股，价格变为原来的 1/4。调整收盘价会把拆股前的价格也除以 4。",
                "front": "一只股票在拆股前一天的收盘价是 200 元，第二天进行 4:1 拆股。调整收盘价会把拆股前的价格调整为多少？",
                "question_id": "q_flash_adjusted_close_example_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_close_vs_adjusted",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "close_vs_adjusted_close",
            "coverage_tags": [
              "close_vs_adjusted_close"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_close_vs_adjusted",
            "learning_goal": "学生能清晰区分收盘价和调整收盘价的核心差异。",
            "linked_steps": [
              "step5"
            ],
            "question_type": "core_difference",
            "retrieval_focus": "两者在是否受公司事件影响上的区别。",
            "term_refs": [
              {
                "display": "收盘价",
                "en": "Close"
              },
              {
                "display": "调整收盘价",
                "en": "Adjusted Close"
              }
            ],
            "variants": [
              {
                "back": "收盘价是当日实际交易的最后价格；调整收盘价是对历史收盘价进行向后修正，以消除分红、拆股等公司事件的影响。",
                "estimated_seconds": 10,
                "explanation": "调整收盘价让历史价格序列具有连续性，适合长期分析。",
                "front": "收盘价和调整收盘价的核心区别是什么？",
                "question_id": "q_flash_close_vs_adjusted_v1"
              },
              {
                "back": "收盘价通常会下跌（除息）。调整收盘价会向后修正分红前的价格，乘以 (1 - 分红/价格) 的因子。",
                "estimated_seconds": 12,
                "explanation": "分红后股价下跌是正常的除息现象，调整收盘价通过乘数修正来保持历史可比性。",
                "front": "当一只股票发放现金分红后，其收盘价通常会如何变化？调整收盘价如何处理这种变化？",
                "question_id": "q_flash_close_vs_adjusted_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_yfinance_advantage",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "yfinance_intro",
            "coverage_tags": [
              "yfinance_intro"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_yfinance_advantage",
            "learning_goal": "学生能理解 yfinance 相对于直接爬取的优势。",
            "linked_steps": [
              "step5"
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
                "explanation": "直接爬取 Yahoo Finance 会遇到很多反爬措施，yfinance 封装了所有复杂请求，让用户几行代码就能获取数据。",
                "options": [
                  "yfinance 可以免费获取实时数据",
                  "yfinance 封装了复杂的请求，避免了反爬措施",
                  "yfinance 是 Yahoo 官方提供的库",
                  "yfinance 不需要安装任何依赖"
                ],
                "question_id": "q_quiz_yfinance_advantage_v1",
                "stem": "以下哪个是使用 yfinance 而不是直接爬取 Yahoo Finance 的主要原因？"
              },
              {
                "answer": 2,
                "estimated_seconds": 12,
                "explanation": "yfinance 是一个非官方的 Yahoo Finance Python 库，用于获取股票数据。",
                "options": [
                  "yfinance 是 Yahoo 官方维护的 Python 库",
                  "yfinance 只能获取美股数据",
                  "yfinance 是一个非官方的 Yahoo Finance Python 库",
                  "yfinance 需要付费才能使用"
                ],
                "question_id": "q_quiz_yfinance_advantage_v2",
                "stem": "关于 yfinance，以下哪个说法是正确的？"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_yfinance_history_period",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "yfinance_basic_usage",
            "coverage_tags": [
              "yfinance_basic_usage"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_yfinance_history_period",
            "learning_goal": "学生能正确选择 yfinance 获取历史数据的方法和参数。",
            "linked_steps": [
              "step5"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "yfinance",
                "en": "yfinance"
              },
              {
                "display": ".history()",
                "en": ".history()"
              }
            ],
            "variants": [
              {
                "answer": 0,
                "estimated_seconds": 15,
                "explanation": ".history(period='5d') 用于获取最近 5 天的历史数据。",
                "options": [
                  "ticker.history(period='5d')",
                  "ticker.history(period='5')",
                  "ticker.info(period='5d')",
                  "yf.download(ticker, period='5d')"
                ],
                "question_id": "q_quiz_yfinance_history_period_v1",
                "stem": "使用 yfinance 获取某只股票最近 5 天的历史数据，应该使用哪个代码？"
              },
              {
                "answer": 1,
                "estimated_seconds": 12,
                "explanation": ".history() 返回的数据包含 Open, High, Low, Close, Volume 等字段。",
                "options": [
                  "仅包含收盘价",
                  "开盘价、最高价、最低价、收盘价和成交量",
                  "仅包含调整收盘价",
                  "开盘价、收盘价和市盈率"
                ],
                "question_id": "q_quiz_yfinance_history_period_v2",
                "stem": "yfinance 的 .history() 方法返回的数据包含以下哪些字段？"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_adjusted_close_usage",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "close_vs_adjusted_close",
            "coverage_tags": [
              "close_vs_adjusted_close",
              "adjusted_close_concept"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_adjusted_close_usage",
            "learning_goal": "学生能判断在什么场景下使用调整收盘价。",
            "linked_steps": [
              "step5"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "调整收盘价",
                "en": "Adjusted Close"
              },
              {
                "display": "收盘价",
                "en": "Close"
              }
            ],
            "variants": [
              {
                "answer": 2,
                "estimated_seconds": 12,
                "explanation": "调整收盘价消除了分红和拆股的影响，更适合长期分析和回测。",
                "options": [
                  "开盘价",
                  "收盘价",
                  "调整收盘价",
                  "最高价"
                ],
                "question_id": "q_quiz_adjusted_close_usage_v1",
                "stem": "进行长期回测时，应该使用哪个价格？"
              },
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "调整收盘价消除了公司事件的影响，适合用于长期回报率的计算和比较。",
                "options": [
                  "计算当日的日内波动率",
                  "比较一只股票过去 5 年的总回报率",
                  "判断今日的收盘价是否高于开盘价",
                  "计算当日的成交量"
                ],
                "question_id": "q_quiz_adjusted_close_usage_v2",
                "stem": "以下哪个场景最适合使用调整收盘价？"
              }
            ]
          }
        },
        {
          "family_id": "qf_long_adjusted_close_mechanism",
          "kind": "longform_families",
          "summary": {
            "concept_key": "adjusted_close_concept",
            "coverage_tags": [
              "adjusted_close_concept",
              "close_vs_adjusted_close"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_adjusted_close_mechanism",
            "learning_goal": "学生能解释调整收盘价的计算机制，并能通过具体数值例子说明拆股和分红如何影响调整。",
            "linked_steps": [
              "step5"
            ],
            "question_type": "mechanism_trace",
            "term_refs": [
              {
                "display": "调整收盘价",
                "en": "Adjusted Close"
              },
              {
                "display": "拆股",
                "en": "Stock Split"
              },
              {
                "display": "分红",
                "en": "Dividend"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "定义调整收盘价",
                  "说明它解决了什么问题",
                  "举例说明拆股场景下的调整逻辑",
                  "说明分红场景下的调整逻辑"
                ],
                "question_id": "q_long_adjusted_close_mechanism_v1",
                "reference_answer": [
                  "调整收盘价是对历史收盘价进行向后修正，以消除分红、拆股等公司事件对价格序列的影响。",
                  "当公司发生分红或拆股时，股价会出现非市场因素的跳变，导致历史价格失去可比性。调整收盘价就是为了消除这些噪音。",
                  "例如，一只股票 2:1 拆股，拆股前收盘价 100 元，拆股后价格变为 50 元。调整收盘价会把拆股前的价格也调整为 50 元，保持连续性。",
                  "对于分红，例如分红 0.08 元，除息前收盘价 24.96 元，调整因子为 (1 - 0.08/24.96) = 0.9968，分红前的价格会乘以这个因子。",
                  "在长期回测中，使用调整收盘价能更准确地反映股票的真实长期表现，避免公司事件造成的虚假收益或损失。"
                ],
                "rubric_points": [
                  "正确解释调整收盘价是经过公司事件修正后的收盘价",
                  "指出它消除了分红、拆股等非市场因素对价格序列的干扰",
                  "能正确说明 2:1 拆股时，拆股前的价格会除以 2",
                  "能正确说明分红时，分红前的价格会乘以 (1 - 分红/价格) 的因子"
                ],
                "stem": "请解释调整收盘价（Adjusted Close）的概念，并说明它为什么对长期回测很重要。"
              },
              {
                "estimated_seconds": 150,
                "prompt_blocks": [
                  "确定需要应用的调整因子",
                  "计算拆股因子",
                  "计算分红因子",
                  "对每个日期应用正确的因子组合",
                  "解释为什么不同日期的调整因子不同"
                ],
                "question_id": "q_long_adjusted_close_mechanism_v2",
                "reference_answer": [
                  "首先确定调整因子：拆股因子 = 0.5（2:1 拆股），分红因子 = 1 - 0.08/24.96 = 0.9968。",
                  "2/16 的调整收盘价：0.5 * 0.9968 * 46.99 = 23.42。因为 2/16 在拆股和除息之前，需要同时调整。",
                  "2/17 的调整收盘价：0.5 * 0.9968 * 48.30 = 24.07。同样在拆股和除息之前。",
                  "2/18 的调整收盘价：0.9968 * 24.96 = 24.88。2/18 是拆股日，拆股发生在收盘前，所以不需要拆股调整，但需要为未来的除息做准备。",
                  "2/21 的调整收盘价：24.53。2/21 是除息日，之后的价格已经是调整后的价格，不需要再调整。"
                ],
                "rubric_points": [
                  "正确计算拆股因子为 0.5",
                  "正确计算分红因子为 (1 - 0.08/24.96) = 0.9968",
                  "2/16 和 2/17 的调整收盘价需要同时应用拆股和分红因子：0.5 * 0.9968 * 原价",
                  "2/18 的调整收盘价只需要应用分红因子：0.9968 * 24.96",
                  "2/21 的调整收盘价等于当日收盘价 24.53，因为它是除息日之后，不需要调整"
                ],
                "stem": "假设某股票在 2/16 的收盘价为 46.99 元，2/17 收盘价为 48.30 元。2/18 发生 2:1 拆股，当日收盘价为 24.96 元。2/21 为除息日，分红 0.08 元，当日收盘价为 24.53 元。请计算 2/16、2/17、2/18 和 2/21 的调整收盘价，并解释每一步的计算逻辑。"
              }
            ]
          }
        }
      ],
      "sequence_id": "step5",
      "step": {
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
      "step_path": "research/pipeline/3-guided_story/L2/step5/step.json"
    },
    {
      "question_bank": {
        "coverage_map": [
          {
            "coverage_tag": "database_management_why",
            "covered_by": [
              "qf_flash_db_why",
              "qf_quiz_db_why"
            ],
            "description": "理解在算法交易中使用数据库存储爬取数据的原因：高效存储与检索、避免重复爬取、数据完整性、高级查询能力。"
          },
          {
            "coverage_tag": "sqlite_basics",
            "covered_by": [
              "qf_flash_sqlite_connect",
              "qf_quiz_sqlite_connect"
            ],
            "description": "掌握 Python 内置 SQLite 库的基本用法：连接数据库、创建游标、执行 SQL、关闭连接。"
          },
          {
            "coverage_tag": "sql_crud_syntax",
            "covered_by": [
              "qf_flash_sql_crud",
              "qf_quiz_sql_crud",
              "qf_long_sql_crud_apply"
            ],
            "description": "掌握 SQL 四种基本操作（CRUD）的语法：SELECT、INSERT、UPDATE、DELETE。"
          },
          {
            "coverage_tag": "create_table",
            "covered_by": [
              "qf_flash_create_table",
              "qf_quiz_create_table"
            ],
            "description": "能够使用 SQL 的 CREATE TABLE 语句定义表结构，包括字段名和数据类型。"
          },
          {
            "coverage_tag": "scrape_to_db",
            "covered_by": [
              "qf_long_scrape_to_db"
            ],
            "description": "能够将爬取到的金融数据（如 yfinance 获取的 K 线数据）写入 SQLite 数据库。"
          }
        ],
        "flashcard_families": [
          {
            "concept_key": "database_management_why",
            "coverage_tags": [
              "database_management_why"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_db_why",
            "learning_goal": "学生能说出在算法交易中使用数据库存储爬取数据的两条核心原因。",
            "linked_steps": [
              "step6"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "数据库相对于重复爬取的核心优势。",
            "term_refs": [
              {
                "display": "数据库",
                "en": "database"
              }
            ],
            "variants": [
              {
                "back": "高效存储与检索大量历史数据，避免重复爬取。",
                "estimated_seconds": 10,
                "explanation": "数据库提供了结构化的存储和高效的查询能力，使得策略回测和数据分析更加快速可靠。",
                "front": "在算法交易中，为什么要把爬取到的数据存入数据库，而不是每次都重新爬取？",
                "question_id": "q_flash_db_why_v1"
              },
              {
                "back": "数据完整性和高级查询能力。",
                "estimated_seconds": 10,
                "explanation": "数据库通过约束和事务保证数据一致性，同时支持复杂的 SQL 查询来筛选和分析数据。",
                "front": "除了高效存储，数据库在算法交易中还能提供哪两个额外好处？",
                "question_id": "q_flash_db_why_v2"
              }
            ]
          },
          {
            "concept_key": "sqlite_basics",
            "coverage_tags": [
              "sqlite_basics"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_sqlite_connect",
            "learning_goal": "学生能写出在 Python 中连接 SQLite 数据库并创建游标的标准代码片段。",
            "linked_steps": [
              "step6"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "Python 中操作 SQLite 的初始化步骤。",
            "term_refs": [
              {
                "display": "SQLite",
                "en": "SQLite"
              }
            ],
            "variants": [
              {
                "back": "import sqlite3",
                "estimated_seconds": 5,
                "explanation": "Python 内置了 sqlite3 模块，无需额外安装。",
                "front": "在 Python 中连接一个名为 'example.db' 的 SQLite 数据库，需要导入哪个模块？",
                "question_id": "q_flash_sqlite_connect_v1"
              },
              {
                "back": "游标对象 (cursor)。",
                "estimated_seconds": 5,
                "explanation": "通过 conn.cursor() 创建游标，然后使用 cursor.execute() 执行 SQL。",
                "front": "连接 SQLite 数据库后，需要创建什么对象来执行 SQL 语句？",
                "question_id": "q_flash_sqlite_connect_v2"
              }
            ]
          },
          {
            "concept_key": "sql_crud_syntax",
            "coverage_tags": [
              "sql_crud_syntax"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_sql_crud",
            "learning_goal": "学生能准确说出 SQL 四种基本操作对应的命令。",
            "linked_steps": [
              "step6"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "SQL CRUD 命令与操作的对应关系。",
            "term_refs": [
              {
                "display": "SQL",
                "en": "SQL"
              }
            ],
            "variants": [
              {
                "back": "INSERT",
                "estimated_seconds": 5,
                "explanation": "INSERT INTO table_name (columns) VALUES (values);",
                "front": "SQL 中用于向表中添加新数据的命令是什么？",
                "question_id": "q_flash_sql_crud_v1"
              },
              {
                "back": "SELECT",
                "estimated_seconds": 5,
                "explanation": "SELECT * FROM table_name WHERE condition;",
                "front": "SQL 中用于从表中检索数据的命令是什么？",
                "question_id": "q_flash_sql_crud_v2"
              },
              {
                "back": "UPDATE",
                "estimated_seconds": 5,
                "explanation": "UPDATE table_name SET column = value WHERE condition;",
                "front": "SQL 中用于修改表中已有数据的命令是什么？",
                "question_id": "q_flash_sql_crud_v3"
              },
              {
                "back": "DELETE",
                "estimated_seconds": 5,
                "explanation": "DELETE FROM table_name WHERE condition;",
                "front": "SQL 中用于从表中移除数据的命令是什么？",
                "question_id": "q_flash_sql_crud_v4"
              }
            ]
          },
          {
            "concept_key": "create_table",
            "coverage_tags": [
              "create_table"
            ],
            "difficulty": "medium",
            "family_id": "qf_flash_create_table",
            "learning_goal": "学生能写出创建包含指定字段的表的 SQL 语句。",
            "linked_steps": [
              "step6"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "CREATE TABLE 语句的基本结构。",
            "term_refs": [
              {
                "display": "CREATE TABLE",
                "en": "CREATE TABLE"
              }
            ],
            "variants": [
              {
                "back": "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL);",
                "estimated_seconds": 10,
                "explanation": "PRIMARY KEY 约束主键，NOT NULL 约束非空。",
                "front": "创建一个名为 'users' 的表，包含 'id' (整数主键) 和 'name' (文本非空) 两个字段，SQL 语句是什么？",
                "question_id": "q_flash_create_table_v1"
              },
              {
                "back": "CREATE TABLE market_candles (symbol text, close_price float);",
                "estimated_seconds": 10,
                "explanation": "字段名后跟数据类型，如 text 和 float。",
                "front": "创建一个名为 'market_candles' 的表，包含 'symbol' (文本) 和 'close_price' (浮点数) 两个字段，SQL 语句是什么？",
                "question_id": "q_flash_create_table_v2"
              }
            ]
          }
        ],
        "lesson_id": "L2",
        "longform_families": [
          {
            "concept_key": "sql_crud_syntax",
            "coverage_tags": [
              "sql_crud_syntax"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_sql_crud_apply",
            "learning_goal": "学生能结合具体场景，写出完整的 SQL 语句完成数据操作任务。",
            "linked_steps": [
              "step6"
            ],
            "question_type": "short_explain",
            "term_refs": [
              {
                "display": "SQL",
                "en": "SQL"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 90,
                "prompt_blocks": [
                  "INSERT 语句",
                  "SELECT 语句"
                ],
                "question_id": "q_long_sql_crud_apply_v1",
                "reference_answer": [
                  "INSERT INTO stock_prices (symbol, date, close) VALUES ('AAPL', '2024-01-15', 185.5);",
                  "SELECT * FROM stock_prices WHERE symbol='AAPL' AND date >= '2024-01-01' AND date < '2024-02-01';"
                ],
                "rubric_points": [
                  "INSERT 语句语法正确，字段和值对应",
                  "SELECT 语句包含 WHERE 条件筛选 symbol 和 date 范围"
                ],
                "stem": "假设你有一个名为 'stock_prices' 的表，包含 'symbol', 'date', 'close' 三个字段。请写出 SQL 语句来完成以下任务：\n1. 插入一条新记录：'AAPL', '2024-01-15', 185.5。\n2. 查询所有 'AAPL' 在 2024 年 1 月的数据。"
              },
              {
                "estimated_seconds": 90,
                "prompt_blocks": [
                  "UPDATE 语句",
                  "DELETE 语句"
                ],
                "question_id": "q_long_sql_crud_apply_v2",
                "reference_answer": [
                  "UPDATE users SET age=26 WHERE name='Bob';",
                  "DELETE FROM users WHERE age < 18;"
                ],
                "rubric_points": [
                  "UPDATE 语句包含 SET 和 WHERE 子句",
                  "DELETE 语句包含正确的 WHERE 条件"
                ],
                "stem": "假设你有一个名为 'users' 的表，包含 'id', 'name', 'age' 三个字段。请写出 SQL 语句来完成以下任务：\n1. 将名为 'Bob' 的用户的年龄更新为 26。\n2. 删除所有年龄小于 18 的用户。"
              }
            ]
          },
          {
            "concept_key": "scrape_to_db",
            "coverage_tags": [
              "scrape_to_db"
            ],
            "difficulty": "hard",
            "family_id": "qf_long_scrape_to_db",
            "learning_goal": "学生能描述将 yfinance 获取的股票数据存入 SQLite 数据库的完整流程。",
            "linked_steps": [
              "step6"
            ],
            "question_type": "mechanism_trace",
            "term_refs": [
              {
                "display": "yfinance",
                "en": "yfinance"
              },
              {
                "display": "SQLite",
                "en": "SQLite"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "数据库连接与建表",
                  "数据下载",
                  "数据插入",
                  "事务提交与关闭"
                ],
                "question_id": "q_long_scrape_to_db_v1",
                "reference_answer": [
                  "1. 导入 sqlite3 和 yfinance，使用 sqlite3.connect('stocks.db') 连接数据库，创建游标 cursor，执行 CREATE TABLE market_candles (symbol text, timestamp text, open float, high float, low float, close float, volume float)。",
                  "2. 使用 yf.Ticker('0700.HK').history(period='max') 下载历史数据，得到一个 DataFrame。",
                  "3. 使用 for index, row in data.iterrows() 遍历 DataFrame，在循环内构造 INSERT INTO market_candles VALUES (?, ?, ?, ?, ?, ?, ?) 语句，并用 cursor.execute() 执行。",
                  "4. 循环结束后调用 conn.commit() 提交事务，最后调用 conn.close() 关闭连接。"
                ],
                "rubric_points": [
                  "正确使用 sqlite3.connect 和 cursor.execute 创建表",
                  "正确使用 yf.Ticker 和 .history() 下载数据",
                  "正确遍历 DataFrame 并使用 INSERT 语句插入每一行",
                  "正确调用 conn.commit() 和 conn.close()"
                ],
                "stem": "请描述使用 Python 将 yfinance 获取的腾讯控股 (0700.HK) 历史 K 线数据存入 SQLite 数据库的完整步骤。需要包括：\n1. 创建数据库和表\n2. 下载数据\n3. 遍历数据并插入\n4. 提交并关闭连接"
              },
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "查询代码",
                  "效率解释"
                ],
                "question_id": "q_long_scrape_to_db_v2",
                "reference_answer": [
                  "代码：\nimport sqlite3\nconn = sqlite3.connect('stocks.db')\ncursor = conn.cursor()\ncursor.execute(\"SELECT timestamp, close FROM market_candles WHERE symbol='0700.HK' AND timestamp >= '2025-06-01' AND timestamp < '2025-07-01'\")\nrows = cursor.fetchall()\nfor row in rows: print(row)\nconn.close()",
                  "效率解释：数据库查询直接在本地磁盘上通过索引快速定位数据，无需网络请求，避免了反爬限制和网页加载时间。对于已经存储的数据，查询速度远快于重新爬取。"
                ],
                "rubric_points": [
                  "正确编写 SELECT 语句，包含 symbol 和日期范围条件",
                  "正确使用 cursor.execute() 和 fetchall() 获取结果",
                  "解释中提及数据库的索引、避免网络请求、本地磁盘读取等优势"
                ],
                "stem": "假设你已经将多只股票的数据存入了 SQLite 数据库的 'market_candles' 表。请写出 Python 代码来查询 '0700.HK' 在 2025 年 6 月的所有收盘价，并解释为什么使用数据库查询比重新爬取更高效。"
              }
            ]
          }
        ],
        "quiz_families": [
          {
            "concept_key": "database_management_why",
            "coverage_tags": [
              "database_management_why"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_db_why",
            "learning_goal": "学生能在具体场景中判断使用数据库存储爬取数据的优势。",
            "linked_steps": [
              "step6"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "数据库",
                "en": "database"
              }
            ],
            "variants": [
              {
                "answer": 2,
                "estimated_seconds": 20,
                "explanation": "数据库只负责存储和查询数据，不会自动生成交易信号。交易信号需要由策略代码生成。",
                "options": [
                  "可以高效地检索历史数据进行回测",
                  "避免每次分析都重新爬取网站，节省时间和带宽",
                  "数据库会自动生成交易信号",
                  "有助于保证数据的完整性和一致性"
                ],
                "question_id": "q_quiz_db_why_v1",
                "stem": "以下哪项不是将爬取到的股票历史数据存入数据库的好处？"
              },
              {
                "answer": 1,
                "estimated_seconds": 20,
                "explanation": "数据库的核心优势在于高效存储和检索，为回测和后续分析提供数据基础。",
                "options": [
                  "让数据可以被多个程序同时修改",
                  "实现高效存储和快速检索，支持策略回测",
                  "将数据自动上传到云端",
                  "替代交易策略的执行逻辑"
                ],
                "question_id": "q_quiz_db_why_v2",
                "stem": "在算法交易中，将爬取的数据持久化到本地数据库的主要目的是什么？"
              }
            ]
          },
          {
            "concept_key": "sqlite_basics",
            "coverage_tags": [
              "sqlite_basics"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_sqlite_connect",
            "learning_goal": "学生能识别 Python 中操作 SQLite 的正确代码顺序。",
            "linked_steps": [
              "step6"
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
                "answer": 1,
                "estimated_seconds": 20,
                "explanation": "必须先建立连接 (connect)，然后从连接创建游标 (cursor)，最后才能用游标执行 SQL。",
                "options": [
                  "cursor = conn.cursor() -> conn = sqlite3.connect('db.db') -> cursor.execute(...)",
                  "conn = sqlite3.connect('db.db') -> cursor = conn.cursor() -> cursor.execute(...)",
                  "cursor.execute(...) -> conn = sqlite3.connect('db.db') -> cursor = conn.cursor()",
                  "conn = sqlite3.connect('db.db') -> cursor.execute(...) -> cursor = conn.cursor()"
                ],
                "question_id": "q_quiz_sqlite_connect_v1",
                "stem": "在 Python 中操作 SQLite 数据库，以下哪个代码顺序是正确的？"
              },
              {
                "answer": 2,
                "estimated_seconds": 15,
                "explanation": "commit() 方法用于提交事务，将更改保存到数据库。close() 只是关闭连接。",
                "options": [
                  "conn.close()",
                  "cursor.close()",
                  "conn.commit()",
                  "cursor.commit()"
                ],
                "question_id": "q_quiz_sqlite_connect_v2",
                "stem": "执行完数据库操作后，应该调用哪个方法来保存更改？"
              }
            ]
          },
          {
            "concept_key": "sql_crud_syntax",
            "coverage_tags": [
              "sql_crud_syntax"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_sql_crud",
            "learning_goal": "学生能在给定场景中选择正确的 SQL 命令。",
            "linked_steps": [
              "step6"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "SQL",
                "en": "SQL"
              }
            ],
            "variants": [
              {
                "answer": 2,
                "estimated_seconds": 15,
                "explanation": "UPDATE 用于修改表中已有的数据。",
                "options": [
                  "SELECT",
                  "INSERT",
                  "UPDATE",
                  "DELETE"
                ],
                "question_id": "q_quiz_sql_crud_v1",
                "stem": "你发现数据库里 'users' 表中 'Alice' 的年龄记录有误，需要从 30 改为 31。应该使用哪个 SQL 命令？"
              },
              {
                "answer": 0,
                "estimated_seconds": 20,
                "explanation": "SELECT 用于从表中检索数据，配合 WHERE 子句可以筛选特定条件的数据。",
                "options": [
                  "SELECT close_price FROM market_candles WHERE symbol='0700.HK'",
                  "INSERT INTO market_candles (close_price) VALUES (500)",
                  "UPDATE market_candles SET close_price=500 WHERE symbol='0700.HK'",
                  "DELETE FROM market_candles WHERE symbol='0700.HK'"
                ],
                "question_id": "q_quiz_sql_crud_v2",
                "stem": "你需要从 'market_candles' 表中查询所有 '0700.HK' 的收盘价。应该使用哪个 SQL 命令？"
              }
            ]
          },
          {
            "concept_key": "create_table",
            "coverage_tags": [
              "create_table"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_create_table",
            "learning_goal": "学生能识别正确的 CREATE TABLE 语句。",
            "linked_steps": [
              "step6"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "CREATE TABLE",
                "en": "CREATE TABLE"
              }
            ],
            "variants": [
              {
                "answer": 3,
                "estimated_seconds": 20,
                "explanation": "正确的语法是 CREATE TABLE 表名 (字段名 数据类型, ...)。选项 3 缺少数据类型，选项 1 缺少 TABLE 关键字。",
                "options": [
                  "CREATE stocks (ticker text, price float)",
                  "CREATE TABLE stocks (ticker text, price float)",
                  "CREATE TABLE stocks (ticker, price)",
                  "CREATE TABLE stocks (ticker TEXT, price FLOAT)"
                ],
                "question_id": "q_quiz_create_table_v1",
                "stem": "以下哪个 SQL 语句可以正确创建一个名为 'stocks' 的表，包含 'ticker' (文本) 和 'price' (浮点数) 字段？"
              },
              {
                "answer": 2,
                "estimated_seconds": 15,
                "explanation": "NOT NULL 约束确保字段在插入数据时不能为空。",
                "options": [
                  "UNIQUE",
                  "PRIMARY KEY",
                  "NOT NULL",
                  "DEFAULT"
                ],
                "question_id": "q_quiz_create_table_v2",
                "stem": "在创建 'market_candles' 表时，如果希望 'timestamp' 字段不能为空，应该使用什么约束？"
              }
            ]
          }
        ],
        "source": {
          "coverage_checklist": "L2: Data scraping and database management with Python - Agenda: Create a simple database using python",
          "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
          "lesson_map": "L2 step6: Storing scraped data in a SQLite database",
          "plain_text": "pipeline/1-plain/L2/plain.txt",
          "related": [
            "pipeline/2-related_important/course_desc.md"
          ],
          "source_outline": "L2: Data scraping and database management with Python - Database Management: Why use a database for algo-trading? SQL Syntax Overview: SELECT, INSERT, UPDATE, DELETE. Using SQLite in Python. Basic Usage: import sqlite3; conn = sqlite3.connect('example.db'); cursor = conn.cursor(). Create a Table, Inserting Data, Querying Data, Updating Data, Deleting Data. Scrape Market Data into database."
        },
        "target_language": "zh-CN"
      },
      "question_bank_path": "research/pipeline/3-guided_story/L2/step6/question_bank.json",
      "question_families": [
        {
          "family_id": "qf_flash_db_why",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "database_management_why",
            "coverage_tags": [
              "database_management_why"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_db_why",
            "learning_goal": "学生能说出在算法交易中使用数据库存储爬取数据的两条核心原因。",
            "linked_steps": [
              "step6"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "数据库相对于重复爬取的核心优势。",
            "term_refs": [
              {
                "display": "数据库",
                "en": "database"
              }
            ],
            "variants": [
              {
                "back": "高效存储与检索大量历史数据，避免重复爬取。",
                "estimated_seconds": 10,
                "explanation": "数据库提供了结构化的存储和高效的查询能力，使得策略回测和数据分析更加快速可靠。",
                "front": "在算法交易中，为什么要把爬取到的数据存入数据库，而不是每次都重新爬取？",
                "question_id": "q_flash_db_why_v1"
              },
              {
                "back": "数据完整性和高级查询能力。",
                "estimated_seconds": 10,
                "explanation": "数据库通过约束和事务保证数据一致性，同时支持复杂的 SQL 查询来筛选和分析数据。",
                "front": "除了高效存储，数据库在算法交易中还能提供哪两个额外好处？",
                "question_id": "q_flash_db_why_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_sqlite_connect",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "sqlite_basics",
            "coverage_tags": [
              "sqlite_basics"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_sqlite_connect",
            "learning_goal": "学生能写出在 Python 中连接 SQLite 数据库并创建游标的标准代码片段。",
            "linked_steps": [
              "step6"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "Python 中操作 SQLite 的初始化步骤。",
            "term_refs": [
              {
                "display": "SQLite",
                "en": "SQLite"
              }
            ],
            "variants": [
              {
                "back": "import sqlite3",
                "estimated_seconds": 5,
                "explanation": "Python 内置了 sqlite3 模块，无需额外安装。",
                "front": "在 Python 中连接一个名为 'example.db' 的 SQLite 数据库，需要导入哪个模块？",
                "question_id": "q_flash_sqlite_connect_v1"
              },
              {
                "back": "游标对象 (cursor)。",
                "estimated_seconds": 5,
                "explanation": "通过 conn.cursor() 创建游标，然后使用 cursor.execute() 执行 SQL。",
                "front": "连接 SQLite 数据库后，需要创建什么对象来执行 SQL 语句？",
                "question_id": "q_flash_sqlite_connect_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_sql_crud",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "sql_crud_syntax",
            "coverage_tags": [
              "sql_crud_syntax"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_sql_crud",
            "learning_goal": "学生能准确说出 SQL 四种基本操作对应的命令。",
            "linked_steps": [
              "step6"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "SQL CRUD 命令与操作的对应关系。",
            "term_refs": [
              {
                "display": "SQL",
                "en": "SQL"
              }
            ],
            "variants": [
              {
                "back": "INSERT",
                "estimated_seconds": 5,
                "explanation": "INSERT INTO table_name (columns) VALUES (values);",
                "front": "SQL 中用于向表中添加新数据的命令是什么？",
                "question_id": "q_flash_sql_crud_v1"
              },
              {
                "back": "SELECT",
                "estimated_seconds": 5,
                "explanation": "SELECT * FROM table_name WHERE condition;",
                "front": "SQL 中用于从表中检索数据的命令是什么？",
                "question_id": "q_flash_sql_crud_v2"
              },
              {
                "back": "UPDATE",
                "estimated_seconds": 5,
                "explanation": "UPDATE table_name SET column = value WHERE condition;",
                "front": "SQL 中用于修改表中已有数据的命令是什么？",
                "question_id": "q_flash_sql_crud_v3"
              },
              {
                "back": "DELETE",
                "estimated_seconds": 5,
                "explanation": "DELETE FROM table_name WHERE condition;",
                "front": "SQL 中用于从表中移除数据的命令是什么？",
                "question_id": "q_flash_sql_crud_v4"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_create_table",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "create_table",
            "coverage_tags": [
              "create_table"
            ],
            "difficulty": "medium",
            "family_id": "qf_flash_create_table",
            "learning_goal": "学生能写出创建包含指定字段的表的 SQL 语句。",
            "linked_steps": [
              "step6"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "CREATE TABLE 语句的基本结构。",
            "term_refs": [
              {
                "display": "CREATE TABLE",
                "en": "CREATE TABLE"
              }
            ],
            "variants": [
              {
                "back": "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL);",
                "estimated_seconds": 10,
                "explanation": "PRIMARY KEY 约束主键，NOT NULL 约束非空。",
                "front": "创建一个名为 'users' 的表，包含 'id' (整数主键) 和 'name' (文本非空) 两个字段，SQL 语句是什么？",
                "question_id": "q_flash_create_table_v1"
              },
              {
                "back": "CREATE TABLE market_candles (symbol text, close_price float);",
                "estimated_seconds": 10,
                "explanation": "字段名后跟数据类型，如 text 和 float。",
                "front": "创建一个名为 'market_candles' 的表，包含 'symbol' (文本) 和 'close_price' (浮点数) 两个字段，SQL 语句是什么？",
                "question_id": "q_flash_create_table_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_db_why",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "database_management_why",
            "coverage_tags": [
              "database_management_why"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_db_why",
            "learning_goal": "学生能在具体场景中判断使用数据库存储爬取数据的优势。",
            "linked_steps": [
              "step6"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "数据库",
                "en": "database"
              }
            ],
            "variants": [
              {
                "answer": 2,
                "estimated_seconds": 20,
                "explanation": "数据库只负责存储和查询数据，不会自动生成交易信号。交易信号需要由策略代码生成。",
                "options": [
                  "可以高效地检索历史数据进行回测",
                  "避免每次分析都重新爬取网站，节省时间和带宽",
                  "数据库会自动生成交易信号",
                  "有助于保证数据的完整性和一致性"
                ],
                "question_id": "q_quiz_db_why_v1",
                "stem": "以下哪项不是将爬取到的股票历史数据存入数据库的好处？"
              },
              {
                "answer": 1,
                "estimated_seconds": 20,
                "explanation": "数据库的核心优势在于高效存储和检索，为回测和后续分析提供数据基础。",
                "options": [
                  "让数据可以被多个程序同时修改",
                  "实现高效存储和快速检索，支持策略回测",
                  "将数据自动上传到云端",
                  "替代交易策略的执行逻辑"
                ],
                "question_id": "q_quiz_db_why_v2",
                "stem": "在算法交易中，将爬取的数据持久化到本地数据库的主要目的是什么？"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_sqlite_connect",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "sqlite_basics",
            "coverage_tags": [
              "sqlite_basics"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_sqlite_connect",
            "learning_goal": "学生能识别 Python 中操作 SQLite 的正确代码顺序。",
            "linked_steps": [
              "step6"
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
                "answer": 1,
                "estimated_seconds": 20,
                "explanation": "必须先建立连接 (connect)，然后从连接创建游标 (cursor)，最后才能用游标执行 SQL。",
                "options": [
                  "cursor = conn.cursor() -> conn = sqlite3.connect('db.db') -> cursor.execute(...)",
                  "conn = sqlite3.connect('db.db') -> cursor = conn.cursor() -> cursor.execute(...)",
                  "cursor.execute(...) -> conn = sqlite3.connect('db.db') -> cursor = conn.cursor()",
                  "conn = sqlite3.connect('db.db') -> cursor.execute(...) -> cursor = conn.cursor()"
                ],
                "question_id": "q_quiz_sqlite_connect_v1",
                "stem": "在 Python 中操作 SQLite 数据库，以下哪个代码顺序是正确的？"
              },
              {
                "answer": 2,
                "estimated_seconds": 15,
                "explanation": "commit() 方法用于提交事务，将更改保存到数据库。close() 只是关闭连接。",
                "options": [
                  "conn.close()",
                  "cursor.close()",
                  "conn.commit()",
                  "cursor.commit()"
                ],
                "question_id": "q_quiz_sqlite_connect_v2",
                "stem": "执行完数据库操作后，应该调用哪个方法来保存更改？"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_sql_crud",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "sql_crud_syntax",
            "coverage_tags": [
              "sql_crud_syntax"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_sql_crud",
            "learning_goal": "学生能在给定场景中选择正确的 SQL 命令。",
            "linked_steps": [
              "step6"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "SQL",
                "en": "SQL"
              }
            ],
            "variants": [
              {
                "answer": 2,
                "estimated_seconds": 15,
                "explanation": "UPDATE 用于修改表中已有的数据。",
                "options": [
                  "SELECT",
                  "INSERT",
                  "UPDATE",
                  "DELETE"
                ],
                "question_id": "q_quiz_sql_crud_v1",
                "stem": "你发现数据库里 'users' 表中 'Alice' 的年龄记录有误，需要从 30 改为 31。应该使用哪个 SQL 命令？"
              },
              {
                "answer": 0,
                "estimated_seconds": 20,
                "explanation": "SELECT 用于从表中检索数据，配合 WHERE 子句可以筛选特定条件的数据。",
                "options": [
                  "SELECT close_price FROM market_candles WHERE symbol='0700.HK'",
                  "INSERT INTO market_candles (close_price) VALUES (500)",
                  "UPDATE market_candles SET close_price=500 WHERE symbol='0700.HK'",
                  "DELETE FROM market_candles WHERE symbol='0700.HK'"
                ],
                "question_id": "q_quiz_sql_crud_v2",
                "stem": "你需要从 'market_candles' 表中查询所有 '0700.HK' 的收盘价。应该使用哪个 SQL 命令？"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_create_table",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "create_table",
            "coverage_tags": [
              "create_table"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_create_table",
            "learning_goal": "学生能识别正确的 CREATE TABLE 语句。",
            "linked_steps": [
              "step6"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "CREATE TABLE",
                "en": "CREATE TABLE"
              }
            ],
            "variants": [
              {
                "answer": 3,
                "estimated_seconds": 20,
                "explanation": "正确的语法是 CREATE TABLE 表名 (字段名 数据类型, ...)。选项 3 缺少数据类型，选项 1 缺少 TABLE 关键字。",
                "options": [
                  "CREATE stocks (ticker text, price float)",
                  "CREATE TABLE stocks (ticker text, price float)",
                  "CREATE TABLE stocks (ticker, price)",
                  "CREATE TABLE stocks (ticker TEXT, price FLOAT)"
                ],
                "question_id": "q_quiz_create_table_v1",
                "stem": "以下哪个 SQL 语句可以正确创建一个名为 'stocks' 的表，包含 'ticker' (文本) 和 'price' (浮点数) 字段？"
              },
              {
                "answer": 2,
                "estimated_seconds": 15,
                "explanation": "NOT NULL 约束确保字段在插入数据时不能为空。",
                "options": [
                  "UNIQUE",
                  "PRIMARY KEY",
                  "NOT NULL",
                  "DEFAULT"
                ],
                "question_id": "q_quiz_create_table_v2",
                "stem": "在创建 'market_candles' 表时，如果希望 'timestamp' 字段不能为空，应该使用什么约束？"
              }
            ]
          }
        },
        {
          "family_id": "qf_long_sql_crud_apply",
          "kind": "longform_families",
          "summary": {
            "concept_key": "sql_crud_syntax",
            "coverage_tags": [
              "sql_crud_syntax"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_sql_crud_apply",
            "learning_goal": "学生能结合具体场景，写出完整的 SQL 语句完成数据操作任务。",
            "linked_steps": [
              "step6"
            ],
            "question_type": "short_explain",
            "term_refs": [
              {
                "display": "SQL",
                "en": "SQL"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 90,
                "prompt_blocks": [
                  "INSERT 语句",
                  "SELECT 语句"
                ],
                "question_id": "q_long_sql_crud_apply_v1",
                "reference_answer": [
                  "INSERT INTO stock_prices (symbol, date, close) VALUES ('AAPL', '2024-01-15', 185.5);",
                  "SELECT * FROM stock_prices WHERE symbol='AAPL' AND date >= '2024-01-01' AND date < '2024-02-01';"
                ],
                "rubric_points": [
                  "INSERT 语句语法正确，字段和值对应",
                  "SELECT 语句包含 WHERE 条件筛选 symbol 和 date 范围"
                ],
                "stem": "假设你有一个名为 'stock_prices' 的表，包含 'symbol', 'date', 'close' 三个字段。请写出 SQL 语句来完成以下任务：\n1. 插入一条新记录：'AAPL', '2024-01-15', 185.5。\n2. 查询所有 'AAPL' 在 2024 年 1 月的数据。"
              },
              {
                "estimated_seconds": 90,
                "prompt_blocks": [
                  "UPDATE 语句",
                  "DELETE 语句"
                ],
                "question_id": "q_long_sql_crud_apply_v2",
                "reference_answer": [
                  "UPDATE users SET age=26 WHERE name='Bob';",
                  "DELETE FROM users WHERE age < 18;"
                ],
                "rubric_points": [
                  "UPDATE 语句包含 SET 和 WHERE 子句",
                  "DELETE 语句包含正确的 WHERE 条件"
                ],
                "stem": "假设你有一个名为 'users' 的表，包含 'id', 'name', 'age' 三个字段。请写出 SQL 语句来完成以下任务：\n1. 将名为 'Bob' 的用户的年龄更新为 26。\n2. 删除所有年龄小于 18 的用户。"
              }
            ]
          }
        },
        {
          "family_id": "qf_long_scrape_to_db",
          "kind": "longform_families",
          "summary": {
            "concept_key": "scrape_to_db",
            "coverage_tags": [
              "scrape_to_db"
            ],
            "difficulty": "hard",
            "family_id": "qf_long_scrape_to_db",
            "learning_goal": "学生能描述将 yfinance 获取的股票数据存入 SQLite 数据库的完整流程。",
            "linked_steps": [
              "step6"
            ],
            "question_type": "mechanism_trace",
            "term_refs": [
              {
                "display": "yfinance",
                "en": "yfinance"
              },
              {
                "display": "SQLite",
                "en": "SQLite"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "数据库连接与建表",
                  "数据下载",
                  "数据插入",
                  "事务提交与关闭"
                ],
                "question_id": "q_long_scrape_to_db_v1",
                "reference_answer": [
                  "1. 导入 sqlite3 和 yfinance，使用 sqlite3.connect('stocks.db') 连接数据库，创建游标 cursor，执行 CREATE TABLE market_candles (symbol text, timestamp text, open float, high float, low float, close float, volume float)。",
                  "2. 使用 yf.Ticker('0700.HK').history(period='max') 下载历史数据，得到一个 DataFrame。",
                  "3. 使用 for index, row in data.iterrows() 遍历 DataFrame，在循环内构造 INSERT INTO market_candles VALUES (?, ?, ?, ?, ?, ?, ?) 语句，并用 cursor.execute() 执行。",
                  "4. 循环结束后调用 conn.commit() 提交事务，最后调用 conn.close() 关闭连接。"
                ],
                "rubric_points": [
                  "正确使用 sqlite3.connect 和 cursor.execute 创建表",
                  "正确使用 yf.Ticker 和 .history() 下载数据",
                  "正确遍历 DataFrame 并使用 INSERT 语句插入每一行",
                  "正确调用 conn.commit() 和 conn.close()"
                ],
                "stem": "请描述使用 Python 将 yfinance 获取的腾讯控股 (0700.HK) 历史 K 线数据存入 SQLite 数据库的完整步骤。需要包括：\n1. 创建数据库和表\n2. 下载数据\n3. 遍历数据并插入\n4. 提交并关闭连接"
              },
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "查询代码",
                  "效率解释"
                ],
                "question_id": "q_long_scrape_to_db_v2",
                "reference_answer": [
                  "代码：\nimport sqlite3\nconn = sqlite3.connect('stocks.db')\ncursor = conn.cursor()\ncursor.execute(\"SELECT timestamp, close FROM market_candles WHERE symbol='0700.HK' AND timestamp >= '2025-06-01' AND timestamp < '2025-07-01'\")\nrows = cursor.fetchall()\nfor row in rows: print(row)\nconn.close()",
                  "效率解释：数据库查询直接在本地磁盘上通过索引快速定位数据，无需网络请求，避免了反爬限制和网页加载时间。对于已经存储的数据，查询速度远快于重新爬取。"
                ],
                "rubric_points": [
                  "正确编写 SELECT 语句，包含 symbol 和日期范围条件",
                  "正确使用 cursor.execute() 和 fetchall() 获取结果",
                  "解释中提及数据库的索引、避免网络请求、本地磁盘读取等优势"
                ],
                "stem": "假设你已经将多只股票的数据存入了 SQLite 数据库的 'market_candles' 表。请写出 Python 代码来查询 '0700.HK' 在 2025 年 6 月的所有收盘价，并解释为什么使用数据库查询比重新爬取更高效。"
              }
            ]
          }
        }
      ],
      "sequence_id": "step6",
      "step": {
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
      "step_path": "research/pipeline/3-guided_story/L2/step6/step.json"
    },
    {
      "question_bank": {
        "coverage_map": [
          {
            "coverage_tag": "database_design_big_table_issue",
            "covered_by": [
              "qf_flash_big_table_issue",
              "qf_quiz_big_table_issue"
            ],
            "description": "将所有数据放入一张大表会导致表体积巨大、查询和备份缓慢的问题。"
          },
          {
            "coverage_tag": "database_design_partition_by_time",
            "covered_by": [
              "qf_flash_partition_by_time",
              "qf_quiz_partition_by_time"
            ],
            "description": "按时间（如月）分区可以缓解大表问题，但跨月分析困难。"
          },
          {
            "coverage_tag": "database_design_partition_by_stock",
            "covered_by": [
              "qf_flash_partition_by_stock",
              "qf_quiz_partition_by_stock"
            ],
            "description": "按股票分区便于回测，但跨资产对比分析困难。"
          },
          {
            "coverage_tag": "database_design_no_perfect_design",
            "covered_by": [
              "qf_flash_no_perfect_design",
              "qf_quiz_no_perfect_design",
              "qf_long_design_tradeoff"
            ],
            "description": "没有适用于所有情况的完美数据库设计，必须根据主要用途权衡。"
          },
          {
            "coverage_tag": "database_design_tradeoff_factors",
            "covered_by": [
              "qf_flash_tradeoff_factors",
              "qf_quiz_tradeoff_factors",
              "qf_long_design_tradeoff"
            ],
            "description": "数据库设计需要在存储空间、运行速度和内存消耗之间取得平衡。"
          }
        ],
        "flashcard_families": [
          {
            "concept_key": "database_design_big_table_issue",
            "coverage_tags": [
              "database_design_big_table_issue"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_big_table_issue",
            "learning_goal": "学生能识别将所有金融数据放入一张大表的主要缺点。",
            "linked_steps": [
              "step7"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "大表问题的两个核心后果：查询慢和备份慢。",
            "term_refs": [
              {
                "display": "大表问题",
                "en": "Big Table Problem"
              }
            ],
            "variants": [
              {
                "back": "查询速度变慢，数据备份变慢。",
                "estimated_seconds": 8,
                "explanation": "一张表包含所有数据会变得极其庞大，导致查询和备份操作都非常耗时。",
                "front": "将所有股票的所有历史数据放在同一个数据库表中，会导致哪两个主要问题？",
                "question_id": "q_flash_big_table_issue_v1"
              },
              {
                "back": "把所有数据塞进一张表，导致表体积巨大、性能下降。",
                "estimated_seconds": 6,
                "explanation": "这是设计金融数据库时的一个常见陷阱，会影响查询和备份效率。",
                "front": "在金融数据库设计中，“大表”问题指的是什么？",
                "question_id": "q_flash_big_table_issue_v2"
              }
            ]
          },
          {
            "concept_key": "database_design_partition_by_time",
            "coverage_tags": [
              "database_design_partition_by_time"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_partition_by_time",
            "learning_goal": "学生能说出按时间分区的一个主要缺点。",
            "linked_steps": [
              "step7"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "按时间分区的主要缺点：跨月分析困难。",
            "term_refs": [
              {
                "display": "按时间分区",
                "en": "Partition by Time"
              }
            ],
            "variants": [
              {
                "back": "难以进行跨月的数据分析。",
                "estimated_seconds": 8,
                "explanation": "当需要分析跨越多个月份的数据时，需要查询多个表并合并结果，增加了复杂性。",
                "front": "按月份将市场数据表分区（例如每个月一个表），这种设计的主要缺点是什么？",
                "question_id": "q_flash_partition_by_time_v1"
              },
              {
                "back": "跨月的数据聚合和分析变得困难。",
                "estimated_seconds": 8,
                "explanation": "需要编写动态SQL来查询不同月份的表，增加了代码的复杂性。",
                "front": "为了缓解大表问题，将数据按时间分区（如每月一表），这会带来什么新的困难？",
                "question_id": "q_flash_partition_by_time_v2"
              }
            ]
          },
          {
            "concept_key": "database_design_partition_by_stock",
            "coverage_tags": [
              "database_design_partition_by_stock"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_partition_by_stock",
            "learning_goal": "学生能说出按股票分区的一个主要缺点。",
            "linked_steps": [
              "step7"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "按股票分区的主要缺点：跨资产对比分析困难。",
            "term_refs": [
              {
                "display": "按股票分区",
                "en": "Partition by Stock"
              }
            ],
            "variants": [
              {
                "back": "不利于跨资产的对比分析。",
                "estimated_seconds": 8,
                "explanation": "当需要比较不同股票在同一时间段的表现时，需要查询多个表，操作繁琐。",
                "front": "按股票代码将市场数据表分区（例如每只股票一个表），这种设计的主要缺点是什么？",
                "question_id": "q_flash_partition_by_stock_v1"
              },
              {
                "back": "跨资产（跨股票）的对比分析。",
                "estimated_seconds": 6,
                "explanation": "例如，比较苹果和微软的股价走势，需要分别查询两个表。",
                "front": "如果数据库按每只股票一个表来设计，进行什么类型的分析会比较困难？",
                "question_id": "q_flash_partition_by_stock_v2"
              }
            ]
          },
          {
            "concept_key": "database_design_no_perfect_design",
            "coverage_tags": [
              "database_design_no_perfect_design"
            ],
            "difficulty": "medium",
            "family_id": "qf_flash_no_perfect_design",
            "learning_goal": "学生能理解数据库设计没有银弹，取决于使用场景。",
            "linked_steps": [
              "step7"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "数据库设计的核心原则：没有适用于所有情况的完美方案，取决于主要用途。",
            "term_refs": [
              {
                "display": "无完美设计",
                "en": "No Perfect Design"
              }
            ],
            "variants": [
              {
                "back": "不存在。因为最佳设计取决于主要用途（如回测或数据分析）。",
                "estimated_seconds": 10,
                "explanation": "不同的使用场景有不同的权衡，例如回测偏好按股票分区，数据分析偏好按时间分区。",
                "front": "设计金融数据库时，是否存在一种适用于所有情况的完美方案？为什么？",
                "question_id": "q_flash_no_perfect_design_v1"
              },
              {
                "back": "根据主要用途在存储空间、运行速度和内存消耗之间找到平衡。",
                "estimated_seconds": 8,
                "explanation": "没有放之四海而皆准的设计，必须根据实际需求进行权衡。",
                "front": "数据库设计的核心艺术在于什么？",
                "question_id": "q_flash_no_perfect_design_v2"
              }
            ]
          },
          {
            "concept_key": "database_design_tradeoff_factors",
            "coverage_tags": [
              "database_design_tradeoff_factors"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_tradeoff_factors",
            "learning_goal": "学生能列举出数据库设计时需要权衡的三个核心因素。",
            "linked_steps": [
              "step7"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "三个核心权衡因素：存储空间、运行速度、内存消耗。",
            "term_refs": [
              {
                "display": "权衡因素",
                "en": "Trade-off Factors"
              }
            ],
            "variants": [
              {
                "back": "存储空间、运行速度、内存消耗。",
                "estimated_seconds": 6,
                "explanation": "这是数据库设计的核心艺术，三者往往不可兼得。",
                "front": "设计金融数据库时，始终需要在哪三个因素之间取得平衡？",
                "question_id": "q_flash_tradeoff_factors_v1"
              },
              {
                "back": "内存消耗。",
                "estimated_seconds": 6,
                "explanation": "查询和处理数据时，内存的使用效率也是一个关键考量。",
                "front": "在数据库设计中，除了存储空间和运行速度，第三个需要权衡的重要因素是什么？",
                "question_id": "q_flash_tradeoff_factors_v2"
              }
            ]
          }
        ],
        "lesson_id": "L2",
        "longform_families": [
          {
            "concept_key": "database_design_tradeoff",
            "coverage_tags": [
              "database_design_no_perfect_design",
              "database_design_tradeoff_factors"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_design_tradeoff",
            "learning_goal": "学生能解释不同数据库设计方案的优缺点，并能根据使用场景提出合理的权衡建议。",
            "linked_steps": [
              "step7"
            ],
            "question_type": "compare_and_contrast",
            "term_refs": [
              {
                "display": "数据库设计权衡",
                "en": "Database Design Trade-off"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "请比较“按股票分区”和“按时间分区”两种设计方案。",
                  "针对团队的两个主要工作（回测和跨资产分析），分别指出哪种方案更优，并解释原因。",
                  "最终，你会推荐哪种设计方案？请说明你的权衡理由。"
                ],
                "question_id": "q_long_design_tradeoff_v1",
                "reference_answer": [
                  "**按股票分区**：为每只股票创建一个表。优点是回测单只股票时，查询速度极快，因为所有数据都在一个表里。缺点是进行跨资产分析时，需要查询多个表，操作复杂且效率低。",
                  "**按时间分区**：按时间（如月或年）创建表。优点是跨资产分析方便，因为同一时间段的数据集中。缺点是回测单只股票时，需要跨多个表查询，效率较低。",
                  "**推荐**：考虑到团队的主要工作是回测（工作1），我会推荐**按股票分区**。虽然这会牺牲跨资产分析（工作2）的便利性，但可以通过编写一个辅助函数来遍历所有股票表，以应对偶尔的跨资产分析需求。这是一种基于主要用途的权衡，优先保证了最频繁操作的性能。"
                ],
                "rubric_points": [
                  "正确指出按股票分区有利于回测，因为单只股票的数据集中，查询快。",
                  "正确指出按时间分区有利于跨资产分析，因为同一时间段的数据集中。",
                  "能认识到没有完美方案，并根据团队主要工作（回测）给出推荐，同时承认跨资产分析的局限性。",
                  "能提及存储空间、运行速度等权衡因素。"
                ],
                "stem": "假设你需要为一个量化交易团队设计一个数据库，用于存储全球股票市场的日线数据。团队的主要工作是：1) 对单只股票进行历史回测；2) 偶尔进行跨资产的统计分析（如计算所有科技股的平均波动率）。"
              },
              {
                "estimated_seconds": 150,
                "prompt_blocks": [
                  "分析“将所有数据放入一张大表”的设计会面临什么问题。",
                  "提出一种你认为合理的分区策略（可以是按时间、按股票或混合策略），并解释它如何解决上述问题。",
                  "说明你的设计在存储空间、查询速度（针对两个使用场景）方面的权衡。"
                ],
                "question_id": "q_long_design_tradeoff_v2",
                "reference_answer": [
                  "**大表问题**：将所有Tick数据放入一张表，表会变得极其巨大，导致任何查询都非常缓慢，备份几乎不可能完成。",
                  "**分区策略**：采用**混合分区**策略。首先按日期分区（例如每天一个表或每月一个表），然后在每个日期分区内，再按股票代码进行哈希或列表分区。",
                  "**权衡分析**：",
                  "- **查询速度**：对于场景1（回测），查询可以快速定位到日期分区，再在分区内定位到具体股票，速度很快。对于场景2（盘后分析），可以只扫描当天的分区，避免了扫描全表。",
                  "- **存储空间**：分区会带来一些元数据开销，但可以接受。可以为高频查询的字段（如股票代码、时间戳）建立索引，这会占用额外空间。",
                  "- **内存消耗**：查询时只需将相关分区加载到内存，而不是整个大表，大大降低了内存压力。"
                ],
                "rubric_points": [
                  "明确指出大表会导致查询和备份极慢。",
                  "提出合理的分区策略，例如先按日期分区，再在每日分区内按股票分区（或反之）。",
                  "能解释该策略如何优化两个主要使用场景的查询速度。",
                  "能讨论存储空间（如索引开销）和内存消耗方面的权衡。"
                ],
                "stem": "你的任务是设计一个数据库来存储某交易所的逐笔交易数据（Tick Data）。数据量极大，每天新增数亿条记录。主要使用场景是：1) 高频策略的回测，需要快速查询某只股票在特定几分钟内的所有交易；2) 盘后分析，需要计算所有股票当日的总成交量。"
              }
            ]
          }
        ],
        "quiz_families": [
          {
            "concept_key": "database_design_big_table_issue",
            "coverage_tags": [
              "database_design_big_table_issue"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_big_table_issue",
            "learning_goal": "学生能在具体场景中识别大表设计带来的问题。",
            "linked_steps": [
              "step7"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "大表问题",
                "en": "Big Table Problem"
              }
            ],
            "variants": [
              {
                "answer": 2,
                "estimated_seconds": 20,
                "explanation": "将所有数据放在一张表里会导致表变得非常巨大，这是典型的“大表”问题，会严重影响查询速度。",
                "options": [
                  "数据库服务器CPU性能不足",
                  "网络带宽不够",
                  "单表数据量过大，导致查询效率下降",
                  "SQL查询语句写错了"
                ],
                "question_id": "q_quiz_big_table_issue_v1",
                "stem": "一个交易团队将所有全球股票的历史日线数据都存储在一个名为 `all_market_data` 的表中。随着数据量增长，他们发现查询特定股票最近一个月的数据变得越来越慢。最可能的原因是什么？"
              },
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "大表不仅查询慢，备份和恢复整个巨大的表也会非常耗时。",
                "options": [
                  "数据更容易丢失",
                  "数据备份和恢复操作会变得非常缓慢",
                  "无法使用SQL进行查询",
                  "不同股票的数据会互相覆盖"
                ],
                "question_id": "q_quiz_big_table_issue_v2",
                "stem": "以下哪一项是“将所有市场数据放入一张大表”这种设计最直接的后果？"
              }
            ]
          },
          {
            "concept_key": "database_design_partition_by_time",
            "coverage_tags": [
              "database_design_partition_by_time"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_partition_by_time",
            "learning_goal": "学生能判断按时间分区的适用场景和局限性。",
            "linked_steps": [
              "step7"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "按时间分区",
                "en": "Partition by Time"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 20,
                "explanation": "跨月分析需要查询并合并多个表，增加了SQL编写的复杂性和维护成本。",
                "options": [
                  "无法查询单只股票的数据",
                  "需要编写动态SQL来联合查询多个表，操作复杂",
                  "查询速度会非常快，没有挑战",
                  "无法计算平均值"
                ],
                "question_id": "q_quiz_partition_by_time_v1",
                "stem": "一个数据库设计为每个月创建一个市场数据表（如 `market_data_202401`, `market_data_202402`）。分析师需要计算某只股票过去六个月的平均收盘价。这个任务会面临什么挑战？"
              },
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "按时间分区的主要目的是将大表拆分成小表，从而缓解“大表”问题，提高单月数据的查询和备份效率。",
                "options": [
                  "方便跨资产对比分析",
                  "避免单表数据量过大",
                  "简化SQL查询语句",
                  "减少数据存储空间"
                ],
                "question_id": "q_quiz_partition_by_time_v2",
                "stem": "按时间分区（如每月一表）的设计，主要为了解决什么问题？"
              }
            ]
          },
          {
            "concept_key": "database_design_partition_by_stock",
            "coverage_tags": [
              "database_design_partition_by_stock"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_partition_by_stock",
            "learning_goal": "学生能判断按股票分区的适用场景和局限性。",
            "linked_steps": [
              "step7"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "按股票分区",
                "en": "Partition by Stock"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 20,
                "explanation": "跨资产对比分析需要查询所有相关的表，这在按股票分区的设计下非常困难。",
                "options": [
                  "无法获取单只股票的数据",
                  "需要编写程序遍历所有股票的表，操作繁琐且效率低",
                  "查询速度会非常快",
                  "无法计算收益率"
                ],
                "question_id": "q_quiz_partition_by_stock_v1",
                "stem": "一个数据库为每只股票创建一个单独的表（如 `AAPL_data`, `MSFT_data`）。一个量化研究员想要找出2023年所有科技股中，哪只股票的年度收益率最高。这个任务会面临什么挑战？"
              },
              {
                "answer": 1,
                "estimated_seconds": 20,
                "explanation": "按股票分区使得针对单只股票的数据查询和操作非常高效，非常适合回测。",
                "options": [
                  "分析整个市场的平均市盈率",
                  "对单只股票进行长时间序列的回测",
                  "比较不同行业板块的表现",
                  "生成包含所有股票每日收益率的报告"
                ],
                "question_id": "q_quiz_partition_by_stock_v2",
                "stem": "以下哪种使用场景最适合采用“按股票分区”的数据库设计？"
              }
            ]
          },
          {
            "concept_key": "database_design_no_perfect_design",
            "coverage_tags": [
              "database_design_no_perfect_design"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_no_perfect_design",
            "learning_goal": "学生能正确判断关于数据库设计通用性的陈述。",
            "linked_steps": [
              "step7"
            ],
            "question_type": "true_false",
            "term_refs": [
              {
                "display": "无完美设计",
                "en": "No Perfect Design"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 10,
                "explanation": "没有完美的设计。最佳方案取决于主要用途，例如回测或数据分析，需要在不同因素间权衡。",
                "options": [
                  "正确",
                  "错误"
                ],
                "question_id": "q_quiz_no_perfect_design_v1",
                "stem": "对于存储全球股票市场数据的数据库，存在一种最优的、适用于所有场景的表结构设计。"
              },
              {
                "answer": 0,
                "estimated_seconds": 10,
                "explanation": "这是数据库设计的核心思想。没有银弹，必须根据实际需求做出取舍。",
                "options": [
                  "正确",
                  "错误"
                ],
                "question_id": "q_quiz_no_perfect_design_v2",
                "stem": "数据库设计的关键在于根据常见的使用场景来权衡利弊。"
              }
            ]
          },
          {
            "concept_key": "database_design_tradeoff_factors",
            "coverage_tags": [
              "database_design_tradeoff_factors"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_tradeoff_factors",
            "learning_goal": "学生能识别出数据库设计中的核心权衡因素。",
            "linked_steps": [
              "step7"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "权衡因素",
                "en": "Trade-off Factors"
              }
            ],
            "variants": [
              {
                "answer": 2,
                "estimated_seconds": 15,
                "explanation": "代码行数不是数据库设计的核心权衡因素。核心权衡是在存储空间、运行速度和内存消耗之间。",
                "options": [
                  "存储空间",
                  "运行速度",
                  "代码行数",
                  "内存消耗"
                ],
                "question_id": "q_quiz_tradeoff_factors_v1",
                "stem": "设计金融数据库时，以下哪一项不是需要权衡的核心因素？"
              },
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "为了追求更快的查询速度，可能需要创建索引或冗余数据，这会占用更多的存储空间。",
                "options": [
                  "数据安全性",
                  "存储空间",
                  "查询灵活性",
                  "代码可读性"
                ],
                "question_id": "q_quiz_tradeoff_factors_v2",
                "stem": "在数据库设计中，如果优先考虑“运行速度”，通常可能需要在哪方面做出牺牲？"
              }
            ]
          }
        ],
        "source": {
          "coverage_checklist": "L2: Data scraping and database management with Python - Database design for financial market data storage",
          "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
          "lesson_map": "L2 step7: Database design considerations for financial data",
          "plain_text": "pipeline/1-plain/L2/plain.txt",
          "related": [
            "pipeline/2-related_important/course_desc.md"
          ],
          "source_outline": "L2: Data scraping and database management with Python - Database design for financial market data storage"
        },
        "target_language": "zh-CN"
      },
      "question_bank_path": "research/pipeline/3-guided_story/L2/step7/question_bank.json",
      "question_families": [
        {
          "family_id": "qf_flash_big_table_issue",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "database_design_big_table_issue",
            "coverage_tags": [
              "database_design_big_table_issue"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_big_table_issue",
            "learning_goal": "学生能识别将所有金融数据放入一张大表的主要缺点。",
            "linked_steps": [
              "step7"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "大表问题的两个核心后果：查询慢和备份慢。",
            "term_refs": [
              {
                "display": "大表问题",
                "en": "Big Table Problem"
              }
            ],
            "variants": [
              {
                "back": "查询速度变慢，数据备份变慢。",
                "estimated_seconds": 8,
                "explanation": "一张表包含所有数据会变得极其庞大，导致查询和备份操作都非常耗时。",
                "front": "将所有股票的所有历史数据放在同一个数据库表中，会导致哪两个主要问题？",
                "question_id": "q_flash_big_table_issue_v1"
              },
              {
                "back": "把所有数据塞进一张表，导致表体积巨大、性能下降。",
                "estimated_seconds": 6,
                "explanation": "这是设计金融数据库时的一个常见陷阱，会影响查询和备份效率。",
                "front": "在金融数据库设计中，“大表”问题指的是什么？",
                "question_id": "q_flash_big_table_issue_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_partition_by_time",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "database_design_partition_by_time",
            "coverage_tags": [
              "database_design_partition_by_time"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_partition_by_time",
            "learning_goal": "学生能说出按时间分区的一个主要缺点。",
            "linked_steps": [
              "step7"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "按时间分区的主要缺点：跨月分析困难。",
            "term_refs": [
              {
                "display": "按时间分区",
                "en": "Partition by Time"
              }
            ],
            "variants": [
              {
                "back": "难以进行跨月的数据分析。",
                "estimated_seconds": 8,
                "explanation": "当需要分析跨越多个月份的数据时，需要查询多个表并合并结果，增加了复杂性。",
                "front": "按月份将市场数据表分区（例如每个月一个表），这种设计的主要缺点是什么？",
                "question_id": "q_flash_partition_by_time_v1"
              },
              {
                "back": "跨月的数据聚合和分析变得困难。",
                "estimated_seconds": 8,
                "explanation": "需要编写动态SQL来查询不同月份的表，增加了代码的复杂性。",
                "front": "为了缓解大表问题，将数据按时间分区（如每月一表），这会带来什么新的困难？",
                "question_id": "q_flash_partition_by_time_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_partition_by_stock",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "database_design_partition_by_stock",
            "coverage_tags": [
              "database_design_partition_by_stock"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_partition_by_stock",
            "learning_goal": "学生能说出按股票分区的一个主要缺点。",
            "linked_steps": [
              "step7"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "按股票分区的主要缺点：跨资产对比分析困难。",
            "term_refs": [
              {
                "display": "按股票分区",
                "en": "Partition by Stock"
              }
            ],
            "variants": [
              {
                "back": "不利于跨资产的对比分析。",
                "estimated_seconds": 8,
                "explanation": "当需要比较不同股票在同一时间段的表现时，需要查询多个表，操作繁琐。",
                "front": "按股票代码将市场数据表分区（例如每只股票一个表），这种设计的主要缺点是什么？",
                "question_id": "q_flash_partition_by_stock_v1"
              },
              {
                "back": "跨资产（跨股票）的对比分析。",
                "estimated_seconds": 6,
                "explanation": "例如，比较苹果和微软的股价走势，需要分别查询两个表。",
                "front": "如果数据库按每只股票一个表来设计，进行什么类型的分析会比较困难？",
                "question_id": "q_flash_partition_by_stock_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_no_perfect_design",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "database_design_no_perfect_design",
            "coverage_tags": [
              "database_design_no_perfect_design"
            ],
            "difficulty": "medium",
            "family_id": "qf_flash_no_perfect_design",
            "learning_goal": "学生能理解数据库设计没有银弹，取决于使用场景。",
            "linked_steps": [
              "step7"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "数据库设计的核心原则：没有适用于所有情况的完美方案，取决于主要用途。",
            "term_refs": [
              {
                "display": "无完美设计",
                "en": "No Perfect Design"
              }
            ],
            "variants": [
              {
                "back": "不存在。因为最佳设计取决于主要用途（如回测或数据分析）。",
                "estimated_seconds": 10,
                "explanation": "不同的使用场景有不同的权衡，例如回测偏好按股票分区，数据分析偏好按时间分区。",
                "front": "设计金融数据库时，是否存在一种适用于所有情况的完美方案？为什么？",
                "question_id": "q_flash_no_perfect_design_v1"
              },
              {
                "back": "根据主要用途在存储空间、运行速度和内存消耗之间找到平衡。",
                "estimated_seconds": 8,
                "explanation": "没有放之四海而皆准的设计，必须根据实际需求进行权衡。",
                "front": "数据库设计的核心艺术在于什么？",
                "question_id": "q_flash_no_perfect_design_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_tradeoff_factors",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "database_design_tradeoff_factors",
            "coverage_tags": [
              "database_design_tradeoff_factors"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_tradeoff_factors",
            "learning_goal": "学生能列举出数据库设计时需要权衡的三个核心因素。",
            "linked_steps": [
              "step7"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "三个核心权衡因素：存储空间、运行速度、内存消耗。",
            "term_refs": [
              {
                "display": "权衡因素",
                "en": "Trade-off Factors"
              }
            ],
            "variants": [
              {
                "back": "存储空间、运行速度、内存消耗。",
                "estimated_seconds": 6,
                "explanation": "这是数据库设计的核心艺术，三者往往不可兼得。",
                "front": "设计金融数据库时，始终需要在哪三个因素之间取得平衡？",
                "question_id": "q_flash_tradeoff_factors_v1"
              },
              {
                "back": "内存消耗。",
                "estimated_seconds": 6,
                "explanation": "查询和处理数据时，内存的使用效率也是一个关键考量。",
                "front": "在数据库设计中，除了存储空间和运行速度，第三个需要权衡的重要因素是什么？",
                "question_id": "q_flash_tradeoff_factors_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_big_table_issue",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "database_design_big_table_issue",
            "coverage_tags": [
              "database_design_big_table_issue"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_big_table_issue",
            "learning_goal": "学生能在具体场景中识别大表设计带来的问题。",
            "linked_steps": [
              "step7"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "大表问题",
                "en": "Big Table Problem"
              }
            ],
            "variants": [
              {
                "answer": 2,
                "estimated_seconds": 20,
                "explanation": "将所有数据放在一张表里会导致表变得非常巨大，这是典型的“大表”问题，会严重影响查询速度。",
                "options": [
                  "数据库服务器CPU性能不足",
                  "网络带宽不够",
                  "单表数据量过大，导致查询效率下降",
                  "SQL查询语句写错了"
                ],
                "question_id": "q_quiz_big_table_issue_v1",
                "stem": "一个交易团队将所有全球股票的历史日线数据都存储在一个名为 `all_market_data` 的表中。随着数据量增长，他们发现查询特定股票最近一个月的数据变得越来越慢。最可能的原因是什么？"
              },
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "大表不仅查询慢，备份和恢复整个巨大的表也会非常耗时。",
                "options": [
                  "数据更容易丢失",
                  "数据备份和恢复操作会变得非常缓慢",
                  "无法使用SQL进行查询",
                  "不同股票的数据会互相覆盖"
                ],
                "question_id": "q_quiz_big_table_issue_v2",
                "stem": "以下哪一项是“将所有市场数据放入一张大表”这种设计最直接的后果？"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_partition_by_time",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "database_design_partition_by_time",
            "coverage_tags": [
              "database_design_partition_by_time"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_partition_by_time",
            "learning_goal": "学生能判断按时间分区的适用场景和局限性。",
            "linked_steps": [
              "step7"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "按时间分区",
                "en": "Partition by Time"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 20,
                "explanation": "跨月分析需要查询并合并多个表，增加了SQL编写的复杂性和维护成本。",
                "options": [
                  "无法查询单只股票的数据",
                  "需要编写动态SQL来联合查询多个表，操作复杂",
                  "查询速度会非常快，没有挑战",
                  "无法计算平均值"
                ],
                "question_id": "q_quiz_partition_by_time_v1",
                "stem": "一个数据库设计为每个月创建一个市场数据表（如 `market_data_202401`, `market_data_202402`）。分析师需要计算某只股票过去六个月的平均收盘价。这个任务会面临什么挑战？"
              },
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "按时间分区的主要目的是将大表拆分成小表，从而缓解“大表”问题，提高单月数据的查询和备份效率。",
                "options": [
                  "方便跨资产对比分析",
                  "避免单表数据量过大",
                  "简化SQL查询语句",
                  "减少数据存储空间"
                ],
                "question_id": "q_quiz_partition_by_time_v2",
                "stem": "按时间分区（如每月一表）的设计，主要为了解决什么问题？"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_partition_by_stock",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "database_design_partition_by_stock",
            "coverage_tags": [
              "database_design_partition_by_stock"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_partition_by_stock",
            "learning_goal": "学生能判断按股票分区的适用场景和局限性。",
            "linked_steps": [
              "step7"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "按股票分区",
                "en": "Partition by Stock"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 20,
                "explanation": "跨资产对比分析需要查询所有相关的表，这在按股票分区的设计下非常困难。",
                "options": [
                  "无法获取单只股票的数据",
                  "需要编写程序遍历所有股票的表，操作繁琐且效率低",
                  "查询速度会非常快",
                  "无法计算收益率"
                ],
                "question_id": "q_quiz_partition_by_stock_v1",
                "stem": "一个数据库为每只股票创建一个单独的表（如 `AAPL_data`, `MSFT_data`）。一个量化研究员想要找出2023年所有科技股中，哪只股票的年度收益率最高。这个任务会面临什么挑战？"
              },
              {
                "answer": 1,
                "estimated_seconds": 20,
                "explanation": "按股票分区使得针对单只股票的数据查询和操作非常高效，非常适合回测。",
                "options": [
                  "分析整个市场的平均市盈率",
                  "对单只股票进行长时间序列的回测",
                  "比较不同行业板块的表现",
                  "生成包含所有股票每日收益率的报告"
                ],
                "question_id": "q_quiz_partition_by_stock_v2",
                "stem": "以下哪种使用场景最适合采用“按股票分区”的数据库设计？"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_no_perfect_design",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "database_design_no_perfect_design",
            "coverage_tags": [
              "database_design_no_perfect_design"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_no_perfect_design",
            "learning_goal": "学生能正确判断关于数据库设计通用性的陈述。",
            "linked_steps": [
              "step7"
            ],
            "question_type": "true_false",
            "term_refs": [
              {
                "display": "无完美设计",
                "en": "No Perfect Design"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 10,
                "explanation": "没有完美的设计。最佳方案取决于主要用途，例如回测或数据分析，需要在不同因素间权衡。",
                "options": [
                  "正确",
                  "错误"
                ],
                "question_id": "q_quiz_no_perfect_design_v1",
                "stem": "对于存储全球股票市场数据的数据库，存在一种最优的、适用于所有场景的表结构设计。"
              },
              {
                "answer": 0,
                "estimated_seconds": 10,
                "explanation": "这是数据库设计的核心思想。没有银弹，必须根据实际需求做出取舍。",
                "options": [
                  "正确",
                  "错误"
                ],
                "question_id": "q_quiz_no_perfect_design_v2",
                "stem": "数据库设计的关键在于根据常见的使用场景来权衡利弊。"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_tradeoff_factors",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "database_design_tradeoff_factors",
            "coverage_tags": [
              "database_design_tradeoff_factors"
            ],
            "difficulty": "easy",
            "family_id": "qf_quiz_tradeoff_factors",
            "learning_goal": "学生能识别出数据库设计中的核心权衡因素。",
            "linked_steps": [
              "step7"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "权衡因素",
                "en": "Trade-off Factors"
              }
            ],
            "variants": [
              {
                "answer": 2,
                "estimated_seconds": 15,
                "explanation": "代码行数不是数据库设计的核心权衡因素。核心权衡是在存储空间、运行速度和内存消耗之间。",
                "options": [
                  "存储空间",
                  "运行速度",
                  "代码行数",
                  "内存消耗"
                ],
                "question_id": "q_quiz_tradeoff_factors_v1",
                "stem": "设计金融数据库时，以下哪一项不是需要权衡的核心因素？"
              },
              {
                "answer": 1,
                "estimated_seconds": 15,
                "explanation": "为了追求更快的查询速度，可能需要创建索引或冗余数据，这会占用更多的存储空间。",
                "options": [
                  "数据安全性",
                  "存储空间",
                  "查询灵活性",
                  "代码可读性"
                ],
                "question_id": "q_quiz_tradeoff_factors_v2",
                "stem": "在数据库设计中，如果优先考虑“运行速度”，通常可能需要在哪方面做出牺牲？"
              }
            ]
          }
        },
        {
          "family_id": "qf_long_design_tradeoff",
          "kind": "longform_families",
          "summary": {
            "concept_key": "database_design_tradeoff",
            "coverage_tags": [
              "database_design_no_perfect_design",
              "database_design_tradeoff_factors"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_design_tradeoff",
            "learning_goal": "学生能解释不同数据库设计方案的优缺点，并能根据使用场景提出合理的权衡建议。",
            "linked_steps": [
              "step7"
            ],
            "question_type": "compare_and_contrast",
            "term_refs": [
              {
                "display": "数据库设计权衡",
                "en": "Database Design Trade-off"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "请比较“按股票分区”和“按时间分区”两种设计方案。",
                  "针对团队的两个主要工作（回测和跨资产分析），分别指出哪种方案更优，并解释原因。",
                  "最终，你会推荐哪种设计方案？请说明你的权衡理由。"
                ],
                "question_id": "q_long_design_tradeoff_v1",
                "reference_answer": [
                  "**按股票分区**：为每只股票创建一个表。优点是回测单只股票时，查询速度极快，因为所有数据都在一个表里。缺点是进行跨资产分析时，需要查询多个表，操作复杂且效率低。",
                  "**按时间分区**：按时间（如月或年）创建表。优点是跨资产分析方便，因为同一时间段的数据集中。缺点是回测单只股票时，需要跨多个表查询，效率较低。",
                  "**推荐**：考虑到团队的主要工作是回测（工作1），我会推荐**按股票分区**。虽然这会牺牲跨资产分析（工作2）的便利性，但可以通过编写一个辅助函数来遍历所有股票表，以应对偶尔的跨资产分析需求。这是一种基于主要用途的权衡，优先保证了最频繁操作的性能。"
                ],
                "rubric_points": [
                  "正确指出按股票分区有利于回测，因为单只股票的数据集中，查询快。",
                  "正确指出按时间分区有利于跨资产分析，因为同一时间段的数据集中。",
                  "能认识到没有完美方案，并根据团队主要工作（回测）给出推荐，同时承认跨资产分析的局限性。",
                  "能提及存储空间、运行速度等权衡因素。"
                ],
                "stem": "假设你需要为一个量化交易团队设计一个数据库，用于存储全球股票市场的日线数据。团队的主要工作是：1) 对单只股票进行历史回测；2) 偶尔进行跨资产的统计分析（如计算所有科技股的平均波动率）。"
              },
              {
                "estimated_seconds": 150,
                "prompt_blocks": [
                  "分析“将所有数据放入一张大表”的设计会面临什么问题。",
                  "提出一种你认为合理的分区策略（可以是按时间、按股票或混合策略），并解释它如何解决上述问题。",
                  "说明你的设计在存储空间、查询速度（针对两个使用场景）方面的权衡。"
                ],
                "question_id": "q_long_design_tradeoff_v2",
                "reference_answer": [
                  "**大表问题**：将所有Tick数据放入一张表，表会变得极其巨大，导致任何查询都非常缓慢，备份几乎不可能完成。",
                  "**分区策略**：采用**混合分区**策略。首先按日期分区（例如每天一个表或每月一个表），然后在每个日期分区内，再按股票代码进行哈希或列表分区。",
                  "**权衡分析**：",
                  "- **查询速度**：对于场景1（回测），查询可以快速定位到日期分区，再在分区内定位到具体股票，速度很快。对于场景2（盘后分析），可以只扫描当天的分区，避免了扫描全表。",
                  "- **存储空间**：分区会带来一些元数据开销，但可以接受。可以为高频查询的字段（如股票代码、时间戳）建立索引，这会占用额外空间。",
                  "- **内存消耗**：查询时只需将相关分区加载到内存，而不是整个大表，大大降低了内存压力。"
                ],
                "rubric_points": [
                  "明确指出大表会导致查询和备份极慢。",
                  "提出合理的分区策略，例如先按日期分区，再在每日分区内按股票分区（或反之）。",
                  "能解释该策略如何优化两个主要使用场景的查询速度。",
                  "能讨论存储空间（如索引开销）和内存消耗方面的权衡。"
                ],
                "stem": "你的任务是设计一个数据库来存储某交易所的逐笔交易数据（Tick Data）。数据量极大，每天新增数亿条记录。主要使用场景是：1) 高频策略的回测，需要快速查询某只股票在特定几分钟内的所有交易；2) 盘后分析，需要计算所有股票当日的总成交量。"
              }
            ]
          }
        }
      ],
      "sequence_id": "step7",
      "step": {
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
      "step_path": "research/pipeline/3-guided_story/L2/step7/step.json"
    },
    {
      "question_bank": {
        "coverage_map": [
          {
            "coverage_tag": "limitations_data_quality",
            "covered_by": [
              "qf_flash_limitations_data_quality",
              "qf_quiz_limitations_data_quality"
            ],
            "description": "网络爬取的数据质量局限性：数据可能不完整或过时。"
          },
          {
            "coverage_tag": "limitations_resource_intensive",
            "covered_by": [
              "qf_flash_limitations_resource_intensive",
              "qf_quiz_limitations_resource_intensive"
            ],
            "description": "网络爬取的资源消耗局限性：消耗大量带宽、存储和时间。"
          },
          {
            "coverage_tag": "limitations_technical_challenges",
            "covered_by": [
              "qf_flash_limitations_technical_challenges",
              "qf_quiz_limitations_technical_challenges"
            ],
            "description": "网络爬取的技术挑战：网站结构变化、反爬机制（IP封锁、频率限制）。"
          },
          {
            "coverage_tag": "selenium_tool",
            "covered_by": [
              "qf_flash_selenium_tool",
              "qf_quiz_selenium_tool",
              "qf_long_selenium_use_case"
            ],
            "description": "Selenium：用于模拟用户交互和自动化浏览器的工具，适用于动态加载网页。"
          },
          {
            "coverage_tag": "xpath_lxml_tool",
            "covered_by": [
              "qf_flash_xpath_lxml_tool",
              "qf_quiz_xpath_lxml_tool",
              "qf_long_xpath_example"
            ],
            "description": "XPath 和 lxml：XPath 是一种在 XML/HTML 文档中导航和选择节点的查询语言，lxml 是支持 XPath 的 Python 库。"
          }
        ],
        "flashcard_families": [
          {
            "concept_key": "limitations_data_quality",
            "coverage_tags": [
              "limitations_data_quality"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_limitations_data_quality",
            "learning_goal": "学生能准确说出网络爬取在数据质量方面的主要局限性。",
            "linked_steps": [
              "step8"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "网络爬取在数据质量方面的具体问题。",
            "term_refs": [
              {
                "display": "数据质量",
                "en": "Data Quality"
              }
            ],
            "variants": [
              {
                "back": "数据可能不完整或过时。",
                "estimated_seconds": 8,
                "explanation": "爬取到的数据可能缺失部分内容，或者不是最新版本，影响分析准确性。",
                "front": "网络爬取在数据质量方面的一个主要局限性是什么？",
                "question_id": "q_flash_limitations_data_quality_v1"
              },
              {
                "back": "不完整和过时。",
                "estimated_seconds": 8,
                "explanation": "网站可能只展示了部分数据，或者数据更新不及时，导致爬取结果有缺陷。",
                "front": "从网站爬取的数据，在质量上可能面临哪两个具体问题？",
                "question_id": "q_flash_limitations_data_quality_v2"
              }
            ]
          },
          {
            "concept_key": "limitations_resource_intensive",
            "coverage_tags": [
              "limitations_resource_intensive"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_limitations_resource_intensive",
            "learning_goal": "学生能说出网络爬取在资源消耗方面的两个主要问题。",
            "linked_steps": [
              "step8"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "网络爬取消耗的具体资源。",
            "term_refs": [
              {
                "display": "资源密集型",
                "en": "Resource Intensive"
              }
            ],
            "variants": [
              {
                "back": "带宽和存储。",
                "estimated_seconds": 8,
                "explanation": "下载大量网页会占用网络带宽，存储爬取的数据也需要大量磁盘空间。",
                "front": "爬取大量数据时，会消耗哪两种主要资源？",
                "question_id": "q_flash_limitations_resource_intensive_v1"
              },
              {
                "back": "加载整个网页很耗时。",
                "estimated_seconds": 8,
                "explanation": "为了提取数据，需要完整加载网页内容，这个过程可能非常缓慢。",
                "front": "除了带宽和存储，爬取大量数据时还有什么时间上的消耗？",
                "question_id": "q_flash_limitations_resource_intensive_v2"
              }
            ]
          },
          {
            "concept_key": "limitations_technical_challenges",
            "coverage_tags": [
              "limitations_technical_challenges"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_limitations_technical_challenges",
            "learning_goal": "学生能列举网络爬取面临的两个主要技术挑战。",
            "linked_steps": [
              "step8"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "网络爬取的技术挑战的具体例子。",
            "term_refs": [
              {
                "display": "技术挑战",
                "en": "Technical Challenges"
              }
            ],
            "variants": [
              {
                "back": "网站结构经常变化。",
                "estimated_seconds": 8,
                "explanation": "如果网站改版，之前写的爬虫代码可能失效，需要重新适配。",
                "front": "网络爬取面临的一个常见技术挑战是什么？",
                "question_id": "q_flash_limitations_technical_challenges_v1"
              },
              {
                "back": "IP 封锁和频率限制。",
                "estimated_seconds": 8,
                "explanation": "IP封锁会禁止某个IP地址访问，频率限制则限制单位时间内的请求次数。",
                "front": "网站常用的两种反爬机制是什么？",
                "question_id": "q_flash_limitations_technical_challenges_v2"
              }
            ]
          },
          {
            "concept_key": "selenium_tool",
            "coverage_tags": [
              "selenium_tool"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_selenium_tool",
            "learning_goal": "学生能说出Selenium的核心功能和适用场景。",
            "linked_steps": [
              "step8"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "Selenium 是什么以及它能解决什么问题。",
            "term_refs": [
              {
                "display": "Selenium",
                "en": "Selenium"
              }
            ],
            "variants": [
              {
                "back": "一个用于模拟用户交互和自动化浏览器的工具。",
                "estimated_seconds": 8,
                "explanation": "它可以模拟点击、输入等操作，适用于处理动态加载的网页。",
                "front": "Selenium 是一个什么工具？",
                "question_id": "q_flash_selenium_tool_v1"
              },
              {
                "back": "Selenium。",
                "estimated_seconds": 8,
                "explanation": "Selenium 可以模拟真实的浏览器行为，从而获取由 JavaScript 动态生成的内容。",
                "front": "当 BeautifulSoup 无法处理动态加载的网页时，应该使用哪个工具？",
                "question_id": "q_flash_selenium_tool_v2"
              }
            ]
          },
          {
            "concept_key": "xpath_lxml_tool",
            "coverage_tags": [
              "xpath_lxml_tool"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_xpath_lxml_tool",
            "learning_goal": "学生能说出XPath是什么以及lxml库的作用。",
            "linked_steps": [
              "step8"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "XPath 和 lxml 的基本定义。",
            "term_refs": [
              {
                "display": "XPath",
                "en": "XPath"
              },
              {
                "display": "lxml",
                "en": "lxml"
              }
            ],
            "variants": [
              {
                "back": "一种在 XML/HTML 文档中导航和选择节点的查询语言。",
                "estimated_seconds": 8,
                "explanation": "它使用路径表达式来精确定位文档中的元素。",
                "front": "XPath 是一种什么语言？",
                "question_id": "q_flash_xpath_lxml_tool_v1"
              },
              {
                "back": "一个用于解析 XPath 和 XML 模式的 Python 库。",
                "estimated_seconds": 8,
                "explanation": "它支持使用 XPath 语法高效地提取 HTML 或 XML 数据。",
                "front": "lxml 是一个什么库？",
                "question_id": "q_flash_xpath_lxml_tool_v2"
              }
            ]
          }
        ],
        "lesson_id": "L2",
        "longform_families": [
          {
            "concept_key": "selenium_tool",
            "coverage_tags": [
              "selenium_tool"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_selenium_use_case",
            "learning_goal": "学生能解释在什么情况下需要使用Selenium，并说明其工作原理。",
            "linked_steps": [
              "step8"
            ],
            "question_type": "short_explain",
            "term_refs": [
              {
                "display": "Selenium",
                "en": "Selenium"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 90,
                "prompt_blocks": [
                  "解释动态加载网页的特点。",
                  "说明 BeautifulSoup 的工作原理及其局限性。",
                  "说明 Selenium 的工作原理及其如何克服这个局限性。"
                ],
                "question_id": "q_long_selenium_use_case_v1",
                "reference_answer": [
                  "动态加载的网页内容（如点击按钮后出现的数据）是由 JavaScript 在浏览器中执行后生成的，这些内容并不在初始的 HTML 源代码中。",
                  "BeautifulSoup 只能解析服务器返回的静态 HTML 源代码，因此无法获取这些动态生成的内容。",
                  "Selenium 通过控制一个真实的浏览器（如 Chrome）来加载网页，它会像真实用户一样执行 JavaScript，从而获取到完整的、动态渲染后的页面内容。"
                ],
                "rubric_points": [
                  "正确指出动态加载的内容由 JavaScript 生成。",
                  "正确指出 BeautifulSoup 只能解析静态 HTML 源代码。",
                  "正确指出 Selenium 可以模拟浏览器执行 JavaScript，从而获取动态内容。"
                ],
                "stem": "请解释为什么对于动态加载的网页，BeautifulSoup 可能无能为力，而 Selenium 可以解决这个问题。"
              },
              {
                "estimated_seconds": 90,
                "prompt_blocks": [
                  "分析使用 Requests 处理登录的复杂性。",
                  "分析使用 Selenium 处理登录的简便性。",
                  "总结 Selenium 在此场景下的优势。"
                ],
                "question_id": "q_long_selenium_use_case_v2",
                "reference_answer": [
                  "使用 Requests 模拟登录需要分析登录表单的结构，手动构造 POST 请求，并处理服务器返回的 Cookie 和 Session，过程繁琐且容易出错。",
                  "使用 Selenium，你可以编写代码自动在输入框中填写用户名和密码，然后点击登录按钮，就像真实用户一样。Selenium 会自动处理 Cookie 和 Session。",
                  "对于包含验证码或两步验证的复杂登录流程，Selenium 的优势更加明显，因为它可以模拟更复杂的用户交互。"
                ],
                "rubric_points": [
                  "指出使用 Requests 需要手动管理 Cookie、Session 和表单提交。",
                  "指出 Selenium 可以自动处理登录表单的填写和提交，并自动管理 Cookie。",
                  "指出 Selenium 更适合处理包含复杂交互（如验证码、两步验证）的登录流程。"
                ],
                "stem": "假设你需要从一个需要登录后才能查看数据的网站爬取信息。请解释为什么使用 Requests + BeautifulSoup 的组合会比较困难，而 Selenium 可以更简单地实现。"
              }
            ]
          },
          {
            "concept_key": "xpath_lxml_tool",
            "coverage_tags": [
              "xpath_lxml_tool"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_xpath_example",
            "learning_goal": "学生能解释一个具体XPath表达式的含义，并说明其相对于其他方法的优势。",
            "linked_steps": [
              "step8"
            ],
            "question_type": "short_explain",
            "term_refs": [
              {
                "display": "XPath",
                "en": "XPath"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 90,
                "prompt_blocks": [
                  "分解并解释 XPath 表达式的每个部分。",
                  "对比 XPath 和 BeautifulSoup 在定位元素时的不同方式。",
                  "说明 XPath 在哪些方面更灵活。"
                ],
                "question_id": "q_long_xpath_example_v1",
                "reference_answer": [
                  "`//table[@id='constituents']//tr` 的含义是：在整个文档中，找到 id 属性等于 'constituents' 的 `<table>` 元素，然后在该元素下找到所有 `<tr>` 元素。",
                  "BeautifulSoup 通常需要先找到父元素，再通过循环或另一个查找方法找到子元素，是一种链式调用。而 XPath 用一个表达式就能完成从根节点到目标节点的路径描述。",
                  "XPath 的灵活性体现在它支持丰富的函数和轴（如 `position()`、`following-sibling`），可以基于元素的位置、文本内容、属性值等进行非常精确的定位，这在处理结构复杂或相似的 HTML 时优势明显。"
                ],
                "rubric_points": [
                  "正确解释 `//` 表示相对路径，`table[@id='constituents']` 表示选择 id 属性为 'constituents' 的 table 元素，`//tr` 表示选择所有 tr 元素。",
                  "指出 BeautifulSoup 的方法链式调用，而 XPath 使用单一的路径表达式。",
                  "指出 XPath 可以基于位置、属性值、文本内容等多种条件进行组合查询，表达能力更强。"
                ],
                "stem": "请解释 XPath 表达式 `//table[@id='constituents']//tr` 的含义，并说明为什么使用 XPath 比使用 BeautifulSoup 的 `find('table', id='constituents')` 然后遍历 `find_all('tr')` 在某些情况下更灵活。"
              },
              {
                "estimated_seconds": 90,
                "prompt_blocks": [
                  "解释第一个表达式的含义和适用场景。",
                  "解释第二个表达式的含义和适用场景。",
                  "总结基于位置和基于属性选择元素的区别。"
                ],
                "question_id": "q_long_xpath_example_v2",
                "reference_answer": [
                  "`//li[2]` 会选择文档中所有 `<li>` 元素中，按文档顺序排列的第二个 `<li>` 元素。它适用于列表结构固定，需要提取特定位置元素的场景。",
                  "`//li[@class='item']` 会选择所有 class 属性值为 'item' 的 `<li>` 元素，无论它们在文档中的位置如何。它适用于根据元素的属性值进行筛选的场景，更加灵活和通用。",
                  "基于位置的选择依赖于元素的顺序，如果 HTML 结构发生变化，表达式可能失效。基于属性的选择不依赖顺序，只要元素的属性值不变，就能正确选中，因此鲁棒性更强。"
                ],
                "rubric_points": [
                  "正确指出 `//li[2]` 选择文档中所有 `<li>` 元素中的第二个。",
                  "正确指出 `//li[@class='item']` 选择所有 class 属性为 'item' 的 `<li>` 元素。",
                  "指出基于位置的选择适用于结构固定的列表，基于属性的选择更通用，不依赖顺序。"
                ],
                "stem": "请解释 XPath 表达式 `//li[2]` 和 `//li[@class='item']` 的区别，并说明它们分别适用于什么场景。"
              }
            ]
          }
        ],
        "quiz_families": [
          {
            "concept_key": "limitations_data_quality",
            "coverage_tags": [
              "limitations_data_quality"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_limitations_data_quality",
            "learning_goal": "学生能在具体场景中识别出数据质量问题。",
            "linked_steps": [
              "step8"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "数据质量",
                "en": "Data Quality"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 20,
                "explanation": "数据缺失是数据质量不完整的具体表现。",
                "options": [
                  "资源消耗大",
                  "数据质量不完整",
                  "网站结构变化",
                  "反爬机制"
                ],
                "question_id": "q_quiz_limitations_data_quality_v1",
                "stem": "你爬取了一个财经网站的历史股价数据，发现某只股票在2023年6月的数据完全缺失。这体现了网络爬取的哪个局限性？"
              },
              {
                "answer": 2,
                "estimated_seconds": 20,
                "explanation": "数据不是最新版本，属于数据过时的问题。",
                "options": [
                  "技术挑战",
                  "资源消耗大",
                  "数据过时",
                  "反爬机制"
                ],
                "question_id": "q_quiz_limitations_data_quality_v2",
                "stem": "你爬取了一个新闻网站的文章列表，但发现列表中的文章发布日期比实际晚了三天。这体现了网络爬取的哪个局限性？"
              }
            ]
          },
          {
            "concept_key": "limitations_resource_intensive",
            "coverage_tags": [
              "limitations_resource_intensive"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_limitations_resource_intensive",
            "learning_goal": "学生能判断哪些情况属于资源消耗问题。",
            "linked_steps": [
              "step8"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "资源密集型",
                "en": "Resource Intensive"
              }
            ],
            "variants": [
              {
                "answer": 2,
                "estimated_seconds": 20,
                "explanation": "存储大量数据会消耗大量磁盘空间，属于资源消耗问题。",
                "options": [
                  "数据质量差",
                  "技术挑战",
                  "资源消耗大（存储）",
                  "网站结构变化"
                ],
                "question_id": "q_quiz_limitations_resource_intensive_v1",
                "stem": "你计划爬取全球所有交易所的股票历史数据，但发现本地磁盘空间不足。这体现了网络爬取的哪个局限性？"
              },
              {
                "answer": 2,
                "estimated_seconds": 20,
                "explanation": "加载包含大量资源的网页非常耗时，属于时间上的资源消耗。",
                "options": [
                  "反爬机制",
                  "数据质量",
                  "资源消耗大（时间）",
                  "网站结构变化"
                ],
                "question_id": "q_quiz_limitations_resource_intensive_v2",
                "stem": "为了爬取一个包含大量高清图片的网页，你的程序运行了很长时间才完成。这主要体现了网络爬取的哪个局限性？"
              }
            ]
          },
          {
            "concept_key": "limitations_technical_challenges",
            "coverage_tags": [
              "limitations_technical_challenges"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_limitations_technical_challenges",
            "learning_goal": "学生能区分不同的技术挑战。",
            "linked_steps": [
              "step8"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "技术挑战",
                "en": "Technical Challenges"
              }
            ],
            "variants": [
              {
                "answer": 2,
                "estimated_seconds": 20,
                "explanation": "网站结构变化会导致基于旧结构编写的爬虫失效。",
                "options": [
                  "IP 封锁",
                  "频率限制",
                  "网站结构变化",
                  "数据过时"
                ],
                "question_id": "q_quiz_limitations_technical_challenges_v1",
                "stem": "你的爬虫程序昨天还能正常工作，但今天却报错了，原因是目标网站改版了。这属于哪种技术挑战？"
              },
              {
                "answer": 1,
                "estimated_seconds": 20,
                "explanation": "IP封锁是常见的反爬机制，用于阻止来自特定IP的请求。",
                "options": [
                  "频率限制",
                  "IP 封锁",
                  "验证码",
                  "数据加密"
                ],
                "question_id": "q_quiz_limitations_technical_challenges_v2",
                "stem": "你的爬虫在短时间内发送了大量请求，随后你的IP地址被目标网站禁止访问。这属于哪种反爬机制？"
              }
            ]
          },
          {
            "concept_key": "selenium_tool",
            "coverage_tags": [
              "selenium_tool"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_selenium_tool",
            "learning_goal": "学生能判断在何种场景下应该使用Selenium。",
            "linked_steps": [
              "step8"
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
                "answer": 2,
                "estimated_seconds": 20,
                "explanation": "Selenium 可以模拟用户点击操作，从而触发动态内容加载。",
                "options": [
                  "BeautifulSoup",
                  "Requests",
                  "Selenium",
                  "yfinance"
                ],
                "question_id": "q_quiz_selenium_tool_v1",
                "stem": "你需要从一个需要点击“加载更多”按钮才能显示全部内容的网页中提取数据。以下哪个工具最适合？"
              },
              {
                "answer": 2,
                "estimated_seconds": 20,
                "explanation": "Selenium 的核心功能是模拟用户与浏览器的交互。",
                "options": [
                  "它是一个用于发送 HTTP 请求的库。",
                  "它只能用于静态网页的爬取。",
                  "它可以模拟真实的浏览器操作。",
                  "它专门用于解析 HTML 文档。"
                ],
                "question_id": "q_quiz_selenium_tool_v2",
                "stem": "以下哪个关于 Selenium 的描述是正确的？"
              }
            ]
          },
          {
            "concept_key": "xpath_lxml_tool",
            "coverage_tags": [
              "xpath_lxml_tool"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_xpath_lxml_tool",
            "learning_goal": "学生能理解XPath的基本语法和用途。",
            "linked_steps": [
              "step8"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "XPath",
                "en": "XPath"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 25,
                "explanation": "`//table[@id='constituents']` 选择目标表格，`//tr` 选择该表格下的所有行。",
                "options": [
                  "选择所有 id 为 'constituents' 的表格。",
                  "选择所有 id 为 'constituents' 的表格中的所有行。",
                  "选择所有表格中的第一个行。",
                  "选择所有行中的第一个表格。"
                ],
                "question_id": "q_quiz_xpath_lxml_tool_v1",
                "stem": "XPath 表达式 `//table[@id='constituents']//tr` 的作用是什么？"
              },
              {
                "answer": 2,
                "estimated_seconds": 20,
                "explanation": "XPath 提供了强大的路径表达式，可以更精确和灵活地定位任何元素。",
                "options": [
                  "不需要安装额外的库。",
                  "只能用于 XML 文档。",
                  "定位元素的方式更灵活。",
                  "运行速度更慢。"
                ],
                "question_id": "q_quiz_xpath_lxml_tool_v2",
                "stem": "与 BeautifulSoup 相比，使用 lxml 和 XPath 进行数据提取的一个主要优势是什么？"
              }
            ]
          }
        ],
        "source": {
          "coverage_checklist": "L2: Data scraping and database management with Python - The limitations of web scraping",
          "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
          "lesson_map": "L2 step8: Limitations of web scraping and alternative tools",
          "plain_text": "pipeline/1-plain/L2/plain.txt",
          "related": [
            "pipeline/2-related_important/course_desc.md"
          ],
          "source_outline": "L2: Data scraping and database management with Python - The limitations of web scraping"
        },
        "target_language": "zh-CN"
      },
      "question_bank_path": "research/pipeline/3-guided_story/L2/step8/question_bank.json",
      "question_families": [
        {
          "family_id": "qf_flash_limitations_data_quality",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "limitations_data_quality",
            "coverage_tags": [
              "limitations_data_quality"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_limitations_data_quality",
            "learning_goal": "学生能准确说出网络爬取在数据质量方面的主要局限性。",
            "linked_steps": [
              "step8"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "网络爬取在数据质量方面的具体问题。",
            "term_refs": [
              {
                "display": "数据质量",
                "en": "Data Quality"
              }
            ],
            "variants": [
              {
                "back": "数据可能不完整或过时。",
                "estimated_seconds": 8,
                "explanation": "爬取到的数据可能缺失部分内容，或者不是最新版本，影响分析准确性。",
                "front": "网络爬取在数据质量方面的一个主要局限性是什么？",
                "question_id": "q_flash_limitations_data_quality_v1"
              },
              {
                "back": "不完整和过时。",
                "estimated_seconds": 8,
                "explanation": "网站可能只展示了部分数据，或者数据更新不及时，导致爬取结果有缺陷。",
                "front": "从网站爬取的数据，在质量上可能面临哪两个具体问题？",
                "question_id": "q_flash_limitations_data_quality_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_limitations_resource_intensive",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "limitations_resource_intensive",
            "coverage_tags": [
              "limitations_resource_intensive"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_limitations_resource_intensive",
            "learning_goal": "学生能说出网络爬取在资源消耗方面的两个主要问题。",
            "linked_steps": [
              "step8"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "网络爬取消耗的具体资源。",
            "term_refs": [
              {
                "display": "资源密集型",
                "en": "Resource Intensive"
              }
            ],
            "variants": [
              {
                "back": "带宽和存储。",
                "estimated_seconds": 8,
                "explanation": "下载大量网页会占用网络带宽，存储爬取的数据也需要大量磁盘空间。",
                "front": "爬取大量数据时，会消耗哪两种主要资源？",
                "question_id": "q_flash_limitations_resource_intensive_v1"
              },
              {
                "back": "加载整个网页很耗时。",
                "estimated_seconds": 8,
                "explanation": "为了提取数据，需要完整加载网页内容，这个过程可能非常缓慢。",
                "front": "除了带宽和存储，爬取大量数据时还有什么时间上的消耗？",
                "question_id": "q_flash_limitations_resource_intensive_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_limitations_technical_challenges",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "limitations_technical_challenges",
            "coverage_tags": [
              "limitations_technical_challenges"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_limitations_technical_challenges",
            "learning_goal": "学生能列举网络爬取面临的两个主要技术挑战。",
            "linked_steps": [
              "step8"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "网络爬取的技术挑战的具体例子。",
            "term_refs": [
              {
                "display": "技术挑战",
                "en": "Technical Challenges"
              }
            ],
            "variants": [
              {
                "back": "网站结构经常变化。",
                "estimated_seconds": 8,
                "explanation": "如果网站改版，之前写的爬虫代码可能失效，需要重新适配。",
                "front": "网络爬取面临的一个常见技术挑战是什么？",
                "question_id": "q_flash_limitations_technical_challenges_v1"
              },
              {
                "back": "IP 封锁和频率限制。",
                "estimated_seconds": 8,
                "explanation": "IP封锁会禁止某个IP地址访问，频率限制则限制单位时间内的请求次数。",
                "front": "网站常用的两种反爬机制是什么？",
                "question_id": "q_flash_limitations_technical_challenges_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_selenium_tool",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "selenium_tool",
            "coverage_tags": [
              "selenium_tool"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_selenium_tool",
            "learning_goal": "学生能说出Selenium的核心功能和适用场景。",
            "linked_steps": [
              "step8"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "Selenium 是什么以及它能解决什么问题。",
            "term_refs": [
              {
                "display": "Selenium",
                "en": "Selenium"
              }
            ],
            "variants": [
              {
                "back": "一个用于模拟用户交互和自动化浏览器的工具。",
                "estimated_seconds": 8,
                "explanation": "它可以模拟点击、输入等操作，适用于处理动态加载的网页。",
                "front": "Selenium 是一个什么工具？",
                "question_id": "q_flash_selenium_tool_v1"
              },
              {
                "back": "Selenium。",
                "estimated_seconds": 8,
                "explanation": "Selenium 可以模拟真实的浏览器行为，从而获取由 JavaScript 动态生成的内容。",
                "front": "当 BeautifulSoup 无法处理动态加载的网页时，应该使用哪个工具？",
                "question_id": "q_flash_selenium_tool_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_xpath_lxml_tool",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "xpath_lxml_tool",
            "coverage_tags": [
              "xpath_lxml_tool"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_xpath_lxml_tool",
            "learning_goal": "学生能说出XPath是什么以及lxml库的作用。",
            "linked_steps": [
              "step8"
            ],
            "question_type": "short_answer",
            "retrieval_focus": "XPath 和 lxml 的基本定义。",
            "term_refs": [
              {
                "display": "XPath",
                "en": "XPath"
              },
              {
                "display": "lxml",
                "en": "lxml"
              }
            ],
            "variants": [
              {
                "back": "一种在 XML/HTML 文档中导航和选择节点的查询语言。",
                "estimated_seconds": 8,
                "explanation": "它使用路径表达式来精确定位文档中的元素。",
                "front": "XPath 是一种什么语言？",
                "question_id": "q_flash_xpath_lxml_tool_v1"
              },
              {
                "back": "一个用于解析 XPath 和 XML 模式的 Python 库。",
                "estimated_seconds": 8,
                "explanation": "它支持使用 XPath 语法高效地提取 HTML 或 XML 数据。",
                "front": "lxml 是一个什么库？",
                "question_id": "q_flash_xpath_lxml_tool_v2"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_limitations_data_quality",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "limitations_data_quality",
            "coverage_tags": [
              "limitations_data_quality"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_limitations_data_quality",
            "learning_goal": "学生能在具体场景中识别出数据质量问题。",
            "linked_steps": [
              "step8"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "数据质量",
                "en": "Data Quality"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 20,
                "explanation": "数据缺失是数据质量不完整的具体表现。",
                "options": [
                  "资源消耗大",
                  "数据质量不完整",
                  "网站结构变化",
                  "反爬机制"
                ],
                "question_id": "q_quiz_limitations_data_quality_v1",
                "stem": "你爬取了一个财经网站的历史股价数据，发现某只股票在2023年6月的数据完全缺失。这体现了网络爬取的哪个局限性？"
              },
              {
                "answer": 2,
                "estimated_seconds": 20,
                "explanation": "数据不是最新版本，属于数据过时的问题。",
                "options": [
                  "技术挑战",
                  "资源消耗大",
                  "数据过时",
                  "反爬机制"
                ],
                "question_id": "q_quiz_limitations_data_quality_v2",
                "stem": "你爬取了一个新闻网站的文章列表，但发现列表中的文章发布日期比实际晚了三天。这体现了网络爬取的哪个局限性？"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_limitations_resource_intensive",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "limitations_resource_intensive",
            "coverage_tags": [
              "limitations_resource_intensive"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_limitations_resource_intensive",
            "learning_goal": "学生能判断哪些情况属于资源消耗问题。",
            "linked_steps": [
              "step8"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "资源密集型",
                "en": "Resource Intensive"
              }
            ],
            "variants": [
              {
                "answer": 2,
                "estimated_seconds": 20,
                "explanation": "存储大量数据会消耗大量磁盘空间，属于资源消耗问题。",
                "options": [
                  "数据质量差",
                  "技术挑战",
                  "资源消耗大（存储）",
                  "网站结构变化"
                ],
                "question_id": "q_quiz_limitations_resource_intensive_v1",
                "stem": "你计划爬取全球所有交易所的股票历史数据，但发现本地磁盘空间不足。这体现了网络爬取的哪个局限性？"
              },
              {
                "answer": 2,
                "estimated_seconds": 20,
                "explanation": "加载包含大量资源的网页非常耗时，属于时间上的资源消耗。",
                "options": [
                  "反爬机制",
                  "数据质量",
                  "资源消耗大（时间）",
                  "网站结构变化"
                ],
                "question_id": "q_quiz_limitations_resource_intensive_v2",
                "stem": "为了爬取一个包含大量高清图片的网页，你的程序运行了很长时间才完成。这主要体现了网络爬取的哪个局限性？"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_limitations_technical_challenges",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "limitations_technical_challenges",
            "coverage_tags": [
              "limitations_technical_challenges"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_limitations_technical_challenges",
            "learning_goal": "学生能区分不同的技术挑战。",
            "linked_steps": [
              "step8"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "技术挑战",
                "en": "Technical Challenges"
              }
            ],
            "variants": [
              {
                "answer": 2,
                "estimated_seconds": 20,
                "explanation": "网站结构变化会导致基于旧结构编写的爬虫失效。",
                "options": [
                  "IP 封锁",
                  "频率限制",
                  "网站结构变化",
                  "数据过时"
                ],
                "question_id": "q_quiz_limitations_technical_challenges_v1",
                "stem": "你的爬虫程序昨天还能正常工作，但今天却报错了，原因是目标网站改版了。这属于哪种技术挑战？"
              },
              {
                "answer": 1,
                "estimated_seconds": 20,
                "explanation": "IP封锁是常见的反爬机制，用于阻止来自特定IP的请求。",
                "options": [
                  "频率限制",
                  "IP 封锁",
                  "验证码",
                  "数据加密"
                ],
                "question_id": "q_quiz_limitations_technical_challenges_v2",
                "stem": "你的爬虫在短时间内发送了大量请求，随后你的IP地址被目标网站禁止访问。这属于哪种反爬机制？"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_selenium_tool",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "selenium_tool",
            "coverage_tags": [
              "selenium_tool"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_selenium_tool",
            "learning_goal": "学生能判断在何种场景下应该使用Selenium。",
            "linked_steps": [
              "step8"
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
                "answer": 2,
                "estimated_seconds": 20,
                "explanation": "Selenium 可以模拟用户点击操作，从而触发动态内容加载。",
                "options": [
                  "BeautifulSoup",
                  "Requests",
                  "Selenium",
                  "yfinance"
                ],
                "question_id": "q_quiz_selenium_tool_v1",
                "stem": "你需要从一个需要点击“加载更多”按钮才能显示全部内容的网页中提取数据。以下哪个工具最适合？"
              },
              {
                "answer": 2,
                "estimated_seconds": 20,
                "explanation": "Selenium 的核心功能是模拟用户与浏览器的交互。",
                "options": [
                  "它是一个用于发送 HTTP 请求的库。",
                  "它只能用于静态网页的爬取。",
                  "它可以模拟真实的浏览器操作。",
                  "它专门用于解析 HTML 文档。"
                ],
                "question_id": "q_quiz_selenium_tool_v2",
                "stem": "以下哪个关于 Selenium 的描述是正确的？"
              }
            ]
          }
        },
        {
          "family_id": "qf_quiz_xpath_lxml_tool",
          "kind": "quiz_families",
          "summary": {
            "concept_key": "xpath_lxml_tool",
            "coverage_tags": [
              "xpath_lxml_tool"
            ],
            "difficulty": "medium",
            "family_id": "qf_quiz_xpath_lxml_tool",
            "learning_goal": "学生能理解XPath的基本语法和用途。",
            "linked_steps": [
              "step8"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "XPath",
                "en": "XPath"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 25,
                "explanation": "`//table[@id='constituents']` 选择目标表格，`//tr` 选择该表格下的所有行。",
                "options": [
                  "选择所有 id 为 'constituents' 的表格。",
                  "选择所有 id 为 'constituents' 的表格中的所有行。",
                  "选择所有表格中的第一个行。",
                  "选择所有行中的第一个表格。"
                ],
                "question_id": "q_quiz_xpath_lxml_tool_v1",
                "stem": "XPath 表达式 `//table[@id='constituents']//tr` 的作用是什么？"
              },
              {
                "answer": 2,
                "estimated_seconds": 20,
                "explanation": "XPath 提供了强大的路径表达式，可以更精确和灵活地定位任何元素。",
                "options": [
                  "不需要安装额外的库。",
                  "只能用于 XML 文档。",
                  "定位元素的方式更灵活。",
                  "运行速度更慢。"
                ],
                "question_id": "q_quiz_xpath_lxml_tool_v2",
                "stem": "与 BeautifulSoup 相比，使用 lxml 和 XPath 进行数据提取的一个主要优势是什么？"
              }
            ]
          }
        },
        {
          "family_id": "qf_long_selenium_use_case",
          "kind": "longform_families",
          "summary": {
            "concept_key": "selenium_tool",
            "coverage_tags": [
              "selenium_tool"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_selenium_use_case",
            "learning_goal": "学生能解释在什么情况下需要使用Selenium，并说明其工作原理。",
            "linked_steps": [
              "step8"
            ],
            "question_type": "short_explain",
            "term_refs": [
              {
                "display": "Selenium",
                "en": "Selenium"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 90,
                "prompt_blocks": [
                  "解释动态加载网页的特点。",
                  "说明 BeautifulSoup 的工作原理及其局限性。",
                  "说明 Selenium 的工作原理及其如何克服这个局限性。"
                ],
                "question_id": "q_long_selenium_use_case_v1",
                "reference_answer": [
                  "动态加载的网页内容（如点击按钮后出现的数据）是由 JavaScript 在浏览器中执行后生成的，这些内容并不在初始的 HTML 源代码中。",
                  "BeautifulSoup 只能解析服务器返回的静态 HTML 源代码，因此无法获取这些动态生成的内容。",
                  "Selenium 通过控制一个真实的浏览器（如 Chrome）来加载网页，它会像真实用户一样执行 JavaScript，从而获取到完整的、动态渲染后的页面内容。"
                ],
                "rubric_points": [
                  "正确指出动态加载的内容由 JavaScript 生成。",
                  "正确指出 BeautifulSoup 只能解析静态 HTML 源代码。",
                  "正确指出 Selenium 可以模拟浏览器执行 JavaScript，从而获取动态内容。"
                ],
                "stem": "请解释为什么对于动态加载的网页，BeautifulSoup 可能无能为力，而 Selenium 可以解决这个问题。"
              },
              {
                "estimated_seconds": 90,
                "prompt_blocks": [
                  "分析使用 Requests 处理登录的复杂性。",
                  "分析使用 Selenium 处理登录的简便性。",
                  "总结 Selenium 在此场景下的优势。"
                ],
                "question_id": "q_long_selenium_use_case_v2",
                "reference_answer": [
                  "使用 Requests 模拟登录需要分析登录表单的结构，手动构造 POST 请求，并处理服务器返回的 Cookie 和 Session，过程繁琐且容易出错。",
                  "使用 Selenium，你可以编写代码自动在输入框中填写用户名和密码，然后点击登录按钮，就像真实用户一样。Selenium 会自动处理 Cookie 和 Session。",
                  "对于包含验证码或两步验证的复杂登录流程，Selenium 的优势更加明显，因为它可以模拟更复杂的用户交互。"
                ],
                "rubric_points": [
                  "指出使用 Requests 需要手动管理 Cookie、Session 和表单提交。",
                  "指出 Selenium 可以自动处理登录表单的填写和提交，并自动管理 Cookie。",
                  "指出 Selenium 更适合处理包含复杂交互（如验证码、两步验证）的登录流程。"
                ],
                "stem": "假设你需要从一个需要登录后才能查看数据的网站爬取信息。请解释为什么使用 Requests + BeautifulSoup 的组合会比较困难，而 Selenium 可以更简单地实现。"
              }
            ]
          }
        },
        {
          "family_id": "qf_long_xpath_example",
          "kind": "longform_families",
          "summary": {
            "concept_key": "xpath_lxml_tool",
            "coverage_tags": [
              "xpath_lxml_tool"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_xpath_example",
            "learning_goal": "学生能解释一个具体XPath表达式的含义，并说明其相对于其他方法的优势。",
            "linked_steps": [
              "step8"
            ],
            "question_type": "short_explain",
            "term_refs": [
              {
                "display": "XPath",
                "en": "XPath"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 90,
                "prompt_blocks": [
                  "分解并解释 XPath 表达式的每个部分。",
                  "对比 XPath 和 BeautifulSoup 在定位元素时的不同方式。",
                  "说明 XPath 在哪些方面更灵活。"
                ],
                "question_id": "q_long_xpath_example_v1",
                "reference_answer": [
                  "`//table[@id='constituents']//tr` 的含义是：在整个文档中，找到 id 属性等于 'constituents' 的 `<table>` 元素，然后在该元素下找到所有 `<tr>` 元素。",
                  "BeautifulSoup 通常需要先找到父元素，再通过循环或另一个查找方法找到子元素，是一种链式调用。而 XPath 用一个表达式就能完成从根节点到目标节点的路径描述。",
                  "XPath 的灵活性体现在它支持丰富的函数和轴（如 `position()`、`following-sibling`），可以基于元素的位置、文本内容、属性值等进行非常精确的定位，这在处理结构复杂或相似的 HTML 时优势明显。"
                ],
                "rubric_points": [
                  "正确解释 `//` 表示相对路径，`table[@id='constituents']` 表示选择 id 属性为 'constituents' 的 table 元素，`//tr` 表示选择所有 tr 元素。",
                  "指出 BeautifulSoup 的方法链式调用，而 XPath 使用单一的路径表达式。",
                  "指出 XPath 可以基于位置、属性值、文本内容等多种条件进行组合查询，表达能力更强。"
                ],
                "stem": "请解释 XPath 表达式 `//table[@id='constituents']//tr` 的含义，并说明为什么使用 XPath 比使用 BeautifulSoup 的 `find('table', id='constituents')` 然后遍历 `find_all('tr')` 在某些情况下更灵活。"
              },
              {
                "estimated_seconds": 90,
                "prompt_blocks": [
                  "解释第一个表达式的含义和适用场景。",
                  "解释第二个表达式的含义和适用场景。",
                  "总结基于位置和基于属性选择元素的区别。"
                ],
                "question_id": "q_long_xpath_example_v2",
                "reference_answer": [
                  "`//li[2]` 会选择文档中所有 `<li>` 元素中，按文档顺序排列的第二个 `<li>` 元素。它适用于列表结构固定，需要提取特定位置元素的场景。",
                  "`//li[@class='item']` 会选择所有 class 属性值为 'item' 的 `<li>` 元素，无论它们在文档中的位置如何。它适用于根据元素的属性值进行筛选的场景，更加灵活和通用。",
                  "基于位置的选择依赖于元素的顺序，如果 HTML 结构发生变化，表达式可能失效。基于属性的选择不依赖顺序，只要元素的属性值不变，就能正确选中，因此鲁棒性更强。"
                ],
                "rubric_points": [
                  "正确指出 `//li[2]` 选择文档中所有 `<li>` 元素中的第二个。",
                  "正确指出 `//li[@class='item']` 选择所有 class 属性为 'item' 的 `<li>` 元素。",
                  "指出基于位置的选择适用于结构固定的列表，基于属性的选择更通用，不依赖顺序。"
                ],
                "stem": "请解释 XPath 表达式 `//li[2]` 和 `//li[@class='item']` 的区别，并说明它们分别适用于什么场景。"
              }
            ]
          }
        }
      ],
      "sequence_id": "step8",
      "step": {
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
      },
      "step_path": "research/pipeline/3-guided_story/L2/step8/step.json"
    }
  ],
  "target_language": "zh-CN",
  "textbook": {
    "content": "---\nlessonId: L2\ntitle: 数据爬取与数据库管理\ntargetLanguage: zh-CN\nmode: textbook\nsectionMap:\n  - sectionId: why-this-lesson-matters\n    title: 为什么这一课重要\n    coverageTags: [\"web_scraping_definition\", \"data_as_fuel_for_algo_trading\", \"web_scraping_algo_trading_use_cases\"]\n    linkedSteps: [\"step1\"]\n  - sectionId: concept-map\n    title: 课程全景与关键问题\n    coverageTags: [\"web_scraping_definition\", \"web_scraping_algo_trading_use_cases\", \"basic_web_structure_three_components\", \"html_purpose_and_nature\", \"css_purpose\", \"javascript_purpose\", \"scraping_focus_html\", \"beautifulsoup_intro_and_setup\", \"requests_download_raw_content\", \"beautifulsoup_parse_and_pretty_print\", \"beautifulsoup_extract_text\", \"beautifulsoup_find_and_find_all\", \"beautifulsoup_table_to_dataframe\", \"real_world_scraping_sp500\", \"handling_anti_bot_measures\", \"yfinance_intro\", \"yfinance_basic_usage\", \"adjusted_close_concept\", \"close_vs_adjusted_close\", \"database_management_why\", \"sqlite_basics\", \"sql_crud_syntax\", \"create_table\", \"scrape_to_db\", \"database_design_big_table_issue\", \"database_design_partition_by_time\", \"database_design_partition_by_stock\", \"database_design_no_perfect_design\", \"database_design_tradeoff_factors\", \"limitations_data_quality\", \"limitations_resource_intensive\", \"limitations_technical_challenges\", \"selenium_tool\", \"xpath_lxml_tool\"]\n    linkedSteps: [\"step1\", \"step2\", \"step3\", \"step4\", \"step5\", \"step6\", \"step7\", \"step8\"]\n  - sectionId: web-scraping-foundations\n    title: 网络爬取基础：从网页结构到数据提取\n    coverageTags: [\"web_scraping_definition\", \"basic_web_structure_three_components\", \"html_purpose_and_nature\", \"css_purpose\", \"javascript_purpose\", \"scraping_focus_html\", \"beautifulsoup_intro_and_setup\", \"requests_download_raw_content\", \"beautifulsoup_parse_and_pretty_print\", \"beautifulsoup_extract_text\", \"beautifulsoup_find_and_find_all\", \"beautifulsoup_table_to_dataframe\"]\n    linkedSteps: [\"step1\", \"step2\", \"step3\"]\n  - sectionId: real-world-scraping\n    title: 实战爬取与可靠数据源\n    coverageTags: [\"real_world_scraping_sp500\", \"handling_anti_bot_measures\", \"yfinance_intro\", \"yfinance_basic_usage\", \"adjusted_close_concept\", \"close_vs_adjusted_close\"]\n    linkedSteps: [\"step4\", \"step5\"]\n  - sectionId: database-management\n    title: 数据库管理：存储与查询金融数据\n    coverageTags: [\"database_management_why\", \"sqlite_basics\", \"sql_crud_syntax\", \"create_table\", \"scrape_to_db\"]\n    linkedSteps: [\"step6\"]\n  - sectionId: database-design\n    title: 金融数据存储的数据库设计\n    coverageTags: [\"database_design_big_table_issue\", \"database_design_partition_by_time\", \"database_design_partition_by_stock\", \"database_design_no_perfect_design\", \"database_design_tradeoff_factors\"]\n    linkedSteps: [\"step7\"]\n  - sectionId: limitations-and-tools\n    title: 爬取的局限性与替代工具\n    coverageTags: [\"limitations_data_quality\", \"limitations_resource_intensive\", \"limitations_technical_challenges\", \"selenium_tool\", \"xpath_lxml_tool\"]\n    linkedSteps: [\"step8\"]\n  - sectionId: pitfalls\n    title: 易错点\n    coverageTags: [\"handling_anti_bot_measures\", \"close_vs_adjusted_close\", \"database_design_big_table_issue\", \"database_design_no_perfect_design\"]\n    linkedSteps: [\"step4\", \"step5\", \"step7\"]\n  - sectionId: review-path\n    title: 复习路线\n    coverageTags: [\"web_scraping_definition\", \"basic_web_structure_three_components\", \"beautifulsoup_intro_and_setup\", \"real_world_scraping_sp500\", \"yfinance_intro\", \"adjusted_close_concept\", \"database_management_why\", \"sql_crud_syntax\", \"database_design_tradeoff_factors\", \"limitations_technical_challenges\"]\n    linkedSteps: [\"step1\", \"step2\", \"step3\", \"step4\", \"step5\", \"step6\", \"step7\", \"step8\"]\ncoverageTrace:\n  - coverageTag: web_scraping_definition\n    description: 网络爬取的定义：从网站自动提取数据的过程。\n    sectionIds: [\"why-this-lesson-matters\", \"concept-map\", \"web-scraping-foundations\", \"review-path\"]\n    linkedSteps: [\"step1\"]\n  - coverageTag: data_as_fuel_for_algo_trading\n    description: 数据是算法交易的燃料，没有数据策略就是空谈。\n    sectionIds: [\"why-this-lesson-matters\", \"concept-map\"]\n    linkedSteps: [\"step1\"]\n  - coverageTag: web_scraping_algo_trading_use_cases\n    description: 网络爬取在算法交易中的常见用例：实时数据收集、市场情绪分析、公司分析、另类数据。\n    sectionIds: [\"why-this-lesson-matters\", \"concept-map\"]\n    linkedSteps: [\"step1\"]\n  - coverageTag: basic_web_structure_three_components\n    description: 理解网页由 HTML、CSS 和 JavaScript 三部分组成。\n    sectionIds: [\"concept-map\", \"web-scraping-foundations\", \"review-path\"]\n    linkedSteps: [\"step2\"]\n  - coverageTag: html_purpose_and_nature\n    description: HTML 定义网页的结构和内容，是一种标记语言而非编程语言。\n    sectionIds: [\"concept-map\", \"web-scraping-foundations\"]\n    linkedSteps: [\"step2\"]\n  - coverageTag: css_purpose\n    description: CSS 控制网页的视觉呈现和布局。\n    sectionIds: [\"concept-map\", \"web-scraping-foundations\"]\n    linkedSteps: [\"step2\"]\n  - coverageTag: javascript_purpose\n    description: JavaScript 为网页添加交互和动态行为。\n    sectionIds: [\"concept-map\", \"web-scraping-foundations\"]\n    linkedSteps: [\"step2\"]\n  - coverageTag: scraping_focus_html\n    description: 爬取数据时最关心 HTML，因为数据通常藏在 HTML 标签中。\n    sectionIds: [\"concept-map\", \"web-scraping-foundations\"]\n    linkedSteps: [\"step2\"]\n  - coverageTag: beautifulsoup_intro_and_setup\n    description: BeautifulSoup 库的用途、安装方式以及与 Requests 配合使用的基本流程。\n    sectionIds: [\"concept-map\", \"web-scraping-foundations\", \"review-path\"]\n    linkedSteps: [\"step3\"]\n  - coverageTag: requests_download_raw_content\n    description: 使用 Requests 库下载网页原始 HTML 内容。\n    sectionIds: [\"concept-map\", \"web-scraping-foundations\"]\n    linkedSteps: [\"step3\"]\n  - coverageTag: beautifulsoup_parse_and_pretty_print\n    description: BeautifulSoup 解析 HTML 为树状结构，prettify() 格式化输出。\n    sectionIds: [\"concept-map\", \"web-scraping-foundations\"]\n    linkedSteps: [\"step3\"]\n  - coverageTag: beautifulsoup_extract_text\n    description: 使用 get_text() 提取网页中所有纯文本。\n    sectionIds: [\"concept-map\", \"web-scraping-foundations\"]\n    linkedSteps: [\"step3\"]\n  - coverageTag: beautifulsoup_find_and_find_all\n    description: 使用 find() 和 find_all() 定位特定 HTML 标签。\n    sectionIds: [\"concept-map\", \"web-scraping-foundations\"]\n    linkedSteps: [\"step3\"]\n  - coverageTag: beautifulsoup_table_to_dataframe\n    description: 使用 BeautifulSoup 找到 <table> 标签后配合 pandas 转换为 DataFrame。\n    sectionIds: [\"concept-map\", \"web-scraping-foundations\"]\n    linkedSteps: [\"step3\"]\n  - coverageTag: real_world_scraping_sp500\n    description: 实战爬取 S&P 500 成分股列表。\n    sectionIds: [\"concept-map\", \"real-world-scraping\", \"review-path\"]\n    linkedSteps: [\"step4\"]\n  - coverageTag: handling_anti_bot_measures\n    description: 处理反爬机制：User-Agent。\n    sectionIds: [\"concept-map\", \"real-world-scraping\", \"pitfalls\"]\n    linkedSteps: [\"step4\"]\n  - coverageTag: yfinance_intro\n    description: yfinance 库的基本介绍。\n    sectionIds: [\"concept-map\", \"real-world-scraping\", \"review-path\"]\n    linkedSteps: [\"step5\"]\n  - coverageTag: yfinance_basic_usage\n    description: yfinance 的基本用法。\n    sectionIds: [\"concept-map\", \"real-world-scraping\"]\n    linkedSteps: [\"step5\"]\n  - coverageTag: adjusted_close_concept\n    description: 调整收盘价的概念。\n    sectionIds: [\"concept-map\", \"real-world-scraping\", \"review-path\"]\n    linkedSteps: [\"step5\"]\n  - coverageTag: close_vs_adjusted_close\n    description: 收盘价与调整收盘价的区别。\n    sectionIds: [\"concept-map\", \"real-world-scraping\", \"pitfalls\"]\n    linkedSteps: [\"step5\"]\n  - coverageTag: database_management_why\n    description: 在算法交易中使用数据库存储爬取数据的原因。\n    sectionIds: [\"concept-map\", \"database-management\", \"review-path\"]\n    linkedSteps: [\"step6\"]\n  - coverageTag: sqlite_basics\n    description: 掌握 Python 内置 SQLite 库的基本用法。\n    sectionIds: [\"concept-map\", \"database-management\"]\n    linkedSteps: [\"step6\"]\n  - coverageTag: sql_crud_syntax\n    description: 掌握 SQL 四种基本操作（CRUD）的语法。\n    sectionIds: [\"concept-map\", \"database-management\", \"review-path\"]\n    linkedSteps: [\"step6\"]\n  - coverageTag: create_table\n    description: 能够使用 SQL 的 CREATE TABLE 语句定义表结构。\n    sectionIds: [\"concept-map\", \"database-management\"]\n    linkedSteps: [\"step6\"]\n  - coverageTag: scrape_to_db\n    description: 能够将爬取到的金融数据写入 SQLite 数据库。\n    sectionIds: [\"concept-map\", \"database-management\"]\n    linkedSteps: [\"step6\"]\n  - coverageTag: database_design_big_table_issue\n    description: 将所有数据放入一张大表会导致的问题。\n    sectionIds: [\"concept-map\", \"database-design\", \"pitfalls\"]\n    linkedSteps: [\"step7\"]\n  - coverageTag: database_design_partition_by_time\n    description: 按时间分区可以缓解大表问题，但跨月分析困难。\n    sectionIds: [\"concept-map\", \"database-design\"]\n    linkedSteps: [\"step7\"]\n  - coverageTag: database_design_partition_by_stock\n    description: 按股票分区便于回测，但跨资产对比分析困难。\n    sectionIds: [\"concept-map\", \"database-design\"]\n    linkedSteps: [\"step7\"]\n  - coverageTag: database_design_no_perfect_design\n    description: 没有适用于所有情况的完美数据库设计。\n    sectionIds: [\"concept-map\", \"database-design\", \"pitfalls\"]\n    linkedSteps: [\"step7\"]\n  - coverageTag: database_design_tradeoff_factors\n    description: 数据库设计需要在存储空间、运行速度和内存消耗之间取得平衡。\n    sectionIds: [\"concept-map\", \"database-design\", \"review-path\"]\n    linkedSteps: [\"step7\"]\n  - coverageTag: limitations_data_quality\n    description: 网络爬取的数据质量局限性。\n    sectionIds: [\"concept-map\", \"limitations-and-tools\"]\n    linkedSteps: [\"step8\"]\n  - coverageTag: limitations_resource_intensive\n    description: 网络爬取的资源消耗局限性。\n    sectionIds: [\"concept-map\", \"limitations-and-tools\"]\n    linkedSteps: [\"step8\"]\n  - coverageTag: limitations_technical_challenges\n    description: 网络爬取的技术挑战。\n    sectionIds: [\"concept-map\", \"limitations-and-tools\", \"review-path\"]\n    linkedSteps: [\"step8\"]\n  - coverageTag: selenium_tool\n    description: Selenium：用于模拟用户交互和自动化浏览器的工具。\n    sectionIds: [\"concept-map\", \"limitations-and-tools\"]\n    linkedSteps: [\"step8\"]\n  - coverageTag: xpath_lxml_tool\n    description: XPath 和 lxml：XPath 是一种在 XML/HTML 文档中导航和选择节点的查询语言。\n    sectionIds: [\"concept-map\", \"limitations-and-tools\"]\n    linkedSteps: [\"step8\"]\n---\n\n# 数据爬取与数据库管理\n\n## 为什么这一课重要 {#why-this-lesson-matters}\n\n想象一下，你每天需要手动查看几百只股票的价格、新闻和财报。这几乎不可能，而且很容易出错。\n\n现在，让程序替你完成这些重复的工作。这就是 <Term id=\"web_scraping\" en=\"Web Scraping\">网络爬取</Term> 的核心思想。\n\n在 <Term id=\"algorithmic_trading\" en=\"Algorithmic Trading\">算法交易</Term> 中，数据就是燃料。没有数据，策略就是空谈。爬取的数据可以用于：\n\n- **实时数据收集**：自动获取股价、经济指标等。\n- **市场情绪分析**：爬取新闻、论坛和社交媒体内容。\n- **公司分析**：自动收集公司公告、财报、管理层变动等信息。\n- **另类数据**：收集天气、网络流量等非传统金融数据。\n\n<KeyPoint>网络爬取是算法交易数据 pipeline 的第一步，它让你能够自动化地、高效地从互联网获取结构化或非结构化数据，为后续的策略开发、回测和分析提供燃料。</KeyPoint>\n\n<QuestionRef id=\"q_quiz_web_scraping_use_case_v1\" />\n\n## 课程全景与关键问题 {#concept-map}\n\n本课将带你走通一条完整的数据 pipeline：从理解网页结构，到使用 Python 工具爬取数据，再到将数据存入数据库并进行合理的设计。\n\n```mermaid\nflowchart LR\n    A[理解网页结构<br/>HTML / CSS / JavaScript] --> B[使用 Python 爬取数据<br/>BeautifulSoup / Requests]\n    B --> C[实战爬取与可靠数据源<br/>S&P 500 / yfinance]\n    C --> D[存储数据到数据库<br/>SQLite / SQL]\n    D --> E[数据库设计权衡<br/>大表 / 分区 / 权衡]\n    B --> F[爬取的局限性与替代工具<br/>Selenium / XPath]\n```\n\n<Checkpoint title=\"全景自测\">\n<QuestionRef id=\"q_quiz_web_scraping_use_case_v2\" />\n</Checkpoint>\n\n## 网络爬取基础：从网页结构到数据提取 {#web-scraping-foundations}\n\n### 网页的三层架构\n\n一个网页通常由三种技术共同构成：\n\n1.  <Term id=\"html\" en=\"HyperText Markup Language\">HTML</Term>：定义网页的结构和内容，如标题、段落、表格。它是一种**标记语言**，不是编程语言。\n2.  <Term id=\"css\" en=\"Cascading Style Sheets\">CSS</Term>：控制网页的视觉呈现和布局，如颜色、字体、间距。\n3.  <Term id=\"javascript\" en=\"JavaScript\">JavaScript</Term>：为网页添加交互和动态行为，如点击按钮后弹出信息。\n\n<KeyPoint>进行网络爬取时，我们最关心的是 HTML。因为目标数据（如股票价格、新闻文本）通常直接包含在 HTML 的标签里。CSS 和 JavaScript 主要控制样式和交互，不直接承载数据。</KeyPoint>\n\n<Definition title=\"HTML 的本质\">\nHTML 使用标签（Tags）来创建元素。例如，`<h1>` 定义一级标题，`<p>` 定义段落，`<a>` 定义超链接，`<table>` 定义表格。爬虫的核心任务就是解析这些标签，提取其中的数据。\n</Definition>\n\n<Figure src=\"images/9a0356d3c69aa54d62765a0ef2abff6400803e8e1063976ac685e8e80ce3ccd6.jpg\" alt=\"一个简单的 HTML 页面在浏览器中的渲染效果，展示了标题和段落\">\n一个简单的 HTML 页面示例。\n</Figure>\n\n<QuestionFamily familyId=\"qf_quiz_web_components_role\" />\n\n### 使用 BeautifulSoup 解析 HTML\n\n<Term id=\"beautifulsoup\" en=\"BeautifulSoup\">BeautifulSoup</Term> 是 Python 中最流行的 HTML 解析库之一，它就像一个 HTML 的“瑞士军刀”。\n\n**标准工作流程：**\n\n1.  **下载网页**：使用 <Term id=\"requests\" en=\"Requests\">Requests</Term> 库下载网页的原始 HTML 内容。\n2.  **解析 HTML**：将原始内容交给 BeautifulSoup 解析，得到一个可操作的树状结构。\n\n```python\nimport requests\nfrom bs4 import BeautifulSoup\n\n# 1. 下载网页\nurl = \"https://example.com\"\nresponse = requests.get(url)\nhtml_content = response.text\n\n# 2. 解析 HTML\nsoup = BeautifulSoup(html_content, 'html.parser')\n```\n\n**核心方法：**\n\n- `soup.prettify()`：打印出格式清晰的 HTML，方便检查结构。\n- `soup.get_text()`：提取网页里所有的纯文本，去掉所有 HTML 标签。\n- `soup.find('p')`：找到第一个 `<p>` 标签。\n- `soup.find_all('a')`：找到所有 `<a>` 标签，返回一个列表。\n- `soup.find('table')`：找到第一个 `<table>` 标签，可配合 pandas 转换为 DataFrame。\n\n<Example title=\"提取表格数据到 DataFrame\">\n```python\nimport pandas as pd\n\n# 假设 soup 已经解析好\ntable = soup.find('table')\n\n# 提取表头\nheaders = [th.text for th in table.find_all('th')]\n\n# 提取数据行\nrows = []\nfor tr in table.find_all('tr')[1:]:  # 跳过表头行\n    cells = tr.find_all('td')\n    row = [cell.text for cell in cells]\n    rows.append(row)\n\n# 创建 DataFrame\ndf = pd.DataFrame(rows, columns=headers)\nprint(df)\n```\n</Example>\n\n<QuestionFamily familyId=\"qf_quiz_bs4_workflow\" />\n<QuestionFamily familyId=\"qf_quiz_bs4_methods\" />\n<QuestionFamily familyId=\"qf_quiz_bs4_find_vs_findall\" />\n\n## 实战爬取与可靠数据源 {#real-world-scraping}\n\n### 实战：爬取 S&P 500 成分股\n\n让我们实战一下：爬取维基百科上的 S&P 500 成分股列表。\n\n**挑战：反爬机制**\n\n直接用 `requests.get(url)` 请求维基百科，可能会返回错误信息。这是因为很多网站会检查请求的 <Term id=\"user_agent\" en=\"User-Agent\">User-Agent</Term>。如果它看起来不像一个真实的浏览器，网站就会拒绝访问。\n\n<KeyPoint>解决方法很简单：在请求头中伪装成浏览器。设置一个常见的 User-Agent 字符串即可。</KeyPoint>\n\n```python\nheaders = {\n    \"user-agent\": \"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36\"\n}\nresponse = requests.get(url, headers=headers)\n```\n\n**定位目标表格**\n\n成功获取页面后，用 BeautifulSoup 找到目标表格。维基百科页面上有多个表格，可以通过表格的 `id` 属性来精确定位。\n\n```python\ntable = soup.find('table', id='constituents')\n```\n\n<QuestionRef id=\"q_quiz_user_agent_fail_v1\" />\n<QuestionRef id=\"q_quiz_table_id_v1\" />\n\n### 使用 yfinance 获取可靠金融数据\n\n直接爬取 Yahoo Finance 会遇到很多反爬措施。好在有一个专门的库：<Term id=\"yfinance\" en=\"yfinance\">yfinance</Term>。它封装了所有复杂的请求，让你几行代码就能获取数据。\n\n```python\nimport yfinance as yf\n\n# 获取腾讯控股 (0700.HK) 的基本信息\nticker = yf.Ticker(\"0700.HK\")\nprint(ticker.info)\n\n# 获取最近一个月的股价历史\ndata = ticker.history(period='1mo')\nprint(data)\n\n# 同时获取多只股票\ndata = yf.download(\"0700.HK AAPL\", start=\"2025-01-01\", end=\"2025-06-30\")\n```\n\n<KeyPoint>yfinance 返回的数据包含 Open, High, Low, Close, Volume 等字段。其中，`Adj Close`（调整收盘价）是一个非常重要的字段。</KeyPoint>\n\n### 理解调整收盘价\n\n<Term id=\"adjusted_close\" en=\"Adjusted Close\">调整收盘价</Term> 和普通收盘价有什么区别？\n\n当公司分红或拆股时，股价会出现非市场因素的跳变。调整收盘价就是为了消除这些“噪音”，让历史价格具有可比性。\n\n<Example title=\"拆股对调整收盘价的影响\">\n假设一只股票进行 **2:1 拆股**，拆股前一天的收盘价是 100 元。拆股后，价格会瞬间变为 50 元。调整收盘价会把拆股前的价格也除以 2，调整为 50 元，保持价格序列的连续性。\n</Example>\n\n<KeyPoint>进行长期回测时，应该使用调整收盘价而不是普通收盘价，因为它能更准确地反映股票的真实长期表现。</KeyPoint>\n\n<QuestionRef id=\"q_quiz_adjusted_close_usage_v1\" />\n\n## 数据库管理：存储与查询金融数据 {#database-management}\n\n爬取到的数据不能每次都重新爬，需要存起来。数据库就是最好的选择。\n\n### 为什么使用数据库？\n\n- **高效存储与检索**：快速查询大量历史数据用于策略回测。\n- **避免重复爬取**：数据持久化到本地磁盘，无需每次都请求网站。\n- **数据完整性**：通过约束和事务保证数据一致性。\n- **高级查询能力**：支持复杂的 SQL 查询来筛选和分析数据。\n\n### SQLite 与 SQL 基础\n\nPython 内置了对 <Term id=\"sqlite\" en=\"SQLite\">SQLite</Term> 的支持。它是一个轻量级的数据库，不需要安装服务器，一个文件就是一个数据库。\n\n使用 <Term id=\"sql\" en=\"Structured Query Language\">SQL</Term> 语言来操作数据库。最基本的四个操作（CRUD）：\n\n| 操作 | SQL 命令 | 说明 |\n|------|----------|------|\n| 查询 | `SELECT` | 从表中检索数据 |\n| 插入 | `INSERT` | 向表中添加新数据 |\n| 更新 | `UPDATE` | 修改表中已有数据 |\n| 删除 | `DELETE` | 从表中移除数据 |\n\n<Example title=\"Python 操作 SQLite 的基本流程\">\n```python\nimport sqlite3\n\n# 1. 连接数据库（或创建）\nconn = sqlite3.connect('example.db')\ncursor = conn.cursor()\n\n# 2. 创建表\ncursor.execute('''\n    CREATE TABLE market_candles (\n        symbol text,\n        timestamp text,\n        open_price float,\n        close_price float,\n        volume float\n    )\n''')\n\n# 3. 插入数据\ncursor.execute(\n    \"INSERT INTO market_candles (symbol, timestamp, close_price) VALUES (?, ?, ?)\",\n    ('0700.HK', '2025-06-01', 500.0)\n)\n\n# 4. 提交事务并关闭\nconn.commit()\nconn.close()\n```\n</Example>\n\n<QuestionFamily familyId=\"qf_quiz_sqlite_connect\" />\n<QuestionFamily familyId=\"qf_quiz_sql_crud\" />\n\n## 金融数据存储的数据库设计 {#database-design}\n\n设计数据库时，一个常见的陷阱：把所有数据塞进一张大表。\n\n### 大表问题\n\n把所有股票的所有历史数据都放在一个表里，会导致：\n- 表体积非常巨大\n- 查询和备份都会很慢\n\n### 分区策略\n\n为了缓解大表问题，可以考虑分区：\n\n| 分区策略 | 优点 | 缺点 |\n|----------|------|------|\n| **按时间分区**（如每月一表） | 跨资产分析方便 | 跨月分析困难 |\n| **按股票分区**（如每只股票一表） | 单只股票回测快 | 跨资产对比分析困难 |\n\n<KeyPoint>没有适用于所有情况的完美数据库设计！关键在于根据你的主要用途来权衡。始终需要在**存储空间**、**运行速度**和**内存消耗**之间找到平衡。</KeyPoint>\n\n<QuestionRef id=\"q_quiz_big_table_issue_v1\" />\n<QuestionRef id=\"q_quiz_tradeoff_factors_v1\" />\n\n## 爬取的局限性与替代工具 {#limitations-and-tools}\n\n网络爬取并非万能，它有很多局限性：\n\n1.  **数据质量**：数据可能不完整或过时。\n2.  **资源消耗**：消耗大量带宽、存储和时间。\n3.  **技术挑战**：\n    - 网站结构经常变化，导致爬虫失效。\n    - 反爬机制，如 IP 封锁和频率限制。\n\n### 替代工具\n\n| 工具 | 用途 | 适用场景 |\n|------|------|----------|\n| <Term id=\"selenium\" en=\"Selenium\">Selenium</Term> | 模拟真实浏览器操作 | 动态加载的网页、需要点击交互的页面 |\n| <Term id=\"xpath\" en=\"XPath\">XPath</Term> + lxml | 用路径表达式精确定位元素 | 需要更灵活、更精确地定位元素 |\n\n<Example title=\"XPath 示例\">\nXPath 表达式 `//table[@id='constituents']//tr` 的含义是：在整个文档中，找到 id 属性等于 'constituents' 的 `<table>` 元素，然后在该元素下找到所有 `<tr>` 元素。这比 BeautifulSoup 的链式查找更灵活。\n</Example>\n\n<QuestionRef id=\"q_quiz_selenium_tool_v1\" />\n\n## 易错点 {#pitfalls}\n\n1.  **忘记设置 User-Agent**：直接请求有反爬机制的网站（如维基百科）会失败。始终记得在请求头中伪装成浏览器。\n2.  **混淆收盘价与调整收盘价**：进行长期回测时，务必使用调整收盘价，否则结果会被分红、拆股等事件扭曲。\n3.  **陷入“大表”陷阱**：将所有数据塞进一张表会导致严重的性能问题。应根据使用场景考虑分区策略。\n4.  **追求“完美”设计**：数据库设计没有银弹。不要试图找到一个适用于所有情况的方案，而应根据主要用途进行权衡。\n\n## 复习路线 {#review-path}\n\n1.  **核心概念**：复习 <Term id=\"web_scraping\" en=\"Web Scraping\">网络爬取</Term> 的定义和它在算法交易中的价值。\n2.  **网页结构**：回顾 HTML、CSS、JavaScript 三者的角色，以及为什么爬虫关注 HTML。\n3.  **爬取工具**：掌握 BeautifulSoup 的标准工作流程（Requests 下载 → BeautifulSoup 解析 → find/find_all 提取）。\n4.  **实战与可靠数据**：回顾爬取 S&P 500 时如何处理 User-Agent，以及如何使用 yfinance 获取数据。\n5.  **调整收盘价**：理解调整收盘价的概念和重要性。\n6.  **数据库管理**：复习为什么需要数据库，以及 SQL 的 CRUD 基本操作。\n7.  **数据库设计**：理解大表问题、分区策略，以及存储空间、运行速度、内存消耗之间的权衡。\n8.  **局限性与工具**：了解爬取的主要局限性，以及 Selenium 和 XPath 的适用场景。\n\n<Checkpoint title=\"最终自测\">\n<QuestionFamily familyId=\"qf_quiz_web_components_role\" />\n<QuestionFamily familyId=\"qf_quiz_bs4_workflow\" />\n<QuestionFamily familyId=\"qf_quiz_user_agent_fail\" />\n<QuestionFamily familyId=\"qf_quiz_adjusted_close_usage\" />\n<QuestionFamily familyId=\"qf_quiz_sql_crud\" />\n<QuestionFamily familyId=\"qf_quiz_tradeoff_factors\" />\n<QuestionFamily familyId=\"qf_quiz_selenium_tool\" />\n</Checkpoint>",
    "path": "research/pipeline/5-textbook/L2.mdx",
    "sections": [
      {
        "content": "",
        "level": 1,
        "section_id": "section_1",
        "title": "数据爬取与数据库管理"
      },
      {
        "content": "\n想象一下，你每天需要手动查看几百只股票的价格、新闻和财报。这几乎不可能，而且很容易出错。\n\n现在，让程序替你完成这些重复的工作。这就是 <Term id=\"web_scraping\" en=\"Web Scraping\">网络爬取</Term> 的核心思想。\n\n在 <Term id=\"algorithmic_trading\" en=\"Algorithmic Trading\">算法交易</Term> 中，数据就是燃料。没有数据，策略就是空谈。爬取的数据可以用于：\n\n- **实时数据收集**：自动获取股价、经济指标等。\n- **市场情绪分析**：爬取新闻、论坛和社交媒体内容。\n- **公司分析**：自动收集公司公告、财报、管理层变动等信息。\n- **另类数据**：收集天气、网络流量等非传统金融数据。\n\n<KeyPoint>网络爬取是算法交易数据 pipeline 的第一步，它让你能够自动化地、高效地从互联网获取结构化或非结构化数据，为后续的策略开发、回测和分析提供燃料。</KeyPoint>\n\n<QuestionRef id=\"q_quiz_web_scraping_use_case_v1\" />\n",
        "level": 2,
        "section_id": "section_2",
        "title": "为什么这一课重要 {#why-this-lesson-matters}"
      },
      {
        "content": "\n本课将带你走通一条完整的数据 pipeline：从理解网页结构，到使用 Python 工具爬取数据，再到将数据存入数据库并进行合理的设计。\n\n```mermaid\nflowchart LR\n    A[理解网页结构<br/>HTML / CSS / JavaScript] --> B[使用 Python 爬取数据<br/>BeautifulSoup / Requests]\n    B --> C[实战爬取与可靠数据源<br/>S&P 500 / yfinance]\n    C --> D[存储数据到数据库<br/>SQLite / SQL]\n    D --> E[数据库设计权衡<br/>大表 / 分区 / 权衡]\n    B --> F[爬取的局限性与替代工具<br/>Selenium / XPath]\n```\n\n<Checkpoint title=\"全景自测\">\n<QuestionRef id=\"q_quiz_web_scraping_use_case_v2\" />\n</Checkpoint>\n",
        "level": 2,
        "section_id": "section_3",
        "title": "课程全景与关键问题 {#concept-map}"
      },
      {
        "content": "",
        "level": 2,
        "section_id": "section_4",
        "title": "网络爬取基础：从网页结构到数据提取 {#web-scraping-foundations}"
      },
      {
        "content": "\n一个网页通常由三种技术共同构成：\n\n1.  <Term id=\"html\" en=\"HyperText Markup Language\">HTML</Term>：定义网页的结构和内容，如标题、段落、表格。它是一种**标记语言**，不是编程语言。\n2.  <Term id=\"css\" en=\"Cascading Style Sheets\">CSS</Term>：控制网页的视觉呈现和布局，如颜色、字体、间距。\n3.  <Term id=\"javascript\" en=\"JavaScript\">JavaScript</Term>：为网页添加交互和动态行为，如点击按钮后弹出信息。\n\n<KeyPoint>进行网络爬取时，我们最关心的是 HTML。因为目标数据（如股票价格、新闻文本）通常直接包含在 HTML 的标签里。CSS 和 JavaScript 主要控制样式和交互，不直接承载数据。</KeyPoint>\n\n<Definition title=\"HTML 的本质\">\nHTML 使用标签（Tags）来创建元素。例如，`<h1>` 定义一级标题，`<p>` 定义段落，`<a>` 定义超链接，`<table>` 定义表格。爬虫的核心任务就是解析这些标签，提取其中的数据。\n</Definition>\n\n<Figure src=\"images/9a0356d3c69aa54d62765a0ef2abff6400803e8e1063976ac685e8e80ce3ccd6.jpg\" alt=\"一个简单的 HTML 页面在浏览器中的渲染效果，展示了标题和段落\">\n一个简单的 HTML 页面示例。\n</Figure>\n\n<QuestionFamily familyId=\"qf_quiz_web_components_role\" />\n",
        "level": 3,
        "section_id": "section_5",
        "title": "网页的三层架构"
      },
      {
        "content": "\n<Term id=\"beautifulsoup\" en=\"BeautifulSoup\">BeautifulSoup</Term> 是 Python 中最流行的 HTML 解析库之一，它就像一个 HTML 的“瑞士军刀”。\n\n**标准工作流程：**\n\n1.  **下载网页**：使用 <Term id=\"requests\" en=\"Requests\">Requests</Term> 库下载网页的原始 HTML 内容。\n2.  **解析 HTML**：将原始内容交给 BeautifulSoup 解析，得到一个可操作的树状结构。\n\n```python\nimport requests\nfrom bs4 import BeautifulSoup\n",
        "level": 3,
        "section_id": "section_6",
        "title": "使用 BeautifulSoup 解析 HTML"
      },
      {
        "content": "url = \"https://example.com\"\nresponse = requests.get(url)\nhtml_content = response.text\n",
        "level": 1,
        "section_id": "section_7",
        "title": "1. 下载网页"
      },
      {
        "content": "soup = BeautifulSoup(html_content, 'html.parser')\n```\n\n**核心方法：**\n\n- `soup.prettify()`：打印出格式清晰的 HTML，方便检查结构。\n- `soup.get_text()`：提取网页里所有的纯文本，去掉所有 HTML 标签。\n- `soup.find('p')`：找到第一个 `<p>` 标签。\n- `soup.find_all('a')`：找到所有 `<a>` 标签，返回一个列表。\n- `soup.find('table')`：找到第一个 `<table>` 标签，可配合 pandas 转换为 DataFrame。\n\n<Example title=\"提取表格数据到 DataFrame\">\n```python\nimport pandas as pd\n",
        "level": 1,
        "section_id": "section_8",
        "title": "2. 解析 HTML"
      },
      {
        "content": "table = soup.find('table')\n",
        "level": 1,
        "section_id": "section_9",
        "title": "假设 soup 已经解析好"
      },
      {
        "content": "headers = [th.text for th in table.find_all('th')]\n",
        "level": 1,
        "section_id": "section_10",
        "title": "提取表头"
      },
      {
        "content": "rows = []\nfor tr in table.find_all('tr')[1:]:  # 跳过表头行\n    cells = tr.find_all('td')\n    row = [cell.text for cell in cells]\n    rows.append(row)\n",
        "level": 1,
        "section_id": "section_11",
        "title": "提取数据行"
      },
      {
        "content": "df = pd.DataFrame(rows, columns=headers)\nprint(df)\n```\n</Example>\n\n<QuestionFamily familyId=\"qf_quiz_bs4_workflow\" />\n<QuestionFamily familyId=\"qf_quiz_bs4_methods\" />\n<QuestionFamily familyId=\"qf_quiz_bs4_find_vs_findall\" />\n",
        "level": 1,
        "section_id": "section_12",
        "title": "创建 DataFrame"
      },
      {
        "content": "",
        "level": 2,
        "section_id": "section_13",
        "title": "实战爬取与可靠数据源 {#real-world-scraping}"
      },
      {
        "content": "\n让我们实战一下：爬取维基百科上的 S&P 500 成分股列表。\n\n**挑战：反爬机制**\n\n直接用 `requests.get(url)` 请求维基百科，可能会返回错误信息。这是因为很多网站会检查请求的 <Term id=\"user_agent\" en=\"User-Agent\">User-Agent</Term>。如果它看起来不像一个真实的浏览器，网站就会拒绝访问。\n\n<KeyPoint>解决方法很简单：在请求头中伪装成浏览器。设置一个常见的 User-Agent 字符串即可。</KeyPoint>\n\n```python\nheaders = {\n    \"user-agent\": \"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36\"\n}\nresponse = requests.get(url, headers=headers)\n```\n\n**定位目标表格**\n\n成功获取页面后，用 BeautifulSoup 找到目标表格。维基百科页面上有多个表格，可以通过表格的 `id` 属性来精确定位。\n\n```python\ntable = soup.find('table', id='constituents')\n```\n\n<QuestionRef id=\"q_quiz_user_agent_fail_v1\" />\n<QuestionRef id=\"q_quiz_table_id_v1\" />\n",
        "level": 3,
        "section_id": "section_14",
        "title": "实战：爬取 S&P 500 成分股"
      },
      {
        "content": "\n直接爬取 Yahoo Finance 会遇到很多反爬措施。好在有一个专门的库：<Term id=\"yfinance\" en=\"yfinance\">yfinance</Term>。它封装了所有复杂的请求，让你几行代码就能获取数据。\n\n```python\nimport yfinance as yf\n",
        "level": 3,
        "section_id": "section_15",
        "title": "使用 yfinance 获取可靠金融数据"
      },
      {
        "content": "ticker = yf.Ticker(\"0700.HK\")\nprint(ticker.info)\n",
        "level": 1,
        "section_id": "section_16",
        "title": "获取腾讯控股 (0700.HK) 的基本信息"
      },
      {
        "content": "data = ticker.history(period='1mo')\nprint(data)\n",
        "level": 1,
        "section_id": "section_17",
        "title": "获取最近一个月的股价历史"
      },
      {
        "content": "data = yf.download(\"0700.HK AAPL\", start=\"2025-01-01\", end=\"2025-06-30\")\n```\n\n<KeyPoint>yfinance 返回的数据包含 Open, High, Low, Close, Volume 等字段。其中，`Adj Close`（调整收盘价）是一个非常重要的字段。</KeyPoint>\n",
        "level": 1,
        "section_id": "section_18",
        "title": "同时获取多只股票"
      },
      {
        "content": "\n<Term id=\"adjusted_close\" en=\"Adjusted Close\">调整收盘价</Term> 和普通收盘价有什么区别？\n\n当公司分红或拆股时，股价会出现非市场因素的跳变。调整收盘价就是为了消除这些“噪音”，让历史价格具有可比性。\n\n<Example title=\"拆股对调整收盘价的影响\">\n假设一只股票进行 **2:1 拆股**，拆股前一天的收盘价是 100 元。拆股后，价格会瞬间变为 50 元。调整收盘价会把拆股前的价格也除以 2，调整为 50 元，保持价格序列的连续性。\n</Example>\n\n<KeyPoint>进行长期回测时，应该使用调整收盘价而不是普通收盘价，因为它能更准确地反映股票的真实长期表现。</KeyPoint>\n\n<QuestionRef id=\"q_quiz_adjusted_close_usage_v1\" />\n",
        "level": 3,
        "section_id": "section_19",
        "title": "理解调整收盘价"
      },
      {
        "content": "\n爬取到的数据不能每次都重新爬，需要存起来。数据库就是最好的选择。\n",
        "level": 2,
        "section_id": "section_20",
        "title": "数据库管理：存储与查询金融数据 {#database-management}"
      },
      {
        "content": "\n- **高效存储与检索**：快速查询大量历史数据用于策略回测。\n- **避免重复爬取**：数据持久化到本地磁盘，无需每次都请求网站。\n- **数据完整性**：通过约束和事务保证数据一致性。\n- **高级查询能力**：支持复杂的 SQL 查询来筛选和分析数据。\n",
        "level": 3,
        "section_id": "section_21",
        "title": "为什么使用数据库？"
      },
      {
        "content": "\nPython 内置了对 <Term id=\"sqlite\" en=\"SQLite\">SQLite</Term> 的支持。它是一个轻量级的数据库，不需要安装服务器，一个文件就是一个数据库。\n\n使用 <Term id=\"sql\" en=\"Structured Query Language\">SQL</Term> 语言来操作数据库。最基本的四个操作（CRUD）：\n\n| 操作 | SQL 命令 | 说明 |\n|------|----------|------|\n| 查询 | `SELECT` | 从表中检索数据 |\n| 插入 | `INSERT` | 向表中添加新数据 |\n| 更新 | `UPDATE` | 修改表中已有数据 |\n| 删除 | `DELETE` | 从表中移除数据 |\n\n<Example title=\"Python 操作 SQLite 的基本流程\">\n```python\nimport sqlite3\n",
        "level": 3,
        "section_id": "section_22",
        "title": "SQLite 与 SQL 基础"
      },
      {
        "content": "conn = sqlite3.connect('example.db')\ncursor = conn.cursor()\n",
        "level": 1,
        "section_id": "section_23",
        "title": "1. 连接数据库（或创建）"
      },
      {
        "content": "cursor.execute('''\n    CREATE TABLE market_candles (\n        symbol text,\n        timestamp text,\n        open_price float,\n        close_price float,\n        volume float\n    )\n''')\n",
        "level": 1,
        "section_id": "section_24",
        "title": "2. 创建表"
      },
      {
        "content": "cursor.execute(\n    \"INSERT INTO market_candles (symbol, timestamp, close_price) VALUES (?, ?, ?)\",\n    ('0700.HK', '2025-06-01', 500.0)\n)\n",
        "level": 1,
        "section_id": "section_25",
        "title": "3. 插入数据"
      },
      {
        "content": "conn.commit()\nconn.close()\n```\n</Example>\n\n<QuestionFamily familyId=\"qf_quiz_sqlite_connect\" />\n<QuestionFamily familyId=\"qf_quiz_sql_crud\" />\n",
        "level": 1,
        "section_id": "section_26",
        "title": "4. 提交事务并关闭"
      },
      {
        "content": "\n设计数据库时，一个常见的陷阱：把所有数据塞进一张大表。\n",
        "level": 2,
        "section_id": "section_27",
        "title": "金融数据存储的数据库设计 {#database-design}"
      },
      {
        "content": "\n把所有股票的所有历史数据都放在一个表里，会导致：\n- 表体积非常巨大\n- 查询和备份都会很慢\n",
        "level": 3,
        "section_id": "section_28",
        "title": "大表问题"
      },
      {
        "content": "\n为了缓解大表问题，可以考虑分区：\n\n| 分区策略 | 优点 | 缺点 |\n|----------|------|------|\n| **按时间分区**（如每月一表） | 跨资产分析方便 | 跨月分析困难 |\n| **按股票分区**（如每只股票一表） | 单只股票回测快 | 跨资产对比分析困难 |\n\n<KeyPoint>没有适用于所有情况的完美数据库设计！关键在于根据你的主要用途来权衡。始终需要在**存储空间**、**运行速度**和**内存消耗**之间找到平衡。</KeyPoint>\n\n<QuestionRef id=\"q_quiz_big_table_issue_v1\" />\n<QuestionRef id=\"q_quiz_tradeoff_factors_v1\" />\n",
        "level": 3,
        "section_id": "section_29",
        "title": "分区策略"
      },
      {
        "content": "\n网络爬取并非万能，它有很多局限性：\n\n1.  **数据质量**：数据可能不完整或过时。\n2.  **资源消耗**：消耗大量带宽、存储和时间。\n3.  **技术挑战**：\n    - 网站结构经常变化，导致爬虫失效。\n    - 反爬机制，如 IP 封锁和频率限制。\n",
        "level": 2,
        "section_id": "section_30",
        "title": "爬取的局限性与替代工具 {#limitations-and-tools}"
      },
      {
        "content": "\n| 工具 | 用途 | 适用场景 |\n|------|------|----------|\n| <Term id=\"selenium\" en=\"Selenium\">Selenium</Term> | 模拟真实浏览器操作 | 动态加载的网页、需要点击交互的页面 |\n| <Term id=\"xpath\" en=\"XPath\">XPath</Term> + lxml | 用路径表达式精确定位元素 | 需要更灵活、更精确地定位元素 |\n\n<Example title=\"XPath 示例\">\nXPath 表达式 `//table[@id='constituents']//tr` 的含义是：在整个文档中，找到 id 属性等于 'constituents' 的 `<table>` 元素，然后在该元素下找到所有 `<tr>` 元素。这比 BeautifulSoup 的链式查找更灵活。\n</Example>\n\n<QuestionRef id=\"q_quiz_selenium_tool_v1\" />\n",
        "level": 3,
        "section_id": "section_31",
        "title": "替代工具"
      },
      {
        "content": "\n1.  **忘记设置 User-Agent**：直接请求有反爬机制的网站（如维基百科）会失败。始终记得在请求头中伪装成浏览器。\n2.  **混淆收盘价与调整收盘价**：进行长期回测时，务必使用调整收盘价，否则结果会被分红、拆股等事件扭曲。\n3.  **陷入“大表”陷阱**：将所有数据塞进一张表会导致严重的性能问题。应根据使用场景考虑分区策略。\n4.  **追求“完美”设计**：数据库设计没有银弹。不要试图找到一个适用于所有情况的方案，而应根据主要用途进行权衡。\n",
        "level": 2,
        "section_id": "section_32",
        "title": "易错点 {#pitfalls}"
      },
      {
        "content": "\n1.  **核心概念**：复习 <Term id=\"web_scraping\" en=\"Web Scraping\">网络爬取</Term> 的定义和它在算法交易中的价值。\n2.  **网页结构**：回顾 HTML、CSS、JavaScript 三者的角色，以及为什么爬虫关注 HTML。\n3.  **爬取工具**：掌握 BeautifulSoup 的标准工作流程（Requests 下载 → BeautifulSoup 解析 → find/find_all 提取）。\n4.  **实战与可靠数据**：回顾爬取 S&P 500 时如何处理 User-Agent，以及如何使用 yfinance 获取数据。\n5.  **调整收盘价**：理解调整收盘价的概念和重要性。\n6.  **数据库管理**：复习为什么需要数据库，以及 SQL 的 CRUD 基本操作。\n7.  **数据库设计**：理解大表问题、分区策略，以及存储空间、运行速度、内存消耗之间的权衡。\n8.  **局限性与工具**：了解爬取的主要局限性，以及 Selenium 和 XPath 的适用场景。\n\n<Checkpoint title=\"最终自测\">\n<QuestionFamily familyId=\"qf_quiz_web_components_role\" />\n<QuestionFamily familyId=\"qf_quiz_bs4_workflow\" />\n<QuestionFamily familyId=\"qf_quiz_user_agent_fail\" />\n<QuestionFamily familyId=\"qf_quiz_adjusted_close_usage\" />\n<QuestionFamily familyId=\"qf_quiz_sql_crud\" />\n<QuestionFamily familyId=\"qf_quiz_tradeoff_factors\" />\n<QuestionFamily familyId=\"qf_quiz_selenium_tool\" />\n</Checkpoint>",
        "level": 2,
        "section_id": "section_33",
        "title": "复习路线 {#review-path}"
      }
    ]
  }
}
</SCORER_INPUT_JSON>
