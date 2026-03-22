# RS-CARGO — Cargo.toml workspace lints checker (9 rules)

**Input:** Cargo.toml (workspace root + per-crate)
**Parser:** TOML
**Current code:** `cargo_lints.rs`, `workspace_metadata.rs`

## Implementation mapping contract

- exactly one `RS-CARGO-*` rule ID per production file
- exactly one sidecar `*_tests.rs` file per production rule file
- `mod.rs` orchestrates only
- `facts.rs`, `inputs.rs`, `discover.rs`, and `lint_support.rs` may contain shared facts, typed inputs, discovery, canonical baseline data, and normalization helpers only

Forbidden:

- production files that bundle multiple independent rule surfaces
- grouped family test files such as `cargo_tests.rs`
- helper files that hide multiple rule predicates behind one API

## Rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-CARGO-01 | R26 | Error | [workspace.lints.rust] + [workspace.lints.clippy] completeness: 6 Rust lints + 31 clippy deny lints + 4 clippy groups present. **Profile-aware:** library profile adds `unreachable_pub = "deny"` to expected Rust lints. `missing_docs` is intentionally not enforced. | Implemented |
| RS-CARGO-02 | R27 | Error/Warn | Lint levels correct (deny/warn/forbid/allow match expected). Error if weakened, warn otherwise | Implemented |
| RS-CARGO-03 | R28 | Info | Approved allow deviations inventory (9 lints: missing_docs_in_private_items, module_name_repetitions, etc.) | Implemented |
| RS-CARGO-04 | R29 | Error | Per-crate lint inheritance: `[lints] workspace = true` in each member Cargo.toml | Implemented |
| RS-CARGO-05 | R55 | Info/Warn | Workspace edition + rust-version metadata. **Enhancement:** library profile Warn if `rust-version` absent (MSRV is a compatibility contract for libraries). Service profile Info inventory only. | Implemented (needs library Warn) |

## New rules from audit

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-CARGO-06 | Error | Per-crate lint overrides weaken workspace. After confirming `workspace = true`, scan each member's `[lints.rust]` and `[lints.clippy]` for entries whose level is WEAKER than the workspace level. Weakening is ordinal: `forbid > deny > warn > allow`. So `forbid→deny` IS a weakening (`deny` can be overridden with `#[allow]`, `forbid` cannot). Each weakening is Error. | Planned |
| RS-CARGO-07 | Warn | Lint group priority ordering. Specific lints with `priority < 0` get overridden by groups at `priority = -1`. Verify all specific deny lints have priority >= 0 (or no priority, default 0) so they take precedence over groups. Flag any specific lint with `priority < 0`. | Planned |
| RS-CARGO-08 | Error | `resolver = "2"` enforcement. Virtual workspaces (no `[package]` section) silently use resolver v1, causing test-only features to leak into production. Must have `resolver = "2"` or `resolver = "3"` explicitly. Non-virtual workspaces with `edition = "2021"+` default to v2 — Info if missing, not Error. | Planned |
| RS-CARGO-09 | Warn | Per-crate edition override detection. A member crate setting `edition = "2018"` while workspace uses `edition = "2024"` silently downgrades safety (e.g., implicit unsafe in extern blocks on older editions). Flag any per-crate edition older than workspace edition. | Planned |
| RS-CARGO-10 | Warn | Declared workspace member missing `Cargo.toml`. If `[workspace].members` declares a directory but no member manifest is discovered there, warn explicitly rather than silently skipping it. | Planned |

## Code fixes for migration

| Location | Bug | Fix |
|----------|-----|-----|
| `cargo_lints.rs` line 313 | `crate_cargo.exists()` bypasses FileSystem trait | Change to `fs.metadata(&crate_cargo).is_none()`. Also emit Warn when a declared member has no Cargo.toml (silent skip currently); this is now first-class `RS-CARGO-10`. |
| `workspace_metadata.rs` wiring | RS-CARGO-05 called from `config_files.rs`, not `cargo_lints.rs` | Move call into cargo_lints module so all RS-CARGO checks are co-located. |
| Canonical lint drift | `EXPECTED_*` arrays in cargo_lints.rs can drift from canonical.rs `CARGO_LINTS` module | Add `#[test]` that parses canonical content and verifies consistency with expected arrays. Not a runtime rule. |

## Explicitly rejected

| Finding | Why rejected |
|---------|-------------|
| `[profile.dev/test]` settings | Subjective. Release profile already covered by RS-CARGO-05. |
| `[workspace.metadata]` custom sections | Extension point for third-party tools. guardrail3 can't validate arbitrary tool config. |
| `default-members` vs `members` | Pre-commit hook uses `cargo test --workspace` which ignores default-members. |
| Workspace deps with dangerous features | deny.toml feature bans (RS-DENY) handle this. Duplicate check. |
| Lint name typos | `warnings = "deny"` (RS-CARGO-01) + cargo clippy catches unknown lint names at compile time. |
| MSRV as separate rule | Folded into RS-CARGO-05 enhancement (library Warn, service Info). |
