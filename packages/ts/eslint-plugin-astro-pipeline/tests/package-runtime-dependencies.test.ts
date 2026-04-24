import assert from "node:assert/strict";
import fs from "node:fs/promises";
import path from "node:path";
import test from "node:test";

interface PackageJsonShape {
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

const IMPORT_RE =
  /\b(?:import|export)\s+(?:[^"'`]*?\s+from\s+)?["']([^"'`]+)["']|\bimport\(\s*["']([^"'`]+)["']\s*\)/g;

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
