fn main() {
    // 1. Tell Cargo to rerun this script if gui.c changes
    println!("cargo:rerun-if-changed=src/gui/gui.c");
    println!("cargo:rerun-if-changed=src/gui/interface.h");

    // 2. Compile the C code into a static library named "libironclad_gui.a"
    cc::Build::new()
        .file("src/gui/gui.c")
        .include("src/gui")
        .compile("ironclad_gui"); // This creates the library

    // 3. Link Raylib and system dependencies (standard for Arch)
    println!("cargo:rustc-link-lib=raylib");
    println!("cargo:rustc-link-lib=GL");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=dl");
}
