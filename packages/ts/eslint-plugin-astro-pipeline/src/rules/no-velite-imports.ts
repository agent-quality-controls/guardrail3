import { AST_NODE_TYPES, ESLintUtils } from "@typescript-eslint/utils";

import {
  collectConstantStringBindings,
  collectImportBindings,
  collectSimpleAliases,
  findNodes,
  getStaticStringValue,
  isRequireLikeCall,
  listStaticImportSources,
  resolveStaticStringExpression
} from "../utils/ast-helpers.js";
import { collectImportClosure } from "../utils/import-closure.js";
import { describeApprovedContentAdapterSurface } from "../utils/message-surfaces.js";
import { classifyModuleRole } from "../utils/module-role.js";
import {
  astroPipelineOptionsSchema,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";
import { inferPathPolicyRoot, normalizePathFromCwd } from "../utils/path-policy.js";

type MessageIds = "forbiddenVeliteImport";

const createRule = ESLintUtils.RuleCreator(
  (name) => `https://github.com/websmasher/guardrail3/tree/main/packages/ts/eslint-plugin-astro-pipeline#${name}`
);

export default createRule<RuleOptionsTuple, MessageIds>({
  name: "no-velite-imports",
  meta: {
    type: "problem",
    docs: {
      description:
        "Disallow route and endpoint import closures from reaching Velite package or .velite outputs."
    },
    schema: astroPipelineOptionsSchema,
    messages: {
      forbiddenVeliteImport:
        "{{module}} reaches Velite surface {{target}} in this route import closure. Remove Velite from this Astro app and load page content through {{surface}} instead. Astro apps must not keep a parallel Velite content pipeline alive."
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
        }).find((moduleRecord) => moduleUsesVelite(moduleRecord, filename));

        if (!offendingModule) {
          return;
        }

        context.report({
          node: programNode,
          messageId: "forbiddenVeliteImport",
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

function moduleUsesVelite(
  moduleRecord: ReturnType<typeof collectImportClosure>[number],
  routeFilename: string
): boolean {
  const cwd = inferPathPolicyRoot(routeFilename);
  const normalizedModulePath = normalizePathFromCwd(moduleRecord.filename, cwd);

  if (isVelitePath(normalizedModulePath)) {
    return true;
  }

  const staticSources = listStaticImportSources(moduleRecord.program);
  if (staticSources.some(isVeliteSpecifier)) {
    return true;
  }

  const constants = collectConstantStringBindings(moduleRecord.program);
  const imports = collectImportBindings(moduleRecord.program);
  const aliases = collectSimpleAliases(moduleRecord.program);
  let found = false;

  findNodes(moduleRecord.program, (node) => {
    if (found) {
      return;
    }

    if (node.type === AST_NODE_TYPES.ImportExpression) {
      found = isVeliteSpecifier(resolveStaticStringExpression(node.source, constants));
      return;
    }

    if (
      node.type === AST_NODE_TYPES.CallExpression &&
      isRequireLikeCall(node, imports, aliases, moduleRecord.scopeManager)
    ) {
      const firstArg = node.arguments[0];
      if (!firstArg || firstArg.type === AST_NODE_TYPES.SpreadElement) {
        return;
      }

      found = isVeliteSpecifier(getStaticStringValue(firstArg));
    }
  });

  return found;
}

function isVeliteSpecifier(value: string | null | undefined): boolean {
  if (!value) {
    return false;
  }

  return value === "velite" || isVelitePath(value);
}

function isVelitePath(value: string): boolean {
  return (
    value.includes(".velite") ||
    value.includes("velite.config.") ||
    value.startsWith(".velite/") ||
    value.endsWith("/.velite")
  );
}
