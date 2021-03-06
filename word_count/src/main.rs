use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("./bible.txt").expect("File was not found");
    let mut bible = String::new();
    f.read_to_string(&mut bible)
        .expect("Something went wrong with reading the file");
    let bible_split = bible.split(" ");
    let mut and_count = 0;
    let mut but_count = 0;
    for word in bible_split {
        match word {
            "and" => and_count += 1,
            "but" => but_count += 1,
            _ => {}
        }
    }
    println!("The word 'and' is used {} times", and_count);
    println!("The word 'but' is used {} times", but_count);
}
