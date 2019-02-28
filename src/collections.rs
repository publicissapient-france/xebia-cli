use std::fmt;
use echo;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Echoes {
    Object(Vec<echo::Echo>)
}

impl fmt::Debug for Echoes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Echoes::Object(echoes) => {
                for echo in echoes {
                    write!(f, "{:?}\n", echo);
                }
                return write!(f, "\n");
            },
        }
    }
}
