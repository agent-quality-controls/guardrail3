use std::collections::BTreeSet;

use guardrail3_domain_modules::deny::build_deny_toml;
use guardrail3_domain_report::Severity;

use super::super::super::test_support::{copy_fixture, run_family, write_file};

#[test]
fn local_unknown_keys_only_warn_for_the_owned_local_root() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "deny.toml",
        &build_deny_toml("service", "", "", ""),
    );
    let local_deny = build_deny_toml("service", "", "", "")
        .replace("[bans]\n", "[bans]\nextra-ban-flag = true\n")
        .replace("[sources]\n", "[sources]\nextra-source-flag = true\n");
    write_file(tmp.path(), "apps/devctl/deny.toml", &local_deny);

    let results = run_family(tmp.path());
    let unknown_results = results
        .iter()
        .filter(|result| result.id == "RS-DENY-28")
        .collect::<Vec<_>>();

    let actual = unknown_results
        .iter()
        .map(|result| {
            (
                result.title.clone(),
                result.message.clone(),
                result.file.clone(),
            )
        })
        .collect::<BTreeSet<_>>();
    let expected = BTreeSet::from([
        (
            "unknown bans key".to_owned(),
            "`apps/devctl/deny.toml` uses unknown `[bans].extra-ban-flag`.".to_owned(),
            Some("apps/devctl/deny.toml".to_owned()),
        ),
        (
            "unknown sources key".to_owned(),
            "`apps/devctl/deny.toml` uses unknown `[sources].extra-source-flag`.".to_owned(),
            Some("apps/devctl/deny.toml".to_owned()),
        ),
    ]);

    assert_eq!(actual, expected);
    assert!(
        unknown_results
            .iter()
            .all(|result| result.severity == Severity::Warn && !result.inventory),
        "{unknown_results:#?}"
    );
}
