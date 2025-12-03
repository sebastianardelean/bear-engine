fn main() {
    // Path to your Assimp libs
    println!("cargo:rustc-link-search=native=lib/assimp");

    // Debug vs Release
    if cfg!(debug_assertions) {
        println!("cargo:rustc-link-lib=assimp-vc143-mtd");
    } else {
        println!("cargo:rustc-link-lib=assimp-vc143-mt");
    }
}