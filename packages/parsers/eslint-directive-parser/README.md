# eslint-directive-parser

Typed parser for ESLint directive comments used by guardrail3 ingestion.

MDX limitation: this parser does not use an MDX AST. If an `.mdx` file contains
a comment-shaped ESLint directive, the parser returns an `Ambiguous` file state
so downstream checks can fail closed instead of trusting raw text scanning.
