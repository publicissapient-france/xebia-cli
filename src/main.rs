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

fn main() {
    //let mut client = RestClient::new("https://beta.todoist.com").unwrap();
    let mut client = RestClient::new("https://api-dev.xebia.fr").unwrap();

    // Authenticate to XDD with token from environment
    let api_key = env::var("XDD_API_KEY").unwrap();
    client.set_header("Authorization", &format!("Bearer {}", api_key)).unwrap();

    let answer: Result<collections::Echoes, restson::Error> = client.get(());
    match answer {
        Ok(echoes)  => {
            // We need to extract the enum value
            match echoes {
                collections::Echoes::Object(echoes) => {
                    println!("{:?}", echoes);

                    println!{"== Tasks =="};
//                    println!("Total: {}", tasks_metrics.total);
//                    println!("Completed: {}", tasks_metrics.completed);
//                    println!("To-do: {}", tasks_metrics.todo);
//                    println!("Total P1: {}", tasks_metrics.total_p1);
                }
            }
        },
        Err(e) => println!("Error: {:?}", e),
    }
}

