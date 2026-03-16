use guardrail3::app::rs::validate::release_checks::CrateInfo;
use guardrail3::app::rs::validate::release_crate_checks::{
    check_license, check_readme, check_readme_quality, check_required_string_field, check_version,
};
use guardrail3::domain::report::Severity;

fn parse_toml(content: &str) -> Option<toml::Value> {
    content.parse().ok()
}

fn pkg_from(table: &toml::Value) -> Option<&toml::Value> {
    table.get("package")
}

// --- R-PUB-01 ---

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
fn pub01_neg_no_description() {
    let t = parse_toml("[package]\nname = \"x\"\nversion = \"0.1.0\"").expect("parse"); // reason: test
    let mut r = Vec::new();
    check_required_string_field(pkg_from(&t), "description", "R-PUB-01", "x", None, &mut r);
    assert_eq!(r.len(), 1, "expected one result");
    assert_eq!(
        r.first().map(|c| c.severity),
        Some(Severity::Error),
        "expected Error"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
fn pub01_pos_has_description() {
    let t = parse_toml("[package]\nname = \"x\"\ndescription = \"A crate\"").expect("parse"); // reason: test
    let mut r = Vec::new();
    check_required_string_field(pkg_from(&t), "description", "R-PUB-01", "x", None, &mut r);
    assert_eq!(r.len(), 1, "expected one result");
    assert_eq!(
        r.first().map(|c| c.severity),
        Some(Severity::Info),
        "expected Info"
    );
}

// --- R-PUB-02 ---

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
fn pub02_neg_no_license() {
    let t = parse_toml("[package]\nname = \"x\"").expect("parse"); // reason: test
    let mut r = Vec::new();
    check_license(pkg_from(&t), "x", None, &mut r);
    assert_eq!(
        r.first().map(|c| c.severity),
        Some(Severity::Error),
        "expected Error"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
fn pub02_pos_license() {
    let t = parse_toml("[package]\nname = \"x\"\nlicense = \"MIT\"").expect("parse"); // reason: test
    let mut r = Vec::new();
    check_license(pkg_from(&t), "x", None, &mut r);
    assert_eq!(
        r.first().map(|c| c.severity),
        Some(Severity::Info),
        "expected Info"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
fn pub02_pos_alt_license_file() {
    let t = parse_toml("[package]\nname = \"x\"\nlicense-file = \"LICENSE\"").expect("parse"); // reason: test
    let mut r = Vec::new();
    check_license(pkg_from(&t), "x", None, &mut r);
    assert_eq!(
        r.first().map(|c| c.severity),
        Some(Severity::Info),
        "expected Info"
    );
}

// --- R-PUB-03 ---

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
fn pub03_neg_no_repository() {
    let t = parse_toml("[package]\nname = \"x\"").expect("parse"); // reason: test
    let mut r = Vec::new();
    check_required_string_field(pkg_from(&t), "repository", "R-PUB-03", "x", None, &mut r);
    assert_eq!(
        r.first().map(|c| c.severity),
        Some(Severity::Error),
        "expected Error"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
fn pub03_pos_has_repository() {
    let t = parse_toml("[package]\nname = \"x\"\nrepository = \"https://github.com/x/x\"")
        .expect("parse"); // reason: test
    let mut r = Vec::new();
    check_required_string_field(pkg_from(&t), "repository", "R-PUB-03", "x", None, &mut r);
    assert_eq!(
        r.first().map(|c| c.severity),
        Some(Severity::Info),
        "expected Info"
    );
}

// --- R-PUB-04 ---

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn pub04_neg_no_field_no_file() {
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    let tmp = std::env::temp_dir().join("guardrail3_pub04_neg");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(&tmp);
    let t = parse_toml("[package]\nname = \"x\"").expect("parse"); // reason: test
    let krate = CrateInfo {
        name: "x".to_owned(),
        cargo_toml_path: tmp.join("Cargo.toml"),
        dir: tmp.clone(),
        publishable: true,
        is_binary: false,
        table: t.clone(),
    };
    let mut r = Vec::new();
    check_readme(&fs, pkg_from(&t), &krate, None, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-PUB-04" && c.severity == Severity::Warn),
        "expected Warn"
    );
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn pub04_neg_dangling_readme() {
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    let tmp = std::env::temp_dir().join("guardrail3_pub04_dangle");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(&tmp);
    let t = parse_toml("[package]\nname = \"x\"\nreadme = \"MISSING.md\"").expect("parse"); // reason: test
    let krate = CrateInfo {
        name: "x".to_owned(),
        cargo_toml_path: tmp.join("Cargo.toml"),
        dir: tmp.clone(),
        publishable: true,
        is_binary: false,
        table: t.clone(),
    };
    let mut r = Vec::new();
    check_readme(&fs, pkg_from(&t), &krate, None, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-PUB-04" && c.severity == Severity::Warn),
        "expected Warn"
    );
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn pub04_pos_readme_exists() {
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    let tmp = std::env::temp_dir().join("guardrail3_pub04_pos");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(&tmp);
    let _ = std::fs::write(
        tmp.join("README.md"),
        format!("# My Crate\n\n{}", "x".repeat(200)),
    );
    let t = parse_toml("[package]\nname = \"x\"\nreadme = \"README.md\"").expect("parse"); // reason: test
    let krate = CrateInfo {
        name: "x".to_owned(),
        cargo_toml_path: tmp.join("Cargo.toml"),
        dir: tmp.clone(),
        publishable: true,
        is_binary: false,
        table: t.clone(),
    };
    let mut r = Vec::new();
    check_readme(&fs, pkg_from(&t), &krate, None, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-PUB-04" && c.severity == Severity::Info),
        "expected Info"
    );
    let _ = std::fs::remove_dir_all(&tmp);
}

// --- R-PUB-05 ---

#[test]
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn pub05_neg_small_readme() {
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    let tmp = std::env::temp_dir().join("guardrail3_pub05_small");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(&tmp);
    let _ = std::fs::write(tmp.join("README.md"), "# Hi\nshort");
    let mut r = Vec::new();
    check_readme_quality(&fs, &tmp.join("README.md"), "x", None, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-PUB-05" && c.severity == Severity::Warn),
        "expected Warn"
    );
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn pub05_neg_no_heading() {
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    let tmp = std::env::temp_dir().join("guardrail3_pub05_nohead");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(&tmp);
    let _ = std::fs::write(tmp.join("README.md"), "x".repeat(300));
    let mut r = Vec::new();
    check_readme_quality(&fs, &tmp.join("README.md"), "x", None, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-PUB-05" && c.severity == Severity::Warn),
        "expected Warn"
    );
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn pub05_pos_good_readme() {
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    let tmp = std::env::temp_dir().join("guardrail3_pub05_good");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(&tmp);
    let _ = std::fs::write(
        tmp.join("README.md"),
        format!("# My Crate\n\n{}", "content ".repeat(50)),
    );
    let mut r = Vec::new();
    check_readme_quality(&fs, &tmp.join("README.md"), "x", None, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-PUB-05" && c.severity == Severity::Info),
        "expected Info"
    );
    let _ = std::fs::remove_dir_all(&tmp);
}

// --- R-PUB-08 ---

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
fn pub08_neg_invalid() {
    let t = parse_toml("[package]\nname = \"x\"\nversion = \"not-a-version\"").expect("parse"); // reason: test
    let mut r = Vec::new();
    check_version(pkg_from(&t), "x", None, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-PUB-08" && c.severity == Severity::Error),
        "expected Error"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
fn pub08_pos_valid() {
    let t = parse_toml("[package]\nname = \"x\"\nversion = \"1.2.3\"").expect("parse"); // reason: test
    let mut r = Vec::new();
    check_version(pkg_from(&t), "x", None, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-PUB-08" && c.severity == Severity::Info),
        "expected Info"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
fn pub08_pos_prerelease() {
    let t = parse_toml("[package]\nname = \"x\"\nversion = \"1.2.3-beta.1\"").expect("parse"); // reason: test
    let mut r = Vec::new();
    check_version(pkg_from(&t), "x", None, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-PUB-08" && c.severity == Severity::Info),
        "expected Info"
    );
}
