import test from "node:test";

import jsonLdRule from "../src/rules/require-approved-json-ld-helper-in-routes.js";
import metadataRule from "../src/rules/require-approved-metadata-helper-in-routes.js";

import {
  astroLanguageOptions,
  baseOptions,
  createFixtureProject,
  createRuleTester
} from "./rule-tester.js";

test("route helper rules require approved metadata and JSON-LD surfaces", async () => {
  const project = await createFixtureProject({
    "src/pages/index.astro": `
      ---
      import { getLandingPage } from "../lib/content/landing-homepage";
      import { pageJsonLd } from "../lib/json-ld/page";
      export const page = await getLandingPage();
      export const jsonLd = pageJsonLd(page);
      ---
      <script type="application/ld+json">{JSON.stringify(jsonLd)}</script>
      <h1>{page.title}</h1>
    `,
    "src/pages/about.tsx": `
      import { pageMetadata } from "../lib/metadata/page";
      import { pageJsonLd } from "../lib/json-ld/page";

      export function AboutPage() {
        const metadata = pageMetadata();
        return (
          <>
            <script type="application/ld+json">{JSON.stringify(pageJsonLd())}</script>
            <h1>{metadata.title}</h1>
          </>
        );
      }
    `,
    "src/pages/unused.tsx": `
      import { pageMetadata } from "../lib/metadata/page";
      import { pageJsonLd } from "../lib/json-ld/page";

      export const metadata = { title: "Hardcoded" };

      export function UnusedPage() {
        return <h1>Hardcoded</h1>;
      }
    `,
    "src/pages/void-used.tsx": `
      import { pageMetadata } from "../lib/metadata/page";
      import { pageJsonLd } from "../lib/json-ld/page";

      void pageMetadata;
      void pageJsonLd;

      export const metadata = { title: "Hardcoded" };

      export function VoidUsedPage() {
        return <h1>Hardcoded</h1>;
      }
    `,
    "src/pages/discarded-call.tsx": `
      import { pageMetadata } from "../lib/metadata/page";
      import { pageJsonLd } from "../lib/json-ld/page";

      pageMetadata();
      pageJsonLd();

      export const metadata = { title: "Hardcoded" };

      export function DiscardedCallPage() {
        return <h1>Hardcoded</h1>;
      }
    `,
    "src/pages/assigned-unused.tsx": `
      import { pageMetadata } from "../lib/metadata/page";
      import { pageJsonLd } from "../lib/json-ld/page";

      const ignoredMetadata = pageMetadata();
      const ignoredJsonLd = pageJsonLd();

      export const metadata = { title: "Hardcoded" };

      export function AssignedUnusedPage() {
        return <h1>Hardcoded</h1>;
      }
    `,
    "src/pages/jsx-noop.tsx": `
      import { pageMetadata } from "../lib/metadata/page";
      import { pageJsonLd } from "../lib/json-ld/page";

      export const metadata = { title: "Hardcoded" };

      export function JsxNoopPage() {
        return (
          <>
            {pageMetadata && null}
            {pageJsonLd && null}
            <h1>Hardcoded</h1>
          </>
        );
      }
    `,
    "src/pages/noop-property.tsx": `
      import { pageMetadata } from "../lib/metadata/page";
      import { pageJsonLd } from "../lib/json-ld/page";

      export const metadata = { title: "Hardcoded", noop: pageMetadata() };

      export function NoopPropertyPage() {
        return (
          <>
            <script type="application/ld+json">
              {JSON.stringify({ "@type": "WebPage", noop: pageJsonLd() })}
            </script>
            <h1>Hardcoded</h1>
          </>
        );
      }
    `,
    "src/pages/exported-unused-function.tsx": `
      import { pageMetadata } from "../lib/metadata/page";
      import { pageJsonLd } from "../lib/json-ld/page";

      export function unusedMetadata() {
        return pageMetadata();
      }

      export const unusedJsonLd = () => pageJsonLd();

      export const metadata = { title: "Hardcoded" };

      export function ExportedUnusedFunctionPage() {
        return <h1>Hardcoded</h1>;
      }
    `,
    "src/pages/type-only.tsx": `
      import type { PageMetadata } from "../lib/metadata/page";
      import type { PageJsonLd } from "../lib/json-ld/page";

      export const metadata: PageMetadata = { title: "Hardcoded" };
      export const jsonLd: PageJsonLd = { "@type": "WebPage" };
    `,
    "src/pages/bad.tsx": `
      export const title = "Hardcoded";

      export function BadPage() {
        return <h1>{title}</h1>;
      }
    `,
    "src/pages/feed.endpoint.ts": `
      export async function GET() {
        return new Response("ok");
      }
    `
  });

  try {
    const tester = createRuleTester();

    tester.run("astro-pipeline/require-approved-metadata-helper-in-routes", metadataRule, {
      valid: [
        {
          code: await project.read("src/pages/index.astro"),
          filename: project.path("src/pages/index.astro"),
          languageOptions: astroLanguageOptions,
          options: [baseOptions]
        },
        {
          code: await project.read("src/pages/about.tsx"),
          filename: project.path("src/pages/about.tsx"),
          options: [baseOptions]
        },
        {
          code: await project.read("src/pages/feed.endpoint.ts"),
          filename: project.path("src/pages/feed.endpoint.ts"),
          options: [baseOptions]
        }
      ],
      invalid: [
        {
          code: await project.read("src/pages/bad.tsx"),
          filename: project.path("src/pages/bad.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "missingMetadataHelper" }]
        },
        {
          code: await project.read("src/pages/unused.tsx"),
          filename: project.path("src/pages/unused.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "missingMetadataHelper" }]
        },
        {
          code: await project.read("src/pages/void-used.tsx"),
          filename: project.path("src/pages/void-used.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "missingMetadataHelper" }]
        },
        {
          code: await project.read("src/pages/discarded-call.tsx"),
          filename: project.path("src/pages/discarded-call.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "missingMetadataHelper" }]
        },
        {
          code: await project.read("src/pages/assigned-unused.tsx"),
          filename: project.path("src/pages/assigned-unused.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "missingMetadataHelper" }]
        },
        {
          code: await project.read("src/pages/jsx-noop.tsx"),
          filename: project.path("src/pages/jsx-noop.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "missingMetadataHelper" }]
        },
        {
          code: await project.read("src/pages/noop-property.tsx"),
          filename: project.path("src/pages/noop-property.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "missingMetadataHelper" }]
        },
        {
          code: await project.read("src/pages/exported-unused-function.tsx"),
          filename: project.path("src/pages/exported-unused-function.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "missingMetadataHelper" }]
        },
        {
          code: await project.read("src/pages/type-only.tsx"),
          filename: project.path("src/pages/type-only.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "missingMetadataHelper" }]
        }
      ]
    });

    tester.run("astro-pipeline/require-approved-json-ld-helper-in-routes", jsonLdRule, {
      valid: [
        {
          code: await project.read("src/pages/index.astro"),
          filename: project.path("src/pages/index.astro"),
          languageOptions: astroLanguageOptions,
          options: [baseOptions]
        },
        {
          code: await project.read("src/pages/about.tsx"),
          filename: project.path("src/pages/about.tsx"),
          options: [baseOptions]
        },
        {
          code: await project.read("src/pages/feed.endpoint.ts"),
          filename: project.path("src/pages/feed.endpoint.ts"),
          options: [baseOptions]
        }
      ],
      invalid: [
        {
          code: await project.read("src/pages/bad.tsx"),
          filename: project.path("src/pages/bad.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "missingJsonLdHelper" }]
        },
        {
          code: await project.read("src/pages/unused.tsx"),
          filename: project.path("src/pages/unused.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "missingJsonLdHelper" }]
        },
        {
          code: await project.read("src/pages/void-used.tsx"),
          filename: project.path("src/pages/void-used.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "missingJsonLdHelper" }]
        },
        {
          code: await project.read("src/pages/discarded-call.tsx"),
          filename: project.path("src/pages/discarded-call.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "missingJsonLdHelper" }]
        },
        {
          code: await project.read("src/pages/assigned-unused.tsx"),
          filename: project.path("src/pages/assigned-unused.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "missingJsonLdHelper" }]
        },
        {
          code: await project.read("src/pages/jsx-noop.tsx"),
          filename: project.path("src/pages/jsx-noop.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "missingJsonLdHelper" }]
        },
        {
          code: await project.read("src/pages/noop-property.tsx"),
          filename: project.path("src/pages/noop-property.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "missingJsonLdHelper" }]
        },
        {
          code: await project.read("src/pages/exported-unused-function.tsx"),
          filename: project.path("src/pages/exported-unused-function.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "missingJsonLdHelper" }]
        },
        {
          code: await project.read("src/pages/type-only.tsx"),
          filename: project.path("src/pages/type-only.tsx"),
          options: [baseOptions],
          errors: [{ messageId: "missingJsonLdHelper" }]
        }
      ]
    });
  } finally {
    await project.cleanup();
  }
});
