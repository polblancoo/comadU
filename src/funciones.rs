use colored::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// Funciones auxiliares
pub fn get_comandos_file_path() -> String {
    let home_dir = dirs::home_dir().expect("No se pudo obtener el directorio HOME");
    let path = home_dir.join("comandosUtiles.toml");
    path.to_string_lossy().to_string()
}

pub fn load_comandos(file_path: &str) -> HashMap<String, HashMap<String, String>> {
    if !Path::new(file_path).exists() {
        eprintln!("{}: Archivo no encontrado en {}", "Error".red(), file_path);
        return HashMap::new();
    }

    let content = fs::read_to_string(file_path).expect("No se pudo leer el archivo");
    match toml::from_str::<HashMap<String, HashMap<String, String>>>(&content) {
        Ok(comandos) => comandos,
        Err(e) => {
            eprintln!("{}: Error al parsear el archivo TOML: {}", "Error".red(), e);
            HashMap::new()
        }
    }
}

pub fn save_comandos(file_path: &str, comandos: &HashMap<String, HashMap<String, String>>) {
    let content = toml::to_string(comandos).expect("Error al serializar comandos a TOML");
    fs::write(file_path, content).expect("No se pudo escribir en el archivo");
}

pub fn add_comando(file_path: &str, seccion: &str, nombre: &str, comando: &str) {
    let mut comandos = load_comandos(file_path);
    let entry = comandos
        .entry(seccion.to_string())
        .or_insert_with(HashMap::new);

    if entry.contains_key(nombre) {
        eprintln!(
            "{}: El comando '{}' ya existe en la sección '{}'",
            "Error".red(),
            nombre,
            seccion
        );
        return;
    }

    entry.insert(nombre.to_string(), comando.to_string());
    save_comandos(file_path, &comandos);
    println!(
        "{}: Comando '{}' agregado exitosamente en la sección '{}'",
        "Éxito".green(),
        nombre,
        seccion
    );
}

pub fn list_comandos(file_path: &str) {
    let comandos = load_comandos(file_path);
    if comandos.is_empty() {
        println!("No hay comandos disponibles.");
        return;
    }

    println!("Comandos disponibles:");
    for (seccion, claves) in comandos {
        println!("+-------------------------+");
        println!("| {} |", seccion.cyan());
        println!("+-------------------------+");
        for (nombre, comando) in claves {
            println!("\t\t{}: {}", nombre.yellow(), comando);
        }
    }
}

pub fn list_secciones(file_path: &str) {
    let comandos = load_comandos(file_path);
    if comandos.is_empty() {
        println!("No hay secciones disponibles.");
        return;
    }

    println!("Secciones disponibles:");
    for seccion in comandos.keys() {
        println!("- {}", seccion.cyan());
    }
}

pub fn delete_comando(file_path: &str, seccion: &str, nombre: &str) {
    let mut comandos = load_comandos(file_path);
    if let Some(claves) = comandos.get_mut(seccion) {
        if claves.remove(nombre).is_some() {
            if claves.is_empty() {
                comandos.remove(seccion);
            }
            save_comandos(file_path, &comandos);
            println!(
                "{}: Comando '{}' eliminado exitosamente de la sección '{}'",
                "Éxito".green(),
                nombre,
                seccion
            );
        } else {
            eprintln!(
                "{}: El comando '{}' no existe en la sección '{}'",
                "Error".red(),
                nombre,
                seccion
            );
        }
    } else {
        eprintln!("{}: La sección '{}' no existe", "Error".red(), seccion);
    }
}

pub fn list_comandos_seccion(file_path: &str, seccion: &str) {
    let comandos = load_comandos(file_path);
    if let Some(seccion_comandos) = comandos.get(seccion) {
        if seccion_comandos.is_empty() {
            println!("No hay comandos en la sección '{}'.", seccion);
            return;
        }

        println!("Comandos en la sección '{}':", seccion.cyan());
        println!("+-------------------------+");
        for (nombre, comando) in seccion_comandos {
            println!("\t\t{}: {}", nombre.yellow(), comando);
        }
    } else {
        eprintln!("{}: La sección '{}' no existe", "Error".red(), seccion);
    }
}
