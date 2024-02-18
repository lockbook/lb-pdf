pub use pdfium_render::prelude::*;

#[cfg(target_vendor = "apple")]
pub fn init(_statically_linked: &str) -> Pdfium {
    extern crate link_cplusplus;

    let bindings = Pdfium::bind_to_statically_linked_library().unwrap();
    Pdfium::new(bindings)
}

#[cfg(not(target_vendor = "apple"))]
fn init_dynamically_linked(binary_folder: &str, binary_name: &str, lib_bytes: &[u8]) -> Pdfium {
    use std::{fs, path::Path};

    fs::create_dir_all(Path::new(&binary_folder).parent().unwrap()).unwrap();
    let binary_path = format!("{}/{}", binary_folder, binary_name);
    fs::write(&binary_path, lib_bytes).unwrap();
    let bindings =
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path(&binary_folder))
            .unwrap();

    Pdfium::new(bindings)
}

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub fn init(binary_folder: &String) -> Pdfium {
    let binary_name = "pdfium.dll";
    let lib_bytes = include_bytes!("../pdfium-windows/x64/lib/pdfium.dll");
    init_dynamically_linked(binary_folder, binary_name, lib_bytes)
}

#[cfg(all(target_os = "windows", target_arch = "aarch64"))]
pub fn init(binary_folder: &String) -> Pdfium {
    let binary_name = "pdfium.dll";
    let lib_bytes = include_bytes!("../pdfium-windows/arm64/lib/pdfium.dll");
    init_dynamically_linked(binary_folder, binary_name, lib_bytes)
}

#[cfg(target_os = "linux")]
pub fn init(binary_folder: &String) -> Pdfium {
    let binary_name = "libpdfium.so";
    let lib_bytes = include_bytes!("../pdfium-linux/lib/libpdfium.so");
    init_dynamically_linked(binary_folder, binary_name, lib_bytes)
}
