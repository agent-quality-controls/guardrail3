import { AST_NODE_TYPES, ESLintUtils } from "@typescript-eslint/utils";
import type { TSESTree } from "@typescript-eslint/utils";

import { classifyModuleRole } from "../utils/module-role.js";
import {
  astroPipelineOptionsSchema,
  resolveOptions,
  type RuleOptionsTuple
} from "../utils/options.js";

type MessageIds = "missingApprovedImages" | "rawImage";

const createRule = ESLintUtils.RuleCreator(
  (name) =>
    `https://github.com/websmasher/guardrail3/tree/main/packages/ts/g3ts-eslint-plugin-astro-pipeline#${name}`
);

const MARKDOWN_IMAGE_PATTERN = /!\[[^\]]*]\([^)]+\)/;

export default createRule<RuleOptionsTuple, MessageIds>({
  name: "no-raw-mdx-images",
  meta: {
    type: "problem",
    docs: {
      description:
        "Forbid raw Markdown and HTML images in MDX content so images use approved wrappers."
    },
    schema: astroPipelineOptionsSchema,
    messages: {
      missingApprovedImages:
        "`approvedMdxImageComponents` is empty. Configure the exact validated image wrapper components MDX may use.",
      rawImage:
        "Raw MDX image usage is forbidden. Use an approved image wrapper from `approvedMdxImageComponents` instead."
    }
  },
  defaultOptions: [{}],
  create(context) {
    const options = resolveOptions(context.options[0]);
    const moduleRole = classifyModuleRole(context.filename, options);
    let reportedMarkdownImage = false;

    if (!moduleRole.isMdxContent) {
      return {};
    }

    function reportMarkdownImage(node: TSESTree.Node): void {
      if (reportedMarkdownImage) {
        return;
      }
      reportedMarkdownImage = true;
      context.report({ node, messageId: "rawImage" });
    }

    return {
      Program(node): void {
        if (options.approvedMdxImageComponents.length === 0) {
          context.report({ node, messageId: "missingApprovedImages" });
        }

        if (MARKDOWN_IMAGE_PATTERN.test(context.sourceCode.getText())) {
          reportMarkdownImage(node);
        }
      },
      JSXElement(node): void {
        if (
          node.openingElement.name.type === AST_NODE_TYPES.JSXIdentifier &&
          node.openingElement.name.name === "img"
        ) {
          context.report({ node, messageId: "rawImage" });
        }
      },
      JSXFragment(node): void {
        const text = context.sourceCode.getText(node);
        if (MARKDOWN_IMAGE_PATTERN.test(text)) {
          reportMarkdownImage(node);
        }
      },
      JSXText(node): void {
        if (MARKDOWN_IMAGE_PATTERN.test(node.value)) {
          reportMarkdownImage(node);
        }
      },
      Literal(node: TSESTree.Literal): void {
        if (typeof node.value === "string" && MARKDOWN_IMAGE_PATTERN.test(node.value)) {
          reportMarkdownImage(node);
        }
      }
    };
  }
});
