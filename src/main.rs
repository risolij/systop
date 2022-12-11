use iced::widget::{button, column, text, Column};
use iced::{Alignment, Element, Sandbox, Settings};

struct Counter {
    value: i32
}

impl Sandbox for Counter {
    type Message = Message;
    
    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Counter - Req")
    }

    fn view(&self) -> Element<Self::Message> {
        column![
            button("+").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("-").on_press(Message::DecrementPressed),
        ]
        .padding(20)
        .align_items(Alignment::Center)
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

fn main() {
    Counter::run(Settings::default());

}
