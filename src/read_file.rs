// use std::fmt;
use std::fs;
use crate::{ItemCard, UnitCard};

pub struct Card;

pub fn read_f(file_name: String) -> ItemCard {
    /**
    @brief: read the file to string format, then parses to a custom struct
    @return: The File Contents as a Card Struct
    **/
    let contents: String = fs::read_to_string("./card_stats/items/dagger.json")
        .expect("Something went wrong reading the file");
    return serde_json::from_str(&*contents).unwrap();;
}
