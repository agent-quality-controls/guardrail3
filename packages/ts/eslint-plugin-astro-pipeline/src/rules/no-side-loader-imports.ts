import { AST_NODE_TYPES, ESLintUtils } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

import { findNodes, listStaticImportSources } from "../utils/ast-helpers.js";
import { collectImportClosure } from "../utils/import-closure.js";
import { classifyModuleRole } from "../utils/module-role.js";
import {
  astroPipelineOptionsSchema,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";
import {
  inferPathPolicyRoot,
  matchesPathPolicy,
  normalizePathFromCwd
} from "../utils/path-policy.js";

type MessageIds = "forbiddenSideLoader";

const createRule = ESLintUtils.RuleCreator(
  (name) => `https://github.com/websmasher/guardrail3/tree/main/packages/ts/eslint-plugin-astro-pipeline#${name}`
);

export default createRule<RuleOptionsTuple, MessageIds>({
  name: "no-side-loader-imports",
  meta: {
    type: "problem",
    docs: {
      description:
        "Disallow route and endpoint modules from importing unapproved side-loader helpers one hop away."
    },
    schema: astroPipelineOptionsSchema,
    messages: {
      forbiddenSideLoader:
        "Route or endpoint modules must not import side-loader helpers outside the approved Astro surfaces. Found {{reason}} in {{module}}."
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

        const findings = collectImportClosure(filename, context.sourceCode.text)
          .filter((moduleRecord) => moduleRecord.importChain.length === 2)
          .flatMap((moduleRecord) =>
            findForbiddenSideLoader(
              moduleRecord.program,
              moduleRecord.filename,
              filename,
              options
            )
          );

        for (const finding of findings) {
          context.report({
            node: programNode,
            messageId: "forbiddenSideLoader",
            data: {
              module: finding.modulePath,
              reason: finding.reason
            }
          });
        }
      }
    };
  }
});

interface ForbiddenSideLoader {
  modulePath: string;
  reason: string;
}

function findForbiddenSideLoader(
  program: TSESTree.Program,
  filename: string,
  routeFilename: string,
  options: ReturnType<typeof resolveOptions>
): ForbiddenSideLoader[] {
  const moduleRole = classifyModuleRole(filename, options);

  if (
    moduleRole.isApprovedContentAdapter ||
    moduleRole.isApprovedLoader ||
    moduleRole.isMdxRuntimeModule ||
    moduleRole.isRouteRegistryModule ||
    moduleRole.isApprovedGeneratedArtifact
  ) {
    return [];
  }

  const cwd = inferPathPolicyRoot(routeFilename);
  const normalizedFilename = normalizePathFromCwd(filename, cwd);

  if (
    normalizedFilename.startsWith("../") ||
    (!normalizedFilename.startsWith("src/") && !normalizedFilename.startsWith("app/"))
  ) {
    return [
      {
        modulePath: filename,
        reason: "cross-root helper import"
      }
    ];
  }

  if (
    matchesPathPolicy(normalizedFilename, options.authoredContentGlobs, cwd) ||
    matchesPathPolicy(normalizedFilename, options.specContentGlobs, cwd)
  ) {
    return [
      {
        modulePath: filename,
        reason: "direct authored/spec content import"
      }
    ];
  }

  if (programImportsAstroContent(program)) {
    return [
      {
        modulePath: filename,
        reason: "helper import of astro:content"
      }
    ];
  }

  return [];
}

function programImportsAstroContent(program: TSESTree.Program): boolean {
  if (listStaticImportSources(program).some((source) => source === "astro:content")) {
    return true;
  }

  let found = false;
  findNodes(program, (node) => {
    if (
      node.type === AST_NODE_TYPES.ImportExpression &&
      node.source.type === AST_NODE_TYPES.Literal &&
      node.source.value === "astro:content"
    ) {
      found = true;
    }
  });

  return found;
}
