import type { JSONSchema4 } from "@typescript-eslint/utils/json-schema";

export interface AstroMediaPolicyOptions {
  mediaHelperModules?: string[];
  approvedMediaHelpers?: string[];
  contentImageComponents?: string[];
  contentImageKeyProps?: string[];
  bannedImageSourceProps?: string[];
  bannedImageAltProps?: string[];
  allowedPublicImagePaths?: string[];
  checkedImageExtensions?: string[];
  metadataImagePropertyNames?: string[];
}

export interface ResolvedAstroMediaPolicyOptions {
  mediaHelperModules: string[];
  approvedMediaHelpers: string[];
  contentImageComponents: string[];
  contentImageKeyProps: string[];
  bannedImageSourceProps: string[];
  bannedImageAltProps: string[];
  allowedPublicImagePaths: string[];
  checkedImageExtensions: string[];
  metadataImagePropertyNames: string[];
}

export type RuleOptionsTuple = [AstroMediaPolicyOptions?];

const stringArraySchema: JSONSchema4 = {
  type: "array",
  items: { type: "string" }
};

export const astroMediaPolicyOptionsSchema: JSONSchema4[] = [
  {
    type: "object",
    additionalProperties: false,
    properties: {
      mediaHelperModules: stringArraySchema,
      approvedMediaHelpers: stringArraySchema,
      contentImageComponents: stringArraySchema,
      contentImageKeyProps: stringArraySchema,
      bannedImageSourceProps: stringArraySchema,
      bannedImageAltProps: stringArraySchema,
      allowedPublicImagePaths: stringArraySchema,
      checkedImageExtensions: stringArraySchema,
      metadataImagePropertyNames: stringArraySchema
    }
  }
];

export function resolveOptions(
  rawOptions: AstroMediaPolicyOptions | null | undefined
): ResolvedAstroMediaPolicyOptions {
  const source = rawOptions ?? {};

  return {
    mediaHelperModules: normalizeStringArray(source.mediaHelperModules),
    approvedMediaHelpers: normalizeStringArray(source.approvedMediaHelpers),
    contentImageComponents: normalizeStringArray(source.contentImageComponents),
    contentImageKeyProps: normalizeStringArray(source.contentImageKeyProps),
    bannedImageSourceProps: normalizeStringArray(source.bannedImageSourceProps),
    bannedImageAltProps: normalizeStringArray(source.bannedImageAltProps),
    allowedPublicImagePaths: normalizeStringArray(source.allowedPublicImagePaths),
    checkedImageExtensions: normalizeStringArray(source.checkedImageExtensions).map(
      normalizeExtension
    ),
    metadataImagePropertyNames: normalizeStringArray(
      source.metadataImagePropertyNames
    )
  };
}

export function missingRequiredOptions(
  options: ResolvedAstroMediaPolicyOptions,
  required: readonly (keyof ResolvedAstroMediaPolicyOptions)[]
): string[] {
  return required.filter((key) => {
    const value = options[key];

    return Array.isArray(value) ? value.length === 0 : value === null;
  });
}

function normalizeStringArray(values: string[] | undefined): string[] {
  return [...new Set((values ?? []).map((value) => value.trim()).filter(Boolean))];
}

export function normalizePublicPath(value: string): string {
  const trimmed = value.trim();
  const prefixed = trimmed.startsWith("/") ? trimmed : `/${trimmed}`;

  return prefixed.replace(/\/+/g, "/").replace(/\/$/, "") || "/";
}

function normalizeExtension(value: string): string {
  const trimmed = value.trim().toLowerCase();

  return trimmed.startsWith(".") ? trimmed : `.${trimmed}`;
}
