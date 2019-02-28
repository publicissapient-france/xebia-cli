use echo;

#[derive(Deserialize,Debug)]
#[serde(untagged)]
pub enum Echoes {
    Object(Vec<echo::Echo>)
}
