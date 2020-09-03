extern crate cc;

fn main() {
    // println!("cargo:rerun-if-changed=src/hello.c");
    println!("cargo:rustc-link-lib=gpiod");

    cc::Build::new()
        .file("src/c_src/spi_func_lib.c")
        .include("src/c_src/")
        .flag("-lgpiod")
        // .extra_warnings(false)
        .compile("spi_func_lib.a");
}
