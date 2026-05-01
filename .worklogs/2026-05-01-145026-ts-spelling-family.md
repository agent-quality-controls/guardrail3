Summary
- Added the G3TS spelling family, backed by cspell and Syncpack wiring checks instead of in-house spelling validation.
- Added a shared cspell JSON config parser and consumed package-script parser facts so fail-open shell chains are visible to guardrails.

Decisions made
- Delegated actual spelling to `cspell`; G3TS only proves `cspell` package/config/scripts/validate routing/Syncpack pinning exist and fail closed.
- Parsed JSON cspell configs through `packages/parsers/cspell-config-parser`; YAML configs are accepted by included-file existence and delegated to cspell runtime.
- Reused the package-script parser for shell semantics so `||` fallbacks cannot be hidden.
- Kept ignored cspell and Syncpack configs out of spelling ingestion so ignored files cannot satisfy policy.

Key files for context
- `packages/ts/spelling/g3ts-spelling-types`
- `packages/ts/spelling/g3ts-spelling-ingestion`
- `packages/ts/spelling/g3ts-spelling-config-checks`
- `packages/ts/spelling/g3ts-spelling-hook-contract`
- `packages/parsers/cspell-config-parser`
- `packages/parsers/package-script-command-parser/crates/runtime/src/parser.rs`
- `apps/guardrail3-ts/crates/logic/family-runner-config/src/run.rs`
- `apps/guardrail3-ts/crates/logic/family-runner-hooks/src/run.rs`

Verification
- `cargo test --manifest-path packages/parsers/package-script-command-parser/Cargo.toml --workspace --offline`
- `cargo test --manifest-path packages/parsers/cspell-config-parser/Cargo.toml --workspace --offline`
- `cargo test --manifest-path packages/ts/spelling/g3ts-spelling-ingestion/Cargo.toml --workspace --offline`
- `cargo test --manifest-path packages/ts/spelling/g3ts-spelling-config-checks/Cargo.toml --workspace --offline`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- `g3rs validate --path packages/parsers/cspell-config-parser --inventory`
- `g3rs validate --path packages/parsers/package-script-command-parser --inventory`
- `g3rs validate --path packages/ts/spelling/g3ts-spelling-ingestion --inventory`
- `g3rs validate --path packages/ts/spelling/g3ts-spelling-config-checks --inventory`
- `g3rs validate --path packages/ts/spelling/g3ts-spelling-types --inventory`
- `g3rs validate --path packages/ts/spelling/g3ts-spelling-hook-contract --inventory`
- `g3rs validate --path apps/guardrail3-ts --inventory`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family spelling --inventory`
- Final adversarial review reported no blocking findings.

Next steps
- Implement the next planned TS tooling family, likely `ts/package`, with the same delegation-first shape.
