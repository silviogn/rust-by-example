use std::fmt::{Display, Formatter};
use std::fs::create_dir;

#[derive(Debug, Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons
    }

    // associated function
    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0,
        }
    }
}

fn main() {
    let mut sh = Shuttle {
        name: String::from("Enhavour"),
        crew_size: 7,
        propellant: 9000.90,
    };

    let second_sh = Shuttle {
        name: String::from("second vehicle"),
        // Tells to Rust that the not specified fields
        // must be filled by the values of the references
        // struct instance
        ..sh
    };

    let second_sh2 = Shuttle { ..sh.clone() };

    println!("second vehicle crew size {}", second_sh.crew_size);
    println!("name is {}", sh.name);

    let ve_name = sh.get_name();
    println!("get name working = {}", ve_name);

    println!("gallons initially = {}", sh.propellant);
    sh.add_fuel(1000.0);
    println!("gallons after add = {}", sh.propellant);

    let mut vehicle = Shuttle::new("Machine");
    println!("Machine name = {}", vehicle.name);
}
