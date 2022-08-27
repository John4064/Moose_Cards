// use std::fmt;
use std::fs;
use crate::UnitCard;

pub struct Card;
pub fn print_f(contents: String){
    println!("{}",contents);
    return;
}

// pub fn string_to_card(contents: String) -> Card{
//     let temp_card=serde_json::from_str(&contents).unwrap();
//
//
//
//     return temp_card
// }


pub fn read_f(file_name: String) -> UnitCard {
    /**
    @brief: read the file to string format, then parses to a custom struct
    @return: The File Contents as a Card Struct
    **/
    let contents: String = fs::read_to_string("./card_stats/moose.json")
        .expect("Something went wrong reading the file");
    return serde_json::from_str(&*contents).unwrap();;
}