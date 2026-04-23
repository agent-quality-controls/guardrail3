Summary
- Tightened toolchain channel classification so malformed strings like `stable-foo`, `nightlyish`, and later-segment `...-nightly` no longer pass as valid channels.
- Preserved accepted channel forms for exact stable/beta/nightly heads, dated beta/nightly heads, target-triple suffixes, and pinned stable versions.

Decisions made
- Fixed the root classifier instead of adding downstream guards.
- Added coverage for malformed channel heads and later-segment channel names.
- Added one positive regression for a stable channel with a target-triple suffix to keep the classifier from becoming too restrictive.

Key files for context
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_01_channel_and_components/rule.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_01_channel_and_components/rule_tests/malformed.rs`
- `.plans/2026-04-23-105841-toolchain-malformed-channel-fix.md`

Next steps
- None.
