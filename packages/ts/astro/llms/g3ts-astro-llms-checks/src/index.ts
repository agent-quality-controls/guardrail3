import fs from "node:fs/promises";
import path from "node:path";

import { fromMarkdown } from "mdast-util-from-markdown";
import type { Content, Link, List, ListItem, Paragraph, Root, Text } from "mdast";
import type { LlmsTxtFile } from "parse-llms-txt";

export interface LlmsChecksConfig {
  outputDir: string;
  requiredSections: string[];
  requiredLinks: string[];
}

export type LlmsCheckCode =
  | "llms-file-missing"
  | "llms-parse-failed"
  | "llms-markdown-structure-invalid"
  | "llms-required-section-missing"
  | "llms-required-link-missing";

export interface LlmsCheckFinding {
  code: LlmsCheckCode;
  message: string;
  path: string;
  expected?: string;
}

export interface LlmsCheckResult {
  ok: boolean;
  findings: LlmsCheckFinding[];
}

export async function checkLlmsTxt(
  config: LlmsChecksConfig
): Promise<LlmsCheckResult> {
  const findings: LlmsCheckFinding[] = [];
  const llmsPath = path.join(config.outputDir, "llms.txt");
  let source: string;

  try {
    source = await fs.readFile(llmsPath, "utf8");
  } catch (error) {
    if (isMissingFileError(error)) {
      return {
        ok: false,
        findings: [
          {
            code: "llms-file-missing",
            message: `Configured llms.txt file does not exist: ${llmsPath}`,
            path: llmsPath
          }
        ]
      };
    }

    throw error;
  }

  let parsed: LlmsTxtFile;

  try {
    const { parseLlmsTxt } = await import("parse-llms-txt");
    parsed = parseLlmsTxt(source);
    if (parsed.title.trim() === "") {
      throw new Error("llms.txt is missing its required H1 title");
    }
  } catch (error) {
    return {
      ok: false,
      findings: [
        {
          code: "llms-parse-failed",
          message: `Configured llms.txt file did not parse: ${formatError(error)}`,
          path: llmsPath
        }
      ]
    };
  }

  findings.push(...validateMarkdownStructure(source, llmsPath));

  const sectionNames = new Set(parsed.sections.map((section) => section.name));
  const linkUrls = new Set(
    parsed.sections.flatMap((section) => section.files.map((file) => file.url))
  );

  for (const requiredSection of config.requiredSections) {
    if (!sectionNames.has(requiredSection)) {
      findings.push({
        code: "llms-required-section-missing",
        message: `Configured required llms.txt section is missing: ${requiredSection}`,
        path: llmsPath,
        expected: requiredSection
      });
    }
  }

  for (const requiredLink of config.requiredLinks) {
    if (!linkUrls.has(requiredLink)) {
      findings.push({
        code: "llms-required-link-missing",
        message: `Configured required llms.txt link is missing: ${requiredLink}`,
        path: llmsPath,
        expected: requiredLink
      });
    }
  }

  return {
    ok: findings.length === 0,
    findings
  };
}

function isMissingFileError(error: unknown): boolean {
  return (
    typeof error === "object" &&
    error !== null &&
    "code" in error &&
    error.code === "ENOENT"
  );
}

function validateMarkdownStructure(
  source: string,
  llmsPath: string
): LlmsCheckFinding[] {
  const root = fromMarkdown(source);
  const findings: LlmsCheckFinding[] = [];

  for (const node of root.children) {
    collectBrokenLinkText(node, llmsPath, findings);
  }

  for (const list of sectionLists(root)) {
    for (const item of list.children) {
      if (!listItemStartsWithLink(item)) {
        findings.push({
          code: "llms-markdown-structure-invalid",
          message:
            "Every llms.txt list item inside a section must start with a valid Markdown link.",
          path: llmsPath
        });
      }
    }
  }

  return findings;
}

function sectionLists(root: Root): List[] {
  const lists: List[] = [];
  let insideSection = false;

  for (const node of root.children) {
    if (node.type === "heading" && node.depth <= 2) {
      insideSection = node.depth === 2;
      continue;
    }

    if (insideSection && node.type === "list") {
      lists.push(node);
    }
  }

  return lists;
}

function listItemStartsWithLink(item: ListItem): boolean {
  const firstChild = item.children[0];
  if (!firstChild || firstChild.type !== "paragraph") {
    return false;
  }

  return firstSignificantPhrasingNode(firstChild)?.type === "link";
}

function firstSignificantPhrasingNode(paragraph: Paragraph): Link | Text | undefined {
  for (const child of paragraph.children) {
    if (child.type === "text" && child.value.trim() === "") {
      continue;
    }
    if (child.type === "link" || child.type === "text") {
      return child;
    }
    return undefined;
  }

  return undefined;
}

function collectBrokenLinkText(
  node: Content,
  llmsPath: string,
  findings: LlmsCheckFinding[]
): void {
  if (node.type === "text" && looksLikeBrokenMarkdownLink(node.value)) {
    findings.push({
      code: "llms-markdown-structure-invalid",
      message: `Malformed Markdown link text in llms.txt: ${node.value.trim()}`,
      path: llmsPath
    });
    return;
  }

  if ("children" in node && Array.isArray(node.children)) {
    for (const child of node.children) {
      collectBrokenLinkText(child, llmsPath, findings);
    }
  }
}

function looksLikeBrokenMarkdownLink(value: string): boolean {
  return value.includes("[") || value.includes("](") || value.includes("] (");
}

function formatError(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}
