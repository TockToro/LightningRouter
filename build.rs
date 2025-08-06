// Jackson Coxson

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_header(
            "// Jackson Coxson\n// Bindings to LightningRouter - https://github.com/jkcoxson/idevice",
        )
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("lightning_router_rs.h");
}
