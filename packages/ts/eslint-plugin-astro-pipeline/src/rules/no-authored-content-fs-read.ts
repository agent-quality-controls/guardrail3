import type { RuleContext } from "@typescript-eslint/utils/ts-eslint";
import { AST_NODE_TYPES, ESLintUtils } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

import {
  collectImportBindings,
  collectSimpleAliases,
  findNodes,
  getStaticStringValue,
  resolveReference
} from "../utils/ast-helpers.js";
import {
  resolvesToApprovedGeneratedArtifact,
  resolvesToAuthoredOrSpecContent
} from "../utils/content-source.js";
import { collectImportClosure } from "../utils/import-closure.js";
import { classifyModuleRole } from "../utils/module-role.js";
import {
  astroPipelineOptionsSchema,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";

type MessageIds = "forbiddenRead";

const createRule = ESLintUtils.RuleCreator(
  (name) => `https://github.com/websmasher/guardrail3/tree/main/packages/ts/eslint-plugin-astro-pipeline#${name}`
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
        "Route or endpoint import closures must not read authored or spec content via {{method}}. Found {{method}} in {{module}} targeting {{target}}."
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

        const modules = collectImportClosure(filename, context.sourceCode.text);
        const offendingReads = modules.flatMap((moduleRecord) =>
          findForbiddenReads(moduleRecord.program, moduleRecord.filename, options)
        );

        for (const read of offendingReads) {
          context.report({
            node: programNode,
            messageId: "forbiddenRead",
            data: {
              method: read.method,
              module: read.modulePath,
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
  program: TSESTree.Program,
  filename: string,
  options: ReturnType<typeof resolveOptions>
): ForbiddenRead[] {
  const moduleRole = classifyModuleRole(filename, options);

  if (moduleRole.isApprovedLoader || moduleRole.isApprovedGeneratedArtifact) {
    return [];
  }

  const imports = collectImportBindings(program);
  const aliases = collectSimpleAliases(program);
  const findings: ForbiddenRead[] = [];

  findNodes(program, (node) => {
    if (node.type !== AST_NODE_TYPES.CallExpression) {
      return;
    }

    const resolvedReference = resolveReference(node.callee, imports, aliases);
    const methodName = classifyFsReadReference(resolvedReference);

    if (!methodName) {
      return;
    }

    const targetLiteral = getStaticStringValue(node.arguments[0] as TSESTree.Expression);

    if (!targetLiteral) {
      return;
    }

    if (
      resolvesToApprovedGeneratedArtifact(targetLiteral, filename, options) ||
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

function classifyFsReadReference(
  reference: ReturnType<typeof resolveReference>
): string | null {
  if (!reference) {
    return null;
  }

  if (reference.kind === "import") {
    if (isNodeFsPromises(reference.source) && reference.importedName === "readFile") {
      return "readFile";
    }

    if (
      isNodeFs(reference.source) &&
      (reference.importedName === "readFile" ||
        reference.importedName === "readFileSync")
    ) {
      return String(reference.importedName);
    }

    return null;
  }

  const object = reference.object;

  if (object.kind === "import" && object.importedName === "*") {
    if (
      isNodeFsPromises(object.source) &&
      reference.property === "readFile"
    ) {
      return "readFile";
    }

    if (
      isNodeFs(object.source) &&
      (reference.property === "readFile" || reference.property === "readFileSync")
    ) {
      return reference.property;
    }
  }

  if (
    object.kind === "import" &&
    isNodeFs(object.source) &&
    object.importedName === "promises" &&
    reference.property === "readFile"
  ) {
    return "promises.readFile";
  }

  if (
    object.kind === "member" &&
    object.object.kind === "import" &&
    object.object.importedName === "*" &&
    isNodeFs(object.object.source) &&
    object.property === "promises" &&
    reference.property === "readFile"
  ) {
    return "fs.promises.readFile";
  }

  return null;
}

function isNodeFs(source: string): boolean {
  return source === "fs" || source === "node:fs";
}

function isNodeFsPromises(source: string): boolean {
  return source === "fs/promises" || source === "node:fs/promises";
}
