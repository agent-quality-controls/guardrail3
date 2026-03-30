# RS-GARDE

Status: current, implemented, self-hosted.

Implementation root:

- `apps/guardrail3/crates/app/rs/families/garde/`

Current source of truth:

- this file for family planning/status
- `apps/guardrail3/crates/app/rs/families/garde/README.md` for family-local behavior

Current state:

- multi-root family
- self-hosted with `crates/runtime`, `crates/assertions`, and `test_support`
- depends on correct `RS-CLIPPY` root/config coverage behavior
- owns garde-specific boundary enforcement, not generic clippy baseline hardening
- owns a narrow runtime-usage rule for `GuardrailConfig` parse sites that skip `.validate()`
- subtree/scoped-file enforcement is now proven in production and test paths
- alias-aware detection now covers renamed `serde`, `garde`, and `sqlx` imports for
  `RS-GARDE-05/07/08/09`
- `RS-GARDE-10` now has direct regressions for unreadable source and malformed
  `guardrail3.toml` policy input
- `RS-GARDE-01` now stays quiet for internal helper crates that do not show real
  garde adoption markers
- family crate tests currently pass:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-garde --lib`
- direct family-root garde execution is now clean:
  - `0 errors, 0 warnings, 0 info` against `apps/guardrail3/crates/app/rs/families/garde`

Scope model:

- workspace-local family
- derive/source scans should stay inside the owning legal workspace while
  preserving workspace-level garde policy context

Agent handoff focus:

- audit production path first:
  - `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
  - `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/discover.rs`
  - `apps/guardrail3/crates/app/rs/family_mapper/src/scoped_files.rs`
- prove derive-target discovery, manual-deserialize checks, and field-level
  rules stay inside the owning workspace surface
- prove root-policy checks still bind to legal workspaces from the whole tree

Known current risk:

- root/file-scope drift is no longer the main risk; that path now has direct
  regressions
- the app-level `guardrail3` runner can still be blocked by unrelated family
  compile failures, so direct family-root execution may require a scratch runner

Done means:

- subtree tests prove source-file checks stay inside the active scoped file set
- root-policy findings still reflect routed roots only
- no direct tree-wide source scan bypasses the route

Historical/supplemental references:

- `.plans/todo/checks/rs/garde.md`
- family stabilization handoff docs under `.plans/todo/family-stabilization-handoffs/`
- old clippy/deny hardening notes only as migration history

Next planning focus:

- keep root-policy inheritance and clippy-coverage dependency explicit
- generalize validate-call enforcement beyond `GuardrailConfig` only after the false-positive story is proven
- keep the `RS-GARDE-01` applicability contract narrow:
  roots with no garde dependency and no garde adoption markers stay silent
