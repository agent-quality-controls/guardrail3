#[test]
fn denylist_option_requires_non_empty_denylist() {
    g3ts_style_ingestion_assertions::eslint::assert_denylist_option_rejected(
        super::super::option_has_non_empty_denylist(&serde_json::json!({
            "denyList": []
        })),
    );
}

#[test]
fn denylist_option_rejects_blank_items() {
    g3ts_style_ingestion_assertions::eslint::assert_denylist_option_rejected(
        super::super::option_has_non_empty_denylist(&serde_json::json!({
            "denyList": ["  "]
        })),
    );
}

#[test]
fn denylist_option_accepts_eslint_owned_policy_values() {
    g3ts_style_ingestion_assertions::eslint::assert_denylist_option_accepted(
        super::super::option_has_non_empty_denylist(&serde_json::json!({
            "denyList": ["text-slate-500", "prose"]
        })),
    );
}
