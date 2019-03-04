use std::fmt;
use std::process;
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
                    // TODO fix unused write! std::result::Result
                    // Construct result string then write it?
                    match write!(f, "{:?}\n", echo) {
                        Ok(_) => {},
                        Err(e) => {
                            println!("Failed to write an Echo: {:?}", e);
                            process::exit(1);
                        }
                    }
                }
                return write!(f, "\n");
            },
        }
    }
}
