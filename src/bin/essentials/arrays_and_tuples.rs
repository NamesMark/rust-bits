fn main() {
    let numbers : [i32; 5] = [9; 5];
    //let numbers = [9; 5]; is also fine
    
    println!("last {}",numbers[4]);
    
    let mut items: (u8, f32, char) = (10, 3.14, 'x');
    //let mut items = (10, 3.14, 'x'); is also fine
    items.0 += 5;
    let first = items.0;
    println!("First is {first}");
    
    let (a, b, c) = items;
    println!("{a}, {b}, {c}");
}