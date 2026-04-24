import { ESLintUtils } from "@typescript-eslint/utils";

import { collectImportClosure } from "../utils/import-closure.js";
import { describeApprovedContentAdapterSurface } from "../utils/message-surfaces.js";
import { classifyModuleRole } from "../utils/module-role.js";
import {
  astroPipelineOptionsSchema,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";
import { matchesFileGlobs } from "../utils/path-policy.js";

type MessageIds = "forbiddenContentDataModule";

const createRule = ESLintUtils.RuleCreator(
  (name) => `https://github.com/websmasher/guardrail3/tree/main/packages/ts/eslint-plugin-astro-pipeline#${name}`
);

export default createRule<RuleOptionsTuple, MessageIds>({
  name: "no-content-data-modules-in-routes",
  meta: {
    type: "problem",
    docs: {
      description:
        "Disallow route and endpoint import closures from sourcing page copy through *.data.* modules."
    },
    schema: astroPipelineOptionsSchema,
    messages: {
      forbiddenContentDataModule:
        "{{module}} reaches content data module {{target}} in this route import closure. Move that page content into {{surface}} and pass typed props from the route instead. Public Astro pages must not source copy from ad hoc `*.data.*` modules."
    }
  },
  defaultOptions: [{}],
  create(context) {
    return {
      "Program:exit"(programNode): void {
        const filename = context.filename;
        const options = resolveOptions(context.options[0]);
        const moduleRole = classifyModuleRole(filename, options);

        if (!moduleRole.isRouteOrEndpoint) {
          return;
        }

        const offendingModule = collectImportClosure(filename, context.sourceCode.text, {
          program: context.sourceCode.ast,
          scopeManager: context.sourceCode.scopeManager ?? null
        }).find((moduleRecord) => {
          if (moduleRecord.importChain.length <= 1) {
            return false;
          }

          return matchesFileGlobs(moduleRecord.filename, options.contentDataModuleGlobs);
        });

        if (!offendingModule) {
          return;
        }

        context.report({
          node: programNode,
          messageId: "forbiddenContentDataModule",
          data: {
            module: filename,
            target: offendingModule.filename,
            surface: describeApprovedContentAdapterSurface(options)
          }
        });
      }
    };
  }
});
