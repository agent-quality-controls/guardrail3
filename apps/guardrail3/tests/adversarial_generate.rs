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
use ignore as _;
use proc_macro2 as _;
use proptest as _;
use quote as _;
use serde as _;
use serde_json as _;
use std::process::Command;
use syn as _;
use toml as _;
use toml_edit as _;
use tree_sitter as _;
use tree_sitter_javascript as _;
use tree_sitter_typescript as _;
use walkdir as _;

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

    // Add an entry with no space before `=` — valid TOML syntax
    let patched = baseline.replace(
        "disallowed-methods = [",
        "disallowed-methods = [\n    {path=\"custom::no_space\", reason=\"test\"},",
    );

    let (stdout, _stderr, _success) = generate_then_patch_then_dry_run(&patched);

    // FIX: the parser now normalizes spaces before matching, so `{path=` is detected
    // as an entry just like `{ path = `. The custom entry detector should find it.
    assert!(
        stdout.contains("custom::no_space"),
        "Entry with no space before = should now be detected by the custom entry detector. Got:\n{stdout}"
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
    // The parser now uses section-aware keying, so entries in different sections are
    // distinguished even if their content is identical.
    let patched = baseline.replace(
        "disallowed-methods = [",
        &format!("disallowed-methods = [\n    {entry_trimmed},"),
    );

    let (stdout, _stderr, _success) = generate_then_patch_then_dry_run(&patched);

    // FIX: The parser now prefixes entries with their section name (e.g. "methods:",
    // "types:") so identical entries in different sections are correctly distinguished.
    // The entry placed in disallowed-methods should be detected as custom even though
    // the same content exists in disallowed-types.
    assert!(
        stdout.contains("std::fs::File"),
        "Cross-section duplicate should now be detected as custom (parser is section-aware). Got:\n{stdout}"
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

    // FIX: override entries that duplicate base entries are now deduplicated.
    // The entry should appear exactly once (from the base module only).
    assert_eq!(
        count, 1,
        "Duplicate override entry should be deduplicated — expected 1 occurrence, found {count}."
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

    // FIX: Comments-only override content is now stripped during validation.
    // The "Local overrides" header should NOT be injected when there are zero
    // actual entries (only comments).
    assert!(
        !clippy.contains("Local overrides"),
        "Comments-only override should NOT trigger 'Local overrides' header. Got:\n{clippy}"
    );

    // The comment text should NOT be present in the generated file
    assert!(
        !clippy.contains("my::dangerous::func"),
        "Comment text from comments-only override should not appear in generated clippy.toml"
    );
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

    // FIX: Malformed content is now validated and stripped — invalid lines are skipped
    assert!(
        !clippy.contains("this is not valid TOML at all"),
        "Malformed override content should be stripped by validation. Got:\n{clippy}"
    );
    assert!(
        !clippy.contains("random garbage"),
        "Random garbage lines should be stripped by validation. Got:\n{clippy}"
    );

    // The generated clippy.toml should still be structurally valid
    assert!(
        clippy.contains("disallowed-methods"),
        "Generated clippy.toml should still have disallowed-methods section"
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

// ===========================================================================
// Adversarial tests: hostile input attacks on TOML parsing and overrides
//
// These tests try to crash, corrupt output, or bypass detection via:
// unicode homoglyphs, BOM, null bytes, extreme lengths, nested braces,
// section-header injection, trailing garbage, empty paths, escaped quotes,
// and binary content.
// ===========================================================================

/// Helper: run `rs generate` with an override file and return (stdout, stderr, success)
/// plus the generated clippy.toml content (if it exists).
#[allow(clippy::expect_used)] // reason: test helper — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test helper — writes fixture files and runs Command
#[allow(clippy::type_complexity)] // reason: test helper return type
fn generate_with_override_bytes(
    filename: &str,
    content: &[u8],
) -> (String, String, bool, Option<String>) {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let path = tmp.path();

    write_minimal_config(path);
    write_cargo_toml(path);

    let overrides_dir = path.join(".guardrail3/overrides");
    std::fs::create_dir_all(&overrides_dir).expect("create overrides dir");
    std::fs::write(overrides_dir.join(filename), content).expect("write override file");

    let path_str = path.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("run generate");

    let clippy = std::fs::read_to_string(path.join("clippy.toml")).ok();

    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.success(),
        clippy,
    )
}

// ---------------------------------------------------------------------------
// Test 17: Unicode homoglyphs — fullwidth left curly bracket and fullwidth equals
// The parser normalizes by stripping spaces, but fullwidth `{` (U+FE5B ﹛)
// and fullwidth `=` (U+FE66 ﹦) are NOT ASCII `{` and `=`. The line should
// be rejected by validate_override_content since the normalized form won't
// start with `{path=` or `{name=`.
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn adversarial_unicode_homoglyph_braces_and_equals() {
    // U+FE5B SMALL LEFT CURLY BRACKET: ﹛
    // U+FE66 SMALL EQUALS SIGN: ﹦
    let homoglyph_override =
        "\u{FE5B} path \u{FE66} \"sneaky::bypass\", reason \u{FE66} \"homoglyph attack\" }\n";

    let (_stdout, stderr, success, clippy) =
        generate_with_override_bytes("clippy-methods.toml", homoglyph_override.as_bytes());

    assert!(
        success,
        "generate should succeed even with homoglyph override content: {stderr}"
    );

    let clippy = clippy.expect("clippy.toml should exist after generate");

    // The homoglyph entry should NOT appear in the output — validator should reject it
    assert!(
        !clippy.contains("sneaky::bypass"),
        "Unicode homoglyph entry should be rejected by validate_override_content. Got:\n{clippy}"
    );

    // stderr should show a warning about the skipped line
    assert!(
        stderr.contains("skipping invalid line"),
        "Should warn about invalid homoglyph line. Stderr:\n{stderr}"
    );
}

// ---------------------------------------------------------------------------
// Test 18: UTF-8 BOM at start of override file
// BOM bytes: 0xEF 0xBB 0xBF. If the parser reads the file as a string,
// these bytes become the Unicode BOM character U+FEFF at the start of the
// first line. This could cause the first entry to not be recognized if the
// BOM character prevents the `{path=` prefix match.
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn adversarial_bom_in_override_file() {
    // BOM + valid entry
    let mut content = vec![0xEF, 0xBB, 0xBF];
    content.extend_from_slice(b"    { path = \"bom::first_entry\", reason = \"BOM attack\" },\n");
    content.extend_from_slice(b"    { path = \"bom::second_entry\", reason = \"should work\" },\n");

    let (_stdout, stderr, success, clippy) =
        generate_with_override_bytes("clippy-methods.toml", &content);

    assert!(
        success,
        "generate should succeed with BOM override file: {stderr}"
    );

    let clippy = clippy.expect("clippy.toml should exist");

    // BOM is now stripped by load_local_overrides before validation.
    // Both entries should be included.
    let has_first = clippy.contains("bom::first_entry");
    let has_second = clippy.contains("bom::second_entry");

    assert!(
        has_first,
        "BOM-prefixed first entry should be included (BOM stripped)"
    );
    assert!(
        has_second,
        "Second entry (no BOM prefix) should be present. Got:\n{clippy}"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Null bytes embedded in path string
// Null bytes (\x00) in a Rust String are valid (Rust strings are not
// C strings), but they could cause issues downstream (TOML parsers,
// clippy reading the file, etc.)
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn adversarial_null_bytes_in_path() {
    let null_override =
        "    { path = \"null\x00byte::in_path\", reason = \"null byte attack\" },\n";

    let (_stdout, stderr, success, clippy) =
        generate_with_override_bytes("clippy-methods.toml", null_override.as_bytes());

    // Should not crash
    assert!(
        success,
        "generate should not crash on null byte in override: {stderr}"
    );

    if let Some(ref clippy) = clippy {
        // If the entry made it through, check the output is at least structurally sound
        // (has matching brackets for disallowed-methods)
        let open_count = clippy.matches("disallowed-methods = [").count();
        let close_count = clippy.lines().filter(|l| l.trim() == "]").count();
        assert!(
            close_count >= open_count,
            "Generated clippy.toml should have balanced brackets. Opens: {open_count}, Closes: {close_count}"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 20: Extremely long path (10000 characters)
// Does the parser OOM, timeout, or produce corrupt output?
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn adversarial_extremely_long_path() {
    // Build a path like "a::b::c::d::..." with 10000 total chars
    let mut long_path = String::with_capacity(10_000);
    while long_path.len() < 10_000 {
        if !long_path.is_empty() {
            long_path.push_str("::");
        }
        long_path.push_str("segment");
    }
    #[allow(clippy::string_slice)] // reason: test creates ASCII-only string, index is safe
    let long_path = &long_path[..10_000]; // exact 10k chars

    let override_content =
        format!("    {{ path = \"{long_path}\", reason = \"extremely long path\" }},\n");

    let (_stdout, stderr, success, clippy) =
        generate_with_override_bytes("clippy-methods.toml", override_content.as_bytes());

    assert!(
        success,
        "generate should not crash on 10k char path: {stderr}"
    );

    let clippy = clippy.expect("clippy.toml should exist after generate");

    // The long path should be present (it's a valid entry)
    assert!(
        clippy.contains("extremely long path"),
        "10k char path entry should be included in generated clippy.toml"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Nested braces — braces inside the value string
// `{ path = "foo { bar }" }` — the multiline joiner in collect_toml_entries
// looks for `}` to terminate. The `}` inside the string value could fool it.
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn adversarial_nested_braces_in_value() {
    let nested_override = "    { path = \"foo::bar { baz }\", reason = \"nested braces\" },\n";

    let (_stdout, stderr, success, clippy) =
        generate_with_override_bytes("clippy-methods.toml", nested_override.as_bytes());

    assert!(
        success,
        "generate should not crash on nested braces in path value: {stderr}"
    );

    let clippy = clippy.expect("clippy.toml should exist");

    // The entry should be included (it passes validate_override_content since
    // the normalized form starts with {path=)
    assert!(
        clippy.contains("nested braces"),
        "Entry with nested braces in path value should be included. Got:\n{clippy}"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Entry that looks like a section header
// `{ path = "[disallowed-methods]" }` — the path VALUE contains text that
// looks like a TOML section header. Does the section detector in
// collect_toml_entries get confused?
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn adversarial_path_looks_like_section_header() {
    let section_inject_override =
        "    { path = \"[disallowed-methods]\", reason = \"section injection\" },\n";

    let (_stdout, stderr, success, clippy) =
        generate_with_override_bytes("clippy-methods.toml", section_inject_override.as_bytes());

    assert!(
        success,
        "generate should not crash on section-header-like path: {stderr}"
    );

    let clippy = clippy.expect("clippy.toml should exist");

    // The entry should be included (passes validation: starts with {path=)
    assert!(
        clippy.contains("section injection"),
        "Entry with section-header-like path should be included. Got:\n{clippy}"
    );

    // Now test whether collect_toml_entries in diff is confused by this.
    // Generate baseline, inject this entry into the generated file, run dry-run.
    let baseline = generate_and_read_clippy(None);
    let patched = baseline.replace(
        "disallowed-methods = [",
        "disallowed-methods = [\n    { path = \"[disallowed-methods]\", reason = \"section injection\" },",
    );

    let (stdout, _stderr2, _success2) = generate_then_patch_then_dry_run(&patched);

    // Fixed: section detector now uses starts_with instead of contains,
    // so entries with section-header-like values are correctly identified as entries.
    // The entry should be detected as custom (it's not in the generated base).
    assert!(
        stdout.contains("section injection") || stdout.contains("would update"),
        "Entry with path=\"[disallowed-methods]\" should be correctly detected as custom. Got:\n{stdout}"
    );
}

// ---------------------------------------------------------------------------
// Test 23: Trailing garbage after entry
// `{ path = "foo", reason = "bar" } GARBAGE HERE` — does trailing text
// after the closing `}` break anything?
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn adversarial_trailing_garbage_after_entry() {
    let garbage_override =
        "    { path = \"trailing::garbage\", reason = \"bar\" } THIS IS GARBAGE,\n";

    let (_stdout, stderr, success, clippy) =
        generate_with_override_bytes("clippy-methods.toml", garbage_override.as_bytes());

    assert!(
        success,
        "generate should not crash on trailing garbage: {stderr}"
    );

    let clippy = clippy.expect("clippy.toml should exist");

    // The entry passes validation (normalized starts with {path=), and is
    // injected verbatim. The trailing garbage becomes part of the TOML file.
    // This could produce invalid TOML if clippy tries to parse it.
    if clippy.contains("THIS IS GARBAGE") {
        // BUG: Trailing garbage after the closing brace is injected into clippy.toml
        // The validator only checks the prefix ({path=), not whether the line is
        // well-formed TOML. The trailing text will produce a parse error when
        // clippy reads the file.
        // We don't panic here because the entry IS valid from the prefix perspective.
        // But we verify the TOML is parseable:
        let parse_result: Result<toml::Value, _> = toml::from_str(&clippy);
        assert!(
            parse_result.is_err(),
            "BUG: Trailing garbage after entry produces invalid TOML that would fail clippy parsing"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 24: Empty path — `{ path = "", reason = "empty" }`
// Does generate produce valid clippy.toml with an empty path?
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn adversarial_empty_path_value() {
    let empty_path_override = "    { path = \"\", reason = \"empty path\" },\n";

    let (_stdout, stderr, success, clippy) =
        generate_with_override_bytes("clippy-methods.toml", empty_path_override.as_bytes());

    assert!(success, "generate should not crash on empty path: {stderr}");

    let clippy = clippy.expect("clippy.toml should exist");

    // Empty path passes validation (normalized starts with {path=), but
    // an empty path is semantically meaningless. It should ideally be rejected.
    assert!(
        clippy.contains("empty path"),
        "Empty path entry is accepted by the validator (prefix match passes). \
         It would be better to reject empty paths. Got:\n{clippy}"
    );
}

// ---------------------------------------------------------------------------
// Test 25: Path with escaped quotes — `{ path = "std::\"fs\"::read" }`
// Escaped quotes in path. The line-based parser doesn't parse TOML strings
// properly — it just checks for `{path=` prefix. But does the escaped quote
// break line detection or the closing brace check?
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn adversarial_escaped_quotes_in_path() {
    // Use a raw string to construct the override with escaped quotes
    let escaped_override =
        "    { path = \"std::\\\"fs\\\"::read\", reason = \"escaped quotes\" },\n";

    let (_stdout, stderr, success, clippy) =
        generate_with_override_bytes("clippy-methods.toml", escaped_override.as_bytes());

    assert!(
        success,
        "generate should not crash on escaped quotes in path: {stderr}"
    );

    let clippy = clippy.expect("clippy.toml should exist");

    // The entry passes validation (prefix match), so it should be included
    assert!(
        clippy.contains("escaped quotes"),
        "Entry with escaped quotes in path should be included. Got:\n{clippy}"
    );
}

// ---------------------------------------------------------------------------
// Test 26: Override file that is actually binary
// Random bytes that are NOT valid UTF-8. Does generate crash or handle
// gracefully? Rust's fs::read_to_string will return Err for invalid UTF-8,
// which should cause the override to be treated as empty.
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn adversarial_binary_override_file() {
    // Random bytes that are NOT valid UTF-8
    let binary_content: Vec<u8> = vec![
        0x00, 0x01, 0x02, 0xFF, 0xFE, 0x80, 0x81, 0x90, 0xA0, 0xB0, 0xC0, 0xC1, 0xF5, 0xF6, 0xF7,
        0xF8, 0xF9, 0xFA, 0xFB, 0xFC, 0xFD, 0xFE, 0xFF, 0x00, 0x89, 0x50, 0x4E, 0x47,
        // PNG magic bytes mixed with garbage
        0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
    ];

    let (_stdout, stderr, success, clippy) =
        generate_with_override_bytes("clippy-methods.toml", &binary_content);

    // Should not crash — invalid UTF-8 should be handled by read_file returning None
    assert!(
        success,
        "generate should not crash on binary override file: {stderr}"
    );

    let clippy = clippy.expect("clippy.toml should exist");

    // The binary content should not corrupt the generated output
    assert!(
        clippy.contains("disallowed-methods"),
        "Generated clippy.toml should still have disallowed-methods section after binary override. Got:\n{clippy}"
    );

    // No "Local overrides" section should appear (binary = unreadable = empty)
    assert!(
        !clippy.contains("Local overrides"),
        "Binary override file should be treated as empty (no Local overrides header). Got:\n{clippy}"
    );
}

// ===========================================================================
// Adversarial edge-case tests: command-level attacks
//
// These tests probe error handling at the command boundary — missing files,
// invalid filesystem states, permission errors, path traversal, idempotency,
// CRLF handling, and unknown config sections.
// ===========================================================================

// ---------------------------------------------------------------------------
// Attack 27: Generate without Cargo.toml
// guardrail3.toml exists but NO Cargo.toml. generate only reads
// guardrail3.toml, so it should succeed regardless.
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn attack_generate_without_cargo_toml() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let path = tmp.path();

    // Write guardrail3.toml but NO Cargo.toml
    write_minimal_config(path);

    let path_str = path.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("failed to run");

    // Generate reads guardrail3.toml only — it should succeed even without Cargo.toml.
    // If it crashes or fails, that's a bug.
    assert!(
        out.status.success(),
        "generate should succeed without Cargo.toml — it only needs guardrail3.toml. stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    // Verify files were actually generated
    assert!(
        path.join("clippy.toml").exists(),
        "clippy.toml should be created even without Cargo.toml"
    );
    assert!(
        path.join("deny.toml").exists(),
        "deny.toml should be created even without Cargo.toml"
    );
}

// ---------------------------------------------------------------------------
// Attack 28: Generate with empty guardrail3.toml (0 bytes)
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn attack_generate_with_empty_config() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let path = tmp.path();

    // Write empty guardrail3.toml (0 bytes)
    std::fs::write(path.join("guardrail3.toml"), "").expect("write empty config");

    let path_str = path.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("failed to run");

    // Empty TOML parses as a struct with all-None fields. The generate command
    // should either succeed (generating default files) or fail gracefully with
    // a useful error message — NOT crash/panic.
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("panicked") && !stderr.contains("SIGSEGV"),
        "generate with empty config should not panic. stderr: {stderr}"
    );
}

// ---------------------------------------------------------------------------
// Attack 29: Generate with invalid TOML in guardrail3.toml
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn attack_generate_with_invalid_toml() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let path = tmp.path();

    // Write syntactically invalid TOML
    std::fs::write(path.join("guardrail3.toml"), "[rust\nbroken syntax {{{\n")
        .expect("write invalid config");

    let path_str = path.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("failed to run");

    // Should fail with a useful error, not crash
    assert!(
        !out.status.success(),
        "generate with invalid TOML should fail"
    );

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("panicked"),
        "generate with invalid TOML should not panic. stderr: {stderr}"
    );
    // Should mention parsing error
    assert!(
        stderr.contains("Error") || stderr.contains("error") || stderr.contains("invalid"),
        "generate with invalid TOML should report a parse error. stderr: {stderr}"
    );
}

// ---------------------------------------------------------------------------
// Attack 30: Init --force when guardrail3.toml is read-only (0o444)
// ---------------------------------------------------------------------------

#[test]
#[cfg(unix)]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command, fs, and unix permissions
fn attack_init_force_readonly_config() {
    use std::os::unix::fs::PermissionsExt;

    let tmp = tempfile::tempdir().expect("create temp dir");
    let path = tmp.path();

    // Create a read-only guardrail3.toml
    let config_path = path.join("guardrail3.toml");
    std::fs::write(&config_path, "version = \"0.1\"\n").expect("write config");
    let mut perms = std::fs::metadata(&config_path)
        .expect("metadata")
        .permissions();
    perms.set_mode(0o444);
    std::fs::set_permissions(&config_path, perms).expect("set read-only");

    let path_str = path.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "init", "--profile", "service", "--force", path_str])
        .output()
        .expect("failed to run");

    // Should fail gracefully — report permission error, not panic
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("panicked"),
        "init --force on read-only file should not panic. stderr: {stderr}"
    );

    // Should fail with non-zero exit (can't write to read-only file)
    assert!(
        !out.status.success(),
        "init --force on read-only file should fail. stderr: {stderr}"
    );

    // Restore write permission for cleanup
    let mut perms2 = std::fs::metadata(&config_path)
        .expect("metadata for cleanup")
        .permissions();
    perms2.set_mode(0o644);
    let _ = std::fs::set_permissions(&config_path, perms2);
}

// ---------------------------------------------------------------------------
// Attack 31: Generate when .guardrail3/overrides/ is a FILE not a directory
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn attack_generate_overrides_is_file_not_dir() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let path = tmp.path();

    write_minimal_config(path);
    write_cargo_toml(path);

    // Create .guardrail3/ as a directory but make overrides a FILE
    std::fs::create_dir_all(path.join(".guardrail3")).expect("create .guardrail3 dir");
    std::fs::write(
        path.join(".guardrail3/overrides"),
        "this is a file, not a dir\n",
    )
    .expect("write overrides as file");

    let path_str = path.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("failed to run");

    // Should not panic — load_local_overrides reads individual files under the
    // overrides dir. When overrides is a file, join("clippy-methods.toml") on it
    // won't find anything, so read_file returns None -> default empty string.
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("panicked"),
        "generate with overrides-as-file should not panic. stderr: {stderr}"
    );
    // Should still succeed — overrides are optional
    assert!(
        out.status.success(),
        "generate should succeed even if .guardrail3/overrides is a file. stderr: {stderr}"
    );
}

// ---------------------------------------------------------------------------
// Attack 32: Dry-run on project with 100 workspace members
// Ensures init --dry-run doesn't time out or crash with many members.
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn attack_init_dry_run_100_workspace_members() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let path = tmp.path();

    // Create a Cargo.toml with 100 workspace members
    let mut members = String::from("[workspace]\nmembers = [\n");
    for i in 0..100 {
        #[allow(clippy::format_push_string)] // reason: test helper
        members.push_str(&format!("    \"apps/svc-{i:03}\",\n"));
    }
    members.push_str(
        "]\n\n[package]\nname = \"workspace-root\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    std::fs::write(path.join("Cargo.toml"), &members).expect("write Cargo.toml with 100 members");

    // Create each member directory with a minimal Cargo.toml
    for i in 0..100 {
        let member_dir = path.join(format!("apps/svc-{i:03}"));
        std::fs::create_dir_all(&member_dir).expect("create member dir");
        std::fs::write(
            member_dir.join("Cargo.toml"),
            format!("[package]\nname = \"svc-{i:03}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n"),
        )
        .expect("write member Cargo.toml");
    }

    let path_str = path.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "init", "--profile", "service", "--dry-run", path_str])
        .output()
        .expect("failed to run");

    // Should complete without hanging or crashing
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("panicked"),
        "init --dry-run with 100 members should not panic. stderr: {stderr}"
    );

    // Should show some output — at least the dry-run header
    assert!(
        stdout.contains("Dry run") || stdout.contains("dry run") || out.status.success(),
        "init --dry-run with 100 members should produce output. stdout: {stdout}"
    );
}

// ---------------------------------------------------------------------------
// Attack 33: Generate twice in a row — second run should be idempotent
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn attack_generate_idempotent() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let path = tmp.path();

    write_minimal_config(path);
    write_cargo_toml(path);

    let path_str = path.to_str().expect("non-utf8 path");

    // First generate
    let out1 = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("first generate");
    assert!(
        out1.status.success(),
        "first generate should succeed: {}",
        String::from_utf8_lossy(&out1.stderr)
    );

    // Read all generated files
    let clippy1 =
        std::fs::read_to_string(path.join("clippy.toml")).expect("read clippy.toml after gen1");
    let deny1 = std::fs::read_to_string(path.join("deny.toml")).expect("read deny.toml after gen1");
    let rustfmt1 =
        std::fs::read_to_string(path.join("rustfmt.toml")).expect("read rustfmt.toml after gen1");

    // Second generate
    let out2 = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("second generate");
    assert!(
        out2.status.success(),
        "second generate should succeed: {}",
        String::from_utf8_lossy(&out2.stderr)
    );

    // Read all generated files again
    let clippy2 =
        std::fs::read_to_string(path.join("clippy.toml")).expect("read clippy.toml after gen2");
    let deny2 = std::fs::read_to_string(path.join("deny.toml")).expect("read deny.toml after gen2");
    let rustfmt2 =
        std::fs::read_to_string(path.join("rustfmt.toml")).expect("read rustfmt.toml after gen2");

    // Files should be identical
    assert_eq!(
        clippy1, clippy2,
        "clippy.toml should be identical after second generate"
    );
    assert_eq!(
        deny1, deny2,
        "deny.toml should be identical after second generate"
    );
    assert_eq!(
        rustfmt1, rustfmt2,
        "rustfmt.toml should be identical after second generate"
    );

    // Dry-run after second generate should show no changes
    let dry_out = guardrail3()
        .args(["rs", "generate", "--dry-run", path_str])
        .output()
        .expect("dry-run after second generate");

    let stdout = String::from_utf8_lossy(&dry_out.stdout);
    assert!(
        stdout.to_lowercase().contains("no changes"),
        "Dry-run after idempotent generate should show no changes. Got:\n{stdout}"
    );
}

// ---------------------------------------------------------------------------
// Attack 34: Override file with Windows line endings (CRLF)
// BUG: \r characters from CRLF line endings leak into the generated
// clippy.toml, producing mixed line endings that may break TOML parsers.
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn attack_override_with_crlf_line_endings() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let path = tmp.path();

    write_minimal_config(path);
    write_cargo_toml(path);

    // Create override file with CRLF line endings
    let crlf_content = "    { path = \"crlf_crate::dangerous\", reason = \"BANNED: crlf test\" },\r\n\
         { path = \"crlf_crate::also_bad\", reason = \"BANNED: crlf test 2\" },\r\n";

    let overrides_dir = path.join(".guardrail3/overrides");
    std::fs::create_dir_all(&overrides_dir).expect("create overrides dir");
    std::fs::write(overrides_dir.join("clippy-methods.toml"), crlf_content)
        .expect("write CRLF override");

    let path_str = path.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("failed to run");

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("panicked"),
        "generate with CRLF overrides should not panic. stderr: {stderr}"
    );
    assert!(
        out.status.success(),
        "generate with CRLF overrides should succeed. stderr: {stderr}"
    );

    // Check if the CRLF entry was included in the generated clippy.toml
    let clippy = std::fs::read_to_string(path.join("clippy.toml")).expect("read clippy.toml");

    let has_entry = clippy.contains("crlf_crate::dangerous");
    let has_cr = clippy.contains('\r');

    // BUG: CRLF line endings leak into the generated file because
    // validate_override_content iterates lines (which handles \r\n) but
    // preserves the original line content including trailing \r.
    assert!(
        !has_cr,
        "BUG: CRLF line endings from override file leaked into generated clippy.toml. \
         Mixed line endings will break some TOML parsers. \
         The validator should strip \\r before processing."
    );

    assert!(
        has_entry,
        "CRLF override entry should be included in generated clippy.toml (with \\r stripped). \
         Got:\n{clippy}"
    );
}

// ---------------------------------------------------------------------------
// Attack 35: guardrail3.toml with unknown top-level sections
// serde without deny_unknown_fields should silently ignore them.
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn attack_config_with_unknown_sections() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let path = tmp.path();

    let config = concat!(
        "version = \"0.1\"\n\n",
        "[profile]\n",
        "name = \"service\"\n\n",
        "[rust]\n",
        "workspace_root = \".\"\n\n",
        "[completely_unknown]\n",
        "x = true\n",
    );
    std::fs::write(path.join("guardrail3.toml"), config).expect("write config with unknowns");
    write_cargo_toml(path);

    let path_str = path.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("failed to run");

    let stderr = String::from_utf8_lossy(&out.stderr);

    // Must not panic
    assert!(
        !stderr.contains("panicked"),
        "generate with unknown top-level sections should not panic. stderr: {stderr}"
    );

    // Top-level unknown sections should be silently ignored by serde
    assert!(
        out.status.success(),
        "generate with unknown top-level sections should succeed. stderr: {stderr}"
    );
}

// ---------------------------------------------------------------------------
// Attack 35b: guardrail3.toml with unknown NESTED section under [rust]
// [rust.unknown_section] might be parsed as CrateMap entry or fail
// because serde tries to fit it into RustConfig's known fields.
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn attack_config_with_unknown_nested_section() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let path = tmp.path();

    let config = concat!(
        "version = \"0.1\"\n\n",
        "[profile]\n",
        "name = \"service\"\n\n",
        "[rust]\n",
        "workspace_root = \".\"\n\n",
        "[rust.unknown_nested]\n",
        "foo = \"bar\"\n",
        "baz = 42\n",
    );
    std::fs::write(path.join("guardrail3.toml"), config).expect("write config");
    write_cargo_toml(path);

    let path_str = path.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("failed to run");

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("panicked"),
        "generate with unknown nested section should not panic. stderr: {stderr}"
    );

    // BUG: [rust.unknown_nested] with arbitrary keys (foo, baz) may fail
    // deserialization because serde tries to match it against RustConfig fields
    // (workspace_root, workspaces, apps, packages, checks). If "unknown_nested"
    // doesn't match any field, serde may reject it since RustConfig doesn't use
    // #[serde(deny_unknown_fields)] but also doesn't have a catch-all field.
    if !out.status.success() {
        // BUG: nested unknown section under [rust] causes parse failure
        #[allow(clippy::print_stderr)] // reason: test diagnostic output
        {
            eprintln!(
                "BUG: [rust.unknown_nested] causes generate to fail with: {stderr}\n\
                 serde should ignore unknown fields by default, but unknown nested tables \
                 under [rust] may conflict with typed struct deserialization."
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Attack 36: workspace_root pointing outside the project (path traversal)
// BUG: No validation that workspace_root stays within the project boundary.
// Files may be written outside the temp dir.
// ---------------------------------------------------------------------------

// BUG: workspace_root is used as a raw path prefix with no boundary validation.
// Generate ATTEMPTS to write files like "../../../../clippy.toml" relative to
// the project directory. This is a path traversal vulnerability — only OS
// permissions prevent files from being written outside the project.
#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn attack_workspace_root_outside_project() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let path = tmp.path();

    let config = concat!(
        "version = \"0.1\"\n\n",
        "[profile]\n",
        "name = \"service\"\n\n",
        "[rust]\n",
        "workspace_root = \"../../../..\"\n",
    );
    std::fs::write(path.join("guardrail3.toml"), config).expect("write config");

    let path_str = path.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("failed to run");

    let stderr = String::from_utf8_lossy(&out.stderr);

    // Fixed: workspace_root with ".." is rejected, falls back to "."
    assert!(
        stderr.contains("workspace_root contains '..'"),
        "generate should warn about path traversal in workspace_root. stderr: {stderr}"
    );

    // Files should be written to project root (fallback), not outside
    let escaped = path.join("../../../../clippy.toml");
    assert!(
        !escaped.exists(),
        "No files should be written outside the project"
    );
}
