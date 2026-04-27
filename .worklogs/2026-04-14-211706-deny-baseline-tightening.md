# Summary
Tightened the active `deny` family baseline so it no longer encodes the stale `yanked = "warn"` downgrade or the stale `regex` wrapper carveout. The deny family now matches the stricter package policy and the clippy config-checks package is clean under `--family deny`.

# Decisions Made
- Fixed the family baseline rather than reverting the package. The stale defaults lived in `baseline.rs`, shared deny fixtures, and rule-local expectations.
- Tightened `g3rs-deny/wrappers` at the rule level too. Rejected the old behavior that downgraded added wrappers on managed bans with an empty wrapper baseline to inventory; a managed ban now has an exact wrapper contract, including the empty-set case.
- Kept the change narrow. Rejected touching unrelated deny defaults or the still-deferred `test` family findings on the clippy package.

# Key Files For Context
- `.plans/2026-04-14-211313-deny-baseline-tightening.md`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/baseline.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/support.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/rs_deny_config_27_wrappers.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/test_support.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/deny.toml`

# Next Steps
- `packages/rs/clippy/g3rs-clippy-config-checks` is now clean under `code`, `arch`, and `deny`; only the intentionally deferred `test` family still signals.
- If continuing package-by-package, pick the next family workspace and repeat the same validate -> decide -> fix loop.
