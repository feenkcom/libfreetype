use std::error::Error;

use shared_library_builder::build_standalone;

use libfreetype_library::latest_libfreetype;

fn main() -> Result<(), Box<dyn Error>> {
    build_standalone(|_| Ok(Box::new(latest_libfreetype())))
}
