// SPDX-License-Identifier: GPL-3.0-only

use crate::app::config::{AppTheme, Repository};
use crate::fl;
use cosmic::app::{Command, Core};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::window;
use cosmic::iced_core::keyboard::Key;
use cosmic::widget::menu::{
    action::MenuAction,
    key_bind::{KeyBind, Modifier},
};
use cosmic::widget::segmented_button::{self, EntityMut, SingleSelect};
use cosmic::{
    cosmic_config, cosmic_theme,
    iced::{Alignment, Length},
    ApplicationExt,
};
use cosmic::{widget, Application, Apply, Element};
use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::{env, process};

pub mod config;
pub mod menu;

/// This is the struct that represents your application.
/// It is used to define the data that will be used by your application.
pub struct App {
    /// This is the core of your application, it is used to communicate with the Cosmic runtime.
    /// It is used to send messages to your application, and to access the resources of the Cosmic runtime.
    core: Core,
    nav_model: segmented_button::SingleSelectModel,
    selected_repository: Option<Repository>,
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
    DialogCancel,
    DialogComplete,
    DialogUpdate(DialogPage),
    ToggleContextPage(ContextPage),
    LaunchUrl(String),
    AppTheme(usize),
    SystemThemeModeChange(cosmic_theme::ThemeMode),
    WindowClose,
    WindowNew,
    CreateRepository(String),
    CreateSnapshot,
    OpenCreateRepositoryDialog,
    OpenCreateSnapshotDialog,
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
    CreateRepository(String),
    CreateSnapshot,
}

#[derive(Clone, Debug)]
pub struct Flags {
    pub config_handler: Option<cosmic_config::Config>,
    pub config: config::CosmicBackupsConfig,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    About,
    CreateRepository,
    CreateSnapshot,
    Settings,
    WindowClose,
    WindowNew,
}

impl MenuAction for Action {
    type Message = Message;
    fn message(&self) -> Self::Message {
        match self {
            Action::About => Message::ToggleContextPage(ContextPage::About),
            Action::CreateRepository => Message::OpenCreateRepositoryDialog,
            Action::CreateSnapshot => Message::OpenCreateSnapshotDialog,
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

    fn create_nav_item(&mut self, repository: Repository) -> EntityMut<SingleSelect> {
        self.nav_model
            .insert()
            .text(repository.name.clone())
            .data(repository.clone())
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

    fn nav_model(&self) -> Option<&widget::nav_bar::Model> {
        Some(&self.nav_model)
    }

    fn init(core: Core, flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let nav_model = segmented_button::ModelBuilder::default().build();
        let mut app = App {
            core,
            nav_model,
            selected_repository: None,
            app_themes: vec![fl!("match-desktop"), fl!("dark"), fl!("light")],
            context_page: ContextPage::Settings,
            config_handler: flags.config_handler,
            config: flags.config,
            dialog_pages: VecDeque::new(),
            dialog_text_input: widget::Id::unique(),
            key_binds: key_binds(),
        };

        let repositories = app.config.repositories.clone();
        for repository in repositories {
            app.create_nav_item(repository);
        }

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
            DialogPage::CreateRepository(name) => widget::dialog(fl!("create-repo"))
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
                            .on_input(move |name| {
                                Message::DialogUpdate(DialogPage::CreateRepository(name))
                            })
                            .into(),
                    ])
                    .spacing(spacing.space_xxs),
                ),
            DialogPage::CreateSnapshot => widget::dialog(fl!("create-snapshot"))
                .body(fl!("snapshot-description"))
                .primary_action(
                    widget::button::suggested(fl!("create"))
                        .on_press_maybe(Some(Message::DialogComplete)),
                )
                .secondary_action(
                    widget::button::standard(fl!("cancel")).on_press(Message::DialogCancel),
                ),
        };

        Some(dialog.into())
    }

    fn on_nav_select(&mut self, entity: widget::nav_bar::Id) -> Command<Self::Message> {
        let mut commands = vec![];
        self.nav_model.activate(entity);

        if let Some(repository) = self.nav_model.data::<Repository>(entity) {
            self.selected_repository = Some(repository.clone());
            let window_title = format!("{} - {}", repository.name, fl!("cosmic-backups"));
            commands.push(self.set_window_title(window_title));
        }

        Command::batch(commands)
    }

    fn view(&self) -> Element<Self::Message> {
        let content: Element<Self::Message> = match &self.selected_repository {
            Some(repository) => {
                widget::text::title1(format!("Selected repository: {}", repository.name.clone()))
                    .into()
            }
            None => widget::text::title1(fl!("welcome")).into(),
        };
        widget::container(content)
            .apply(widget::container)
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
            Message::OpenCreateRepositoryDialog => {
                self.dialog_pages
                    .push_back(DialogPage::CreateRepository(String::new()));
                return widget::text_input::focus(self.dialog_text_input.clone());
            }
            Message::OpenCreateSnapshotDialog => {
                self.dialog_pages.push_back(DialogPage::CreateSnapshot);
            }
            Message::CreateRepository(path) => match crate::backup::init(&path, "password") {
                Ok(_) => {
                    let path = PathBuf::from(&path);
                    let name = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    let repository = Repository { name, path };
                    self.config.repositories.push(repository.clone());
                    self.create_nav_item(repository);
                    config_set!(repositories, self.config.repositories.clone());
                }
                Err(e) => {
                    // TODO: Show error to user.
                    eprintln!("failed to create repository: {}", e)
                }
            },
            Message::CreateSnapshot => {
                if let Some(repository) = &self.selected_repository {
                    let Some(path) = repository.path.to_str() else {
                        return Command::none();
                    };
                    match crate::backup::snapshot(path, "password", vec!["/etc"]) {
                        Ok(_) => {}
                        Err(e) => {
                            // TODO: Show error to user.
                            eprintln!("failed to create snapshot: {}", e)
                        }
                    }
                }
            }
            Message::DialogCancel => {
                self.dialog_pages.pop_front();
            }
            Message::DialogComplete => {
                if let Some(dialog_page) = self.dialog_pages.pop_front() {
                    match dialog_page {
                        DialogPage::CreateRepository(name) => {
                            return self.update(Message::CreateRepository(name));
                        }
                        DialogPage::CreateSnapshot => {
                            return self.update(Message::CreateSnapshot);
                        }
                    }
                }
            }
            Message::DialogUpdate(dialog_page) => {
                //TODO: panicless way to do this?
                self.dialog_pages[0] = dialog_page;
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
            },
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

    bind!([Ctrl], Key::Character("r".into()), CreateRepository);
    bind!([Ctrl, Shift], Key::Character("r".into()), CreateSnapshot);
    bind!([Ctrl], Key::Character("w".into()), WindowClose);
    bind!([Ctrl, Shift], Key::Character("n".into()), WindowNew);

    key_binds
}
