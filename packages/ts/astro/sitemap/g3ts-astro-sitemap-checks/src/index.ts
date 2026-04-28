import fs from "node:fs/promises";
import path from "node:path";

import { XMLParser, XMLValidator } from "fast-xml-parser";

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
  | "loc_slash_pair";

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
  const canonicalSite = parseCanonicalSite(config.site, findings);
  const roots = resolveEntryFiles(config);

  if (!canonicalSite || roots.length === 0) {
    return toResult(findings, [], []);
  }

  const visited = new Set<string>();
  const allLocs: LocSite[] = [];

  for (const root of roots) {
    await visitSitemapFile({
      filePath: root.filePath,
      sitemapRoot: root.sitemapRoot,
      canonicalSite,
      visited,
      allLocs,
      findings
    });
  }

  validateLocSet(allLocs, canonicalSite, findings);

  return toResult(
    findings,
    [...visited].sort(),
    allLocs.map((site) => site.loc)
  );
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
  findings: SitemapFinding[]
): void {
  const byLoc = new Map<string, LocSite>();
  const bySlashless = new Map<string, LocSite>();
  const hostsByBare = new Map<string, Set<string>>();

  for (const site of sites) {
    validateLocHost(site, canonicalSite, findings);

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
