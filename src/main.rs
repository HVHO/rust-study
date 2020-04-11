
struct Point {
    x: i32,
    y: i32
}


fn main() {

    let x = Some("hello");
    let y = 10;

    match x {
        Some(&x)if x == y => println!("case 1"),
        _ => println!("case 2")
    }
}