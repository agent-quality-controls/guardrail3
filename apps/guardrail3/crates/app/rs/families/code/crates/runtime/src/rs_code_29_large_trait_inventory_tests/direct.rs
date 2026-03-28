use guardrail3_domain_report::Severity;

use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::rs_code_29_large_trait_inventory::{
    assert_normalized_len, findings,
};

#[test]
fn errors_on_trait_with_thirteen_methods() {
    let mut methods = String::new();
    for index in 0..13 {
        methods.push_str(&format!("    fn m{index}(&self);\n"));
    }
    let content = format!("pub trait Service {{\n{methods}}}");
    let binding = check_source("src/lib.rs", &content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-29");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "large trait surface");
    assert_eq!(
        results[0].message,
        "Trait `Service` has 13 methods (warn above 8, error above 12)."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn warns_on_trait_with_nine_methods() {
    let mut methods = String::new();
    for index in 0..9 {
        methods.push_str(&format!("    fn m{index}(&self);\n"));
    }
    let content = format!("pub trait Service {{\n{methods}}}");
    let binding = check_source("src/lib.rs", &content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-29");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "large trait surface");
    assert_eq!(
        results[0].message,
        "Trait `Service` has 9 methods (warn above 8, error above 12)."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}
