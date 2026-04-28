#!/usr/bin/env node
import { checkLlmsTxt, type LlmsChecksConfig } from "./index.js";

interface ParsedArgs {
  config?: LlmsChecksConfig;
  help: boolean;
  error?: string;
}

type ArgValue = { ok: true; value: string } | { ok: false; error: string };

const parsedArgs = parseArgs(process.argv.slice(2));

if (parsedArgs.help) {
  console.log(usage());
  process.exit(0);
}

if (parsedArgs.error || !parsedArgs.config) {
  console.error(parsedArgs.error ?? usage());
  process.exit(2);
}

const result = await checkLlmsTxt(parsedArgs.config);
console.log(JSON.stringify(result, null, 2));
process.exit(result.ok ? 0 : 1);

function parseArgs(args: string[]): ParsedArgs {
  let outputDir: string | undefined;
  const requiredSections: string[] = [];
  const requiredLinks: string[] = [];

  for (let index = 0; index < args.length; index += 1) {
    const arg = args[index];

    if (arg === "--help" || arg === "-h") {
      return { help: true };
    }

    if (arg === "--output-dir") {
      const value = readValue(args, index, arg);
      if (!value.ok) {
        return { help: false, error: value.error };
      }
      outputDir = value.value;
      index += 1;
      continue;
    }

    if (arg === "--required-section") {
      const value = readValue(args, index, arg);
      if (!value.ok) {
        return { help: false, error: value.error };
      }
      requiredSections.push(value.value);
      index += 1;
      continue;
    }

    if (arg === "--required-link") {
      const value = readValue(args, index, arg);
      if (!value.ok) {
        return { help: false, error: value.error };
      }
      requiredLinks.push(value.value);
      index += 1;
      continue;
    }

    return {
      help: false,
      error: `Unknown argument: ${arg}`
    };
  }

  if (!outputDir) {
    return {
      help: false,
      error: "Missing required --output-dir argument"
    };
  }

  return {
    help: false,
    config: {
      outputDir,
      requiredSections,
      requiredLinks
    }
  };
}

function readValue(
  args: string[],
  index: number,
  flag: string
): ArgValue {
  const value = args[index + 1];
  if (!value || value.startsWith("--")) {
    return { ok: false, error: `Missing value for ${flag}` };
  }

  return { ok: true, value };
}

function usage(): string {
  return [
    "Usage: g3ts-astro-llms-checks --output-dir <dir> [--required-section <name>] [--required-link <url>]",
    "",
    "Validates a generated llms.txt artifact only. It does not generate or mutate output."
  ].join("\n");
}
