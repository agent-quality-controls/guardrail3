import { describe } from "node:test";

import rule from "../src/rules/no-inline-image-alt.js";
import { astroLanguageOptions, baseOptions, createRuleTester } from "./rule-tester.js";

describe("no-inline-image-alt", () => {
  createRuleTester().run("no-inline-image-alt", rule, {
    valid: [
      {
        code: `<ArticleImage image="hero" />`,
        options: [baseOptions]
      },
      {
        code: `<img alt="Plain UI image" />`,
        options: [baseOptions]
      },
      {
        code: `<ArticleImage image="hero" alt={image.alt} />`,
        options: [baseOptions]
      }
    ],
    invalid: [
      {
        code: `<ArticleImage image="hero" alt="English text" />`,
        options: [baseOptions],
        errors: [{ messageId: "inlineAlt" }]
      },
      {
        code: `<ContentImage image="hero" alt={"English text"} />`,
        options: [baseOptions],
        errors: [{ messageId: "inlineAlt" }]
      },
      {
        code: `<ContentImage image="hero" alt={"English " + "text"} />`,
        options: [baseOptions],
        errors: [{ messageId: "inlineAlt" }]
      },
      {
        code: "<ContentImage image=\"hero\" alt={`English ${word}`} />",
        options: [baseOptions],
        errors: [{ messageId: "inlineAlt" }]
      },
      {
        code: `<ContentImage image="hero" alt={"English " + word} />`,
        options: [baseOptions],
        errors: [{ messageId: "inlineAlt" }]
      },
      {
        code: `<ArticleImage image="hero" alt="" />`,
        options: [baseOptions],
        errors: [{ messageId: "inlineAlt" }]
      },
      {
        code: "---\n---\n<ArticleImage image=\"hero\" alt=\"English text\" />",
        filename: "src/pages/index.astro",
        languageOptions: astroLanguageOptions,
        options: [baseOptions],
        errors: [{ messageId: "inlineAlt" }]
      },
      {
        code: `<ArticleImage image="hero" alt="   " />`,
        options: [baseOptions],
        errors: [{ messageId: "inlineAlt" }]
      },
      {
        code: `<ArticleImage image="hero" alt="English text" />`,
        options: [{}],
        errors: [{ messageId: "missingConfig" }]
      }
    ]
  });
});
