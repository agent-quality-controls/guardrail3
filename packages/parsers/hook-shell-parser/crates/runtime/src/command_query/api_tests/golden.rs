use hook_shell_parser_runtime_assertions::command_query::api as query_assertions;

#[test]
fn resolves_called_function_with_path_qualified_command() {
    let script = "check_conflicts() {\n    /usr/bin/rg '<<<<<<<' .\n}\ncheck_conflicts\n";

    query_assertions::assert_script_has_resolved_command(
        script,
        "rg",
        "<<<<<<<",
    );
}
