//! Adversarial integration tests for the generate command.
//!
//! These tests verify:
//! - The rename from `local/` to `.guardrail3/overrides/` works correctly
//! - The `generate --dry-run` shows correct file statuses
//! - The `init` command does NOT create override directories

// Suppress unused crate dependency warnings for crates used only by the main binary
use clap as _;
use colored as _;
use garde as _;
use glob as _;
use guardrail3 as _;
use proc_macro2 as _;
use proptest as _;
use quote as _;
use serde as _;
use serde_json as _;
use syn as _;
use toml as _;
use tree_sitter as _;
use tree_sitter_typescript as _;
use walkdir as _;

use std::process::Command;

#[allow(clippy::disallowed_methods)] // reason: Command::new needed to invoke binary under test
fn guardrail3() -> Command {
    Command::new(env!("CARGO_BIN_EXE_guardrail3"))
}

/// Helper: write a minimal guardrail3.toml with service profile and rust section.
#[allow(clippy::disallowed_methods)] // reason: test helper — writes fixture files to temp dir
#[allow(clippy::expect_used)] // reason: test helper — panics on write failure
fn write_minimal_config(dir: &std::path::Path) {
    std::fs::write(
        dir.join("guardrail3.toml"),
        "version = \"0.1\"\n\n[profile]\nname = \"service\"\n\n[rust]\nworkspace_root = \".\"\n",
    )
    .expect("write guardrail3.toml"); // reason: test setup — panic on failure
}

/// Helper: write a Cargo.toml so generate doesn't complain.
#[allow(clippy::disallowed_methods)] // reason: test helper — writes fixture files to temp dir
#[allow(clippy::expect_used)] // reason: test helper — panics on write failure
fn write_cargo_toml(dir: &std::path::Path) {
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"test-project\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("write Cargo.toml"); // reason: test setup — panic on failure
}

// ---------------------------------------------------------------------------
// Test 1: generate reads overrides from .guardrail3/overrides/ convention path
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn generate_reads_overrides_from_convention_path() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let path = tmp.path();

    write_minimal_config(path);
    write_cargo_toml(path);

    // Create .guardrail3/overrides/ with a custom clippy-methods.toml ban
    let overrides_dir = path.join(".guardrail3/overrides");
    std::fs::create_dir_all(&overrides_dir).expect("create overrides dir");
    std::fs::write(
        overrides_dir.join("clippy-methods.toml"),
        concat!(
            "    # --- custom project ban ---\n",
            "    { path = \"my_crate::dangerous_function\", reason = \"BANNED: test custom ban\" },\n",
        ),
    )
    .expect("write custom clippy-methods.toml");

    let path_str = path.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("failed to run");

    assert!(
        out.status.success(),
        "generate should succeed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    // Read the generated clippy.toml and verify our custom ban is present
    let clippy_content =
        std::fs::read_to_string(path.join("clippy.toml")).expect("read generated clippy.toml");
    assert!(
        clippy_content.contains("my_crate::dangerous_function"),
        "Generated clippy.toml should include the custom ban from .guardrail3/overrides/clippy-methods.toml, got:\n{clippy_content}"
    );
}

// ---------------------------------------------------------------------------
// Test 2: generate ignores the old local/ directory path
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn generate_ignores_old_local_dir() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let path = tmp.path();

    write_minimal_config(path);
    write_cargo_toml(path);

    // Create overrides in the OLD local/ path (should be ignored)
    let old_local_dir = path.join("local");
    std::fs::create_dir_all(&old_local_dir).expect("create old local dir");
    std::fs::write(
        old_local_dir.join("clippy-methods.toml"),
        concat!(
            "    # --- old local ban ---\n",
            "    { path = \"old_crate::should_not_appear\", reason = \"BANNED: old local ban\" },\n",
        ),
    )
    .expect("write old local clippy-methods.toml");

    let path_str = path.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("failed to run");

    assert!(
        out.status.success(),
        "generate should succeed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    // Read the generated clippy.toml and verify the old local/ ban is NOT present
    let clippy_content =
        std::fs::read_to_string(path.join("clippy.toml")).expect("read generated clippy.toml");
    assert!(
        !clippy_content.contains("old_crate::should_not_appear"),
        "Generated clippy.toml should NOT include bans from old local/ path — only .guardrail3/overrides/ is supported"
    );
}

// ---------------------------------------------------------------------------
// Test 3: generate --dry-run shows "would create" for new files
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn generate_dry_run_shows_create_for_new_files() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let path = tmp.path();

    write_minimal_config(path);
    write_cargo_toml(path);

    // Do NOT run generate first — files don't exist yet
    let path_str = path.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", "--dry-run", path_str])
        .output()
        .expect("failed to run");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("would create"),
        "Dry run on fresh project should show 'would create' for missing files, got:\n{stdout}"
    );

    // Verify no files were actually written (dry run should be non-destructive)
    assert!(
        !path.join("clippy.toml").exists(),
        "Dry run should NOT create clippy.toml on disk"
    );
    assert!(
        !path.join("deny.toml").exists(),
        "Dry run should NOT create deny.toml on disk"
    );
}

// ---------------------------------------------------------------------------
// Test 4: generate --dry-run shows "no changes" when files are current
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn generate_dry_run_shows_no_changes_when_current() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let path = tmp.path();

    write_minimal_config(path);
    write_cargo_toml(path);

    let path_str = path.to_str().expect("non-utf8 path");

    // First: run actual generate to create the files
    let gen_out = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("failed to run generate");
    assert!(
        gen_out.status.success(),
        "initial generate should succeed: {}",
        String::from_utf8_lossy(&gen_out.stderr)
    );

    // Second: run dry-run — should report no changes
    let out = guardrail3()
        .args(["rs", "generate", "--dry-run", path_str])
        .output()
        .expect("failed to run dry-run");

    let stdout = String::from_utf8_lossy(&out.stdout);
    let stdout_lower = stdout.to_lowercase();
    assert!(
        stdout_lower.contains("no changes"),
        "Dry run after generate should report 'No changes', got:\n{stdout}"
    );

    // When no changes, exit code should be 0
    assert!(
        out.status.success(),
        "Dry run with no changes should exit 0"
    );
}

// ---------------------------------------------------------------------------
// Test 5: generate --dry-run shows update after tampering
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn generate_dry_run_shows_update_after_tampering() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let path = tmp.path();

    write_minimal_config(path);
    write_cargo_toml(path);

    let path_str = path.to_str().expect("non-utf8 path");

    // Generate files first
    let gen_out = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("failed to run generate");
    assert!(
        gen_out.status.success(),
        "initial generate should succeed: {}",
        String::from_utf8_lossy(&gen_out.stderr)
    );

    // Tamper with clippy.toml
    std::fs::write(path.join("clippy.toml"), "# tampered content\n")
        .expect("tamper with clippy.toml");

    // Dry run should detect the tampering
    let out = guardrail3()
        .args(["rs", "generate", "--dry-run", path_str])
        .output()
        .expect("failed to run dry-run");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("would update") || stdout.contains("would create"),
        "Dry run after tampering should show the file would be updated, got:\n{stdout}"
    );

    // Exit code should be non-zero when changes are needed
    assert!(
        !out.status.success(),
        "Dry run with pending changes should exit non-zero"
    );
}

// ---------------------------------------------------------------------------
// Test 6: init does NOT create .guardrail3/overrides/ directory
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn init_does_not_create_override_files() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let path_str = tmp.path().to_str().expect("non-utf8 path");

    let out = guardrail3()
        .args(["rs", "init", "--profile", "service", path_str])
        .output()
        .expect("failed to run init");

    assert!(
        out.status.success(),
        "init should succeed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    // .guardrail3/overrides/ should NOT exist after init — only generate creates it
    assert!(
        !tmp.path().join(".guardrail3/overrides").exists(),
        ".guardrail3/overrides/ directory should NOT be created by init"
    );

    // Also verify .guardrail3/ directory itself doesn't exist after init
    assert!(
        !tmp.path().join(".guardrail3").exists(),
        ".guardrail3/ directory should NOT be created by init"
    );
}

// ===========================================================================
// Adversarial tests for custom entry detection in diff.rs
//
// The `collect_toml_entries` parser works line-by-line, matching lines that
// start with `{ path =` or `{ name =`. These tests probe its edges.
// ===========================================================================

/// Helper: generate files first, then write a custom clippy.toml, then run dry-run.
/// Returns (stdout, stderr, success).
#[allow(clippy::expect_used)] // reason: test helper — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test helper — writes fixture files and runs Command
fn generate_then_patch_then_dry_run(clippy_content: &str) -> (String, String, bool) {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let path = tmp.path();

    write_minimal_config(path);
    write_cargo_toml(path);

    let path_str = path.to_str().expect("non-utf8 path");

    // Step 1: run actual generate to create baseline files
    let gen_out = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("run generate");
    assert!(
        gen_out.status.success(),
        "baseline generate failed: {}",
        String::from_utf8_lossy(&gen_out.stderr)
    );

    // Step 2: overwrite clippy.toml with adversarial content
    std::fs::write(path.join("clippy.toml"), clippy_content)
        .expect("write adversarial clippy.toml");

    // Step 3: run dry-run
    let out = guardrail3()
        .args(["rs", "generate", "--dry-run", path_str])
        .output()
        .expect("run dry-run");

    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.success(),
    )
}

/// Helper: generate files, then read the generated clippy.toml content.
#[allow(clippy::expect_used)] // reason: test helper — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test helper — writes fixture files and runs Command
#[allow(clippy::type_complexity)] // reason: test helper — tuple option parameter
fn generate_and_read_clippy(overrides_dir_content: Option<(&str, &str)>) -> String {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let path = tmp.path();

    write_minimal_config(path);
    write_cargo_toml(path);

    if let Some((filename, content)) = overrides_dir_content {
        let overrides_dir = path.join(".guardrail3/overrides");
        std::fs::create_dir_all(&overrides_dir).expect("create overrides dir");
        std::fs::write(overrides_dir.join(filename), content).expect("write override file");
    }

    let path_str = path.to_str().expect("non-utf8 path");
    let gen_out = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("run generate");
    assert!(
        gen_out.status.success(),
        "generate failed: {}",
        String::from_utf8_lossy(&gen_out.stderr)
    );

    std::fs::read_to_string(path.join("clippy.toml")).expect("read clippy.toml")
}

// ---------------------------------------------------------------------------
// Test 7: Multiline entry — parser only sees first line
// BUG: this test exposes a parser limitation — multiline entries are not
// recognized as a single entry. The first line IS matched, but the second
// line (continuation) is orphaned. If the generated base has the same entry
// on a single line, the multiline variant won't match it.
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn adversarial_multiline_entry_not_matched_as_single() {
    // Get the generated baseline first
    let baseline = generate_and_read_clippy(None);

    // Take a real entry from the baseline and split it across two lines
    // The parser should still recognize it, but it won't because it's line-based
    let multiline_clippy = baseline.replace(
        "{ path = \"std::env::var\", reason = \"Use the centralized config module -- direct env access scatters configuration and is untestable\" },",
        "{ path = \"std::env::var\",\n      reason = \"Use the centralized config module -- direct env access scatters configuration and is untestable\" },",
    );

    // If content didn't change, the entry format is different — skip
    if multiline_clippy == baseline {
        return;
    }

    let (stdout, _stderr, _success) = generate_then_patch_then_dry_run(&multiline_clippy);

    // BUG: the parser will report `{ path = "std::env::var",` as a "custom entry"
    // because the normalized form (without trailing comma) is `{ path = "std::env::var"`
    // which doesn't match the full single-line entry in the generated base.
    // It SHOULD recognize this as the same entry, just reformatted.
    //
    // The dry-run will show "would update" with a custom entry that isn't actually custom.
    assert!(
        stdout.contains("Custom entries found") || stdout.contains("would update"),
        "Multiline entry should cause the parser to either detect a false custom entry or show update. Got:\n{stdout}"
    );
}

// ---------------------------------------------------------------------------
// Test 8: Commented-out entry — parser should ignore it
// This should PASS: `# { path = ...}` does not start with `{ path =`
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn adversarial_commented_entry_is_ignored() {
    let baseline = generate_and_read_clippy(None);

    // Add a commented-out entry that looks like a real ban
    let patched = baseline.replace(
        "disallowed-methods = [",
        "disallowed-methods = [\n    # { path = \"fake::commented_out\", reason = \"this is commented\" },",
    );

    let (stdout, _stderr, _success) = generate_then_patch_then_dry_run(&patched);

    // The commented entry should NOT appear as a custom entry
    assert!(
        !stdout.contains("fake::commented_out"),
        "Commented-out entry should not be detected as custom. Got:\n{stdout}"
    );
}

// ---------------------------------------------------------------------------
// Test 9: Whitespace variation — `{path=` with no space before equals
// BUG: this test exposes a parser limitation — `{path="foo"}` is valid TOML
// inline table syntax but the parser only matches `{path =` (space before =).
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn adversarial_no_space_before_equals_not_matched() {
    let baseline = generate_and_read_clippy(None);

    // Add an entry with no space before `=` — valid TOML but parser won't match
    let patched = baseline.replace(
        "disallowed-methods = [",
        "disallowed-methods = [\n    {path=\"custom::no_space\", reason=\"test\"},",
    );

    let (stdout, _stderr, _success) = generate_then_patch_then_dry_run(&patched);

    // BUG: the parser won't detect `{path="custom::no_space"...}` as an entry at all
    // because it requires `{path =` (space before equals). The entry will be invisible
    // to the custom entry detector — it shows as a plain diff, not a custom entry.
    assert!(
        !stdout.contains("custom::no_space"),
        "Entry with no space before = should be invisible to custom entry detector (it won't match). Got:\n{stdout}"
    );
}

// ---------------------------------------------------------------------------
// Test 10: Same path, different reason — are they deduplicated?
// The normalized form strips trailing comma but keeps the reason.
// Two entries with same path but different reason should be TWO entries.
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn adversarial_same_path_different_reason_are_distinct() {
    let baseline = generate_and_read_clippy(None);

    // Add two entries with the same path but different reasons
    let patched = baseline.replace(
        "disallowed-methods = [",
        concat!(
            "disallowed-methods = [\n",
            "    { path = \"custom::dup_path\", reason = \"reason alpha\" },\n",
            "    { path = \"custom::dup_path\", reason = \"reason beta\" },",
        ),
    );

    let (stdout, _stderr, _success) = generate_then_patch_then_dry_run(&patched);

    // Both should appear as separate custom entries since the full normalized string differs
    assert!(
        stdout.contains("reason alpha") && stdout.contains("reason beta"),
        "Entries with same path but different reasons should both appear as custom. Got:\n{stdout}"
    );
}

// ---------------------------------------------------------------------------
// Test 11: Entry in disallowed-types section detected same as disallowed-methods
// The parser doesn't distinguish sections — it just looks for `{ path = ...}`
// anywhere in the file. This means a type entry and a method entry with the
// same content are deduplicated across sections.
// BUG: this is a design limitation — cross-section deduplication means if the
// generated base has `{ path = "std::fs::File" }` in disallowed-types, and
// the user adds the exact same text in disallowed-methods, it won't be detected
// as custom.
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn adversarial_cross_section_deduplication() {
    let baseline = generate_and_read_clippy(None);

    // Find the exact std::fs::File entry from the generated baseline.
    // Extract it so we use the exact same reason text.
    let file_entry_line = baseline.lines().find(|l| {
        let t = l.trim();
        t.starts_with("{ path = \"std::fs::File\"")
    });
    let Some(entry_text) = file_entry_line else {
        return; // entry not found in baseline, skip
    };
    let entry_trimmed = entry_text.trim().trim_end_matches(',');

    // Inject the EXACT same entry (same path + same reason) into disallowed-methods.
    // The parser collects entries across ALL sections into a single BTreeSet.
    let patched = baseline.replace(
        "disallowed-methods = [",
        &format!("disallowed-methods = [\n    {entry_trimmed},"),
    );

    let (stdout, _stderr, _success) = generate_then_patch_then_dry_run(&patched);

    // BUG: The parser collects ALL `{ path = ...}` lines regardless of which TOML
    // section they're in. Since this EXACT entry already exists in disallowed-types
    // in the generated base, the BTreeSet deduplicates it — so it won't be flagged
    // as custom even though it's in the WRONG section (methods vs types).
    // The parser is section-unaware — it treats the file as a flat bag of entries.
    assert!(
        !stdout.contains("std::fs::File"),
        "Cross-section exact duplicate is invisible to the parser (design limitation — deduplicates across sections). Got:\n{stdout}"
    );
}

// ---------------------------------------------------------------------------
// Test 12: Override file that duplicates a generated entry — double inclusion
// BUG: if a user puts an entry in .guardrail3/overrides/clippy-methods.toml
// that is already in the generated base modules, it gets included TWICE in
// the final clippy.toml.
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn adversarial_override_duplicates_generated_entry() {
    // Add an override that duplicates a base module entry
    let duplicate_override = concat!(
        "    # --- duplicate of base ---\n",
        "    { path = \"std::env::var\", reason = \"Use the centralized config module -- direct env access scatters configuration and is untestable\" },\n",
    );

    let clippy = generate_and_read_clippy(Some(("clippy-methods.toml", duplicate_override)));

    // Count how many times this exact entry appears
    let count = clippy.matches("{ path = \"std::env::var\"").count();

    // BUG: the entry appears twice — once from the base module, once from the override.
    // generate does not deduplicate overrides against the base. The file is syntactically
    // valid (TOML arrays allow duplicates) but clippy may warn or behave unpredictably.
    assert!(
        count >= 2,
        "Duplicate override entry should appear at least twice in generated output (no dedup). Found {count} occurrences."
    );
}

// ---------------------------------------------------------------------------
// Test 13: Empty override file — should not crash or inject garbage
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn adversarial_empty_override_file() {
    let clippy = generate_and_read_clippy(Some(("clippy-methods.toml", "")));

    // Should produce valid clippy.toml without any "Local overrides" section
    assert!(
        !clippy.contains("Local overrides"),
        "Empty override file should not inject a 'Local overrides' comment. Got:\n{clippy}"
    );

    // Verify the file is not corrupted — should still have disallowed-methods
    assert!(
        clippy.contains("disallowed-methods"),
        "Generated clippy.toml should still have disallowed-methods section"
    );
}

// ---------------------------------------------------------------------------
// Test 14: Override file with only comments — no actual entries
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn adversarial_comments_only_override_file() {
    let comments_only = concat!(
        "    # Example custom ban:\n",
        "    # { path = \"my::dangerous::func\", reason = \"do not use\" },\n",
        "    # Another example:\n",
        "    # { path = \"other::bad::func\", reason = \"also bad\" },\n",
    );

    let clippy = generate_and_read_clippy(Some(("clippy-methods.toml", comments_only)));

    // BUG: The "Local overrides" header and comment block are injected verbatim
    // even though there are ZERO actual entries — just comments. The generator
    // checks `!extra_methods.trim().is_empty()` but comments are non-empty text.
    // This means the generated clippy.toml gets a "Local overrides" section with
    // only commented-out entries, which is misleading.
    assert!(
        clippy.contains("Local overrides"),
        "Comments-only override triggers 'Local overrides' header because content is non-empty after trim."
    );

    // The comment text IS present in the generated file (verbatim injection)
    assert!(
        clippy.contains("# { path = \"my::dangerous::func\""),
        "Comment text from override file should be injected verbatim into generated clippy.toml"
    );

    // However, these commented entries should NOT appear as UNCOMMENTED entries.
    // Every line containing `{ path = "my::dangerous::func"` should be preceded by `#`.
    for line in clippy.lines() {
        if line.contains("my::dangerous::func") {
            assert!(
                line.trim().starts_with('#'),
                "Entry from comments-only override must remain commented. Found uncommented line: {line}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Test 15: Malformed TOML in override — produces invalid clippy.toml
// BUG: override content is injected verbatim with no validation. Broken
// TOML in the override produces a broken clippy.toml that clippy will reject.
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn adversarial_malformed_override_produces_invalid_toml() {
    let malformed = concat!(
        "    this is not valid TOML at all {{{{\n",
        "    { path = unclosed quote\n",
        "    random garbage 12345\n",
    );

    let clippy = generate_and_read_clippy(Some(("clippy-methods.toml", malformed)));

    // The malformed content is injected verbatim — generate does not validate overrides
    assert!(
        clippy.contains("this is not valid TOML at all"),
        "Malformed override content should be injected verbatim (no validation). Got:\n{clippy}"
    );

    // Verify the result is NOT valid TOML
    let parse_result: Result<toml::Value, _> = toml::from_str(&clippy);
    assert!(
        parse_result.is_err(),
        "Generated clippy.toml with malformed override should be invalid TOML. BUG: generate does not validate override content."
    );
}

// ---------------------------------------------------------------------------
// Test 16: Entry with `{ name = ...}` (deny.toml style) in clippy.toml
// The parser matches `{ name = ...}` entries too, even in clippy.toml where
// they don't belong. This is harmless but shows the parser is section-unaware.
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn adversarial_name_entry_in_clippy_detected_as_custom() {
    let baseline = generate_and_read_clippy(None);

    // Inject a `{ name = ...}` entry (deny.toml style) into clippy.toml
    let patched = baseline.replace(
        "disallowed-methods = [",
        "disallowed-methods = [\n    { name = \"fake-crate\", reason = \"wrong file\" },",
    );

    let (stdout, _stderr, _success) = generate_then_patch_then_dry_run(&patched);

    // The parser will detect this as a custom entry because it matches `{ name = ...}`
    // even though `{ name = }` entries belong in deny.toml, not clippy.toml
    assert!(
        stdout.contains("fake-crate"),
        "A `{{ name = }}` entry in clippy.toml should be detected as custom (parser is section-unaware). Got:\n{stdout}"
    );
}
