# Deny Second Hardening Pass

**Date:** 2026-03-30 21:17
**Scope:** `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/*`, deny rule sidecar tests

## Summary
Ran a second adversarial pass on the deny family after the first hardening checkpoint. This pass fixed additional fail-open and over-broad behaviors around profile routing, duplicate detection, malformed exception/source entries, and exact allow-list enforcement, then revalidated the family through both the full deny test suite and the isolated family-only binary path.

## Context & Problem
The first deny hardening pass closed several obvious gaps, but the family still had uncovered edge cases. The next pass was driven by direct test attacks plus focused subagent review. The main goal was to make deny stricter where it governs policy escape hatches and to remove places where malformed or duplicate config could either evade enforcement or create misleading noise.

The user also reaffirmed the testing bar for family owners: the family must still compile and run through the lean `family-<family>` path even when unrelated work in the repo is broken.

## Decisions Made

### Tighten exact policy ownership instead of accepting close-enough config
- **Chose:** `g3rs-deny/confidence-threshold` now errors on extra globally allowed licenses, not just missing expected ones.
- **Why:** `[licenses].allow` is an allow-list, and the documented escape hatch for exceptions is `[[licenses.exceptions]]`. Allowing silent extras weakens the policy surface.
- **Alternatives considered:**
  - Only check for missing expected licenses — rejected because it permits unreviewed global broadening.
  - Push extra-license handling into schema warnings — rejected because this is semantic drift, not just malformed shape.

### Make app-specific deny profile routing win over generic package routing
- **Chose:** `rust.apps.<name>` profile mappings are preserved when `rust.packages` also applies to the same resolved root.
- **Why:** app-specific profile routing is more specific than the generic package fallback. Overwriting it caused standalone apps to inherit the wrong deny profile.
- **Alternatives considered:**
  - Keep last-writer-wins insertion order — rejected because it makes broader package policy override a more specific app rule.
  - Remove `rust.packages` handling from deny entirely — rejected because current policy still allows package-level fallback until the workspace-only model lands.

### Evaluate all managed tokio feature-ban entries, not just the first one
- **Chose:** `g3rs-deny/skip-hygiene` now checks every `tokio` feature-ban entry and warns if any managed `tokio` entry drifts.
- **Why:** duplicate `tokio` entries could previously hide a noncanonical second entry behind a canonical first entry.
- **Alternatives considered:**
  - Let `g3rs-deny/license-exceptions-inventory` own the problem alone — rejected because duplicate detection should not be the only guard against semantic drift.
  - Emit one warning per bad duplicate — rejected because that would create noisy repetition; one semantic warning plus duplicate reporting is enough.

### Normalize duplicate skip identity semantically
- **Chose:** `g3rs-deny/license-exceptions-inventory` now keys skip duplicates by normalized crate-plus-version identity when version exists, ignores malformed skip/ignore entries, and trims advisory IDs.
- **Why:** same-crate different-version skip entries are distinct exceptions and should not warn as duplicates. Malformed entries should not collapse into fake duplicate buckets.
- **Alternatives considered:**
  - Keep name-only duplicate keys — rejected because it creates false positives for valid version-distinct exceptions.
  - Treat all malformed entries as a single duplicate identity — rejected because it invents noise from broken input that other rules should own.

### Turn blank exception/source strings into hard errors
- **Chose:** `RS-DENY-17` now rejects blank crate identifiers and blank license names inside exception allow-lists; `g3rs-deny/extra-feature-bans-inventory` now errors on blank `allow-git` entries.
- **Why:** these are malformed escape hatches. Inventory is only useful when the exception/source entry is real and reviewable.
- **Alternatives considered:**
  - Leave blank strings to `g3rs-deny/allow-override-channel` schema warnings — rejected because the data shape is technically string-typed but still semantically unusable.
  - Ignore blank strings silently — rejected because that fails open.

### Prove profile-sensitive ownership end-to-end
- **Chose:** added rule-level tests showing when a standalone app profile should override package fallback and when a local library override must not rewrite the ancestor service root.
- **Why:** the facts-layer tests proved routing, but the family also needed end-to-end ownership proof through real rule output.
- **Alternatives considered:**
  - Rely on facts tests only — rejected because profile-sensitive rules need end-to-end coverage, not just route construction coverage.

## Architectural Notes
The deny family remains a policy-enforcement family, not a workspace-shape checker. This pass kept the fixes inside deny’s actual remit:
- profile resolution and config ownership
- canonical baseline parity
- duplicate exception semantics
- malformed escape-hatch rejection

The upcoming repo move toward workspace-only Rust roots should still be enforced by cargo/placement families rather than by deny pretending to own workspace topology.

## Information Sources
- Prior deny checkpoint: `.worklogs/2026-03-30-205737-deny-family-hardening.md`
- Family plans: `.plans/by_family/rs/deny.md`, `.plans/todo/checks/rs/deny.md`
- Current deny runtime and tests under `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src`
- Adversarial subagent reports from the second pass:
  - duplicate `tokio` bypass and duplicate-identity drift
  - app-vs-package profile precedence bug
  - blank license exception and blank `allow-git` malformed-input slips
- Lean family validation commands executed locally:
  - `cargo check --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 --no-default-features --features family-deny`
  - `cargo build --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 --no-default-features --features family-deny`
  - `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 --no-default-features --features family-deny -- rs validate apps/guardrail3 --family deny --format json`

## Open Questions / Future Considerations
- `g3rs-deny/ignore-hygiene` still likely duplicates inventory for repeated extra feature-ban entries; that is policy noise, not a correctness blocker, and was left for a later pass.
- `RS-DENY-30` likely has similar duplicate-inventory noise for wrapper-bearing duplicate bans.
- The repo still reports `apps/guardrail3/crates/app/rs/validate` as a standalone package root under the current cargo/placement model. That is a cargo/workspace-shape issue, not a deny bug.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/facts_support.rs` — profile routing for deny configs, including app-vs-package precedence
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/facts_tests/mod.rs` — deny facts coverage, scoped routing, and profile-context fail-closed tests
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/bans/rs_deny_config_18_tokio_full_ban.rs` — managed tokio feature policy enforcement
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/bans/rs_deny_config_24_duplicate_entries.rs` — duplicate identity normalization for deny/skip/ignore/features
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/licenses/rs_deny_config_11_license_allow_baseline.rs` — exact global allow-list enforcement
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/licenses/rs_deny_17_license_exceptions_inventory.rs` — license exception hygiene and inventory
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/sources/rs_deny_config_17_allow_git_inventory.rs` — explicit git-source exception handling
- `.worklogs/2026-03-30-205737-deny-family-hardening.md` — first deny hardening pass that this work builds on

## Next Steps / Continuation Plan
1. Run one more adversarial pass on the remaining duplicate/inventory-only rules, especially `g3rs-deny/ignore-hygiene` and `RS-DENY-30`, and decide whether duplicate inventory should collapse to one logical item plus one duplicate warning.
2. If the repo moves to the workspace-only Rust-root model, coordinate the structural enforcement in cargo/placement and then narrow deny’s allowed-root model accordingly.
3. When the cargo/workspace family is ready, revisit the remaining standalone package root reported under `apps/guardrail3` and remove that structural noise from isolated deny runs.
