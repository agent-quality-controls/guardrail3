import { ESLintUtils } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

import { createApprovedModuleUseTracker } from "../utils/approved-module-imports.js";
import { describeApprovedJsonLdHelperSurface } from "../utils/message-surfaces.js";
import { classifyModuleRole } from "../utils/module-role.js";
import {
  astroPipelineOptionsSchema,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";

type MessageIds = "missingJsonLdHelper";

const createRule = ESLintUtils.RuleCreator(
  (name) =>
    `https://github.com/websmasher/guardrail3/tree/main/packages/ts/g3ts-eslint-plugin-astro-pipeline#${name}`
);

export default createRule<RuleOptionsTuple, MessageIds>({
  name: "require-approved-json-ld-helper-in-routes",
  meta: {
    type: "problem",
    docs: {
      description:
        "Require public Astro page routes to use approved JSON-LD helper surfaces."
    },
    schema: astroPipelineOptionsSchema,
    messages: {
      missingJsonLdHelper:
        "{{module}} does not import {{surface}}. Public routes must render structured data through approved typed JSON-LD helpers, not string-built JSON blobs."
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

    const jsonLdTracker = createApprovedModuleUseTracker(
      filename,
      options.approvedJsonLdHelperModules
    );

    return {
      ImportDeclaration(node): void {
        jsonLdTracker.trackImportDeclaration(node);
      },
      CallExpression(node): void {
        const ancestors = context.sourceCode.getAncestors(
          node as never
        ) as unknown as readonly TSESTree.Node[];
        jsonLdTracker.trackCallExpression(node, ancestors);
      },
      VariableDeclarator(node): void {
        const ancestors = context.sourceCode.getAncestors(
          node as never
        ) as unknown as readonly TSESTree.Node[];
        jsonLdTracker.trackVariableDeclarator(node, ancestors);
      },
      JSXExpressionContainer(node): void {
        jsonLdTracker.trackJsxExpressionContainer(node);
      },
      "Program:exit"(programNode): void {
        if (jsonLdTracker.hasUsedImport()) {
          return;
        }

        context.report({
          node: programNode,
          messageId: "missingJsonLdHelper",
          data: {
            module: filename,
            surface: describeApprovedJsonLdHelperSurface(options)
          }
        });
      }
    };
  }
});
