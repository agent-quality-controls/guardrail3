import { describe } from "node:test";

import rule from "../src/rules/no-unlocalized-internal-hrefs.js";
import { astroLanguageOptions, baseOptions, createRuleTester } from "./rule-tester.js";

describe("no-unlocalized-internal-hrefs", () => {
  createRuleTester().run("no-unlocalized-internal-hrefs", rule, {
    valid: [
      {
        code: `<a href="/en/blog/foo">Post</a>`,
        options: [baseOptions]
      },
      {
        code: `<a href="/">Home</a>`,
        options: [baseOptions]
      },
      {
        code: `<a href="https://example.com/blog/foo">External</a>`,
        options: [baseOptions]
      },
      {
        code: `<LocalizedLink href="/blog/foo">Post</LocalizedLink>`,
        options: [baseOptions]
      },
      {
        code: `const href = localizedHref("/blog/foo");`,
        options: [baseOptions]
      },
      {
        code: `const label = t("/blog/foo");`,
        options: [baseOptions]
      },
      {
        code: `console.log("/blog/foo");`,
        options: [baseOptions]
      },
      {
        code: `<a href="/fr/guides/setup">Guide</a>`,
        languageOptions: astroLanguageOptions,
        options: [baseOptions]
      }
    ],
    invalid: [
      {
        code: `<a href="/blog/foo">Post</a>`,
        options: [baseOptions],
        errors: [{ messageId: "unlocalizedHref" }]
      },
      {
        code: `<a to="/guides/setup">Guide</a>`,
        options: [baseOptions],
        errors: [{ messageId: "unlocalizedHref" }]
      },
      {
        code: `const href = buildPath("/blog/foo");`,
        options: [baseOptions],
        errors: [{ messageId: "unlocalizedHref" }]
      },
      {
        code: `<a href="/blog/foo">Post</a>`,
        options: [
          {
            ...baseOptions,
            requireLocalePrefixForContentRoutes: undefined
          }
        ],
        errors: [{ messageId: "missingConfig" }]
      }
    ]
  });
});
