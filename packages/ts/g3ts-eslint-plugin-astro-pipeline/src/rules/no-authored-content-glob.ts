import path from "node:path";

import { AST_NODE_TYPES, ESLintUtils } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

import {
  collectImportBindings,
  collectSimpleAliases,
  findNodes,
  getIdentifierInitializer,
  getPropertyName,
  getStaticStringValue,
  isProcessCwdLikeCall,
  resolveReference,
  resolveStaticStringExpression,
  unwrapExpression
} from "../utils/ast-helpers.js";
import { resolvesToAuthoredOrSpecContent } from "../utils/content-source.js";
import { collectImportClosure } from "../utils/import-closure.js";
import { describeApprovedLoaderOrAdapterSurface } from "../utils/message-surfaces.js";
import { resolveImportedModuleBinding, type ResolvedModuleBinding } from "../utils/module-exports.js";
import { classifyModuleRole } from "../utils/module-role.js";
import {
  astroPipelineOptionsSchema,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";

type MessageIds = "forbiddenGlob";

const createRule = ESLintUtils.RuleCreator(
  (name) => `https://github.com/websmasher/guardrail3/tree/main/packages/ts/g3ts-eslint-plugin-astro-pipeline#${name}`
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
        "{{module}} matches authored or spec content at {{target}} via {{method}} in this route import closure. Move that glob into {{surface}} and import the typed result from there instead. Routes must stay off raw content discovery so content goes through one validated pipeline."
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

        const modules = collectImportClosure(filename, context.sourceCode.text, {
          program: context.sourceCode.ast,
          scopeManager: context.sourceCode.scopeManager ?? null
        });
        const findings = modules.flatMap((moduleRecord) =>
          findForbiddenGlobs(
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
            messageId: "forbiddenGlob",
            data: {
              method: finding.method,
              module: finding.modulePath,
              surface: describeApprovedLoaderOrAdapterSurface(options),
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
  modules: ReturnType<typeof collectImportClosure>,
  program: TSESTree.Program,
  scopeManager: ReturnType<typeof collectImportClosure>[number]["scopeManager"],
  filename: string,
  options: ReturnType<typeof resolveOptions>
): ForbiddenGlob[] {
  const moduleRole = classifyModuleRole(filename, options);

  if (
    moduleRole.isApprovedContentAdapter ||
    moduleRole.isApprovedLoader ||
    moduleRole.isApprovedGeneratedArtifact
  ) {
    return [];
  }

  const findings: ForbiddenGlob[] = [];
  const importMetaAliases = collectImportMetaAliases(program);
  const globAliases = collectGlobAliases(program, importMetaAliases);
  const imports = collectImportBindings(program);
  const aliases = collectSimpleAliases(program);
  const constants = collectConstantBindings(program);

  findNodes(program, (node) => {
    if (node.type !== AST_NODE_TYPES.CallExpression) {
      return;
    }

    const methodName = classifyGlobLikeExpression(
      modules,
      filename,
      node.callee,
      globAliases,
      importMetaAliases,
      imports,
      aliases
    );

    if (!methodName) {
      return;
    }

    const targetLiterals = resolveStaticPathLikes(
      node.arguments[0] as TSESTree.Expression,
      imports,
      aliases,
      constants,
      scopeManager
    );

    for (const targetLiteral of targetLiterals) {
      if (!resolvesToAuthoredOrSpecContent(targetLiteral, filename, options)) {
        continue;
      }

      findings.push({
        method: methodName,
        modulePath: filename,
        target: targetLiteral
      });
    }
  });

  return findings;
}

function classifyGlobLikeExpression(
  modules: ReturnType<typeof collectImportClosure>,
  importerFilename: string,
  expression: TSESTree.Expression | TSESTree.PrivateIdentifier,
  globAliases: Map<string, string>,
  importMetaAliases: Set<string>,
  imports: ReturnType<typeof collectImportBindings>,
  aliases: ReturnType<typeof collectSimpleAliases>
): string | null {
  if (expression.type === AST_NODE_TYPES.PrivateIdentifier) {
    return null;
  }

  if (
    expression.type === AST_NODE_TYPES.Identifier &&
    globAliases.has(expression.name)
  ) {
    return globAliases.get(expression.name) ?? null;
  }

  const resolvedReference = resolveReference(expression, imports, aliases);
  const importedBinding = resolveImportedModuleBinding(
    modules,
    importerFilename,
    resolvedReference
  );

  if (importedBinding) {
    return classifyGlobBinding(modules, importedBinding);
  }

  if (expression.type !== AST_NODE_TYPES.MemberExpression) {
    return null;
  }

  const propertyName = getPropertyName(expression);

  if (
    expression.object.type === AST_NODE_TYPES.Identifier &&
    importMetaAliases.has(expression.object.name) &&
    (propertyName === "glob" || propertyName === "globEager")
  ) {
    return `import.meta.${propertyName}`;
  }

  if (
    expression.object.type !== AST_NODE_TYPES.MetaProperty ||
    expression.object.meta.name !== "import" ||
    expression.object.property.name !== "meta"
  ) {
    return null;
  }

  if (propertyName === "glob" || propertyName === "globEager") {
    return `import.meta.${propertyName}`;
  }

  return null;
}

function classifyGlobBinding(
  modules: ReturnType<typeof collectImportClosure>,
  binding: ResolvedModuleBinding
): string | null {
  const imports = collectImportBindings(binding.moduleRecord.program);
  const aliases = collectSimpleAliases(binding.moduleRecord.program);
  const localImportMetaAliases = collectImportMetaAliases(binding.moduleRecord.program);
  const localGlobAliases = collectGlobAliases(
    binding.moduleRecord.program,
    localImportMetaAliases
  );

  if (binding.kind === "reference") {
    return null;
  }

  return classifyGlobLikeExpression(
    modules,
    binding.moduleRecord.filename,
    binding.expression,
    localGlobAliases,
    localImportMetaAliases,
    imports,
    aliases
  );
}

function collectGlobAliases(
  program: TSESTree.Program,
  importMetaAliases: Set<string>
): Map<string, string> {
  const aliases = new Map<string, string>();

  let changed = true;
  while (changed) {
    changed = false;
    findNodes(program, (node) => {
      if (node.type !== AST_NODE_TYPES.VariableDeclarator || !node.init) {
        return;
      }

      if (node.id.type === AST_NODE_TYPES.ObjectPattern) {
        if (
          collectImportMetaGlobPatternAliases(
            node.id,
            node.init,
            importMetaAliases,
            aliases
          )
        ) {
          changed = true;
        }
        return;
      }

      if (node.id.type !== AST_NODE_TYPES.Identifier || aliases.has(node.id.name)) {
        return;
      }

      const methodName = classifyGlobLikeExpression(
        [],
        "",
        node.init,
        aliases,
        importMetaAliases,
        new Map(),
        new Map()
      );

      if (methodName) {
        aliases.set(node.id.name, methodName);
        changed = true;
      }
    });
  }

  return aliases;
}

function collectImportMetaAliases(program: TSESTree.Program): Set<string> {
  const aliases = new Set<string>();

  let changed = true;
  while (changed) {
    changed = false;
    findNodes(program, (node) => {
      if (
        node.type !== AST_NODE_TYPES.VariableDeclarator ||
        node.id.type !== AST_NODE_TYPES.Identifier ||
        !node.init ||
        aliases.has(node.id.name)
      ) {
        return;
      }

      if (isImportMetaExpression(node.init, aliases)) {
        aliases.add(node.id.name);
        changed = true;
      }
    });
  }

  return aliases;
}

function collectImportMetaGlobPatternAliases(
  pattern: TSESTree.ObjectPattern,
  init: TSESTree.Expression,
  importMetaAliases: Set<string>,
  aliases: Map<string, string>
): boolean {
  if (!isImportMetaExpression(init, importMetaAliases)) {
    return false;
  }

  let changed = false;

  for (const property of pattern.properties) {
    if (property.type !== AST_NODE_TYPES.Property || property.kind !== "init") {
      continue;
    }

    const keyName = patternPropertyName(property);
    if (keyName !== "glob" && keyName !== "globEager") {
      continue;
    }

    const localName = patternLocalName(property.value);
    if (!localName || aliases.has(localName)) {
      continue;
    }

    aliases.set(localName, `import.meta.${keyName}`);
    changed = true;
  }

  return changed;
}

function isImportMetaExpression(
  expression: TSESTree.Expression,
  importMetaAliases: Set<string>
): boolean {
  const unwrappedExpression = unwrapExpression(expression);

  return (
    (unwrappedExpression.type === AST_NODE_TYPES.MetaProperty &&
      unwrappedExpression.meta.name === "import" &&
      unwrappedExpression.property.name === "meta") ||
    (unwrappedExpression.type === AST_NODE_TYPES.Identifier &&
      importMetaAliases.has(unwrappedExpression.name))
  );
}

function patternPropertyName(property: TSESTree.Property): string | null {
  if (property.computed) {
    return getStaticStringValue(property.key);
  }

  if (property.key.type === AST_NODE_TYPES.Identifier) {
    return property.key.name;
  }

  return getStaticStringValue(property.key);
}

function patternLocalName(pattern: TSESTree.Property["value"]): string | null {
  if (pattern.type === AST_NODE_TYPES.Identifier) {
    return pattern.name;
  }

  if (
    pattern.type === AST_NODE_TYPES.AssignmentPattern &&
    pattern.left.type === AST_NODE_TYPES.Identifier
  ) {
    return pattern.left.name;
  }

  return null;
}

function collectConstantBindings(
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

function resolveStaticPathLikes(
  node: TSESTree.Expression | TSESTree.PrivateIdentifier | null | undefined,
  imports: Map<string, ReturnType<typeof collectImportBindings> extends Map<string, infer T> ? T : never>,
  aliases: ReturnType<typeof collectSimpleAliases>,
  constants: Map<string, TSESTree.Expression>,
  scopeManager: ReturnType<typeof collectImportClosure>[number]["scopeManager"],
  seen = new Set<string>()
): string[] {
  if (!node || node.type === AST_NODE_TYPES.PrivateIdentifier) {
    return [];
  }

  const unwrapped = unwrapExpression(node);
  if (unwrapped !== node) {
    return resolveStaticPathLikes(unwrapped, imports, aliases, constants, scopeManager, seen);
  }

  if (node.type === AST_NODE_TYPES.ArrayExpression) {
    return node.elements.flatMap((element) => {
      if (!element || element.type === AST_NODE_TYPES.SpreadElement) {
        return [];
      }

      return resolveStaticPathLikes(element, imports, aliases, constants, scopeManager, seen);
    });
  }

  if (node.type === AST_NODE_TYPES.Identifier) {
    if (seen.has(node.name)) {
      return [];
    }

    const target =
      scopeManager == null ? constants.get(node.name) : getIdentifierInitializer(scopeManager, node);

    if (!target) {
      return [];
    }

    seen.add(node.name);
    const resolved = resolveStaticPathLikes(
      target,
      imports,
      aliases,
      constants,
      scopeManager,
      seen
    );
    seen.delete(node.name);
    return resolved;
  }

  const pathLike = resolveStaticPathLike(node, imports, aliases, constants, scopeManager, seen);
  return pathLike ? [pathLike] : [];
}

function resolveStaticPathLike(
  node: TSESTree.Expression | TSESTree.PrivateIdentifier | null | undefined,
  imports: Map<string, ReturnType<typeof collectImportBindings> extends Map<string, infer T> ? T : never>,
  aliases: ReturnType<typeof collectSimpleAliases>,
  constants: Map<string, TSESTree.Expression>,
  scopeManager: ReturnType<typeof collectImportClosure>[number]["scopeManager"],
  seen = new Set<string>()
): string | null {
  if (node && node.type !== AST_NODE_TYPES.PrivateIdentifier) {
    const staticValue = resolveStaticStringExpression(node, constants, new Set(), scopeManager);
    if (staticValue) {
      return staticValue;
    }
  }

  const direct = getStaticStringValue(node);

  if (direct) {
    return direct;
  }

  if (!node || node.type === AST_NODE_TYPES.PrivateIdentifier) {
    return null;
  }

  const unwrapped = unwrapExpression(node);
  if (unwrapped !== node) {
    return resolveStaticPathLike(unwrapped, imports, aliases, constants, scopeManager, seen);
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
    const resolved = resolveStaticPathLike(
      target,
      imports,
      aliases,
      constants,
      scopeManager,
      seen
    );
    seen.delete(node.name);
    return resolved;
  }

  if (node.type !== AST_NODE_TYPES.CallExpression) {
    return null;
  }

  const reference = resolveReference(node.callee, imports, aliases);

  if (!reference || !isNodePathJoinLike(reference)) {
    return null;
  }

  const parts = node.arguments
    .map((argument, index) => {
      if (argument.type === AST_NODE_TYPES.SpreadElement) {
        return null;
      }

      if (index === 0 && isProcessCwdLikeCall(argument, constants, scopeManager)) {
        return "";
      }

      return resolveStaticPathLike(argument, imports, aliases, constants, scopeManager, seen);
    });

  if (parts.some((part) => part == null)) {
    return null;
  }

  return path.posix.join(...(parts as string[]));
}

function isNodePathJoinLike(
  reference: ReturnType<typeof resolveReference>
): boolean {
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
    (reference.object.importedName === "*" || reference.object.importedName === "default") &&
    (reference.property === "join" || reference.property === "resolve")
  ) {
    return true;
  }

  if (
    reference.object.kind === "import" &&
    isNodePathModule(reference.object.source) &&
    reference.object.importedName === "posix" &&
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
