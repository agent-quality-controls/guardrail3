import assert from "node:assert/strict";
import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { pathToFileURL } from "node:url";

import g3tsLlmsAuditor, { checkLlmsTxt } from "../src/index.js";

const packageRoot = path.resolve(import.meta.dirname, "..");
const baseConfig = {
  site: "https://example.com",
  requiredSections: [],
  requiredRoutePatterns: [],
  allowedExternalUrls: [],
  allowedNonPageUrls: [],
  ignoredHtmlFiles: []
};

test("fails when the configured llms.txt file is missing", async () => {
  const result = await checkLlmsTxt({
    ...baseConfig,
    outputDir: path.join(os.tmpdir(), "g3ts-missing-llms"),
  });

  assert.deepEqual(result, {
    ok: false,
    findings: [
      {
        code: "llms-file-missing",
        message: `Configured llms.txt file does not exist: ${path.join(
          os.tmpdir(),
          "g3ts-missing-llms",
          "llms.txt"
        )}`,
        path: path.join(os.tmpdir(), "g3ts-missing-llms", "llms.txt")
      }
    ]
  });
});

test("fails when parse-llms-txt cannot produce a valid llms.txt title", async () => {
  await withFixture("## Docs\n- [Docs](https://example.com/docs/)\n", async (llmsPath) => {
    const result = await checkLlmsTxt({
      ...baseConfig,
      outputDir: path.dirname(llmsPath),
    });

    assert.equal(result.ok, false);
    assert.deepEqual(result.findings.map((finding) => finding.code), [
      "llms-parse-failed"
    ]);
  });
});

test("fails when malformed Markdown link text is parsed permissively", async () => {
  await withFixture(
    ["# Example", "", "## Docs", "", "- [Docs](https://example.com/docs/"].join("\n"),
    async (llmsPath) => {
      const result = await checkLlmsTxt({
        ...baseConfig,
        outputDir: path.dirname(llmsPath),
      });

      assert.equal(result.ok, false);
      assert.deepEqual(result.findings.map((finding) => finding.code), [
        "llms-markdown-structure-invalid",
        "llms-markdown-structure-invalid"
      ]);
    }
  );
});

test("fails when a configured required section is missing", async () => {
  await withFixture(validLlmsTxt(), async (llmsPath) => {
    const result = await checkLlmsTxt({
      ...baseConfig,
      outputDir: path.dirname(llmsPath),
      requiredSections: ["Policies"],
      allowedNonPageUrls: ["https://example.com/docs/"]
    });

    assert.deepEqual(result, {
      ok: false,
      findings: [
        {
          code: "llms-required-section-missing",
          message: "Configured required llms.txt section is missing: Policies",
          path: llmsPath,
          expected: "Policies"
        }
      ]
    });
  });
});

test("fails when a configured required route pattern is missing", async () => {
  await withFixture(validLlmsTxt(), async (llmsPath) => {
    const result = await checkLlmsTxt({
      ...baseConfig,
      outputDir: path.dirname(llmsPath),
      requiredRoutePatterns: ["/policies/**"],
      allowedNonPageUrls: ["https://example.com/docs/"]
    });

    assert.deepEqual(result, {
      ok: false,
      findings: [
        {
          code: "llms-required-route-pattern-missing",
          message:
            "Configured required llms.txt route pattern is missing: /policies/**",
          path: llmsPath,
          expected: "/policies/**"
        }
      ]
    });
  });
});

test("passes when the file exists, parses, configured sections exist, and links map to built pages", async () => {
  await withOutputFixture(validLlmsTxt(), async (outputDir) => {
    const result = await checkLlmsTxt({
      ...baseConfig,
      outputDir,
      requiredSections: ["Docs"],
      requiredRoutePatterns: ["/docs/"]
    });

    assert.deepEqual(result, {
      ok: true,
      findings: []
    });
  });
});

test("Astro integration exposes the same explicit post-build checks", async () => {
  await withOutputFixture(validLlmsTxt(), async (outputDir) => {
    const integration = g3tsLlmsAuditor({
      ...baseConfig,
      requiredSections: ["Docs"],
      requiredRoutePatterns: ["/docs/"]
    });

    await integration.hooks["astro:build:done"]?.({
      pages: [],
      dir: pathToFileURL(`${outputDir}/`),
      assets: new Map(),
      logger: {} as never
    });
  });
});

test("Astro integration throws on invalid llms.txt", async () => {
  await withOutputFixture("## Docs\n- [Docs](https://example.com/docs/)\n", async (outputDir) => {
    const integration = g3tsLlmsAuditor({
      ...baseConfig
    });

    await assert.rejects(
      integration.hooks["astro:build:done"]?.({
        pages: [],
        dir: pathToFileURL(`${outputDir}/`),
        assets: new Map(),
        logger: {} as never
      }),
      /llms-parse-failed/
    );
  });
});

test("Astro integration rejects missing required config at construction", () => {
  assert.throws(
    () => g3tsLlmsAuditor({} as never),
    /site must be a non-empty HTTPS URL string/
  );
});

test("Astro integration rejects unknown config keys at construction", () => {
  assert.throws(
    () =>
      g3tsLlmsAuditor({
        ...baseConfig,
        requiredLinks: ["https://example.com/docs/"]
      } as never),
    /unknown key/
  );
});

test("fails when llms.txt links to an unapproved external URL", async () => {
  await withOutputFixture(
    [
      "# Example",
      "",
      "## Docs",
      "",
      "- [Docs](https://example.com/docs/): Documentation",
      "- [External](https://external.example/docs/): External"
    ].join("\n"),
    async (outputDir) => {
      const result = await checkLlmsTxt({
        ...baseConfig,
        outputDir
      });

      assert.equal(result.ok, false);
      assert.deepEqual(result.findings.map((finding) => finding.code), [
        "llms-external-link-unapproved"
      ]);
    }
  );
});

test("fails when llms.txt internal link has no built page", async () => {
  await withOutputFixture(
    [
      "# Example",
      "",
      "## Docs",
      "",
      "- [Docs](https://example.com/docs/): Documentation",
      "- [Missing](https://example.com/missing/): Missing"
    ].join("\n"),
    async (outputDir) => {
      const result = await checkLlmsTxt({
        ...baseConfig,
        outputDir
      });

      assert.equal(result.ok, false);
      assert.deepEqual(result.findings.map((finding) => finding.code), [
        "llms-internal-link-missing-page"
      ]);
    }
  );
});

test("fails when same-host llms.txt link is not HTTPS even if allowed as non-page URL", async () => {
  await withOutputFixture(
    [
      "# Example",
      "",
      "## Docs",
      "",
      "- [Docs](https://example.com/docs/): Documentation",
      "- [Feed](http://example.com/feed.xml): Feed"
    ].join("\n"),
    async (outputDir) => {
      const result = await checkLlmsTxt({
        ...baseConfig,
        outputDir,
        allowedNonPageUrls: ["/feed.xml"]
      });

      assert.equal(result.ok, false);
      assert.deepEqual(result.findings.map((finding) => finding.code), [
        "llms-external-link-unapproved"
      ]);
    }
  );
});

test("package metadata exposes Astro integration and library API", async () => {
  const packageJson = JSON.parse(
    await fs.readFile(path.join(packageRoot, "package.json"), "utf8")
  ) as {
    name?: string;
    bin?: Record<string, string>;
    dependencies?: Record<string, string>;
  };

  assert.equal(packageJson.name, "g3ts-astro-llms-auditor");
  assert.equal(packageJson.bin, undefined);
  assert.equal(packageJson.dependencies?.["parse-llms-txt"], "0.0.10");
  assert.equal(typeof packageJson.dependencies?.["minimatch"], "string");
});

async function withFixture(
  contents: string,
  run: (llmsPath: string) => Promise<void>
): Promise<void> {
  const fixtureDir = await fs.mkdtemp(path.join(os.tmpdir(), "g3ts-llms-checks-"));
  const llmsPath = path.join(fixtureDir, "llms.txt");

  try {
    await fs.writeFile(llmsPath, contents, "utf8");
    await run(llmsPath);
  } finally {
    await fs.rm(fixtureDir, { recursive: true, force: true });
  }
}

async function withOutputFixture(
  contents: string,
  run: (outputDir: string) => Promise<void>
): Promise<void> {
  const fixtureDir = await fs.mkdtemp(path.join(os.tmpdir(), "g3ts-llms-checks-"));
  const outputDir = path.join(fixtureDir, "dist");

  try {
    await fs.mkdir(outputDir);
    await fs.writeFile(path.join(outputDir, "llms.txt"), contents, "utf8");
    await fs.mkdir(path.join(outputDir, "docs"), { recursive: true });
    await fs.writeFile(path.join(outputDir, "docs", "index.html"), "<html></html>", "utf8");
    await run(outputDir);
  } finally {
    await fs.rm(fixtureDir, { recursive: true, force: true });
  }
}

function validLlmsTxt(): string {
  return [
    "# Example",
    "",
    "> Example content.",
    "",
    "## Docs",
    "",
    "- [Docs](https://example.com/docs/): Documentation"
  ].join("\n");
}
