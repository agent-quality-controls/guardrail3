# Goal

Enforce "no authored public copy in Astro source files" with as little custom code as possible.

The implementation should delegate source AST scanning to `eslint-plugin-i18next/no-literal-string`, because local testing showed that it works with `astro-eslint-parser` and catches:

- Astro frontmatter string literals
- Astro template text
- Astro public-copy attributes such as `alt`
- TSX JSX text
- TSX object/array literal copy

G3TS should not scan source. It should only enforce that the delegated rule is installed, configured, and effective on Astro public source lanes.

# Approach

1. Update `eslint-plugin-astro-pipeline`.
   - Add exact runtime dependency on `eslint-plugin-i18next`.
   - Export a named strict content config that wires `i18next/no-literal-string`.
   - Keep the current 8 Astro pipeline rules.
   - Do not add a custom `no-inline-public-content` AST rule in this pass.
   - Document that `i18next/no-literal-string` is the delegated rule.

2. Add plugin tests.
   - Verify the strict config includes plugin key `i18next`.
   - Verify the strict config enables `i18next/no-literal-string` at `error`.
   - Verify it catches TSX inline copy, TS object and array copy, Astro template copy, Astro frontmatter copy, i18n-call bypasses, and public-copy attributes.
   - Verify it allows class names, `src`, imports, TS type literals, asset helper calls, and structural tokens.

3. Update G3TS Astro family.
   - Require effective delegated plugin `i18next` in the ESLint config.
   - Add required delegated rule `i18next/no-literal-string`.
   - Add effective public-content rule facts for Astro, TS, and TSX lanes.
   - Require the rule to be `error` on every present public source lane.
   - Require option `mode: "all"` so object literals are scanned, not only JSX text.
   - Require a non-empty custom message that tells the agent to move copy into Astro content entries.
   - Reject delegated-rule option shapes that hide authored copy through broad `words`, `jsx-components`, `callees`, `object-properties`, or `jsx-attributes` allowlists.
   - Do not require `sourceGlobs` inside the i18next rule, because ESLint file scoping owns where the rule is applied.

4. Keep scope coverage simple but real.
   - Existing probe selection already checks Astro/TS/TSX source lanes under the active ESLint config scope.
   - The effective-config parser can see whether `i18next/no-literal-string` is active for those lanes.
   - This is enough for first enforcement and avoids writing another path coverage algorithm.

5. Update package floors.
   - Raise `eslint-plugin-astro-pipeline` minimum floor to the version that exports the strict config.
   - Do not add `eslint-plugin-i18next` to Astro app package floors. It is a runtime dependency owned by `eslint-plugin-astro-pipeline`, so apps should install only the Astro pipeline plugin.
   - Ban direct Astro app dependencies on `eslint-plugin-i18next` through Syncpack for the same ownership reason.

# Files To Modify

- `packages/ts/eslint-plugin-astro-pipeline/package.json`
- `packages/ts/eslint-plugin-astro-pipeline/package-lock.json`
- `packages/ts/eslint-plugin-astro-pipeline/src/configs/recommended.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/index.ts`
- `packages/ts/eslint-plugin-astro-pipeline/README.md`
- new plugin test file under `packages/ts/eslint-plugin-astro-pipeline/tests`
- `packages/ts/astro/g3ts-astro-types/src/types.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/support.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_07_pipeline_plugin_wired.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/cases.rs`

# Verification

Plugin:

```sh
cd packages/ts/eslint-plugin-astro-pipeline
npm install
npm test
npm pack --dry-run
```

G3TS:

```sh
cargo test -q --manifest-path packages/ts/astro/g3ts-astro-ingestion/Cargo.toml --workspace
cargo test -q --manifest-path packages/ts/astro/g3ts-astro-config-checks/Cargo.toml --workspace
cargo test -q --manifest-path apps/guardrail3-ts/Cargo.toml --workspace
```

# Non-goals

- Do not write a custom source literal AST scanner.
- Do not make G3TS parse TS/TSX/Astro source.
- Do not solve guardrail waiver detection in this commit unless the existing parser wiring is already trivial to extend.
