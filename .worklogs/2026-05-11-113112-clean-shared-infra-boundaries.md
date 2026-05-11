Summary

- Split shared workspace crawling out of the Rust namespace into `packages/shared/g3-workspace-crawl`.
- Moved TypeScript policy parsing out of `g3rs-toml-parser` and into `g3ts-toml-parser`.
- Added a manifest verifier that proves active code no longer depends on the old Rust-owned shared crawl name or the wrong TOML parser boundary.

Decisions Made

- `g3-workspace-crawl` is the neutral shared package name because the crawl is used by both `g3rs` and `g3ts`.
- `g3rs-toml-parser` now owns only `guardrail3-rs.toml` fields. TS policy fields were removed from its public types and tests.
- `g3ts-toml-parser` now owns `[ts.style]` and `[ts.astro]` policy fields because TS ingestion packages consume that config.
- The old `packages/rs/g3rs-workspace-crawl` package was deleted instead of kept as a compatibility alias.
- A `g3rs-test/real-proof-site` bug was fixed because normal Rust imports use underscores even when the Cargo package name contains hyphens.

Key Files

- `.plans/2026-05-11-105304-clean-shared-infra-boundaries.md`
- `.plans/2026-05-11-105304-clean-shared-infra-boundaries.manifest.toml`
- `scripts/verify-shared-infra-boundaries.py`
- `packages/shared/g3-workspace-crawl`
- `packages/parsers/g3rs-toml-parser/crates/types/src/guardrail3_rs_toml.rs`
- `packages/parsers/g3ts-toml-parser/crates/types/src/guardrail3_ts_toml.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/real_proof_site/rule.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/source_analysis/proof_helpers.rs`

Verification

- `cargo test --manifest-path packages/shared/g3-workspace-crawl/Cargo.toml --workspace`
- `cargo test --manifest-path packages/parsers/g3ts-toml-parser/Cargo.toml --workspace`
- `cargo test --manifest-path packages/parsers/g3rs-toml-parser/Cargo.toml --workspace`
- `cargo test --manifest-path packages/rs/test/g3rs-test-source-checks/Cargo.toml --workspace`
- `cargo test --manifest-path packages/rs/test/g3rs-test-ingestion/Cargo.toml --workspace`
- `cargo build --release --manifest-path apps/guardrail3-rs/Cargo.toml -p guardrail3-rs`
- `apps/guardrail3-rs/target/release/g3rs validate --path packages/shared/g3-workspace-crawl`
- `apps/guardrail3-rs/target/release/g3rs validate --path packages/parsers/g3rs-toml-parser`
- `apps/guardrail3-rs/target/release/g3rs validate --path packages/parsers/g3ts-toml-parser`
- `apps/guardrail3-rs/target/release/g3rs validate --path apps/guardrail3-rs`
- `apps/guardrail3-rs/target/release/g3rs validate --path apps/guardrail3-ts`
- `python3 scripts/verify-shared-infra-boundaries.py`
- `git diff --check`

Next Steps

- Fix the existing `g3rs validate` output noise where raw `cargo metadata` JSON is printed before findings.
- Decide whether warning-only findings in `apps/guardrail3-rs`, `apps/guardrail3-ts`, and the TS parser should be reduced in separate cleanup work.
