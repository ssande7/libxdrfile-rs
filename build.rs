
fn main() {
    println!("cargo:rerun-if-changed=libxdrfile");

    let xdrfile = cmake::Config::new("libxdrfile").build();

    println!("cargo:rustc-link-search=native={}/lib", xdrfile.display());
    println!("cargo:rustc-link-lib=xdrfile");
}
