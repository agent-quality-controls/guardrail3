# RS-RELEASE

Status: current, implemented, self-hosted, README still missing.

Implementation root:

- `apps/guardrail3/crates/app/rs/families/release/`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/rs/release.md` as the detailed current rule ledger until a family README exists

Current state:

- mixed-scope family for repo-level release artifacts, per-package publishability, and local release-edge checks
- current code uses a self-hosted split with root crate, `assertions/`, and `test_support/`
- this is the main Rust family still missing a family-local README, so the old ledger remains more important than it should be

Historical/supplemental references:

- `.plans/todo/checks/rs/release.md`
- release hardening docs under `.plans/todo/check_review/test_hardening/03-*` and `13-*`
- `.plans/by_file/rs/release-plz-toml.md` and `.plans/by_file/rs/cliff-toml.md` as upstream/file-behavior research

Next planning focus:

- add `apps/guardrail3/crates/app/rs/families/release/README.md`
- after that, demote the old rule ledger to historical detail the way the other families now work
