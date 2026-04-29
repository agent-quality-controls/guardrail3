import assert from "node:assert/strict";
import { execFile } from "node:child_process";
import fs from "node:fs/promises";
import path from "node:path";
import test from "node:test";
import { promisify } from "node:util";

import * as checksPackage from "../src/index.js";
import { structuredDataPresentCheck } from "../src/index.js";

const packageRoot = path.resolve(import.meta.dirname, "..");
const distRoot = path.join(packageRoot, "dist");
const execFileAsync = promisify(execFile);
const IMPORT_RE =
  /\b(?:import|export)\s+(?:[^"'`]*?\s+from\s+)?["']([^"'`]+)["']|\bimport\(\s*["']([^"'`]+)["']\s*\)/g;

test("package exports only the shared structured data presence check", () => {
  assert.deepEqual(Object.keys(checksPackage), ["structuredDataPresentCheck"]);
  assert.equal(structuredDataPresentCheck.kind, "page");
  assert.equal(structuredDataPresentCheck.id, "g3/structured-data-present");
  assert.equal(structuredDataPresentCheck.name, "Structured Data Present");
  assert.equal(structuredDataPresentCheck.domain, "seo");
  assert.equal(structuredDataPresentCheck.defaultSeverity, "error");
  assert.equal(structuredDataPresentCheck.essential, true);
});

test("structuredDataPresentCheck fails when page has no JSON-LD", async () => {
  const results = await structuredDataPresentCheck.run({
    pagePath: "/",
    filePath: "/dist/index.html",
    distDir: "/dist",
    html: "<html></html>",
    root: {} as never,
    pageData: {
      jsonLd: []
    } as never
  });

  assert.deepEqual(results, [
    {
      message: "Page is missing JSON-LD structured data",
      suggestion: "Render a schema-dts typed JSON-LD object in the page head."
    }
  ]);
});

test("structuredDataPresentCheck passes when page has JSON-LD", async () => {
  const results = await structuredDataPresentCheck.run({
    pagePath: "/",
    filePath: "/dist/index.html",
    distDir: "/dist",
    html: "<html></html>",
    root: {} as never,
    pageData: {
      jsonLd: [{ type: "Organization", raw: "{}", valid: true, line: 1 }]
    } as never
  });

  assert.deepEqual(results, []);
});

test("structuredDataPresentCheck relies on Nuasite extracted JSON-LD data", async () => {
  const results = await structuredDataPresentCheck.run({
    pagePath: "/",
    filePath: "/dist/index.html",
    distDir: "/dist",
    html: '<script type="application/ld+json">{"@type":"Organization"}</script>',
    root: {} as never,
    pageData: {
      jsonLd: []
    } as never
  });

  assert.equal(results.length, 1);
});

test("published package surface matches the G3TS Nuasite helper contract", async () => {
  const packageJson = JSON.parse(
    await fs.readFile(path.join(packageRoot, "package.json"), "utf8")
  ) as {
    name?: string;
    version?: string;
    dependencies?: Record<string, string>;
    peerDependencies?: Record<string, string>;
  };
  const distFiles = await collectJsFiles(distRoot);
  const runtimePackages = new Set<string>();

  for (const filePath of distFiles) {
    const source = await fs.readFile(filePath, "utf8");
    for (const specifier of collectBareSpecifiers(source)) {
      runtimePackages.add(specifier);
    }
  }

  assert.equal(packageJson.name, "g3ts-astro-nuasite-checks");
  assert.equal(packageJson.version, "0.1.2");
  assert.deepEqual(packageJson.dependencies ?? {}, {});
  assert.equal(packageJson.peerDependencies?.["@nuasite/checks"], "0.18.0");
  assert.deepEqual([...runtimePackages], []);
});

test("published package contains only dist and package metadata", async () => {
  const { stdout } = await execFileAsync(
    "npm",
    ["pack", "--dry-run", "--json", "--ignore-scripts"],
    { cwd: packageRoot }
  );
  const [packResult] = JSON.parse(stdout) as Array<{
    name: string;
    version: string;
    files: Array<{ path: string }>;
  }>;
  const filePaths = packResult.files.map((file) => file.path).sort();

  assert.equal(packResult.name, "g3ts-astro-nuasite-checks");
  assert.equal(packResult.version, "0.1.2");
  assert.deepEqual(
    filePaths.filter((filePath) => !isAllowedPublishedPath(filePath)),
    [],
    "published package contains files outside dist, README, license, and package metadata"
  );
  assert.equal(
    filePaths.some((filePath) => filePath.startsWith("src/")),
    false,
    "published package must not include source files"
  );
  assert.equal(
    filePaths.some((filePath) => filePath.startsWith("tests/")),
    false,
    "published package must not include tests"
  );
});

async function collectJsFiles(rootDir: string): Promise<string[]> {
  const entries = await fs.readdir(rootDir, { withFileTypes: true });
  const nested = await Promise.all(
    entries.map(async (entry) => {
      const entryPath = path.join(rootDir, entry.name);
      if (entry.isDirectory()) {
        return collectJsFiles(entryPath);
      }

      if (entry.isFile() && entry.name.endsWith(".js")) {
        return [entryPath];
      }

      return [];
    })
  );

  return nested.flat();
}

function collectBareSpecifiers(source: string): string[] {
  const specifiers: string[] = [];

  for (const match of source.matchAll(IMPORT_RE)) {
    const specifier = match[1] ?? match[2];
    if (!specifier || isRelativeSpecifier(specifier)) {
      continue;
    }

    specifiers.push(normalizePackageName(specifier));
  }

  return specifiers;
}

function isRelativeSpecifier(specifier: string): boolean {
  return (
    specifier.startsWith(".") ||
    specifier.startsWith("/") ||
    specifier.startsWith("file:")
  );
}

function normalizePackageName(specifier: string): string {
  if (specifier.startsWith("@")) {
    const segments = specifier.split("/");
    return segments.slice(0, 2).join("/");
  }

  return specifier.split("/")[0] ?? specifier;
}

function isAllowedPublishedPath(filePath: string): boolean {
  return (
    filePath === "LICENSE" ||
    filePath === "README.md" ||
    filePath === "package.json" ||
    filePath.startsWith("dist/")
  );
}
