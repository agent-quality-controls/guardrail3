/// Plain-text rendering for the CLI report output.
mod plain_text;

#[cfg(feature = "api")]
pub use plain_text::PlainTextReportRenderer;
