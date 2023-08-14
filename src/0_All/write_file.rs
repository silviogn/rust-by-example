use std::fs;
use std::io::prelude::*;

fn main(){
    let mut speech = String::new();

    speech.push_str("We choose to go to the moon \n");
    speech.push_str("we are ready to go to the moon.\n");

    match fs::write("speech.txt", speech) {
        Ok(()) => println!("Write ok"),
        Err(ex) => println!("Write error please check {}", ex),
    }

    let mut file = fs::OpenOptions::new().append(true).open("speech.txt").unwrap();
    let _ = file.write(b"\n My name is pluto");

}