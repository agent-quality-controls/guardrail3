# g3rs-deny-config-ingestion

Ingestion package for deny config checks. Takes a workspace crawl result,
selects `deny.toml` / `.deny.toml`, parses it, and produces the input type
for `g3rs-deny-config-checks`.
