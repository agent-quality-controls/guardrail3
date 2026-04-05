# g3-clippy-content-checks

Extracted typed `clippy.toml` content checks for guardrail3.

This package is intentionally narrower than the in-app `clippy` family:

- it validates already parsed `ClippyToml` content only
- it does not discover or route policy roots
- it does not report malformed parse/schema inputs
- it does not inspect `.cargo/config.toml` override surfaces
- it does not resolve guardrail profile or garde policy context

Current scope:

- `RS-CLIPPY-02`: `max-struct-bools`
- `RS-CLIPPY-03`: `max-fn-params-bools`
- `RS-CLIPPY-09`: `too-many-lines-threshold`
- `RS-CLIPPY-10`: `too-many-arguments-threshold`
- `RS-CLIPPY-11`: `excessive-nesting-threshold`
- `RS-CLIPPY-17`: test relaxation exactness
- `RS-CLIPPY-21`: `cognitive-complexity-threshold`
- `RS-CLIPPY-22`: `type-complexity-threshold`

The app remains responsible for:

- coverage, placement, shadowing, and routing
- `RS-CLIPPY-25` parse/schema gating
- policy-context failures from `guardrail3.toml`
- `RS-CLIPPY-24` cargo-config override checks
- the remaining raw-section and policy-context-sensitive clippy rules
