# Plan Hygiene And Migration Closure

## lingering legacy dependencies

- `rs/code` still depends on legacy `ast_helpers` for several suppression-related parsing paths.
- `rs/test` still depends on legacy `ast_helpers` for some parsing logic.
- old `app/rs/validate/*` wiring still exists and still influences behavior.
- the migration-closure criterion itself is still not written down clearly.
- Finish migrating those helpers into family-local code or explicitly retire legacy validator helpers.

## stale statuses and stale `Current code` pointers

- `fmt.md` and `toolchain.md` still mark implemented rules as `Planned`.
- `cargo.md` and `deps.md` still mark implemented rules as `Planned`.
- `garde.md` still marks implemented rules as planned.
- `test.md` is materially stale about implementation state.
- `deny.md` still marks the whole family as `Todo`.

- Several active Rust family plans still point `Current code` at legacy validator files instead of the live `app/rs/checks/rs/*` families:
  - `cargo.md`
  - `deps.md`
  - `fmt.md`
  - `toolchain.md`
  - `code.md`
  - `release.md`
  - `hexarch.md`

## stale plan prose

- `code.md` still reads like a future-tense migration document even though `rs/code` is implemented.
- `garde.md` still overstates missing extractor-ban work that is already implemented.
- whole-type `#[garde(skip)]` ownership is still missing as an explicit bypass-surface item.
- `garde.md` still carries live prose requirements with no explicit rule IDs:
  - wrapper-based boundary enforcement
  - field-level garde quality checks
  - `#[garde(dive)]`
  - context-driven validation
- `RS-GARDE-06` rule text overclaims wrapper enforcement; current implementation only validates clippy method-ban completeness.
- `RS-GARDE-09` plan text should explicitly mention both `query_as!` and `query_as_unchecked!`.
- `code.md` still contains stale cross-checker action items already implemented (`std::process::abort`, `std::any::Any`, `unreachable_pub`, `lazy_static`).

## archive/relabel candidates

- `deploy/ts.md`, `hooks/ts.md`, and `hooks_deploy_audit.md` should be archived or relabeled legacy-only.
