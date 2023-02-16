use std::{env, path::PathBuf};

fn main() {
    let filename = "c/header.h";

    let bindings = bindgen::Builder::default()
        .header(filename)
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Could not write bindings");
}
