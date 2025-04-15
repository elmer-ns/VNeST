// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use image;
use slint::{Rgba8Pixel, SharedPixelBuffer};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let icon = load_icon()?;
    ui.set_ico(slint::Image::from_rgba8(icon));

    ui.run()?;
    Ok(())
}

fn load_icon() -> Result<SharedPixelBuffer<Rgba8Pixel>, Box<dyn Error>> {
    let icon_raw = include_bytes!("../icon.ico");
    let icon = image::load_from_memory(icon_raw)?
        .to_rgba8();

    Ok(SharedPixelBuffer::clone_from_slice(icon.as_raw(), icon.width(), icon.height()))
}