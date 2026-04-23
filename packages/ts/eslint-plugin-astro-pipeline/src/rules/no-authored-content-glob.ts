import path from "node:path";

import { AST_NODE_TYPES, ESLintUtils } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

import {
  collectImportBindings,
  collectSimpleAliases,
  findNodes,
  getPropertyName,
  getStaticStringValue,
  resolveReference
} from "../utils/ast-helpers.js";
import { resolvesToAuthoredOrSpecContent } from "../utils/content-source.js";
import { collectImportClosure } from "../utils/import-closure.js";
import { classifyModuleRole } from "../utils/module-role.js";
import {
  astroPipelineOptionsSchema,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";

type MessageIds = "forbiddenGlob";

const createRule = ESLintUtils.RuleCreator(
  (name) => `https://github.com/websmasher/guardrail3/tree/main/packages/ts/eslint-plugin-astro-pipeline#${name}`
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
        "Route or endpoint import closures must not use {{method}} over authored or spec content. Found {{method}} in {{module}} targeting {{target}}."
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

        const findings = collectImportClosure(filename, context.sourceCode.text, {
          program: context.sourceCode.ast,
          scopeManager: context.sourceCode.scopeManager ?? null
        }).flatMap(
          (moduleRecord) => findForbiddenGlobs(moduleRecord.program, moduleRecord.filename, options)
        );

        for (const finding of findings) {
          context.report({
            node: programNode,
            messageId: "forbiddenGlob",
            data: {
              method: finding.method,
              module: finding.modulePath,
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
  program: TSESTree.Program,
  filename: string,
  options: ReturnType<typeof resolveOptions>
): ForbiddenGlob[] {
  const moduleRole = classifyModuleRole(filename, options);

  if (moduleRole.isApprovedContentAdapter) {
    return [];
  }

  const findings: ForbiddenGlob[] = [];
  const globAliases = collectGlobAliases(program);
  const imports = collectImportBindings(program);
  const aliases = collectSimpleAliases(program);
  const constants = collectConstantBindings(program);

  findNodes(program, (node) => {
    if (node.type !== AST_NODE_TYPES.CallExpression) {
      return;
    }

    const methodName = getImportMetaGlobMethod(node.callee, globAliases);

    if (!methodName) {
      return;
    }

    const targetLiteral = resolveStaticPathLike(
      node.arguments[0] as TSESTree.Expression,
      imports,
      aliases,
      constants
    );

    if (
      !targetLiteral ||
      !resolvesToAuthoredOrSpecContent(targetLiteral, filename, options)
    ) {
      return;
    }

    findings.push({
      method: methodName,
      modulePath: filename,
      target: targetLiteral
    });
  });

  return findings;
}

function getImportMetaGlobMethod(
  callee: TSESTree.CallExpression["callee"],
  globAliases: Map<string, string>
): string | null {
  if (
    callee.type === AST_NODE_TYPES.Identifier &&
    globAliases.has(callee.name)
  ) {
    return globAliases.get(callee.name) ?? null;
  }

  if (callee.type !== AST_NODE_TYPES.MemberExpression) {
    return null;
  }

  if (
    callee.object.type !== AST_NODE_TYPES.MetaProperty ||
    callee.object.meta.name !== "import" ||
    callee.object.property.name !== "meta"
  ) {
    return null;
  }

  const propertyName = getPropertyName(callee);

  if (propertyName === "glob" || propertyName === "globEager") {
    return `import.meta.${propertyName}`;
  }

  return null;
}

function collectGlobAliases(program: TSESTree.Program): Map<string, string> {
  const aliases = new Map<string, string>();

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

      const methodName = getImportMetaGlobMethod(node.init, aliases);

      if (methodName) {
        aliases.set(node.id.name, methodName);
        changed = true;
      }
    });
  }

  return aliases;
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
  seen = new Set<string>()
): string | null {
  const direct = getStaticStringValue(node);

  if (direct) {
    return direct;
  }

  if (!node || node.type === AST_NODE_TYPES.PrivateIdentifier) {
    return null;
  }

  if (node.type === AST_NODE_TYPES.Identifier) {
    if (seen.has(node.name)) {
      return null;
    }

    const target = constants.get(node.name);

    if (!target) {
      return null;
    }

    seen.add(node.name);
    const resolved = resolveStaticPathLike(target, imports, aliases, constants, seen);
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

      if (index === 0 && isProcessCwdCall(argument)) {
        return "";
      }

      return resolveStaticPathLike(argument, imports, aliases, constants, seen);
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

function isProcessCwdCall(node: TSESTree.Expression): boolean {
  return (
    node.type === AST_NODE_TYPES.CallExpression &&
    node.arguments.length === 0 &&
    node.callee.type === AST_NODE_TYPES.MemberExpression &&
    node.callee.object.type === AST_NODE_TYPES.Identifier &&
    node.callee.object.name === "process" &&
    !node.callee.computed &&
    node.callee.property.type === AST_NODE_TYPES.Identifier &&
    node.callee.property.name === "cwd"
  );
}
