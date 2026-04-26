import test from "node:test";

import rule from "../src/rules/mdx-component-imports-from-approved-map.js";

import { baseOptions, createFixtureProject, createRuleTester } from "./rule-tester.js";

test("mdx-component-imports-from-approved-map allows only approved MDX component-map imports", async () => {
  const project = await createFixtureProject({
    "src/content/blog/good.mdx": `
      import { Callout, Figure } from "@/components/mdx";
      import type { CalloutProps } from "@/ui/private/widget";
    `,
    "src/content/blog/bad.mdx": `
      import { SomeRandomWidget } from "@/ui/private/widget";
      import Chart from "../../components/chart";
    `,
    "src/content/blog/bad-side-effect.mdx": `
      import "@/ui/private/widget";
    `,
    "src/content/blog/bad-re-export.mdx": `
      export { SomeRandomWidget } from "@/ui/private/widget";
      export * from "../../components/chart";
    `,
    "src/content/blog/bad-dynamic.mdx": `
      const source = "@/ui/private/widget";
      await import(source);
      require("../../components/chart");
    `,
    "src/content/blog/bad-unknown-dynamic.mdx": `
      await import(getComponentPath());
      require(getComponentPath());
    `,
    "src/pages/not-content.mdx": `
      import { SomeRandomWidget } from "@/ui/private/widget";
    `
  });

  try {
    const tester = createRuleTester();

    tester.run("astro-pipeline/mdx-component-imports-from-approved-map", rule, {
      valid: [
        {
          code: await project.read("src/content/blog/good.mdx"),
          filename: project.path("src/content/blog/good.mdx"),
          options: [baseOptions]
        },
        {
          code: await project.read("src/pages/not-content.mdx"),
          filename: project.path("src/pages/not-content.mdx"),
          options: [baseOptions]
        }
      ],
      invalid: [
        {
          code: await project.read("src/content/blog/bad.mdx"),
          filename: project.path("src/content/blog/bad.mdx"),
          options: [baseOptions],
          errors: [
            { messageId: "unapprovedMdxImport" },
            { messageId: "unapprovedMdxImport" }
          ]
        },
        {
          code: await project.read("src/content/blog/bad-side-effect.mdx"),
          filename: project.path("src/content/blog/bad-side-effect.mdx"),
          options: [baseOptions],
          errors: [{ messageId: "unapprovedMdxImport" }]
        },
        {
          code: await project.read("src/content/blog/bad-re-export.mdx"),
          filename: project.path("src/content/blog/bad-re-export.mdx"),
          options: [baseOptions],
          errors: [
            { messageId: "unapprovedMdxImport" },
            { messageId: "unapprovedMdxImport" }
          ]
        },
        {
          code: await project.read("src/content/blog/bad-dynamic.mdx"),
          filename: project.path("src/content/blog/bad-dynamic.mdx"),
          options: [baseOptions],
          errors: [
            { messageId: "unapprovedMdxImport" },
            { messageId: "unapprovedMdxImport" }
          ]
        },
        {
          code: await project.read("src/content/blog/bad-unknown-dynamic.mdx"),
          filename: project.path("src/content/blog/bad-unknown-dynamic.mdx"),
          options: [baseOptions],
          errors: [
            { messageId: "unapprovedMdxImport" },
            { messageId: "unapprovedMdxImport" }
          ]
        }
      ]
    });
  } finally {
    await project.cleanup();
  }
});
