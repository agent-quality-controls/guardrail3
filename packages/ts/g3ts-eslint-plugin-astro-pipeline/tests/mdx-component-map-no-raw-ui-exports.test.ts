import test from "node:test";

import rule from "../src/rules/mdx-component-map-no-raw-ui-exports.js";

import { baseOptions, createFixtureProject, createRuleTester } from "./rule-tester.js";

test("mdx-component-map-no-raw-ui-exports forbids exposing raw UI directly", async () => {
  const project = await createFixtureProject({
    "src/components/mdx/good.tsx": `
      import { FaqSection } from "@project/ui";

      export function FAQ(rawProps: unknown) {
        return <FaqSection items={[]} />;
      }
    `,
    "src/components/mdx/bad-re-export.tsx": `
      export { FaqSection } from "@project/ui";
    `,
    "src/components/mdx/bad-export-all.tsx": `
      export * from "@project/ui";
    `,
    "src/components/mdx/bad-export-imported.tsx": `
      import { FaqSection } from "@project/ui";
      export { FaqSection };
    `,
    "src/components/mdx/bad-alias.tsx": `
      import { FaqSection } from "@project/ui";
      export const FAQ = FaqSection;
    `
  });

  try {
    createRuleTester().run("astro-pipeline/mdx-component-map-no-raw-ui-exports", rule, {
      valid: [
        {
          code: await project.read("src/components/mdx/good.tsx"),
          filename: project.path("src/components/mdx/good.tsx"),
          options: [baseOptions]
        }
      ],
      invalid: [
        {
          code: await project.read("src/components/mdx/bad-re-export.tsx"),
          filename: project.path("src/components/mdx/bad-re-export.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "rawUiExport" }]
        },
        {
          code: await project.read("src/components/mdx/bad-export-all.tsx"),
          filename: project.path("src/components/mdx/bad-export-all.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "rawUiExport" }]
        },
        {
          code: await project.read("src/components/mdx/bad-export-imported.tsx"),
          filename: project.path("src/components/mdx/bad-export-imported.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "rawUiExport" }]
        },
        {
          code: await project.read("src/components/mdx/bad-alias.tsx"),
          filename: project.path("src/components/mdx/bad-alias.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "rawUiExport" }]
        },
        {
          code: await project.read("src/components/mdx/good.tsx"),
          filename: project.path("src/components/mdx/good.tsx"),
          options: [{ ...baseOptions, rawUiModuleGlobs: [] }],
          errors: [{ messageId: "missingRawUiGlobs" }]
        }
      ]
    });
  } finally {
    await project.cleanup();
  }
});
