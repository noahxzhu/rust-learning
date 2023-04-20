fn main() {
    let mut a: i32 = 10;
    let b: &i32 = &a;
    println!("b: {b}");

    {
        let c: &mut i32 = &mut a;
        *c = 20;
    }

    println!("a: {a}");
}
