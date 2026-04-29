import { describe } from "node:test";

import rule from "../src/rules/no-raw-public-image-paths.js";
import { baseOptions, createRuleTester } from "./rule-tester.js";

describe("no-raw-public-image-paths", () => {
  createRuleTester().run("no-raw-public-image-paths", rule, {
    valid: [
      {
        code: `const icon = "/favicon.svg";`,
        options: [baseOptions]
      },
      {
        code: `import hero from "/hero.jpg";`,
        options: [baseOptions]
      },
      {
        code: `const copy = "/blog/hero";`,
        options: [baseOptions]
      },
      {
        code: `const hero = imageMetadata("/hero.jpg");`,
        options: [baseOptions]
      }
    ],
    invalid: [
      {
        code: `const hero = "/hero.jpg";`,
        options: [baseOptions],
        errors: [{ messageId: "rawPublicImagePath" }]
      },
      {
        code: `<img src="/hero.webp" />`,
        options: [baseOptions],
        errors: [{ messageId: "rawPublicImagePath" }]
      },
      {
        code: "const hero = `/hero.png`;",
        options: [baseOptions],
        errors: [{ messageId: "rawPublicImagePath" }]
      },
      {
        code: `const hero = "/hero.jpg";`,
        options: [{}],
        errors: [{ messageId: "missingConfig" }]
      }
    ]
  });
});
