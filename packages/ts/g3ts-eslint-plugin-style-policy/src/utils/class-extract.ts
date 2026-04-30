import { AST_NODE_TYPES } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

import { staticStringFromExpression } from "./ast.js";

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

export function classTokenSitesFromExpression(node: TSESTree.Node): ClassTokenSite[] {
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
    return classTokenSitesFromExpression(node.expression);
  }

  if (node.type === AST_NODE_TYPES.ArrayExpression) {
    return node.elements.flatMap((element) =>
      element === null ? [] : classTokenSitesFromExpression(element)
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
      if (
        property.key.type === AST_NODE_TYPES.TemplateLiteral &&
        property.key.expressions.length === 0
      ) {
        return classTokenSitesFromExpression(property.key);
      }
      return [];
    });
  }

  if (node.type === AST_NODE_TYPES.ConditionalExpression) {
    return [
      ...classTokenSitesFromExpression(node.consequent),
      ...classTokenSitesFromExpression(node.alternate)
    ];
  }

  if (node.type === AST_NODE_TYPES.LogicalExpression) {
    return [
      ...classTokenSitesFromExpression(node.left),
      ...classTokenSitesFromExpression(node.right)
    ];
  }

  if (node.type === AST_NODE_TYPES.CallExpression) {
    return node.arguments.flatMap(classTokenSitesFromExpression);
  }

  if (
    node.type === AST_NODE_TYPES.TSAsExpression ||
    node.type === AST_NODE_TYPES.TSNonNullExpression ||
    node.type === AST_NODE_TYPES.TSTypeAssertion
  ) {
    return classTokenSitesFromExpression(node.expression);
  }

  return [];
}
