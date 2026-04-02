# Create FamilyView crate — test attacked and hardened

**Date:** 2026-04-02 12:52

## Summary
New crate guardrail3-app-rs-family-view with FamilyView type. Test attack
found 3 issues, all fixed:
1. root_path() was pub — removed entirely (root is internal to abs_path)
2. abs_path() didn't reject .. traversal — added segment check
3. is_in_scope had unnecessary fallback paths — narrowed to scope_roots only

## Key design
- FamilyView.build() takes full structure/content maps and filters to scope
- abs_path() returns Option — None for out-of-scope or .. paths
- No root path exposed to families
- DirEntry re-exported (harmless data type)
