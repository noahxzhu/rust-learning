#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Default for Person {
    fn default() -> Self {
        Person {
            name: "Bot".to_string(),
            age: 0,
        }
    }
}

fn create_default() {
    let tmp = Person {
        ..Default::default()
    };
    let tmp = Person {
        name: "Sam".to_string(),
        ..Default::default()
    };
}

fn main() {
    println!("Hello, world!");
}
