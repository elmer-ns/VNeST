slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = Test::new()?;

    main_window.run()
}
