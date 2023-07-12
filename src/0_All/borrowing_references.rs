fn main(){
    let rocket_fuel = String::from("RP-1");
    let length = process_fuel_size(&rocket_fuel);
    println!("The rocket fuel is {} and the len is {}", rocket_fuel, length);
}

fn process_fuel_size(value: &String) -> usize {
    value.len()
}