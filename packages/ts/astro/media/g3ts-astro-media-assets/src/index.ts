import { access } from "node:fs/promises";
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
  const normalized = normalizeOptions(options);
  const assetPaths = [
    normalized.favicon,
    ...normalized.appIcons,
    normalized.defaultSocialImage
  ];

  for (const assetPath of assetPaths) {
    if (!normalized.allowSvgIcons && assetPath.toLowerCase().endsWith(".svg")) {
      throw new Error(
        `g3ts-astro-media-assets: SVG media asset is not allowed by allowSvgIcons=false: ${assetPath}`
      );
    }

    await assertAssetExists(normalized.outputDir, assetPath);
  }
}

function normalizeOptions(
  options: G3TsAstroMediaAssetsCheckOptions
): Required<G3TsAstroMediaAssetsCheckOptions> {
  return {
    outputDir: normalizeOutputDir(options.outputDir),
    favicon: normalizePublicPath(options.favicon, "favicon"),
    appIcons: normalizePublicPathArray(options.appIcons, "appIcons"),
    defaultSocialImage: normalizePublicPath(
      options.defaultSocialImage,
      "defaultSocialImage"
    ),
    allowSvgIcons: options.allowSvgIcons
  };
}

function normalizeOutputDir(outputDir: string): string {
  const normalized = outputDir.trim();
  if (!normalized) {
    throw new Error("g3ts-astro-media-assets: outputDir must be non-empty.");
  }

  return normalized;
}

function normalizePublicPath(value: string, field: string): string {
  const normalized = value.trim();
  if (!normalized) {
    throw new Error(`g3ts-astro-media-assets: ${field} must be non-empty.`);
  }
  if (/^[a-z][a-z0-9+.-]*:/i.test(normalized) || normalized.startsWith("//")) {
    throw new Error(
      `g3ts-astro-media-assets: ${field} must be a root-relative public path, not ${normalized}.`
    );
  }
  if (!normalized.startsWith("/")) {
    throw new Error(
      `g3ts-astro-media-assets: ${field} must start with "/": ${normalized}.`
    );
  }
  if (normalized.split("/").includes("..")) {
    throw new Error(
      `g3ts-astro-media-assets: ${field} must not traverse with "..": ${normalized}.`
    );
  }

  return normalized.replace(/\/+/g, "/");
}

function normalizePublicPathArray(values: string[], field: string): string[] {
  if (!Array.isArray(values) || values.length === 0) {
    throw new Error(`g3ts-astro-media-assets: ${field} must be non-empty.`);
  }

  return values.map((value, index) => normalizePublicPath(value, `${field}[${index}]`));
}

async function assertAssetExists(outputDir: string, assetPath: string): Promise<void> {
  const relativePath = assetPath.replace(/^\/+/, "");
  const fullPath = path.join(outputDir, relativePath);

  await access(fullPath).catch(() => {
    throw new Error(
      `g3ts-astro-media-assets: required media asset ${assetPath} was not found in built output ${outputDir}.`
    );
  });
}
