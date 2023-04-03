fn main() {
    // let s = "hello";

    // let mut s = String::from("hello");
    // s.push_str(", world!");
    // println!("{}", s);

    // let x = 5;
    // let y = x;

    // let s1 = String::from("hello");
    // let s2 = s1;
    //
    // println!("{}, world", s1);
    // println!("{}, world", s2);

    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    //
    // println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    println!("{}", x);

    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("s1 = {},  s3 = {}", s1, s3);

    let s5 = String::from("hello");
    let (s6, len) = calculate_length(s5);

    println!("The length of '{}' is {}", s6, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
