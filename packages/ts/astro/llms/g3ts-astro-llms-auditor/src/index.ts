import fs from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { fromMarkdown } from "mdast-util-from-markdown";
import { minimatch } from "minimatch";
import type { Content, Link, List, ListItem, Paragraph, Root, Text } from "mdast";
import type { LlmsTxtFile } from "parse-llms-txt";
import type { AstroIntegration } from "astro";

export interface LlmsChecksConfig {
  outputDir: string;
  site: string;
  requiredSections: readonly string[];
  requiredRoutePatterns: readonly string[];
  allowedExternalUrls: readonly string[];
  allowedNonPageUrls: readonly string[];
  ignoredHtmlFiles: readonly string[];
}

export interface G3TsLlmsAuditorConfig {
  site: string;
  requiredSections: readonly string[];
  requiredRoutePatterns: readonly string[];
  allowedExternalUrls: readonly string[];
  allowedNonPageUrls: readonly string[];
  ignoredHtmlFiles: readonly string[];
}

export type LlmsCheckCode =
  | "llms-file-missing"
  | "llms-parse-failed"
  | "llms-markdown-structure-invalid"
  | "llms-required-section-missing"
  | "llms-required-route-pattern-missing"
  | "llms-external-link-unapproved"
  | "llms-internal-link-missing-page";

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
  const canonicalSite = parseSite(config.site);
  const builtPageUrls = await collectBuiltHtmlPageUrls({
    outputDir: config.outputDir,
    canonicalSite,
    ignoredHtmlFiles: config.ignoredHtmlFiles
  });

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

  for (const requiredPattern of config.requiredRoutePatterns) {
    if (!linkMatchesPattern(linkUrls, requiredPattern)) {
      findings.push({
        code: "llms-required-route-pattern-missing",
        message: `Configured required llms.txt route pattern is missing: ${requiredPattern}`,
        path: llmsPath,
        expected: requiredPattern
      });
    }
  }

  findings.push(
    ...validateLinks({
      linkUrls,
      builtPageUrls,
      canonicalSite,
      llmsPath,
      allowedExternalUrls: config.allowedExternalUrls,
      allowedNonPageUrls: config.allowedNonPageUrls
    })
  );

  return {
    ok: findings.length === 0,
    findings
  };
}

export default function g3tsLlmsAuditor(config: G3TsLlmsAuditorConfig): AstroIntegration {
  const auditorConfig = validateAuditorConfig(config);
  return {
    name: "g3ts-astro-llms-auditor",
    hooks: {
      "astro:build:done": async ({ dir }) => {
        const result = await checkLlmsTxt({
          ...auditorConfig,
          outputDir: fileURLToPath(dir)
        });
        if (!result.ok) {
          throw new Error(formatLlmsFailure(result));
        }
      }
    }
  };
}

function validateAuditorConfig(config: G3TsLlmsAuditorConfig): G3TsLlmsAuditorConfig {
  const properties = assertObject(config);
  rejectUnknownKeys(properties, [
    "site",
    "requiredSections",
    "requiredRoutePatterns",
    "allowedExternalUrls",
    "allowedNonPageUrls",
    "ignoredHtmlFiles"
  ]);
  if (typeof config.site !== "string" || config.site.trim() === "") {
    throw new Error("Invalid g3ts-astro-llms-auditor config: site must be a non-empty HTTPS URL string.");
  }
  const site = parseConfigSite(config.site);
  if (site.protocol !== "https:" || site.hostname.length === 0) {
    throw new Error("Invalid g3ts-astro-llms-auditor config: site must be a non-empty HTTPS URL string.");
  }
  assertRequiredStringArray(config.requiredSections, "requiredSections");
  assertRequiredStringArray(config.requiredRoutePatterns, "requiredRoutePatterns");
  assertRequiredStringArray(config.allowedExternalUrls, "allowedExternalUrls");
  assertRequiredStringArray(config.allowedNonPageUrls, "allowedNonPageUrls");
  assertRequiredStringArray(config.ignoredHtmlFiles, "ignoredHtmlFiles");
  return config;
}

function assertObject(value: unknown): Record<string, unknown> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw new Error("Invalid g3ts-astro-llms-auditor config: config must be an object.");
  }
  return value as Record<string, unknown>;
}

function parseConfigSite(value: string): URL {
  try {
    return new URL(value);
  } catch {
    throw new Error("Invalid g3ts-astro-llms-auditor config: site must be a non-empty HTTPS URL string.");
  }
}

function rejectUnknownKeys(
  properties: Record<string, unknown>,
  allowedKeys: readonly string[]
): void {
  const unknownKeys = Object.keys(properties).filter((key) => !allowedKeys.includes(key));
  if (unknownKeys.length > 0) {
    throw new Error(`Invalid g3ts-astro-llms-auditor config: unknown key(s): ${unknownKeys.join(", ")}.`);
  }
}

function assertRequiredStringArray(value: unknown, key: string): void {
  if (!Array.isArray(value) || value.some((item) => typeof item !== "string")) {
    throw new Error(`Invalid g3ts-astro-llms-auditor config: ${key} must be an array of strings.`);
  }
}

function formatLlmsFailure(result: LlmsCheckResult): string {
  return [
    `g3ts-astro-llms-auditor: ${result.findings.length} finding(s)`,
    ...result.findings.map((finding) => `${finding.code}: ${finding.message} (${finding.path})`)
  ].join("\n");
}

function parseSite(site: string): URL {
  const url = new URL(site);
  if (url.protocol !== "https:") {
    throw new Error("site must be an HTTPS URL");
  }
  return url;
}

function linkMatchesPattern(linkUrls: Set<string>, pattern: string): boolean {
  for (const linkUrl of linkUrls) {
    const url = new URL(linkUrl);
    if (minimatch(url.pathname, pattern) || minimatch(linkUrl, pattern)) {
      return true;
    }
  }
  return false;
}

function validateLinks(input: {
  linkUrls: Set<string>;
  builtPageUrls: Set<string>;
  canonicalSite: URL;
  llmsPath: string;
  allowedExternalUrls: readonly string[];
  allowedNonPageUrls: readonly string[];
}): LlmsCheckFinding[] {
  const findings: LlmsCheckFinding[] = [];
  for (const linkUrl of input.linkUrls) {
    let parsed: URL;
    try {
      parsed = new URL(linkUrl);
    } catch {
      findings.push({
        code: "llms-markdown-structure-invalid",
        message: `llms.txt link is not an absolute URL: ${linkUrl}`,
        path: input.llmsPath,
        expected: linkUrl
      });
      continue;
    }

    if (parsed.protocol !== "https:") {
      findings.push({
        code: "llms-external-link-unapproved",
        message: `llms.txt link must use HTTPS: ${linkUrl}`,
        path: input.llmsPath,
        expected: linkUrl
      });
      continue;
    }

    if (parsed.host !== input.canonicalSite.host) {
      if (!matchesAny(linkUrl, input.allowedExternalUrls)) {
        findings.push({
          code: "llms-external-link-unapproved",
          message: `llms.txt external link must use HTTPS and be explicitly allowed: ${linkUrl}`,
          path: input.llmsPath,
          expected: linkUrl
        });
      }
      continue;
    }

    if (input.builtPageUrls.has(parsed.toString())) {
      continue;
    }
    if (matchesAny(linkUrl, input.allowedNonPageUrls) || matchesAny(parsed.pathname, input.allowedNonPageUrls)) {
      continue;
    }

    findings.push({
      code: "llms-internal-link-missing-page",
      message: `llms.txt internal link does not map to a built HTML page: ${linkUrl}`,
      path: input.llmsPath,
      expected: linkUrl
    });
  }
  return findings;
}

async function collectBuiltHtmlPageUrls(input: {
  outputDir: string;
  canonicalSite: URL;
  ignoredHtmlFiles: readonly string[];
}): Promise<Set<string>> {
  const files = await collectHtmlFiles(input.outputDir);
  const urls = files.flatMap((file) => {
    const relPath = toPosixPath(path.relative(input.outputDir, file));
    if (matchesAny(relPath, input.ignoredHtmlFiles)) {
      return [];
    }
    return [new URL(htmlRelPathToRoutePath(relPath), input.canonicalSite).toString()];
  });
  return new Set(urls);
}

async function collectHtmlFiles(root: string): Promise<string[]> {
  let entries: import("node:fs").Dirent[];
  try {
    entries = await fs.readdir(root, { withFileTypes: true });
  } catch {
    return [];
  }

  const files: string[] = [];
  for (const entry of entries) {
    const entryPath = path.join(root, entry.name);
    if (entry.isDirectory()) {
      files.push(...(await collectHtmlFiles(entryPath)));
      continue;
    }
    if (entry.isFile() && entry.name.endsWith(".html")) {
      files.push(entryPath);
    }
  }
  return files;
}

function htmlRelPathToRoutePath(relPath: string): string {
  if (relPath === "index.html") {
    return "/";
  }
  if (relPath.endsWith("/index.html")) {
    return `/${relPath.slice(0, -"index.html".length)}`;
  }
  return `/${relPath.slice(0, -".html".length)}`;
}

function matchesAny(value: string, patterns: readonly string[]): boolean {
  return patterns.some((pattern) => minimatch(value, pattern));
}

function toPosixPath(value: string): string {
  return value.split(path.sep).join("/");
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
