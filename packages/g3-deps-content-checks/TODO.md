# TODO

- Keep `RS-DEPS-01..04`, `RS-DEPS-09..11` in the app family.
- Package boundary is the current legacy workspace `guardrail3.toml` shape,
  not `guardrail3-rs.toml`.
- Keep structural malformed-input ownership in the app family; package input
  site collection must not duplicate `RS-DEPS-11`.
- Exactness gap to resolve before app parity claim: non-workspace local path
  dependencies without an explicit `package = "..."` name may still need
  additional parsed manifests if we want package-level identity to match the
  old runtime exactly.
