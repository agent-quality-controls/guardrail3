pub(super) fn fixture(failures: Vec<crate::support::InputFailureSite>) -> super::super::Fixture {
    super::super::fixture(failures)
}

pub(super) fn failure(rel_path: &str, message: &str) -> crate::support::InputFailureSite {
    crate::support::InputFailureSite {
        rel_path: rel_path.to_owned(),
        message: message.to_owned(),
    }
}
