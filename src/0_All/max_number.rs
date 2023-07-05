fn main(){

    let numbers = vec![5, 2, 8, 1, 3];

    let max_element= numbers.iter().max();

    match max_element {
        Some(&max) => println!("Maximum element number {num}", num = max),
        None => println!("The vector is empty!"),
    }

}