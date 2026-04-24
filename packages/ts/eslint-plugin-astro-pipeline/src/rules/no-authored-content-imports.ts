import { AST_NODE_TYPES, ESLintUtils } from "@typescript-eslint/utils";

import {
  collectConstantStringBindings,
  collectImportBindings,
  collectSimpleAliases,
  findNodes,
  getStaticStringValue,
  isRequireLikeCall,
  listStaticImportSources,
  resolveStaticStringExpression
} from "../utils/ast-helpers.js";
import {
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
  (name) => `https://github.com/websmasher/guardrail3/tree/main/packages/ts/eslint-plugin-astro-pipeline#${name}`
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

  if (moduleRole.isApprovedLoader || moduleRole.isApprovedGeneratedArtifact) {
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
      const source = resolveStaticStringExpression(node.source, constants);

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

      const source = getStaticStringValue(firstArg);

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
