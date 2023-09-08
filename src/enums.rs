use std::fmt;

enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64)
}

impl Location {
    fn display(&self) {
        match self {
            Location::Known(x, y) => println!("{x}, {y}"),
            _ => println!("Location is unknown.")
        }
    } 
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Location::Known(x, y) => return write!(f, "Location is {x}, {y}"),
            _ => return write!(f, "Location is unknown.")
        }
    }
}

pub fn main() {
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(28.608295, -80.604177);
    address.display();

    println!("{address}");
}
