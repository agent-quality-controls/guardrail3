# Garde Agent Brief

This is the droppable handoff file for the `rs/garde` hardening pass.

## Read First

Read these in order:

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
4. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
5. `.plans/todo/checks/rs/garde.md`

## Primary Code

- `apps/guardrail3/crates/app/rs/checks/rs/garde/`

Important family files:
- `mod.rs`
- `discover.rs`
- `facts.rs`
- `inputs.rs`
- `parse.rs`
- `garde_support.rs`
- `test_support.rs`

Rules:
- `rs_garde_config_01_dependency_present.rs`
- `rs_garde_config_02_core_method_bans.rs`
- `rs_garde_config_03_extractor_type_bans.rs`
- `rs_garde_config_04_reqwest_json_ban.rs`
- `rs_garde_ast_01_struct_derive_validate.rs`
- `rs_garde_config_05_additional_method_bans.rs`
- `rs_garde_ast_02_manual_deserialize_impl.rs`
- `rs_garde_ast_03_enum_derive_validate.rs`
- `rs_garde_ast_04_query_as_inventory.rs`
- `rs_garde_10_input_failures.rs`
- `rs_garde_ast_05_field_level_constraints.rs`
- `rs_garde_ast_06_nested_validation_dive.rs`
- `rs_garde_ast_07_context_validation_surface.rs`

## Legacy Seed Material

- `apps/guardrail3/tests/unit/test_garde_checks.rs`

Use old tests as attack-vector seed material only. Do not port them mechanically.

## Family Contract

`rs/garde` is a multi-root family.

Owned Rust policy roots:
- workspace roots
- standalone package roots that are not members of a workspace

Per owned root, the family must determine:
- whether garde is actually enabled for that root
- which covering `clippy.toml` applies to that root
- which Rust source files belong to that root

Important verified semantics:
- if the root config is package-driven by `[rust.packages]`, root garde gating must inherit `[rust.packages.checks]`
- the root must not always fall back to the global default garde setting
- otherwise `g3rs-garde/core-method-bans..09` can fail open or overfire at the root

Rules `g3rs-garde/core-method-bans..09` are conditional:
- if garde is absent for an owned root, those rules do not fire for that root

This is product gating, not a fail-open loophole.

## Fail-Closed Contract

Required inputs for an owned root:
- root `Cargo.toml`
- covering `clippy.toml`
- relevant Rust source files
- root/policy inputs needed to decide whether garde is enabled

Unreadable or unparsable required inputs must surface through `RS-GARDE-10`.

That includes:
- Cargo root discovery failures
- `clippy.toml` parse failures for owned roots
- Rust source read failures
- Rust source parse failures

Malformed inputs must not silently suppress source-level findings.

## Cross-Family Dependency

`rs/garde` depends on `rs/clippy` being modeled correctly.

Specifically:
- clippy owns canonical ban configuration
- garde owns garde-specific ban requirements and source-level derive/bypass rules
- garde must resolve the covering `clippy.toml` per owned root, not repo-globally

## Known Live Gaps

These remain active even though the family is implemented:

- the facts layer just closed one real mixed-profile/root bug:
  - `facts.rs` was treating the root as default-garde-driven even when `[rust.packages]` owned the root config shape
  - that root package-policy inheritance is now fixed
  - a direct regression exists in `garde_facts_tests.rs`
  - focused verification should keep using a stable family `CARGO_TARGET_DIR` such as `target/garde` to avoid lock fights with other agents

- expanded extractor bans are no longer a live gap:
  - the canonical clippy generator now emits the full g3rs-garde/extractor-type-bans extractor set
  - rule-local parity exists in `rs_garde_config_03_extractor_type_bans_tests/parity.rs`
  - generator/checker parity is also pinned by the clippy type-ban parity tests
- expanded deserialization method bans are no longer a live gap:
  - the canonical clippy generator now emits the g3rs-garde/additional-method-bans method set as part of the managed garde method baseline
  - `g3rs-garde/core-method-bans/04/06` golden tests now read the canonical generated clippy baseline instead of hand-written method lists
  - clippy-side parity is pinned through `rs_clippy_04_missing_method_ban_tests/parity.rs`
- source-level multi-root tests were corrected to the actual root model:
  - `RS-GARDE-AST-01/07/08/09` multi-root tests now use standalone package roots not enrolled in a workspace
  - do not model workspace members as owned garde roots; that contradicts both the plan and `facts.rs`
- derive coverage is broader now:
  - `RS-GARDE-AST-01` covers `Parser`, `Args`, and `FromRow`, plus the primitive-only `char` exemption
  - `RS-GARDE-AST-03` covers `Parser`, `Args`, and `FromRow`
  - `RS-GARDE-AST-04` now has the explicit `query_scalar!` non-hit the plan calls out
- field-level garde quality is no longer just prose:
  - `RS-GARDE-AST-05` now enforces meaningful field-level garde constraints
  - `RS-GARDE-AST-06` now enforces `#[garde(dive)]` on nested validated fields
  - `RS-GARDE-AST-07` now enforces `#[garde(context(...))]` when field validators reference `ctx`
- wrapper-based validation boundary surface remains checker-adjacent guidance, not a distinct garde AST rule:
  - the active enforceable contract here is still the clippy ban surface on raw extractors and raw deserialization
  - do not reopen this as a garde rule unless the project introduces an actual wrapper API surface that guardrail3 can statically detect

## Current Test Shape

The family already uses the required structure:
- one rule-specific `*_tests/` directory per rule
- one test file per attack vector

The remaining job is semantic hardening:
- exact owned hit / non-hit assertions
- canonical generator parity
- multi-root ownership coverage
- fail-closed coverage on policy and source inputs

Current verification status:
- `CARGO_TARGET_DIR=target/garde-check cargo check --manifest-path apps/guardrail3/Cargo.toml --tests` passed
- `CARGO_TARGET_DIR=target/clippy-check cargo check --manifest-path apps/guardrail3/Cargo.toml --tests` passed
- the new `RS-GARDE-AST-05/12/13` packet compile-checks cleanly under the garde target too:
  - `CARGO_TARGET_DIR=target/garde-check cargo check --manifest-path apps/guardrail3/Cargo.toml --tests`
- the new `RS-GARDE-AST-05/12/13` packet runtime verification command set is:
  - `CARGO_TARGET_DIR=target/garde cargo test --manifest-path apps/guardrail3/Cargo.toml --lib rs_garde_11_`
  - `CARGO_TARGET_DIR=target/garde cargo test --manifest-path apps/guardrail3/Cargo.toml --lib rs_garde_12_`
  - `CARGO_TARGET_DIR=target/garde cargo test --manifest-path apps/guardrail3/Cargo.toml --lib rs_garde_13_`
- those runtime commands currently spend a long time in the monolithic `guardrail3` lib-test link step rather than failing fast on the garde packet itself
- full `cargo test` on the monolithic `guardrail3` test binary is still expensive enough that repeated family-scoped links are not a good tight-loop verifier; use compile-check first, then run full tests when the lane is ready

## Required Attack Classes

Every rule should move toward:

1. golden pass
2. attack-vector tests
3. exact owned hit set
4. exact owned non-hit set
5. multi-root coverage
6. precedence / inheritance / shadowing where applicable
7. false-positive control
8. fail-closed coverage where applicable
9. exact severity assertions

For this family, the highest-signal attack classes are:
- garde gating per owned root
- covering `clippy.toml` precedence per owned root
- alias-aware `Deserialize` / `Validate` / `query_as!` detection
- manual `Deserialize` impl bypasses
- enum false-positive control for C-like enums
- fail-closed Rust source and policy-input failures
- field-level garde validator adequacy
- nested `#[garde(dive)]` ownership
- explicit `ctx` surface wiring

## Mission

Harden `rs/garde` only.

Required outcomes:
- verify structure against the current checker architecture
- add golden coverage for every rule
- add at least one real attack-vector test for every rule
- assert exact owned hits, exact owned non-hits, exact rule ID, and exact severity
- port legacy garde test ideas by attack vector, not by filename
- fix real semantic bugs you find
- update `.plans/todo/checks/rs/garde.md` with:
  - gaps closed
  - gaps remaining
  - policy questions, if any

## Do Not

- add grouped family test files
- leave rule tests as `*_tests.rs`
- write happy-path-only tests
- assert only that “some result exists”
- silently change policy to make tests pass
- expand work into `clippy` or other families unless there is a genuine blocker

## Done Means

The pass is not done until:

- every rule has a rule-specific `*_tests/` directory
- every rule has golden coverage
- every rule has at least one real attack-vector test
- generator/checker parity is pinned where the rule depends on canonical clippy policy
- exact-result assertions replace loose presence checks
- semantic bugs are either fixed or written down explicitly

## Suggested Start Order

1. read `garde.md` and map all 13 rules to current files
2. audit `mod.rs` / `facts.rs` / `discover.rs` for owned-root and covering-config behavior
3. map old `test_garde_checks.rs` tests to current rule IDs and attack vectors
4. harden the highest-risk rules first:
   - `RS-GARDE-AST-01`
   - `RS-GARDE-AST-02`
   - `RS-GARDE-AST-03`
   - `RS-GARDE-10`
5. close generator/checker drift in the canonical clippy baseline before trusting golden cases
6. update `garde.md` with closed and remaining gaps before stopping
