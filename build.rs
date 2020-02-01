#[cfg(all(unix, not(target_os = "macos")))]
fn main() {

    // add unix dependencies below
    // println!("cargo:rustc-flags=-l readline");
}

#[cfg(target_os = "macos")]
fn main() {

    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}

#[cfg(target_os = "windows")]
fn main() {
    // add windows dependencies below
}
