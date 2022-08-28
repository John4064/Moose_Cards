use crate::card::{ItemCard, UnitCard,test};

pub mod read_file;
pub mod card;

fn main() {
    let temp_unit:ItemCard = read_file::read_f("items/dagger.json".to_string());
    let albino: test = test{ id: 0, name: "".to_string() };
    println!("{}",test::);
}
