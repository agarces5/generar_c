use std::process::exit;
use terminal_menu::{button, label, menu, mut_menu, run};

pub fn main_menu() -> String {
    let menu = menu(vec![
        label("░█▀▀░█▀▀░█▀█░█▀▀░█▀▄░█▀█░█▀▄░░░█▀▀░░░█░█░█▀█░▀█▀░█▀▀░█░░░█▀▀░█▀▀"),
        label("░█░█░█▀▀░█░█░█▀▀░█▀▄░█▀█░█▀▄░░░█░░░░░█▀█░█░█░░█░░█▀▀░█░░░█▀▀░▀▀█"),
        label("░▀▀▀░▀▀▀░▀░▀░▀▀▀░▀░▀░▀░▀░▀░▀░░░▀▀▀░░░▀░▀░▀▀▀░░▀░░▀▀▀░▀▀▀░▀▀▀░▀▀▀"),
        label("SELECCIONA CON ENTER"),
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
        button("SALIR"),
    ]);
    run(&menu);

    let selection = mut_menu(&menu);
    let selection = selection.selected_item_name();

    if selection == "SALIR" {
        exit(0);
    }

    selection.get(1..5).unwrap().to_string()
}
