// SPDX-License-Identifier: GPL-3.0-only

use crate::app::config::AppTheme;
use crate::fl;
use cosmic::app::{message, Command, Core};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::window;
use cosmic::iced_core::keyboard::Key;
use cosmic::widget::menu::{
    action::MenuAction,
    key_bind::{KeyBind, Modifier},
};
use cosmic::widget::segmented_button::Entity;
use cosmic::{
    cosmic_config, cosmic_theme,
    iced::{Alignment, Length},
    ApplicationExt,
};
use cosmic::{widget, Application, Element};
use std::collections::{HashMap, VecDeque};
use std::{env, process};

pub mod config;
pub mod menu;

/// This is the struct that represents your application.
/// It is used to define the data that will be used by your application.
#[derive(Clone)]
pub struct App {
    /// This is the core of your application, it is used to communicate with the Cosmic runtime.
    /// It is used to send messages to your application, and to access the resources of the Cosmic runtime.
    core: Core,
    app_themes: Vec<String>,
    config_handler: Option<cosmic_config::Config>,
    config: config::CosmicBackupsConfig,
    context_page: ContextPage,
    dialog_pages: VecDeque<DialogPage>,
    dialog_text_input: widget::Id,
    key_binds: HashMap<KeyBind, Action>,
}

/// This is the enum that contains all the possible variants that your application will need to transmit messages.
/// This is used to communicate between the different parts of your application.
/// If your application does not need to send messages, you can use an empty enum or `()`.
#[derive(Debug, Clone)]
pub enum Message {
    // Cut(Option<Entity>),
    DialogCancel,
    DialogComplete,
    DialogUpdate(DialogPage),
    ToggleContextPage(ContextPage),
    LaunchUrl(String),
    OpenNewRepoDialog,
    AppTheme(usize),
    SystemThemeModeChange(cosmic_theme::ThemeMode),
    NewSnap,
    WindowClose,
    WindowNew,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContextPage {
    About,
    Settings,
}

impl ContextPage {
    fn title(&self) -> String {
        match self {
            Self::About => fl!("about"),
            Self::Settings => fl!("settings"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DialogPage {
    New(String),
}

#[derive(Clone, Debug)]
pub struct Flags {
    pub config_handler: Option<cosmic_config::Config>,
    pub config: config::CosmicBackupsConfig,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    About,
    NewRepo,
    NewSnap,
    // Cut,
    Settings,
    WindowClose,
    WindowNew,
}

impl MenuAction for Action {
    type Message = Message;
    fn message(&self, _entity_opt: Option<Entity>) -> Self::Message {
        match self {
            Action::About => Message::ToggleContextPage(ContextPage::About),
            // Action::Cut => Message::Cut(entity_opt),
            Action::NewRepo => Message::OpenNewRepoDialog,
            Action::NewSnap => Message::NewSnap,
            Action::Settings => Message::ToggleContextPage(ContextPage::Settings),
            Action::WindowClose => Message::WindowClose,
            Action::WindowNew => Message::WindowNew,
        }
    }
}

impl App {
    fn update_config(&mut self) -> Command<Message> {
        cosmic::app::command::set_theme(self.config.app_theme.theme())
    }

    fn about(&self) -> Element<Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = cosmic::theme::active().cosmic().spacing;
        let repository = "https://github.com/ahoneybun/cosmic-backups";
        let hash = env!("VERGEN_GIT_SHA");
        let short_hash: String = hash.chars().take(7).collect();
        let date = env!("VERGEN_GIT_COMMIT_DATE");
        widget::column::with_children(vec![
            widget::svg(widget::svg::Handle::from_memory(
                &include_bytes!(
                    "../res/icons/hicolor/128x128/apps/com.example.CosmicAppTemplate.svg"
                )[..],
            ))
            .into(),
            widget::text::title3(fl!("cosmic-backups")).into(),
            widget::button::link(repository)
                .on_press(Message::LaunchUrl(repository.to_string()))
                .padding(0)
                .into(),
            widget::button::link(fl!(
                "git-description",
                hash = short_hash.as_str(),
                date = date
            ))
            .on_press(Message::LaunchUrl(format!(
                "{}/commits/{}",
                repository, hash
            )))
            .padding(0)
            .into(),
        ])
        .align_items(Alignment::Center)
        .spacing(space_xxs)
        .into()
    }

    fn settings(&self) -> Element<Message> {
        let app_theme_selected = match self.config.app_theme {
            AppTheme::Dark => 1,
            AppTheme::Light => 2,
            AppTheme::System => 0,
        };
        widget::settings::view_column(vec![widget::settings::view_section(fl!("appearance"))
            .add(
                widget::settings::item::builder(fl!("theme")).control(widget::dropdown(
                    &self.app_themes,
                    Some(app_theme_selected),
                    Message::AppTheme,
                )),
            )
            .into()])
        .into()
    }
}

impl Application for App {
    type Executor = cosmic::executor::Default;

    type Flags = Flags;

    type Message = Message;

    const APP_ID: &'static str = "com.system76.CosmicBackups";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// This is the header of your application, it can be used to display the title of your application.
    fn header_start(&self) -> Vec<Element<Self::Message>> {
        vec![menu::menu_bar(&self.key_binds)]
    }

    fn init(core: Core, flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let app = App {
            core,
            app_themes: vec![fl!("match-desktop"), fl!("dark"), fl!("light")],
            context_page: ContextPage::Settings,
            config_handler: flags.config_handler,
            config: flags.config,
            dialog_pages: VecDeque::new(),
            dialog_text_input: widget::Id::unique(),
            key_binds: key_binds(),
        };

        (app, Command::none())
    }

    fn context_drawer(&self) -> Option<Element<Message>> {
        if !self.core.window.show_context {
            return None;
        }

        Some(match self.context_page {
            ContextPage::About => self.about(),
            ContextPage::Settings => self.settings(),
        })
    }

    fn dialog(&self) -> Option<Element<Message>> {
        let dialog_page = match self.dialog_pages.front() {
            Some(some) => some,
            None => return None,
        };

        let spacing = cosmic::theme::active().cosmic().spacing;

        let dialog = match dialog_page {
            DialogPage::New(name) => widget::dialog(fl!("create-repo"))
                .primary_action(
                    widget::button::suggested(fl!("save"))
                        .on_press_maybe(Some(Message::DialogComplete)),
                )
                .secondary_action(
                    widget::button::standard(fl!("cancel")).on_press(Message::DialogCancel),
                )
                .control(
                    widget::column::with_children(vec![
                        widget::text::body(fl!("repo-location")).into(),
                        widget::text_input("", name.as_str())
                            .id(self.dialog_text_input.clone())
                            .on_input(move |name| Message::DialogUpdate(DialogPage::New(name)))
                            .into(),
                    ])
                    .spacing(spacing.space_xxs),
                ),
        };

        Some(dialog.into())
    }

    fn view(&self) -> Element<Self::Message> {
        widget::container(widget::text::title1(fl!("welcome")))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into()
    }

    /// Handle application events here.
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        // Helper for updating config values efficiently
        macro_rules! config_set {
            ($name: ident, $value: expr) => {
                match &self.config_handler {
                    Some(config_handler) => {
                        match paste::paste! { self.config.[<set_ $name>](config_handler, $value) } {
                            Ok(_) => {}
                            Err(err) => {
                                log::warn!(
                                    "failed to save config {:?}: {}",
                                    stringify!($name),
                                    err
                                );
                            }
                        }
                    }
                    None => {
                        self.config.$name = $value;
                        log::warn!(
                            "failed to save config {:?}: no config handler",
                            stringify!($name)
                        );
                    }
                }
            };
        }

        match message {
            Message::ToggleContextPage(context_page) => {
                //TODO: ensure context menus are closed
                if self.context_page == context_page {
                    self.core.window.show_context = !self.core.window.show_context;
                } else {
                    self.context_page = context_page;
                    self.core.window.show_context = true;
                }
                self.set_context_title(context_page.title());
            }
            Message::OpenNewRepoDialog => {
                self.dialog_pages.push_back(DialogPage::New(String::new()));
                return widget::text_input::focus(self.dialog_text_input.clone());
            }
            Message::NewSnap => {
                println!("snapshot saved");
            }
            Message::DialogCancel => {
                self.dialog_pages.pop_front();
            }
            Message::DialogComplete => {
                if let Some(dialog_page) = self.dialog_pages.pop_front() {
                    match dialog_page {
                        DialogPage::New(name) => {}
                    }
                }
            }
            Message::WindowClose => {
                return window::close(window::Id::MAIN);
            }
            Message::WindowNew => match env::current_exe() {
                Ok(exe) => match process::Command::new(&exe).spawn() {
                    Ok(_child) => {}
                    Err(err) => {
                        eprintln!("failed to execute {:?}: {}", exe, err);
                    }
                },
                Err(err) => {
                    eprintln!("failed to get current executable path: {}", err);
                }
            }
            Message::DialogUpdate(dialog_page) => {
                //TODO: panicless way to do this?
                self.dialog_pages[0] = dialog_page;
            }
            Message::LaunchUrl(url) => match open::that_detached(&url) {
                Ok(()) => {}
                Err(err) => {
                    log::warn!("failed to open {:?}: {}", url, err);
                }
            },
            Message::AppTheme(index) => {
                let app_theme = match index {
                    1 => AppTheme::Dark,
                    2 => AppTheme::Light,
                    _ => AppTheme::System,
                };
                config_set!(app_theme, app_theme);
                return self.update_config();
            }
            Message::SystemThemeModeChange(_) => {
                return self.update_config();
            }
        }

        Command::none()
    }
}

pub fn key_binds() -> HashMap<KeyBind, Action> {
    let mut key_binds = HashMap::new();

    macro_rules! bind {
        ([$($modifier:ident),* $(,)?], $key:expr, $action:ident) => {{
            key_binds.insert(
                KeyBind {
                    modifiers: vec![$(Modifier::$modifier),*],
                    key: $key,
                },
                Action::$action,
            );
        }};
    }

    bind!([Ctrl], Key::Character("w".into()), WindowClose);
    bind!([Ctrl, Shift], Key::Character("n".into()), WindowNew);

    key_binds
}
