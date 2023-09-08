use std::fmt;
use std::cmp::Ordering;

struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"A {} with a velocity of {} km/s", self.name, self.velocity)
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

trait Altitude {
    fn get_altitude (&self) -> f64;
}

impl Altitude for Satellite {
    fn get_altitude (&self) -> f64 {
        const G: f64 = 6.67430e-11;     // gravitational constant in m^3 kg^-1 s^-2
        const M: f64 = 5.972e24;        // mass of the Earth in kg
        const R_EARTH: f64 = 6371e3;    // radius of the Earth in m 
        let r = G * M / (self.velocity*1000.0).powi(2);
        r - R_EARTH
    }
}

pub fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 7.5
    };
    println!("hubble is {}", hubble);

    let webb = Satellite {
        name: String::from("James Webb Telescope"),
        velocity: 0.7772
    };
    println!("{} is larger than {} - it's {}", hubble.name,webb.name,hubble > webb);
    println!("{}'s altitude is {} km!", webb,   (webb  .get_altitude()/1000.0) as u64);
    println!("{}'s altitude is {} km!", hubble, (hubble.get_altitude()/1000.0) as u64);
}
