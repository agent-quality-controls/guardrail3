import fs from "node:fs";
import path from "node:path";

import * as astroParser from "astro-eslint-parser";
import * as mdxParser from "eslint-mdx";
import tsParser from "@typescript-eslint/parser";
import { AST_NODE_TYPES } from "@typescript-eslint/utils";
import type { TSESLint, TSESTree } from "@typescript-eslint/utils";

import {
  collectConstantStringBindings,
  findNodes,
  isUnresolvedIdentifierReference,
  listStaticImportSources,
  resolveStaticStringExpression
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
  ".astro",
  ".mdx"
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
      : filename.endsWith(".mdx")
      ? mdxParser.parseForESLint(sourceText, {
          ecmaVersion: "latest",
          filePath: filename,
          loc: true,
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

function resolveLocalImport(
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

  findNodes(program, (node) => {
    if (
      node.type !== AST_NODE_TYPES.CallExpression ||
      node.callee.type !== AST_NODE_TYPES.Identifier ||
      node.callee.name !== "require" ||
      !isUnresolvedIdentifierReference(scopeManager, node.callee)
    ) {
      return;
    }

    const firstArg = node.arguments[0];

    if (!firstArg || firstArg.type === AST_NODE_TYPES.SpreadElement) {
      return;
    }

    const source = resolveStaticStringExpression(firstArg, constants);

    if (source) {
      sources.add(source);
    }
  });

  return [...sources];
}
