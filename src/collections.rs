use echo;
use meti;
use std::fmt;
use std::process;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Echoes {
    Object(Vec<echo::Echo>),
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Metis {
    Object(Vec<meti::Meti>),
}

impl fmt::Debug for Echoes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Echoes::Object(echoes) => {
                for echo in echoes {
                    // TODO fix unused write! std::result::Result
                    // Construct result string then write it?
                    match writeln!(f, "{:?}", echo) {
                        Ok(_) => {}
                        Err(e) => {
                            log::error!("Failed to format an Echo: {:?}", e);
                            process::exit(1);
                        }
                    }
                }
                return writeln!(f);
            }
        }
    }
}

impl fmt::Debug for Metis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Metis::Object(metis) => {
                for meti in metis {
                    // TODO fix unused write! std::result::Result
                    // Construct result string then write it?
                    match writeln!(f, "{:?}", meti) {
                        Ok(_) => {}
                        Err(e) => {
                            log::error!("Failed to format a METI: {:?}", e);
                            process::exit(1);
                        }
                    }
                }
                return writeln!(f);
            }
        }
    }
}
