# RS-RELEASE

Status: current, implemented, self-hosted, family README present.

Implementation root:

- `apps/guardrail3/crates/app/rs/families/release/`

Current source of truth:

- `apps/guardrail3/crates/app/rs/families/release/README.md` for family behavior/shape
- this file for family planning/status
- `.plans/todo/checks/rs/release.md` as the detailed rule ledger and migration/history reference

Current state:

- mixed-scope family for repo-level release artifacts, per-package publishability, and local release-edge checks
- current code uses a self-hosted split with root crate, `assertions/`, and `test_support/`
- family-local README now exists, so the old ledger can stay secondary the way the other Rust families do

Historical/supplemental references:

- `.plans/todo/checks/rs/release.md`
- release hardening docs under `.plans/todo/check_review/test_hardening/03-*` and `13-*`
- `.plans/by_file/rs/release-plz-toml.md` and `.plans/by_file/rs/cliff-toml.md` as upstream/file-behavior research

Next planning focus:

- keep the new family README, this plan file, and the detailed ledger aligned when rule inventory or file shape changes
- treat the old ledger as detailed history and rule-by-rule backstory rather than the primary family entry point
