use crate::gui::{Message, PlaneView};

const FIELD_BG_COLOR: iced::Color = iced::Color::from_rgb(0.204, 0.231, 0.282);

#[derive(Default)]
pub struct InstanceView {
    name: String,
}

impl InstanceView {
    fn create_field<'a>(
        &'a self,
        label: &str,
        value: iced::Element<'a, Message>,
    ) -> iced::Element<'a, Message> {
        PlaneView::create(
            iced::widget::row![iced::widget::text!("{label}:").width(100), value]
                .align_y(iced::alignment::Vertical::Center)
                .spacing(20)
                .into(),
            FIELD_BG_COLOR,
        )
        .padding(iced::Padding::from(10))
        .width(iced::Length::Fill)
        .into()
    }

    pub fn view(&self) -> iced::Element<Message> {
        iced::widget::column![self.create_field(
            "Name",
            iced::widget::text_input("tbd", &self.name)
                .on_input(Message::InstanceNameChanged)
                .width(200)
                .into()
        ),]
        .spacing(20)
        .into()
    }
}
