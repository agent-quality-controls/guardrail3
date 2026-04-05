# Tighten Remaining Parser Contracts

**Date:** 2026-04-05 14:34
**Scope:** `packages/deny-toml-parser/`, `packages/rustfmt-toml-parser/`, `packages/rust-toolchain-toml-parser/`, `packages/cargo-config-toml-parser/`

## Summary
Tightened four remaining parser packages against the actual file contracts they are supposed to model. The work combined upstream docs/source checks with direct tool attacks (`rustup show active-toolchain` and nightly `cargo config get`) so the parser behavior converges on what the real tools accept or reject instead of drifting into guessed schemas.

## Context & Problem
After the first parser-tightening batch, four packages still had concrete gaps:

- `deny-toml-parser` still under-modeled several documented cargo-deny fields.
- `rustfmt-toml-parser` still accepted many constrained option domains as plain strings.
- `rust-toolchain-toml-parser` only deserialized TOML and did not enforce rustup’s path/channel exclusivity rules.
- `cargo-config-toml-parser` did not enforce Cargo’s real include-path rule that included files must end with lowercase `.toml`.

The user explicitly wanted parser packages to represent the real file contracts “no more, no less”, which ruled out both consumer-shaped helpers and speculative tightening not backed by upstream behavior.

## Decisions Made

### Tighten `deny-toml-parser` To More Of The Standard cargo-deny Shape
- **Chose:** Add the remaining typed fields that were clearly documented in cargo-deny’s advisories, licenses, and sources config:
  - `AdvisoryScope`
  - `unsound`
  - `git_fetch_with_cli`
  - `disable_yank_checking`
  - `maximum_db_staleness`
  - integer `clarify.files[].hash`
  - typed `GitSpec`
- **Why:** These are real documented config keys and discrete value domains, so leaving them as untyped strings or generic values was unnecessary under-modeling.
- **Alternatives considered:**
  - Leave them in `extra` for forward compatibility — rejected because these fields are already stable and documented.
  - Convert more cargo-deny string fields into enums preemptively — rejected because only the fields clearly backed by current docs/source were tightened in this pass.

### Tighten `rustfmt-toml-parser` Only Where rustfmt Has Closed Choice Domains
- **Chose:** Convert several rustfmt string fields into enums that mirror rustfmt’s own option domains, such as `NewlineStyle`, `BraceStyle`, `GroupImportsTactic`, `Edition`, `StyleEdition`, `EmitMode`, and related output/style settings.
- **Why:** rustfmt’s config surface has many fixed string domains, and the parser should reject invalid option names instead of accepting arbitrary strings.
- **Alternatives considered:**
  - Keep everything as `String` and let downstream consumers validate — rejected because that weakens the parser contract and duplicates upstream validation later.
  - Convert every remaining string field into enums immediately — rejected because some fields still need another pass against rustfmt’s source/options before tightening further.

### Make `rust-toolchain-toml-parser` Match rustup’s Real Semantic Constraints
- **Chose:** Keep unknown keys preserved, but add parser-time validation that rejects:
  - `path` combined with `channel`
  - `path` combined with `components`, `targets`, or `profile`
- **Why:** Direct rustup attacks showed that rustup accepts unknown keys in `rust-toolchain.toml`, but rejects those path/toolchain option combinations. The parser was previously too permissive because it only performed TOML deserialization.
- **Alternatives considered:**
  - Reject unknown keys as well — rejected because rustup currently ignores them rather than failing.
  - Leave semantic validation to downstream code — rejected because these are core file-level validity rules owned by rustup itself.

### Tighten `cargo-config-toml-parser` Only On A Contract Cargo Actually Enforces
- **Chose:** Add include-path validation requiring `include` entries to end with lowercase `.toml`.
- **Why:** Nightly `cargo config get -Z unstable-options` rejects `include = ["foo.txt"]` and also rejects `include = ["foo.TOML"]`, so this is a real Cargo config rule that belongs in the parser.
- **Alternatives considered:**
  - Make `[target.<triple>.<links>]` fully typed in this pass — rejected because empirical Cargo probing showed Cargo currently accepts looser shapes there than the docs imply, so tightening that branch immediately risked overmodeling.
  - Use a case-insensitive `.toml` check to satisfy Clippy — rejected because Cargo itself is case-sensitive for include-path extension matching.

## Architectural Notes
The common pattern in this batch was:

- prefer upstream tool behavior over assumptions
- encode semantic validation inside the parser package when the real tool rejects a shape during parsing/config loading
- avoid tightening fields just because docs show a narrower idealized shape if the tool currently accepts looser input

For `rust-toolchain-toml-parser`, semantic validation was implemented through a raw serde intermediary and custom `Deserialize` for `ToolchainSection`, which keeps the public output type faithful while letting the parser reject rustup-invalid combinations.

For `cargo-config-toml-parser`, the include-path rule is attached directly to `IncludeEntry` deserialization so both string and table forms share the same validation.

## Information Sources
- cargo-deny source/docs:
  - `https://raw.githubusercontent.com/EmbarkStudios/cargo-deny/main/src/advisories/cfg.rs`
  - `https://raw.githubusercontent.com/EmbarkStudios/cargo-deny/main/src/licenses/cfg.rs`
  - `https://raw.githubusercontent.com/EmbarkStudios/cargo-deny/main/src/sources/cfg.rs`
- rustfmt source/docs:
  - `https://raw.githubusercontent.com/rust-lang/rustfmt/master/src/config/options.rs`
  - `https://raw.githubusercontent.com/rust-lang/rustfmt/master/Configurations.md`
- Cargo config reference:
  - `https://doc.rust-lang.org/stable/cargo/reference/config.html`
- Direct tool attacks:
  - `rustup show active-toolchain` in temp dirs containing crafted `rust-toolchain.toml`
  - `rustup run nightly cargo -Z unstable-options config get --format=json-value` in temp dirs containing crafted `.cargo/config.toml`
- Prior parser-tightening batch:
  - `.worklogs/2026-04-05-141702-parser-contract-tightening-batch.md`

## Open Questions / Future Considerations
- `cargo-config-toml-parser` still leaves `[target.<triple>.<links>]` largely open. The docs show a more structured shape, but current Cargo probing is looser than the docs. Tightening that branch should wait for stronger source-level confirmation.
- `rustfmt-toml-parser` still has additional string-valued settings that may deserve enum tightening in a later pass if rustfmt’s source confirms closed domains.
- `guardrail3-rs-toml-parser` was audited against the local schema note in this session but did not need changes; any future tightening there should be driven by the repo’s own schema decisions rather than upstream tools.

## Key Files for Context
- `packages/deny-toml-parser/crates/parser/types/src/advisories.rs` — typed advisories config additions and new `AdvisoryScope`
- `packages/deny-toml-parser/crates/parser/types/src/licenses.rs` — clarify hash tightening to integer
- `packages/deny-toml-parser/crates/parser/types/src/sources.rs` — typed `GitSpec`
- `packages/rustfmt-toml-parser/crates/parser/types/src/rustfmt_toml.rs` — enum-based rustfmt option tightening
- `packages/rust-toolchain-toml-parser/crates/parser/types/src/rust_toolchain_toml.rs` — custom deserialization enforcing rustup path/toolchain constraints
- `packages/cargo-config-toml-parser/crates/parser/types/src/cargo_config_toml.rs` — include-path validation for Cargo config
- `.worklogs/2026-04-05-141702-parser-contract-tightening-batch.md` — previous parser-tightening batch that this work continues

## Next Steps / Continuation Plan
1. Stage only the four parser packages touched in this batch plus this worklog, and commit them without the unrelated clippy-family/runtime edits in the worktree.
2. Resume parser audit only where there is still a concrete contract mismatch backed by upstream behavior, starting with any remaining ambiguous branches in `cargo-config-toml-parser` or additional closed-choice fields in `rustfmt-toml-parser`.
3. When parser work pauses, fold the tightened parser outputs into the extracted `g3-*-content-checks` packages instead of adding new ad hoc parsing in family runtimes.
