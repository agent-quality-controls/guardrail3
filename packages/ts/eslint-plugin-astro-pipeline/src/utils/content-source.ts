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
