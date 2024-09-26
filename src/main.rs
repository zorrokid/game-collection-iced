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

#[derive(Debug, Clone)]
enum Message {
    AddGame,
    Games,
    Home,
    Settings,
    About,
}

impl GameCollection {
    fn update(&mut self, message: Message) {
        match message {
            Message::AddGame => {
                println!("Add game");
            }
            Message::Games => {
                println!("Games");
            }
            Message::Home => {
                println!("Home");
            }
            Message::Settings => {
                println!("Settings");
            }
            Message::About => {
                println!("About");
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let header = container(
            row![horizontal_space(), "Game Collection", horizontal_space(),]
                .padding(10)
                .align_y(Center),
        );
        let sidebar = container(
            column![
                button("Home").on_press(Message::Home),
                button("Add game").on_press(Message::AddGame),
                button("Games").on_press(Message::Games),
                button("Settings").on_press(Message::Settings),
                button("About").on_press(Message::About),
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
