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
