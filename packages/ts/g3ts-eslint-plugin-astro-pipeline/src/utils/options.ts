export interface AstroPipelineOptions {
  routeGlobs?: string[];
  endpointGlobs?: string[];
  mdxContentGlobs?: string[];
  contentDataModuleGlobs?: string[];
  adapterModuleGlobs?: string[];
  mdxRuntimeModuleGlobs?: string[];
  routeRegistryModuleGlobs?: string[];
  approvedContentAdapterModules?: string[];
  approvedMetadataHelperModules?: string[];
  approvedJsonLdHelperModules?: string[];
  approvedLoaderModules?: string[];
  approvedMdxComponentModules?: string[];
  approvedMdxComponentNames?: string[];
  allowedMdxComponentMapExports?: string[];
  approvedMdxImageComponents?: string[];
  mdxPropsParserName?: string[];
  rawUiModuleGlobs?: string[];
  approvedGeneratedArtifactRoots?: string[];
  authoredContentGlobs?: string[];
  specContentGlobs?: string[];
}

export interface ResolvedAstroPipelineOptions {
  routeGlobs: string[];
  endpointGlobs: string[];
  mdxContentGlobs: string[];
  contentDataModuleGlobs: string[];
  adapterModuleGlobs: string[];
  mdxRuntimeModuleGlobs: string[];
  routeRegistryModuleGlobs: string[];
  approvedContentAdapterModules: string[];
  approvedMetadataHelperModules: string[];
  approvedJsonLdHelperModules: string[];
  approvedLoaderModules: string[];
  approvedMdxComponentModules: string[];
  approvedMdxComponentNames: string[];
  allowedMdxComponentMapExports: string[];
  approvedMdxImageComponents: string[];
  mdxPropsParserName: string[];
  rawUiModuleGlobs: string[];
  approvedGeneratedArtifactRoots: string[];
  authoredContentGlobs: string[];
  specContentGlobs: string[];
}

const ARRAY_OPTION_KEYS = [
  "routeGlobs",
  "endpointGlobs",
  "mdxContentGlobs",
  "contentDataModuleGlobs",
  "adapterModuleGlobs",
  "mdxRuntimeModuleGlobs",
  "routeRegistryModuleGlobs",
  "approvedContentAdapterModules",
  "approvedMetadataHelperModules",
  "approvedJsonLdHelperModules",
  "approvedLoaderModules",
  "approvedMdxComponentModules",
  "approvedMdxComponentNames",
  "allowedMdxComponentMapExports",
  "approvedMdxImageComponents",
  "mdxPropsParserName",
  "rawUiModuleGlobs",
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
    mdxContentGlobs: normalizeStringArray(source.mdxContentGlobs),
    contentDataModuleGlobs: normalizeStringArray(source.contentDataModuleGlobs),
    adapterModuleGlobs: normalizeStringArray(source.adapterModuleGlobs),
    mdxRuntimeModuleGlobs: normalizeStringArray(source.mdxRuntimeModuleGlobs),
    routeRegistryModuleGlobs: normalizeStringArray(source.routeRegistryModuleGlobs),
    approvedContentAdapterModules: normalizeStringArray(
      source.approvedContentAdapterModules
    ),
    approvedMetadataHelperModules: normalizeStringArray(
      source.approvedMetadataHelperModules
    ),
    approvedJsonLdHelperModules: normalizeStringArray(
      source.approvedJsonLdHelperModules
    ),
    approvedLoaderModules: normalizeStringArray(source.approvedLoaderModules),
    approvedMdxComponentModules: normalizeStringArray(
      source.approvedMdxComponentModules
    ),
    approvedMdxComponentNames: normalizeStringArray(
      source.approvedMdxComponentNames
    ),
    allowedMdxComponentMapExports: normalizeStringArray(
      source.allowedMdxComponentMapExports
    ),
    approvedMdxImageComponents: normalizeStringArray(
      source.approvedMdxImageComponents
    ),
    mdxPropsParserName: normalizeStringArray(source.mdxPropsParserName),
    rawUiModuleGlobs: normalizeStringArray(source.rawUiModuleGlobs),
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
