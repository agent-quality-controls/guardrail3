#[test]
fn parse_command_accepts_family_and_inventory_flags() {
    guardrail3_rs_assertions::cli::assert_validate_command_from(
        [
            "g3rs",
            "validate",
            "workspace",
            "--path",
            ".",
            "--family",
            "fmt",
            "--inventory",
        ],
        ".",
        &["fmt"],
        true,
    );
}

#[test]
fn parse_command_accepts_init_repo() {
    let command =
        super::super::parse_command_from(["g3rs", "init", "repo"]).expect("init repo should parse");

    assert_eq!(
        command,
        super::super::Command::Init {
            command: super::super::InitCommand::Repo {
                path: ".".into(),
                force: false,
            },
        },
    );
}

#[test]
fn parse_command_accepts_init_workspace() {
    let command = super::super::parse_command_from(["g3rs", "init", "workspace", "--path", "."])
        .expect("init workspace should parse");

    assert_eq!(
        command,
        super::super::Command::Init {
            command: super::super::InitCommand::Workspace {
                path: ".".into(),
                force: false,
            },
        },
    );
}

#[test]
fn parse_command_accepts_validate_repo() {
    let command = super::super::parse_command_from(["g3rs", "validate", "repo"])
        .expect("validate repo should parse");

    assert_eq!(
        command,
        super::super::Command::Validate {
            command: super::super::ValidateCommand::Repo {
                path: ".".into(),
                inventory: false,
            },
        },
    );
}

#[test]
fn parse_command_rejects_removed_hexarch_family() {
    let error = super::super::parse_command_from([
        "guardrail3-rs",
        "validate",
        "workspace",
        "--path",
        ".",
        "--family",
        "hexarch",
    ])
    .expect_err("removed family should fail CLI parsing");

    guardrail3_rs_assertions::cli::assert_parse_error_mentions(&error, "hexarch");
}

#[test]
fn parse_command_rejects_old_validate_repo_command() {
    let error = super::super::parse_command_from(["g3rs", "validate-repo"])
        .expect_err("old validate-repo command should fail");

    guardrail3_rs_assertions::cli::assert_parse_error_mentions(&error, "validate-repo");
}

#[test]
fn parse_command_rejects_old_validate_path_shape() {
    let error = super::super::parse_command_from(["g3rs", "validate", "--path", "."])
        .expect_err("old validate --path command should fail");

    guardrail3_rs_assertions::cli::assert_parse_error_mentions(&error, "--path");
}

#[test]
fn parse_command_rejects_init_workspace_profile() {
    let error = super::super::parse_command_from([
        "g3rs",
        "init",
        "workspace",
        "--path",
        ".",
        "--profile",
        "library",
    ])
    .expect_err("init workspace --profile should fail");

    guardrail3_rs_assertions::cli::assert_parse_error_mentions(&error, "--profile");
}
