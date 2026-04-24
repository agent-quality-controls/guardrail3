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
    "src/pages/helper.astro": `
      ---
      import HomePage from "../app/page";
      ---
      <HomePage />
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
          code: await project.read("src/pages/helper.astro"),
          filename: project.path("src/pages/helper.astro"),
          languageOptions: astroLanguageOptions,
          options: [baseOptions],
          errors: [{ messageId: "forbiddenImport" }]
        }
      ]
    });
  } finally {
    await project.cleanup();
  }
});
