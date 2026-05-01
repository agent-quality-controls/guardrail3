import { AST_NODE_TYPES, ESLintUtils } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

import {
  calleeName,
  jsxAttributeName,
  staticStringFromJsxAttribute
} from "../utils/ast.js";
import {
  classTokenSitesFromExpression,
  classTokenSitesFromString
} from "../utils/class-extract.js";
import {
  missingRequiredOptions,
  resolveOptions,
  stylePolicyOptionsSchema,
  type RuleOptionsTuple
} from "../utils/options.js";

type MessageIds = "missingConfig" | "deniedClassToken";

const createRule = ESLintUtils.RuleCreator(
  (name) =>
    `https://github.com/websmasher/guardrail3/tree/main/packages/ts/g3ts-eslint-plugin-style-policy#${name}`
);

export default createRule<RuleOptionsTuple, MessageIds>({
  name: "no-denied-class-tokens",
  meta: {
    type: "problem",
    docs: {
      description:
        "Forbid configured denied class tokens in static class attributes and class helper inputs."
    },
    schema: stylePolicyOptionsSchema,
    messages: {
      missingConfig:
        "style-policy/no-denied-class-tokens requires non-empty options: {{missing}}.",
      deniedClassToken:
        "Class token `{{token}}` is denied by style policy. Use the approved design token or component abstraction instead."
    }
  },
  defaultOptions: [{}],
  create(context) {
    const options = resolveOptions(context.options[0]);
    const missing = missingRequiredOptions(options, ["denyList", "classAttributes"]);

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

    const denied = new Set(options.denyList);
    const classAttributes = new Set(options.classAttributes);
    const classListAttributes = new Set(options.classListAttributes);
    const classHelpers = new Set(options.classHelpers);
    const reportedSites = new Set<string>();

    function reportDenied(sites: ReturnType<typeof classTokenSitesFromExpression>): void {
      for (const site of sites) {
        const siteKey = `${site.node.range?.[0] ?? "unknown"}:${site.token}`;
        if (denied.has(site.token) && !reportedSites.has(siteKey)) {
          reportedSites.add(siteKey);
          context.report({
            node: site.node,
            messageId: "deniedClassToken",
            data: { token: site.token }
          });
        }
      }
    }

    return {
      JSXAttribute(node): void {
        const attributeName = jsxAttributeName(node.name);
        if (classAttributes.has(attributeName)) {
          const staticValue = staticStringFromJsxAttribute(node);
          if (staticValue !== null) {
            reportDenied(classTokenSitesFromString(node.value ?? node, staticValue));
            return;
          }
          if (node.value?.type === AST_NODE_TYPES.JSXExpressionContainer) {
            reportDenied(classTokenSitesFromExpression(node.value.expression, classHelpers));
          }
          return;
        }

        if (
          classListAttributes.has(attributeName) &&
          node.value?.type === AST_NODE_TYPES.JSXExpressionContainer
        ) {
          reportDenied(classTokenSitesFromExpression(node.value.expression, classHelpers));
        }
      },
      CallExpression(node): void {
        const name = calleeName(node.callee);
        if (name !== null && classHelpers.has(name)) {
          reportDenied(
            node.arguments.flatMap((argument) =>
              classTokenSitesFromExpression(argument, classHelpers)
            )
          );
        }
      }
    };
  }
});
