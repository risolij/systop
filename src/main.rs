use iced::widget::{button, column, text, Column, container};
use iced::{Alignment, Length, Element, Sandbox, Settings};
use iced::theme::Theme;

struct Themer {
    theme: Theme
}

impl Sandbox for Themer {
    type Message = Message;
    
    fn new() -> Self {
        Self { theme: Theme::Dark }
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn title(&self) -> String {
        String::from("Themer - Req")
    }

    fn view(&self) -> Element<Self::Message> {
        let theming = column![
            button("TESTING").on_press(Message::ThemePressed),
            text("Change Theme").size(50)
        ]
        .align_items(Alignment::Center);


        container(theming)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()


    }

    fn update(&mut self, message: Self::Message) {
        match self.theme {
            Theme::Light => self.theme = Theme::Dark,
            Theme::Dark => self.theme = Theme::Light,
            _ => self.theme = Theme::Light,
        };
    }
}


#[derive(Debug, Clone, Copy)]
pub enum Message {
    ThemePressed,
}

fn main() -> iced::Result {
    Themer::run(Settings::default())

}
