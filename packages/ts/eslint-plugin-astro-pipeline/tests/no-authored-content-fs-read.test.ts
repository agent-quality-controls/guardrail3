import test from "node:test";

import rule from "../src/rules/no-authored-content-fs-read.js";

import {
  astroLanguageOptions,
  baseOptions,
  createFixtureProject,
  createRuleTester
} from "./rule-tester.js";

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
    "src/pages/alias-blog.ts": `
      import { loadPost } from "@/lib/load-post";

      export async function GET() {
        return await loadPost();
      }
    `,
    "src/pages/computed-blog.ts": `
      import { loadPost } from "../lib/load-post-computed";

      export async function GET() {
        return await loadPost();
      }
    `,
    "src/pages/dynamic-blog.ts": `
      export async function GET() {
        const { loadPost } = await import("../lib/load-post");
        return await loadPost();
      }
    `,
    "src/pages/blog.astro": `
      ---
      import { loadPost } from "../lib/load-post";

      export async function GET() {
        return await loadPost();
      }
      ---
      <main />
    `,
    "src/lib/content/approved-loader.ts": `
      import { promises } from "node:fs";

      export async function readCollectionEntry() {
        return await promises.readFile("./src/content/posts/ok.mdx", "utf8");
      }
    `,
    "src/lib/load-post-computed.ts": `
      import { readFile } from "node:fs/promises";
      import path from "node:path";

      const postPath = path.join("src", "content", "posts", "computed.mdx");

      export async function loadPost() {
        return await readFile(postPath, "utf8");
      }
    `,
    "src/pages/local-computed-blog.ts": `
      import { loadPost } from "../lib/load-post-local";

      export async function GET() {
        return await loadPost();
      }
    `,
    "src/lib/load-post-local.ts": `
      import { readFile } from "node:fs/promises";
      import path from "node:path";

      export async function loadPost() {
        const postPath = path.join("src", "content", "posts", "local.mdx");
        return await readFile(postPath, "utf8");
      }
    `,
    "src/pages/posix-blog.ts": `
      import { loadPost } from "../lib/load-post-posix";

      export async function GET() {
        return await loadPost();
      }
    `,
    "src/lib/load-post-posix.ts": `
      import { readFile } from "node:fs/promises";
      import { posix } from "node:path";

      export async function loadPost() {
        const postPath = posix.join("src", "content", "posts", "posix.mdx");
        return await readFile(postPath, "utf8");
      }
    `,
    "src/pages/process-cwd-blog.ts": `
      import { loadPost } from "../lib/load-post-process-cwd";

      export async function GET() {
        return await loadPost();
      }
    `,
    "src/lib/load-post-process-cwd.ts": `
      import { readFile } from "node:fs/promises";
      import path from "node:path";

      export async function loadPost() {
        return await readFile(path.resolve(process.cwd(), "src", "content", "posts", "cwd.mdx"), "utf8");
      }
    `,
    "src/pages/url-blog.ts": `
      import { loadPost } from "../lib/load-post-url";

      export async function GET() {
        return await loadPost();
      }
    `,
    "src/pages/require-blog.ts": `
      import { loadPost } from "../lib/load-post-require";

      export async function GET() {
        return await loadPost();
      }
    `,
    "src/lib/load-post-url.ts": `
      import { readFile } from "node:fs/promises";

      export async function loadPost() {
        return await readFile(new URL("./src/content/posts/url.mdx", import.meta.url), "utf8");
      }
    `,
    "src/lib/load-post-require.ts": `
      const { readPost } = require("./reader");

      export async function loadPost() {
        return await readPost();
      }
    `,
    "src/lib/reader.ts": `
      import { readFile } from "node:fs/promises";

      export async function readPost() {
        return await readFile("./src/content/posts/require.mdx", "utf8");
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
        },
        {
          code: await project.read("src/pages/alias-blog.ts"),
          filename: project.path("src/pages/alias-blog.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenRead"
            }
          ]
        },
        {
          code: await project.read("src/pages/dynamic-blog.ts"),
          filename: project.path("src/pages/dynamic-blog.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenRead"
            }
          ]
        },
        {
          code: await project.read("src/pages/computed-blog.ts"),
          filename: project.path("src/pages/computed-blog.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenRead"
            }
          ]
        },
        {
          code: await project.read("src/pages/local-computed-blog.ts"),
          filename: project.path("src/pages/local-computed-blog.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenRead"
            }
          ]
        },
        {
          code: await project.read("src/pages/url-blog.ts"),
          filename: project.path("src/pages/url-blog.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenRead"
            }
          ]
        },
        {
          code: await project.read("src/pages/require-blog.ts"),
          filename: project.path("src/pages/require-blog.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenRead"
            }
          ]
        },
        {
          code: await project.read("src/pages/posix-blog.ts"),
          filename: project.path("src/pages/posix-blog.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenRead"
            }
          ]
        },
        {
          code: await project.read("src/pages/process-cwd-blog.ts"),
          filename: project.path("src/pages/process-cwd-blog.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenRead"
            }
          ]
        },
        {
          code: await project.read("src/pages/blog.astro"),
          filename: project.path("src/pages/blog.astro"),
          languageOptions: astroLanguageOptions,
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
