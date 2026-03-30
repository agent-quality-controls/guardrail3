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
- the family-local closure pass is complete; current follow-up is cross-family enforcement, not more `rs/test` implementation
- generated/cache Rust under `target/**` is outside owned-root analysis
- old test-hardening briefs remain execution history, not primary authority

Historical/supplemental references:

- `.plans/todo/checks/rs/test.md`
- `.plans/todo/check_review/test_hardening/00-*`, `26-*`, `34-*`
- `.plans/todo/rs-test-compliance-handoffs/*`
- `.plans/todo/validator/2026-03-26-rs-test-self-hosting-gotchas.md`

Next planning focus:

- keep the README and live implementation aligned; this family already acts as the hardening baseline for the other Rust families
- drive remaining semantic-ownership cleanup from the target families that still violate `RS-TEST`, starting with cargo's `crates/assertions_common/src/lib.rs`
- avoid reintroducing flat test sidecars or local family-owned routing logic
- keep attacking `RS-TEST-16/17/18` and discovery boundaries for new bypasses rather than assuming the closure pass was exhaustive
