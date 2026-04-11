Summary

Cleaned the remaining stale topology migration docs after the package hardening. The ledgers now match the actual extracted workspace-local topology subset and no longer claim that only `11/12/13/16` migrated.

Decisions made

- Renamed the topology table from "Planned rules" to "Rule ledger" because it mixes old app-global and new package-local status.
- Marked `RS-TOPOLOGY-11`, `RS-TOPOLOGY-13`, and `RS-TOPOLOGY-16` as implemented in the ledger.
- Expanded the extracted workspace-local subset to include `RS-TOPOLOGY-07` in the ledger and package TODO.
- Kept the rest of the old topology family described as repo-global policy, not new package work.

Key files for context

- `.plans/todo/checks/rs/topology.md`
- `.plans/by_family/rs/topology.md`
- `packages/rs/topology/g3rs-topology-file-tree-checks/README.md`
- `packages/rs/topology/g3rs-topology-ingestion/TODO.md`

Next steps

- Keep migrating only real package-layer work.
- Do not move old repo-global topology policy into workspace-local packages without an explicit new package-layer home.
