# Worklog - 2026-04-09 code ledger stale text fix

Summary:
- Updated the RS-CODE ledger wording in `.plans/todo/checks/rs/code.md` to match the current extracted and legacy behavior for the stale rows called out by the recent attack rounds.

Decisions made:
- Removed the stale count claim from the file title so the ledger no longer advertises an obsolete rule total.
- Reworded RS-CODE-24 to keep the rule in the extracted code-lane ledger while acknowledging legacy drift without claiming ownership moved elsewhere.
- Reworded RS-CODE-25/26/27 to say silent placeholder, moved-redundant, and not migration target.
- Reworded RS-CODE-29 to remove the stale library-profile-only restriction.
- Reworded RS-CODE-31 to reflect the warn/error threshold behavior.
- Reworded RS-CODE-33 to cover public free functions, public trait methods, and public methods on public types.

Key files for context:
- `.plans/todo/checks/rs/code.md`

Next steps:
- If any further attack round finds ledger drift, update the same file again and keep the wording aligned with the checked behavior.
