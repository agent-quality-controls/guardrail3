import { ESLintUtils } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

import { createApprovedModuleUseTracker } from "../utils/approved-module-imports.js";
import {
  describeApprovedContentAdapterSurface,
  describeApprovedMetadataHelperSurface
} from "../utils/message-surfaces.js";
import { classifyModuleRole } from "../utils/module-role.js";
import {
  astroPipelineOptionsSchema,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";

type MessageIds = "missingMetadataHelper";

const createRule = ESLintUtils.RuleCreator(
  (name) =>
    `https://github.com/websmasher/guardrail3/tree/main/packages/ts/g3ts-eslint-plugin-astro-pipeline#${name}`
);

export default createRule<RuleOptionsTuple, MessageIds>({
  name: "require-approved-metadata-helper-in-routes",
  meta: {
    type: "problem",
    docs: {
      description:
        "Require public Astro page routes to use approved metadata helper surfaces."
    },
    schema: astroPipelineOptionsSchema,
    messages: {
      missingMetadataHelper:
        "{{module}} does not import {{metadataSurface}} or {{adapterSurface}}. Public routes must derive metadata through approved typed surfaces, not hardcoded page/layout defaults."
    }
  },
  defaultOptions: [{}],
  create(context) {
    const filename = context.filename;
    const options = resolveOptions(context.options[0]);
    const moduleRole = classifyModuleRole(filename, options);

    if (
      !moduleRole.isRoute ||
      moduleRole.isEndpoint ||
      moduleRole.isApprovedContentAdapter
    ) {
      return {};
    }

    const metadataTracker = createApprovedModuleUseTracker(
      filename,
      options.approvedMetadataHelperModules
    );
    const adapterTracker = createApprovedModuleUseTracker(
      filename,
      options.approvedContentAdapterModules
    );

    return {
      ImportDeclaration(node): void {
        metadataTracker.trackImportDeclaration(node);
        adapterTracker.trackImportDeclaration(node);
      },
      CallExpression(node): void {
        const ancestors = context.sourceCode.getAncestors(
          node as never
        ) as unknown as readonly TSESTree.Node[];
        metadataTracker.trackCallExpression(node, ancestors);
        adapterTracker.trackCallExpression(node, ancestors);
      },
      VariableDeclarator(node): void {
        const ancestors = context.sourceCode.getAncestors(
          node as never
        ) as unknown as readonly TSESTree.Node[];
        metadataTracker.trackVariableDeclarator(node, ancestors);
        adapterTracker.trackVariableDeclarator(node, ancestors);
      },
      JSXExpressionContainer(node): void {
        metadataTracker.trackJsxExpressionContainer(node);
        adapterTracker.trackJsxExpressionContainer(node);
      },
      "Program:exit"(programNode): void {
        if (metadataTracker.hasUsedImport() || adapterTracker.hasUsedImport()) {
          return;
        }

        context.report({
          node: programNode,
          messageId: "missingMetadataHelper",
          data: {
            module: filename,
            metadataSurface: describeApprovedMetadataHelperSurface(options),
            adapterSurface: describeApprovedContentAdapterSurface(options)
          }
        });
      }
    };
  }
});
