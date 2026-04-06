# RS-DEPS

Status: current, implemented, self-hosted, inventory-complete, still worth hardening.

Implementation root:

- `apps/guardrail3/crates/app/rs/families/deps/`

Current source of truth:

- this file for family planning, target state, and remaining execution pressure
- `apps/guardrail3/crates/app/rs/families/deps/README.md` for family-local runtime behavior and structure
- `.plans/todo/checks/rs/deps.md` as the detailed historical rule ledger and migration record

## What RS-DEPS Is For

`RS-DEPS` is the Rust dependency-policy family.

It exists to enforce:

- required external tool presence on `PATH`
- direct external dependency allowlist policy from validation-root `guardrail3.toml`
- allowlist coverage expectations for library-profile crates
- `Cargo.lock` presence and `.gitignore` masking policy
- fail-closed reporting when dependency-policy inputs cannot be trusted

It is workspace-local:

- universal dependency baseline rules should be enforced per workspace
- local allowlist tightening should remain workspace-local
- the family must not collapse to one repo-global weakest-common-denominator policy

## What It Does Not Own

`RS-DEPS` must stay out of:

- banned crates in the resolved dependency graph or lockfile
  - that belongs to `RS-DENY`
- architecture and dependency direction
  - that belongs to `RS-HEXARCH`
- release/publish dependency policy
  - that belongs to `RS-RELEASE`
- hook execution of dependency tools
  - that belongs to hook families
- tool minimum-version policy
  - not currently frozen enough to harden here

If a future change starts pushing `RS-DEPS` into those surfaces, it is probably the wrong family.

## Current State

Implemented and live:

- `RS-DEPS-01..12`
- self-hosted family layout with:
  - `crates/runtime`
  - `crates/assertions`
  - `test_support`

Current explicit gaps:

- the family still needs continued pressure on workspace-local exactness and
  fail-closed behavior as adjacent families evolve

Recently hardened:

- `RS-DEPS-11` now ignores foreign `rust.*` and crate-policy keys when deps-owned fields are valid
- local path dependencies that point at undeclared Cargo packages under a workspace root now fail closed through `RS-DEPS-11`
- malformed `target.*` dependency tables now fail closed through `RS-DEPS-11` without suppressing top-level `RS-DEPS-CONFIG-01..07` findings
- `RS-DEPS-CONFIG-01..07` now explicitly cover both top-level and `target.*` dependency tables for their owned section kinds
- subtree regressions now prove routed `RS-DEPS-CONFIG-01` and `RS-DEPS-CONFIG-05` findings do not leak into sibling crates
- `RS-DEPS-09/10` no longer invent nested non-member helper crates under a workspace root as separate lockfile roots
- deps routing/runtime now preserve ancestor workspace roots needed for lockfile policy when scoped app/package config enables the family

## Scope Model

- workspace-local family
- universal baseline rules should apply to every routed legal workspace
- local allowlist and dependency-cap rules stay workspace-local rather than
  collapsing into one repo-global policy
- lockfile checks bind to legal workspace roots rather than standalone package
  escape hatches

## Agent Handoff Focus

- audit production path first:
  - `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
  - `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/facts.rs`
  - `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`
- prove the workspace-local contract survives the whole-project walker change
- prove subtree runs do not overreach into unrelated workspaces for lockfile
  and allowlist checks

## Known Current Risk

- no confirmed production bug yet, but this family still carries old mixed-scope
  assumptions in docs/runtime and can drift silently if that cleanup stalls

## Done Means

- subtree tests prove baseline, lockfile, and allowlist findings all keep the
  right workspace-local boundary
- no ad hoc root rediscovery bypasses the route
- target-table behavior remains explicit rather than silently broadened

## Recently Closed Inventory Gap

### `RS-DEPS-CONFIG-05`

This was the last still-planned rule needed to make the dependency-policy inventory complete.

Target contract:

- one finding per crate/package whose unique direct dependency count exceeds `25`
- count unique dependency package names, not aliases
- count across the direct dependency tables that belong to the rule’s contract
- do not double-count the same resolved package name when it appears in multiple sections or tables

Counted universe should be explicit:

- top-level:
  - `[dependencies]`
  - `[build-dependencies]`
  - `[dev-dependencies]`
- target-specific:
  - `target.*.dependencies`
  - `target.*.build-dependencies`
  - `target.*.dev-dependencies`

Resolution rules should stay aligned with existing dependency normalization:

- renamed dependencies count by real `package` name when present
- `workspace = true` counts when it resolves to an external package
- internal workspace-path dependencies do not count
- non-workspace external path dependencies do count

The family is now explicit about those choices in code, tests, and docs.

## Hard Target State

`RS-DEPS` is only in a strong target state when all of the following are true:

- `RS-DEPS-CONFIG-05` is implemented and stays fact-driven rather than reparsing manifests per rule
- the workspace-local contract remains explicit and visible in docs
- dependency counting semantics are centralized instead of re-parsed ad hoc in each rule
- `workspace = true`, renamed dependencies, and internal workspace-path exclusions behave consistently between allowlist ownership and dependency-count ownership
- target-specific dependency tables are explicitly owned where enforced and never silently ignored
- malformed manifests that are needed for owned dependency-policy behavior fail closed through `RS-DEPS-11`
- tests prove exact ownership and edge behavior rather than just broad count changes

## Design Rules For Future Work

When extending this family:

- prefer one normalized dependency universe over per-rule bespoke parsers
- keep external-vs-internal dependency resolution explicit
- do not collapse crate-local dependency policy into repo-root-only behavior
- do not let target-table support leak silently from one rule into another
- do not weaken the fail-closed model to avoid implementation work
- do not let deps-owned config parsing reject adjacent-family `guardrail3.toml` growth

Good future changes:

- a new fact type for one new rule with clear ownership
- better exactness tests for existing workspace-local behavior
- local README + plan updates in the same change as semantic shifts

Bad future changes:

- broad “count everything in Cargo.toml” heuristics
- repo-root-only shortcuts
- counting internal workspace path crates just to make the cap fire
- adding release or deny semantics here because the data is nearby

## Attack Surfaces To Keep Pressure On

Any new deps work should actively pressure these cases:

- renamed dependency aliases that resolve to one package
- the same package repeated across multiple sections
- `workspace = true` resolving to:
  - external package
  - internal workspace path package
  - missing workspace dependency entry
- local path dependencies that point at real Cargo packages under a workspace root without workspace membership
- non-workspace path dependencies that point at vendored external crates
- hybrid workspace roots that are also packages
- target-specific dependency tables
- malformed `Cargo.toml` inputs that would otherwise suppress owned findings

If a change does not make these cases clearer, it is probably not hardening the family.

## Immediate Planning Focus

Near-term priority:

1. keep the counted universe for `RS-DEPS-CONFIG-05` explicit and minimal
2. preserve the existing family boundary with `deny`, `hexarch`, and `release`
3. keep target-specific allowlist ownership explicit and tested
4. continue adversarial pressure on workspace-local and fail-closed cases

This is not a broad “dependency cleanup” lane.
It is now a family-hardening lane.

## Done Means

The current deps lane is in a good state when:

- `RS-DEPS-CONFIG-05` stays live in `crates/runtime`
- matching assertions and sidecar regressions stay in place
- family tests keep passing
- the deps ledger and local README stay aligned with the live contract
- target-table ownership stays explicit in both docs and tests

## Historical / Supplemental References

- `.plans/todo/checks/rs/deps.md`
- `.plans/todo/family-implementation-handoffs/deps.md`
- family stabilization handoff docs under `.plans/todo/family-stabilization-handoffs/`
