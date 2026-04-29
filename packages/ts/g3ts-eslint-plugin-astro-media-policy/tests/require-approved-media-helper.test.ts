import { describe } from "node:test";

import rule from "../src/rules/require-approved-media-helper.js";
import { baseOptions, createRuleTester } from "./rule-tester.js";

describe("require-approved-media-helper", () => {
  createRuleTester().run("require-approved-media-helper", rule, {
    valid: [
      {
        code: `const metadata = { image: imageMetadata("hero") };`,
        options: [baseOptions]
      },
      {
        code: `const metadata = { title: "/hero.jpg" };`,
        options: [baseOptions]
      }
    ],
    invalid: [
      {
        code: `const metadata = { image: "/hero.jpg" };`,
        options: [baseOptions],
        errors: [{ messageId: "rawMetadataImage" }]
      },
      {
        code: `const metadata = { ogImage: "/hero.webp" };`,
        options: [baseOptions],
        errors: [{ messageId: "rawMetadataImage" }]
      },
      {
        code: `const metadata = { image: "/hero.jpg" };`,
        options: [{}],
        errors: [{ messageId: "missingConfig" }]
      }
    ]
  });
});
