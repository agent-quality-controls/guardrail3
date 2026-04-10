# Goal
Prove that each extracted ingestion package only operates within the workspace it is given, and does not widen discovery outside that workspace root or outside the intended workspace scope.

# Approach
1. Audit ingestion runtimes for boundary-widening behavior: root scanning, repo-global assumptions, fallback reads outside crawl entries, or package-specific hardcoded roots.
2. Add failing tests first for any package that widens scope beyond the pointed workspace.
3. Fix the boundary at the ingestion layer, not in checks.
4. Re-run package-local tests and an adversarial workspace-boundary pass.

# Key decisions
- Scope this audit to ingestion packages. Checks packages should already operate on typed inputs and should not touch the filesystem.
- Treat "looks outside workspace" as any discovery/read behavior that depends on repo-global paths or scans siblings not present in the pointed crawl root.
- For mixed families like arch/hexarch, preserve family semantics while still requiring the pointed root to be the only discovery universe.

# Files to modify
- package-local ingestion runtimes and tests where workspace-boundary leaks exist
- plan/worklog files for this audit
