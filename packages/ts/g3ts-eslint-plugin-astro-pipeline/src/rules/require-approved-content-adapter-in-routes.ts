import { AST_NODE_TYPES, ESLintUtils } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

import {
  collectConstantStringBindings,
  collectImportBindings,
  collectSimpleAliases,
  isRequireLikeCall,
  resolveStaticStringExpression
} from "../utils/ast-helpers.js";
import { describeApprovedContentAdapterSurface } from "../utils/message-surfaces.js";
import { classifyModuleRole } from "../utils/module-role.js";
import {
  astroPipelineOptionsSchema,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";
import { matchesFileGlobs, resolvePathLike } from "../utils/path-policy.js";

type MessageIds = "missingAdapter";

const createRule = ESLintUtils.RuleCreator(
  (name) =>
    `https://github.com/websmasher/guardrail3/tree/main/packages/ts/g3ts-eslint-plugin-astro-pipeline#${name}`
);

export default createRule<RuleOptionsTuple, MessageIds>({
  name: "require-approved-content-adapter-in-routes",
  meta: {
    type: "problem",
    docs: {
      description:
        "Require public Astro page routes to import an approved content adapter module."
    },
    schema: astroPipelineOptionsSchema,
    messages: {
      missingAdapter:
        "{{module}} does not import {{surface}}. Public Astro page routes must load page data through the approved content adapter so authored copy stays in Astro content collections."
    }
  },
  defaultOptions: [{}],
  create(context) {
    const filename = context.filename;
    const options = resolveOptions(context.options[0]);
    const moduleRole = classifyModuleRole(filename, options);

    if (
      !moduleRole.isRoute ||
      moduleRole.isEndpoint ||
      moduleRole.isApprovedContentAdapter
    ) {
      return {};
    }

    const importAliases = collectConstantStringBindings(context.sourceCode.ast);
    const imports = collectImportBindings(context.sourceCode.ast);
    const requireAliases = collectSimpleAliases(context.sourceCode.ast);
    const scopeManager = context.sourceCode.scopeManager ?? null;
    let importsApprovedAdapter = false;

    function markIfApprovedSource(rawSource: string): void {
      if (importsApprovedAdapter) {
        return;
      }

      importsApprovedAdapter = sourceMatchesApprovedAdapter(
        filename,
        rawSource,
        options.approvedContentAdapterModules
      );
    }

    function visitStaticModuleEdge(
      node: TSESTree.ImportDeclaration
    ): void {
      const source = node.source?.value;

      if (typeof source === "string") {
        markIfApprovedSource(source);
      }
    }

    return {
      ImportDeclaration(node): void {
        if (hasRuntimeImportSpecifiers(node)) {
          visitStaticModuleEdge(node);
        }
      },
      ImportExpression(node): void {
        const source = resolveStaticStringExpression(
          node.source,
          importAliases,
          new Set(),
          scopeManager
        );
        if (source) {
          markIfApprovedSource(source);
        }
      },
      CallExpression(node): void {
        if (
          isRequireLikeCall(
            node,
            imports,
            requireAliases,
            scopeManager
          ) &&
          node.arguments.length > 0 &&
          node.arguments[0]?.type !== AST_NODE_TYPES.SpreadElement
        ) {
          const source = resolveStaticStringExpression(
            node.arguments[0],
            importAliases,
            new Set(),
            scopeManager
          );
          if (source) {
            markIfApprovedSource(source);
          }
        }
      },
      "Program:exit"(programNode): void {
        if (importsApprovedAdapter) {
          return;
        }

        context.report({
          node: programNode,
          messageId: "missingAdapter",
          data: {
            module: filename,
            surface: describeApprovedContentAdapterSurface(options)
          }
        });
      }
    };
  }
});

function sourceMatchesApprovedAdapter(
  importerFilename: string,
  rawSource: string,
  approvedContentAdapterModules: readonly string[]
): boolean {
  if (approvedContentAdapterModules.length === 0) {
    return false;
  }

  const resolvedPath = resolvePathLike(importerFilename, rawSource);

  return adapterCandidatePaths(resolvedPath).some((candidatePath) =>
    matchesFileGlobs(candidatePath, approvedContentAdapterModules)
  );
}

function adapterCandidatePaths(resolvedPath: string): string[] {
  if (/\.[cm]?[jt]sx?$/.test(resolvedPath) || resolvedPath.endsWith(".astro")) {
    return [resolvedPath];
  }

  return [
    resolvedPath,
    `${resolvedPath}.ts`,
    `${resolvedPath}.tsx`,
    `${resolvedPath}.mts`,
    `${resolvedPath}.cts`,
    `${resolvedPath}.js`,
    `${resolvedPath}.jsx`,
    `${resolvedPath}.mjs`,
    `${resolvedPath}.cjs`,
    `${resolvedPath}/index.ts`,
    `${resolvedPath}/index.tsx`,
    `${resolvedPath}/index.mts`,
    `${resolvedPath}/index.cts`,
    `${resolvedPath}/index.js`,
    `${resolvedPath}/index.jsx`,
    `${resolvedPath}/index.mjs`,
    `${resolvedPath}/index.cjs`
  ];
}

function isTypeOnlyImportDeclaration(node: TSESTree.ImportDeclaration): boolean {
  return (
    node.importKind === "type" ||
    (node.specifiers.length > 0 &&
      node.specifiers.every(
        (specifier) =>
          specifier.type === AST_NODE_TYPES.ImportSpecifier &&
          specifier.importKind === "type"
      ))
  );
}

function hasRuntimeImportSpecifiers(node: TSESTree.ImportDeclaration): boolean {
  return !isTypeOnlyImportDeclaration(node) && node.specifiers.length > 0;
}
