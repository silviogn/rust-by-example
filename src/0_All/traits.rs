use std::{any, fmt};

// Derivable traits
#[derive(PartialEq, PartialOrd)]
struct Satellite {
    name: String,
    velocity: f64,
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32,
}

trait Description {
    fn describe(&self) -> String;

    fn describe_default(&self) -> String {
        format!("This is the default implementation")
    }
}

impl Description for Satellite {
    fn describe(&self) -> String {
        format!("{} {}", self.name, self.velocity)
    }
}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!("{} {}", self.name, self.altitude)
    }
}

// traits bounds

fn print_type<T: fmt::Display>(item: T) {
    println!("{} is {}", item, any::type_name::<T>())
}

// multiple trait bounds
fn compare_and_print<T, U>(a: T, b: U)
where
    T: fmt::Display + PartialEq + From<U>,
    U: fmt::Display + PartialEq + Copy,
{
    if a == T::from(b) {
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is NOT equal to {}", a, b);
    }
}

// return types with implemented tratits
fn get_displayable() -> impl fmt::Display {
    13
}


//! This is a test for the documentation.
//! This is another line for the documentation that I am writing right now.
//! End.
fn main() {
    let hubble = Satellite {
        name: "Hubble Telescope".to_string(),
        velocity: 5.72,
    };

    let no_hubble = Satellite {
        name: "Hubble Telescope".to_string(),
        velocity: 4.80,
    };

    let iss = SpaceStation {
        name: "International Space Station".to_string(),
        crew_size: 6,
        altitude: 254,
    };

    println!("hubble is {}", hubble.describe());
    println!("iss is {}", iss.describe());
    println!("{}", iss.describe_default());
    println!("hubble == gps is {}", hubble == no_hubble);
    println!("hubble >= gps is {}", hubble >= no_hubble);

    print_type(100.900);
    print_type(100);
    print_type("hello");

    compare_and_print(1.0, 1);
    compare_and_print(1.1, 1);

    println!("output is {}", get_displayable());
}
