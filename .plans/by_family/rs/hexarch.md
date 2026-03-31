# RS-HEXARCH

Status: current, implemented, heavily hardened, self-hosted.

Implementation root:

- `apps/guardrail3/crates/app/rs/families/hexarch/`

Current source of truth:

- this file for family planning/status
- `apps/guardrail3/crates/app/rs/families/hexarch/README.md` for family-local behavior
- `apps/guardrail3/crates/app/rs/README.md` for shared placement/routing architecture

Current state:

- self-hosted with `crates/runtime`, `crates/assertions`, `crates/assertions_common`, and `test_support`
- owns app-internal hex structure and dependency-direction semantics
- workspace-boundary enforcement was recently tightened so nested workspaces under app roots are forbidden
- no longer owns generic workspace-membership exactness; that moved to `RS-ARCH`
- `assertions_common` is a real current implementation detail and should stay explicit in docs rather than hidden

Scope model:

- routed app roots plus routed scoped files
- subtree validation should narrow app-local source/dependency analysis to the
  active scoped file set inside the routed app

Agent handoff focus:

- audit production path first:
  - `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
  - `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/facts.rs`
  - `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/dependency_facts.rs`
  - `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/source_facts.rs`
- fix the current known bug: production collectors receive routed roots but do
  not currently honor routed `scoped_files`
- add subtree tests that prove sibling files inside the same app do not bleed
  into nested-path runs

Known current risk:

- confirmed production bug: scoped files are carried in the route, but the
  current collectors still scan the whole routed app

Done means:

- production collectors consume `route.scoped_files()` where required
- subtree tests prove file- and dependency-level narrowing inside one routed app
- repo-global misplaced-root behavior stays with `RS-ARCH`, not `RS-HEXARCH`

Historical/supplemental references:

- `.plans/todo/checks/rs/hexarch.md`
- `.plans/todo/hexarch-workspace-boundary-handoff.md`
- hexarch hardening docs under `.plans/todo/check_review/test_hardening/01-*`, `11-*`, `16-*`, `31-*`, `32-*`, `33-*`

Next planning focus:

- clean the stale old-path references in the older ledger
- keep app-local discovery strictly inside routed scope from shared placement/mapper inputs
