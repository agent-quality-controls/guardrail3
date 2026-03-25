# Hexarch Layered Test Map

This file makes the layered Rust test architecture concrete for `rs/hexarch`.

It answers:

- which `RS-HEXARCH-*` rules need core rule tests
- which behaviors belong to collector/facts tests instead
- which behaviors belong to family-orchestrator tests
- which behaviors belong only to golden integration tests

Use this with:

- `.plans/todo/checks/2026-03-25-rust-layered-test-architecture.md`
- `.plans/todo/checks/rs/hexarch.md`
- `.plans/todo/check_review/test_hardening/16-hexarch-execution-plan.md`
- `.plans/todo/check_review/test_hardening/32-hexarch-01-06-layered-migration-checklist.md`

## Hexarch-specific ownership

For `hexarch`, the family naturally splits into four test substrates:

1. structural facts
- `facts.rs`
- app-root discovery
- crate-dir discovery
- workspace member resolution
- nested-hex detection
- owned-path classification

2. dependency facts
- `dependency_facts.rs`
- manifest parsing
- dependency-edge collection
- workspace dependency inheritance
- rename / alias resolution
- cycle collection
- boundary config loading

3. source facts
- `source_facts.rs`
- reachable module graph
- source file collection
- trait / impl counting
- adapter trait visibility

4. family orchestrator
- `mod.rs`
- shared-input fan-out
- edge fan-out
- member-to-edge prebinding
- cross-rule ownership splits
- family-level exact owned hit/non-hit sets on non-golden inputs

Rules should stay thin over those facts.

If a test is mostly proving one of those collectors, it does not belong in a rule-sidecar unit layer.

## Standard per-rule shape

Every `RS-HEXARCH-*` rule gets:

1. core rule tests
- tiny typed facts only
- no filesystem
- no project walker
- no manifest parsing

2. collector/facts tests when the rule depends on discovery/parsing/classification
- exact discovered roots, members, edges, or source items
- raw collector fail-closed capture

3. family-orchestrator tests when the rule depends on shared fan-out or ownership splits
- shared input fan-out
- rule-to-rule ownership splits
- collector-backed rule application
- non-hit boundaries across sibling rules or sibling zones

4. family integration tests
- golden
- one attack vector per test
- exact owned hit/non-hit sets

The orchestrator layer is required for `hexarch`.
Do not force those tests into either raw collector assertions or full golden integration.

## Rule-by-rule map

### RS-HEXARCH-01 — `crates/` exists at app level

Core rule tests:
- pass when an owned app root fact says `crates/` exists
- fail when an owned app root fact says `crates/` is absent
- exact `Error` severity and app-root attribution

Facts tests:
- app-root discovery under `apps/*`
- nested inner hex roots are not owned by rule 01
- file/symlink replacement is captured as “not a real crates dir” in facts
- non-app roots are not materialized as owned app roots

Family-orchestrator tests:
- rule-01 ownership boundaries against non-app and nested-inner roots on real family inputs

Integration tests:
- golden app passes
- remove `crates/` from every owned app root at once
- replace `crates/` with file/symlink in every owned app root
- packages/non-Rust roots do not hit

### RS-HEXARCH-02 — exact top-level `crates/` contents

Core rule tests:
- pass on exact required set
- pass when `macros/` is present as the only optional extra
- fail on missing required dir
- fail on unexpected sibling

Facts tests:
- immediate child enumeration under owned `crates/`
- child symlink identity preserved
- child symlink identity preserved
- top-level loose-file discovery under `crates/`

Family-orchestrator tests:
- ownership split from rule 04 on top-level loose files
- ownership split from rule 03 when required directional containers are missing because parents are absent

Integration tests:
- golden pass
- remove required dirs across all owned app roots
- add unexpected dirs/files across all owned app roots
- nested-root parity attack
- optional `macros/` non-hit

### RS-HEXARCH-03 — `inbound/` and `outbound/` required in `adapters/` and `ports/`

Core rule tests:
- pass when both required children are present
- fail when either child is missing
- fail on unexpected directional sibling if the rule owns it

Facts tests:
- directional container discovery only when parent exists
- child symlink identity for `inbound/` / `outbound/`
- nested directional containers discovered correctly
- rule 03 does not over-own rule-02 parent absence

Integration tests:
- golden pass
- remove `inbound/`/`outbound/` in all matching containers
- add unexpected directional siblings
- nested-root parity
- rule-02 ownership split controls

### RS-HEXARCH-04 — loose files banned in structural/container dirs

Core rule tests:
- pass when only directories or allowed `.gitkeep` exist
- fail on a loose non-`.gitkeep` file
- exact file attribution
- real `.gitkeep` exemption only
- symlinked `.gitkeep` is not exempt

Facts tests:
- immediate loose-file discovery in structural dirs
- non-owned dirs do not surface owned loose-file facts

Family-orchestrator tests:
- ownership split from rule 02 and rule 05 on mixed structural attacks

Integration tests:
- golden pass
- add loose files broadly across owned structural/container dirs
- near-miss controls outside owned dirs
- mixed attacks that still leave rule-05 and rule-02 ownership separate

### RS-HEXARCH-05 — container dirs must not be empty

Core rule tests:
- pass when container has subdir content
- pass when container has real `.gitkeep`
- fail when container is empty
- empty vs `.gitkeep` placeholder semantics
- symlink placeholder does not count as valid content

Facts tests:
- raw child snapshot fields only
- non-owned containers excluded

Family-orchestrator tests:
- ownership split from rule 04 on mixed placeholder/loose-file attacks

Integration tests:
- golden pass
- empty all matching containers at once
- `.gitkeep` boundary controls
- nested parity where container ownership applies

### RS-HEXARCH-06 — leaf valid

Core rule tests:
- pass on valid package leaf
- pass on nested `crates/` inner-hex leaf
- pass on real `.gitkeep` placeholder leaf when allowed
- fail on invalid leaf shape

Facts tests:
- leaf classification:
  - package leaf
  - nested-hex leaf
  - placeholder leaf
  - invalid leaf
- ignored-dir and non-leaf boundaries
- symlink/permission edges
- manifest-kind validation once strengthened beyond “file named Cargo.toml exists”

Integration tests:
- golden pass
- broad invalid-leaf attacks
- valid alternative leaf controls
- nested-hex allowance controls

### RS-HEXARCH-07 — workspace members match crate dirs

Core rule tests:
- pass when discovered crate dirs exactly match resolved workspace members
- fail on missing member coverage
- exact missing-member attribution
- malformed workspace parse ownership split

Facts tests:
- owned crate-dir discovery
- workspace member normalization and resolution
- glob/normalized member semantics
- nested inner-hex leaves not misclassified as top-level member obligations

Family-orchestrator tests:
- shared-input ownership split across rules 07, 09, and 10

Integration tests:
- golden pass
- remove matching members from all owned app workspaces
- discovery-boundary controls
- nested-inner interactions only where rule 07 truly owns them

### RS-HEXARCH-08 — app `Cargo.toml` is workspace

Core rule tests:
- pass on workspace manifest
- fail on package-only manifest
- exact `Error` severity
- malformed TOML fail-closed ownership

Facts tests:
- app root manifest loading
- app-root-only ownership

Integration tests:
- golden pass
- strip `[workspace]` from every owned app root
- malformed manifest fail-closed attack
- valid workspace controls

### RS-HEXARCH-09 — no extra workspace members

Core rule tests:
- pass when every member corresponds to a discovered crate dir
- fail on extra/phantom member
- exact extra-member attribution

Facts tests:
- workspace member normalization
- internal normalized/glob members that are actually valid
- non-owned member strings not over-normalized into owned false positives

Integration tests:
- golden pass
- add extra members across all owned app workspaces
- valid normalized-member non-hits
- exact ownership split from rule 10

### RS-HEXARCH-10 — members remain within app boundary

Core rule tests:
- pass when all members resolve inside the app boundary
- fail on member escaping the app boundary
- exact offending member path

Facts tests:
- path normalization and boundary resolution
- internal normalized members remain in-boundary
- external or upward-traversal members resolve out-of-boundary
- malformed path resolution fail-closed where applicable

Integration tests:
- golden pass
- broad out-of-boundary member attacks
- exact offending-path assertions
- valid in-boundary controls

### RS-HEXARCH-11 — root workspace must not include apps

Core rule tests:
- pass when root workspace excludes app members
- fail when root workspace includes app crates
- exact root-manifest attribution
- malformed root workspace parse ownership
- app-member recognition from resolved members

Facts tests:
- root workspace member resolution

Integration tests:
- golden pass
- add app members to root workspace
- valid root workspace controls

### RS-HEXARCH-12 — app-level `src/` banned

Core rule tests:
- pass when app root has no `src/`
- fail when app root has `src/`
- exact app-root attribution

Facts tests:
- app-root child discovery
- actual crate `src/` dirs under member crates are not owned
- nested or non-app `src/` directories are non-hits

Integration tests:
- golden pass
- add `src/` to every owned app root
- actual member-crate `src/` controls

### RS-HEXARCH-13 — dependency direction

Core rule tests:
- pass on allowed direction edges
- fail on forbidden direction edges
- exact source/target attribution
- normal dependency table ownership only

Facts tests:
- dependency edge collection
- real path-backed member ownership
- same-name external crate non-hit
- out-of-tree path with layer-like name non-hit

Family-orchestrator tests:
- ownership split between rules 13, 20, 24, and 25 on shared edge sets

Integration tests:
- golden pass
- broad illegal-direction attack across all matching edges
- allowed-direction controls
- exact owned edge set

### RS-HEXARCH-14 — dependency inventory

Core rule tests:
- exact inventory result shape for a typed edge set
- `Info` severity exactness

Facts tests:
- dependency inventory collection exactness
- edge deduping / normalization

Integration tests:
- golden smoke baseline only
- non-inventory rule separation controls

### RS-HEXARCH-15 — boundary config present

Core rule tests:
- pass when required per-app boundary config exists
- warn when missing
- exact file attribution
- malformed `guardrail3.toml` parse-error ownership

Facts tests:
- boundary config lookup by app
- non-app roots excluded

Integration tests:
- golden pass
- omit config for one app
- omit config for all apps
- malformed config fail-closed

### RS-HEXARCH-16 — `[patch]` / `[replace]` bypass

Core rule tests:
- fail when a patch/replace path resolves to a forbidden layer target
- pass when patch target is out of owned layered space or otherwise valid

Facts tests:
- workspace patch / replace table parsing
- resolved path ownership
- broken patch target non-hit if the rule requires a real resolved owned target

Integration tests:
- golden pass
- add `[patch]` path bypasses
- add `[replace]` path bypasses
- valid external patch targets non-hit

### RS-HEXARCH-17 — inherited workspace dependency direction

Core rule tests:
- fail when inherited workspace path dep resolves to a forbidden direction
- pass for allowed inherited edges

Facts tests:
- `workspace = true` resolution into `[workspace.dependencies]`
- path-backed inherited dep ownership
- version-only inherited dep non-hit

Family-orchestrator tests:
- ownership split between rules 17 and 18 on inherited renamed dependencies

Integration tests:
- golden pass
- inherited forbidden-edge attack
- version-only inherited dependency controls

### RS-HEXARCH-18 — renamed dependency direction

Core rule tests:
- fail when alias + `package` resolves to forbidden direction
- pass when alias resolves to allowed target
- excludes inherited/dev/target edges outside rule-18 ownership

Facts tests:
- alias resolution
- `package` field resolution

Family-orchestrator tests:
- ownership split between rules 17 and 18
- ownership split from rules 20 and 25 on non-normal edges

Integration tests:
- golden pass
- renamed forbidden-edge attack
- valid alias controls
- inherited-renamed ownership split from rule 17

### RS-HEXARCH-19 — same-layer cycles

Core rule tests:
- fail on a supplied same-layer cycle fact
- exact emitted result shape, severity, and attribution

Facts tests:
- pass on acyclic graph
- exact cycle member set construction
- cycle collection exactness
- unlayered members excluded from same-layer cycle ownership
- cross-layer cycle ownership boundaries

Family-orchestrator tests:
- family suppression / ownership boundaries when cycle facts are absent or out of scope

Integration tests:
- golden pass
- create same-layer cycle
- acyclic same-layer chain control
- exact cycle-member assertions

### RS-HEXARCH-20 — dev-dependency direction

Core rule tests:
- warn on forbidden dev edge
- pass on allowed dev edge
- exact `Warn` severity
- dev-only ownership split

Facts tests:
- dev dependency table collection
- out-of-tree layer-like path non-hit

Family-orchestrator tests:
- ownership split between rules 13, 20, and 25

Integration tests:
- golden pass
- forbidden dev-edge attack
- target-dev ownership split to rule 25

### RS-HEXARCH-21 — domain purity

Core rule tests:
- pass on allowed pure deps
- fail on forbidden external dep
- fail on forbidden optional dep
- pass on configured `allowed_deps`
- pure allowlist application
- dev deps excluded from rule ownership

Facts tests:
- manifest dep collection across normal/build/optional surfaces
- workspace inherited external dep resolution
- workspace inherited alias resolution
- non-member/out-of-tree pure-looking paths non-hit

Family-orchestrator tests:
- member-to-edge prebinding and ownership over collected edges

Integration tests:
- golden pass
- broad forbidden-dep attack
- build-dependency inclusion
- optional dependency inclusion
- configured allowlist controls

### RS-HEXARCH-22 — ports trait dominance

Core rule tests:
- pass when `pub trait` count is dominant or balanced per policy
- warn when impl-heavy ports crate violates the rule
- exact crate attribution
- non-ports crates are out of scope
- parse / missing-entrypoint warning ownership

Facts tests:
- reachable module graph from entrypoint
- trait/impl counting across files
- inline module descent
- unreachable orphan file exclusion
- test-only item exclusion

Family-orchestrator tests:
- filesystem-backed source collector feeding final rule ownership on non-golden inputs

Integration tests:
- golden pass
- impl-heavy ports crate attack through real source tree
- DTO/private-trait controls
- multi-file aggregation attack

### RS-HEXARCH-23 — adapter `pub trait` banned

Core rule tests:
- fail on `pub trait` in adapter crate
- pass on `pub(crate)` / `pub(super)` traits
- exact source file attribution
- non-adapter crates are out of scope
- parse / missing-entrypoint error ownership

Facts tests:
- reachable module graph
- trait visibility extraction
- inline and nested module handling
- unreachable/test-only item exclusion

Family-orchestrator tests:
- filesystem-backed source collector feeding final rule ownership on non-golden inputs

Integration tests:
- golden pass
- add `pub trait` to adapter source reachable from entrypoint
- `pub(crate)` controls
- inline/nested module attacks

### RS-HEXARCH-24 — cross-app boundary violation

Core rule tests:
- fail when a real resolved path dep crosses app boundaries
- pass on same-app or package/shared edges
- exact source app / target app attribution
- unresolved or broken target stays out of rule ownership

Facts tests:
- app ownership resolution for dependency endpoints
- cross-app vs package/shared classification

Family-orchestrator tests:
- ownership split between rules 24 and 25 on target-specific cross-app edges

Integration tests:
- golden pass
- cross-app leaks across normal/dev/build/target sections
- package/shared controls
- broken-target ownership controls

### RS-HEXARCH-25 — target dependency direction

Core rule tests:
- fail on forbidden target-specific edge
- pass on allowed target-specific edge
- exact source/target attribution
- target-only ownership split
- handoff to rule 24 for cross-app target edges

Facts tests:
- target dependency table collection:
  - `target.*.dependencies`
  - `target.*.dev-dependencies`
  - `target.*.build-dependencies`

Family-orchestrator tests:
- ownership split between rules 13, 20, 24, and 25

Integration tests:
- golden pass
- forbidden target-edge attacks across all target table kinds
- valid target-edge controls
- cross-app target ownership split

## Immediate reclassification work

Current `hexarch` test directories already contain many files that belong in collector, orchestrator, or integration layers instead of rule-sidecar unit coverage.

First explicit moves:

- anything proving raw child discovery, root discovery, workspace member normalization, edge collection, or source reachability moves to collector tests
- anything proving rule ownership split, path coverage, sibling-rule non-hit behavior, or collector-backed rule application moves to orchestrator tests
- anything mutating the golden tree broadly across owned roots stays or moves to integration tests
- rule-sidecar unit tests keep only direct rule-semantic assertions over typed facts

Concrete examples:

- `discovery_scope.rs`
- `ownership_boundaries.rs`
- `path_resolution.rs`
- `reachable_modules.rs`
- `source_layout.rs`
- `cycle_collection.rs`

These are not core rule tests.
Some are collector tests, and some are orchestrator tests.
Do not move them wholesale into one bucket.

Files like:

- `golden.rs`
- `broad_attacks.rs`
- `compound_attacks.rs`

are integration tests unless they are rewritten to use tiny inline typed inputs only.

## First migration slice

Do `RS-HEXARCH-01..06` first.

Why:

- they are the clearest example of the current confusion between rule semantics and discovery semantics
- they exercise the structural facts layer heavily
- they will force the right split between:
  - app-root discovery
  - child discovery
  - rule ownership

Then do:

- `RS-HEXARCH-07..12`

Then the dependency block:

- `RS-HEXARCH-13..21`

Then the source block:

- `RS-HEXARCH-22..23`

Then finish with:

- `RS-HEXARCH-24..25`

because those depend on the dependency-facts ownership split being clean first.
