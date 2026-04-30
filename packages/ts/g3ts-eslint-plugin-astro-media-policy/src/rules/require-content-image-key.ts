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

type MessageIds =
  | "missingConfig"
  | "missingImageKey"
  | "invalidImageKey"
  | "bannedSourceProp";

const createRule = ESLintUtils.RuleCreator(
  (name) =>
    `https://github.com/websmasher/guardrail3/tree/main/packages/ts/g3ts-eslint-plugin-astro-media-policy#${name}`
);

export default createRule<RuleOptionsTuple, MessageIds>({
  name: "require-content-image-key",
  meta: {
    type: "problem",
    docs: {
      description:
        "Require configured content image components to use locale-owned image keys instead of raw image sources."
    },
    schema: astroMediaPolicyOptionsSchema,
    messages: {
      missingConfig:
        "astro-media-policy/require-content-image-key requires non-empty options: {{missing}}.",
      missingImageKey:
        "`{{component}}` must pass one configured image key prop: {{props}}.",
      invalidImageKey:
        "`{{component}}` passes image key prop `{{prop}}` without a usable key value.",
      bannedSourceProp:
        "`{{component}}` uses raw image source prop `{{prop}}`. Use one configured image key prop instead: {{props}}."
    }
  },
  defaultOptions: [{}],
  create(context) {
    const options = resolveOptions(context.options[0]);
    const missing = missingRequiredOptions(options, [
      "contentImageComponents",
      "contentImageKeyProps",
      "bannedImageSourceProps"
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
    const keyProps = new Set(options.contentImageKeyProps);
    const bannedSourceProps = new Set(options.bannedImageSourceProps);

    return {
      JSXOpeningElement(node): void {
        const component = jsxElementName(node.name);

        if (!component || !components.has(component)) {
          return;
        }

        let hasKey = false;

        for (const attribute of node.attributes) {
          if (attribute.type !== AST_NODE_TYPES.JSXAttribute) {
            continue;
          }

          const prop = jsxAttributeName(attribute.name);
          if (keyProps.has(prop)) {
            if (hasUsableKeyValue(attribute)) {
              hasKey = true;
            } else {
              context.report({
                node: attribute,
                messageId: "invalidImageKey",
                data: { component, prop }
              });
            }
          }

          if (bannedSourceProps.has(prop)) {
            context.report({
              node: attribute,
              messageId: "bannedSourceProp",
              data: {
                component,
                prop,
                props: options.contentImageKeyProps.join(", ")
              }
            });
          }
        }

        if (!hasKey) {
          context.report({
            node,
            messageId: "missingImageKey",
            data: {
              component,
              props: options.contentImageKeyProps.join(", ")
            }
          });
        }
      }
    };
  }
});

function hasUsableKeyValue(attribute: TSESTree.JSXAttribute): boolean {
  if (!attribute.value) {
    return false;
  }

  const staticValue = staticStringFromJsxAttribute(attribute);
  if (staticValue !== null) {
    return staticValue.trim().length > 0;
  }

  if (attribute.value.type !== AST_NODE_TYPES.JSXExpressionContainer) {
    return true;
  }

  const expression = attribute.value.expression;

  if (expression.type === AST_NODE_TYPES.JSXEmptyExpression) {
    return false;
  }
  if (
    expression.type === AST_NODE_TYPES.Literal &&
    (typeof expression.value === "boolean" ||
      typeof expression.value === "number" ||
      expression.value === null)
  ) {
    return false;
  }

  if (
    expression.type === AST_NODE_TYPES.ObjectExpression ||
    expression.type === AST_NODE_TYPES.ArrayExpression
  ) {
    return false;
  }

  return expression.type !== AST_NODE_TYPES.Identifier || expression.name !== "undefined";
}
