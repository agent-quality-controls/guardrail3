# cspell.json

## Location

**Where cspell looks:** Walks UP from the file being checked. Checks for `cspell.json`, `cspell.config.*`, `.cspell.json`, `.cspell.config.*`, `.config/cspell.*`. Nearest wins, no merge. Supports `import` for explicit inheritance.

**In steady-parent:** DOES NOT EXIST.

## Category: Scaffold-once

- If file doesn't exist: create with guardrail3 defaults
- If file exists: LEAVE entirely (user owns it)
- Validate checks existence

## What we scaffold

```json
{
  "$schema": "https://raw.githubusercontent.com/streetsidesoftware/cspell/main/cspell.schema.json",
  "version": "0.2",
  "language": "en",
  "ignorePaths": [
    "node_modules", ".next", "dist", "target", "coverage",
    ".plans", ".worklogs", "drizzle", "*.generated.*",
    ".claude", "pnpm-lock.yaml", "Cargo.lock"
  ],
  "words": []
}
```

## Why scaffold-once (not merge-managed)

cspell.json's main user content is the `words` array — project-specific terms, names, abbreviations. This grows over time as the team adds words. On a real project, the `words` array might have 50-200 entries.

There's nothing in cspell.json that guardrail3 needs to ENFORCE after creation. The `language`, `version`, and `ignorePaths` are reasonable defaults that users rarely change. If they do change them, it's a legitimate project choice.

Merge would risk: accidentally deduplicating user's words, changing ignore patterns the user customized, conflicting with `import` chains if the user uses those.

## Edge cases

1. **Project already has cspell.json with custom words:** Don't touch. Don't scaffold. File exists = user owns it.
2. **cspell.config.yaml or .cspell.json:** Different naming conventions exist. If ANY cspell config exists (check all naming patterns), don't scaffold.
3. **Per-app cspell configs:** cspell walks up from file, so per-app configs shadow the root. Similar to clippy — warn if per-app exists without importing root. But cspell supports `import` natively, so per-app with `"import": ["../../cspell.json"]` is the correct pattern.
