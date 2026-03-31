# RS-TOPOLOGY

Status: current, implemented, audited, self-hosted.

Implementation roots:

- `apps/guardrail3/crates/app/rs/families/topology/`
- `apps/guardrail3/crates/app/rs/placement/`
- `apps/guardrail3/crates/app/rs/family_selection/`

Current source of truth:

- this file for family planning/status
- `apps/guardrail3/crates/app/rs/families/topology/README.md` for family-local behavior
- `apps/guardrail3/crates/app/rs/README.md` for shared placement/routing architecture

Current state:

- repo-global Rust root placement and owner-family coherence live here
- repo-global legality should expand here as the shared pre-family legality surface, not as ad hoc family-order dependence
- exact workspace membership for governed workspaces now lives here as one topology concept
- the family is self-hosted with `crates/runtime`, `crates/assertions`, and `test_support`
- recent hardening closed the main audit gaps:
  - `RS-TOPOLOGY-04` is a layout-level overlap rule
  - `RS-TOPOLOGY-07` fails closed for malformed governed manifests and governed `arch_role`
  - explicit `--family topology` still runs even when `[rust.checks] topology = false`
  - app-scoped `hexarch` overrides are covered

Scope model:

- repo-global reporting family over shared Rust structure and legality facts
- subtree validation must not silently localize misplaced-root or overlap
  findings away

Target split:

- shared Rust structure pass discovers roots and attaches family-owned files
- shared Rust legality pass decides what is legal before family slicing
- `RS-TOPOLOGY` reports those legality failures
- mapper slices legal family surfaces
- runners fan workspace-local surfaces into one invocation per legal workspace

Test contract:

- `topology` is the owner of illegal topology and illegal placement tests
- `topology` is the owner of exact workspace-membership tests
- workspace-local families must not keep those tests by rebuilding fake routes
- if a test needs an illegal root or misplaced family file, it belongs here
- if a test is pure rule logic, it should use the rule input directly instead of pretending to be a routed family test

Agent handoff focus:

- audit production path first:
  - `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
  - `apps/guardrail3/crates/app/rs/families/topology/crates/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/placement/src/roots.rs`
  - `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`
- prove whole-project placement facts still reach `topology` when validation starts
  from a nested path
- prove repo-global findings remain repo-global by contract rather than by
  accidental overreach

Known current risk:

- subtree behavior is not pinned by enough current runtime tests

Done means:

- nested-path runtime tests prove `topology` still sees repo-global placement
  findings
- no family-local rediscovery of roots reappears
- README, plan, and tests agree that `topology` is global-only

Historical/supplemental references:

- `.plans/todo/checks/rs/topology.md`
- historical handoffs under `.plans/todo/check_review/test_hardening/29-*` and `35-*`

Next planning focus:

- keep shared structure/legality ownership in shared crates rather than re-growing family-local discovery
- pin the mapper-vs-runner split explicitly:
  - mapper builds legal family surfaces
  - runners build actual invocation units
- add a shared README note for explicit requested-family override behavior if that becomes a repeated product rule
- drive the remaining cargo/deny migration by rewriting illegal family fixtures into:
  - legal workspace-local routed tests
  - `topology` legality tests
  - direct typed-input rule tests
