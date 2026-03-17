# Narrow content auto-detection signals

**Date:** 2026-03-17 19:37
**Scope:** mod.rs

## Summary
Removed weak content detection signals (remark/rehype/shiki/mdx) that could appear in any app (admin with markdown editor). Kept only strong signals: velite, contentlayer, nextra, next-seo, next-sitemap.
