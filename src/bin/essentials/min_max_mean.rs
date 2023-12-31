use std::cmp;

fn main() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32 = 0;
    let mut min: i32 = i32::MAX;
    let mut mean: f64 = 0.;

    for n in numbers {
        max = cmp::max(max, n);
        min = cmp::min(min, n);
        mean += n as f64;
    }
    mean /= numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
}
