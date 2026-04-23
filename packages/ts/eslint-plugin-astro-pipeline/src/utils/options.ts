export interface AstroPipelineOptions {
  routeGlobs?: string[];
  endpointGlobs?: string[];
  adapterModuleGlobs?: string[];
  mdxRuntimeModuleGlobs?: string[];
  routeRegistryModuleGlobs?: string[];
  approvedContentAdapterModules?: string[];
  approvedLoaderModules?: string[];
  approvedMdxComponentModules?: string[];
  approvedGeneratedArtifactRoots?: string[];
  authoredContentGlobs?: string[];
  specContentGlobs?: string[];
}

export interface ResolvedAstroPipelineOptions {
  routeGlobs: string[];
  endpointGlobs: string[];
  adapterModuleGlobs: string[];
  mdxRuntimeModuleGlobs: string[];
  routeRegistryModuleGlobs: string[];
  approvedContentAdapterModules: string[];
  approvedLoaderModules: string[];
  approvedMdxComponentModules: string[];
  approvedGeneratedArtifactRoots: string[];
  authoredContentGlobs: string[];
  specContentGlobs: string[];
}

const ARRAY_OPTION_KEYS = [
  "routeGlobs",
  "endpointGlobs",
  "adapterModuleGlobs",
  "mdxRuntimeModuleGlobs",
  "routeRegistryModuleGlobs",
  "approvedContentAdapterModules",
  "approvedLoaderModules",
  "approvedMdxComponentModules",
  "approvedGeneratedArtifactRoots",
  "authoredContentGlobs",
  "specContentGlobs"
] as const satisfies readonly (keyof ResolvedAstroPipelineOptions)[];

export type AstroPipelineOptionKey = (typeof ARRAY_OPTION_KEYS)[number];

export type RuleOptionsTuple = [AstroPipelineOptions?];

const stringArraySchema: JSONSchema4 = {
  type: "array",
  items: { type: "string" }
};

export const astroPipelineOptionsSchema: JSONSchema4[] = [
  {
    type: "object",
    additionalProperties: false,
    properties: Object.fromEntries(
      ARRAY_OPTION_KEYS.map((key) => [key, stringArraySchema])
    )
  }
];

export function resolveOptions(
  rawOptions: AstroPipelineOptions | null | undefined
): ResolvedAstroPipelineOptions {
  const source = rawOptions ?? {};

  return {
    routeGlobs: normalizeStringArray(source.routeGlobs),
    endpointGlobs: normalizeStringArray(source.endpointGlobs),
    adapterModuleGlobs: normalizeStringArray(source.adapterModuleGlobs),
    mdxRuntimeModuleGlobs: normalizeStringArray(source.mdxRuntimeModuleGlobs),
    routeRegistryModuleGlobs: normalizeStringArray(source.routeRegistryModuleGlobs),
    approvedContentAdapterModules: normalizeStringArray(
      source.approvedContentAdapterModules
    ),
    approvedLoaderModules: normalizeStringArray(source.approvedLoaderModules),
    approvedMdxComponentModules: normalizeStringArray(
      source.approvedMdxComponentModules
    ),
    approvedGeneratedArtifactRoots: normalizeStringArray(
      source.approvedGeneratedArtifactRoots
    ),
    authoredContentGlobs: normalizeStringArray(source.authoredContentGlobs),
    specContentGlobs: normalizeStringArray(source.specContentGlobs)
  };
}

function normalizeStringArray(values: string[] | undefined): string[] {
  return [...new Set((values ?? []).map((value) => value.trim()).filter(Boolean))];
}
import type { JSONSchema4 } from "@typescript-eslint/utils/json-schema";
