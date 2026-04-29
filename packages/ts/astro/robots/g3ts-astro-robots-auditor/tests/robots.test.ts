import assert from "node:assert/strict";
import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { pathToFileURL } from "node:url";

import {
  default as g3tsRobotsAuditor,
  validateRobotsTxt,
  validateRobotsTxtContent,
  type RobotsCheckCode
} from "../src/index.js";

const packageRoot = path.resolve(import.meta.dirname, "..");

const baseConfig = {
  site: "https://example.com",
  sitemapUrls: ["https://example.com/sitemap-index.xml"]
};

test("valid generated robots.txt passes", () => {
  const result = validateRobotsTxtContent(
    [
      "User-agent: *",
      "Allow: /",
      "Sitemap: https://example.com/sitemap-index.xml"
    ].join("\n"),
    baseConfig
  );

  assert.equal(result.ok, true);
  assert.deepEqual(result.issues, []);
  assert.deepEqual(result.sitemapUrls, [
    "https://example.com/sitemap-index.xml"
  ]);
});

test("multiple explicitly approved sitemap URLs pass", () => {
  const result = validateRobotsTxtContent(
    [
      "User-agent: *",
      "Allow: /",
      "Sitemap: https://example.com/sitemap-index.xml",
      "Sitemap: https://example.com/pages-sitemap.xml"
    ].join("\n"),
    {
      ...baseConfig,
      sitemapUrls: [
        "https://example.com/sitemap-index.xml",
        "https://example.com/pages-sitemap.xml"
      ]
    }
  );

  assert.equal(result.ok, true);
  assert.deepEqual(result.issues, []);
});

test("missing robots.txt fails", async () => {
  const tempDir = await fs.mkdtemp(path.join(os.tmpdir(), "g3ts-robots-"));
  const result = await validateRobotsTxt({
    ...baseConfig,
    outputDir: path.join(tempDir, "dist")
  });

  assert.equal(result.ok, false);
  assertCodes(result, ["robots-missing"]);
});

test("unparseable sitemap directive fails robots parse check", () => {
  const result = validateRobotsTxtContent(
    ["User-agent: *", "Sitemap: not-a-url"].join("\n"),
    baseConfig
  );

  assert.equal(result.ok, false);
  assertCodes(result, ["robots-parse-error", "sitemap-unapproved"]);
});

test("wrong sitemap path fails exact approved set check", () => {
  const result = validateRobotsTxtContent(
    ["User-agent: *", "Sitemap: https://example.com/wrong.xml"].join("\n"),
    baseConfig
  );

  assert.equal(result.ok, false);
  assertCodes(result, ["sitemap-unapproved", "sitemap-unapproved"]);
});

test("missing sitemap fails exact approved set check", () => {
  const result = validateRobotsTxtContent("User-agent: *\nAllow: /", baseConfig);

  assert.equal(result.ok, false);
  assertCodes(result, ["sitemap-count-mismatch", "sitemap-unapproved"]);
});

test("extra sitemap fails exact approved set check", () => {
  const result = validateRobotsTxtContent(
    [
      "User-agent: *",
      "Sitemap: https://example.com/sitemap-index.xml",
      "Sitemap: https://example.com/extra.xml"
    ].join("\n"),
    baseConfig
  );

  assert.equal(result.ok, false);
  assertCodes(result, ["sitemap-count-mismatch", "sitemap-unapproved"]);
});

test("http sitemap fails", () => {
  const result = validateRobotsTxtContent(
    ["User-agent: *", "Sitemap: http://example.com/sitemap-index.xml"].join(
      "\n"
    ),
    baseConfig
  );

  assert.equal(result.ok, false);
  assertCodes(result, [
    "sitemap-http",
    "sitemap-unapproved",
    "sitemap-unapproved"
  ]);
});

test("wrong sitemap host fails", () => {
  const result = validateRobotsTxtContent(
    ["User-agent: *", "Sitemap: https://attacker.example/sitemap-index.xml"].join(
      "\n"
    ),
    baseConfig
  );

  assert.equal(result.ok, false);
  assertCodes(result, [
    "sitemap-wrong-host",
    "sitemap-unapproved",
    "sitemap-unapproved"
  ]);
});

test("wrong sitemap port fails exact configured host check", () => {
  const result = validateRobotsTxtContent(
    ["User-agent: *", "Sitemap: https://example.com:444/sitemap-index.xml"].join(
      "\n"
    ),
    {
      ...baseConfig,
      sitemapUrls: ["https://example.com:444/sitemap-index.xml"]
    }
  );

  assert.equal(result.ok, false);
  assertCodes(result, ["sitemap-wrong-host"]);
});

test("non-canonical www variant fails for bare canonical site", () => {
  const result = validateRobotsTxtContent(
    ["User-agent: *", "Sitemap: https://www.example.com/sitemap-index.xml"].join(
      "\n"
    ),
    baseConfig
  );

  assert.equal(result.ok, false);
  assertCodes(result, [
    "sitemap-non-canonical-host",
    "sitemap-unapproved",
    "sitemap-unapproved"
  ]);
});

test("non-canonical bare variant fails for www canonical site", () => {
  const result = validateRobotsTxtContent(
    ["User-agent: *", "Sitemap: https://example.com/sitemap-index.xml"].join(
      "\n"
    ),
    {
      ...baseConfig,
      site: "https://www.example.com",
      sitemapUrls: ["https://www.example.com/sitemap-index.xml"]
    }
  );

  assert.equal(result.ok, false);
  assertCodes(result, [
    "sitemap-non-canonical-host",
    "sitemap-unapproved",
    "sitemap-unapproved"
  ]);
});

test("duplicate sitemap URLs fail", () => {
  const result = validateRobotsTxtContent(
    [
      "User-agent: *",
      "Sitemap: https://example.com/sitemap-index.xml",
      "Sitemap: https://example.com/sitemap-index.xml"
    ].join("\n"),
    baseConfig
  );

  assert.equal(result.ok, false);
  assertCodes(result, ["sitemap-duplicate", "sitemap-count-mismatch"]);
});

test("Astro integration passes for valid robots.txt", async () => {
  const tempDir = await fs.mkdtemp(path.join(os.tmpdir(), "g3ts-robots-"));
  const outputDir = path.join(tempDir, "dist");
  await fs.mkdir(outputDir);
  const robotsFilePath = path.join(outputDir, "robots.txt");
  await fs.writeFile(
    robotsFilePath,
    "User-agent: *\nSitemap: https://example.com/sitemap-index.xml\n",
    "utf8"
  );
  const integration = g3tsRobotsAuditor(baseConfig);

  await integration.hooks["astro:build:done"]?.({
    pages: [],
    dir: pathToFileURL(`${outputDir}/`),
    assets: new Map(),
    logger: {} as never
  });
});

test("Astro integration throws on invalid generated robots.txt", async () => {
  const tempDir = await fs.mkdtemp(path.join(os.tmpdir(), "g3ts-robots-"));
  const outputDir = path.join(tempDir, "dist");
  await fs.mkdir(outputDir);
  await fs.writeFile(
    path.join(outputDir, "robots.txt"),
    "User-agent: *\nSitemap: http://example.com/sitemap-index.xml\n",
    "utf8"
  );
  const integration = g3tsRobotsAuditor(baseConfig);

  await assert.rejects(
    integration.hooks["astro:build:done"]?.({
      pages: [],
      dir: pathToFileURL(`${outputDir}/`),
      assets: new Map(),
      logger: {} as never
    }),
    /sitemap-http/
  );
});

test("Astro integration rejects missing required config at construction", () => {
  assert.throws(
    () => g3tsRobotsAuditor({} as never),
    /site must be a non-empty HTTPS URL string/
  );
});

test("Astro integration rejects unknown config keys at construction", () => {
  assert.throws(
    () =>
      g3tsRobotsAuditor({
        ...baseConfig,
        approvedSitemapUrls: baseConfig.sitemapUrls
      } as never),
    /unknown key/
  );
});

test("package metadata exposes Astro integration and library API", async () => {
  const packageJson = JSON.parse(
    await fs.readFile(path.join(packageRoot, "package.json"), "utf8")
  ) as {
    name?: string;
    bin?: Record<string, string>;
    dependencies?: Record<string, string>;
  };

  assert.equal(packageJson.name, "g3ts-astro-robots-auditor");
  assert.equal(packageJson.bin, undefined);
  assert.equal(packageJson.dependencies?.["robots-parser"], "3.0.1");
});

function assertCodes(
  result: { issues: Array<{ code: RobotsCheckCode }> },
  expected: RobotsCheckCode[]
): void {
  assert.deepEqual(
    result.issues.map((issue) => issue.code).sort(),
    [...expected].sort()
  );
}
