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
}

#[derive(Debug, Clone)]
pub enum Message {
    SetRepository(Repository),
    SetSnapshots(Vec<SnapshotFile>),
    Delete(Id),
    Select(Id),
}

pub enum Command {
    FetchSnapshots(String, String),
}

impl Content {
    pub fn new() -> Self {
        Self {
            repository: None,
            snapshots: None,
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
            Message::SetRepository(repository) => {
                self.repository = Some(repository.clone());
                let path = repository.path.display().to_string();
                commands.push(Command::FetchSnapshots(path, "password".into()))
            }
            Message::SetSnapshots(snapshots) => self.snapshots = Some(snapshots),
            Message::Delete(_) => todo!(),
            Message::Select(_) => todo!(),
        }
        commands
    }

    pub fn list_view<'a>(&'a self, repository: &'a Repository) -> Element<'a, Message> {
        let spacing = theme::active().cosmic().spacing;

        if self.snapshots.is_none() {
            return self.loading();
        }

        if self.snapshots.as_ref().unwrap().is_empty() {
            return self.empty(repository);
        }

        let mut items = widget::list::list_column()
            .style(theme::Container::ContextDrawer)
            .spacing(spacing.space_xxxs)
            .padding([spacing.space_none, spacing.space_xxs]);

        for item in self.snapshots.as_ref().unwrap() {
            let delete_button = widget::button(IconCache::get("user-trash-full-symbolic", 18))
                .padding(spacing.space_xxs)
                .style(theme::Button::Destructive)
                .on_press(Message::Delete(item.id));

            let details_button = widget::button(IconCache::get("info-outline-symbolic", 18))
                .padding(spacing.space_xxs)
                .style(theme::Button::Standard)
                .on_press(Message::Select(item.id));

            let task_item_text = widget::text(item.id.to_string()).width(Length::Fill);

            let row = widget::row::with_capacity(4)
                .align_items(Alignment::Center)
                .spacing(spacing.space_xxs)
                .padding([spacing.space_xxxs, spacing.space_xxs])
                .push(task_item_text)
                .push(details_button)
                .push(delete_button);

            items = items.add(row);
        }

        widget::column::with_capacity(2)
            .spacing(spacing.space_xxs)
            .push(self.repository_header(repository))
            .push(items)
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
            .padding([spacing.space_none, spacing.space_xxs])
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
