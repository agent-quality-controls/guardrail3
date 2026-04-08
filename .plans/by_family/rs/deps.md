# RS-DEPS

Status: current planning target, package rewrite required before ingestion.

Implementation root:

- `packages/rs/deps/g3rs-deps-types`
- `packages/rs/deps/g3rs-deps-config-checks`
- `packages/rs/deps/g3rs-deps-config-ingestion` (planned)
- future file-tree package(s) for local path target validation (planned)

Current source of truth:

- this file for family planning, target state, and remaining execution pressure
- `.plans/todo/checks/rs/deps.md` as the detailed historical rule ledger and migration record

## What RS-DEPS Is For

`RS-DEPS` is the Rust dependency-policy family.

It exists to enforce:

- required external tool presence on `PATH`
- direct external dependency allowlist policy from required workspace `guardrail3-rs.toml`
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

What exists today:

- `g3rs-deps-config-checks` exists
- `g3rs-deps-types` exists
- no `g3rs-deps-config-ingestion` yet
- no deps file-tree checks package yet

Known stale surfaces:

- current `packages/rs/deps/*` still depend on app `GuardrailConfig`
- current `packages/rs/deps/*` still describe `guardrail3.toml`
- legacy app-family mapper / facts / route code is not part of the target architecture

Current explicit gaps:

- rewrite `g3rs-deps-types` around `guardrail3-rs.toml`
- rewrite `g3rs-deps-config-checks` to consume normalized dependency facts rather than raw local path discovery state
- build `g3rs-deps-config-ingestion`
- define deps file-tree checks for local path target validation and malformed local target ownership

## Scope Model

- workspace-local family
- universal baseline rules should apply to every legal Rust workspace
- local allowlist and dependency-cap rules stay workspace-local rather than
  collapsing into one repo-global policy
- lockfile checks bind to legal workspace roots rather than standalone package
  escape hatches
- ingestion packages are the orchestrators/mappers; no app-family mapper is part of the target design

## Package Boundary

Target split:

- config checks:
  - `RS-DEPS-CONFIG-01`
  - `RS-DEPS-CONFIG-02`
  - `RS-DEPS-CONFIG-03`
  - `RS-DEPS-CONFIG-04`
  - `RS-DEPS-CONFIG-05`
- file-tree checks:
  - local path target existence
  - local path target `Cargo.toml` validity
  - local path target package identity validity
  - internal/external local path classification
  - in-workspace non-member local target failures
- structural / environment checks:
  - `RS-DEPS-01..04`
  - `RS-DEPS-09..11`

Config checks must not receive:

- app config types
- family routes
- local path target manifests
- raw local path target strings used only for file-tree validation

Config checks should receive one input per crate containing:

- crate identity
- workspace-local deps policy from `guardrail3-rs.toml`
- normalized external dependency entries already resolved for:
  - section ownership
  - package identity
  - internal-vs-external classification

## Config Input Contract

Target shape:

```rust
pub struct G3RsDepsConfigChecksInput {
    pub crate_cargo_rel_path: String,
    pub crate_name: String,
    pub profile: Option<RustProfile>,
    pub allowlist_present: bool,
    pub allowed_deps: Vec<String>,
    pub dependencies: Vec<G3RsDepsResolvedDependency>,
}
```

```rust
pub struct G3RsDepsResolvedDependency {
    pub package_name: String,
    pub section: G3RsDepsDependencySection,
    pub table_label: String,
}
```

Why this is the target:

- `RS-DEPS-CONFIG-01..03` need normalized external dependency names plus section ownership
- `RS-DEPS-CONFIG-04` needs profile/library-ness plus allowlist presence
- `allowlist_present` is explicit because the current `guardrail3-rs.toml` parser normalizes missing and empty `allowed_deps` to the same `Vec<String>` shape
- `RS-DEPS-CONFIG-05` needs normalized dependency names for unique counting

They do not need full workspace manifests, full crate manifests, or file-tree target manifests once ingestion has normalized dependency identity.

## Ingestion Contract

Target API:

```rust
ingest_for_config_checks(&crawl) -> Result<Vec<G3RsDepsConfigChecksInput>, Error>
```

Responsibilities:

1. require and parse workspace `Cargo.toml`
2. require and parse workspace `guardrail3-rs.toml`
3. discover workspace members
4. parse member `Cargo.toml` files
5. normalize dependency entries for config checks
6. fail if required config inputs are missing or invalid

Not owned by config ingestion output:

- file-tree rule results
- local path target structural failure reporting as config facts

## Agent Handoff Focus

- fix package boundary first:
  - `packages/rs/deps/g3rs-deps-types`
  - `packages/rs/deps/g3rs-deps-config-checks`
  - `packages/parsers/guardrail3-rs-toml-parser`
- then build config ingestion around workspace crawl
- then define the file-tree package for local path target failures
- do not carry app-family mapper or app `GuardrailConfig` assumptions into the rewrite

## Known Current Risk

- the current deps packages encode the wrong architecture boundary and can drift farther if new code keeps building on the app-config model

## Done Means

- config checks run only on package-owned normalized dependency facts
- deps packages use `guardrail3-rs.toml`, not app `GuardrailConfig`
- config ingestion exists and returns one input per crate
- file-tree ownership for local path target failures is explicit and separate
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
- malformed config inputs needed for config ingestion fail ingestion cleanly
- malformed local path targets are owned by file-tree checks, not smuggled into config checks
- tests prove exact ownership and edge behavior rather than just broad count changes

## Design Rules For Future Work

When extending this family:

- prefer one normalized dependency universe over per-rule bespoke parsers
- keep external-vs-internal dependency resolution explicit
- do not collapse crate-local dependency policy into repo-root-only behavior
- do not let target-table support leak silently from one rule into another
- do not weaken the fail-closed model to avoid implementation work
- do not depend on app config types in `packages/rs/*`
- do not push file-tree target validation back into config checks

Good future changes:

- a new normalized config fact type with clear ownership
- better exactness tests for existing workspace-local behavior
- local README + plan updates in the same change as semantic shifts

Bad future changes:

- broad “count everything in Cargo.toml” heuristics
- repo-root-only shortcuts
- counting internal workspace path crates just to make the cap fire
- adding release or deny semantics here because the data is nearby
- reintroducing raw local path manifests into config-check inputs
- reintroducing app-family mapper logic into package ingestion

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
- malformed member `Cargo.toml` inputs needed for config normalization
- malformed local target `Cargo.toml` inputs that belong to file-tree ownership

If a change does not make these cases clearer, it is probably not hardening the family.

## Immediate Planning Focus

Near-term priority:

1. rewrite `g3rs-deps-types` to remove app `GuardrailConfig`
2. rewrite `g3rs-deps-config-checks` around normalized dependency facts
3. build `g3rs-deps-config-ingestion`
4. design deps file-tree checks for local path target validation
5. preserve the existing family boundary with `deny`, `hexarch`, and `release`

This is not a legacy app-family cleanup lane.
It is a package-architecture correction lane.

## Done Means

The current deps lane is in a good state when:

- `g3rs-deps-config-checks` consumes only package-owned normalized facts
- `g3rs-deps-config-ingestion` exists
- deps config packages depend on `guardrail3-rs.toml`
- file-tree/local-path ownership is explicit in a separate package or rule lane
- the deps ledger stays aligned with that contract

## Historical / Supplemental References

- `.plans/todo/checks/rs/deps.md`
- older handoff docs under `.plans/todo/family-implementation-handoffs/` and `.plans/todo/family-stabilization-handoffs/` are historical only
