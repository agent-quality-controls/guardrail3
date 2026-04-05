# TODO

- Implement `RS-DEPS-05`, `RS-DEPS-06`, and `RS-DEPS-07` using:
  - crate `Cargo.toml`
  - workspace `Cargo.toml`
  - workspace `guardrail3-rs.toml`
- Implement `RS-DEPS-08` using crate `Cargo.toml` plus workspace
  `guardrail3-rs.toml`.
- Implement `RS-DEPS-12` as a pure direct-dependency-count rule.
- Keep `RS-DEPS-01..04`, `RS-DEPS-09..11` in the app family.
- Add direct package-local tests from the start; do not rely only on app-family
  tests.
- Before wiring the package, verify the app `deps` family is sourcing policy
  from `guardrail3-rs.toml` rather than the legacy `guardrail3.toml` shape.
- Exactness gap to resolve before app parity claim: non-workspace local path
  dependencies without an explicit `package = "..."` name may still need
  additional parsed manifests if we want package-level identity to match the
  old runtime exactly.
