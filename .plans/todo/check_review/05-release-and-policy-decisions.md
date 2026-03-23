# Release And Policy Decisions

## release-family semantic gaps

- `RS-RELEASE-05..07` and `RS-BIN-01..02` still overclaim “actual execution step” semantics while relying on broad string matching over parsed workflow data.
- Tighten them to command-context / concrete wiring detection or narrow the plan wording.
- `rs/release` still drifts from the architecture plan’s minimal-input contract:
  - some rules still consume family-sized aggregates
  - support helpers still contain `ProjectTree` / filesystem-dependent extraction work that should live in orchestrator/facts code

## release-family concrete bugs

- `readme = false` is still not honored correctly by `rs/release` README checks.
- `RS-PUB-10` / `11` still miss `workspace = true` inherited local path dependency edges.

## scope decisions

- `fmt` / `toolchain` plans still read like workspace-aware families, but current implementations are effectively repo-root-only.
- `cargo` is still repo-root-only, not Rust-root-aware.
- Decide whether to:
  - implement actual Rust-root/workspace-aware discovery
  - or narrow those plans to explicit repo-root-only semantics
- keep the specific under-implementation notes visible while making that decision:
  - `RS-TOOLCHAIN-03` profile-context gap
  - `RS-FMT-04` repo-root toolchain shallowness
  - `RS-FMT-06` repo-root Cargo metadata shallowness

## deny policy decision

- `RS-DENY-19` needs an explicit policy decision:
  - allow extra registries if crates.io is present
  - or forbid any non-crates.io registry and tighten the plan to match the current rule

## release config semantic baseline

- Turn the semantic `release-plz.toml` / `cliff.toml` baseline into explicit rules if we actually want that contract enforced.
