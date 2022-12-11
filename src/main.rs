use iced::widget::{button, column, text, Column, container};
use iced::{Alignment, Length, Element, Sandbox, Settings};
use iced::theme::Theme;

struct Counter {
    value: i32
}

impl Sandbox for Counter {
    type Message = Message;
    
    fn new() -> Self {
        Self { value: 0 }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn title(&self) -> String {
        String::from("Counter - Req")
    }

    fn view(&self) -> Element<Self::Message> {
        let content = column![
            button("+").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("-").on_press(Message::DecrementPressed),
        ]
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::IncrementPressed => self.value += 1,
            Message::DecrementPressed => self.value -= 1,
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

fn main() -> iced::Result {
    Counter::run(Settings::default())

}
