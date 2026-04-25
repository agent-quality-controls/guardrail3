import test from "node:test";

import rule from "../src/rules/no-authored-content-imports.js";

import {
  astroLanguageOptions,
  baseOptions,
  createFixtureProject,
  createRuleTester
} from "./rule-tester.js";

test("no-authored-content-imports catches direct raw content imports in route closures", async () => {
  const project = await createFixtureProject({
    "src/pages/direct.astro": `
      ---
      import { HOME } from "../content/home.json";
      ---
      <section>{HOME.title}</section>
    `,
    "src/pages/alias-direct.astro": `
      ---
      import { HOME } from "@/content/home.json";
      ---
      <section>{HOME.title}</section>
    `,
    "src/pages/content-alias-direct.astro": `
      ---
      import HOME from "@content/posts/hello.mdx";
      ---
      <section>{HOME.title}</section>
    `,
    "src/pages/content-root-alias-direct.ts": `
      import content from "@content";

      export function GET() {
        return new Response(String(content));
      }
    `,
    "src/pages/helper.astro": `
      ---
      import HomePage from "../app/page";
      ---
      <HomePage />
    `,
    "src/pages/require-computed.ts": `
      const contentSource = "../content/home.json";
      const data = require(contentSource);

      export function GET() {
        return new Response(data.title);
      }
    `,
    "src/pages/shadowed-computed.ts": `
      const contentSource = "../content/home.json";

      function unused() {
        const contentSource = "../generated/content/post";
        return contentSource;
      }

      export async function GET() {
        const data = await import(contentSource);
        return new Response(data.default.title);
      }
    `,
    "src/pages/url-pathname-dynamic.ts": `
      export async function GET() {
        const data = await import(new URL("../content/home.json", import.meta.url).pathname);
        return new Response(String(data.default.title));
      }
    `,
    "src/pages/path-built-dynamic.ts": `
      import path from "node:path";

      const source = path.join("..", "content", "home.json");

      export async function GET() {
        const data = await import(source);
        return new Response(String(data.default.title));
      }
    `,
    "src/pages/process-cwd-built-dynamic.ts": `
      import path from "node:path";

      const source = path.resolve(process.cwd(), "src", "content", "posts", "hello.mdx");

      export async function GET() {
        const data = await import(source);
        return new Response(String(data.default.title));
      }
    `,
    "src/pages/process-cwd-alias-built-dynamic.ts": `
      import path from "node:path";

      const root = process.cwd();
      const source = path.resolve(root, "src", "content", "posts", "hello.mdx");

      export async function GET() {
        const data = await import(source);
        return new Response(String(data.default.title));
      }
    `,
    "src/pages/posix-built-dynamic.ts": `
      import { posix } from "node:path";

      const source = posix.join("..", "content", "home.json");

      export async function GET() {
        const data = await import(source);
        return new Response(String(data.default.title));
      }
    `,
    "src/app/page.tsx": `
      import { hero } from "../content/landing/home.mdx";

      export default function Page() {
        return <section>{hero.title}</section>;
      }
    `,
    "src/pages/generated-ok.astro": `
      ---
      import { post } from "../generated/content/post";
      ---
      <section>{post.title}</section>
    `,
    "src/generated/content/post.ts": `
      export const post = { title: "Generated artifact" };
    `,
    "src/pages/adapter-ok.astro": `
      ---
      import HomePage from "../app/adapter-page";
      ---
      <HomePage />
    `,
    "src/pages/planned-adapter-ok.ts": `
      import { getLanding } from "../content/landing-homepage";

      export function GET() {
        return new Response(getLanding().title);
      }
    `,
    "src/app/adapter-page.tsx": `
      import { loadLandingPage } from "../lib/content/loaders/landing";

      export default function Page() {
        const page = loadLandingPage();
        return <section>{page.title}</section>;
      }
    `,
    "src/lib/content/loaders/landing.ts": `
      import data from "../../content/home.json";

      export function loadLandingPage() {
        return data;
      }
    `,
    "src/content/landing-homepage.ts": `
      import raw from "./landing/home.json";

      export function getLanding() {
        return raw;
      }
    `
  });

  try {
    const tester = createRuleTester();

    tester.run("astro-pipeline/no-authored-content-imports", rule, {
      valid: [
        {
          code: await project.read("src/pages/generated-ok.astro"),
          filename: project.path("src/pages/generated-ok.astro"),
          languageOptions: astroLanguageOptions,
          options: [baseOptions]
        },
        {
          code: await project.read("src/pages/adapter-ok.astro"),
          filename: project.path("src/pages/adapter-ok.astro"),
          languageOptions: astroLanguageOptions,
          options: [baseOptions]
        },
        {
          code: await project.read("src/pages/planned-adapter-ok.ts"),
          filename: project.path("src/pages/planned-adapter-ok.ts"),
          options: [
            {
              ...baseOptions,
              approvedContentAdapterModules: ["src/content/landing-homepage.ts"],
              approvedLoaderModules: []
            }
          ]
        }
      ],
      invalid: [
        {
          code: await project.read("src/pages/direct.astro"),
          filename: project.path("src/pages/direct.astro"),
          languageOptions: astroLanguageOptions,
          options: [baseOptions],
          errors: [{ messageId: "forbiddenImport" }]
        },
        {
          code: await project.read("src/pages/alias-direct.astro"),
          filename: project.path("src/pages/alias-direct.astro"),
          languageOptions: astroLanguageOptions,
          options: [baseOptions],
          errors: [{ messageId: "forbiddenImport" }]
        },
        {
          code: await project.read("src/pages/content-alias-direct.astro"),
          filename: project.path("src/pages/content-alias-direct.astro"),
          languageOptions: astroLanguageOptions,
          options: [baseOptions],
          errors: [{ messageId: "forbiddenImport" }]
        },
        {
          code: await project.read("src/pages/content-root-alias-direct.ts"),
          filename: project.path("src/pages/content-root-alias-direct.ts"),
          options: [baseOptions],
          errors: [{ messageId: "forbiddenImport" }]
        },
        {
          code: await project.read("src/pages/helper.astro"),
          filename: project.path("src/pages/helper.astro"),
          languageOptions: astroLanguageOptions,
          options: [baseOptions],
          errors: [{ messageId: "forbiddenImport" }]
        },
        {
          code: await project.read("src/pages/require-computed.ts"),
          filename: project.path("src/pages/require-computed.ts"),
          options: [baseOptions],
          errors: [{ messageId: "forbiddenImport" }]
        },
        {
          code: await project.read("src/pages/shadowed-computed.ts"),
          filename: project.path("src/pages/shadowed-computed.ts"),
          options: [baseOptions],
          errors: [{ messageId: "forbiddenImport" }]
        },
        {
          code: await project.read("src/pages/url-pathname-dynamic.ts"),
          filename: project.path("src/pages/url-pathname-dynamic.ts"),
          options: [baseOptions],
          errors: [{ messageId: "forbiddenImport" }]
        },
        {
          code: await project.read("src/pages/path-built-dynamic.ts"),
          filename: project.path("src/pages/path-built-dynamic.ts"),
          options: [baseOptions],
          errors: [{ messageId: "forbiddenImport" }]
        },
        {
          code: await project.read("src/pages/process-cwd-built-dynamic.ts"),
          filename: project.path("src/pages/process-cwd-built-dynamic.ts"),
          options: [baseOptions],
          errors: [{ messageId: "forbiddenImport" }]
        },
        {
          code: await project.read("src/pages/process-cwd-alias-built-dynamic.ts"),
          filename: project.path("src/pages/process-cwd-alias-built-dynamic.ts"),
          options: [baseOptions],
          errors: [{ messageId: "forbiddenImport" }]
        },
        {
          code: await project.read("src/pages/posix-built-dynamic.ts"),
          filename: project.path("src/pages/posix-built-dynamic.ts"),
          options: [baseOptions],
          errors: [{ messageId: "forbiddenImport" }]
        }
      ]
    });
  } finally {
    await project.cleanup();
  }
});
