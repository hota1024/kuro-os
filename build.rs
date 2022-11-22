use cc::Build;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    Build::new()
        .flag("-march=rv64imac")
        .flag("-mabi=lp64")
        .file("boot.s")
        .compile("boot.o");

    Ok(())
}
