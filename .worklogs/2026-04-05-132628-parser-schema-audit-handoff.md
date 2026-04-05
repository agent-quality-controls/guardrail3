# Parser Schema Audit Handoff

**Date:** 2026-04-05 13:26
**Scope:** `packages/cargo-toml-parser/`, `packages/cargo-config-toml-parser/`, `packages/clippy-toml-parser/`, `packages/mutants-toml-parser/`, `packages/nextest-toml-parser/`, `packages/rustfmt-toml-parser/`

## Summary
Audited the remaining non-deny Rust parser packages against their upstream file formats and tightened the ones that were still under-modeled. The largest remaining gap was `cargo-toml-parser`, which was expanded from a narrow manifest subset into a much broader file-faithful `Cargo.toml` representation; `cargo-config-toml-parser` was rechecked against the current Cargo config reference and did not require code changes.

This worklog is the broader companion to `.worklogs/2026-04-05-125907-deny-toml-parser-full-schema-handoff.md`, which covers the deny parser separately.

## Context & Problem
After the deny parser pass, the user asked for the same kind of schema audit and handoff for the rest of the parser family. The governing requirement was explicit during the session: parsers are not supposed to be shaped around current consumer convenience. They should model the actual file structure as faithfully as possible, use typed fields for the known schema, and preserve unknown keys in `extra`.

The parser family was in a mixed state:
- some packages were already close to upstream structure and only needed confirmation
- some had already been expanded earlier in the session but still needed to be captured in a handoff
- `cargo-toml-parser` still lagged well behind Cargo's actual manifest schema and needed a substantive rewrite

## Decisions Made

### Treat Cargo docs and source as the schema authority
- **Chose:** Audit parser coverage against official upstream references for each tool rather than inferring completeness from local tests.
- **Why:** The local tests were proving only the subset we had modeled, not the standard file shape. The user explicitly wanted robust file representations, not parsers optimized around current downstream callers.
- **Alternatives considered:**
  - Trust existing tests as proof of completeness — rejected because the tests only covered the already-modeled subset.
  - Leave parser coverage intentionally narrow and rely on `extra` — rejected because that hides known standard fields behind untyped escape hatches.

### Keep parsers file-faithful and field-first
- **Chose:** Model known fields directly in the parsed structs, keep inheritance syntax explicit where the file format supports it, and avoid convenience getter layers.
- **Why:** This matches the user’s design rule for parser packages: the primary API should be the parsed file representation itself, not a consumer-shaped normalized facade.
- **Alternatives considered:**
  - Add secondary helper APIs for downstream ergonomics — rejected because the user explicitly said not to add a secondary API.
  - Normalize multiple file shapes into a single consumer-friendly representation — rejected because it erases distinctions the source file actually makes.

### Expand `cargo-toml-parser` into a broader manifest model
- **Chose:** Rework `cargo-toml-parser` to cover the major current Cargo manifest sections and shapes: `cargo-features`, `project`, `badges`, `hints`, richer `package` fields, richer `workspace` fields, broader dependency detail, broader target detail, and broader profile detail.
- **Why:** `cargo-toml-parser` was the main remaining parser that still reflected only a narrow local subset of its real file format.
- **Alternatives considered:**
  - Add only the immediately-needed manifest fields — rejected because it would preserve the same piecemeal, consumer-driven drift that caused the gap.
  - Copy Cargo’s internal schema types wholesale — rejected because those types include normalization/accessor behavior and internal book-keeping that are not a clean fit for this parser package.

### Re-audit `cargo-config-toml-parser` without forcing changes
- **Chose:** Recheck `cargo-config-toml-parser` against the current Cargo config reference and keep it unchanged after verification.
- **Why:** The package already covered the current documented section set well, including `include`, `env`, `profile`, `registries`, `registry`, `source`, `target`, and `term`. There was no value in changing it just to look busy.
- **Alternatives considered:**
  - Force extra changes to “match” the amount of movement in other parsers — rejected because unnecessary churn would make the audit less credible.

## Architectural Notes
The non-deny parser family currently falls into three buckets:

- audited and changed in this broader pass:
  - `cargo-toml-parser`
- audited earlier in the session and already broadened:
  - `clippy-toml-parser`
  - `mutants-toml-parser`
  - `nextest-toml-parser`
  - `rustfmt-toml-parser`
- audited in this pass and confirmed already adequate:
  - `cargo-config-toml-parser`

The main structural change in `cargo-toml-parser` was adding explicit file-shape types that Cargo manifests actually use:
- `InheritableValue<T>` for `{ workspace = true }` inheritance syntax
- `PackageBuildValue` for `build = false | "build.rs" | ["..."]`
- broader `DependencyDetail` coverage, including `registry-index`, `base`, `public`, `artifact`, `lib`, and `target`
- broader `TargetSection` coverage, including `crate-type`, `filename`, `doc-scrape-examples`, and `edition`
- broader `ProfileConfig` coverage, including `codegen-backend`, `dir-name`, `rustflags`, `trim-paths`, `hint-mostly-unused`, and `frame-pointers`

The package still preserves unknown keys in `extra` where Cargo’s manifest can evolve further or where nested open-ended tables exist.

## Information Sources
- Cargo manifest schema source:
  - `https://raw.githubusercontent.com/rust-lang/cargo/master/crates/cargo-util-schemas/src/manifest/mod.rs`
- Cargo manifest reference:
  - `https://raw.githubusercontent.com/rust-lang/cargo/master/src/doc/src/reference/manifest.md`
- Cargo config reference:
  - `https://raw.githubusercontent.com/rust-lang/cargo/master/src/doc/src/reference/config.md`
- Current parser packages in this repo:
  - `packages/cargo-toml-parser/`
  - `packages/cargo-config-toml-parser/`
  - `packages/clippy-toml-parser/`
  - `packages/mutants-toml-parser/`
  - `packages/nextest-toml-parser/`
  - `packages/rustfmt-toml-parser/`
- Prior related worklog:
  - `.worklogs/2026-04-05-125907-deny-toml-parser-full-schema-handoff.md`

## Open Questions / Future Considerations
- `cargo-toml-parser` is now much broader, but Cargo’s manifest surface is large and still evolving. If a future family needs rarer manifest shapes, audit them back against Cargo source before adding anything ad hoc.
- `guardrail3-rs-toml-parser` was not part of this standard-schema audit because it models a local file format rather than an external tool-owned schema.
- `rust-toolchain-toml-parser` already has current `path` support in the checked-in tree, so it did not require further changes in this batch.

## Key Files for Context
- `packages/cargo-toml-parser/crates/parser/types/src/cargo_toml.rs` — expanded `Cargo.toml` file model with inheritance, richer targets, and richer profiles
- `packages/cargo-toml-parser/crates/parser/runtime/src/parser_tests/parsing.rs` — broad manifest fixture that proves the widened schema surface
- `packages/cargo-config-toml-parser/crates/parser/types/src/cargo_config_toml.rs` — audited Cargo config model; used as the “already broad enough” specimen in this pass
- `packages/clippy-toml-parser/crates/parser/types/src/clippy_toml.rs` — broadened Clippy config model from the earlier parser audit pass
- `packages/nextest-toml-parser/crates/parser/types/src/nextest_toml.rs` — broadened nextest config model from the earlier parser audit pass
- `packages/mutants-toml-parser/crates/parser/types/src/mutants_toml.rs` — updated cargo-mutants config model from the earlier parser audit pass
- `packages/rustfmt-toml-parser/crates/parser/types/src/rustfmt_toml.rs` — updated rustfmt config model from the earlier parser audit pass
- `.worklogs/2026-04-05-125907-deny-toml-parser-full-schema-handoff.md` — separate deny parser handoff and the closest backstory for this broader parser audit

## Next Steps / Continuation Plan
1. If this parser batch is going to be committed, stage only the parser packages that belong in the batch plus this handoff worklog and the deny-specific handoff worklog; do not accidentally pull in unrelated family-runtime work already present in the worktree.
2. Rewire downstream content-check packages and family runtimes to consume the richer parser types instead of re-parsing or assuming older, narrower shapes.
3. If any parser is questioned again for completeness, repeat the same process: diff against the tool’s current official docs/source first, then add typed fields only for schema the tool really owns.
