use std::{path::PathBuf, env, fs};

fn main() {

    println!("cargo:rustc-link-search=/usr/local/cuda/lib64");
    println!("cargo:rustc-link-lib=cudart");
    println!("cargo:rustc-link-lib=cuda");
    println!("cargo:rustc-link-lib=cupti");
    println!("cargo:include=/usr/local/cuda/include");

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I/usr/local/cuda/include")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // bindgen will generate both __packed__ and __align__. Getting rid of one seems to pass the tests.
    // https://github.com/rust-lang/rust-bindgen/issues/1538
    // #define ACTIVITY_RECORD_ALIGNMENT 8
    // #define PACKED_ALIGNMENT __attribute__ ((__packed__)) __attribute__ ((aligned (ACTIVITY_RECORD_ALIGNMENT)))
    let hacky_fix = bindings.to_string().replace(", packed(8)", "");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(out_dir).join("bindings.rs");
    fs::write(out_path.as_path(), hacky_fix).expect("Couldn't write bindings");
}
