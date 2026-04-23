import test from "node:test";

import rule from "../src/rules/no-direct-astro-content-in-routes.js";

import { baseOptions, createFixtureProject, createRuleTester } from "./rule-tester.js";

test("no-direct-astro-content-in-routes only fires on direct route imports", async () => {
  const project = await createFixtureProject({
    "src/pages/blog.ts": `
      import { getCollection } from "astro:content";

      export async function GET() {
        return await getCollection("posts");
      }
    `,
    "src/pages/adapter-ok.ts": `
      import { getPosts } from "../lib/content/posts";

      export async function GET() {
        return await getPosts();
      }
    `,
    "src/pages/types.ts": `
      import type { CollectionEntry } from "astro:content";
      export type { CollectionEntry } from "astro:content";

      const post: CollectionEntry<"posts"> | null = null;

      export function GET() {
        return post;
      }
    `,
    "src/pages/data.endpoint.ts": `
      export async function GET() {
        return await import("astro:content");
      }
    `,
    "src/pages/data-alias.endpoint.ts": `
      export async function GET() {
        const source = "astro:content";
        return await import(source);
      }
    `,
    "src/pages/data-require.endpoint.ts": `
      export async function GET() {
        const { getCollection } = require("astro:content");
        return await getCollection("posts");
      }
    `,
    "src/pages/data-require-alias.endpoint.ts": `
      export async function GET() {
        const req = require;
        const { getCollection } = req("astro:content");
        return await getCollection("posts");
      }
    `,
    "src/pages/data-create-require.endpoint.ts": `
      import { createRequire } from "node:module";

      export async function GET() {
        const req = createRequire(import.meta.url);
        const { getCollection } = req("astro:content");
        return await getCollection("posts");
      }
    `,
    "src/pages/data-module-require-alias.endpoint.ts": `
      import * as mod from "node:module";

      export async function GET() {
        const { require: req } = mod;
        const { getCollection } = req("astro:content");
        return await getCollection("posts");
      }
    `,
    "src/lib/content/posts.ts": `
      import { getCollection } from "astro:content";

      export function getPosts() {
        return getCollection("posts");
      }
    `
  });

  try {
    const tester = createRuleTester();

    tester.run("astro-pipeline/no-direct-astro-content-in-routes", rule, {
      valid: [
        {
          code: await project.read("src/pages/adapter-ok.ts"),
          filename: project.path("src/pages/adapter-ok.ts"),
          options: [baseOptions]
        },
        {
          code: await project.read("src/pages/types.ts"),
          filename: project.path("src/pages/types.ts"),
          options: [baseOptions]
        }
      ],
      invalid: [
        {
          code: await project.read("src/pages/blog.ts"),
          filename: project.path("src/pages/blog.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenImport"
            }
          ]
        },
        {
          code: await project.read("src/pages/data.endpoint.ts"),
          filename: project.path("src/pages/data.endpoint.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenImport"
            }
          ]
        },
        {
          code: await project.read("src/pages/data-alias.endpoint.ts"),
          filename: project.path("src/pages/data-alias.endpoint.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenImport"
            }
          ]
        },
        {
          code: await project.read("src/pages/data-require.endpoint.ts"),
          filename: project.path("src/pages/data-require.endpoint.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenImport"
            }
          ]
        },
        {
          code: await project.read("src/pages/data-require-alias.endpoint.ts"),
          filename: project.path("src/pages/data-require-alias.endpoint.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenImport"
            }
          ]
        },
        {
          code: await project.read("src/pages/data-create-require.endpoint.ts"),
          filename: project.path("src/pages/data-create-require.endpoint.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenImport"
            }
          ]
        },
        {
          code: await project.read("src/pages/data-module-require-alias.endpoint.ts"),
          filename: project.path("src/pages/data-module-require-alias.endpoint.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenImport"
            }
          ]
        }
      ]
    });
  } finally {
    await project.cleanup();
  }
});
