fn main() {
    // let mut s = String::new();
    //
    // let data = "initial contents";
    //
    // let s = data.to_string();
    //
    // let s = "initial contents".to_string();
    //
    // let s = String::from("initial contents");
    //
    // let s = String::from("foo");
    // s.push_str("bar");

    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2 is {s2}");

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;

    let s1 = "你好";
    for c in s1.chars() {
        println!("{c}");
    }

    for b in s1.bytes() {
        println!("{b}");
    }
}
