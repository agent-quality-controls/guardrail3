import { AST_NODE_TYPES, ESLintUtils } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";
import { simpleTraverse } from "@typescript-eslint/typescript-estree";

import { collectImportBindings } from "../utils/ast-helpers.js";
import {
  astroPipelineOptionsSchema,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";
import { matchesPathPolicy } from "../utils/path-policy.js";

type MessageIds =
  | "missingApprovedNames"
  | "missingParserName"
  | "missingZodImport"
  | "unexpectedExport"
  | "missingParserCall"
  | "invalidParserCall"
  | "missingZodSchema";

const createRule = ESLintUtils.RuleCreator(
  (name) =>
    `https://github.com/websmasher/guardrail3/tree/main/packages/ts/g3ts-eslint-plugin-astro-pipeline#${name}`
);

export default createRule<RuleOptionsTuple, MessageIds>({
  name: "mdx-component-wrapper-requires-zod-parse",
  meta: {
    type: "problem",
    docs: {
      description:
        "Require every approved MDX component-map export to validate props through a local Zod schema."
    },
    schema: astroPipelineOptionsSchema,
    messages: {
      missingApprovedNames:
        "`approvedMdxComponentNames` is empty. Configure every validated MDX component-map export explicitly.",
      missingParserName:
        "`mdxPropsParserName` is empty. Configure the prop parser helper name, for example `parseMdxComponentProps`.",
      missingZodImport:
        "MDX component-map modules must import `z` from `zod` so exported wrappers validate props at Astro build time.",
      unexpectedExport:
        "Runtime export `{{name}}` is not listed in `approvedMdxComponentNames` or `allowedMdxComponentMapExports`. Component maps must export only explicit validated MDX wrappers plus explicit non-component map exports.",
      missingParserCall:
        "MDX component-map export `{{name}}` must call `{{parserName}}(\"{{name}}\", schema, rawProps)` before rendering UI.",
      invalidParserCall:
        "MDX component-map export `{{name}}` calls `{{parserName}}`, but not with the required wrapper name, local Zod schema, and raw props parameter.",
      missingZodSchema:
        "MDX component-map export `{{name}}` must pass a local schema initialized from `z.object(...)` to `{{parserName}}`."
    }
  },
  defaultOptions: [{}],
  create(context) {
    const filename = context.filename;
    const options = resolveOptions(context.options[0]);

    if (!matchesPathPolicy(filename, options.approvedMdxComponentModules)) {
      return {};
    }

    const approvedNames = new Set(options.approvedMdxComponentNames);
    const allowedNonComponentExports = new Set(options.allowedMdxComponentMapExports);
    const parserName = options.mdxPropsParserName[0] ?? "";

    function reportProgramSetup(program: TSESTree.Program): void {
      if (approvedNames.size === 0) {
        context.report({ node: program, messageId: "missingApprovedNames" });
      }
      if (!parserName) {
        context.report({ node: program, messageId: "missingParserName" });
      }
      if (!importsZod(program)) {
        context.report({ node: program, messageId: "missingZodImport" });
      }
    }

    return {
      Program(program): void {
        reportProgramSetup(program);
        if (approvedNames.size === 0 || !parserName) {
          return;
        }

        const schemas = collectZodObjectSchemas(program);
        for (const exported of collectRuntimeExports(program)) {
          if (allowedNonComponentExports.has(exported.name)) {
            continue;
          }

          if (!approvedNames.has(exported.name)) {
            context.report({
              node: exported.node,
              messageId: "unexpectedExport",
              data: { name: exported.name }
            });
            continue;
          }

          const fn = exportedFunction(exported.node);
          if (!fn) {
            context.report({
              node: exported.node,
              messageId: "missingParserCall",
              data: { name: exported.name, parserName }
            });
            continue;
          }

          const rawParam = firstParamName(fn);
          const parserCall = findParserCall(fn, parserName);
          if (!parserCall) {
            context.report({
              node: exported.node,
              messageId: "missingParserCall",
              data: { name: exported.name, parserName }
            });
            continue;
          }

          const schemaName = validParserCallSchema(parserCall, exported.name, rawParam);
          if (!schemaName) {
            context.report({
              node: parserCall,
              messageId: "invalidParserCall",
              data: { name: exported.name, parserName }
            });
            continue;
          }

          if (!schemas.has(schemaName)) {
            context.report({
              node: parserCall,
              messageId: "missingZodSchema",
              data: { name: exported.name, parserName }
            });
          }
        }
      }
    };
  }
});

interface RuntimeExport {
  name: string;
  node: TSESTree.ExportNamedDeclaration | TSESTree.ExportDefaultDeclaration;
}

function collectRuntimeExports(program: TSESTree.Program): RuntimeExport[] {
  const exports: RuntimeExport[] = [];

  for (const statement of program.body) {
    if (statement.type === AST_NODE_TYPES.ExportDefaultDeclaration) {
      exports.push({ name: "default", node: statement });
      continue;
    }

    if (statement.type !== AST_NODE_TYPES.ExportNamedDeclaration) {
      continue;
    }

    if (statement.exportKind === "type") {
      continue;
    }

    if (statement.declaration?.type === AST_NODE_TYPES.FunctionDeclaration) {
      const name = statement.declaration.id?.name;
      if (name) {
        exports.push({ name, node: statement });
      }
      continue;
    }

    if (statement.declaration?.type === AST_NODE_TYPES.VariableDeclaration) {
      for (const declaration of statement.declaration.declarations) {
        if (declaration.id.type === AST_NODE_TYPES.Identifier) {
          exports.push({ name: declaration.id.name, node: statement });
        }
      }
      continue;
    }

    for (const specifier of statement.specifiers) {
      if ("exportKind" in specifier && specifier.exportKind === "type") {
        continue;
      }
      exports.push({ name: specifierName(specifier.exported), node: statement });
    }
  }

  return exports;
}

function exportedFunction(
  node: TSESTree.ExportNamedDeclaration | TSESTree.ExportDefaultDeclaration
): TSESTree.FunctionDeclaration | TSESTree.ArrowFunctionExpression | TSESTree.FunctionExpression | null {
  if (
    node.type === AST_NODE_TYPES.ExportNamedDeclaration &&
    node.declaration?.type === AST_NODE_TYPES.FunctionDeclaration
  ) {
    return node.declaration;
  }

  if (
    node.type === AST_NODE_TYPES.ExportNamedDeclaration &&
    node.declaration?.type === AST_NODE_TYPES.VariableDeclaration
  ) {
    const init = node.declaration.declarations[0]?.init;
    if (
      init?.type === AST_NODE_TYPES.ArrowFunctionExpression ||
      init?.type === AST_NODE_TYPES.FunctionExpression
    ) {
      return init;
    }
  }

  return null;
}

function firstParamName(
  fn: TSESTree.FunctionDeclaration | TSESTree.ArrowFunctionExpression | TSESTree.FunctionExpression
): string | null {
  const first = fn.params[0];
  return first?.type === AST_NODE_TYPES.Identifier ? first.name : null;
}

function findParserCall(
  fn: TSESTree.FunctionDeclaration | TSESTree.ArrowFunctionExpression | TSESTree.FunctionExpression,
  parserName: string
): TSESTree.CallExpression | null {
  let found: TSESTree.CallExpression | null = null;

  simpleTraverse(fn.body, {
    enter(node) {
      if (
        !found &&
        node.type === AST_NODE_TYPES.CallExpression &&
        node.callee.type === AST_NODE_TYPES.Identifier &&
        node.callee.name === parserName
      ) {
        found = node;
      }
    }
  });
  return found;
}

function validParserCallSchema(
  call: TSESTree.CallExpression,
  exportName: string,
  rawParam: string | null
): string | null {
  const [nameArg, schemaArg, rawArg] = call.arguments;
  if (
    nameArg?.type !== AST_NODE_TYPES.Literal ||
    nameArg.value !== exportName ||
    schemaArg?.type !== AST_NODE_TYPES.Identifier ||
    rawArg?.type !== AST_NODE_TYPES.Identifier ||
    rawArg.name !== rawParam
  ) {
    return null;
  }

  return schemaArg.name;
}

function collectZodObjectSchemas(program: TSESTree.Program): Set<string> {
  const schemas = new Set<string>();

  for (const statement of program.body) {
    if (statement.type !== AST_NODE_TYPES.VariableDeclaration) {
      continue;
    }
    for (const declaration of statement.declarations) {
      if (
        declaration.id.type === AST_NODE_TYPES.Identifier &&
        declaration.init &&
        expressionRootedAtZObject(declaration.init)
      ) {
        schemas.add(declaration.id.name);
      }
    }
  }

  return schemas;
}

function expressionRootedAtZObject(node: TSESTree.Expression): boolean {
  if (
    node.type === AST_NODE_TYPES.CallExpression &&
    node.callee.type === AST_NODE_TYPES.MemberExpression &&
    node.callee.object.type === AST_NODE_TYPES.Identifier &&
    node.callee.object.name === "z" &&
    node.callee.property.type === AST_NODE_TYPES.Identifier &&
    node.callee.property.name === "object"
  ) {
    return true;
  }

  if (
    node.type === AST_NODE_TYPES.CallExpression &&
    node.callee.type === AST_NODE_TYPES.MemberExpression
  ) {
    return expressionRootedAtZObject(node.callee.object);
  }

  return false;
}

function importsZod(program: TSESTree.Program): boolean {
  for (const binding of collectImportBindings(program).values()) {
    if (binding.source === "zod" && binding.importedName === "z") {
      return true;
    }
  }

  return false;
}

function specifierName(
  node: TSESTree.ExportSpecifier["local"] | TSESTree.ExportSpecifier["exported"]
): string {
  return node.type === AST_NODE_TYPES.Identifier ? node.name : String(node.value);
}
