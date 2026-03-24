# Rust Generator Gap Analysis

This file holds current implementation mapping and known mismatches between the target generator specs in this directory and the current generator code.

The family spec files are pure target-state contracts.
This file is the current-state map.

## Current shared generator entrypoints

- `apps/guardrail3/crates/adapters/inbound/cli/generate.rs`
- `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers.rs`

Current canonical/builders:
- `apps/guardrail3/crates/domain/modules/canonical.rs`
- `apps/guardrail3/crates/domain/modules/clippy/`
- `apps/guardrail3/crates/domain/modules/deny.rs`
- `apps/guardrail3/crates/domain/modules/release.rs`
- `apps/guardrail3/crates/domain/modules/pre_commit.rs`

## Current family status

### fmt
- current code writes `rustfmt.toml` at multiple Rust roots
- target contract is repository-root-only
- nested `rustfmt.toml` generation is a target-state mismatch

### toolchain
- current code writes root `rust-toolchain.toml`
- this is close to the target contract

### clippy
- current code writes local `clippy.toml` files derived from profile and overrides
- target contract keeps this family local to Rust policy roots
- current helper/root-selection logic is still CLI/config-driven rather than root-taxonomy-driven

### deny
- current code writes local `deny.toml` files derived from profile and overrides
- target contract keeps this family local to Rust policy roots
- current helper/root-selection logic is still CLI/config-driven rather than root-taxonomy-driven

### cargo
- current code does not patch `Cargo.toml`
- generator only prints manual cargo-lints guidance

### release
- current code writes `cliff.toml`
- current code writes a placeholder `release-plz.toml`
- current code does not generate release workflows
- current code does not generate binary release workflows

### hooks
- current code writes monolithic `.githooks/pre-commit`
- target contract is modular `.githooks/pre-commit` plus `.githooks/pre-commit.d/*.sh`
- current code still contains TS stack branches; active Rust planning is Rust/shared only

### hexarch
- current code does not scaffold Rust hex architecture

### libarch
- current code does not scaffold library architecture

## Golden-fixture shape the generator must respect

The primary mixed fixture is:
- `apps/guardrail3/tests/fixtures/r_arch_01/golden`

Important realities in that fixture:
- root packages workspace
- multiple nested app workspaces
- standalone package/library area
- nested inner hex roots inside an outer app workspace
- Rust roots and non-Rust roots coexisting in the same repo

Important supporting adversarial fixtures:
- `apps/guardrail3/tests/adversarial_deep_nesting.rs`
- `apps/guardrail3/tests/adversarial_path_resolution.rs`
- `apps/guardrail3/tests/adversarial_nightmare_monorepo.rs`

## Immediate contract mismatches already identified

- `fmt` draft originally modeled per-root generation; checker contract is root-only
- generator family drafts originally described ownership in terms of current helpers like `resolve_rust_root(cfg)` instead of project-shape roots
- release parity requires workflow generation, not only config-file generation
- hook parity requires generator-owned structural shape, not only semantic line content
- `libarch` in a mixed monorepo needs its own workspace boundary and must not be silently treated as an ordinary member of an ancestor workspace
