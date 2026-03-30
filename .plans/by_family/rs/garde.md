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

Historical/supplemental references:

- `.plans/todo/checks/rs/garde.md`
- family stabilization handoff docs under `.plans/todo/family-stabilization-handoffs/`
- old clippy/deny hardening notes only as migration history

Next planning focus:

- keep root-policy inheritance and clippy-coverage dependency explicit
- generalize validate-call enforcement beyond `GuardrailConfig` only after the false-positive story is proven
- add stronger family-local docs if source-rule boundaries expand
