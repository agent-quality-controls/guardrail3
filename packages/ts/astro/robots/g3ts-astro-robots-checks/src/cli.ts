#!/usr/bin/env node

import { validateRobotsTxt } from "./index.js";

interface CliOptions {
  outputDir?: string;
  site?: string;
  approvedSitemapUrls: string[];
}

let options: CliOptions;

try {
  options = parseArgs(process.argv.slice(2));
} catch (error) {
  console.error(error instanceof Error ? error.message : String(error));
  printUsage();
  process.exit(2);
}

if (!options.outputDir || !options.site || options.approvedSitemapUrls.length === 0) {
  printUsage();
  process.exitCode = 2;
} else {
  const result = await validateRobotsTxt({
    outputDir: options.outputDir,
    site: options.site,
    approvedSitemapUrls: options.approvedSitemapUrls
  });

  if (result.ok) {
    console.log("robots.txt passed G3TS robots checks");
  } else {
    for (const issue of result.issues) {
      console.error(`${issue.code}: ${issue.message}`);
    }
    process.exitCode = 1;
  }
}

function parseArgs(args: readonly string[]): CliOptions {
  const options: CliOptions = {
    approvedSitemapUrls: []
  };

  for (let index = 0; index < args.length; index += 1) {
    const arg = args[index];
    const value = args[index + 1];

    if (arg === "--output-dir" && value) {
      options.outputDir = value;
      index += 1;
      continue;
    }

    if (arg === "--site" && value) {
      options.site = value;
      index += 1;
      continue;
    }

    if (arg === "--sitemap" && value) {
      options.approvedSitemapUrls.push(value);
      index += 1;
      continue;
    }

    if (arg === "--help" || arg === "-h") {
      printUsage();
      process.exit(0);
    }

    throw new Error(`Unknown or incomplete argument: ${arg}`);
  }

  return options;
}

function printUsage(): void {
  console.error(
    [
      "Usage:",
      "  g3ts-astro-robots-checks --output-dir dist --site https://example.com --sitemap https://example.com/sitemap-index.xml",
      "",
      "Options:",
      "  --output-dir <dir> Generated Astro output directory containing robots.txt",
      "  --site <url>       Canonical HTTPS site URL",
      "  --sitemap <url>    Approved sitemap URL; repeat for multiple approved URLs"
    ].join("\n")
  );
}
