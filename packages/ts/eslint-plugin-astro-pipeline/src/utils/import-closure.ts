import fs from "node:fs";
import path from "node:path";

import tsParser from "@typescript-eslint/parser";
import type { TSESLint, TSESTree } from "@typescript-eslint/utils";

import { listStaticImportSources } from "./ast-helpers.js";

const SOURCE_EXTENSIONS = [
  ".ts",
  ".tsx",
  ".js",
  ".jsx",
  ".mts",
  ".cts",
  ".mjs",
  ".cjs"
] as const;

export interface ModuleRecord {
  filename: string;
  program: TSESTree.Program;
  scopeManager: TSESLint.Scope.ScopeManager | null;
  importChain: string[];
}

export function collectImportClosure(
  entryFilename: string,
  entrySource: string
): ModuleRecord[] {
  const entryModule = parseModule(entryFilename, entrySource);

  if (!entryModule) {
    return [];
  }

  const visited = new Set<string>();
  const queue: ModuleRecord[] = [
    {
      filename: entryFilename,
      program: entryModule.program,
      scopeManager: entryModule.scopeManager,
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

    for (const importSource of listStaticImportSources(current.program)) {
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
    const parsed = tsParser.parseForESLint(sourceText, {
      ecmaVersion: "latest",
      filePath: filename,
      jsx: hasJsxSyntax(filename),
      loc: true,
      range: true,
      sourceType: "module"
    });

    return {
      program: parsed.ast,
      scopeManager: parsed.scopeManager ?? null
    };
  } catch {
    return null;
  }
}

function resolveLocalImport(
  importerFilename: string,
  importSource: string
): string | null {
  if (!importSource.startsWith(".") && !path.isAbsolute(importSource)) {
    return null;
  }

  const resolvedBase = path.resolve(path.dirname(importerFilename), importSource);
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
