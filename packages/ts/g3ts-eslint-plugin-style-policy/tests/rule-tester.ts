import { RuleTester } from "eslint";
import * as astroParser from "astro-eslint-parser";
import tsParser from "@typescript-eslint/parser";

import type { StylePolicyOptions } from "../src/utils/options.js";

export const baseOptions: StylePolicyOptions = {
  denyList: ["text-black", "text-9xl", "bg-red-500"],
  classAttributes: ["class", "className"],
  classListAttributes: ["class:list"],
  classHelpers: ["cn", "clsx", "twMerge"]
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
