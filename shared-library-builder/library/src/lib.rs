mod bzip2_library;
mod freetype_library;
mod png_library;
mod zlib_library;

use crate::freetype_library::FreetypeLibrary;
use shared_library_builder::{GitLocation, LibraryLocation};

pub fn libfreetype(binary_version: Option<impl Into<String>>) -> FreetypeLibrary {
    FreetypeLibrary::default().with_release_location(binary_version.map(|version| {
        LibraryLocation::Git(GitLocation::github("feenkcom", "libfreetype").tag(version))
    }))
}
