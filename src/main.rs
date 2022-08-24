
pub mod readFile;

fn main() {
    let file_contents=readFile::read_f("moose.json".to_string());
    readFile::print_f(file_contents);
}
