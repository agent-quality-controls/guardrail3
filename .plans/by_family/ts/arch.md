# TS-ARCH

Status: planned family contract, no dedicated family runtime yet.

Implementation roots:

- `apps/guardrail3/crates/app/ts/validate/mod.rs`
- no cohesive `ts/arch` family implementation yet

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/arch.md` as the detailed family ledger until the cutover is complete

Current state:

- owns repo-global TypeScript root placement and architecture ownership in planning
- still missing a dedicated runtime/orchestrator surface in code

Rule inventory:

- `TS-ARCH-01` root discovery and classification
  - Should discover every governed TS root and classify it as `service`, `extension`, `content`, `library`, or `other`.
  - It is for establishing the shared ownership substrate that every other TS architecture family depends on.
- `TS-ARCH-02` misplaced root detection
  - Should error when a discovered TS root lives outside the allowed architecture zones.
  - It is for preventing silent repo sprawl and enforcing where TS roots are allowed to exist at all.
- `TS-ARCH-03` ambiguous architecture ownership
  - Should error when one TS root matches more than one governed family shape or declared type.
  - It is for keeping family ownership unambiguous before inside-the-zone families start applying their own rules.
- `TS-ARCH-04` illegal zone overlap or nesting
  - Should error on forbidden containment or overlap between governed TS roots.
  - It is for catching bad repo geometry that would otherwise create dual semantics and confusing ownership.
- `TS-ARCH-05` enablement coherence
  - Should fail closed when classified roots require `ts/hexarch`, `ts/libarch`, `ts/content`, `ts/i18n`, or `ts/seo` but the effective family enablement does not match.
  - It is for preventing a repo from looking clean simply because the required family was disabled or never routed.
- `TS-ARCH-06` config vs auto-detection mismatch
  - Should inventory or warn when explicit app/root typing disagrees with strong auto-detection signals.
  - It is for surfacing stale metadata before it turns into false ownership or misrouted family checks.

Current implementation mapping:

- there is no dedicated `TS-ARCH` family runtime yet
- the closest precursors are:
  - `discover_ts_apps(...)` in `apps/guardrail3/crates/app/ts/validate/ts_arch_checks.rs`
  - `auto_detect_app_type(...)` in `apps/guardrail3/crates/app/ts/validate/mod.rs`
  - `resolve_app_contexts(...)` in `apps/guardrail3/crates/app/ts/validate/mod.rs`
- those are currently grouped-runtime dispatch helpers, not a standalone repo-global placement family

Known reconciliation notes:

- the planning contract is clear that `TS-ARCH` should exist, but current code has no cohesive family owner for repo-global TS placement
- current grouped runtime defaults untyped apps to `service`, which is too opinionated to serve as a final `TS-ARCH` ownership decision
- current code knows about `service` and `content` auto-detection, but not a full explicit repo-global root inventory with overlap and misplaced-root findings

Historical/supplemental references:

- `.plans/todo/checks/ts/arch.md`
- `.plans/per-app-config-design/02-typescript-config-scoping.md`

Next planning focus:

- define the shared TS root/owner model against actual current code discovery
- decide whether `TS-ARCH-06` is a real user-facing rule or just internal reconciliation support
