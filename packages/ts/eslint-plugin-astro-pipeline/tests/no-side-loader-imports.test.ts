import test from "node:test";

import rule from "../src/rules/no-side-loader-imports.js";

import { baseOptions, createFixtureProject, createRuleTester } from "./rule-tester.js";

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
    "src/pages/helper-bypass.ts": `
      import { loadPosts } from "../lib/helpers/posts";

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
    "src/pages/cross-root.ts": `
      import { mirroredPosts } from "../../packages/content-mirror/src/posts";

      export function GET() {
        return mirroredPosts;
      }
    `,
    "packages/content-mirror/src/posts.ts": `
      export const mirroredPosts = [];
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
          code: await project.read("src/pages/cross-root.ts"),
          filename: project.path("src/pages/cross-root.ts"),
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
