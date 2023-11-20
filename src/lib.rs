use pdfium_render::prelude::*;
use std::{fs, path::Path};

pub struct PdfiumWrapper {
    pub pdfium: Pdfium,
}

impl PdfiumWrapper {
    #[cfg(target_os = "macos")]
    pub fn init(binary_path: &String) {
        let lib_bytes = include_bytes!("../pdfium-macos/lib/libpdfium.dylib");

        fs::create_dir_all(Path::new(&binary_path).parent().unwrap()).unwrap();
        let binary_path = format!("{}/pdfium.dylib", binary_path);
        println!("{:#?}", binary_path);
        //make sure the path is valid
        fs::write(binary_path, lib_bytes).unwrap();
    }

    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    pub fn init(binary_path: &String) {
        let lib_bytes = include_bytes!("../pdfium-windows/x64/lib/pdfium.dll");

        fs::create_dir_all(Path::new(&binary_path).parent().unwrap()).unwrap();
        let binary_path = format!("{}/pdfium.dll", binary_path);
        println!("{:#?}", binary_path);
        //make sure the path is valid
        fs::write(binary_path, lib_bytes).unwrap();
    }

    #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
    pub fn init(binary_path: &String) {
        let lib_bytes = include_bytes!("../pdfium-windows/arm64/lib/pdfium.dll");

        fs::create_dir_all(Path::new(&binary_path).parent().unwrap()).unwrap();
        let binary_path = format!("{}/pdfium.dll", binary_path);
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
