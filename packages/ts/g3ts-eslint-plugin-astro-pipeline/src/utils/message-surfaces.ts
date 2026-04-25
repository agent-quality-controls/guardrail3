import type { ResolvedAstroPipelineOptions } from "./options.js";

export function describeApprovedContentAdapterSurface(
  options: ResolvedAstroPipelineOptions
): string {
  return describeSurface(
    options.approvedContentAdapterModules,
    "approved content adapter module"
  );
}

export function describeApprovedLoaderOrAdapterSurface(
  options: ResolvedAstroPipelineOptions
): string {
  if (options.approvedLoaderModules.length > 0) {
    return describeSurface(options.approvedLoaderModules, "approved loader module");
  }

  return describeApprovedContentAdapterSurface(options);
}

function describeSurface(patterns: string[], label: string): string {
  if (patterns.length === 0) {
    return `the configured ${label} surface`;
  }

  if (patterns.length === 1) {
    return `${label} matching \`${patterns[0]}\``;
  }

  const listedPatterns = patterns.map((pattern) => `\`${pattern}\``).join(", ");
  return `${label} matching one of ${listedPatterns}`;
}
