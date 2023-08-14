use std::env;

fn main(){
    for (index, argument) in env::args().enumerate(){
        println!("argument {} value {}", index, argument);
    }

    let arg2 = env::args().nth(1).unwrap();
    println!("arg 1 is {}", arg2);
}