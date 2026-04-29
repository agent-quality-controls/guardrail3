import assert from "node:assert/strict";
import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { pathToFileURL } from "node:url";

import g3tsSitemapAuditor, { checkSitemap } from "../src/index.js";

const packageRoot = path.resolve(import.meta.dirname, "..");

test("valid sitemap index recursively validates child sitemap", async () => {
  const root = await tempOutput({
    "sitemap-index.xml": sitemapIndex(["https://example.com/sitemap-0.xml"]),
    "sitemap-0.xml": urlset(["https://example.com/", "https://example.com/about/"])
  });

  const result = await checkSitemap({
    site: "https://example.com",
    trailingSlash: "always",
    allowedExtraUrls: ["**"],
    outputDir: root
  });

  assert.equal(result.ok, true);
  assert.deepEqual(result.findings, []);
  assert.deepEqual(
    result.files.map((file) => path.basename(file)).sort(),
    ["sitemap-0.xml", "sitemap-index.xml"]
  );
  assert.deepEqual(result.locs.sort(), [
    "https://example.com/",
    "https://example.com/about/",
    "https://example.com/sitemap-0.xml"
  ]);
});

test("invalid XML fails parsing", async () => {
  const outputDir = await tempSitemapOutput("<urlset><url></urlset>");
  const result = await checkSitemap({
    site: "https://example.com",
    trailingSlash: "always",
    allowedExtraUrls: ["**"],
    outputDir
  });

  assertFinding(result, "xml_parse");
});

test("unsupported sitemap root fails", async () => {
  const outputDir = await tempSitemapOutput("<feed><url><loc>https://example.com/</loc></url></feed>");
  const result = await checkSitemap({
    site: "https://example.com",
    trailingSlash: "always",
    allowedExtraUrls: ["**"],
    outputDir
  });

  assertFinding(result, "unsupported_root");
});

test("missing loc fails", async () => {
  const outputDir = await tempSitemapOutput(urlsetRaw("<url></url>"));
  const result = await checkSitemap({
    site: "https://example.com",
    trailingSlash: "always",
    allowedExtraUrls: ["**"],
    outputDir
  });

  assertFinding(result, "loc_missing");
});

test("relative loc fails", async () => {
  const outputDir = await tempSitemapOutput(urlsetRaw("<url><loc>/relative</loc></url>"));
  const result = await checkSitemap({
    site: "https://example.com",
    trailingSlash: "always",
    allowedExtraUrls: ["**"],
    outputDir
  });

  assertFinding(result, "loc_foreign_host");
});

test("sitemap index target outside output root fails", async () => {
  const outputDir = await tempSitemapOutput(
    sitemapIndex(["https://example.com/%2F..%2Fevil.xml"])
  );
  const result = await checkSitemap({
    site: "https://example.com",
    trailingSlash: "always",
    allowedExtraUrls: ["**"],
    outputDir
  });

  assertFinding(result, "sitemap_index_target");
});

test("sitemap index recursion reports missing child file", async () => {
  const root = await tempOutput({
    "sitemap-index.xml": sitemapIndex(["https://example.com/missing.xml"])
  });

  const result = await checkSitemap({
    site: "https://example.com",
    trailingSlash: "always",
    allowedExtraUrls: ["**"],
    outputDir: root
  });

  assertFinding(result, "missing_file");
});

test("every loc must use exact configured HTTPS host", async () => {
  const outputDir = await tempSitemapOutput(urlset(["https://example.com:444/"]));
  const result = await checkSitemap({
    site: "https://example.com",
    trailingSlash: "always",
    allowedExtraUrls: ["**"],
    outputDir
  });

  assertFinding(result, "loc_host_mismatch");
});

test("http loc is rejected", async () => {
  const outputDir = await tempSitemapOutput(urlset(["http://example.com/"]));
  const result = await checkSitemap({
    site: "https://example.com",
    trailingSlash: "always",
    allowedExtraUrls: ["**"],
    outputDir
  });

  assertFinding(result, "loc_http");
});

test("foreign host loc is rejected", async () => {
  const outputDir = await tempSitemapOutput(urlset(["https://elsewhere.example/"]));
  const result = await checkSitemap({
    site: "https://example.com",
    trailingSlash: "always",
    allowedExtraUrls: ["**"],
    outputDir
  });

  assertFinding(result, "loc_foreign_host");
});

test("bare and www host variants cannot be mixed", async () => {
  const outputDir = await tempSitemapOutput(
    urlset(["https://example.com/", "https://www.example.com/about"])
  );
  const result = await checkSitemap({
    site: "https://example.com",
    trailingSlash: "always",
    allowedExtraUrls: ["**"],
    outputDir
  });

  assertFinding(result, "loc_bare_www_mixing");
});

test("duplicate loc is rejected", async () => {
  const outputDir = await tempSitemapOutput(
    urlset(["https://example.com/about", "https://example.com/about"])
  );
  const result = await checkSitemap({
    site: "https://example.com",
    trailingSlash: "always",
    allowedExtraUrls: ["**"],
    outputDir
  });

  assertFinding(result, "loc_duplicate");
});

test("slash and no-slash pair is rejected", async () => {
  const outputDir = await tempSitemapOutput(
    urlset(["https://example.com/about", "https://example.com/about/"])
  );
  const result = await checkSitemap({
    site: "https://example.com",
    trailingSlash: "always",
    allowedExtraUrls: ["**"],
    outputDir
  });

  assertFinding(result, "loc_slash_pair");
});

test("custom index filename is explicit", async () => {
  const root = await tempOutput({
    "sitemap.xml": urlset(["https://example.com/"])
  });

  const result = await checkSitemap({
    site: "https://example.com",
    trailingSlash: "always",
    allowedExtraUrls: ["**"],
    outputDir: root,
    indexFilename: "sitemap.xml"
  });

  assert.equal(result.ok, true);
  assert.deepEqual(result.findings, []);
});

test("package exposes Astro integration and library API only", async () => {
  const packageJson = JSON.parse(
    await fs.readFile(path.join(packageRoot, "package.json"), "utf8")
  ) as {
    name?: string;
    bin?: Record<string, string>;
    dependencies?: Record<string, string>;
    exports?: Record<string, unknown>;
  };

  assert.equal(packageJson.name, "g3ts-astro-sitemap-auditor");
  assert.equal(packageJson.bin, undefined);
  assert.equal(typeof packageJson.dependencies?.["fast-xml-parser"], "string");
  assert.equal(typeof packageJson.dependencies?.["minimatch"], "string");
  assert.deepEqual(Object.keys(packageJson.exports ?? {}), ["."]);
});

test("Astro integration throws on invalid generated sitemap", async () => {
  const outputDir = await tempSitemapOutput(urlset(["http://example.com/"]));
  const integration = g3tsSitemapAuditor({
    site: "https://example.com",
    trailingSlash: "always"
  });

  await assert.rejects(
    integration.hooks["astro:build:done"]?.({
      pages: [],
      dir: pathToFileURL(`${outputDir}/`),
      assets: new Map(),
      logger: {} as never
    }),
    /loc_http/
  );
});

test("Astro integration rejects missing required config at construction", () => {
  assert.throws(
    () => g3tsSitemapAuditor({} as never),
    /site must be a non-empty HTTPS URL string/
  );
});

test("Astro integration rejects unknown config keys at construction", () => {
  assert.throws(
    () =>
      g3tsSitemapAuditor({
        site: "https://example.com",
        trailingSlash: "always",
        extra: true
      } as never),
    /unknown key/
  );
});

test("fails when a built HTML page is missing from sitemap", async () => {
  const root = await tempOutput({
    "sitemap-index.xml": urlset(["https://example.com/"]),
    "missing/index.html": "<html></html>"
  });

  const result = await checkSitemap({
    site: "https://example.com",
    trailingSlash: "always",
    outputDir: root
  });

  assertFinding(result, "html_page_missing_from_sitemap");
});

test("fails when a sitemap URL does not map to a built HTML page", async () => {
  const root = await tempOutput({
    "sitemap-index.xml": urlset(["https://example.com/extra/"])
  });

  const result = await checkSitemap({
    site: "https://example.com",
    trailingSlash: "always",
    outputDir: root
  });

  assertFinding(result, "sitemap_url_missing_html_page");
});

test("allows explicit sitemap and built-page coverage exceptions", async () => {
  const root = await tempOutput({
    "sitemap-index.xml": urlset(["https://example.com/extra/"]),
    "missing/index.html": "<html></html>"
  });

  const result = await checkSitemap({
    site: "https://example.com",
    trailingSlash: "always",
    outputDir: root,
    allowedMissingRoutes: ["/missing/"],
    allowedExtraUrls: ["/extra/"]
  });

  assert.equal(result.ok, true);
  assert.deepEqual(result.findings, []);
});

async function tempSitemapOutput(xml: string): Promise<string> {
  const outputDir = await fs.mkdtemp(path.join(os.tmpdir(), "g3ts-sitemap-output-"));
  await fs.writeFile(path.join(outputDir, "sitemap-index.xml"), xml, "utf8");
  return outputDir;
}

function assertFinding(
  result: Awaited<ReturnType<typeof checkSitemap>>,
  code: string
): void {
  assert.equal(result.ok, false);
  assert.equal(
    result.findings.some((finding) => finding.code === code),
    true,
    `expected finding ${code}, got ${JSON.stringify(result.findings, null, 2)}`
  );
}

async function tempOutput(files: Record<string, string>): Promise<string> {
  const root = await fs.mkdtemp(path.join(os.tmpdir(), "g3ts-sitemap-"));
  await Promise.all(
    Object.entries(files).map(async ([name, contents]) => {
      const filePath = path.join(root, name);
      await fs.mkdir(path.dirname(filePath), { recursive: true });
      await fs.writeFile(filePath, contents);
    })
  );
  return root;
}

function sitemapIndex(locs: string[]): string {
  return `<?xml version="1.0" encoding="UTF-8"?>
<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${locs.map((loc) => `  <sitemap><loc>${loc}</loc></sitemap>`).join("\n")}
</sitemapindex>`;
}

function urlset(locs: string[]): string {
  return urlsetRaw(locs.map((loc) => `  <url><loc>${loc}</loc></url>`).join("\n"));
}

function urlsetRaw(items: string): string {
  return `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${items}
</urlset>`;
}
