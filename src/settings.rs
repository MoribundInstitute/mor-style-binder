use crate::core::ExportMode;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub const DEFAULT_CONFIG_FILE: &str = "./config.toml";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DefaultExportMode {
    #[default]
    PlainCssBundle,
    BloggerSkinWrapper,
}

impl DefaultExportMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::PlainCssBundle => "Plain CSS Bundle",
            Self::BloggerSkinWrapper => "Blogger Skin Wrapper",
        }
    }

    pub fn toggled(self) -> Self {
        match self {
            Self::PlainCssBundle => Self::BloggerSkinWrapper,
            Self::BloggerSkinWrapper => Self::PlainCssBundle,
        }
    }
}

impl From<DefaultExportMode> for ExportMode {
    fn from(mode: DefaultExportMode) -> Self {
        match mode {
            DefaultExportMode::PlainCssBundle => ExportMode::Standard,
            DefaultExportMode::BloggerSkinWrapper => ExportMode::BloggerXml,
        }
    }
}

impl From<ExportMode> for DefaultExportMode {
    fn from(mode: ExportMode) -> Self {
        match mode {
            ExportMode::Standard => DefaultExportMode::PlainCssBundle,
            ExportMode::BloggerXml => DefaultExportMode::BloggerSkinWrapper,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    pub confirm_before_overwriting_sheets: bool,
    pub default_export_mode: DefaultExportMode,
    pub preview_font_size: f32,
    pub remember_window_size: bool,
    pub auto_inspect_import_after_drop: bool,
    pub show_verbose_status_messages: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            confirm_before_overwriting_sheets: true,
            default_export_mode: DefaultExportMode::PlainCssBundle,
            preview_font_size: 14.0,
            remember_window_size: true,
            auto_inspect_import_after_drop: true,
            show_verbose_status_messages: true,
        }
    }
}

pub fn load_or_create_default_settings() -> std::io::Result<AppSettings> {
    load_or_create_settings(DEFAULT_CONFIG_FILE)
}

pub fn load_or_create_settings(path: impl AsRef<Path>) -> std::io::Result<AppSettings> {
    let path = path.as_ref();

    if path.exists() {
        return load_settings(path);
    }

    let settings = AppSettings::default();
    save_settings(path, &settings)?;
    Ok(settings)
}

pub fn load_settings(path: impl AsRef<Path>) -> std::io::Result<AppSettings> {
    let text = fs::read_to_string(path)?;
    let settings = toml::from_str::<AppSettings>(&text).map_err(toml_error_to_io_error)?;
    Ok(settings)
}

pub fn save_settings(path: impl AsRef<Path>, settings: &AppSettings) -> std::io::Result<()> {
    let path = path.as_ref();

    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    let text = toml::to_string_pretty(settings).map_err(toml_error_to_io_error)?;
    fs::write(path, text)
}

fn toml_error_to_io_error(error: impl std::error::Error + Send + Sync + 'static) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings_are_sensible() {
        let settings = AppSettings::default();

        assert!(settings.confirm_before_overwriting_sheets);
        assert_eq!(
            settings.default_export_mode,
            DefaultExportMode::PlainCssBundle
        );
        assert_eq!(settings.preview_font_size, 14.0);
        assert!(settings.remember_window_size);
        assert!(settings.auto_inspect_import_after_drop);
        assert!(settings.show_verbose_status_messages);
    }

    #[test]
    fn settings_round_trip_through_toml() {
        let settings = AppSettings {
            confirm_before_overwriting_sheets: false,
            default_export_mode: DefaultExportMode::BloggerSkinWrapper,
            preview_font_size: 16.0,
            remember_window_size: false,
            auto_inspect_import_after_drop: false,
            show_verbose_status_messages: false,
        };

        let text = toml::to_string_pretty(&settings).expect("settings should serialize");
        let parsed = toml::from_str::<AppSettings>(&text).expect("settings should deserialize");

        assert_eq!(settings, parsed);
        assert!(text.contains("blogger_skin_wrapper"));
    }
}
