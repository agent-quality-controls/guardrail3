use g3rs_topology_file_tree_checks_assertions::has_rule;
use g3rs_topology_types::G3RsTopologyFileTreeInputFailure;

use crate::test_support::input_with_failures;

#[test]
fn descendant_manifest_failure_fires() {
    let input = input_with_failures(
        "[workspace]\nmembers = [\"bad\"]\n",
        Vec::new(),
        Vec::new(),
        vec![G3RsTopologyFileTreeInputFailure {
            rel_path: "bad/Cargo.toml".to_owned(),
            message: "parse failed".to_owned(),
        }],
    );

    let results = crate::check(&input);

    assert!(has_rule(&results, "RS-TOPOLOGY-07"));
    assert!(results.iter().any(|result| {
        result.id() == "RS-TOPOLOGY-07" && result.file() == Some("bad/Cargo.toml")
    }));
}
