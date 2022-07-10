mod bzip2_library;
mod freetype_library;
mod png_library;
mod zlib_library;

pub use shared_library_builder::{CMakeLibrary, GitLocation, LibraryLocation};

pub use crate::bzip2_library::BZip2Library;
pub use crate::freetype_library::FreetypeLibrary;
pub use crate::png_library::PngLibrary;
pub use crate::zlib_library::ZLibLibrary;

pub fn libfreetype(binary_version: Option<impl Into<String>>) -> CMakeLibrary {
    FreetypeLibrary::v2_10_4()
        .into_cmake_library()
        .with_release_location(binary_version.map(|version| {
            LibraryLocation::Git(GitLocation::github("feenkcom", "libfreetype").tag(version))
        }))
}

pub fn latest_libfreetype() -> CMakeLibrary {
    let version: Option<String> = None;
    libfreetype(version)
}

pub fn libpng() -> CMakeLibrary {
    PngLibrary::v1_6_37().into_cmake_library()
}

pub fn libzlib() -> CMakeLibrary {
    ZLibLibrary::v1_2_11().into_cmake_library()
}

pub fn libbzip2() -> CMakeLibrary {
    BZip2Library::latest().into_cmake_library()
}
