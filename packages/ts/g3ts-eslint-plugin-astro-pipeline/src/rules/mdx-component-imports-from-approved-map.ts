import { AST_NODE_TYPES, ESLintUtils } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";
import { simpleTraverse } from "@typescript-eslint/typescript-estree";

import {
  isRuntimeImportDeclaration,
  sourceMatchesApprovedModule
} from "../utils/approved-module-imports.js";
import {
  collectConstantStringBindings,
  collectImportBindings,
  collectSimpleAliases,
  isRequireLikeCall,
  resolveStaticStringExpression
} from "../utils/ast-helpers.js";
import { describeApprovedMdxComponentSurface } from "../utils/message-surfaces.js";
import { classifyModuleRole } from "../utils/module-role.js";
import {
  astroPipelineOptionsSchema,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";

type MessageIds = "unapprovedMdxImport";

const createRule = ESLintUtils.RuleCreator(
  (name) =>
    `https://github.com/websmasher/guardrail3/tree/main/packages/ts/g3ts-eslint-plugin-astro-pipeline#${name}`
);

export default createRule<RuleOptionsTuple, MessageIds>({
  name: "mdx-component-imports-from-approved-map",
  meta: {
    type: "problem",
    docs: {
      description:
        "Require MDX component imports to come from approved component-map modules."
    },
    schema: astroPipelineOptionsSchema,
    messages: {
      unapprovedMdxImport:
        "MDX import {{source}} is outside {{surface}}. MDX files may use React components only through approved component-map modules so article rendering stays typed and reviewable."
    }
  },
  defaultOptions: [{}],
  create(context) {
    const filename = context.filename;
    const options = resolveOptions(context.options[0]);
    const moduleRole = classifyModuleRole(filename, options);

    if (!moduleRole.isMdxContent) {
      return {};
    }

    function reportUnapproved(
      node: TSESTree.Node,
      source: string | null
    ): void {
      if (sourceMatchesApprovedMdxModule(source)) {
        return;
      }

      context.report({
        node,
        messageId: "unapprovedMdxImport",
        data: {
          source: `\`${source}\``,
          surface: describeApprovedMdxComponentSurface(options)
        }
      });
    }

    function sourceMatchesApprovedMdxModule(source: string | null): boolean {
      return (
        source !== null &&
        sourceMatchesApprovedModule(
          filename,
          source,
          options.approvedMdxComponentModules
        )
      );
    }

    return {
      ImportDeclaration(node): void {
        if (!isRuntimeImportDeclaration(node)) {
          return;
        }

        const source = node.source?.value;

        if (typeof source === "string") {
          reportUnapproved(node, source);
        }
      },
      ExportAllDeclaration(node): void {
        if (node.exportKind === "type") {
          return;
        }

        const source = node.source?.value;

        if (typeof source === "string") {
          reportUnapproved(node, source);
        }
      },
      ExportNamedDeclaration(node): void {
        if (
          node.exportKind === "type" ||
          (node.specifiers.length > 0 &&
            node.specifiers.every(
              (specifier) =>
                "exportKind" in specifier && specifier.exportKind === "type"
            ))
        ) {
          return;
        }

        const source = node.source?.value;

        if (typeof source === "string") {
          reportUnapproved(node, source);
        }
      },
      "Program:exit"(): void {
        const constants = collectConstantStringBindings(context.sourceCode.ast);
        const imports = collectImportBindings(context.sourceCode.ast);
        const requireAliases = collectSimpleAliases(context.sourceCode.ast);
        const scopeManager = context.sourceCode.scopeManager ?? null;

        function inspectNode(node: TSESTree.Node): void {
          if (node.type === AST_NODE_TYPES.ImportExpression) {
            const source = resolveStaticStringExpression(
              node.source,
              constants,
              new Set(),
              scopeManager
            );
            reportUnapproved(node, source);
            return;
          }

          if (
            node.type === AST_NODE_TYPES.CallExpression &&
            isRequireLikeCall(node, imports, requireAliases, scopeManager) &&
            node.arguments.length > 0
          ) {
            const firstArg = node.arguments[0];
            const source =
              firstArg?.type === AST_NODE_TYPES.SpreadElement
                ? null
                : resolveStaticStringExpression(
                    firstArg,
                    constants,
                    new Set(),
                    scopeManager
                  );
            reportUnapproved(node, source);
          }
        }

        simpleTraverse(context.sourceCode.ast, {
          enter(node) {
            inspectNode(node);
          }
        });
      }
    };
  }
});
