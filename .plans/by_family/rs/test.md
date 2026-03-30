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
- active `RS-TEST` violations now escalate as errors; non-error output is reserved for passing inventory and explicit escape-hatch enumeration
- generated/cache Rust under `target/**` is outside owned-root analysis
- routed subtree runs now keep root-level activation bound to scoped family surface instead of sibling test files
- routed subtree runs now skip whole roots when the scoped target does not intersect live `RS-TEST` surface, including generated `target/**` paths
- fixture Rust under `tests/fixtures/**`, `*_tests/fixtures/**`, `assertions/src/fixtures/**`, and `test_support/src/fixtures/**` is outside owned analysis
- validation-root mutation hooks now propagate to workspace members without waking unrelated standalone roots
- test-only attributes hidden behind `cfg_attr(...)` no longer evade test, ignore, or `should_panic` checks
- `#[cfg(all(test, ...))]` module declarations still count as owned test modules for inline-body and sidecar-shape rules
- weak payload matching now includes `assert_matches!(...)` and rest-pattern elision like `Some(..)` or `Foo { .. }`
- external harnesses can no longer hide owned proof behind associated-function wrappers or bare function-pointer aliases to local helpers
- `test_support` can no longer hide semantic finding selectors behind local `type Alias = CheckResult` indirection
- external harnesses now fail on any `#[path = ...]` source inclusion, not just direct runtime `src/` includes
- sidecars can no longer hide semantic result ownership behind transitive local helpers or method-based `CheckResult` selectors
- external harnesses can no longer hide owned-assertions proof behind transitive local wrapper helpers
- owned assertions calls that merely pass a rule id string no longer trip a fake sidecar semantic-proof violation
- old test-hardening briefs remain execution history, not primary authority

Scope model:

- global family over all non-excluded owned Rust test surfaces
- per-crate activation still exists, but the route must stay repo-global rather
  than narrowing to one routed workspace

Agent handoff focus:

- audit production path first:
  - `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
  - `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/discover.rs`
  - `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs`
- migrate the family from routed-root activation to repo-global owned test
  surfaces with per-crate activation inside the family
- prove global runs and nested-path runs still keep illegal/out-of-workspace
  test surfaces visible
- keep route construction out of assertions and test helpers

Known current risk:

- the family runtime is now materially harder, but discovery/activation is still
  the highest-churn surface; keep attacking routed scope, hook parsing, and
  helper-indirection rather than assuming the AST hardening is exhaustive

Done means:

- repo-global route tests prove the family sees all owned test surfaces outside
  exclusions
- per-crate activation still binds mutation/config checks to the correct owned
  crate without making sibling test files disappear
- illegal or out-of-workspace test surfaces do not escape `RS-TEST`
- no test helper grows production routing responsibility
- local helper indirection cannot hide sidecar/external-harness/test_support
  semantic ownership violations

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
