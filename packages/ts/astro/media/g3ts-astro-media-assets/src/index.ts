import { stat } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import path from "node:path";

import type { AstroIntegration } from "astro";

export interface G3TsAstroMediaAssetsOptions {
  favicon: string;
  appIcons: string[];
  defaultSocialImage: string;
  allowSvgIcons: boolean;
}

export interface G3TsAstroMediaAssetsCheckOptions
  extends G3TsAstroMediaAssetsOptions {
  outputDir: string;
}

export default function g3tsAstroMediaAssets(
  options: G3TsAstroMediaAssetsOptions
): AstroIntegration {
  return {
    name: "g3ts-astro-media-assets",
    hooks: {
      "astro:build:done": async ({ dir }) => {
        await checkMediaAssets({
          ...options,
          outputDir: fileURLToPath(dir)
        });
      }
    }
  };
}

export async function checkMediaAssets(
  options: G3TsAstroMediaAssetsCheckOptions
): Promise<void> {
  const errors: string[] = [];
  const normalized = normalizeOptions(options, errors);
  const assetPaths = [
    normalized.favicon,
    ...normalized.appIcons,
    normalized.defaultSocialImage
  ];

  for (const assetPath of assetPaths) {
    if (!normalized.allowSvgIcons && assetPath.toLowerCase().endsWith(".svg")) {
      errors.push(
        `g3ts-astro-media-assets: SVG media asset is not allowed by allowSvgIcons=false: ${assetPath}`
      );
      continue;
    }

    await assertAssetExists(normalized.outputDir, assetPath, errors);
  }

  if (errors.length > 0) {
    throw new Error(errors.join("\n"));
  }
}

function normalizeOptions(
  options: G3TsAstroMediaAssetsCheckOptions,
  errors: string[]
): Required<G3TsAstroMediaAssetsCheckOptions> {
  const rawOptions = options as Partial<G3TsAstroMediaAssetsCheckOptions>;

  return {
    outputDir: normalizeOutputDir(rawOptions.outputDir, errors),
    favicon: normalizePublicPath(rawOptions.favicon, "favicon", errors),
    appIcons: normalizePublicPathArray(rawOptions.appIcons, "appIcons", errors),
    defaultSocialImage: normalizePublicPath(
      rawOptions.defaultSocialImage,
      "defaultSocialImage",
      errors
    ),
    allowSvgIcons: normalizeBoolean(rawOptions.allowSvgIcons, "allowSvgIcons", errors)
  };
}

function normalizeOutputDir(outputDir: unknown, errors: string[]): string {
  if (typeof outputDir !== "string") {
    errors.push("g3ts-astro-media-assets: outputDir must be a string.");

    return "";
  }

  const normalized = outputDir.trim();
  if (!normalized) {
    errors.push("g3ts-astro-media-assets: outputDir must be non-empty.");
  }

  return normalized;
}

function normalizePublicPath(
  value: unknown,
  field: string,
  errors: string[]
): string {
  if (typeof value !== "string") {
    errors.push(`g3ts-astro-media-assets: ${field} must be a string.`);

    return "";
  }

  const normalized = value.trim();
  if (!normalized) {
    errors.push(`g3ts-astro-media-assets: ${field} must be non-empty.`);

    return "";
  }
  if (/^[a-z][a-z0-9+.-]*:/i.test(normalized) || normalized.startsWith("//")) {
    errors.push(
      `g3ts-astro-media-assets: ${field} must be a root-relative public path, not ${normalized}.`
    );

    return "";
  }
  if (!normalized.startsWith("/")) {
    errors.push(
      `g3ts-astro-media-assets: ${field} must start with "/": ${normalized}.`
    );

    return "";
  }
  if (containsTraversal(normalized)) {
    errors.push(
      `g3ts-astro-media-assets: ${field} must not traverse with "..": ${normalized}.`
    );

    return "";
  }

  return normalized.replace(/\/+/g, "/");
}

function normalizePublicPathArray(
  values: unknown,
  field: string,
  errors: string[]
): string[] {
  if (!Array.isArray(values) || values.length === 0) {
    errors.push(`g3ts-astro-media-assets: ${field} must be a non-empty array.`);

    return [];
  }

  return values.map((value, index) =>
    normalizePublicPath(value, `${field}[${index}]`, errors)
  );
}

function normalizeBoolean(value: unknown, field: string, errors: string[]): boolean {
  if (typeof value !== "boolean") {
    errors.push(`g3ts-astro-media-assets: ${field} must be a boolean.`);
  }

  return value === true;
}

function containsTraversal(value: string): boolean {
  const lower = value.toLowerCase();
  if (value.includes("\\") || lower.includes("%2f") || lower.includes("%5c")) {
    return true;
  }

  try {
    return decodeURIComponent(value).split("/").includes("..");
  } catch {
    return true;
  }
}

async function assertAssetExists(
  outputDir: string,
  assetPath: string,
  errors: string[]
): Promise<void> {
  if (!outputDir || !assetPath) {
    return;
  }

  const relativePath = assetPath.replace(/^\/+/, "");
  const fullPath = path.join(outputDir, relativePath);

  const fileStat = await stat(fullPath).catch(() => null);
  if (!fileStat) {
    errors.push(
      `g3ts-astro-media-assets: required media asset ${assetPath} was not found in built output ${outputDir}.`
    );
    return;
  }

  if (!fileStat.isFile()) {
    errors.push(
      `g3ts-astro-media-assets: required media asset ${assetPath} exists in built output ${outputDir}, but it is not a file.`
    );
  }
}
