## Goal

Repair `g3rs-test/real-proof-site` so the rule consumes ingestion-owned local proof-helper facts instead of rebuilding proof-helper state and guessing from proof-ish names or arbitrary `assertions` path segments.

## Approach

- Add red tests in the `g3rs-test/real-proof-site` rule suite proving the current rule is wrong when:
  - a same-file local proof helper ends in a real assertion path but its name does not look proof-like
  - a local module or imported path contains `assertions` but is not the owned assertions surface
- Move local proof-helper binding into `rs/test` ingestion and types so analyzed source files carry the exact local helper set needed by the rule.
- Refactor `rs_test_07_real_proof_site/rule.rs` to:
  - consume the prebound helper set
  - stop classifying local proof paths from loose `assertions` segment heuristics
  - keep the rule narrow to proof-site validation instead of rebuilding local proof topology

## Key decisions

- Fix this at the ingestion boundary, not by adding more local heuristics inside the rule.
  - Rejected: widening `looks_like_proof_helper_name(...)` or adding more special-case `assertions` matching.
- Keep the fix inside the existing `rs/test` source lane.
  - Rejected: inventing a new family lane when the analyzed source file already owns proof-bearing function facts and is the right place to extend.

## Files to modify

- `packages/rs/test/g3rs-test-types/src/types.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/source_analysis.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule_tests/cases.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/assertions/src/rs_test_07_real_proof_site/rule.rs` if assertion helpers need tightening
- `.worklogs/<timestamp>-rs-test-real-proof-site-boundary-repair.md`
