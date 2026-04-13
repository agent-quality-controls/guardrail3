# g3rs-deny-ingestion

Ingestion package for deny checks. Takes a workspace crawl result, selects the
root deny config surface, parses typed config state, resolves deny policy
context, and produces:

- `G3RsDenyConfigChecksInput`
- `G3RsDenyFileTreeChecksInput`

Source ingestion is still intentionally unimplemented.
