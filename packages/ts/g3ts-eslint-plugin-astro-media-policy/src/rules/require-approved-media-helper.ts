import { AST_NODE_TYPES, ESLintUtils } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

import {
  hasLocalBindingBefore,
} from "../utils/ast.js";
import {
  importMatchesConfiguredModule,
  normalizeConfiguredModules
} from "../utils/module-identity.js";
import {
  astroMediaPolicyOptionsSchema,
  missingRequiredOptions,
  normalizePublicPath,
  publicPathWithoutSearchOrHash,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";

type MessageIds = "missingConfig" | "rawMetadataImage" | "unapprovedMetadataHelper";

const createRule = ESLintUtils.RuleCreator(
  (name) =>
    `https://github.com/websmasher/guardrail3/tree/main/packages/ts/g3ts-eslint-plugin-astro-media-policy#${name}`
);

export default createRule<RuleOptionsTuple, MessageIds>({
  name: "require-approved-media-helper",
  meta: {
    type: "problem",
    docs: {
      description:
        "Require metadata image fields to be built through approved media helper calls."
    },
    schema: astroMediaPolicyOptionsSchema,
    messages: {
      missingConfig:
        "astro-media-policy/require-approved-media-helper requires non-empty options: {{missing}}.",
      rawMetadataImage:
        "Metadata image property `{{prop}}` uses raw public image path `{{path}}`. Build it through one approved media helper: {{helpers}}.",
      unapprovedMetadataHelper:
        "Metadata image property `{{prop}}` must be built through an approved media helper imported from one configured media helper module: {{helpers}}."
    }
  },
  defaultOptions: [{}],
  create(context) {
    const options = resolveOptions(context.options[0]);
    const missing = missingRequiredOptions(options, [
      "mediaHelperModules",
      "approvedMediaHelpers",
      "checkedImageExtensions",
      "metadataImagePropertyNames"
    ]);

    if (missing.length > 0) {
      return {
        Program(node): void {
          context.report({
            node,
            messageId: "missingConfig",
            data: { missing: missing.join(", ") }
          });
        }
      };
    }

    const metadataProps = new Set(options.metadataImagePropertyNames);
    const helpers = options.approvedMediaHelpers.join(", ");
    const approvedHelpers = new Set(options.approvedMediaHelpers);
    const mediaHelperModules = normalizeConfiguredModules(options.mediaHelperModules);
    const checkedExtensions = new Set(options.checkedImageExtensions);
    const importedApprovedHelpers = new Map<string, string>();

    return {
      ImportDeclaration(node): void {
        if (
          typeof node.source.value !== "string" ||
          !importMatchesConfiguredModule(node.source.value, context, mediaHelperModules)
        ) {
          return;
        }

        for (const specifier of node.specifiers) {
          if (
            specifier.type === AST_NODE_TYPES.ImportSpecifier &&
            specifier.imported.type === AST_NODE_TYPES.Identifier &&
            approvedHelpers.has(specifier.imported.name)
          ) {
            importedApprovedHelpers.set(specifier.local.name, specifier.imported.name);
          }
          if (
            specifier.type === AST_NODE_TYPES.ImportDefaultSpecifier &&
            approvedHelpers.has("default")
          ) {
            importedApprovedHelpers.set(specifier.local.name, "default");
          }
        }
      },
      Property(node): void {
        const prop = propertyName(node.key);
        if (!prop || !metadataProps.has(prop)) {
          return;
        }

        const value = staticStringFromExpression(node.value);
        if (value !== null && shouldReportImagePath(value, checkedExtensions)) {
          context.report({
            node: node.value,
            messageId: "rawMetadataImage",
            data: {
              prop,
              path: normalizePublicPath(value),
              helpers
            }
          });

          return;
        }

        if (containsRawImagePath(node.value, checkedExtensions)) {
          context.report({
            node: node.value,
            messageId: "rawMetadataImage",
            data: {
              prop,
              path: "<nested metadata image path>",
              helpers
            }
          });

          return;
        }

        if (!isApprovedHelperCall(context, node.value, approvedHelpers, importedApprovedHelpers)) {
          context.report({
            node: node.value,
            messageId: "unapprovedMetadataHelper",
            data: { prop, helpers }
          });
        }
      }
    };
  }
});

function shouldReportImagePath(value: string, extensions: Set<string>): boolean {
  if (!value.trim().startsWith("/")) {
    return false;
  }

  const normalized = publicPathWithoutSearchOrHash(value).toLowerCase();

  return [...extensions].some((extension) => normalized.endsWith(extension));
}

function containsRawImagePath(node: TSESTree.Node, extensions: Set<string>): boolean {
  if (
    node.type === AST_NODE_TYPES.Literal &&
    typeof node.value === "string" &&
    shouldReportImagePath(node.value, extensions)
  ) {
    return true;
  }

  if (node.type === AST_NODE_TYPES.TemplateLiteral && node.expressions.length === 0) {
    const value = node.quasis.map((quasi) => quasi.value.cooked ?? quasi.value.raw).join("");

    return shouldReportImagePath(value, extensions);
  }

  if (node.type === AST_NODE_TYPES.TemplateLiteral) {
    return shouldReportTemplate(node, extensions);
  }

  if (node.type === AST_NODE_TYPES.ArrayExpression) {
    return node.elements.some((element) => element !== null && containsRawImagePath(element, extensions));
  }

  if (node.type === AST_NODE_TYPES.ObjectExpression) {
    return node.properties.some((property) => {
      if (property.type !== AST_NODE_TYPES.Property) {
        return false;
      }

      return containsRawImagePath(property.value, extensions);
    });
  }

  return false;
}

function shouldReportTemplate(
  node: TSESTree.TemplateLiteral,
  extensions: Set<string>
): boolean {
  const first = node.quasis[0]?.value.raw.trim() ?? "";
  const last = publicPathWithoutSearchOrHash(node.quasis.at(-1)?.value.raw.trim() ?? "");

  return first.startsWith("/") && [...extensions].some((extension) => last.toLowerCase().endsWith(extension));
}

function isApprovedHelperCall(
  context: Parameters<Parameters<typeof createRule>[0]["create"]>[0],
  node: TSESTree.Property["value"],
  approvedHelpers: Set<string>,
  importedApprovedHelpers: Map<string, string>
): boolean {
  if (node.type !== AST_NODE_TYPES.CallExpression) {
    const value = staticStringFromExpression(node);

    return value !== null && value.trim().startsWith("/");
  }

  const helperName = calleeName(node.callee);
  if (helperName === null) {
    return false;
  }

  if (node.callee.type !== AST_NODE_TYPES.Identifier) {
    return false;
  }

  const importedName = importedApprovedHelpers.get(helperName);

  return (
    importedName !== undefined &&
    approvedHelpers.has(importedName) &&
    !hasLocalBindingBefore(context, node.callee, helperName)
  );
}

function calleeName(callee: TSESTree.CallExpression["callee"]): string | null {
  if (callee.type === AST_NODE_TYPES.Identifier) {
    return callee.name;
  }

  if (
    callee.type === AST_NODE_TYPES.MemberExpression &&
    callee.property.type === AST_NODE_TYPES.Identifier
  ) {
    return callee.property.name;
  }

  return null;
}

function propertyName(node: TSESTree.Property["key"]): string | null {
  if (node.type === AST_NODE_TYPES.Identifier) {
    return node.name;
  }
  if (node.type === AST_NODE_TYPES.Literal && typeof node.value === "string") {
    return node.value;
  }

  return null;
}

function staticStringFromExpression(node: TSESTree.Property["value"]): string | null {
  if (node.type === AST_NODE_TYPES.Literal && typeof node.value === "string") {
    return node.value;
  }

  if (node.type === AST_NODE_TYPES.TemplateLiteral && node.expressions.length === 0) {
    return node.quasis.map((quasi) => quasi.value.cooked ?? quasi.value.raw).join("");
  }

  return null;
}
