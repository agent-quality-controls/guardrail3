# g3rs-garde-source-checks TODO

- `RS-GARDE-SOURCE-04` depends on legacy `guardrail3.toml` escape hatches. If the garde family migrates that policy surface later, the package contract should switch to the new parsed policy file deliberately.
- Garde applicability is derived from the package input:
  - direct `garde` dependency
  - or source adoption markers in governed files
