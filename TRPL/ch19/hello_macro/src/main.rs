use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
