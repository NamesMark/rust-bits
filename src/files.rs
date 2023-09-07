use std::fs;
//use std::io::prelude::*;
use std::io::Write;

pub fn main () {
    let content = fs::read_to_string("../assets/planets.txt").unwrap();
    println!("contents is {content}");

    for line in content.lines() {
        println!("Line is {line}");
    }

    let content = fs::read("../assets/planets.txt").unwrap();
    println!("contents is {content:?}");

    let mut speech = String::new();
    speech.push_str("We chose to go\n");
    speech.push_str("Boldly into\n");
    speech.push_str("The unknown\n");
    speech.push_str("Not because it's easy\n");
    speech.push_str("But because it's fun\n");

    fs::write("../assets/speech.txt", speech);
    println!("Write successful");

    let mut file = fs::OpenOptions::new().append(true).open("../assets/speech.txt").unwrap();
    file.write(b"\n...And cool");
    println!("Append successful");
}