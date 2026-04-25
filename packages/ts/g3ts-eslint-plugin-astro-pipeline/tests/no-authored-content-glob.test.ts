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
    "src/pages/alias-glob.ts": `
      export function GET() {
        return import.meta.glob("@/content/posts/**/*.mdx");
      }
    `,
    "src/pages/array-glob.ts": `
      export function GET() {
        return import.meta.glob(["../content/posts/**/*.mdx"]);
      }
    `,
    "src/pages/array-alias-glob.ts": `
      const globs = ["../content/posts/**/*.mdx"];

      export function GET() {
        return import.meta.glob(globs);
      }
    `,
    "src/pages/array-ts-wrapped-glob.ts": `
      export function GET() {
        return import.meta.glob((["../content/posts/**/*.mdx"] as const));
      }
    `,
    "src/pages/destructured-glob.ts": `
      const { glob } = import.meta;

      export function GET() {
        return glob("../content/posts/**/*.mdx");
      }
    `,
    "src/pages/renamed-destructured-glob.ts": `
      const { glob: loadGlob } = import.meta;

      export function GET() {
        return loadGlob("../content/posts/**/*.mdx");
      }
    `,
    "src/pages/import-meta-alias-glob.ts": `
      const meta = import.meta;

      export function GET() {
        return meta.glob("../content/posts/**/*.mdx");
      }
    `,
    "src/pages/import-meta-alias-destructured-glob.ts": `
      const meta = import.meta;
      const { glob } = meta;

      export function GET() {
        return glob("../content/posts/**/*.mdx");
      }
    `,
    "src/pages/import-meta-member-alias-glob.ts": `
      const meta = import.meta;
      const glob = meta.glob;

      export function GET() {
        return glob("../content/posts/**/*.mdx");
      }
    `,
    "src/pages/defaulted-destructured-glob.ts": `
      const { glob = null } = import.meta;

      export function GET() {
        return glob("../content/posts/**/*.mdx");
      }
    `,
    "src/pages/adapter-ok.ts": `
      import { loadCollection } from "../lib/content/collection-loader";

      export function GET() {
        return loadCollection();
      }
    `,
    "src/pages/content-adapter-loader-ok.ts": `
      import { getPosts } from "../content/landing-homepage";

      export function GET() {
        return getPosts();
      }
    `,
    "src/lib/content/collection-loader.ts": `
      export function loadCollection() {
        return import.meta.glob("./src/content/posts/**/*.mdx");
      }
    `,
    "src/content/landing-homepage.ts": `
      import { loadPosts } from "./loaders/posts";

      export function getPosts() {
        return loadPosts();
      }
    `,
    "src/content/loaders/posts.ts": `
      export function loadPosts() {
        return import.meta.glob("../posts/**/*.mdx");
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
    "src/pages/shadowed-glob.ts": `
      const postGlob = "../content/posts/**/*.mdx";

      function unused() {
        const postGlob = "../generated/**/*.json";
        return postGlob;
      }

      export function GET() {
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
    "src/pages/process-cwd-alias-glob.ts": `
      import path from "node:path";

      const root = process.cwd();

      export function GET() {
        return import.meta.glob(path.resolve(root, "src", "content", "posts", "**/*.mdx"));
      }
    `,
    "src/pages/alias-chain-posts.ts": `
      import { loadAllPosts } from "../lib/post-loader-alias-chain";

      export function GET() {
        return loadAllPosts();
      }
    `,
    "src/pages/reexport-posts.ts": `
      import { loadAllPosts } from "../lib/post-loader-reexport";

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
    "src/lib/glob-export.ts": `
      export const globPosts = import.meta.glob;
    `,
    "src/lib/post-loader-reexport.ts": `
      import { globPosts } from "./glob-export";

      export function loadAllPosts() {
        return globPosts("./src/content/posts/**/*.mdx");
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
          code: await project.read("src/pages/content-adapter-loader-ok.ts"),
          filename: project.path("src/pages/content-adapter-loader-ok.ts"),
          options: [
            {
              ...baseOptions,
              approvedContentAdapterModules: ["src/content/landing-homepage.ts"],
              approvedLoaderModules: ["src/content/loaders/**/*.{ts,tsx,js,jsx}"]
            }
          ]
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
          code: await project.read("src/pages/alias-glob.ts"),
          filename: project.path("src/pages/alias-glob.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenGlob"
            }
          ]
        },
        {
          code: await project.read("src/pages/array-glob.ts"),
          filename: project.path("src/pages/array-glob.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenGlob"
            }
          ]
        },
        {
          code: await project.read("src/pages/array-alias-glob.ts"),
          filename: project.path("src/pages/array-alias-glob.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenGlob"
            }
          ]
        },
        {
          code: await project.read("src/pages/array-ts-wrapped-glob.ts"),
          filename: project.path("src/pages/array-ts-wrapped-glob.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenGlob"
            }
          ]
        },
        {
          code: await project.read("src/pages/destructured-glob.ts"),
          filename: project.path("src/pages/destructured-glob.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenGlob"
            }
          ]
        },
        {
          code: await project.read("src/pages/renamed-destructured-glob.ts"),
          filename: project.path("src/pages/renamed-destructured-glob.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenGlob"
            }
          ]
        },
        {
          code: await project.read("src/pages/import-meta-alias-glob.ts"),
          filename: project.path("src/pages/import-meta-alias-glob.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenGlob"
            }
          ]
        },
        {
          code: await project.read("src/pages/import-meta-alias-destructured-glob.ts"),
          filename: project.path("src/pages/import-meta-alias-destructured-glob.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenGlob"
            }
          ]
        },
        {
          code: await project.read("src/pages/import-meta-member-alias-glob.ts"),
          filename: project.path("src/pages/import-meta-member-alias-glob.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenGlob"
            }
          ]
        },
        {
          code: await project.read("src/pages/defaulted-destructured-glob.ts"),
          filename: project.path("src/pages/defaulted-destructured-glob.ts"),
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
          code: await project.read("src/pages/process-cwd-alias-glob.ts"),
          filename: project.path("src/pages/process-cwd-alias-glob.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenGlob"
            }
          ]
        },
        {
          code: await project.read("src/pages/shadowed-glob.ts"),
          filename: project.path("src/pages/shadowed-glob.ts"),
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
          code: await project.read("src/pages/reexport-posts.ts"),
          filename: project.path("src/pages/reexport-posts.ts"),
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
