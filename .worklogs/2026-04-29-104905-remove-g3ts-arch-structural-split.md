## Summary

Removed `g3ts-arch/structural-split` from the active TypeScript arch family. The rule's sibling-directory threshold produced low-value failures on real TS/Astro layouts, so the check and its source-tree metric ingestion were deleted instead of tuned or waived.

## Decisions made

- Removed the rule invocation and deleted the rule file rather than raising thresholds.
- Removed `G3TsArchSourceTree` and the `source_tree` ingestion path because those facts only fed the deleted rule.
- Kept `g3ts-arch/declared-entrypoint-exists` unchanged.
- Did not change Rust `g3rs-arch/structural-split`; that is a separate rule with separate existing waiver behavior.

## Key files for context

- `.plans/2026-04-29-104412-remove-g3ts-arch-structural-split.md`
- `packages/ts/arch/g3ts-arch-file-tree-checks/crates/runtime/src/run.rs`
- `packages/ts/arch/g3ts-arch-file-tree-checks/crates/runtime/src/declared_entrypoint_exists.rs`
- `packages/ts/arch/g3ts-arch-ingestion/crates/runtime/src/file_tree.rs`
- `packages/ts/arch/g3ts-arch-types/src/types.rs`

## Verification

- `cargo fmt --manifest-path apps/guardrail3-ts/Cargo.toml --all`
- `cargo test --manifest-path packages/ts/arch/g3ts-arch-ingestion/crates/runtime/Cargo.toml --offline --locked`
- `cargo test --manifest-path packages/ts/arch/g3ts-arch-file-tree-checks/crates/runtime/Cargo.toml --offline --locked`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml -p guardrail3-ts-family-runner-structure --offline --locked`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline --locked`
- `g3rs validate --path apps/guardrail3-ts`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher-integrate-static-railway/apps/landing --family arch --inventory`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family arch --inventory`
- `git diff --check`

## Verification notes

- `cargo clippy --manifest-path apps/guardrail3-ts/Cargo.toml -p guardrail3-ts-family-runner-structure -p g3ts-arch-ingestion -p g3ts-arch-file-tree-checks -p g3ts-arch-types --all-targets --offline --locked -- -D warnings` still fails on pre-existing aggregator issues: duplicate `siphasher` versions and `family-runner-structure::run` too many lines.
- Direct runtime clippy with `-D warnings` also hits existing package lint debt such as missing private docs and unrelated source-ingestion pedantic lints.

## Next steps

- Decide separately whether Rust `g3rs-arch/structural-split` still carries enough value to keep with waivers.
