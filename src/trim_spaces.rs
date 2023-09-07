fn main() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");
    
    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");
    
    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");   
    
    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");
    
    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");
    
    let test6 = "";
    assert_eq!(trim_spaces(test6), "");
    
    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}

fn trim_spaces(str: &str) -> &str {
    let mut start = 0;
    let mut end = 0;
    for (i, &c) in str.as_bytes().iter().enumerate() {
        if c != b' ' {
            start = i;
            break;
        }
    }

    for (i, &c) in str.as_bytes().iter().rev().enumerate() {
        if c != b' ' {
            end = str.len() - i;
            break;
        }
    }
    if start >= end {
        return "";
    }
    &str[start..end]
}