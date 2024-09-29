use game_collection_iced::model::game::Game;
use iced::widget::{button, column, container, horizontal_space, row, text, text_input, Column};
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
    games: Vec<Game>,
    selected_game_id: Option<u32>,
}

#[derive(Debug, Clone)]
enum Message {
    AddGame,
    Games,
    Home,
    Settings,
    About,
    GameTitleChanged(String),
    EditGame(u32),
}

impl GameCollection {
    fn update(&mut self, message: Message) {
        match message {
            Message::AddGame => {
                let new_game = create_new_game(&self.games);
                self.selected_game_id = Some(new_game.id);
                self.games.push(new_game);
                self.main_content = MainContent::AddGame;
            }
            Message::Games => {
                self.main_content = MainContent::Games;
            }
            Message::Home => {
                self.main_content = MainContent::Home;
            }
            Message::Settings => {
                self.main_content = MainContent::Settings;
            }
            Message::About => {
                self.main_content = MainContent::About;
            }
            Message::GameTitleChanged(title) => {
                if let Some(selected_game_id) = self.selected_game_id {
                    if let Some(game) = self.games.iter_mut().find(|g| g.id == selected_game_id) {
                        game.title = title;
                    }
                }
            }
            Message::EditGame(game_id) => {
                self.selected_game_id = Some(game_id);
                self.main_content = MainContent::AddGame;
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
            MainContent::AddGame => add_game_content(
                self.selected_game_id
                    .and_then(|selected_game_id| {
                        self.games.iter().find(|game| game.id == selected_game_id)
                    })
                    .unwrap(),
            ),
            MainContent::Games => games_content(&self.games),
            MainContent::Settings => settings_content(),
            MainContent::About => about_content(),
        };

        column![header, row![sidebar, content]].into()
    }
}

fn create_new_game(games: &Vec<Game>) -> Game {
    let new_game_id = games.len() as u32;
    Game {
        id: new_game_id,
        title: "".to_string(),
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

fn add_game_content<'a>(game: &Game) -> Element<'a, Message> {
    column![
        text("Add game").size(50),
        text_input("Title", game.title.as_str()).on_input(|str| Message::GameTitleChanged(str)),
    ]
    .padding(20)
    .into()
}
fn games_content<'a>(games: &'a Vec<Game>) -> Element<'a, Message> {
    let games_list = games.iter().map(|game| {
        row![
            text(game.title.as_str()),
            button("Edit").on_press(Message::EditGame(game.id)),
        ]
        .into()
    });
    let game_list_with_container =
        Column::with_children(games_list.collect::<Vec<Element<'a, Message>>>());

    column![text("games").size(50), game_list_with_container]
        .padding(20)
        .into()
}
fn settings_content<'a>() -> Element<'a, Message> {
    column![text("Settings").size(50),].padding(20).into()
}
fn about_content<'a>() -> Element<'a, Message> {
    column![text("About").size(50),].padding(20).into()
}
