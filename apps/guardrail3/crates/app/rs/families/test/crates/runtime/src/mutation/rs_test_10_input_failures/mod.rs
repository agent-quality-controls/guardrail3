mod rule;
pub use rule::check;
pub(crate) use rule::emit_inventory_if_clean;

#[cfg(test)]

mod tests;
