import test from "node:test";

import rule from "../src/rules/no-authored-content-glob.js";

import { baseOptions, createFixtureProject, createRuleTester } from "./rule-tester.js";

test("no-authored-content-glob catches route helper globbing and respects approved adapters", async () => {
  const project = await createFixtureProject({
    "src/pages/posts.ts": `
      import { loadAllPosts } from "../lib/post-loader";

      export async function GET() {
        return await loadAllPosts();
      }
    `,
    "src/lib/post-loader.ts": `
      export function loadAllPosts() {
        return import.meta["glob"]("./src/content/posts/**/*.md");
      }
    `,
    "src/pages/adapter-ok.ts": `
      import { loadCollection } from "../lib/content/collection-loader";

      export function GET() {
        return loadCollection();
      }
    `,
    "src/lib/content/collection-loader.ts": `
      export function loadCollection() {
        return import.meta.glob("./src/content/posts/**/*.mdx");
      }
    `,
    "src/pages/generated-ok.ts": `
      export function GET() {
        return import.meta.glob("./src/generated/**/*.json");
      }
    `,
    "src/pages/computed-posts.ts": `
      import { loadAllPosts } from "../lib/post-loader-computed";

      export function GET() {
        return loadAllPosts();
      }
    `,
    "src/pages/legacy-adapter.ts": `
      import { loadLegacyPosts } from "../lib/adapters/legacy";

      export function GET() {
        return loadLegacyPosts();
      }
    `,
    "src/lib/post-loader-computed.ts": `
      import path from "node:path";

      const postGlob = path.join("src", "content", "posts", "**/*.mdx");

      export function loadAllPosts() {
        return import.meta.glob(postGlob);
      }
    `,
    "src/pages/posix-posts.ts": `
      import { loadAllPosts } from "../lib/post-loader-posix";

      export function GET() {
        return loadAllPosts();
      }
    `,
    "src/lib/post-loader-posix.ts": `
      import { posix } from "node:path";

      const postGlob = posix.join("src", "content", "posts", "**/*.mdx");

      export function loadAllPosts() {
        return import.meta.glob(postGlob);
      }
    `,
    "src/pages/process-cwd-posts.ts": `
      import { loadAllPosts } from "../lib/post-loader-process-cwd";

      export function GET() {
        return loadAllPosts();
      }
    `,
    "src/pages/alias-chain-posts.ts": `
      import { loadAllPosts } from "../lib/post-loader-alias-chain";

      export function GET() {
        return loadAllPosts();
      }
    `,
    "src/lib/post-loader-process-cwd.ts": `
      import path from "node:path";

      const postGlob = path.resolve(process.cwd(), "src", "content", "posts", "**/*.mdx");

      export function loadAllPosts() {
        return import.meta.glob(postGlob);
      }
    `,
    "src/lib/post-loader-alias-chain.ts": `
      const glob = import.meta.glob;
      const g = glob;

      export function loadAllPosts() {
        return g("./src/content/posts/**/*.mdx");
      }
    `,
    "src/lib/adapters/legacy.ts": `
      export function loadLegacyPosts() {
        return import.meta.glob("./src/content/posts/**/*.mdx");
      }
    `
  });

  try {
    const tester = createRuleTester();

    tester.run("astro-pipeline/no-authored-content-glob", rule, {
      valid: [
        {
          code: await project.read("src/pages/adapter-ok.ts"),
          filename: project.path("src/pages/adapter-ok.ts"),
          options: [baseOptions]
        },
        {
          code: await project.read("src/pages/generated-ok.ts"),
          filename: project.path("src/pages/generated-ok.ts"),
          options: [baseOptions]
        }
      ],
      invalid: [
        {
          code: await project.read("src/pages/posts.ts"),
          filename: project.path("src/pages/posts.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenGlob"
            }
          ]
        },
        {
          code: await project.read("src/pages/computed-posts.ts"),
          filename: project.path("src/pages/computed-posts.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenGlob"
            }
          ]
        },
        {
          code: await project.read("src/pages/posix-posts.ts"),
          filename: project.path("src/pages/posix-posts.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenGlob"
            }
          ]
        },
        {
          code: await project.read("src/pages/process-cwd-posts.ts"),
          filename: project.path("src/pages/process-cwd-posts.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenGlob"
            }
          ]
        },
        {
          code: await project.read("src/pages/alias-chain-posts.ts"),
          filename: project.path("src/pages/alias-chain-posts.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenGlob"
            }
          ]
        },
        {
          code: await project.read("src/pages/legacy-adapter.ts"),
          filename: project.path("src/pages/legacy-adapter.ts"),
          options: [
            {
              ...baseOptions,
              adapterModuleGlobs: ["src/lib/**/*.{ts,tsx,js,jsx}"]
            }
          ],
          errors: [
            {
              messageId: "forbiddenGlob"
            }
          ]
        }
      ]
    });
  } finally {
    await project.cleanup();
  }
});
