#[macro_use]
extern crate serde_derive;
extern crate log;

//  TODO Rust offline documentation
extern crate config;
extern crate restson;
extern crate serde;
extern crate serde_json;

//use std::collections::HashMap;
use restson::{RestClient, RestPath};


// TODO: this could probably be improved and moved to a lib.rs file
pub mod collections;
pub mod echo;
pub mod settings;

impl RestPath<()> for collections::Echoes {
    fn get_path(_: ()) -> Result<String, restson::Error> {
        Ok(String::from("echoes"))
    }
}

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    log::debug!("Creating CLI and parsing arguments...");
    // CLI arguments and commands handling
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    // Settings handling
    log::debug!("Defining settings...");
    let settings = settings::Settings::new(matches.is_present("debug"));

    match settings {
        Ok(settings) => {
            log::debug!("Settings: {:?}", settings);
            // Dispatch work
            if let Some(_matches) = matches.subcommand_matches("echoes") {
                // TODO or None
                if _matches.subcommand_matches("list").is_some() {
                    println!("Listing!");
                }
                let mut client = RestClient::new(settings.xdd.endpoint.as_str()).unwrap();

                // Authenticate to XDD with token from environment
                match settings.xdd.api_token.clone() {
                    Some(api_key) => {
                        client
                            .set_header("Authorization", &format!("Bearer {}", api_key))
                            .unwrap();

                        let answer: Result<collections::Echoes, restson::Error> = client.get(());
                        match answer {
                            Ok(echoes) => {
                                // We need to extract the enum value
                                println!("{:?}", echoes);
                            }
                            Err(e) => println!("Error: {:?}", e),
                        }
                    }
                    None => {
                        log::error!("Could not find XDD API token. It can be passed through environment under the name {:?}",
                                 settings::XDD_API_TOKEN_ENV_VAR);
                        std::process::exit(1);
                    }
                };
            }
        }
        Err(config_error) => {
            println!("Error in settings: {:?}", config_error);
        }
    }
}
