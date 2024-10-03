use cosmic::{
    iced::{
        alignment::{Horizontal, Vertical},
        Alignment, Length,
    },
    theme, widget, Apply, Element,
};
use rustic_core::{repofile::SnapshotFile, Id};

use crate::{
    app::{config::Repository, icon_cache::IconCache},
    backup::snapshot::fetch,
    fl,
};

pub struct Content {
    pub repository: Option<Repository>,
    snapshots: Option<Vec<SnapshotFile>>,
    password: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    SetRepository(Repository, String),
    SetSnapshots(Vec<SnapshotFile>),
    ReloadSnapshots,
    Delete(Id, String),
    Select(Id),
}

pub enum Command {
    FetchSnapshots(String, String),
    DeleteSnapshots(String, String, Vec<rustic_core::Id>),
}

impl Content {
    pub fn new() -> Self {
        Self {
            repository: None,
            snapshots: None,
            password: String::new(),
        }
    }

    pub fn snapshots(repository: &str, password: &str) -> Vec<SnapshotFile> {
        match fetch(repository, password) {
            Ok(snapshots) => snapshots,
            Err(err) => {
                log::error!("error getting snapshots: {:?}", err);
                Vec::new()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let spacing = theme::active().cosmic().spacing;
        let Some(ref repository) = self.repository else {
            return widget::container(
                widget::column::with_children(vec![
                    IconCache::get("harddisk-symbolic", 56).into(),
                    widget::text::title1(fl!("no-repository-selected")).into(),
                    widget::text(fl!("no-repository-suggestion")).into(),
                ])
                .spacing(10)
                .align_items(Alignment::Center),
            )
            .align_y(Vertical::Center)
            .align_x(Horizontal::Center)
            .height(Length::Fill)
            .width(Length::Fill)
            .into();
        };

        widget::column::with_capacity(2)
            .push(self.list_view(repository))
            .spacing(spacing.space_xxs)
            .apply(widget::container)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }

    pub fn update(&mut self, message: Message) -> Vec<Command> {
        let mut commands = vec![];
        match message {
            Message::SetRepository(repository, password) => {
                self.password = password;
                self.snapshots = None;
                self.repository = Some(repository.clone());
                let path = repository.path.display().to_string();
                commands.push(Command::FetchSnapshots(path, self.password.clone()))
            }
            Message::SetSnapshots(snapshots) => self.snapshots = Some(snapshots),
            Message::Delete(id, password) => {
                let path = self.repository.as_ref().unwrap().path.display().to_string();
                commands.push(Command::DeleteSnapshots(path, password, vec![id]))
            }
            Message::Select(_) => todo!(),
            Message::ReloadSnapshots => {
                let path = self.repository.as_ref().unwrap().path.display().to_string();
                commands.push(Command::FetchSnapshots(path, "password".into()))
            }
        }
        commands
    }

    pub fn list_view<'a>(&'a self, repository: &'a Repository) -> Element<'a, Message> {
        let spacing = theme::active().cosmic().spacing;

        let Some(ref snapshots) = self.snapshots else {
            return self.loading();
        };

        if snapshots.is_empty() {
            return self.empty(repository);
        }

        let mut section = widget::settings::view_section(fl!("snapshots"));

        for item in snapshots {
            let delete_button = widget::button(IconCache::get("user-trash-full-symbolic", 18))
                .padding(spacing.space_xxs)
                .style(theme::Button::Destructive)
                .on_press(Message::Delete(item.id, self.password.clone()));

            let _details_button = widget::button(IconCache::get("info-outline-symbolic", 18))
                .padding(spacing.space_xxs)
                .style(theme::Button::Standard)
                .on_press(Message::Select(item.id));

            let row = widget::settings::item(
                item.id.to_string(),
                widget::row::with_capacity(4)
                    .align_items(Alignment::Center)
                    .spacing(spacing.space_xxs)
                    .padding([spacing.space_xxxs, spacing.space_xxs])
                    // .push(details_button)
                    .push(delete_button),
            );

            section = section.add(row);
        }

        widget::column::with_capacity(2)
            .spacing(spacing.space_xxs)
            .padding(spacing.space_xxs)
            .push(self.repository_header(repository))
            .push(section)
            .apply(widget::container)
            .height(Length::Shrink)
            .apply(widget::scrollable)
            .height(Length::Fill)
            .into()
    }

    pub fn empty<'a>(&'a self, repository: &'a Repository) -> Element<'a, Message> {
        let spacing = theme::active().cosmic().spacing;

        let container = widget::container(
            widget::column::with_children(vec![
                IconCache::get("box-outline-symbolic", 56).into(),
                widget::text::title1(fl!("no-snapshots")).into(),
                widget::text(fl!("no-snapshots-suggestion")).into(),
            ])
            .spacing(10)
            .align_items(Alignment::Center),
        )
        .align_y(Vertical::Center)
        .align_x(Horizontal::Center)
        .height(Length::Fill)
        .width(Length::Fill);

        widget::column::with_capacity(2)
            .spacing(spacing.space_xxs)
            .push(self.repository_header(repository))
            .push(container)
            .into()
    }

    fn repository_header<'a>(&'a self, repository: &'a Repository) -> Element<'a, Message> {
        let spacing = theme::active().cosmic().spacing;

        widget::row::with_capacity(3)
            .align_items(Alignment::Center)
            .spacing(spacing.space_s)
            .push(widget::text::title3(&repository.name).width(Length::Fill))
            .into()
    }

    pub fn loading(&self) -> Element<Message> {
        widget::container(
            widget::column::with_children(vec![
                IconCache::get("hourglass-symbolic", 56).into(),
                widget::text::title1(fl!("loading")).into(),
                widget::text(fl!("loading-snapshots")).into(),
            ])
            .spacing(10)
            .align_items(Alignment::Center),
        )
        .align_y(Vertical::Center)
        .align_x(Horizontal::Center)
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
    }
}
