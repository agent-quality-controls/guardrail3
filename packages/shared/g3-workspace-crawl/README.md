# g3-workspace-crawl

Shared per-workspace filesystem crawl for guardrail3 pipelines.

This package owns only neutral workspace crawl semantics:

- path enumeration under one explicit workspace root
- `.gitignore` state
- basic file kind and readability facts
- simple path/file queries over the crawl result

It does not know:

- family semantics
- rule ownership
- legality or overlapping-root precedence
- config coverage

Downstream ingestion packages consume its crawl output and decide which files
they need for their own family-specific parsing.
