use lazy_static::lazy_static;

#[cfg(target_os = "windows")]
lazy_static! {
    pub static ref FOLDER: String = format!("{}\\Bud", std::env::var("APPDATA").unwrap());
}

#[cfg(target_os = "linux")]
lazy_static! {
    pub static ref FOLDER: String = format!("{}/.bud", std::env::var("HOME").unwrap());
}

#[cfg(target_os = "macos")]
lazy_static! {
    pub static ref FOLDER: String = format!("{}/.bud", std::env::var("HOME").unwrap());
}
