# Package Syncpack And Shell Parser Cleanup

## Goal

Move package-family dependency policy out of direct manifest validation and into validator-enforcement checks, and remove local shell parsing from package script checks.

Desired end state:

- `g3ts-package/local-banned-dependencies` proves Syncpack is installed, run fail-closed, and configured to ban the package-family dependency policy.
- `g3ts-package/root-scripts` proves `only-allow pnpm` through parsed command facts, not raw string matching.
- `package-script-command-parser` uses a dedicated shell parser library for command splitting and word extraction.
- Package-family checks consume typed ingestion facts and do not parse shell scripts or Syncpack config themselves.

## Approach

1. Add parser-backed package script facts.

- Modify `packages/parsers/package-script-command-parser/crates/runtime/Cargo.toml`.
- Add `tree-sitter` and `tree-sitter-bash`.
- Replace `split_segments`, `split_tokens`, and quote-state parsing in `parser.rs` with a small adapter over the `tree-sitter-bash` AST.
- Keep the existing public document model stable: `PackageScriptCommand`, separators, tool invocations, ESLint invocations, and safe-tool query.
- Fail closed when guardrail-related scripts contain unsupported shell grammar such as pipes, background commands, redirects, command substitutions, or expansions.
- Keep existing wrapper normalization for `pnpm`, `npm`, `yarn`, `bun`, `npx`, `bunx`, `env`, and `cross-env`.

2. Add typed package-family script and Syncpack facts.

- Modify `packages/ts/package/g3ts-package-types/src/types.rs`.
- Add package script command/tool invocation/parse-blocker types mirroring the Astro integration contract types.
- Add root booleans for `safely_runs_only_allow_pnpm` and `safely_runs_syncpack_lint`.
- Add Syncpack config state and snapshot types for package-family source coverage and missing banned dependency groups.
- Keep package-family checks independent from parser crates.

3. Update package ingestion.

- Modify `packages/ts/package/g3ts-package-ingestion/crates/runtime/Cargo.toml`.
- Depend on `package-script-command-parser` and `syncpack-config-parser`.
- Parse root package scripts once during ingestion.
- Select root `.syncpackrc` for package-manager roots.
- Compute package-family Syncpack facts:
  - missing `.syncpackrc`
  - unreadable or parse-error `.syncpackrc`
  - exact source coverage for every package manifest under package-family scope
  - missing canonical banned `versionGroups`
- Do not scan local dependency names for banned packages in the package family.

4. Update package checks.

- Modify `g3ts-package/root-scripts` to require `safely_runs_only_allow_pnpm`.
- Modify `g3ts-package/local-banned-dependencies` to require Syncpack setup/configuration instead of direct local manifest dependency scanning.
- Remove banned dependency lists and local dependency scanning helpers from package config checks.
- Keep `g3ts-package/root-package-manager` unchanged unless verification shows an active bug. Package-manager semver parsing is not part of this requested cleanup.

5. Update tests.

- Add parser tests proving shell metacharacter bypasses are parser-backed and fail closed.
- Update package ingestion tests for root script facts and Syncpack facts.
- Update package config check tests:
  - safe `npx only-allow pnpm` passes
  - fake text like `echo only-allow pnpm` fails
  - fail-open `syncpack lint || true` fails
  - missing `.syncpackrc` fails
  - missing source coverage fails
  - missing banned groups fails
  - canonical Syncpack config passes

## Key Decisions

- Syncpack owns dependency policy because it is the external validator whose execution can be enforced by guardrails.
- G3TS package checks own the presence, fail-closed script wiring, and exact configuration contract for Syncpack.
- `tree-sitter-bash` is the shell parser dependency because it parses Bash grammar; `shell-words` and `shlex` are tokenizers and would still require hand-rolled command splitting.
- Exact Syncpack source entries are required instead of glob interpretation. G3TS does not need to emulate Syncpack glob semantics to prove the intended files are covered.
- The package family keeps broad package-manager root rules. It does not become an Astro or app-specific dependency policy.

## Files To Modify

- `packages/parsers/package-script-command-parser/crates/runtime/Cargo.toml`
- `packages/parsers/package-script-command-parser/crates/runtime/src/parser.rs`
- `packages/parsers/package-script-command-parser/crates/runtime/src/parser_tests/cases.rs`
- `packages/ts/package/g3ts-package-types/src/types.rs`
- `packages/ts/package/g3ts-package-types/src/convert.rs`
- `packages/ts/package/g3ts-package-ingestion/crates/runtime/Cargo.toml`
- `packages/ts/package/g3ts-package-ingestion/crates/runtime/src/run.rs`
- `packages/ts/package/g3ts-package-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/package/g3ts-package-ingestion/crates/assertions/src/run.rs`
- `packages/ts/package/g3ts-package-config-checks/crates/runtime/src/support.rs`
- `packages/ts/package/g3ts-package-config-checks/crates/runtime/src/ts_package_config_06_root_scripts.rs`
- `packages/ts/package/g3ts-package-config-checks/crates/runtime/src/ts_package_config_08_local_banned_dependencies.rs`
- `packages/ts/package/g3ts-package-config-checks/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/package/g3ts-package-config-checks/crates/runtime/src/run_tests/cases.rs`

## Verification

- `cargo test -q --manifest-path packages/parsers/package-script-command-parser/Cargo.toml --workspace`
- `cargo clippy -q --manifest-path packages/parsers/package-script-command-parser/Cargo.toml --workspace --all-targets -- -D warnings`
- `cargo test -q --manifest-path packages/ts/package/g3ts-package-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/package/g3ts-package-config-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`

## Adversarial Review Checklist

- A root script with `echo only-allow pnpm` must not satisfy `g3ts-package/root-scripts`.
- A root script with `only-allow pnpm || true` must not satisfy `g3ts-package/root-scripts`.
- A root script with `echo syncpack lint` must not satisfy `g3ts-package/local-banned-dependencies`.
- A root script with `syncpack lint || true` must not satisfy `g3ts-package/local-banned-dependencies`.
- A local manifest with a banned dependency must be allowed by G3TS only if Syncpack is installed, run, and configured to ban that dependency.
- Missing local `package.json` source coverage in `.syncpackrc` must fail even if banned groups exist.
- Missing canonical banned groups must fail even if `syncpack lint` is wired.
- The package checks must not contain package-family banned dependency scanning.
