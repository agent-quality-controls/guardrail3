import { describe } from "node:test";

import rule from "../src/rules/require-approved-media-helper.js";
import { baseOptions, createRuleTester } from "./rule-tester.js";

describe("require-approved-media-helper", () => {
  createRuleTester().run("require-approved-media-helper", rule, {
    valid: [
      {
        code: `import { imageMetadata } from "src/media/images.ts"; const metadata = { image: imageMetadata("hero") };`,
        options: [baseOptions]
      },
      {
        code: `import { imageMetadata as img } from "src/media/images.ts"; const metadata = { image: img("hero") };`,
        options: [baseOptions]
      },
      {
        code: `const metadata = { title: "/hero.jpg" };`,
        options: [baseOptions]
      },
      {
        code: `const metadata = { image: "/blog/post" };`,
        options: [baseOptions]
      },
      {
        code: `const metadata = { ogImage: "/api/og" };`,
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
        code: `const metadata = { ogImage: "/hero.webp?v=1" };`,
        options: [baseOptions],
        errors: [{ messageId: "rawMetadataImage" }]
      },
      {
        code: "const metadata = { ogImage: `/images/${slug}.webp` };",
        options: [baseOptions],
        errors: [{ messageId: "rawMetadataImage" }]
      },
      {
        code: "const metadata = { ogImage: `/images/${slug}.webp#hash` };",
        options: [baseOptions],
        errors: [{ messageId: "rawMetadataImage" }]
      },
      {
        code: `const metadata = { image: ["/hero.jpg"] };`,
        options: [baseOptions],
        errors: [{ messageId: "rawMetadataImage" }]
      },
      {
        code: `const metadata = { image: { url: "/hero.jpg" } };`,
        options: [baseOptions],
        errors: [{ messageId: "rawMetadataImage" }]
      },
      {
        code: `const metadata = { image: "/hero.jpg" };`,
        options: [{}],
        errors: [{ messageId: "missingConfig" }]
      },
      {
        code: `const metadata = { image: "hero" };`,
        options: [baseOptions],
        errors: [{ messageId: "unapprovedMetadataHelper" }]
      },
      {
        code: `const metadata = { image: makeImage("hero") };`,
        options: [baseOptions],
        errors: [{ messageId: "unapprovedMetadataHelper" }]
      },
      {
        code: `const metadata = { image: media.imageMetadata("hero") };`,
        options: [baseOptions],
        errors: [{ messageId: "unapprovedMetadataHelper" }]
      },
      {
        code: `function imageMetadata(value) { return value; } const metadata = { image: imageMetadata("hero") };`,
        options: [baseOptions],
        errors: [{ messageId: "unapprovedMetadataHelper" }]
      },
      {
        code: `import { imageMetadata } from "src/media/images.ts"; { const imageMetadata = (value) => value; const metadata = { image: imageMetadata("hero") }; }`,
        options: [baseOptions],
        errors: [{ messageId: "unapprovedMetadataHelper" }]
      },
      {
        code: `import { imageMetadata } from "src/media/images.ts"; const metadata = { image: imageMetadata("hero") }; function imageMetadata(value) { return value; }`,
        options: [baseOptions],
        errors: [{ messageId: "unapprovedMetadataHelper" }]
      }
    ]
  });
});
