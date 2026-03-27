# Tighten RS-CODE Filesystem Attack Coverage

**Date:** 2026-03-27 17:10
**Scope:** `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/fs_visitors.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_15_direct_fs_usage.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_21_fs_glob_import.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_15_direct_fs_usage_tests/*`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_21_fs_glob_import_tests/*`, `.plans/todo/checks/rs/code.md`

## Summary
Adversarial review of `RS-CODE` found two concrete detector bugs in the filesystem rules. I fixed `RS-CODE-15` / `RS-CODE-21` so `extern crate std as alias` no longer bypasses detection, and so explicit filesystem-boundary modules are exempted consistently, including crate-root `fs/src/lib.rs`.

## Context & Problem
After stabilizing the `code` family, the next step was to attack the live `RS-CODE` buckets instead of blindly reducing counts. Sampling repo-wide inventory showed three things:

- `RS-CODE-24` and `RS-CODE-32` were dominated by plausible real debt.
- `RS-CODE-15` still reported the shared filesystem abstraction crate at `apps/guardrail3/crates/shared/fs/src/lib.rs`, which is the intended central `std::fs` boundary.
- The `std::fs` visitors tracked `use std as alias` but not `extern crate std as alias`, leaving a real bypass hole for both direct calls and glob imports.

The work here was therefore detector hardening, not repo cleanup.

## Decisions Made

### Treat crate-root `fs/src/lib.rs` as an explicit filesystem boundary
- **Chose:** Extend the exemption logic in `RS-CODE-15` and `RS-CODE-21` from only `src/fs.rs` to a generic filesystem-boundary shape: `src/fs.rs`, `src/fs/mod.rs`, and crate-root `fs/src/lib.rs`.
- **Why:** The repo already has a dedicated shared fs crate whose whole purpose is to centralize `std::fs` access. Reporting that crate made the rule contradict the architecture it is supposed to protect.
- **Alternatives considered:**
  - Hardcode `apps/guardrail3/crates/shared/fs/src/lib.rs` — rejected because it would bake repo-specific naming into a generic code rule.
  - Leave the rule as-is and treat the shared fs crate as debt — rejected because it would force callers away from the intended boundary instead of toward it.

### Track `extern crate std as alias` in fs visitors
- **Chose:** Extend `fs_visitors.rs` so all `std::fs` detectors collect aliases introduced by `extern crate std as s;` in addition to `use std as s;`.
- **Why:** `RS-CODE-15` and `RS-CODE-21` are bypass rules. Missing one valid Rust alias form would leave a silent escape hatch.
- **Alternatives considered:**
  - Only fix `RS-CODE-15` — rejected because the same alias model is shared by the glob-import rule.
  - Ignore `extern crate` as obsolete syntax — rejected because the whole point of the family is to close bypasses, including awkward but valid ones.

### Update inventory expectations instead of weakening them
- **Chose:** Rewrite the `RS-CODE-21` inventory tests to use ordinary owned files rather than the newly exempt fs-boundary crate.
- **Why:** The exemption should narrow the rule only where architecture requires it. Inventory coverage still needs to prove the rule fires on normal owned source files.
- **Alternatives considered:**
  - Delete the old inventory expectations outright — rejected because it would reduce attack coverage.
  - Keep expecting hits from fs-boundary crates — rejected because that would encode the old bug into tests.

## Architectural Notes
`RS-CODE-15` and `RS-CODE-21` are not “ban all filesystem references” rules. They are boundary-enforcement rules: direct `std::fs` should be concentrated into explicit filesystem surfaces. The correct detector shape is therefore:

- broad enough to catch alias tricks and glob tricks
- narrow enough to exempt dedicated fs-boundary modules
- not dependent on repo-specific paths or crate names outside those structural roles

This keeps the rule aligned with the repo’s shared-fs architecture and prevents the rule from fighting the intended abstraction layer.

## Information Sources
- `.plans/todo/checks/rs/code.md` — current `RS-CODE` rule contract and wording
- `apps/guardrail3/crates/app/rs/families/code/README.md` — family-level architecture intent
- `apps/guardrail3/crates/shared/fs/src/lib.rs` — concrete evidence that the repo centralizes filesystem access in a dedicated crate
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/fs_visitors.rs` — current alias/detection implementation
- repo-wide inventory snapshots:
  - `/tmp/rs_code_attack_inventory.json`
  - `/tmp/rs_code_attack_inventory_after.json`

## Open Questions / Future Considerations
- `RS-CODE-24` still reports a large bucket of `#[path]` usage. Sampling suggests most of it is real debt or tolerated legacy shape rather than a detector bug, but it still needs a later policy decision on how much old test wiring we want to keep.
- `RS-CODE-32` also has a large bucket. The sampled hits looked legitimate, but it should get another adversarial pass focused on edge-case message literals and mixed test/non-test contexts.
- `RS-CODE-15` still reports old family-local `test_support.rs` files. That currently looks like real debt, not a rule bug.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/fs_visitors.rs` — alias-aware `std::fs` detection logic
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_15_direct_fs_usage.rs` — direct-call/import rule and filesystem-boundary exemption
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_21_fs_glob_import.rs` — glob-import companion rule and shared exemption
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_15_direct_fs_usage_tests/false_positives.rs` — regression for fs-boundary exemptions
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_21_fs_glob_import_tests/direct.rs` — regression for `extern crate std as alias`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_21_fs_glob_import_tests/inventory.rs` — updated inventory expectations after the exemption
- `.plans/todo/checks/rs/code.md` — source-of-truth rule table
- `.worklogs/2026-03-27-161428-rs-code-attack-fixes.md` — prior `RS-CODE` adversarial hardening pass
- `.worklogs/2026-03-27-165201-enforce-test-expect-policy.md` — prior `RS-CODE-32` addition and expect-policy split

## Next Steps / Continuation Plan
1. Continue adversarial review of the remaining dominant live buckets, starting with `RS-CODE-24` and `RS-CODE-32`, using repo-wide inventory samples instead of just family self-host tests.
2. If another concrete detector bug is found, add a focused regression first, then patch the rule before touching repo code.
3. Once the `RS-CODE` attack surface looks stable, move to the next family stabilization target (`release` is the current best candidate).
