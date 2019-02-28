#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate restson;

//use std::collections::HashMap;
use std::env;
use restson::{RestPath, RestClient};

// TODO: this could probably be improved and moved to a lib.rs file
pub mod echo;
pub mod collections;

impl RestPath<()> for collections::Echoes {
    fn get_path(_: ()) -> Result<String, restson::Error> { Ok(String::from("echoes")) }
}

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if let Some(_matches) = matches.subcommand_matches("echoes") {
        let mut client = RestClient::new("https://api-dev.xebia.fr").unwrap();

        // Authenticate to XDD with token from environment
        let env_token = env::var("XDD_API_KEY");
        match env_token {
            Ok(api_key) => {
                client.set_header("Authorization", &format!("Bearer {}", api_key)).unwrap();

                let answer: Result<collections::Echoes, restson::Error> = client.get(());
                match answer {
                    Ok(echoes)  => {
                        // We need to extract the enum value
                        println!("{:?}", echoes);
                    },
                    Err(e) => println!("Error: {:?}", e),
                }
            }
            Err(_) => {
                println!("Could not find XDD_API_KEY in environment");
                std::process::exit(1);
            },
        };
    }
}

