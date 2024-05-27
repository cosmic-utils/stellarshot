use super::config::CosmicBackupsConfig;
use crate::app::Flags;
use cosmic::app::Settings;
use cosmic::iced::{Limits, Size};

pub fn init() -> (Settings, Flags) {
    set_logger();
    let settings = get_app_settings();
    let flags = get_flags();
    (settings, flags)
}

pub fn get_app_settings() -> Settings {
    let config = CosmicBackupsConfig::config();

    let mut settings = Settings::default();
    settings = settings.theme(config.app_theme.theme());
    settings = settings.size_limits(Limits::NONE.min_width(400.0).min_height(180.0));
    settings = settings.size(Size::new(800.0, 800.0));
    settings = settings.debug(false);
    settings
}

pub fn set_logger() {
    tracing_subscriber::fmt().json().init();
}

pub fn get_flags() -> Flags {
    let (config_handler, config) = (
        CosmicBackupsConfig::config_handler(),
        CosmicBackupsConfig::config(),
    );

    let flags = Flags {
        config_handler,
        config,
    };
    flags
}
