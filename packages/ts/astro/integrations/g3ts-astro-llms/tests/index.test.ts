import assert from "node:assert/strict";
import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { pathToFileURL } from "node:url";

import g3tsLlms, { generateLlmsTxt } from "../src/index.js";

test("generates deterministic llms.txt with normalized URLs", async () => {
  const tempDir = await fs.mkdtemp(path.join(os.tmpdir(), "g3ts-astro-llms-"));
  const integration = g3tsLlms({
    title: "Example Docs",
    site: "https://example.com/base?ignored=true#ignored",
    sections: [
      {
        heading: "Reference",
        links: [{ title: "API", href: "/api/" }]
      },
      {
        heading: "Docs",
        links: [
          { title: "Tutorial", href: "guides/start?step=1#draft" },
          { title: "Home", href: "/", description: "Start here." }
        ]
      }
    ]
  });

  await integration.hooks["astro:build:done"]?.({
    pages: [],
    dir: pathToFileURL(`${tempDir}/`),
    assets: new Map(),
    logger: {} as never
  });

  assert.equal(
    await fs.readFile(path.join(tempDir, "llms.txt"), "utf8"),
    [
      "# Example Docs",
      "",
      "> https://example.com/",
      "",
      "## Docs",
      "",
      "- [Home](https://example.com/): Start here.",
      "- [Tutorial](https://example.com/guides/start?step=1)",
      "",
      "## Reference",
      "",
      "- [API](https://example.com/api/)",
      ""
    ].join("\n")
  );
});

test("fails when required config is missing", () => {
  assert.throws(
    () =>
      g3tsLlms({
        site: "https://example.com",
        sections: [{ heading: "Docs", links: [{ title: "Home", href: "/" }] }]
      } as never),
    /Invalid g3ts-astro-llms config: title: Invalid input/
  );
});

test("fails when site is not a valid URL", () => {
  assert.throws(
    () =>
      g3tsLlms({
        title: "Example Docs",
        site: "not a url",
        sections: [{ heading: "Docs", links: [{ title: "Home", href: "/" }] }]
      }),
    /site: Invalid URL/
  );
});

test("fails when site is not HTTPS", () => {
  assert.throws(
    () =>
      g3tsLlms({
        title: "Example Docs",
        site: "http://example.com",
        sections: [{ heading: "Docs", links: [{ title: "Home", href: "/" }] }]
      }),
    /site: site must use https/
  );
});

test("fails when text fields contain newlines", () => {
  assert.throws(
    () =>
      g3tsLlms({
        title: "Example Docs",
        site: "https://example.com",
        sections: [
          {
            heading: "Docs\nInjected",
            links: [{ title: "Home", href: "/" }]
          }
        ]
      }),
    /heading: text fields must be single-line/
  );
});

test("escapes generated Markdown link labels", () => {
  assert.equal(
    generateLlmsTxt({
      title: "Example Docs",
      site: "https://example.com/",
      sections: [
        {
          heading: "Docs",
          links: [{ title: "Docs [draft]", href: "/" }]
        }
      ]
    }),
    "# Example Docs\n\n> https://example.com/\n\n## Docs\n\n- [Docs \\[draft\\]](https://example.com/)\n"
  );
});

test("rejects unsafe generated link hrefs", () => {
  const baseConfig = {
    title: "Example Docs",
    site: "https://example.com",
    sections: [
      {
        heading: "Docs",
        links: [{ title: "Home", href: "/" }]
      }
    ]
  };

  assert.throws(
    () =>
      generateLlmsTxt({
        ...baseConfig,
        sections: [{ heading: "Docs", links: [{ title: "JS", href: "javascript:alert(1)" }] }]
      }),
    /href must resolve to HTTPS/
  );
  assert.throws(
    () =>
      generateLlmsTxt({
        ...baseConfig,
        sections: [{ heading: "Docs", links: [{ title: "Offsite", href: "https:\/\/attacker.example/" }] }]
      }),
    /href must stay on the configured site host/
  );
  assert.throws(
    () =>
      generateLlmsTxt({
        ...baseConfig,
        sections: [{ heading: "Docs", links: [{ title: "Injected", href: "/docs/<bad>" }] }]
      }),
    /href must be a single URL token/
  );
});

test("does not emit sitemap, robots, JSON-LD, or header behavior", async () => {
  const tempDir = await fs.mkdtemp(path.join(os.tmpdir(), "g3ts-astro-llms-"));
  const integration = g3tsLlms({
    title: "Example Docs",
    site: "https://example.com",
    sections: [{ heading: "Docs", links: [{ title: "Home", href: "/" }] }]
  });

  assert.deepEqual(Object.keys(integration.hooks), ["astro:build:done"]);

  await integration.hooks["astro:build:done"]?.({
    pages: [{ pathname: "/" }],
    dir: pathToFileURL(`${tempDir}/`),
    assets: new Map([["client", [new URL("file:///asset.js")]]]),
    logger: {} as never
  });

  assert.equal(await exists(path.join(tempDir, "llms.txt")), true);
  assert.equal(await exists(path.join(tempDir, "sitemap.xml")), false);
  assert.equal(await exists(path.join(tempDir, "robots.txt")), false);

  const output = await fs.readFile(path.join(tempDir, "llms.txt"), "utf8");
  assert.equal(output.includes("application/ld+json"), false);
  assert.equal(output.includes('"@context"'), false);
  assert.equal(output.includes("_headers"), false);
  assert.equal(output.includes("Header"), false);
});

test("rejects hidden defaults and unknown config keys", () => {
  assert.throws(() => g3tsLlms({} as never), /title: Invalid input/);
  assert.throws(
    () =>
      g3tsLlms({
        title: "Example Docs",
        site: "https://example.com",
        sections: [{ heading: "Docs", links: [{ title: "Home", href: "/" }] }],
        sitemap: true
      } as never),
    /config: Unrecognized key/
  );
});

test("pure generator matches build hook output", () => {
  assert.equal(
    generateLlmsTxt({
      title: "Example Docs",
      site: "https://example.com/",
      sections: [{ heading: "Docs", links: [{ title: "Home", href: "/" }] }]
    }),
    "# Example Docs\n\n> https://example.com/\n\n## Docs\n\n- [Home](https://example.com/)\n"
  );
});

async function exists(filePath: string): Promise<boolean> {
  try {
    await fs.access(filePath);
    return true;
  } catch {
    return false;
  }
}
