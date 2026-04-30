import { mkdir, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { test } from "node:test";
import assert from "node:assert/strict";

import { checkMediaAssets } from "../src/index.js";

test("passes when every required media asset exists", async () => {
  const outputDir = await outputFixture([
    "favicon.svg",
    "apple-touch-icon.png",
    "og/default.png"
  ]);

  await checkMediaAssets({
    outputDir,
    favicon: "/favicon.svg",
    appIcons: ["/apple-touch-icon.png"],
    defaultSocialImage: "/og/default.png",
    allowSvgIcons: true
  });
});

test("fails when a required media asset is missing", async () => {
  const outputDir = await outputFixture(["favicon.svg", "apple-touch-icon.png"]);

  await assert.rejects(
    checkMediaAssets({
      outputDir,
      favicon: "/favicon.svg",
      appIcons: ["/apple-touch-icon.png"],
      defaultSocialImage: "/og/default.png",
      allowSvgIcons: true
    }),
    /required media asset/
  );
});

test("reports every missing and invalid media asset in one failure", async () => {
  const outputDir = await outputFixture([]);

  await assert.rejects(
    checkMediaAssets({
      outputDir,
      favicon: "/favicon.svg",
      appIcons: ["/apple-touch-icon.png"],
      defaultSocialImage: "/og/default.png",
      allowSvgIcons: false
    }),
    (error) => {
      assert(error instanceof Error);
      assert.match(error.message, /SVG media asset is not allowed/);
      assert.match(error.message, /apple-touch-icon\.png/);
      assert.match(error.message, /og\/default\.png/);

      return true;
    }
  );
});

test("fails when svg assets are disabled", async () => {
  const outputDir = await outputFixture(["favicon.svg", "apple-touch-icon.png", "og/default.png"]);

  await assert.rejects(
    checkMediaAssets({
      outputDir,
      favicon: "/favicon.svg",
      appIcons: ["/apple-touch-icon.png"],
      defaultSocialImage: "/og/default.png",
      allowSvgIcons: false
    }),
    /SVG media asset is not allowed/
  );
});

test("fails on external paths", async () => {
  const outputDir = await outputFixture(["favicon.svg", "apple-touch-icon.png", "og/default.png"]);

  await assert.rejects(
    checkMediaAssets({
      outputDir,
      favicon: "https://example.com/favicon.svg",
      appIcons: ["/apple-touch-icon.png"],
      defaultSocialImage: "/og/default.png",
      allowSvgIcons: true
    }),
    /root-relative public path/
  );
});

test("fails on backslash and encoded traversal paths", async () => {
  const outputDir = await outputFixture(["favicon.svg", "apple-touch-icon.png", "og/default.png"]);

  await assert.rejects(
    checkMediaAssets({
      outputDir,
      favicon: "/..\\secret.svg",
      appIcons: ["/apple-touch-icon.png"],
      defaultSocialImage: "/og/%2e%2e/default.png",
      allowSvgIcons: true
    }),
    (error) => {
      assert(error instanceof Error);
      assert.match(error.message, /favicon must not traverse/);
      assert.match(error.message, /defaultSocialImage must not traverse/);

      return true;
    }
  );
});

test("fails on encoded slash and backslash paths", async () => {
  const outputDir = await outputFixture(["favicon.svg", "apple-touch-icon.png", "og/default.png"]);

  await assert.rejects(
    checkMediaAssets({
      outputDir,
      favicon: "/fav%5cicon.svg",
      appIcons: ["/apple%2ftouch-icon.png"],
      defaultSocialImage: "/og/default.png",
      allowSvgIcons: true
    }),
    (error) => {
      assert(error instanceof Error);
      assert.match(error.message, /favicon must not traverse/);
      assert.match(error.message, /appIcons\[0\] must not traverse/);

      return true;
    }
  );
});

test("allows safe encoded dot filenames", async () => {
  const outputDir = await outputFixture([
    "favicon.svg",
    "apple-touch-icon.png",
    "og/logo%2emark.png"
  ]);

  await checkMediaAssets({
    outputDir,
    favicon: "/favicon.svg",
    appIcons: ["/apple-touch-icon.png"],
    defaultSocialImage: "/og/logo%2emark.png",
    allowSvgIcons: true
  });
});

test("fails when configured asset path is a directory", async () => {
  const outputDir = await outputFixture(["favicon.svg", "apple-touch-icon.png"]);
  await mkdir(path.join(outputDir, "og/default.png"), { recursive: true });

  await assert.rejects(
    checkMediaAssets({
      outputDir,
      favicon: "/favicon.svg",
      appIcons: ["/apple-touch-icon.png"],
      defaultSocialImage: "/og/default.png",
      allowSvgIcons: true
    }),
    /not a file/
  );
});

test("fails with actionable option type errors", async () => {
  const outputDir = await outputFixture([]);

  await assert.rejects(
    checkMediaAssets({
      outputDir,
      favicon: 1,
      appIcons: ["/apple-touch-icon.png", false],
      defaultSocialImage: "/og/default.png",
      allowSvgIcons: "true"
    } as unknown as Parameters<typeof checkMediaAssets>[0]),
    (error) => {
      assert(error instanceof Error);
      assert.match(error.message, /favicon must be a string/);
      assert.match(error.message, /appIcons\[1\] must be a string/);
      assert.match(error.message, /allowSvgIcons must be a boolean/);

      return true;
    }
  );
});

test("fails with actionable array and outputDir type errors", async () => {
  await assert.rejects(
    checkMediaAssets({
      outputDir: 1,
      favicon: "/favicon.svg",
      appIcons: [],
      defaultSocialImage: "/og/default.png",
      allowSvgIcons: true
    } as unknown as Parameters<typeof checkMediaAssets>[0]),
    (error) => {
      assert(error instanceof Error);
      assert.match(error.message, /outputDir must be a string/);
      assert.match(error.message, /appIcons must be a non-empty array/);

      return true;
    }
  );

  await assert.rejects(
    checkMediaAssets({
      outputDir: "",
      favicon: "/favicon.svg",
      appIcons: "apple-touch-icon.png",
      defaultSocialImage: "/og/default.png",
      allowSvgIcons: true
    } as unknown as Parameters<typeof checkMediaAssets>[0]),
    (error) => {
      assert(error instanceof Error);
      assert.match(error.message, /outputDir must be non-empty/);
      assert.match(error.message, /appIcons must be a non-empty array/);

      return true;
    }
  );
});

async function outputFixture(files: string[]): Promise<string> {
  const root = path.join(
    tmpdir(),
    `g3ts-astro-media-assets-${process.pid}-${Date.now()}-${Math.random()
      .toString(16)
      .slice(2)}`
  );

  for (const file of files) {
    const fullPath = path.join(root, file);
    await mkdir(path.dirname(fullPath), { recursive: true });
    await writeFile(fullPath, "asset");
  }

  return root;
}
