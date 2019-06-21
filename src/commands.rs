use clap::ArgMatches;
use collections;
use settings;
use restson::{RestClient, RestPath};

impl RestPath<()> for collections::Echoes {
    fn get_path(_: ()) -> Result<String, restson::Error> {
        Ok(String::from("echoes"))
    }
}


pub fn process_echoes_command(command: &ArgMatches, settings: settings::Settings) {
    if command.subcommand_matches("list").is_some() {
        println!("Listing!");
    }
    // TODO extract API call from command logic?
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