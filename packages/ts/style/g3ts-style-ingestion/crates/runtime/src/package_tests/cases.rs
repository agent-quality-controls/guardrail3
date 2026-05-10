#[test]
fn package_manager_run_scripts_are_normalized_as_package_script_refs() {
    let fact = super::super::parse_package_script("validate", "pnpm run lint:css");
    let invocations = super::super::script_tool_invocations(&fact);

    let invocation = invocations
        .first()
        .expect("pnpm run lint:css should create one tool invocation");
    g3ts_style_ingestion_assertions::package::assert_tool_invocation(
        invocation,
        "package-script",
        &["lint:css"],
    );
}

#[test]
fn package_manager_script_shorthand_is_normalized_as_script_executable() {
    let fact = super::super::parse_package_script("validate", "pnpm lint:css");
    let invocations = super::super::script_tool_invocations(&fact);

    let invocation = invocations
        .first()
        .expect("pnpm lint:css should create one tool invocation");
    g3ts_style_ingestion_assertions::package::assert_tool_invocation(invocation, "lint:css", &[]);
}

#[test]
fn package_manager_direct_stylelint_is_normalized_to_stylelint_executable() {
    let fact = super::super::parse_package_script("validate", "pnpm stylelint --max-warnings 0");
    let invocations = super::super::script_tool_invocations(&fact);

    let invocation = invocations
        .first()
        .expect("pnpm stylelint should create one tool invocation");
    g3ts_style_ingestion_assertions::package::assert_tool_invocation(
        invocation,
        "stylelint",
        &["--max-warnings", "0"],
    );
}
