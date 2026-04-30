use g3ts_hooks_types::G3TsHooksSourceChecksInput;
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};

pub(crate) fn any_script_command(
    inputs: &[G3TsHooksSourceChecksInput],
    predicate: impl Fn(&ResolvedCommand) -> bool + Copy,
) -> bool {
    inputs
        .iter()
        .any(|input| any_resolved_command(input.parsed(), predicate))
}

pub(crate) fn app_roots(input: &G3TsHooksSourceChecksInput) -> Vec<String> {
    if input.app_package_roots().is_empty() {
        return vec![".".to_owned()];
    }
    input.app_package_roots().to_vec()
}

pub(crate) fn is_g3ts_validate_path_command(command: &ResolvedCommand, app_root: &str) -> bool {
    if command.command_name() != "g3ts" {
        return false;
    }
    let args = command.args();
    args.first().is_some_and(|arg| arg == "validate")
        && !args.iter().any(|arg| arg == "--family")
        && path_arg_matches(args, app_root)
}

fn path_arg_matches(args: &[String], app_root: &str) -> bool {
    args.iter().enumerate().any(|(index, arg)| {
        let path = if arg == "--path" {
            args.get(index.saturating_add(1)).map(String::as_str)
        } else {
            arg.strip_prefix("--path=")
        };
        path.is_some_and(|path| path_matches_app_root(path, app_root))
    })
}

fn path_matches_app_root(path: &str, app_root: &str) -> bool {
    path == app_root
        || (app_root == "." && path == ".")
        || path.strip_prefix("./").is_some_and(|path| path == app_root)
        || path.ends_with(format!("/{app_root}").as_str())
}

pub(crate) fn is_app_validate_command(command: &ResolvedCommand, app_root: &str) -> bool {
    match command.command_name() {
        "pnpm" => pnpm_validate(command.args(), app_root),
        "npm" | "bun" => app_root == "." && run_validate(command.args()),
        "yarn" => app_root == "." && yarn_validate(command.args()),
        _ => false,
    }
}

fn pnpm_validate(args: &[String], app_root: &str) -> bool {
    let mut index = 0usize;
    let mut filter_matches = app_root == ".";
    while let Some(arg) = args.get(index).map(String::as_str) {
        match arg {
            "--filter" | "-F" => {
                filter_matches = args
                    .get(index.saturating_add(1))
                    .is_some_and(|filter| filter_matches_app_root(filter, app_root));
                index = index.saturating_add(2);
            }
            "--dir" | "-C" => index = index.saturating_add(2),
            "run" => {
                return filter_matches
                    && args
                        .get(index.saturating_add(1))
                        .is_some_and(|arg| arg == "validate");
            }
            "validate" => return filter_matches,
            _ if arg.starts_with("--filter=") => {
                filter_matches = arg
                    .strip_prefix("--filter=")
                    .is_some_and(|filter| filter_matches_app_root(filter, app_root));
                index = index.saturating_add(1);
            }
            _ if arg.starts_with("-F=") => {
                filter_matches = arg
                    .strip_prefix("-F=")
                    .is_some_and(|filter| filter_matches_app_root(filter, app_root));
                index = index.saturating_add(1);
            }
            _ => index = index.saturating_add(1),
        }
    }
    false
}

fn filter_matches_app_root(filter: &str, app_root: &str) -> bool {
    filter == app_root
        || app_root
            .rsplit('/')
            .next()
            .is_some_and(|name| filter == name)
        || filter.ends_with(format!("/{app_root}").as_str())
}

pub(crate) fn command_mentions_guardrail_ts_config(command: &ResolvedCommand) -> bool {
    !matches!(command.command_name(), "echo" | "printf")
        && command
            .tokens()
            .iter()
            .any(|token| token.contains("guardrail3-ts.toml"))
}

pub(crate) fn command_mentions_pattern(command: &ResolvedCommand, pattern: &str) -> bool {
    !matches!(command.command_name(), "echo" | "printf")
        && command.tokens().iter().any(|token| token.contains(pattern))
}

fn run_validate(args: &[String]) -> bool {
    args.windows(2).any(|window| {
        window.first() == Some(&"run".to_owned()) && window.get(1) == Some(&"validate".to_owned())
    })
}

fn yarn_validate(args: &[String]) -> bool {
    args.first().is_some_and(|arg| arg == "validate") || run_validate(args)
}
