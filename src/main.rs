use std::{
    collections::HashMap,
    process::{Command, Output},
};

use clap::Parser;
use env::{CLAVE, EJECUTABLE, RUTA, SERVIDOR, USUARIO};

use crate::cli::Cli;
use crate::env::HOTELES;

mod cli;
mod env;
mod menu;

fn make_c(bbdd: &str, unidad: &str, fecha: &str) -> Output {
    let args = vec![SERVIDOR, bbdd, USUARIO, CLAVE, RUTA, unidad, fecha, fecha];
    Command::new("Winsolution.ContaPS.exe")
        .current_dir(EJECUTABLE)
        .args(args)
        .output()
        .expect("failed to execute process")
}

fn main() -> Result<(), time::error::Error> {
    let hoteles: HashMap<String, String> =
        serde_json::from_str(HOTELES).expect("Fallo al serializar HOTELES");
    let cli = Cli::parse();
    let fecha = cli.get_fecha();
    let unidad = cli.get_unidad(&hoteles);

    let base_datos = hoteles.get(unidad.as_str()).unwrap();

    let output = make_c(base_datos, &unidad, &fecha);
    println!("Status: {}", output.status);
    println!("Output: {}", String::from_utf8_lossy(&output.stdout));

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}
