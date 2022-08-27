use std::borrow::Borrow;
use crate::card::UnitCard;

pub mod read_file;
pub mod card;

fn main() {
    let temp_unit:UnitCard = read_file::read_f("moose.json".to_string());


    println!("{}",temp_unit.borrow("id"))
}
