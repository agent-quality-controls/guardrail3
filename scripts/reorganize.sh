#!/bin/bash
set -euo pipefail

# Reorganize packages/ into grouped folder structure.
# This script: creates dirs, git mv packages, rewrites all path deps.

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

echo "=== Creating directory structure ==="
mkdir -p packages/rs/cargo
mkdir -p packages/rs/clippy
mkdir -p packages/rs/deny
mkdir -p packages/rs/deps
mkdir -p packages/rs/fmt
mkdir -p packages/rs/garde
mkdir -p packages/rs/release
mkdir -p packages/rs/toolchain
mkdir -p packages/parsers
mkdir -p packages/shared

echo "=== Moving parser packages ==="
git mv packages/cargo-toml-parser packages/parsers/
git mv packages/cargo-config-toml-parser packages/parsers/
git mv packages/cliff-toml-parser packages/parsers/
git mv packages/clippy-toml-parser packages/parsers/
git mv packages/deny-toml-parser packages/parsers/
git mv packages/mutants-toml-parser packages/parsers/
git mv packages/nextest-toml-parser packages/parsers/
git mv packages/release-plz-toml-parser packages/parsers/
git mv packages/rust-toolchain-toml-parser packages/parsers/
git mv packages/rustfmt-toml-parser packages/parsers/
git mv packages/g3rs-toml-parser packages/parsers/

echo "=== Moving shared packages ==="
git mv packages/guardrail3-check-types packages/shared/
git mv packages/reason-policy packages/shared/

echo "=== Moving g3rs workspace crawl ==="
git mv packages/g3rs-workspace-crawl packages/rs/

echo "=== Moving family packages ==="
git mv packages/g3rs-cargo-config-checks packages/rs/cargo/
git mv packages/g3rs-cargo-ingestion packages/rs/cargo/

git mv packages/g3rs-clippy-config-checks packages/rs/clippy/
git mv packages/g3rs-clippy-ingestion packages/rs/clippy/

git mv packages/g3rs-deny-config-checks packages/rs/deny/
git mv packages/g3rs-deny-ingestion packages/rs/deny/

git mv packages/g3rs-deps-config-checks packages/rs/deps/

git mv packages/g3rs-fmt-config-checks packages/rs/fmt/
git mv packages/g3rs-fmt-ingestion packages/rs/fmt/

git mv packages/g3rs-garde-ast-checks packages/rs/garde/
git mv packages/g3rs-garde-config-checks packages/rs/garde/
git mv packages/g3rs-garde-ingestion packages/rs/garde/

git mv packages/g3rs-release-config-checks packages/rs/release/
git mv packages/g3rs-release-ingestion packages/rs/release/

git mv packages/g3rs-toolchain-config-checks packages/rs/toolchain/
git mv packages/g3rs-toolchain-ingestion packages/rs/toolchain/

echo "=== Rewriting path dependencies ==="

# Build a mapping of old package paths to new package paths (relative to repo root)
declare -A MOVES
# Parsers
MOVES[packages/cargo-toml-parser]=packages/parsers/cargo-toml-parser
MOVES[packages/cargo-config-toml-parser]=packages/parsers/cargo-config-toml-parser
MOVES[packages/cliff-toml-parser]=packages/parsers/cliff-toml-parser
MOVES[packages/clippy-toml-parser]=packages/parsers/clippy-toml-parser
MOVES[packages/deny-toml-parser]=packages/parsers/deny-toml-parser
MOVES[packages/mutants-toml-parser]=packages/parsers/mutants-toml-parser
MOVES[packages/nextest-toml-parser]=packages/parsers/nextest-toml-parser
MOVES[packages/release-plz-toml-parser]=packages/parsers/release-plz-toml-parser
MOVES[packages/rust-toolchain-toml-parser]=packages/parsers/rust-toolchain-toml-parser
MOVES[packages/rustfmt-toml-parser]=packages/parsers/rustfmt-toml-parser
MOVES[packages/g3rs-toml-parser]=packages/parsers/g3rs-toml-parser
# Shared
MOVES[packages/guardrail3-check-types]=packages/shared/guardrail3-check-types
MOVES[packages/reason-policy]=packages/shared/reason-policy
# Crawl
MOVES[packages/g3rs-workspace-crawl]=packages/rs/g3rs-workspace-crawl
# Families
MOVES[packages/g3rs-cargo-config-checks]=packages/rs/cargo/g3rs-cargo-config-checks
MOVES[packages/g3rs-cargo-ingestion]=packages/rs/cargo/g3rs-cargo-ingestion
MOVES[packages/g3rs-clippy-config-checks]=packages/rs/clippy/g3rs-clippy-config-checks
MOVES[packages/g3rs-clippy-ingestion]=packages/rs/clippy/g3rs-clippy-ingestion
MOVES[packages/g3rs-deny-config-checks]=packages/rs/deny/g3rs-deny-config-checks
MOVES[packages/g3rs-deny-ingestion]=packages/rs/deny/g3rs-deny-ingestion
MOVES[packages/g3rs-deps-config-checks]=packages/rs/deps/g3rs-deps-config-checks
MOVES[packages/g3rs-fmt-config-checks]=packages/rs/fmt/g3rs-fmt-config-checks
MOVES[packages/g3rs-fmt-ingestion]=packages/rs/fmt/g3rs-fmt-ingestion
MOVES[packages/g3rs-garde-ast-checks]=packages/rs/garde/g3rs-garde-ast-checks
MOVES[packages/g3rs-garde-config-checks]=packages/rs/garde/g3rs-garde-config-checks
MOVES[packages/g3rs-garde-ingestion]=packages/rs/garde/g3rs-garde-ingestion
MOVES[packages/g3rs-release-config-checks]=packages/rs/release/g3rs-release-config-checks
MOVES[packages/g3rs-release-ingestion]=packages/rs/release/g3rs-release-ingestion
MOVES[packages/g3rs-toolchain-config-checks]=packages/rs/toolchain/g3rs-toolchain-config-checks
MOVES[packages/g3rs-toolchain-ingestion]=packages/rs/toolchain/g3rs-toolchain-ingestion

# For each Cargo.toml with path deps, resolve and rewrite
find packages/ apps/ -name 'Cargo.toml' -not -path '*/target/*' | while read -r toml_file; do
  # Get the directory of this Cargo.toml
  toml_dir="$(dirname "$toml_file")"

  # Find all path = "..." references
  grep -n 'path = "' "$toml_file" | while IFS=: read -r line_num line_content; do
    # Extract the path value
    old_rel_path=$(echo "$line_content" | sed 's/.*path = "\([^"]*\)".*/\1/')

    # Skip internal deps (within same package workspace)
    case "$old_rel_path" in
      ../types|../runtime|../assertions|../parser*|crates/*|src/*) continue ;;
    esac

    # Resolve to absolute path
    old_abs="$(cd "$toml_dir" && realpath -m "$old_rel_path" 2>/dev/null || echo "")"
    if [ -z "$old_abs" ]; then continue; fi

    # Make it relative to repo root
    old_repo_rel="${old_abs#$REPO_ROOT/}"

    # Check if this path (or a prefix of it) is in our move map
    matched_old=""
    matched_new=""
    for old_key in "${!MOVES[@]}"; do
      if [[ "$old_repo_rel" == "$old_key"* ]]; then
        # Found a match — compute the suffix after the package root
        suffix="${old_repo_rel#$old_key}"
        matched_old="$old_key"
        matched_new="${MOVES[$old_key]}$suffix"
        break
      fi
    done

    if [ -z "$matched_old" ]; then continue; fi

    # Compute new relative path from this Cargo.toml's directory to the new location
    new_abs="$REPO_ROOT/$matched_new"
    new_rel_path="$(python3 -c "import os.path; print(os.path.relpath('$new_abs', '$REPO_ROOT/$toml_dir'))")"

    if [ "$old_rel_path" != "$new_rel_path" ]; then
      # Escape for sed
      old_escaped=$(echo "$old_rel_path" | sed 's/[\/&]/\\&/g')
      new_escaped=$(echo "$new_rel_path" | sed 's/[\/&]/\\&/g')
      sed -i '' "s|path = \"$old_escaped\"|path = \"$new_escaped\"|g" "$toml_file"
      echo "  $toml_file: $old_rel_path → $new_rel_path"
    fi
  done
done

echo "=== Done ==="
