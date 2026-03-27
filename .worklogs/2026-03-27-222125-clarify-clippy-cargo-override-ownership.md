# Clarify Clippy Cargo Override Ownership

**Date:** 2026-03-27 22:21
**Scope:** `apps/guardrail3/crates/app/rs/families/clippy/README.md`, `apps/guardrail3/crates/app/rs/families/cargo/README.md`

## Summary
Clarified the family-boundary docs around `CLIPPY_CONF_DIR` and repo-local `.cargo/config(.toml)` override surfaces. The documented contract now makes explicit that this override remains `RS-CLIPPY`-owned even though it lives in Cargo config files, while `RS-CARGO` continues to own generic Cargo manifest and lint-table policy.

## Context & Problem
After adding `RS-CLIPPY-24`, the remaining ambiguity was ownership, not implementation. The new rule reads Cargo config files, which makes it look superficially like Cargo-family territory, but the semantic violation is still Clippy-specific: redirecting Clippy config discovery away from the routed policy-root model.

That ambiguity is dangerous because it invites future churn:
- moving the rule into `RS-CARGO` because the file path says “Cargo”
- or re-adding a parallel check in Cargo later

The current code already models the rule inside `RS-CLIPPY`. The docs needed to say that explicitly.

## Decisions Made

### Keep `CLIPPY_CONF_DIR` override ownership in `RS-CLIPPY`
- **Chose:** Document `RS-CLIPPY-24` as clippy-owned.
- **Why:** The semantic question is “has effective Clippy config discovery been redirected away from the routed policy-root model?” That belongs with the Clippy family, not with generic Cargo manifest policy.
- **Alternatives considered:**
  - Move it to `RS-CARGO` because the file is `.cargo/config.toml` — rejected because that would expand cargo from generic Cargo policy into tool-specific config-discovery semantics.
  - Leave ownership ambiguous and rely on current code — rejected because the ambiguity would likely reappear in later hardening work.

### Keep `RS-CARGO` scoped to generic Cargo policy
- **Chose:** Update cargo docs to say it does not own tool-specific Cargo config overrides that redirect another tool’s own policy discovery.
- **Why:** This preserves the current family split: Cargo owns Cargo manifest/lint-table policy, while tool families own their own effective policy semantics.
- **Alternatives considered:**
  - Say nothing in cargo docs — rejected because that leaves the apparent conflict unresolved.

## Architectural Notes
This keeps the current architecture coherent:

- `RS-CARGO` owns:
  - `Cargo.toml` workspace/member policy
  - lint tables
  - resolver / edition / rust-version / metadata drift
- `RS-CLIPPY` owns:
  - allowed Clippy policy roots
  - Clippy config coverage and shadowing
  - effective Clippy config-discovery bypasses such as `CLIPPY_CONF_DIR`

The key principle is semantic ownership over file-extension ownership. A tool-specific redirector should live with the tool whose policy semantics it bypasses.

## Information Sources
- Live family docs:
  - `apps/guardrail3/crates/app/rs/families/clippy/README.md`
  - `apps/guardrail3/crates/app/rs/families/cargo/README.md`
- Shared Rust family-boundary plan:
  - `apps/guardrail3/crates/app/rs/README.md`
- Live rule implementation:
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_24_forbid_clippy_conf_dir_override.rs`
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs`
- Prior worklog for the rule itself:
  - `.worklogs/2026-03-27-221725-close-clippy-cargo-config-override-gap.md`

## Open Questions / Future Considerations
- If the project later adds a broader “Cargo execution surfaces” family, this decision may be worth revisiting. Under the current family split, though, keeping the rule in `RS-CLIPPY` is the cleanest contract.
- The next clippy work should focus on new concrete detector bugs or top-level validator reruns once the outer workspace is healthy, not more ownership debate.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/clippy/README.md` — family-local ownership contract for Clippy
- `apps/guardrail3/crates/app/rs/families/cargo/README.md` — family-local ownership contract for Cargo
- `apps/guardrail3/crates/app/rs/README.md` — shared Rust root-scope and family-boundary architecture
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_24_forbid_clippy_conf_dir_override.rs` — live override rule
- `.worklogs/2026-03-27-221725-close-clippy-cargo-config-override-gap.md` — prior implementation checkpoint

## Next Steps / Continuation Plan
1. When the outer app workspace is healthy again, rerun top-level `RS-CLIPPY` and `RS-TEST` against `apps/guardrail3/crates/app/rs/families/clippy` to confirm the nested-workspace green state survives full validator entrypoints.
2. Continue clippy attack work only when there is a concrete new surface to test:
   - additional Clippy config-discovery bypasses
   - generator/checker drift
   - repo-visible false positives/false negatives
3. Otherwise, keep the local high-context lane moving to the next family while parallel handoffs continue on `deps` and `garde`.
