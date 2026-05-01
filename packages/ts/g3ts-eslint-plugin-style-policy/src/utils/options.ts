import type { JSONSchema4 } from "@typescript-eslint/utils/json-schema";

export interface StylePolicyOptions {
  denyList?: string[];
  denyPrefixes?: string[];
  denyPatterns?: string[];
  classAttributes?: string[];
  classListAttributes?: string[];
  classHelpers?: string[];
}

export interface ResolvedStylePolicyOptions {
  denyList: string[];
  denyPrefixes: string[];
  denyPatterns: string[];
  classAttributes: string[];
  classListAttributes: string[];
  classHelpers: string[];
}

export type RuleOptionsTuple = [StylePolicyOptions?];

const stringArraySchema: JSONSchema4 = {
  type: "array",
  items: { type: "string" }
};

export const stylePolicyOptionsSchema: JSONSchema4[] = [
  {
    type: "object",
    additionalProperties: false,
    properties: {
      denyList: stringArraySchema,
      denyPrefixes: stringArraySchema,
      denyPatterns: stringArraySchema,
      classAttributes: stringArraySchema,
      classListAttributes: stringArraySchema,
      classHelpers: stringArraySchema
    }
  }
];

export function resolveOptions(
  rawOptions: StylePolicyOptions | null | undefined
): ResolvedStylePolicyOptions {
  const source = rawOptions ?? {};

  return {
    denyList: normalizeStringArray(source.denyList),
    denyPrefixes: normalizeStringArray(source.denyPrefixes),
    denyPatterns: normalizeStringArray(source.denyPatterns),
    classAttributes: normalizeStringArray(source.classAttributes),
    classListAttributes: normalizeStringArray(source.classListAttributes),
    classHelpers: normalizeStringArray(source.classHelpers)
  };
}

export function missingRequiredOptions(
  options: ResolvedStylePolicyOptions,
  required: readonly (keyof ResolvedStylePolicyOptions)[]
): string[] {
  return required.filter((key) => options[key].length === 0);
}

function normalizeStringArray(values: string[] | undefined): string[] {
  return [...new Set((values ?? []).map((value) => value.trim()).filter(Boolean))];
}
