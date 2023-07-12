fn main(){
    let mut rocket_fuel = String::from("RP-1");
    let length = process_fuel_size(&mut rocket_fuel);
    println!("The rocket fuel is {} and the len is {}", rocket_fuel, length);
}

fn process_fuel_size(value: &mut String) -> usize {
    value.push_str("it is flammable");
    value.len()
}