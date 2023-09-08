
use std::env;
use std::fs;
use std::collections::HashMap;

pub fn main () {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide 1 argument (source to the file)");
        std::process::exit(1);
    }

    let path = format!("../assets/{}",args[1]); // path to file

    let buffer = fs::read_to_string(path);

    let contents = match buffer {
        Ok(l) => l,
        Err(e) => {
            println!("Failed to read the file, got an error {e}");
            std::process::exit(2);
            //"".to_string()
        }
    };
    
    let mut word_counts: HashMap<String, u32> = HashMap::new();
    
    for word in contents.split_whitespace() {
        let cleaned_word = word.to_lowercase().trim_matches(|c: char| !c.is_alphanumeric()).to_string();
        let current_word_count = word_counts.entry(cleaned_word).or_insert(0);
        *current_word_count += 1;
    }
    
    let mut max_count = 0;
    let mut most_popular_words : Vec<&str> = Vec::new();

    for (key, val) in word_counts.iter() {
        if val > &max_count {
            max_count = *val;
            most_popular_words.clear();
            most_popular_words.push(key);
        } else if val == &max_count {
            most_popular_words.push(key);
        }
    }
    let number_of_words = most_popular_words.len();
    match number_of_words {
        0 => println!("There's no words."),
        1 => println!("The most common word is \"{most_popular}\" with a count of {max_count}", most_popular=most_popular_words[0]),
        _ => println!("The most common words are {most_popular_words:?} with a count of {max_count}")
    };


}

/* 
    Examples:

    cargo run earth_to_the_moon.txt
    The most common word is "the" with a count of 7983

    cargo run speech.txt
    The most common words are ["it's", "because"] with a count of 2

    cargo run planets.txt
    The most common words are ["venus", "mercury", "mars", "earth", "jupiter", "saturn", "uranus", "neptune"] with a count of 1
*/