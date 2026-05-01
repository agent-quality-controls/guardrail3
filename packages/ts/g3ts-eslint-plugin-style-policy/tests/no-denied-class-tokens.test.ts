import { describe } from "node:test";
import type { Rule } from "eslint";

import rule from "../src/rules/no-denied-class-tokens.js";
import { astroLanguageOptions, baseOptions, createRuleTester } from "./rule-tester.js";

describe("no-denied-class-tokens", () => {
  createRuleTester().run("no-denied-class-tokens", rule as unknown as Rule.RuleModule, {
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
      },
      {
        code: `<div className={cx("text-black")} />`,
        options: [baseOptions]
      },
      {
        code: `<div className="text-brand" />`,
        options: [
          {
            denyPrefixes: ["text-["],
            classAttributes: ["className"]
          }
        ]
      }
    ],
    invalid: [
      {
        code: `<div className="text-black" />`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } }]
      },
      {
        code: `<div class="text-black" />`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } }]
      },
      {
        code: "---\n---\n<div class=\"text-black\" />",
        filename: "src/pages/index.astro",
        languageOptions: astroLanguageOptions,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } }]
      },
      {
        code: "---\n---\n<div class:list={[\"text-black\", active && \"text-brand\"]} />",
        filename: "src/pages/index.astro",
        languageOptions: astroLanguageOptions,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } }]
      },
      {
        code: "---\n---\n<div class:list={{ \"text-black\": active }} />",
        filename: "src/pages/index.astro",
        languageOptions: astroLanguageOptions,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } }]
      },
      {
        code: "---\n---\n<div class:list={active ? \"text-black\" : \"text-brand\"} />",
        filename: "src/pages/index.astro",
        languageOptions: astroLanguageOptions,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } }]
      },
      {
        code: "---\n---\n<div class:list={`text-black ${tone}`} />",
        filename: "src/pages/index.astro",
        languageOptions: astroLanguageOptions,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } }]
      },
      {
        code: `<div className={"text-black"} />`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } }]
      },
      {
        code: "<div className={`text-black`} />",
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } }]
      },
      {
        code: "<div className={`text-black ${tone}`} />",
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } }]
      },
      {
        code: `<div className={"text-black " + tone} />`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } }]
      },
      {
        code: `<div className={tone + " bg-red-500"} />`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken", data: { token: "bg-red-500", policy: "denyList:bg-red-500" } }]
      },
      {
        code: `<div className={active ? "text-black" : "text-brand"} />`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } }]
      },
      {
        code: `<div className={active && "text-black"} />`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } }]
      },
      {
        code: `<div className={["text-black", active && "text-brand"]} />`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } }]
      },
      {
        code: `<div className={{ "text-black": active, "text-brand": true }} />`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } }]
      },
      {
        code: "<div className={{ [`text-black ${tone}`]: active }} />",
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } }]
      },
      {
        code: `<div className={{ ["text-black " + tone]: active }} />`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } }]
      },
      {
        code: `const className = cn("text-black", active && "text-brand");`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } }]
      },
      {
        code: `<div className={cn("text-black", active && "text-brand")} />`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } }]
      },
      {
        code: `const className = clsx(["text-black"]);`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } }]
      },
      {
        code: `const className = twMerge({ "text-black": active });`,
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } }]
      },
      {
        code: "const className = cn({ [`text-black ${tone}`]: active });",
        options: [baseOptions],
        errors: [{ messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } }]
      },
      {
        code: `const className = cn(active ? "text-black" : "text-brand", ["bg-red-500"]);`,
        options: [baseOptions],
        errors: [
          { messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } },
          { messageId: "deniedClassToken", data: { token: "bg-red-500", policy: "denyList:bg-red-500" } }
        ]
      },
      {
        code: `<div className="text-black bg-red-500" />`,
        options: [baseOptions],
        errors: [
          { messageId: "deniedClassToken", data: { token: "text-black", policy: "denyList:text-black" } },
          { messageId: "deniedClassToken", data: { token: "bg-red-500", policy: "denyList:bg-red-500" } }
        ]
      },
      {
        code: `<div className="text-black" />`,
        options: [{}],
        errors: [{ messageId: "missingConfig" }]
      },
      {
        code: `<div className="text-[13px]" />`,
        options: [baseOptions],
        errors: [
          {
            messageId: "deniedClassToken",
            data: { token: "text-[13px]", policy: "denyPrefixes:text-[" }
          }
        ]
      },
      {
        code: `<div className={cn("leading-[1.13]", active && "text-brand")} />`,
        options: [baseOptions],
        errors: [
          {
            messageId: "deniedClassToken",
            data: { token: "leading-[1.13]", policy: "denyPrefixes:leading-[" }
          }
        ]
      },
      {
        code: "<div className={`tracking-[-0.02em] ${tone}`} />",
        options: [baseOptions],
        errors: [
          {
            messageId: "deniedClassToken",
            data: { token: "tracking-[-0.02em]", policy: "denyPrefixes:tracking-[" }
          }
        ]
      },
      {
        code: `<div className="font-[Inter]" />`,
        options: [baseOptions],
        errors: [
          {
            messageId: "deniedClassToken",
            data: { token: "font-[Inter]", policy: "denyPatterns:^font-\\[[^\\]]+\\]$" }
          }
        ]
      },
      {
        code: `<div className="text-[13px]" />`,
        options: [
          {
            denyPrefixes: ["text-["],
            classAttributes: ["className"]
          }
        ],
        errors: [
          {
            messageId: "deniedClassToken",
            data: { token: "text-[13px]", policy: "denyPrefixes:text-[" }
          }
        ]
      },
      {
        code: `<div className="font-[Inter]" />`,
        options: [
          {
            denyPatterns: ["^font-\\["],
            classAttributes: ["className"]
          }
        ],
        errors: [
          {
            messageId: "deniedClassToken",
            data: { token: "font-[Inter]", policy: "denyPatterns:^font-\\[" }
          }
        ]
      },
      {
        code: `<div className="text-brand" />`,
        options: [
          {
            denyPatterns: ["["],
            classAttributes: ["className"]
          }
        ],
        errors: [{ messageId: "invalidPattern" }]
      }
    ]
  });
});
