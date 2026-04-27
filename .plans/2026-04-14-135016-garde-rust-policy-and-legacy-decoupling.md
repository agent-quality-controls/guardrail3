Goal

- Make the `garde` package family independent from deleted `apps/guardrail3` code and dead `guardrail3.toml`.
- Keep the family package-local and Rust-only.
- Preserve the meaningful `garde` rules, delete the dead `GuardrailConfig` rule, and move query-as waiver resolution onto `guardrail3-rs.toml`.

Approach

- Add proving tests first in `g3rs-garde-ingestion` and `g3rs-garde-source-checks`:
  - `guardrail3-rs.toml` missing does not block active garde checks.
  - malformed or unreadable `guardrail3-rs.toml` surfaces through `g3rs-garde/input-failures`.
  - `g3rs-garde/ast-04-query-as-inventory` reads waivers from `guardrail3-rs.toml`.
  - `RS-GARDE-SOURCE-08` no longer exists in routing or tests.
  - config tests no longer depend on deleted `guardrail3-domain-modules`.
- Replace old config selection in `g3rs-garde-ingestion`:
  - stop selecting `guardrail3.toml`
  - optionally select and parse `guardrail3-rs.toml`
  - make missing `guardrail3-rs.toml` mean "no waivers"
  - keep unreadable or malformed `guardrail3-rs.toml` as typed source-policy failure state
- Replace source-input contract in `g3rs-garde-types`:
  - remove `guardrail_toml`
  - add Rust policy state/waiver facts needed by source checks
- Remove old-app semantic leakage from `g3rs-garde-source-checks`:
  - delete `RS-GARDE-SOURCE-08`
  - remove `GuardrailConfig` parsing support
  - resolve query-as waivers from typed Rust policy facts
- Remove deleted-app test dependencies from `g3rs-garde-config-checks`:
  - replace `build_clippy_toml(...)` helpers with package-owned helpers
- Update docs and error messages from `guardrail3.toml` to `guardrail3-rs.toml` or "Rust policy" where applicable.

Key decisions

- `RS-GARDE-SOURCE-08` should be deleted, not ported.
  - Reason: it enforces validation on dead `GuardrailConfig`, which no longer exists in the Rust-only architecture.
- Missing `guardrail3-rs.toml` is not an ingestion error for `garde`.
  - Reason: `garde` can run without waivers; missing config means zero waivers.
- Invalid or unreadable `guardrail3-rs.toml` is still a real input failure.
  - Reason: once the file exists, the family must not fail open on broken waiver policy.

Files to modify

- `packages/rs/garde/g3rs-garde-types/src/lib.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/{run.rs,select.rs,parse.rs,ingest.rs}`
- `packages/rs/garde/g3rs-garde-ingestion/crates/types/src/error.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/{run.rs,support.rs,test_support.rs}`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/rs_garde_ast_04_query_as_inventory/rule.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/parse/{mod.rs,guardrail_config.rs}`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/rs_garde_ast_08_guardrail_config_validate_call/*`
- `packages/rs/garde/g3rs-garde-source-checks/crates/assertions/src/rs_garde_ast_08_guardrail_config_validate_call.rs`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/{Cargo.toml,src/run_tests/mod.rs}`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/rs_garde_config_0{2,3,4,5}_*/rule_tests/helpers.rs`
- `packages/rs/garde/g3rs-garde-ingestion/README.md`
- `packages/rs/garde/g3rs-garde-source-checks/README.md`
