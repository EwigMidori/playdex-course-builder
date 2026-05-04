const fs = require("node:fs");
const path = require("node:path");
const { chromium } = require("playwright");

const reportRoot = path.resolve(__dirname, "..");
const screenshotsDir = path.join(reportRoot, "screenshots");

fs.mkdirSync(screenshotsDir, { recursive: true });

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

async function screenshot(page, filename, options = {}) {
  if (page.url().includes("5173")) {
    await applyMagyBranding(page);
  }
  await page.evaluate(() => window.scrollTo(0, 0));
  await page.waitForTimeout(options.delay ?? 450);
  await page.screenshot({
    path: path.join(screenshotsDir, filename),
    fullPage: options.fullPage ?? false
  });
}

async function clickText(page, text) {
  await page.getByText(text, { exact: false }).first().click();
}

async function applyMagyBranding(page) {
  await page.evaluate(() => {
    const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT);
    const replacements = [
      ["Playdex Player", "Magy"],
      ["Playdex", "Magy"]
    ];
    const nodes = [];
    while (walker.nextNode()) {
      nodes.push(walker.currentNode);
    }
    for (const node of nodes) {
      let value = node.nodeValue ?? "";
      for (const [from, to] of replacements) {
        value = value.replaceAll(from, to);
      }
      node.nodeValue = value;
    }
  });
}

async function openPlayerCourseHome(page) {
  await page.goto("http://127.0.0.1:5173/", { waitUntil: "networkidle" });
  await page.waitForSelector("text=Pick up your game");
  await clickText(page, "COMP7415A Algorithmic Trading");
  await page.waitForSelector("text=Verified Course Game");
}

async function capturePlayer(browser) {
  const page = await browser.newPage({ viewport: { width: 1440, height: 960 }, deviceScaleFactor: 1 });
  await page.goto("http://127.0.0.1:5173/", { waitUntil: "networkidle" });
  await page.evaluate(() => localStorage.clear());
  await page.reload({ waitUntil: "networkidle" });
  await page.waitForSelector("text=Pick up your game");
  await screenshot(page, "01_player_library.png");

  await clickText(page, "COMP7415A Algorithmic Trading");
  await page.waitForSelector("text=Verified Course Game");
  await screenshot(page, "02_player_course_home.png");

  await clickText(page, "Statistical Arbitrage and Pairs Trading");
  await page.waitForSelector("text=Scene Select");
  await screenshot(page, "03_player_chapter_route.png");

  await page.getByRole("button", { name: /Start Route/i }).click();
  await page.waitForSelector(".story-overlay");
  await screenshot(page, "04_player_story_overlay.png");

  await page.getByRole("button", { name: /Exit/i }).click();
  await page.waitForSelector("text=Scene Select");
  await page.getByRole("button", { name: /Read Textbook/i }).click();
  await page.waitForTimeout(2500);
  await screenshot(page, "05_player_textbook_room.png");

  await openPlayerCourseHome(page);
  await page.getByRole("button", { name: /Recall Center/i }).first().click();
  await page.waitForSelector("text=Recall Center");
  await screenshot(page, "06_player_recall_center.png");

  await openPlayerCourseHome(page);
  await page.getByRole("button", { name: /Open Vault/i }).first().click();
  await page.waitForSelector("text=The Vault");
  await screenshot(page, "07_player_vault.png");

  await page.close();
}

async function captureViewer(browser) {
  const page = await browser.newPage({ viewport: { width: 1440, height: 960 }, deviceScaleFactor: 1 });
  await page.goto("http://127.0.0.1:5174/", { waitUntil: "networkidle" });
  await page.waitForSelector("text=Course Builder Workspace");
  await clickText(page, "Data Scraping and Database Management");
  await page.waitForSelector("text=Lecture 2 / L2");
  await screenshot(page, "08_viewer_overview.png");

  await page.getByRole("button", { name: /MDX viewer/i }).click();
  await page.waitForSelector("text=Textbook preview");
  await screenshot(page, "09_viewer_mdx_preview.png");

  await page.getByRole("button", { name: /Story viewer/i }).click();
  await page.waitForSelector("text=Screens -");
  await screenshot(page, "10_viewer_story_review.png");

  await page.getByRole("button", { name: /Practice viewer/i }).click();
  await page.waitForSelector("text=Practice assets");
  await screenshot(page, "11_viewer_practice_review.png");

  await page.getByRole("button", { name: /Relevance/i }).click();
  await page.waitForSelector("text=Relevance");
  await screenshot(page, "12_viewer_relevance_scores.png");

  await page.close();
}

(async () => {
  const browser = await launchBrowser();
  try {
    await capturePlayer(browser);
    await captureViewer(browser);
  } finally {
    await browser.close();
  }
})();
