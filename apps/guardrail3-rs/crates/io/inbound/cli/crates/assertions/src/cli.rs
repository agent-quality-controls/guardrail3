/// Checks that the parsed CLI command matches the expected validate payload.
///
/// # Panics
///
/// Panics if parsing fails or if the parsed command does not match the expected validate payload.
pub fn assert_validate_command_from<I, T>(
    args: I,
    expected_path: &str,
    expected_family: &[&str],
    expected_inventory: bool,
) where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let command = guardrail3_rs::parse_command_from(args);
    assert!(
        command.is_ok(),
        "CLI parsing through the shared crate should succeed: {command:?}"
    );
    let Ok(command) = command else {
        return;
    };

    match command {
        guardrail3_rs::Command::Validate {
            command:
                guardrail3_rs::ValidateCommand::Workspace {
                    path,
                    family,
                    inventory,
                    staged: _,
                    rules_only: _,
                },
        } => {
            let actual_family = family
                .iter()
                .map(|value| {
                    let possible_value = clap::ValueEnum::to_possible_value(value);
                    assert!(
                        possible_value.is_some(),
                        "family arg should always have a clap name"
                    );
                    possible_value
                        .map_or_else(String::new, |resolved| resolved.get_name().to_owned())
                })
                .collect::<Vec<_>>();
            assert_eq!(
                path,
                std::path::PathBuf::from(expected_path),
                "parsed validate path should match the expected workspace root"
            );
            assert_eq!(
                actual_family,
                expected_family
                    .iter()
                    .map(|value| (*value).to_owned())
                    .collect::<Vec<_>>(),
                "parsed family list should match the expected CLI family order"
            );
            assert_eq!(
                inventory, expected_inventory,
                "parsed inventory flag should match the expected CLI value"
            );
        }
        other @ (guardrail3_rs::Command::Init { .. } | guardrail3_rs::Command::Validate { .. }) => {
            unreachable!("expected validate workspace command, got {other:?}");
        }
    }
}

/// Checks that a CLI parse error mentions the rejected value.
///
/// # Panics
///
/// Panics if the rendered parse error does not mention the rejected value.
pub fn assert_parse_error_mentions(error: &clap::Error, needle: &str) {
    let rendered = error.to_string();
    assert!(rendered.contains(needle), "{rendered}");
}
