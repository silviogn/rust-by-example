struct Rectangle<T> {
    width: T,
    height: T
}

struct RectangleTwo<T, U> {
    width: T,
    height: U
}

impl<T, U> RectangleTwo<T, U> {
    fn get_width(&self) -> &T {
        &self.width
    }
}


impl RectangleTwo<u8, u8>{
    fn get_perimeter(&self) -> u8 {
        2 * self.width + 2 * self.height
    }
}

fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

//! Utilities for comparing and ordering values.
//!
//! This module contains various tools for comparing and ordering values. In
//! summary:
//!
//! * [`Eq`] and [`PartialEq`] are traits that allow you to define total and
//!   partial equality between values, respectively. Implementing them overloads
//!   the `==` and `!=` operators.
//! * [`Ord`] and [`PartialOrd`] are traits that allow you to define total and
//!   partial orderings between values, respectively. Implementing them overloads
//!   the `<`, `<=`, `>`, and `>=` operators.
//! * [`Ordering`] is an enum returned by the main functions of [`Ord`] and
//!   [`PartialOrd`], and describes an ordering.
//! * [`Reverse`] is a struct that allows you to easily reverse an ordering.
//! * [`max`] and [`min`] are functions that build off of [`Ord`] and allow you
//!   to find the maximum or minimum of two values.
//!
//! For more details, see the respective documentation of each item in the list.
//!
//! [`max`]: Ord::max
//! [`min`]: Ord::min
fn main(){
    let rec = Rectangle{
        width: 1,
        height: 3
    };

    let rec = RectangleTwo{
        width: 1,
        height: 3
    };

    println!("{}", rec.get_width());
    println!("{}", rec.get_perimeter());

    let big = get_biggest(10, 90);
    println!("{}", big);
    let big = get_biggest(10.0, 90.988);
    println!("{}", big);

}