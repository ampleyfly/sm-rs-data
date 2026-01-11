use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(untagged)]
pub enum Note {
    #[default]
    None,
    SingleLine(String),
    MultiLine(Vec<String>),
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(untagged)]
pub enum DevNote {
    #[default]
    None,
    SingleLine(String),
    MultiLine(Vec<String>),
}
