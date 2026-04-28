import assert from "node:assert/strict";
import { execFile } from "node:child_process";
import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { promisify } from "node:util";

import { checkLlmsTxt } from "../src/index.js";

const execFileAsync = promisify(execFile);
const packageRoot = path.resolve(import.meta.dirname, "..");

test("fails when the configured llms.txt file is missing", async () => {
  const result = await checkLlmsTxt({
    outputDir: path.join(os.tmpdir(), "g3ts-missing-llms"),
    requiredSections: [],
    requiredLinks: []
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
      outputDir: path.dirname(llmsPath),
      requiredSections: [],
      requiredLinks: []
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
        outputDir: path.dirname(llmsPath),
        requiredSections: [],
        requiredLinks: []
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
      outputDir: path.dirname(llmsPath),
      requiredSections: ["Policies"],
      requiredLinks: []
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

test("fails when a configured required link URL is missing", async () => {
  await withFixture(validLlmsTxt(), async (llmsPath) => {
    const result = await checkLlmsTxt({
      outputDir: path.dirname(llmsPath),
      requiredSections: [],
      requiredLinks: ["https://example.com/policies/privacy/"]
    });

    assert.deepEqual(result, {
      ok: false,
      findings: [
        {
          code: "llms-required-link-missing",
          message:
            "Configured required llms.txt link is missing: https://example.com/policies/privacy/",
          path: llmsPath,
          expected: "https://example.com/policies/privacy/"
        }
      ]
    });
  });
});

test("passes when the file exists, parses, and configured sections and links exist", async () => {
  await withFixture(validLlmsTxt(), async (llmsPath) => {
    const result = await checkLlmsTxt({
      outputDir: path.dirname(llmsPath),
      requiredSections: ["Docs"],
      requiredLinks: ["https://example.com/docs/"]
    });

    assert.deepEqual(result, {
      ok: true,
      findings: []
    });
  });
});

test("CLI exposes the same explicit post-build checks", async () => {
  await withOutputFixture(validLlmsTxt(), async (outputDir) => {
    const { stdout } = await execFileAsync(
      "node",
      [
        path.join(packageRoot, "dist/cli.js"),
        "--output-dir",
        outputDir,
        "--required-section",
        "Docs",
        "--required-link",
        "https://example.com/docs/"
      ],
      { cwd: packageRoot }
    );

    assert.deepEqual(JSON.parse(stdout), {
      ok: true,
      findings: []
    });
  });
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
