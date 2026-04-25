import test from "node:test";

import { ESLint } from "eslint";
import tsParser from "@typescript-eslint/parser";

import rule from "../src/rules/no-side-loader-imports.js";

import {
  astroLanguageOptions,
  baseOptions,
  createFixtureProject,
  createRuleTester
} from "./rule-tester.js";

test("no-side-loader-imports catches one-hop astro content and cross-root helpers", async () => {
  const project = await createFixtureProject({
    "src/pages/adapter-ok.ts": `
      import { getPosts } from "../lib/content/posts";

      export async function GET() {
        return await getPosts();
      }
    `,
    "src/lib/content/posts.ts": `
      import { getCollection } from "astro:content";

      export function getPosts() {
        return getCollection("posts");
      }
    `,
    "src/pages/type-only.ts": `
      import { getCollectionTypes } from "../lib/helpers/type-only";

      export function GET() {
        return getCollectionTypes();
      }
    `,
    "src/pages/reexport-type-only.ts": `
      import { getCollectionTypes } from "../lib/helpers/reexport-type-only";

      export function GET() {
        return getCollectionTypes();
      }
    `,
    "src/pages/cross-root-type-only.ts": `
      import { getCollectionTypes } from "../lib/helpers/cross-root-type-only";

      export function GET() {
        return getCollectionTypes();
      }
    `,
    "src/pages/helper-bypass.ts": `
      import { loadPosts } from "../lib/helpers/posts";

      export async function GET() {
        return await loadPosts();
      }
    `,
    "src/pages/nested-helper-bypass.ts": `
      import { loadPosts } from "../lib/helpers/nested-entry";

      export async function GET() {
        return await loadPosts();
      }
    `,
    "src/pages/require-computed-bypass.ts": `
      import { loadPosts } from "../lib/helpers/require-computed";

      export async function GET() {
        return await loadPosts();
      }
    `,
    "src/lib/helpers/posts.ts": `
      import { getCollection } from "astro:content";

      export function loadPosts() {
        return getCollection("posts");
      }
    `,
    "src/lib/helpers/type-only.ts": `
      import type { CollectionEntry } from "astro:content";

      export function getCollectionTypes(): CollectionEntry<"posts">[] {
        return [];
      }
    `,
    "src/lib/helpers/reexport-type-only.ts": `
      export { type CollectionEntry } from "astro:content";

      export function getCollectionTypes(): [] {
        return [];
      }
    `,
    "src/lib/helpers/cross-root-type-only.ts": `
      export type * from "../../../packages/content-mirror/src/posts";

      export function getCollectionTypes(): [] {
        return [];
      }
    `,
    "src/lib/helpers/nested-entry.ts": `
      import { loadNestedPosts } from "./nested-reader";

      export function loadPosts() {
        return loadNestedPosts();
      }
    `,
    "src/lib/helpers/nested-reader.ts": `
      import { getCollection } from "astro:content";

      export function loadNestedPosts() {
        return getCollection("posts");
      }
    `,
    "src/lib/helpers/require-computed.ts": `
      const source = "astro:content";
      const { getCollection } = require(source);

      export function loadPosts() {
        return getCollection("posts");
      }
    `,
    "src/pages/cross-root.ts": `
      import { mirroredPosts } from "../../packages/content-mirror/src/posts";

      export function GET() {
        return mirroredPosts;
      }
    `,
    "src/pages/safe-cross-root.ts": `
      import { formatDate } from "../../packages/shared/src/date";

      export function GET() {
        return formatDate("2026-04-23");
      }
    `,
    "src/pages/alias-bypass.ts": `
      import { loadPosts } from "@/lib/helpers/posts";

      export async function GET() {
        return await loadPosts();
      }
    `,
    "src/pages/dynamic-bypass.ts": `
      export async function GET() {
        const { loadPosts } = await import("../lib/helpers/posts");
        return await loadPosts();
      }
    `,
    "src/pages/computed-dynamic-bypass.ts": `
      export async function GET() {
        const helperPath = "../lib/helpers/posts";
        const { loadPosts } = await import(helperPath);
        return await loadPosts();
      }
    `,
    "src/pages/helper-bypass.astro": `
      ---
      import { loadPosts } from "../lib/helpers/posts";

      export async function GET() {
        return await loadPosts();
      }
      ---
      <main />
    `,
    "src/pages/astro-helper-bypass.ts": `
      import { loadPosts } from "../lib/helpers/posts.astro";

      export async function GET() {
        return await loadPosts();
      }
    `,
    "src/lib/helpers/posts.astro": `
      ---
      import { getCollection } from "astro:content";

      export async function loadPosts() {
        return await getCollection("posts");
      }
      ---
      <main />
    `,
    "packages/content-mirror/src/posts.ts": `
      import { getCollection } from "astro:content";

      export async function mirroredPosts() {
        return await getCollection("posts");
      }
    `,
    "packages/shared/src/date.ts": `
      export function formatDate(value: string) {
        return value;
      }
    `
  });

  try {
    const tester = createRuleTester();

    tester.run("astro-pipeline/no-side-loader-imports", rule, {
      valid: [
        {
          code: await project.read("src/pages/adapter-ok.ts"),
          filename: project.path("src/pages/adapter-ok.ts"),
          options: [baseOptions]
        },
        {
          code: await project.read("src/pages/type-only.ts"),
          filename: project.path("src/pages/type-only.ts"),
          options: [baseOptions]
        },
        {
          code: await project.read("src/pages/reexport-type-only.ts"),
          filename: project.path("src/pages/reexport-type-only.ts"),
          options: [baseOptions]
        },
        {
          code: await project.read("src/pages/cross-root-type-only.ts"),
          filename: project.path("src/pages/cross-root-type-only.ts"),
          options: [baseOptions]
        }
      ],
      invalid: [
        {
          code: await project.read("src/pages/helper-bypass.ts"),
          filename: project.path("src/pages/helper-bypass.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenSideLoader"
            }
          ]
        },
        {
          code: await project.read("src/pages/alias-bypass.ts"),
          filename: project.path("src/pages/alias-bypass.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenSideLoader"
            }
          ]
        },
        {
          code: await project.read("src/pages/nested-helper-bypass.ts"),
          filename: project.path("src/pages/nested-helper-bypass.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenSideLoader"
            }
          ]
        },
        {
          code: await project.read("src/pages/require-computed-bypass.ts"),
          filename: project.path("src/pages/require-computed-bypass.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenSideLoader"
            }
          ]
        },
        {
          code: await project.read("src/pages/dynamic-bypass.ts"),
          filename: project.path("src/pages/dynamic-bypass.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenSideLoader"
            }
          ]
        },
        {
          code: await project.read("src/pages/computed-dynamic-bypass.ts"),
          filename: project.path("src/pages/computed-dynamic-bypass.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenSideLoader"
            }
          ]
        },
        {
          code: await project.read("src/pages/helper-bypass.astro"),
          filename: project.path("src/pages/helper-bypass.astro"),
          languageOptions: astroLanguageOptions,
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenSideLoader"
            }
          ]
        },
        {
          code: await project.read("src/pages/astro-helper-bypass.ts"),
          filename: project.path("src/pages/astro-helper-bypass.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenSideLoader"
            }
          ]
        }
      ]
    });
  } finally {
    await project.cleanup();
  }
});

test("no-side-loader-imports catches unsafe cross-root helpers and allows safe cross-root utilities", async () => {
  const project = await createFixtureProject({
    "src/pages/cross-root.ts": `
      import { mirroredPosts } from "../../packages/content-mirror/src/posts";

      export async function GET() {
        return await mirroredPosts();
      }
    `,
    "src/pages/cross-root-require.ts": `
      import { mirroredPosts } from "../../packages/content-mirror/src/posts-require";

      export async function GET() {
        return await mirroredPosts();
      }
    `,
    "src/pages/cross-root-require-alias.ts": `
      import { mirroredPosts } from "../../packages/content-mirror/src/posts-require-alias";

      export async function GET() {
        return await mirroredPosts();
      }
    `,
    "src/pages/cross-root-require-computed.ts": `
      import { mirroredPosts } from "../../packages/content-mirror/src/posts-require-computed";

      export async function GET() {
        return await mirroredPosts();
      }
    `,
    "src/pages/cross-root-create-require.ts": `
      import { mirroredPosts } from "../../packages/content-mirror/src/posts-create-require";

      export async function GET() {
        return await mirroredPosts();
      }
    `,
    "src/pages/cross-root-module-require-alias.ts": `
      import { mirroredPosts } from "../../packages/content-mirror/src/posts-module-require-alias";

      export async function GET() {
        return await mirroredPosts();
      }
    `,
    "src/pages/safe-cross-root.ts": `
      import { formatDate } from "../../packages/shared/src/date";

      export function GET() {
        return formatDate("2026-04-23");
      }
    `,
    "packages/content-mirror/src/posts.ts": `
      import { getCollection } from "astro:content";

      export async function mirroredPosts() {
        return await getCollection("posts");
      }
    `,
    "packages/content-mirror/src/posts-require.ts": `
      const { getCollection } = require("astro:content");

      export async function mirroredPosts() {
        return await getCollection("posts");
      }
    `,
    "packages/content-mirror/src/posts-require-alias.ts": `
      const req = require;
      const { getCollection } = req("astro:content");

      export async function mirroredPosts() {
        return await getCollection("posts");
      }
    `,
    "packages/content-mirror/src/posts-require-computed.ts": `
      const source = "astro:content";
      const { getCollection } = require(source);

      export async function mirroredPosts() {
        return await getCollection("posts");
      }
    `,
    "packages/content-mirror/src/posts-create-require.ts": `
      import { createRequire } from "node:module";

      const req = createRequire(import.meta.url);
      const { getCollection } = req("astro:content");

      export async function mirroredPosts() {
        return await getCollection("posts");
      }
    `,
    "packages/content-mirror/src/posts-module-require-alias.ts": `
      import * as mod from "node:module";

      const { require: req } = mod;
      const { getCollection } = req("astro:content");

      export async function mirroredPosts() {
        return await getCollection("posts");
      }
    `,
    "packages/shared/src/date.ts": `
      export function formatDate(value: string) {
        return value;
      }
    `
  });

  try {
    async function lint(relativePath: string) {
      const eslint = new ESLint({
        cwd: project.rootDir,
        ignore: false,
        overrideConfigFile: true,
        overrideConfig: [
          {
            files: ["**/*.{ts,tsx}"],
            languageOptions: {
              ecmaVersion: "latest",
              parser: tsParser,
              sourceType: "module"
            },
            plugins: {
              "astro-pipeline": {
                rules: {
                  "no-side-loader-imports": rule
                }
              }
            },
            rules: {
              "astro-pipeline/no-side-loader-imports": ["error", baseOptions]
            }
          }
        ]
      });

      return eslint.lintText(await project.read(relativePath), {
        filePath: project.path(relativePath)
      });
    }

    const invalidCrossRoot = await lint("src/pages/cross-root.ts");
    const invalidCrossRootRequire = await lint("src/pages/cross-root-require.ts");
    const invalidCrossRootRequireAlias = await lint("src/pages/cross-root-require-alias.ts");
    const invalidCrossRootRequireComputed = await lint("src/pages/cross-root-require-computed.ts");
    const invalidCrossRootCreateRequire = await lint("src/pages/cross-root-create-require.ts");
    const invalidCrossRootModuleRequireAlias = await lint("src/pages/cross-root-module-require-alias.ts");
    const validSafeCrossRoot = await lint("src/pages/safe-cross-root.ts");

    if (invalidCrossRoot[0]?.messages.length !== 1) {
      throw new Error(`expected one cross-root violation, got ${JSON.stringify(invalidCrossRoot[0]?.messages)}`);
    }

    if (invalidCrossRootRequire[0]?.messages.length !== 1) {
      throw new Error(`expected one require-based cross-root violation, got ${JSON.stringify(invalidCrossRootRequire[0]?.messages)}`);
    }

    if (invalidCrossRootRequireAlias[0]?.messages.length !== 1) {
      throw new Error(`expected one require-alias cross-root violation, got ${JSON.stringify(invalidCrossRootRequireAlias[0]?.messages)}`);
    }

    if (invalidCrossRootRequireComputed[0]?.messages.length !== 1) {
      throw new Error(`expected one computed-require cross-root violation, got ${JSON.stringify(invalidCrossRootRequireComputed[0]?.messages)}`);
    }

    if (invalidCrossRootCreateRequire[0]?.messages.length !== 1) {
      throw new Error(`expected one createRequire-based cross-root violation, got ${JSON.stringify(invalidCrossRootCreateRequire[0]?.messages)}`);
    }

    if (invalidCrossRootModuleRequireAlias[0]?.messages.length !== 1) {
      throw new Error(`expected one module-require-alias cross-root violation, got ${JSON.stringify(invalidCrossRootModuleRequireAlias[0]?.messages)}`);
    }

    if ((validSafeCrossRoot[0]?.messages.length ?? 0) != 0) {
      throw new Error(`expected safe cross-root helper to pass, got ${JSON.stringify(validSafeCrossRoot[0]?.messages)}`);
    }
  } finally {
    await project.cleanup();
  }
});
