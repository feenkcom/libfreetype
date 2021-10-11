mod bzip2_library;
mod freetype_library;
mod png_library;
mod zlib_library;

use shared_library_builder::{GitLocation, LibraryLocation};
use crate::bzip2_library::BZip2Library;
use crate::freetype_library::FreetypeLibrary;
use crate::png_library::PngLibrary;
use crate::zlib_library::ZLibLibrary;

pub fn libfreetype(binary_version: Option<impl Into<String>>) -> FreetypeLibrary {
    FreetypeLibrary::default().with_release_location(binary_version.map(|version| {
        LibraryLocation::Git(GitLocation::github("feenkcom", "libfreetype").tag(version))
    }))
}

pub fn libpng() -> PngLibrary {
    PngLibrary::default()
}

pub fn libzlib() -> ZLibLibrary {
    ZLibLibrary::default()
}

pub fn libbzip2() -> BZip2Library {
    BZip2Library::default()
}