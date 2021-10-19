use std::env;
use std::path::{PathBuf};

#[cfg(unix)]
fn get_home() -> PathBuf {
    PathBuf::from(env::var("HOME").unwrap())
}

#[cfg(target_os = "windows")]
fn get_home() -> PathBuf {
    PathBuf::from(env::var("APPDATA").unwrap())
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
fn get_minecraft_dir() -> PathBuf {
    let mut path = get_home();
    path.push(".minecraft");
    path
}

#[cfg(target_os = "macos")]
fn get_minecraft_dir() -> PathBuf {
    let mut path = get_home();
    path.push("Library/Application Support/minecraft");
    path
}

lazy_static! {
    pub static ref MINECRAFT_DIR: PathBuf = get_minecraft_dir();
}

lazy_static! {
    pub static ref VERSIONS_DIR: PathBuf = {
        let mut tmp = MINECRAFT_DIR.clone();
        tmp.push("versions");
        tmp
    };
}


pub fn generate_random_uuid() -> String {
    uuid::Uuid::new_v4().to_hyphenated().to_string()
}
