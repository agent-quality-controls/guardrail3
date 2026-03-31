# RS-DENY

Status: current, implemented, self-hosted.

Implementation root:

- `apps/guardrail3/crates/app/rs/families/deny/`

Current source of truth:

- this file for family planning/status
- `apps/guardrail3/crates/app/rs/families/deny/README.md` for family-local behavior

Current state:

- self-hosted with `crates/runtime`, `crates/assertions`, and `test_support`
- owns cargo-deny config coverage and deny-policy semantics inside legal workspaces
- malformed `guardrail3.toml` now fails closed for deny profile selection instead of degrading to service defaults
- by-file deny docs remain research on tool behavior, not the family contract

Scope model:

- workspace-local family
- should compute legal local deny facts over routed legal workspaces and routed
  deny-relevant files rather than rediscovering policy roots inside the family
- illegal placement belongs to `arch`, not to deny

Agent handoff focus:

- audit production path first:
  - `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
  - `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/facts.rs`
  - `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`
- prove allowed-location, shadowing, and uncovered-root behavior are computed
  from legal workspaces plus all deny-relevant files after the whole-project
  walker change
- add subtree tests for sibling-root non-bleed and malformed-input fail-closed

Known current risk:

- no confirmed production routing bug yet, but this family has several mixed
  config-placement surfaces and not enough subtree proof coverage
- older deny tests still assert placement/shadowing ownership that should now
  belong to `arch`

Done means:

- subtree tests prove sibling deny roots do not leak into nested-path runs
- misplaced deny configs outside legal workspaces are reported by `arch`
- malformed routed manifests and profile-map inputs still fail closed
- production facts stay route-bounded
- deny test helpers do not rebuild fake routed surfaces when the mapper returns
  no legal workspaces
- deny routed tests use legal workspace fixtures only
- pure deny rule semantics are covered by direct typed-input sidecar tests

Historical/supplemental references:

- `.plans/todo/checks/rs/deny.md`
- `.plans/by_file/rs/deny-toml.md`
- `.plans/by_file/tools/cargo-deny.md`

Next planning focus:

- close the remaining deny hardening-matrix gaps, especially broader mixed-root/profile attacks and end-to-end parity evidence
- finish the old-ledger cleanup if stale implementation pointers remain in the secondary docs
- rewrite stale placement/shadowing tests into either:
  - legal workspace-local deny tests
  - `arch` legality tests
  - direct typed-input deny rule tests
