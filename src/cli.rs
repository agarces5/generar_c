use std::{collections::HashMap, process::exit};

use clap::Parser;
use time::Duration;

use crate::menu::main_menu;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
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
