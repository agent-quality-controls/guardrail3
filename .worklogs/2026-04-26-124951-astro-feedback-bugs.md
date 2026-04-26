# Summary

Fixed Astro feedback bugs from the landing integration pass. G3TS no longer requires brittle `astro-seo`, no longer treats unrelated unsupported package scripts as proof that `astro check` is missing, and no longer makes Astro Syncpack policy order-sensitive.

# Decisions Made

- Kept delegated inline-copy enforcement through `eslint-plugin-i18next`; the bug was not delegation, it was parser and policy detection.
- Removed `astro-seo` from the required Astro package stack because `astro-seo@1.1.0` exports TypeScript source directly. G3TS now requires `schema-dts` for typed JSON-LD and keeps rendered output quality delegated to Nuasite.
- Changed Syncpack Astro policy matching from fixed-prefix matching to unordered exact canonical groups. Duplicate or shadow groups for the same dependency still fail.
- Changed `dependencyTypes` matching to exact unordered set matching. Array order no longer matters, but missing, extra, or duplicate values still fail.
- Changed `has_safe_tool_invocation` to ignore unrelated unsupported scripts but fail closed when unsupported shell syntax contains the requested target invocation.

# Key Files For Context

- `packages/parsers/package-script-command-parser/crates/runtime/src/parser.rs`
- `packages/parsers/package-script-command-parser/crates/types/src/document.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_09_syncpack_stack_pins.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_10_syncpack_forbidden_deps.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_17_seo_packages.rs`
- `.plans/2026-04-26-121146-astro-feedback-bugs.md`

# Verification

- `cargo test -q --manifest-path packages/parsers/package-script-command-parser/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-config-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`
- `cargo fmt --check --manifest-path packages/parsers/package-script-command-parser/Cargo.toml`
- `cargo fmt --check --manifest-path packages/ts/astro/g3ts-astro-ingestion/Cargo.toml`
- `cargo fmt --check --manifest-path packages/ts/astro/g3ts-astro-config-checks/Cargo.toml`
- `git diff --check`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`

# Adversarial Review

- First parser review found unsupported target invocations hidden in non-top-level shell syntax could be masked by a safe script. Fixed by tracking all tool invocations separately from top-level visible invocations.
- First Syncpack review found `dependencyTypes` were still positional and shadow groups with different dependency types could escape. Fixed by matching dependencies separately from canonical group validation.
- First SEO review found no blocking issue and requested an explicit no-`astro-seo` regression. Added it.
- Second parser and Syncpack adversarial reviews found no remaining blocking gaps.

# Next Steps

- Landing agent can rerun `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory` against the installed local CLI.
