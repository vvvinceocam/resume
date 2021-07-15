use std::env;
use std::path::PathBuf;

use blake3::hash;

#[cfg(target_os = "macos")]
fn get_cache_folder() -> PathBuf {
    PathBuf::from(format!(
        "{}/Library/Caches/resume",
        env::var("HOME").unwrap(),
    ))
}

#[cfg(target_os = "linux")]
pub fn get_cache_folder() -> PathBuf {
    PathBuf::from(
        env::var("XDG_CACHE_HOME")
            .unwrap_or_else(|| format!("{}/.cache/resume", env::var("HOME").unwrap(),)),
    )
}

pub fn get_repo_cache_folder(origin: &str) -> PathBuf {
    let mut path = get_cache_folder();
    path.push(hash(origin.as_bytes()).to_hex().as_str());
    path
}
