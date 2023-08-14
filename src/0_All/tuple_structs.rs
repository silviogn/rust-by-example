struct Color(u8, u8, u8);

struct Point(u8, u8, u8);

fn get_y(p: Point) -> u8 {
    p.1
}


fn main() {
    let red = (255, 0, 0);
    println!("{}", red.0);

    let y = get_y(Point(255, 0, 0);
}
