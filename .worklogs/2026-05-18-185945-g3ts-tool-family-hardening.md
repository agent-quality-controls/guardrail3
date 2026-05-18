Summary:
- Added manifest-driven hardening for G3TS `fmt`, `spelling`, and `typecov` families.
- The families now require standard check script names, fail-closed tool invocations, validate-script reachability, Syncpack pin coverage, and hook trigger coverage.
- Added fixtures and golden outputs for valid contracts, broken policy layers, bad script routing, targetless command forms, Syncpack selector edge cases, and typecov policy thresholds.

Decisions made:
- `fmt` owns `format:check`; it no longer requires an optional write-format script.
- `spelling` owns `spellcheck`; bare `cspell`, `cspell lint --root .`, `cspell check ...`, `cspell trace ...`, and `--no-exit-code` are not accepted as fail-closed spelling checks.
- `typecov` owns `typecov`; `[typecov] minimum` is parsed from `guardrail3-ts.toml` and all `--at-least` thresholds must meet that policy.
- Syncpack matching now models selector patterns and first matching version group behavior. Later pinned groups cannot rescue an earlier matching ignored, banned, or unpinned group.
- Dependency specifier classification is conservative. Non-exact specifiers such as `latest`, tags, `file:`, `link:`, git, URL, `catalog:`, `npm:` alias, and `workspace:` do not satisfy `specifierTypes = ["exact"]`.
- `package-json-parser` now owns dependency declaration extraction and dependency specifier classification so fmt, spelling, and typecov do not duplicate package parsing.
- `syncpack-config-parser` now owns Syncpack selector matching and `globset`; fmt, spelling, and typecov consume that shared parser API instead of duplicating matching logic.
- `js-semver` was rejected for ingestion because its `MIT-0` license violates the current G3RS license baseline.
- The generated G3TS pre-commit hook now excludes `behavior/fixtures/**` before lockfile integrity and workspace routing. Fixture `package.json` and `guardrail3-ts.toml` files are test data and must not trigger repo-root `pnpm install --frozen-lockfile` or broken-fixture workspace validation.
- The generated G3TS pre-commit hook and CLI `--staged` filter now recognize `.syncpackrc`, Prettier config files, CSpell config files, and `tsconfig*.json` so tool-config-only changes route to workspace validation.
- Prettier config discovery now accepts the same `prettier.config.*` and `.prettierrc*` filename families that the hook routes.
- CSpell config discovery and hook contract now accept the same `cspell.config.*` filename family that the hook routes.
- Prettier target detection treats `--cache` as a boolean option, and the clean fmt fixture now proves `prettier --check --cache .` is accepted.
- Parser workspaces touched by the shared parser refactor use Cargo resolver 3, matching the existing G3TS TOML parser workspace and satisfying G3RS rust-version-aware resolver policy.

Key files for context:
- `.plans/2026-05-18-160504-g3ts-tool-family-hardening.md`
- `.plans/2026-05-18-160504-g3ts-tool-family-hardening.md.manifest.toml`
- `scripts/verify-g3ts-tool-family-hardening.py`
- `packages/ts/fmt/g3ts-fmt-config-checks/crates/runtime/src/common.rs`
- `packages/ts/fmt/g3ts-fmt-config-checks/crates/runtime/src/syncpack_prettier_pin.rs`
- `packages/ts/spelling/g3ts-spelling-config-checks/crates/runtime/src/common.rs`
- `packages/ts/spelling/g3ts-spelling-config-checks/crates/runtime/src/syncpack_cspell_pin.rs`
- `packages/ts/typecov/g3ts-typecov-config-checks/crates/runtime/src/syncpack_type_coverage_pin.rs`
- `packages/ts/typecov/g3ts-typecov-ingestion/crates/runtime/src/policy.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/init.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/execute.rs`
- `packages/parsers/package-json-parser/crates/runtime/src/parser.rs`
- `packages/parsers/syncpack-config-parser/crates/runtime/src/matcher.rs`
- `behavior/fixtures/g3ts-rule/fmt`
- `behavior/fixtures/g3ts-rule/spelling`
- `behavior/fixtures/g3ts-rule/typecov`
- `behavior/golden/g3ts-rule/approved.normalized.json`

Verification:
- `python3 scripts/verify-g3ts-tool-family-hardening.py`
- `cargo fmt --all --manifest-path apps/guardrail3-ts/Cargo.toml -- --check`
- `cargo test --workspace --manifest-path apps/guardrail3-ts/Cargo.toml`
- `cargo clippy --workspace --all-targets --all-features --manifest-path apps/guardrail3-ts/Cargo.toml -- -D warnings`
- `fixture3 check --all`
- `g3ts validate repo --path .`
- `g3rs validate repo --path .`
- `g3rs validate workspace --path <touched workspace>` for parser, fmt, spelling, and typecov workspaces. This exited 0 with existing warnings only.
- Adversarial review converged with `NO FINDINGS`.

Next steps:
- Use the same manifest and fixture pattern for the next G3TS family instead of adding unit tests.
- If Syncpack specifier matching needs full upstream parity later, add a maintained parser or expose Syncpack's own classifier instead of expanding local classifier logic.
