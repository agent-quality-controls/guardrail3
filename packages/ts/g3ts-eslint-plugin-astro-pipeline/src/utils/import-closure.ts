import fs from "node:fs";
import path from "node:path";

import * as astroParser from "astro-eslint-parser";
import tsParser from "@typescript-eslint/parser";
import { AST_NODE_TYPES } from "@typescript-eslint/utils";
import type { TSESLint, TSESTree } from "@typescript-eslint/utils";

import {
  collectConstantStringBindings,
  collectImportBindings,
  collectSimpleAliases,
  findNodes,
  getIdentifierInitializer,
  getPropertyName,
  isProcessCwdLikeCall,
  isRequireLikeCall,
  listStaticImportSources,
  resolveReference,
  resolveStaticStringExpression,
  unwrapExpression
} from "./ast-helpers.js";
import { inferPathPolicyRoot } from "./path-policy.js";

const SOURCE_EXTENSIONS = [
  ".ts",
  ".tsx",
  ".js",
  ".jsx",
  ".mts",
  ".cts",
  ".mjs",
  ".cjs",
  ".astro"
] as const;

export interface ModuleRecord {
  filename: string;
  program: TSESTree.Program;
  scopeManager: TSESLint.Scope.ScopeManager | null;
  importChain: string[];
}

export interface EntryModuleRecord {
  program: TSESTree.Program;
  scopeManager: TSESLint.Scope.ScopeManager | null;
}

export function collectImportClosure(
  entryFilename: string,
  entrySource: string,
  entryModule?: EntryModuleRecord
): ModuleRecord[] {
  const fallbackEntryModule = parseModule(entryFilename, entrySource);
  const parsedEntryModule = entryModule
    ? {
        program: entryModule.program,
        scopeManager: entryModule.scopeManager ?? fallbackEntryModule?.scopeManager ?? null
      }
    : fallbackEntryModule;

  if (!parsedEntryModule) {
    return [];
  }

  const visited = new Set<string>();
  const queue: ModuleRecord[] = [
    {
      filename: entryFilename,
      program: parsedEntryModule.program,
      scopeManager: parsedEntryModule.scopeManager,
      importChain: [entryFilename]
    }
  ];
  const modules: ModuleRecord[] = [];

  while (queue.length > 0) {
    const current = queue.shift();

    if (!current || visited.has(current.filename)) {
      continue;
    }

    visited.add(current.filename);
    modules.push(current);

    for (const importSource of listDependencySources(current.program, current.scopeManager)) {
      const resolvedImport = resolveLocalImport(current.filename, importSource);

      if (!resolvedImport || visited.has(resolvedImport)) {
        continue;
      }

      const sourceText = safeReadFile(resolvedImport);

      if (!sourceText) {
        continue;
      }

      const moduleRecord = parseModule(resolvedImport, sourceText);

      if (!moduleRecord) {
        continue;
      }

      queue.push({
        filename: resolvedImport,
        program: moduleRecord.program,
        scopeManager: moduleRecord.scopeManager,
        importChain: [...current.importChain, resolvedImport]
      });
    }
  }

  return modules;
}

function parseModule(
  filename: string,
  sourceText: string
): { program: TSESTree.Program; scopeManager: TSESLint.Scope.ScopeManager | null } | null {
  try {
    const parsed = filename.endsWith(".astro")
      ? astroParser.parseForESLint(sourceText, {
          ecmaVersion: "latest",
          filePath: filename,
          loc: true,
          parser: tsParser,
          range: true,
          sourceType: "module"
        })
      : tsParser.parseForESLint(sourceText, {
          ecmaVersion: "latest",
          filePath: filename,
          jsx: hasJsxSyntax(filename),
          loc: true,
          range: true,
          sourceType: "module"
        });

    return {
      program: parsed.ast as unknown as TSESTree.Program,
      scopeManager: (parsed.scopeManager as TSESLint.Scope.ScopeManager | null) ?? null
    };
  } catch {
    return null;
  }
}

export function resolveLocalImport(
  importerFilename: string,
  importSource: string
): string | null {
  const resolvedBase = resolveImportBase(importerFilename, importSource);

  if (!resolvedBase) {
    return null;
  }

  const resolvedAsFile = resolveExistingFile(resolvedBase);

  if (resolvedAsFile) {
    return resolvedAsFile;
  }

  for (const extension of SOURCE_EXTENSIONS) {
    const candidate = path.join(resolvedBase, `index${extension}`);

    if (fs.existsSync(candidate) && fs.statSync(candidate).isFile()) {
      return candidate;
    }
  }

  return null;
}

function resolveImportBase(
  importerFilename: string,
  importSource: string
): string | null {
  if (path.isAbsolute(importSource)) {
    return importSource;
  }

  if (importSource.startsWith(".")) {
    return path.resolve(path.dirname(importerFilename), importSource);
  }

  const appRoot = inferPathPolicyRoot(importerFilename);

  if (importSource.startsWith("@/") || importSource.startsWith("~/")) {
    return path.resolve(appRoot, "src", importSource.slice(2));
  }

  if (importSource.startsWith("src/")) {
    return path.resolve(appRoot, importSource);
  }

  return null;
}

function resolveExistingFile(resolvedBase: string): string | null {
  if (fs.existsSync(resolvedBase) && fs.statSync(resolvedBase).isFile()) {
    return resolvedBase;
  }

  for (const extension of SOURCE_EXTENSIONS) {
    const candidate = `${resolvedBase}${extension}`;

    if (fs.existsSync(candidate) && fs.statSync(candidate).isFile()) {
      return candidate;
    }
  }

  return null;
}

function safeReadFile(filename: string): string | null {
  try {
    return fs.readFileSync(filename, "utf8");
  } catch {
    return null;
  }
}

function hasJsxSyntax(filename: string): boolean {
  return [".tsx", ".jsx"].includes(path.extname(filename));
}

function listDependencySources(
  program: TSESTree.Program,
  scopeManager: TSESLint.Scope.ScopeManager | null
): string[] {
  const sources = new Set(listStaticImportSources(program));
  const constants = collectConstantStringBindings(program);
  const imports = collectImportBindings(program);
  const aliases = collectSimpleAliases(program);

  findNodes(program, (node) => {
    if (node.type === AST_NODE_TYPES.ImportExpression) {
      const source = resolveDependencySourceExpression(
        node.source,
        constants,
        imports,
        aliases,
        scopeManager,
        new Set()
      );

      if (source) {
        sources.add(source);
      }
      return;
    }

    if (
      node.type !== AST_NODE_TYPES.CallExpression ||
      !isRequireLikeCall(node, imports, aliases, scopeManager)
    ) {
      return;
    }

    const firstArg = node.arguments[0];

    if (!firstArg || firstArg.type === AST_NODE_TYPES.SpreadElement) {
      return;
    }

    const source = resolveDependencySourceExpression(
      firstArg,
      constants,
      imports,
      aliases,
      scopeManager,
      new Set()
    );

    if (source) {
      sources.add(source);
    }
  });

  return [...sources];
}

function resolveDependencySourceExpression(
  node: TSESTree.Expression,
  constants: Map<string, TSESTree.Expression>,
  imports: ReturnType<typeof collectImportBindings>,
  aliases: ReturnType<typeof collectSimpleAliases>,
  scopeManager: TSESLint.Scope.ScopeManager | null,
  seen: Set<string>
): string | null {
  const direct = resolveStaticStringExpression(node, constants, seen, scopeManager);

  if (direct) {
    return direct;
  }

  const unwrapped = unwrapExpression(node);
  if (unwrapped !== node) {
    return resolveDependencySourceExpression(
      unwrapped,
      constants,
      imports,
      aliases,
      scopeManager,
      seen
    );
  }

  if (node.type === AST_NODE_TYPES.Identifier) {
    if (seen.has(node.name)) {
      return null;
    }

    const target =
      scopeManager == null ? constants.get(node.name) : getIdentifierInitializer(scopeManager, node);

    if (!target) {
      return null;
    }

    seen.add(node.name);
    const resolved = resolveDependencySourceExpression(
      target,
      constants,
      imports,
      aliases,
      scopeManager,
      seen
    );
    seen.delete(node.name);
    return resolved;
  }

  if (
    node.type === AST_NODE_TYPES.MemberExpression &&
    getPropertyName(node) === "pathname" &&
    node.object.type !== AST_NODE_TYPES.Super
  ) {
    return resolveDependencySourceExpression(
      node.object,
      constants,
      imports,
      aliases,
      scopeManager,
      seen
    );
  }

  if (node.type === AST_NODE_TYPES.NewExpression) {
    return resolveFileUrlLike(node, constants, imports, aliases, scopeManager, seen);
  }

  if (node.type !== AST_NODE_TYPES.CallExpression) {
    return null;
  }

  const reference = resolveReference(node.callee, imports, aliases);

  if (!reference || !isNodePathJoinLike(reference)) {
    return null;
  }

  const parts = node.arguments.map((argument, index) => {
    if (argument.type === AST_NODE_TYPES.SpreadElement) {
      return null;
    }

    if (index === 0 && isProcessCwdLikeCall(argument, constants, scopeManager)) {
      return "";
    }

    return resolveDependencySourceExpression(
      argument,
      constants,
      imports,
      aliases,
      scopeManager,
      seen
    );
  });

  if (parts.some((part) => part == null)) {
    return null;
  }

  return path.posix.join(...(parts as string[]));
}

function resolveFileUrlLike(
  node: TSESTree.NewExpression,
  constants: Map<string, TSESTree.Expression>,
  imports: ReturnType<typeof collectImportBindings>,
  aliases: ReturnType<typeof collectSimpleAliases>,
  scopeManager: TSESLint.Scope.ScopeManager | null,
  seen: Set<string>
): string | null {
  if (
    node.callee.type !== AST_NODE_TYPES.Identifier ||
    node.callee.name !== "URL" ||
    node.arguments.length < 2
  ) {
    return null;
  }

  const target = node.arguments[0];
  const base = node.arguments[1];

  if (
    target.type === AST_NODE_TYPES.SpreadElement ||
    base.type === AST_NODE_TYPES.SpreadElement ||
    !isImportMetaUrlLike(base, constants, scopeManager, seen)
  ) {
    return null;
  }

  return resolveDependencySourceExpression(
    target,
    constants,
    imports,
    aliases,
    scopeManager,
    seen
  );
}

function isImportMetaUrlLike(
  node: TSESTree.Expression,
  constants: Map<string, TSESTree.Expression>,
  scopeManager: TSESLint.Scope.ScopeManager | null,
  seen: Set<string>
): boolean {
  const unwrapped = unwrapExpression(node);

  if (isImportMetaUrl(unwrapped)) {
    return true;
  }

  if (unwrapped.type !== AST_NODE_TYPES.Identifier) {
    return false;
  }

  if (seen.has(unwrapped.name)) {
    return false;
  }

  const target =
    scopeManager == null
      ? constants.get(unwrapped.name)
      : getIdentifierInitializer(scopeManager, unwrapped);

  if (!target) {
    return false;
  }

  seen.add(unwrapped.name);
  const matches = isImportMetaUrlLike(target, constants, scopeManager, seen);
  seen.delete(unwrapped.name);
  return matches;
}

function isImportMetaUrl(node: TSESTree.Expression): boolean {
  return (
    node.type === AST_NODE_TYPES.MemberExpression &&
    node.object.type === AST_NODE_TYPES.MetaProperty &&
    node.object.meta.name === "import" &&
    node.object.property.name === "meta" &&
    !node.computed &&
    node.property.type === AST_NODE_TYPES.Identifier &&
    node.property.name === "url"
  );
}

function isNodePathJoinLike(reference: ReturnType<typeof resolveReference>): boolean {
  if (!reference) {
    return false;
  }

  if (reference.kind === "import") {
    return (
      isNodePathModule(reference.source) &&
      (reference.importedName === "join" || reference.importedName === "resolve")
    );
  }

  if (reference.kind !== "member") {
    return false;
  }

  if (
    reference.object.kind === "import" &&
    isNodePathModule(reference.object.source) &&
    reference.object.importedName === "posix" &&
    (reference.property === "join" || reference.property === "resolve")
  ) {
    return true;
  }

  if (
    reference.object.kind === "import" &&
    isNodePathModule(reference.object.source) &&
    (reference.object.importedName === "*" || reference.object.importedName === "default") &&
    (reference.property === "join" || reference.property === "resolve")
  ) {
    return true;
  }

  return (
    reference.object.kind === "member" &&
    reference.object.object.kind === "import" &&
    isNodePathModule(reference.object.object.source) &&
    (reference.object.object.importedName === "*" ||
      reference.object.object.importedName === "default") &&
    reference.object.property === "posix" &&
    (reference.property === "join" || reference.property === "resolve")
  );
}

function isNodePathModule(source: string): boolean {
  return source === "path" || source === "node:path";
}
