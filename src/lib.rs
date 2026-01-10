#![allow(unused)]

pub mod items;
pub mod notes;
pub mod requirements;
pub mod rooms;

use crate::{items::*, requirements::*, rooms::*};
use std::fmt;
use std::io::prelude::*;
use std::{fs::File, path::Path};

#[derive(Debug)]
pub enum Error {
    Parse(String, serde_json::Error),
    Open(String, std::io::Error),
    Read(String, std::io::Error),
}

pub fn load_items() -> Result<Items, Error> {
    let path = Path::new("data/items.json");

    let text = load_json_text(path)?;

    let items: Items = serde_json::from_str(&text)
        .map_err(|why| Error::Parse(path.to_string_lossy().to_string(), why))?;

    Ok(items)
}

pub fn load_room() -> Result<Room, Error> {
    let path = Path::new("data/region/brinstar/blue/Morph Ball Room.json");

    let text = load_json_text(path)?;

    let room: Room = serde_json::from_str(&text)
        .map_err(|why| Error::Parse(path.to_string_lossy().to_string(), why))?;

    Ok(room)
}

fn load_json_text(path: &Path) -> Result<String, Error> {
    let mut text = String::new();

    let mut file = File::open(path).map_err(|why| Error::Open(path.to_string_lossy().to_string(), why))?;

    file.read_to_string(&mut text)
        .map_err(|why| Error::Read(path.to_string_lossy().to_string(), why))?;

    Ok(text)
}

impl std::error::Error for Error {
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parse(path, error) => write!(
                f,
                "couldn't parse json at '{}': {}",
                path,
                error
            ),
            Error::Open(path, error) => write!(
                f,
                "couldn't open json at '{}': {}",
                path,
                error
            ),
            Error::Read(path, error) => write!(
                f,
                "couldn't read json at '{}': {}",
                path,
                error
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_deserializes_items() {
        let config = load_items();
        assert!(config.is_ok());
    }

    #[test]
    fn it_deserializes_room() {
        let rooms = load_room();
        assert!(rooms.is_ok());
        let r: Room = match rooms {
            Ok(room) => room,
            Err(error) => panic!("Couldn't unwrap room: {:?}", error),
        };
        println!("{:#?}", r.nodes[0].locks);
    }
}
