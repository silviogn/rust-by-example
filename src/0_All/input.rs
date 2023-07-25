use std::io;


fn main(){
    let mut buffer = String::new();
    println!("Enter the message");
    let _ = io::stdin().read_line(&mut buffer);
    println!("The entered message is: {}", buffer);

    let number:i32 = buffer.trim().parse().unwrap();
    println!("number + 1 {}: ", number+1);


}