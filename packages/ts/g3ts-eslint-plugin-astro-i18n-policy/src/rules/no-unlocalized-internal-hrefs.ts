import { AST_NODE_TYPES, ESLintUtils } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

import {
  jsxAttributeName,
  jsxElementName,
  staticStringFromExpression,
  staticStringFromJsxAttribute
} from "../utils/ast.js";
import {
  astroI18nPolicyOptionsSchema,
  missingRequiredOptions,
  normalizeRoutePath,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";

type MessageIds = "missingConfig" | "unlocalizedHref";

const createRule = ESLintUtils.RuleCreator(
  (name) =>
    `https://github.com/websmasher/guardrail3/tree/main/packages/ts/g3ts-eslint-plugin-astro-i18n-policy#${name}`
);

export default createRule<RuleOptionsTuple, MessageIds>({
  name: "no-unlocalized-internal-hrefs",
  meta: {
    type: "problem",
    docs: {
      description:
        "Require durable internal content links to use configured locale prefixes or approved localized helpers."
    },
    schema: astroI18nPolicyOptionsSchema,
    messages: {
      missingConfig:
        "astro-i18n-policy/no-unlocalized-internal-hrefs requires non-empty options: {{missing}}.",
      unlocalizedHref:
        "Internal content link `{{href}}` matches content route prefixes {{prefixes}} but has no locale prefix from {{locales}}. Use an approved localized helper/component: {{helpers}}."
    }
  },
  defaultOptions: [{}],
  create(context) {
    const options = resolveOptions(context.options[0]);
    const missing = missingRequiredOptions(options, [
      "locales",
      "contentRoutePrefixes",
      "approvedInternalLinkHelpers",
      "approvedLocalizedLinkComponents"
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

    const allowedComponents = new Set(options.approvedLocalizedLinkComponents);
    const approvedHelpers = new Set(options.approvedInternalLinkHelpers);

    function reportIfUnlocalized(node: TSESTree.Node, rawValue: string): void {
      if (!requiresLocalePrefix(rawValue)) {
        return;
      }

      context.report({
        node,
        messageId: "unlocalizedHref",
        data: {
          href: rawValue,
          prefixes: options.contentRoutePrefixes.join(", "),
          locales: options.locales.join(", "),
          helpers: [
            ...options.approvedInternalLinkHelpers,
            ...options.approvedLocalizedLinkComponents
          ].join(", ")
        }
      });
    }

    function requiresLocalePrefix(rawValue: string): boolean {
      if (!options.requireLocalePrefixForContentRoutes) {
        return false;
      }

      if (isIgnoredLink(rawValue)) {
        return false;
      }

      const path = normalizeRoutePath(rawValue.split(/[?#]/, 1)[0] ?? rawValue);

      if (options.allowedUnprefixedRoutes.includes(path)) {
        return false;
      }

      if (hasLocalePrefix(path)) {
        return false;
      }

      return options.contentRoutePrefixes.some(
        (prefix) => path === prefix || path.startsWith(`${prefix}/`)
      );
    }

    function hasLocalePrefix(path: string): boolean {
      return options.locales.some((locale) => {
        const prefix = normalizeRoutePath(locale);

        return path === prefix || path.startsWith(`${prefix}/`);
      });
    }

    return {
      JSXOpeningElement(node): void {
        const componentName = jsxElementName(node.name);

        if (componentName && allowedComponents.has(componentName)) {
          return;
        }

        for (const attribute of node.attributes) {
          if (attribute.type !== AST_NODE_TYPES.JSXAttribute) {
            continue;
          }

          const attrName = jsxAttributeName(attribute.name);
          if (attrName !== "href" && attrName !== "to") {
            continue;
          }

          const value = staticStringFromJsxAttribute(attribute);
          if (value !== null) {
            reportIfUnlocalized(attribute, value);
          }
        }
      },
      CallExpression(node): void {
        const helperName = calleeName(node.callee);

        if (!helperName || approvedHelpers.has(helperName)) {
          return;
        }

        const firstArg = node.arguments[0];
        if (!firstArg || firstArg.type === AST_NODE_TYPES.SpreadElement) {
          return;
        }

        const value = staticStringFromExpression(firstArg);
        if (value !== null) {
          reportIfUnlocalized(node, value);
        }
      }
    };
  }
});

function isIgnoredLink(value: string): boolean {
  return (
    value === "" ||
    value.startsWith("#") ||
    value.startsWith("//") ||
    /^[a-z][a-z0-9+.-]*:/i.test(value)
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
