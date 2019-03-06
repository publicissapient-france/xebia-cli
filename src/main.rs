#[macro_use]
extern crate serde_derive;

//  TODO Rust offline documentation
extern crate config;
extern crate serde;
extern crate serde_json;
extern crate restson;

//use std::collections::HashMap;
use restson::{RestPath, RestClient};

// TODO: this could probably be improved and moved to a lib.rs file
pub mod echo;
pub mod collections;
pub mod settings;

impl RestPath<()> for collections::Echoes {
    fn get_path(_: ()) -> Result<String, restson::Error> { Ok(String::from("echoes")) }
}

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    // CLI arguments and commands handling
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    // Settings handling
    let settings = settings::Settings::new();
    match settings {
        Ok(settings) => {
            println!("Settings: {:?}", settings);
            // Dispatch work
            if let Some(_matches) = matches.subcommand_matches("echoes") {
                if let Some(_) = _matches.subcommand_matches("list") {
                    println!("Listing!");
                }
                let mut client = RestClient::new("https://api-dev.xebia.fr").unwrap();

                // Authenticate to XDD with token from environment
                match settings.xdd.api_token {
                    Some(api_key) => {
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
                    None => {
                        println!("Could not find XDD API token. It can be passed through environment under the name {:?}",
                                 settings::XDD_API_TOKEN_ENV_VAR);
                        std::process::exit(1);
                    },
                };
            }
        }
        Err(config_error) => {
            println!("Error in settings: {:?}", config_error);
        }
    }
}

