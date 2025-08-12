use crate::gui::Message;

pub struct PlaneView; 

impl PlaneView {
    pub fn create(
        widget: iced::Element<Message>,
        background: iced::Color,
    ) -> iced::widget::Container<Message> {
        iced::widget::Container::new(widget).style(move |_| {
            iced::widget::container::Style::default()
                .background(iced::Background::Color(background))
        })
    }
}
