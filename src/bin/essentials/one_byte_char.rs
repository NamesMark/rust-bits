fn main() {
    let character = 'A'; // This is a char type
    let byte_value: u8 = character as u8; // Cast char to u8
    let back_to_char: char = byte_value as char; // Cast u8 back to char

    println!("character: {character}");
    println!("byte_value: {byte_value}");
    println!("back_to_char: {back_to_char}");
}