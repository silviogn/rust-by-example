fn main(){
    // testing tuples
    let mut stuff = (1,2,3,4,5);
    println!("{}", stuff.4);
    stuff.1 += 10;
    println!("{}", stuff.1);

    let _a = {
        println!("This is an expression");
        100 + 900
    };

    let celsius: f32 = 20.0;
    println!("Converting C to F -> {c} to {f} ",c = celsius, f = convert_temperature(celsius));

    let mut count: u8 = 0;
    loop {
        println!("{}",count);
        count+=1;
        if count == 5 {
            break;
        }
    }

    let mut count2:u32 = 0;
   let result:u32 = loop {
        println!("{}",count2);
       count2+=1;
        if count2 == 5 {
            break count2;
        }
    };

    println!("The result is {}", result);

    let numbers = [1,2,3];
    let a = numbers.min();
    

}


fn convert_temperature(celsius: f32) -> f32 {
    // F = (1.8 * C) + 32
    return (1.8f32 * celsius) + 32 as f32;
}
