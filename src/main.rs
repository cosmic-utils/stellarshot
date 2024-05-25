// SPDX-License-Identifier: GPL-3.0-only

use crate::app::App;
use app::{config::CosmicBackupsConfig, Flags};
use cosmic::app::Settings;

use tracing::info;
use tracing_subscriber;

/// The `app` module is used by convention to indicate the main component of our application.
mod app;
mod backup;
mod core;

/// The `cosmic::app::run()` function is the starting point of your application.
/// It takes two arguments:
/// - `settings` is a structure that contains everything relevant with your app's configuration, such as antialiasing, themes, icons, etc...
/// - `()` is the flags that your app needs to use before it starts.
///  If your app does not need any flags, you can pass in `()`.
fn main() -> cosmic::iced::Result {
    tracing_subscriber::fmt()
        .json()
        .init();

    let (settings, flags) = settings();
    cosmic::app::run::<App>(settings, flags)
}

fn settings() -> (Settings, Flags) {
    let settings = Settings::default();
    let flags = Flags {
        config_handler: CosmicBackupsConfig::config_handler(),
        config: CosmicBackupsConfig::config(),
    };
    (settings, flags)
}
