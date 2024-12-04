use std::{fs, io, string};

fn main() {
    println!("In file {day1}");
}

fn get_data(filepath: String) {
    println!("Reading file of {filepath}");

    let contents = fs::read_to_string(filepath)
        .expect("Should have been readable");

    return contents
}
