pub trait Identifiable {
    fn id(&self) -> Option<u32>;
    fn set_id(&mut self, id: u32);
}

#[derive(Clone, Default)]
pub struct Game {
    pub id: Option<u32>,
    pub title: String,
}

impl Identifiable for Game {
    fn id(&self) -> Option<u32> {
        self.id
    }
    fn set_id(&mut self, id: u32) {
        self.id = Some(id);
    }
}

#[derive(Clone, Default)]
pub struct System {
    pub id: Option<u32>,
    pub name: String,
}

impl Identifiable for System {
    fn id(&self) -> Option<u32> {
        self.id
    }
    fn set_id(&mut self, id: u32) {
        self.id = Some(id);
    }
}
