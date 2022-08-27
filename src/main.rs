use crate::card::Card;

pub mod read_file;
pub mod card;

fn main() {
    let file_contents= read_file::read_f("moose.json".to_string());
    
    let deserialized: Card = serde_json::from_str(&*file_contents).unwrap();
    println!("Hi");
    //readFile::print_f(file_contents);
}
