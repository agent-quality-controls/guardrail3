use super::facts::LibraryPackageFacts;

pub(crate) struct PackageLibarchInput<'a> {
    pub package: &'a LibraryPackageFacts,
}

impl<'a> PackageLibarchInput<'a> {
    pub const fn new(package: &'a LibraryPackageFacts) -> Self {
        Self { package }
    }
}
