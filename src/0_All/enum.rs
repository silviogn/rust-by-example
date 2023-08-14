
#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64)
}

impl Shape {
    fn get_perimeter(&self) -> f64 {
        match *self {
            Shape::Circle(r) => r * 1.1,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle(a,b,c) => a * b * c
        }
    }
}

/*
enum Command {
    Clear,
    DrawLine(f64, f64),
    DrawShape(Shape)
}
*/
// Enum can be used to match expressions
fn main(){
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("{:?}", my_shape);

    match my_shape {
        Shape::Circle(r) => println!("Circle with radius {}", r),
        Shape::Rectangle(w, h) => println!("Rectangle with radius {} {}", w, h),
        Shape::Triangle(a, b,c ) => println!("Triangle {} {} {}", a, b, c)
    }

    let my_value = 100;

    let result = match my_value {
        0 => "A",
        10 => "B",
        _ => "C"
    };
    println!("{}", result);
    let perimeter = my_shape.get_perimeter();
    println!("{}", perimeter);


    // Option<T> Enum - Some(T), None
    let something = Some(13);
    let result = match something {
        Some(s) => s,
        None => 0
    };

    println!("{}", result);

    let count = [1,2,3,4,5];
    let number = count.get(1).unwrap();
    let number_op = count.get(900).unwrap_or(&-1);
    println!("{}", number_op);

    // if let
    let new_number = Some(13);

    if let Some(13) = new_number {
        println!("thirteen")
    } else {
        println!("Another value");
    }
}