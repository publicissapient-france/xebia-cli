use std::fmt;

#[derive(Deserialize)]
pub struct Echo {
    pub edition: u32,
    pub title: String,
    pub author: String,
}

impl fmt::Debug for Echo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}. {} ({})", self.edition, self.title, self.author)
    }
}
