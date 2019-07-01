#[macro_use]
extern crate serde_derive;
extern crate log;

extern crate simplelog;
use simplelog::*;

//  TODO Rust offline documentation
extern crate config;
extern crate restson;
extern crate serde;
extern crate serde_json;

// For Login
extern crate base64;
extern crate oauth2;
extern crate rand;
extern crate url;

//use std::collections::HashMap;

// TODO: this could probably be improved and moved to a lib.rs file
pub mod auth;
pub mod collections;
pub mod commands;
pub mod echo;
pub mod meti;
pub mod settings;
pub mod stats;
pub mod xdd_api_client;

#[macro_use]
extern crate clap;

use clap::App;

fn main() {
    // Initialize loggers
    let term_logger = match TermLogger::new(LevelFilter::Debug, Config::default()) {
        Some(tl) => tl,
        None => {
            println!("Failed to create TermLogger; exiting.");
            std::process::exit(1);
        }
    };
    CombinedLogger::init(vec![term_logger]).expect("Failed to initialize logger");
    log::debug!("Creating CLI and parsing arguments...");

    // Handle CLI arguments and commands
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Handle Settings
    log::debug!("Defining settings...");
    let settings = settings::Settings::new(matches.clone());

    match settings {
        Ok(settings) => {
            log::debug!("Settings defined successfully: {:?}", settings);

            // Dispatch work
            if let Some(_matches) = matches.subcommand_matches("login") {
                commands::process_login_command(_matches, settings);
            } else if let Some(_matches) = matches.subcommand_matches("echoes") {
                commands::process_echoes_command(_matches, settings);
            } else if let Some(_matches) = matches.subcommand_matches("meti") {
                commands::process_meti_command(_matches, settings);
            } // None case doesn't have to be handled, it's simply not the "echoes" command
        }
        Err(config_error) => {
            println!("Error in settings: {:?}", config_error);
        }
    }
}
