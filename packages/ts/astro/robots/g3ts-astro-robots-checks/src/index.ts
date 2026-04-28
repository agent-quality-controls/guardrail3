import fs from "node:fs/promises";
import path from "node:path";

import robotsParser from "robots-parser";

export type RobotsCheckCode =
  | "robots-missing"
  | "robots-parse-error"
  | "sitemap-count-mismatch"
  | "sitemap-duplicate"
  | "sitemap-http"
  | "sitemap-wrong-host"
  | "sitemap-non-canonical-host"
  | "sitemap-unapproved";

export interface RobotsCheckConfig {
  outputDir: string;
  site: string;
  approvedSitemapUrls: readonly string[];
}

export interface RobotsContentCheckConfig {
  site: string;
  approvedSitemapUrls: readonly string[];
}

export interface RobotsCheckIssue {
  code: RobotsCheckCode;
  message: string;
  value?: string;
}

export interface RobotsCheckResult {
  ok: boolean;
  sitemapUrls: string[];
  issues: RobotsCheckIssue[];
}

export async function validateRobotsTxt(
  config: RobotsCheckConfig
): Promise<RobotsCheckResult> {
  const normalized = normalizeConfig(config);
  const robotsFilePath = path.join(config.outputDir, "robots.txt");
  let robotsTxt: string;

  try {
    robotsTxt = await fs.readFile(robotsFilePath, "utf8");
  } catch (error) {
    if (isNodeError(error) && error.code === "ENOENT") {
      return result([], [
        {
          code: "robots-missing",
          message: `robots.txt does not exist at ${robotsFilePath}`
        }
      ]);
    }

    throw error;
  }

  return validateRobotsTxtContent(robotsTxt, normalized);
}

export function validateRobotsTxtContent(
  robotsTxt: string,
  config: RobotsContentCheckConfig
): RobotsCheckResult {
  const normalized = normalizeConfig(config);
  const robotsUrl = new URL("/robots.txt", normalized.siteUrl).toString();
  let sitemapUrls: string[];

  try {
    const parsed = robotsParser(robotsUrl, robotsTxt);
    sitemapUrls = parsed.getSitemaps();
  } catch (error) {
    return result([], [
      {
        code: "robots-parse-error",
        message: `robots.txt could not be parsed: ${formatError(error)}`
      }
    ]);
  }

  const issues = validateSitemaps(sitemapUrls, normalized);
  return result(sitemapUrls, issues);
}

interface NormalizedConfig extends RobotsContentCheckConfig {
  siteUrl: URL;
  canonicalHostWithPort: string;
  approvedSitemapSet: Set<string>;
}

function normalizeConfig(config: RobotsContentCheckConfig): NormalizedConfig {
  if (config.approvedSitemapUrls.length === 0) {
    throw new Error("approvedSitemapUrls must contain at least one URL");
  }

  const siteUrl = new URL(config.site);
  if (siteUrl.protocol !== "https:") {
    throw new Error("site must be an HTTPS URL");
  }

  const approvedSitemapUrls = config.approvedSitemapUrls.map((url) =>
    new URL(url).toString()
  );

  return {
    site: config.site,
    approvedSitemapUrls,
    siteUrl,
    canonicalHostWithPort: siteUrl.host,
    approvedSitemapSet: new Set(approvedSitemapUrls)
  };
}

function validateSitemaps(
  sitemapUrls: readonly string[],
  config: NormalizedConfig
): RobotsCheckIssue[] {
  const issues: RobotsCheckIssue[] = [];
  const seen = new Set<string>();
  const duplicates = new Set<string>();

  for (const sitemapUrl of sitemapUrls) {
    if (seen.has(sitemapUrl)) {
      duplicates.add(sitemapUrl);
    }
    seen.add(sitemapUrl);
  }

  for (const duplicate of duplicates) {
    issues.push({
      code: "sitemap-duplicate",
      message: `robots.txt contains duplicate Sitemap URL ${duplicate}`,
      value: duplicate
    });
  }

  if (sitemapUrls.length !== config.approvedSitemapUrls.length) {
    issues.push({
      code: "sitemap-count-mismatch",
      message: `robots.txt contains ${sitemapUrls.length} Sitemap URL(s), expected ${config.approvedSitemapUrls.length}`
    });
  }

  for (const sitemapUrl of sitemapUrls) {
    const parsed = parseSitemapUrl(sitemapUrl, issues);
    if (!parsed) {
      continue;
    }

    if (parsed.protocol === "http:") {
      issues.push({
        code: "sitemap-http",
        message: `Sitemap URL must use HTTPS: ${sitemapUrl}`,
        value: sitemapUrl
      });
    }

    if (isBareWwwVariant(parsed.host, config.canonicalHostWithPort)) {
      issues.push({
        code: "sitemap-non-canonical-host",
        message: `Sitemap URL uses non-canonical bare/www host variant: ${sitemapUrl}`,
        value: sitemapUrl
      });
    } else if (parsed.host !== config.canonicalHostWithPort) {
      issues.push({
        code: "sitemap-wrong-host",
        message: `Sitemap URL host ${parsed.host} does not match canonical host ${config.canonicalHostWithPort}`,
        value: sitemapUrl
      });
    }

    if (!config.approvedSitemapSet.has(parsed.toString())) {
      issues.push({
        code: "sitemap-unapproved",
        message: `Sitemap URL is not approved: ${sitemapUrl}`,
        value: sitemapUrl
      });
    }
  }

  for (const approvedUrl of config.approvedSitemapUrls) {
    if (!seen.has(approvedUrl)) {
      issues.push({
        code: "sitemap-unapproved",
        message: `Approved Sitemap URL is missing from robots.txt: ${approvedUrl}`,
        value: approvedUrl
      });
    }
  }

  return issues;
}

function parseSitemapUrl(
  sitemapUrl: string,
  issues: RobotsCheckIssue[]
): URL | null {
  try {
    return new URL(sitemapUrl);
  } catch {
    issues.push({
      code: "robots-parse-error",
      message: `Sitemap directive is not an absolute URL: ${sitemapUrl}`,
      value: sitemapUrl
    });
    return null;
  }
}

function isBareWwwVariant(host: string, canonicalHost: string): boolean {
  return stripWww(host) === stripWww(canonicalHost) && host !== canonicalHost;
}

function stripWww(host: string): string {
  return host.startsWith("www.") ? host.slice(4) : host;
}

function result(
  sitemapUrls: string[],
  issues: RobotsCheckIssue[]
): RobotsCheckResult {
  return {
    ok: issues.length === 0,
    sitemapUrls,
    issues
  };
}

function isNodeError(error: unknown): error is NodeJS.ErrnoException {
  return error instanceof Error && "code" in error;
}

function formatError(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}
