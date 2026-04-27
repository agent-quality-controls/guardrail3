# g3rs-clippy-config-checks

Extracted `clippy.toml`, `guardrail3-rs.toml`, and `.cargo/config*` config checks for guardrail3.

Current scope:

- `g3rs-clippy/max-struct-bools`: `max-struct-bools`
- `g3rs-clippy/max-fn-params-bools`: `max-fn-params-bools`
- `g3rs-clippy/too-many-lines-threshold`: `too-many-lines-threshold`
- `g3rs-clippy/too-many-arguments-threshold`: `too-many-arguments-threshold`
- `g3rs-clippy/excessive-nesting-threshold`: `excessive-nesting-threshold`
- `g3rs-clippy/test-relaxations`: test relaxation exactness
- `g3rs-clippy/cognitive-complexity-threshold`: `cognitive-complexity-threshold`
- `g3rs-clippy/type-complexity-threshold`: `type-complexity-threshold`
- `g3rs-clippy/missing-method-ban`: missing method bans
- `g3rs-clippy/missing-type-ban`: missing type bans
- `g3rs-clippy/extra-method-ban`: extra method bans
- `g3rs-clippy/extra-type-ban`: extra type bans
- `g3rs-clippy/ban-reason-quality`: ban reason quality
- `g3rs-clippy/library-global-state`: library global-state bans
- `g3rs-clippy/avoid-breaking-exported-api`: `avoid-breaking-exported-api`
- `g3rs-clippy/duplicate-bans`: duplicate bans
- `g3rs-clippy/unknown-keys`: suspicious managed-key typos
- `g3rs-clippy/macro-bans`: macro bans
- `g3rs-clippy/policy-context-parseable`: `guardrail3-rs.toml` rust-policy parseability
- `g3rs-clippy/forbid-clippy-conf-dir-override`: forbidden `CLIPPY_CONF_DIR` override surfaces
- `g3rs-clippy/config-parseable`: `clippy.toml` parseability

This package intentionally keeps the package model boundary:

- one pointed workspace at a time
- root-local Rust policy context
- root-local cargo override surfaces
- no source lane

Old app-only repo-wide routed descendants do not carry over as a separate package lane.
