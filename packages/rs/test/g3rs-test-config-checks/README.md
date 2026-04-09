# g3rs-test-config-checks

Runs the `test` family config checks.

Current rules:

- `RS-TEST-09` nextest timeout config
- `RS-TEST-11` `cargo-mutants` installed when mutation checks are active
- `RS-TEST-12` `.cargo/mutants.toml` exists when mutation checks are active
- `RS-TEST-13` `[profile.mutants]` exists when mutation checks are active
- `RS-TEST-14` active hooks run `cargo mutants`
- `RS-TEST-15` mutation config is sane
