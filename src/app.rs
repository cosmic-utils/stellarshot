// SPDX-License-Identifier: GPL-3.0-only

use std::any::TypeId;
use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::{env, process};

use cosmic::app::{message, Command, Core};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::{event, keyboard::Event as KeyEvent, window, Event, Subscription};
use cosmic::iced_core::keyboard::{Key, Modifiers};
use cosmic::widget::menu::{action::MenuAction, key_bind::KeyBind};
use cosmic::widget::segmented_button::{self, EntityMut, SingleSelect};
use cosmic::{
    cosmic_config, cosmic_theme,
    iced::{Alignment, Length},
    ApplicationExt,
};
use cosmic::{widget, Application, Apply, Element};
use views::content::{self, Content};

use crate::app::config::{AppTheme, Repository, CONFIG_VERSION};
use crate::app::key_bind::key_binds;
use crate::fl;

use self::icon_cache::IconCache;

pub mod config;
pub mod icon_cache;
mod key_bind;
pub mod menu;
pub mod settings;
pub mod views;

/// This is the struct that represents your application.
/// It is used to define the data that will be used by your application.
pub struct App {
    /// This is the core of your application, it is used to communicate with the Cosmic runtime.
    /// It is used to send messages to your application, and to access the resources of the Cosmic runtime.
    core: Core,
    nav_model: segmented_button::SingleSelectModel,
    content: Content,
    app_themes: Vec<String>,
    config_handler: Option<cosmic_config::Config>,
    config: config::StellarshotConfig,
    context_page: ContextPage,
    dialog_pages: VecDeque<DialogPage>,
    dialog_text_input: widget::Id,
    key_binds: HashMap<KeyBind, Action>,
    modifiers: Modifiers,
}

/// This is the enum that contains all the possible variants that your application will need to transmit messages.
/// This is used to communicate between the different parts of your application.
/// If your application does not need to send messages, you can use an empty enum or `()`.
#[derive(Debug, Clone)]
pub enum Message {
    Content(content::Message),
    DialogCancel,
    DialogComplete,
    DialogUpdate(DialogPage),
    ToggleContextPage(ContextPage),
    LaunchUrl(String),
    AppTheme(usize),
    SystemThemeModeChange(cosmic_theme::ThemeMode),
    Key(Modifiers, Key),
    Modifiers(Modifiers),
    WindowClose,
    WindowNew,
    Repository(RepositoryAction),
    CreateSnapshot,
    OpenCreateRepositoryDialog,
    OpenCreateSnapshotDialog,
    DeleteRepositoryDialog,
    DeleteSnapshotDialog,
}

#[derive(Debug, Clone)]
pub enum RepositoryAction {
    Init(String),
    Created(Repository),
    Error(String),
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
    pub config: config::StellarshotConfig,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    About,
    CreateRepository,
    CreateSnapshot,
    DeleteRepository,
    DeleteSnapshot,
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
            Action::DeleteRepository => Message::DeleteRepositoryDialog,
            Action::DeleteSnapshot => Message::DeleteSnapshotDialog,
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
        let repository = "https://github.com/ahoneybun/Stellarshot";
        let hash = env!("VERGEN_GIT_SHA");
        let short_hash: String = hash.chars().take(7).collect();
        let date = env!("VERGEN_GIT_COMMIT_DATE");
        widget::column::with_children(vec![
            widget::svg(widget::svg::Handle::from_memory(
                &include_bytes!(
                    "../res/icons/hicolor/scalable/apps/com.github.ahoneybun.Stellarshot.svg"
                )[..],
            ))
            .into(),
            widget::text::title3(fl!("stellarshot")).into(),
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

    fn create_nav_item(
        &mut self,
        repository: Repository,
        icon: &'static str,
    ) -> EntityMut<SingleSelect> {
        self.nav_model
            .insert()
            .icon(IconCache::get(icon, 18))
            .text(repository.name.clone())
            .data(repository.clone())
            .activate()
    }
}

impl Application for App {
    type Executor = cosmic::executor::Default;

    type Flags = Flags;

    type Message = Message;

    const APP_ID: &'static str = "com.github.ahoneybun.Stellarshot";

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
            content: Content::new(),
            app_themes: vec![fl!("match-desktop"), fl!("dark"), fl!("light")],
            context_page: ContextPage::Settings,
            config_handler: flags.config_handler,
            config: flags.config,
            dialog_pages: VecDeque::new(),
            dialog_text_input: widget::Id::unique(),
            key_binds: key_binds(),
            modifiers: Modifiers::empty(),
        };

        let repositories = app.config.repositories.clone();
        for repository in repositories {
            app.create_nav_item(repository, "harddisk-symbolic");
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
            let name = repository.name.clone();
            commands.push(
                self.update(Message::Content(content::Message::SetRepository(
                    repository.clone(),
                ))),
            );
            let window_title = format!("{} - {}", name, fl!("stellarshot"));
            commands.push(self.set_window_title(window_title, self.main_window_id()));
        }

        Command::batch(commands)
    }

    fn view(&self) -> Element<Self::Message> {
        widget::container(self.content.view().map(Message::Content))
            .apply(widget::container)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        struct ConfigSubscription;
        struct ThemeSubscription;

        let subscriptions = vec![
            event::listen_with(|event, status| match event {
                Event::Keyboard(KeyEvent::KeyPressed { key, modifiers, .. }) => match status {
                    event::Status::Ignored => Some(Message::Key(modifiers, key)),
                    event::Status::Captured => None,
                },
                Event::Keyboard(KeyEvent::ModifiersChanged(modifiers)) => {
                    Some(Message::Modifiers(modifiers))
                }
                _ => None,
            }),
            cosmic_config::config_subscription(
                TypeId::of::<ConfigSubscription>(),
                Self::APP_ID.into(),
                CONFIG_VERSION,
            )
            .map(|update| {
                if !update.errors.is_empty() {
                    log::info!(
                        "errors loading config {:?}: {:?}",
                        update.keys,
                        update.errors
                    );
                }
                Message::SystemThemeModeChange(update.config)
            }),
            cosmic_config::config_subscription::<_, cosmic_theme::ThemeMode>(
                TypeId::of::<ThemeSubscription>(),
                cosmic_theme::THEME_MODE_ID.into(),
                cosmic_theme::ThemeMode::version(),
            )
            .map(|update| {
                if !update.errors.is_empty() {
                    log::info!(
                        "errors loading theme mode {:?}: {:?}",
                        update.keys,
                        update.errors
                    );
                }
                Message::SystemThemeModeChange(update.config)
            }),
        ];

        // subscriptions.push(self.content.subscription().map(Message::Content));

        Subscription::batch(subscriptions)
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
            Message::Content(message) => {
                let commands = self.content.update(message);
                for command in commands {
                    match command {
                        content::Command::FetchSnapshots(repository, password) => {
                            return Command::perform(
                                async move { Content::snapshots(&repository, &password) },
                                |result| {
                                    cosmic::app::Message::App(Message::Content(
                                        content::Message::SetSnapshots(result),
                                    ))
                                },
                            )
                        }
                    }
                }
            }
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
            Message::Repository(state) => match state {
                RepositoryAction::Init(path) => {
                    let init_path = path.clone();
                    let name = PathBuf::from(&path)
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    let repository = Repository {
                        name,
                        path: PathBuf::from(&path),
                    };
                    self.create_nav_item(repository.clone(), "timer-sand-symbolic");
                    return Command::perform(
                        async move { crate::backup::init(&init_path, "password") },
                        |result| match result {
                            Ok(_) => message::app(Message::Repository(RepositoryAction::Created(
                                repository,
                            ))),
                            Err(e) => message::app(Message::Repository(RepositoryAction::Error(
                                e.to_string(),
                            ))),
                        },
                    );
                }
                RepositoryAction::Created(repository) => {
                    if self.nav_model.active_data::<Repository>().is_some() {
                        let entity = self.nav_model.active();
                        self.nav_model
                            .icon_set(entity, IconCache::get("harddisk-symbolic", 18));
                    }
                    let mut repositories = self.config.repositories.clone();
                    repositories.push(repository);
                    config_set!(repositories, repositories);
                }
                RepositoryAction::Error(error) => log::error!("{}", error),
            },
            Message::DeleteRepositoryDialog => {
                println!("Deleting repository");
            }
            Message::DeleteSnapshotDialog => {
                println!("Deleting snapshot");
            }
            Message::CreateSnapshot => {
                if let Some(repository) = &self.content.repository {
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
                        DialogPage::CreateRepository(path) => {
                            return self.update(Message::Repository(RepositoryAction::Init(path)));
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
            Message::Key(modifiers, key) => {
                for (key_bind, action) in self.key_binds.iter() {
                    if key_bind.matches(modifiers, &key) {
                        return self.update(action.message());
                    }
                }
            }
            Message::Modifiers(modifiers) => {
                self.modifiers = modifiers;
            }
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
