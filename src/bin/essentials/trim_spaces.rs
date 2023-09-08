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

    let test8 = "        123        
    ";
    assert_eq!(trim_spaces(test8), "123");
    println!("Additional test passed!");
}

fn trim_spaces(str: &str) -> &str {
    let mut start = 0;
    let mut end = 0;
    for (i, &c) in str.as_bytes().iter().enumerate() {
        if c != b' ' && c != b'\t' && c != b'\n' {
            start = i;
            break;
        }
    }

    for (i, &c) in str.as_bytes().iter().rev().enumerate() {
        if c != b' ' && c != b'\t' && c != b'\n' {
            end = str.len() - i;
            break;
        }
    }
    if start >= end {
        return "";
    }
    &str[start..end]
}