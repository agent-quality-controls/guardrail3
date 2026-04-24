# package-script-command-parser

Typed parser for package.json script command lines used by guardrail3 ingestion.

The parser emits command tokens plus extracted ESLint invocations for direct
`eslint` calls and the supported wrapper forms: `pnpm`, `npm`, `yarn`, `bun`,
`npx`, `bunx`, `env`, and `cross-env`.

Lint-related scripts fail closed as `Unsupported` when they use shell syntax
that can hide command behavior from the typed model, including pipes,
redirection, background execution, variable expansion, command substitution,
and command separators other than `&&` or `||`.

ESLint flags that require values fail closed as `ParseError` when the value is
missing or empty. Option scanning stops after `--`.
