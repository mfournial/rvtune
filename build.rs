extern crate bindgen;

use std::env;
use std::path::PathBuf;

const HEADERS_DIR: &'static str = "intel_headers";

pub fn main() {

    // Finds the header files
    let mut header = PathBuf::from(HEADERS_DIR);
    header.push("wrapper.h");

    //println!("{}{}", "cargo:rustc-link-search=", HEADERS_DIR);
    println!("{} {}", "cargo:rustc-flags= -L", HEADERS_DIR);
    println!("cargo:rustc-link-lib=static=ittnotify");

    let bindings = bindgen::Builder::default()
        .header(header.to_str().unwrap())
		    .layout_tests(false)
        .generate_comments(false)
		    .prepend_enum_name(false)
		    .rustfmt_bindings(true)
        .rustfmt_configuration_file(None)
        //.whitelist_function(".*pause.*")
        //.whitelist_type(".*pause.*")
        //.whitelist_var(".*pause.*")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
