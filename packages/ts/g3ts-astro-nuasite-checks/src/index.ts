import type { Check, PageCheckContext } from "@nuasite/checks";

export const structuredDataPresentCheck: Check = {
  kind: "page",
  id: "g3/structured-data-present",
  name: "Structured Data Present",
  domain: "seo",
  defaultSeverity: "error",
  description: "Every public page must render at least one JSON-LD block.",
  essential: true,
  run(ctx: PageCheckContext) {
    return ctx.pageData.jsonLd.length === 0
      ? [
          {
            message: "Page is missing JSON-LD structured data",
            suggestion: "Render a schema-dts typed JSON-LD object in the page head."
          }
        ]
      : [];
  }
};
