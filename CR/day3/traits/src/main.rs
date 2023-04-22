trait Pet {
    fn name(&self) -> String;
}

struct Dog {
    name: String,
}

struct Cat;

impl Pet for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Pet for Cat {
    fn name(&self) -> String {
        String::from("The cat")
    }
}

fn main() {
    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Dog {
            name: String::from("Fido"),
        }),
        Box::new(Cat),
    ];

    for pet in pets {
        println!("Hello {}!", pet.name());
    }
    println!(
        "{} {}",
        std::mem::size_of::<Dog>(),
        std::mem::size_of::<Cat>()
    );
    println!(
        "{} {}",
        std::mem::size_of::<&Dog>(),
        std::mem::size_of::<&Cat>()
    );
    println!("{}", std::mem::size_of::<&dyn Pet>());
    println!("{}", std::mem::size_of::<Box<dyn Pet>>());
}
