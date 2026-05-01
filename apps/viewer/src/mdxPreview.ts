import type { QuestionFamily, QuestionVariant } from "./types";

export type MdxPreviewIndex = {
  families: Map<string, QuestionFamily>;
  questions: Map<string, { family: QuestionFamily; variant: QuestionVariant }>;
};

const titledBlocks = ["Definition", "Example", "Checkpoint"];
const calloutBlocks = ["KeyPoint", "Pitfall"];

const escapeRegExp = (value: string) => value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");

const extractAttr = (attrs: string, name: string) => {
  const match = attrs.match(new RegExp(`${name}=["']([^"']+)["']`));
  return match?.[1];
};

const cleanMdxBody = (body: string) =>
  body
    .trim()
    .replace(/<Term\b([^>]*)>([\s\S]*?)<\/Term>/g, (_match, attrs, content) => {
      const label = String(content).replace(/<[^>]+>/g, "").trim();
      const id = extractAttr(attrs, "id");
      const en = extractAttr(attrs, "en");
      const suffix = [id ? `term:${id}` : null, en].filter(Boolean).join(" / ");
      return `**${label || id || "Term"}**${suffix ? ` (${suffix})` : ""}`;
    });

const questionKind = (family?: QuestionFamily) => {
  const id = family?.family_id ?? "";
  if (id.includes("_quiz_")) {
    return "Quiz";
  }
  if (id.includes("_long_")) {
    return "Longform";
  }
  return "Active recall";
};

const renderQuestionRef = (id: string, index?: MdxPreviewIndex) => {
  const entry = index?.questions.get(id);
  if (!entry) {
    return `\n\n**Practice item** \`${id}\`\n\n`;
  }

  const { family, variant } = entry;
  const prompt = variant.front ?? variant.stem ?? variant.prompt_blocks?.join(" / ") ?? id;
  const answer = variant.back ?? variant.explanation ?? variant.reference_answer?.join(" ");
  const optionLines = variant.options?.map((option, optionIndex) => {
    const marker = optionIndex === variant.answer ? " *" : "";
    return `  ${optionIndex + 1}. ${option}${marker}`;
  });

  return [
    "",
    `**${questionKind(family)}** \`${id}\``,
    "",
    `- Prompt: ${prompt}`,
    answer ? `- Answer: ${answer}` : null,
    optionLines?.length ? `- Options:\n${optionLines.join("\n")}` : null,
    ""
  ]
    .filter(Boolean)
    .join("\n");
};

const renderQuestionFamily = (familyId: string, index?: MdxPreviewIndex) => {
  const family = index?.families.get(familyId);
  if (!family) {
    return `\n\n**Practice set** \`${familyId}\`\n\n`;
  }

  return [
    "",
    `**${questionKind(family)} set** \`${familyId}\``,
    "",
    family.learning_goal ? `- Goal: ${family.learning_goal}` : null,
    family.retrieval_focus ? `- Retrieval focus: ${family.retrieval_focus}` : null,
    `- Type: ${family.question_type ?? "unknown"}`,
    `- Variants: ${family.variants?.length ?? 0}`,
    ""
  ]
    .filter(Boolean)
    .join("\n");
};

export function mdxToMarkdownPreview(source: string, index?: MdxPreviewIndex) {
  let output = source.replace(/^---[\s\S]*?---\s*/, "");
  output = output.replace(/^(#{1,6}\s+.+?)\s+\{#[^}]+}\s*$/gm, "$1");

  output = output.replace(/<Term\b([^>]*)>([\s\S]*?)<\/Term>/g, (_match, attrs, content) => {
    const label = content.replace(/<[^>]+>/g, "").trim();
    const id = extractAttr(attrs, "id");
    const en = extractAttr(attrs, "en");
    const suffix = [id ? `term:${id}` : null, en].filter(Boolean).join(" / ");
    return `**${label || id || "Term"}**${suffix ? ` (${suffix})` : ""}`;
  });

  output = output.replace(/<QuestionRef\b([^>]*)\s*\/>/g, (_match, attrs) => {
    const id = extractAttr(attrs, "id") ?? "unknown";
    return renderQuestionRef(id, index);
  });

  output = output.replace(/<QuestionFamily\b([^>]*)\s*\/>/g, (_match, attrs) => {
    const familyId = extractAttr(attrs, "familyId") ?? extractAttr(attrs, "id") ?? "unknown";
    return renderQuestionFamily(familyId, index);
  });

  for (const component of titledBlocks) {
    const name = escapeRegExp(component);
    output = output.replace(
      new RegExp(`<${name}\\b([^>]*)>([\\s\\S]*?)<\\/${name}>`, "g"),
      (_match, attrs, body) => {
        const title = extractAttr(attrs, "title");
        const label = component === "Checkpoint" ? "Checkpoint" : component;
        return `\n\n### ${label}${title ? `: ${title}` : ""}\n\n${cleanMdxBody(String(body))}\n\n`;
      }
    );
  }

  for (const component of calloutBlocks) {
    const name = escapeRegExp(component);
    output = output.replace(
      new RegExp(`<${name}\\b([^>]*)>([\\s\\S]*?)<\\/${name}>`, "g"),
      (_match, _attrs, body) => `\n\n**${component === "KeyPoint" ? "Key point" : "Pitfall"}:** ${cleanMdxBody(String(body))}\n\n`
    );
  }

  output = output.replace(/<Formula\b([^>]*)>([\s\S]*?)<\/Formula>/g, (_match, attrs, body) => {
    const latex = extractAttr(attrs, "latex");
    return `\n\n### Formula\n\n${latex ? `\`${latex}\`\n\n` : ""}${cleanMdxBody(String(body))}\n\n`;
  });

  output = output.replace(/<Figure\b([^>]*)>([\s\S]*?)<\/Figure>/g, (_match, attrs, body) => {
    const src = extractAttr(attrs, "src");
    const alt = extractAttr(attrs, "alt");
    const caption = cleanMdxBody(String(body));
    return `\n\n### Figure${alt ? `: ${alt}` : ""}\n\n${src ? `\`${src}\`\n\n` : ""}${caption}\n\n`;
  });

  output = output.replace(/<([A-Z][A-Za-z0-9_]*)\b([^>]*)>([\s\S]*?)<\/\1>/g, (_match, name, attrs, body) => {
    const title = extractAttr(attrs, "title") ?? extractAttr(attrs, "id");
    return `\n\n### ${name}${title ? `: ${title}` : ""}\n\n${cleanMdxBody(String(body))}\n\n`;
  });

  output = output.replace(/<([A-Z][A-Za-z0-9_]*)\b([^>]*)\s*\/>/g, (_match, name, attrs) => {
    const id = extractAttr(attrs, "id") ?? extractAttr(attrs, "familyId");
    return `\n\n**${name}**${id ? ` \`${id}\`` : ""}\n\n`;
  });

  return output;
}
