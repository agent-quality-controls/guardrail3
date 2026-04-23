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

  for (const statement of program.body) {
    if (statement.type !== AST_NODE_TYPES.VariableDeclaration) {
      continue;
    }

    for (const declaration of statement.declarations) {
      if (!declaration.init) {
        continue;
      }

      const aliasReference = expressionToAliasReference(declaration.init);

      if (declaration.id.type === AST_NODE_TYPES.Identifier && aliasReference) {
        aliases.set(declaration.id.name, aliasReference);
        continue;
      }

      if (declaration.id.type === AST_NODE_TYPES.ObjectPattern && aliasReference) {
        collectObjectPatternAliases(aliases, declaration.id, aliasReference);
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
  node: TSESTree.Expression
): AliasReference | null {
  if (node.type === AST_NODE_TYPES.CallExpression) {
    return requireCallToAliasReference(node);
  }

  if (node.type === AST_NODE_TYPES.Identifier) {
    return {
      kind: "identifier",
      name: node.name
    };
  }

  if (node.type !== AST_NODE_TYPES.MemberExpression) {
    return null;
  }

  const objectReference = expressionToAliasReference(node.object);
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
  node: TSESTree.CallExpression
): AliasReference | null {
  if (
    node.callee.type !== AST_NODE_TYPES.Identifier ||
    node.callee.name !== "require"
  ) {
    return null;
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
): void {
  for (const property of pattern.properties) {
    if (property.type !== AST_NODE_TYPES.Property || property.computed) {
      continue;
    }

    const propertyName = getObjectPatternPropertyName(property.key);
    const localName = getPatternLocalName(property.value);

    if (!propertyName || !localName) {
      continue;
    }

    aliases.set(localName, {
      kind: "member",
      object: sourceReference,
      property: propertyName
    });
  }
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
