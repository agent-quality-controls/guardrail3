# Freeze Generator Specs

**Date:** 2026-03-24 13:23
**Scope:** `.plans/todo/generate/README.md`, `.plans/todo/generate/rs/README.md`, `.plans/todo/generate/rs/gap-analysis.md`, `.plans/todo/generate/rs/{fmt,toolchain,clippy,deny,cargo,release,hooks,hexarch,libarch}.md`

## Summary
Created the Rust generator planning surface as a real target-state spec set instead of a mixed roadmap/current-state note bundle. The family specs now define only end-state generator ownership, root taxonomy, checker targets, and parity proofs, while current implementation mapping moved into a dedicated generator gap-analysis doc.

## Context & Problem
The checker-side family plans had already been tightened into verification-grade contracts, but the new generator planning docs were still written partly in terms of current helper functions and current CLI behavior. That made them bad source-of-truth files: an implementation agent could not reliably compare code to plan because the plan itself still leaked current implementation and helper-specific assumptions.

The user explicitly wanted the generator plans to describe the exact state we are going toward, with no “eventually”, no helper-shaped ownership, and no prioritization language. The generator specs also had to account for the actual mixed-root fixture complexity already present in the golden Rust fixture:
- multiple app workspaces
- package libraries
- nested inner hex roots
- non-Rust apps living beside Rust roots

## Decisions Made

### Split target-state spec from current implementation mapping
- **Chose:** keep the family files under `.plans/todo/generate/rs/` as pure target-state contracts and move current implementation notes into `.plans/todo/generate/rs/gap-analysis.md`.
- **Why:** a planning file that mixes “what must exist” with “what code does today” is not a trustworthy contract for later verification.
- **Alternatives considered:**
  - Leave source-file/function references inside each family spec — rejected because helper names like `resolve_rust_root(cfg)` and current CLI wiring are implementation details, not architecture.
  - Write no current-state doc at all — rejected because the project still needs one place to record generator/checker mismatches and current implementation gaps.

### Define generator ownership in root-taxonomy terms
- **Chose:** rewrite family ownership around validation root, Rust policy root, workspace member root, hex structural root, and layered library root.
- **Why:** the old drafts leaked current config/helper semantics. The real contract has to survive generator refactors and remain correct for mixed repos.
- **Alternatives considered:**
  - Keep ownership phrased via current helper/config concepts — rejected because that bakes current CLI plumbing into the plan.
  - Define everything from repo-root only — rejected because clippy/deny/cargo/hexarch/libarch all need mixed-root behavior.

### Make generator families obey checker-family ownership, not current generator behavior
- **Chose:** align generator family ownership to the active checker contracts even when current generator behavior differs.
- **Why:** checker contracts are already the enforcement target. Generator parity only means something if generation aims at the checker-owned end state.
- **Alternatives considered:**
  - Mirror current generator behavior first and “tighten later” — rejected because that would lock in already-known mismatches.

### Freeze the strongest mixed-root decisions explicitly
- **Chose:** freeze these specific points in the generator plans:
  - `fmt` is validation-root-only, not per-app/per-root
  - `toolchain` is validation-root-only
  - `clippy`, `deny`, and `cargo` are Rust-policy-root families
  - `release` is validation-root-only even in mixed repos
  - `hooks` standardizes on `.githooks/` only; checker compatibility for legacy `hooks/pre-commit` does not make that generator-owned
  - nested inner hex roots remain structural only and do not become policy/workspace roots
  - layered `libarch` roots own their own workspace boundary and are not members of ancestor workspaces
- **Why:** these are the places where mixed monorepo shape could otherwise be “interpreted” by future agents in incompatible ways.
- **Alternatives considered:**
  - Leave the mixed-root edge cases implicit — rejected because the golden fixture and adversarial fixtures already show those shapes are real.

## Architectural Notes
- The generator planning layer now mirrors the checker layer:
  - one family spec per family
  - explicit ownership mode
  - explicit root selection
  - explicit checker target
  - explicit parity proof
- `gap-analysis.md` is intentionally the only file in this new planning area that is allowed to talk about current implementation and current helper functions.
- `release` remains the only meaningful open product-level question after this pass: whether release should stay validation-root-owned or grow into a multi-release-domain model. The docs now describe the current intended contract cleanly instead of leaving it half-implicit.

## Information Sources
- `.plans/todo/checks/rs/{fmt,toolchain,clippy,deny,cargo,release,hexarch,libarch}.md`
- `.plans/todo/checks/hooks/{shared,rs}.md`
- `apps/guardrail3/tests/fixtures/r_arch_01/golden`
- `apps/guardrail3/tests/adversarial_deep_nesting.rs`
- `apps/guardrail3/tests/adversarial_path_resolution.rs`
- `apps/guardrail3/tests/adversarial_nightmare_monorepo.rs`
- current generator-related code for mapping only:
  - `apps/guardrail3/crates/adapters/inbound/cli/generate.rs`
  - `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers.rs`
  - `apps/guardrail3/crates/domain/modules/{canonical,deny,release,pre_commit}.rs`
  - `apps/guardrail3/crates/domain/modules/clippy/*`
- prior worklogs:
  - `.worklogs/2026-03-24-114556-tighten-rust-plan-contracts.md`
  - `.worklogs/2026-03-24-121556-release-hardening-and-parity-audit.md`
  - `.worklogs/2026-03-24-121642-finalize-libarch-root-shape.md`

## Open Questions / Future Considerations
- `release` is now cleanly specified as validation-root-only, but that is still a product choice rather than a forced law of nature. If nested app workspaces need independent release domains later, the family and generator plans will need a deliberate redesign.
- `hooks` are now generator-specified as modular `.githooks/`; current monolithic generation is tracked only in `gap-analysis.md`.
- `cargo`, `hexarch`, and `libarch` remain the biggest generator implementation gaps, but those are implementation concerns now, not planning ambiguities.

## Key Files for Context

- `.plans/todo/generate/README.md` — top-level Rust generator contract and ownership-mode definitions.
- `.plans/todo/generate/rs/README.md` — Rust generator root taxonomy and family contract template.
- `.plans/todo/generate/rs/gap-analysis.md` — only current-state mapping/gap file for Rust generator work.
- `.plans/todo/generate/rs/fmt.md` — root-only formatting generator contract, intentionally stricter than current code.
- `.plans/todo/generate/rs/cargo.md` — semantic-patch manifest ownership contract for Rust policy roots.
- `.plans/todo/generate/rs/release.md` — validation-root-only release generator contract, including workflows.
- `.plans/todo/generate/rs/hooks.md` — preferred modular `.githooks/` generator contract.
- `.plans/todo/generate/rs/hexarch.md` — mixed-root app + nested-inner-hex scaffold contract.
- `.plans/todo/generate/rs/libarch.md` — layered library scaffold contract, including ancestor-workspace exclusion.
- `.plans/todo/checks/rs/release.md` — checker-side release ownership that informed the release generator contract.
- `.plans/todo/checks/rs/libarch.md` — checker-side layered library contract that informed the generator shape.
- `.worklogs/2026-03-24-121556-release-hardening-and-parity-audit.md` — prior decision record on release checker/generator parity pressure.

## Next Steps / Continuation Plan

1. Use the new generator family specs as the only source of truth when implementing generator parity in code. Do not re-introduce helper-specific ownership language into the family plans.
2. When generator implementation begins, keep all current-code notes and mismatches in `.plans/todo/generate/rs/gap-analysis.md` and update that file instead of diluting the family specs.
3. Decide the product-level release-domain question explicitly before any attempt to make release generation multi-root. If the answer stays validation-root-only, keep nested app workspaces out of release generation.
4. Start actual generator implementation with the cleanest exact-owned surfaces first:
   - `toolchain`
   - `fmt`
   - `clippy`
   - `deny`
   - modular `hooks`
   Then move to semantic-patch/scaffold families:
   - `cargo`
   - `hexarch`
   - `libarch`
   - release workflow parity
