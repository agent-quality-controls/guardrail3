use super::{finding, rule_files, run_family, tempdir, write_file};
use guardrail3_domain_report::Severity;

#[test]
fn tokio_dependency_only_root_without_nextest_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ntokio = { version = \"1\", features = [\"macros\", \"rt\"] }\n",
    );
    write_file(
        root,
        "tests/basic.rs",
        "#[test]\nfn runs() { assert!(true); }\n",
    );

    let results = run_family(root);

    assert_eq!(
        rule_files(&results, "RS-TEST-09"),
        vec![".config/nextest.toml".to_owned()]
    );
    let finding = finding(&results, "RS-TEST-09");
    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "nextest config missing");
    assert_eq!(finding.file.as_deref(), Some(".config/nextest.toml"));
    assert_eq!(finding.line, None);
}

#[test]
fn tokio_attr_only_root_without_nextest_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/async.rs",
        "#[tokio::test]\nasync fn runs() { assert!(true); }\n",
    );

    let results = run_family(root);

    assert_eq!(
        rule_files(&results, "RS-TEST-09"),
        vec![".config/nextest.toml".to_owned()]
    );
    let finding = finding(&results, "RS-TEST-09");
    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "nextest config missing");
    assert_eq!(finding.file.as_deref(), Some(".config/nextest.toml"));
    assert_eq!(finding.line, None);
}

#[test]
fn plain_tests_without_tokio_are_inactive_without_nextest() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "tests/basic.rs",
        "#[test]\nfn runs() { assert_eq!(1, 1); }\n",
    );

    let results = run_family(root);

    assert!(rule_files(&results, "RS-TEST-09").is_empty());
}

#[test]
fn missing_slow_timeout_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ntokio = { version = \"1\", features = [\"macros\", \"rt\"] }\n",
    );
    write_file(
        root,
        "tests/async.rs",
        "#[tokio::test]\nasync fn runs() { assert!(true); }\n",
    );
    write_file(
        root,
        ".config/nextest.toml",
        "[profile.default]\nleak-timeout = \"100ms\"\n",
    );

    let results = run_family(root);

    assert_eq!(
        rule_files(&results, "RS-TEST-09"),
        vec![".config/nextest.toml".to_owned()]
    );
    let finding = finding(&results, "RS-TEST-09");
    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "nextest timeouts incomplete");
    assert_eq!(finding.file.as_deref(), Some(".config/nextest.toml"));
    assert_eq!(finding.line, None);
}

#[test]
fn missing_leak_timeout_is_reported() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ntokio = { version = \"1\", features = [\"macros\", \"rt\"] }\n",
    );
    write_file(
        root,
        "tests/async.rs",
        "#[tokio::test]\nasync fn runs() { assert!(true); }\n",
    );
    write_file(
        root,
        ".config/nextest.toml",
        "[profile.default]\nslow-timeout = { period = \"60s\" }\n",
    );

    let results = run_family(root);

    assert_eq!(
        rule_files(&results, "RS-TEST-09"),
        vec![".config/nextest.toml".to_owned()]
    );
    let finding = finding(&results, "RS-TEST-09");
    assert_eq!(finding.severity, Severity::Warn);
    assert_eq!(finding.title, "nextest timeouts incomplete");
    assert_eq!(finding.file.as_deref(), Some(".config/nextest.toml"));
    assert_eq!(finding.line, None);
}
