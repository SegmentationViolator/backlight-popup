use std::borrow;
use std::time;
use std::fs;
use std::io;
use std::io::Read;

use xdg;

/// For storing the checked/compiled config.
pub struct Config {
    /// 6 digit hexadecimal code (with leading '#') for foreground color (text color)
    pub accent_color: borrow::Cow<'static, str>,
    /// duration in milliseconds to wait before updating the popup every cycle
    pub refresh_interval: time::Duration,
    /// opacity of the popup window. (must be in the range 0.0..=1.0)
    pub window_opacity: f64,
}

/// For storing the unchecked config.
#[derive(serde::Deserialize)]
struct UncheckedConfig {
    /// rgb color values
    pub accent_color: (u8, u8, u8),
    /// duration in milliseconds to wait before updating the popup every cycle
    pub refresh_interval: u64,
    /// opacity of the popup window.
    pub window_opacity: f64,
}

impl Config {
    /// Load config or return `default` if config file does not exist
    ///
    /// # Errors
    /// - if loaded config values are invalid
    /// - if error occurs while loading config or parsing the config file
    pub fn load(default: Self) -> Result<Self, String> {
        match UncheckedConfig::load()? {
            None => Ok(default),
            Some(UncheckedConfig {
                accent_color,
                refresh_interval,
                window_opacity,
            }) => {
                if window_opacity < 0.0 || window_opacity > 1.0 {
                    return Err(
                        "Window opacity value is out of bounds (lesser than 0.0 or greater than 1.0)"
                            .to_string(),
                    );
                }

                Ok(Self {
                    accent_color: borrow::Cow::Owned(format!(
                        "#{:02X}{:02X}{:02X}",
                        accent_color.0, accent_color.1, accent_color.2
                    )),
                    refresh_interval: time::Duration::from_millis(refresh_interval),
                    window_opacity,
                })
            }
        }
    }
}

impl UncheckedConfig {
    /// load config from config file or return `None` if config file does not exist
    ///
    /// # Errors
    /// - if an error occurs while loading or parsing the config file
    fn load() -> Result<Option<Self>, String> {
        let config_file_path = xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"))
            .map_err(|error| error.to_string())?
            .place_config_file("config.ron")
            .map_err(|error| error.to_string())?;

        let mut config_file = match fs::File::options()
            .read(true)
            .open(config_file_path)
        {
            Err(error) if matches!(error.kind(), io::ErrorKind::NotFound) => return Ok(None),
            result => result,
        }
        .map_err(|error| error.to_string())?;

        let config_file_len = config_file
            .metadata()
            .map_err(|error| error.to_string())?
            .len() as usize;

        let mut config_string = String::with_capacity(config_file_len);
        config_file
            .read_to_string(&mut config_string)
            .map_err(|error| error.to_string())?;

        Ok(Some(
            ron::from_str(&config_string).map_err(|error| error.to_string())?,
        ))
    }
}
