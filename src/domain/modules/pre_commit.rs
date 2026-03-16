use super::Module;

pub const PRE_COMMIT_SCRIPT: Module = Module {
    name: "hooks/pre-commit",
    description: "Pre-commit hook dispatcher script (base — without duplication section)",
    content: PRE_COMMIT_BASE,
};

/// Base pre-commit script — everything except the duplication detection section.
/// The duplication section is appended by the generate command based on profile/config.
pub const PRE_COMMIT_BASE: &str = r#"#!/usr/bin/env bash
set -uo pipefail

# Rust workspace root — override with GUARDRAIL3_RUST_WORKSPACE env var
RUST_WORKSPACE="${GUARDRAIL3_RUST_WORKSPACE:-.}"

# Collect staged files once
STAGED_FILES=$(git diff --cached --name-only --diff-filter=ACM)

if [ -z "$STAGED_FILES" ]; then
    echo "No staged files. Skipping pre-commit checks."
    exit 0
fi

# --- Secret scanning ---
echo "Scanning for secrets..."
if ! command -v gitleaks &> /dev/null; then
    echo "ERROR: gitleaks not installed. Install: brew install gitleaks"
    exit 1
fi
if ! gitleaks protect --staged --no-banner; then
    echo "Secret detected in staged files. Remove it before committing."
    exit 1
fi

# --- File size check ---
echo "Checking file sizes..."
MAX_FILE_SIZE=1048576  # 1MB
while IFS= read -r file; do
    [ -n "$file" ] || continue
    if git cat-file -e ":$file" 2>/dev/null; then
        file_size=$(git cat-file -s ":$file")
        if [ "$file_size" -gt "$MAX_FILE_SIZE" ]; then
            echo "File $file is too large (${file_size} bytes, max ${MAX_FILE_SIZE}). Use Git LFS for large files."
            exit 1
        fi
    fi
done <<< "$STAGED_FILES"

# --- Migration consistency ---
echo "Checking migration consistency..."
MIGRATION_FILES=$(echo "$STAGED_FILES" | grep -E 'drizzle/' || true)
if [ -n "$MIGRATION_FILES" ]; then
    # Check that migration files are not being modified (only new ones allowed)
    MODIFIED_MIGRATIONS=$(git diff --cached --name-only --diff-filter=M | grep -E 'drizzle/' || true)
    if [ -n "$MODIFIED_MIGRATIONS" ]; then
        echo "Error: Existing migration files should not be modified:"
        echo "$MODIFIED_MIGRATIONS"
        echo "Create a new migration instead."
        exit 1
    fi
fi

# Schema changed without migration = reject
SCHEMA_CHANGED=$(echo "$STAGED_FILES" | grep -E 'db/schema/.*\.ts$' || true)
NEW_MIGRATIONS=$(echo "$STAGED_FILES" | grep -E 'drizzle/.*\.sql$' || true)
if [ -n "$SCHEMA_CHANGED" ] && [ -z "$NEW_MIGRATIONS" ]; then
    echo "Error: DB schema changed but no migration file added."
    echo "Run 'pnpm exec drizzle-kit generate' and stage the migration."
    exit 1
fi

# --- Guardrail tamper detection ---
echo "Checking guardrail integrity..."
tamper_fail=0

# 1. eslint-disable without reason
STAGED_TS=$(echo "$STAGED_FILES" | grep -E '\.(ts|tsx)$' || true)
if [ -n "$STAGED_TS" ]; then
    while IFS= read -r file; do
        [ -n "$file" ] || continue
        [ -f "$file" ] || continue
        bad_disables=$(grep -nE 'eslint-disable' "$file" | grep -v -- '-- ' || true)
        if [ -n "$bad_disables" ]; then
            echo "FAIL: $file has eslint-disable without reason (use '-- reason' syntax):"
            echo "$bad_disables"
            tamper_fail=1
        fi
    done <<< "$STAGED_TS"
fi

# 2. Item-level allow without reason comment
ALLOW_PAT='#\[allow('
STAGED_RS=$(echo "$STAGED_FILES" | grep -E '\.rs$' | grep -v '/target/' | grep -v '/tests/' || true)
if [ -n "$STAGED_RS" ]; then
    while IFS= read -r file; do
        [ -n "$file" ] || continue
        [ -f "$file" ] || continue
        bad_allows=$(grep -n "$ALLOW_PAT" "$file" | grep -v '#!\[allow' | grep -v '//' || true)
        if [ -n "$bad_allows" ]; then
            echo "FAIL: $file has item-level allow without a justification comment:"
            echo "$bad_allows"
            tamper_fail=1
        fi
    done <<< "$STAGED_RS"
fi

# 3. Config relaxation detection
GUARDRAIL_FILES=$(echo "$STAGED_FILES" | grep -E '(eslint\.config\.mjs|tsconfig\.base\.json|\.jscpd\.json|Cargo\.toml|clippy\.toml|deny\.toml|rustfmt\.toml)$' || true)
while IFS= read -r gf; do
    [ -n "$gf" ] || continue
    added=$(git diff --cached -- "$gf" | grep '^+' | grep -v '^+++' || true)
    relaxed=$(echo "$added" | grep -iE '"off"|"warn"|= "allow"|"allow"' \
        | grep -v 'priority' \
        | grep -v '// EXCEPTION:' \
        | grep -v '# EXCEPTION:' \
        | grep -v 'wildcards' \
        | grep -v 'allow-wildcard' \
        || true)
    if [ -n "$relaxed" ]; then
        echo "FAIL: $gf appears to relax guardrails (added 'off', 'warn', or 'allow'):"
        echo "$relaxed"
        echo "  If intentional, add '// EXCEPTION: reason' or '# EXCEPTION: reason' comment"
        tamper_fail=1
    fi
done <<< "$GUARDRAIL_FILES"

if [ "$tamper_fail" -ne 0 ]; then
    echo "Guardrail tamper detection failed. Fix suppressions or justify with reason comments."
    exit 1
fi

# --- Detect which stacks changed ---
TS_CHANGED=$(echo "$STAGED_FILES" | grep -cE '\.(ts|tsx|mjs)$' || true)
RUST_CHANGED=$(echo "$STAGED_FILES" | grep -cE '\.(rs)$' || true)
CARGO_CHANGED=$(echo "$STAGED_FILES" | grep -cE '(Cargo\.toml|Cargo\.lock)$' || true)

# --- TypeScript checks (only if TS files changed) ---
if [ "$TS_CHANGED" -gt 0 ]; then
    echo "Running TypeScript type check..."
    for tsconfig in apps/*/tsconfig.json; do
        app_dir=$(dirname "$tsconfig")
        app_name=$(basename "$app_dir")
        if echo "$STAGED_FILES" | grep -q "^${app_dir}/"; then
            echo "  Type-checking ${app_name}..."
            if ! pnpm exec tsc -p "$tsconfig" --noEmit; then
                echo "TypeScript type check failed for ${app_name}. Fix type errors before committing."
                exit 1
            fi
        fi
    done

    echo "Running ESLint..."
    if ! NODE_OPTIONS="--max-old-space-size=8192" pnpm exec eslint --max-warnings 0 $(echo "$STAGED_FILES" | grep -E '\.(ts|tsx|mjs)$'); then
        echo "ESLint failed. Fix lint errors before committing."
        exit 1
    fi
fi

# --- Rust checks (only if Rust or Cargo files changed) ---
if [ "$RUST_CHANGED" -gt 0 ] || [ "$CARGO_CHANGED" -gt 0 ]; then
    echo "Running Rust format check..."
    if ! (cd "$RUST_WORKSPACE" && cargo fmt --all -- --check); then
        echo "Rust formatting check failed. Run 'cd "$RUST_WORKSPACE" && cargo fmt --all' to fix."
        exit 1
    fi

    echo "Running Rust clippy..."
    if ! (cd "$RUST_WORKSPACE" && cargo clippy --workspace --all-targets --all-features -- -D warnings); then
        echo "Clippy failed. Fix errors before committing."
        exit 1
    fi

    echo "Running cargo-deny..."
    if ! command -v cargo-deny &> /dev/null; then
        echo "ERROR: cargo-deny not installed. Install: cargo install cargo-deny"
        exit 1
    fi
    if ! (cd "$RUST_WORKSPACE" && cargo deny check); then
        echo "cargo-deny check failed. Fix dependency issues before committing."
        exit 1
    fi

    # --- Rust structural health checks ---
    echo "Running Rust structural health checks..."
    RS_FILES=$(echo "$STAGED_FILES" | grep '\.rs$' | grep -v '/target/' | grep -v '/tests/' || true)

    if [ -n "$RS_FILES" ]; then
        MAX_RS_LINES=500
        rs_fail=0
        while IFS= read -r file; do
            [ -n "$file" ] || continue
            [ -f "$file" ] || continue
            lines=$(grep -c '' "$file")
            blank=$(grep -c '^[[:space:]]*$' "$file" || echo 0)
            comments=$(grep -c '^[[:space:]]*//' "$file" || echo 0)
            effective=$((lines - blank - comments))
            if [ "$effective" -gt "$MAX_RS_LINES" ]; then
                echo "FAIL: $file ($effective effective lines, max $MAX_RS_LINES)"
                rs_fail=1
            fi
        done <<< "$RS_FILES"

        MAX_USE=20
        while IFS= read -r file; do
            [ -n "$file" ] || continue
            [ -f "$file" ] || continue
            uses=$(grep -cE '^(pub )?use ' "$file" 2>/dev/null) || uses=0
            if [ "$uses" -gt "$MAX_USE" ]; then
                echo "FAIL: $file ($uses use statements, max $MAX_USE)"
                rs_fail=1
            fi
        done <<< "$RS_FILES"

        while IFS= read -r file; do
            [ -n "$file" ] || continue
            [ -f "$file" ] || continue
            CRATE_ALLOW_PAT='#!\[allow'
            matches=$(grep -n "$CRATE_ALLOW_PAT" "$file" | grep -v 'unused_crate_dependencies' || true)
            if [ -n "$matches" ]; then
                echo "FAIL: $file contains crate-wide lint suppression -- do not suppress lints crate-wide"
                echo "$matches"
                rs_fail=1
            fi
        done <<< "$RS_FILES"

        if [ "$rs_fail" -ne 0 ]; then
            echo "Rust structural health checks failed."
            exit 1
        fi
    fi

    echo "Running cargo-machete (unused dependency detection)..."
    if ! command -v cargo-machete &> /dev/null; then
        echo "ERROR: cargo-machete not installed. Install: cargo install cargo-machete"
        exit 1
    fi
    if ! (cd "$RUST_WORKSPACE" && cargo machete); then
        echo "Unused dependencies found. Remove them from Cargo.toml."
        exit 1
    fi

    echo "Running Rust tests..."
    if ! (cd "$RUST_WORKSPACE" && cargo test --workspace); then
        echo "Rust tests failed. Fix tests before committing."
        exit 1
    fi
fi
"#;

/// Duplication detection section for Rust projects using cargo-dupes (AST-aware).
pub const DUPLICATION_CARGO_DUPES: &str = r#"
# === Copy-paste detection (cargo-dupes, AST-aware) ===
if [ "$RUST_CHANGED" -gt 0 ]; then
    echo "Running Rust copy-paste detection..."
    if ! command -v cargo-dupes &> /dev/null; then
        echo "ERROR: cargo-dupes not installed. Install: cargo install cargo-dupes"
        exit 1
    fi
    if ! (cd "$RUST_WORKSPACE" && cargo dupes check --max-exact 0 --max-exact-percent 0 --exclude-tests); then
        echo "Duplicate Rust code detected. Refactor before committing."
        exit 1
    fi
fi
"#;

/// Duplication detection section for TypeScript projects using jscpd.
pub const DUPLICATION_JSCPD: &str = r#"
# === Copy-paste detection (jscpd, TypeScript) ===
if [ "$TS_CHANGED" -gt 0 ]; then
    echo "Running TypeScript copy-paste detection..."
    if ! pnpm exec jscpd --format typescript .; then
        echo "Duplicate TypeScript code detected. Refactor before committing."
        exit 1
    fi
fi
"#;

/// Footer for the pre-commit script (final success message).
pub const PRE_COMMIT_FOOTER: &str = r#"
echo "All pre-commit checks passed."
"#;

/// Build the complete pre-commit script with the appropriate duplication section(s).
/// - Rust-only (service/library without TypeScript): cargo-dupes only
/// - TS-only (no Rust): jscpd only
/// - Mixed (both stacks present): both cargo-dupes and jscpd
pub fn build_pre_commit_script(has_rust: bool, has_typescript: bool) -> String {
    let mut script = PRE_COMMIT_BASE.to_owned();

    if has_rust {
        script.push_str(DUPLICATION_CARGO_DUPES);
    }
    if has_typescript {
        script.push_str(DUPLICATION_JSCPD);
    }

    // If neither stack is detected, include both as a safe default
    if !has_rust && !has_typescript {
        script.push_str(DUPLICATION_CARGO_DUPES);
        script.push_str(DUPLICATION_JSCPD);
    }

    script.push_str(PRE_COMMIT_FOOTER);
    script
}
