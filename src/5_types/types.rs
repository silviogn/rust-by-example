 //Casting
// Rust provides no implicit type conversion (coercion) between primitive types.
// But, explicit type conversion (casting) can be performed using the as keyword.

// Rules for converting between integral types follow C conventions generally, except in cases
// where C has undefined behavior. The behavior of all casts between integral types is well defined
// in Rust.

fn main(){
    let _decimal = 65.100_f32;
    let _integer: u32 = _decimal as u32;

    //Explicit conversion
    let _integer = _decimal as u8;
    let _character = _integer as char;

    // Error! There are limitations in conversion rules.
    // A float cannot be directly converted to a char.
    let _character = _decimal as u8 as char;
    // FIXME ^ Comment out this line
    println!("Casting: {} -> {} -> {}", _decimal, _integer, _character);

    // when casting any value to an unsigned type, T,
    // T::MAX + 1 is added or subtracted until the value
    // fits into the new type
    // 1000 already fits in a u16
    println!("1000 as a u16 is: {}", 1000i16 as u16);
    // -1 + 256 = 255
    println!("-1 as a u8 is : {}", (-1i8) as u8);
    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);


}