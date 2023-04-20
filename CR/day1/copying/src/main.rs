#[derive(Clone, Debug)]
struct Point(i32, i32, String);

fn main() {
    // let x = 42;
    // let y = x;
    // println!("x: {x}");
    // println!("y: {y}");
    let p1 = Point(3, 4, String::from("hello"));
    let p2 = p1.clone();
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}
