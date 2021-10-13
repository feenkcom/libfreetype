use crate::ZLibLibrary;
use shared_library_builder::{
    CMakeLibrary, CompiledLibraryName, GitLocation, Library, LibraryLocation,
};

#[derive(Debug, Clone)]
pub struct PngLibrary(CMakeLibrary);

impl PngLibrary {
    pub fn v1_6_37() -> Self {
        Self(
            CMakeLibrary::new(
                "png",
                LibraryLocation::Git(GitLocation::github("glennrp", "libpng").tag("v1.6.37")),
            )
            .depends(ZLibLibrary::v1_2_11().into())
            .compiled_name(CompiledLibraryName::Matching("png".to_string()))
            .define_static("PNG_SHARED", "OFF")
            .define_static("PNG_STATIC", "ON")
            .define_shared("PNG_SHARED", "ON")
            .define_shared("PNG_STATIC", "OFF")
            .define_common("PNG_EXECUTABLES", "OFF")
            .define_common("PNG_TESTS", "OFF")
            .define_common("PNG_ARM_NEON", "off")
            .into(),
        )
    }

    pub fn into_cmake_library(self) -> CMakeLibrary {
        self.0
    }
}

impl From<PngLibrary> for Box<dyn Library> {
    fn from(library: PngLibrary) -> Self {
        library.0.into()
    }
}
