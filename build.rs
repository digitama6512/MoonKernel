fn main() {
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let linker_path = format!("arch/{arch}/linker.ld");
    // Tell cargo to pass the linker script to the linker..
    println!("cargo:rustc-link-arg=-T{}", linker_path);
    // ..and to re-run if it changes.
    println!("cargo:rerun-if-changed={}", linker_path);
}
