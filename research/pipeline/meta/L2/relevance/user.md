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
        "concept": "MVP lesson slice",
        "file": "research/pipeline/3-guided_story/L2/step1/step.json",
        "sequence_id": "step1"
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
            "coverage_tag": "basic_website_structure",
            "covered_by": [
              "qf_flash_html_tags",
              "qf_flash_web_tech_roles",
              "qf_long_web_structure_explain"
            ],
            "description": "理解网页基本结构：HTML、CSS、JavaScript 的角色"
          },
          {
            "coverage_tag": "web_scraping_libraries",
            "covered_by": [
              "qf_flash_bs4_methods",
              "qf_flash_xpath_syntax",
              "qf_flash_selenium_purpose",
              "qf_flash_yfinance_usage",
              "qf_long_scraping_tool_choice",
              "qf_long_scrape_sp500"
            ],
            "description": "使用不同 Python 库进行网页抓取：BeautifulSoup, lxml/XPath, Selenium, yfinance"
          },
          {
            "coverage_tag": "web_scraping_limitations",
            "covered_by": [
              "qf_flash_scraping_limits",
              "qf_long_scraping_limitations_explain"
            ],
            "description": "理解网页抓取的局限性：数据质量、资源消耗、技术挑战"
          },
          {
            "coverage_tag": "sqlite_database",
            "covered_by": [
              "qf_flash_sqlite_crud",
              "qf_long_sqlite_crud_example"
            ],
            "description": "使用 Python 创建和管理 SQLite 数据库：CRUD 操作"
          },
          {
            "coverage_tag": "adjusted_close_price",
            "covered_by": [
              "qf_flash_adjusted_close_concept",
              "qf_long_adjusted_close_calculation"
            ],
            "description": "理解调整收盘价的概念：分红、拆股对价格的影响及调整方法"
          },
          {
            "coverage_tag": "database_design_financial_data",
            "covered_by": [
              "qf_flash_db_design_tradeoffs",
              "qf_long_db_design_analysis"
            ],
            "description": "金融数据存储的数据库设计考量：分区策略、权衡"
          }
        ],
        "flashcard_families": [
          {
            "concept_key": "html_structure",
            "coverage_tags": [
              "basic_website_structure"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_html_tags",
            "learning_goal": "学生能识别常见 HTML 标签的基本用途。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "term_match",
            "term_refs": [
              {
                "display": "HTML",
                "en": "HyperText Markup Language"
              },
              {
                "display": "标题标签",
                "en": "<title>"
              },
              {
                "display": "段落标签",
                "en": "<p>"
              },
              {
                "display": "超链接标签",
                "en": "<a>"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 8,
                "explanation": "<title> 标签位于 <head> 中，定义网页标题，显示在浏览器标签栏。",
                "options": [
                  "<body>",
                  "<title>",
                  "<h1>",
                  "<p>"
                ],
                "question_id": "q_flash_html_tags_v1",
                "stem": "以下哪个 HTML 标签用于定义网页的标题，并显示在浏览器标签栏上？"
              },
              {
                "answer": 2,
                "estimated_seconds": 8,
                "explanation": "<a> 标签用于创建超链接，href 属性指定链接目标。",
                "options": [
                  "<img>",
                  "<div>",
                  "<a>",
                  "<span>"
                ],
                "question_id": "q_flash_html_tags_v2",
                "stem": "以下哪个 HTML 标签用于创建一个超链接？"
              },
              {
                "answer": 2,
                "estimated_seconds": 6,
                "explanation": "<p> 标签用于定义段落文本。",
                "options": [
                  "<h1>",
                  "<li>",
                  "<p>",
                  "<td>"
                ],
                "question_id": "q_flash_html_tags_v3",
                "stem": "以下哪个 HTML 标签用于定义一个段落？"
              }
            ]
          },
          {
            "concept_key": "web_technologies",
            "coverage_tags": [
              "basic_website_structure"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_web_tech_roles",
            "learning_goal": "学生能区分 HTML、CSS、JavaScript 在网页中的不同作用。",
            "linked_steps": [
              "step1"
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
                "answer": 1,
                "estimated_seconds": 10,
                "explanation": "CSS（层叠样式表）用于控制网页的视觉呈现和布局，如颜色、字体等。",
                "options": [
                  "HTML",
                  "CSS",
                  "JavaScript",
                  "Python"
                ],
                "question_id": "q_flash_web_tech_roles_v1",
                "stem": "在网页的三大基础技术中，负责控制颜色、字体和布局的是哪一种？"
              },
              {
                "answer": 0,
                "estimated_seconds": 10,
                "explanation": "HTML（超文本标记语言）用于定义网页的结构和内容。",
                "options": [
                  "HTML",
                  "CSS",
                  "JavaScript",
                  "SQL"
                ],
                "question_id": "q_flash_web_tech_roles_v2",
                "stem": "在网页的三大基础技术中，负责定义网页结构和内容（如标题、段落、表格）的是哪一种？"
              }
            ]
          },
          {
            "concept_key": "beautifulsoup_usage",
            "coverage_tags": [
              "web_scraping_libraries"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_bs4_methods",
            "learning_goal": "学生能正确选择 BeautifulSoup 的方法来提取特定元素。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "BeautifulSoup",
                "en": "BeautifulSoup"
              },
              {
                "display": "find",
                "en": "find()"
              },
              {
                "display": "find_all",
                "en": "find_all()"
              }
            ],
            "variants": [
              {
                "answer": 0,
                "estimated_seconds": 10,
                "explanation": "find('p') 返回第一个匹配的 <p> 标签。",
                "options": [
                  "soup.find('p')",
                  "soup.find_all('p')",
                  "soup.select('p')",
                  "soup.get_text()"
                ],
                "question_id": "q_flash_bs4_methods_v1",
                "stem": "使用 BeautifulSoup 解析 HTML 后，要提取页面上第一个 <p> 标签，应该使用哪个方法？"
              },
              {
                "answer": 1,
                "estimated_seconds": 10,
                "explanation": "find_all('a') 返回所有匹配的 <a> 标签列表。",
                "options": [
                  "soup.find('a')",
                  "soup.find_all('a')",
                  "soup.get_text()",
                  "soup.prettify()"
                ],
                "question_id": "q_flash_bs4_methods_v2",
                "stem": "使用 BeautifulSoup 解析 HTML 后，要提取页面上所有 <a> 标签，应该使用哪个方法？"
              }
            ]
          },
          {
            "concept_key": "xpath_usage",
            "coverage_tags": [
              "web_scraping_libraries"
            ],
            "difficulty": "medium",
            "family_id": "qf_flash_xpath_syntax",
            "learning_goal": "学生能理解 XPath 的基本语法和用途。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "single_choice",
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
                "answer": 1,
                "estimated_seconds": 12,
                "explanation": "`//li[@class='item']` 选择文档中所有 class 属性值为 'item' 的 <li> 元素。",
                "options": [
                  "选择所有 <li> 元素",
                  "选择 class 属性为 'item' 的所有 <li> 元素",
                  "选择第一个 <li> 元素",
                  "选择所有 <li> 元素的父元素"
                ],
                "question_id": "q_flash_xpath_syntax_v1",
                "stem": "XPath 表达式 `//li[@class='item']` 的作用是什么？"
              },
              {
                "answer": 1,
                "estimated_seconds": 12,
                "explanation": "`//table[@id='constituents']` 选择文档中 id 属性值为 'constituents' 的 <table> 元素。",
                "options": [
                  "选择所有 <table> 元素",
                  "选择 id 为 'constituents' 的 <table> 元素",
                  "选择第一个 <table> 元素",
                  "选择所有 <table> 元素的子元素"
                ],
                "question_id": "q_flash_xpath_syntax_v2",
                "stem": "XPath 表达式 `//table[@id='constituents']` 的作用是什么？"
              }
            ]
          },
          {
            "concept_key": "selenium_usage",
            "coverage_tags": [
              "web_scraping_libraries"
            ],
            "difficulty": "medium",
            "family_id": "qf_flash_selenium_purpose",
            "learning_goal": "学生能理解 Selenium 在网页抓取中的适用场景。",
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
                "answer": 2,
                "estimated_seconds": 10,
                "explanation": "Selenium 模拟真实浏览器，能执行 JavaScript，适合抓取动态加载的内容。",
                "options": [
                  "BeautifulSoup",
                  "lxml",
                  "Selenium",
                  "requests"
                ],
                "question_id": "q_flash_selenium_purpose_v1",
                "stem": "当目标网页的数据是通过 JavaScript 动态加载时，以下哪个工具最适合抓取？"
              },
              {
                "answer": 1,
                "estimated_seconds": 8,
                "explanation": "Selenium 是一个网页自动化工具，可以模拟用户操作，如点击、滚动等。",
                "options": [
                  "解析 HTML 和 XML 文档",
                  "模拟用户操作，自动化网页交互",
                  "发送 HTTP 请求",
                  "计算调整收盘价"
                ],
                "question_id": "q_flash_selenium_purpose_v2",
                "stem": "Selenium 的主要功能是什么？"
              }
            ]
          },
          {
            "concept_key": "yfinance_usage",
            "coverage_tags": [
              "web_scraping_libraries"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_yfinance_usage",
            "learning_goal": "学生能理解 yfinance 库的基本用法和适用场景。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "yfinance",
                "en": "yfinance"
              },
              {
                "display": "Ticker",
                "en": "Ticker"
              }
            ],
            "variants": [
              {
                "answer": 0,
                "estimated_seconds": 10,
                "explanation": "先创建 Ticker 对象，再调用 history(period=\"1mo\") 获取最近一个月的历史数据。",
                "options": [
                  "yf.Ticker(\"0700.HK\").history(period=\"1mo\")",
                  "yf.download(\"0700.HK\", period=\"1mo\")",
                  "yf.Ticker(\"0700.HK\").info()",
                  "yf.history(\"0700.HK\", period=\"1mo\")"
                ],
                "question_id": "q_flash_yfinance_usage_v1",
                "stem": "使用 yfinance 获取腾讯控股（0700.HK）最近一个月的股价历史数据，正确的代码是？"
              },
              {
                "answer": 1,
                "estimated_seconds": 8,
                "explanation": "yfinance 是一个非官方的 Python 库，用于从 Yahoo Finance 获取金融数据。",
                "options": [
                  "Google Finance",
                  "Yahoo Finance",
                  "Bloomberg",
                  "Reuters"
                ],
                "question_id": "q_flash_yfinance_usage_v2",
                "stem": "yfinance 库主要用于从哪个数据源获取金融数据？"
              }
            ]
          },
          {
            "concept_key": "scraping_limitations",
            "coverage_tags": [
              "web_scraping_limitations"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_scraping_limits",
            "learning_goal": "学生能识别网页抓取的主要局限性。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "true_false",
            "term_refs": [
              {
                "display": "网页抓取",
                "en": "Web Scraping"
              }
            ],
            "variants": [
              {
                "answer": 0,
                "estimated_seconds": 8,
                "explanation": "网站结构变化是网页抓取的技术挑战之一，需要维护抓取代码。",
                "options": [
                  "正确",
                  "错误"
                ],
                "question_id": "q_flash_scraping_limits_v1",
                "stem": "网页抓取的一个局限性是：网站结构可能频繁变化，导致抓取代码失效。"
              },
              {
                "answer": 1,
                "estimated_seconds": 8,
                "explanation": "抓取大量数据会消耗大量带宽和存储空间，这是资源密集型操作。",
                "options": [
                  "正确",
                  "错误"
                ],
                "question_id": "q_flash_scraping_limits_v2",
                "stem": "网页抓取不会消耗带宽和存储资源。"
              }
            ]
          },
          {
            "concept_key": "adjusted_close",
            "coverage_tags": [
              "adjusted_close_price"
            ],
            "difficulty": "medium",
            "family_id": "qf_flash_adjusted_close_concept",
            "learning_goal": "学生能理解调整收盘价的核心作用。",
            "linked_steps": [
              "step1"
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
                "answer": 1,
                "estimated_seconds": 10,
                "explanation": "调整收盘价向后修正了历史价格，消除分红、拆股等公司事件的影响，使长期回测更准确。",
                "options": [
                  "反映当天最后一笔交易价格",
                  "消除分红、拆股等事件对历史价格的影响",
                  "显示最高价和最低价的平均值",
                  "用于计算交易量"
                ],
                "question_id": "q_flash_adjusted_close_concept_v1",
                "stem": "调整收盘价（Adj Close）的主要作用是什么？"
              },
              {
                "answer": 1,
                "estimated_seconds": 10,
                "explanation": "2:1 拆股后，拆股前的价格会乘以 0.5 进行调整，以保持价格连续性。",
                "options": [
                  "保持不变",
                  "乘以 0.5",
                  "乘以 2",
                  "加上拆股差价"
                ],
                "question_id": "q_flash_adjusted_close_concept_v2",
                "stem": "一只股票在 2:1 拆股后，调整收盘价会如何调整拆股前的历史价格？"
              }
            ]
          },
          {
            "concept_key": "sqlite_crud",
            "coverage_tags": [
              "sqlite_database"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_sqlite_crud",
            "learning_goal": "学生能识别 SQLite 的基本 CRUD 操作语法。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "SQLite",
                "en": "SQLite"
              },
              {
                "display": "INSERT",
                "en": "INSERT"
              },
              {
                "display": "SELECT",
                "en": "SELECT"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 8,
                "explanation": "INSERT INTO 语句用于向表中添加新数据。",
                "options": [
                  "SELECT * FROM users",
                  "INSERT INTO users (name, age) VALUES ('Amy', 18)",
                  "UPDATE users SET age=28 WHERE name='Amy'",
                  "DELETE FROM users WHERE name='Amy'"
                ],
                "question_id": "q_flash_sqlite_crud_v1",
                "stem": "在 SQLite 中，向 users 表插入一条新记录的 SQL 语句是？"
              },
              {
                "answer": 0,
                "estimated_seconds": 8,
                "explanation": "SELECT ... WHERE 用于条件查询数据。",
                "options": [
                  "SELECT * FROM users WHERE age > 25",
                  "SELECT * FROM users",
                  "INSERT INTO users VALUES (age > 25)",
                  "DELETE FROM users WHERE age > 25"
                ],
                "question_id": "q_flash_sqlite_crud_v2",
                "stem": "在 SQLite 中，从 users 表中查询所有年龄大于 25 的用户的 SQL 语句是？"
              }
            ]
          },
          {
            "concept_key": "database_design_tradeoffs",
            "coverage_tags": [
              "database_design_financial_data"
            ],
            "difficulty": "medium",
            "family_id": "qf_flash_db_design_tradeoffs",
            "learning_goal": "学生能理解金融数据库设计中的不同分区策略及其适用场景。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "数据库设计",
                "en": "Database Design"
              },
              {
                "display": "分区",
                "en": "Partitioning"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 10,
                "explanation": "按股票分表可以快速读取单只股票的完整历史，适合回测。",
                "options": [
                  "所有股票放在同一张大表",
                  "按股票代码分表",
                  "按日期（如每月）分表",
                  "不分区，每次全量扫描"
                ],
                "question_id": "q_flash_db_design_tradeoffs_v1",
                "stem": "对于策略回测场景，推荐的数据分区方式是？"
              },
              {
                "answer": 2,
                "estimated_seconds": 10,
                "explanation": "所有股票放在同一张大表便于进行跨股票的数据聚合和分析。",
                "options": [
                  "按股票代码分表",
                  "按日期（如每月）分表",
                  "所有股票放在同一张大表",
                  "每只股票一个独立数据库"
                ],
                "question_id": "q_flash_db_design_tradeoffs_v2",
                "stem": "对于跨股票的数据分析场景，哪种数据库设计更灵活？"
              }
            ]
          }
        ],
        "lesson_id": "L2",
        "longform_families": [
          {
            "concept_key": "web_structure_explanation",
            "coverage_tags": [
              "basic_website_structure"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_web_structure_explain",
            "learning_goal": "学生能用自己的语言解释 HTML、CSS、JavaScript 在网页中的角色，并说明为什么理解 HTML 结构对网页抓取至关重要。",
            "linked_steps": [
              "step1"
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
                "estimated_seconds": 90,
                "prompt_blocks": [
                  "HTML 的角色",
                  "CSS 的角色",
                  "JavaScript 的角色",
                  "与抓取的关系"
                ],
                "question_id": "q_long_web_structure_explain_v1",
                "reference_answer": [
                  "HTML 定义网页的结构和内容，如标题、段落、表格。",
                  "CSS 控制网页的视觉呈现，如颜色、字体、布局。",
                  "JavaScript 为网页添加交互和动态行为。",
                  "网页抓取的核心是从 HTML 中提取数据，因为数据通常嵌入在 HTML 标签中。CSS 和 JavaScript 主要影响呈现和交互，不直接包含数据。理解 HTML 结构（如标签、属性、层级）是定位和提取目标数据的基础。"
                ],
                "rubric_points": [
                  "正确指出 HTML 定义结构和内容",
                  "正确指出 CSS 控制样式和布局",
                  "正确指出 JavaScript 添加交互和动态行为",
                  "解释抓取的核心是解析 HTML 结构以提取数据，CSS 和 JS 影响不大或需要特殊处理"
                ],
                "stem": "请简要解释 HTML、CSS 和 JavaScript 在网页中各扮演什么角色。为什么在网页抓取中，理解 HTML 结构是核心？"
              },
              {
                "estimated_seconds": 90,
                "prompt_blocks": [
                  "定位表格的方法",
                  "利用 HTML 结构",
                  "CSS/JS 的非必要性"
                ],
                "question_id": "q_long_web_structure_explain_v2",
                "reference_answer": [
                  "我会先找到包含数据的 <table> 标签。如果表格有唯一的 id 属性（如 id='constituents'），可以直接用 soup.find('table', id='constituents') 或 XPath //table[@id='constituents'] 定位。",
                  "然后遍历 <tr> 和 <td> 标签提取每一行的数据。",
                  "CSS 只负责表格的样式（如边框、颜色），不影响表格内的数据内容，所以不需要理解 CSS。",
                  "如果表格是静态 HTML 的一部分（不是通过 JavaScript 动态加载的），那么 JavaScript 知识也不是必需的。"
                ],
                "rubric_points": [
                  "提到使用 <table> 标签定位",
                  "提到利用 id、class 等属性精确定位",
                  "提到使用 find('table') 或 XPath 等方法",
                  "解释 CSS 只控制样式，不影响数据内容",
                  "解释如果数据不是 JS 动态加载的，则无需 JS 知识"
                ],
                "stem": "假设你要抓取一个包含股票代码和公司名称的 HTML 表格。请说明你会如何利用 HTML 的结构知识（如标签、属性）来定位这个表格，并解释为什么 CSS 和 JavaScript 的知识在这种情况下不是必需的。"
              }
            ]
          },
          {
            "concept_key": "scraping_tool_comparison",
            "coverage_tags": [
              "web_scraping_libraries"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_scraping_tool_choice",
            "learning_goal": "学生能比较不同抓取工具的适用场景，并根据需求选择合适的工具。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "compare_and_contrast",
            "term_refs": [
              {
                "display": "BeautifulSoup",
                "en": "BeautifulSoup"
              },
              {
                "display": "Selenium",
                "en": "Selenium"
              },
              {
                "display": "yfinance",
                "en": "yfinance"
              },
              {
                "display": "lxml",
                "en": "lxml"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "BeautifulSoup 适用场景",
                  "Selenium 适用场景",
                  "yfinance 的优势"
                ],
                "question_id": "q_long_scraping_tool_choice_v1",
                "reference_answer": [
                  "BeautifulSoup 适用于网页内容在 HTML 源码中直接可获取的静态页面，解析速度快，使用简单。",
                  "Selenium 适用于网页内容通过 JavaScript 动态加载的场景，它模拟真实浏览器，能执行 JS 并等待内容加载完成，但速度较慢，资源消耗大。",
                  "yfinance 是专门为 Yahoo Finance 设计的库，封装了数据请求和解析过程。当需要获取股票历史价格、公司信息等金融数据时，使用 yfinance 比自己用 BeautifulSoup 或 Selenium 抓取 Yahoo Finance 更简单、稳定，因为它自动处理了数据格式和反爬机制。"
                ],
                "rubric_points": [
                  "指出 BeautifulSoup 适合解析静态 HTML",
                  "指出 Selenium 适合处理 JavaScript 动态加载的内容",
                  "指出 yfinance 专门为 Yahoo Finance 设计，简化了金融数据获取",
                  "提到 yfinance 无需处理反爬、登录等问题"
                ],
                "stem": "比较 BeautifulSoup 和 Selenium 在网页抓取中的不同适用场景。什么情况下你会选择使用 yfinance 而不是自己编写抓取代码？"
              },
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "抓取 Wikipedia 列表",
                  "获取 Yahoo Finance 历史价格",
                  "工具选择理由"
                ],
                "question_id": "q_long_scraping_tool_choice_v2",
                "reference_answer": [
                  "抓取 Wikipedia 的 S&P 500 列表：我会使用 BeautifulSoup（配合 requests）。因为 Wikipedia 页面是静态 HTML，表格结构清晰，使用 find('table', id='constituents') 即可定位并提取数据。",
                  "获取历史价格：我会使用 yfinance。因为 Yahoo Finance 的页面数据通过 JavaScript 动态加载，且有反爬机制。yfinance 封装了这些复杂性，可以直接通过 Ticker.history() 获取干净的数据。"
                ],
                "rubric_points": [
                  "Wikipedia 使用 BeautifulSoup 或 lxml 即可",
                  "Yahoo Finance 使用 yfinance",
                  "解释 Wikipedia 是静态页面，适合 BeautifulSoup",
                  "解释 Yahoo Finance 有反爬机制，yfinance 是更稳定的选择"
                ],
                "stem": "假设你需要从 Wikipedia 抓取一个静态的 S&P 500 成分股列表，并从 Yahoo Finance 获取每只股票的历史价格。请分别说明你会使用什么工具，并解释原因。"
              }
            ]
          },
          {
            "concept_key": "practical_scraping",
            "coverage_tags": [
              "web_scraping_libraries"
            ],
            "difficulty": "hard",
            "family_id": "qf_long_scrape_sp500",
            "learning_goal": "学生能描述使用 BeautifulSoup 和 pandas 从 Wikipedia 抓取 S&P 500 成分股列表并存入 DataFrame 的完整步骤。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "worked_example",
            "term_refs": [
              {
                "display": "BeautifulSoup",
                "en": "BeautifulSoup"
              },
              {
                "display": "pandas",
                "en": "pandas"
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
                  "发送请求并设置 User-Agent",
                  "解析 HTML 并定位表格",
                  "提取表头和数据行",
                  "创建 DataFrame"
                ],
                "question_id": "q_long_scrape_sp500_v1",
                "reference_answer": [
                  "1. 导入 requests, BeautifulSoup, pandas。",
                  "2. 设置 headers 包含 User-Agent，模拟浏览器访问。",
                  "3. 使用 requests.get(url, headers=headers) 获取页面内容。",
                  "4. 使用 BeautifulSoup(text, 'html.parser') 解析 HTML。",
                  "5. 使用 soup.find('table', id='constituents') 定位目标表格。",
                  "6. 遍历 table.find_all('th') 提取表头文本，存入列表。",
                  "7. 遍历 table.find_all('tr')[1:]（跳过表头行），对每一行遍历 find_all('td') 提取单元格文本，存入列表。",
                  "8. 使用 pd.DataFrame(data, columns=headers) 创建 DataFrame。"
                ],
                "rubric_points": [
                  "提到使用 requests.get() 并设置合适的 User-Agent 头",
                  "提到使用 BeautifulSoup 解析 HTML",
                  "提到使用 find('table', id='constituents') 定位目标表格",
                  "提到遍历 <th> 提取表头",
                  "提到遍历 <tr> 和 <td> 提取数据行",
                  "提到使用 pandas DataFrame 存储数据"
                ],
                "stem": "请描述使用 Python 的 requests、BeautifulSoup 和 pandas 从 Wikipedia 的 S&P 500 页面抓取成分股列表并存入 DataFrame 的完整步骤。重点说明如何定位正确的表格以及如何处理表头和行数据。"
              },
              {
                "estimated_seconds": 90,
                "prompt_blocks": [
                  "User-Agent 的作用",
                  "定位正确表格的方法",
                  "代码示例"
                ],
                "question_id": "q_long_scrape_sp500_v2",
                "reference_answer": [
                  "设置 User-Agent 是为了让服务器认为请求来自真实的浏览器，而不是爬虫程序。许多网站（包括 Wikipedia）会检查 User-Agent，没有设置可能会被拒绝访问。",
                  "Wikipedia 的 S&P 500 页面包含多个表格，但成分股列表的表格有唯一的 id 属性 'constituents'。通过 id 定位可以确保抓取到正确的表格。",
                  "定位代码：table = soup.find('table', id='constituents')"
                ],
                "rubric_points": [
                  "解释 User-Agent 用于模拟浏览器，避免被服务器拒绝",
                  "提到使用表格的唯一 id 属性进行定位",
                  "写出正确的定位代码：soup.find('table', id='constituents')"
                ],
                "stem": "在使用 BeautifulSoup 抓取 Wikipedia 的 S&P 500 页面时，为什么需要设置 User-Agent？如果页面中有多个表格，如何确保抓取到正确的那个？请写出定位该表格的代码。"
              }
            ]
          },
          {
            "concept_key": "scraping_limitations_detail",
            "coverage_tags": [
              "web_scraping_limitations"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_scraping_limitations_explain",
            "learning_goal": "学生能解释网页抓取的主要局限性及其对实际项目的影响。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "short_explain",
            "term_refs": [
              {
                "display": "网页抓取",
                "en": "Web Scraping"
              },
              {
                "display": "反爬措施",
                "en": "Anti-bot measures"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 90,
                "prompt_blocks": [
                  "数据质量",
                  "资源消耗",
                  "技术挑战",
                  "对长期项目的影响"
                ],
                "question_id": "q_long_scraping_limitations_explain_v1",
                "reference_answer": [
                  "1. 数据质量：网站数据可能不完整、不及时，影响分析准确性。",
                  "2. 资源密集型：抓取大量数据消耗大量带宽和存储空间，且加载页面耗时。",
                  "3. 技术挑战：网站结构可能频繁变化，导致抓取代码失效；许多网站实施反爬措施（如 IP 封锁、速率限制）。",
                  "对长期项目的影响：需要持续监控和维护抓取代码，应对网站变化；反爬措施可能导致数据获取中断；存储和带宽成本会随着数据量增长而增加。"
                ],
                "rubric_points": [
                  "指出数据可能不完整或过时",
                  "指出抓取大量数据消耗带宽和存储",
                  "指出网站结构变化和反爬措施",
                  "说明这些因素导致需要持续维护、增加成本、可能数据中断"
                ],
                "stem": "请列举并简要解释网页抓取的三个主要局限性。这些局限性如何影响一个需要长期运行的金融数据抓取项目？"
              },
              {
                "estimated_seconds": 90,
                "prompt_blocks": [
                  "技术挑战 1",
                  "技术挑战 2",
                  "缓解策略"
                ],
                "question_id": "q_long_scraping_limitations_explain_v2",
                "reference_answer": [
                  "技术挑战 1：网站结构可能频繁变化，导致基于特定标签或 class 的定位代码失效。",
                  "缓解策略：使用更通用的定位方式，如 XPath 基于文本内容或元素层级定位；定期检查并更新抓取代码。",
                  "技术挑战 2：网站可能实施反爬措施，如 IP 封锁或请求频率限制。",
                  "缓解策略：使用代理 IP 轮换；在请求之间设置随机延迟，模拟人类浏览行为；设置合适的 User-Agent 和请求头。"
                ],
                "rubric_points": [
                  "提到网站结构变化",
                  "提到反爬措施（如 IP 封锁、速率限制）",
                  "提出使用更健壮的定位方式（如 XPath）",
                  "提出使用代理 IP、设置请求间隔、模拟浏览器行为"
                ],
                "stem": "假设你正在抓取一个财经新闻网站用于市场情绪分析。请说明你可能遇到的两个技术挑战，并提出相应的缓解策略。"
              }
            ]
          },
          {
            "concept_key": "adjusted_close_mechanism",
            "coverage_tags": [
              "adjusted_close_price"
            ],
            "difficulty": "hard",
            "family_id": "qf_long_adjusted_close_calculation",
            "learning_goal": "学生能解释调整收盘价的计算机制，并手动计算简单的调整案例。",
            "linked_steps": [
              "step1"
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
                  "拆股调整因子",
                  "分红调整因子",
                  "计算 2/17 调整收盘价",
                  "调整的意义"
                ],
                "question_id": "q_long_adjusted_close_calculation_v1",
                "reference_answer": [
                  "拆股调整因子：2:1 拆股，因子 = 1/2 = 0.5",
                  "分红调整因子：1 - (0.08 / 24.96) = 1 - 0.0032 = 0.9968",
                  "2/17 调整收盘价 = 0.5 * 0.9968 * 48.30 = 24.07",
                  "调整的意义：拆股和分红会导致股价出现非市场因素的跳变。调整后的价格消除了这些事件的影响，使历史价格序列连续，能够真实反映股票的价值变化，对于长期回测和趋势分析至关重要。"
                ],
                "rubric_points": [
                  "正确计算拆股调整因子为 0.5",
                  "正确计算分红调整因子为 (1 - 0.08/24.96) = 0.9968",
                  "正确计算调整收盘价 = 0.5 * 0.9968 * 48.30 = 24.07",
                  "解释调整是为了消除公司事件对历史价格的影响，使价格序列连续，便于回测和分析"
                ],
                "stem": "一只股票在 2/18 进行 2:1 拆股，2/21 除息（分红 $0.08），2/18 的收盘价为 $24.96。请计算 2/17 的调整收盘价（2/17 收盘价为 $48.30）。并解释为什么需要调整历史价格。"
              },
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "分红调整因子计算",
                  "计算调整收盘价",
                  "调整收盘价 vs 原始收盘价"
                ],
                "question_id": "q_long_adjusted_close_calculation_v2",
                "reference_answer": [
                  "分红调整因子：1 - (2.40 / 16.51) = 1 - 0.1454 = 0.8546",
                  "5/11 调整收盘价 = 0.8546 * 16.51 = 14.11",
                  "原始收盘价在除息日会因分红派发而下跌，这个下跌不代表市场价值损失，而是公司价值的转移。如果使用原始收盘价进行回测，会错误地认为股价下跌，导致策略表现被低估。",
                  "调整收盘价通过向后调整历史价格，消除了分红事件造成的价格缺口，使价格序列反映的是股票的真实市场表现，因此更适合用于长期回测和绩效分析。"
                ],
                "rubric_points": [
                  "正确计算分红调整因子为 (1 - 2.40/16.51) = 0.8546",
                  "正确计算调整收盘价 = 0.8546 * 16.51 = 14.11",
                  "解释原始收盘价在除息日会突然下跌，造成价格缺口",
                  "解释调整收盘价消除了这种缺口，使价格序列平滑，回测结果更准确"
                ],
                "stem": "一只股票在 5/12 除息（分红 $2.40），5/11 的收盘价为 $16.51。请计算 5/11 的调整收盘价。并解释为什么调整收盘价更适合用于长期回测，而原始收盘价可能误导分析。"
              }
            ]
          },
          {
            "concept_key": "sqlite_crud_python",
            "coverage_tags": [
              "sqlite_database"
            ],
            "difficulty": "hard",
            "family_id": "qf_long_sqlite_crud_example",
            "learning_goal": "学生能写出使用 Python 的 sqlite3 模块进行数据库 CRUD 操作的代码。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "worked_example",
            "term_refs": [
              {
                "display": "SQLite",
                "en": "SQLite"
              },
              {
                "display": "cursor",
                "en": "cursor"
              },
              {
                "display": "execute",
                "en": "execute()"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "连接数据库",
                  "创建表",
                  "插入数据",
                  "查询数据"
                ],
                "question_id": "q_long_sqlite_crud_example_v1",
                "reference_answer": [
                  "import sqlite3",
                  "conn = sqlite3.connect('market.db')",
                  "cursor = conn.cursor()",
                  "cursor.execute('''CREATE TABLE IF NOT EXISTS stocks (symbol TEXT, date TEXT, close_price REAL)''')",
                  "cursor.execute(\"INSERT INTO stocks (symbol, date, close_price) VALUES (?, ?, ?)\", ('0700.HK', '2025-01-02', 412.4))",
                  "conn.commit()",
                  "cursor.execute(\"SELECT * FROM stocks WHERE symbol = ?\", ('0700.HK',))",
                  "for row in cursor.fetchall(): print(row)",
                  "conn.close()"
                ],
                "rubric_points": [
                  "正确使用 sqlite3.connect('market.db')",
                  "正确使用 cursor.execute() 执行 CREATE TABLE 语句",
                  "正确使用参数化查询插入数据",
                  "正确使用 SELECT 语句查询并遍历结果"
                ],
                "stem": "请写出使用 Python 的 sqlite3 模块完成以下操作的代码：1) 连接（或创建）一个名为 'market.db' 的数据库；2) 创建一个名为 'stocks' 的表，包含 symbol (TEXT), date (TEXT), close_price (REAL) 三个字段；3) 插入一条记录 ('0700.HK', '2025-01-02', 412.4)；4) 查询并打印所有 symbol 为 '0700.HK' 的记录。"
              },
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "更新数据",
                  "删除数据",
                  "查询并排序"
                ],
                "question_id": "q_long_sqlite_crud_example_v2",
                "reference_answer": [
                  "import sqlite3",
                  "conn = sqlite3.connect('market.db')",
                  "cursor = conn.cursor()",
                  "# 更新数据",
                  "cursor.execute(\"UPDATE market_candles SET close_price = ? WHERE symbol = ? AND timestamp = ?\", (195.5, 'AAPL', '2025-06-01'))",
                  "# 删除数据",
                  "cursor.execute(\"DELETE FROM market_candles WHERE timestamp < ?\", ('2024-01-01',))",
                  "# 查询并排序",
                  "cursor.execute(\"SELECT * FROM market_candles WHERE symbol = ? ORDER BY timestamp ASC\", ('AAPL',))",
                  "for row in cursor.fetchall(): print(row)",
                  "conn.commit()",
                  "conn.close()"
                ],
                "rubric_points": [
                  "正确使用 UPDATE 语句和参数化查询",
                  "正确使用 DELETE 语句",
                  "正确使用 SELECT 语句和 ORDER BY 子句",
                  "正确提交事务和关闭连接"
                ],
                "stem": "假设你有一个名为 'market_candles' 的表，包含 symbol, timestamp, close_price 等字段。请写出 Python 代码：1) 更新 symbol 为 'AAPL' 且 timestamp 为 '2025-06-01' 的记录，将 close_price 改为 195.5；2) 删除所有 timestamp 早于 '2024-01-01' 的记录；3) 查询并打印所有 symbol 为 'AAPL' 的记录，按 timestamp 升序排列。"
              }
            ]
          },
          {
            "concept_key": "database_design_strategies",
            "coverage_tags": [
              "database_design_financial_data"
            ],
            "difficulty": "hard",
            "family_id": "qf_long_db_design_analysis",
            "learning_goal": "学生能分析不同数据库设计策略的优缺点，并根据使用场景提出合理的设计方案。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "compare_and_contrast",
            "term_refs": [
              {
                "display": "数据库设计",
                "en": "Database Design"
              },
              {
                "display": "分区",
                "en": "Partitioning"
              },
              {
                "display": "回测",
                "en": "Backtesting"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "设计 1 优缺点",
                  "设计 2 优缺点",
                  "设计 3 优缺点",
                  "推荐方案及理由"
                ],
                "question_id": "q_long_db_design_analysis_v1",
                "reference_answer": [
                  "设计 1（同一张大表）：优点是结构简单，易于跨股票查询；缺点是表会非常大，查询和备份速度慢。",
                  "设计 2（按月分表）：优点是按时间范围查询快，便于月度数据管理；缺点是跨月查询需要动态拼接 SQL，跨股票分析复杂。",
                  "设计 3（按股票分表）：优点是单只股票的回测查询非常快；缺点是跨股票分析需要查询多张表，代码复杂。",
                  "推荐方案：对于既需要回测又需要跨股票分析的团队，可以考虑使用设计 1（同一张大表），但建立合适的索引（如 symbol + date 复合索引）来加速查询。或者使用按股票分表（设计 3），但创建一个视图或汇总表来支持跨股票分析。没有完美设计，需要根据实际使用频率权衡。"
                ],
                "rubric_points": [
                  "指出设计 1 简单但查询大表速度慢",
                  "指出设计 2 便于按时间查询但跨月分析复杂",
                  "指出设计 3 便于单股票回测但跨股票分析复杂",
                  "提出折中方案，如按股票分区但保留索引，或使用混合策略"
                ],
                "stem": "比较以下三种金融数据库设计策略的优缺点：1) 所有股票数据放在同一张大表；2) 按月份分表；3) 按股票代码分表。对于一个既需要进行单只股票回测，又需要进行跨股票分析的量化研究团队，你会推荐哪种设计？请说明理由。"
              },
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "按股票分表分析",
                  "按日期分表分析",
                  "权衡与决策"
                ],
                "question_id": "q_long_db_design_analysis_v2",
                "reference_answer": [
                  "按股票分表：优点是单只股票的历史数据查询极快，适合回测；缺点是表数量庞大（50,000 张），管理复杂，跨股票查询需要遍历多张表。",
                  "按日期分表：优点是按时间范围（如某个月）查询所有股票的数据很快，适合时间序列分析；缺点是跨年查询需要合并多张表，单只股票的完整历史查询较慢。",
                  "权衡：存储空间上两者差异不大。如果团队主要进行单股票回测，按股票分表更优；如果主要进行跨股票的截面分析，按日期分表更优。",
                  "折中方案：可以按股票分区（如使用数据库的分区表功能），在逻辑上是一张表，物理上按股票存储，兼顾查询灵活性和性能。"
                ],
                "rubric_points": [
                  "分析按股票分表：单股票查询快，但表数量多，跨股票查询复杂",
                  "分析按日期分表：按时间查询快，但跨时间查询复杂",
                  "提到存储空间两者相近",
                  "提出根据主要使用场景选择，或使用混合分区策略"
                ],
                "stem": "假设你需要设计一个数据库来存储全球 50,000 只股票的历史日线数据（从 2000 年至今）。请分析按股票分表和按日期分表两种策略的优缺点，并说明你会如何权衡存储空间、查询速度和维护成本。"
              }
            ]
          }
        ],
        "source": {
          "coverage_checklist": "L2: Data scraping and database management with Python - Agenda items: basic website structure, web scraping libraries, limitations, SQLite, adjusted price, database design",
          "guided_story_manifest": "pipeline/3-guided_story/manifest.json",
          "lesson_map": "{\"lesson_id\":\"L2\",\"steps\":[{\"concept\":\"MVP lesson slice\",\"sequence_id\":\"step1\"}]}",
          "plain_text": "pipeline/1-plain/L2/plain.txt",
          "related": [
            "pipeline/2-related_important/course_desc.md"
          ],
          "source_outline": "L2: Data scraping and database management with Python - Agenda items: basic website structure, web scraping libraries, limitations, SQLite, adjusted price, database design"
        },
        "target_language": "zh-CN"
      },
      "question_bank_path": "research/pipeline/3-guided_story/L2/step1/question_bank.json",
      "question_families": [
        {
          "family_id": "qf_flash_html_tags",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "html_structure",
            "coverage_tags": [
              "basic_website_structure"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_html_tags",
            "learning_goal": "学生能识别常见 HTML 标签的基本用途。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "term_match",
            "term_refs": [
              {
                "display": "HTML",
                "en": "HyperText Markup Language"
              },
              {
                "display": "标题标签",
                "en": "<title>"
              },
              {
                "display": "段落标签",
                "en": "<p>"
              },
              {
                "display": "超链接标签",
                "en": "<a>"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 8,
                "explanation": "<title> 标签位于 <head> 中，定义网页标题，显示在浏览器标签栏。",
                "options": [
                  "<body>",
                  "<title>",
                  "<h1>",
                  "<p>"
                ],
                "question_id": "q_flash_html_tags_v1",
                "stem": "以下哪个 HTML 标签用于定义网页的标题，并显示在浏览器标签栏上？"
              },
              {
                "answer": 2,
                "estimated_seconds": 8,
                "explanation": "<a> 标签用于创建超链接，href 属性指定链接目标。",
                "options": [
                  "<img>",
                  "<div>",
                  "<a>",
                  "<span>"
                ],
                "question_id": "q_flash_html_tags_v2",
                "stem": "以下哪个 HTML 标签用于创建一个超链接？"
              },
              {
                "answer": 2,
                "estimated_seconds": 6,
                "explanation": "<p> 标签用于定义段落文本。",
                "options": [
                  "<h1>",
                  "<li>",
                  "<p>",
                  "<td>"
                ],
                "question_id": "q_flash_html_tags_v3",
                "stem": "以下哪个 HTML 标签用于定义一个段落？"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_web_tech_roles",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "web_technologies",
            "coverage_tags": [
              "basic_website_structure"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_web_tech_roles",
            "learning_goal": "学生能区分 HTML、CSS、JavaScript 在网页中的不同作用。",
            "linked_steps": [
              "step1"
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
                "answer": 1,
                "estimated_seconds": 10,
                "explanation": "CSS（层叠样式表）用于控制网页的视觉呈现和布局，如颜色、字体等。",
                "options": [
                  "HTML",
                  "CSS",
                  "JavaScript",
                  "Python"
                ],
                "question_id": "q_flash_web_tech_roles_v1",
                "stem": "在网页的三大基础技术中，负责控制颜色、字体和布局的是哪一种？"
              },
              {
                "answer": 0,
                "estimated_seconds": 10,
                "explanation": "HTML（超文本标记语言）用于定义网页的结构和内容。",
                "options": [
                  "HTML",
                  "CSS",
                  "JavaScript",
                  "SQL"
                ],
                "question_id": "q_flash_web_tech_roles_v2",
                "stem": "在网页的三大基础技术中，负责定义网页结构和内容（如标题、段落、表格）的是哪一种？"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_bs4_methods",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "beautifulsoup_usage",
            "coverage_tags": [
              "web_scraping_libraries"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_bs4_methods",
            "learning_goal": "学生能正确选择 BeautifulSoup 的方法来提取特定元素。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "BeautifulSoup",
                "en": "BeautifulSoup"
              },
              {
                "display": "find",
                "en": "find()"
              },
              {
                "display": "find_all",
                "en": "find_all()"
              }
            ],
            "variants": [
              {
                "answer": 0,
                "estimated_seconds": 10,
                "explanation": "find('p') 返回第一个匹配的 <p> 标签。",
                "options": [
                  "soup.find('p')",
                  "soup.find_all('p')",
                  "soup.select('p')",
                  "soup.get_text()"
                ],
                "question_id": "q_flash_bs4_methods_v1",
                "stem": "使用 BeautifulSoup 解析 HTML 后，要提取页面上第一个 <p> 标签，应该使用哪个方法？"
              },
              {
                "answer": 1,
                "estimated_seconds": 10,
                "explanation": "find_all('a') 返回所有匹配的 <a> 标签列表。",
                "options": [
                  "soup.find('a')",
                  "soup.find_all('a')",
                  "soup.get_text()",
                  "soup.prettify()"
                ],
                "question_id": "q_flash_bs4_methods_v2",
                "stem": "使用 BeautifulSoup 解析 HTML 后，要提取页面上所有 <a> 标签，应该使用哪个方法？"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_xpath_syntax",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "xpath_usage",
            "coverage_tags": [
              "web_scraping_libraries"
            ],
            "difficulty": "medium",
            "family_id": "qf_flash_xpath_syntax",
            "learning_goal": "学生能理解 XPath 的基本语法和用途。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "single_choice",
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
                "answer": 1,
                "estimated_seconds": 12,
                "explanation": "`//li[@class='item']` 选择文档中所有 class 属性值为 'item' 的 <li> 元素。",
                "options": [
                  "选择所有 <li> 元素",
                  "选择 class 属性为 'item' 的所有 <li> 元素",
                  "选择第一个 <li> 元素",
                  "选择所有 <li> 元素的父元素"
                ],
                "question_id": "q_flash_xpath_syntax_v1",
                "stem": "XPath 表达式 `//li[@class='item']` 的作用是什么？"
              },
              {
                "answer": 1,
                "estimated_seconds": 12,
                "explanation": "`//table[@id='constituents']` 选择文档中 id 属性值为 'constituents' 的 <table> 元素。",
                "options": [
                  "选择所有 <table> 元素",
                  "选择 id 为 'constituents' 的 <table> 元素",
                  "选择第一个 <table> 元素",
                  "选择所有 <table> 元素的子元素"
                ],
                "question_id": "q_flash_xpath_syntax_v2",
                "stem": "XPath 表达式 `//table[@id='constituents']` 的作用是什么？"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_selenium_purpose",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "selenium_usage",
            "coverage_tags": [
              "web_scraping_libraries"
            ],
            "difficulty": "medium",
            "family_id": "qf_flash_selenium_purpose",
            "learning_goal": "学生能理解 Selenium 在网页抓取中的适用场景。",
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
                "answer": 2,
                "estimated_seconds": 10,
                "explanation": "Selenium 模拟真实浏览器，能执行 JavaScript，适合抓取动态加载的内容。",
                "options": [
                  "BeautifulSoup",
                  "lxml",
                  "Selenium",
                  "requests"
                ],
                "question_id": "q_flash_selenium_purpose_v1",
                "stem": "当目标网页的数据是通过 JavaScript 动态加载时，以下哪个工具最适合抓取？"
              },
              {
                "answer": 1,
                "estimated_seconds": 8,
                "explanation": "Selenium 是一个网页自动化工具，可以模拟用户操作，如点击、滚动等。",
                "options": [
                  "解析 HTML 和 XML 文档",
                  "模拟用户操作，自动化网页交互",
                  "发送 HTTP 请求",
                  "计算调整收盘价"
                ],
                "question_id": "q_flash_selenium_purpose_v2",
                "stem": "Selenium 的主要功能是什么？"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_yfinance_usage",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "yfinance_usage",
            "coverage_tags": [
              "web_scraping_libraries"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_yfinance_usage",
            "learning_goal": "学生能理解 yfinance 库的基本用法和适用场景。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "yfinance",
                "en": "yfinance"
              },
              {
                "display": "Ticker",
                "en": "Ticker"
              }
            ],
            "variants": [
              {
                "answer": 0,
                "estimated_seconds": 10,
                "explanation": "先创建 Ticker 对象，再调用 history(period=\"1mo\") 获取最近一个月的历史数据。",
                "options": [
                  "yf.Ticker(\"0700.HK\").history(period=\"1mo\")",
                  "yf.download(\"0700.HK\", period=\"1mo\")",
                  "yf.Ticker(\"0700.HK\").info()",
                  "yf.history(\"0700.HK\", period=\"1mo\")"
                ],
                "question_id": "q_flash_yfinance_usage_v1",
                "stem": "使用 yfinance 获取腾讯控股（0700.HK）最近一个月的股价历史数据，正确的代码是？"
              },
              {
                "answer": 1,
                "estimated_seconds": 8,
                "explanation": "yfinance 是一个非官方的 Python 库，用于从 Yahoo Finance 获取金融数据。",
                "options": [
                  "Google Finance",
                  "Yahoo Finance",
                  "Bloomberg",
                  "Reuters"
                ],
                "question_id": "q_flash_yfinance_usage_v2",
                "stem": "yfinance 库主要用于从哪个数据源获取金融数据？"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_scraping_limits",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "scraping_limitations",
            "coverage_tags": [
              "web_scraping_limitations"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_scraping_limits",
            "learning_goal": "学生能识别网页抓取的主要局限性。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "true_false",
            "term_refs": [
              {
                "display": "网页抓取",
                "en": "Web Scraping"
              }
            ],
            "variants": [
              {
                "answer": 0,
                "estimated_seconds": 8,
                "explanation": "网站结构变化是网页抓取的技术挑战之一，需要维护抓取代码。",
                "options": [
                  "正确",
                  "错误"
                ],
                "question_id": "q_flash_scraping_limits_v1",
                "stem": "网页抓取的一个局限性是：网站结构可能频繁变化，导致抓取代码失效。"
              },
              {
                "answer": 1,
                "estimated_seconds": 8,
                "explanation": "抓取大量数据会消耗大量带宽和存储空间，这是资源密集型操作。",
                "options": [
                  "正确",
                  "错误"
                ],
                "question_id": "q_flash_scraping_limits_v2",
                "stem": "网页抓取不会消耗带宽和存储资源。"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_adjusted_close_concept",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "adjusted_close",
            "coverage_tags": [
              "adjusted_close_price"
            ],
            "difficulty": "medium",
            "family_id": "qf_flash_adjusted_close_concept",
            "learning_goal": "学生能理解调整收盘价的核心作用。",
            "linked_steps": [
              "step1"
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
                "answer": 1,
                "estimated_seconds": 10,
                "explanation": "调整收盘价向后修正了历史价格，消除分红、拆股等公司事件的影响，使长期回测更准确。",
                "options": [
                  "反映当天最后一笔交易价格",
                  "消除分红、拆股等事件对历史价格的影响",
                  "显示最高价和最低价的平均值",
                  "用于计算交易量"
                ],
                "question_id": "q_flash_adjusted_close_concept_v1",
                "stem": "调整收盘价（Adj Close）的主要作用是什么？"
              },
              {
                "answer": 1,
                "estimated_seconds": 10,
                "explanation": "2:1 拆股后，拆股前的价格会乘以 0.5 进行调整，以保持价格连续性。",
                "options": [
                  "保持不变",
                  "乘以 0.5",
                  "乘以 2",
                  "加上拆股差价"
                ],
                "question_id": "q_flash_adjusted_close_concept_v2",
                "stem": "一只股票在 2:1 拆股后，调整收盘价会如何调整拆股前的历史价格？"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_sqlite_crud",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "sqlite_crud",
            "coverage_tags": [
              "sqlite_database"
            ],
            "difficulty": "easy",
            "family_id": "qf_flash_sqlite_crud",
            "learning_goal": "学生能识别 SQLite 的基本 CRUD 操作语法。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "SQLite",
                "en": "SQLite"
              },
              {
                "display": "INSERT",
                "en": "INSERT"
              },
              {
                "display": "SELECT",
                "en": "SELECT"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 8,
                "explanation": "INSERT INTO 语句用于向表中添加新数据。",
                "options": [
                  "SELECT * FROM users",
                  "INSERT INTO users (name, age) VALUES ('Amy', 18)",
                  "UPDATE users SET age=28 WHERE name='Amy'",
                  "DELETE FROM users WHERE name='Amy'"
                ],
                "question_id": "q_flash_sqlite_crud_v1",
                "stem": "在 SQLite 中，向 users 表插入一条新记录的 SQL 语句是？"
              },
              {
                "answer": 0,
                "estimated_seconds": 8,
                "explanation": "SELECT ... WHERE 用于条件查询数据。",
                "options": [
                  "SELECT * FROM users WHERE age > 25",
                  "SELECT * FROM users",
                  "INSERT INTO users VALUES (age > 25)",
                  "DELETE FROM users WHERE age > 25"
                ],
                "question_id": "q_flash_sqlite_crud_v2",
                "stem": "在 SQLite 中，从 users 表中查询所有年龄大于 25 的用户的 SQL 语句是？"
              }
            ]
          }
        },
        {
          "family_id": "qf_flash_db_design_tradeoffs",
          "kind": "flashcard_families",
          "summary": {
            "concept_key": "database_design_tradeoffs",
            "coverage_tags": [
              "database_design_financial_data"
            ],
            "difficulty": "medium",
            "family_id": "qf_flash_db_design_tradeoffs",
            "learning_goal": "学生能理解金融数据库设计中的不同分区策略及其适用场景。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "single_choice",
            "term_refs": [
              {
                "display": "数据库设计",
                "en": "Database Design"
              },
              {
                "display": "分区",
                "en": "Partitioning"
              }
            ],
            "variants": [
              {
                "answer": 1,
                "estimated_seconds": 10,
                "explanation": "按股票分表可以快速读取单只股票的完整历史，适合回测。",
                "options": [
                  "所有股票放在同一张大表",
                  "按股票代码分表",
                  "按日期（如每月）分表",
                  "不分区，每次全量扫描"
                ],
                "question_id": "q_flash_db_design_tradeoffs_v1",
                "stem": "对于策略回测场景，推荐的数据分区方式是？"
              },
              {
                "answer": 2,
                "estimated_seconds": 10,
                "explanation": "所有股票放在同一张大表便于进行跨股票的数据聚合和分析。",
                "options": [
                  "按股票代码分表",
                  "按日期（如每月）分表",
                  "所有股票放在同一张大表",
                  "每只股票一个独立数据库"
                ],
                "question_id": "q_flash_db_design_tradeoffs_v2",
                "stem": "对于跨股票的数据分析场景，哪种数据库设计更灵活？"
              }
            ]
          }
        },
        {
          "family_id": "qf_long_web_structure_explain",
          "kind": "longform_families",
          "summary": {
            "concept_key": "web_structure_explanation",
            "coverage_tags": [
              "basic_website_structure"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_web_structure_explain",
            "learning_goal": "学生能用自己的语言解释 HTML、CSS、JavaScript 在网页中的角色，并说明为什么理解 HTML 结构对网页抓取至关重要。",
            "linked_steps": [
              "step1"
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
                "estimated_seconds": 90,
                "prompt_blocks": [
                  "HTML 的角色",
                  "CSS 的角色",
                  "JavaScript 的角色",
                  "与抓取的关系"
                ],
                "question_id": "q_long_web_structure_explain_v1",
                "reference_answer": [
                  "HTML 定义网页的结构和内容，如标题、段落、表格。",
                  "CSS 控制网页的视觉呈现，如颜色、字体、布局。",
                  "JavaScript 为网页添加交互和动态行为。",
                  "网页抓取的核心是从 HTML 中提取数据，因为数据通常嵌入在 HTML 标签中。CSS 和 JavaScript 主要影响呈现和交互，不直接包含数据。理解 HTML 结构（如标签、属性、层级）是定位和提取目标数据的基础。"
                ],
                "rubric_points": [
                  "正确指出 HTML 定义结构和内容",
                  "正确指出 CSS 控制样式和布局",
                  "正确指出 JavaScript 添加交互和动态行为",
                  "解释抓取的核心是解析 HTML 结构以提取数据，CSS 和 JS 影响不大或需要特殊处理"
                ],
                "stem": "请简要解释 HTML、CSS 和 JavaScript 在网页中各扮演什么角色。为什么在网页抓取中，理解 HTML 结构是核心？"
              },
              {
                "estimated_seconds": 90,
                "prompt_blocks": [
                  "定位表格的方法",
                  "利用 HTML 结构",
                  "CSS/JS 的非必要性"
                ],
                "question_id": "q_long_web_structure_explain_v2",
                "reference_answer": [
                  "我会先找到包含数据的 <table> 标签。如果表格有唯一的 id 属性（如 id='constituents'），可以直接用 soup.find('table', id='constituents') 或 XPath //table[@id='constituents'] 定位。",
                  "然后遍历 <tr> 和 <td> 标签提取每一行的数据。",
                  "CSS 只负责表格的样式（如边框、颜色），不影响表格内的数据内容，所以不需要理解 CSS。",
                  "如果表格是静态 HTML 的一部分（不是通过 JavaScript 动态加载的），那么 JavaScript 知识也不是必需的。"
                ],
                "rubric_points": [
                  "提到使用 <table> 标签定位",
                  "提到利用 id、class 等属性精确定位",
                  "提到使用 find('table') 或 XPath 等方法",
                  "解释 CSS 只控制样式，不影响数据内容",
                  "解释如果数据不是 JS 动态加载的，则无需 JS 知识"
                ],
                "stem": "假设你要抓取一个包含股票代码和公司名称的 HTML 表格。请说明你会如何利用 HTML 的结构知识（如标签、属性）来定位这个表格，并解释为什么 CSS 和 JavaScript 的知识在这种情况下不是必需的。"
              }
            ]
          }
        },
        {
          "family_id": "qf_long_scraping_tool_choice",
          "kind": "longform_families",
          "summary": {
            "concept_key": "scraping_tool_comparison",
            "coverage_tags": [
              "web_scraping_libraries"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_scraping_tool_choice",
            "learning_goal": "学生能比较不同抓取工具的适用场景，并根据需求选择合适的工具。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "compare_and_contrast",
            "term_refs": [
              {
                "display": "BeautifulSoup",
                "en": "BeautifulSoup"
              },
              {
                "display": "Selenium",
                "en": "Selenium"
              },
              {
                "display": "yfinance",
                "en": "yfinance"
              },
              {
                "display": "lxml",
                "en": "lxml"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "BeautifulSoup 适用场景",
                  "Selenium 适用场景",
                  "yfinance 的优势"
                ],
                "question_id": "q_long_scraping_tool_choice_v1",
                "reference_answer": [
                  "BeautifulSoup 适用于网页内容在 HTML 源码中直接可获取的静态页面，解析速度快，使用简单。",
                  "Selenium 适用于网页内容通过 JavaScript 动态加载的场景，它模拟真实浏览器，能执行 JS 并等待内容加载完成，但速度较慢，资源消耗大。",
                  "yfinance 是专门为 Yahoo Finance 设计的库，封装了数据请求和解析过程。当需要获取股票历史价格、公司信息等金融数据时，使用 yfinance 比自己用 BeautifulSoup 或 Selenium 抓取 Yahoo Finance 更简单、稳定，因为它自动处理了数据格式和反爬机制。"
                ],
                "rubric_points": [
                  "指出 BeautifulSoup 适合解析静态 HTML",
                  "指出 Selenium 适合处理 JavaScript 动态加载的内容",
                  "指出 yfinance 专门为 Yahoo Finance 设计，简化了金融数据获取",
                  "提到 yfinance 无需处理反爬、登录等问题"
                ],
                "stem": "比较 BeautifulSoup 和 Selenium 在网页抓取中的不同适用场景。什么情况下你会选择使用 yfinance 而不是自己编写抓取代码？"
              },
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "抓取 Wikipedia 列表",
                  "获取 Yahoo Finance 历史价格",
                  "工具选择理由"
                ],
                "question_id": "q_long_scraping_tool_choice_v2",
                "reference_answer": [
                  "抓取 Wikipedia 的 S&P 500 列表：我会使用 BeautifulSoup（配合 requests）。因为 Wikipedia 页面是静态 HTML，表格结构清晰，使用 find('table', id='constituents') 即可定位并提取数据。",
                  "获取历史价格：我会使用 yfinance。因为 Yahoo Finance 的页面数据通过 JavaScript 动态加载，且有反爬机制。yfinance 封装了这些复杂性，可以直接通过 Ticker.history() 获取干净的数据。"
                ],
                "rubric_points": [
                  "Wikipedia 使用 BeautifulSoup 或 lxml 即可",
                  "Yahoo Finance 使用 yfinance",
                  "解释 Wikipedia 是静态页面，适合 BeautifulSoup",
                  "解释 Yahoo Finance 有反爬机制，yfinance 是更稳定的选择"
                ],
                "stem": "假设你需要从 Wikipedia 抓取一个静态的 S&P 500 成分股列表，并从 Yahoo Finance 获取每只股票的历史价格。请分别说明你会使用什么工具，并解释原因。"
              }
            ]
          }
        },
        {
          "family_id": "qf_long_scrape_sp500",
          "kind": "longform_families",
          "summary": {
            "concept_key": "practical_scraping",
            "coverage_tags": [
              "web_scraping_libraries"
            ],
            "difficulty": "hard",
            "family_id": "qf_long_scrape_sp500",
            "learning_goal": "学生能描述使用 BeautifulSoup 和 pandas 从 Wikipedia 抓取 S&P 500 成分股列表并存入 DataFrame 的完整步骤。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "worked_example",
            "term_refs": [
              {
                "display": "BeautifulSoup",
                "en": "BeautifulSoup"
              },
              {
                "display": "pandas",
                "en": "pandas"
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
                  "发送请求并设置 User-Agent",
                  "解析 HTML 并定位表格",
                  "提取表头和数据行",
                  "创建 DataFrame"
                ],
                "question_id": "q_long_scrape_sp500_v1",
                "reference_answer": [
                  "1. 导入 requests, BeautifulSoup, pandas。",
                  "2. 设置 headers 包含 User-Agent，模拟浏览器访问。",
                  "3. 使用 requests.get(url, headers=headers) 获取页面内容。",
                  "4. 使用 BeautifulSoup(text, 'html.parser') 解析 HTML。",
                  "5. 使用 soup.find('table', id='constituents') 定位目标表格。",
                  "6. 遍历 table.find_all('th') 提取表头文本，存入列表。",
                  "7. 遍历 table.find_all('tr')[1:]（跳过表头行），对每一行遍历 find_all('td') 提取单元格文本，存入列表。",
                  "8. 使用 pd.DataFrame(data, columns=headers) 创建 DataFrame。"
                ],
                "rubric_points": [
                  "提到使用 requests.get() 并设置合适的 User-Agent 头",
                  "提到使用 BeautifulSoup 解析 HTML",
                  "提到使用 find('table', id='constituents') 定位目标表格",
                  "提到遍历 <th> 提取表头",
                  "提到遍历 <tr> 和 <td> 提取数据行",
                  "提到使用 pandas DataFrame 存储数据"
                ],
                "stem": "请描述使用 Python 的 requests、BeautifulSoup 和 pandas 从 Wikipedia 的 S&P 500 页面抓取成分股列表并存入 DataFrame 的完整步骤。重点说明如何定位正确的表格以及如何处理表头和行数据。"
              },
              {
                "estimated_seconds": 90,
                "prompt_blocks": [
                  "User-Agent 的作用",
                  "定位正确表格的方法",
                  "代码示例"
                ],
                "question_id": "q_long_scrape_sp500_v2",
                "reference_answer": [
                  "设置 User-Agent 是为了让服务器认为请求来自真实的浏览器，而不是爬虫程序。许多网站（包括 Wikipedia）会检查 User-Agent，没有设置可能会被拒绝访问。",
                  "Wikipedia 的 S&P 500 页面包含多个表格，但成分股列表的表格有唯一的 id 属性 'constituents'。通过 id 定位可以确保抓取到正确的表格。",
                  "定位代码：table = soup.find('table', id='constituents')"
                ],
                "rubric_points": [
                  "解释 User-Agent 用于模拟浏览器，避免被服务器拒绝",
                  "提到使用表格的唯一 id 属性进行定位",
                  "写出正确的定位代码：soup.find('table', id='constituents')"
                ],
                "stem": "在使用 BeautifulSoup 抓取 Wikipedia 的 S&P 500 页面时，为什么需要设置 User-Agent？如果页面中有多个表格，如何确保抓取到正确的那个？请写出定位该表格的代码。"
              }
            ]
          }
        },
        {
          "family_id": "qf_long_scraping_limitations_explain",
          "kind": "longform_families",
          "summary": {
            "concept_key": "scraping_limitations_detail",
            "coverage_tags": [
              "web_scraping_limitations"
            ],
            "difficulty": "medium",
            "family_id": "qf_long_scraping_limitations_explain",
            "learning_goal": "学生能解释网页抓取的主要局限性及其对实际项目的影响。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "short_explain",
            "term_refs": [
              {
                "display": "网页抓取",
                "en": "Web Scraping"
              },
              {
                "display": "反爬措施",
                "en": "Anti-bot measures"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 90,
                "prompt_blocks": [
                  "数据质量",
                  "资源消耗",
                  "技术挑战",
                  "对长期项目的影响"
                ],
                "question_id": "q_long_scraping_limitations_explain_v1",
                "reference_answer": [
                  "1. 数据质量：网站数据可能不完整、不及时，影响分析准确性。",
                  "2. 资源密集型：抓取大量数据消耗大量带宽和存储空间，且加载页面耗时。",
                  "3. 技术挑战：网站结构可能频繁变化，导致抓取代码失效；许多网站实施反爬措施（如 IP 封锁、速率限制）。",
                  "对长期项目的影响：需要持续监控和维护抓取代码，应对网站变化；反爬措施可能导致数据获取中断；存储和带宽成本会随着数据量增长而增加。"
                ],
                "rubric_points": [
                  "指出数据可能不完整或过时",
                  "指出抓取大量数据消耗带宽和存储",
                  "指出网站结构变化和反爬措施",
                  "说明这些因素导致需要持续维护、增加成本、可能数据中断"
                ],
                "stem": "请列举并简要解释网页抓取的三个主要局限性。这些局限性如何影响一个需要长期运行的金融数据抓取项目？"
              },
              {
                "estimated_seconds": 90,
                "prompt_blocks": [
                  "技术挑战 1",
                  "技术挑战 2",
                  "缓解策略"
                ],
                "question_id": "q_long_scraping_limitations_explain_v2",
                "reference_answer": [
                  "技术挑战 1：网站结构可能频繁变化，导致基于特定标签或 class 的定位代码失效。",
                  "缓解策略：使用更通用的定位方式，如 XPath 基于文本内容或元素层级定位；定期检查并更新抓取代码。",
                  "技术挑战 2：网站可能实施反爬措施，如 IP 封锁或请求频率限制。",
                  "缓解策略：使用代理 IP 轮换；在请求之间设置随机延迟，模拟人类浏览行为；设置合适的 User-Agent 和请求头。"
                ],
                "rubric_points": [
                  "提到网站结构变化",
                  "提到反爬措施（如 IP 封锁、速率限制）",
                  "提出使用更健壮的定位方式（如 XPath）",
                  "提出使用代理 IP、设置请求间隔、模拟浏览器行为"
                ],
                "stem": "假设你正在抓取一个财经新闻网站用于市场情绪分析。请说明你可能遇到的两个技术挑战，并提出相应的缓解策略。"
              }
            ]
          }
        },
        {
          "family_id": "qf_long_adjusted_close_calculation",
          "kind": "longform_families",
          "summary": {
            "concept_key": "adjusted_close_mechanism",
            "coverage_tags": [
              "adjusted_close_price"
            ],
            "difficulty": "hard",
            "family_id": "qf_long_adjusted_close_calculation",
            "learning_goal": "学生能解释调整收盘价的计算机制，并手动计算简单的调整案例。",
            "linked_steps": [
              "step1"
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
                  "拆股调整因子",
                  "分红调整因子",
                  "计算 2/17 调整收盘价",
                  "调整的意义"
                ],
                "question_id": "q_long_adjusted_close_calculation_v1",
                "reference_answer": [
                  "拆股调整因子：2:1 拆股，因子 = 1/2 = 0.5",
                  "分红调整因子：1 - (0.08 / 24.96) = 1 - 0.0032 = 0.9968",
                  "2/17 调整收盘价 = 0.5 * 0.9968 * 48.30 = 24.07",
                  "调整的意义：拆股和分红会导致股价出现非市场因素的跳变。调整后的价格消除了这些事件的影响，使历史价格序列连续，能够真实反映股票的价值变化，对于长期回测和趋势分析至关重要。"
                ],
                "rubric_points": [
                  "正确计算拆股调整因子为 0.5",
                  "正确计算分红调整因子为 (1 - 0.08/24.96) = 0.9968",
                  "正确计算调整收盘价 = 0.5 * 0.9968 * 48.30 = 24.07",
                  "解释调整是为了消除公司事件对历史价格的影响，使价格序列连续，便于回测和分析"
                ],
                "stem": "一只股票在 2/18 进行 2:1 拆股，2/21 除息（分红 $0.08），2/18 的收盘价为 $24.96。请计算 2/17 的调整收盘价（2/17 收盘价为 $48.30）。并解释为什么需要调整历史价格。"
              },
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "分红调整因子计算",
                  "计算调整收盘价",
                  "调整收盘价 vs 原始收盘价"
                ],
                "question_id": "q_long_adjusted_close_calculation_v2",
                "reference_answer": [
                  "分红调整因子：1 - (2.40 / 16.51) = 1 - 0.1454 = 0.8546",
                  "5/11 调整收盘价 = 0.8546 * 16.51 = 14.11",
                  "原始收盘价在除息日会因分红派发而下跌，这个下跌不代表市场价值损失，而是公司价值的转移。如果使用原始收盘价进行回测，会错误地认为股价下跌，导致策略表现被低估。",
                  "调整收盘价通过向后调整历史价格，消除了分红事件造成的价格缺口，使价格序列反映的是股票的真实市场表现，因此更适合用于长期回测和绩效分析。"
                ],
                "rubric_points": [
                  "正确计算分红调整因子为 (1 - 2.40/16.51) = 0.8546",
                  "正确计算调整收盘价 = 0.8546 * 16.51 = 14.11",
                  "解释原始收盘价在除息日会突然下跌，造成价格缺口",
                  "解释调整收盘价消除了这种缺口，使价格序列平滑，回测结果更准确"
                ],
                "stem": "一只股票在 5/12 除息（分红 $2.40），5/11 的收盘价为 $16.51。请计算 5/11 的调整收盘价。并解释为什么调整收盘价更适合用于长期回测，而原始收盘价可能误导分析。"
              }
            ]
          }
        },
        {
          "family_id": "qf_long_sqlite_crud_example",
          "kind": "longform_families",
          "summary": {
            "concept_key": "sqlite_crud_python",
            "coverage_tags": [
              "sqlite_database"
            ],
            "difficulty": "hard",
            "family_id": "qf_long_sqlite_crud_example",
            "learning_goal": "学生能写出使用 Python 的 sqlite3 模块进行数据库 CRUD 操作的代码。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "worked_example",
            "term_refs": [
              {
                "display": "SQLite",
                "en": "SQLite"
              },
              {
                "display": "cursor",
                "en": "cursor"
              },
              {
                "display": "execute",
                "en": "execute()"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "连接数据库",
                  "创建表",
                  "插入数据",
                  "查询数据"
                ],
                "question_id": "q_long_sqlite_crud_example_v1",
                "reference_answer": [
                  "import sqlite3",
                  "conn = sqlite3.connect('market.db')",
                  "cursor = conn.cursor()",
                  "cursor.execute('''CREATE TABLE IF NOT EXISTS stocks (symbol TEXT, date TEXT, close_price REAL)''')",
                  "cursor.execute(\"INSERT INTO stocks (symbol, date, close_price) VALUES (?, ?, ?)\", ('0700.HK', '2025-01-02', 412.4))",
                  "conn.commit()",
                  "cursor.execute(\"SELECT * FROM stocks WHERE symbol = ?\", ('0700.HK',))",
                  "for row in cursor.fetchall(): print(row)",
                  "conn.close()"
                ],
                "rubric_points": [
                  "正确使用 sqlite3.connect('market.db')",
                  "正确使用 cursor.execute() 执行 CREATE TABLE 语句",
                  "正确使用参数化查询插入数据",
                  "正确使用 SELECT 语句查询并遍历结果"
                ],
                "stem": "请写出使用 Python 的 sqlite3 模块完成以下操作的代码：1) 连接（或创建）一个名为 'market.db' 的数据库；2) 创建一个名为 'stocks' 的表，包含 symbol (TEXT), date (TEXT), close_price (REAL) 三个字段；3) 插入一条记录 ('0700.HK', '2025-01-02', 412.4)；4) 查询并打印所有 symbol 为 '0700.HK' 的记录。"
              },
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "更新数据",
                  "删除数据",
                  "查询并排序"
                ],
                "question_id": "q_long_sqlite_crud_example_v2",
                "reference_answer": [
                  "import sqlite3",
                  "conn = sqlite3.connect('market.db')",
                  "cursor = conn.cursor()",
                  "# 更新数据",
                  "cursor.execute(\"UPDATE market_candles SET close_price = ? WHERE symbol = ? AND timestamp = ?\", (195.5, 'AAPL', '2025-06-01'))",
                  "# 删除数据",
                  "cursor.execute(\"DELETE FROM market_candles WHERE timestamp < ?\", ('2024-01-01',))",
                  "# 查询并排序",
                  "cursor.execute(\"SELECT * FROM market_candles WHERE symbol = ? ORDER BY timestamp ASC\", ('AAPL',))",
                  "for row in cursor.fetchall(): print(row)",
                  "conn.commit()",
                  "conn.close()"
                ],
                "rubric_points": [
                  "正确使用 UPDATE 语句和参数化查询",
                  "正确使用 DELETE 语句",
                  "正确使用 SELECT 语句和 ORDER BY 子句",
                  "正确提交事务和关闭连接"
                ],
                "stem": "假设你有一个名为 'market_candles' 的表，包含 symbol, timestamp, close_price 等字段。请写出 Python 代码：1) 更新 symbol 为 'AAPL' 且 timestamp 为 '2025-06-01' 的记录，将 close_price 改为 195.5；2) 删除所有 timestamp 早于 '2024-01-01' 的记录；3) 查询并打印所有 symbol 为 'AAPL' 的记录，按 timestamp 升序排列。"
              }
            ]
          }
        },
        {
          "family_id": "qf_long_db_design_analysis",
          "kind": "longform_families",
          "summary": {
            "concept_key": "database_design_strategies",
            "coverage_tags": [
              "database_design_financial_data"
            ],
            "difficulty": "hard",
            "family_id": "qf_long_db_design_analysis",
            "learning_goal": "学生能分析不同数据库设计策略的优缺点，并根据使用场景提出合理的设计方案。",
            "linked_steps": [
              "step1"
            ],
            "question_type": "compare_and_contrast",
            "term_refs": [
              {
                "display": "数据库设计",
                "en": "Database Design"
              },
              {
                "display": "分区",
                "en": "Partitioning"
              },
              {
                "display": "回测",
                "en": "Backtesting"
              }
            ],
            "variants": [
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "设计 1 优缺点",
                  "设计 2 优缺点",
                  "设计 3 优缺点",
                  "推荐方案及理由"
                ],
                "question_id": "q_long_db_design_analysis_v1",
                "reference_answer": [
                  "设计 1（同一张大表）：优点是结构简单，易于跨股票查询；缺点是表会非常大，查询和备份速度慢。",
                  "设计 2（按月分表）：优点是按时间范围查询快，便于月度数据管理；缺点是跨月查询需要动态拼接 SQL，跨股票分析复杂。",
                  "设计 3（按股票分表）：优点是单只股票的回测查询非常快；缺点是跨股票分析需要查询多张表，代码复杂。",
                  "推荐方案：对于既需要回测又需要跨股票分析的团队，可以考虑使用设计 1（同一张大表），但建立合适的索引（如 symbol + date 复合索引）来加速查询。或者使用按股票分表（设计 3），但创建一个视图或汇总表来支持跨股票分析。没有完美设计，需要根据实际使用频率权衡。"
                ],
                "rubric_points": [
                  "指出设计 1 简单但查询大表速度慢",
                  "指出设计 2 便于按时间查询但跨月分析复杂",
                  "指出设计 3 便于单股票回测但跨股票分析复杂",
                  "提出折中方案，如按股票分区但保留索引，或使用混合策略"
                ],
                "stem": "比较以下三种金融数据库设计策略的优缺点：1) 所有股票数据放在同一张大表；2) 按月份分表；3) 按股票代码分表。对于一个既需要进行单只股票回测，又需要进行跨股票分析的量化研究团队，你会推荐哪种设计？请说明理由。"
              },
              {
                "estimated_seconds": 120,
                "prompt_blocks": [
                  "按股票分表分析",
                  "按日期分表分析",
                  "权衡与决策"
                ],
                "question_id": "q_long_db_design_analysis_v2",
                "reference_answer": [
                  "按股票分表：优点是单只股票的历史数据查询极快，适合回测；缺点是表数量庞大（50,000 张），管理复杂，跨股票查询需要遍历多张表。",
                  "按日期分表：优点是按时间范围（如某个月）查询所有股票的数据很快，适合时间序列分析；缺点是跨年查询需要合并多张表，单只股票的完整历史查询较慢。",
                  "权衡：存储空间上两者差异不大。如果团队主要进行单股票回测，按股票分表更优；如果主要进行跨股票的截面分析，按日期分表更优。",
                  "折中方案：可以按股票分区（如使用数据库的分区表功能），在逻辑上是一张表，物理上按股票存储，兼顾查询灵活性和性能。"
                ],
                "rubric_points": [
                  "分析按股票分表：单股票查询快，但表数量多，跨股票查询复杂",
                  "分析按日期分表：按时间查询快，但跨时间查询复杂",
                  "提到存储空间两者相近",
                  "提出根据主要使用场景选择，或使用混合分区策略"
                ],
                "stem": "假设你需要设计一个数据库来存储全球 50,000 只股票的历史日线数据（从 2000 年至今）。请分析按股票分表和按日期分表两种策略的优缺点，并说明你会如何权衡存储空间、查询速度和维护成本。"
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
              "想象一下，你正在监控 500 家公司的股价。",
              "手动刷新网页？太慢了。"
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
              "你需要一个自动化的方式，从网站上提取数据。",
              "这就是 <term id=\"web_scraping\">网页抓取</term>。"
            ],
            "type": "narration"
          },
          {
            "focus_terms": [],
            "id": "s003",
            "introduced_terms": [],
            "lines": [
              "但网页不是一张白纸。",
              "它由三种语言搭建而成。"
            ],
            "type": "narration"
          },
          {
            "focus_terms": [
              "html",
              "css"
            ],
            "id": "s004",
            "introduced_terms": [
              "html",
              "css"
            ],
            "lines": [
              "**<term id=\"html\">HTML</term>** 定义结构——标题、段落、表格。",
              "**<term id=\"css\">CSS</term>** 控制样式——颜色、字体、布局。"
            ],
            "type": "narration"
          },
          {
            "focus_terms": [
              "javascript"
            ],
            "id": "s005",
            "introduced_terms": [
              "javascript"
            ],
            "lines": [
              "**<term id=\"javascript\">JavaScript</term>** 添加交互——点击、弹窗、动态加载。",
              "抓取的核心，就是读懂 HTML 的结构。"
            ],
            "type": "narration"
          },
          {
            "exercise": {
              "answer": 1,
              "explanation": "<title> 标签位于 <head> 中，定义网页的标题。",
              "kind": "single_choice",
              "options": [
                "<body>",
                "<head> 里的 <title>",
                "<h1>",
                "<p>"
              ],
              "prompt": "标题信息通常放在哪个 HTML 标签中？"
            },
            "focus_terms": [],
            "id": "s006",
            "introduced_terms": [],
            "lines": [
              "一个网页的标题显示在浏览器标签栏上，它最可能放在哪个标签里？"
            ],
            "type": "exercise"
          },
          {
            "focus_terms": [
              "beautifulsoup"
            ],
            "id": "s007",
            "introduced_terms": [
              "beautifulsoup"
            ],
            "lines": [
              "Python 里最常用的抓取工具是 **<term id=\"beautifulsoup\">BeautifulSoup</term>**。",
              "它能把杂乱的 HTML 变成可搜索的树。"
            ],
            "type": "narration"
          },
          {
            "focus_terms": [],
            "id": "s008",
            "introduced_terms": [],
            "lines": [
              "先用 `requests` 下载网页源码，",
              "再用 BeautifulSoup 解析。"
            ],
            "type": "narration"
          },
          {
            "focus_terms": [],
            "id": "s009",
            "introduced_terms": [],
            "lines": [
              "想找第一个段落？用 `find('p')`。",
              "想找所有链接？用 `find_all('a')`。"
            ],
            "type": "narration"
          },
          {
            "exercise": {
              "answer": 1,
              "explanation": "find_all 返回所有匹配的标签列表。",
              "kind": "single_choice",
              "options": [
                "soup.find('table')",
                "soup.find_all('table')",
                "soup.select('table')",
                "soup.get_text()"
              ],
              "prompt": "BeautifulSoup 中，提取页面上所有 <table> 标签的方法是？"
            },
            "focus_terms": [],
            "id": "s010",
            "introduced_terms": [],
            "lines": [
              "假设你抓到一个表格，想提取所有行。",
              "应该用哪个方法？"
            ],
            "type": "exercise"
          },
          {
            "focus_terms": [],
            "id": "s011",
            "introduced_terms": [],
            "lines": [
              "有些网站用 JavaScript 动态加载数据。",
              "BeautifulSoup 看不到这些内容。"
            ],
            "type": "narration"
          },
          {
            "focus_terms": [
              "selenium"
            ],
            "id": "s012",
            "introduced_terms": [
              "selenium"
            ],
            "lines": [
              "这时需要 **<term id=\"selenium\">Selenium</term>**——它模拟真实浏览器。",
              "能执行 JavaScript，等待页面加载完成。"
            ],
            "type": "narration"
          },
          {
            "focus_terms": [
              "lxml",
              "xpath"
            ],
            "id": "s013",
            "introduced_terms": [
              "lxml",
              "xpath"
            ],
            "lines": [
              "另一种方式是使用 **<term id=\"lxml\">lxml</term>** 和 **<term id=\"xpath\">XPath</term>**。",
              "XPath 像一条路径，直接定位到深层元素。"
            ],
            "type": "narration"
          },
          {
            "focus_terms": [],
            "id": "s014",
            "introduced_terms": [],
            "lines": [
              "比如 `//table[@id='constituents']`",
              "就能直接选中 id 为 constituents 的表格。"
            ],
            "type": "narration"
          },
          {
            "focus_terms": [
              "yfinance"
            ],
            "id": "s015",
            "introduced_terms": [
              "yfinance"
            ],
            "lines": [
              "对于金融数据，还有更省力的办法。",
              "**<term id=\"yfinance\">yfinance</term>** 专门从 Yahoo Finance 拉数据。"
            ],
            "type": "narration"
          },
          {
            "focus_terms": [],
            "id": "s016",
            "introduced_terms": [],
            "lines": [
              "三行代码就能拿到腾讯的历史股价。",
              "`yf.Ticker(\"0700.HK\").history(period=\"1mo\")`"
            ],
            "type": "narration"
          },
          {
            "exercise": {
              "answer": 1,
              "explanation": "调整收盘价向后修正了历史价格，使长期回测更准确。",
              "kind": "single_choice",
              "options": [
                "反映当天最后一笔交易价格",
                "消除分红、拆股等事件对历史价格的影响",
                "显示最高价和最低价的平均值",
                "用于计算交易量"
              ],
              "prompt": "调整收盘价（Adj Close）的主要作用是？"
            },
            "focus_terms": [],
            "id": "s017",
            "introduced_terms": [],
            "lines": [
              "yfinance 返回的数据里，有一列叫“Adj Close”。",
              "它和“Close”有什么区别？"
            ],
            "type": "exercise"
          },
          {
            "focus_terms": [
              "adjusted_close"
            ],
            "id": "s018",
            "introduced_terms": [
              "adjusted_close"
            ],
            "lines": [
              "没错，**<term id=\"adjusted_close\">调整收盘价</term>** 修正了分红和拆股。",
              "比如 2:1 拆股后，之前的股价会乘以 0.5。"
            ],
            "type": "narration"
          },
          {
            "focus_terms": [],
            "id": "s019",
            "introduced_terms": [],
            "lines": [
              "抓下来的数据不能每次都重新爬。",
              "你需要一个本地数据库。"
            ],
            "type": "narration"
          },
          {
            "focus_terms": [
              "sqlite"
            ],
            "id": "s020",
            "introduced_terms": [
              "sqlite"
            ],
            "lines": [
              "Python 内置了 **<term id=\"sqlite\">SQLite</term>**。",
              "一个文件就是一个数据库，零配置。"
            ],
            "type": "narration"
          },
          {
            "focus_terms": [],
            "id": "s021",
            "introduced_terms": [],
            "lines": [
              "建表、插入、查询，和标准 SQL 一样。",
              "`CREATE TABLE market_candles (...)`"
            ],
            "type": "narration"
          },
          {
            "focus_terms": [],
            "id": "s022",
            "introduced_terms": [],
            "lines": [
              "但设计数据库时有个问题：",
              "所有股票放同一张表，查询会越来越慢。"
            ],
            "type": "narration"
          },
          {
            "focus_terms": [],
            "id": "s023",
            "introduced_terms": [],
            "lines": [
              "按股票分表？跨股票分析就麻烦了。",
              "按月份分表？跨月查询又得拼 SQL。"
            ],
            "type": "narration"
          },
          {
            "focus_terms": [],
            "id": "s024",
            "introduced_terms": [],
            "lines": [
              "没有完美的设计。",
              "关键看你的主要用途：回测还是分析？"
            ],
            "type": "narration"
          },
          {
            "focus_terms": [],
            "id": "s025",
            "introduced_terms": [],
            "lines": [
              "回测场景下，按股票分区更高效。",
              "分析场景下，按时间分区更灵活。"
            ],
            "type": "narration"
          },
          {
            "exercise": {
              "answer": 1,
              "explanation": "按股票分表可以快速读取单只股票的完整历史，适合回测。",
              "kind": "single_choice",
              "options": [
                "所有股票放在同一张大表",
                "按股票代码分表",
                "按日期（如每月）分表",
                "不分区，每次全量扫描"
              ],
              "prompt": "对于策略回测，推荐的数据分区方式是？"
            },
            "focus_terms": [],
            "id": "s026",
            "introduced_terms": [],
            "lines": [
              "假设你要回测过去 5 年 500 只股票的策略。",
              "哪种数据库设计最合适？"
            ],
            "type": "exercise"
          },
          {
            "focus_terms": [],
            "id": "s027",
            "introduced_terms": [],
            "lines": [
              "抓取是手段，存储是基础。",
              "而这一切，都是为了让你能专注在策略上。"
            ],
            "type": "narration"
          }
        ],
        "sequence_id": "step1",
        "source": {
          "plain_text": "L2: Data scraping and database management with Python. Covers web scraping using BeautifulSoup, lxml, Selenium, yfinance; limitations of scraping; SQLite database management; adjusted close price concept; database design for financial data.",
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
            "gloss": "经过分红、拆股等公司事件调整后的收盘价，用于反映股票的真实历史价值。"
          },
          "beautifulsoup": {
            "aliases": [
              "bs4"
            ],
            "display": "BeautifulSoup",
            "gloss": "一个流行的 Python 库，用于解析和搜索 HTML 及 XML 数据。"
          },
          "css": {
            "aliases": [
              "Cascading Style Sheets"
            ],
            "display": "CSS",
            "gloss": "层叠样式表，用于控制网页的视觉呈现和布局。"
          },
          "html": {
            "aliases": [
              "HyperText Markup Language"
            ],
            "display": "HTML",
            "gloss": "超文本标记语言，用于定义网页的结构和内容。"
          },
          "javascript": {
            "aliases": [
              "JS"
            ],
            "display": "JavaScript",
            "gloss": "一种脚本语言，用于为网页添加交互和动态行为。"
          },
          "lxml": {
            "aliases": [],
            "display": "lxml",
            "gloss": "一个 Python 库，用于解析 XPath 和 XML 文档。"
          },
          "selenium": {
            "aliases": [],
            "display": "Selenium",
            "gloss": "一个网页自动化工具，可以模拟用户操作，用于抓取动态内容。"
          },
          "sqlite": {
            "aliases": [],
            "display": "SQLite",
            "gloss": "一个轻量级的、基于磁盘的数据库引擎，Python 内置支持。"
          },
          "web_scraping": {
            "aliases": [
              "Web Scraping",
              "Data Scraping"
            ],
            "display": "网页抓取",
            "gloss": "从网站上自动提取数据的过程。"
          },
          "xpath": {
            "aliases": [],
            "display": "XPath",
            "gloss": "一种查询语言，用于在 XML 文档中导航和选择元素。"
          },
          "yfinance": {
            "aliases": [],
            "display": "yfinance",
            "gloss": "一个非官方的 Python 库，用于从 Yahoo Finance 获取金融数据。"
          }
        }
      },
      "step_path": "research/pipeline/3-guided_story/L2/step1/step.json"
    }
  ],
  "target_language": "zh-CN",
  "textbook": {
    "content": "---\nlessonId: L2\ntitle: 数据抓取与数据库管理\ntargetLanguage: zh-CN\nmode: textbook\nsectionMap:\n  - sectionId: why-this-lesson-matters\n    title: 为什么这一课重要\n    coverageTags: [\"core_theme\"]\n    linkedSteps: [\"step1\"]\n  - sectionId: concept-map\n    title: 课程全景与关键问题\n    coverageTags: [\"core_theme\", \"basic_website_structure\", \"web_scraping_libraries\", \"web_scraping_limitations\", \"sqlite_database\", \"adjusted_close_price\", \"database_design_financial_data\"]\n    linkedSteps: [\"step1\"]\n  - sectionId: web-structure\n    title: 网页的三大基石：HTML、CSS、JavaScript\n    coverageTags: [\"basic_website_structure\"]\n    linkedSteps: [\"step1\"]\n  - sectionId: scraping-tools\n    title: 网页抓取工具与方法\n    coverageTags: [\"web_scraping_libraries\"]\n    linkedSteps: [\"step1\"]\n  - sectionId: scraping-limitations\n    title: 网页抓取的局限性\n    coverageTags: [\"web_scraping_limitations\"]\n    linkedSteps: [\"step1\"]\n  - sectionId: adjusted-close\n    title: 调整收盘价：理解价格背后的故事\n    coverageTags: [\"adjusted_close_price\"]\n    linkedSteps: [\"step1\"]\n  - sectionId: sqlite-database\n    title: 使用 Python 管理 SQLite 数据库\n    coverageTags: [\"sqlite_database\"]\n    linkedSteps: [\"step1\"]\n  - sectionId: database-design\n    title: 金融数据存储的数据库设计\n    coverageTags: [\"database_design_financial_data\"]\n    linkedSteps: [\"step1\"]\n  - sectionId: pitfalls\n    title: 易错点\n    coverageTags: [\"web_scraping_limitations\", \"adjusted_close_price\", \"database_design_financial_data\"]\n    linkedSteps: [\"step1\"]\n  - sectionId: review-path\n    title: 复习与自测\n    coverageTags: [\"basic_website_structure\", \"web_scraping_libraries\", \"web_scraping_limitations\", \"sqlite_database\", \"adjusted_close_price\", \"database_design_financial_data\"]\n    linkedSteps: [\"step1\"]\ncoverageTrace:\n  - coverageTag: core_theme\n    description: 本课核心主题：数据抓取与数据库管理在量化交易中的角色\n    sectionIds: [\"why-this-lesson-matters\", \"concept-map\"]\n    linkedSteps: [\"step1\"]\n  - coverageTag: basic_website_structure\n    description: 理解网页基本结构：HTML、CSS、JavaScript 的角色\n    sectionIds: [\"concept-map\", \"web-structure\"]\n    linkedSteps: [\"step1\"]\n  - coverageTag: web_scraping_libraries\n    description: 使用不同 Python 库进行网页抓取：BeautifulSoup, lxml/XPath, Selenium, yfinance\n    sectionIds: [\"concept-map\", \"scraping-tools\"]\n    linkedSteps: [\"step1\"]\n  - coverageTag: web_scraping_limitations\n    description: 理解网页抓取的局限性：数据质量、资源消耗、技术挑战\n    sectionIds: [\"concept-map\", \"scraping-limitations\", \"pitfalls\"]\n    linkedSteps: [\"step1\"]\n  - coverageTag: sqlite_database\n    description: 使用 Python 创建和管理 SQLite 数据库：CRUD 操作\n    sectionIds: [\"concept-map\", \"sqlite-database\"]\n    linkedSteps: [\"step1\"]\n  - coverageTag: adjusted_close_price\n    description: 理解调整收盘价的概念：分红、拆股对价格的影响及调整方法\n    sectionIds: [\"concept-map\", \"adjusted-close\", \"pitfalls\"]\n    linkedSteps: [\"step1\"]\n  - coverageTag: database_design_financial_data\n    description: 金融数据存储的数据库设计考量：分区策略、权衡\n    sectionIds: [\"concept-map\", \"database-design\", \"pitfalls\"]\n    linkedSteps: [\"step1\"]\n---\n\n# 数据抓取与数据库管理\n\n## 为什么这一课重要 {#why-this-lesson-matters}\n\n想象一下，你正在监控 500 家公司的股价，手动刷新网页？太慢了。你需要一个自动化的方式，从网站上提取数据，这就是 <Term id=\"web_scraping\" en=\"Web Scraping\">网页抓取</Term>。\n\n在量化交易中，数据是策略的基石。无论是获取实时行情、分析市场情绪，还是回测历史策略，都离不开高效的数据获取与管理。本课将带你掌握从网页抓取数据到存储管理的完整流程，为后续的策略开发打下坚实基础。\n\n<KeyPoint>\n本课的核心目标是让你能够：1) 理解网页结构；2) 使用 Python 工具抓取数据；3) 理解调整收盘价；4) 将数据存入数据库并设计合理的存储方案。\n</KeyPoint>\n\n## 课程全景与关键问题 {#concept-map}\n\n本课围绕一个核心问题展开：**如何从互联网获取金融数据，并将其高效、准确地存储起来，用于后续的策略分析与回测？**\n\n整个流程可以概括为：\n\n1.  **理解数据来源**：网页由 <Term id=\"html\" en=\"HyperText Markup Language\">HTML</Term>、<Term id=\"css\" en=\"Cascading Style Sheets\">CSS</Term> 和 <Term id=\"javascript\" en=\"JavaScript\">JavaScript</Term> 构成。抓取的核心是解析 HTML 结构。\n2.  **选择抓取工具**：根据网页的静态或动态特性，选择合适的 Python 库，如 <Term id=\"beautifulsoup\" en=\"BeautifulSoup\">BeautifulSoup</Term>、<Term id=\"lxml\" en=\"lxml\">lxml</Term>、<Term id=\"selenium\" en=\"Selenium\">Selenium</Term> 或专门的 <Term id=\"yfinance\" en=\"yfinance\">yfinance</Term>。\n3.  **理解数据含义**：金融数据中的 <Term id=\"adjusted_close\" en=\"Adjusted Close\">调整收盘价</Term> 消除了公司事件的影响，是进行长期分析和回测的关键。\n4.  **设计存储方案**：使用 <Term id=\"sqlite\" en=\"SQLite\">SQLite</Term> 等数据库持久化数据，并根据查询需求（如回测 vs 分析）设计合理的表结构。\n\n<Checkpoint title=\"全局理解\">\n在深入学习之前，请思考：为什么不能直接使用从网页上看到的原始价格进行回测？为什么不能把所有数据都放在一个 Excel 文件里？\n</Checkpoint>\n\n## 网页的三大基石：HTML、CSS、JavaScript {#web-structure}\n\n一个网页通常由三种技术共同构建：\n\n- **<Term id=\"html\" en=\"HyperText Markup Language\">HTML</Term>**：定义网页的**结构**和**内容**，如标题、段落、表格、链接等。它是一种标记语言，不是编程语言。\n- **<Term id=\"css\" en=\"Cascading Style Sheets\">CSS</Term>**：控制网页的**视觉呈现**和**布局**，如颜色、字体、间距、位置等。\n- **<Term id=\"javascript\" en=\"JavaScript\">JavaScript</Term>**：为网页添加**交互**和**动态行为**，如点击弹窗、动态加载数据、表单验证等。\n\n<KeyPoint>\n网页抓取的核心是**解析 HTML 结构**来提取数据。CSS 和 JavaScript 主要影响呈现和交互，数据通常嵌入在 HTML 标签中。\n</KeyPoint>\n\n### 常见 HTML 标签\n\n以下是抓取时最常遇到的 HTML 标签：\n\n| 标签 | 描述 |\n| :--- | :--- |\n| `<html>` | HTML 文档的根元素 |\n| `<head>` | 包含文档的元信息（如标题、样式） |\n| `<title>` | 定义网页标题，显示在浏览器标签栏 |\n| `<body>` | 包含文档的可见内容 |\n| `<h1>` 到 `<h6>` | 定义不同级别的标题 |\n| `<p>` | 定义一个段落 |\n| `<a>` | 创建一个超链接 |\n| `<img>` | 插入一张图片 |\n| `<div>` | 定义一个块级容器，用于分组和样式化 |\n| `<span>` | 定义一个行内容器，用于样式化部分文本 |\n| `<ul>`, `<ol>`, `<li>` | 创建无序或有序列表 |\n| `<table>`, `<tr>`, `<td>`, `<th>` | 创建表格 |\n| `<form>`, `<input>`, `<button>` | 创建表单和输入控件 |\n\n<Example title=\"一个简单的 HTML 页面\">\n```html\n<!DOCTYPE html>\n<html>\n<head>\n    <meta charset=\"utf-8\">\n    <title>我的第一个网页</title>\n</head>\n<body>\n    <h1>欢迎来到我的网站</h1>\n    <p>这是一个段落。</p>\n    <a href=\"https://www.example.com\">访问示例网站</a>\n</body>\n</html>\n```\n</Example>\n\n<QuestionRef id=\"q_flash_html_tags_v1\" />\n<QuestionRef id=\"q_flash_web_tech_roles_v1\" />\n\n## 网页抓取工具与方法 {#scraping-tools}\n\nPython 提供了多种库来应对不同的抓取场景。\n\n### 1. BeautifulSoup：静态页面的利器\n\n<Term id=\"beautifulsoup\" en=\"BeautifulSoup\">BeautifulSoup</Term> 是最流行的 Python 库之一，用于解析 HTML 和 XML 数据。它能将杂乱的 HTML 文档转换成一个可搜索的树形结构。\n\n**基本流程：**\n1.  使用 `requests` 库下载网页源码。\n2.  使用 `BeautifulSoup` 解析源码。\n3.  使用 `find()`、`find_all()` 等方法定位并提取数据。\n\n<Example title=\"使用 BeautifulSoup 提取数据\">\n```python\nimport requests\nfrom bs4 import BeautifulSoup\n\n# 1. 下载网页\nurl = 'https://example.com'\nresponse = requests.get(url)\ntext = response.text\n\n# 2. 解析 HTML\nsoup = BeautifulSoup(text, 'html.parser')\n\n# 3. 提取数据\n# 提取第一个 <p> 标签\nfirst_paragraph = soup.find('p')\nprint(first_paragraph.text)\n\n# 提取所有 <a> 标签\nall_links = soup.find_all('a')\nfor link in all_links:\n    print(link.text, link.get('href'))\n```\n</Example>\n\n<QuestionRef id=\"q_flash_bs4_methods_v1\" />\n\n### 2. lxml 与 XPath：精准定位\n\n<Term id=\"lxml\" en=\"lxml\">lxml</Term> 是一个高效的 XML 和 HTML 解析库。它常与 <Term id=\"xpath\" en=\"XPath\">XPath</Term> 配合使用。XPath 是一种在 XML 文档中导航和选择节点的查询语言，可以像路径一样直接定位到深层元素。\n\n**常见 XPath 语法：**\n- `//element`：选择文档中所有名为 `element` 的元素。\n- `//element[@attribute='value']`：选择属性 `attribute` 值为 `value` 的所有 `element` 元素。\n- `//element[position()=1]`：选择第一个 `element` 元素。\n\n<Example title=\"使用 XPath 定位表格\">\n```python\nfrom lxml import html\nimport requests\n\nurl = 'https://en.wikipedia.org/wiki/List_of_S%26P_500_companies'\nheaders = {'user-agent': 'Mozilla/5.0'}\nresponse = requests.get(url, headers=headers)\ntree = html.fromstring(response.content)\n\n# 使用 XPath 定位 id 为 'constituents' 的表格的所有行\ntable_rows = tree.xpath('//*[@id=\"constituents\"]/tbody/tr')\nprint(f\"找到 {len(table_rows)} 行数据\")\n```\n</Example>\n\n<QuestionRef id=\"q_flash_xpath_syntax_v1\" />\n\n### 3. Selenium：处理动态内容\n\n当网页数据是通过 JavaScript 动态加载时，BeautifulSoup 无法直接获取。这时需要 <Term id=\"selenium\" en=\"Selenium\">Selenium</Term>。它模拟真实浏览器，能执行 JavaScript，等待页面加载完成，并模拟用户操作（如点击、滚动）。\n\n<KeyPoint>\n**何时使用 Selenium？** 当你在网页源码中看不到目标数据，但浏览器渲染后能看到时，就需要 Selenium。\n</KeyPoint>\n\n<QuestionRef id=\"q_flash_selenium_purpose_v1\" />\n\n### 4. yfinance：金融数据的快捷方式\n\n对于金融数据，有更省力的办法。<Term id=\"yfinance\" en=\"yfinance\">yfinance</Term> 是一个非官方的 Python 库，专门用于从 Yahoo Finance 获取金融数据。它封装了复杂的请求和解析过程，几行代码就能拿到数据。\n\n<Example title=\"使用 yfinance 获取股票历史数据\">\n```python\nimport yfinance as yf\n\n# 获取腾讯控股 (0700.HK) 最近一个月的历史数据\nticker = yf.Ticker(\"0700.HK\")\ndata = ticker.history(period=\"1mo\")\nprint(data.head())\n```\n</Example>\n\n<QuestionRef id=\"q_flash_yfinance_usage_v1\" />\n\n### 实战案例：抓取 S&P 500 成分股\n\n这是一个综合案例，展示如何从 Wikipedia 抓取 S&P 500 成分股列表。\n\n<Example title=\"抓取 S&P 500 成分股\">\n```python\nimport requests\nfrom bs4 import BeautifulSoup\nimport pandas as pd\n\nheaders = {\n    \"user-agent\": \"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36\"\n}\nurl = 'https://en.wikipedia.org/wiki/List_of_S%26P_500_companies'\ntext = requests.get(url, headers=headers).text\nsoup = BeautifulSoup(text, 'html.parser')\n\n# 通过 id 定位正确的表格\ntable = soup.find('table', id=\"constituents\")\n\n# 提取表头\nheaders_list = [th.text.strip() for th in table.find_all('th')]\n\n# 提取数据行\nrows = []\nfor tr in table.find_all('tr')[1:]:  # 跳过表头行\n    cells = tr.find_all('td')\n    row = [cell.text.strip() for cell in cells]\n    rows.append(row)\n\n# 创建 DataFrame\ndf = pd.DataFrame(rows, columns=headers_list)\nprint(df.head())\n```\n</Example>\n\n<QuestionFamily familyId=\"qf_long_scrape_sp500\" />\n\n## 网页抓取的局限性 {#scraping-limitations}\n\n网页抓取虽然强大，但并非万能，存在以下主要局限性：\n\n1.  **数据质量**：网站数据可能不完整、不及时，影响分析准确性。\n2.  **资源密集型**：抓取大量数据会消耗大量带宽和存储空间，且加载页面耗时。\n3.  **技术挑战**：\n    - **网站结构变化**：网站改版可能导致基于特定标签或 class 的定位代码失效。\n    - **反爬措施**：许多网站会实施 IP 封锁、请求频率限制、验证码等反爬措施。\n\n<QuestionRef id=\"q_flash_scraping_limits_v1\" />\n<QuestionFamily familyId=\"qf_long_scraping_limitations_explain\" />\n\n## 调整收盘价：理解价格背后的故事 {#adjusted-close}\n\n在金融数据中，你经常会看到两列价格：`Close`（收盘价）和 `Adj Close`（调整收盘价）。它们有什么区别？\n\n- **收盘价 (Close)**：当天最后一笔交易的价格。\n- **调整收盘价 (Adjusted Close)**：经过分红、拆股等公司事件调整后的收盘价。\n\n<KeyPoint>\n**为什么需要调整？** 公司事件（如分红、拆股）会导致股价出现非市场因素的跳变。例如，2:1 拆股后，股价会瞬间减半。如果使用原始收盘价进行长期回测，会错误地认为股价暴跌，导致策略表现被严重低估。调整收盘价消除了这些事件的影响，使历史价格序列连续，真实反映股票的价值变化。\n</KeyPoint>\n\n### 调整机制\n\nYahoo Finance 使用乘数法向后调整历史价格。\n\n- **拆股调整**：对于 2:1 拆股，拆股前的所有价格乘以 `0.5`。\n- **分红调整**：对于每股 $0.08 的分红，除息日前的所有价格乘以 `(1 - 0.08 / 除息日前一日收盘价)`。\n\n<Example title=\"计算调整收盘价\">\n假设一只股票在 2/18 进行 2:1 拆股，2/21 除息（分红 $0.08）。2/17 收盘价为 $48.30，2/18 收盘价为 $24.96。\n\n1.  **计算调整因子**：\n    - 拆股因子 = 0.5\n    - 分红因子 = 1 - (0.08 / 24.96) = 0.9968\n2.  **计算 2/17 的调整收盘价**：\n    - 调整收盘价 = 0.5 * 0.9968 * 48.30 = **24.07**\n</Example>\n\n<QuestionRef id=\"q_flash_adjusted_close_concept_v1\" />\n<QuestionFamily familyId=\"qf_long_adjusted_close_calculation\" />\n\n## 使用 Python 管理 SQLite 数据库 {#sqlite-database}\n\n抓取下来的数据不能每次都重新爬，你需要一个本地数据库。Python 内置了 <Term id=\"sqlite\" en=\"SQLite\">SQLite</Term>，一个文件就是一个数据库，零配置，非常适合学习和中小型项目。\n\n### 核心操作 (CRUD)\n\n<Definition title=\"CRUD\">\n指数据库的四种基本操作：**C**reate (创建/插入), **R**ead (查询), **U**pdate (更新), **D**elete (删除)。\n</Definition>\n\n<Example title=\"SQLite 基本操作\">\n```python\nimport sqlite3\n\n# 1. 连接数据库 (如果不存在则创建)\nconn = sqlite3.connect('market.db')\ncursor = conn.cursor()\n\n# 2. 创建表 (CREATE)\ncursor.execute('''\n    CREATE TABLE IF NOT EXISTS stocks (\n        symbol TEXT,\n        date TEXT,\n        close_price REAL\n    )\n''')\n\n# 3. 插入数据 (INSERT)\ncursor.execute(\"INSERT INTO stocks (symbol, date, close_price) VALUES (?, ?, ?)\", \n               ('0700.HK', '2025-01-02', 412.4))\nconn.commit()\n\n# 4. 查询数据 (SELECT)\ncursor.execute(\"SELECT * FROM stocks WHERE symbol = ?\", ('0700.HK',))\nfor row in cursor.fetchall():\n    print(row)\n\n# 5. 更新数据 (UPDATE)\ncursor.execute(\"UPDATE stocks SET close_price = ? WHERE symbol = ? AND date = ?\", \n               (415.0, '0700.HK', '2025-01-02'))\nconn.commit()\n\n# 6. 删除数据 (DELETE)\ncursor.execute(\"DELETE FROM stocks WHERE symbol = ?\", ('0700.HK',))\nconn.commit()\n\n# 7. 关闭连接\nconn.close()\n```\n</Example>\n\n<QuestionRef id=\"q_flash_sqlite_crud_v1\" />\n<QuestionFamily familyId=\"qf_long_sqlite_crud_example\" />\n\n## 金融数据存储的数据库设计 {#database-design}\n\n设计存储金融数据的数据库时，没有完美的方案，关键在于权衡。主要考虑三种策略：\n\n### 策略一：所有股票放在同一张大表\n\n- **优点**：结构简单，易于跨股票查询和分析。\n- **缺点**：表会非常庞大，查询和备份速度慢。\n\n### 策略二：按股票代码分表\n\n- **优点**：单只股票的回测查询非常快。\n- **缺点**：表数量庞大，跨股票分析需要查询多张表，代码复杂。\n\n### 策略三：按时间（如月份）分表\n\n- **优点**：按时间范围查询快，便于月度数据管理。\n- **缺点**：跨月查询需要动态拼接 SQL，单只股票的完整历史查询较慢。\n\n<KeyPoint>\n**如何选择？**\n- **策略回测**：主要查询单只股票的完整历史，推荐**按股票分表**。\n- **数据分析**：主要进行跨股票的截面分析，推荐**所有股票放在同一张大表**并建立合适的索引。\n</KeyPoint>\n\n<QuestionRef id=\"q_flash_db_design_tradeoffs_v1\" />\n<QuestionFamily familyId=\"qf_long_db_design_analysis\" />\n\n## 易错点 {#pitfalls}\n\n1.  **忽略 User-Agent**：直接使用 `requests.get(url)` 可能会被服务器拒绝。务必设置 `headers` 模拟浏览器访问。\n2.  **混淆 `Close` 与 `Adj Close`**：进行长期回测时，必须使用 `Adj Close`，否则结果会严重失真。\n3.  **数据库设计一刀切**：没有最好的设计，只有最适合当前场景的设计。不要盲目选择一种策略，要根据主要用途（回测 vs 分析）来权衡。\n4.  **忘记提交事务**：在 SQLite 中执行 `INSERT`、`UPDATE`、`DELETE` 后，必须调用 `conn.commit()` 才能使更改生效。\n\n## 复习与自测 {#review-path}\n\n### 核心概念回顾\n\n- 网页的三大技术是什么？它们在抓取中各扮演什么角色？\n- 比较 BeautifulSoup、Selenium 和 yfinance 的适用场景。\n- 什么是调整收盘价？为什么它对回测至关重要？\n- SQLite 的 CRUD 操作分别对应哪些 SQL 语句？\n- 在设计金融数据库时，按股票分表和按时间分表各有什么优缺点？\n\n### 自测题目\n\n<QuestionFamily familyId=\"qf_flash_html_tags\" />\n<QuestionFamily familyId=\"qf_flash_web_tech_roles\" />\n<QuestionFamily familyId=\"qf_flash_bs4_methods\" />\n<QuestionFamily familyId=\"qf_flash_xpath_syntax\" />\n<QuestionFamily familyId=\"qf_flash_selenium_purpose\" />\n<QuestionFamily familyId=\"qf_flash_yfinance_usage\" />\n<QuestionFamily familyId=\"qf_flash_scraping_limits\" />\n<QuestionFamily familyId=\"qf_flash_adjusted_close_concept\" />\n<QuestionFamily familyId=\"qf_flash_sqlite_crud\" />\n<QuestionFamily familyId=\"qf_flash_db_design_tradeoffs\" />\n\n### 深入练习\n\n<QuestionFamily familyId=\"qf_long_web_structure_explain\" />\n<QuestionFamily familyId=\"qf_long_scraping_tool_choice\" />\n<QuestionFamily familyId=\"qf_long_scraping_limitations_explain\" />\n<QuestionFamily familyId=\"qf_long_adjusted_close_calculation\" />\n<QuestionFamily familyId=\"qf_long_sqlite_crud_example\" />\n<QuestionFamily familyId=\"qf_long_db_design_analysis\" />",
    "path": "research/pipeline/5-textbook/L2.mdx",
    "sections": [
      {
        "content": "",
        "level": 1,
        "section_id": "section_1",
        "title": "数据抓取与数据库管理"
      },
      {
        "content": "\n想象一下，你正在监控 500 家公司的股价，手动刷新网页？太慢了。你需要一个自动化的方式，从网站上提取数据，这就是 <Term id=\"web_scraping\" en=\"Web Scraping\">网页抓取</Term>。\n\n在量化交易中，数据是策略的基石。无论是获取实时行情、分析市场情绪，还是回测历史策略，都离不开高效的数据获取与管理。本课将带你掌握从网页抓取数据到存储管理的完整流程，为后续的策略开发打下坚实基础。\n\n<KeyPoint>\n本课的核心目标是让你能够：1) 理解网页结构；2) 使用 Python 工具抓取数据；3) 理解调整收盘价；4) 将数据存入数据库并设计合理的存储方案。\n</KeyPoint>\n",
        "level": 2,
        "section_id": "section_2",
        "title": "为什么这一课重要 {#why-this-lesson-matters}"
      },
      {
        "content": "\n本课围绕一个核心问题展开：**如何从互联网获取金融数据，并将其高效、准确地存储起来，用于后续的策略分析与回测？**\n\n整个流程可以概括为：\n\n1.  **理解数据来源**：网页由 <Term id=\"html\" en=\"HyperText Markup Language\">HTML</Term>、<Term id=\"css\" en=\"Cascading Style Sheets\">CSS</Term> 和 <Term id=\"javascript\" en=\"JavaScript\">JavaScript</Term> 构成。抓取的核心是解析 HTML 结构。\n2.  **选择抓取工具**：根据网页的静态或动态特性，选择合适的 Python 库，如 <Term id=\"beautifulsoup\" en=\"BeautifulSoup\">BeautifulSoup</Term>、<Term id=\"lxml\" en=\"lxml\">lxml</Term>、<Term id=\"selenium\" en=\"Selenium\">Selenium</Term> 或专门的 <Term id=\"yfinance\" en=\"yfinance\">yfinance</Term>。\n3.  **理解数据含义**：金融数据中的 <Term id=\"adjusted_close\" en=\"Adjusted Close\">调整收盘价</Term> 消除了公司事件的影响，是进行长期分析和回测的关键。\n4.  **设计存储方案**：使用 <Term id=\"sqlite\" en=\"SQLite\">SQLite</Term> 等数据库持久化数据，并根据查询需求（如回测 vs 分析）设计合理的表结构。\n\n<Checkpoint title=\"全局理解\">\n在深入学习之前，请思考：为什么不能直接使用从网页上看到的原始价格进行回测？为什么不能把所有数据都放在一个 Excel 文件里？\n</Checkpoint>\n",
        "level": 2,
        "section_id": "section_3",
        "title": "课程全景与关键问题 {#concept-map}"
      },
      {
        "content": "\n一个网页通常由三种技术共同构建：\n\n- **<Term id=\"html\" en=\"HyperText Markup Language\">HTML</Term>**：定义网页的**结构**和**内容**，如标题、段落、表格、链接等。它是一种标记语言，不是编程语言。\n- **<Term id=\"css\" en=\"Cascading Style Sheets\">CSS</Term>**：控制网页的**视觉呈现**和**布局**，如颜色、字体、间距、位置等。\n- **<Term id=\"javascript\" en=\"JavaScript\">JavaScript</Term>**：为网页添加**交互**和**动态行为**，如点击弹窗、动态加载数据、表单验证等。\n\n<KeyPoint>\n网页抓取的核心是**解析 HTML 结构**来提取数据。CSS 和 JavaScript 主要影响呈现和交互，数据通常嵌入在 HTML 标签中。\n</KeyPoint>\n",
        "level": 2,
        "section_id": "section_4",
        "title": "网页的三大基石：HTML、CSS、JavaScript {#web-structure}"
      },
      {
        "content": "\n以下是抓取时最常遇到的 HTML 标签：\n\n| 标签 | 描述 |\n| :--- | :--- |\n| `<html>` | HTML 文档的根元素 |\n| `<head>` | 包含文档的元信息（如标题、样式） |\n| `<title>` | 定义网页标题，显示在浏览器标签栏 |\n| `<body>` | 包含文档的可见内容 |\n| `<h1>` 到 `<h6>` | 定义不同级别的标题 |\n| `<p>` | 定义一个段落 |\n| `<a>` | 创建一个超链接 |\n| `<img>` | 插入一张图片 |\n| `<div>` | 定义一个块级容器，用于分组和样式化 |\n| `<span>` | 定义一个行内容器，用于样式化部分文本 |\n| `<ul>`, `<ol>`, `<li>` | 创建无序或有序列表 |\n| `<table>`, `<tr>`, `<td>`, `<th>` | 创建表格 |\n| `<form>`, `<input>`, `<button>` | 创建表单和输入控件 |\n\n<Example title=\"一个简单的 HTML 页面\">\n```html\n<!DOCTYPE html>\n<html>\n<head>\n    <meta charset=\"utf-8\">\n    <title>我的第一个网页</title>\n</head>\n<body>\n    <h1>欢迎来到我的网站</h1>\n    <p>这是一个段落。</p>\n    <a href=\"https://www.example.com\">访问示例网站</a>\n</body>\n</html>\n```\n</Example>\n\n<QuestionRef id=\"q_flash_html_tags_v1\" />\n<QuestionRef id=\"q_flash_web_tech_roles_v1\" />\n",
        "level": 3,
        "section_id": "section_5",
        "title": "常见 HTML 标签"
      },
      {
        "content": "\nPython 提供了多种库来应对不同的抓取场景。\n",
        "level": 2,
        "section_id": "section_6",
        "title": "网页抓取工具与方法 {#scraping-tools}"
      },
      {
        "content": "\n<Term id=\"beautifulsoup\" en=\"BeautifulSoup\">BeautifulSoup</Term> 是最流行的 Python 库之一，用于解析 HTML 和 XML 数据。它能将杂乱的 HTML 文档转换成一个可搜索的树形结构。\n\n**基本流程：**\n1.  使用 `requests` 库下载网页源码。\n2.  使用 `BeautifulSoup` 解析源码。\n3.  使用 `find()`、`find_all()` 等方法定位并提取数据。\n\n<Example title=\"使用 BeautifulSoup 提取数据\">\n```python\nimport requests\nfrom bs4 import BeautifulSoup\n",
        "level": 3,
        "section_id": "section_7",
        "title": "1. BeautifulSoup：静态页面的利器"
      },
      {
        "content": "url = 'https://example.com'\nresponse = requests.get(url)\ntext = response.text\n",
        "level": 1,
        "section_id": "section_8",
        "title": "1. 下载网页"
      },
      {
        "content": "soup = BeautifulSoup(text, 'html.parser')\n",
        "level": 1,
        "section_id": "section_9",
        "title": "2. 解析 HTML"
      },
      {
        "content": "",
        "level": 1,
        "section_id": "section_10",
        "title": "3. 提取数据"
      },
      {
        "content": "first_paragraph = soup.find('p')\nprint(first_paragraph.text)\n",
        "level": 1,
        "section_id": "section_11",
        "title": "提取第一个 <p> 标签"
      },
      {
        "content": "all_links = soup.find_all('a')\nfor link in all_links:\n    print(link.text, link.get('href'))\n```\n</Example>\n\n<QuestionRef id=\"q_flash_bs4_methods_v1\" />\n",
        "level": 1,
        "section_id": "section_12",
        "title": "提取所有 <a> 标签"
      },
      {
        "content": "\n<Term id=\"lxml\" en=\"lxml\">lxml</Term> 是一个高效的 XML 和 HTML 解析库。它常与 <Term id=\"xpath\" en=\"XPath\">XPath</Term> 配合使用。XPath 是一种在 XML 文档中导航和选择节点的查询语言，可以像路径一样直接定位到深层元素。\n\n**常见 XPath 语法：**\n- `//element`：选择文档中所有名为 `element` 的元素。\n- `//element[@attribute='value']`：选择属性 `attribute` 值为 `value` 的所有 `element` 元素。\n- `//element[position()=1]`：选择第一个 `element` 元素。\n\n<Example title=\"使用 XPath 定位表格\">\n```python\nfrom lxml import html\nimport requests\n\nurl = 'https://en.wikipedia.org/wiki/List_of_S%26P_500_companies'\nheaders = {'user-agent': 'Mozilla/5.0'}\nresponse = requests.get(url, headers=headers)\ntree = html.fromstring(response.content)\n",
        "level": 3,
        "section_id": "section_13",
        "title": "2. lxml 与 XPath：精准定位"
      },
      {
        "content": "table_rows = tree.xpath('//*[@id=\"constituents\"]/tbody/tr')\nprint(f\"找到 {len(table_rows)} 行数据\")\n```\n</Example>\n\n<QuestionRef id=\"q_flash_xpath_syntax_v1\" />\n",
        "level": 1,
        "section_id": "section_14",
        "title": "使用 XPath 定位 id 为 'constituents' 的表格的所有行"
      },
      {
        "content": "\n当网页数据是通过 JavaScript 动态加载时，BeautifulSoup 无法直接获取。这时需要 <Term id=\"selenium\" en=\"Selenium\">Selenium</Term>。它模拟真实浏览器，能执行 JavaScript，等待页面加载完成，并模拟用户操作（如点击、滚动）。\n\n<KeyPoint>\n**何时使用 Selenium？** 当你在网页源码中看不到目标数据，但浏览器渲染后能看到时，就需要 Selenium。\n</KeyPoint>\n\n<QuestionRef id=\"q_flash_selenium_purpose_v1\" />\n",
        "level": 3,
        "section_id": "section_15",
        "title": "3. Selenium：处理动态内容"
      },
      {
        "content": "\n对于金融数据，有更省力的办法。<Term id=\"yfinance\" en=\"yfinance\">yfinance</Term> 是一个非官方的 Python 库，专门用于从 Yahoo Finance 获取金融数据。它封装了复杂的请求和解析过程，几行代码就能拿到数据。\n\n<Example title=\"使用 yfinance 获取股票历史数据\">\n```python\nimport yfinance as yf\n",
        "level": 3,
        "section_id": "section_16",
        "title": "4. yfinance：金融数据的快捷方式"
      },
      {
        "content": "ticker = yf.Ticker(\"0700.HK\")\ndata = ticker.history(period=\"1mo\")\nprint(data.head())\n```\n</Example>\n\n<QuestionRef id=\"q_flash_yfinance_usage_v1\" />\n",
        "level": 1,
        "section_id": "section_17",
        "title": "获取腾讯控股 (0700.HK) 最近一个月的历史数据"
      },
      {
        "content": "\n这是一个综合案例，展示如何从 Wikipedia 抓取 S&P 500 成分股列表。\n\n<Example title=\"抓取 S&P 500 成分股\">\n```python\nimport requests\nfrom bs4 import BeautifulSoup\nimport pandas as pd\n\nheaders = {\n    \"user-agent\": \"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36\"\n}\nurl = 'https://en.wikipedia.org/wiki/List_of_S%26P_500_companies'\ntext = requests.get(url, headers=headers).text\nsoup = BeautifulSoup(text, 'html.parser')\n",
        "level": 3,
        "section_id": "section_18",
        "title": "实战案例：抓取 S&P 500 成分股"
      },
      {
        "content": "table = soup.find('table', id=\"constituents\")\n",
        "level": 1,
        "section_id": "section_19",
        "title": "通过 id 定位正确的表格"
      },
      {
        "content": "headers_list = [th.text.strip() for th in table.find_all('th')]\n",
        "level": 1,
        "section_id": "section_20",
        "title": "提取表头"
      },
      {
        "content": "rows = []\nfor tr in table.find_all('tr')[1:]:  # 跳过表头行\n    cells = tr.find_all('td')\n    row = [cell.text.strip() for cell in cells]\n    rows.append(row)\n",
        "level": 1,
        "section_id": "section_21",
        "title": "提取数据行"
      },
      {
        "content": "df = pd.DataFrame(rows, columns=headers_list)\nprint(df.head())\n```\n</Example>\n\n<QuestionFamily familyId=\"qf_long_scrape_sp500\" />\n",
        "level": 1,
        "section_id": "section_22",
        "title": "创建 DataFrame"
      },
      {
        "content": "\n网页抓取虽然强大，但并非万能，存在以下主要局限性：\n\n1.  **数据质量**：网站数据可能不完整、不及时，影响分析准确性。\n2.  **资源密集型**：抓取大量数据会消耗大量带宽和存储空间，且加载页面耗时。\n3.  **技术挑战**：\n    - **网站结构变化**：网站改版可能导致基于特定标签或 class 的定位代码失效。\n    - **反爬措施**：许多网站会实施 IP 封锁、请求频率限制、验证码等反爬措施。\n\n<QuestionRef id=\"q_flash_scraping_limits_v1\" />\n<QuestionFamily familyId=\"qf_long_scraping_limitations_explain\" />\n",
        "level": 2,
        "section_id": "section_23",
        "title": "网页抓取的局限性 {#scraping-limitations}"
      },
      {
        "content": "\n在金融数据中，你经常会看到两列价格：`Close`（收盘价）和 `Adj Close`（调整收盘价）。它们有什么区别？\n\n- **收盘价 (Close)**：当天最后一笔交易的价格。\n- **调整收盘价 (Adjusted Close)**：经过分红、拆股等公司事件调整后的收盘价。\n\n<KeyPoint>\n**为什么需要调整？** 公司事件（如分红、拆股）会导致股价出现非市场因素的跳变。例如，2:1 拆股后，股价会瞬间减半。如果使用原始收盘价进行长期回测，会错误地认为股价暴跌，导致策略表现被严重低估。调整收盘价消除了这些事件的影响，使历史价格序列连续，真实反映股票的价值变化。\n</KeyPoint>\n",
        "level": 2,
        "section_id": "section_24",
        "title": "调整收盘价：理解价格背后的故事 {#adjusted-close}"
      },
      {
        "content": "\nYahoo Finance 使用乘数法向后调整历史价格。\n\n- **拆股调整**：对于 2:1 拆股，拆股前的所有价格乘以 `0.5`。\n- **分红调整**：对于每股 $0.08 的分红，除息日前的所有价格乘以 `(1 - 0.08 / 除息日前一日收盘价)`。\n\n<Example title=\"计算调整收盘价\">\n假设一只股票在 2/18 进行 2:1 拆股，2/21 除息（分红 $0.08）。2/17 收盘价为 $48.30，2/18 收盘价为 $24.96。\n\n1.  **计算调整因子**：\n    - 拆股因子 = 0.5\n    - 分红因子 = 1 - (0.08 / 24.96) = 0.9968\n2.  **计算 2/17 的调整收盘价**：\n    - 调整收盘价 = 0.5 * 0.9968 * 48.30 = **24.07**\n</Example>\n\n<QuestionRef id=\"q_flash_adjusted_close_concept_v1\" />\n<QuestionFamily familyId=\"qf_long_adjusted_close_calculation\" />\n",
        "level": 3,
        "section_id": "section_25",
        "title": "调整机制"
      },
      {
        "content": "\n抓取下来的数据不能每次都重新爬，你需要一个本地数据库。Python 内置了 <Term id=\"sqlite\" en=\"SQLite\">SQLite</Term>，一个文件就是一个数据库，零配置，非常适合学习和中小型项目。\n",
        "level": 2,
        "section_id": "section_26",
        "title": "使用 Python 管理 SQLite 数据库 {#sqlite-database}"
      },
      {
        "content": "\n<Definition title=\"CRUD\">\n指数据库的四种基本操作：**C**reate (创建/插入), **R**ead (查询), **U**pdate (更新), **D**elete (删除)。\n</Definition>\n\n<Example title=\"SQLite 基本操作\">\n```python\nimport sqlite3\n",
        "level": 3,
        "section_id": "section_27",
        "title": "核心操作 (CRUD)"
      },
      {
        "content": "conn = sqlite3.connect('market.db')\ncursor = conn.cursor()\n",
        "level": 1,
        "section_id": "section_28",
        "title": "1. 连接数据库 (如果不存在则创建)"
      },
      {
        "content": "cursor.execute('''\n    CREATE TABLE IF NOT EXISTS stocks (\n        symbol TEXT,\n        date TEXT,\n        close_price REAL\n    )\n''')\n",
        "level": 1,
        "section_id": "section_29",
        "title": "2. 创建表 (CREATE)"
      },
      {
        "content": "cursor.execute(\"INSERT INTO stocks (symbol, date, close_price) VALUES (?, ?, ?)\", \n               ('0700.HK', '2025-01-02', 412.4))\nconn.commit()\n",
        "level": 1,
        "section_id": "section_30",
        "title": "3. 插入数据 (INSERT)"
      },
      {
        "content": "cursor.execute(\"SELECT * FROM stocks WHERE symbol = ?\", ('0700.HK',))\nfor row in cursor.fetchall():\n    print(row)\n",
        "level": 1,
        "section_id": "section_31",
        "title": "4. 查询数据 (SELECT)"
      },
      {
        "content": "cursor.execute(\"UPDATE stocks SET close_price = ? WHERE symbol = ? AND date = ?\", \n               (415.0, '0700.HK', '2025-01-02'))\nconn.commit()\n",
        "level": 1,
        "section_id": "section_32",
        "title": "5. 更新数据 (UPDATE)"
      },
      {
        "content": "cursor.execute(\"DELETE FROM stocks WHERE symbol = ?\", ('0700.HK',))\nconn.commit()\n",
        "level": 1,
        "section_id": "section_33",
        "title": "6. 删除数据 (DELETE)"
      },
      {
        "content": "conn.close()\n```\n</Example>\n\n<QuestionRef id=\"q_flash_sqlite_crud_v1\" />\n<QuestionFamily familyId=\"qf_long_sqlite_crud_example\" />\n",
        "level": 1,
        "section_id": "section_34",
        "title": "7. 关闭连接"
      },
      {
        "content": "\n设计存储金融数据的数据库时，没有完美的方案，关键在于权衡。主要考虑三种策略：\n",
        "level": 2,
        "section_id": "section_35",
        "title": "金融数据存储的数据库设计 {#database-design}"
      },
      {
        "content": "\n- **优点**：结构简单，易于跨股票查询和分析。\n- **缺点**：表会非常庞大，查询和备份速度慢。\n",
        "level": 3,
        "section_id": "section_36",
        "title": "策略一：所有股票放在同一张大表"
      },
      {
        "content": "\n- **优点**：单只股票的回测查询非常快。\n- **缺点**：表数量庞大，跨股票分析需要查询多张表，代码复杂。\n",
        "level": 3,
        "section_id": "section_37",
        "title": "策略二：按股票代码分表"
      },
      {
        "content": "\n- **优点**：按时间范围查询快，便于月度数据管理。\n- **缺点**：跨月查询需要动态拼接 SQL，单只股票的完整历史查询较慢。\n\n<KeyPoint>\n**如何选择？**\n- **策略回测**：主要查询单只股票的完整历史，推荐**按股票分表**。\n- **数据分析**：主要进行跨股票的截面分析，推荐**所有股票放在同一张大表**并建立合适的索引。\n</KeyPoint>\n\n<QuestionRef id=\"q_flash_db_design_tradeoffs_v1\" />\n<QuestionFamily familyId=\"qf_long_db_design_analysis\" />\n",
        "level": 3,
        "section_id": "section_38",
        "title": "策略三：按时间（如月份）分表"
      },
      {
        "content": "\n1.  **忽略 User-Agent**：直接使用 `requests.get(url)` 可能会被服务器拒绝。务必设置 `headers` 模拟浏览器访问。\n2.  **混淆 `Close` 与 `Adj Close`**：进行长期回测时，必须使用 `Adj Close`，否则结果会严重失真。\n3.  **数据库设计一刀切**：没有最好的设计，只有最适合当前场景的设计。不要盲目选择一种策略，要根据主要用途（回测 vs 分析）来权衡。\n4.  **忘记提交事务**：在 SQLite 中执行 `INSERT`、`UPDATE`、`DELETE` 后，必须调用 `conn.commit()` 才能使更改生效。\n",
        "level": 2,
        "section_id": "section_39",
        "title": "易错点 {#pitfalls}"
      },
      {
        "content": "",
        "level": 2,
        "section_id": "section_40",
        "title": "复习与自测 {#review-path}"
      },
      {
        "content": "\n- 网页的三大技术是什么？它们在抓取中各扮演什么角色？\n- 比较 BeautifulSoup、Selenium 和 yfinance 的适用场景。\n- 什么是调整收盘价？为什么它对回测至关重要？\n- SQLite 的 CRUD 操作分别对应哪些 SQL 语句？\n- 在设计金融数据库时，按股票分表和按时间分表各有什么优缺点？\n",
        "level": 3,
        "section_id": "section_41",
        "title": "核心概念回顾"
      },
      {
        "content": "\n<QuestionFamily familyId=\"qf_flash_html_tags\" />\n<QuestionFamily familyId=\"qf_flash_web_tech_roles\" />\n<QuestionFamily familyId=\"qf_flash_bs4_methods\" />\n<QuestionFamily familyId=\"qf_flash_xpath_syntax\" />\n<QuestionFamily familyId=\"qf_flash_selenium_purpose\" />\n<QuestionFamily familyId=\"qf_flash_yfinance_usage\" />\n<QuestionFamily familyId=\"qf_flash_scraping_limits\" />\n<QuestionFamily familyId=\"qf_flash_adjusted_close_concept\" />\n<QuestionFamily familyId=\"qf_flash_sqlite_crud\" />\n<QuestionFamily familyId=\"qf_flash_db_design_tradeoffs\" />\n",
        "level": 3,
        "section_id": "section_42",
        "title": "自测题目"
      },
      {
        "content": "\n<QuestionFamily familyId=\"qf_long_web_structure_explain\" />\n<QuestionFamily familyId=\"qf_long_scraping_tool_choice\" />\n<QuestionFamily familyId=\"qf_long_scraping_limitations_explain\" />\n<QuestionFamily familyId=\"qf_long_adjusted_close_calculation\" />\n<QuestionFamily familyId=\"qf_long_sqlite_crud_example\" />\n<QuestionFamily familyId=\"qf_long_db_design_analysis\" />",
        "level": 3,
        "section_id": "section_43",
        "title": "深入练习"
      }
    ]
  }
}
</SCORER_INPUT_JSON>
