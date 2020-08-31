extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/c_src/spi_func_lib.c")
        .include("src/c_src/")
        .compile("spi_func_lib.a");
}
