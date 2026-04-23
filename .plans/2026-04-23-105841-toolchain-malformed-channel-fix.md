Goal
- Stop malformed `rust-toolchain.toml` channel strings from being accepted as valid stable/beta/nightly channels.

Approach
- Add rule regressions for malformed channel heads such as `stable-foo`, `nightlyish`, and a target-suffix form with channel only in a later segment.
- Tighten `classify_channel` in `rs_toolchain_config_01_channel_and_components/rule.rs` so only exact channel heads count.
- Re-run the toolchain config checks tests and `g3rs validate` on the package.

Key decisions
- Fix the root classifier instead of adding special-case guards in tests or downstream rules.
- Treat channel names in later segments as unsupported; the channel semantic belongs at the head of the toolchain string.

Files to modify
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_01_channel_and_components/rule.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_01_channel_and_components/rule_tests/malformed.rs`
