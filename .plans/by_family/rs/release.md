# RS-RELEASE

Status: current, implemented, self-hosted, family README present.

Implementation root:

- `apps/guardrail3/crates/app/rs/families/release/`

Current source of truth:

- `apps/guardrail3/crates/app/rs/families/release/README.md` for family behavior/shape
- this file for family planning/status
- `.plans/todo/checks/rs/release.md` as the detailed rule ledger and migration/history reference

Current state:

- workspace-local family for actual release units
- current code uses a self-hosted split with root crate, `assertions/`, and `test_support/`
- family-local README now exists, so the old ledger can stay secondary the way the other Rust families do

Scope model:

- workspace-local family
- release policy should bind to actual releaseable workspaces rather than a
  repo-global release unit
- any repo-level release files should be treated as shared inputs to local
  release rules, not as proof of a repo-global release contract

Agent handoff focus:

- audit production path first:
  - `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
  - `apps/guardrail3/crates/app/rs/families/release/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/families/release/src/facts/`
  - `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`
- prove the workspace-local contract after the whole-project walker change
- add subtree tests showing nested-path runs do not emit unrelated release
  findings from sibling workspaces

Known current risk:

- no confirmed production bug yet, but the family still carries legacy
  repo-global wording that should not survive the workspace-local transition

Done means:

- subtree tests prove publishability and release-edge checks exclude unrelated
  sibling workspaces
- docs stay clear that this family is workspace-local

Historical/supplemental references:

- `.plans/todo/checks/rs/release.md`
- release hardening docs under `.plans/todo/check_review/test_hardening/03-*` and `13-*`
- `.plans/by_file/rs/release-plz-toml.md` and `.plans/by_file/rs/cliff-toml.md` as upstream/file-behavior research

Next planning focus:

- keep the new family README, this plan file, and the detailed ledger aligned when rule inventory or file shape changes
- treat the old ledger as detailed history and rule-by-rule backstory rather than the primary family entry point
