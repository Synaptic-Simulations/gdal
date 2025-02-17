use std::io::{Cursor, Read};
use std::path::Path;
use zip::ZipArchive;

fn main() {
    if cfg!(windows) && std::env::var_os("GDAL_NO_DOWNLOAD").is_none() {
		let out = std::env::var_os("OUT_DIR").unwrap();
        let out_path = Path::new(&out);
        let gdal_path = out_path.join("gdal.lib");
        if !gdal_path.exists() {
            let data = reqwest::blocking::get(
                "https://drive.google.com/u/1/uc?id=1OF0OAZPHd7pGCgDhTHQ0-QAt7GI4xqT1&export=download&confirm=t",
            )
                .unwrap()
                .bytes()
                .unwrap();

            let mut archive = ZipArchive::new(Cursor::new(&data[..])).unwrap();
            let mut data = Vec::new();
            archive.by_index(0).unwrap().read_to_end(&mut data).unwrap();
            std::fs::write(&gdal_path, data).unwrap();
        }
		
		println!(
            "cargo:rustc-link-search=native={}",
            out_path.display()
        );
    } else {
        println!("cargo:rerun-if-env-changed=GDAL_PATH");
        let path = std::env::var("GDAL_PATH")
            .expect("Expected environment variable GDAL_PATH to point to a static GDAL library");

        println!("cargo:rustc-link-search=native={}", path);
    }
}
