const fs = require("node:fs");
const path = require("node:path");
const { pathToFileURL } = require("node:url");
const { chromium } = require("playwright");
const { marked } = require("marked");

const reportRoot = path.resolve(__dirname, "..");
const markdownPath = path.join(reportRoot, "progress_report.md");
const htmlPath = path.join(reportRoot, "progress_report.html");
const pdfPath = path.join(reportRoot, "progress_report.pdf");

async function launchBrowser() {
  try {
    return await chromium.launch({ channel: "chrome", headless: true });
  } catch {
    return chromium.launch({
      executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
      headless: true
    });
  }
}

function renderDocument(body) {
  return `<!doctype html>
<html>
<head>
  <meta charset="utf-8">
  <title>Magy Progress Report</title>
  <style>
    @page {
      size: A4;
      margin: 16mm 14mm 18mm;
    }
    * {
      box-sizing: border-box;
    }
    body {
      background: #ffffff;
      color: #17201a;
      font-family: "Avenir Next", "Helvetica Neue", Arial, sans-serif;
      font-size: 10.5pt;
      line-height: 1.55;
      margin: 0;
    }
    html {
      background: #ffffff;
    }
    h1, h2, h3 {
      color: #07110c;
      line-height: 1.18;
      margin: 0 0 8px;
    }
    h1 {
      border-bottom: 2px solid #111;
      font-size: 26pt;
      margin-bottom: 14px;
      padding-bottom: 10px;
    }
    h2 {
      border-top: 1px solid #cfd7d1;
      font-size: 16pt;
      margin-top: 24px;
      padding-top: 14px;
    }
    h3 {
      font-size: 12pt;
      margin-top: 18px;
    }
    p {
      margin: 7px 0;
    }
    ul, ol {
      margin: 7px 0 10px 22px;
      padding: 0;
    }
    li {
      margin: 3px 0;
    }
    table {
      border-collapse: collapse;
      font-size: 9.5pt;
      margin: 10px 0 14px;
      width: 100%;
    }
    th, td {
      border: 1px solid #cfd7d1;
      padding: 6px 8px;
      vertical-align: top;
    }
    th {
      background: #eef4ef;
      font-weight: 700;
      text-align: left;
    }
    code {
      background: #eef4ef;
      border-radius: 3px;
      font-family: "SFMono-Regular", Consolas, monospace;
      font-size: 0.92em;
      padding: 1px 3px;
    }
    img {
      border: 1px solid #c5cec8;
      border-radius: 6px;
      display: block;
      margin: 12px auto 5px;
      max-height: 128mm;
      max-width: 100%;
      object-fit: contain;
    }
    p:has(img) {
      break-inside: avoid;
      margin-top: 14px;
    }
    em {
      color: #526158;
      display: block;
      font-size: 9.2pt;
      margin-bottom: 10px;
      text-align: center;
    }
    strong {
      font-weight: 700;
    }
  </style>
</head>
<body>
${body}
</body>
</html>`;
}

(async () => {
  marked.setOptions({ gfm: true });

  const markdown = fs.readFileSync(markdownPath, "utf8");
  const html = renderDocument(marked.parse(markdown));
  fs.writeFileSync(htmlPath, html);

  const browser = await launchBrowser();
  try {
    const page = await browser.newPage();
    await page.goto(pathToFileURL(htmlPath).href, { waitUntil: "networkidle" });
    await page.pdf({
      path: pdfPath,
      format: "A4",
      printBackground: true,
      preferCSSPageSize: true
    });
  } finally {
    await browser.close();
  }
})();
