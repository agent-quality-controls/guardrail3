# g3rs-clippy-config-checks

Extracted typed `clippy.toml` config checks for guardrail3.

This package is intentionally narrower than the in-app `clippy` family:

- it validates already parsed `ClippyToml` content only
- it does not discover or route policy roots
- it does not report malformed parse/schema inputs
- it does not inspect `.cargo/config.toml` override surfaces
- it does not resolve guardrail profile or garde policy context

Current scope:

- `RS-CLIPPY-CONFIG-01`: `max-struct-bools`
- `RS-CLIPPY-CONFIG-02`: `max-fn-params-bools`
- `RS-CLIPPY-CONFIG-03`: `too-many-lines-threshold`
- `RS-CLIPPY-CONFIG-04`: `too-many-arguments-threshold`
- `RS-CLIPPY-CONFIG-05`: `excessive-nesting-threshold`
- `RS-CLIPPY-CONFIG-06`: test relaxation exactness
- `RS-CLIPPY-CONFIG-07`: `cognitive-complexity-threshold`
- `RS-CLIPPY-CONFIG-08`: `type-complexity-threshold`

The app remains responsible for:

- coverage, placement, shadowing, and routing
- `RS-CLIPPY-25` parse/schema gating
- policy-context failures from `guardrail3.toml`
- `RS-CLIPPY-24` cargo-config override checks
- the remaining raw-section and policy-context-sensitive clippy rules
