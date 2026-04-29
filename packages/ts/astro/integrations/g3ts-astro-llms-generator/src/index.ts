import { mkdir, writeFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";

import type { AstroIntegration } from "astro";
import { z } from "zod";

const safeTextSchema = z.string().trim().min(1).refine((value) => !/[\r\n]/u.test(value), {
  message: "text fields must be single-line"
});

const linkConfigSchema = z.object({
  title: safeTextSchema,
  href: z.string().trim().min(1).refine((value) => !/[\r\n<>\s]/u.test(value), {
    message: "href must be a single URL token without whitespace or angle brackets"
  }),
  description: safeTextSchema.optional()
});

const sectionConfigSchema = z.object({
  heading: safeTextSchema,
  links: z.array(linkConfigSchema).min(1)
});

const httpsUrlSchema = z.url().refine(
  (value) => {
    try {
      return new URL(value).protocol === "https:";
    } catch {
      return false;
    }
  },
  {
    message: "site must use https"
  }
);

const llmsConfigSchema = z
  .object({
    title: safeTextSchema,
    site: httpsUrlSchema,
    sections: z.array(sectionConfigSchema).min(1)
  })
  .strict();

export type G3TsLlmsGeneratorConfig = z.input<typeof llmsConfigSchema>;
export type G3TsAstroLlmsGeneratorConfig = G3TsLlmsGeneratorConfig;
type ParsedLlmsConfig = z.output<typeof llmsConfigSchema>;
type LlmsLinkConfig = z.output<typeof linkConfigSchema>;
type LlmsSectionConfig = z.output<typeof sectionConfigSchema>;

export default function g3tsLlmsGenerator(
  config: G3TsLlmsGeneratorConfig
): AstroIntegration {
  const parsedConfig = parseConfig(config);

  return {
    name: "g3ts-astro-llms-generator",
    hooks: {
      "astro:build:done": async ({ dir }) => {
        await writeLlmsTxt(dir, parsedConfig);
      }
    }
  };
}

export function generateLlmsTxt(config: G3TsLlmsGeneratorConfig): string {
  return renderLlmsTxt(parseConfig(config));
}

async function writeLlmsTxt(outDir: URL, config: ParsedLlmsConfig): Promise<void> {
  const outputPath = new URL("llms.txt", ensureDirectoryUrl(outDir));
  await mkdir(fileURLToPath(new URL(".", outputPath)), { recursive: true });
  await writeFile(outputPath, renderLlmsTxt(config), "utf8");
}

function parseConfig(config: G3TsLlmsGeneratorConfig): ParsedLlmsConfig {
  const result = llmsConfigSchema.safeParse(config);
  if (result.success) {
    assertLinksAreSafe(result.data);
    return result.data;
  }

  const message = result.error.issues
    .map((issue) => {
      const path = issue.path.length > 0 ? issue.path.join(".") : "config";
      return `${path}: ${issue.message}`;
    })
    .join("; ");

  throw new Error(`Invalid g3ts-astro-llms-generator config: ${message}`);
}

function renderLlmsTxt(config: ParsedLlmsConfig): string {
  const lines = [`# ${escapeText(config.title)}`, "", `> ${normalizeSite(config.site)}`, ""];

  for (const section of sortSections(config.sections)) {
    lines.push(`## ${escapeText(section.heading)}`, "");

    for (const link of sortLinks(section.links)) {
      lines.push(renderLink(link, config.site));
    }

    lines.push("");
  }

  return `${lines.join("\n").trimEnd()}\n`;
}

function renderLink(link: LlmsLinkConfig, site: string): string {
  const url = normalizeHref(site, link.href);
  if (link.description) {
    return `- [${escapeLinkText(link.title)}](${url}): ${escapeText(link.description)}`;
  }

  return `- [${escapeLinkText(link.title)}](${url})`;
}

function escapeLinkText(value: string): string {
  return escapeText(value).replaceAll("[", "\\[").replaceAll("]", "\\]");
}

function escapeText(value: string): string {
  return value.replaceAll("\\", "\\\\");
}

function sortSections(sections: LlmsSectionConfig[]): LlmsSectionConfig[] {
  return [...sections].sort((left, right) =>
    left.heading.localeCompare(right.heading, "en", { sensitivity: "base" })
  );
}

function sortLinks(links: LlmsLinkConfig[]): LlmsLinkConfig[] {
  return [...links].sort((left, right) => {
    const titleOrder = left.title.localeCompare(right.title, "en", {
      sensitivity: "base"
    });
    if (titleOrder !== 0) {
      return titleOrder;
    }

    return left.href.localeCompare(right.href, "en", { sensitivity: "base" });
  });
}

function normalizeSite(site: string): string {
  const url = new URL(site);
  url.hash = "";
  url.search = "";
  url.pathname = "/";
  return url.toString();
}

function normalizeHref(site: string, href: string): string {
  const base = normalizeSite(site);
  const url = new URL(href, base);
  const canonicalSite = new URL(base);
  if (url.protocol !== "https:") {
    throw new Error("Invalid g3ts-astro-llms-generator config: href must resolve to HTTPS");
  }
  if (url.host !== canonicalSite.host) {
    throw new Error("Invalid g3ts-astro-llms-generator config: href must stay on the configured site host");
  }
  url.hash = "";
  return url.toString();
}

function assertLinksAreSafe(config: ParsedLlmsConfig): void {
  for (const section of config.sections) {
    for (const link of section.links) {
      normalizeHref(config.site, link.href);
    }
  }
}

function ensureDirectoryUrl(url: URL): URL {
  return url.pathname.endsWith("/") ? url : new URL(`${url.toString()}/`);
}
