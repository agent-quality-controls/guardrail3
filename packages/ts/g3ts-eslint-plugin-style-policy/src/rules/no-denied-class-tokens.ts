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

type MessageIds = "missingConfig" | "invalidPattern" | "deniedClassToken";

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
      invalidPattern:
        "style-policy/no-denied-class-tokens has invalid denyPatterns entry `{{pattern}}`: {{reason}}.",
      deniedClassToken:
        "Class token `{{token}}` is denied by style policy rule `{{policy}}`. Use the approved design token or component abstraction instead."
    }
  },
  defaultOptions: [{}],
  create(context) {
    const options = resolveOptions(context.options[0]);
    const missing = missingRequiredOptions(options, ["classAttributes"]);
    if (
      options.denyList.length === 0 &&
      options.denyPrefixes.length === 0 &&
      options.denyPatterns.length === 0
    ) {
      missing.push("denyList|denyPrefixes|denyPatterns");
    }

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
    const denyPrefixes = options.denyPrefixes;
    const denyPatterns = compilePatterns(options.denyPatterns);
    const classAttributes = new Set(options.classAttributes);
    const classListAttributes = new Set(options.classListAttributes);
    const classHelpers = new Set(options.classHelpers);
    const reportedSites = new Set<string>();

    const invalidPattern = denyPatterns.find((pattern) => pattern.error !== null);
    if (invalidPattern !== undefined) {
      return {
        Program(node): void {
          context.report({
            node,
            messageId: "invalidPattern",
            data: {
              pattern: invalidPattern.source,
              reason: invalidPattern.error ?? "unknown regex error"
            }
          });
        }
      };
    }

    function reportDenied(sites: ReturnType<typeof classTokenSitesFromExpression>): void {
      for (const site of sites) {
        const policy = matchingPolicy(site.token, denied, denyPrefixes, denyPatterns);
        const siteKey = `${site.node.range?.[0] ?? "unknown"}:${site.token}:${policy ?? ""}`;
        if (policy !== null && !reportedSites.has(siteKey)) {
          reportedSites.add(siteKey);
          context.report({
            node: site.node,
            messageId: "deniedClassToken",
            data: { token: site.token, policy }
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

interface CompiledPattern {
  source: string;
  regex: RegExp | null;
  error: string | null;
}

function compilePatterns(patterns: readonly string[]): CompiledPattern[] {
  return patterns.map((pattern) => {
    try {
      return { source: pattern, regex: new RegExp(pattern), error: null };
    } catch (error) {
      return {
        source: pattern,
        regex: null,
        error: error instanceof Error ? error.message : String(error)
      };
    }
  });
}

function matchingPolicy(
  token: string,
  denied: ReadonlySet<string>,
  denyPrefixes: readonly string[],
  denyPatterns: readonly CompiledPattern[]
): string | null {
  if (denied.has(token)) {
    return `denyList:${token}`;
  }

  const prefix = denyPrefixes.find((prefix) => token.startsWith(prefix));
  if (prefix !== undefined) {
    return `denyPrefixes:${prefix}`;
  }

  const pattern = denyPatterns.find((pattern) => pattern.regex?.test(token));
  if (pattern !== undefined) {
    return `denyPatterns:${pattern.source}`;
  }

  return null;
}
