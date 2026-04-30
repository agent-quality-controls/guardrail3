import { AST_NODE_TYPES } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

export function jsxAttributeName(name: TSESTree.JSXAttribute["name"]): string {
  if (name.type === AST_NODE_TYPES.JSXIdentifier) {
    return name.name;
  }

  return `${name.namespace.name}:${name.name.name}`;
}

export function staticStringFromExpression(
  node:
    | TSESTree.Expression
    | TSESTree.PrivateIdentifier
    | TSESTree.JSXEmptyExpression
    | TSESTree.SpreadElement
): string | null {
  if (node.type === AST_NODE_TYPES.Literal && typeof node.value === "string") {
    return node.value;
  }

  if (node.type === AST_NODE_TYPES.TemplateLiteral && node.expressions.length === 0) {
    return node.quasis.map((quasi) => quasi.value.cooked ?? quasi.value.raw).join("");
  }

  if (node.type === AST_NODE_TYPES.BinaryExpression && node.operator === "+") {
    const left = staticStringFromExpression(node.left);
    const right = staticStringFromExpression(node.right);

    return left !== null && right !== null ? `${left}${right}` : null;
  }

  if (
    node.type === AST_NODE_TYPES.TSAsExpression ||
    node.type === AST_NODE_TYPES.TSNonNullExpression ||
    node.type === AST_NODE_TYPES.TSTypeAssertion
  ) {
    return staticStringFromExpression(node.expression);
  }

  return null;
}

export function staticStringFromJsxAttribute(
  attribute: TSESTree.JSXAttribute
): string | null {
  const value = attribute.value;

  if (!value) {
    return null;
  }

  if (value.type === AST_NODE_TYPES.Literal && typeof value.value === "string") {
    return value.value;
  }

  if (value.type === AST_NODE_TYPES.JSXExpressionContainer) {
    return staticStringFromExpression(value.expression);
  }

  return null;
}

export function calleeName(callee: TSESTree.CallExpression["callee"]): string | null {
  if (callee.type === AST_NODE_TYPES.Identifier) {
    return callee.name;
  }

  return null;
}
