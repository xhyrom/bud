use lazy_static::lazy_static;

#[cfg(target_os = "windows")]
lazy_static! {
    pub static ref DATA_FOLDER: String = format!("{}\\Bud", std::env::var("APPDATA").unwrap());
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
lazy_static! {
    pub static ref DATA_FOLDER: String = format!("{}/.bud", std::env::var("HOME").unwrap());
}
