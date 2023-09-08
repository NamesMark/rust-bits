
use std::env;
use std::fs;
use std::collections::HashMap;

pub fn main () {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Please provide at least 2 arguments");
        return;
    }

    let path = format!("../assets/{}",args[1]); // path to file

    let mut word_counts: HashMap<String, u32> = HashMap::new();

    let buffer = fs::read_to_string(path);

    let contents = match buffer {
        Ok(l) => l,
        Err(E) => {
            println!("Failed to read the file, got an error {E}");
            "".to_string()
        }
    };

    for word in contents.split_whitespace() {
        let current_word_count = word_counts.entry(word.to_string()).or_insert(0);
        *current_word_count += 1;
    }

    let mut max_count = 0;
    let mut most_popular_words : Vec<&String> = Vec::new();
    for (key, val) in word_counts.iter() {
        if val > &max_count {
            max_count = *val;
            most_popular_words.clear();
            most_popular_words.push(key);
        } else if val == &max_count {
            most_popular_words.push(key);
        }
    }
    println!("The most common words are {most_popular_words:?} with a count of {max_count}");

}