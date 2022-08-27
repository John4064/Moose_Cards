use serde::{Serialize, Deserialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct UnitCard {
    id: i32,
    name: String,
    card_stats: Stats
}

#[derive(Deserialize, Serialize, Debug)]
struct Stats{
    hp: i32,
    power: i32,
    speed: i32
}