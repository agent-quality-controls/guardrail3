# Hexarch 01-06 Layered Migration Checklist

This is the concrete migration checklist for:

- `RS-HEXARCH-01`
- `RS-HEXARCH-02`
- `RS-HEXARCH-03`
- `RS-HEXARCH-04`
- `RS-HEXARCH-05`
- `RS-HEXARCH-06`

It applies the corrected 4-layer model:

1. core rule tests
2. collector / facts tests
3. family-orchestrator tests
4. family integration tests

Use with:

- `.plans/todo/checks/2026-03-25-rust-layered-test-architecture.md`
- `.plans/todo/check_review/test_hardening/31-hexarch-layered-test-map.md`
- `.plans/todo/check_review/test_hardening/16-hexarch-execution-plan.md`

## Goal

Stop treating structural discovery and ownership-boundary tests as if they were rule-unit tests.

For `01..06`, the main split is:

- raw structural discovery belongs to `facts.rs` collector tests
- cross-rule ownership and boundary behavior belongs to family-orchestrator tests
- broad golden attacks stay integration tests
- rule semantics over typed facts stay core rule tests

## Target file shape

Within the current monolith:

```text
apps/guardrail3/crates/app/rs/checks/rs/hexarch/
  tests/
    mod.rs
    testkit/
      mod.rs
      structural.rs
    collectors/
      mod.rs
      structural_roots.rs
      structural_children.rs
      leaf_classification.rs
    orchestrator/
      mod.rs
      structural_ownership.rs
    rs_hexarch_01_crates_exists_tests/
      mod.rs
      core.rs
    rs_hexarch_02_exact_contents_tests/
      mod.rs
      core.rs
    rs_hexarch_03_inbound_outbound_tests/
      mod.rs
      core.rs
    rs_hexarch_04_loose_files_tests/
      mod.rs
      core.rs
    rs_hexarch_05_container_not_empty_tests/
      mod.rs
      core.rs
    rs_hexarch_06_leaf_valid_tests/
      mod.rs
      core.rs
  tests/
    facts.rs
    orchestrator.rs
    integration.rs
    facts/
      structural_roots.rs
      structural_children.rs
      leaf_classification.rs
    orchestrator/
      structural_ownership.rs
    integration/
      structural_roots.rs
      structural_children.rs
      leaf_validity.rs
```

Notes:

- `core.rs` is the only place that keeps direct rule-semantics tests for `01..06`
- existing `golden.rs`, `broad_attacks.rs`, `replacement_edges.rs`, `ownership.rs`, `ownership_boundaries.rs`, `discovery_scope.rs`, `child_symlinks.rs`, `symlink_edges.rs`, and similar files should be reclassified, not blindly retained in place
- external harnesses are:
  - `tests/facts.rs`
  - `tests/orchestrator.rs`
  - `tests/integration.rs`

## Shared migration rules for 01-06

### Core rule layer

Keep only tests that can be expressed over typed structural facts.

Allowed examples:

- `01`
  - app root with `has_crates_dir = true` passes
  - app root with `has_crates_dir = false` fails
- `02`
  - exact required children pass
  - missing required child fails
  - `macros/` optional child passes
- `03`
  - both `inbound/` and `outbound/` present passes
  - either missing fails
- `04`
  - loose non-`.gitkeep` file fails
  - real `.gitkeep` exemption passes
  - symlinked `.gitkeep` does not exempt
- `05`
  - empty container fails
  - real `.gitkeep` placeholder passes
  - symlink placeholder does not count
- `06`
  - valid package leaf passes
  - nested `crates/` leaf passes
  - invalid leaf fails

Forbidden in core:

- project walking
- real fixture loading
- path ownership resolution across sibling rules
- symlink filesystem behavior unless already represented in typed facts

### Collector / facts layer

This layer owns raw structural snapshot correctness.

Collector cases for `01..06`:

- app-root discovery under `apps/*`
- nested inner-hex roots discovered but not promoted to app roots
- real child enumeration under owned `crates/`
- loose-file snapshots
- child symlink identity preservation
- raw `.gitkeep` presence
- raw empty-child snapshots
- leaf-kind classification:
  - package
  - nested-hex
  - placeholder
  - invalid

Forbidden in collectors:

- asserting final rule IDs
- asserting sibling-rule ownership like `02` vs `04`
- asserting final family hit sets

### Family-orchestrator layer

This layer owns cross-rule structural splits for `01..06`.

Required orchestrator cases:

- `01` does not over-own nested inner-hex roots
- `02` owns missing required top-level dirs, not `03`
- `03` owns missing `inbound/` / `outbound/` only when parent exists
- `04` owns loose-file hits, not `02` or `05`
- `05` owns empty-container hits, not `04`
- `06` owns invalid leaf shape, not ancestor structural rules
- mixed attacks produce exact rule hit and non-hit sets across `01..06`

These tests may use:

- small tempdir-backed trees
- real `ProjectTree`
- family entrypoint through existing `test_support`

But they are not broad golden attacks yet.

### Integration layer

This layer owns the real golden structural attack matrix.

Required integration cases:

- golden fixture pass
- remove required roots everywhere they are owned
- broad missing-child attacks
- broad unexpected-child/loose-file attacks
- broad empty-container attacks
- broad invalid-leaf attacks
- nested-root parity where applicable
- exact owned hit and non-hit sets

## Rule-by-rule migration

### RS-HEXARCH-01

Keep in rule core:

- typed app-root pass/fail
- exact severity and path attribution

Move to collectors:

- current `discovery_scope` logic that proves which roots are recognized as app roots

Move to orchestrator:

- nested-inner-root non-ownership
- non-app-root ownership boundaries

Keep/move to integration:

- `golden`
- broad root removal
- replacement-edge attacks across all owned app roots

### RS-HEXARCH-02

Keep in rule core:

- exact required set pass/fail
- `macros/` optional allowance

Move to collectors:

- top-level child enumeration
- child symlink identity
- raw top-level loose-file discovery

Move to orchestrator:

- ownership split from `04`
- ownership split from `03` when directional parents are absent

Keep/move to integration:

- broad missing required dir attacks
- broad unexpected child/file attacks
- nested parity

### RS-HEXARCH-03

Keep in rule core:

- directional child presence semantics

Move to collectors:

- parent-exists directional container discovery
- directional child symlink identity

Move to orchestrator:

- parent absence belongs with `02`, not `03`
- missing child under present parent belongs with `03`

Keep/move to integration:

- broad missing `inbound/` / `outbound/` attacks
- broad unexpected directional sibling attacks

### RS-HEXARCH-04

Keep in rule core:

- loose non-`.gitkeep` fails
- real `.gitkeep` exempt
- symlinked `.gitkeep` not exempt

Move to collectors:

- raw loose-file snapshots
- raw `.gitkeep` presence snapshots

Move to orchestrator:

- ownership split from `02`
- ownership split from `05`

Keep/move to integration:

- broad loose-file attacks across structural/container dirs
- near-miss controls outside owned dirs

### RS-HEXARCH-05

Keep in rule core:

- empty container fails
- real `.gitkeep` placeholder passes
- symlink placeholder does not count

Move to collectors:

- raw child-count / placeholder snapshot fields

Move to orchestrator:

- mixed placeholder/loose-file attacks split with `04`

Keep/move to integration:

- broad empty-container attacks
- `.gitkeep` boundary controls

### RS-HEXARCH-06

Keep in rule core:

- valid package leaf
- valid nested-hex leaf
- valid placeholder leaf if policy still allows it
- invalid leaf fails

Move to collectors:

- leaf-kind classification
- ignored-dir and non-leaf boundaries
- raw symlink/permission/manifest-kind capture

Move to orchestrator:

- invalid leaf ownership vs ancestor structural rules

Keep/move to integration:

- broad invalid-leaf attacks
- valid alternative controls
- nested-hex allowance controls

## Existing file reclassification

These current files are likely collectors, not rule core:

- `rs_hexarch_01_crates_exists_tests/discovery_scope.rs`
- `rs_hexarch_02_exact_contents_tests/child_symlinks.rs`
- `rs_hexarch_06_leaf_valid_tests/symlink_edges.rs`
- `rs_hexarch_06_leaf_valid_tests/permission_edges.rs`
- `rs_hexarch_06_leaf_valid_tests/ignored_dirs.rs`

These current files are likely orchestrator tests, not collectors:

- `rs_hexarch_03_inbound_outbound_tests/ownership_boundaries.rs`
- `rs_hexarch_05_container_not_empty_tests/ownership.rs`
- `rs_hexarch_05_container_not_empty_tests/non_owned_boundaries.rs`
- `rs_hexarch_06_leaf_valid_tests/ownership.rs`

These current files are likely integration tests:

- every `golden.rs`
- every `broad_attacks.rs`
- `compound_attacks.rs`
- `replacement_edges.rs`
- nested-root parity attacks that mutate the real golden tree

## Ordered execution

1. Create the new harness skeleton:
- `src/tests/{mod.rs,testkit/,collectors/,orchestrator/}`
- `tests/{facts.rs,orchestrator.rs,integration.rs}`

2. Migrate `RS-HEXARCH-01`
- keep only typed pass/fail in `core.rs`
- split `discovery_scope` into collector vs orchestrator coverage

3. Migrate `RS-HEXARCH-02`
- move child enumeration and child-symlink coverage to collectors
- move `02` vs `03/04` ownership splits to orchestrator

4. Migrate `RS-HEXARCH-03`
- separate directional collector coverage from ownership coverage

5. Migrate `RS-HEXARCH-04`
- keep `.gitkeep` semantics in core
- move raw child snapshots to collectors

6. Migrate `RS-HEXARCH-05`
- keep placeholder semantics in core
- move raw child snapshots to collectors

7. Migrate `RS-HEXARCH-06`
- keep leaf acceptance/rejection semantics in core
- move raw leaf classification to collectors

8. Build one shared structural orchestrator suite
- exact mixed-rule ownership across `01..06`

9. Re-home broad golden tests under integration
- assert exact owned hit and non-hit sets

10. Update the family docs
- mark old files reclassified
- record any new ownership bugs found

## Done means

This slice is done only when:

- `01..06` each have a minimal `core.rs`
- collector tests assert raw structure snapshots, not final rule IDs
- orchestrator tests assert cross-rule ownership and non-hit sets
- golden attack files live under integration harnesses
- no structural discovery test is still being counted as rule-unit coverage
