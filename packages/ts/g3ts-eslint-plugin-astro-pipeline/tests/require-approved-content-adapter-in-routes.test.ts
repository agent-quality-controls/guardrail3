import test from "node:test";

import rule from "../src/rules/require-approved-content-adapter-in-routes.js";

import {
  astroLanguageOptions,
  baseOptions,
  createFixtureProject,
  createRuleTester
} from "./rule-tester.js";

test("require-approved-content-adapter-in-routes requires public routes to import approved adapter", async () => {
  const project = await createFixtureProject({
    "src/pages/index.astro": `
      ---
      import { getLandingPage } from "../lib/content/landing-homepage";
      const page = await getLandingPage();
      ---
      <h1>{page.title}</h1>
    `,
    "src/pages/about.ts": `
      import { getLandingPage } from "../lib/content/landing-homepage";

      export async function getStaticPaths() {
        return [{ params: { slug: "about" }, props: { page: await getLandingPage() } }];
      }
    `,
    "src/pages/blog.tsx": `
      export function BlogPage() {
        return <h1>Hardcoded page</h1>;
      }
    `,
    "src/pages/dynamic.ts": `
      const adapter = "../lib/content/landing-homepage";
      export async function getStaticPaths() {
        return [{ props: { page: await import(adapter) } }];
      }
    `,
    "src/pages/dynamic-ts-wrapper.ts": `
      const adapter = "../lib/content/landing-homepage" as const;
      export async function getStaticPaths() {
        return [{ props: { page: await import(adapter) } }];
      }
    `,
    "src/pages/dynamic-satisfies.ts": `
      const adapter = "../lib/content/" + "landing-homepage" satisfies string;
      export async function getStaticPaths() {
        return [{ props: { page: await import(adapter) } }];
      }
    `,
    "src/pages/static-concat.ts": `
      export async function getStaticPaths() {
        const page = await import("../lib/content/" + "landing-homepage");
        return [{ props: { page } }];
      }
    `,
    "src/pages/template.ts": `
      const contentRoot = "../lib/content";

      export async function getStaticPaths() {
        const page = await import(\`\${contentRoot}/landing-homepage\`);
        return [{ props: { page } }];
      }
    `,
    "src/pages/require-alias.ts": `
      const req = require;

      export function getStaticPaths() {
        const page = req("../lib/content/landing-homepage");
        return [{ props: { page } }];
      }
    `,
    "src/pages/extension.ts": `
      export async function getStaticPaths() {
        const page = await import("../lib/content/landing-homepage.mjs");
        return [{ props: { page } }];
      }
    `,
    "src/pages/empty-import.ts": `
      import {} from "../lib/content/landing-homepage";

      export function getStaticPaths() {
        return [];
      }
    `,
    "src/pages/side-effect-import.ts": `
      import "../lib/content/landing-homepage";

      export function getStaticPaths() {
        return [];
      }
    `,
    "src/pages/type-only.ts": `
      import type { LandingPage } from "../lib/content/landing-homepage";

      export function getStaticPaths(): LandingPage[] {
        return [];
      }
    `,
    "src/pages/reexport-type-only.ts": `
      export type { LandingPage } from "../lib/content/landing-homepage";

      export function getStaticPaths() {
        return [];
      }
    `,
    "src/pages/reexport-runtime.ts": `
      export { getLandingPage } from "../lib/content/landing-homepage";

      export function getStaticPaths() {
        return [];
      }
    `,
    "src/pages/reexport-all-runtime.ts": `
      export * from "../lib/content/landing-homepage";

      export function getStaticPaths() {
        return [];
      }
    `,
    "src/pages/inline-type-only.ts": `
      import { type LandingPage } from "../lib/content/landing-homepage";

      export function getStaticPaths(): LandingPage[] {
        return [];
      }
    `,
    "src/pages/feed.endpoint.ts": `
      export async function GET() {
        return new Response("ok");
      }
    `,
    "src/lib/content/landing-homepage.ts": `
      export interface LandingPage {
        title: string;
      }

      export function getLandingPage() {
        return { title: "from content" };
      }
    `,
    "src/lib/content/landing-homepage.mjs": `
      export function getLandingPage() {
        return { title: "from content" };
      }
    `
  });

  try {
    const tester = createRuleTester();

    tester.run("astro-pipeline/require-approved-content-adapter-in-routes", rule, {
      valid: [
        {
          code: await project.read("src/pages/index.astro"),
          filename: project.path("src/pages/index.astro"),
          languageOptions: astroLanguageOptions,
          options: [baseOptions]
        },
        {
          code: await project.read("src/pages/about.ts"),
          filename: project.path("src/pages/about.ts"),
          options: [baseOptions]
        },
        {
          code: await project.read("src/pages/dynamic.ts"),
          filename: project.path("src/pages/dynamic.ts"),
          options: [baseOptions]
        },
        {
          code: await project.read("src/pages/dynamic-ts-wrapper.ts"),
          filename: project.path("src/pages/dynamic-ts-wrapper.ts"),
          options: [baseOptions]
        },
        {
          code: await project.read("src/pages/dynamic-satisfies.ts"),
          filename: project.path("src/pages/dynamic-satisfies.ts"),
          options: [baseOptions]
        },
        {
          code: await project.read("src/pages/static-concat.ts"),
          filename: project.path("src/pages/static-concat.ts"),
          options: [baseOptions]
        },
        {
          code: await project.read("src/pages/template.ts"),
          filename: project.path("src/pages/template.ts"),
          options: [baseOptions]
        },
        {
          code: await project.read("src/pages/require-alias.ts"),
          filename: project.path("src/pages/require-alias.ts"),
          options: [baseOptions]
        },
        {
          code: await project.read("src/pages/extension.ts"),
          filename: project.path("src/pages/extension.ts"),
          options: [
            {
              ...baseOptions,
              approvedContentAdapterModules: ["src/lib/content/landing-homepage.mjs"]
            }
          ]
        },
        {
          code: await project.read("src/pages/feed.endpoint.ts"),
          filename: project.path("src/pages/feed.endpoint.ts"),
          options: [baseOptions]
        }
      ],
      invalid: [
        {
          code: await project.read("src/pages/blog.tsx"),
          filename: project.path("src/pages/blog.tsx"),
          options: [baseOptions],
          errors: [
            {
              messageId: "missingAdapter"
            }
          ]
        },
        {
          code: await project.read("src/pages/type-only.ts"),
          filename: project.path("src/pages/type-only.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "missingAdapter"
            }
          ]
        },
        {
          code: await project.read("src/pages/reexport-type-only.ts"),
          filename: project.path("src/pages/reexport-type-only.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "missingAdapter"
            }
          ]
        },
        {
          code: await project.read("src/pages/reexport-runtime.ts"),
          filename: project.path("src/pages/reexport-runtime.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "missingAdapter"
            }
          ]
        },
        {
          code: await project.read("src/pages/reexport-all-runtime.ts"),
          filename: project.path("src/pages/reexport-all-runtime.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "missingAdapter"
            }
          ]
        },
        {
          code: await project.read("src/pages/inline-type-only.ts"),
          filename: project.path("src/pages/inline-type-only.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "missingAdapter"
            }
          ]
        },
        {
          code: await project.read("src/pages/empty-import.ts"),
          filename: project.path("src/pages/empty-import.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "missingAdapter"
            }
          ]
        },
        {
          code: await project.read("src/pages/side-effect-import.ts"),
          filename: project.path("src/pages/side-effect-import.ts"),
          options: [baseOptions],
          errors: [
            {
              messageId: "missingAdapter"
            }
          ]
        }
      ]
    });
  } finally {
    await project.cleanup();
  }
});
