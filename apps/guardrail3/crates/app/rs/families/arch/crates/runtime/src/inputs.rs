use super::facts::LibraryPackageFacts;

pub(crate) struct PackageArchInput<'a> {
    pub package: &'a LibraryPackageFacts,
}

impl<'a> PackageArchInput<'a> {
    pub const fn new(package: &'a LibraryPackageFacts) -> Self {
        Self { package }
    }
}
