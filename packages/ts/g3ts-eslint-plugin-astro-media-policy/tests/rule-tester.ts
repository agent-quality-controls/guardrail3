import { RuleTester } from "eslint";
import * as astroParser from "astro-eslint-parser";
import tsParser from "@typescript-eslint/parser";

import type { AstroMediaPolicyOptions } from "../src/utils/options.js";

export const baseOptions: AstroMediaPolicyOptions = {
  publicSourceGlobs: ["src/**/*.{astro,ts,tsx}", "content/**/*.mdx"],
  mediaHelperModules: ["src/media/images.ts"],
  approvedMediaHelpers: ["imageMetadata"],
  contentImageComponents: ["ArticleImage", "ContentImage"],
  contentImageKeyProps: ["image"],
  bannedImageSourceProps: ["src", "url"],
  bannedImageAltProps: ["alt"],
  allowedPublicImagePaths: ["/favicon.svg"],
  checkedImageExtensions: [".jpg", ".jpeg", ".png", ".webp", ".avif", ".gif", ".svg", ".ico"],
  metadataImagePropertyNames: ["image", "ogImage"]
};

export function createRuleTester(): RuleTester {
  return new RuleTester({
    languageOptions: {
      ecmaVersion: "latest",
      parser: tsParser,
      parserOptions: {
        ecmaFeatures: {
          jsx: true
        }
      },
      sourceType: "module"
    }
  });
}

export const astroLanguageOptions = {
  ecmaVersion: "latest" as const,
  parser: astroParser,
  parserOptions: {
    ecmaVersion: "latest" as const,
    extraFileExtensions: [".astro"],
    parser: tsParser,
    sourceType: "module" as const
  }
};
