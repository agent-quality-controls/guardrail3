import { AST_NODE_TYPES } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

import { calleeName, staticStringFromExpression } from "./ast.js";

export interface ClassTokenSite {
  node: TSESTree.Node;
  token: string;
}

export function classTokenSitesFromString(
  node: TSESTree.Node,
  value: string
): ClassTokenSite[] {
  return value
    .split(/\s+/)
    .map((token) => token.trim())
    .filter(Boolean)
    .map((token) => ({ node, token }));
}

export function classTokenSitesFromExpression(
  node: TSESTree.Node,
  classHelpers: ReadonlySet<string> = new Set()
): ClassTokenSite[] {
  const staticValue = staticStringFromExpression(
    node as
      | TSESTree.Expression
      | TSESTree.PrivateIdentifier
      | TSESTree.JSXEmptyExpression
      | TSESTree.SpreadElement
  );
  if (staticValue !== null) {
    return classTokenSitesFromString(node, staticValue);
  }

  if (node.type === AST_NODE_TYPES.JSXExpressionContainer) {
    return classTokenSitesFromExpression(node.expression, classHelpers);
  }

  if (node.type === AST_NODE_TYPES.ArrayExpression) {
    return node.elements.flatMap((element) =>
      element === null ? [] : classTokenSitesFromExpression(element, classHelpers)
    );
  }

  if (node.type === AST_NODE_TYPES.ObjectExpression) {
    return node.properties.flatMap((property) => {
      if (property.type !== AST_NODE_TYPES.Property) {
        return [];
      }
      if (property.key.type === AST_NODE_TYPES.Literal && typeof property.key.value === "string") {
        return classTokenSitesFromString(property.key, property.key.value);
      }
      if (property.computed) {
        return classTokenSitesFromExpression(property.key, classHelpers);
      }
      return [];
    });
  }

  if (node.type === AST_NODE_TYPES.TemplateLiteral) {
    return node.quasis.flatMap((quasi) =>
      classTokenSitesFromString(quasi, quasi.value.cooked ?? quasi.value.raw)
    );
  }

  if (node.type === AST_NODE_TYPES.BinaryExpression && node.operator === "+") {
    return [
      ...classTokenSitesFromExpression(node.left, classHelpers),
      ...classTokenSitesFromExpression(node.right, classHelpers)
    ];
  }

  if (node.type === AST_NODE_TYPES.ConditionalExpression) {
    return [
      ...classTokenSitesFromExpression(node.consequent, classHelpers),
      ...classTokenSitesFromExpression(node.alternate, classHelpers)
    ];
  }

  if (node.type === AST_NODE_TYPES.LogicalExpression) {
    return [
      ...classTokenSitesFromExpression(node.left, classHelpers),
      ...classTokenSitesFromExpression(node.right, classHelpers)
    ];
  }

  if (node.type === AST_NODE_TYPES.CallExpression) {
    const name = calleeName(node.callee);
    if (name === null || !classHelpers.has(name)) {
      return [];
    }
    return node.arguments.flatMap((argument) =>
      classTokenSitesFromExpression(argument, classHelpers)
    );
  }

  if (
    node.type === AST_NODE_TYPES.TSAsExpression ||
    node.type === AST_NODE_TYPES.TSNonNullExpression ||
    node.type === AST_NODE_TYPES.TSTypeAssertion
  ) {
    return classTokenSitesFromExpression(node.expression, classHelpers);
  }

  return [];
}
