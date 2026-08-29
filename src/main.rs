#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
fn main() -> Result<(), slint::PlatformError> {
    let main_window = AppWindow::new()?;

    main_window.run()
}
