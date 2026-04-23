import type { TSESLint, TSESTree } from "@typescript-eslint/utils";
import { AST_NODE_TYPES } from "@typescript-eslint/utils";
import { simpleTraverse } from "@typescript-eslint/typescript-estree";

export interface ImportBinding {
  localName: string;
  source: string;
  importedName: string | "*";
}

type AliasReference =
  | {
      kind: "identifier";
      name: string;
    }
  | {
      kind: "require";
    }
  | {
      kind: "import";
      source: string;
      importedName: string | "*";
    }
  | {
      kind: "member";
      object: AliasReference;
      property: string;
    };

export type ResolvedReference =
  | {
      kind: "import";
      source: string;
      importedName: string | "*";
    }
  | {
      kind: "member";
      object: ResolvedReference;
      property: string;
    };

export function getStaticStringValue(
  node: TSESTree.Expression | TSESTree.PrivateIdentifier | null | undefined
): string | null {
  if (!node) {
    return null;
  }

  if (node.type === AST_NODE_TYPES.Literal && typeof node.value === "string") {
    return node.value;
  }

  if (
    node.type === AST_NODE_TYPES.TemplateLiteral &&
    node.expressions.length === 0 &&
    node.quasis.length === 1
  ) {
    return node.quasis[0]?.value.cooked ?? null;
  }

  return null;
}

export function getPropertyName(node: TSESTree.MemberExpression): string | null {
  if (node.computed) {
    return getStaticStringValue(node.property);
  }

  if (node.property.type === AST_NODE_TYPES.Identifier) {
    return node.property.name;
  }

  return getStaticStringValue(node.property);
}

export function collectImportBindings(
  program: TSESTree.Program
): Map<string, ImportBinding> {
  const bindings = new Map<string, ImportBinding>();

  for (const statement of program.body) {
    if (statement.type !== AST_NODE_TYPES.ImportDeclaration) {
      continue;
    }

    const source = statement.source.value;

    if (typeof source !== "string") {
      continue;
    }

    for (const specifier of statement.specifiers) {
      switch (specifier.type) {
        case AST_NODE_TYPES.ImportSpecifier:
          bindings.set(specifier.local.name, {
            localName: specifier.local.name,
            source,
            importedName:
              specifier.imported.type === AST_NODE_TYPES.Identifier
                ? specifier.imported.name
                : String(specifier.imported.value)
          });
          break;
        case AST_NODE_TYPES.ImportNamespaceSpecifier:
          bindings.set(specifier.local.name, {
            localName: specifier.local.name,
            source,
            importedName: "*"
          });
          break;
        case AST_NODE_TYPES.ImportDefaultSpecifier:
          bindings.set(specifier.local.name, {
            localName: specifier.local.name,
            source,
            importedName: "default"
          });
          break;
        default:
          break;
      }
    }
  }

  return bindings;
}

export function collectSimpleAliases(
  program: TSESTree.Program
): Map<string, AliasReference> {
  const aliases = new Map<string, AliasReference>();
  const imports = collectImportBindings(program);

  let changed = true;
  while (changed) {
    changed = false;

    for (const statement of program.body) {
      if (statement.type !== AST_NODE_TYPES.VariableDeclaration) {
        continue;
      }

      for (const declaration of statement.declarations) {
        if (!declaration.init) {
          continue;
        }

        const aliasReference = expressionToAliasReference(declaration.init, imports, aliases);

        if (declaration.id.type === AST_NODE_TYPES.Identifier && aliasReference) {
          if (!aliases.has(declaration.id.name)) {
            aliases.set(declaration.id.name, aliasReference);
            changed = true;
          }
          continue;
        }

        if (declaration.id.type === AST_NODE_TYPES.ObjectPattern && aliasReference) {
          changed =
            collectObjectPatternAliases(aliases, declaration.id, aliasReference) || changed;
        }
      }
    }
  }

  return aliases;
}

export function resolveReference(
  node: TSESTree.Expression | TSESTree.PrivateIdentifier | null | undefined,
  imports: Map<string, ImportBinding>,
  aliases: Map<string, AliasReference>,
  seen = new Set<string>()
): ResolvedReference | null {
  if (!node || node.type === AST_NODE_TYPES.PrivateIdentifier) {
    return null;
  }

  if (node.type === AST_NODE_TYPES.ChainExpression) {
    return resolveReference(node.expression, imports, aliases, seen);
  }

  if (node.type === AST_NODE_TYPES.Identifier) {
    const directImport = imports.get(node.name);

    if (directImport) {
      return {
        kind: "import",
        source: directImport.source,
        importedName: directImport.importedName
      };
    }

    if (seen.has(node.name)) {
      return null;
    }

    const aliasReference = aliases.get(node.name);

    if (!aliasReference) {
      return null;
    }

    seen.add(node.name);
    const resolvedAlias = resolveAliasReference(aliasReference, imports, aliases, seen);
    seen.delete(node.name);

    return resolvedAlias;
  }

  if (node.type === AST_NODE_TYPES.MemberExpression) {
    const objectReference = resolveReference(node.object, imports, aliases, seen);
    const propertyName = getPropertyName(node);

    if (!objectReference || !propertyName) {
      return null;
    }

    return {
      kind: "member",
      object: objectReference,
      property: propertyName
    };
  }

  return null;
}

export function resolveNameReference(
  name: string,
  imports: Map<string, ImportBinding>,
  aliases: Map<string, AliasReference>
): ResolvedReference | null {
  const directImport = imports.get(name);

  if (directImport) {
    return {
      kind: "import",
      source: directImport.source,
      importedName: directImport.importedName
    };
  }

  const aliasReference = aliases.get(name);

  if (!aliasReference) {
    return null;
  }

  return resolveAliasReference(aliasReference, imports, aliases, new Set([name]));
}

export function isCallLike(
  node: TSESTree.Node,
  predicate: (reference: ResolvedReference) => boolean
): node is TSESTree.CallExpression {
  return (
    node.type === AST_NODE_TYPES.CallExpression &&
    predicateFromExpression(node.callee, predicate)
  );
}

export function isNewLike(
  node: TSESTree.Node,
  predicate: (reference: ResolvedReference) => boolean
): node is TSESTree.NewExpression {
  return (
    node.type === AST_NODE_TYPES.NewExpression &&
    predicateFromExpression(node.callee, predicate)
  );
}

export function findNodes(
  program: TSESTree.Program,
  callback: (node: TSESTree.Node) => void
): void {
  simpleTraverse(program as TSESTree.Node, {
    enter(node) {
      callback(node);
    }
  });
}

export function listStaticImportSources(program: TSESTree.Program): string[] {
  const sources: string[] = [];
  const constants = collectConstantStringBindings(program);

  for (const statement of program.body) {
    if (statement.type === AST_NODE_TYPES.ImportDeclaration) {
      if (
        statement.importKind === "type" ||
        (statement.specifiers.length > 0 &&
          statement.specifiers.every(
            (specifier) =>
              specifier.type === AST_NODE_TYPES.ImportSpecifier &&
              specifier.importKind === "type"
          ))
      ) {
        continue;
      }
    }

    if (
      statement.type === AST_NODE_TYPES.ExportAllDeclaration &&
      statement.exportKind === "type"
    ) {
      continue;
    }

    if (
      statement.type === AST_NODE_TYPES.ExportNamedDeclaration &&
      (statement.exportKind === "type" ||
        (statement.specifiers.length > 0 &&
          statement.specifiers.every(
            (specifier) => "exportKind" in specifier && specifier.exportKind === "type"
          )))
    ) {
      continue;
    }

    if (
      statement.type === AST_NODE_TYPES.ImportDeclaration ||
      statement.type === AST_NODE_TYPES.ExportAllDeclaration ||
      statement.type === AST_NODE_TYPES.ExportNamedDeclaration
    ) {
      const source = statement.source?.value;

      if (typeof source === "string") {
        sources.push(source);
      }
    }
  }

  findNodes(program, (node) => {
    if (node.type === AST_NODE_TYPES.ImportExpression) {
      const source = resolveStaticStringExpression(node.source, constants);

      if (source) {
        sources.push(source);
      }
    }
  });

  return sources;
}

export function hasStaticRequireSource(
  program: TSESTree.Program,
  scopeManager: TSESLint.Scope.ScopeManager | null | undefined,
  targetSource: string
): boolean {
  let found = false;

  findNodes(program, (node) => {
    if (
      found ||
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

    if (getStaticStringValue(firstArg) === targetSource) {
      found = true;
    }
  });

  return found;
}

export function isUnresolvedIdentifierReference(
  scopeManager: TSESLint.Scope.ScopeManager | null | undefined,
  identifier: TSESTree.Identifier
): boolean {
  if (!scopeManager || !identifier.range) {
    return false;
  }

  for (const scope of scopeManager.scopes) {
    for (const reference of [...scope.references, ...scope.through]) {
      if (
        reference.identifier.range?.[0] === identifier.range[0] &&
        reference.identifier.range?.[1] === identifier.range[1]
      ) {
        return reference.resolved == null;
      }
    }
  }

  return false;
}

export function isRequireLikeCall(
  node: TSESTree.CallExpression,
  imports: Map<string, ImportBinding>,
  aliases: Map<string, AliasReference>,
  scopeManager?: TSESLint.Scope.ScopeManager | null
): boolean {
  if (
    node.callee.type === AST_NODE_TYPES.Identifier &&
    isRequireLikeIdentifierReference(node.callee, imports, aliases, scopeManager ?? null)
  ) {
    return true;
  }

  return (
    node.callee.type === AST_NODE_TYPES.MemberExpression &&
    isModuleRequireMemberExpression(node.callee, imports, aliases)
  );
}

function predicateFromExpression(
  node: TSESTree.Expression | TSESTree.PrivateIdentifier,
  predicate: (reference: ResolvedReference) => boolean
): boolean {
  if (node.type === AST_NODE_TYPES.PrivateIdentifier) {
    return false;
  }

  return predicate(resolveReference(node, new Map(), new Map()) as never);
}

export function collectConstantStringBindings(
  program: TSESTree.Program
): Map<string, TSESTree.Expression> {
  const constants = new Map<string, TSESTree.Expression>();

  findNodes(program, (node) => {
    if (node.type !== AST_NODE_TYPES.VariableDeclarator || !node.init) {
      return;
    }

    if (node.id.type !== AST_NODE_TYPES.Identifier) {
      return;
    }

    constants.set(node.id.name, node.init);
  });

  return constants;
}

export function resolveStaticStringExpression(
  node: TSESTree.Expression,
  constants: Map<string, TSESTree.Expression>,
  seen = new Set<string>()
): string | null {
  const direct = getStaticStringValue(node);

  if (direct) {
    return direct;
  }

  if (node.type !== AST_NODE_TYPES.Identifier) {
    return null;
  }

  if (seen.has(node.name)) {
    return null;
  }

  const target = constants.get(node.name);

  if (!target) {
    return null;
  }

  seen.add(node.name);
  const resolved = resolveStaticStringExpression(target, constants, seen);
  seen.delete(node.name);
  return resolved;
}

function expressionToAliasReference(
  node: TSESTree.Expression,
  imports: Map<string, ImportBinding>,
  aliases: Map<string, AliasReference>
): AliasReference | null {
  if (node.type === AST_NODE_TYPES.CallExpression) {
    return requireCallToAliasReference(node, imports, aliases);
  }

  if (node.type === AST_NODE_TYPES.Identifier) {
    const directImport = imports.get(node.name);

    if (directImport) {
      return {
        kind: "import",
        source: directImport.source,
        importedName: directImport.importedName
      };
    }

    return {
      kind: "identifier",
      name: node.name
    };
  }

  if (node.type !== AST_NODE_TYPES.MemberExpression) {
    return null;
  }

  if (isModuleRequireMemberExpression(node, imports, aliases)) {
    return {
      kind: "require"
    };
  }

  const objectReference = expressionToAliasReference(node.object, imports, aliases);
  const propertyName = getPropertyName(node);

  if (!objectReference || !propertyName) {
    return null;
  }

  return {
    kind: "member",
    object: objectReference,
    property: propertyName
  };
}

function resolveAliasReference(
  reference: AliasReference,
  imports: Map<string, ImportBinding>,
  aliases: Map<string, AliasReference>,
  seen: Set<string>
): ResolvedReference | null {
  if (reference.kind === "require") {
    return null;
  }

  if (reference.kind === "import") {
    return {
      kind: "import",
      source: reference.source,
      importedName: reference.importedName
    };
  }

  if (reference.kind === "identifier") {
    const directImport = imports.get(reference.name);

    if (directImport) {
      return {
        kind: "import",
        source: directImport.source,
        importedName: directImport.importedName
      };
    }

    if (seen.has(reference.name)) {
      return null;
    }

    const aliasReference = aliases.get(reference.name);

    if (!aliasReference) {
      return null;
    }

    seen.add(reference.name);
    const resolvedAlias = resolveAliasReference(aliasReference, imports, aliases, seen);
    seen.delete(reference.name);

    return resolvedAlias;
  }

  const objectReference = resolveAliasReference(reference.object, imports, aliases, seen);

  if (!objectReference) {
    return null;
  }

  return {
    kind: "member",
    object: objectReference,
    property: reference.property
  };
}

function requireCallToAliasReference(
  node: TSESTree.CallExpression,
  imports: Map<string, ImportBinding>,
  aliases: Map<string, AliasReference>
): AliasReference | null {
  const isCreateRequire = isCreateRequireCall(node, imports, aliases);
  const isRequireCall =
    node.callee.type === AST_NODE_TYPES.Identifier &&
    isRequireLikeIdentifier(node.callee.name, aliases);

  if (!isCreateRequire && !isRequireCall) {
    return null;
  }

  if (isCreateRequire) {
    return {
      kind: "require"
    };
  }

  const source = getStaticStringValue(node.arguments[0] as TSESTree.Expression);

  if (!source) {
    return null;
  }

  return {
    kind: "import",
    source,
    importedName: "*"
  };
}

function collectObjectPatternAliases(
  aliases: Map<string, AliasReference>,
  pattern: TSESTree.ObjectPattern,
  sourceReference: AliasReference
): boolean {
  let changed = false;

  for (const property of pattern.properties) {
    if (property.type !== AST_NODE_TYPES.Property || property.computed) {
      continue;
    }

    const propertyName = getObjectPatternPropertyName(property.key);
    const localName = getPatternLocalName(property.value);

    if (!propertyName || !localName) {
      continue;
    }

    if (!aliases.has(localName)) {
      aliases.set(localName, {
        kind: "member",
        object: sourceReference,
        property: propertyName
      });
      changed = true;
    }
  }

  return changed;
}

function getObjectPatternPropertyName(
  key: TSESTree.Property["key"]
): string | null {
  if (key.type === AST_NODE_TYPES.Identifier) {
    return key.name;
  }

  if (key.type === AST_NODE_TYPES.Literal && typeof key.value === "string") {
    return key.value;
  }

  return null;
}

function getPatternLocalName(
  value: TSESTree.Property["value"]
): string | null {
  if (value.type === AST_NODE_TYPES.Identifier) {
    return value.name;
  }

  if (
    value.type === AST_NODE_TYPES.AssignmentPattern &&
    value.left.type === AST_NODE_TYPES.Identifier
  ) {
    return value.left.name;
  }

  return null;
}

function isRequireLikeIdentifier(
  name: string,
  aliases: Map<string, AliasReference>,
  seen = new Set<string>()
): boolean {
  if (name === "require") {
    return true;
  }

  if (seen.has(name)) {
    return false;
  }

  const aliasReference = aliases.get(name);

  if (!aliasReference) {
    return false;
  }

  seen.add(name);
  const isRequireAlias = isRequireLikeAliasReference(aliasReference, aliases, seen);
  seen.delete(name);
  return isRequireAlias;
}

function isRequireLikeIdentifierReference(
  identifier: TSESTree.Identifier,
  imports: Map<string, ImportBinding>,
  aliases: Map<string, AliasReference>,
  scopeManager: TSESLint.Scope.ScopeManager | null,
  seen = new Set<string>()
): boolean {
  if (identifier.name === "require") {
    return scopeManager == null || isUnresolvedIdentifierReference(scopeManager, identifier);
  }

  if (seen.has(identifier.name)) {
    return false;
  }

  if (isRequireLikeIdentifier(identifier.name, aliases, seen)) {
    return true;
  }

  if (!scopeManager) {
    return false;
  }

  const resolvedReference = resolvedReferenceForIdentifier(scopeManager, identifier);
  const definitionNode = resolvedReference?.resolved?.defs?.[0]?.node;

  if (
    definitionNode &&
    definitionNode.type === AST_NODE_TYPES.VariableDeclarator &&
    definitionNode.id.type === AST_NODE_TYPES.ObjectPattern &&
    definitionNode.init &&
    objectPatternPropertyForLocal(definitionNode.id, identifier.name) === "require"
  ) {
    return isNodeModuleRequireSource(
      definitionNode.init,
      imports,
      aliases,
      scopeManager
    );
  }

  if (
    !definitionNode ||
    definitionNode.type !== AST_NODE_TYPES.VariableDeclarator ||
    !definitionNode.init
  ) {
    return false;
  }

  seen.add(identifier.name);
  const isRequireAlias = requireLikeInitializer(
    definitionNode.init,
    imports,
    aliases,
    scopeManager,
    seen
  );
  seen.delete(identifier.name);
  return isRequireAlias;
}

function resolvedReferenceForIdentifier(
  scopeManager: TSESLint.Scope.ScopeManager,
  identifier: TSESTree.Identifier
): TSESLint.Scope.Reference | null {
  for (const scope of scopeManager.scopes) {
    for (const reference of [...scope.references, ...scope.through]) {
      if (
        reference.identifier.range?.[0] === identifier.range?.[0] &&
        reference.identifier.range?.[1] === identifier.range?.[1]
      ) {
        return reference;
      }
    }
  }

  return null;
}

function requireLikeInitializer(
  node: TSESTree.Expression,
  imports: Map<string, ImportBinding>,
  aliases: Map<string, AliasReference>,
  scopeManager: TSESLint.Scope.ScopeManager | null,
  seen: Set<string>
): boolean {
  if (node.type === AST_NODE_TYPES.Identifier) {
    return isRequireLikeIdentifierReference(node, imports, aliases, scopeManager, seen);
  }

  return (
    isCreateRequireInitializer(node, imports, aliases) ||
    isNodeModuleRequireSource(node, imports, aliases, scopeManager)
  );
}

function isRequireLikeAliasReference(
  reference: AliasReference,
  aliases: Map<string, AliasReference>,
  seen: Set<string>
): boolean {
  if (reference.kind === "require") {
    return true;
  }

  if (reference.kind === "identifier") {
    return isRequireLikeIdentifier(reference.name, aliases, seen);
  }

  return (
    reference.kind === "member" &&
    reference.property === "require" &&
    isNodeModuleAliasReference(reference.object, aliases, seen)
  );
}

function isNodeModuleAliasReference(
  reference: AliasReference,
  aliases: Map<string, AliasReference>,
  seen: Set<string>
): boolean {
  if (reference.kind === "import") {
    return isNodeModuleNamespaceImport(reference.source, reference.importedName);
  }

  if (reference.kind !== "identifier") {
    return false;
  }

  if (seen.has(reference.name)) {
    return false;
  }

  const aliasReference = aliases.get(reference.name);

  if (!aliasReference) {
    return false;
  }

  seen.add(reference.name);
  const matches = isNodeModuleAliasReference(aliasReference, aliases, seen);
  seen.delete(reference.name);
  return matches;
}

function isNodeModuleRequireSource(
  node: TSESTree.Expression,
  imports: Map<string, ImportBinding>,
  aliases: Map<string, AliasReference>,
  scopeManager: TSESLint.Scope.ScopeManager | null
): boolean {
  if (node.type === AST_NODE_TYPES.MemberExpression) {
    return isModuleRequireMemberExpression(node, imports, aliases);
  }

  if (node.type !== AST_NODE_TYPES.Identifier) {
    return false;
  }

  const directImport = imports.get(node.name);

  if (directImport) {
    return isNodeModuleNamespaceImport(directImport.source, directImport.importedName);
  }

  if (!scopeManager) {
    return false;
  }

  const reference = resolvedReferenceForIdentifier(scopeManager, node);
  const definitionNode = reference?.resolved?.defs?.[0]?.node;

  return (
    definitionNode?.type === AST_NODE_TYPES.VariableDeclarator &&
    definitionNode.id.type === AST_NODE_TYPES.Identifier &&
    definitionNode.init != null &&
    isNodeModuleRequireSource(definitionNode.init, imports, aliases, scopeManager)
  );
}

function objectPatternPropertyForLocal(
  pattern: TSESTree.ObjectPattern,
  localName: string
): string | null {
  for (const property of pattern.properties) {
    if (property.type !== AST_NODE_TYPES.Property || property.computed) {
      continue;
    }

    if (getPatternLocalName(property.value) !== localName) {
      continue;
    }

    return getObjectPatternPropertyName(property.key);
  }

  return null;
}

function isCreateRequireCall(
  node: TSESTree.CallExpression,
  imports: Map<string, ImportBinding>,
  aliases: Map<string, AliasReference>
): boolean {
  return isCreateRequireInitializer(node, imports, aliases);
}

function isCreateRequireInitializer(
  node: TSESTree.Expression,
  imports: Map<string, ImportBinding>,
  aliases: Map<string, AliasReference>
): boolean {
  if (node.type !== AST_NODE_TYPES.CallExpression) {
    return false;
  }

  const reference = resolveReference(node.callee, imports, aliases);

  if (
    reference?.kind === "import" &&
    isNodeModuleCreateRequireImport(reference.source, reference.importedName)
  ) {
    return true;
  }

  return (
    reference?.kind === "member" &&
    reference.object.kind === "import" &&
    isNodeModuleNamespaceImport(reference.object.source, reference.object.importedName) &&
    reference.property === "createRequire"
  );
}

function isModuleRequireMemberExpression(
  node: TSESTree.MemberExpression,
  imports: Map<string, ImportBinding>,
  aliases: Map<string, AliasReference>
): boolean {
  const propertyName = getPropertyName(node);

  if (propertyName !== "require") {
    return false;
  }

  const objectReference = resolveReference(node.object, imports, aliases);

  return (
    objectReference?.kind === "import" &&
    isNodeModuleNamespaceImport(objectReference.source, objectReference.importedName)
  );
}

function isNodeModuleCreateRequireImport(source: string, importedName: string | "*"): boolean {
  return (source === "module" || source === "node:module") && importedName === "createRequire";
}

function isNodeModuleNamespaceImport(source: string, importedName: string | "*"): boolean {
  return (source === "module" || source === "node:module") &&
    (importedName === "*" || importedName === "default");
}
