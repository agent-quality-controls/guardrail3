use hook_shell_parser::command_query::{
    ResolvedCommand, any_resolved_command, any_resolved_command_relaxed,
};

pub(crate) fn script_command(
    input: &g3ts_hooks_types::G3TsHooksSourceChecksInput,
    predicate: impl Fn(&ResolvedCommand) -> bool,
) -> bool {
    any_resolved_command(input.parsed(), predicate)
}

pub(crate) fn script_category_command(
    input: &g3ts_hooks_types::G3TsHooksSourceChecksInput,
    predicate: impl Fn(&ResolvedCommand) -> bool,
) -> bool {
    any_resolved_command_relaxed(input.parsed(), &predicate)
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

pub(crate) fn is_g3ts_verify_pre_commit_command(command: &ResolvedCommand, app_roots: &[String]) -> bool {
    if !matches!(
        command.command_path(),
        "scripts/g3ts/verify" | "./scripts/g3ts/verify" | "$REPO_ROOT/scripts/g3ts/verify" | "${REPO_ROOT}/scripts/g3ts/verify"
    ) {
        return false;
    }
    let args = command.args();
    if args
        .iter()
        .any(|arg| arg.starts_with("--mode=") || arg.starts_with("--scope="))
    {
        return false;
    }
    arg_values(args, "--mode").as_slice() == ["pre-commit"]
        && arg_values(args, "--scope").len() == 1
        && arg_value(args, "--scope").is_some_and(|scope| valid_pre_commit_scope(scope, app_roots))
}

pub(crate) fn arg_value<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    arg_values(args, name).into_iter().next()
}

fn arg_values<'a>(args: &'a [String], name: &str) -> Vec<&'a str> {
    args.iter().enumerate().filter_map(|(index, arg)| {
        if arg == name {
            return args.get(index.saturating_add(1)).map(String::as_str);
        }
        None
    }).collect()
}

pub(crate) fn command_has_arg(args: &[String], expected: &str) -> bool {
    args.iter().any(|arg| arg == expected)
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
    path == "$SCOPE"
        || path == "${SCOPE}"
        || path == app_root
        || (app_root == "." && path == ".")
        || path.strip_prefix("./").is_some_and(|path| path == app_root)
        || path == format!("$REPO_ROOT/{app_root}")
        || path == format!("${{REPO_ROOT}}/{app_root}")
}

fn valid_pre_commit_scope(scope: &str, app_roots: &[String]) -> bool {
    app_roots
        .iter()
        .any(|root| pre_commit_scope_matches_app_root(scope, root))
}

fn pre_commit_scope_matches_app_root(scope: &str, app_root: &str) -> bool {
    if scope == "." || scope == "./" || app_root == "." {
        return false;
    }
    scope == app_root
        || scope.strip_prefix("./").is_some_and(|path| path == app_root)
        || scope == format!("$REPO_ROOT/{app_root}")
        || scope == format!("${{REPO_ROOT}}/{app_root}")
}
