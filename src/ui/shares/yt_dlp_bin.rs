use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use dirs;
pub fn ytdlp_cache() -> Option<PathBuf> {
    let raw_bytes = include_bytes!("../../../yt-dlp/yt-dlp_linux");
    let azul_box = match dirs::cache_dir() {
        Some(p) => p.join("azul_box"),
        None => PathBuf::new(),
    };
    if !azul_box.exists() {
        fs::create_dir_all(&azul_box).expect("Failed to create directory in ytdlp_cache");
    }

    let bin_path = azul_box.join("yt-dlp");
    // Write the raw bytes to the specified path
    if let Err(e) = fs::write(&bin_path, raw_bytes) {
        eprintln!("Failed to write bytes to file: {}", e);
        return None; // Return None if writing fails
    }

    // Set the file as executable
    let mut permissions = fs::metadata(&bin_path).ok()?.permissions();

    permissions.set_mode(0o755);
    if let Err(e) = fs::set_permissions(&bin_path, permissions) {
        eprintln!("Failed to set file permissions: {}", e);
        return None;
    }

    fs::write(&bin_path, raw_bytes).expect("Failed to write bytes to file in ytdlp_cache");
    Some(bin_path)
}
