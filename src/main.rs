use anyhow::{Context, Result};
use clap::{Arg, ArgAction, Command};
use colored::*;

mod buscarCheat;
mod buscarCrate;
mod funciones;

use buscarCheat::buscar_cheat;
use buscarCrate::buscar_crate;
use funciones::*;

fn main() -> Result<()> {
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
            Arg::new("lc")
                .long("lc")
                .short('c')
                .value_name("SECCION")
                .help("Lista comandos de una sección específica"),
        )
        .subcommand(
            Command::new("crateSearch")
                .about("Buscar información de crates")
                .arg(
                    Arg::new("buscar")
                        .help("Busca un crate en crates.io")
                        .required(true),
                ),
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
        .subcommand(
            Command::new("cheatSearch")
                .about("Busca cheat sheets de comandos")
                .arg(
                    Arg::new("comando")
                        .required(true)
                        .help("Comando a buscar en cheat.sh"),
                )
                .arg(
                    Arg::new("seccion")
                        .long("seccion")
                        .short('s')
                        .help("Sección específica a mostrar")
                        .required(false),
                ),
        )
        .get_matches();

    if matches.get_flag("ls") {
        list_secciones(&file_path);
    } else if matches.get_flag("list") {
        list_comandos(&file_path);
    } else if let Some(seccion) = matches.get_one::<String>("lc") {
        list_comandos_seccion(&file_path, seccion);
    } else {
        match matches.subcommand() {
            Some(("crateSearch", sub_m)) => {
                let crate_name = sub_m.get_one::<String>("buscar").unwrap();
                buscar_crate(crate_name);
            }
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
            Some(("cheatSearch", sub_m)) => {
                let comando = sub_m
                    .get_one::<String>("comando")
                    .context("Se requiere un comando")?;
                let seccion = sub_m.get_one::<String>("seccion");

                buscar_cheat(comando, seccion.map(|s| s.as_str()))?;
            }
            _ => {
                println!(
                    "{}: Usa '--help' para ver las opciones disponibles",
                    "Info".yellow()
                );
            }
        }
    }

    Ok(())
}
