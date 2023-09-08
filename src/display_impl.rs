use std::fmt;
use std::cmp::Ordering;

struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"A {} with a velocity of {}", self.name, self.velocity)
    }
}

impl PartialOrd for Satellite {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.velocity.partial_cmp(&other.velocity)
    }
}

impl PartialEq for Satellite {
    fn eq(&self, other: &Self) -> bool {
        self.velocity == other.velocity
    }
}


pub fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    println!("hubble is {}", hubble);

    let webb = Satellite {
        name: String::from("James Webb Telescope"),
        velocity: 0.7772
    };
    println!("{} is larger than {} - it's {}",hubble.name,webb.name,hubble > webb);
}
