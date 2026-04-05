# g3-clippy-content-checks TODO

## Deferred From Initial Package Boundary

### RS-CLIPPY-24 remains app-side for now

- `RS-CLIPPY-24` checks forbidden `CLIPPY_CONF_DIR` override surfaces in
  `.cargo/config.toml` / `.cargo/config`.
- The initial `g3-clippy-content-checks` package should not include Cargo
  config files in its input contract.
- Keep `RS-CLIPPY-24` in the app until the package boundary is intentionally
  expanded to include that file as a normal parsed input.

Follow-up:

- When revisiting the clippy package boundary, decide whether Cargo config
  should become a required package input.
- If yes, move `RS-CLIPPY-24` into the package at that time.

## Deferred Typed-Schema / Policy Work

- `RS-CLIPPY-25` stays in the app as the typed parse/schema gate. This package
  only accepts `ClippyToml`.
- Rules that currently depend on malformed raw section shapes stay in the app
  until those malformed cases are intentionally reclassified as structural app
  failures.
- Profile-sensitive and guardrail-policy-sensitive rules such as
  `RS-CLIPPY-13`, `RS-CLIPPY-14`, and `RS-CLIPPY-16` stay app-side for now.
- The initial extracted slice is the typed-safe `clippy.toml` subset:
  `RS-CLIPPY-02`, `03`, `09`, `10`, `11`, `17`, `21`, `22`.
