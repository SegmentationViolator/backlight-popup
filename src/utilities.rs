use gtk;
use gtk::gdk;
use gtk::traits::WidgetExt;
use std::process;

/// Enable rgba visual
///
/// # Errors
/// if rgba visual is not supported by the current screen
pub fn enable_rgba_visual(
    window: &gtk::ApplicationWindow,
    screen: &gdk::Screen,
) -> Result<(), String> {
    let visual = screen.rgba_visual();

    if visual.is_none() {
        return Err("Current screen doesn't support rgba visual (transparency/opacity)".to_string());
    }
    window.set_visual(visual.as_ref());

    Ok(())
}

/// Get the current backlight percentage
///
/// # Errors
/// - if the output returned by xbacklight is not valid UTF8
/// - if the output returned by xbacklight is not valid float
/// - if the output returned by xbacklight is not in range 0.0..=100.0
pub fn get_backlight_percentage() -> Result<u8, String> {
    let mut command = process::Command::new("xbacklight");
    let output = command.output().map_err(|error| error.to_string())?;

    let percentage = String::from_utf8(output.stdout)
        .map_err(|error| error.to_string())?
        .trim()
        .parse::<f64>()
        .map_err(|error| error.to_string())?;
    if percentage < 0.0 || percentage > 100.0 {
        return Err("xbacklight returned unexpected output".to_string());
    }

    Ok(percentage.round() as u8)
}
