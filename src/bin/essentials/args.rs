use std::env;

pub fn main() {
    // Good idea: 
    // let args: Vec<String> = env::args().collect();

    if env::args().len() <= 2 {
        println!("At least 2 arguments please.");
        return;
    }
    for (i, argument) in env::args().enumerate() {
        println!("Argument {i} is {argument}");
    }

    let arg2 = env::args().nth(2).unwrap();
    println!("arg2 is {arg2}");
}

/*
    Example:
    Argument 0 is PATH
    Argument 1 is one
    Argument 2 is 2
    Argument 3 is three
    Argument 4 is 4.0
*/