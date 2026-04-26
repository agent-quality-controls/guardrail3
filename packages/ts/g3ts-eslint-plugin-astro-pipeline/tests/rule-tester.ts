import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import { RuleTester } from "eslint";
import * as astroParser from "astro-eslint-parser";
import tsParser from "@typescript-eslint/parser";

import type { AstroPipelineOptions } from "../src/utils/options.js";

export const baseOptions: AstroPipelineOptions = {
  routeGlobs: ["src/pages/**/*.{ts,tsx,js,jsx,astro}"],
  endpointGlobs: ["src/pages/**/*.endpoint.{ts,tsx,js,jsx}"],
  mdxContentGlobs: ["src/content/**/*.mdx"],
  contentDataModuleGlobs: [
    "src/**/*.data.{ts,tsx,js,jsx,mts,cts,mjs,cjs}",
    "app/**/*.data.{ts,tsx,js,jsx,mts,cts,mjs,cjs}"
  ],
  adapterModuleGlobs: ["src/lib/content/**/*.{ts,tsx,js,jsx}"],
  mdxRuntimeModuleGlobs: ["src/lib/mdx/**/*.{ts,tsx,js,jsx}"],
  routeRegistryModuleGlobs: ["src/lib/routes/**/*.{ts,tsx,js,jsx}"],
  approvedContentAdapterModules: ["src/lib/content/**/*.{ts,tsx,js,jsx}"],
  approvedMetadataHelperModules: ["src/lib/metadata/**/*.{ts,tsx,js,jsx}"],
  approvedJsonLdHelperModules: ["src/lib/json-ld/**/*.{ts,tsx,js,jsx}"],
  approvedLoaderModules: ["src/lib/content/**/*.{ts,tsx,js,jsx}"],
  approvedMdxComponentModules: ["src/components/mdx/**/*.{ts,tsx,js,jsx}"],
  approvedGeneratedArtifactRoots: ["src/generated/**"],
  authoredContentGlobs: ["src/content/**"],
  specContentGlobs: ["specs/**"]
};

export function createRuleTester(): RuleTester {
  return new RuleTester({
    languageOptions: {
      ecmaVersion: "latest",
      parser: tsParser,
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

export interface FixtureProject {
  rootDir: string;
  read: (relativePath: string) => Promise<string>;
  path: (relativePath: string) => string;
  cleanup: () => Promise<void>;
}

export async function createFixtureProject(
  files: Record<string, string>
): Promise<FixtureProject> {
  const rootDir = await fs.mkdtemp(
    path.join(os.tmpdir(), "g3ts-eslint-plugin-astro-pipeline-")
  );

  await Promise.all(
    Object.entries(files).map(async ([relativePath, contents]) => {
      const targetPath = path.join(rootDir, relativePath);
      await fs.mkdir(path.dirname(targetPath), { recursive: true });
      await fs.writeFile(targetPath, contents, "utf8");
    })
  );

  return {
    rootDir,
    read(relativePath) {
      return fs.readFile(path.join(rootDir, relativePath), "utf8");
    },
    path(relativePath) {
      return path.join(rootDir, relativePath);
    },
    cleanup() {
      return fs.rm(rootDir, { recursive: true, force: true });
    }
  };
}
