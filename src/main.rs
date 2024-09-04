// SPDX-License-Identifier: GPL-3.0-only

use crate::app::App;
use app::settings;

mod app;
mod backup;
mod core;

fn main() -> cosmic::iced::Result {
    let (settings, flags) = settings::init();
    cosmic::app::run::<App>(settings, flags)
}
