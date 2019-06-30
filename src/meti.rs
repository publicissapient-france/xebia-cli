use std::fmt;

#[derive(Deserialize)]
pub struct Meti {
    pub edition: u32,
    pub speakers: Vec<String>,
    pub title: String,
    pub date: String,
    pub client: String,
}

impl fmt::Debug for Meti {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}. {} ({})", self.edition, self.title, self.client)
    }
}
