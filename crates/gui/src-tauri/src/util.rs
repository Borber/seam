#[cfg(windows)]
const SEPARATOR: &str = "\\";

#[cfg(not(windows))]
const SEPARATOR: &str = "/";

pub fn bin_dir() -> String {
    let p = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    format!("{p}{SEPARATOR}")
}
