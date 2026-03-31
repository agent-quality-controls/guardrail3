# RS-ARCH

Status: current, newly introduced, actively expanding.

Implementation root:

- `apps/guardrail3/crates/app/rs/families/arch/`

Current source of truth:

- this file for family planning/status
- `apps/guardrail3/crates/app/rs/families/arch/README.md` for family-local behavior
- `apps/guardrail3/crates/app/rs/README.md` for shared runtime/mapper architecture

Current state:

- runtime/model/config/reporting selection know `arch`
- lean app builds and runs through `--features family-arch`
- current live rules own the generic split-library contract that used to sit in `libarch`:
  - escalation from flat library into split architecture
  - split root must remain a workspace facade package
  - split root must actually own internal member crates
  - external crates must not depend directly on those internal member crates
- current candidate detection is intentionally package-scoped while this family is being established

Scope model:

- routed root family
- current production facts only materialize package-scoped split-library candidates
- findings may still reference external consumers outside `packages/*` when those consumers depend on internal member crates directly

Agent handoff focus:

- audit production path first:
  - `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
  - `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`
  - `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/facts.rs`
- prove the family sees flat package libraries, split package roots, and cross-root dependency hits
- keep `arch` generic; do not reintroduce the old `api/core/infra` layered-shape policy here

Known current risk:

- the first cut is intentionally narrower than the long-term goal; it currently targets package-scoped split-library candidates, not every conceivable app-local crate architecture pattern

Done means:

- flat package libraries over threshold emit `RS-ARCH-01`
- broken split roots emit `RS-ARCH-02/03`
- direct dependencies on internal member crates emit `RS-ARCH-04`
- `libarch` no longer emits the migrated generic split rules
- lean CLI runs show `RS-ARCH-*` without `RS-LIBARCH-01/02/03`

Historical/supplemental references:

- `.plans/todo/arch-topology-libarch-migration-handoff.md`
- `.plans/todo/arch-workspace-membership-exactness-handoff.md`
- `.plans/todo/checks/rs/libarch.md`

Next planning focus:

- broaden `arch` from package-scoped split-library candidates toward the final repo-wide crate-architecture surface
- continue shrinking `libarch` until only genuinely obsolete legacy layered-shape checks remain
