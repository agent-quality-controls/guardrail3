import test from "node:test";

import rule from "../src/rules/no-authored-content-fs-read.js";

import { baseOptions, createFixtureProject, createRuleTester } from "./rule-tester.js";

test("no-authored-content-fs-read catches route helper filesystem reads and respects loader exemptions", async () => {
  const project = await createFixtureProject({
    "src/pages/blog.ts": `
      import { loadPost } from "../lib/load-post";

      export async function GET() {
        return await loadPost();
      }
    `,
    "src/lib/load-post.ts": `
      import { readFile as read } from "node:fs/promises";

      export async function loadPost() {
        return await read("./src/content/posts/hello.mdx", "utf8");
      }
    `,
    "src/pages/valid.ts": `
      import { loadGenerated } from "../lib/generated-reader";

      export async function GET() {
        return await loadGenerated();
      }
    `,
    "src/lib/generated-reader.ts": `
      import { readFile } from "node:fs/promises";

      export async function loadGenerated() {
        return await readFile("./src/generated/posts.json", "utf8");
      }
    `,
    "src/pages/adapter-ok.ts": `
      import { readCollectionEntry } from "../lib/content/approved-loader";

      export async function GET() {
        return await readCollectionEntry();
      }
    `,
    "src/lib/content/approved-loader.ts": `
      import { promises } from "node:fs";

      export async function readCollectionEntry() {
        return await promises.readFile("./src/content/posts/ok.mdx", "utf8");
      }
    `
  });

  try {
    const tester = createRuleTester();
    const invalidCode = await project.read("src/pages/blog.ts");
    const validCode = await project.read("src/pages/valid.ts");
    const adapterCode = await project.read("src/pages/adapter-ok.ts");

    tester.run("astro-pipeline/no-authored-content-fs-read", rule, {
      valid: [
        {
          code: validCode,
          filename: project.path("src/pages/valid.ts"),
          options: [baseOptions]
        },
        {
          code: adapterCode,
          filename: project.path("src/pages/adapter-ok.ts"),
          options: [baseOptions]
        }
      ],
      invalid: [
        {
          code: invalidCode,
          filename: project.path("src/pages/blog.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenRead"
            }
          ]
        }
      ]
    });
  } finally {
    await project.cleanup();
  }
});
