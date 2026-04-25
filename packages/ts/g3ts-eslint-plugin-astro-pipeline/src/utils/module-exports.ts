import { AST_NODE_TYPES } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

import {
  collectImportBindings,
  collectSimpleAliases,
  resolveNameReference,
  type ResolvedReference
} from "./ast-helpers.js";
import {
  resolveLocalImport,
  type ModuleRecord
} from "./import-closure.js";

export type ResolvedModuleBinding =
  | {
      kind: "expression";
      moduleRecord: ModuleRecord;
      expression: TSESTree.Expression;
    }
  | {
      kind: "reference";
      moduleRecord: ModuleRecord;
      reference: ResolvedReference;
    };

export function resolveImportedModuleBinding(
  modules: ModuleRecord[],
  importerFilename: string,
  reference: ResolvedReference | null
): ResolvedModuleBinding | null {
  if (!reference) {
    return null;
  }

  if (reference.kind === "import") {
    return resolveImportedBindingBySource(
      modules,
      importerFilename,
      reference.source,
      reference.importedName,
      new Set()
    );
  }

  if (
    reference.kind === "member" &&
    reference.object.kind === "import" &&
    reference.object.importedName === "*"
  ) {
    return resolveImportedBindingBySource(
      modules,
      importerFilename,
      reference.object.source,
      reference.property,
      new Set()
    );
  }

  return null;
}

function resolveImportedBindingBySource(
  modules: ModuleRecord[],
  importerFilename: string,
  source: string,
  importedName: string | "*",
  seen: Set<string>
): ResolvedModuleBinding | null {
  if (importedName === "*") {
    return null;
  }

  const resolvedFilename = resolveLocalImport(importerFilename, source);

  if (!resolvedFilename) {
    return null;
  }

  const seenKey = `${resolvedFilename}:${importedName}`;

  if (seen.has(seenKey)) {
    return null;
  }

  const moduleRecord = modules.find((candidate) => candidate.filename === resolvedFilename);

  if (!moduleRecord) {
    return null;
  }

  seen.add(seenKey);
  const resolved = resolveExportedName(modules, moduleRecord, importedName, seen);
  seen.delete(seenKey);
  return resolved;
}

function resolveExportedName(
  modules: ModuleRecord[],
  moduleRecord: ModuleRecord,
  exportedName: string,
  seen: Set<string>
): ResolvedModuleBinding | null {
  for (const statement of moduleRecord.program.body) {
    if (statement.type === AST_NODE_TYPES.ExportNamedDeclaration) {
      const declarationBinding = resolveDeclarationExport(moduleRecord, statement, exportedName);

      if (declarationBinding) {
        return declarationBinding;
      }

      for (const specifier of statement.specifiers) {
        const exportedSpecifierName = getSpecifierName(specifier.exported);

        if (exportedSpecifierName !== exportedName) {
          continue;
        }

        const localName = getSpecifierName(specifier.local);

        if (!localName) {
          return null;
        }

        if (statement.source?.value && typeof statement.source.value === "string") {
          return resolveReExportBinding(
            modules,
            moduleRecord,
            statement.source.value,
            localName,
            seen
          );
        }

        return resolveLocalBinding(moduleRecord, modules, localName, seen);
      }
    }

    if (
      statement.type === AST_NODE_TYPES.ExportAllDeclaration &&
      typeof statement.source?.value === "string"
    ) {
      const reExportBinding = resolveReExportBinding(
        modules,
        moduleRecord,
        statement.source.value,
        exportedName,
        seen
      );

      if (reExportBinding) {
        return reExportBinding;
      }
    }

    if (
      exportedName === "default" &&
      statement.type === AST_NODE_TYPES.ExportDefaultDeclaration
    ) {
      if (statement.declaration.type === AST_NODE_TYPES.Identifier) {
        return resolveLocalBinding(
          moduleRecord,
          modules,
          statement.declaration.name,
          seen
        );
      }

      if ("type" in statement.declaration && isExpression(statement.declaration)) {
        return {
          kind: "expression",
          moduleRecord,
          expression: statement.declaration
        };
      }
    }
  }

  return null;
}

function resolveDeclarationExport(
  moduleRecord: ModuleRecord,
  statement: TSESTree.ExportNamedDeclaration,
  exportedName: string
): ResolvedModuleBinding | null {
  const declaration = statement.declaration;

  if (!declaration) {
    return null;
  }

  if (declaration.type === AST_NODE_TYPES.VariableDeclaration) {
    for (const declarator of declaration.declarations) {
      if (
        declarator.id.type === AST_NODE_TYPES.Identifier &&
        declarator.id.name === exportedName &&
        declarator.init
      ) {
        return {
          kind: "expression",
          moduleRecord,
          expression: declarator.init
        };
      }
    }
  }

  return null;
}

function resolveReExportBinding(
  modules: ModuleRecord[],
  moduleRecord: ModuleRecord,
  source: string,
  localName: string,
  seen: Set<string>
): ResolvedModuleBinding | null {
  const resolvedFilename = resolveLocalImport(moduleRecord.filename, source);

  if (!resolvedFilename) {
    return {
      kind: "reference",
      moduleRecord,
      reference: {
        kind: "import",
        source,
        importedName: localName
      }
    };
  }

  return resolveImportedBindingBySource(modules, moduleRecord.filename, source, localName, seen);
}

function resolveLocalBinding(
  moduleRecord: ModuleRecord,
  modules: ModuleRecord[],
  localName: string,
  seen: Set<string>
): ResolvedModuleBinding | null {
  const imports = collectImportBindings(moduleRecord.program);
  const aliases = collectSimpleAliases(moduleRecord.program);
  const resolvedReference = resolveNameReference(localName, imports, aliases);

  if (resolvedReference) {
    if (resolvedReference.kind === "import") {
      const importedBinding = resolveImportedBindingBySource(
        modules,
        moduleRecord.filename,
        resolvedReference.source,
        resolvedReference.importedName,
        seen
      );

      if (importedBinding) {
        return importedBinding;
      }
    }

    return {
      kind: "reference",
      moduleRecord,
      reference: resolvedReference
    };
  }

  for (const statement of moduleRecord.program.body) {
    if (statement.type !== AST_NODE_TYPES.VariableDeclaration) {
      continue;
    }

    for (const declarator of statement.declarations) {
      if (
        declarator.id.type === AST_NODE_TYPES.Identifier &&
        declarator.id.name === localName &&
        declarator.init
      ) {
        return {
          kind: "expression",
          moduleRecord,
          expression: declarator.init
        };
      }
    }
  }

  return null;
}

function getSpecifierName(
  node: TSESTree.ExportSpecifier["local"] | TSESTree.ExportSpecifier["exported"]
): string | null {
  if (node.type === AST_NODE_TYPES.Identifier) {
    return node.name;
  }

  if (node.type === AST_NODE_TYPES.Literal && typeof node.value === "string") {
    return node.value;
  }

  return null;
}

function isExpression(
  node: TSESTree.ExportDefaultDeclaration["declaration"]
): node is TSESTree.Expression {
  return ![
    AST_NODE_TYPES.FunctionDeclaration,
    AST_NODE_TYPES.ClassDeclaration,
    AST_NODE_TYPES.TSInterfaceDeclaration
  ].includes(node.type as AST_NODE_TYPES);
}
