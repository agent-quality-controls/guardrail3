import assert from "node:assert/strict";
import test from "node:test";

import plugin from "../src/index.js";
import rule from "../src/rules/no-runtime-mdx-eval.js";

import { baseOptions, createFixtureProject, createRuleTester } from "./rule-tester.js";

test("recommended config exports all implemented rules at error", () => {
  assert.deepEqual(plugin.configs.recommended.rules, {
    "astro-pipeline/no-authored-content-fs-read": "error",
    "astro-pipeline/no-authored-content-glob": "error",
    "astro-pipeline/no-direct-astro-content-in-routes": "error",
    "astro-pipeline/no-runtime-mdx-eval": "error",
    "astro-pipeline/no-side-loader-imports": "error"
  });
});

test("no-runtime-mdx-eval catches runtime bridges in MDX runtime modules", async () => {
  const project = await createFixtureProject({
    "src/lib/mdx/runtime.tsx": `
      import { run as execute } from "@mdx-js/mdx";
      import { compileAtRuntime } from "./helper";

      export async function render(code: string) {
        return [compileAtRuntime(code), await execute(code)];
      }
    `,
    "src/lib/mdx/helper.tsx": `
      export function compileAtRuntime(code: string) {
        return new Function(code);
      }
    `,
    "src/lib/mdx/safe.tsx": `
      export function render(content: { Content: () => unknown }) {
        return content.Content();
      }
    `,
    "src/lib/mdx/shadowed.tsx": `
      const Function = class LocalFunction {
        constructor(readonly value: string) {}
      };

      export function render(code: string) {
        return new Function(code);
      }
    `,
    "src/lib/mdx/global.tsx": `
      export function render(code: string) {
        return globalThis.Function(code);
      }
    `,
    "src/generated/mdx/runtime.tsx": `
      export function generated(code: string) {
        return new Function(code);
      }
    `
  });

  try {
    const tester = createRuleTester();

    tester.run("astro-pipeline/no-runtime-mdx-eval", rule, {
      valid: [
        {
          code: await project.read("src/lib/mdx/safe.tsx"),
          filename: project.path("src/lib/mdx/safe.tsx"),
          options: [baseOptions]
        },
        {
          code: await project.read("src/lib/mdx/shadowed.tsx"),
          filename: project.path("src/lib/mdx/shadowed.tsx"),
          options: [baseOptions]
        },
        {
          code: await project.read("src/generated/mdx/runtime.tsx"),
          filename: project.path("src/generated/mdx/runtime.tsx"),
          options: [baseOptions]
        }
      ],
      invalid: [
        {
          code: await project.read("src/lib/mdx/runtime.tsx"),
          filename: project.path("src/lib/mdx/runtime.tsx"),
          options: [baseOptions],
          errors: [
            {
              messageId: "runtimeEval"
            },
            {
              messageId: "runtimeEval"
            }
          ]
        },
        {
          code: await project.read("src/lib/mdx/global.tsx"),
          filename: project.path("src/lib/mdx/global.tsx"),
          options: [baseOptions],
          errors: [
            {
              messageId: "runtimeEval"
            }
          ]
        }
      ]
    });
  } finally {
    await project.cleanup();
  }
});
