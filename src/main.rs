use game_collection_iced::model::game::{self, Game, Identifiable, System};
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
    AddSystem,
}

#[derive(Default)]
struct GameCollection {
    main_content: MainContent,
    games: Vec<Game>,
    selected_game: Option<Game>,
    systems: Vec<System>,
    selected_system: Option<System>,
}

#[derive(Debug, Clone)]
enum Message {
    Home,
    Games,
    About,
    Settings,
    // Game
    AddGame,
    CancelAddGame,
    EditGame(u32),
    GameTitleChanged(String),
    SaveGame,
    // System
    AddSystem,
    CancelAddSystem,
    EditSystem(u32),
    SystemNameChanged(String),
    SaveSystem,
}

impl GameCollection {
    fn update(&mut self, message: Message) {
        match message {
            Message::AddGame => {
                let new_game = Game::default();
                self.selected_game = Some(new_game);
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
                if let Some(selected_game_id) = &mut self.selected_game {
                    selected_game_id.title = title;
                }
            }
            Message::EditGame(game_id) => {
                self.selected_game = Some(self.games[game_id as usize].clone());
                self.main_content = MainContent::AddGame;
            }
            Message::EditSystem(system_id) => {
                self.selected_system = Some(self.systems[system_id as usize].clone());
                self.main_content = MainContent::AddSystem;
            }
            Message::AddSystem => {
                let new_system = System::default();
                self.selected_system = Some(new_system);
                self.main_content = MainContent::AddSystem;
            }
            Message::SystemNameChanged(name) => {
                if let Some(selected_system_id) = &mut self.selected_system {
                    selected_system_id.name = name;
                }
            }
            Message::CancelAddGame => {
                self.selected_game = None;
                self.main_content = MainContent::Home;
            }
            Message::CancelAddSystem => {
                self.selected_system = None;
                self.main_content = MainContent::Settings;
            }
            Message::SaveGame => {
                save(&mut self.games, &self.selected_game.as_ref().unwrap());
                self.selected_game = None;
                self.main_content = MainContent::Home;
            }
            Message::SaveSystem => {
                save(&mut self.systems, &self.selected_system.as_ref().unwrap());
                self.selected_system = None;
                self.main_content = MainContent::Settings;
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
                &self.selected_game.as_ref().unwrap(), /* TODO handler error */
            ),
            MainContent::Games => games_content(&self.games),
            MainContent::Settings => settings_content(&self.systems),
            MainContent::About => about_content(),
            MainContent::AddSystem => add_system_content(&self.selected_system.as_ref().unwrap()),
        };

        column![header, row![sidebar, content]].into()
    }
}

fn save<T: Identifiable + Clone>(items: &mut Vec<T>, item: &T) {
    if let Some(id) = item.id() {
        items[id as usize] = item.clone();
    } else {
        let mut new_item = item.clone();
        new_item.set_id(items.len() as u32);
        items.push(new_item);
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
        button("Cancel").on_press(Message::CancelAddGame),
        button("Save").on_press(Message::SaveGame),
    ]
    .padding(20)
    .into()
}

fn add_system_content<'a>(system: &System) -> Element<'a, Message> {
    column![
        text("Add system").size(50),
        text_input("Name", system.name.as_str()).on_input(|str| Message::SystemNameChanged(str)),
        button("Cancel").on_press(Message::CancelAddSystem),
        button("Save").on_press(Message::SaveSystem),
    ]
    .padding(20)
    .into()
}

fn games_content<'a>(games: &'a Vec<Game>) -> Element<'a, Message> {
    let games_list = games.iter().map(|game| {
        row![
            text(game.title.as_str()),
            button("Edit").on_press_maybe(game.id.map(Message::EditGame)),
        ]
        .into()
    });
    let game_list_with_container =
        Column::with_children(games_list.collect::<Vec<Element<'a, Message>>>());

    column![text("games").size(50), game_list_with_container]
        .padding(20)
        .into()
}
fn settings_content<'a>(systems: &'a Vec<System>) -> Element<'a, Message> {
    let systems_list = systems.iter().map(|system| {
        row![
            text(system.name.as_str()),
            button("Edit").on_press_maybe(system.id.map(Message::EditSystem)),
        ]
        .into()
    });
    let system_list_with_container =
        Column::with_children(systems_list.collect::<Vec<Element<'a, Message>>>());
    let add_new_system_button = button("Add new system").on_press(Message::AddSystem);
    column![
        text("Settings").size(50),
        system_list_with_container,
        add_new_system_button
    ]
    .padding(20)
    .into()
}
fn about_content<'a>() -> Element<'a, Message> {
    column![text("About").size(50),].padding(20).into()
}
