# g3rs-garde-ast-checks TODO

- The package currently assumes the app is still the sole owner of malformed-input reporting through `RS-GARDE-10`. Unreadable or unparsable source files are skipped here rather than producing package-local findings.
- `RS-GARDE-AST-04` depends on legacy `guardrail3.toml` escape hatches. If the garde family migrates that policy surface later, the package contract should switch to the new parsed policy file deliberately.
- Garde applicability detection is still app-side. The package executes only on roots the app already decided are garde-governed.
