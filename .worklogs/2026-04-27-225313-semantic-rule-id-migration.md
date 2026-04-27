## Summary

Migrated active G3TS and G3RS rule identifiers from numeric IDs to semantic IDs across runtime checks, assertions, tests, reports, config references, and rule module filenames. Added migration inventory artifacts and fixed follow-up review defects in the MDX rule split, FMT rule names, and migration map hygiene.

## Decisions

- Numeric external IDs were removed from active emitted findings instead of kept as aliases. The migration has no backward compatibility layer because the old IDs were intentionally retired.
- Rule filenames and Rust module names were renamed to semantic names where they were part of the active rule implementation surface.
- `g3ts-astro-mdx/mdx-lane` was split into `g3ts-astro-mdx/mdx-eslint-plugin-package-present` and `g3ts-astro-mdx/mdx-eslint-lane-wired` because one ID was covering two independent assertions.
- Vague FMT IDs were renamed to `g3rs-fmt/rustfmt-required-settings`, `g3rs-fmt/rustfmt-extra-settings-inventory`, and `g3rs-fmt/rustfmt-config-exists`.
- TODO-only and retired placeholder entries were removed from the active migration map instead of pretending they are emitted rules.

## Key Files

- `.plans/2026-04-27-222733-semantic-rule-id-migration.md`
- `.plans/rule-id-migration/rule-id-map.toml`
- `.plans/rule-id-migration/ts-inventory.md`
- `.plans/rule-id-migration/rs-inventory.md`
- `packages/ts`
- `apps/guardrail3-ts`
- `packages/rs`
- `apps/guardrail3-rs`

## Verification

- `cargo fmt --all` in `apps/guardrail3-rs`
- `cargo fmt --all` in `apps/guardrail3-ts`
- `cargo fmt --manifest-path packages/ts/arch/g3ts-arch-ingestion/Cargo.toml --all`
- `cargo test --workspace --offline --locked` in `apps/guardrail3-rs`
- `cargo test --workspace --offline --locked` in `apps/guardrail3-ts`
- `cargo test --manifest-path packages/ts/arch/g3ts-arch-ingestion/Cargo.toml --workspace --offline --locked`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `cargo install --path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3rs validate --path apps/guardrail3-rs --inventory`
- `g3rs validate --path apps/guardrail3-ts --inventory`
- `g3ts validate --path apps/guardrail3-ts --inventory` emits semantic IDs; it still fails because that Rust app is not a G3TS project root.
- Active-code grep found no old numeric emitted IDs, stale old module name prefixes, `g3ts-astro-mdx/mdx-lane`, or retired vague FMT IDs.
- Final adversarial review returned no blockers.

## Next Steps

- Use the semantic IDs only in future guardrail waivers and documentation.
- If a retired historical rule becomes active again, add it as a new semantic ID instead of restoring its old numeric ID.
