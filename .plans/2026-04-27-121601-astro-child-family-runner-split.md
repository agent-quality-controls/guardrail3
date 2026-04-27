# Goal

Finish the Astro split at the runner/check ownership layer so `--family astro` no longer executes one flat Astro check bucket.

# Approach

- Keep the current shared Astro ingestion snapshot for this commit.
- Split the config-check runtime into explicit child-family entry points:
  - `check_setup`
  - `check_content`
  - `check_mdx`
  - `check_seo`
- Split the file-tree runtime into explicit child-family entry points:
  - `check_setup`
  - `check_content`
  - `check_state`
- Wire `apps/guardrail3-ts` to run those child entry points in order for `SupportedFamily::Astro`.
- Keep existing public `check(...)` facade as a compatibility aggregate only inside the crate, backed by the child functions.
- Do not add new rule semantics in this commit. This commit is mechanical ownership split only.

# Rule Ownership

Astro setup config:

- `TS-ASTRO-CONFIG-01`
- `TS-ASTRO-CONFIG-02`
- `TS-ASTRO-CONFIG-03`
- `TS-ASTRO-CONFIG-05`
- `TS-ASTRO-CONFIG-06`
- `TS-ASTRO-CONFIG-07`
- `TS-ASTRO-CONFIG-09`
- `TS-ASTRO-CONFIG-10`
- `TS-ASTRO-CONFIG-11`
- `TS-ASTRO-CONFIG-12`
- `TS-ASTRO-CONFIG-21`

Astro content config:

- `TS-ASTRO-CONFIG-18`
- `TS-ASTRO-CONFIG-19`
- `TS-ASTRO-CONFIG-23`
- `TS-ASTRO-CONFIG-24`
- `TS-ASTRO-CONFIG-25`
- `TS-ASTRO-CONFIG-26`
- `TS-ASTRO-CONFIG-27`
- `TS-ASTRO-CONFIG-28`

Astro MDX config:

- `TS-ASTRO-CONFIG-20`
- `TS-ASTRO-CONFIG-30`

Astro SEO config:

- `TS-ASTRO-CONFIG-13`
- `TS-ASTRO-CONFIG-14`
- `TS-ASTRO-CONFIG-15`
- `TS-ASTRO-CONFIG-16`
- `TS-ASTRO-CONFIG-17`
- `TS-ASTRO-CONFIG-22`
- `TS-ASTRO-CONFIG-29`
- `TS-ASTRO-CONFIG-31`
- `TS-ASTRO-CONFIG-32`

Astro setup file-tree:

- `TS-ASTRO-FILETREE-01`
- `TS-ASTRO-FILETREE-03`

Astro content file-tree:

- `TS-ASTRO-FILETREE-02`
- `TS-ASTRO-FILETREE-04`
- `TS-ASTRO-FILETREE-05`
- `TS-ASTRO-FILETREE-06`

Astro state file-tree:

- `TS-ASTRO-FILETREE-11`
- `TS-ASTRO-FILETREE-12`

# Why This Before New Crates

The old packages already contain cohesive test fixtures and shared support. Creating Cargo packages first would mostly copy files and tests before proving the rule split. This commit makes runtime ownership explicit first. A later package extraction can move each child entry point into its own package without changing rule behavior.

# Files To Modify

- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/lib.rs`
- `packages/ts/astro/g3ts-astro-config-checks/src/lib.rs`
- `packages/ts/astro/g3ts-astro-file-tree-checks/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-file-tree-checks/crates/runtime/src/lib.rs`
- `packages/ts/astro/g3ts-astro-file-tree-checks/src/lib.rs`
- `apps/guardrail3-ts/crates/logic/family-runner-structure/src/run.rs`

# Verification

- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-config-checks`
- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-file-tree-checks`
- `cargo test --workspace` in `apps/guardrail3-ts`
- `g3rs validate --path packages/ts/astro/g3ts-astro-config-checks`
- `g3rs validate --path packages/ts/astro/g3ts-astro-file-tree-checks`
- `g3rs validate --path apps/guardrail3-ts`
- install local G3TS
- run G3TS against landing
