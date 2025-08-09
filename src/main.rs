mod actions;
mod arch;
mod commands;
mod emulator;
mod env;
mod error;
mod fs;
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

use iced::widget::{
    button, column, row,
    scrollable::{Direction, Scrollbar},
    text, Column, Scrollable,
};
use iced::Length;

type Message = ();

const COLOR1: iced::Color = iced::Color::from_rgb(0.11, 0.11, 0.11);
const COLOR2: iced::Color = iced::Color::from_rgb(0.145, 0.145, 0.145);
const COLOR3: iced::Color = iced::Color::from_rgb(0.122, 0.122, 0.122);
const COLOR4: iced::Color = iced::Color::from_rgb(0.137, 0.137, 0.137);

fn create_container_bg_color(bg: iced::Color) -> iced::widget::container::Style {
    iced::widget::container::Style::default()
        .background(iced::Background::Color(bg))
        .border(iced::Border {
            color: COLOR1,
            width: 0.,
            radius: iced::border::Radius::from(5.0),
        })
}

fn create_button_bg_color() -> iced::widget::button::Style {
    iced::widget::button::Style {
        text_color: iced::Color::WHITE,
        background: Some(iced::Background::Gradient(iced::Gradient::Linear(
            iced::gradient::Linear::new(0.)
                .add_stop(0., COLOR2)
                .add_stop(1., COLOR4),
        ))),
        border: iced::Border {
            color: COLOR1,
            width: 1.,
            radius: iced::border::Radius::from(5.0),
        },
        ..Default::default()
    }
}

#[derive(Default)]
struct MyApp;

impl MyApp {
    fn create_menubar(&self) -> iced::Element<Message> {
        iced::widget::Container::new(row![button("New")
            .width(60)
            .height(30)
            .style(|_, _| create_button_bg_color()),])
        .padding(iced::Padding::from(5))
        .width(Length::Fill)
        .style(|_| {
            iced::widget::container::Style::default()
                .background(iced::Background::Gradient(iced::Gradient::Linear(
                    iced::gradient::Linear::new(0.)
                        .add_stop(0., COLOR2)
                        .add_stop(1., COLOR4),
                )))
                .border(iced::Border {
                    color: iced::Color::BLACK,
                    width: 1.,
                    radius: iced::border::Radius::from(0.),
                })
        })
        .into()
    }

    fn create_instance(&self, index: u32) -> iced::Element<Message> {
        iced::widget::Container::new(
            row![
                text(format!("Instance {index}")).width(150),
                button("Start").style(|_, _| create_button_bg_color()),
                button("Stop").style(|_, _| create_button_bg_color()),
                button("Restart").style(|_, _| create_button_bg_color()),
                button("Edit").style(|_, _| create_button_bg_color()),
                button("Delete").style(|_, _| create_button_bg_color()),
            ]
            .width(Length::Fill)
            .spacing(5)
            .align_y(iced::alignment::Vertical::Center),
        )
        .width(Length::Fill)
        .padding(iced::Padding::from(10))
        .style(|_| create_container_bg_color(COLOR3))
        .into()
    }

    fn create_instances(&self) -> iced::Element<Message> {
        let mut view = Column::new().spacing(5).padding(iced::Padding::from(20));

        for i in 0..20 {
            view = view.push(self.create_instance(i));
        }

        iced::widget::Container::new(
            Scrollable::new(view)
                .width(Length::Fill)
                .height(Length::Fill)
                .direction(Direction::Vertical(Scrollbar::new())),
        )
        .style(|_| create_container_bg_color(COLOR2))
        .into()
    }

    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<Message> {
        column![self.create_menubar(), self.create_instances(),].into()
    }
}
