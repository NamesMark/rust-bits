fn main () {
    let a = 13;
    let b = 2.3;
    let c : f32 = 120.0;
    
    let mut avg : f32 = (a as f32 + b as f32 + c) / 3.0;
    println!("{}",avg);
    avg = (avg*10.0).round()/10.0; // because in f32 it's 45.100002 so I round
    
    assert_eq!(avg, 45.1);
    println!("Passed 1");
    
    let avg2 = (a as f64 + b + c as f64) / 3.0; // in f64 it's fine
    
    assert_eq!(avg2, 45.1);
    println!("Passed 2");
    
}
