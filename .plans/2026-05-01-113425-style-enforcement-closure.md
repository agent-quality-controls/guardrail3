# Style Enforcement Closure

## Goal

Make the style family prove that delegated style validators are installed, configured, run through the standard app validation path, and cannot be silently disabled.

## Non-goals

- Do not scan source files in G3TS for Tailwind classes.
- Do not move app-owned deny policy into guardrail3-ts.toml.
- Do not hand-parse package versions.
- Do not build a rendered media or CSS auditor.
- Do not add Astro-specific style rules.

## Approach

### 1. Validate script closure

- File: `packages/ts/style/g3ts-style-types/src/lib.rs`
  - Add package-script facts needed to decide whether `validate` safely invokes `lint:css`.
- File: `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run.rs`
  - Add rule `g3ts-style/validate-runs-css-lint`.
  - It passes only when `package.json` has a `validate` script with a safe invocation of `pnpm run lint:css`, `npm run lint:css`, `yarn lint:css`, or direct `stylelint --max-warnings 0`.
  - It fails if the `validate` script is missing, unparseable, or uses `||` fail-open around the style step.

### 2. ESLint suppression protection

- File: `packages/ts/style/g3ts-style-types/src/lib.rs`
  - Add ESLint effective-config facts for warn/error rules and restricted-disable patterns.
- File: `packages/ts/style/g3ts-style-ingestion/crates/runtime/src/eslint.rs`
  - Extract `@eslint-community/eslint-comments/no-restricted-disable` patterns from each style source probe.
  - Keep this in ingestion because it reads parsed ESLint facts; rules receive normalized facts only.
- File: `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run.rs`
  - Add rule `g3ts-style/protected-style-rule-disables-restricted`.
  - It requires `@eslint-community/eslint-comments/no-restricted-disable` at warn or error for every source probe.
  - It must restrict `style-policy/*` and `tailwind-ban/*` so agents cannot hide style-policy bypasses.

### 3. Disable inventory

- File: `packages/ts/style/g3ts-style-types/src/lib.rs`
  - Add directive inventory input for style source files.
- File: `packages/ts/style/g3ts-style-ingestion/crates/runtime/src/run.rs`
  - Collect ESLint disable directives from `[ts.style].source_globs` paths using the existing eslint directive parser if available in the shared parser stack.
- File: `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run.rs`
  - Add rule `g3ts-style/eslint-disable-inventory`.
  - It emits inventory warnings for each disable directive and fails closed on parse errors.
  - It does not ban described disables. Visibility is the rule.

### 4. Syncpack package floor

- File: `packages/ts/style/g3ts-style-ingestion/Cargo.toml`
  - Add `syncpack-config-parser` if not already present.
- File: `packages/ts/style/g3ts-style-types/src/lib.rs`
  - Add Syncpack state: config path, source coverage, missing required pins.
- File: `packages/ts/style/g3ts-style-ingestion/crates/runtime/src/syncpack.rs`
  - Parse app `.syncpackrc` through `syncpack-config-parser`.
  - Require canonical pin group for `g3ts-eslint-plugin-style-policy = 0.1.3`.
  - Require the Syncpack source covers the app `package.json`.
- File: `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run.rs`
  - Add rule `g3ts-style/syncpack-style-policy-pin`.
  - Error message must say Syncpack owns the version floor; G3TS does not parse dependency ranges from `package.json`.

## Tests

- Add unit tests before or alongside each rule:
  - `validate` missing `lint:css` fails.
  - `validate` with `pnpm run lint:css` passes.
  - `validate` with `pnpm run lint:css || true` fails.
  - missing restricted-disable protection fails.
  - wildcard/prefix restricted-disable coverage passes.
  - disable inventory warns on one directive.
  - Syncpack missing required pin fails.
  - Syncpack required pin passes.

## Verification

- `cargo test --manifest-path packages/ts/style/g3ts-style-ingestion/crates/runtime/Cargo.toml`
- `cargo test --manifest-path packages/ts/style/g3ts-style-config-checks/crates/runtime/Cargo.toml`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml`
- `g3rs validate --path packages/ts/style/g3ts-style-types --inventory`
- `g3rs validate --path packages/ts/style/g3ts-style-ingestion --inventory`
- `g3rs validate --path packages/ts/style/g3ts-style-config-checks --inventory`
- Install local G3TS after changes.
