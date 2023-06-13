use std::{collections::HashMap, process::exit};

use clap::Parser;
use time::Duration;

use crate::menu::main_menu;

/// Genera el archivo C de los hoteles directamente a la carpeta de "hoteles"
#[derive(Parser, Debug)]
#[command(name = "Generador archivo C")]
#[command(author = "Antonio Garcés")]
#[command(
    help_template = "{about-section} \n {usage-heading} {usage} \n {all-args} {tab} \n\n Autor: {author-with-newline}"
)]
#[command(about, long_about)]
pub struct Cli {
    /// Unidad hotelera
    unidad: Option<String>,

    /// Fecha del C que se genera. (Vacio para el dia anterior).
    /// Formato: "DD MM AAAA"
    fecha: Option<String>,
}

impl Cli {
    pub fn get_unidad(&self, hoteles: &HashMap<String, String>) -> String {
        match &self.unidad {
            Some(u) => {
                if hoteles.keys().any(|key| key == u) {
                    u.to_string()
                } else {
                    println!("Unidad hotelera mal introducida");
                    exit(1);
                }
            }
            None => main_menu(),
        }
    }
    pub fn get_fecha(&self) -> String {
        match &self.fecha {
            Some(date) => date.to_string(),
            None => Self::get_yesterday_formatted().unwrap(),
        }
    }

    fn get_yesterday_formatted() -> Result<String, time::error::Error> {
        let format = time::format_description::parse("[day] [month] [year]")?;

        Ok(time::OffsetDateTime::now_local()?
            .checked_sub(Duration::days(1))
            .unwrap()
            .format(&format)?)
    }
}
