Goal

- Migrate the still-useful non-structural `hexarch` semantics into `apparch`.
- Prove the current `apparch` family misses sneaky behaviors that should be caught.
- Add the missing `apparch` rules at the package boundary, not in the CLI app.
- Leave `hexarch` deletion for a later step after semantic parity is settled.

Approach

- Extend `g3rs-apparch-types` so config/source inputs can express:
  - dependency section kind
  - internal patch/replace bypasses
  - external dependency usage for purity checks
  - Rust policy state from `guardrail3-rs.toml`
  - public free function and public inherent method facts for `types/*`
- Extend `g3rs-apparch-ingestion` to parse and normalize:
  - `guardrail3-rs.toml`
  - internal `[patch.*]` and `[replace]` path overrides
  - external dependencies from runtime/build/target tables
  - cycle candidates over non-dev workspace-internal edges
  - public free function and public inherent method counts/facts for `types/*`
- Add failing regressions first for hidden bad behavior:
  - root patch alias makes `logic/*` reach `io/outbound/*`
  - root replace makes `types/*` point at another apparch layer
  - same-layer cycle hidden through multiple crates
  - forbidden dev-only edge that should warn separately
  - `types/*` and `logic/*` smuggling impure external crates
  - `types/*` hiding public behavior in concrete inherent methods and free fns
  - invalid `guardrail3-rs.toml` not being mistaken for an empty allowlist or empty waiver set
- Implement the new rules in `g3rs-apparch-config-checks` and `g3rs-apparch-source-checks`.
- Run package tests and an adversarial review against this plan and the old `hexarch` rule semantics.

Rules to add

- `g3rs-apparch/patch-replace-bypass`
  - internal `[patch.*]` / `[replace]` bypass requires an explicit waiver in `guardrail3-rs.toml`
  - missing waiver is `Error`
  - weak waiver reason is `Error`
  - documented bypass is `Warn`
- `g3rs-apparch/same-layer-cycles`
  - same-layer non-dev workspace-internal dependency cycle is `Error`
- `g3rs-apparch/dev-dependency-direction`
  - forbidden `dev-dependencies` direction is `Warn`
  - target-specific dev edges also count
- `g3rs-apparch/types-purity`
  - `types/*` purity allowlist
  - allowed:
    - workspace-internal `types/*`
    - built-in pure externals
    - `allowed_deps` from `guardrail3-rs.toml`
- `g3rs-apparch/logic-purity`
  - `logic/*` purity allowlist
  - allowed:
    - workspace-internal `types/*`
    - built-in pure externals
    - `allowed_deps` from `guardrail3-rs.toml`
- `g3rs-apparch/types-public-surface`
  - `types/*` public surface must stay contract/data-oriented
  - public free functions and public inherent methods on concrete types are findings

Key decisions

- Carry the old `RS-HEXARCH-16` semantics forward as waiver-tracked bypasses, not as a plain direction violation.
  - Reason: root overrides are architectural escape hatches and should stay explicitly documented.
- Keep cycle detection scoped to non-dev workspace-internal edges.
  - Reason: the old separate dev-edge rule exists precisely so test-only coupling is not conflated with runtime graph integrity.
- Re-derive old domain purity into two `apparch` rules: `types/*` purity and `logic/*` purity.
  - Reason: the old domain layer split no longer exists.
- Make invalid Rust policy state fail closed for waiver-gated and allowlist-gated rules.
  - Reason: malformed policy must not silently degrade into "no waivers" or "empty allowlist."

Files to modify

- `packages/rs/apparch/g3rs-apparch-types/src/lib.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/Cargo.toml`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/run.rs`
- new rule files and sidecar test dirs under `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src`
- `packages/rs/apparch/g3rs-apparch-source-checks/crates/runtime/src/run.rs`
- new rule file and sidecar tests under `packages/rs/apparch/g3rs-apparch-source-checks/crates/runtime/src`
