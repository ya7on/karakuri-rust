pub struct Entity {
    pub id: usize,
}

impl Entity {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Clone for Entity {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Entity {}
