import { AST_NODE_TYPES, ESLintUtils } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

import {
  astroMediaPolicyOptionsSchema,
  missingRequiredOptions,
  normalizePublicPath,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";

type MessageIds = "missingConfig" | "rawMetadataImage";

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
        "Metadata image property `{{prop}}` uses raw public image path `{{path}}`. Build it through one approved media helper: {{helpers}}."
    }
  },
  defaultOptions: [{}],
  create(context) {
    const options = resolveOptions(context.options[0]);
    const missing = missingRequiredOptions(options, [
      "approvedMediaHelpers",
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

    return {
      Property(node): void {
        const prop = propertyName(node.key);
        if (!prop || !metadataProps.has(prop)) {
          return;
        }

        const value = staticStringFromExpression(node.value);
        if (value === null || !value.trim().startsWith("/")) {
          return;
        }

        context.report({
          node: node.value,
          messageId: "rawMetadataImage",
          data: {
            prop,
            path: normalizePublicPath(value),
            helpers
          }
        });
      }
    };
  }
});

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
