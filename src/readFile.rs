use std::fmt;
use std::fs;



pub fn read_f(file_name: String) -> String {
    /**
    @return: File contents as a string
    **/
    let contents: String = fs::read_to_string("./card_stats/moose.json")
        .expect("Something went wrong reading the file");
    return contents;
}