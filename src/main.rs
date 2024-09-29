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
enum MainContent {
    #[default]
    Home,
    AddGame,
    Games,
    Settings,
    About,
}

#[derive(Default)]
struct GameCollection {
    main_content: MainContent,
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
                self.main_content = MainContent::AddGame;
                println!("Add game");
            }
            Message::Games => {
                self.main_content = MainContent::Games;
                println!("Games");
            }
            Message::Home => {
                self.main_content = MainContent::Home;
                println!("Home");
            }
            Message::Settings => {
                self.main_content = MainContent::Settings;
                println!("Settings");
            }
            Message::About => {
                self.main_content = MainContent::About;
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

        let content = match self.main_content {
            MainContent::Home => home_content(),
            MainContent::AddGame => add_game_content(),
            MainContent::Games => games_content(),
            MainContent::Settings => settings_content(),
            MainContent::About => about_content(),
        };

        column![header, row![sidebar, content]].into()
    }
}

fn home_content<'a>() -> Element<'a, Message> {
    column![
        text("Welcome to Game Collection!").size(50),
        text("This is a collection of games that you can play.").size(20),
    ]
    .padding(20)
    .into()
}

fn add_game_content<'a>() -> Element<'a, Message> {
    column![text("Add game").size(50),].padding(20).into()
}
fn games_content<'a>() -> Element<'a, Message> {
    column![text("games").size(50),].padding(20).into()
}
fn settings_content<'a>() -> Element<'a, Message> {
    column![text("Settings").size(50),].padding(20).into()
}
fn about_content<'a>() -> Element<'a, Message> {
    column![text("About").size(50),].padding(20).into()
}
