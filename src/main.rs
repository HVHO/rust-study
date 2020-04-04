use std::fmt::Display;

struct Point<T> {
    x: T,
}

impl<T> Point<T>
{
    pub fn new(x: T) -> Point<T> {
        Point{
            x
        }
    }
    pub fn do_something<U,V,W>(&self, y: U, a: V, b: W) {
        println!("{}{}{}{}",self.x,y,a,b);
    }

}

fn main() {

    let point = Point::new(1);
    point.do_something(1,2,4);

}