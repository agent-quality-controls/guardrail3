import type { JSONSchema4 } from "@typescript-eslint/utils/json-schema";

export interface AstroI18nPolicyOptions {
  locales?: string[];
  defaultLocale?: string;
  requireLocalePrefixForContentRoutes?: boolean;
  allowedUnprefixedRoutes?: string[];
  contentRoutePrefixes?: string[];
  checkedInternalLinkHelpers?: string[];
  approvedInternalLinkHelpers?: string[];
  approvedLocalizedLinkComponents?: string[];
}

export interface ResolvedAstroI18nPolicyOptions {
  locales: string[];
  defaultLocale: string | null;
  requireLocalePrefixForContentRoutes: boolean;
  allowedUnprefixedRoutes: string[];
  contentRoutePrefixes: string[];
  checkedInternalLinkHelpers: string[];
  approvedInternalLinkHelpers: string[];
  approvedLocalizedLinkComponents: string[];
}

export type RuleOptionsTuple = [AstroI18nPolicyOptions?];

const stringArraySchema: JSONSchema4 = {
  type: "array",
  items: { type: "string" }
};

export const astroI18nPolicyOptionsSchema: JSONSchema4[] = [
  {
    type: "object",
    additionalProperties: false,
    properties: {
      locales: stringArraySchema,
      defaultLocale: { type: "string" },
      requireLocalePrefixForContentRoutes: { type: "boolean" },
      allowedUnprefixedRoutes: stringArraySchema,
      contentRoutePrefixes: stringArraySchema,
      checkedInternalLinkHelpers: stringArraySchema,
      approvedInternalLinkHelpers: stringArraySchema,
      approvedLocalizedLinkComponents: stringArraySchema
    }
  }
];

export function resolveOptions(
  rawOptions: AstroI18nPolicyOptions | null | undefined
): ResolvedAstroI18nPolicyOptions {
  const source = rawOptions ?? {};

  return {
    locales: normalizeStringArray(source.locales),
    defaultLocale: normalizeOptionalString(source.defaultLocale),
    requireLocalePrefixForContentRoutes:
      source.requireLocalePrefixForContentRoutes === true,
    allowedUnprefixedRoutes: normalizeRouteArray(source.allowedUnprefixedRoutes),
    contentRoutePrefixes: normalizeRouteArray(source.contentRoutePrefixes),
    checkedInternalLinkHelpers: normalizeStringArray(
      source.checkedInternalLinkHelpers
    ),
    approvedInternalLinkHelpers: normalizeStringArray(
      source.approvedInternalLinkHelpers
    ),
    approvedLocalizedLinkComponents: normalizeStringArray(
      source.approvedLocalizedLinkComponents
    )
  };
}

export function missingRequiredOptions(
  options: ResolvedAstroI18nPolicyOptions,
  required: readonly (keyof ResolvedAstroI18nPolicyOptions)[]
): string[] {
  return required.filter((key) => {
    const value = options[key];

    return Array.isArray(value) ? value.length === 0 : value === null;
  });
}

function normalizeStringArray(values: string[] | undefined): string[] {
  return [...new Set((values ?? []).map((value) => value.trim()).filter(Boolean))];
}

function normalizeRouteArray(values: string[] | undefined): string[] {
  return normalizeStringArray(values).map(normalizeRoutePath);
}

function normalizeOptionalString(value: string | undefined): string | null {
  const normalized = value?.trim();

  return normalized ? normalized : null;
}

export function normalizeRoutePath(value: string): string {
  const trimmed = value.trim();
  const prefixed = trimmed.startsWith("/") ? trimmed : `/${trimmed}`;

  return prefixed.replace(/\/+/g, "/").replace(/\/$/, "") || "/";
}
