use shared_library_builder::{
    CMakeLibrary, CompiledLibraryName, GitLocation, Library, LibraryLocation,
};

#[derive(Debug, Clone)]
pub struct BZip2Library(CMakeLibrary);

impl BZip2Library {
    pub fn latest() -> Self {
        Self(
            CMakeLibrary::new(
                "bzip2",
                LibraryLocation::Git(
                    GitLocation::gitlab("federicomenaquintero", "bzip2")
                        .commit("bf905ea2251191ff9911ae7ec0cfc35d41f9f7f6"),
                ),
            )
            .compiled_name(CompiledLibraryName::Matching("bzip2".to_string()))
            .define_common("ENABLE_LIB_ONLY", "ON")
            .define_static("ENABLE_STATIC_LIB", "ON")
            .define_static("BUILD_SHARED_LIBS", "OFF")
            .define_static("ENABLE_SHARED_LIB", "OFF")
            .define_shared("ENABLE_STATIC_LIB", "OFF")
            .define_shared("ENABLE_SHARED_LIB", "ON")
            .define_shared("BUILD_SHARED_LIBS", "ON")
            .into(),
        )
    }

    pub fn into_cmake_library(self) -> CMakeLibrary {
        self.0
    }
}

impl From<BZip2Library> for Box<dyn Library> {
    fn from(library: BZip2Library) -> Self {
        library.0.into()
    }
}
