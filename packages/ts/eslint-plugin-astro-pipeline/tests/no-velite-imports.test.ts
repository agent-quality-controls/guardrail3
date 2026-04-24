import test from "node:test";

import rule from "../src/rules/no-velite-imports.js";

import {
  astroLanguageOptions,
  baseOptions,
  createFixtureProject,
  createRuleTester
} from "./rule-tester.js";

test("no-velite-imports catches velite package and generated artifact imports in route closures", async () => {
  const project = await createFixtureProject({
    "src/pages/generated.astro": `
      ---
      import HomePage from "../app/generated-page";
      ---
      <HomePage />
    `,
    "src/app/generated-page.tsx": `
      import { landing } from "../../.velite";

      export default function Page() {
        return <section>{landing.hero.title}</section>;
      }
    `,
    "src/pages/package.astro": `
      ---
      import HomePage from "../app/package-page";
      ---
      <HomePage />
    `,
    "src/app/package-page.tsx": `
      import { defineCollection } from "velite";

      export default function Page() {
        return <section>{typeof defineCollection}</section>;
      }
    `,
    "src/pages/valid.astro": `
      ---
      import HomePage from "../app/valid-page";
      ---
      <HomePage />
    `,
    "src/app/valid-page.tsx": `
      import { LandingHomepage } from "../components/landing/valid-homepage";

      export default function Page() {
        return <LandingHomepage title="Typed content" />;
      }
    `,
    "src/components/landing/valid-homepage.tsx": `
      export function LandingHomepage(props: { title: string }) {
        return <section>{props.title}</section>;
      }
    `
  });

  try {
    const tester = createRuleTester();

    tester.run("astro-pipeline/no-velite-imports", rule, {
      valid: [
        {
          code: await project.read("src/pages/valid.astro"),
          filename: project.path("src/pages/valid.astro"),
          languageOptions: astroLanguageOptions,
          options: [baseOptions]
        }
      ],
      invalid: [
        {
          code: await project.read("src/pages/generated.astro"),
          filename: project.path("src/pages/generated.astro"),
          languageOptions: astroLanguageOptions,
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenVeliteImport"
            }
          ]
        },
        {
          code: await project.read("src/pages/package.astro"),
          filename: project.path("src/pages/package.astro"),
          languageOptions: astroLanguageOptions,
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenVeliteImport"
            }
          ]
        }
      ]
    });
  } finally {
    await project.cleanup();
  }
});
