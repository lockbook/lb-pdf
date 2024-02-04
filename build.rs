#[cfg(target_os = "macos")]
fn main() {
    let mut current_dir = std::env::current_dir().unwrap();
    current_dir.push("pdfium-macos");
    let current_dir = current_dir.display();
    println!("cargo:rustc-link-lib=static=pdfium");
    println!("cargo:rustc-link-search=native={current_dir}");
}

#[cfg(target_os = "ios")]
fn main() {
    let mut current_dir = std::env::current_dir().unwrap();
    current_dir.push("pdfium-macos");
    let current_dir = current_dir.display();
    println!("cargo:rustc-link-lib=static=pdfium");
    println!("cargo:rustc-link-search=native={current_dir}");
}