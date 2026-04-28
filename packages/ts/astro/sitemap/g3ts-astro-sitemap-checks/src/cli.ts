#!/usr/bin/env node
import { checkSitemap, type SitemapCheckConfig } from "./index.js";

interface ParsedArgs {
  config?: SitemapCheckConfig;
  help: boolean;
}

const usage = `Usage:
  g3ts-astro-sitemap-checks --site https://example.com --output-dir dist [--index-filename sitemap-index.xml]

Options:
  --site <url>             Canonical HTTPS site origin.
  --output-dir <dir>       Generated output directory containing the sitemap index.
  --index-filename <name>  Sitemap index filename under output dir. Defaults to sitemap-index.xml.
  --help                  Show this help text.
`;

async function main(): Promise<void> {
  const parsed = parseArgs(process.argv.slice(2));
  if (parsed.help) {
    process.stdout.write(usage);
    return;
  }

  if (!parsed.config) {
    throw new Error("Missing sitemap checker config.");
  }

  const result = await checkSitemap(parsed.config);

  if (result.ok) {
    process.stdout.write(
      `g3ts-astro-sitemap-checks: ok (${result.files.length} files, ${result.locs.length} locs)\n`
    );
    return;
  }

  for (const finding of result.findings) {
    const location = [finding.file, finding.loc].filter(Boolean).join(" ");
    process.stderr.write(
      `${finding.code}: ${finding.message}${location ? ` (${location})` : ""}\n`
    );
  }

  process.exitCode = 1;
}

function parseArgs(args: string[]): ParsedArgs {
  const config: Partial<SitemapCheckConfig> = { site: "" };

  for (let index = 0; index < args.length; index += 1) {
    const arg = args[index];
    if (arg === "--help" || arg === "-h") {
      return { help: true };
    }

    if (arg === "--site") {
      config.site = requiredValue(args, (index += 1), arg);
      continue;
    }

    if (arg === "--output-dir") {
      config.outputDir = requiredValue(args, (index += 1), arg);
      continue;
    }

    if (arg === "--index-filename") {
      config.indexFilename = requiredValue(args, (index += 1), arg);
      continue;
    }

    throw new Error(`Unknown argument: ${arg}`);
  }

  if (!config.outputDir) {
    throw new Error("--output-dir requires a value.");
  }

  return { config: config as SitemapCheckConfig, help: false };
}

function requiredValue(args: string[], index: number, flag: string): string {
  const value = args[index];
  if (!value || value.startsWith("--")) {
    throw new Error(`${flag} requires a value.`);
  }

  return value;
}

main().catch((error: unknown) => {
  process.stderr.write(`${error instanceof Error ? error.message : String(error)}\n`);
  process.exitCode = 1;
});
