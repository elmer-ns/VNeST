// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, string};
use image;
use slint::{Rgba8Pixel, SharedPixelBuffer};
use text_measurer_harfbuzz::measure_text_width;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let icon = load_icon()?;
    ui.set__icon(slint::Image::from_rgba8(icon));

    ui.global::<ValidateString>().on_limit_size({
        //let ui_handle = ui.as_weak();
        move |mut string: slint::SharedString, max_length: f32,font_size: f32| {

            if string == "" {return font_size}

            font_size_calc(font_size, string.into(), max_length)
        }
    });

    ui.run()?;
    Ok(())
}

fn load_icon() -> Result<SharedPixelBuffer<Rgba8Pixel>, Box<dyn Error>> {
    let icon_raw = include_bytes!("../icon.ico");
    let icon = image::load_from_memory(icon_raw)?
        .to_rgba8();

    Ok(SharedPixelBuffer::clone_from_slice(icon.as_raw(), icon.width(), icon.height()))
}

fn font_size_calc(start_size: f32, string: String, max_width: f32) -> f32 {
    let mut step_size: f32 = 0.5;

    let mut size = start_size;

    while measure_text_width(include_bytes!("../SpaceMono-Regular.ttf").to_vec(), size as u32, &string).unwrap() as f32 > max_width*0.9 {
        size -= step_size;
        if size < 0.0 {
            size += step_size;
            step_size *= 0.5;
        }
    }

    size
}