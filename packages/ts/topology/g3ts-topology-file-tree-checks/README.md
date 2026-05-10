# g3ts-topology-file-tree-checks

File-tree topology checks for the TS adopted-unit family. Currently exposes the
`g3ts-topology/no-nested-guardrail3-ts-toml` rule, which fires when a descendant
`guardrail3-ts.toml` is found inside an outer adopted unit's tree.
