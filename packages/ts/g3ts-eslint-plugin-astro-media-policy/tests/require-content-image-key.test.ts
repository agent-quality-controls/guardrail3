import { describe } from "node:test";

import rule from "../src/rules/require-content-image-key.js";
import { baseOptions, createRuleTester } from "./rule-tester.js";

describe("require-content-image-key", () => {
  createRuleTester().run("require-content-image-key", rule, {
    valid: [
      {
        code: `<ArticleImage image="hero" />`,
        options: [baseOptions]
      },
      {
        code: `<img src="/hero.jpg" />`,
        options: [baseOptions]
      }
    ],
    invalid: [
      {
        code: `<ArticleImage />`,
        options: [baseOptions],
        errors: [{ messageId: "missingImageKey" }]
      },
      {
        code: `<ArticleImage src="/hero.jpg" />`,
        options: [baseOptions],
        errors: [{ messageId: "missingImageKey" }, { messageId: "bannedSourceProp" }]
      },
      {
        code: `<ArticleImage image />`,
        options: [baseOptions],
        errors: [{ messageId: "missingImageKey" }, { messageId: "invalidImageKey" }]
      },
      {
        code: `<ArticleImage image={undefined} />`,
        options: [baseOptions],
        errors: [{ messageId: "missingImageKey" }, { messageId: "invalidImageKey" }]
      },
      {
        code: `<ArticleImage image={true} />`,
        options: [baseOptions],
        errors: [{ messageId: "missingImageKey" }, { messageId: "invalidImageKey" }]
      },
      {
        code: `<ArticleImage image={false} />`,
        options: [baseOptions],
        errors: [{ messageId: "missingImageKey" }, { messageId: "invalidImageKey" }]
      },
      {
        code: `<ArticleImage image={null} />`,
        options: [baseOptions],
        errors: [{ messageId: "missingImageKey" }, { messageId: "invalidImageKey" }]
      },
      {
        code: `<ArticleImage image={0} />`,
        options: [baseOptions],
        errors: [{ messageId: "missingImageKey" }, { messageId: "invalidImageKey" }]
      },
      {
        code: `<ArticleImage image={{}} />`,
        options: [baseOptions],
        errors: [{ messageId: "missingImageKey" }, { messageId: "invalidImageKey" }]
      },
      {
        code: `<ArticleImage image={[]} />`,
        options: [baseOptions],
        errors: [{ messageId: "missingImageKey" }, { messageId: "invalidImageKey" }]
      },
      {
        code: `<ArticleImage image="" />`,
        options: [baseOptions],
        errors: [{ messageId: "missingImageKey" }, { messageId: "invalidImageKey" }]
      },
      {
        code: `<ArticleImage image={"   "} />`,
        options: [baseOptions],
        errors: [{ messageId: "missingImageKey" }, { messageId: "invalidImageKey" }]
      },
      {
        code: "<ArticleImage image={``} />",
        options: [baseOptions],
        errors: [{ messageId: "missingImageKey" }, { messageId: "invalidImageKey" }]
      },
      {
        code: `<ArticleImage image={" " + ""} />`,
        options: [baseOptions],
        errors: [{ messageId: "missingImageKey" }, { messageId: "invalidImageKey" }]
      },
      {
        code: `<ArticleImage image="hero" url="/hero.jpg" />`,
        options: [baseOptions],
        errors: [{ messageId: "bannedSourceProp" }]
      },
      {
        code: `<ArticleImage image="hero" />`,
        options: [{}],
        errors: [{ messageId: "missingConfig" }]
      }
    ]
  });
});
