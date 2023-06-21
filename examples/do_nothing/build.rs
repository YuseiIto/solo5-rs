extern crate cc;
use which::which;

fn main() {
    println!("cargo:rerun-if-changed=manifest.c");
    let compiler = which("x86_64-solo5-none-static-cc").unwrap();
    cc::Build::new()
        .file("manifest.c")
        .flag("-z solo5-abi=hvt")
        .compiler(compiler)
        .compile("manifest.o");
}
