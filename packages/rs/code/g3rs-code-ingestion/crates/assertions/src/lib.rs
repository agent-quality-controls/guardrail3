#![expect(
    clippy::module_name_repetitions,
    clippy::missing_assert_message,
    clippy::indexing_slicing,
    clippy::too_many_lines,
    clippy::panic,
    clippy::type_complexity,
    reason = "this is the test-assertion harness for the code-ingestion family; rule-id-aligned submodule names (run_config, run_file_tree, ...) mirror the ingestion entry points and renaming would break the rule-id-to-module mapping the runtime tests rely on. The harness deliberately uses indexing on freshly built fixtures with known shape, panics on unexpected variants (the assertion's whole purpose is to fail-fast), and accepts complex tuple types directly because the entry points for each assertion are part of the contract under test"
)]

#[cfg(feature = "ingest")]
use g3rs_code_ingestion_runtime as _;

#[cfg(feature = "ingest")]
pub mod run;
/// Per-config-fact assertion helpers shared by the runtime tests.
#[cfg(feature = "ingest")]
mod run_config;
/// Per-file-tree-fact assertion helpers shared by the runtime tests.
#[cfg(feature = "ingest")]
mod run_file_tree;
/// Per-pipeline assertion helpers shared by the runtime tests.
#[cfg(feature = "ingest")]
mod run_pipeline;
/// Per-results assertion helpers shared by the runtime tests.
#[cfg(feature = "ingest")]
mod run_results;
/// Per-source-file assertion helpers shared by the runtime tests.
#[cfg(feature = "ingest")]
mod run_source;
