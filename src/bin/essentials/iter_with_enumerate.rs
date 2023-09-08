fn main() {
    let message = ['h', 'e', 'l', 'l', 'o'];
    
    for (i, c) in message.iter().enumerate() {
        println!("Item {i} is {c}");
        if *c == 'e' {  // iter() returns a reference, so we dereference here, 
                        // or use &c above 
    //  if c == &'e' {  // or like this
            break;
        }
    }
    
    for n in 0..5 {
        println!("{n}");
    }

}
