import { AST_NODE_TYPES, ESLintUtils } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

import {
  hasLocalBindingBefore,
  staticStringFromExpression,
  staticStringFromJsxAttribute
} from "../utils/ast.js";
import {
  astroMediaPolicyOptionsSchema,
  missingRequiredOptions,
  normalizePublicPath,
  publicPathWithoutSearchOrHash,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";

type MessageIds = "missingConfig" | "rawPublicImagePath";

const createRule = ESLintUtils.RuleCreator(
  (name) =>
    `https://github.com/websmasher/guardrail3/tree/main/packages/ts/g3ts-eslint-plugin-astro-media-policy#${name}`
);

export default createRule<RuleOptionsTuple, MessageIds>({
  name: "no-raw-public-image-paths",
  meta: {
    type: "problem",
    docs: {
      description:
        "Forbid raw public image paths in source; media paths must flow through approved helpers or content image keys."
    },
    schema: astroMediaPolicyOptionsSchema,
    messages: {
      missingConfig:
        "astro-media-policy/no-raw-public-image-paths requires non-empty options: {{missing}}.",
      rawPublicImagePath:
        "Raw public image path `{{path}}` is not allowed here. Use an approved media helper or a content image key."
    }
  },
  defaultOptions: [{}],
  create(context) {
    const options = resolveOptions(context.options[0]);
    const missing = missingRequiredOptions(options, [
      "mediaHelperModules",
      "approvedMediaHelpers",
      "allowedPublicImagePaths",
      "checkedImageExtensions"
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

    const allowed = new Set(
      options.allowedPublicImagePaths.map((value) => normalizePublicPath(value))
    );
    const checkedExtensions = new Set(options.checkedImageExtensions);
    const approvedHelpers = new Set(options.approvedMediaHelpers);
    const mediaHelperModules = new Set(options.mediaHelperModules);
    const importedApprovedHelpers = new Set<string>();

    function shouldReport(value: string): boolean {
      const path = normalizePublicPath(value);
      const pathForExtension = publicPathWithoutSearchOrHash(value);

      return (
        value.trim().startsWith("/") &&
        hasCheckedExtension(pathForExtension, checkedExtensions) &&
        !allowed.has(path)
      );
    }

    function report(node: TSESTree.Node, value: string): void {
      context.report({
        node,
        messageId: "rawPublicImagePath",
        data: { path: value }
      });
    }

    return {
      ImportDeclaration(node): void {
        if (
          typeof node.source.value !== "string" ||
          !mediaHelperModules.has(node.source.value)
        ) {
          return;
        }

        for (const specifier of node.specifiers) {
          if (
            specifier.type === AST_NODE_TYPES.ImportSpecifier &&
            specifier.imported.type === AST_NODE_TYPES.Identifier &&
            approvedHelpers.has(specifier.imported.name)
          ) {
            importedApprovedHelpers.add(specifier.local.name);
          }
          if (
            specifier.type === AST_NODE_TYPES.ImportDefaultSpecifier &&
            approvedHelpers.has("default")
          ) {
            importedApprovedHelpers.add(specifier.local.name);
          }
        }
      },
      Literal(node): void {
        if (
          typeof node.value !== "string" ||
          isImportOrExportSource(node) ||
          isApprovedHelperArgument(context, node, importedApprovedHelpers)
        ) {
          return;
        }
        if (shouldReport(node.value)) {
          report(node, node.value);
        }
      },
      TemplateLiteral(node): void {
        if (isApprovedHelperArgument(context, node, importedApprovedHelpers)) {
          return;
        }
        const value = staticStringFromExpression(node);
        if (value !== null && shouldReport(value)) {
          report(node, value);
        }
        if (value === null && shouldReportTemplate(node, checkedExtensions)) {
          report(node, context.sourceCode.getText(node));
        }
      },
      JSXAttribute(node): void {
        const value = staticStringFromJsxAttribute(node);
        if (value !== null && shouldReport(value)) {
          report(node, value);
        }
      }
    };
  }
});

function hasCheckedExtension(path: string, extensions: Set<string>): boolean {
  const lower = path.toLowerCase();

  return [...extensions].some((extension) => lower.endsWith(extension));
}

function shouldReportTemplate(
  node: TSESTree.TemplateLiteral,
  extensions: Set<string>
): boolean {
  const first = node.quasis[0]?.value.raw.trim() ?? "";
  const combined = publicPathWithoutSearchOrHash(
    node.quasis.map((quasi) => quasi.value.raw).join("")
  );

  return first.startsWith("/") && hasCheckedExtension(combined, extensions);
}

function isImportOrExportSource(node: TSESTree.Literal): boolean {
  const parent = node.parent;

  return (
    parent?.type === AST_NODE_TYPES.JSXAttribute ||
    parent?.type === AST_NODE_TYPES.ImportDeclaration ||
    parent?.type === AST_NODE_TYPES.ExportAllDeclaration ||
    parent?.type === AST_NODE_TYPES.ExportNamedDeclaration
  );
}

function isApprovedHelperArgument(
  context: Parameters<Parameters<typeof createRule>[0]["create"]>[0],
  node: TSESTree.Node,
  approvedHelpers: Set<string>
): boolean {
  const parent = node.parent;
  if (parent?.type !== AST_NODE_TYPES.CallExpression) {
    return false;
  }

  const name = calleeName(parent.callee);

  return (
    name !== null &&
    approvedHelpers.has(name) &&
    !hasLocalBindingBefore(context, parent.callee, name)
  );
}

function calleeName(callee: TSESTree.CallExpression["callee"]): string | null {
  if (callee.type === AST_NODE_TYPES.Identifier) {
    return callee.name;
  }

  return null;
}
