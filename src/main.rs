use iced::widget::{button, column, container, horizontal_space, row, text};
use iced::{Center, Element};
pub fn main() -> iced::Result {
    iced::run(
        "Game Collection",
        GameCollection::update,
        GameCollection::view,
    )
}

#[derive(Default)]
struct GameCollection {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl GameCollection {
    fn update(&mut self, message: Message) {}

    fn view(&self) -> Element<Message> {
        let header = container(
            row![horizontal_space(), "Game Collection", horizontal_space(),]
                .padding(10)
                .align_y(Center),
        );
        let sidebar = container(
            column![
                button("Home"),
                button("Games"),
                button("Settings"),
                button("About"),
            ]
            .padding(10),
        );
        let content = container(
            column![
                text("Welcome to Game Collection!").size(50),
                text("This is a collection of games that you can play.").size(20),
            ]
            .padding(20),
        );
        column![header, row![sidebar, content]].into()
    }
}
