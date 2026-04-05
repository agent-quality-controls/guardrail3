# Tighten Parser File Contracts

**Date:** 2026-04-05 14:17
**Scope:** `packages/mutants-toml-parser/`, `packages/clippy-toml-parser/`, `packages/nextest-toml-parser/`, `packages/cargo-toml-parser/`

## Summary
Tightened four parser packages toward the actual upstream file contracts instead of the looser local shapes they had drifted into. The batch removed invalid permissiveness in `mutants-toml-parser` and `clippy-toml-parser`, broadened `nextest-toml-parser` to cover the current documented nextest configuration surface with real incompatibility checks, and finished the remaining high-signal typed branch in `cargo-toml-parser` by modeling `trim-paths`.

## Context & Problem
The user explicitly redirected the parser work after a bad overstep: instead of broad unrequested edits, fix parsers one by one and keep sending adversarial contract checks until they converge on the real file format, no more and no less.

The concrete problems surfaced by that parser-by-parser review were:
- `mutants-toml-parser` had dropped real upstream fields and accepted unknown keys even though upstream uses `deny_unknown_fields`
- `clippy-toml-parser` had previously allowed unknown keys and invalid `disallowed-fields` detail shapes; it needed to stay exact rather than permissive
- `nextest-toml-parser` still had large documented areas represented as raw `Value` and did not enforce several real top-level compatibility constraints around scripts and experimental features
- `cargo-toml-parser` was already much broader than before, but still left `profile.*.trim-paths` untyped and did not prove several multi-shape manifest branches it claimed to support

The governing constraint was stable through the whole batch: parsers are not consumer-oriented convenience APIs. They should represent the file faithfully, with typed known schema, exact accepted/rejected shapes where the upstream tool is opinionated, and `extra` only where the tool itself tolerates open-ended configuration.

## Decisions Made

### Restore Exact Upstream Behavior In `mutants-toml-parser`
- **Chose:** Reintroduce `test_tool` and `sharding`, remove the top-level `extra`, and switch the root struct to `deny_unknown_fields`.
- **Why:** `cargo-mutants` owns a small, strict config surface. Preserving unknown keys locally was not forward-compatible correctness; it was false permissiveness.
- **Alternatives considered:**
  - Keep unknown keys in `extra` for future-proofing — rejected because upstream rejects them.
  - Leave `test_tool` and `sharding` out until a downstream consumer needs them — rejected because the parser is supposed to model the file, not current call sites.

### Keep `clippy-toml-parser` Strict
- **Chose:** Preserve the stricter `deny_unknown_fields` root, keep `disallowed-fields` separate from the replacement-supporting path entries, and leave the parser typed rather than reopening `Value` escape hatches.
- **Why:** Rechecking against current Clippy config source showed the tightened field inventory and structured-entry split were aligned with upstream. The earlier attack finding about raw `Value` fields was stale after the latest parser rewrite.
- **Alternatives considered:**
  - Rework Clippy again because of the earlier review notes — rejected because the current code already matched the updated source inventory closely.
  - Loosen the parser to tolerate future keys — rejected because Clippy reports unknown config fields as errors.

### Expand `nextest-toml-parser` To The Real Documented Shape
- **Chose:** Replace major `Value` buckets with typed models for `nextest-version`, `experimental`, `test-groups`, modern/deprecated scripts, profile overrides, profile scripts, `junit`, `archive`, `test-group`, and the current retry/fail-fast/thread enums.
- **Why:** nextest’s config reference is broad and explicit. Leaving these sections as raw TOML would hide known schema behind untyped blobs and make the parser less trustworthy than the tool it models.
- **Alternatives considered:**
  - Only type the fields guardrail currently uses — rejected because that keeps the parser consumer-shaped instead of file-shaped.
  - Type every deep branch immediately, including all remaining open-ended/extension areas — rejected because the documented major sections were the highest-signal contract gap, and unknown-key preservation is still correct for nextest’s warning-based behavior.

### Enforce nextest Script Compatibility Rules In The Parser
- **Chose:** Add root-level validation that rejects:
  - setup scripts without `experimental = ["setup-scripts"]`
  - wrapper scripts without `experimental = ["wrapper-scripts"]`
  - mixed deprecated `[script.*]` and modern `[scripts.setup.*]`
- **Why:** These are not downstream policy decisions; they are actual nextest config constraints enforced by nextest itself.
- **Alternatives considered:**
  - Leave these checks to a future content package or app family — rejected because they are part of the file contract, not guardrail policy.
  - Preserve both script tables and let consumers decide — rejected because that accepts invalid nextest files as if they were valid parser output.

### Finish The Remaining High-Signal Cargo Gap
- **Chose:** Model `profile.*.trim-paths` as a typed multi-shape enum and add tests for `build`, `artifact`, `publish`, `readme`, and `trim-paths` branch forms.
- **Why:** Cargo already documents and implements these shapes explicitly. `trim-paths` was the most obvious remaining standard field still hidden behind `Value`.
- **Alternatives considered:**
  - Leave `trim-paths` as `Value` because Cargo continues evolving — rejected because Cargo’s accepted forms are already explicit in the manifest schema.
  - Attempt a full final Cargo manifest closure in the same batch — rejected because the goal here was incremental contract convergence, not another sprawling speculative rewrite.

## Architectural Notes
The batch reinforced three distinct parser stances depending on the upstream tool:

- strict reject-on-unknown:
  - `mutants-toml-parser`
  - `clippy-toml-parser`
- preserve unknown keys because the tool warns/ignores them:
  - `nextest-toml-parser`
  - `cargo-toml-parser`

That distinction matters. “Use `extra`” is not a general parser rule; it depends on how the upstream tool treats unknown keys.

The `nextest` work also clarified a useful parser boundary:
- documented section structure belongs in the parser
- real top-level incompatibility rules for those sections also belong in the parser
- policy about whether a config file should exist or be preferred still belongs outside

The `cargo` work narrowed the remaining known-typed gap in the manifest model without trying to eliminate all `Value`-typed open-ended subtrees like `metadata` or `badges`, which are intentionally schema-open.

## Information Sources
- Current parser package code:
  - `packages/mutants-toml-parser/`
  - `packages/clippy-toml-parser/`
  - `packages/nextest-toml-parser/`
  - `packages/cargo-toml-parser/`
- Upstream sources and docs used to derive the actual contracts:
  - `https://raw.githubusercontent.com/sourcefrog/cargo-mutants/main/src/config.rs`
  - `https://raw.githubusercontent.com/sourcefrog/cargo-mutants/main/src/options.rs`
  - `https://raw.githubusercontent.com/sourcefrog/cargo-mutants/main/src/shard.rs`
  - `https://raw.githubusercontent.com/rust-lang/rust-clippy/master/clippy_config/src/conf.rs`
  - `https://nexte.st/docs/configuration/reference/`
  - `https://raw.githubusercontent.com/nextest-rs/nextest/main/nextest-runner/src/config/core/imp.rs`
  - `https://raw.githubusercontent.com/rust-lang/cargo/master/crates/cargo-util-schemas/src/manifest/mod.rs`
- Prior context:
  - `.worklogs/2026-04-05-132628-parser-schema-audit-handoff.md`

## Open Questions / Future Considerations
- `deny-toml-parser` is still the largest remaining standard-schema surface and should be the next parser attacked and tightened.
- `cargo-toml-parser` still has intentionally open-ended areas like `metadata`, `badges`, and `hints.mostly_unused`; those should only be typed further if Cargo’s schema makes them concrete enough to do so honestly.
- `nextest-toml-parser` is much closer to the real contract now, but there are still additional nextest substructures that could be typed later if a future attack finds a concrete mismatch.

## Key Files for Context
- `packages/mutants-toml-parser/crates/parser/types/src/mutants_toml.rs` — strict cargo-mutants root shape including `test_tool` and `sharding`
- `packages/mutants-toml-parser/crates/parser/runtime/src/parser_tests/parsing.rs` — reject-path tests for unknown keys and unsupported old options
- `packages/clippy-toml-parser/crates/parser/types/src/clippy_toml.rs` — strict Clippy config model with separated detailed entry shapes
- `packages/clippy-toml-parser/crates/parser/runtime/src/parser_tests/parsing.rs` — Clippy accept/reject coverage proving the strict parser contract
- `packages/nextest-toml-parser/crates/parser/types/src/nextest_toml.rs` — expanded nextest model and script/experimental compatibility checks
- `packages/nextest-toml-parser/crates/parser/runtime/src/parser_tests/parsing.rs` — typed accept/reject proofs for nextest sections and incompatibilities
- `packages/cargo-toml-parser/crates/parser/types/src/cargo_toml.rs` — broader Cargo manifest model including typed `TomlTrimPaths`
- `packages/cargo-toml-parser/crates/parser/runtime/src/parser_tests/parsing.rs` — manifest branch-coverage fixture plus reject test for invalid `trim-paths`
- `.worklogs/2026-04-05-132628-parser-schema-audit-handoff.md` — earlier audit/handoff that this tightening batch corrects and narrows

## Next Steps / Continuation Plan
1. Continue with `packages/deny-toml-parser/`, using the same attack-first loop: compare current typed fields to cargo-deny’s documented/source-backed shapes, then add or tighten only the contract that the tool really owns.
2. Keep commit boundaries narrow per parser batch or tightly related parser batch; do not mix parser contract work with family runtime or content-check extraction changes already in the tree.
3. After `deny-toml-parser` converges, re-evaluate whether `rustfmt-toml-parser` still needs any exact-value tightening beyond the already-added key inventory.
