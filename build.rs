#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    println!("cargo:rustc-flags=-l SDL2 -l X11 -l ncursesw -l tinfo");
}
