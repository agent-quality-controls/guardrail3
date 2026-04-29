import { AST_NODE_TYPES, ESLintUtils } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

import {
  staticStringFromExpression,
  staticStringFromJsxAttribute
} from "../utils/ast.js";
import {
  astroMediaPolicyOptionsSchema,
  missingRequiredOptions,
  normalizePublicPath,
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
      "approvedMediaHelpers",
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

    function shouldReport(value: string): boolean {
      const path = normalizePublicPath(value);

      return (
        value.trim().startsWith("/") &&
        hasCheckedExtension(path, checkedExtensions) &&
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
      Literal(node): void {
        if (
          typeof node.value !== "string" ||
          isImportOrExportSource(node) ||
          isApprovedHelperArgument(node, approvedHelpers)
        ) {
          return;
        }
        if (shouldReport(node.value)) {
          report(node, node.value);
        }
      },
      TemplateLiteral(node): void {
        if (isApprovedHelperArgument(node, approvedHelpers)) {
          return;
        }
        const value = staticStringFromExpression(node);
        if (value !== null && shouldReport(value)) {
          report(node, value);
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
  node: TSESTree.Node,
  approvedHelpers: Set<string>
): boolean {
  const parent = node.parent;
  if (parent?.type !== AST_NODE_TYPES.CallExpression) {
    return false;
  }

  const name = calleeName(parent.callee);

  return name !== null && approvedHelpers.has(name);
}

function calleeName(callee: TSESTree.CallExpression["callee"]): string | null {
  if (callee.type === AST_NODE_TYPES.Identifier) {
    return callee.name;
  }
  if (callee.type === AST_NODE_TYPES.MemberExpression) {
    const property = callee.property;
    if (!callee.computed && property.type === AST_NODE_TYPES.Identifier) {
      return property.name;
    }
  }

  return null;
}
