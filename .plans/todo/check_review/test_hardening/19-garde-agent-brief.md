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
- `rs_garde_01_dependency_present.rs`
- `rs_garde_02_core_method_bans.rs`
- `rs_garde_03_extractor_type_bans.rs`
- `rs_garde_04_reqwest_json_ban.rs`
- `rs_garde_05_struct_derive_validate.rs`
- `rs_garde_06_additional_method_bans.rs`
- `rs_garde_07_manual_deserialize_impl.rs`
- `rs_garde_08_enum_derive_validate.rs`
- `rs_garde_09_query_as_inventory.rs`
- `rs_garde_10_input_failures.rs`

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

Rules `RS-GARDE-02..09` are conditional:
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

- expanded extractor bans are still missing from the canonical clippy baseline:
  - `axum::extract::Path`
  - `axum::extract::Multipart`
  - `axum::extract::ConnectInfo`
  - `axum_extra::extract::CookieJar`
  - `axum_extra::extract::cookie::Cookie`
  - `axum_extra::extract::TypedHeader`
  - `axum_extra::extract::JsonDeserializer`
  - `axum_extra::extract::JsonLines`
  - `axum_extra::extract::Protobuf`
  - `axum_extra::extract::Cbor`
  - `axum_extra::extract::MsgPack`
- wrapper-based validation boundary surface is still not implemented:
  - `ValidatedJson<T>`
  - `ValidatedQuery<T>`
  - `ValidatedForm<T>`
  - `Validated<T>`
- field-level garde quality is still not enforced:
  - meaningful garde constraints
  - `#[garde(dive)]` for nested validated fields
  - context-driven validation surfaces

## Main Structural Problem To Fix

The family still uses old rule sidecars like:
- `rs_garde_01_dependency_present_tests.rs`

The current standard is:
- one rule-specific `*_tests/` directory per rule
- one test = one attack vector
- each attack mutates every owned target it should matter for

The agent should convert the family to that structure.

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

## Mission

Harden `rs/garde` only.

Required outcomes:
- verify structure against the current checker architecture
- convert every rule from `*_tests.rs` to a rule-specific `*_tests/` directory
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
- exact-result assertions replace loose presence checks
- semantic bugs are either fixed or written down explicitly

## Suggested Start Order

1. read `garde.md` and map all 10 rules to current files
2. audit `mod.rs` / `facts.rs` / `discover.rs` for owned-root and covering-config behavior
3. map old `test_garde_checks.rs` tests to current rule IDs and attack vectors
4. convert rule sidecars from `*_tests.rs` to `*_tests/`
5. harden the highest-risk rules first:
   - `RS-GARDE-05`
   - `RS-GARDE-07`
   - `RS-GARDE-08`
   - `RS-GARDE-10`
6. update `garde.md` with closed and remaining gaps before stopping
