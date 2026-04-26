import { AST_NODE_TYPES } from "@typescript-eslint/utils";
import type { TSESLint, TSESTree } from "@typescript-eslint/utils";
import { simpleTraverse } from "@typescript-eslint/typescript-estree";

import {
  collectConstantStringBindings,
  collectImportBindings,
  collectSimpleAliases,
  isRequireLikeCall,
  resolveStaticStringExpression
} from "./ast-helpers.js";
import { matchesFileGlobs, resolvePathLike } from "./path-policy.js";

export function sourceMatchesApprovedModule(
  importerFilename: string,
  rawSource: string,
  approvedModules: readonly string[]
): boolean {
  if (approvedModules.length === 0) {
    return false;
  }

  const resolvedPath = resolvePathLike(importerFilename, rawSource);

  return moduleCandidatePaths(resolvedPath).some((candidatePath) =>
    matchesFileGlobs(candidatePath, approvedModules)
  );
}

export function hasRuntimeImportFromApprovedModule(
  context: Readonly<{
    filename: string;
    sourceCode: {
      ast: TSESTree.Program;
      scopeManager?: TSESLint.Scope.ScopeManager | null;
    };
  }>,
  approvedModules: readonly string[]
): boolean {
  const importAliases = collectConstantStringBindings(context.sourceCode.ast);
  const imports = collectImportBindings(context.sourceCode.ast);
  const requireAliases = collectSimpleAliases(context.sourceCode.ast);
  const scopeManager = context.sourceCode.scopeManager ?? null;

  for (const node of context.sourceCode.ast.body) {
    if (
      node.type === AST_NODE_TYPES.ImportDeclaration &&
      hasRuntimeImportSpecifiers(node) &&
      typeof node.source?.value === "string" &&
      sourceMatchesApprovedModule(
        context.filename,
        node.source.value,
        approvedModules
      )
    ) {
      return true;
    }
  }

  let found = false;

  function inspectNode(node: TSESTree.Node): void {
    if (found) {
      return;
    }

    if (node.type === AST_NODE_TYPES.ImportExpression) {
      const source = resolveStaticStringExpression(
        node.source,
        importAliases,
        new Set(),
        scopeManager
      );
      found = sourceMatches(source);
    }

    if (
      node.type === AST_NODE_TYPES.CallExpression &&
      isRequireLikeCall(node, imports, requireAliases, scopeManager) &&
      node.arguments.length > 0 &&
      node.arguments[0]?.type !== AST_NODE_TYPES.SpreadElement
    ) {
      const source = resolveStaticStringExpression(
        node.arguments[0],
        importAliases,
        new Set(),
        scopeManager
      );
      found = sourceMatches(source);
    }
  }

  function sourceMatches(source: string | null): boolean {
    return (
      source !== null &&
      sourceMatchesApprovedModule(context.filename, source, approvedModules)
    );
  }

  simpleTraverse(context.sourceCode.ast, {
    enter(node) {
      inspectNode(node);
    }
  });

  return found;
}

export function hasUsedStaticRuntimeImportFromApprovedModule(
  context: Readonly<{
    filename: string;
    sourceCode: {
      ast: TSESTree.Program;
    };
  }>,
  approvedModules: readonly string[]
): boolean {
  const importedLocals = new Map<string, Set<string>>();

  for (const statement of context.sourceCode.ast.body) {
    if (
      statement.type !== AST_NODE_TYPES.ImportDeclaration ||
      !isRuntimeImportDeclaration(statement)
    ) {
      continue;
    }

    const source = statement.source?.value;
    if (
      typeof source !== "string" ||
      !sourceMatchesApprovedModule(context.filename, source, approvedModules)
    ) {
      continue;
    }

    for (const specifier of statement.specifiers) {
      const ranges = importedLocals.get(specifier.local.name) ?? new Set<string>();
      if (specifier.local.range) {
        ranges.add(rangeKey(specifier.local.range));
      }
      importedLocals.set(specifier.local.name, ranges);
    }
  }

  if (importedLocals.size === 0) {
    return false;
  }

  let foundUse = false;

  simpleTraverse(context.sourceCode.ast, {
    enter(node) {
      if (foundUse || node.type !== AST_NODE_TYPES.Identifier) {
        return;
      }

      const declarationRanges = importedLocals.get(node.name);
      if (!declarationRanges) {
        return;
      }

      if (node.range && declarationRanges.has(rangeKey(node.range))) {
        return;
      }

      foundUse = true;
    }
  });

  return foundUse;
}

export interface ApprovedModuleUseTracker {
  trackImportDeclaration: (node: TSESTree.ImportDeclaration) => void;
  trackCallExpression: (
    node: TSESTree.CallExpression,
    ancestors: readonly TSESTree.Node[]
  ) => void;
  trackVariableDeclarator: (
    node: TSESTree.VariableDeclarator,
    ancestors: readonly TSESTree.Node[]
  ) => void;
  trackJsxExpressionContainer: (node: TSESTree.JSXExpressionContainer) => void;
  hasUsedImport: () => boolean;
}

export function createApprovedModuleUseTracker(
  importerFilename: string,
  approvedModules: readonly string[]
): ApprovedModuleUseTracker {
  const importedLocals = new Map<string, Set<string>>();
  const derivedLocals = new Map<string, Set<string>>();
  let foundUse = false;

  return {
    trackImportDeclaration(node) {
      if (!isRuntimeImportDeclaration(node)) {
        return;
      }

      const source = node.source?.value;
      if (
        typeof source !== "string" ||
        !sourceMatchesApprovedModule(importerFilename, source, approvedModules)
      ) {
        return;
      }

      for (const specifier of node.specifiers) {
        const ranges = importedLocals.get(specifier.local.name) ?? new Set<string>();
        if (specifier.local.range) {
          ranges.add(rangeKey(specifier.local.range));
        }
        importedLocals.set(specifier.local.name, ranges);
      }
    },
    trackCallExpression(node, ancestors) {
      if (foundUse) {
        return;
      }

      if (
        expressionIsDirectRouteHelperValue(importedLocals, derivedLocals, node) &&
        callFeedsRouteOutput(ancestors)
      ) {
        foundUse = true;
      }
    },
    trackVariableDeclarator(node, ancestors) {
      if (
        node.id.type !== AST_NODE_TYPES.Identifier ||
        !node.init ||
        !expressionIsDirectRouteHelperValue(importedLocals, derivedLocals, node.init)
      ) {
        return;
      }

      const ranges = derivedLocals.get(node.id.name) ?? new Set<string>();
      if (node.id.range) {
        ranges.add(rangeKey(node.id.range));
      }
      derivedLocals.set(node.id.name, ranges);

      if (ancestors.some((ancestor) => ancestor.type === AST_NODE_TYPES.ExportNamedDeclaration)) {
        foundUse = true;
      }
    },
    trackJsxExpressionContainer(node) {
      if (foundUse) {
        return;
      }

      if (
        node.expression.type !== AST_NODE_TYPES.JSXEmptyExpression &&
        expressionIsDirectRouteHelperValue(importedLocals, derivedLocals, node.expression)
      ) {
        foundUse = true;
      }
    },
    hasUsedImport() {
      return foundUse;
    }
  };
}

function callFeedsRouteOutput(ancestors: readonly TSESTree.Node[]): boolean {
  return (
    ancestors.some(
      (ancestor) =>
        ancestor.type === AST_NODE_TYPES.JSXExpressionContainer
    ) &&
    !ancestors.some(
      (ancestor) =>
        ancestor.type === AST_NODE_TYPES.ExpressionStatement ||
        ancestor.type === AST_NODE_TYPES.ObjectExpression ||
        ancestor.type === AST_NODE_TYPES.ArrayExpression ||
        ancestor.type === AST_NODE_TYPES.LogicalExpression ||
        ancestor.type === AST_NODE_TYPES.BinaryExpression ||
        ancestor.type === AST_NODE_TYPES.ConditionalExpression ||
        (ancestor.type === AST_NODE_TYPES.UnaryExpression &&
          ancestor.operator === "void")
    )
  );
}

function expressionIsDirectRouteHelperValue(
  importedLocals: Map<string, Set<string>>,
  derivedLocals: Map<string, Set<string>>,
  node: TSESTree.Expression | TSESTree.PrivateIdentifier | TSESTree.Super
): boolean {
  if (node.type === AST_NODE_TYPES.Identifier) {
    return isImportedIdentifierUse(derivedLocals, node);
  }

  if (node.type === AST_NODE_TYPES.MemberExpression) {
    return expressionIsDirectRouteHelperValue(importedLocals, derivedLocals, node.object);
  }

  if (node.type === AST_NODE_TYPES.CallExpression) {
    return (
      expressionUsesApprovedImport(importedLocals, node.callee) ||
      expressionIsDirectRouteHelperValue(importedLocals, derivedLocals, node.callee)
    );
  }

  if (node.type === AST_NODE_TYPES.ChainExpression) {
    return expressionIsDirectRouteHelperValue(importedLocals, derivedLocals, node.expression);
  }

  if (node.type === AST_NODE_TYPES.TSNonNullExpression) {
    return expressionIsDirectRouteHelperValue(importedLocals, derivedLocals, node.expression);
  }

  if (node.type === AST_NODE_TYPES.AwaitExpression) {
    return expressionIsDirectRouteHelperValue(importedLocals, derivedLocals, node.argument);
  }

  if (
    node.type === AST_NODE_TYPES.TSAsExpression ||
    node.type === AST_NODE_TYPES.TSSatisfiesExpression ||
    node.type === AST_NODE_TYPES.TSTypeAssertion
  ) {
    return expressionIsDirectRouteHelperValue(importedLocals, derivedLocals, node.expression);
  }

  return false;
}

function expressionUsesApprovedImport(
  importedLocals: Map<string, Set<string>>,
  node: TSESTree.Expression | TSESTree.PrivateIdentifier | TSESTree.Super
): boolean {
  if (node.type === AST_NODE_TYPES.Identifier) {
    return isImportedIdentifierUse(importedLocals, node);
  }

  if (node.type === AST_NODE_TYPES.MemberExpression) {
    return expressionUsesApprovedImport(importedLocals, node.object);
  }

  if (node.type === AST_NODE_TYPES.ChainExpression) {
    return expressionUsesApprovedImport(importedLocals, node.expression);
  }

  if (node.type === AST_NODE_TYPES.TSNonNullExpression) {
    return expressionUsesApprovedImport(importedLocals, node.expression);
  }

  return false;
}

function isImportedIdentifierUse(
  importedLocals: Map<string, Set<string>>,
  node: TSESTree.Identifier
): boolean {
  const declarationRanges = importedLocals.get(node.name);
  if (!declarationRanges) {
    return false;
  }

  return !(node.range && declarationRanges.has(rangeKey(node.range)));
}

export function hasRuntimeImportSpecifiers(
  node: TSESTree.ImportDeclaration
): boolean {
  return !isTypeOnlyImportDeclaration(node) && node.specifiers.length > 0;
}

export function isRuntimeImportDeclaration(
  node: TSESTree.ImportDeclaration
): boolean {
  return !isTypeOnlyImportDeclaration(node);
}

function moduleCandidatePaths(resolvedPath: string): string[] {
  if (/\.[cm]?[jt]sx?$/.test(resolvedPath) || resolvedPath.endsWith(".astro")) {
    return [resolvedPath];
  }

  return [
    resolvedPath,
    `${resolvedPath}.ts`,
    `${resolvedPath}.tsx`,
    `${resolvedPath}.mts`,
    `${resolvedPath}.cts`,
    `${resolvedPath}.js`,
    `${resolvedPath}.jsx`,
    `${resolvedPath}.mjs`,
    `${resolvedPath}.cjs`,
    `${resolvedPath}/index.ts`,
    `${resolvedPath}/index.tsx`,
    `${resolvedPath}/index.mts`,
    `${resolvedPath}/index.cts`,
    `${resolvedPath}/index.js`,
    `${resolvedPath}/index.jsx`,
    `${resolvedPath}/index.mjs`,
    `${resolvedPath}/index.cjs`
  ];
}

function rangeKey(range: TSESTree.Range): string {
  return `${range[0]}:${range[1]}`;
}

export function isTypeOnlyImportDeclaration(
  node: TSESTree.ImportDeclaration
): boolean {
  return (
    node.importKind === "type" ||
    (node.specifiers.length > 0 &&
      node.specifiers.every(
        (specifier) =>
          specifier.type === AST_NODE_TYPES.ImportSpecifier &&
          specifier.importKind === "type"
      ))
  );
}
