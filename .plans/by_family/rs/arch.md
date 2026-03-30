# RS-ARCH

Status: current, implemented, audited, self-hosted.

Implementation roots:

- `apps/guardrail3/crates/app/rs/families/arch/`
- `apps/guardrail3/crates/app/rs/placement/`
- `apps/guardrail3/crates/app/rs/family_selection/`

Current source of truth:

- this file for family planning/status
- `apps/guardrail3/crates/app/rs/families/arch/README.md` for family-local behavior
- `apps/guardrail3/crates/app/rs/README.md` for shared placement/routing architecture

Current state:

- repo-global Rust root placement and owner-family coherence live here
- the family is self-hosted with `crates/runtime`, `crates/assertions`, and `test_support`
- recent hardening closed the main audit gaps:
  - `RS-ARCH-04` is a layout-level overlap rule
  - `RS-ARCH-07` fails closed for malformed governed manifests and governed `arch_role`
  - explicit `--family arch` still runs even when `[rust.checks] arch = false`
  - app-scoped `hexarch` overrides are covered

Scope model:

- repo-global family over shared placement facts
- subtree validation must not silently localize misplaced-root or overlap
  findings away

Agent handoff focus:

- audit production path first:
  - `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
  - `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/placement/src/roots.rs`
  - `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`
- prove whole-project placement facts still reach `arch` when validation starts
  from a nested path
- prove repo-global findings remain repo-global by contract rather than by
  accidental overreach

Known current risk:

- subtree behavior is not pinned by enough current runtime tests

Done means:

- nested-path runtime tests prove `arch` still sees repo-global placement
  findings
- no family-local rediscovery of roots reappears
- README, plan, and tests agree that `arch` is global-only

Historical/supplemental references:

- `.plans/todo/checks/rs/arch.md`
- historical handoffs under `.plans/todo/check_review/test_hardening/29-*` and `35-*`

Next planning focus:

- keep shared placement/routing ownership in shared crates rather than re-growing family-local discovery
- add a shared README note for explicit requested-family override behavior if that becomes a repeated product rule
