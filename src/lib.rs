use pdfium_render::prelude::*;
use std::{fs, path::Path};

pub struct PdfiumWrapper {
    pub pdfium: Pdfium,
}

impl PdfiumWrapper {
    #[cfg(target_os = "windows")]
    pub fn init(binary_path: &String) {
        let lib_bytes = include_bytes!("../pdfium-windows/lib/pdfium.dll.lib");

        fs::create_dir_all(Path::new(&binary_path).parent().unwrap()).unwrap();
        let binary_path = format!("{}/pdfium.dll.lib", binary_path);
        println!("{:#?}", binary_path);
        //make sure the path is valid
        fs::write(binary_path, lib_bytes).unwrap();
    }

    #[cfg(target_os = "linux")]
    pub fn init(binary_path: &String) {
        let lib_bytes = include_bytes!("../pdfium-linux/lib/libpdfium.so");

        fs::create_dir_all(Path::new(&binary_path).parent().unwrap()).unwrap();
        let binary_path = format!("{}/libpdfium.so", binary_path);
        //make sure the path is valid
        fs::write(binary_path, lib_bytes).unwrap();
    }
}
