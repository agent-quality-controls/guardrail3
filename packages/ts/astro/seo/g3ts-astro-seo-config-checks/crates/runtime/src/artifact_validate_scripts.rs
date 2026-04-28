use g3ts_astro_seo_types::{
    G3TsAstroConfigSurfaceState, G3TsAstroPackageScriptCommandSeparator,
    G3TsAstroPackageScriptToolInvocation, G3TsAstroPackageSurfaceSnapshot,
    G3TsAstroSeoIntegrationContractInput,
};
use guardrail3_check_types::G3CheckResult;

const SITEMAP_ID: &str = "g3ts-astro-seo/sitemap-checks-validate-script";
const ROBOTS_ID: &str = "g3ts-astro-seo/robots-checks-validate-script";
const LLMS_ID: &str = "g3ts-astro-seo/llms-checks-validate-script";

pub(crate) fn check(
    contract: &G3TsAstroSeoIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    check_required(contract, results, SITEMAP_ID, "g3ts-astro-sitemap-checks");
    check_required(contract, results, ROBOTS_ID, "g3ts-astro-robots-checks");
    if crate::support::strict_ai_readable_enabled(&contract.astro_policy) {
        check_required(contract, results, LLMS_ID, "g3ts-astro-llms-checks");
    }
}

fn check_required(
    contract: &G3TsAstroSeoIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
    id: &str,
    checker_executable: &str,
) {
    let rel_path = crate::support::package_rel_path(&contract.package);
    let Some(site) = canonical_site(contract) else {
        results.push(crate::support::error(
            id,
            "Validate script cannot prove artifact checker target",
            "`validate` checker arguments require a parseable HTTPS `astro.config.site`."
                .to_owned(),
            rel_path,
        ));
        return;
    };
    let output_dir = astro_output_dir(contract);
    let valid = crate::support::parsed_package(&contract.package).is_some_and(|snapshot| {
        validate_runs_build_before_checker(
            snapshot,
            contract,
            checker_executable,
            &site,
            &output_dir,
        )
    });

    if valid {
        if let Some(rel_path) = rel_path {
            results.push(crate::support::info(
                id,
                "Validate script runs Astro build before artifact checker",
                format!("`{rel_path}` runs `astro build` before `{checker_executable}`."),
                rel_path,
            ));
        }
        return;
    }

    results.push(crate::support::error(
        id,
        "Validate script must run Astro build before artifact checker",
        format!("`validate` must fail closed and run `astro build` before `{checker_executable}`."),
        rel_path,
    ));
}

fn canonical_site(contract: &G3TsAstroSeoIntegrationContractInput) -> Option<String> {
    match &contract.astro_config {
        G3TsAstroConfigSurfaceState::Parsed { snapshot } => snapshot.site.clone(),
        G3TsAstroConfigSurfaceState::Missing { .. }
        | G3TsAstroConfigSurfaceState::Unreadable { .. }
        | G3TsAstroConfigSurfaceState::ParseError { .. } => None,
    }
}

fn astro_output_dir(contract: &G3TsAstroSeoIntegrationContractInput) -> String {
    match &contract.astro_config {
        G3TsAstroConfigSurfaceState::Parsed { snapshot } => snapshot
            .out_dir
            .clone()
            .unwrap_or_else(|| "dist".to_owned()),
        G3TsAstroConfigSurfaceState::Missing { .. }
        | G3TsAstroConfigSurfaceState::Unreadable { .. }
        | G3TsAstroConfigSurfaceState::ParseError { .. } => "dist".to_owned(),
    }
}

fn validate_runs_build_before_checker(
    snapshot: &G3TsAstroPackageSurfaceSnapshot,
    contract: &G3TsAstroSeoIntegrationContractInput,
    checker_executable: &str,
    site: &str,
    output_dir: &str,
) -> bool {
    !script_graph_has_parse_blocker(snapshot, "validate", 0)
        && validate_commands_fail_closed(snapshot)
        && script_has_checker_after_build(
            snapshot,
            contract,
            "validate",
            checker_executable,
            site,
            output_dir,
            0,
        )
}

fn script_graph_has_parse_blocker(
    snapshot: &G3TsAstroPackageSurfaceSnapshot,
    script_name: &str,
    depth: usize,
) -> bool {
    if depth >= 3 {
        return false;
    }
    snapshot
        .script_parse_blockers
        .iter()
        .any(|blocker| blocker.script_name == script_name)
        || snapshot
            .script_tool_invocations
            .iter()
            .filter(|invocation| invocation.script_name == script_name)
            .filter(|invocation| invocation.executable == "package-script")
            .filter_map(|invocation| invocation.args.first())
            .any(|child_script| script_graph_has_parse_blocker(snapshot, child_script, depth + 1))
}

fn invocation_resolves_to_checker(
    snapshot: &G3TsAstroPackageSurfaceSnapshot,
    contract: &G3TsAstroSeoIntegrationContractInput,
    invocation: &G3TsAstroPackageScriptToolInvocation,
    checker_executable: &str,
    site: &str,
    output_dir: &str,
    depth: usize,
) -> bool {
    if invocation.executable == checker_executable {
        return checker_args_valid(invocation, contract, checker_executable, site, output_dir);
    }

    if depth >= 3 || invocation.executable != "package-script" {
        return false;
    }

    let Some(script_name) = invocation.args.first() else {
        return false;
    };

    script_commands_fail_closed(snapshot, script_name)
        && snapshot
            .script_tool_invocations
            .iter()
            .filter(|child| child.script_name == *script_name)
            .any(|child| {
                invocation_has_safe_edges(child)
                    && invocation_resolves_to_checker(
                        snapshot,
                        contract,
                        child,
                        checker_executable,
                        site,
                        output_dir,
                        depth + 1,
                    )
            })
}

fn script_has_checker_after_build(
    snapshot: &G3TsAstroPackageSurfaceSnapshot,
    contract: &G3TsAstroSeoIntegrationContractInput,
    script_name: &str,
    checker_executable: &str,
    site: &str,
    output_dir: &str,
    depth: usize,
) -> bool {
    if depth >= 3 || !script_commands_fail_closed(snapshot, script_name) {
        return false;
    }

    let mut invocations: Vec<&G3TsAstroPackageScriptToolInvocation> = snapshot
        .script_tool_invocations
        .iter()
        .filter(|invocation| invocation.script_name == script_name)
        .collect();
    invocations.sort_by_key(|invocation| invocation.command_index);

    let mut has_prior_build = false;
    for invocation in invocations {
        if !invocation_has_safe_edges(invocation) {
            continue;
        }

        if has_prior_build
            && invocation_resolves_to_checker(
                snapshot,
                contract,
                invocation,
                checker_executable,
                site,
                output_dir,
                depth,
            )
        {
            return true;
        }

        if invocation_resolves_to_checker_after_build(
            snapshot,
            contract,
            invocation,
            checker_executable,
            site,
            output_dir,
            depth + 1,
        ) {
            return true;
        }

        if invocation_resolves_to_astro_build(snapshot, invocation, depth) {
            has_prior_build = true;
        }
    }

    false
}

fn invocation_resolves_to_checker_after_build(
    snapshot: &G3TsAstroPackageSurfaceSnapshot,
    contract: &G3TsAstroSeoIntegrationContractInput,
    invocation: &G3TsAstroPackageScriptToolInvocation,
    checker_executable: &str,
    site: &str,
    output_dir: &str,
    depth: usize,
) -> bool {
    if depth >= 3 || invocation.executable != "package-script" {
        return false;
    }

    let Some(script_name) = invocation.args.first() else {
        return false;
    };

    script_has_checker_after_build(
        snapshot,
        contract,
        script_name,
        checker_executable,
        site,
        output_dir,
        depth,
    )
}

fn checker_args_valid(
    invocation: &G3TsAstroPackageScriptToolInvocation,
    contract: &G3TsAstroSeoIntegrationContractInput,
    checker_executable: &str,
    site: &str,
    output_dir: &str,
) -> bool {
    match checker_executable {
        "g3ts-astro-sitemap-checks" => {
            has_arg_pair(&invocation.args, "--site", site)
                && has_arg_pair(&invocation.args, "--output-dir", output_dir)
        }
        "g3ts-astro-robots-checks" => {
            has_arg_pair(&invocation.args, "--site", site)
                && has_arg_pair(&invocation.args, "--output-dir", output_dir)
                && has_arg_pair(
                    &invocation.args,
                    "--sitemap",
                    &format!("{}/sitemap-index.xml", site.trim_end_matches('/')),
                )
        }
        "g3ts-astro-llms-checks" => {
            has_arg_pair(&invocation.args, "--output-dir", output_dir)
                && configured_llms_args_present(&invocation.args, contract)
        }
        _ => false,
    }
}

fn configured_llms_args_present(
    args: &[String],
    contract: &G3TsAstroSeoIntegrationContractInput,
) -> bool {
    let Some(policy) = crate::support::parsed_seo_policy(&contract.astro_policy) else {
        return false;
    };

    policy
        .llms_required_sections
        .iter()
        .all(|section| has_arg_pair(args, "--required-section", section))
        && policy
            .llms_required_links
            .iter()
            .all(|link| has_arg_pair(args, "--required-link", link))
}

fn has_arg_pair(args: &[String], flag: &str, value: &str) -> bool {
    args.windows(2).any(|pair| {
        pair.first().is_some_and(|arg| arg == flag) && pair.get(1).is_some_and(|arg| arg == value)
    })
}

fn validate_commands_fail_closed(snapshot: &G3TsAstroPackageSurfaceSnapshot) -> bool {
    snapshot.script_commands.iter().all(|command| {
        command.script_name != "validate"
            || command.preceded_by != Some(G3TsAstroPackageScriptCommandSeparator::Or)
    })
}

fn invocation_has_safe_edges(invocation: &G3TsAstroPackageScriptToolInvocation) -> bool {
    invocation.preceded_by != Some(G3TsAstroPackageScriptCommandSeparator::Or)
        && invocation.followed_by != Some(G3TsAstroPackageScriptCommandSeparator::Or)
}

fn invocation_resolves_to_astro_build(
    snapshot: &G3TsAstroPackageSurfaceSnapshot,
    invocation: &G3TsAstroPackageScriptToolInvocation,
    depth: usize,
) -> bool {
    if invocation.executable == "astro" && invocation.args.first().is_some_and(|arg| arg == "build")
    {
        return true;
    }

    if depth >= 3 || invocation.executable != "package-script" {
        return false;
    }

    let Some(script_name) = invocation.args.first() else {
        return false;
    };

    script_commands_fail_closed(snapshot, script_name)
        && snapshot
            .script_tool_invocations
            .iter()
            .filter(|child| child.script_name == *script_name)
            .any(|child| {
                invocation_has_safe_edges(child)
                    && invocation_resolves_to_astro_build(snapshot, child, depth + 1)
            })
}

fn script_commands_fail_closed(
    snapshot: &G3TsAstroPackageSurfaceSnapshot,
    script_name: &str,
) -> bool {
    snapshot.script_commands.iter().all(|command| {
        command.script_name != script_name
            || command.preceded_by != Some(G3TsAstroPackageScriptCommandSeparator::Or)
    })
}
