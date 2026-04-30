# Goal

Fix the TS style family boundary so G3TS verifies style enforcement plumbing without owning the app-specific Tailwind denylist.

# Problem

The current `style` family requires `[ts.style].tailwind_denylist` in `guardrail3-ts.toml` and then requires `eslint-plugin-tailwind-ban` to use exactly the same list.

That duplicates the actual style policy between Guardrail config and ESLint config. It makes G3TS a mirror of ESLint rule options instead of a guardrail that proves delegated tooling is active.

# Approach

- Remove `tailwind_denylist` from `G3TsStylePolicySnapshot`.
- Stop parsing `tailwind_denylist` from `[ts.style]`.
- Change `g3ts-style/strict-policy-configured` so it requires only:
  - non-empty `source_globs`
  - non-empty `stylelint_css_globs`
  - relative source/stylelint globs without `..`
- Change ESLint ingestion so `tailwind-ban/no-deny-tailwind-tokens` is effective when:
  - the rule is active at `error`
  - at least one rule option has a `denyList` array
  - the array has at least one non-empty string
- Keep G3TS requiring the package and the effective ESLint rule.
- Do not move the actual denied class list into G3TS.

# Key Decisions

- ESLint owns Tailwind policy values because ESLint is the delegated validator that reports source violations.
- G3TS owns package/config/script/hook plumbing and no-op prevention.
- Empty ESLint `denyList` is still an error because the delegated rule would be configured but ineffective.
- No optional `tailwind_ban_required` flag is added. Style apps using this family must have a non-empty Tailwind deny rule unless waived at the guardrail level.

# Files To Modify

- `packages/ts/style/g3ts-style-types/src/lib.rs`
- `packages/ts/style/g3ts-style-ingestion/crates/runtime/src/policy.rs`
- `packages/ts/style/g3ts-style-ingestion/crates/runtime/src/eslint.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run_tests/*`
- `packages/ts/style/g3ts-style-ingestion/crates/runtime/src/eslint_tests/*`
- `packages/ts/style/g3ts-style-ingestion/crates/assertions/src/*`
- `packages/parsers/guardrail3-rs-toml-parser/crates/types/src/guardrail3_rs_toml.rs`

# Verification

- `cargo test --manifest-path packages/ts/style/g3ts-style-config-checks/crates/runtime/Cargo.toml --offline`
- `cargo test --manifest-path packages/ts/style/g3ts-style-ingestion/Cargo.toml --offline`
- `cargo test --manifest-path packages/ts/style/g3ts-style-ingestion/crates/runtime/Cargo.toml --offline`
- `cargo test --manifest-path packages/parsers/guardrail3-rs-toml-parser/Cargo.toml --offline`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- `g3rs validate --path packages/parsers/guardrail3-rs-toml-parser`
- `g3rs validate --path packages/ts/style/g3ts-style-types`
- `g3rs validate --path packages/ts/style/g3ts-style-ingestion`
- `g3rs validate --path packages/ts/style/g3ts-style-config-checks`
- `g3rs validate --path apps/guardrail3-ts`
