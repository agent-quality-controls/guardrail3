#!/bin/sh
# normalize.sh — Normalize guardrail3 JSON output for golden comparison.
#
# Usage: normalize.sh <project_root> < raw.json > normalized.json
#
# Normalization:
#   1. Replace absolute project path with "." in project field and file fields
#   2. Sort results within each section by (id, severity, title, message, file)
#   3. Keep all fields (id, severity, title, message, file, line)

set -eu

PROJECT_ROOT="${1:?Usage: normalize.sh <project_root>}"

# Escape the project root for use in jq string replacement
# jq's gsub uses regex, so we need to escape regex metacharacters
ESCAPED_ROOT=$(printf '%s' "$PROJECT_ROOT" | sed 's/[[\.*^$()+?{|]/\\&/g')

jq --arg root "$ESCAPED_ROOT" '
  # Replace absolute project path with "." everywhere
  .project = (.project | gsub($root; ".")) |
  .sections = [
    .sections[] |
    .results = [
      .results[] |
      .file = (if .file then (.file | gsub($root; ".")) else null end)
    ] |
    # Sort results by id, then severity, then title, then message, then file
    .results |= sort_by([.id, .severity, .title, .message, (.file // "")])
  ] |
  # Remove summary (it is derived from the data and will match if checks match)
  del(.summary)
'
