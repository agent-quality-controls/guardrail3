import { describe } from "node:test";

import rule from "../src/rules/no-denied-class-tokens.js";
import { astroLanguageOptions, baseOptions, createRuleTester } from "./rule-tester.js";

describe("no-denied-class-tokens", () => {
  createRuleTester().run("no-denied-class-tokens", rule, {
    valid: [
      {
        code: `<div className="text-brand bg-surface" />`,
        options: [baseOptions]
      },
      {
        code: `<div className={dynamicClass} />`,
        options: [baseOptions]
      },
      {
        code: `const value = "text-black";`,
        options: [baseOptions]
      },
      {
        code: `<div data-class="text-black" />`,
        options: [baseOptions]
      },
      {
        code: `const className = cx("text-black");`,
        options: [baseOptions]
      }
    ],
    invalid: [
      {
        code: `<div className="text-black" />`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken" }]
      },
      {
        code: `<div class="text-black" />`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken" }]
      },
      {
        code: "---\n---\n<div class=\"text-black\" />",
        filename: "src/pages/index.astro",
        languageOptions: astroLanguageOptions,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken" }]
      },
      {
        code: "---\n---\n<div class:list={[\"text-black\", active && \"text-brand\"]} />",
        filename: "src/pages/index.astro",
        languageOptions: astroLanguageOptions,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken" }]
      },
      {
        code: `<div className={"text-black"} />`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken" }]
      },
      {
        code: "<div className={`text-black`} />",
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken" }]
      },
      {
        code: `<div className={active ? "text-black" : "text-brand"} />`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken" }]
      },
      {
        code: `<div className={active && "text-black"} />`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken" }]
      },
      {
        code: `<div className={["text-black", active && "text-brand"]} />`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken" }]
      },
      {
        code: `<div className={{ "text-black": active, "text-brand": true }} />`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken" }]
      },
      {
        code: `const className = cn("text-black", active && "text-brand");`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken" }]
      },
      {
        code: `<div className={cn("text-black", active && "text-brand")} />`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken" }]
      },
      {
        code: `const className = clsx(["text-black"]);`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken" }]
      },
      {
        code: `const className = twMerge({ "text-black": active });`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken" }]
      },
      {
        code: `<div className="text-black bg-red-500" />`,
        options: [baseOptions],
        errors: [
          { messageId: "deniedClassToken" },
          { messageId: "deniedClassToken" }
        ]
      },
      {
        code: `<div className="text-black" />`,
        options: [{}],
        errors: [{ messageId: "missingConfig" }]
      }
    ]
  });
});
