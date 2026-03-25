use guardrail3_domain_modules::deny::build_deny_toml;
use guardrail3_domain_report::Severity;

use super::super::super::test_support::{copy_fixture, run_family, write_file};

#[test]
fn nearest_local_cargo_deny_variant_still_owns_its_workspace_root() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "deny.toml",
        &build_deny_toml("service", "", "", ""),
    );
    write_file(
        tmp.path(),
        "apps/devctl/.cargo/deny.toml",
        &build_deny_toml("service", "", "", ""),
    );

    let results = run_family(tmp.path());
    let coverage = results
        .iter()
        .filter(|result| result.id == "RS-DENY-01")
        .collect::<Vec<_>>();

    let devctl = coverage
        .iter()
        .find(|result| {
            result.message
                == "workspace root `apps/devctl` is covered by `apps/devctl/.cargo/deny.toml`."
        })
        .expect("expected apps/devctl coverage");
    assert_eq!(devctl.severity, Severity::Info);
    assert_eq!(devctl.file.as_deref(), Some("apps/devctl/.cargo/deny.toml"));
    assert!(devctl.inventory);

    let backend = coverage
        .iter()
        .find(|result| result.message == "workspace root `apps/backend` is covered by `deny.toml`.")
        .expect("expected apps/backend coverage");
    assert_eq!(backend.severity, Severity::Info);
    assert_eq!(backend.file.as_deref(), Some("deny.toml"));
}
