fn main() {
    if should_static_link() {
        let mut current_dir = std::env::current_dir().unwrap();

        if macos() {
            current_dir.push("pdfium-macos");
        } else if ios() {
            current_dir.push("pdfium-ios");
        } else if ios_sim() {
            current_dir.push("pdfium-ios-sim");
        }

        let current_dir = current_dir.display();

        println!("cargo:rustc-link-lib=static=pdfium");
        println!("cargo:rustc-link-search=native={current_dir}");
    }
}

fn macos() -> bool {
    std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "macos"
}

fn ios() -> bool {
    std::env::var("TARGET").unwrap() == "aarch64-apple-ios"
}

fn ios_sim() -> bool {
    std::env::var("TARGET").unwrap() == "aarch64-apple-ios-sim"
}

fn apple() -> bool {
    std::env::var("CARGO_CFG_TARGET_VENDOR").unwrap() == "apple"
}

fn should_static_link() -> bool {
    apple()
}
