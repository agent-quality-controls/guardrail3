# Auto-detect TS app type + fix 400-line limit

**Date:** 2026-03-17 19:18
**Task:** Auto-detect service/content/library from app structure, change TS line limit to 400

## Auto-detection logic

In resolve_app_contexts, when no config type is set:

1. Read app's package.json deps
2. Check directory structure
3. Determine type:
   - `src/modules/domain/` exists → Service
   - `velite` in deps OR `content/` dir exists → Content
   - `express`/`fastify`/`hono` in deps → Service
   - None of the above → Service (safe fallback, strictest)
4. Config always wins over auto-detection

## Also update has_content_app to auto-detect
The has_content_app function (for global content checks like stylelint) needs auto-detection too — scan discovered apps for content signals even without config.

## Files
- mod.rs — add auto_detect_app_type function, update resolve_app_contexts and has_content_app
- source_scan.rs — 300→400 line limit (done)
