use std::env;
use config::{ConfigError, Config, File}; // , Environment};

#[derive(Debug, Deserialize)]
pub struct XDD {
    pub api_token: Option<String>,
    pub endpoint: String,
}
// TODO use every XDD_ from env?
pub const XDD_API_TOKEN_ENV_VAR: &str = "XDD_API_KEY";

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub xdd: XDD,
}

impl Settings {
    pub fn new(debug: bool) -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/default"))?;
        if debug {
            s.merge(File::with_name("config/debug"))?;
        }

        let api_key = env::var(XDD_API_TOKEN_ENV_VAR);
        match api_key {
            Ok(result) => {
                // Got API key from environment
                println!("[Settings] Found XDD API token in env");
                s.set("xdd.api_token", result)?;
            }
            Err(e) => {
                // Failed to get API key from environment
                // TODO : exit
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