use std::process::Command;

pub fn buscar_crate(crate_name: &str) {
    let url = format!("https://crates.io/crates/{}", crate_name);
    if let Err(e) = Command::new("xdg-open").arg(&url).status() {
        eprintln!("Error al intentar abrir la URL en el navegador: {}", e);
    } else {
        println!("Abriendo {} en el navegador...", url);
    }
}
