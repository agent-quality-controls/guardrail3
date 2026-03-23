# DEPLOY-TS — TypeScript deployment checker (5 rules)

**Input:** railpack-*.json, next.config.*, package.json
**Current code:** `deploy_checks.rs`

## Rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| DEPLOY-TS-01 | D1 | Warn | railpack-*.json config files exist | Implemented |
| DEPLOY-TS-02 | D2 | Error/Warn | `provider` field in each railpack config (Error if Node.js heuristic matches) | Implemented |
| DEPLOY-TS-03 | D3 | Error | Next.js `output: "standalone"` in next.config.* | Implemented |
| DEPLOY-TS-04 | D4 | Warn | `outputFileTracingRoot` in Next.js config (monorepo tracing) | Implemented |
| DEPLOY-TS-05 | D5 | Warn | tailwindcss in dependencies (not devDependencies) for Railway build | Implemented |
