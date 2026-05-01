import assert from "node:assert/strict";
import { test } from "node:test";

import { recommended } from "../src/index.js";

test("recommended config factory registers plugin and configured rule options", () => {
  const config = recommended({
    denyList: ["text-black"],
    classAttributes: ["class", "className"],
    classListAttributes: ["class:list"],
    classHelpers: ["cn"]
  });

  assert.equal(typeof config.plugins?.["style-policy"], "object");
  assert.deepEqual(config.rules?.["style-policy/no-denied-class-tokens"], [
    "error",
    {
      denyList: ["text-black"],
      classAttributes: ["class", "className"],
      classListAttributes: ["class:list"],
      classHelpers: ["cn"]
    }
  ]);
});
