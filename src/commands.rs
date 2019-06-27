use clap::ArgMatches;
use collections::Echoes;
use restson::RestPath;
use settings;
use xdd_api_client;
use stats::EchoesStats;

impl RestPath<()> for Echoes {
    fn get_path(_: ()) -> Result<String, restson::Error> {
        Ok(String::from("echoes"))
    }
}
impl RestPath<()> for EchoesStats {
    fn get_path(_: ()) -> Result<String, restson::Error> {
        Ok(String::from("stats"))
    }
}

pub fn process_echoes_command(command: &ArgMatches, settings: settings::Settings) {
    let mut client = xdd_api_client::new(settings);

    if command.subcommand_matches("list").is_some() {
        println!("Listing!");
        let answer: Result<Echoes, restson::Error> = client.get(());
        match answer {
            // We need to extract the enum value
            Ok(echoes) => {
                println!("{:?}", echoes);
            }
            Err(e) => println!("Error: {:?}", e),
        }
    } else if command.subcommand_matches("stats").is_some() {
        println!("Stats!");
        let answer: Result<EchoesStats, restson::Error> = client.get(());
        match answer {
            // We need to extract the enum value
            Ok(stats) => {
                println!("{:?}", stats);
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
