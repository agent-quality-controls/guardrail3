import { AST_NODE_TYPES } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

export function jsxElementName(
  name: TSESTree.JSXOpeningElement["name"]
): string | null {
  if (name.type === AST_NODE_TYPES.JSXIdentifier) {
    return name.name;
  }

  if (name.type === AST_NODE_TYPES.JSXMemberExpression) {
    const objectName = jsxElementName(name.object);
    const propertyName = jsxElementName(name.property);

    return objectName && propertyName ? `${objectName}.${propertyName}` : null;
  }

  if (name.type === AST_NODE_TYPES.JSXNamespacedName) {
    return `${name.namespace.name}:${name.name.name}`;
  }

  return null;
}

export function jsxAttributeName(name: TSESTree.JSXAttribute["name"]): string {
  if (name.type === AST_NODE_TYPES.JSXIdentifier) {
    return name.name;
  }

  return `${name.namespace.name}:${name.name.name}`;
}

export function staticStringFromExpression(
  node: TSESTree.Expression | TSESTree.PrivateIdentifier | TSESTree.JSXEmptyExpression
): string | null {
  if (node.type === AST_NODE_TYPES.Literal && typeof node.value === "string") {
    return node.value;
  }

  if (node.type === AST_NODE_TYPES.TemplateLiteral && node.expressions.length === 0) {
    return node.quasis.map((quasi) => quasi.value.cooked ?? quasi.value.raw).join("");
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
