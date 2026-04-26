import { ESLintUtils } from "@typescript-eslint/utils";

import { hasRuntimeImportFromApprovedModule } from "../utils/approved-module-imports.js";
import { describeApprovedContentAdapterSurface } from "../utils/message-surfaces.js";
import { classifyModuleRole } from "../utils/module-role.js";
import {
  astroPipelineOptionsSchema,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";
import { matchesFileGlobs, resolvePathLike } from "../utils/path-policy.js";

type MessageIds = "missingAdapter";

const createRule = ESLintUtils.RuleCreator(
  (name) =>
    `https://github.com/websmasher/guardrail3/tree/main/packages/ts/g3ts-eslint-plugin-astro-pipeline#${name}`
);

export default createRule<RuleOptionsTuple, MessageIds>({
  name: "require-approved-content-adapter-in-routes",
  meta: {
    type: "problem",
    docs: {
      description:
        "Require public Astro page routes to import an approved content adapter module."
    },
    schema: astroPipelineOptionsSchema,
    messages: {
      missingAdapter:
        "{{module}} does not import {{surface}}. Public Astro page routes must load page data through the approved content adapter so authored copy stays in Astro content collections."
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

    return {
      "Program:exit"(programNode): void {
        if (
          hasRuntimeImportFromApprovedModule(
            {
              filename,
              sourceCode: context.sourceCode
            },
            options.approvedContentAdapterModules
          )
        ) {
          return;
        }

        context.report({
          node: programNode,
          messageId: "missingAdapter",
          data: {
            module: filename,
            surface: describeApprovedContentAdapterSurface(options)
          }
        });
      }
    };
  }
});
