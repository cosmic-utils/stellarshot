use std::sync::Mutex;

use super::config::StellarshotConfig;
use super::icon_cache::{IconCache, ICON_CACHE};
use crate::app::Flags;
use cosmic::app::Settings;
use cosmic::iced::{Limits, Size};

pub fn init() -> (Settings, Flags) {
    set_logger();
    set_icon_cache();
    let settings = get_app_settings();
    let flags = get_flags();
    (settings, flags)
}

pub fn get_app_settings() -> Settings {
    let config = StellarshotConfig::config();

    let mut settings = Settings::default();
    settings = settings.theme(config.app_theme.theme());
    settings = settings.size_limits(Limits::NONE.min_width(400.0).min_height(180.0));
    settings = settings.size(Size::new(800.0, 800.0));
    settings = settings.debug(false);
    settings
}

pub fn set_logger() {
    tracing_subscriber::fmt().init();
}

pub fn set_icon_cache() {
    ICON_CACHE.get_or_init(|| Mutex::new(IconCache::new()));
}

pub fn get_flags() -> Flags {
    let (config_handler, config) = (
        StellarshotConfig::config_handler(),
        StellarshotConfig::config(),
    );

    let flags = Flags {
        config_handler,
        config,
    };
    flags
}
