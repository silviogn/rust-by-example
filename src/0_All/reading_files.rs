use std::fs;
use std::io::BufRead;

fn main(){
    let contents = fs::read_to_string("planets.txt").unwrap();
    println!("{}", contents);

    for line in contents.lines(){
        println!("{}", line);
    }
}