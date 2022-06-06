use std::path::Path;

fn main() {
    println!(
        "cargo:rustc-link-search=native={}",
        std::fs::canonicalize(Path::new("vendor"))
            .unwrap()
            .display()
    );
}
