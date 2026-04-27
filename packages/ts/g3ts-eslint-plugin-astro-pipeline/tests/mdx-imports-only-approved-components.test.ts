import test from "node:test";

import rule from "../src/rules/mdx-imports-only-approved-components.js";

import { baseOptions, createFixtureProject, createRuleTester } from "./rule-tester.js";

test("mdx-imports-only-approved-components requires approved modules and approved names", async () => {
  const project = await createFixtureProject({
    "src/content/blog/good.mdx": `
      import { Callout, ArticleImage } from "@/components/mdx";
    `,
    "src/content/blog/bad-name.mdx": `
      import { RandomWidget } from "@/components/mdx";
    `,
    "src/content/blog/bad-module.mdx": `
      import { Callout } from "@/ui/private/widget";
    `,
    "src/content/blog/bad-default.mdx": `
      import Callout from "@/components/mdx";
    `,
    "src/content/blog/bad-namespace.mdx": `
      import * as MdxComponents from "@/components/mdx";
    `
  });

  try {
    createRuleTester().run("astro-pipeline/mdx-imports-only-approved-components", rule, {
      valid: [
        {
          code: await project.read("src/content/blog/good.mdx"),
          filename: project.path("src/content/blog/good.mdx"),
          options: [baseOptions]
        }
      ],
      invalid: [
        {
          code: await project.read("src/content/blog/bad-name.mdx"),
          filename: project.path("src/content/blog/bad-name.mdx"),
          options: [baseOptions],
          errors: [{ messageId: "unapprovedMdxImportName" }]
        },
        {
          code: await project.read("src/content/blog/bad-module.mdx"),
          filename: project.path("src/content/blog/bad-module.mdx"),
          options: [baseOptions],
          errors: [{ messageId: "unapprovedMdxImport" }]
        },
        {
          code: await project.read("src/content/blog/bad-default.mdx"),
          filename: project.path("src/content/blog/bad-default.mdx"),
          options: [baseOptions],
          errors: [{ messageId: "forbiddenDefaultImport" }]
        },
        {
          code: await project.read("src/content/blog/bad-namespace.mdx"),
          filename: project.path("src/content/blog/bad-namespace.mdx"),
          options: [baseOptions],
          errors: [{ messageId: "forbiddenNamespaceImport" }]
        },
        {
          code: await project.read("src/content/blog/good.mdx"),
          filename: project.path("src/content/blog/good.mdx"),
          options: [{ ...baseOptions, approvedMdxComponentNames: [] }],
          errors: [{ messageId: "missingApprovedNames" }]
        }
      ]
    });
  } finally {
    await project.cleanup();
  }
});
