# g3rs-test-config-checks

Runs the `test` family config checks.

Current rules:

- `RS-TEST-CONFIG-09` nextest timeout config
- `RS-TEST-CONFIG-11` `cargo-mutants` installed when mutation checks are active
- `RS-TEST-CONFIG-12` `.cargo/mutants.toml` exists when mutation checks are active
- `RS-TEST-CONFIG-13` `[profile.mutants]` exists when mutation checks are active
- `RS-TEST-CONFIG-14` active hooks run `cargo mutants`
- `RS-TEST-CONFIG-15` mutation config is sane
