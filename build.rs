use cmake::Config;

fn main() {
    let mut config = Config::new("mcl");
    config.define("MCL_STATIC_LIB", "ON").define("MCL_STANDALONE", "ON");

    if cfg!(any(target_arch = "x86_64", target_os = "windows")) {
        config.define("CMAKE_CXX_COMPILER", "clang++");
    }

    if cfg!(all(target_arch = "x86_64", not(target_os = "windows"))) {
        println!("cargo:rustc-link-lib=stdc++");
    }

    let dst = config.build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=mcl");
    println!("cargo:rustc-link-lib=static=mclbn384_256");
}
