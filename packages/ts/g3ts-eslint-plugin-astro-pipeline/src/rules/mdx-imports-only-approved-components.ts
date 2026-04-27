import { AST_NODE_TYPES, ESLintUtils } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

import { sourceMatchesApprovedModule } from "../utils/approved-module-imports.js";
import { describeApprovedMdxComponentSurface } from "../utils/message-surfaces.js";
import { classifyModuleRole } from "../utils/module-role.js";
import {
  astroPipelineOptionsSchema,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";

type MessageIds =
  | "missingApprovedNames"
  | "unapprovedMdxImport"
  | "unapprovedMdxImportName"
  | "forbiddenDefaultImport"
  | "forbiddenNamespaceImport";

const createRule = ESLintUtils.RuleCreator(
  (name) =>
    `https://github.com/websmasher/guardrail3/tree/main/packages/ts/g3ts-eslint-plugin-astro-pipeline#${name}`
);

export default createRule<RuleOptionsTuple, MessageIds>({
  name: "mdx-imports-only-approved-components",
  meta: {
    type: "problem",
    docs: {
      description:
        "Require MDX files to import only explicitly approved validated component-map exports."
    },
    schema: astroPipelineOptionsSchema,
    messages: {
      missingApprovedNames:
        "`approvedMdxComponentNames` is empty. Configure the exact validated MDX component names this content app may import from {{surface}}.",
      unapprovedMdxImport:
        "MDX import {{source}} is outside {{surface}}.",
      unapprovedMdxImportName:
        "MDX import `{{name}}` is not listed in `approvedMdxComponentNames`. Add the validated wrapper name to the rule options or stop importing it from MDX.",
      forbiddenDefaultImport:
        "Default imports from MDX component-map modules are forbidden. Export and import a named validated MDX component instead.",
      forbiddenNamespaceImport:
        "Namespace imports from MDX component-map modules are forbidden. Import explicit validated component names instead."
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

    const approvedNames = new Set(options.approvedMdxComponentNames);

    function sourceIsApproved(source: string): boolean {
      return sourceMatchesApprovedModule(
        filename,
        source,
        options.approvedMdxComponentModules
      );
    }

    function reportMissingNames(node: TSESTree.Node): boolean {
      if (approvedNames.size > 0) {
        return false;
      }

      context.report({
        node,
        messageId: "missingApprovedNames",
        data: {
          surface: describeApprovedMdxComponentSurface(options)
        }
      });
      return true;
    }

    function importedSpecifierName(specifier: TSESTree.ImportSpecifier): string {
      return specifier.imported.type === AST_NODE_TYPES.Identifier
        ? specifier.imported.name
        : String(specifier.imported.value);
    }

    return {
      ImportDeclaration(node): void {
        if (node.importKind === "type") {
          context.report({
            node,
            messageId: "unapprovedMdxImportName",
            data: { name: "type-only import" }
          });
          return;
        }

        const source = node.source?.value;
        if (typeof source !== "string") {
          return;
        }

        if (!sourceIsApproved(source)) {
          context.report({
            node,
            messageId: "unapprovedMdxImport",
            data: {
              source: `\`${source}\``,
              surface: describeApprovedMdxComponentSurface(options)
            }
          });
          return;
        }

        if (reportMissingNames(node)) {
          return;
        }

        for (const specifier of node.specifiers) {
          if (specifier.type === AST_NODE_TYPES.ImportDefaultSpecifier) {
            context.report({ node: specifier, messageId: "forbiddenDefaultImport" });
            continue;
          }

          if (specifier.type === AST_NODE_TYPES.ImportNamespaceSpecifier) {
            context.report({ node: specifier, messageId: "forbiddenNamespaceImport" });
            continue;
          }

          const importedName = importedSpecifierName(specifier);
          if (!approvedNames.has(importedName)) {
            context.report({
              node: specifier,
              messageId: "unapprovedMdxImportName",
              data: { name: importedName }
            });
          }
        }
      }
    };
  }
});
