Goal
- Finish the Astro split cleanup so no rule file receives a whole Astro family input bag when a runner can pass a narrower typed slice.
- Keep shared Astro support limited to reusable file/surface readers, not aggregate family ingestion.
- Verify the split mechanically and through adversarial review before committing.

Approach
- Narrow setup file-tree rules:
  - `ts_astro_filetree_01_astro_config_exists.rs` receives one `G3TsAstroSetupAppRootInput`.
  - `ts_astro_filetree_03_live_config_exists.rs` receives one `G3TsAstroSetupAppRootInput` for a live collection root.
  - `run.rs` keeps the package input and fans out exact slices.
- Narrow content file-tree rules:
  - `ts_astro_filetree_02_content_config_exists.rs` receives build collection roots.
  - `ts_astro_filetree_04_no_route_markdown_pages.rs` receives a boolean for whether any collection root exists plus route markdown pages.
  - `ts_astro_filetree_05_no_velite_config.rs` receives app roots.
  - `ts_astro_filetree_06_no_velite_output.rs` receives strict content roots.
  - `run.rs` owns the cross-list fanout.
- Narrow state file-tree rules:
  - `ts_astro_filetree_11_no_legacy_parallel_state.rs` receives strict content roots.
  - `ts_astro_filetree_12_configured_forbidden_state.rs` receives strict content roots.
  - `run.rs` owns build/live root concatenation.
- Run grep checks for removed aggregate names and rule signatures.
- Run cargo tests for the app workspace and affected Astro packages.
- Install the local G3TS CLI and run it against the landing app.
- Run adversarial agents against the code after implementation and fix every real finding before commit.

Key Decisions
- Keep `--family astro` as a CLI alias only. It is not a concrete `SupportedFamily` variant and reports concrete Astro subfamily names.
- Do not remove shared support surface readers unless they assemble cross-family facts. Shared parsing is acceptable; shared aggregate ingestion is not.
- Do not create new compatibility shims for old aggregate input names.

Files To Modify
- `packages/ts/astro/setup/g3ts-astro-setup-file-tree-checks/crates/runtime/src/run.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-file-tree-checks/crates/runtime/src/ts_astro_filetree_01_astro_config_exists.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-file-tree-checks/crates/runtime/src/ts_astro_filetree_03_live_config_exists.rs`
- `packages/ts/astro/content/g3ts-astro-content-file-tree-checks/crates/runtime/src/run.rs`
- `packages/ts/astro/content/g3ts-astro-content-file-tree-checks/crates/runtime/src/ts_astro_filetree_02_content_config_exists.rs`
- `packages/ts/astro/content/g3ts-astro-content-file-tree-checks/crates/runtime/src/ts_astro_filetree_04_no_route_markdown_pages.rs`
- `packages/ts/astro/content/g3ts-astro-content-file-tree-checks/crates/runtime/src/ts_astro_filetree_05_no_velite_config.rs`
- `packages/ts/astro/content/g3ts-astro-content-file-tree-checks/crates/runtime/src/ts_astro_filetree_06_no_velite_output.rs`
- `packages/ts/astro/state/g3ts-astro-state-file-tree-checks/crates/runtime/src/run.rs`
- `packages/ts/astro/state/g3ts-astro-state-file-tree-checks/crates/runtime/src/ts_astro_filetree_11_no_legacy_parallel_state.rs`
- `packages/ts/astro/state/g3ts-astro-state-file-tree-checks/crates/runtime/src/ts_astro_filetree_12_configured_forbidden_state.rs`
