import test from "node:test";

import rule from "../src/rules/no-content-data-modules-in-routes.js";

import {
  astroLanguageOptions,
  baseOptions,
  createFixtureProject,
  createRuleTester
} from "./rule-tester.js";

test("no-content-data-modules-in-routes catches route closures that reach data modules", async () => {
  const project = await createFixtureProject({
    "src/pages/index.astro": `
      ---
      import HomePage from "../app/page";
      ---
      <HomePage />
    `,
    "src/app/page.tsx": `
      import { LandingHomepage } from "../components/landing/homepage";

      export default function Page() {
        return <LandingHomepage />;
      }
    `,
    "src/components/landing/homepage.tsx": `
      import { HERO_COPY } from "./homepage-v2.data";

      export function LandingHomepage() {
        return <section>{HERO_COPY.title}</section>;
      }
    `,
    "src/components/landing/homepage-v2.data.ts": `
      export const HERO_COPY = {
        title: "Hardcoded landing copy"
      };
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
    `,
    "src/app/custom-page.tsx": `
      import { HOME_PAGE_COPY } from "../marketing/copy/homepage";

      export default function Page() {
        return <section>{HOME_PAGE_COPY.title}</section>;
      }
    `,
    "src/marketing/copy/homepage.ts": `
      export const HOME_PAGE_COPY = {
        title: "Custom configured data module"
      };
    `
  });

  try {
    const tester = createRuleTester();

    tester.run("astro-pipeline/no-content-data-modules-in-routes", rule, {
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
          code: await project.read("src/pages/index.astro"),
          filename: project.path("src/pages/index.astro"),
          languageOptions: astroLanguageOptions,
          options: [baseOptions],
          errors: [
            {
              messageId: "forbiddenContentDataModule"
            }
          ]
        },
        {
          code: `
            ---
            import HomePage from "../app/custom-page";
            ---
            <HomePage />
          `,
          filename: project.path("src/pages/custom.astro"),
          languageOptions: astroLanguageOptions,
          options: [
            {
              ...baseOptions,
              contentDataModuleGlobs: ["src/marketing/copy/**/*.{ts,tsx}"]
            }
          ],
          errors: [
            {
              messageId: "forbiddenContentDataModule"
            }
          ]
        }
      ]
    });
  } finally {
    await project.cleanup();
  }
});
