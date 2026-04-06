# Fix Clippy Macro Ban Paths

**Date:** 2026-03-27 21:27
**Scope:** `apps/guardrail3/crates/domain/modules/clippy/macros.rs`, `apps/guardrail3/crates/app/rs/families/clippy/clippy.toml`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/{clippy_support.rs,rs_clippy_20_macro_bans.rs,rs_clippy_20_macro_bans_tests/missing_macros.rs,rs_clippy_18_duplicate_bans_tests/multiple_sections.rs,rs_clippy_08_reason_quality_tests/missing_reasons.rs,rs_clippy_15_trivial_reason_tests/placeholder_reasons.rs}`

## Summary
Corrected the `RS-CLIPPY` macro-ban baseline to match real Clippy semantics. The family and canonical generator were previously banning bare macro names like `println`, but Clippy only honors reachable macro paths like `std::println`; the family is now aligned to that reality while keeping user-facing diagnostics in plain macro form (`println!`, `dbg!`, etc.).

## Context & Problem
The clippy family had become internally self-consistent: generated `clippy.toml`, the family root `clippy.toml`, and `RS-CLIPPY-20` all agreed on banning:

- `println`
- `eprintln`
- `dbg`
- `todo`
- `unimplemented`

But I attacked this against the actual tool instead of just the family tests:

1. tiny probe crate with `disallowed-macros = ["println"]`
2. `cargo clippy`

Clippy reported:
- `` `println` does not refer to a reachable macro ``

Then the same probe with:
- `disallowed-macros = ["std::println"]`

produced the real lint on `println!`.

That means the family had a semantic false green: it was validating a config shape that Clippy itself does not actually enforce.

## Decisions Made

### Move the canonical macro baseline to fully qualified paths
- **Chose:** Change `EXPECTED_MACRO_BANS` and the rendered `MACRO_DEBUGGING` module to:
  - `std::println`
  - `std::eprintln`
  - `std::dbg`
  - `std::todo`
  - `std::unimplemented`
- **Why:** This is what real Clippy reaches and enforces.
- **Alternatives considered:**
  - Keep the short names and treat the Clippy warning as acceptable noise — rejected because that would mean guardrail3 is validating dead config.
  - Try to special-case bare names only in `RS-CLIPPY-20` — rejected because the generator and root config would still be wrong.

### Keep user-facing rule messages human-sized
- **Chose:** Add `display_macro_name(...)` in `clippy_support.rs` so `RS-CLIPPY-20` still reports:
  - `` `println!` is banned.``
  - not `` `std::println!` is banned.``
- **Why:** The config path must be fully qualified, but the user concept is still the macro invocation form.
- **Alternatives considered:**
  - Echo the raw config path in diagnostics — rejected because it is technically correct but unnecessarily noisy.

### Update the family’s own root config and targeted hand-written tests
- **Chose:** Patch the committed `apps/.../families/clippy/clippy.toml` and the specific tests that hand-wrote macro entries.
- **Why:** Leaving the family root on the old dead config would keep the family semantically non-self-hosting, even if the generator and rule changed.
- **Alternatives considered:**
  - Regenerate only via the domain module and leave the family root for a later pass — rejected because the family root is part of current self-hosted enforcement.

## Architectural Notes
- This was a rule-correctness fix, not repo cleanup.
- The canonical source of truth for macro bans remains the domain module generator in `domain/modules/clippy/`.
- `RS-CLIPPY-20` now matches:
  - canonical generation
  - the family root config
  - actual Clippy behavior
- `RS-CLIPPY-08`, `15`, and `18` manual tests were updated only where they referenced macro-ban paths directly.

## Information Sources
- Official Clippy docs:
  - `https://doc.rust-lang.org/clippy/lint_configuration.html`
- Direct local tool probes:
  - probe with `disallowed-macros = ["println"]` showed Clippy warning that the macro path is unreachable
  - probe with `disallowed-macros = ["std::println"]` produced the actual `disallowed_macros` lint
  - probe with `disallowed-macros = ["std::todo"]` likewise produced the actual lint
- Code and tests:
  - `apps/guardrail3/crates/domain/modules/clippy/macros.rs`
  - `apps/guardrail3/crates/app/rs/families/clippy/clippy.toml`
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_20_macro_bans.rs`
  - nested clippy workspace tests:
    - `cargo test --manifest-path apps/guardrail3/crates/app/rs/families/clippy/Cargo.toml -p guardrail3-app-rs-family-clippy --lib`

## Open Questions / Future Considerations
- The direct Clippy probe also suggests a possible follow-up on `RS-CARGO`: decide whether `clippy::disallowed_macros` should be explicitly denied in the Cargo lint baseline, even though Clippy currently warns on these bans without extra flags.
- The outer app workspace is still broken by the unrelated deny migration, so top-level `cargo test -p guardrail3-domain-modules` from `apps/guardrail3/Cargo.toml` still cannot run right now. The nested clippy workspace test run still recompiles `guardrail3-domain-modules`, so this change did get exercised indirectly.

## Key Files for Context
- `apps/guardrail3/crates/domain/modules/clippy/macros.rs` — canonical macro-ban source of truth
- `apps/guardrail3/crates/app/rs/families/clippy/clippy.toml` — family self-hosted root config now aligned to real Clippy semantics
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_20_macro_bans.rs` — macro-ban rule now using human-friendly display names over fully qualified config paths
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/clippy_support.rs` — macro display helper
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_20_macro_bans_tests/missing_macros.rs` — updated missing-macro regression
- `.worklogs/2026-03-27-211613-fix-clippy-policy-root-gaps.md` — earlier semantic clippy attack fix
- `.worklogs/2026-03-27-212201-split-clippy-library-type-ban-ownership.md` — immediate precursor on rule ownership cleanup

## Next Steps / Continuation Plan
1. Continue the `RS-CLIPPY` attack pass on the remaining semantic candidates:
   - `RS-CLIPPY-16`
   - `RS-CLIPPY-CONFIG-15`
   - `RS-CLIPPY-19`
2. Decide whether the Cargo lint baseline should explicitly deny `clippy::disallowed_macros` or whether the current Clippy default warning is sufficient for the project’s hardening bar.
3. Once the deny workspace conflict is gone, rerun top-level `RS-TEST`/family validation for `clippy` from the app root to confirm the structural side remains green after the semantic hardening.
