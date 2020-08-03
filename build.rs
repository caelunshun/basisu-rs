fn main() {
    println!("cargo:rerun-if-changed=bindings/bindings.h");

    let bindings = bindgen::Builder::default()
        .header("bindings/bindings.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("failed to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write bindings");

    cc::Build::new()
        .file("bindings/bindings.cpp")
        .include("bindings")
        .cpp(true)
        .flag_if_supported("-std=c++11")
        .compile("basis_universal");

    println!("cargo:rustc-link-lib=static=basis_universal");

    let target = std::env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
}
