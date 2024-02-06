pub use pdfium_render::prelude::*;

#[cfg(target_vendor = "apple")]
pub fn init(_statically_linked: &String) -> Pdfium {
    extern crate link_cplusplus;

    let bindings = Pdfium::bind_to_statically_linked_library().unwrap();
    Pdfium::new(bindings)
}

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub fn init(binary_path: &String) -> Pdfium {
    use std::{fs, path::Path};

    let lib_bytes = include_bytes!("../pdfium-windows/x64/lib/pdfium.dll");

    fs::create_dir_all(Path::new(&binary_path).parent().unwrap()).unwrap();
    let binary_path = format!("{}/pdfium.dll", binary_path);
    println!("{:#?}", binary_path);
    //make sure the path is valid
    fs::write(binary_path, lib_bytes).unwrap();

    let bindings = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path(
        &binary_path,
    ))
        .unwrap();

    Pdfium::new(bindings)
}

#[cfg(all(target_os = "windows", target_arch = "aarch64"))]
pub fn init(binary_path: &String) -> Pdfium {
    use std::{fs, path::Path};
    let lib_bytes = include_bytes!("../pdfium-windows/arm64/lib/pdfium.dll");

    fs::create_dir_all(Path::new(&binary_path).parent().unwrap()).unwrap();
    let binary_path = format!("{}/pdfium.dll", binary_path);
    println!("{:#?}", binary_path);
    //make sure the path is valid
    fs::write(binary_path, lib_bytes).unwrap();

    let bindings = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path(
        &binary_path,
    ))
        .unwrap();

    Pdfium::new(bindings)
}

#[cfg(target_os = "linux")]
pub fn init(binary_path: &String) -> Pdfium {
    use std::{fs, path::Path};

    let lib_bytes = include_bytes!("../pdfium-linux/lib/libpdfium.so");

    fs::create_dir_all(Path::new(&binary_path).parent().unwrap()).unwrap();
    let binary_path = format!("{}/libpdfium.so", &binary_path);
    //make sure the path is valid
    fs::write(binary_path, lib_bytes).unwrap();

    let bindings = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path(
        &binary_path,
    ))
        .unwrap();

    Pdfium::new(bindings)
}

