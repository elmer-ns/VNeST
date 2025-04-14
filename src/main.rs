// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    println!("RUST");

    ui.on_verb_edited({
        let ui_handle = ui.as_weak();
        move |verb| {
            let ui = ui_handle.unwrap();
            ui.set_verb(verb);
        }
    });

    ui.run()?;
    Ok(())
}
