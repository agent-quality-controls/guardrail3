use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, run_family};

#[test]
fn missing_one_app_boundary_only_hits_that_app() {
    let tmp = copy_fixture();
    let guardrail_path = tmp.path().join("guardrail3.toml");
    let guardrail = std::fs::read_to_string(&guardrail_path).expect("read guardrail config");
    let updated = guardrail.replace(
        "\n[rust.apps.worker]\ntype = \"service\"\n\n[rust.apps.worker.checks]\narchitecture = true\ngarde = false\ntests = true\nrelease = false\n",
        "\n",
    );
    std::fs::write(&guardrail_path, updated).expect("remove worker boundary config");

    let results = run_family(tmp.path());
    let titles = results
        .iter()
        .filter(|result| result.id == "RS-HEXARCH-15")
        .map(|result| result.title.clone())
        .collect::<BTreeSet<_>>();
    let expected = ["app boundary `apps/worker` missing rust.apps config".to_owned()]
        .into_iter()
        .collect::<BTreeSet<_>>();

    assert_eq!(
        titles, expected,
        "boundary config should only warn for the app whose config entry was removed: {results:#?}"
    );
}
