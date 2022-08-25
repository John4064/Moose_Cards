
pub mod readFile;
pub mod card;

fn main() {
    let file_contents=readFile::read_f("moose.json".to_string());
    println!("Hi");
    //readFile::print_f(file_contents);
}
