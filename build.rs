use std::process::Command;
use std::path::PathBuf;

fn main() {
    // Folder where the external lib will be downloaded
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let assimp_dir = out_dir.join("assimp");

    // 1. Clone Assimp if not exists
    if !assimp_dir.exists() {
        Command::new("git")
            .args(&["clone", "--depth", "1", "https://github.com/assimp/assimp.git"])
            .arg(&assimp_dir)
            .status()
            .unwrap();
    }

    // 2. Build with cmake crate
    let profile = std::env::var("PROFILE").unwrap(); // "debug" or "release"
    let dst = cmake::Config::new(&assimp_dir)
        .generator("Visual Studio 17 2022")
        .define("CMAKE_GENERATOR_PLATFORM", "x64")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_CXX_FLAGS", "/EHsc /wd4530") // Enable exceptions, disable this warning
        .define("ASSIMP_WARNINGS_AS_ERRORS", "OFF") // Assimp-specific
        .define("BUILD_SHARED_LIBS", "OFF")
        .profile(&profile)
        .build();

    // 3. Tell cargo where the library is
    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    // Debug vs Release
    if cfg!(debug_assertions) {
        println!("cargo:rustc-link-lib=assimp-vc143-mtd");
    } else {
        println!("cargo:rustc-link-lib=assimp-vc143-mt");
    }
}
// fn main() {
//     // Path to your Assimp libs
//     println!("cargo:rustc-link-search=native=lib/assimp");
//
//     // Debug vs Release
//     if cfg!(debug_assertions) {
//         println!("cargo:rustc-link-lib=assimp-vc143-mtd");
//     } else {
//         println!("cargo:rustc-link-lib=assimp-vc143-mt");
//     }
// }