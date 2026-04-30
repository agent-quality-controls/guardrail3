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
        code: `const copy = "/blog/hero?ref=image";`,
        options: [baseOptions]
      },
      {
        code: `import { imageMetadata } from "src/media/images.ts"; const hero = imageMetadata("/hero.jpg");`,
        options: [baseOptions]
      },
      {
        code: `import { imageMetadata as img } from "src/media/images.ts"; const hero = img("/hero.jpg");`,
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
      },
      {
        code: `const hero = tracking.imageMetadata("/hero.jpg");`,
        options: [baseOptions],
        errors: [{ messageId: "rawPublicImagePath" }]
      },
      {
        code: `const hero = "/hero.jpg?v=1";`,
        options: [baseOptions],
        errors: [{ messageId: "rawPublicImagePath" }]
      },
      {
        code: "const hero = `/images/${slug}.jpg`;",
        options: [baseOptions],
        errors: [{ messageId: "rawPublicImagePath" }]
      },
      {
        code: "const hero = `/images/${slug}.jpg?v=1`;",
        options: [baseOptions],
        errors: [{ messageId: "rawPublicImagePath" }]
      },
      {
        code: "const hero = `/images/${slug}.jpg?w=${width}`;",
        options: [baseOptions],
        errors: [{ messageId: "rawPublicImagePath" }]
      },
      {
        code: `const hero = makeImage("/hero.jpg");`,
        options: [baseOptions],
        errors: [{ messageId: "rawPublicImagePath" }]
      },
      {
        code: `function imageMetadata(value) { return value; } const hero = imageMetadata("/hero.jpg");`,
        options: [baseOptions],
        errors: [{ messageId: "rawPublicImagePath" }]
      },
      {
        code: `import { imageMetadata } from "src/media/images.ts"; { const imageMetadata = (value) => value; const hero = imageMetadata("/hero.jpg"); }`,
        options: [baseOptions],
        errors: [{ messageId: "rawPublicImagePath" }]
      },
      {
        code: `import { imageMetadata } from "src/media/images.ts"; const hero = imageMetadata("/hero.jpg"); function imageMetadata(value) { return value; }`,
        options: [baseOptions],
        errors: [{ messageId: "rawPublicImagePath" }]
      }
    ]
  });
});
