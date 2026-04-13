# g3rs-clippy-config-checks

Extracted `clippy.toml`, `guardrail3.toml`, and `.cargo/config*` config checks for guardrail3.

Current scope:

- `RS-CLIPPY-CONFIG-01`: `max-struct-bools`
- `RS-CLIPPY-CONFIG-02`: `max-fn-params-bools`
- `RS-CLIPPY-CONFIG-03`: `too-many-lines-threshold`
- `RS-CLIPPY-CONFIG-04`: `too-many-arguments-threshold`
- `RS-CLIPPY-CONFIG-05`: `excessive-nesting-threshold`
- `RS-CLIPPY-CONFIG-06`: test relaxation exactness
- `RS-CLIPPY-CONFIG-07`: `cognitive-complexity-threshold`
- `RS-CLIPPY-CONFIG-08`: `type-complexity-threshold`
- `RS-CLIPPY-CONFIG-09`: missing method bans
- `RS-CLIPPY-CONFIG-10`: missing type bans
- `RS-CLIPPY-CONFIG-11`: extra method bans
- `RS-CLIPPY-CONFIG-12`: extra type bans
- `RS-CLIPPY-CONFIG-13`: ban reason quality
- `RS-CLIPPY-CONFIG-14`: library global-state bans
- `RS-CLIPPY-CONFIG-15`: `avoid-breaking-exported-api`
- `RS-CLIPPY-CONFIG-16`: duplicate bans
- `RS-CLIPPY-CONFIG-17`: suspicious managed-key typos
- `RS-CLIPPY-CONFIG-18`: macro bans
- `RS-CLIPPY-CONFIG-19`: `guardrail3.toml` policy-context parseability
- `RS-CLIPPY-CONFIG-20`: forbidden `CLIPPY_CONF_DIR` override surfaces
- `RS-CLIPPY-CONFIG-21`: `clippy.toml` parseability

This package intentionally keeps the package model boundary:

- one pointed workspace at a time
- root-local policy context
- root-local cargo override surfaces
- no source lane

Old app-only repo-wide routed descendants do not carry over as a separate package lane.
