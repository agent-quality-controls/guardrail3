import assert from "node:assert/strict";
import { execFile } from "node:child_process";
import fs from "node:fs/promises";
import path from "node:path";
import test from "node:test";
import { promisify } from "node:util";

import plugin from "../src/index.js";

interface PackageJsonShape {
  name?: string;
  version?: string;
  dependencies?: Record<string, string>;
  peerDependencies?: Record<string, string>;
}

interface LockfileShape {
  packages?: Record<
    string,
    {
      peerDependencies?: Record<string, string>;
      peerDependenciesMeta?: Record<string, { optional?: boolean }>;
    }
  >;
}

const packageRoot = path.resolve(import.meta.dirname, "..");
const distRoot = path.join(packageRoot, "dist");
const execFileAsync = promisify(execFile);
const forbiddenDelegatedPackages = [
  "eslint-mdx",
  "eslint-plugin-mdx",
  "eslint-plugin-i18next"
];

const IMPORT_RE =
  /\b(?:import|export)\s+(?:[^"'`]*?\s+from\s+)?["']([^"'`]+)["']|\bimport\(\s*["']([^"'`]+)["']\s*\)/g;
const OLD_PACKAGE_RE = /(?<!g3ts-)eslint-plugin-astro-pipeline/;

test("published runtime imports are declared in package dependencies", async () => {
  const packageJson = JSON.parse(
    await fs.readFile(path.join(packageRoot, "package.json"), "utf8")
  ) as PackageJsonShape;
  const packageLock = JSON.parse(
    await fs.readFile(path.join(packageRoot, "package-lock.json"), "utf8")
  ) as LockfileShape;

  const declared = new Set([
    ...Object.keys(packageJson.dependencies ?? {}),
    ...Object.keys(packageJson.peerDependencies ?? {})
  ]);

  const distFiles = await collectJsFiles(distRoot);
  const missing = new Set<string>();

  const runtimePackages = new Set<string>();

  for (const filePath of distFiles) {
    const source = await fs.readFile(filePath, "utf8");

    for (const specifier of collectBareSpecifiers(source)) {
      runtimePackages.add(specifier);
      if (!declared.has(specifier) && !isNodeBuiltin(specifier)) {
        missing.add(specifier);
      }
    }
  }

  for (const packageName of runtimePackages) {
    for (const peerName of requiredRuntimePeers(packageLock, packageName)) {
      if (!declared.has(peerName)) {
        missing.add(peerName);
      }
    }
  }

  assert.deepEqual(
    [...missing],
    [],
    `dist runtime imports are missing from dependencies or peerDependencies: ${[
      ...missing
    ].join(", ")}`
  );
});

test("package contract does not hide delegated ESLint validators", async () => {
  const packageJson = JSON.parse(
    await fs.readFile(path.join(packageRoot, "package.json"), "utf8")
  ) as PackageJsonShape;
  const runtimePackages = await collectRuntimePackages();

  assert.equal(packageJson.name, "g3ts-eslint-plugin-astro-pipeline");
  assert.equal(packageJson.version, "0.1.6");
  assert.deepEqual(Object.keys(plugin.configs), ["recommended"]);
  assert.equal(
    "strict-content" in plugin.configs,
    false,
    "g3ts-eslint-plugin-astro-pipeline must not expose the removed wrapper config"
  );

  for (const packageName of forbiddenDelegatedPackages) {
    assert.equal(
      packageJson.dependencies?.[packageName],
      undefined,
      `${packageName} must not be a runtime dependency`
    );
    assert.equal(
      packageJson.peerDependencies?.[packageName],
      undefined,
      `${packageName} must not be a peer dependency`
    );
    assert.equal(
      runtimePackages.has(packageName),
      false,
      `${packageName} must not be imported by published runtime files`
    );
  }
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

  assert.equal(packResult.name, "g3ts-eslint-plugin-astro-pipeline");
  assert.equal(packResult.version, "0.1.6");
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
  for (const filePath of filePaths) {
    const content = await fs.readFile(path.join(packageRoot, filePath), "utf8");
    assert.equal(
      OLD_PACKAGE_RE.test(content),
      false,
      `${filePath} must not reference the deprecated eslint-plugin-astro-pipeline package`
    );
    assert.equal(
      content.includes("strict-content"),
      false,
      `${filePath} must not reference the removed strict-content wrapper config`
    );
  }
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

async function collectRuntimePackages(): Promise<Set<string>> {
  const runtimePackages = new Set<string>();
  const distFiles = await collectJsFiles(distRoot);

  for (const filePath of distFiles) {
    const source = await fs.readFile(filePath, "utf8");
    for (const specifier of collectBareSpecifiers(source)) {
      runtimePackages.add(specifier);
    }
  }

  return runtimePackages;
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

function isNodeBuiltin(specifier: string): boolean {
  return specifier.startsWith("node:");
}

function requiredRuntimePeers(
  packageLock: LockfileShape,
  packageName: string
): string[] {
  const packageEntry = packageLock.packages?.[`node_modules/${packageName}`];
  if (!packageEntry?.peerDependencies) {
    return [];
  }

  return Object.keys(packageEntry.peerDependencies).filter((peerName) => {
    const meta = packageEntry.peerDependenciesMeta?.[peerName];
    return meta?.optional !== true;
  });
}

function isAllowedPublishedPath(filePath: string): boolean {
  return (
    filePath === "LICENSE" ||
    filePath === "README.md" ||
    filePath === "package.json" ||
    filePath.startsWith("dist/")
  );
}
