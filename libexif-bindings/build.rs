extern crate pkg_config;
use std::env;
use std::path::PathBuf;

fn main() {
    let libexif_path = env::var("LIBEXIF_STATIC_LIBRARY_PATH");
    match libexif_path {
        Ok(path) => {
            println!("cargo:rustc-link-lib=static={}", path);
        }
        Err(_) => {
            pkg_config::Config::new()
                .atleast_version("0.6.24")
                .statik(true)
                .probe("libexif")
                .unwrap();
        }
    };

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
