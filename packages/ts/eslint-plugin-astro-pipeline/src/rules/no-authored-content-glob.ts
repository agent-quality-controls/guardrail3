import { AST_NODE_TYPES, ESLintUtils } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

import {
  findNodes,
  getPropertyName,
  getStaticStringValue
} from "../utils/ast-helpers.js";
import { resolvesToAuthoredOrSpecContent } from "../utils/content-source.js";
import { collectImportClosure } from "../utils/import-closure.js";
import { classifyModuleRole } from "../utils/module-role.js";
import {
  astroPipelineOptionsSchema,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";

type MessageIds = "forbiddenGlob";

const createRule = ESLintUtils.RuleCreator(
  (name) => `https://github.com/websmasher/guardrail3/tree/main/packages/ts/eslint-plugin-astro-pipeline#${name}`
);

export default createRule<RuleOptionsTuple, MessageIds>({
  name: "no-authored-content-glob",
  meta: {
    type: "problem",
    docs: {
      description:
        "Disallow route and endpoint import closures from globbing authored or spec content directly."
    },
    schema: astroPipelineOptionsSchema,
    messages: {
      forbiddenGlob:
        "Route or endpoint import closures must not use {{method}} over authored or spec content. Found {{method}} in {{module}} targeting {{target}}."
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

        const findings = collectImportClosure(filename, context.sourceCode.text).flatMap(
          (moduleRecord) => findForbiddenGlobs(moduleRecord.program, moduleRecord.filename, options)
        );

        for (const finding of findings) {
          context.report({
            node: programNode,
            messageId: "forbiddenGlob",
            data: {
              method: finding.method,
              module: finding.modulePath,
              target: finding.target
            }
          });
        }
      }
    };
  }
});

interface ForbiddenGlob {
  method: string;
  modulePath: string;
  target: string;
}

function findForbiddenGlobs(
  program: TSESTree.Program,
  filename: string,
  options: ReturnType<typeof resolveOptions>
): ForbiddenGlob[] {
  const moduleRole = classifyModuleRole(filename, options);

  if (moduleRole.isApprovedContentAdapter) {
    return [];
  }

  const findings: ForbiddenGlob[] = [];
  const globAliases = collectGlobAliases(program);

  findNodes(program, (node) => {
    if (node.type !== AST_NODE_TYPES.CallExpression) {
      return;
    }

    const methodName = getImportMetaGlobMethod(node.callee, globAliases);

    if (!methodName) {
      return;
    }

    const targetLiteral = getStaticStringValue(node.arguments[0] as TSESTree.Expression);

    if (
      !targetLiteral ||
      !resolvesToAuthoredOrSpecContent(targetLiteral, filename, options)
    ) {
      return;
    }

    findings.push({
      method: methodName,
      modulePath: filename,
      target: targetLiteral
    });
  });

  return findings;
}

function getImportMetaGlobMethod(
  callee: TSESTree.CallExpression["callee"],
  globAliases: Map<string, string>
): string | null {
  if (
    callee.type === AST_NODE_TYPES.Identifier &&
    globAliases.has(callee.name)
  ) {
    return globAliases.get(callee.name) ?? null;
  }

  if (callee.type !== AST_NODE_TYPES.MemberExpression) {
    return null;
  }

  if (
    callee.object.type !== AST_NODE_TYPES.MetaProperty ||
    callee.object.meta.name !== "import" ||
    callee.object.property.name !== "meta"
  ) {
    return null;
  }

  const propertyName = getPropertyName(callee);

  if (propertyName === "glob" || propertyName === "globEager") {
    return `import.meta.${propertyName}`;
  }

  return null;
}

function collectGlobAliases(program: TSESTree.Program): Map<string, string> {
  const aliases = new Map<string, string>();

  findNodes(program, (node) => {
    if (
      node.type !== AST_NODE_TYPES.VariableDeclarator ||
      node.id.type !== AST_NODE_TYPES.Identifier ||
      !node.init
    ) {
      return;
    }

    const methodName = getImportMetaGlobMethod(node.init, new Map());

    if (methodName) {
      aliases.set(node.id.name, methodName);
    }
  });

  return aliases;
}
