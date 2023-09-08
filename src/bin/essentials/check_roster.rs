use std::env;
use std::fs;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Please provide at least 2 arguments");
        return;
    }
    let path = format!("../assets/{}",args[1]);
    let name: &String = &args[2];
    let contents = fs::read_to_string(path).unwrap();
    for (i, line) in contents.lines().enumerate() {
        if line.contains(name) {
            println!("Moonwalker {name} is on the roster! He's number {index}.",index=i+1);
            return;
        }
    }
    println!("Moonwalker {name} is not on the roster 😔");
}