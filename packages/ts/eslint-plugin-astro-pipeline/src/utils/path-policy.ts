import path from "node:path";

import { minimatch } from "minimatch";

const GLOB_TOKEN_RE = /[*?[\]{}]/;
const ROOT_MARKERS = ["/src/", "/app/", "/pages/", "/content/", "/specs/"] as const;

export function normalizeSlashes(value: string): string {
  return value.replaceAll("\\", "/");
}

export function normalizeGlob(value: string): string {
  return normalizeSlashes(value).replace(/^\.\//, "").replace(/\/+/g, "/");
}

export function normalizePathFromCwd(value: string, cwd = process.cwd()): string {
  const relativePath = path.isAbsolute(value) ? path.relative(cwd, value) : value;

  return normalizeGlob(relativePath);
}

export function resolvePathLike(
  importerFilename: string,
  rawPath: string,
  cwd = inferPathPolicyRoot(importerFilename)
): string {
  if (path.isAbsolute(rawPath)) {
    return normalizePathFromCwd(rawPath, cwd);
  }

  if (rawPath.startsWith(".")) {
    return normalizePathFromCwd(
      path.resolve(path.dirname(importerFilename), rawPath),
      cwd
    );
  }

  return normalizeGlob(rawPath);
}

export function matchesFileGlobs(
  filename: string,
  globs: readonly string[],
  cwd = inferPathPolicyRoot(filename)
): boolean {
  if (globs.length === 0) {
    return false;
  }

  const normalizedFilename = normalizePathFromCwd(filename, cwd);

  return globs.some((pattern) =>
    minimatch(normalizedFilename, normalizeGlob(pattern), { dot: true })
  );
}

export function matchesPathPolicy(
  pathValue: string,
  globs: readonly string[],
  cwd = inferPathPolicyRoot(pathValue)
): boolean {
  if (globs.length === 0) {
    return false;
  }

  const normalizedValue = normalizePathFromCwd(pathValue, cwd);

  return globs.some((pattern) => {
    const normalizedPattern = normalizeGlob(pattern);

    return (
      minimatch(normalizedValue, normalizedPattern, { dot: true }) ||
      patternsOverlap(normalizedValue, normalizedPattern)
    );
  });
}

export function inferPathPolicyRoot(value: string): string {
  if (!path.isAbsolute(value)) {
    return process.cwd();
  }

  const normalizedValue = normalizeSlashes(value);

  for (const marker of ROOT_MARKERS) {
    const markerIndex = normalizedValue.indexOf(marker);

    if (markerIndex > 0) {
      return normalizedValue.slice(0, markerIndex);
    }
  }

  return process.cwd();
}

function patternsOverlap(left: string, right: string): boolean {
  const leftPrefix = staticPrefix(left);
  const rightPrefix = staticPrefix(right);

  return (
    leftPrefix.length > 0 &&
    rightPrefix.length > 0 &&
    (leftPrefix.startsWith(rightPrefix) || rightPrefix.startsWith(leftPrefix))
  );
}

function staticPrefix(value: string): string {
  const normalizedValue = normalizeGlob(value);
  const globIndex = normalizedValue.search(GLOB_TOKEN_RE);
  const prefix = globIndex === -1 ? normalizedValue : normalizedValue.slice(0, globIndex);

  return prefix.replace(/\/+$/, "");
}
