import fs from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import type { Dirent } from "node:fs";

import { XMLParser, XMLValidator } from "fast-xml-parser";
import { minimatch } from "minimatch";
import type { AstroIntegration } from "astro";

export type SitemapFindingCode =
  | "invalid_config"
  | "missing_file"
  | "xml_parse"
  | "unsupported_root"
  | "loc_missing"
  | "sitemap_index_target"
  | "loc_http"
  | "loc_foreign_host"
  | "loc_host_mismatch"
  | "loc_bare_www_mixing"
  | "loc_duplicate"
  | "loc_slash_pair"
  | "loc_trailing_slash_policy"
  | "html_page_missing_from_sitemap"
  | "sitemap_url_missing_html_page";

export interface SitemapFinding {
  code: SitemapFindingCode;
  message: string;
  file?: string;
  loc?: string;
  relatedLoc?: string;
}

export interface SitemapCheckConfig {
  site: string;
  outputDir: string;
  indexFilename?: string;
  trailingSlash: "always" | "never";
  allowedMissingRoutes?: readonly string[];
  allowedExtraUrls?: readonly string[];
  ignoredHtmlFiles?: readonly string[];
}

export interface G3TsSitemapAuditorConfig {
  site: string;
  indexFilename?: string;
  trailingSlash: "always" | "never";
  allowedMissingRoutes?: readonly string[];
  allowedExtraUrls?: readonly string[];
  ignoredHtmlFiles?: readonly string[];
}

export interface SitemapCheckResult {
  ok: boolean;
  findings: SitemapFinding[];
  files: string[];
  locs: string[];
}

interface ParsedSitemap {
  kind: "sitemapindex" | "urlset";
  locs: string[];
}

interface LocSite {
  loc: string;
  file: string;
  url: URL;
}

const DEFAULT_INDEX_FILENAME = "sitemap-index.xml";

const parser = new XMLParser({
  ignoreAttributes: false,
  parseAttributeValue: false,
  parseTagValue: false,
  preserveOrder: false,
  processEntities: true,
  trimValues: true
});

export async function checkSitemap(
  config: SitemapCheckConfig
): Promise<SitemapCheckResult> {
  const findings: SitemapFinding[] = [];
  validateSitemapConfig(config, findings);
  const canonicalSite = parseCanonicalSite(config.site, findings);
  const roots = resolveEntryFiles(config);

  if (!canonicalSite || roots.length === 0) {
    return toResult(findings, [], []);
  }

  const visited = new Set<string>();
  const allLocs: LocSite[] = [];
  const pageLocs: LocSite[] = [];

  for (const root of roots) {
    await visitSitemapFile({
      filePath: root.filePath,
      sitemapRoot: root.sitemapRoot,
      canonicalSite,
      visited,
      allLocs,
      pageLocs,
      findings
    });
  }

  validateLocSet(allLocs, canonicalSite, config.trailingSlash, findings);
  await validateBuiltPages({
    config,
    canonicalSite,
    pageLocs,
    findings
  });

  return toResult(
    findings,
    [...visited].sort(),
    allLocs.map((site) => site.loc)
  );
}

function validateSitemapConfig(
  config: SitemapCheckConfig,
  findings: SitemapFinding[]
): void {
  if (config.trailingSlash !== "always" && config.trailingSlash !== "never") {
    findings.push({
      code: "invalid_config",
      message: "`trailingSlash` must be exactly \"always\" or \"never\"."
    });
  }
}

export default function g3tsSitemapAuditor(
  config: G3TsSitemapAuditorConfig
): AstroIntegration {
  const auditorConfig = validateAuditorConfig(config);
  return {
    name: "g3ts-astro-sitemap-auditor",
    hooks: {
      "astro:build:done": async ({ dir }) => {
        const result = await checkSitemap({
          ...auditorConfig,
          outputDir: fileURLToPath(dir)
        });
        if (!result.ok) {
          throw new Error(formatSitemapFailure(result));
        }
      }
    }
  };
}

function validateAuditorConfig(
  config: G3TsSitemapAuditorConfig
): G3TsSitemapAuditorConfig {
  const properties = assertObject(config);
  const allowedKeys = [
    "site",
    "indexFilename",
    "trailingSlash",
    "allowedMissingRoutes",
    "allowedExtraUrls",
    "ignoredHtmlFiles"
  ];
  rejectUnknownKeys(properties, allowedKeys);

  if (typeof config.site !== "string" || config.site.trim() === "") {
    throw new Error("Invalid g3ts-astro-sitemap-auditor config: site must be a non-empty HTTPS URL string.");
  }
  const site = parseConfigUrl(config.site);
  if (site.protocol !== "https:" || site.hostname.length === 0) {
    throw new Error("Invalid g3ts-astro-sitemap-auditor config: site must be a non-empty HTTPS URL string.");
  }
  if (config.trailingSlash !== "always" && config.trailingSlash !== "never") {
    throw new Error("Invalid g3ts-astro-sitemap-auditor config: trailingSlash must be \"always\" or \"never\".");
  }
  if (config.indexFilename !== undefined && typeof config.indexFilename !== "string") {
    throw new Error("Invalid g3ts-astro-sitemap-auditor config: indexFilename must be a string when provided.");
  }
  assertStringArray(config.allowedMissingRoutes, "allowedMissingRoutes");
  assertStringArray(config.allowedExtraUrls, "allowedExtraUrls");
  assertStringArray(config.ignoredHtmlFiles, "ignoredHtmlFiles");
  return config;
}

function assertObject(value: unknown): Record<string, unknown> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw new Error("Invalid g3ts-astro-sitemap-auditor config: config must be an object.");
  }
  return value as Record<string, unknown>;
}

function parseConfigUrl(value: string): URL {
  try {
    return new URL(value);
  } catch {
    throw new Error("Invalid g3ts-astro-sitemap-auditor config: site must be a non-empty HTTPS URL string.");
  }
}

function rejectUnknownKeys(
  properties: Record<string, unknown>,
  allowedKeys: readonly string[]
): void {
  const unknownKeys = Object.keys(properties).filter((key) => !allowedKeys.includes(key));
  if (unknownKeys.length > 0) {
    throw new Error(`Invalid g3ts-astro-sitemap-auditor config: unknown key(s): ${unknownKeys.join(", ")}.`);
  }
}

function assertStringArray(value: unknown, key: string): void {
  if (value === undefined) {
    return;
  }
  if (!Array.isArray(value) || value.some((item) => typeof item !== "string")) {
    throw new Error(`Invalid g3ts-astro-sitemap-auditor config: ${key} must be an array of strings when provided.`);
  }
}

function formatSitemapFailure(result: SitemapCheckResult): string {
  return [
    `g3ts-astro-sitemap-auditor: ${result.findings.length} finding(s)`,
    ...result.findings.map((finding) => {
      const location = [finding.file, finding.loc].filter(Boolean).join(" ");
      return `${finding.code}: ${finding.message}${location ? ` (${location})` : ""}`;
    })
  ].join("\n");
}

function parseCanonicalSite(
  site: string,
  findings: SitemapFinding[]
): URL | undefined {
  let parsed: URL;
  try {
    parsed = new URL(site);
  } catch {
    findings.push({
      code: "invalid_config",
      message: "`site` must be a valid absolute HTTPS URL."
    });
    return undefined;
  }

  if (parsed.protocol !== "https:" || parsed.hostname.length === 0) {
    findings.push({
      code: "invalid_config",
      message: "`site` must use HTTPS and include a host."
    });
    return undefined;
  }

  return parsed;
}

function resolveEntryFiles(
  config: SitemapCheckConfig
): Array<{ filePath: string; sitemapRoot: string }> {
  const sitemapRoot = path.resolve(config.outputDir);
  return [
    {
      filePath: path.join(
        sitemapRoot,
        config.indexFilename ?? DEFAULT_INDEX_FILENAME
      ),
      sitemapRoot
    }
  ];
}

async function visitSitemapFile(input: {
  filePath: string;
  sitemapRoot: string;
  canonicalSite: URL;
  visited: Set<string>;
  allLocs: LocSite[];
  pageLocs: LocSite[];
  findings: SitemapFinding[];
}): Promise<void> {
  const resolvedFile = path.resolve(input.filePath);
  if (input.visited.has(resolvedFile)) {
    return;
  }

  input.visited.add(resolvedFile);

  let xml: string;
  try {
    xml = await fs.readFile(resolvedFile, "utf8");
  } catch {
    input.findings.push({
      code: "missing_file",
      message: `Sitemap file does not exist or cannot be read: ${resolvedFile}`,
      file: resolvedFile
    });
    return;
  }

  const parsed = parseSitemapXml(xml, resolvedFile, input.findings);
  if (!parsed) {
    return;
  }

  for (const loc of parsed.locs) {
    const url = parseLocUrl(loc, resolvedFile, input.findings);
    if (url) {
      input.allLocs.push({ loc, file: resolvedFile, url });
      if (parsed.kind === "urlset") {
        input.pageLocs.push({ loc, file: resolvedFile, url });
      }
    }
  }

  if (parsed.kind !== "sitemapindex") {
    return;
  }

  for (const loc of parsed.locs) {
    const url = parseLocUrl(loc, resolvedFile, input.findings);
    if (!url) {
      continue;
    }

    const nextFile = resolveSitemapIndexTarget(
      url,
      input.sitemapRoot,
      input.canonicalSite,
      resolvedFile,
      input.findings
    );
    if (!nextFile) {
      continue;
    }

    await visitSitemapFile({
      ...input,
      filePath: nextFile
    });
  }
}

function parseSitemapXml(
  xml: string,
  file: string,
  findings: SitemapFinding[]
): ParsedSitemap | undefined {
  const validation = XMLValidator.validate(xml);
  if (validation !== true) {
    findings.push({
      code: "xml_parse",
      message: `Sitemap XML failed to parse: ${validation.err.msg}`,
      file
    });
    return undefined;
  }

  const document = parser.parse(xml) as unknown;
  if (!isRecord(document)) {
    findings.push({
      code: "unsupported_root",
      message: "Sitemap XML root must be urlset or sitemapindex.",
      file
    });
    return undefined;
  }

  const root = getLocalEntry(document, "sitemapindex");
  if (root) {
    return {
      kind: "sitemapindex",
      locs: extractLocs(root.value, "sitemap", file, findings)
    };
  }

  const urlset = getLocalEntry(document, "urlset");
  if (urlset) {
    return {
      kind: "urlset",
      locs: extractLocs(urlset.value, "url", file, findings)
    };
  }

  findings.push({
    code: "unsupported_root",
    message: "Sitemap XML root must be urlset or sitemapindex.",
    file
  });
  return undefined;
}

function extractLocs(
  root: unknown,
  itemName: string,
  file: string,
  findings: SitemapFinding[]
): string[] {
  const items = collectChildValues(root, itemName);
  return items.flatMap((item) => {
    const locs = collectChildValues(item, "loc").flatMap((loc) => textValues(loc));
    if (locs.length === 0) {
      findings.push({
        code: "loc_missing",
        message: `Sitemap ${itemName} entry must contain a non-empty loc.`,
        file
      });
    }
    return locs;
  });
}

function collectChildValues(value: unknown, localName: string): unknown[] {
  if (!isRecord(value)) {
    return [];
  }

  return Object.entries(value).flatMap(([key, child]) => {
    if (toLocalName(key) !== localName) {
      return [];
    }

    return Array.isArray(child) ? child : [child];
  });
}

function textValues(value: unknown): string[] {
  if (typeof value === "string") {
    return [value.trim()].filter(Boolean);
  }

  if (typeof value === "number" || typeof value === "boolean") {
    return [String(value)];
  }

  if (Array.isArray(value)) {
    return value.flatMap((item) => textValues(item));
  }

  if (isRecord(value)) {
    const text = value["#text"];
    return textValues(text);
  }

  return [];
}

function parseLocUrl(
  loc: string,
  file: string,
  findings: SitemapFinding[]
): URL | undefined {
  try {
    return new URL(loc);
  } catch {
    findings.push({
      code: "loc_foreign_host",
      message: `Sitemap loc is not an absolute URL: ${loc}`,
      file,
      loc
    });
    return undefined;
  }
}

function resolveSitemapIndexTarget(
  url: URL,
  sitemapRoot: string,
  canonicalSite: URL,
  file: string,
  findings: SitemapFinding[]
): string | undefined {
  if (url.protocol !== "https:" || url.host !== canonicalSite.host) {
    findings.push({
      code: "sitemap_index_target",
      message: `Sitemap index target must use configured HTTPS host: ${url.href}`,
      file,
      loc: url.href
    });
    return undefined;
  }

  const targetPath = path.resolve(
    sitemapRoot,
    `.${decodeURIComponent(url.pathname)}`
  );
  const resolvedRoot = path.resolve(sitemapRoot);

  if (!isPathInside(targetPath, resolvedRoot)) {
    findings.push({
      code: "sitemap_index_target",
      message: `Sitemap index target resolves outside sitemap root: ${url.href}`,
      file,
      loc: url.href
    });
    return undefined;
  }

  return targetPath;
}

function validateLocSet(
  sites: LocSite[],
  canonicalSite: URL,
  trailingSlash: "always" | "never",
  findings: SitemapFinding[]
): void {
  const byLoc = new Map<string, LocSite>();
  const bySlashless = new Map<string, LocSite>();
  const hostsByBare = new Map<string, Set<string>>();

  for (const site of sites) {
    validateLocHost(site, canonicalSite, findings);
    validateTrailingSlashPolicy(site, trailingSlash, findings);

    const duplicate = byLoc.get(site.loc);
    if (duplicate) {
      findings.push({
        code: "loc_duplicate",
        message: `Duplicate sitemap loc: ${site.loc}`,
        file: site.file,
        loc: site.loc,
        relatedLoc: duplicate.loc
      });
    } else {
      byLoc.set(site.loc, site);
    }

    const slashless = slashlessKey(site.url);
    const slashPair = bySlashless.get(slashless);
    if (slashPair && slashPair.loc !== site.loc) {
      findings.push({
        code: "loc_slash_pair",
        message: `Sitemap contains slash/no-slash pair: ${slashPair.loc} and ${site.loc}`,
        file: site.file,
        loc: site.loc,
        relatedLoc: slashPair.loc
      });
    } else {
      bySlashless.set(slashless, site);
    }

    const bareHost = stripWww(site.url.host);
    const hostSet = hostsByBare.get(bareHost) ?? new Set<string>();
    hostSet.add(site.url.host);
    hostsByBare.set(bareHost, hostSet);
  }

  for (const [bareHost, hostSet] of hostsByBare) {
    if (hostSet.size <= 1 || !hostSet.has(bareHost)) {
      continue;
    }

    const variants = [...hostSet].sort();
    for (const site of sites) {
      if (stripWww(site.url.host) !== bareHost) {
        continue;
      }

      findings.push({
        code: "loc_bare_www_mixing",
        message: `Sitemap mixes bare and www host variants: ${variants.join(", ")}`,
        file: site.file,
        loc: site.loc
      });
    }
  }
}

function validateTrailingSlashPolicy(
  site: LocSite,
  trailingSlash: "always" | "never",
  findings: SitemapFinding[]
): void {
  const pathname = site.url.pathname;
  if (pathname === "/" || path.extname(pathname) !== "") {
    return;
  }

  const hasTrailingSlash = pathname.endsWith("/");
  if (trailingSlash === "always" && !hasTrailingSlash) {
    findings.push({
      code: "loc_trailing_slash_policy",
      message: `Sitemap loc must use trailing slash: ${site.loc}`,
      file: site.file,
      loc: site.loc
    });
  }
  if (trailingSlash === "never" && hasTrailingSlash) {
    findings.push({
      code: "loc_trailing_slash_policy",
      message: `Sitemap loc must not use trailing slash: ${site.loc}`,
      file: site.file,
      loc: site.loc
    });
  }
}

function validateLocHost(
  site: LocSite,
  canonicalSite: URL,
  findings: SitemapFinding[]
): void {
  if (site.url.protocol === "http:") {
    findings.push({
      code: "loc_http",
      message: `Sitemap loc must not use HTTP: ${site.loc}`,
      file: site.file,
      loc: site.loc
    });
  }

  if (site.url.host !== canonicalSite.host) {
    findings.push({
      code: "loc_foreign_host",
      message: `Sitemap loc host must match ${canonicalSite.host}: ${site.loc}`,
      file: site.file,
      loc: site.loc
    });
  }

  if (site.url.protocol !== "https:" || site.url.host !== canonicalSite.host) {
    findings.push({
      code: "loc_host_mismatch",
      message: `Sitemap loc must use exact configured HTTPS origin ${canonicalSite.origin}: ${site.loc}`,
      file: site.file,
      loc: site.loc
    });
  }
}

function slashlessKey(url: URL): string {
  const pathname =
    url.pathname.length > 1 && url.pathname.endsWith("/")
      ? url.pathname.slice(0, -1)
      : url.pathname;

  return `${url.protocol}//${url.host}${pathname}${url.search}`;
}

function stripWww(host: string): string {
  return host.startsWith("www.") ? host.slice("www.".length) : host;
}

async function validateBuiltPages(input: {
  config: SitemapCheckConfig;
  canonicalSite: URL;
  pageLocs: LocSite[];
  findings: SitemapFinding[];
}): Promise<void> {
  const builtPages = await collectBuiltHtmlPageUrls({
    outputDir: input.config.outputDir,
    canonicalSite: input.canonicalSite,
    trailingSlash: input.config.trailingSlash,
    ignoredHtmlFiles: input.config.ignoredHtmlFiles ?? []
  });
  const sitemapUrls = new Set(input.pageLocs.map((site) => site.url.toString()));
  const pageUrls = new Set(builtPages.map((page) => page.url));

  for (const page of builtPages) {
    if (sitemapUrls.has(page.url)) {
      continue;
    }
    if (matchesAny(page.url, input.config.allowedMissingRoutes ?? [])) {
      continue;
    }
    if (matchesAny(new URL(page.url).pathname, input.config.allowedMissingRoutes ?? [])) {
      continue;
    }

    input.findings.push({
      code: "html_page_missing_from_sitemap",
      message: `Built HTML page is missing from sitemap: ${page.url}`,
      file: page.file,
      loc: page.url
    });
  }

  for (const site of input.pageLocs) {
    const loc = site.url.toString();
    if (pageUrls.has(loc)) {
      continue;
    }
    if (matchesAny(loc, input.config.allowedExtraUrls ?? [])) {
      continue;
    }
    if (matchesAny(site.url.pathname, input.config.allowedExtraUrls ?? [])) {
      continue;
    }

    input.findings.push({
      code: "sitemap_url_missing_html_page",
      message: `Sitemap URL does not map to a built HTML page: ${loc}`,
      file: site.file,
      loc
    });
  }
}

async function collectBuiltHtmlPageUrls(input: {
  outputDir: string;
  canonicalSite: URL;
  trailingSlash: "always" | "never";
  ignoredHtmlFiles: readonly string[];
}): Promise<Array<{ file: string; url: string }>> {
  const files = await collectHtmlFiles(input.outputDir);
  return files.flatMap((file) => {
    const relPath = toPosixPath(path.relative(input.outputDir, file));
    if (matchesAny(relPath, input.ignoredHtmlFiles)) {
      return [];
    }

    const routePath = htmlRelPathToRoutePath(relPath, input.trailingSlash);
    const url = new URL(routePath, input.canonicalSite).toString();
    return [{ file, url }];
  });
}

async function collectHtmlFiles(root: string): Promise<string[]> {
  let entries: Dirent[];
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

function htmlRelPathToRoutePath(
  relPath: string,
  trailingSlash: "always" | "never"
): string {
  if (relPath === "index.html") {
    return "/";
  }

  if (relPath.endsWith("/index.html")) {
    const dirname = relPath.slice(0, -"index.html".length);
    return trailingSlash === "always" ? `/${dirname}` : `/${dirname.slice(0, -1)}`;
  }

  const withoutExt = relPath.slice(0, -".html".length);
  return trailingSlash === "always" ? `/${withoutExt}/` : `/${withoutExt}`;
}

function matchesAny(value: string, patterns: readonly string[]): boolean {
  return patterns.some((pattern) => minimatch(value, pattern));
}

function toPosixPath(value: string): string {
  return value.split(path.sep).join("/");
}

function getLocalEntry(
  record: Record<string, unknown>,
  localName: string
): { key: string; value: unknown } | undefined {
  for (const [key, value] of Object.entries(record)) {
    if (toLocalName(key) === localName) {
      return { key, value };
    }
  }

  return undefined;
}

function toLocalName(name: string): string {
  return name.includes(":") ? name.split(":").at(-1) ?? name : name;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function isPathInside(candidate: string, root: string): boolean {
  const relative = path.relative(root, candidate);
  return relative === "" || (!relative.startsWith("..") && !path.isAbsolute(relative));
}

function toResult(
  findings: SitemapFinding[],
  files: string[],
  locs: string[]
): SitemapCheckResult {
  return {
    ok: findings.length === 0,
    findings,
    files,
    locs
  };
}
