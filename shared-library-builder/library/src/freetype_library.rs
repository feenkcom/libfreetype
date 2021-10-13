use crate::{BZip2Library, PngLibrary, ZLibLibrary};
use shared_library_builder::{
    CMakeLibrary, CompiledLibraryName, GitLocation, Library, LibraryLocation,
};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FreetypeLibrary(CMakeLibrary);

impl FreetypeLibrary {
    pub fn v2_10_4() -> Self {
        Self(
            CMakeLibrary::new(
                "freetype",
                LibraryLocation::Git(GitLocation::github("freetype", "freetype").tag("VER-2-10-4")),
            )
            .depends(PngLibrary::v1_6_37().into())
            .depends(ZLibLibrary::v1_2_11().into())
            .depends(BZip2Library::latest().into())
            .define_common("FT_REQUIRE_ZLIB", "ON")
            .define_common("FT_REQUIRE_PNG", "ON")
            .define_common("FT_REQUIRE_BZIP2", "ON")
            .define_common("FT_WITH_HARFBUZZ", "OFF")
            .define_common("CMAKE_DISABLE_FIND_PACKAGE_HarfBuzz", "TRUE")
            .define_shared("BUILD_SHARED_LIBS", "ON")
            .define_static("BUILD_SHARED_LIBS", "OFF")
            .compiled_name(CompiledLibraryName::Matching("freetype".to_string()))
            .with_headers(PathBuf::new().join("include").join("freetype2"))
            .into(),
        )
    }

    pub fn into_cmake_library(self) -> CMakeLibrary {
        self.0
    }
}

impl From<FreetypeLibrary> for Box<dyn Library> {
    fn from(library: FreetypeLibrary) -> Self {
        library.0.into()
    }
}
