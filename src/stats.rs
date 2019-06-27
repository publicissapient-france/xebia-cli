use std::collections::HashMap;
use std::fmt;

#[derive(Deserialize, Clone)]
pub struct EchoesStats {
    pub by_author: HashMap<String, usize>,
    pub editions: usize,
}

impl fmt::Debug for EchoesStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "=== Echoes stats ==n")?;
        writeln!(f, "\nBy author:")?;
        for (author, authored) in self.by_author.clone() {
            writeln!(f, "- {} : {}", author, authored)?;
        }
        writeln!(f, "\nTotal editions: {}", self.editions)?;
        return writeln!(f);
    }
}
