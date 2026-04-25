import assert from "node:assert/strict";
import test from "node:test";

import { ESLint } from "eslint";
import * as astroParser from "astro-eslint-parser";
import tsParser from "@typescript-eslint/parser";

import plugin, { strictContent } from "../src/index.js";

const noLiteralStringRule = strictContent.rules?.[
  "i18next/no-literal-string"
];
const noLiteralStringOptions = Array.isArray(noLiteralStringRule)
  ? noLiteralStringRule[1]
  : undefined;

const protectedCopyAttributes = [
  "alt",
  "aria-label",
  "title",
  "placeholder",
  "value"
] as const;

function ruleIds(result: { messages: Array<{ ruleId: string | null }> }) {
  return result.messages.map((message) => message.ruleId);
}

const tsxLanguageOptions = {
  ecmaVersion: "latest" as const,
  parser: tsParser,
  parserOptions: {
    ecmaFeatures: { jsx: true },
    ecmaVersion: "latest" as const,
    sourceType: "module" as const
  }
};

const astroLanguageOptions = {
  ecmaVersion: "latest" as const,
  parser: astroParser,
  parserOptions: {
    ecmaVersion: "latest" as const,
    extraFileExtensions: [".astro"],
    parser: tsParser,
    sourceType: "module" as const
  }
};

test("strict-content config exposes delegated literal-string enforcement", () => {
  assert.equal(
    strictContent.rules?.["i18next/no-literal-string"]?.[0],
    "error"
  );
  assert.deepEqual(Object.keys(strictContent.plugins ?? {}), ["i18next"]);
  assert.equal(
    plugin.configs["strict-content"].rules?.["i18next/no-literal-string"]?.[0],
    "error"
  );
  assert.equal(noLiteralStringOptions?.framework, "react");
  assert.equal(noLiteralStringOptions?.mode, "all");
  assert.equal(noLiteralStringOptions?.["should-validate-template"], true);
  assert.match(noLiteralStringOptions?.message ?? "", /Astro content/);
  for (const attr of protectedCopyAttributes) {
    assert.equal(
      noLiteralStringOptions?.["jsx-attributes"]?.exclude.includes(attr),
      false,
      `${attr} must stay checked as public copy`
    );
  }
});

test("strict-content config catches TSX inline public copy and allows structural strings", async () => {
  const eslint = new ESLint({
    overrideConfigFile: true,
    overrideConfig: [
      {
        files: ["**/*.tsx"],
        languageOptions: tsxLanguageOptions
      },
      strictContent
    ]
  });

  const [invalidResult] = await eslint.lintText(
    `
      import type { HomepageV2Content } from "@/content/schemas";
      type Hero = HomepageV2Content["hero"];
      const HERO = "INTERNAL_TOKEN";
      const hero = { title: "Request an audit" };
      export function Page() {
        return (
          <section className="font-mono" data-state="open">
            <h1>Request an audit</h1>
            <img alt="SEO report screenshot" src="/brand/report.webp" />
          </section>
        );
      }
    `,
    { filePath: "src/ui/page.tsx" }
  );

  assert.deepEqual(ruleIds(invalidResult), [
    "i18next/no-literal-string",
    "i18next/no-literal-string",
    "i18next/no-literal-string"
  ]);
  assert.match(invalidResult.messages[0]?.message ?? "", /Request an audit/);
  assert.match(invalidResult.messages[2]?.message ?? "", /SEO report screenshot/);

  const [validResult] = await eslint.lintText(
    `
      import type { HomepageV2Content } from "@/content/schemas";
      type Hero = HomepageV2Content["hero"];
      const HERO = "INTERNAL_TOKEN";
      export function Page({ hero }: { hero: Hero }) {
        return (
          <section className="font-mono" data-state="open">
            <h1>{hero.title}</h1>
            <img alt="" src="/brand/report.webp" aria-hidden="true" />
          </section>
        );
      }
    `,
    { filePath: "src/ui/page.tsx" }
  );

  assert.deepEqual(validResult.messages, []);
});

test("strict-content config catches TS source object and array copy", async () => {
  const eslint = new ESLint({
    overrideConfigFile: true,
    overrideConfig: [
      {
        files: ["**/*.ts"],
        languageOptions: {
          ecmaVersion: "latest" as const,
          parser: tsParser,
          parserOptions: {
            ecmaVersion: "latest" as const,
            sourceType: "module" as const
          }
        }
      },
      strictContent
    ]
  });

  const [invalidResult] = await eslint.lintText(
    `
      export const blocks = [
        { title: "Request an audit" },
        { name: "Request an audit" },
        "See the complete analysis"
      ];
    `,
    { filePath: "src/content/homepage.data.ts" }
  );

  assert.deepEqual(ruleIds(invalidResult), [
    "i18next/no-literal-string",
    "i18next/no-literal-string",
    "i18next/no-literal-string"
  ]);
});

test("strict-content config catches i18n call literal bypasses", async () => {
  const eslint = new ESLint({
    overrideConfigFile: true,
    overrideConfig: [
      {
        files: ["**/*.ts"],
        languageOptions: {
          ecmaVersion: "latest" as const,
          parser: tsParser,
          parserOptions: {
            ecmaVersion: "latest" as const,
            sourceType: "module" as const
          }
        }
      },
      strictContent
    ]
  });

  const [invalidResult] = await eslint.lintText(
    `
      export const title = t("Request an audit");
      export const label = i18n("See the complete analysis");
      export const schema = z.enum(["Request an audit"]);
      export const sent = postMessage("Request an audit");
      export const checked = copy.includes("Request an audit");
    `,
    { filePath: "src/content/homepage.data.ts" }
  );

  assert.deepEqual(ruleIds(invalidResult), [
    "i18next/no-literal-string",
    "i18next/no-literal-string",
    "i18next/no-literal-string",
    "i18next/no-literal-string",
    "i18next/no-literal-string"
  ]);
});

test("strict-content config allows structural source strings", async () => {
  const eslint = new ESLint({
    overrideConfigFile: true,
    overrideConfig: [
      {
        files: ["**/*.astro"],
        languageOptions: astroLanguageOptions
      },
      {
        files: ["**/*.tsx"],
        languageOptions: tsxLanguageOptions
      },
      strictContent
    ]
  });

  const [tsxResult] = await eslint.lintText(
    `
      import { clsx } from "clsx";

      const heroAsset = new URL("../assets/hero-card.png", import.meta.url);
      export function Card() {
        return <CardShell tone="warm" intent="primary" className={clsx("grid gap-4")} src={heroAsset.href} />;
      }
    `,
    { filePath: "src/ui/card.tsx" }
  );
  const [astroResult] = await eslint.lintText(
    `
      ---
      const { content } = Astro.props;
      ---
      <Fragment slot="head" />
      <h1>{content.title}</h1>
    `,
    { filePath: "src/pages/index.astro" }
  );

  assert.deepEqual(tsxResult.messages, []);
  assert.deepEqual(astroResult.messages, []);
});

test("strict-content config catches public-copy attributes", async () => {
  const eslint = new ESLint({
    overrideConfigFile: true,
    overrideConfig: [
      {
        files: ["**/*.tsx"],
        languageOptions: tsxLanguageOptions
      },
      strictContent
    ]
  });

  const [invalidResult] = await eslint.lintText(
    `
      export function Form() {
        return (
          <form aria-label="Contact the audit team">
            <img alt="SEO report screenshot" src="/brand/report.webp" />
            <input title="Work email" placeholder="name@example.com" value="Start audit" readOnly />
          </form>
        );
      }
    `,
    { filePath: "src/ui/form.tsx" }
  );

  assert.deepEqual(ruleIds(invalidResult), [
    "i18next/no-literal-string",
    "i18next/no-literal-string",
    "i18next/no-literal-string",
    "i18next/no-literal-string",
    "i18next/no-literal-string"
  ]);
});

test("strict-content config catches Astro template and frontmatter copy", async () => {
  const eslint = new ESLint({
    overrideConfigFile: true,
    overrideConfig: [
      {
        files: ["**/*.astro"],
        languageOptions: astroLanguageOptions
      },
      strictContent
    ]
  });

  const [invalidResult] = await eslint.lintText(
    `
      ---
      const title = "Request an audit";
      ---
      <h1>Request an audit</h1>
      <img alt="SEO report screenshot" src="/brand/report.webp" />
    `,
    { filePath: "src/pages/index.astro" }
  );

  assert.deepEqual(ruleIds(invalidResult), [
    "i18next/no-literal-string",
    "i18next/no-literal-string",
    "i18next/no-literal-string"
  ]);

  const [validResult] = await eslint.lintText(
    `
      ---
      const { title, image } = Astro.props;
      ---
      <h1>{title}</h1>
      <img alt="" src={image.src} aria-hidden="true" />
    `,
    { filePath: "src/pages/index.astro" }
  );

  assert.deepEqual(validResult.messages, []);
});
