use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    id: i32,
    name: String,
    card_stats: Stats
}


#[derive(Serialize, Deserialize, Debug)]
struct Stats{
    hp: i32,
    power: i32,
    speed: i32
}