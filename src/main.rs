use std::{
    collections::HashMap,
    process::{exit, Command, Output},
};

use clap::Parser;
use env::{CLAVE, EJECUTABLE, RUTA, SERVIDOR, USUARIO};
use terminal_menu::{button, label, menu, mut_menu, run};
use time::Duration;

use crate::env::HOTELES;

mod env;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Unidad hotelera
    unidad: Option<String>,

    /// Fecha del C que se genera. (Vacio para el dia anterior).
    /// Formato: "DD MM AAAA"
    fecha: Option<String>,
}

fn generar_c(bbdd: &str, unidad: &str, fecha: &str) -> Output {
    let args = vec![SERVIDOR, bbdd, USUARIO, CLAVE, RUTA, unidad, fecha, fecha];
    Command::new("Winsolution.ContaPS.exe")
        .current_dir(EJECUTABLE)
        .args(args)
        .output()
        .expect("failed to execute process")
}

fn generar_fecha_anterior() -> Result<String, time::error::Error> {
    let format = time::format_description::parse("[day] [month] [year]")?;

    Ok(time::OffsetDateTime::now_local()?
        .checked_sub(Duration::days(1))
        .unwrap()
        .format(&format)?)
}

fn main() -> Result<(), time::error::Error> {
    let cli = Cli::parse();
    let hoteles: HashMap<String, String> =
        serde_json::from_str(HOTELES).expect("Fallo al serializar HOTELES");
    println!("{hoteles:?}");
    let fecha = match cli.fecha {
        Some(date) => date,
        None => generar_fecha_anterior()?,
    };
    let unidad = match cli.unidad {
        Some(u) => {
            if hoteles.keys().any(|key| *key == u) {
                u
            } else {
                println!("Unidad hotelera mal introducida");
                exit(1);
            }
        }
        None => {
            let menu = menu(vec![
                label("░█▀▀░█▀▀░█▀█░█▀▀░█▀▄░█▀█░█▀▄░░░█▀▀░░░█░█░█▀█░▀█▀░█▀▀░█░░░█▀▀░█▀▀"),
                label("░█░█░█▀▀░█░█░█▀▀░█▀▄░█▀█░█▀▄░░░█░░░░░█▀█░█░█░░█░░█▀▀░█░░░█▀▀░▀▀█"),
                label("░▀▀▀░▀▀▀░▀░▀░▀▀▀░▀░▀░▀░▀░▀░▀░░░▀▀▀░░░▀░▀░▀▀▀░░▀░░▀▀▀░▀▀▀░▀▀▀░▀▀▀"),
                label("\tSELECCIONA CON ENTER"),
                label("------------------------------"),
                button("\t0010 - PLAYA CAPRICHO"),
                button("\t0020 - SNT CASTELLANA"),
                button("\t0030 - PLAYA DULCE"),
                button("\t0040 - DIVER AGUADULCE"),
                button("\t0050 - PLAYA SOL"),
                button("\t0060 - DIVER ROQUETAS"),
                button("\t0080 - VERA PLAYA"),
                button("\t0090 - ZIMBALI PLAYA"),
                button("\t0130 - SNT MAR MENOR"),
                button("\t0140 - CABO DE GATA"),
                button("\t0150 - SNT BARAJAS"),
                button("\t0160 - CALEIA MAR MENOR"),
                button("\t0180 - PLAYA BALLENA"),
                button("\t0190 - PLAYA CALIDA"),
                button("\t0200 - PLAYA LINDA"),
                button("\t0260 - CALA MILLOR"),
                button("\t0270 - PLAYA CANELA"),
                button("\t0280 - SNT HUELVA"),
                button("\t0290 - MOJACAR PLAYA"),
                button("\t0340 - VIRGEN DE LOS REYES"),
                button("\t0350 - SNT MARBELLA"),
                button("\t0360 - SNT VALENCIA"),
                button("\t0410 - PLAYA MARINA"),
                button("\t0420 - PLAYA CARTAYA"),
                button("\t0440 - SNT CADIZ"),
                button("\t0450 - SNT GRANADA"),
                button("\t0460 - TALAYOT"),
                button("\t0470 - SNT BANUS"),
                button("\t0480 - APTO PARAISO PLAYA"),
                button("\t0500 - ALMUÑECAR PLAYA"),
                button("\t0600 - ALHAURIN"),
                button("\t0610 - SNT GANDIA"),
                button("\t0650 - CLUB SIMO"),
                button("\t0700 - MONTANYA"),
                button("\t0750 - GUADACORTE"),
                button("\t2520 - AQUARIUM"),
                label("-------------------------------"),
                // button:
                //  exit the menu
                button("\tSALIR"),
            ]);
            run(&menu);

            let selection = mut_menu(&menu);
            let selection = selection.selected_item_name();

            if selection == "\tSALIR" {
                exit(0);
            }

            selection.get(1..5).unwrap().to_string()
        }
    };

    let base_datos = hoteles.get(unidad.as_str()).unwrap();

    let output = generar_c(base_datos, &unidad, &fecha);
    println!("Status: {}", output.status);
    println!("Output: {}", String::from_utf8_lossy(&output.stdout));

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}
