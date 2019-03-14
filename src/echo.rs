use std::fmt;

#[derive(Deserialize)]
pub struct Echo {
    pub edition: u32,
    pub author: String,
    pub title: String,
    pub main_topic: String,
}

impl fmt::Debug for Echo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}. {} ({})", self.edition, self.title, self.author)
    }
}
