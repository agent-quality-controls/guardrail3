import { AST_NODE_TYPES, ESLintUtils } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

import {
  collectImportBindings,
  collectConstantStringBindings,
  collectSimpleAliases,
  isRequireLikeCall,
  resolveStaticStringExpression
} from "../utils/ast-helpers.js";
import { classifyModuleRole } from "../utils/module-role.js";
import {
  astroPipelineOptionsSchema,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";

type MessageIds = "forbiddenImport";

const createRule = ESLintUtils.RuleCreator(
  (name) => `https://github.com/websmasher/guardrail3/tree/main/packages/ts/eslint-plugin-astro-pipeline#${name}`
);

export default createRule<RuleOptionsTuple, MessageIds>({
  name: "no-direct-astro-content-in-routes",
  meta: {
    type: "problem",
    docs: {
      description:
        "Disallow route and endpoint modules from importing Astro content helpers directly."
    },
    schema: astroPipelineOptionsSchema,
    messages: {
      forbiddenImport:
        "Route and endpoint modules must not import {{source}} directly. Use an approved content adapter module instead."
    }
  },
  defaultOptions: [{}],
  create(context) {
    const filename = context.filename;
    const options = resolveOptions(context.options[0]);
    const moduleRole = classifyModuleRole(filename, options);

    if (!moduleRole.isRouteOrEndpoint || moduleRole.isApprovedContentAdapter) {
      return {};
    }

    const importAliases = collectConstantStringBindings(context.sourceCode.ast);
    const imports = collectImportBindings(context.sourceCode.ast);
    const requireAliases = collectSimpleAliases(context.sourceCode.ast);

    function reportIfAstroContentImport(
      node: TSESTree.ImportDeclaration | TSESTree.ExportNamedDeclaration | TSESTree.ExportAllDeclaration
    ): void {
      const source = node.source?.value;

      if (isTypeOnlyAstroContentEdge(node)) {
        return;
      }

      if (source === "astro:content") {
        context.report({
          node,
          messageId: "forbiddenImport",
          data: {
            source: "astro:content"
          }
        });
      }
    }

    return {
      ImportDeclaration(node): void {
        reportIfAstroContentImport(node);
      },
      ExportNamedDeclaration(node): void {
        reportIfAstroContentImport(node);
      },
      ExportAllDeclaration(node): void {
        reportIfAstroContentImport(node);
      },
      ImportExpression(node): void {
        if (resolveStaticStringExpression(node.source, importAliases) === "astro:content") {
          context.report({
            node,
            messageId: "forbiddenImport",
            data: {
              source: "astro:content"
            }
          });
        }
      },
      CallExpression(node): void {
        if (
          isRequireLikeCall(
            node,
            imports,
            requireAliases,
            context.sourceCode.scopeManager ?? null
          ) &&
          node.arguments.length > 0 &&
          node.arguments[0]?.type !== AST_NODE_TYPES.SpreadElement &&
          resolveStaticStringExpression(node.arguments[0], importAliases) === "astro:content"
        ) {
          context.report({
            node,
            messageId: "forbiddenImport",
            data: {
              source: "astro:content"
            }
          });
        }
      }
    };
  }
});

function isTypeOnlyAstroContentEdge(
  node: TSESTree.ImportDeclaration | TSESTree.ExportNamedDeclaration | TSESTree.ExportAllDeclaration
): boolean {
  if (node.type === AST_NODE_TYPES.ImportDeclaration) {
    return (
      node.importKind === "type" ||
      (node.specifiers.length > 0 &&
        node.specifiers.every(
          (specifier) =>
            specifier.type === AST_NODE_TYPES.ImportSpecifier &&
            specifier.importKind === "type"
        ))
    );
  }

  if (node.type === AST_NODE_TYPES.ExportAllDeclaration) {
    return node.exportKind === "type";
  }

  return (
    node.exportKind === "type" ||
    (node.specifiers.length > 0 &&
      node.specifiers.every(
        (specifier) => "exportKind" in specifier && specifier.exportKind === "type"
      ))
  );
}
