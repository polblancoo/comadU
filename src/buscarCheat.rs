use anyhow::{Context, Result};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn buscar_cheat(comando: &str, seccion: Option<&str>) -> Result<()> {
    // Ejecutar curl para obtener la información del comando
    let mut curl_cmd = Command::new("curl")
        .arg(format!("https://cheat.sh/{}", comando))
        .stdout(Stdio::piped())
        .spawn()
        .context("Error al ejecutar curl")?;

    let stdout = curl_cmd
        .stdout
        .take()
        .context("No se pudo capturar la salida")?;
    let reader = BufReader::new(stdout);

    // Si se especifica una sección, filtrar líneas
    let lineas: Vec<String> = if let Some(sec) = seccion {
        reader
            .lines()
            .filter_map(Result::ok)
            .filter(|linea| linea.trim_start().starts_with(&format!("{}: ", sec)))
            .collect()
    } else {
        reader.lines().filter_map(Result::ok).collect()
    };

    // Imprimir resultados
    for linea in lineas {
        println!("{}", linea);
    }

    Ok(())
}
