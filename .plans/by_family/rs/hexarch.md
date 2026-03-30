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
- `assertions_common` is a real current implementation detail and should stay explicit in docs rather than hidden

Historical/supplemental references:

- `.plans/todo/checks/rs/hexarch.md`
- `.plans/todo/hexarch-workspace-boundary-handoff.md`
- hexarch hardening docs under `.plans/todo/check_review/test_hardening/01-*`, `11-*`, `16-*`, `31-*`, `32-*`, `33-*`

Next planning focus:

- clean the stale old-path references in the older ledger
- keep app-local discovery strictly inside routed scope from shared placement/mapper inputs
