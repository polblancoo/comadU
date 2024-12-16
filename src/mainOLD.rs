use clap::{Arg, ArgAction, Command};
use colored::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
mod buscarCrate;
use buscarCrate::buscar_crate;

const COMANDOS_FILE: &str = "comandosUtiles.toml";

fn get_comandos_file_path() -> String {
    let home_dir = dirs::home_dir().expect("No se pudo obtener el directorio HOME");
    let path = home_dir.join(COMANDOS_FILE);
    path.to_string_lossy().to_string()
}

fn load_comandos(file_path: &str) -> HashMap<String, HashMap<String, String>> {
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

fn save_comandos(file_path: &str, comandos: &HashMap<String, HashMap<String, String>>) {
    let content = toml::to_string(comandos).expect("Error al serializar comandos a TOML");
    fs::write(file_path, content).expect("No se pudo escribir en el archivo");
}

fn add_comando(file_path: &str, seccion: &str, nombre: &str, comando: &str) {
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

fn list_comandos(file_path: &str) {
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
            println!("  {}: {}", nombre.yellow(), comando);
        }
    }
}

fn list_secciones(file_path: &str) {
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

fn delete_comando(file_path: &str, seccion: &str, nombre: &str) {
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

fn main() {
    let file_path = get_comandos_file_path();

    let matches = Command::new("Comandos Utiles")
        .version("1.0")
        .author("Tu Nombre <tuemail@example.com>")
        .about("Gestiona comandos útiles")
        .arg(
            Arg::new("ls")
                .long("ls")
                .short('l')
                .action(ArgAction::SetTrue)
                .help("Lista todas las secciones disponibles"),
        )
        .arg(
            Arg::new("list")
                .long("list")
                .short('a')
                .action(ArgAction::SetTrue)
                .help("Lista todos los comandos"),
        )
        .arg(
            Arg::new("buscar")
                .long("buscar")
                .short('b')
                .value_name("CRATE")
                .help("Busca un crate en crates.io")
                .action(ArgAction::Set),
        )
        .subcommand(
            Command::new("add")
                .about("Agrega un nuevo comando")
                .arg(
                    Arg::new("seccion")
                        .required(true)
                        .help("Sección donde agregar el comando"),
                )
                .arg(Arg::new("nombre").required(true).help("Nombre del comando"))
                .arg(Arg::new("comando").required(true).help("El comando en sí")),
        )
        .subcommand(
            Command::new("delete")
                .about("Elimina un comando")
                .arg(
                    Arg::new("seccion")
                        .required(true)
                        .help("Sección del comando a eliminar"),
                )
                .arg(
                    Arg::new("nombre")
                        .required(true)
                        .help("Nombre del comando a eliminar"),
                ),
        )
        .get_matches();

    if matches.get_flag("ls") {
        list_secciones(&file_path);
    } else if matches.get_flag("list") {
        list_comandos(&file_path);
    } else if let Some(crate_name) = matches.get_one::<String>("buscar") {
        buscar_crate(crate_name);
    } else {
        match matches.subcommand() {
            Some(("add", sub_m)) => {
                let seccion = sub_m.get_one::<String>("seccion").unwrap();
                let nombre = sub_m.get_one::<String>("nombre").unwrap();
                let comando = sub_m.get_one::<String>("comando").unwrap();
                add_comando(&file_path, seccion, nombre, comando);
            }
            Some(("delete", sub_m)) => {
                let seccion = sub_m.get_one::<String>("seccion").unwrap();
                let nombre = sub_m.get_one::<String>("nombre").unwrap();
                delete_comando(&file_path, seccion, nombre);
            }
            _ => {
                println!(
                    "{}: Usa '--help' para ver las opciones disponibles",
                    "Info".yellow()
                );
            }
        }
    }
}
