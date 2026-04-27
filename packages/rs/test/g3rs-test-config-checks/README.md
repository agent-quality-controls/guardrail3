# g3rs-test-config-checks

Runs the `test` family config checks.

Current rules:

- `g3rs-test/nextest-timeouts` nextest timeout config
- `g3rs-test/cargo-mutants-installed` `cargo-mutants` installed when mutation checks are active
- `g3rs-test/mutants-toml-exists` `.cargo/mutants.toml` exists when mutation checks are active
- `g3rs-test/mutants-profile-present` `[profile.mutants]` exists when mutation checks are active
- `g3rs-test/mutation-hook-present` active hooks run `cargo mutants`
- `g3rs-test/mutants-config-sane` mutation config is sane
