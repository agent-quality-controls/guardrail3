import { AST_NODE_TYPES, ESLintUtils } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

import {
  jsxAttributeName,
  jsxElementName,
  staticStringFromJsxAttribute
} from "../utils/ast.js";
import {
  astroMediaPolicyOptionsSchema,
  missingRequiredOptions,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";

type MessageIds = "missingConfig" | "inlineAlt";

const createRule = ESLintUtils.RuleCreator(
  (name) =>
    `https://github.com/websmasher/guardrail3/tree/main/packages/ts/g3ts-eslint-plugin-astro-media-policy#${name}`
);

export default createRule<RuleOptionsTuple, MessageIds>({
  name: "no-inline-image-alt",
  meta: {
    type: "problem",
    docs: {
      description:
        "Forbid inline localized alt text on configured content image components."
    },
    schema: astroMediaPolicyOptionsSchema,
    messages: {
      missingConfig:
        "astro-media-policy/no-inline-image-alt requires non-empty options: {{missing}}.",
      inlineAlt:
        "`{{component}}` uses inline localized image text in prop `{{prop}}`. Move alt text into the locale-owned content image entry and pass an image key."
    }
  },
  defaultOptions: [{}],
  create(context) {
    const options = resolveOptions(context.options[0]);
    const missing = missingRequiredOptions(options, [
      "contentImageComponents",
      "bannedImageAltProps"
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

    const components = new Set(options.contentImageComponents);
    const bannedAltProps = new Set(options.bannedImageAltProps);

    return {
      JSXOpeningElement(node): void {
        const component = jsxElementName(node.name);

        if (!component || !components.has(component)) {
          return;
        }

        for (const attribute of node.attributes) {
          if (attribute.type !== AST_NODE_TYPES.JSXAttribute) {
            continue;
          }

          const prop = jsxAttributeName(attribute.name);
          if (!bannedAltProps.has(prop)) {
            continue;
          }

          if (hasStaticInlineAlt(attribute)) {
            context.report({
              node: attribute,
              messageId: "inlineAlt",
              data: { component, prop }
            });
          }
        }
      }
    };
  }
});

function hasStaticInlineAlt(attribute: TSESTree.JSXAttribute): boolean {
  const value = staticStringFromJsxAttribute(attribute);
  if (value !== null) {
    return true;
  }

  return (
    attribute.value?.type === AST_NODE_TYPES.JSXExpressionContainer &&
    isStaticStringExpression(attribute.value.expression)
  );
}

function isStaticStringExpression(
  node: TSESTree.Expression | TSESTree.JSXEmptyExpression
): boolean {
  if (node.type === AST_NODE_TYPES.Literal) {
    return typeof node.value === "string";
  }

  if (node.type === AST_NODE_TYPES.TemplateLiteral) {
    return true;
  }

  if (node.type === AST_NODE_TYPES.BinaryExpression && node.operator === "+") {
    return isStaticStringExpression(node.left) || isStaticStringExpression(node.right);
  }

  return false;
}
