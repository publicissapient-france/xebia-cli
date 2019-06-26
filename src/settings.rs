use config::{Config, ConfigError, File}; // , Environment};
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct XDD {
    pub api_token: Option<String>,
    pub endpoint: String,
}
// TODO use every XDD_ from env?
pub const XDD_API_TOKEN_ENV_VAR: &str = "XDD_API_KEY";

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub debug: bool,
    pub xdd: XDD,
}

impl Settings {
    pub fn new(cli_flags: clap::ArgMatches) -> Result<Self, ConfigError> {
        log::trace!("Instantiating new Settings");
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        log::trace!("Loading default configuration");
        s.merge(File::with_name("config/default"))?;

        // Setup Debug mode
        if cli_flags.is_present("debug") {
            log::info!("Debug mode detected; loading debug config");
            s.merge(File::with_name("config/debug"))?;
        }

        // Load XDD API Key
        let api_key = env::var(XDD_API_TOKEN_ENV_VAR);
        match api_key {
            Ok(result) => {
                // Got API key from environment
                log::info!("[Settings] Found XDD API token in env");
                s.set("xdd.api_token", result)?;
            }
            Err(_e) => {
                // Failed to get API key from environment
                // TODO : exit
                log::error!("Could not find XDD API token in env")
            }
        }
        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        //let env = env::var("RUN_MODE").unwrap_or("development".into());
        //s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        //s.merge(File::with_name("config/local").required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        //s.merge(Environment::with_prefix("xdd"))?;

        // Now that we're done, let's access our configuration
        //println!("debug: {:?}", s.get_bool("debug"));
        //println!("database: {:?}", s.get::<String>("database.url"));

        // You can deserialize (and thus freeze) the entire configuration as
        // TODO ??
        //s.try_into()
        s.try_into()
    }
}
