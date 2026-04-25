import path from "node:path";

import { AST_NODE_TYPES, ESLintUtils } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

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
} from "../utils/ast-helpers.js";
import {
  resolvesToApprovedContentAdapter,
  resolvesToApprovedGeneratedArtifact,
  resolvesToAuthoredOrSpecContent
} from "../utils/content-source.js";
import { collectImportClosure } from "../utils/import-closure.js";
import { describeApprovedLoaderOrAdapterSurface } from "../utils/message-surfaces.js";
import { classifyModuleRole } from "../utils/module-role.js";
import {
  astroPipelineOptionsSchema,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";

type MessageIds = "forbiddenImport";

const createRule = ESLintUtils.RuleCreator(
  (name) => `https://github.com/websmasher/guardrail3/tree/main/packages/ts/g3ts-eslint-plugin-astro-pipeline#${name}`
);

export default createRule<RuleOptionsTuple, MessageIds>({
  name: "no-authored-content-imports",
  meta: {
    type: "problem",
    docs: {
      description:
        "Disallow route and endpoint import closures from importing authored or spec content directly."
    },
    schema: astroPipelineOptionsSchema,
    messages: {
      forbiddenImport:
        "{{module}} imports authored or spec content at {{target}} in this route import closure. Move that content access into {{surface}} and import the typed result from there instead. Routes must not import raw content modules directly because that bypasses the shared content pipeline."
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
          findForbiddenImports(moduleRecord, options)
        );

        for (const finding of findings) {
          context.report({
            node: programNode,
            messageId: "forbiddenImport",
            data: {
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

interface ForbiddenImport {
  modulePath: string;
  target: string;
}

function findForbiddenImports(
  moduleRecord: ReturnType<typeof collectImportClosure>[number],
  options: ReturnType<typeof resolveOptions>
): ForbiddenImport[] {
  const moduleRole = classifyModuleRole(moduleRecord.filename, options);

  if (
    moduleRole.isApprovedContentAdapter ||
    moduleRole.isApprovedLoader ||
    moduleRole.isApprovedGeneratedArtifact
  ) {
    return [];
  }

  const findings: ForbiddenImport[] = [];

  for (const source of listStaticImportSources(moduleRecord.program)) {
    if (isForbiddenContentImport(source, moduleRecord.filename, options)) {
      findings.push({
        modulePath: moduleRecord.filename,
        target: source
      });
    }
  }

  const constants = collectConstantStringBindings(moduleRecord.program);
  const imports = collectImportBindings(moduleRecord.program);
  const aliases = collectSimpleAliases(moduleRecord.program);

  findNodes(moduleRecord.program, (node) => {
    if (node.type === AST_NODE_TYPES.ImportExpression) {
      const source = resolveImportSourceExpression(
        node.source,
        constants,
        imports,
        aliases,
        new Set(),
        moduleRecord.scopeManager
      );

      if (isForbiddenContentImport(source, moduleRecord.filename, options)) {
        findings.push({
          modulePath: moduleRecord.filename,
          target: source!
        });
      }
      return;
    }

    if (
      node.type === AST_NODE_TYPES.CallExpression &&
      isRequireLikeCall(node, imports, aliases, moduleRecord.scopeManager)
    ) {
      const firstArg = node.arguments[0];

      if (!firstArg || firstArg.type === AST_NODE_TYPES.SpreadElement) {
        return;
      }

      const source = resolveImportSourceExpression(
        firstArg,
        constants,
        imports,
        aliases,
        new Set(),
        moduleRecord.scopeManager
      );

      if (isForbiddenContentImport(source, moduleRecord.filename, options)) {
        findings.push({
          modulePath: moduleRecord.filename,
          target: source!
        });
      }
    }
  });

  return dedupeFindings(findings);
}

function resolveImportSourceExpression(
  node: TSESTree.Expression,
  constants: Map<string, TSESTree.Expression>,
  imports: ReturnType<typeof collectImportBindings>,
  aliases: ReturnType<typeof collectSimpleAliases>,
  seen: Set<string>,
  scopeManager: ReturnType<typeof collectImportClosure>[number]["scopeManager"]
): string | null {
  const direct = resolveStaticStringExpression(node, constants, seen, scopeManager);

  if (direct) {
    return direct;
  }

  const unwrapped = unwrapExpression(node);
  if (unwrapped !== node) {
    return resolveImportSourceExpression(unwrapped, constants, imports, aliases, seen, scopeManager);
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
    const resolved = resolveImportSourceExpression(
      target,
      constants,
      imports,
      aliases,
      seen,
      scopeManager
    );
    seen.delete(node.name);
    return resolved;
  }

  if (
    node.type === AST_NODE_TYPES.MemberExpression &&
    getPropertyName(node) === "pathname" &&
    node.object.type !== AST_NODE_TYPES.Super
  ) {
    return resolveImportSourceExpression(
      node.object,
      constants,
      imports,
      aliases,
      seen,
      scopeManager
    );
  }

  if (node.type === AST_NODE_TYPES.CallExpression) {
    return resolvePathJoinLikeCall(node, constants, imports, aliases, seen, scopeManager);
  }

  if (node.type !== AST_NODE_TYPES.NewExpression) {
    return null;
  }

  if (
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

  return resolveImportSourceExpression(target, constants, imports, aliases, seen, scopeManager);
}

function resolvePathJoinLikeCall(
  node: TSESTree.CallExpression,
  constants: Map<string, TSESTree.Expression>,
  imports: ReturnType<typeof collectImportBindings>,
  aliases: ReturnType<typeof collectSimpleAliases>,
  seen: Set<string>,
  scopeManager: ReturnType<typeof collectImportClosure>[number]["scopeManager"]
): string | null {
  const reference = resolveReference(node.callee, imports, aliases);

  if (!isNodePathJoinLike(reference)) {
    return null;
  }

  const parts = node.arguments.map((argument, index) => {
    if (argument.type === AST_NODE_TYPES.SpreadElement) {
      return null;
    }

    if (index === 0 && isProcessCwdLikeCall(argument, constants, scopeManager)) {
      return "";
    }

    return resolveImportSourceExpression(
      argument,
      constants,
      imports,
      aliases,
      seen,
      scopeManager
    );
  });

  if (parts.some((part) => part == null)) {
    return null;
  }

  return path.posix.join(...(parts as string[]));
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

function isForbiddenContentImport(
  source: string | null | undefined,
  importerFilename: string,
  options: ReturnType<typeof resolveOptions>
): boolean {
  if (!source) {
    return false;
  }

  return (
    resolvesToAuthoredOrSpecContent(source, importerFilename, options) &&
    !resolvesToApprovedContentAdapter(source, importerFilename, options) &&
    !resolvesToApprovedGeneratedArtifact(source, importerFilename, options)
  );
}

function dedupeFindings(findings: ForbiddenImport[]): ForbiddenImport[] {
  const seen = new Set<string>();

  return findings.filter((finding) => {
    const key = `${finding.modulePath}:${finding.target}`;

    if (seen.has(key)) {
      return false;
    }

    seen.add(key);
    return true;
  });
}
