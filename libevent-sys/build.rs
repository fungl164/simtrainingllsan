use std::env;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search=native={}", manifest_dir);
    println!("cargo:rustc-link-lib=static=event_core");
    //println!("cargo:rustc-flags=-L {} -l event_core", manifest_dir);
}
