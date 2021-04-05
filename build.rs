extern crate cc;

// This is here to build a small C library on Mac systems that requests a few
// additional resources and priority from the operating system to provide
// better frame synchronization. It shouldn't interfere with other programs,
// so you can probably just accept the default of `processing-rs` using it
// automatically.

fn main() {
    if cfg!(target_os = "macos") {
        cc::Build::new().file("pri.c").compile("pri");
    } else if cfg!(target_os = "windows") {
        println!(
            "cargo:rustc-link-search={}",
            "C:\\Users\\me\\Documents\\source_code\\glfw-3.3.2.bin.WIN64\\lib-vc2019"
        );
        println!("cargo:rustc-link-lib=glfw3");
    }
}
