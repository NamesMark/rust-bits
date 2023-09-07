fn main() {
    let mut count = 0;
    
    let res = loop {
        if count == 10 {
            break count * 10; // while loop cant do that
        }
        count += 1;
        println!("Count is {count}");
    };
    
    println!("Result is {res}");
}
