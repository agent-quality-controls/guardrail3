# RS-DEPS

Status: current, implemented, self-hosted.

Implementation root:

- `apps/guardrail3/crates/app/rs/families/deps/`

Current source of truth:

- this file for family planning/status
- `apps/guardrail3/crates/app/rs/families/deps/README.md` for family-local behavior

Current state:

- mixed-scope family for Rust dependency policy and required external tool presence
- self-hosted with `crates/runtime`, `crates/assertions`, and `test_support`
- distinct from `deny`, `hexarch`, and `release`; it should keep owning allowlists/tool presence rather than architecture or release semantics

Historical/supplemental references:

- `.plans/todo/checks/rs/deps.md`
- family stabilization handoff docs under `.plans/todo/family-stabilization-handoffs/`

Next planning focus:

- keep the mixed-scope contract explicit: tool checks, lockfile checks, and per-crate allowlist checks are not the same scope
- reconcile any stale repo-root-only wording left in older docs
