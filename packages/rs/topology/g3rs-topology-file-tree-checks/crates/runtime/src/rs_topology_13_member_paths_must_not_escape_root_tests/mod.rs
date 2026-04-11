use g3rs_topology_file_tree_checks_assertions::has_rule;

use crate::test_support::input;

#[test]
fn escaping_member_path_fires() {
    let input = input(
        "[workspace]\nmembers = [\"crates/api\", \"../shared\"]\n",
        Vec::new(),
        Vec::new(),
    );

    let results = crate::check(&input);

    assert!(has_rule(&results, "RS-TOPOLOGY-13"));
}

#[test]
fn absolute_member_path_fires() {
    let input = input(
        "[workspace]\nmembers = [\"/tmp/shared\"]\n",
        Vec::new(),
        Vec::new(),
    );

    let results = crate::check(&input);

    assert!(has_rule(&results, "RS-TOPOLOGY-13"));
    assert!(results.iter().any(|result| {
        result.id() == "RS-TOPOLOGY-13"
            && result
                .message()
                .contains("not absolute paths or `..` escapes")
    }));
}

#[test]
fn normal_member_path_stays_quiet() {
    let input = input(
        "[workspace]\nmembers = [\"crates/api\"]\n",
        Vec::new(),
        Vec::new(),
    );

    let results = crate::check(&input);

    assert!(!has_rule(&results, "RS-TOPOLOGY-13"));
}
