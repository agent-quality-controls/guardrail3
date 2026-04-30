import { RuleTester } from "eslint";
import * as astroParser from "astro-eslint-parser";
import tsParser from "@typescript-eslint/parser";

import type { AstroI18nPolicyOptions } from "../src/utils/options.js";

export const baseOptions: AstroI18nPolicyOptions = {
  locales: ["en", "fr"],
  defaultLocale: "en",
  requireLocalePrefixForContentRoutes: true,
  allowedUnprefixedRoutes: ["/", "/robots.txt", "/llms.txt", "/sitemap-index.xml"],
  contentRoutePrefixes: ["/blog", "/guides"],
  checkedInternalLinkHelpers: ["buildPath"],
  approvedInternalLinkHelpers: ["localizedHref", "buildLocalizedPath"],
  approvedLocalizedLinkComponents: ["LocalizedLink"]
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
