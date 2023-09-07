fn main() {
    let test1 = "We need more space.";
    assert_eq!(single_word_by_number(test1, 2), "need");
    
    let test2 = String::from("   There's space in front.");
    assert_eq!(single_word_by_number(&test2, 1), "There's");
    
    let test3 = String::from("We need more space.");
    assert_eq!(single_word_by_number(&test3, 4), "space.");
    
    let test4 = String::from("We need more space.");
    assert_eq!(single_word_by_number(&test4, 5), "");
    println!("Tests passed!");
}

fn single_word_by_number(str: &str, target: u32) -> &str {
    let mut start = 0;
    let mut count = 0;
    let string = &str.trim();
    
    for (i, c) in string.chars().enumerate() {
        if c == ' ' {
            count += 1;
            if count == target {
                return &string[start..i];
            }
            start = i+1;
        }
    }
    if count + 1 == target {
        return &string[start..];
    }
    return ""
    
}

