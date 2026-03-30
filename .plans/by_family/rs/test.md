# RS-TEST

Status: current, implemented, heavily hardened, self-hosted.

Implementation root:

- `apps/guardrail3/crates/app/rs/families/test/`

Current source of truth:

- this file for family planning/status
- `apps/guardrail3/crates/app/rs/families/test/README.md` for family-local behavior and accepted rule contract

Current state:

- self-hosted with `crates/runtime`, `crates/assertions`, and `test_support`
- the family README is the strongest live contract for rule semantics and family shape
- old test-hardening briefs remain execution history, not primary authority

Historical/supplemental references:

- `.plans/todo/checks/rs/test.md`
- `.plans/todo/check_review/test_hardening/00-*`, `26-*`, `34-*`
- `.plans/todo/rs-test-compliance-handoffs/*`
- `.plans/todo/validator/2026-03-26-rs-test-self-hosting-gotchas.md`

Next planning focus:

- keep the README and live implementation aligned; this family already acts as the hardening baseline for the other Rust families
- avoid reintroducing flat test sidecars or local family-owned routing logic
