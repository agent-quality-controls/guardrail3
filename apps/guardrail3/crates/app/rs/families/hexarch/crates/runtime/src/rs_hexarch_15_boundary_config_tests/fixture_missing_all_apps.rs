use std::collections::BTreeSet;

use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_15_boundary_config as assertions;
use crate::test_support::copy_fixture;

#[test]
fn missing_all_app_boundaries_hits_each_app_boundary() {
    let tmp = copy_fixture();
    let guardrail_path = tmp.path().join("guardrail3.toml");
    let guardrail = std::fs::read_to_string(&guardrail_path).expect("read guardrail config");
    let updated = guardrail
        .replace(
            "\n[rust.apps.backend]\ntype = \"service\"\n\n[rust.apps.backend.checks]\nhexarch = true\ngarde = false\ntest = true\nrelease = false\n",
            "\n",
        )
        .replace(
            "\n[rust.apps.worker]\ntype = \"service\"\n\n[rust.apps.worker.checks]\nhexarch = true\ngarde = false\ntest = true\nrelease = false\n",
            "\n",
        )
        .replace(
            "\n[rust.apps.devctl]\ntype = \"service\"\n\n[rust.apps.devctl.checks]\nhexarch = true\ngarde = false\ntest = true\nrelease = false\n",
            "\n",
        );
    std::fs::write(&guardrail_path, updated).expect("remove all app boundary configs");

    let results = assertions::run_family(tmp.path());
    let titles = results
        .iter()
        .filter(|result| result.id == "RS-HEXARCH-15")
        .map(|result| result.title.clone())
        .collect::<BTreeSet<_>>();
    let expected = [
        "app boundary `apps/backend` missing rust.apps config".to_owned(),
        "app boundary `apps/devctl` missing rust.apps config".to_owned(),
        "app boundary `apps/worker` missing rust.apps config".to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        titles, expected,
        "missing app config should be reported for every app boundary: {results:#?}"
    );
}
