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
import { resolveImportedModuleBinding, type ResolvedModuleBinding } from "../utils/module-exports.js";
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
        "{{module}} evaluates MDX at runtime via {{pattern}}. Precompile the MDX into a generated module and import that generated artifact instead. Runtime evaluation creates a second MDX pipeline outside the approved Astro build path."
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

        const modules = collectImportClosure(filename, context.sourceCode.text);
        const findings = modules.flatMap(
          (moduleRecord) =>
            findRuntimeEvalPatterns(
              modules,
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
              pattern: finding.pattern,
              module: finding.modulePath
            }
          });
        }
      }
    };
  }
});

interface RuntimeEvalFinding {
  modulePath: string;
  pattern: string;
}

function findRuntimeEvalPatterns(
  modules: ReturnType<typeof collectImportClosure>,
  program: TSESTree.Program,
  scopeManager: TSESLint.Scope.ScopeManager | null,
  filename: string,
  options: ReturnType<typeof resolveOptions>
): RuntimeEvalFinding[] {
  const moduleRole = classifyModuleRole(filename, options);

  if (moduleRole.isApprovedGeneratedArtifact) {
    return [];
  }

  const imports = collectImportBindings(program);
  const aliases = collectSimpleAliases(program);
  const declaredNames = collectDeclaredNames(program);
  const functionAliases = collectRuntimeFunctionAliases(
    program,
    scopeManager,
    declaredNames
  );
  const findings = new Set<string>();

  findNodes(program, (node) => {
    if (
      node.type === AST_NODE_TYPES.NewExpression &&
      isRuntimeFunctionReference(node.callee, scopeManager, functionAliases, declaredNames)
    ) {
      findings.add(classifyFunctionPattern(node.callee, "new"));
      return;
    }

    if (node.type !== AST_NODE_TYPES.CallExpression) {
      return;
    }

    if (
      isRuntimeFunctionReference(
        node.callee,
        scopeManager,
        functionAliases,
        declaredNames
      )
    ) {
      findings.add(classifyFunctionPattern(node.callee, "call"));
      return;
    }

    const reference = resolveReference(node.callee, imports, aliases);
    const importedBinding = resolveImportedModuleBinding(modules, filename, reference);

    if (importedBinding) {
      const importedPattern = classifyImportedRuntimeEval(modules, importedBinding);

      if (importedPattern) {
        findings.add(importedPattern);
        return;
      }
    }

    if (
      reference?.kind === "import" &&
      reference.source === "@mdx-js/mdx" &&
      (reference.importedName === "evaluate" || reference.importedName === "run")
    ) {
      findings.add(`@mdx-js/mdx ${String(reference.importedName)}`);
      return;
    }

    if (
      reference?.kind === "member" &&
      reference.object.kind === "import" &&
      reference.object.source === "@mdx-js/mdx" &&
      (reference.object.importedName === "*" ||
        reference.object.importedName === "default") &&
      (reference.property === "evaluate" || reference.property === "run")
    ) {
      findings.add(`@mdx-js/mdx ${reference.property}`);
    }
  });

  return [...findings].map((pattern) => ({
    modulePath: filename,
    pattern
  }));
}

function classifyImportedRuntimeEval(
  modules: ReturnType<typeof collectImportClosure>,
  binding: ResolvedModuleBinding
): string | null {
  if (binding.kind === "reference") {
    if (
      binding.reference.kind === "import" &&
      binding.reference.source === "@mdx-js/mdx" &&
      (binding.reference.importedName === "evaluate" ||
        binding.reference.importedName === "run")
    ) {
      return `@mdx-js/mdx ${String(binding.reference.importedName)}`;
    }

    if (
      binding.reference.kind === "member" &&
      binding.reference.object.kind === "import" &&
      binding.reference.object.source === "@mdx-js/mdx" &&
      (binding.reference.object.importedName === "*" ||
        binding.reference.object.importedName === "default") &&
      (binding.reference.property === "evaluate" ||
        binding.reference.property === "run")
    ) {
      return `@mdx-js/mdx ${binding.reference.property}`;
    }

    return null;
  }

  const imports = collectImportBindings(binding.moduleRecord.program);
  const aliases = collectSimpleAliases(binding.moduleRecord.program);
  const declaredNames = collectDeclaredNames(binding.moduleRecord.program);
  const functionAliases = collectRuntimeFunctionAliases(
    binding.moduleRecord.program,
    binding.moduleRecord.scopeManager,
    declaredNames
  );

  if (
    isRuntimeFunctionReference(
      binding.expression,
      binding.moduleRecord.scopeManager,
      functionAliases,
      declaredNames
    )
  ) {
    return classifyFunctionPattern(binding.expression, "call");
  }

  const reference = resolveReference(binding.expression, imports, aliases);
  const nestedBinding = resolveImportedModuleBinding(
    modules,
    binding.moduleRecord.filename,
    reference
  );

  if (nestedBinding) {
    return classifyImportedRuntimeEval(modules, nestedBinding);
  }

  if (
    reference?.kind === "import" &&
    reference.source === "@mdx-js/mdx" &&
    (reference.importedName === "evaluate" || reference.importedName === "run")
  ) {
    return `@mdx-js/mdx ${String(reference.importedName)}`;
  }

  if (
    reference?.kind === "member" &&
    reference.object.kind === "import" &&
    reference.object.source === "@mdx-js/mdx" &&
    (reference.object.importedName === "*" ||
      reference.object.importedName === "default") &&
    (reference.property === "evaluate" || reference.property === "run")
  ) {
    return `@mdx-js/mdx ${reference.property}`;
  }

  return null;
}

function isRuntimeFunctionReference(
  callee: TSESTree.CallExpression["callee"] | TSESTree.NewExpression["callee"],
  scopeManager: TSESLint.Scope.ScopeManager | null,
  functionAliases: Set<string>,
  declaredNames: Set<string>
): boolean {
  if (callee.type === AST_NODE_TYPES.Identifier) {
    return (
      (callee.name === "Function" &&
        (scopeManager != null
          ? isUnresolvedIdentifierReference(scopeManager, callee)
          : !declaredNames.has("Function"))) ||
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
    ["globalThis", "window", "global"].includes(callee.object.name)
  );
}

function collectRuntimeFunctionAliases(
  program: TSESTree.Program,
  scopeManager: TSESLint.Scope.ScopeManager | null,
  declaredNames: Set<string>
): Set<string> {
  const aliases = new Set<string>();
  const globalObjectAliases = new Set(["globalThis", "window", "global"]);

  let changed = true;
  while (changed) {
    changed = false;
    findNodes(program, (node) => {
      if (node.type !== AST_NODE_TYPES.VariableDeclarator || !node.init) {
        return;
      }

      if (node.id.type === AST_NODE_TYPES.ObjectPattern) {
        const nextAliases = collectRuntimeFunctionObjectPatternAliases(
          node.id,
          node.init,
          globalObjectAliases
        );

        for (const aliasName of nextAliases) {
          if (!aliases.has(aliasName)) {
            aliases.add(aliasName);
            changed = true;
          }
        }

        return;
      }

      if (node.id.type !== AST_NODE_TYPES.Identifier || aliases.has(node.id.name)) {
        return;
      }

      if (
        node.init.type === AST_NODE_TYPES.Identifier &&
        globalObjectAliases.has(node.init.name) &&
        !globalObjectAliases.has(node.id.name)
      ) {
        globalObjectAliases.add(node.id.name);
        changed = true;
        return;
      }

      if (
        isRuntimeFunctionAliasInitializer(
          node.init,
          scopeManager,
          declaredNames,
          aliases,
          globalObjectAliases
        )
      ) {
        aliases.add(node.id.name);
        changed = true;
        return;
      }
    });
  }

  return aliases;
}

function collectRuntimeFunctionObjectPatternAliases(
  pattern: TSESTree.ObjectPattern,
  init: TSESTree.Expression,
  globalObjectAliases: Set<string>
): string[] {
  if (
    init.type !== AST_NODE_TYPES.Identifier ||
    !globalObjectAliases.has(init.name)
  ) {
    return [];
  }

  const aliases: string[] = [];

  for (const property of pattern.properties) {
    if (property.type !== AST_NODE_TYPES.Property || property.computed) {
      continue;
    }

    const keyName =
      property.key.type === AST_NODE_TYPES.Identifier
        ? property.key.name
        : property.key.type === AST_NODE_TYPES.Literal &&
            typeof property.key.value === "string"
          ? property.key.value
          : null;

    if (keyName !== "Function") {
      continue;
    }

    if (property.value.type === AST_NODE_TYPES.Identifier) {
      aliases.push(property.value.name);
    }
  }

  return aliases;
}

function isRuntimeFunctionAliasInitializer(
  init: TSESTree.Expression,
  scopeManager: TSESLint.Scope.ScopeManager | null,
  declaredNames: Set<string>,
  aliases: Set<string>,
  globalObjectAliases: Set<string>
): boolean {
  return (
    (init.type === AST_NODE_TYPES.Identifier &&
      ((init.name === "Function" &&
        (scopeManager != null
          ? isUnresolvedIdentifierReference(scopeManager, init)
          : !declaredNames.has("Function"))) ||
        aliases.has(init.name))) ||
    (init.type === AST_NODE_TYPES.MemberExpression &&
      getPropertyName(init) === "Function" &&
      init.object.type === AST_NODE_TYPES.Identifier &&
      globalObjectAliases.has(init.object.name))
  );
}

function collectDeclaredNames(program: TSESTree.Program): Set<string> {
  const names = new Set<string>();

  findNodes(program, (node) => {
    if (
      node.type === AST_NODE_TYPES.VariableDeclarator &&
      node.id.type === AST_NODE_TYPES.Identifier
    ) {
      names.add(node.id.name);
      return;
    }

    if (
      (node.type === AST_NODE_TYPES.FunctionDeclaration ||
        node.type === AST_NODE_TYPES.FunctionExpression ||
        node.type === AST_NODE_TYPES.ArrowFunctionExpression ||
        node.type === AST_NODE_TYPES.ClassDeclaration) &&
      node.id
    ) {
      names.add(node.id.name);
    }

    if (
      node.type === AST_NODE_TYPES.FunctionDeclaration ||
      node.type === AST_NODE_TYPES.FunctionExpression ||
      node.type === AST_NODE_TYPES.ArrowFunctionExpression
    ) {
      for (const param of node.params) {
        collectPatternNames(param, names);
      }
    }
  });

  return names;
}

function collectPatternNames(
  pattern: TSESTree.Node,
  names: Set<string>
): void {
  if (pattern.type === AST_NODE_TYPES.Identifier) {
    names.add(pattern.name);
    return;
  }

  if (pattern.type === AST_NODE_TYPES.AssignmentPattern) {
    collectPatternNames(pattern.left, names);
    return;
  }

  if (pattern.type === AST_NODE_TYPES.RestElement) {
    collectPatternNames(pattern.argument, names);
    return;
  }

  if (pattern.type === AST_NODE_TYPES.ArrayPattern) {
    for (const element of pattern.elements) {
      if (element) {
        collectPatternNames(element, names);
      }
    }
    return;
  }

  if (pattern.type === AST_NODE_TYPES.ObjectPattern) {
    for (const property of pattern.properties) {
      if (property.type === AST_NODE_TYPES.Property) {
        collectPatternNames(property.value, names);
      } else if (property.type === AST_NODE_TYPES.RestElement) {
        collectPatternNames(property.argument, names);
      }
    }
  }
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
