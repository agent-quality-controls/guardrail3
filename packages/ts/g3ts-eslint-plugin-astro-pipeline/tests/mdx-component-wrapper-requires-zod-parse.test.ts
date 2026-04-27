import test from "node:test";

import rule from "../src/rules/mdx-component-wrapper-requires-zod-parse.js";

import { baseOptions, createFixtureProject, createRuleTester } from "./rule-tester.js";

test("mdx-component-wrapper-requires-zod-parse requires every approved export to parse props", async () => {
  const project = await createFixtureProject({
    "src/components/mdx/good.tsx": `
      import { z } from "zod";
      import { FaqSection } from "@project/ui";
      import { parseMdxComponentProps } from "./parse";

      const faqPropsSchema = z.object({ items: z.array(z.string()).min(1) });

      export function FAQ(rawProps: unknown) {
        const props = parseMdxComponentProps("FAQ", faqPropsSchema, rawProps);
        return <FaqSection items={props.items} />;
      }
    `,
    "src/components/mdx/bad-missing-parse.tsx": `
      import { z } from "zod";
      const faqPropsSchema = z.object({ items: z.array(z.string()) });
      export function FAQ(rawProps: unknown) {
        return null;
      }
    `,
    "src/components/mdx/bad-wrong-name.tsx": `
      import { z } from "zod";
      import { parseMdxComponentProps } from "./parse";
      const faqPropsSchema = z.object({ items: z.array(z.string()) });
      export function FAQ(rawProps: unknown) {
        parseMdxComponentProps("Callout", faqPropsSchema, rawProps);
        return null;
      }
    `,
    "src/components/mdx/bad-non-zod-schema.tsx": `
      import { z } from "zod";
      import { parseMdxComponentProps } from "./parse";
      const faqPropsSchema = {};
      export function FAQ(rawProps: unknown) {
        parseMdxComponentProps("FAQ", faqPropsSchema, rawProps);
        return null;
      }
    `,
    "src/components/mdx/bad-extra-export.tsx": `
      import { z } from "zod";
      export const Helper = 1;
    `,
    "src/components/mdx/good-map-export.tsx": `
      import { z } from "zod";
      import { parseMdxComponentProps } from "./parse";

      const faqPropsSchema = z.object({ items: z.array(z.string()).min(1) });

      export function FAQ(rawProps: unknown) {
        const props = parseMdxComponentProps("FAQ", faqPropsSchema, rawProps);
        return null;
      }

      export const mdxComponents = { FAQ };
    `
  });

  try {
    createRuleTester().run("astro-pipeline/mdx-component-wrapper-requires-zod-parse", rule, {
      valid: [
        {
          code: await project.read("src/components/mdx/good.tsx"),
          filename: project.path("src/components/mdx/good.tsx"),
          options: [baseOptions]
        },
        {
          code: await project.read("src/components/mdx/good-map-export.tsx"),
          filename: project.path("src/components/mdx/good-map-export.tsx"),
          options: [baseOptions]
        }
      ],
      invalid: [
        {
          code: await project.read("src/components/mdx/bad-missing-parse.tsx"),
          filename: project.path("src/components/mdx/bad-missing-parse.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "missingParserCall" }]
        },
        {
          code: await project.read("src/components/mdx/bad-wrong-name.tsx"),
          filename: project.path("src/components/mdx/bad-wrong-name.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "invalidParserCall" }]
        },
        {
          code: await project.read("src/components/mdx/bad-non-zod-schema.tsx"),
          filename: project.path("src/components/mdx/bad-non-zod-schema.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "missingZodSchema" }]
        },
        {
          code: await project.read("src/components/mdx/bad-extra-export.tsx"),
          filename: project.path("src/components/mdx/bad-extra-export.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "unexpectedExport" }]
        },
        {
          code: await project.read("src/components/mdx/good.tsx"),
          filename: project.path("src/components/mdx/good.tsx"),
          options: [{ ...baseOptions, mdxPropsParserName: [] }],
          errors: [{ messageId: "missingParserName" }]
        }
      ]
    });
  } finally {
    await project.cleanup();
  }
});
