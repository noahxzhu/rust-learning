use std::usize;

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("{}", s2);

    let mut s3 = String::from("hello");
    {
        let r1 = &mut s3;
        println!("{}", r1);
    }

    let r2 = &mut s3;
    println!("{}", r2);

    // println!("{}, {}", r1, r2);

    let mut s11 = String::from("hello");
    let r11 = &s11;
    let r22 = &s11;

    println!("{}, {}", r11, r22);
    // println!("{}, {}", r11, r22);

    // if r11 and r22 no longer used, we can add a new mutable reference again
    let r33 = &mut s11;

    // println!("{}, {}", r11, r22);
    println!("{}", r33);

    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String {
    String::from("hello")
    // let s = String::from("hello");
    // &s
}
