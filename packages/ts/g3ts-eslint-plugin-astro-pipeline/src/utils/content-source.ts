import type { ResolvedAstroPipelineOptions } from "./options.js";
import {
  inferPathPolicyRoot,
  matchesPathPolicy,
  resolvePathLike
} from "./path-policy.js";

const URL_OR_SCHEME_RE = /^[a-z]+:/i;

export function resolvesToAuthoredOrSpecContent(
  rawPath: string,
  importerFilename: string,
  options: ResolvedAstroPipelineOptions,
  cwd = inferPathPolicyRoot(importerFilename)
): boolean {
  if (!looksLikePathLiteral(rawPath)) {
    return false;
  }

  return resolveCandidatePaths(rawPath, importerFilename, cwd).some(
    (candidate) =>
      matchesPathPolicy(candidate, options.authoredContentGlobs, cwd) ||
      matchesPathPolicy(candidate, options.specContentGlobs, cwd)
  );
}

export function couldResolveToAuthoredOrSpecContent(
  rawPath: string,
  importerFilename: string,
  options: ResolvedAstroPipelineOptions,
  cwd = inferPathPolicyRoot(importerFilename)
): boolean {
  if (!looksLikePathLiteral(rawPath)) {
    return false;
  }

  return resolveCandidatePaths(rawPath, importerFilename, cwd).some(
    (candidate) =>
      pathCouldMatchPolicy(candidate, options.authoredContentGlobs, cwd) ||
      pathCouldMatchPolicy(candidate, options.specContentGlobs, cwd)
  );
}

export function resolvesToApprovedGeneratedArtifact(
  rawPath: string,
  importerFilename: string,
  options: ResolvedAstroPipelineOptions,
  cwd = inferPathPolicyRoot(importerFilename)
): boolean {
  if (!looksLikePathLiteral(rawPath)) {
    return false;
  }

  return resolveCandidatePaths(rawPath, importerFilename, cwd).some((candidate) =>
    matchesPathPolicy(candidate, options.approvedGeneratedArtifactRoots, cwd)
  );
}

function pathCouldMatchPolicy(
  pathValue: string,
  globs: readonly string[],
  cwd: string
): boolean {
  const normalizedValue = resolvePathLike(cwd, pathValue, cwd);
  const prefix = staticPrefix(normalizedValue);

  if (prefix.length === 0) {
    return false;
  }

  return globs.some((glob) => {
    const normalizedGlob = resolvePathLike(cwd, glob, cwd);
    const globPrefix = staticPrefix(normalizedGlob);

    return (
      globPrefix.length > 0 &&
      (prefix.startsWith(globPrefix) || globPrefix.startsWith(prefix))
    );
  });
}

function staticPrefix(value: string): string {
  const globIndex = value.search(/[*?[\]{}]/);
  const prefix = globIndex === -1 ? value : value.slice(0, globIndex);

  return prefix.replace(/\/+$/, "");
}

export function resolvesToApprovedContentAdapter(
  rawPath: string,
  importerFilename: string,
  options: ResolvedAstroPipelineOptions,
  cwd = inferPathPolicyRoot(importerFilename)
): boolean {
  if (!looksLikePathLiteral(rawPath)) {
    return false;
  }

  return resolveCandidatePaths(rawPath, importerFilename, cwd).some((candidate) =>
    matchesPathPolicy(candidate, options.approvedContentAdapterModules, cwd)
  );
}

function looksLikePathLiteral(value: string): boolean {
  if (value.length === 0) {
    return false;
  }

  if (URL_OR_SCHEME_RE.test(value) && !value.startsWith("./") && !value.startsWith("../")) {
    return false;
  }

  return (
    value.startsWith(".") ||
    value.startsWith("/") ||
    value.startsWith("@") ||
    value.includes("/") ||
    value.includes("*")
  );
}

function resolveCandidatePaths(
  rawPath: string,
  importerFilename: string,
  cwd: string
): string[] {
  const candidates = new Set<string>([resolvePathLike(importerFilename, rawPath, cwd)]);

  if (rawPath.startsWith("./src/") || rawPath.startsWith("./specs/")) {
    candidates.add(rawPath.slice(2));
  }

  return [...candidates];
}
