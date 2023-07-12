
fn main(){
    let rocket_fuel = 1;
    process_fuel(rocket_fuel);
    process_fuel_value_change(rocket_fuel);
    println!("the rocket fuel is {}", rocket_fuel);

    let rocket_fuel_string = String::from("this is the ERP");
    process_fuel_string(rocket_fuel_string.clone());
    println!("the rocket fuel string is {}", rocket_fuel_string);

    let size: usize = 200;
    println!("metadata about size: {}", size);

}

fn process_fuel_string(value: String){
    println!("processing value {value_fetch}", value_fetch = value);
}


fn process_fuel(value: i32){
    println!("processing value {value_fetch}", value_fetch = value);
}

fn process_fuel_value_change(mut value: i32){
    value+=1;
    println!("processing value change {value_fetch}", value_fetch = value);
}





