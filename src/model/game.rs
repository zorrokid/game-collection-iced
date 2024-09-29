#[derive(Clone, Default)]
pub struct Game {
    pub id: Option<u32>,
    pub title: String,
}

#[derive(Clone, Default)]
pub struct System {
    pub id: Option<u32>,
    pub name: String,
}
