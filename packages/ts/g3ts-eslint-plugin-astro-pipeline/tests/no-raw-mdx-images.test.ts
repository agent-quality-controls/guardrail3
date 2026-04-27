import test from "node:test";

import rule from "../src/rules/no-raw-mdx-images.js";

import { baseOptions, createFixtureProject, createRuleTester } from "./rule-tester.js";

test("no-raw-mdx-images forbids markdown and raw html images", async () => {
  const project = await createFixtureProject({
    "src/content/blog/good.tsx": `
      import { ArticleImage } from "@/components/mdx";
      <ArticleImage src="/image.webp" alt="Good" />
    `,
    "src/content/blog/bad-jsx.tsx": `
      <img src="/image.webp" alt="Bad" />
    `,
    "src/content/blog/bad-markdown.tsx": `
      const markdownImage = "![Bad](/image.webp)";
    `
  });
  const options = [{ ...baseOptions, mdxContentGlobs: ["src/content/**/*.tsx"] }];

  try {
    createRuleTester().run("astro-pipeline/no-raw-mdx-images", rule, {
      valid: [
        {
          code: await project.read("src/content/blog/good.tsx"),
          filename: project.path("src/content/blog/good.tsx"),
          options
        }
      ],
      invalid: [
        {
          code: await project.read("src/content/blog/bad-jsx.tsx"),
          filename: project.path("src/content/blog/bad-jsx.tsx"),
          options,
          errors: [{ messageId: "rawImage" }]
        },
        {
          code: await project.read("src/content/blog/bad-markdown.tsx"),
          filename: project.path("src/content/blog/bad-markdown.tsx"),
          options,
          errors: [{ messageId: "rawImage" }]
        },
        {
          code: await project.read("src/content/blog/good.tsx"),
          filename: project.path("src/content/blog/good.tsx"),
          options: [{ ...baseOptions, mdxContentGlobs: ["src/content/**/*.tsx"], approvedMdxImageComponents: [] }],
          errors: [{ messageId: "missingApprovedImages" }]
        }
      ]
    });
  } finally {
    await project.cleanup();
  }
});
