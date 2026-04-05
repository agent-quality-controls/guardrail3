# g3-clippy-content-checks TODO

## Deferred Boundary Work

### `RS-CLIPPY-24` remains app-side for now

- `RS-CLIPPY-24` checks forbidden `CLIPPY_CONF_DIR` override surfaces in
  `.cargo/config.toml` / `.cargo/config`.
- Keep it in the app until the package boundary is intentionally expanded to
  include that parsed file as a normal required input.

### `RS-CLIPPY-25` remains app-side

- `RS-CLIPPY-25` owns typed parse/schema failure for the active `clippy.toml`.
- This package only accepts valid parsed `ClippyToml`.

### Policy-sensitive clippy rules remain app-side

- `RS-CLIPPY-13`
- `RS-CLIPPY-14`
- `RS-CLIPPY-16`

Reason:
- they still depend on app-owned policy/profile context rather than only the
  parsed `clippy.toml`

## Current Extracted Slice

- `RS-CLIPPY-02`
- `RS-CLIPPY-03`
- `RS-CLIPPY-09`
- `RS-CLIPPY-10`
- `RS-CLIPPY-11`
- `RS-CLIPPY-17`
- `RS-CLIPPY-21`
- `RS-CLIPPY-22`

## Follow-up Hardening

- If more bridge coverage is needed later, add app-family smoke for a migrated
  multi-result rule shape such as `RS-CLIPPY-17`, not just threshold-style
  single-result rules.
