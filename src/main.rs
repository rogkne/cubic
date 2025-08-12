mod actions;
mod arch;
mod commands;
mod emulator;
mod env;
mod error;
mod fs;
mod gui;
mod image;
mod instance;
mod qemu;
mod ssh_cmd;
mod util;
mod view;
mod web;

use crate::commands::CommandDispatcher;

fn main() -> iced::Result {
    let result = iced::application("Cubic", MyApp::update, MyApp::view)
        .theme(|_| iced::Theme::Dark)
        .window(iced::window::Settings {
            size: iced::Size {
                width: 600.,
                height: 400.,
            },
            min_size: Some(iced::Size {
                width: 600.,
                height: 400.,
            }),
            ..iced::window::Settings::default()
        })
        .run();

    CommandDispatcher::new()
        .dispatch()
        .map_err(error::print_error)
        .ok();

    result
}

use gui::{Message, Selection};
use iced::widget::{button, column, row, text, Column};
use iced::Length;

const FIELD_BG_COLOR: iced::Color = iced::Color::from_rgb(0.204, 0.231, 0.282);
const SIDEBAR_BG_COLOR: iced::Color = iced::Color::from_rgb(0.106, 0.118, 0.137);
const CONTENT_BG_COLOR: iced::Color = iced::Color::from_rgb(0.173, 0.192, 0.235);

#[derive(Default)]
struct MyApp {
    instances: Vec<instance::Instance>,
    selection: Selection,
    field: String,
    instance_view: gui::InstanceView,
}

impl MyApp {
    fn create_container(
        widget: iced::Element<Message>,
        background: iced::Color,
    ) -> iced::widget::Container<Message> {
        iced::widget::Container::new(widget).style(move |_| {
            iced::widget::container::Style::default()
                .background(iced::Background::Color(background))
        })
    }

    fn create_sidebar_button(
        &self,
        icon: String,
        name: String,
        selected: bool,
    ) -> iced::widget::Button<Message> {
        let color = if selected {
            CONTENT_BG_COLOR
        } else {
            SIDEBAR_BG_COLOR
        };

        button(row![text!("{icon}").width(20), text!("{name}")])
            .width(180)
            .padding(iced::Padding::from(10))
            .style(move |_, _| iced::widget::button::Style {
                text_color: iced::Color::WHITE,
                background: Some(iced::Background::Color(color)),
                border: iced::Border {
                    color,
                    width: 1.,
                    radius: iced::border::Radius::new(5.0)
                        .top_right(0.)
                        .bottom_right(0.),
                },
                ..Default::default()
            })
    }

    fn create_logo(&self) -> iced::Element<Message> {
        Self::create_container(
            iced::widget::svg("docs/logo.svg").into(),
            iced::Color::BLACK,
        )
        .width(200)
        .padding(iced::Padding::from(20))
        .align_y(iced::alignment::Vertical::Center)
        .into()
    }

    fn create_sidebar(&self) -> iced::Element<Message> {
        let mut view = Column::new().spacing(5).width(Length::Fill);

        view = view.push(
            self.create_sidebar_button(
                "-".to_string(),
                "Overview".to_string(),
                self.selection == Selection::Overview,
            )
            .on_press(Message::SelectionChanged(Selection::Overview)),
        );

        view = view.push(
            self.create_sidebar_button(
                "+".to_string(),
                "Add instance".to_string(),
                matches!(self.selection, Selection::AddInstance(_)),
            )
            .on_press(Message::SelectionChanged(Selection::AddInstance(
                instance::Instance {
                    name: "New Instance".to_string(),
                    arch: arch::Arch::AMD64,
                    user: "cubic".to_string(),
                    cpus: 4,
                    mem: 4 * 1024 * 1024 * 1024,
                    disk_capacity: 100 * 1024 * 1024 * 1024,
                    ssh_port: 9001,
                    hostfwd: Vec::new(),
                },
            ))),
        );

        view = view.push(
            Self::create_container(
                Self::create_container(
                    iced::widget::Space::new(160, 0.5).into(),
                    iced::Color::WHITE,
                )
                .into(),
                SIDEBAR_BG_COLOR,
            )
            .width(180)
            .padding(iced::Padding::from(0).right(20)),
        );

        for i in 0..self.instances.len() {
            view = view.push(
                self.create_sidebar_button(
                    String::new(),
                    self.instances.get(i).unwrap().name.to_string(),
                    if let Selection::Instance(index, _) = self.selection {
                        index == i
                    } else {
                        false
                    },
                )
                .on_press(Message::SelectionChanged(Selection::Instance(
                    i,
                    self.instances.get(i).unwrap().clone(),
                ))),
            );
        }

        let pages = Self::create_container(iced::widget::scrollable(view).into(), SIDEBAR_BG_COLOR)
            .padding(iced::Padding::from(20).right(0))
            .width(200)
            .height(Length::Fill);

        column![self.create_logo(), pages].into()
    }

    fn create_field<'a>(
        &'a self,
        label: &str,
        value: iced::Element<'a, Message>,
    ) -> iced::Element<'a, Message> {
        Self::create_container(
            row![text!("{label}:").width(100), value]
                .align_y(iced::alignment::Vertical::Center)
                .spacing(20)
                .into(),
            FIELD_BG_COLOR,
        )
        .padding(iced::Padding::from(10))
        .width(Length::Fill)
        .into()
    }

    fn create_content_button(&self, name: String) -> iced::widget::Button<Message> {
        button(text!("{name}")).style(move |_, _| iced::widget::button::Style {
            text_color: iced::Color::WHITE,
            background: Some(iced::Background::Color(CONTENT_BG_COLOR)),
            border: iced::Border {
                color: iced::Color::BLACK,
                width: 1.,
                radius: iced::border::Radius::new(5.0)
                    .top_right(0.)
                    .bottom_right(0.),
            },
            ..Default::default()
        })
    }

    fn create_modify_view(
        &self,
        instance: &instance::Instance,
        new: bool,
    ) -> iced::Element<Message> {
        column![
            self.create_field(
                "Name",
                iced::widget::text_input("tbd", &instance.name)
                    .on_input(Message::InstanceNameChanged)
                    .width(200)
                    .into()
            ),
            self.create_field(
                "Arch",
                iced::widget::text_input("tbd", &self.field)
                    .width(200)
                    .into()
            ),
            self.create_field(
                "CPUs",
                iced::widget::text_input("tbd", &self.field)
                    .width(200)
                    .into()
            ),
            self.create_field(
                "Memory",
                iced::widget::text_input("tbd", &self.field)
                    .width(200)
                    .into()
            ),
            if new {
                row![self
                    .create_content_button("create".to_string())
                    .on_press(Message::Save)]
            } else {
                row![
                    self.create_content_button("delete".to_string())
                        .on_press(Message::Delete),
                    self.create_content_button("save".to_string())
                        .on_press(Message::Save)
                ]
                .spacing(5)
            },
        ]
        .spacing(20)
        .into()
    }

    fn create_content_view(&self) -> iced::Element<Message> {
        let content = match &self.selection {
            Selection::Instance(_, instance) => self.create_modify_view(instance, false),
            Selection::AddInstance(instance) => self.create_modify_view(instance, true),
            _ => "nothing".into(),
        };

        Self::create_container(content, CONTENT_BG_COLOR)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(iced::Padding::from(20))
            .into()
    }

    fn update(state: &mut Self, message: Message) {
        match message {
            Message::SelectionChanged(selection) => {
                state.selection = selection;
            }
            Message::InstanceNameChanged(content) => match &mut state.selection {
                Selection::Instance(_, instance) => instance.name = content,
                Selection::AddInstance(instance) => instance.name = content,
                _ => {}
            },
            Message::Delete => {
                if let Selection::Instance(_, ref instance) = &state.selection {
                    state.instances.retain(|i| i.name != instance.name);
                }
            }
            Message::Save => match &state.selection {
                Selection::AddInstance(instance) => {
                    let index = state.instances.len();
                    state.instances.push(instance.clone());
                    state.selection = Selection::Instance(index, instance.clone());
                }
                Selection::Instance(index, ref instance) => {
                    *state.instances.get_mut(*index).unwrap() = instance.clone();
                }
                _ => {}
            },
        }
    }

    fn view(&self) -> iced::Element<Message> {
        row![
            self.create_sidebar(),
            self.create_content_view(),
            self.instance_view.view()
        ]
        .into()
    }
}
