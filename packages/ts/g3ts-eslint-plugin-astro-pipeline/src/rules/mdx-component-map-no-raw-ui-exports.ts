import { AST_NODE_TYPES, ESLintUtils } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

import { collectImportBindings } from "../utils/ast-helpers.js";
import { classifyModuleRole } from "../utils/module-role.js";
import {
  astroPipelineOptionsSchema,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";
import { matchesPathPolicy, resolvePathLike } from "../utils/path-policy.js";

type MessageIds = "missingRawUiGlobs" | "rawUiExport";

const createRule = ESLintUtils.RuleCreator(
  (name) =>
    `https://github.com/websmasher/guardrail3/tree/main/packages/ts/g3ts-eslint-plugin-astro-pipeline#${name}`
);

export default createRule<RuleOptionsTuple, MessageIds>({
  name: "mdx-component-map-no-raw-ui-exports",
  meta: {
    type: "problem",
    docs: {
      description:
        "Forbid approved MDX component-map modules from exporting raw UI components directly."
    },
    schema: astroPipelineOptionsSchema,
    messages: {
      missingRawUiGlobs:
        "`rawUiModuleGlobs` is empty. Configure the raw UI module surfaces that component maps may wrap but must not re-export directly.",
      rawUiExport:
        "MDX component-map export `{{name}}` exposes raw UI from `{{source}}`. Export a local validated wrapper that parses props with Zod instead."
    }
  },
  defaultOptions: [{}],
  create(context) {
    const filename = context.filename;
    const options = resolveOptions(context.options[0]);
    const moduleRole = classifyModuleRole(filename, options);

    if (
      !matchesPathPolicy(filename, options.approvedMdxComponentModules) &&
      !moduleRole.isMdxRuntimeModule
    ) {
      return {};
    }

    function rawUiSource(source: string): boolean {
      return matchesPathPolicy(resolvePathLike(filename, source), options.rawUiModuleGlobs);
    }

    function report(node: TSESTree.Node, name: string, source: string): void {
      context.report({
        node,
        messageId: "rawUiExport",
        data: { name, source }
      });
    }

    return {
      Program(node): void {
        if (options.rawUiModuleGlobs.length === 0) {
          context.report({ node, messageId: "missingRawUiGlobs" });
        }
      },
      ExportAllDeclaration(node): void {
        const source = node.source?.value;
        if (typeof source === "string" && rawUiSource(source)) {
          report(node, "*", source);
        }
      },
      ExportNamedDeclaration(node): void {
        const source = node.source?.value;
        if (typeof source === "string" && rawUiSource(source)) {
          for (const specifier of node.specifiers) {
            report(node, specifierName(specifier.exported), source);
          }
          if (node.specifiers.length === 0) {
            report(node, "declaration", source);
          }
          return;
        }

        if (!node.source) {
          const imports = collectImportBindings(context.sourceCode.ast);
          for (const specifier of node.specifiers) {
            const localName = specifierName(specifier.local);
            const binding = imports.get(localName);
            if (binding && rawUiSource(binding.source)) {
              report(specifier, specifierName(specifier.exported), binding.source);
            }
          }
        }
      },
      "Program:exit"(): void {
        const imports = collectImportBindings(context.sourceCode.ast);
        for (const statement of context.sourceCode.ast.body) {
          if (statement.type !== AST_NODE_TYPES.ExportNamedDeclaration) {
            continue;
          }
          if (statement.declaration?.type !== AST_NODE_TYPES.VariableDeclaration) {
            continue;
          }
          for (const declaration of statement.declaration.declarations) {
            if (
              declaration.id.type === AST_NODE_TYPES.Identifier &&
              declaration.init?.type === AST_NODE_TYPES.Identifier
            ) {
              const binding = imports.get(declaration.init.name);
              if (binding && rawUiSource(binding.source)) {
                report(declaration, declaration.id.name, binding.source);
              }
            }
          }
        }
      }
    };
  }
});

function specifierName(
  node: TSESTree.ExportSpecifier["local"] | TSESTree.ExportSpecifier["exported"]
): string {
  return node.type === AST_NODE_TYPES.Identifier ? node.name : String(node.value);
}
