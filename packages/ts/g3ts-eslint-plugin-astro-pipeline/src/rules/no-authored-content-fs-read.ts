import path from "node:path";

import type { RuleContext } from "@typescript-eslint/utils/ts-eslint";
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
  isUnresolvedIdentifierReference,
  resolveReference,
  resolveStaticStringExpression,
  unwrapExpression
} from "../utils/ast-helpers.js";
import {
  couldResolveToAuthoredOrSpecContent,
  resolvesToApprovedGeneratedArtifact,
  resolvesToAuthoredOrSpecContent
} from "../utils/content-source.js";
import { collectImportClosure } from "../utils/import-closure.js";
import { describeApprovedLoaderOrAdapterSurface } from "../utils/message-surfaces.js";
import { resolveImportedModuleBinding, type ResolvedModuleBinding } from "../utils/module-exports.js";
import { classifyModuleRole } from "../utils/module-role.js";
import {
  astroPipelineOptionsSchema,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";

type MessageIds = "forbiddenRead";

const createRule = ESLintUtils.RuleCreator(
  (name) => `https://github.com/websmasher/guardrail3/tree/main/packages/ts/g3ts-eslint-plugin-astro-pipeline#${name}`
);

export default createRule<RuleOptionsTuple, MessageIds>({
  name: "no-authored-content-fs-read",
  meta: {
    type: "problem",
    docs: {
      description:
        "Disallow route and endpoint import closures from reading authored or spec content directly from the filesystem."
    },
    schema: astroPipelineOptionsSchema,
    messages: {
      forbiddenRead:
        "{{module}} reads authored or spec content at {{target}} via {{method}} in this route import closure. Move that read into {{surface}} and import the typed result from there instead. Routes must stay off raw content files so content goes through one validated pipeline."
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
        const offendingReads = modules.flatMap((moduleRecord) =>
          findForbiddenReads(
            modules,
            moduleRecord.program,
            moduleRecord.scopeManager,
            moduleRecord.filename,
            options
          )
        );

        for (const read of offendingReads) {
          context.report({
            node: programNode,
            messageId: "forbiddenRead",
            data: {
              method: read.method,
              module: read.modulePath,
              surface: describeApprovedLoaderOrAdapterSurface(options),
              target: read.target
            }
          });
        }
      }
    };
  }
});

interface ForbiddenRead {
  method: string;
  modulePath: string;
  target: string;
}

function findForbiddenReads(
  modules: ReturnType<typeof collectImportClosure>,
  program: TSESTree.Program,
  scopeManager: ReturnType<typeof collectImportClosure>[number]["scopeManager"],
  filename: string,
  options: ReturnType<typeof resolveOptions>
): ForbiddenRead[] {
  const moduleRole = classifyModuleRole(filename, options);

  if (
    moduleRole.isApprovedContentAdapter ||
    moduleRole.isApprovedLoader ||
    moduleRole.isApprovedGeneratedArtifact
  ) {
    return [];
  }

  const imports = collectImportBindings(program);
  const aliases = collectSimpleAliases(program);
  const constants = collectConstantBindings(program);
  const findings: ForbiddenRead[] = [];

  findNodes(program, (node) => {
    if (node.type !== AST_NODE_TYPES.CallExpression) {
      return;
    }

    const resolvedReference = resolveReference(node.callee, imports, aliases);
    const methodName = classifyFsReadReference(modules, filename, resolvedReference);

    if (!methodName) {
      return;
    }

    const targetLiteral = resolveStaticPathLike(
      node.arguments[0] as TSESTree.Expression,
      imports,
      aliases,
      constants,
      scopeManager
    );

    const targetPrefix = targetLiteral
      ? null
      : resolveStaticPathPrefix(
          node.arguments[0] as TSESTree.Expression,
          constants,
          scopeManager
        );

    if (!targetLiteral && !targetPrefix) {
      return;
    }

    if (targetLiteral) {
      if (
        resolvesToApprovedGeneratedArtifact(targetLiteral, filename, options) ||
        !resolvesToAuthoredOrSpecContent(targetLiteral, filename, options)
      ) {
        return;
      }
    } else if (
      !couldResolveToAuthoredOrSpecContent(targetPrefix!, filename, options)
    ) {
      return;
    }

    findings.push({
      method: methodName,
      modulePath: filename,
      target: targetLiteral ?? targetPrefix!
    });
  });

  return findings;
}

function resolveStaticPathPrefix(
  node: TSESTree.Expression | TSESTree.PrivateIdentifier | null | undefined,
  constants: Map<string, TSESTree.Expression>,
  scopeManager: ReturnType<typeof collectImportClosure>[number]["scopeManager"],
  seen = new Set<string>()
): string | null {
  if (!node || node.type === AST_NODE_TYPES.PrivateIdentifier) {
    return null;
  }

  const unwrapped = unwrapExpression(node);
  if (unwrapped !== node) {
    return resolveStaticPathPrefix(unwrapped, constants, scopeManager, seen);
  }

  if (node.type === AST_NODE_TYPES.TemplateLiteral) {
    return node.quasis[0]?.value.cooked ?? null;
  }

  if (node.type === AST_NODE_TYPES.BinaryExpression && node.operator === "+") {
    return resolveStaticPathPrefix(node.left, constants, scopeManager, seen);
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
    const resolved = resolveStaticPathPrefix(target, constants, scopeManager, seen);
    seen.delete(node.name);
    return resolved;
  }

  if (
    node.type === AST_NODE_TYPES.MemberExpression &&
    getPropertyName(node) === "pathname" &&
    node.object.type !== AST_NODE_TYPES.Super
  ) {
    return resolveStaticPathPrefix(node.object, constants, scopeManager, seen);
  }

  if (
    node.type !== AST_NODE_TYPES.NewExpression ||
    node.callee.type !== AST_NODE_TYPES.Identifier ||
    node.callee.name !== "URL" ||
    node.arguments.length < 2
  ) {
    return null;
  }

  const target = node.arguments[0];

  if (target.type === AST_NODE_TYPES.SpreadElement) {
    return null;
  }

  return resolveStaticPathPrefix(target, constants, scopeManager, seen);
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

  if (node.type === AST_NODE_TYPES.CallExpression) {
    return (
      resolveRequireResolveLikeCall(node, imports, aliases, constants, scopeManager, seen) ??
      resolvePathJoinLikeCall(node, imports, aliases, constants, scopeManager, seen)
    );
  }

  if (node.type === AST_NODE_TYPES.NewExpression) {
    return resolveFileUrlLike(node, imports, aliases, constants, scopeManager, seen);
  }

  if (node.type === AST_NODE_TYPES.MemberExpression) {
    return resolvePathMemberExpression(node, imports, aliases, constants, scopeManager, seen);
  }

  return null;
}

function resolvePathMemberExpression(
  node: TSESTree.MemberExpression,
  imports: Map<string, ReturnType<typeof collectImportBindings> extends Map<string, infer T> ? T : never>,
  aliases: ReturnType<typeof collectSimpleAliases>,
  constants: Map<string, TSESTree.Expression>,
  scopeManager: ReturnType<typeof collectImportClosure>[number]["scopeManager"],
  seen: Set<string>
): string | null {
  if (getPropertyName(node) !== "pathname" || node.object.type === AST_NODE_TYPES.Super) {
    return null;
  }

  return resolveStaticPathLike(
    node.object,
    imports,
    aliases,
    constants,
    scopeManager,
    seen
  );
}

function resolveRequireResolveLikeCall(
  node: TSESTree.CallExpression,
  imports: Map<string, ReturnType<typeof collectImportBindings> extends Map<string, infer T> ? T : never>,
  aliases: ReturnType<typeof collectSimpleAliases>,
  constants: Map<string, TSESTree.Expression>,
  scopeManager: ReturnType<typeof collectImportClosure>[number]["scopeManager"],
  seen: Set<string>
): string | null {
  if (!isRequireResolveLikeCall(node, imports, aliases, scopeManager)) {
    return null;
  }

  const firstArgument = node.arguments[0];

  if (!firstArgument || firstArgument.type === AST_NODE_TYPES.SpreadElement) {
    return null;
  }

  return resolveStaticPathLike(firstArgument, imports, aliases, constants, scopeManager, seen);
}

function isRequireResolveLikeCall(
  node: TSESTree.CallExpression,
  imports: Map<string, ReturnType<typeof collectImportBindings> extends Map<string, infer T> ? T : never>,
  aliases: ReturnType<typeof collectSimpleAliases>,
  scopeManager: ReturnType<typeof collectImportClosure>[number]["scopeManager"]
): boolean {
  if (node.callee.type !== AST_NODE_TYPES.MemberExpression) {
    return false;
  }

  const propertyName = getPropertyName(node.callee);
  if (propertyName !== "resolve") {
    return false;
  }

  const object = node.callee.object;
  if (object.type !== AST_NODE_TYPES.Identifier) {
    return false;
  }

  if (
    object.name === "require" &&
    isUnresolvedIdentifierReference(scopeManager, object)
  ) {
    return true;
  }

  return resolveReference(object, imports, aliases)?.kind === "require";
}

function resolvePathJoinLikeCall(
  node: TSESTree.CallExpression,
  imports: Map<string, ReturnType<typeof collectImportBindings> extends Map<string, infer T> ? T : never>,
  aliases: ReturnType<typeof collectSimpleAliases>,
  constants: Map<string, TSESTree.Expression>,
  scopeManager: ReturnType<typeof collectImportClosure>[number]["scopeManager"],
  seen: Set<string>
): string | null {
  const reference = resolveReference(node.callee, imports, aliases);

  if (reference && isNodeUrlFileUrlToPathLike(reference)) {
    const firstArgument = node.arguments[0];

    if (!firstArgument || firstArgument.type === AST_NODE_TYPES.SpreadElement) {
      return null;
    }

    return resolveStaticPathLike(
      firstArgument,
      imports,
      aliases,
      constants,
      scopeManager,
      seen
    );
  }

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

function resolveFileUrlLike(
  node: TSESTree.NewExpression,
  imports: Map<string, ReturnType<typeof collectImportBindings> extends Map<string, infer T> ? T : never>,
  aliases: ReturnType<typeof collectSimpleAliases>,
  constants: Map<string, TSESTree.Expression>,
  scopeManager: ReturnType<typeof collectImportClosure>[number]["scopeManager"],
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

  return resolveStaticPathLike(target, imports, aliases, constants, scopeManager, seen);
}

function isNodeUrlFileUrlToPathLike(
  reference: ReturnType<typeof resolveReference>
): boolean {
  if (!reference) {
    return false;
  }

  if (reference.kind === "import") {
    return isNodeUrlModule(reference.source) && reference.importedName === "fileURLToPath";
  }

  return (
    reference.kind === "member" &&
    reference.object.kind === "import" &&
    isNodeUrlModule(reference.object.source) &&
    (reference.object.importedName === "*" ||
      reference.object.importedName === "default") &&
    reference.property === "fileURLToPath"
  );
}

function isNodeUrlModule(source: string): boolean {
  return source === "url" || source === "node:url";
}

function isImportMetaUrlLike(
  node: TSESTree.Expression,
  constants: Map<string, TSESTree.Expression>,
  scopeManager: ReturnType<typeof collectImportClosure>[number]["scopeManager"],
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

function classifyFsReadReference(
  modules: ReturnType<typeof collectImportClosure>,
  importerFilename: string,
  reference: ReturnType<typeof resolveReference>
): string | null {
  if (!reference) {
    return null;
  }

  if (reference.kind === "member" && reference.object.kind === "import") {
    const importedObjectBinding = resolveImportedModuleBinding(
      modules,
      importerFilename,
      reference.object
    );

    if (importedObjectBinding) {
      return classifyFsMemberBinding(modules, importedObjectBinding, reference.property);
    }
  }

  if (reference.kind === "import") {
    if (
      isNodeFsPromises(reference.source) &&
      (reference.importedName === "readFile" || reference.importedName === "open")
    ) {
      return String(reference.importedName);
    }

    if (
      isNodeFs(reference.source) &&
      (reference.importedName === "readFile" ||
        reference.importedName === "readFileSync" ||
        reference.importedName === "createReadStream" ||
        reference.importedName === "open" ||
        reference.importedName === "openSync")
    ) {
      return String(reference.importedName);
    }
  } else if (reference.kind === "member") {
    const object = reference.object;

    if (
      object.kind === "import" &&
      (object.importedName === "*" || object.importedName === "default")
    ) {
      if (
        isNodeFsPromises(object.source) &&
        (reference.property === "readFile" || reference.property === "open")
      ) {
        return reference.property;
      }

      if (
        isNodeFs(object.source) &&
        (reference.property === "readFile" ||
          reference.property === "readFileSync" ||
          reference.property === "createReadStream" ||
          reference.property === "open" ||
          reference.property === "openSync")
      ) {
        return reference.property;
      }
    }

    if (
      object.kind === "import" &&
      isNodeFs(object.source) &&
      object.importedName === "promises" &&
      (reference.property === "readFile" || reference.property === "open")
    ) {
      return `promises.${reference.property}`;
    }

    if (
      object.kind === "member" &&
      object.object.kind === "import" &&
      (object.object.importedName === "*" ||
        object.object.importedName === "default") &&
      isNodeFs(object.object.source) &&
      object.property === "promises" &&
      (reference.property === "readFile" || reference.property === "open")
    ) {
      return `fs.promises.${reference.property}`;
    }
  }

  const importedBinding = resolveImportedModuleBinding(modules, importerFilename, reference);

  if (!importedBinding) {
    return null;
  }

  return classifyResolvedModuleBinding(modules, importedBinding);
}

function classifyFsMemberBinding(
  modules: ReturnType<typeof collectImportClosure>,
  binding: ResolvedModuleBinding,
  property: string
): string | null {
  if (binding.kind === "reference") {
    return classifyFsReadReference(modules, binding.moduleRecord.filename, {
      kind: "member",
      object: binding.reference,
      property
    });
  }

  const imports = collectImportBindings(binding.moduleRecord.program);
  const aliases = collectSimpleAliases(binding.moduleRecord.program);
  const resolvedReference = resolveReference(binding.expression, imports, aliases);

  if (!resolvedReference) {
    return null;
  }

  return classifyFsReadReference(modules, binding.moduleRecord.filename, {
    kind: "member",
    object: resolvedReference,
    property
  });
}

function classifyResolvedModuleBinding(
  modules: ReturnType<typeof collectImportClosure>,
  binding: ResolvedModuleBinding
): string | null {
  if (binding.kind === "reference") {
    return classifyFsReadReference(modules, binding.moduleRecord.filename, binding.reference);
  }

  const imports = collectImportBindings(binding.moduleRecord.program);
  const aliases = collectSimpleAliases(binding.moduleRecord.program);
  const resolvedReference = resolveReference(binding.expression, imports, aliases);

  if (!resolvedReference) {
    return null;
  }

  return classifyFsReadReference(modules, binding.moduleRecord.filename, resolvedReference);
}

function isNodeFs(source: string): boolean {
  return source === "fs" || source === "node:fs";
}

function isNodeFsPromises(source: string): boolean {
  return source === "fs/promises" || source === "node:fs/promises";
}
