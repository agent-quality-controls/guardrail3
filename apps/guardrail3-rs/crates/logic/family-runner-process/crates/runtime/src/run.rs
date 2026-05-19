use g3_workspace_crawl::G3WorkspaceCrawl;
use g3rs_hooks_contract_types::G3HookRequirement;
use guardrail3_rs_app_types::{FamilyResults, FamilyRunError, SupportedFamily};

/// Runs the hooks or release family group against the prepared crawl.
///
/// # Errors
///
/// Returns [`FamilyRunError`] when ingestion for the selected family fails.
pub fn run(
    family: SupportedFamily,
    crawl: &G3WorkspaceCrawl,
    enabled_families: &[SupportedFamily],
) -> Result<FamilyResults, FamilyRunError> {
    match family {
        SupportedFamily::Hooks => {
            let requirements = rust_hook_requirements(enabled_families);
            let mut config_input =
                g3rs_hooks_ingestion::ingest_for_config_checks(crawl).map_err(|error| {
                    FamilyRunError {
                        message: format!("{error:?}"),
                    }
                })?;
            let filetree_input =
                g3rs_hooks_ingestion::ingest_for_file_tree_checks(crawl).map_err(|error| {
                    FamilyRunError {
                        message: format!("{error:?}"),
                    }
                })?;
            let mut source_inputs =
                g3rs_hooks_ingestion::ingest_for_source_checks(crawl).map_err(|error| {
                    FamilyRunError {
                        message: format!("{error:?}"),
                    }
                })?;

            config_input.requirements.clone_from(&requirements);
            for input in &mut source_inputs {
                input.requirements.clone_from(&requirements);
            }

            let mut results = Vec::new();
            results.extend(g3rs_hooks_config_checks::check(&config_input));
            results.extend(g3rs_hooks_file_tree_checks::check(&filetree_input));
            results.extend(g3rs_hooks_source_checks::check_all(&source_inputs));
            Ok(results)
        }
        SupportedFamily::Release => {
            let config_input =
                g3rs_release_ingestion::ingest_for_config_checks(crawl).map_err(|error| {
                    FamilyRunError {
                        message: format!("{error:?}"),
                    }
                })?;
            let filetree_input = g3rs_release_ingestion::ingest_for_file_tree_checks(crawl)
                .map_err(|error| FamilyRunError {
                    message: format!("{error:?}"),
                })?;
            let source_input =
                g3rs_release_ingestion::ingest_for_source_checks(crawl).map_err(|error| {
                    FamilyRunError {
                        message: format!("{error:?}"),
                    }
                })?;
            let repo_root_input = g3rs_release_ingestion::ingest_for_repo_root_checks(crawl)
                .map_err(|error| FamilyRunError {
                    message: format!("{error:?}"),
                })?;

            let mut results = Vec::new();
            results.extend(g3rs_release_config_checks::check(&config_input));
            results.extend(g3rs_release_filetree_checks::check(&filetree_input));
            results.extend(g3rs_release_source_checks::check(&source_input));
            results.extend(g3rs_release_repo_root_checks::check(&repo_root_input));
            Ok(results)
        }
        SupportedFamily::Topology
        | SupportedFamily::Toolchain
        | SupportedFamily::Fmt
        | SupportedFamily::Cargo
        | SupportedFamily::Clippy
        | SupportedFamily::Deny
        | SupportedFamily::Code
        | SupportedFamily::Arch
        | SupportedFamily::Deps
        | SupportedFamily::Garde
        | SupportedFamily::Test
        | SupportedFamily::Apparch => Err(FamilyRunError {
            message: format!("process group does not handle {family:?}"),
        }),
    }
}

/// Collects Rust family hook requirements from enabled family-owned hook contract packages.
pub(crate) fn rust_hook_requirements(
    enabled_families: &[SupportedFamily],
) -> Vec<G3HookRequirement> {
    let mut requirements = Vec::new();
    if enabled_families.contains(&SupportedFamily::Topology) {
        requirements.extend(g3rs_topology_hook_contract::hook_contract());
    }
    if enabled_families.contains(&SupportedFamily::Toolchain) {
        requirements.extend(g3rs_toolchain_hook_contract::hook_contract());
    }
    if enabled_families.contains(&SupportedFamily::Fmt) {
        requirements.extend(g3rs_fmt_hook_contract::hook_contract());
    }
    if enabled_families.contains(&SupportedFamily::Cargo) {
        requirements.extend(g3rs_cargo_hook_contract::hook_contract());
    }
    if enabled_families.contains(&SupportedFamily::Clippy) {
        requirements.extend(g3rs_clippy_hook_contract::hook_contract());
    }
    if enabled_families.contains(&SupportedFamily::Deny) {
        requirements.extend(g3rs_deny_hook_contract::hook_contract());
    }
    if enabled_families.contains(&SupportedFamily::Code) {
        requirements.extend(g3rs_code_hook_contract::hook_contract());
    }
    if enabled_families.contains(&SupportedFamily::Arch) {
        requirements.extend(g3rs_arch_hook_contract::hook_contract());
    }
    if enabled_families.contains(&SupportedFamily::Deps) {
        requirements.extend(g3rs_deps_hook_contract::hook_contract());
    }
    if enabled_families.contains(&SupportedFamily::Garde) {
        requirements.extend(g3rs_garde_hook_contract::hook_contract());
    }
    if enabled_families.contains(&SupportedFamily::Test) {
        requirements.extend(g3rs_test_hook_contract::hook_contract());
    }
    if enabled_families.contains(&SupportedFamily::Release) {
        requirements.extend(g3rs_release_hook_contract::hook_contract());
    }
    if enabled_families.contains(&SupportedFamily::Apparch) {
        requirements.extend(g3rs_apparch_hook_contract::hook_contract());
    }
    requirements
}
