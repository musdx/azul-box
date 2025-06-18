use std::fs;
use std::path::Path;
pub fn config_file() {
    let azul_conf = "AzulBox";
    let azul_conf_file = "azul.config";
    let config_dir = dirs::config_dir()
        .map(|path| path.to_string_lossy().into_owned())
        .unwrap_or_else(|| String::from(""));
    let azul_conf_dir = format!("{config_dir}/{azul_conf}");
    if !Path::new(&azul_conf_dir).exists() {
        let _ = fs::create_dir(Path::new(&azul_conf_dir));
    }
    let azul_conf_file_with_dir = format!("{azul_conf_dir}/{azul_conf_file}");
    if !Path::new(&azul_conf_file_with_dir).exists() {
        let contents = "";
        let _ = fs::write(Path::new(&azul_conf_file_with_dir), contents);
    }
}
