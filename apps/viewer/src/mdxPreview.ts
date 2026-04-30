const blockComponents = [
  "QuestionFamily",
  "QuestionRef",
  "Figure",
  "KeyPoint",
  "Checkpoint",
  "Example",
  "Definition",
  "Formula"
];

const escapeRegExp = (value: string) => value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");

const extractAttr = (attrs: string, name: string) => {
  const match = attrs.match(new RegExp(`${name}=["']([^"']+)["']`));
  return match?.[1];
};

const summarizeAttrs = (attrs: string) => {
  const bits = ["id", "title", "en", "src", "alt"].map((name) => {
    const value = extractAttr(attrs, name);
    return value ? `${name}: ${value}` : null;
  });
  return bits.filter(Boolean).join(" | ");
};

export function mdxToMarkdownPreview(source: string) {
  let output = source.replace(/^---[\s\S]*?---\s*/, "");

  output = output.replace(/<Term\b([^>]*)>([\s\S]*?)<\/Term>/g, (_match, attrs, content) => {
    const label = content.replace(/<[^>]+>/g, "").trim();
    const id = extractAttr(attrs, "id");
    return `**${label || id || "Term"}**${id ? ` \`term:${id}\`` : ""}`;
  });

  for (const component of blockComponents) {
    const name = escapeRegExp(component);
    output = output.replace(
      new RegExp(`<${name}\\b([^>]*)>([\\s\\S]*?)<\\/${name}>`, "g"),
      (_match, attrs, body) => {
        const summary = summarizeAttrs(attrs);
        const cleanedBody = String(body).trim();
        return `\n\n> ${component}${summary ? ` (${summary})` : ""}\n>\n${cleanedBody
          .split("\n")
          .map((line) => `> ${line}`)
          .join("\n")}\n\n`;
      }
    );
    output = output.replace(new RegExp(`<${name}\\b([^>]*)\\s*\\/>`, "g"), (_match, attrs) => {
      const summary = summarizeAttrs(attrs);
      return `\n\n> ${component}${summary ? ` (${summary})` : ""}\n\n`;
    });
  }

  output = output.replace(/<([A-Z][A-Za-z0-9_]*)\b([^>]*)>([\s\S]*?)<\/\1>/g, (_match, name, attrs, body) => {
    const summary = summarizeAttrs(attrs);
    return `\n\n> ${name}${summary ? ` (${summary})` : ""}\n>\n${String(body)
      .trim()
      .split("\n")
      .map((line) => `> ${line}`)
      .join("\n")}\n\n`;
  });

  output = output.replace(/<([A-Z][A-Za-z0-9_]*)\b([^>]*)\s*\/>/g, (_match, name, attrs) => {
    const summary = summarizeAttrs(attrs);
    return `\n\n> ${name}${summary ? ` (${summary})` : ""}\n\n`;
  });

  return output;
}
