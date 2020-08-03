fn main() {
    println!("cargo:rerun-if-changed=bindings/bindings.cpp");

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
