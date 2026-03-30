use super::{
    EXPECTED_MACRO_BANS, MAX_FN_PARAMS_BOOLS, METHOD_ENV_VARS, TYPE_DYNAMIC, TYPE_GLOBAL_STATE,
    build_clippy_toml, library_profile_methods, library_profile_types, service_profile_methods,
    service_profile_types,
};

#[test]
fn profile_method_sets_stay_stable() {
    let service = service_profile_methods();
    let library = library_profile_methods();

    assert_eq!(service.len(), 7);
    assert_eq!(library.len(), service.len());
    assert_eq!(service[0].name, METHOD_ENV_VARS.name);
}

#[test]
fn library_profile_types_include_global_state_and_service_does_not() {
    let service = service_profile_types();
    let library = library_profile_types();

    assert_eq!(service.len(), 5);
    assert!(
        service
            .iter()
            .any(|module| module.name == TYPE_DYNAMIC.name)
    );
    assert!(
        library
            .iter()
            .any(|module| module.name == TYPE_GLOBAL_STATE.name)
    );
    assert!(
        !service
            .iter()
            .any(|module| module.name == TYPE_GLOBAL_STATE.name)
    );
}

#[test]
fn build_clippy_toml_renders_sections_and_overrides() {
    let rendered = build_clippy_toml(
        "library",
        false,
        true,
        r#"    { path = "custom::method", reason = "custom method override" },"#,
        r#"    { path = "custom::Type", reason = "custom type override" },"#,
    );

    assert!(rendered.contains("# THRESHOLDS"));
    assert!(rendered.contains(&format!("max-fn-params-bools = {MAX_FN_PARAMS_BOOLS}")));
    assert!(rendered.contains("allow-dbg-in-tests = false"));
    assert!(rendered.contains("allow-expect-in-tests = true"));
    assert!(rendered.contains("disallowed-methods = ["));
    assert!(rendered.contains("disallowed-types = ["));
    assert!(rendered.contains("disallowed-macros = ["));
    assert!(rendered.contains("allow-unwrap-in-tests = false"));
    assert!(rendered.contains("custom::method"));
    assert!(rendered.contains("custom::Type"));
    assert!(rendered.contains("std::sync::LazyLock"));
    for macro_name in EXPECTED_MACRO_BANS {
        assert!(rendered.contains(&format!("path = \"{macro_name}\"")));
    }
}

#[test]
fn build_clippy_toml_can_disable_garde_baseline() {
    let rendered = build_clippy_toml("service", false, false, "", "");

    assert!(!rendered.contains("serde_json::from_str"));
    assert!(!rendered.contains("axum::extract::Json"));
}
