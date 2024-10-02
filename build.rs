fn main() {
    println!("cargo:rustc-link-search=all=src");      // works like "rustc -L src ..." 
    println!("cargo:rustc-link-lib=dylib=vuln.o"); // works like "rustc -l doubler.o"
}
