import { AST_NODE_TYPES, ESLintUtils } from "@typescript-eslint/utils";
import type { TSESLint, TSESTree } from "@typescript-eslint/utils";

import {
  collectImportBindings,
  collectSimpleAliases,
  findNodes,
  getPropertyName,
  isUnresolvedIdentifierReference,
  resolveReference
} from "../utils/ast-helpers.js";
import { collectImportClosure } from "../utils/import-closure.js";
import { classifyModuleRole } from "../utils/module-role.js";
import {
  astroPipelineOptionsSchema,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";

type MessageIds = "runtimeEval";

const createRule = ESLintUtils.RuleCreator(
  (name) => `https://github.com/websmasher/guardrail3/tree/main/packages/ts/eslint-plugin-astro-pipeline#${name}`
);

export default createRule<RuleOptionsTuple, MessageIds>({
  name: "no-runtime-mdx-eval",
  meta: {
    type: "problem",
    docs: {
      description:
        "Disallow runtime MDX evaluation bridges such as new Function and @mdx-js/mdx evaluate or run."
    },
    schema: astroPipelineOptionsSchema,
    messages: {
      runtimeEval:
        "Runtime MDX evaluation is forbidden. Found {{pattern}} in {{module}}."
    }
  },
  defaultOptions: [{}],
  create(context) {
    return {
      "Program:exit"(programNode): void {
        const filename = context.filename;
        const options = resolveOptions(context.options[0]);
        const moduleRole = classifyModuleRole(filename, options);

        if (moduleRole.isApprovedGeneratedArtifact) {
          return;
        }

        if (
          options.mdxRuntimeModuleGlobs.length > 0 &&
          !moduleRole.isMdxRuntimeModule
        ) {
          return;
        }

        const findings = collectImportClosure(filename, context.sourceCode.text).flatMap(
          (moduleRecord) =>
            findRuntimeEvalPatterns(
              moduleRecord.program,
              moduleRecord.scopeManager,
              moduleRecord.filename,
              options
            )
        );

        for (const finding of findings) {
          context.report({
            node: programNode,
            messageId: "runtimeEval",
            data: {
              pattern: finding,
              module: filename
            }
          });
        }
      }
    };
  }
});

function findRuntimeEvalPatterns(
  program: TSESTree.Program,
  scopeManager: TSESLint.Scope.ScopeManager | null,
  filename: string,
  options: ReturnType<typeof resolveOptions>
): string[] {
  const moduleRole = classifyModuleRole(filename, options);

  if (moduleRole.isApprovedGeneratedArtifact) {
    return [];
  }

  const imports = collectImportBindings(program);
  const aliases = collectSimpleAliases(program);
  const functionAliases = collectRuntimeFunctionAliases(program, scopeManager);
  const findings = new Set<string>();

  findNodes(program, (node) => {
    if (
      node.type === AST_NODE_TYPES.NewExpression &&
      isRuntimeFunctionReference(node.callee, scopeManager, functionAliases)
    ) {
      findings.add(classifyFunctionPattern(node.callee, "new"));
      return;
    }

    if (node.type !== AST_NODE_TYPES.CallExpression) {
      return;
    }

    if (isRuntimeFunctionReference(node.callee, scopeManager, functionAliases)) {
      findings.add(classifyFunctionPattern(node.callee, "call"));
      return;
    }

    const reference = resolveReference(node.callee, imports, aliases);

    if (
      reference?.kind === "import" &&
      reference.source === "@mdx-js/mdx" &&
      (reference.importedName === "evaluate" || reference.importedName === "run")
    ) {
      findings.add(`@mdx-js/mdx ${String(reference.importedName)}`);
    }
  });

  return [...findings];
}

function isRuntimeFunctionReference(
  callee: TSESTree.CallExpression["callee"] | TSESTree.NewExpression["callee"],
  scopeManager: TSESLint.Scope.ScopeManager | null,
  functionAliases: Set<string>
): boolean {
  if (callee.type === AST_NODE_TYPES.Identifier) {
    return (
      (callee.name === "Function" &&
        isUnresolvedIdentifierReference(scopeManager, callee)) ||
      functionAliases.has(callee.name)
    );
  }

  if (callee.type !== AST_NODE_TYPES.MemberExpression) {
    return false;
  }

  const propertyName = getPropertyName(callee);

  if (propertyName !== "Function" || callee.object.type !== AST_NODE_TYPES.Identifier) {
    return false;
  }

  return (
    ["globalThis", "window", "global"].includes(callee.object.name) &&
    isUnresolvedIdentifierReference(scopeManager, callee.object)
  );
}

function collectRuntimeFunctionAliases(
  program: TSESTree.Program,
  scopeManager: TSESLint.Scope.ScopeManager | null
): Set<string> {
  const aliases = new Set<string>();

  findNodes(program, (node) => {
    if (node.type !== AST_NODE_TYPES.VariableDeclarator || !node.init) {
      return;
    }

    if (node.id.type !== AST_NODE_TYPES.Identifier) {
      return;
    }

    if (
      node.init.type === AST_NODE_TYPES.Identifier &&
      node.init.name === "Function" &&
      isUnresolvedIdentifierReference(scopeManager, node.init)
    ) {
      aliases.add(node.id.name);
      return;
    }

    if (
      node.init.type === AST_NODE_TYPES.MemberExpression &&
      getPropertyName(node.init) === "Function" &&
      node.init.object.type === AST_NODE_TYPES.Identifier &&
      ["globalThis", "window", "global"].includes(node.init.object.name) &&
      isUnresolvedIdentifierReference(scopeManager, node.init.object)
    ) {
      aliases.add(node.id.name);
    }
  });

  return aliases;
}

function classifyFunctionPattern(
  callee: TSESTree.CallExpression["callee"] | TSESTree.NewExpression["callee"],
  invocationKind: "call" | "new"
): string {
  if (callee.type === AST_NODE_TYPES.Identifier) {
    return invocationKind === "new" ? "new Function" : "Function";
  }

  if (callee.type !== AST_NODE_TYPES.MemberExpression) {
    return invocationKind === "new" ? "new Function" : "Function";
  }

  const objectName =
    callee.object.type === AST_NODE_TYPES.Identifier ? callee.object.name : "globalThis";

  return invocationKind === "new"
    ? `new ${objectName}.Function`
    : `${objectName}.Function`;
}
