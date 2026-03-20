// Suppress unused crate dependency warnings for crates used only by the main binary/lib
use clap as _;
use colored as _;
use garde as _;
use glob as _;
use ignore as _;
use proc_macro2 as _;
use proptest as _;
use quote as _;
use serde as _;
use serde_json as _;
use syn as _;
use tempfile as _;
use toml as _;
use toml_edit as _;
use tree_sitter as _;
use tree_sitter_javascript as _;
use tree_sitter_typescript as _;
use walkdir as _;
#[path = "unit/allow_checks_test.rs"]
mod allow_checks_test;
#[path = "unit/ast_helpers_test.rs"]
mod ast_helpers_test;
#[path = "unit/ast_visitors_test.rs"]
mod ast_visitors_test;
#[path = "unit/code_quality_checks_test.rs"]
mod code_quality_checks_test;
#[path = "unit/deny_inventory_test.rs"]
mod deny_inventory_test;
#[path = "unit/dependency_allowlist_test.rs"]
mod dependency_allowlist_test;
#[path = "unit/discover_test.rs"]
mod discover_test;
#[path = "unit/help_gen_test.rs"]
mod help_gen_test;
#[path = "unit/report_test.rs"]
mod report_test;
#[path = "unit/rs_structure_checks_test.rs"]
mod rs_structure_checks_test;
#[path = "unit/rs_test_checks_test.rs"]
mod rs_test_checks_test;
#[path = "unit/rs_test_quality_checks_test.rs"]
mod rs_test_quality_checks_test;
#[path = "unit/test_garde_checks.rs"]
mod test_garde_checks;
#[path = "unit/test_hex_arch_checks.rs"]
mod test_hex_arch_checks;
#[path = "unit/test_release_bin_checks.rs"]
mod test_release_bin_checks;
#[path = "unit/test_release_checks.rs"]
mod test_release_checks;
#[path = "unit/test_release_crate_checks.rs"]
mod test_release_crate_checks;
#[path = "unit/test_release_crate_deps.rs"]
mod test_release_crate_deps;
#[path = "unit/test_release_repo_checks.rs"]
mod test_release_repo_checks;
#[path = "unit/test_source_scan.rs"]
mod test_source_scan;
#[path = "unit/ts_arch_checks_test.rs"]
mod ts_arch_checks_test;
#[path = "unit/ts_ast_helpers_test.rs"]
mod ts_ast_helpers_test;
#[path = "unit/ts_code_analysis_test.rs"]
mod ts_code_analysis_test;
#[path = "unit/ts_comment_checks_test.rs"]
mod ts_comment_checks_test;
#[path = "unit/ts_source_scan_test.rs"]
mod ts_source_scan_test;
#[path = "unit/ts_test_checks_test.rs"]
mod ts_test_checks_test;
