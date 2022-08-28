use serde::{Serialize, Deserialize};
use lombok::{Builder};

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
pub struct ItemCard{
    id: i32,
    card_type: String,
    item_name: String,
    item_description : String,
    item_stats: Stats
}

impl ItemCard{
    pub fn print_val(&self) -> &String{
        return &self.item_name;
    }
}
#[derive(Builder, Debug)]
pub struct test{
    pub id: i32,
    pub name : String
}