use serde::{Serialize, Deserialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct UnitCard {
    id: i32,
    card_type: String,
    name: String,
    description: String,
    card_stats: Stats
}

#[derive(Deserialize, Serialize, Debug)]
struct Stats{
    hp: i32,
    power: i32,
    speed: i32
}

#[derive(Deserialize, Serialize, Debug)]
struct ItemCard{
    id: i32,
    item_type: String,
    item_name: String,
    item_description : String,
    item_stats: Stats
}