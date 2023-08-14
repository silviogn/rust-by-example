use std::fs;
use std::io;

fn main(){
    //panic!("Error panic!")

    let count_down = [2,1,0];

    //for a in count_down {
       // let ab = 10 / a;
       // println!("{}", ab);
    //}

    let result = fs::read_to_string("the.txt");

    let contents = match result {
        Ok(m) => m,
        Err(error) => match error.kind()  {
            io::ErrorKind::NotFound => String::from("File not found"),
            io::ErrorKind::PermissionDenied => String::from("Permission denied"),
            _ => panic!("Another type of error {:?}", error)
        }
    };
    println!("content is {}", contents);
}