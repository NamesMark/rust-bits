fn main() {
    let temperature = 23.0;
    
    let fahrenheit = celsius_to_fahrenheit(temperature);
    println!("{temperature}c is {fahrenheit}f");
    
    assert_eq!(fahrenheit,73.4);
    println!("Passed");
}
fn celsius_to_fahrenheit (centigrade : f64) -> f64 {
    1.8 * centigrade + 32.0
}