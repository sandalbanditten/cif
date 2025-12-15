use iced::widget::{button, center, column};
use iced::{Element, Task};

use tracing::{Level, event};

#[derive(Debug, Default, Clone)]
pub struct App {
    character: Option<Character>,
    page: Page,
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        event!(Level::DEBUG, "{message:?}");
        match message {
            Message::NewCharacter => {
                self.character = Some(Character::new());
                self.page = Page::Character;
                Task::none()
            }
            Message::LoadCharacter => App::open_character_file(),
            Message::Home => {
                self.page = Page::Home;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self.page {
            Page::Home => center(
                column![
                    button("New character").on_press(Message::NewCharacter),
                    button("Load character").on_press(Message::LoadCharacter),
                ]
                .spacing(10),
            )
            .into(),
            Page::Character => center(column![
                button("Home").on_press(Message::Home),
                "character stuff here:\n"
            ])
            .into(),
        }
    }

    fn open_character_file() -> Task<Message> {
        // TODO: Some serde shit
        todo!();
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Page {
    #[default]
    Home,
    Character,
}

#[derive(Debug, Clone)]
pub enum Error {
    InvalidCharacter,
}

#[derive(Debug, Clone)]
pub enum Message {
    NewCharacter,
    LoadCharacter,
    Home,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Character {
    name: String,
    player_name: String,
    path: Path,
    level: u8,
    ancestry: Ancestry,
    stats: Stats,
}

impl Character {
    pub fn new() -> Self {
        Self {
            name: String::from("test"),
            player_name: String::from("test"),
            path: Path {},
            level: 0,
            ancestry: Ancestry {},
            stats: Stats::default(),
        }
    }

    pub fn show(&self) -> String {
        // TODO
        String::from("test character")
    }
}

#[derive(Debug, Clone)]
pub struct Path {}

#[derive(Debug, Clone)]
pub struct Ancestry {}

#[derive(Clone, Copy, Debug, Default)]
pub struct Stats {
    strength: u8,
    speed: u8,
    intellect: u8,
    willpower: u8,
    awareness: u8,
    presence: u8,
}

impl Stats {
    pub fn physical_defense(&self) -> u8 {
        10 + self.strength + self.speed
    }

    pub fn cognitive_defense(&self) -> u8 {
        10 + self.intellect + self.willpower
    }

    pub fn spiritual_defense(&self) -> u8 {
        10 + self.awareness + self.presence
    }
}
